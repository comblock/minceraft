use crate::packets;

packets! {
    LoginStart(0x00) {
        name String;
    },
    EncryptionResponse(0x01) {
        shared_secret VarIntPrefixedVec<u8>;
        verify_token VarIntPrefixedVec<u8>;
    },
}
