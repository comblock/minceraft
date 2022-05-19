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
    LoginStart(0x00) {
        name String;
    },
    EncryptionResponse(0x01) {
        shared_secret VarIntPrefixedVec<u8>;
        verify_token VarIntPrefixedVec<u8>;
    },

    KeepAlive(0x00) {
        keep_alive VarInt;
    },
}
