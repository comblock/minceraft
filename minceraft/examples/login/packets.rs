use minceraft::{*, net::types::{VarInt, ByteArray}};

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
pub struct Disconnect {
    pub reason: String
}

#[derive(Packet)]
#[id(0x01)]
pub struct EncryptionRequest {
    pub server_id: String,
    pub public_key: ByteArray,
    pub verify_token: ByteArray,
}

#[derive(Packet)]
#[id(0x02)]
pub struct LoginSuccess {
    pub uuid: String,
    pub username: String,
}

#[derive(Packet)]
#[id(0x03)]
pub struct SetCompression {
    pub threshold: VarInt
}

#[derive(Packet)]
#[id(0x00)]
pub struct LoginStart {
    pub name: String
}

#[derive(Packet)]
#[id(0x01)]
pub struct EncryptionResponse {
    pub shared_secret: ByteArray,
    pub verify_token: ByteArray,
}

#[derive(Packet)]
#[id(0x00)]
pub struct KeepAlive {
    pub keep_alive: VarInt,
}