use crate::p47::enums::*;
use crate::p47::inv::Slot;
use crate::p47::metadata::EntityMetaData;
use crate::packets;

packets! {
    KeepAlive(0x00) {
        id VarInt
    },
    JoinGame(0x01) {
        entity_id i32;
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
        entity_id VarInt;
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
        entity_id VarInt;
        location Position;
    }
    Animation(0x0B) {
        entity_id VarInt;
        animation AnimationId;
    }
    SpawnPlayer(0x0C) {
        entity_id VarInt;
        player_uuid Uuid;
        x i32;
        y i32;
        z i32;
        yaw Angle;
        pitch Angle;
        current_item i16; // 0 for no item instead of -1
        metadata EntityMetaData;
    }
}
