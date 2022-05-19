use anyhow::{bail, Result};
use byteorder::{ReadBytesExt, WriteBytesExt, BE};
use std::io;
use uuid::Uuid;

#[inline]
pub fn write_bool(t: &mut impl io::Write, v: bool) -> Result<()> {
    t.write_u8(v as u8).map_err(From::from)
}

#[inline]
pub fn read_bool(t: &mut impl io::Read) -> Result<bool> {
    let i = t.read_u8()?;
    if i == 0 {
        return Ok(false);
    } else if i == 1 {
        return Ok(true);
    }
    Err(anyhow::Error::msg("not a bool"))
}

#[inline]
pub fn write_byte(t: &mut impl io::Write, v: i8) -> Result<()> {
    t.write_i8(v).map_err(From::from)
}

#[inline]
pub fn read_byte(t: &mut impl io::Read) -> Result<i8> {
    t.read_i8().map_err(From::from)
}

#[inline]
pub fn write_unsigned_byte(t: &mut impl io::Write, v: u8) -> Result<()> {
    t.write_u8(v).map_err(From::from)
}

#[inline]
pub fn read_unsigned_byte(t: &mut impl io::Read) -> Result<u8> {
    t.read_u8().map_err(From::from)
}

#[inline]
pub fn write_short(t: &mut impl io::Write, v: i16) -> Result<()> {
    t.write_i16::<BE>(v).map_err(From::from)
}

#[inline]
pub fn read_short(t: &mut impl io::Read) -> Result<i16> {
    t.read_i16::<BE>().map_err(From::from)
}

#[inline]
pub fn write_unsigned_short(t: &mut impl io::Write, v: u16) -> Result<()> {
    t.write_u16::<BE>(v).map_err(From::from)
}

#[inline]
pub fn read_unsigned_short(t: &mut impl io::Read) -> Result<u16> {
    t.read_u16::<BE>().map_err(From::from)
}

#[inline]
pub fn write_int(t: &mut impl io::Write, v: i32) -> Result<()> {
    t.write_i32::<BE>(v).map_err(From::from)
}

#[inline]
pub fn read_int(t: &mut impl io::Read) -> Result<i32> {
    t.read_i32::<BE>().map_err(From::from)
}

#[inline]
pub fn write_long(t: &mut impl io::Write, v: i64) -> Result<()> {
    t.write_i64::<BE>(v).map_err(From::from)
}

#[inline]
pub fn read_long(t: &mut impl io::Read) -> Result<i64> {
    t.read_i64::<BE>().map_err(From::from)
}

#[inline]
pub fn write_float(t: &mut impl io::Write, v: f32) -> Result<()> {
    t.write_f32::<BE>(v).map_err(From::from)
}

#[inline]
pub fn read_float(t: &mut impl io::Read) -> Result<f32> {
    t.read_f32::<BE>().map_err(From::from)
}

#[inline]
pub fn write_double(t: &mut impl io::Write, v: f64) -> Result<()> {
    t.write_f64::<BE>(v).map_err(From::from)
}

#[inline]
pub fn read_double(t: &mut impl io::Read) -> Result<f64> {
    t.read_f64::<BE>().map_err(From::from)
}

#[inline]
pub fn write_string(t: &mut impl io::Write, v: &String) -> Result<()> {
    write_var_int(t, v.len() as i32)?;
    t.write_all(v.as_bytes()).map_err(From::from)
}

#[inline]
pub fn read_string(t: &mut impl io::Read) -> Result<String> {
    let len = read_var_int(t)? as usize;
    let mut buf = vec![0; len];
    t.read_exact(&mut buf)?;
    Ok(String::from_utf8(buf)?)
}

#[inline]
pub fn write_chat(t: &mut impl io::Write, v: &String) -> Result<()> {
    if v.len() > 262144 {
        return Err(anyhow::Error::msg(
            "chat message exceeds maximum length of 26144",
        ));
    }
    write_string(t, v)
}

#[inline]
pub fn read_chat(t: &mut impl io::Read) -> Result<String> {
    let chat = read_string(t)?;
    if chat.len() > 262144 {
        return Err(anyhow::Error::msg(
            "chat message exceeds maximum length of 26144",
        ));
    }
    Ok(chat)
}

#[inline]
pub fn write_identifier(t: &mut impl io::Write, v: &String) -> Result<()> {
    if v.len() > 32767 {
        return Err(anyhow::Error::msg(
            "identifier exceeds maximum length of 32767",
        ));
    }
    write_string(t, v)
}

#[inline]
pub fn read_identifier(t: &mut impl io::Read) -> Result<String> {
    let id = read_string(t)?;
    if id.len() > 32767 {
        return Err(anyhow::Error::msg(
            "identifier exceeds maximum length of 32767",
        ));
    }
    Ok(id)
}

