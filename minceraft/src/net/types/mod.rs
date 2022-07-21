pub mod raw;
pub use crate::inv::{
    enchant::Enchant,
    item::{Item, Itemstack},
    Slot,
};
use anyhow::Result;
pub use nbt::Blob as Nbt;
use std::{
    convert::{TryFrom, TryInto},
    io,
    num::TryFromIntError,
};
pub use uuid::Uuid;

pub trait Encoder: Sized {
    fn write_to(&self, w: &mut impl io::Write) -> Result<()>;
}

impl<'a, T> Encoder for &'a T
where
    T: Encoder,
{
    fn write_to(&self, w: &mut impl io::Write) -> Result<()> {
        T::write_to(*self, w)
    }
}
pub trait Decoder: Sized {
    fn read_from(r: &mut impl io::Read) -> Result<Self>;
}

impl Encoder for u8 {
    fn write_to(&self, w: &mut impl io::Write) -> Result<()> {
        raw::write_unsigned_byte(w, *self)
    }
}

impl Decoder for u8 {
    fn read_from(r: &mut impl io::Read) -> Result<Self> {
        raw::read_unsigned_byte(r)
    }
}

impl Encoder for i8 {
    fn write_to(&self, w: &mut impl io::Write) -> Result<()> {
        raw::write_byte(w, *self)
    }
}

impl Decoder for i8 {
    fn read_from(r: &mut impl io::Read) -> Result<Self> {
        raw::read_byte(r)
    }
}

impl Encoder for u16 {
    fn write_to(&self, w: &mut impl io::Write) -> Result<()> {
        raw::write_unsigned_short(w, *self)
    }
}

impl Decoder for u16 {
    fn read_from(r: &mut impl io::Read) -> Result<Self> {
        raw::read_unsigned_short(r)
    }
}

impl Encoder for i16 {
    fn write_to(&self, w: &mut impl io::Write) -> Result<()> {
        raw::write_short(w, *self)
    }
}

impl Decoder for i16 {
    fn read_from(r: &mut impl io::Read) -> Result<Self> {
        raw::read_short(r)
    }
}

impl Encoder for i32 {
    fn write_to(&self, w: &mut impl io::Write) -> Result<()> {
        raw::write_int(w, *self)
    }
}

impl Decoder for i32 {
    fn read_from(r: &mut impl io::Read) -> Result<Self> {
        raw::read_int(r)
    }
}

impl Encoder for i64 {
    fn write_to(&self, w: &mut impl io::Write) -> Result<()> {
        raw::write_long(w, *self)
    }
}

impl Decoder for i64 {
    fn read_from(r: &mut impl io::Read) -> Result<Self> {
        raw::read_long(r)
    }
}

impl Encoder for f32 {
    fn write_to(&self, w: &mut impl io::Write) -> Result<()> {
        raw::write_float(w, *self)
    }
}

impl Decoder for f32 {
    fn read_from(r: &mut impl io::Read) -> Result<Self> {
        raw::read_float(r)
    }
}

impl Encoder for f64 {
    fn write_to(&self, w: &mut impl io::Write) -> Result<()> {
        raw::write_double(w, *self)
    }
}

impl Decoder for f64 {
    fn read_from(r: &mut impl io::Read) -> Result<Self> {
        raw::read_double(r)
    }
}

impl Encoder for bool {
    fn write_to(&self, w: &mut impl io::Write) -> Result<()> {
        raw::write_bool(w, *self)
    }
}

impl Decoder for bool {
    fn read_from(r: &mut impl io::Read) -> Result<Self> {
        raw::read_bool(r)
    }
}

impl<T> Encoder for Option<T>
where
    T: Encoder,
{
    fn write_to(&self, w: &mut impl io::Write) -> Result<()> {
        self.is_some().write_to(w)?;

        if let Some(v) = self {
            v.write_to(w)?;
        }

        Ok(())
    }
}

