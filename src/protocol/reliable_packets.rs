use crate::inner::rooms::{room_exists, GameRoom};
use crate::structs::structs::{GameOptionsData, PlatformSpecificData};
use crate::util::hazel::HazelMessage;
use crate::util::inner::GameCode;
use crate::{convert, Buffer, Packet, User};
use rand::distributions::{Alphanumeric, DistString, Standard};
use rand::Rng;
use tokio::net::UdpSocket;
use tracing::info;
use tracing::log::{debug, log};

pub struct HostGamePacket {
    pub code: Option<GameCode>,
}
pub struct JoinGamePacket {
    pub code: Option<GameCode>,
}
pub struct JoinedGamePacket {
    pub code: GameCode,
}

impl Packet for HostGamePacket {
    fn deserialize(&mut self, buffer: &mut Buffer) {
        let pos = buffer.position;
        info!("Hazel: {:?}", &buffer.array[pos..]);
        let settings = GameOptionsData::deserialize(buffer);
        info!("Game Options: {:?}", settings);
        info!("Crossplay Flags: {:?}", buffer.read_i32());
        self.code = Some(GameCode::new_random());
        while room_exists(self.code.as_ref().unwrap().clone()) {
            self.code = Some(GameCode::new_random());
        }
        info!("Code: {:?}", self.code.as_ref().unwrap().code_string);
    }

    fn serialize(self, buffer: &mut Buffer) {
        let mut hazel_message = HazelMessage::start_message(0x00);
        hazel_message
            .buffer
            .write_i32_le(self.code.unwrap().code_int);
        hazel_message.end_message();
        hazel_message.copy_to(buffer);
    }

    fn process(self, user: &mut &User, socket: &UdpSocket) {
        GameRoom::new(self.code.as_ref().unwrap().clone());
        user.send_reliable_packet(self, socket);
    }
}

impl Packet for JoinGamePacket {
    fn deserialize(&mut self, buffer: &mut Buffer) {
        self.code = Some(GameCode::new_code_int(buffer.read_i32()));
    }

    fn serialize(self, buffer: &mut Buffer) {}

    fn process(self, user: &mut &User, socket: &UdpSocket) {
        user.send_reliable_packet(
            JoinedGamePacket {
                code: self.code.unwrap(),
            },
            socket,
        );
    }
}

impl Packet for JoinedGamePacket {
    fn deserialize(&mut self, buffer: &mut Buffer) {}

    fn serialize(self, buffer: &mut Buffer) {
        let mut hazel_message = HazelMessage::start_message(0x07);
        hazel_message.buffer.write_i32_le(self.code.code_int);
        hazel_message.buffer.write_i32_le(1);
        hazel_message.buffer.write_i32_le(1);
        hazel_message.buffer.write_packed_u32(0);
        hazel_message.end_message();
        hazel_message.copy_to(buffer);
        println!("Finished buffer: {:?}", convert(&buffer.array));
    }

    fn process(self, user: &mut &User, socket: &UdpSocket) {}
}
