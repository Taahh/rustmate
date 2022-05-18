use tokio::net::UdpSocket;
use crate::networking::buffer::Buffer;
use crate::user::User;

pub mod core_packets;

pub trait Packet {
    fn get_packet_id(&self) -> u8;
    fn deserialize(&self, buffer: &mut Buffer);
    fn serialize(&self, buffer: &mut Buffer);
    fn process_packet(&self, socket: &UdpSocket, user: &User);
}