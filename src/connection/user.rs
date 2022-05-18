use tokio::net::UdpSocket;
use crate::networking::buffer::Buffer;
use crate::Packet;

pub struct User {
    connection: UdpSocket
}

impl User {
    pub fn send_packet<T: Packet>(nonce: Option<u16>, packet: T) {
        let buff: [u8; 2048] = [0; 2048];
        let mut buffer = Buffer::new(buff.to_vec());
        buffer.write_byte(packet.get_packet_id());
        if nonce.is_some() {
            buffer.write_uint_16(nonce.unwrap());
        }
        packet.serialize(&mut buffer);
        let length = buff.len();
        let current_position = buffer.

    }
}