mod builder;
use super::types::*;
use anyhow::{bail, Result};
use flate2::{
    bufread::{ZlibDecoder, ZlibEncoder},
    Compression,
};
use std::{pin::Pin};
use tokio::{
    io::{AsyncRead, AsyncWrite /*AsyncReadExt, AsyncWriteExt*/},
    task::spawn_blocking,
};

pub trait Packet: Encoder + Decoder {
    const ID: VarInt;
    fn encode(&self) -> Result<RawPacket> {
        let mut buf = Vec::new();
        self.write_to(&mut buf)?;
        Ok(RawPacket {
            id: Self::ID,
            data: buf,
        })
    }

    fn decode(raw: RawPacket) -> Result<Self> {
        let mut buf = raw.data.as_slice();
        Self::read_from(&mut buf)
    }
}

#[derive(Debug)]
pub struct RawPacket {
    pub id: VarInt,
    pub data: Vec<u8>,
}

impl AsyncWrite for RawPacket {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize, std::io::Error>> {
        Pin::new(&mut self.get_mut().data).poll_write(cx, buf)
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        Pin::new(&mut self.get_mut().data).poll_flush(cx)
    }
    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        Pin::new(&mut self.get_mut().data).poll_shutdown(cx)
    }
    //fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    //    self.data.write(buf)
    //}

    // /fn flush(&mut self) -> io::Result<()> {
    // /    self.data.flush()
    // /}
}

impl RawPacket {
    pub async fn unpack<T: AsyncRead + Unpin>(r: &mut T, threshold: i32) -> Result<Self> {
        if threshold >= 0 {
            Self::unpack_with_compression(r, threshold).await
        } else {
            Self::unpack_without_compression(r).await
        }
    }

    /// This reads the packet length and uses that to read the rest of the packet.
    /// Works for both compressed and uncompressed packets.
    async fn unpack_helper<T: AsyncRead + Unpin>(r: &mut T) -> Result<Vec<u8>> {
        // Read the first four bytes asynchronously since a varint can be a maximum of four bytes
        let mut first_four_bytes = [0u8; 4];
        tokio::io::AsyncReadExt::read_exact(r, &mut first_four_bytes).await?;

        // Spawn a thread that can block to read the packet length
        let (remaining_bytes, len) = spawn_blocking(move || -> Result<(Vec<u8>, usize)> {
            let mut first_four_bytes: &[u8] = first_four_bytes.as_slice();
            let len = VarInt::read_from(&mut first_four_bytes)?.0 as usize; // read the varint from the first four bytes
            Ok((first_four_bytes.into(), len)) // return the varint and the remaining bytes
        })
        .await??;

        let mut buf = Vec::<u8>::new();

        // Read the length of the packet - the remaining bytes asynchronously
        let mut exact_buf = vec![0u8; len - remaining_bytes.len()];
        tokio::io::AsyncReadExt::read_exact(r, &mut exact_buf).await?;

        // Write the remaining bytes to a buffer
        tokio::io::AsyncWriteExt::write_all(&mut buf, &remaining_bytes).await?;
        // Write the rest to the buffer
        tokio::io::AsyncWriteExt::write_all(&mut buf, &exact_buf).await?;
        // The buffer should now hold all of the data
        Ok(buf)
    }

    async fn unpack_without_compression<T: AsyncRead + Unpin>(r: &mut T) -> Result<Self> {
        let buf = Self::unpack_helper(r).await?;
        let (id, data) = spawn_blocking(move || -> Result<(VarInt, Vec<u8>)> {
            let mut buf = buf.as_slice();
            let id = VarInt::read_from(&mut buf)?; // Read the id
            Ok((id, buf.into()))
        })
        .await??;

        Ok(Self { id, data })
        //let len = VarInt::read_from(r)?.0 as usize;
        //let mut buf = vec![0; len];
        //tokio::io::AsyncReadExt::read_exact(&mut r, &mut buf).await?;
        //
        //let mut buf = buf.as_slice();
        //let id = VarInt::read_from(&mut buf)?;
        //
        //Ok(Self {
        //    id,
        //    data: buf.to_vec(),
        //})
    }

