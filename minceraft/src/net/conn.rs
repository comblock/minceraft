use super::packet::*;
use aes::Aes128;
use anyhow::Result;
use cfb8::{
    cipher::{AsyncStreamCipher, NewCipher},
    Cfb8,
};
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::time::Duration;
use std::{
    convert::TryFrom,
    io::{self, Write},
    net::{TcpListener, ToSocketAddrs},
};

#[derive(Clone, Copy)]
pub enum ProtocolState {
    Handshake,
    Status,
    Login,
    Play,
}

impl ProtocolState {
    pub fn to_i32(&self) -> i32 {
        match self {
            ProtocolState::Handshake => 0,
            ProtocolState::Status => 1,
            ProtocolState::Login => 2,
            ProtocolState::Play => 3,
        }
    }
}

pub struct Listener(pub TcpListener);

impl Listener {
    pub fn bind(addr: impl ToSocketAddrs) -> Result<Listener> {
        Ok(Listener(TcpListener::bind(addr)?))
    }
    pub fn accept(&mut self) -> Result<Conn> {
        Ok(Conn::try_from(self.0.accept()?.0)?)
    }
}

/// Conn wraps around TcpStream to simplify sending and receiving packets.
pub struct Conn {
    pub peer: SocketAddr,
    pub stream: TcpStream,
    /// State is set to Handshake on connect but is not handled by Conn.
    pub state: ProtocolState,
    cipher: Option<Cipher>,
    pub writer: io::BufWriter<TcpStream>,
    pub reader: io::BufReader<TcpStream>,
    pub threshhold: i32,
}

struct Cipher {
    write: Cfb8<Aes128>,
    read: Cfb8<Aes128>,
}

impl io::Write for Conn {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.cipher.as_mut() {
            Some(ref mut cipher) => {
                let mut data = vec![0; buf.len()];
                data[..buf.len()].clone_from_slice(&buf[..]);
                cipher.write.encrypt(&mut data);

                self.writer.write(&data)
            }
            None => self.writer.write(buf),
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl io::Read for Conn {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self.cipher.as_mut() {
            Option::None => self.reader.read(buf),
            Option::Some(cipher) => {
                let ret = self.reader.read(buf)?;
                cipher.read.decrypt(&mut buf[..ret]);

                Ok(ret)
            }
        }
    }
}

impl TryFrom<TcpStream> for Conn {
    type Error = anyhow::Error;
    fn try_from(stream: TcpStream) -> Result<Self> {
        let writer = io::BufWriter::new(stream.try_clone()?);
        let reader = io::BufReader::new(stream.try_clone()?);
        Ok(Self {
            peer: stream.peer_addr()?,
            stream,
            state: ProtocolState::Handshake,
            cipher: None,
            writer,
            reader,
            threshhold: -1,
        })
    }
}

impl Conn {
    pub fn connect(addr: SocketAddr) -> anyhow::Result<Self> {
        let stream = TcpStream::connect(addr)?;
        let writer = io::BufWriter::new(stream.try_clone()?);
        let reader = io::BufReader::new(stream.try_clone()?);
        Ok(Self {
            peer: addr,
            stream,
            state: ProtocolState::Handshake,
            cipher: None,
            writer,
            reader,
            threshhold: -1,
        })
    }

    pub fn connect_timeout(addr: &SocketAddr, timeout: Duration) -> anyhow::Result<Self> {
        let stream = TcpStream::connect_timeout(addr, timeout)?;
        let writer = io::BufWriter::new(stream.try_clone()?);
        let reader = io::BufReader::new(stream.try_clone()?);
        Ok(Self {
            peer: *addr,
            stream,
            state: ProtocolState::Handshake,
            cipher: None,
            writer,
            reader,
            threshhold: -1,
        })
    }

    pub fn shutdown(&mut self) -> io::Result<()> {
        self.stream.shutdown(Shutdown::Both)
    }

    pub fn send_packet(&mut self, packet: &impl Packet) -> anyhow::Result<()> {
        packet.encode()?.pack(self, self.threshhold)?;
        self.flush()?;
        Ok(())
    }

    pub fn read_packet(&mut self) -> Result<RawPacket> {
        RawPacket::unpack(self, self.threshhold)
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
