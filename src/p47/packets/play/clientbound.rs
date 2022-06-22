use crate::{def_enum, packets};
use super::super::super::inv::Slot;

def_enum! {
    Dimension(i8) {
        -1 = Nether,
        0 = Overworld,
        1 = End
    }
}

def_enum! {
    ChatPosition(i8) {
        0 = ChatBox,
        1 = SystemMessage,
        2 = Hotbar,
    }
}

def_enum! {
    EquipmentSlot(i16) {
        0i16 = Held,
        1i16 = Boots,
        2i16 = Leggings,
        3i16 = Chestplate,
        4i16 = Helmet,
    }
}

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
}
