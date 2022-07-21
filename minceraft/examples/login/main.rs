mod packets;
use minceraft::auth;
use minceraft::net;
use minceraft::net::types::ByteArray;
use net::conn::Conn;
use net::packet::Packet;
use net::types::VarInt;
use rand::Rng;
use serde_json::json;
use sha1::Digest;
use std::{net::ToSocketAddrs, time::Duration};

fn main() {
    let http = reqwest::blocking::Client::new();

    let dc = auth::DeviceCode::new("389b1b32-b5d5-43b2-bddc-84ce938d6737", None, &http).unwrap();

    if let Some(inner) = &dc.inner {
        println!("{}", inner.message);
    }

    let auth = dc.authenticate(&http).unwrap();

    let addr = "mc.hypixel.net:25565"
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();

    let mut conn = Conn::connect_timeout(&addr, Duration::new(30, 0)).unwrap();

    let packet = packets::Handshake {
        protocol_version: VarInt(47),
        server_address: String::from("mc.hypixel.net"),
        server_port: 25565,
        next_state: packets::HandshakeState::Login,
    };
    conn.send_packet(&packet).unwrap();
    conn.send_packet(&packets::LoginStart { name: auth.name })
        .unwrap();

    loop {
        let packet = conn.read_packet().unwrap();
        match packet.id {
            packets::Disconnect::ID => {
                let packet = packets::Disconnect::decode(packet).unwrap();
                println!("disconnected: \"{}\"", packet.reason);
                break;
            }
            packets::EncryptionRequest::ID => {
                let packet = packets::EncryptionRequest::decode(packet).unwrap();
                let shared = rand::thread_rng().gen::<[u8; 16]>();

                let shared_e =
                    rsa_public_encrypt_pkcs1::encrypt(&packet.public_key.0, &shared).unwrap();
                let token_e =
                    rsa_public_encrypt_pkcs1::encrypt(&packet.public_key.0, &packet.verify_token.0)
                        .unwrap();

                let mut hasher = sha1::Sha1::new();
                hasher.update(&packet.server_id.as_bytes());
                hasher.update(&shared);
                hasher.update(packet.public_key.0);
                let mut hash = hasher.finalize();

                let negative = (hash[0] & 0x80) == 0x80;
                if negative {
                    let mut carry = true;
                    for i in (0..hash.len()).rev() {
                        hash[i] = !hash[i];
                        if carry {
                            carry = hash[i] == 0xFF;
                            hash[i] = hash[i].wrapping_add(1);
                        }
                    }
                }

                let hash_str = hash
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<Vec<String>>()
                    .join("");

                let hash_val = hash_str.trim_start_matches('0');
                let hash_str = if negative {
                    "-".to_owned() + &hash_val[..]
                } else {
                    hash_val.to_owned()
                };

                let _resp = http
                    .post("https://sessionserver.mojang.com/session/minecraft/join")
                    .json(&json!({
                        "accessToken": auth.token,
                        "selectedProfile": auth.uuid,
                        "serverId": hash_str,
                    }))
                    .send()
                    .unwrap()
                    .bytes()
                    .unwrap();

                conn.send_packet(&packets::EncryptionResponse {
                    shared_secret: ByteArray(shared_e),
                    verify_token: ByteArray(token_e),
                })
                .unwrap();

                conn.enable_encryption(&shared).unwrap();
            }
            packets::LoginSuccess::ID => {
                let packet = packets::LoginSuccess::decode(packet).unwrap();
                println!("logged in as {}", packet.username);
                break;
            }
            packets::SetCompression::ID => {
                let packet = packets::SetCompression::decode(packet).unwrap();
                conn.set_compression_threshhold(packet.threshold.0)
            }
            _ => {
                println!("Received packet: {:?}", packet.id);
            }
        }
    }
    loop {
        // Keeps the connection alive
        let packet = conn.read_packet().unwrap();
        if packet.id == packets::KeepAlive::ID {
            let packet = packets::KeepAlive::decode(packet).unwrap();
            conn.send_packet(&packets::KeepAlive {
                keep_alive: packet.keep_alive,
            })
            .unwrap();
        }
    }
}
