use crate::packets;

packets! {
    Response(0x00) {
        response String
    },
    Pong(0x01) {
        payload i64
    }
}
