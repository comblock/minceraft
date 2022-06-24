use crate::packets;

packets! {
    Disconnect(0x00) {
        reason String;
    },
    EncryptionRequest(0x01) {
        server_id String;
        public_key VarIntPrefixedVec<u8>;
        verify_token VarIntPrefixedVec<u8>;
    },
    LoginSuccess(0x02) {
        uuid String;
        username String;
    }

    SetCompression(0x03) {
        threshold VarInt;
    }
}
