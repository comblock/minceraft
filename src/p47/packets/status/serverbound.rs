use crate::packets;

packets! {
    Request(0x00) {},
    Ping(0x01) {
        payload i64
    }
}
