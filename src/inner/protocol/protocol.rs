use crate::networking::buffer::Buffer;
use crate::user::User;
use tokio::net::UdpSocket;

pub mod core_packets;
pub mod root_packets;

pub trait Packet {
    fn get_packet_id(&self) -> u8;
    fn deserialize(&mut self, buffer: &mut Buffer);
    fn serialize(&self, buffer: &mut Buffer) -> Buffer;
    fn process_packet(&mut self, socket: &UdpSocket, user: &User);
}
