use crate::packets;

packets! {
    LoginStart(0x00) {
        name String;
    },
    EncryptionResponse(0x01) {
        shared_secret VarIntPrefixedArray<u8>;
        verify_token VarIntPrefixedArray<u8>;
    },
}
