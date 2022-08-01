use crate::p47::enums::HandshakeState;
use crate::packets;

packets!(
    Handshake(0x00) {
        protocol_version VarInt;
        server_address String;
        server_port u16;
        next_state HandshakeState;
    },
);
