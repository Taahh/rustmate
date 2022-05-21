use crate::inner::server::GameOptionsData;
use crate::networking::buffer::Buffer;
use crate::{code_to_int, convert, int_to_code, HazelMessage, Packet, User};
use tokio::net::UdpSocket;

pub struct HostGame {
    pub game_options_data: Option<GameOptionsData>,
    pub quick_chat_mode: Option<u32>,
}

pub struct JoinGame {
    pub code: Option<i32>,
}

pub struct JoinedGame {
    pub code: i32,
    pub join_id: i32,
    pub host_id: i32,
}

pub struct ModdedHandshake {
    pub protocol_version: i8,
    pub mod_count: u32
}

impl Packet for ModdedHandshake {
    fn get_packet_id(&self) -> u8 {
        todo!()
    }

    fn deserialize(&mut self, buffer: &mut Buffer) {
    }

    fn serialize(&self, buffer: &mut Buffer) -> Buffer {
        let mut hazel_msg = HazelMessage::start_message(255);
        hazel_msg
            .payload()
            .write_byte(0);
        hazel_msg.payload().write_string("Test Server".to_string());
        hazel_msg.payload().write_string("0.0.1".to_string());
        hazel_msg.payload().write_packed_uint_32(0);
        hazel_msg.end_message();
        println!(
            "Hazel Msg: {:?}",
            convert(&hazel_msg.payload().array()[0..])
        );
        buffer.combine(&mut hazel_msg.payload().array());
        println!("New Buffer: {:?}", convert(&buffer.array()[0..]));
        buffer.set_position(hazel_msg.payload().position() + buffer.position() + 2);
        return buffer.clone();
    }

    fn process_packet(&mut self, socket: &UdpSocket, user: &User) {}
}

impl Packet for HostGame {
    fn get_packet_id(&self) -> u8 {
        todo!()
    }

    fn deserialize(&mut self, buffer: &mut Buffer) {
        self.set_game_options_data(Some(GameOptionsData::read(buffer)));
        self.set_quick_chat_mode(Some(buffer.read_uint_32()));
        println!("Options: {:?}", self.game_options_data.as_ref().unwrap());
        println!(
            "Cross-play All Platforms? {}",
            self.quick_chat_mode.as_ref().unwrap()
        );
    }

    fn serialize(&self, buffer: &mut Buffer) -> Buffer {
        let mut hazel_msg = HazelMessage::start_message(0x00);
        hazel_msg
            .payload()
            .write_int_32(code_to_int("REDSUS".to_string()));
        hazel_msg.end_message();
        println!(
            "Hazel Msg: {:?}",
            convert(&hazel_msg.payload().array()[0..])
        );
        buffer.combine(&mut hazel_msg.payload().array());
        println!("New Buffer: {:?}", convert(&buffer.array()[0..]));
        buffer.set_position(hazel_msg.payload().position() + buffer.position() + 2);
        return buffer.clone();
    }

    fn process_packet(&mut self, socket: &UdpSocket, user: &User) {}
}

impl HostGame {
    pub fn game_options_data(&self) -> &Option<GameOptionsData> {
        &self.game_options_data
    }
    pub fn quick_chat_mode(&self) -> Option<u32> {
        self.quick_chat_mode
    }
    pub fn set_game_options_data(&mut self, game_options_data: Option<GameOptionsData>) {
        self.game_options_data = game_options_data;
    }
    pub fn set_quick_chat_mode(&mut self, quick_chat_mode: Option<u32>) {
        self.quick_chat_mode = quick_chat_mode;
    }
}

impl Packet for JoinGame {
    fn get_packet_id(&self) -> u8 {
        todo!()
    }

    fn deserialize(&mut self, buffer: &mut Buffer) {
        println!("Join Packet: {:?}", convert(&buffer.array()[0..]));
        let code = buffer.read_int_32();
        println!("Join Code: {}, {}", code, int_to_code(code));
    }

    fn serialize(&self, buffer: &mut Buffer) -> Buffer {
        return buffer.clone();
    }

    fn process_packet(&mut self, socket: &UdpSocket, user: &User) {}
}

impl Packet for JoinedGame {
    fn get_packet_id(&self) -> u8 {
        todo!()
    }

    fn deserialize(&mut self, buffer: &mut Buffer) {
        //     println!("Join Packet: {:?}", convert(&buffer.array()[0..]));
        //     let code = buffer.read_int_32();
        //     println!("Join Code: {}, {}", code, int_to_code(code));
    }

    fn serialize(&self, buffer: &mut Buffer) -> Buffer {
        println!("Serializing Joined Game Packet");
        let mut hazel_msg = HazelMessage::start_message(0x07);
        hazel_msg
            .payload()
            .write_int_32(code_to_int("REDSUS".to_string()));
        hazel_msg.payload().write_int_32(1);
        hazel_msg.payload().write_int_32(1);
        hazel_msg.payload().write_packed_uint_32(0);
        hazel_msg.end_message();
        println!("Hazel: {:?}", convert(&hazel_msg.payload().array()[0..]));
        buffer.combine(&mut hazel_msg.payload().array());
        buffer.set_position(hazel_msg.payload().position() + buffer.position() + 2);
        return buffer.clone();
    }

    fn process_packet(&mut self, socket: &UdpSocket, user: &User) {}
}
