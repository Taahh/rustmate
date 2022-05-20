use crate::inner::protocol::root_packets::{HostGame, JoinGame, JoinedGame};
use crate::inner::protocol::Packet;
use crate::networking::buffer::Buffer;
use crate::user::User;
use crate::{code_to_int, HazelMessage};
use std::borrow::BorrowMut;
use tokio::net::UdpSocket;

pub struct HelloPacket {
    pub nonce: u16,
}
pub struct AcknowledgePacket {
    pub nonce: u16,
}

pub struct ReactorPacket;

pub struct ReliablePacket {
    pub nonce: u16,
    pub hazel_msg: Option<HazelMessage>,
}

impl Packet for ReactorPacket {
    fn get_packet_id(&self) -> u8 {
        255
    }

    fn deserialize(&mut self, buffer: &mut Buffer) {}

    fn serialize(&self, buffer: &mut Buffer) -> Buffer {
        buffer.write_byte(0);
        buffer.write_string("Rustmate".to_string());
        buffer.write_string("0.0.1".to_string());
        buffer.write_packed_uint_32(0);
        return buffer.clone();
    }

    fn process_packet(&mut self, socket: &UdpSocket, user: &User) {}
}

impl Packet for ReliablePacket {
    fn get_packet_id(&self) -> u8 {
        1
    }

    fn deserialize(&mut self, buffer: &mut Buffer) {
        self.set_hazel_msg(HazelMessage::read(buffer));
    }

    fn serialize(&self, buffer: &mut Buffer) -> Buffer {
        return buffer.clone();
    }

    fn process_packet(&mut self, socket: &UdpSocket, user: &User) {
        user.send_ack(socket, self.nonce);
        user.send_packet(socket, None, ReactorPacket {});
        let msg: &mut HazelMessage = self.hazel_msg.as_mut().unwrap();
        println!("Tag: {}", msg.tag());
        match msg.tag() {
            0 => {
                println!("Host Game Packet!");
                let mut host_game = HostGame {
                    quick_chat_mode: None,
                    game_options_data: None,
                };
                host_game.deserialize(&mut Buffer::from(msg.payload()));
                host_game.process_packet(socket, user);
                user.send_reliable_packet(socket, self.nonce, host_game);
            }
            1 => {
                println!("Join Game Packet!");
                let mut join_game = JoinGame { code: None };
                join_game.deserialize(&mut Buffer::from(msg.payload()));
                let joined_game = JoinedGame {
                    code: code_to_int("REDSUS".to_string()),
                    host_id: 0,
                    join_id: 0,
                };
                user.send_reliable_packet(socket, self.nonce, joined_game);
            }
            _ => {}
        }
    }
}

impl ReliablePacket {
    fn set_hazel_msg(&mut self, msg: Option<HazelMessage>) {
        self.hazel_msg = msg;
    }
}

impl Packet for AcknowledgePacket {
    fn get_packet_id(&self) -> u8 {
        0x0a
    }

    fn deserialize(&mut self, buffer: &mut Buffer) {
        todo!()
    }

    fn serialize(&self, buffer: &mut Buffer) -> Buffer {
        return buffer.clone();
    }

    fn process_packet(&mut self, socket: &UdpSocket, user: &User) {}
}

impl Packet for HelloPacket {
    fn get_packet_id(&self) -> u8 {
        8
    }

    fn deserialize(&mut self, buffer: &mut Buffer) {
        let hazel_version = buffer.read_byte();
        let mut client_version = buffer.read_int_32();
        println!("Version: {}", client_version);
        println!("Username: {}", buffer.read_string());
        if buffer.position() < buffer.size() {
            println!(
                "Remaining Buffer: {:?}",
                &buffer.array()[buffer.position()..]
            );
            println!("Protocol Version: {}", buffer.read_byte());
            println!("Mod Count: {}", buffer.read_packed_uint_32());
        }
    }

    fn serialize(&self, buffer: &mut Buffer) -> Buffer {
        return buffer.clone();
    }

    fn process_packet(&mut self, socket: &UdpSocket, user: &User) {
        println!("Process");
        user.send_ack(socket, self.nonce);
    }
}