#[inline]
pub fn write_uuid(t: &mut impl io::Write, v: Uuid) -> Result<()> {
    t.write_all(v.as_bytes()).map_err(From::from)
}

#[inline]
pub fn read_uuid(t: &mut impl io::Read) -> Result<Uuid> {
    let mut bytes = [0; 16];
    t.read_exact(&mut bytes)?;
    Ok(Uuid::from_bytes(bytes))
}

#[inline]
pub fn write_byte_array(t: &mut impl io::Write, v: &Vec<u8>) -> Result<()> {
    write_var_int(t, v.len() as i32)?;
    t.write_all(v.as_slice()).map_err(From::from)
}

#[inline]
pub fn read_byte_array(t: &mut impl io::Read) -> Result<Vec<u8>> {
    let len = read_var_int(t)? as usize;
    let mut buf = vec![0; len];
    t.read_exact(&mut buf[..])?;
    Ok(buf)
}

#[inline]
pub fn write_var_int(t: &mut impl io::Write, v: i32) -> Result<()> {
    let mut x = v as u32;
    loop {
        let mut temp = (x & 0b0111_1111) as u8;
        x >>= 7;
        if x != 0 {
            temp |= 0b1000_0000;
        }

        t.write_all(&[temp])?;

        if x == 0 {
            break;
        }
    }
    Ok(())
}

#[inline]
pub fn read_var_int(t: &mut impl io::Read) -> Result<i32> {
    let mut num_read = 0;
    let mut result = 0;

    loop {
        let read = t.read_u8()?;
        let value = i32::from(read & 0b0111_1111);
        result |= value.overflowing_shl(7 * num_read).0;

        num_read += 1;

        if num_read > 5 {
            bail!("Varint is too large");
        }
        if read & 0b1000_0000 == 0 {
            break;
        }
    }

    Ok(result)
}

#[inline]
pub fn write_var_long(t: &mut impl io::Write, v: i64) -> Result<()> {
    let mut x = v as u64;
    loop {
        let mut temp = (x & 0b0111_1111) as u8;
        x >>= 7;
        if x != 0 {
            temp |= 0b1000_0000;
        }

        t.write_u8(temp).unwrap();

        if x == 0 {
            break;
        }
    }

    Ok(())
}

#[inline]
pub fn read_var_long(t: &mut impl io::Read) -> Result<i64> {
    let mut num_read = 0;
    let mut result = 0;

    loop {
        let read = read_unsigned_byte(t)?;
        let value = i64::from(read & 0b0111_1111);
        result |= value.overflowing_shl(7 * num_read).0;

        num_read += 1;

        if num_read > 10 {
            bail!("VarLong is too large.");
        }
        if read & 0b1000_0000 == 0 {
            break;
        }
    }
    Ok(result)
}

#[inline]
pub fn write_bitset(t: &mut impl io::Write, v: &Vec<i64>) -> Result<()> {
    write_var_int(t, v.len() as i32)?;
    for i in v {
        write_long(t, *i)?
    }
    Ok(())
}

#[inline]
pub fn read_bitset(t: &mut impl io::Read) -> Result<Vec<i64>> {
    let mut data: Vec<i64> = Vec::new();
    let len = read_var_int(t)? as usize;
    for i in 0..len {
        data.insert(i, read_long(t)?);
    }
    Ok(data)
}

#[inline]
pub fn write_position(t: &mut impl io::Write, x: i32, y: i32, z: i32) -> Result<()> {
    t.write_u64::<BE>(
        (x as u64 & 0x3FFFFFF) << 38 | (z as u64 & 0x3FFFFFF) << 12 | (y as u64 & 0xFFF),
    )
    .map_err(From::from)
}

#[inline]
pub fn read_position(t: &mut impl io::Read) -> Result<(i32, i32, i32)> {
    let v = read_long(t)?;

    let mut x = (v >> 38) as i32;
    let mut y = (v & 0xFFF) as i32;
    let mut z = (v << 26 >> 38) as i32;

    if x >= 1 << 25 {
        x -= 1 << 26;
    }
    if y >= 1 << 11 {
        y -= 1 << 12;
    }
    if z >= 1 << 25 {
        z -= 1 << 26;
    }
    return Ok((x, y, z));
}

#[inline]
pub fn write_nbt(t: &mut impl io::Write, v: &nbt::Blob) -> Result<()> {
    v.to_writer(t).map_err(From::from)
}

#[inline]
pub fn read_nbt(t: &mut impl io::Read) -> Result<nbt::Blob> {
    nbt::Blob::from_reader(t).map_err(From::from)
}
