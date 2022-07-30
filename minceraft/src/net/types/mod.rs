pub mod raw;
pub use crate::inv::{
    enchant::Enchant,
    item::{Item, Itemstack},
    Slot,
};
use anyhow::{Result, bail};
pub use nbt::Blob as Nbt;
use std::{
    borrow::Cow,
    convert::{TryFrom, TryInto},
    io,
    num::TryFromIntError, marker::PhantomData,
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

/// An Array of type T which is prefixed by type U
// TODO: Optimise
// The following code was heavily inspired by 
// https://github.com/feather-rs/feather/blob/2f99d76aaad022e65550c88594b7b9b259503c16/feather/protocol/src/io.rs 
// which is under the apache 2.0 license
// ----------------------------------------------------------------------------
pub struct Array<'a, T, U> (pub Cow<'a, [T]>, PhantomData<U>)
where
    [T]: ToOwned<Owned = Vec<T>>;

impl<'a, T, U> Array<'a, T, U> 
where
    [T]: ToOwned<Owned = Vec<T>>
{
    const MAX_LENGTH: usize = 2^20;
}


impl<'a, T, U> Encoder for Array<'a, T, U> 
where 
    T: Encoder,
    U: Encoder + TryFrom<usize>,
    U::Error: std::error::Error + Send + Sync + 'static,
    [T]: ToOwned<Owned = Vec<T>>
{
    fn write_to(&self, w: &mut impl io::Write) -> Result<()> {
        let len = U::try_from(self.0.len())?;
        len.write_to(w)?;
        for i in self.0.iter() {
            i.write_to(w)?;
        }
        Ok(())
    }
}

impl<'a, T, U> Decoder for Array<'a, T, U> 
where 
    T: Decoder,
    U: Decoder + TryInto<usize>,
    U::Error: std::error::Error + Send + Sync + 'static,
    [T]: ToOwned<Owned = Vec<T>>
{
    fn read_from(r: &mut impl io::Read) -> Result<Self> {
        let len: usize = U::read_from(r)?.try_into()?;
        if len > Self::MAX_LENGTH {
            bail!("array length too large! {} > 2^20", len)
        }

        let mut vec = Vec::<T>::new();
        for _ in 0..=len {
            vec.push(T::read_from(r)?);
        }
        Ok(Self(Cow::Owned(vec), PhantomData))
    }
}

impl<'a, T, U> From::<Array::<'a, T, U>> for Vec<T> 
where
    [T]: ToOwned<Owned = Vec<T>>
{
    fn from(arr: Array::<'a, T, U>) -> Self {
        arr.0.into_owned()
    }
}

impl<'a, T, U> From::<Vec::<T>> for Array<'a, T, U> 
where 
    [T]: ToOwned<Owned = Vec<T>>
{
    fn from(vec: Vec::<T>) -> Self {
        Self(Cow::Owned(vec), PhantomData)
    }
}

impl<'a, T, U> From<&'a [T]> for Array<'a, T, U>
where
    [T]: ToOwned<Owned = Vec<T>>,
{
    fn from(slice: &'a [T]) -> Self {
        Self(Cow::Borrowed(slice), PhantomData)
    }
}

pub type VarIntPrefixedArray<'a, T> = Array<'a, T, VarInt>;
pub type ShortPrefixedArray<'a, T> = Array<'a, T, u8>;
pub type ByteArray<'a, U> = Array<'a, u8, U>;
pub type BitSet<'a, U> = Array<'a, u8, U>;


pub struct LengthInferredByteArray<'a>(pub Cow<'a, [u8]>);

impl<'a> Encoder for LengthInferredByteArray<'a> {
    fn write_to(&self, w: &mut impl io::Write) -> Result<()> {
        w.write_all(&*self.0).map_err(From::from)
    }
}

impl<'a> Decoder for LengthInferredByteArray<'a> {
    fn read_from(r: &mut impl io::Read) -> Result<Self> {
        let mut vec = Vec::new();
        r.read_to_end(&mut vec)?;
        Ok(LengthInferredByteArray(Cow::Owned(vec)))
    }
}

impl<'a> From<&'a [u8]> for LengthInferredByteArray<'a> {
    fn from(slice: &'a [u8]) -> Self {
        LengthInferredByteArray(Cow::Borrowed(slice))
    }
}

impl<'a> From<LengthInferredByteArray<'a>> for Vec<u8> {
    fn from(vec: LengthInferredByteArray<'a>) -> Self {
        vec.0.into_owned()
    }
}

// ----------------------------------------------------------------------------

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

