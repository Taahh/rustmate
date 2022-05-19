extern crate core;

use crate::inner::protocol::core_packets::{HelloPacket, ReliablePacket};
use crate::inner::protocol::Packet;
use crate::networking::hazel_message::HazelMessage;
use crate::user::User;
use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use std::str::FromStr;
use tokio::net::UdpSocket;

#[path = "./networking/networking.rs"]
mod networking;

#[path = "./inner/inner.rs"]
mod inner;

#[path = "./connection/user.rs"]
mod user;

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
    let mut users: HashMap<SocketAddr, User> = HashMap::new();

    let addr = SocketAddr::from_str("127.0.0.1:22023").unwrap();
    let socket = UdpSocket::bind(&addr).await?;
    println!("Listening on /{:?}", addr);
    let mut buf = [0; 2048];
    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        let mut user: Option<User>;
        if !users.contains_key(&addr) {
            /*user = Some(User {
                addr
            });*/
            // USERS.push(user.unwrap());
            let mut new_user = User { addr };
            users.insert(addr, new_user);
            user = Some(new_user);
        } else {
            let pointer_user = users.get(&addr).unwrap();
            user = Some(User::from(pointer_user));
        }
        let mut buffer = networking::buffer::Buffer::new(buf.to_vec());
        let packet_type = buffer.read_byte();
        println!(
            "Received a packet of length {}, packet ID {}, from /{:?}",
            len, packet_type, addr
        );
        println!("Packet Buffer: {:?}", convert(buf.as_ref()));
        match packet_type {
            8 => {
                let nonce = buffer.read_uint_16();
                println!("Hello Packet: {}", nonce);
                let mut packet = HelloPacket { nonce };
                packet.deserialize(&mut buffer);
                packet.process_packet(&socket, &user.unwrap());
            }
            1 => {
                let nonce = buffer.read_uint_16();
                let mut reliable_packet = ReliablePacket {
                    nonce,
                    hazel_msg: None,
                };
                reliable_packet.deserialize(&mut buffer);
                reliable_packet.process_packet(&socket, &user.unwrap());
            }
            0x0c => {
                let nonce = buffer.read_uint_16();
                user.unwrap().send_ack(&socket, nonce);
            }
            _ => {}
        }
    }
}

fn convert(array: &[u8]) -> Vec<String> {
    let mut arr: Vec<String> = Vec::new();
    for x in array {
        arr.push(format!("{:#04X?}", x));
    }
    return arr;
}
