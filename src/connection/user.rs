use std::net::SocketAddr;
use tokio::net::UdpSocket;
use crate::inner::protocol::core_packets::AcknowledgePacket;
use crate::networking::buffer::Buffer;
use crate::{convert, Packet, Server};

pub struct User {
    pub addr: SocketAddr
}

impl User {
    pub fn send_packet<T: Packet>(&self, socket: &UdpSocket, nonce: Option<u16>, packet: T) {
        let buff: [u8; 2048] = [0; 2048];
        let mut buffer = Buffer::new(buff.to_vec());
        buffer.write_byte(packet.get_packet_id());
        if nonce.is_some() {
            buffer.write_uint_16(nonce.unwrap());
        }
        packet.serialize(&mut buffer);
        let current_position = buffer.position();
        buffer.set_position(0);
        buffer.write_uint_16(buffer.size() as u16);
        buffer.set_position(current_position);
        println!("Raw Buffer: {:?}", buff);
        socket.send_to(&buff[..current_position], self.get_address().to_string());
        println!("Sending {:?}", &buff[..current_position])
    }

    pub fn send_ack(&self, socket: &UdpSocket, packet_nonce: u16) {
        let buff: [u8; 2048] = [0; 2048];
        let mut buffer = Buffer::new(buff.to_vec());
        let packet = AcknowledgePacket {
            nonce: packet_nonce
        };
        buffer.write_byte(packet.get_packet_id());
        buffer.write_uint_16(packet_nonce);
        buffer.write_byte(0xff);
        let buff = &buffer.array()[..buffer.position()];
        println!("Raw Buffer: {:?}", buff);
        socket.send_to(buff, self.get_address().to_string());
        println!("Sending to {}, Ack: {:?}", self.get_address().to_string(), convert(buff));
    }

    pub fn get_address(&self) -> SocketAddr {
        self.addr
    }
}