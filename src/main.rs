use std::io;
use std::net::SocketAddr;
use std::str::FromStr;
use tokio::net::UdpSocket;
use crate::inner::protocol::core_packets::HelloPacket;
use crate::inner::protocol::Packet;

#[path = "./networking/networking.rs"]
mod networking;

#[path = "./inner/inner.rs"]
mod inner;

#[path = "./connection/user.rs"]
mod user;

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = SocketAddr::from_str("127.0.0.1:22023").unwrap();
    let socket = UdpSocket::bind(&addr).await?;
    println!("Listening on /{:?}", addr);
    let mut buf = [0; 2048];
    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        println!("Received a packet of {} length from /{:?}", len, addr);
        println!("Packet Buffer: {:?}", convert(buf.as_ref()));
        let mut buffer = networking::buffer::Buffer::new(buf.to_vec());
        let packet_type = buffer.read_byte();
        println!("Packet Type: {}", packet_type);
        println!("Nonce: {}", buffer.read_uint_16());
        match packet_type {
            8 => {
                let packet = HelloPacket {};
                packet.deserialize(&mut buffer);

            },
            2 => {

            }
            _ => {}
        }
    }
}

fn convert(array: &[u8]) -> Vec<String> {
    let mut arr: Vec<String> = Vec::new();
    for x in array {
        arr.push(format!("{:02X?}", x));
    }
    return arr;
}
