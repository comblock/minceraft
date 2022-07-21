use minceraft::{*, net::types::VarInt};

def_enum! {
    HandshakeState (VarInt) {
        1 = Status,
        2 = Login,
    }
}

#[derive(Packet)]
#[id(0x00)]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: HandshakeState,
}

#[derive(Packet)]
#[id(0x00)]
pub struct Request {}

#[derive(Packet)]
#[id(0x00)]
pub struct Response {
    pub json_response: String,
}

#[derive(Packet)]
#[id(0x01)]
pub struct Ping {
    pub payload: i64
}

#[derive(Packet)]
#[id(0x01)]
pub struct Pong {
    pub payload: i64
}
