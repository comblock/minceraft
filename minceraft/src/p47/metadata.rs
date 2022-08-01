use anyhow::bail;
use std::collections::BTreeMap;

use crate::net::types::{Decoder, Encoder, Position};

use super::inv::Slot;

impl Into<EntityMetaDataEntry> for u8 {
    fn into(self) -> EntityMetaDataEntry {
        EntityMetaDataEntry::Byte(self)
    }
}

impl Into<EntityMetaDataEntry> for i16 {
    fn into(self) -> EntityMetaDataEntry {
        EntityMetaDataEntry::Short(self)
    }
}

impl Into<EntityMetaDataEntry> for i32 {
    fn into(self) -> EntityMetaDataEntry {
        EntityMetaDataEntry::Int(self)
    }
}

impl Into<EntityMetaDataEntry> for f32 {
    fn into(self) -> EntityMetaDataEntry {
        EntityMetaDataEntry::Float(self)
    }
}

impl Into<EntityMetaDataEntry> for String {
    fn into(self) -> EntityMetaDataEntry {
        EntityMetaDataEntry::String(self)
    }
}

impl Into<EntityMetaDataEntry> for Slot {
    fn into(self) -> EntityMetaDataEntry {
        EntityMetaDataEntry::Slot(self)
    }
}

impl Into<EntityMetaDataEntry> for Position {
    fn into(self) -> EntityMetaDataEntry {
        EntityMetaDataEntry::Position {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl Into<EntityMetaDataEntry> for [f32; 3] {
    fn into(self) -> EntityMetaDataEntry {
        EntityMetaDataEntry::Look {
            pitch: self[0],
            yaw: self[1],
            roll: self[2],
        }
    }
}

#[derive(Debug, Clone)]
pub enum EntityMetaDataEntry {
    Byte(u8),
    Short(i16),
    Int(i32),
    Float(f32),
    String(String),
    Slot(super::inv::Slot),
    Position { x: i32, y: i32, z: i32 }, // unused
    Look { pitch: f32, yaw: f32, roll: f32 },
}

impl EntityMetaDataEntry {
    pub fn ty(&self) -> u8 {
        match self {
            Self::Byte(_) => 0,
            Self::Short(_) => 1,
            Self::Int(_) => 2,
            Self::Float(_) => 3,
            Self::String(_) => 4,
            Self::Slot(_) => 5,
            Self::Position { x: _, y: _, z: _ } => 6,
            Self::Look {
                pitch: _,
                yaw: _,
                roll: _,
            } => 7,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EntityMetaData(pub BTreeMap<u8, EntityMetaDataEntry>);

impl EntityMetaData {
    pub fn new() -> Self {
        Self(BTreeMap::<u8, EntityMetaDataEntry>::new())
    }

    pub fn set<T: Into<EntityMetaDataEntry>>(&mut self, i: u8, value: T) -> anyhow::Result<()> {
        if i | 0xE0 > 0x00 {
            bail!("the top 3 bits of the index should be 0")
        }
        let value: EntityMetaDataEntry = value.into();
        self.0.insert(i, value);
        Ok(())
    }

    // The following code was taken from
    // https://github.com/feather-rs/feather/blob/2f99d76aaad022e65550c88594b7b9b259503c16/feather/base/src/metadata.rs
    // which is under the apache 2.0 license.
    // It was modified slightly.

    pub fn _with<T: Into<EntityMetaDataEntry>>(mut self, i: u8, value: T) -> anyhow::Result<Self> {
        self.set(i, value)?;
        Ok(self)
    }

    pub fn _with_many(mut self, values: &[(u8, EntityMetaDataEntry)]) -> Self {
        for val in values {
            self.0.insert(val.0, val.1.clone());
        }

        self
    }

    pub fn _get(&self, i: u8) -> Option<&EntityMetaDataEntry> {
        self.0.get(&i)
    }

    pub fn iter(&self) -> impl Iterator<Item = (u8, &EntityMetaDataEntry)> {
        self.0.iter().map(|(key, entry)| (*key, entry))
    }
}

impl Default for EntityMetaData {
    fn default() -> Self {
        Self::new()
    }
}

impl Encoder for EntityMetaData {
    fn write_to(&self, w: &mut impl std::io::Write) -> anyhow::Result<()> {
        for (i, v) in self.iter() {
            ((v.ty() << 5 | i/*& 0x1F*/) & 0xFF/* does & 0xFF even do anything? */).write_to(w)?;
            match v {
                EntityMetaDataEntry::Byte(v) => v.write_to(w),
                EntityMetaDataEntry::Short(v) => v.write_to(w),
                EntityMetaDataEntry::Int(v) => v.write_to(w),
                EntityMetaDataEntry::Float(v) => v.write_to(w),
                EntityMetaDataEntry::String(v) => v.write_to(w),
                EntityMetaDataEntry::Slot(v) => v.write_to(w),
                EntityMetaDataEntry::Position { x, y, z } => {
                    x.write_to(w)?;
                    y.write_to(w)?;
                    z.write_to(w)?;
                    Ok(())
                }
                EntityMetaDataEntry::Look { pitch, yaw, roll } => {
                    pitch.write_to(w)?;
                    yaw.write_to(w)?;
                    roll.write_to(w)?;
                    Ok(())
                }
            }?;
        }
        0x7F.write_to(w)
    }
}

impl Decoder for EntityMetaData {
    fn read_from(r: &mut impl std::io::Read) -> anyhow::Result<Self> {
        let mut data = Self::new();
        loop {
            let byte = u8::read_from(r)?;
            if byte == 0x7F {
                break;
            };
            let i = byte & 0x1F;
            let ty = byte >> 5;
            match ty {
                0 => data.set(i, u8::read_from(r)?),
                1 => data.set(i, i16::read_from(r)?),
                2 => data.set(i, i32::read_from(r)?),
                3 => data.set(i, f32::read_from(r)?),
                4 => data.set(i, String::read_from(r)?),
                5 => data.set(i, Slot::read_from(r)?),
                6 => data.set(
                    i,
                    Position {
                        x: i32::read_from(r)?,
                        y: i32::read_from(r)?,
                        z: i32::read_from(r)?,
                    },
                ),
                7 => {
                    let pitch = f32::read_from(r)?;
                    let yaw = f32::read_from(r)?;
                    let roll = f32::read_from(r)?;
                    data.set(i, [pitch, yaw, roll])
                }
                _ => {
                    bail!("Invalid type id!")
                }
            }?
        }
        todo!()
    }
}
