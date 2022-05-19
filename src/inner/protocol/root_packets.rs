use crate::inner::server::GameOptionsData;
use crate::networking::buffer::Buffer;
use crate::{Packet, User};
use tokio::net::UdpSocket;

pub struct HostGame {
    pub game_options_data: Option<GameOptionsData>,
    pub quick_chat_mode: Option<u8>,
}

impl Packet for HostGame {
    fn get_packet_id(&self) -> u8 {
        todo!()
    }

    fn deserialize(&mut self, buffer: &mut Buffer) {
        self.set_game_options_data(Some(GameOptionsData::read(buffer)));
        self.set_quick_chat_mode(Some(buffer.read_unsigned_byte()));
        println!("Options: {:?}", self.game_options_data.as_ref().unwrap());
        println!(
            "Quick Chat Mode: {}",
            self.quick_chat_mode.as_ref().unwrap()
        );
    }

    fn serialize(&self, buffer: &mut Buffer) {
        todo!()
    }

    fn process_packet(&self, socket: &UdpSocket, user: &User) {
        todo!()
    }
}

impl HostGame {
    pub fn game_options_data(&self) -> &Option<GameOptionsData> {
        &self.game_options_data
    }
    pub fn quick_chat_mode(&self) -> Option<u8> {
        self.quick_chat_mode
    }
    pub fn set_game_options_data(&mut self, game_options_data: Option<GameOptionsData>) {
        self.game_options_data = game_options_data;
    }
    pub fn set_quick_chat_mode(&mut self, quick_chat_mode: Option<u8>) {
        self.quick_chat_mode = quick_chat_mode;
    }
}
