mod packets;
use minceraft::net::conn::Conn;
use minceraft::net::packet::Packet;
use minceraft::net::types::VarInt;
use std::{net::ToSocketAddrs, time::Duration};

fn main() {
    let addr = "localhost:25565".to_socket_addrs().unwrap().next().unwrap();
    let mut conn = Conn::connect_timeout(&addr, Duration::new(30, 0)).unwrap();

    let packet = packets::Handshake {
        protocol_version: VarInt(47),
        server_address: String::from("localhost"),
        server_port: 25565,
        next_state: packets::HandshakeState::Status,
    };

    conn.send_packet(&packet).unwrap();
    conn.send_packet(&packets::Request {}).unwrap();

    let packet = packets::Response::decode(conn.read_packet().unwrap()).unwrap();
    println!("{}", packet.json_response);

    let time = chrono::Utc::now();
    let timestamp = time.timestamp_millis();

    let packet = packets::Ping { payload: timestamp };
    conn.send_packet(&packet).unwrap();

    let resp = conn.read_packet().unwrap();
    let ping = chrono::Utc::now()
        .signed_duration_since(time)
        .num_milliseconds();

    let pong = packets::Pong::decode(resp).unwrap();
    assert_eq!(pong.payload, timestamp);

    println!("Ping: {}ms", ping);
}
