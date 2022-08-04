use crate::p47::enums::*;
use crate::p47::inv::Slot;
use crate::p47::metadata::EntityMetaData;
use crate::packets;

packets! {
    KeepAlive(0x00) {
        id VarInt
    },
    JoinGame(0x01) {
        eid i32;
        gamemode u8;
        dimension Dimension;
        difficulty u8;
        max_players u8;
        level_type String;
        reduced_debug_info bool;
    },
    ChatMessage(0x02) {
        json String; // to be replaced with a Chat type
        position ChatPosition;
    },
    TimeUpdate(0x03) {
        world_age i64;
        time_of_day i64;
    }
    EntityEquipment(0x04) {
        eid VarInt;
        slot EquipmentSlot;
        item Slot;
    }
    SpawnPosition(0x05) {
        position Position;
    }
    UpdateHealth(0x06) {
        health f32; // <= 0 = dead, 20 = full HP
        food VarInt; // 0-20
        food_saturation f32; // 0.0-5.0 in integer increments
    }
    Respawn(0x07) {
        dimension Dimension;
        difficulty Difficulty;
        gamemode Gamemode;
        level_type String;
    }
    PlayerPositionAndLook(0x08) {
        x f64;
        y f64;
        z f64;
        yaw f32;
        pitch f32;
        flags i8; // Bit field
    }
    HeldItemChange(0x09) {
        slot i8; // 0-8
    }
    UseBed(0x0A) {
        eid VarInt;
        location Position;
    }
    Animation(0x0B) {
        eid VarInt;
        animation AnimationId;
    }
    SpawnPlayer(0x0C) {
        eid VarInt;
        player_uuid Uuid;
        x i32;
        y i32;
        z i32;
        yaw Angle;
        pitch Angle;
        current_item i16; // 0 for no item instead of -1
        metadata EntityMetaData;
    }
    CollectItem(0x0D) {
        collected_eid VarInt;
        collector_eid VarInt;
    }
    SpawnObject(0x0E) {
        eid VarInt;
        ty u8;
        x i32;
        y i32;
        z i32;
        pitch Angle;
        yaw Angle;
        inner SpawnObjectInner;
    }
    SpawnMob(0x0F) {
        eid VarInt;
        ty u8;
        x i32;
        y i32;
        z i32;
        yaw Angle;
        pitch Angle;
        head_pitch Angle;
        velocity_x i16;
        velocity_y i16;
        velocity_z i16;
        metadata EntityMetaData;
    }
    SpawnPainting(0x10) {
        eid VarInt;
        title String;
        location Position;
        direction WindDirection;
    }
    SpawnExperienceOrb(0x11) {
        eid VarInt;
        x i32;
        y i32;
        z i32;
        count i16;
    }
    EntityVelocity(0x11) {
        eid VarInt;
        velocity_x i16;
        velocity_y i16;
        velocity_z i16;
    }
    DestroyEntities(0x13) {
        eids VarIntPrefixedArray<u8>
    }
    Entity(0x14) {
        eid VarInt;
    }
    EntityRelativeMove(0x15) {
        eid VarInt;
        delta_x i8;
        delta_y i8;
        delta_z i8;
        on_ground bool;
    }
    EntityLook(0x16) {
        eid VarInt;
        yaw Angle;
        pitch Angle;
        on_ground bool;
    }
    EntityLookAndRelativeMove(0x17) {
        eid VarInt;
        delta_x i8;
        delta_y i8;
        delta_z i8;
        yaw Angle;
        pitch Angle;
        on_ground bool;
    }
    EntityTeleport(0x18) {
        eid VarInt;
        x i32;
        y i32;
        z i32;
        yaw Angle;
        pitch Angle;
        on_ground bool;
    }
    EntityHeadLook(0x19) {
        eid VarInt;
        head_yaw Angle;
    }
    UpdateEntityStatus(0x1A) {
        eid i32;
        status EntityStatus;
    }
    AttachEntity(0x1B) {
        eid i32;
        vehicle_eid i32; // set to -1 to detach
        leash bool; // if true leashes the entity to the vehicle
    }
    UpdataEntityMetaData(0x1C) {
        eid VarInt;
        metadata EntityMetaData;
    }
    EntityEffect(0x1D) {
        eid VarInt;
        effect i8; // TODO: add a propper effect type (can be generated with https://github.com/PrismarineJS/minecraft-data/blob/master/data/pc/1.8/effects.json)
        amplifier i8; // Notchian client displas effect level as amplifier + 1 + shouldn't this be u8???
        duration VarInt; // Seconds
        hide_particles bool;
    }
    RemoveEntityEffect(0x1E) {
        eid VarInt;
        effect i8;
    }
    SetExperience(0x1F) {
        experience_bar f32; // between 0 and 1
        level VarInt;
        total_experience VarInt;
    }
    //TODO: finish entity properties packet, this includes making a proper attribute type which can be generated with https://github.com/PrismarineJS/minecraft-data/blob/master/data/pc/1.8/attributes.json
    EntityProperties(20) {
        eid VarInt;
        //...
    }
}

#[derive(Debug, Clone)]
pub struct SpawnObjectInner {
    pub data: i32,
    pub velocity: Option<SpawnObjectVelocity>,
}

impl Encoder for SpawnObjectInner {
    fn write_to(&self, w: &mut impl std::io::Write) -> anyhow::Result<()> {
        self.data.write_to(w)?;
        if self.data != 0 {
            self.velocity.as_ref().unwrap().write_to(w)?;
        }
        Ok(())
    }
}

impl Decoder for SpawnObjectInner {
    fn read_from(r: &mut impl std::io::Read) -> anyhow::Result<Self> {
        let data = i32::read_from(r)?;
        let velocity = if data != 0 {
            Some(SpawnObjectVelocity {
                x: i16::read_from(r)?,
                y: i16::read_from(r)?,
                z: i16::read_from(r)?,
            })
        } else {
            None
        };
        Ok(Self { data, velocity })
    }
}

// TODO: Add a encoder and decoder derive and use that instead
#[derive(Debug, Clone)]
pub struct SpawnObjectVelocity {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl Encoder for SpawnObjectVelocity {
    fn write_to(&self, w: &mut impl std::io::Write) -> anyhow::Result<()> {
        self.x.write_to(w)?;
        self.y.write_to(w)?;
        self.z.write_to(w)
    }
}

impl Decoder for SpawnObjectVelocity {
    fn read_from(r: &mut impl std::io::Read) -> anyhow::Result<Self> {
        Ok(Self {
            x: i16::read_from(r)?,
            y: i16::read_from(r)?,
            z: i16::read_from(r)?,
        })
    }
}
