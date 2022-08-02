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

def_enum! {
    WindDirection(u8) {
        0 = North,
        1 = West,
        2 = South,
        3 = East,
    }
}

def_enum! {
    // TODO: Figure out what's up with the duplicate 10
    EntityStatus(i8) {
        1 = ResetMobSpawnMinecartTimerOrRabbitJumpAnimation, //TODO: Come up with a better name for this mess
        2 = LivingEntityHurt,
        3 = LivingEntityDead,
        4 = IronGolemThrowingUpArms,
        // where is 5?
        6 = TamingSpawnHeartParticles,
        7 = TamingSpawnSmokeParticles,
        8 = WolfShakingWater,
        9 = PlayerEatingAcceptedByServer,
        10 = SheepEatingGrassOrPlayTntIgniteSound, // since there was a duplicate 10 I combined these
        // 10 = PlayTntIgniteSound, // duplicate 10?
        11 = IronGolemHandingOverRose,
        12 = VillagerMatingSpawnHeartParticles,
        13 = SpawnAngryVillagerParticles,
        14 = SpawnHappyVillagerParticles,
        15 = WitchSpawnMagicParticles,
        16 = PlayZombieConvertingIntoVillagerSound,
        17 = FireWorkExploding,
        18 = AnimalInLoveSpawnHeartParticles,
        19 = ResetSquidRotation,
        20 = SpawnExplosionParticle,
        21 = PlayGuardianSound,
        22 = EnableReducedDebugInfo,
        23 = DisableReducedDebugInfo,
    }
}