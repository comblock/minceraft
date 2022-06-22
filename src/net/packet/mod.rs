use anyhow::{bail, Result};
use std::{
    io::{self, Read, Write},
    vec,
};
mod builder;
use super::types::*;

use flate2::{
    bufread::{ZlibDecoder, ZlibEncoder},
    Compression,
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

impl io::Write for RawPacket {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.data.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.data.flush()
    }
}

impl RawPacket {
    pub fn unpack(r: &mut impl io::Read, threshold: i32) -> Result<Self> {
        if threshold >= 0 {
            Self::unpack_with_compression(r, threshold)
        } else {
            Self::unpack_without_compression(r)
        }
    }

    fn unpack_without_compression(r: &mut impl io::Read) -> Result<Self> {
        let len = VarInt::read_from(r)?.0 as usize;
        let mut buf = vec![0; len];
        r.read_exact(&mut buf)?;

        let mut buf = buf.as_slice();
        let id = VarInt::read_from(&mut buf)?;

        Ok(Self {
            id,
            data: buf.to_vec(),
        })
    }

    fn unpack_with_compression(r: &mut impl io::Read, threshold: i32) -> Result<Self> {
        let pk_len = VarInt::read_from(r)?.0 as usize;
        let mut buf = vec![0u8; pk_len];
        r.read_exact(&mut buf)?;
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

            decoder.read_to_end(buf)?;

            let mut buf = buf.as_slice();

            id = VarInt::read_from(&mut buf)?;

            data = buf.to_vec();
        } else {
            id = VarInt::read_from(&mut buf)?;

            data = buf.to_vec();
        }

        Ok(Self { id, data })
    }

    pub fn pack(&mut self, w: &mut impl io::Write, threshold: i32) -> Result<()> {
        if threshold >= 0 {
            self.pack_with_compression(w, threshold)
        } else {
            self.pack_without_compression(w)
        }
    }

    fn pack_with_compression(&mut self, w: &mut impl io::Write, threshold: i32) -> Result<()> {
        let mut buf = Vec::new();
        self.id.write_to(&mut buf)?;
        buf.write_all(&self.data)?;
        let data_len = buf.len();

        if data_len < threshold as usize {
            let mut buf2 = Vec::new();
            VarInt(0).write_to(&mut buf2)?;
            buf2.write_all(&buf)?;
            VarInt(buf2.len() as i32).write_to(w)?;
            w.write_all(&buf2)?;
        } else {
            let mut encoder = ZlibEncoder::new(buf.as_slice(), Compression::default());
            let mut buf2 = Vec::new();
            VarInt(data_len as i32).write_to(&mut buf2)?;
            encoder.read_to_end(&mut buf2)?;
            VarInt(buf2.len() as i32).write_to(w)?;
            w.write_all(&buf2)?;
        }

        //if self.data.len() < threshold as usize {
        //    let buf = &mut Vec::new();
        //    VarInt(0).write_to(buf)?;
        //    self.id.write_to(buf)?;
        //    buf.write_all(&self.data)?;
        //    VarInt(buf.len() as i32).write_to(w)?;
        //    w.write_all(buf)?;
        //} else {
        //    let buf: &mut Vec<u8> = &mut Vec::new();
        //    self.id.write_to(buf)?;
        //    buf.write_all(&self.data)?;
        //    let len = VarInt(buf.len() as i32);
        //
        //    let buf2 = &mut Vec::new();
        //    len.write_to(buf2)?;
        //    ZlibEncoder::new(buf.as_slice(), Compression::default()).read_to_end(buf2)?;
        //
        //    let len = VarInt(buf2.len() as i32);
        //    len.write_to(w)?;
        //    w.write_all(buf2)?;
        //}
        Ok(())
    }

    fn pack_without_compression(&self, w: &mut impl io::Write) -> Result<()> {
        let buf = &mut Vec::new();

        self.id.write_to(buf)?;
        buf.write_all(self.data.as_slice())?;

        VarInt(buf.len() as i32).write_to(w)?;

        w.write_all(buf.as_slice())?;

        Ok(())
    }
}
