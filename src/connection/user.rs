use crate::inner::protocol::core_packets::AcknowledgePacket;
use crate::networking::buffer::Buffer;
use crate::{convert, Packet, ReliablePacket, Server};
use std::net::SocketAddr;
use tokio::net::UdpSocket;

#[derive(Clone, Copy)]
pub struct User {
    pub addr: SocketAddr,
}

impl User {
    pub fn send_reliable_packet<T: Packet>(&self, socket: &UdpSocket, nonce: u16, packet: T) {
        let buff: [u8; 2048] = [0; 2048];
        let mut buffer = Buffer::new(buff.to_vec());
        let reliable_packet = ReliablePacket {
            nonce,
            hazel_msg: None,
        };
        buffer.write_byte(reliable_packet.get_packet_id());
        // println!("Reliable Packet: ")
        buffer.write_uint_16(nonce);
        packet.serialize(&mut buffer);
        socket.send_to(
            &buffer.array()[..buffer.position()],
            self.get_address().to_string(),
        );
        println!(
            "Sending Reliable Packet {:?}",
            convert(&buffer.array()[..buffer.position()])
        )
    }

    pub fn send_packet<T: Packet>(&self, socket: &UdpSocket, nonce: Option<u16>, packet: T) {
        let buff: [u8; 2048] = [0; 2048];
        let mut buffer = Buffer::new(buff.to_vec());
        buffer.write_byte(packet.get_packet_id());
        if nonce.is_some() {
            buffer.write_uint_16(nonce.unwrap());
        }
        packet.serialize(&mut buffer);
        println!(
            "Sending Non-Reliable Packet {:?}",
            convert(&buffer.array()[..buffer.position()])
        );
        socket.send_to(&buffer.array()[..buffer.position()], self.get_address().to_string());
    }

    pub fn send_ack(&self, socket: &UdpSocket, packet_nonce: u16) {
        let buff: [u8; 2048] = [0; 2048];
        let mut buffer = Buffer::new(buff.to_vec());
        let packet = AcknowledgePacket {
            nonce: packet_nonce,
        };
        buffer.write_byte(packet.get_packet_id());
        buffer.write_uint_16(packet_nonce);
        buffer.write_byte(0xff);
        let packet_buffer = &buffer.array()[..buffer.position()];
        let length_sent = futures::executor::block_on(
            socket.send_to(&packet_buffer, self.get_address().to_string()),
        )
            .unwrap();
        println!(
            "Sending Ack to {} length {}, Ack: {:?}",
            self.get_address().to_string(),
            length_sent,
            convert(packet_buffer)
        );
    }

    pub fn get_address(&self) -> SocketAddr {
        self.addr
    }

    pub fn from(user: &User) -> Self {
        User { addr: user.addr }
    }
}
