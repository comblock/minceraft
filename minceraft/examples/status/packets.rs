use minceraft::*;

def_enum! {
    HandshakeState (VarInt) {
        1 = Status,
        2 = Login,
    }
}

packets! {
    Handshake(0x00) {
        protocol_version VarInt;
        server_address String;
        server_port u16;
        next_state HandshakeState;
    },
    Request(0x00) {},
    Response(0x00) {
        json_response String;
    },
    Ping(0x01) {
        payload i64;
    },
    Pong(0x01) {
        payload i64;
    },
}
