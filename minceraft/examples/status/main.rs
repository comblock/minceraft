mod packets;
use minceraft::net::conn::Conn;
use minceraft::net::packet::Packet;
use std::net::ToSocketAddrs;

#[tokio::main]
async fn main() {
    let port = 25565;
    let addr = "127.0.0.1";
    let mut conn = Conn::connect(format!("{addr}:{port}").to_socket_addrs().unwrap().next().unwrap()).await.unwrap();

    let packet = packets::Handshake {
        protocol_version: 47,
        server_address: String::from(addr),
        server_port: port,
        next_state: packets::HandshakeState::Status,
    };
    conn.send_packet(packet).await.unwrap();
    conn.send_packet(packets::Request {}).await.unwrap();
    let packet = packets::Response::decode(conn.read_packet().await.unwrap()).unwrap();
    println!("{}", packet.json_response);

    let time = chrono::Utc::now();
    let timestamp = time.timestamp_millis();

    let packet = packets::Ping { payload: timestamp };
    conn.send_packet(packet).await.unwrap();

    let resp = conn.read_packet().await.unwrap();
    let ping = chrono::Utc::now()
        .signed_duration_since(time)
        .num_milliseconds();

    let pong = packets::Pong::decode(resp).unwrap();
    assert_eq!(pong.payload, timestamp);

    println!("Ping: {}ms", ping);
}
