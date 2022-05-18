use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use std::str::FromStr;
use tokio::net::UdpSocket;
use uuid::Uuid;
use crate::inner::protocol::core_packets::HelloPacket;
use crate::inner::protocol::Packet;
use crate::user::User;

#[path = "./networking/networking.rs"]
mod networking;

#[path = "./inner/inner.rs"]
mod inner;

#[path = "./connection/user.rs"]
mod user;

const USERS: Vec<User> = Vec::new();

pub struct Server {
    pub bind_addr: SocketAddr,
    pub socket: UdpSocket,
}

impl Server {
    pub fn get_socket(&self) -> &UdpSocket {
        &self.socket
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = SocketAddr::from_str("127.0.0.1:22023").unwrap();
    let socket = UdpSocket::bind(&addr).await?;
    println!("Listening on /{:?}", addr);
    let mut buf = [0; 2048];
    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        /*let mut user: Option<User> = None;
        if !USERS.iter().any(|u| u.get_address().eq(&addr)) {
            user = Some(User {
                addr
            });
            USERS.push(user.unwrap());
        } else {
            for x in USERS {
                if x.get_address().eq(&addr) {
                    user = Some(x);
                    break;
                }
            }
        }*/
        println!("Received a packet of {} length from /{:?}", len, addr);
        println!("Packet Buffer: {:?}", convert(buf.as_ref()));
        let mut buffer = networking::buffer::Buffer::new(buf.to_vec());
        let packet_type = buffer.read_byte();
        println!("Packet Type: {}", packet_type);
        let nonce = buffer.read_uint_16();
        println!("Nonce: {}", nonce);
        match packet_type {
            8 => {
                let packet = HelloPacket {
                    nonce
                };
                packet.deserialize(&mut buffer);
                packet.process_packet(&socket, &User { addr });
            }
            2 => {}
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
