use crate::def_enum;

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

def_enum! {
    Difficulty(u8) {
        0u8 = Peaceful,
        1u8 = Easy,
        2u8 = Normal,
        3u8 = Hard,
    }
}

def_enum! {
    HandshakeState(VarInt) {
        1 = Status,
        2 = Login,
    }
}

def_enum! {
    Gamemode(u8) {
        0 = Survival,
        1 = Creative,
        2 = Adventure,
    }
}

def_enum! {
    AnimationId(u8) {
        0 = SwingArm,
        1 = TakeDamage,
        2 = LeaveBed,
        3 = EatFood,
        4 = CriticalEffect,
        5 = MagicalCriticalEffect,
    }
}