impl<T> Decoder for Option<T>
where
    T: Decoder,
{
    fn read_from(r: &mut impl io::Read) -> Result<Self> {
        let present = bool::read_from(r)?;

        match present {
            true => Ok(Some(T::read_from(r)?)),
            false => Ok(None),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct VarInt(pub i32);

impl Encoder for VarInt {
    fn write_to(&self, w: &mut impl io::Write) -> Result<()> {
        raw::write_var_int(w, self.0)
    }
}

impl Decoder for VarInt {
    fn read_from(r: &mut impl io::Read) -> Result<Self> {
        Ok(Self(raw::read_var_int(r)?))
    }
}

impl TryFrom<VarInt> for usize {
    type Error = TryFromIntError;
    fn try_from(value: VarInt) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl From<usize> for VarInt {
    fn from(v: usize) -> Self {
        VarInt(v as i32)
    }
}

impl From<VarInt> for i32 {
    fn from(v: VarInt) -> Self {
        v.0
    }
}

impl From<i32> for VarInt {
    fn from(v: i32) -> Self {
        VarInt(v)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct VarLong(pub i64);

impl Decoder for VarLong {
    fn read_from(r: &mut impl io::Read) -> Result<Self> {
        Ok(Self(raw::read_var_long(r)?))
    }
}

impl Encoder for VarLong {
    fn write_to(&self, w: &mut impl io::Write) -> Result<()> {
        raw::write_var_long(w, self.0)
    }
}

impl From<VarLong> for i64 {
    fn from(v: VarLong) -> Self {
        v.0
    }
}

impl From<i64> for VarLong {
    fn from(v: i64) -> Self {
        VarLong(v)
    }
}

impl Encoder for String {
    fn write_to(&self, w: &mut impl io::Write) -> Result<()> {
        raw::write_string(w, self)
    }
}

impl Decoder for String {
    fn read_from(r: &mut impl io::Read) -> Result<Self> {
        raw::read_string(r)
    }
}

impl Encoder for Nbt {
    fn write_to(&self, w: &mut impl io::Write) -> Result<()> {
        raw::write_nbt(w, self)
    }
}

impl Decoder for Nbt {
    fn read_from(r: &mut impl io::Read) -> Result<Self> {
        raw::read_nbt(r)
    }
}

impl Encoder for Uuid {
    fn write_to(&self, w: &mut impl io::Write) -> Result<()> {
        raw::write_uuid(w, *self)
    }
}

impl Decoder for Uuid {
    fn read_from(r: &mut impl io::Read) -> Result<Self> {
        raw::read_uuid(r)
    }
}

pub struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Encoder for Position {
    fn write_to(&self, w: &mut impl io::Write) -> Result<()> {
        raw::write_position(w, self.x, self.y, self.z)
    }
}

impl Decoder for Position {
    fn read_from(r: &mut impl io::Read) -> Result<Self> {
        let (x, y, z) = raw::read_position(r)?;
        Ok(Self { x, y, z })
    }
}

pub struct BitSet(pub Vec<i64>);

impl Encoder for BitSet {
    fn write_to(&self, w: &mut impl io::Write) -> Result<()> {
        raw::write_bitset(w, &self.0)
    }
}

impl Decoder for BitSet {
    fn read_from(r: &mut impl io::Read) -> Result<Self> {
        Ok(Self(raw::read_bitset(r)?))
    }
}

pub struct ByteArray(pub Vec<u8>);

impl Encoder for ByteArray {
    fn write_to(&self, w: &mut impl io::Write) -> Result<()> {
        raw::write_byte_array(w, &self.0)
    }
}

impl Decoder for ByteArray {
    fn read_from(r: &mut impl io::Read) -> Result<Self> {
        Ok(Self(raw::read_byte_array(r)?))
    }
}

pub struct Array<T: Encoder + Decoder> (Vec<T>);


pub type Angle = u8;

impl<T: Item, U: Enchant> Encoder for Slot<T, U> {
    fn write_to(&self, w: &mut impl io::Write) -> Result<()> {
        match self {
            Self::Empty => false.write_to(w),
            Self::Filled(i) => {
                true.write_to(w)?;
                VarInt(i.item.id().into()).write_to(w)?;

                let count: i8 = i.count.into();
                count.write_to(w)?;

                // TODO: Handle NBT properly
                0_u8.write_to(w)?;

                Ok(())
            }
        }
    }
}

impl<T: Item, U: Enchant> Decoder for Slot<T, U> {
    fn read_from(r: &mut impl io::Read) -> Result<Self> {
        let present: bool = bool::read_from(r)?;
        Ok(match present {
            false => Self::Empty,
            true => {
                Self::Filled(Itemstack {
                    item: {
                        let id = VarInt::read_from(r)?.0 as u16;
                        T::from_id(id)?
                    },
                    count: i8::read_from(r)?,
                    meta: {
                        nbt::from_reader(r)?;
                        // TODO: Handle NBT properly
                        None
                    },
                })
            }
        })
    }
}

