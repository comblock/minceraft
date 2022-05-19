pub mod raw;
use anyhow::{bail, Result};
pub use nbt::Blob as Nbt;
use std::{
    borrow::Cow,
    convert::{TryFrom, TryInto},
    io, iter,
    marker::PhantomData,
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

impl Encoder for Vec<i64> {
    fn write_to(&self, w: &mut impl io::Write) -> Result<()> {
        raw::write_bitset(w, self)
    }
}

impl Decoder for Vec<i64> {
    fn read_from(r: &mut impl io::Read) -> Result<Self> {
        raw::read_bitset(r)
    }
}

impl Encoder for Vec<u8> {
    fn write_to(&self, w: &mut impl io::Write) -> Result<()> {
        raw::write_byte_array(w, self)
    }
}

pub struct Angle(u8);

impl Encoder for Angle {
    fn write_to(&self, w: &mut impl io::Write) -> Result<()> {
        raw::write_unsigned_byte(w, self.0)
    }
}

impl Decoder for Angle {
    fn read_from(r: &mut impl io::Read) -> Result<Self> {
        Ok(Self(raw::read_unsigned_byte(r)?))
    }
}

// The following code was taken from https://github.com/feather-rs/feather/blob/main/feather/protocol/src/io.rs which is licensed under Apache 2.0.
// It was modified slightly to fit the traits of this module.
// ------------------------------------------------------------------------------------------------------------------------------------------------

pub const MAX_LENGTH: usize = 1024 * 1024; // 2^20 elements

/// Reads and writes an array of inner `Writeable`s.
/// The array is prefixed with a `VarInt` length.
///
/// This will reject arrays of lengths larger than MAX_LENGTH.
pub struct LengthPrefixedVec<'a, P, T>(pub Cow<'a, [T]>, PhantomData<P>)
where
    [T]: ToOwned<Owned = Vec<T>>;

impl<'a, P, T> Decoder for LengthPrefixedVec<'a, P, T>
where
    T: Decoder,
    [T]: ToOwned<Owned = Vec<T>>,
    P: TryInto<usize> + Decoder,
    P::Error: std::error::Error + Send + Sync + 'static,
{
    fn read_from(r: &mut impl io::Read) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let length: usize = P::read_from(r)?.try_into()?;

        if length > MAX_LENGTH {
            bail!("array length too large ({} > {})", length, MAX_LENGTH);
        }

        let vec = iter::repeat_with(|| T::read_from(r))
            .take(length)
            .collect::<anyhow::Result<Vec<T>>>()?;
        Ok(Self(Cow::Owned(vec), PhantomData))
    }
}

impl<'a, P, T> Encoder for LengthPrefixedVec<'a, P, T>
where
    T: Encoder,
    [T]: ToOwned<Owned = Vec<T>>,
    P: TryFrom<usize> + Encoder,
    P::Error: std::error::Error + Send + Sync + 'static,
{
    fn write_to(&self, w: &mut impl io::Write) -> anyhow::Result<()> {
        P::try_from(self.0.len())?.write_to(w)?;
        self.0
            .iter()
            .for_each(|item| item.write_to(w).expect("failed to write to vec"));

        Ok(())
    }
}

impl<'a, P, T> From<LengthPrefixedVec<'a, P, T>> for Vec<T>
where
    [T]: ToOwned<Owned = Vec<T>>,
{
    fn from(x: LengthPrefixedVec<'a, P, T>) -> Self {
        x.0.into_owned()
    }
}

impl<'a, P, T> From<&'a [T]> for LengthPrefixedVec<'a, P, T>
where
    [T]: ToOwned<Owned = Vec<T>>,
{
    fn from(slice: &'a [T]) -> Self {
        Self(Cow::Borrowed(slice), PhantomData)
    }
}

impl<'a, P, T> From<Vec<T>> for LengthPrefixedVec<'a, P, T>
where
    [T]: ToOwned<Owned = Vec<T>>,
{
    fn from(vec: Vec<T>) -> Self {
        Self(Cow::Owned(vec), PhantomData)
    }
}

pub type VarIntPrefixedVec<'a, T> = LengthPrefixedVec<'a, VarInt, T>;
pub type ShortPrefixedVec<'a, T> = LengthPrefixedVec<'a, u16, T>;

/// A vector of bytes which consumes all remaining bytes in this packet.
/// This is used by the plugin messaging packets, for one.
pub struct LengthInferredVecU8<'a>(pub Cow<'a, [u8]>);

impl<'a> Decoder for LengthInferredVecU8<'a> {
    fn read_from(buffer: &mut impl io::Read) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let mut vec = Vec::new();
        buffer.read_to_end(&mut vec)?;
        Ok(LengthInferredVecU8(Cow::Owned(vec)))
    }
}

impl<'a> Encoder for LengthInferredVecU8<'a> {
    fn write_to(&self, w: &mut impl io::Write) -> anyhow::Result<()> {
        w.write_all(&self.0)?;
        Ok(())
    }
}

impl<'a> From<&'a [u8]> for LengthInferredVecU8<'a> {
    fn from(slice: &'a [u8]) -> Self {
        LengthInferredVecU8(Cow::Borrowed(slice))
    }
}

impl<'a> From<LengthInferredVecU8<'a>> for Vec<u8> {
    fn from(x: LengthInferredVecU8<'a>) -> Self {
        x.0.into_owned()
    }
}

// ------------------------------------------------------------------------------------------------------------------------------------------------

// The following code was taken from https://github.com/feather-rs/feather/blob/main/feather/protocol/src/packets.rs

// ------------------------------------------------------------------------------------------------------------------------------------------------

/// Trait implemented for packets which can be converted from a packet
/// enum. For example, `SpawnEntity` implements `VariantOf<ServerPlayPacket>`.
pub trait VariantOf<Enum> {
    /// Returns the unique ID used to determine whether
    /// an enum variant matches this variant.
    fn discriminant_id() -> u32;

    /// Attempts to destructure the `Enum` into this type.
    /// Returns `None` if `enum` is not the correct variant.
    fn destructure(e: Enum) -> Option<Self>
    where
        Self: Sized;
}

// ------------------------------------------------------------------------------------------------------------------------------------------------