    async fn unpack_with_compression<T: AsyncRead + Unpin>(
        r: &mut T,
        threshold: i32,
    ) -> Result<Self> {
        let buf = Self::unpack_helper(r).await?;
        // Spawn a thread that can block to do the unpacking
        spawn_blocking(move || -> Result<RawPacket> {
            let mut buf = buf.as_slice();
            let data_len = VarInt::read_from(&mut buf)?.0;

            let id: VarInt;

            let data: Vec<u8>;

            if data_len != 0 {
                if data_len < threshold {
                    bail!(
                        "data length is smaller than threshold: {} < {}",
                        data_len,
                        threshold
                    );
                }

                if data_len > 2097152 {
                    bail!(
                        "data length is larger than protocol maximum: {} > {}",
                        data_len,
                        2097152
                    );
                }

                let mut decoder = ZlibDecoder::new(&buf[..]);
                let buf = &mut Vec::new();

                std::io::Read::read_to_end(&mut decoder, buf)?;

                let mut buf = buf.as_slice();

                id = VarInt::read_from(&mut buf)?;

                data = buf.to_vec();
            } else {
                id = VarInt::read_from(&mut buf)?;

                data = buf.to_vec();
            }

            Ok(Self { id, data })
        })
        .await?
    }

    pub async fn pack<T: AsyncWrite + Unpin>(
        &self,
        w: &mut T,
        threshold: i32,
    ) -> Result<()> {
        //SAFETY: Since I know the lifetime doesn't *have* to be 'static (but spawn_blocking requires it to be) this should be fine
        let packet = unsafe {std::mem::transmute::<&Self, &'static Self>(&self)};
        if threshold >= 0 {
            packet.pack_with_compression(w, threshold).await
        } else {
            packet.pack_without_compression(w).await
        }
    }

    async fn pack_with_compression<T: AsyncWrite + Unpin>(
        &'static self,
        w: &mut T,
        threshold: i32,
    ) -> Result<()> {
        let wb = spawn_blocking(move || -> Result<Vec<u8>> {
            let mut buf = Vec::new();
            self.id.write_to(&mut buf)?;
            let mut wb = Vec::<u8>::new();
            std::io::Write::write_all(&mut buf, &self.data)?;
            let data_len = buf.len();

            if data_len < threshold as usize {
                let mut buf2 = Vec::new();
                VarInt(0).write_to(&mut buf2)?;
                std::io::Write::write_all(&mut buf2, &buf)?;

                VarInt(buf2.len() as i32).write_to(&mut wb)?;
                std::io::Write::write_all(&mut wb, &buf2)?;
                Ok(wb)
            } else {
                let mut encoder = ZlibEncoder::new(buf.as_slice(), Compression::default());
                let mut buf2 = Vec::new();
                VarInt(data_len as i32).write_to(&mut buf2)?;
                std::io::Read::read_to_end(&mut encoder, &mut buf2)?;
                VarInt(buf2.len() as i32).write_to(&mut wb)?;
                std::io::Write::write_all(&mut wb, &buf2)?;
                Ok(wb)
            }
        })
        .await??;
        tokio::io::AsyncWriteExt::write_all(w, &wb).await?;

        Ok(())
    }

    async fn pack_without_compression<T: AsyncWrite + Unpin>(
        &'static self,
        w: &mut T,
    ) -> Result<()> {
        let wb = spawn_blocking(move || -> Result<Vec<u8>> {
            let mut buf = Vec::new();
            let mut wb = Vec::new();

            self.id.write_to(&mut buf)?;
            std::io::Write::write_all(&mut buf, self.data.as_slice())?;

            VarInt(buf.len() as i32).write_to(&mut wb)?;
            std::io::Write::write_all(&mut wb, &buf)?;

            Ok(wb)
        })
        .await??;
        tokio::io::AsyncWriteExt::write_all(w, &wb).await?;

        Ok(())
    }
}
