use super::packet::*;
use aes::Aes128;
use anyhow::Result;
use cfb8::{
    cipher::{AsyncStreamCipher, NewCipher},
    Cfb8,
};
use futures::ready;
use std::net::SocketAddr;
use std::{convert::TryFrom, pin::Pin, task::Poll};
use tokio::{net::{TcpStream, ToSocketAddrs}, task::spawn_blocking};
use tokio::{
    io::{self, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader, BufWriter},
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
};

// Commented out because I don't see a reason for anyone to use this instead of `tokio::net::TcpListener`.
//pub struct Listener(pub TcpListener);
//
//impl Listener {
//    pub async fn bind(addr: impl ToSocketAddrs) -> Result<Listener> {
//        Ok(Listener(TcpListener::bind(addr).await?))
//    }
//    pub async fn accept(&mut self) -> Result<Conn> {
//        Ok(Conn::try_from(self.0.accept().await?.0)?)
//    }
//}

/// Conn wraps around TcpStream to simplify sending and receiving packets.
pub struct Conn {
    pub peer: SocketAddr,
    cipher: Option<Cipher>,
    writer: BufWriter<OwnedWriteHalf>,
    reader: BufReader<OwnedReadHalf>,
    pub threshhold: i32,
}

struct Cipher {
    write: Cfb8<Aes128>,
    read: Cfb8<Aes128>,
}

impl AsyncWrite for Conn {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize, std::io::Error>> {
        let this = self.get_mut();
        match this.cipher {
            Some(ref mut cipher) => {
                let mut data = vec![0; buf.len()];
                data[..buf.len()].clone_from_slice(&buf[..]);
                cipher.write.encrypt(&mut data);
                Pin::new(&mut this.writer).poll_write(cx, &data)
            }
            None => Pin::new(&mut this.writer).poll_write(cx, buf),
        }
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        Pin::new(&mut self.get_mut().writer).poll_flush(cx)
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        Pin::new(&mut self.get_mut().writer).poll_shutdown(cx)
    }

    //fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    //    match self.cipher.as_mut() {
    //        Some(ref mut cipher) => {
    //            let mut data = vec![0; buf.len()];
    //            data[..buf.len()].clone_from_slice(&buf[..]);
    //            cipher.write.encrypt(&mut data);
    //
    //            self.writer.write(&data)
    //        }
    //        None => self.writer.write(buf),
    //    }
    //}
    //fn flush(&mut self) -> io::Result<()> {
    //    self.writer.flush()
    //}
}

impl AsyncRead for Conn {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        let this = self.get_mut();
        match &mut this.cipher {
            Option::None => Pin::new(&mut this.reader).poll_read(cx, buf),
            Option::Some(cipher) => {
                //let initial_filled = buf.filled().len();
                ready!(Pin::new(&mut this.reader).poll_read(cx, buf))?;
                let data = buf.filled_mut();
                cipher.read.decrypt(data);
                Poll::Ready(Ok(()))
            }
        }
    }

    //fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    //    match self.cipher.as_mut() {
    //        Option::None => self.reader.read(buf),
    //        Option::Some(cipher) => {
    //            let ret = self.reader.read(buf)?;
    //            cipher.read.decrypt(&mut buf[..ret]);
    //
    //            Ok(ret)
    //        }
    //    }
    //}
}

impl TryFrom<TcpStream> for Conn {
    type Error = anyhow::Error;
    fn try_from(stream: TcpStream) -> Result<Self> {
        let peer = stream.peer_addr()?;
        let (reader, writer) = split_stream(stream);
        Ok(Self {
            peer,
            cipher: None,
            writer,
            reader,
            threshhold: -1,
        })
    }
}

impl TryInto<TcpStream> for Conn {
    type Error = anyhow::Error;
    fn try_into(self) -> Result<TcpStream> {
        let read_half = self.reader.into_inner();
        let write_half = self.writer.into_inner();
        Ok(read_half.reunite(write_half)?)
    }
}

impl Conn {
    pub async fn connect<A: ToSocketAddrs>(addr: A) -> anyhow::Result<Conn> {
        let stream = TcpStream::connect(addr).await?;
        let peer = stream.peer_addr()?;
        let (reader, writer) = split_stream(stream);
        Ok(Self {
            peer,
            cipher: None,
            writer,
            reader,
            threshhold: -1,
        })
    }

    //pub fn connect_timeout(addr: &SocketAddr, timeout: Duration) -> anyhow::Result<Self> {
    //    let stream = TcpStream::connect_timeout(addr, timeout)?;
    //    let (reader, writer) = split_stream(stream);
    //    Ok(Self {
    //        peer: *addr,
    //        cipher: None,
    //        writer,
    //        reader,
    //        threshhold: -1,
    //    })
    //}

    pub async fn shutdown(self) -> Result<()> {
        let mut stream: TcpStream = self.try_into()?;
        stream.shutdown().await?;
        Ok(())
    }

    pub async fn send_packet<T: Packet + Send + Sync + 'static>(&mut self, packet: &T) -> anyhow::Result<()> {
        //SAFETY: Since I know the spawn_blocking is awaited in this function this should be fine
        let packet = unsafe {std::mem::transmute::<&T, &'static T>(&packet)};
        let encoded = spawn_blocking(|| -> Result<RawPacket> {
                packet.encode()
            }).await??;
        encoded.pack(self, self.threshhold).await?;
        self.flush().await?;
        Ok(())
    }

    pub async fn read_packet(&mut self) -> Result<RawPacket> {
        RawPacket::unpack(self, self.threshhold).await
    }

    pub fn set_compression_threshhold(&mut self, threshhold: i32) {
        self.threshhold = threshhold;
    }

    pub fn enable_encryption(&mut self, key: &[u8]) -> anyhow::Result<()> {
        let cipher = Cfb8::<Aes128>::new_from_slices(key, key);
        let write = match cipher {
            Ok(c) => c,
            Err(e) => return Err(anyhow::anyhow!("{}", e)),
        };
        let cipher = Cfb8::<Aes128>::new_from_slices(key, key);
        let read = match cipher {
            Ok(c) => c,
            Err(e) => return Err(anyhow::anyhow!("{}", e)),
        };
        self.cipher = Some(Cipher { write, read });

        Ok(())
    }
}

fn split_stream(stream: TcpStream) -> (BufReader<OwnedReadHalf>, BufWriter<OwnedWriteHalf>) {
    let (reader, writer) = stream.into_split();
    let reader = BufReader::new(reader);
    let writer = BufWriter::new(writer);
    (reader, writer)
}
