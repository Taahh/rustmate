use crate::inner::protocol::Packet;
use crate::networking::buffer::Buffer;

pub struct HelloPacket;
pub struct AcknowledgePacket {
    nonce: u16
}

impl Packet for AcknowledgePacket {
    fn get_packet_id(&self) -> u8 {
        0x0a
    }

    fn deserialize(&self, buffer: &mut Buffer) {
        todo!()
    }

    fn serialize(&self, buffer: &mut Buffer) -> Buffer {

        return buffer.clone();
    }
}

impl Packet for HelloPacket {
    fn get_packet_id(&self) -> u8 {
        8
    }

    fn deserialize(&self, buffer: &mut Buffer) {
        let hazel_version = buffer.read_byte();
        let mut client_version = buffer.read_int_32();
        println!("Version: {}", client_version);
        println!("Username: {}", buffer.read_string());
    }

    fn serialize(&self, buffer: &mut Buffer) -> Buffer {
        todo!()
    }
}