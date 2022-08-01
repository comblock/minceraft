use minceraft::net::packet::Packet;
use minceraft_derive::Packet;

#[derive(Packet)]
#[id(0x00)]
struct Abc {
    a: bool,
    b: i32,
    c: u8,
}

fn main() {
    let a = Abc {
        a: true,
        b: 12,
        c: 255,
    };

    def(a)
}

fn def(g: impl Packet) {
    let a = g.encode().unwrap();
    for i in a.data {
        println!("{:08b}", i)
    }
}
