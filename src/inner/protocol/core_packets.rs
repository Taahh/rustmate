use crate::inner::protocol::root_packets::{HostGame, JoinGame, JoinedGame, ModdedHandshake};
use crate::inner::protocol::Packet;
use crate::networking::buffer::Buffer;
use crate::user::User;
use crate::{code_to_int, convert, HazelMessage};
use std::borrow::BorrowMut;
use tokio::net::UdpSocket;

pub struct HelloPacket {
    pub nonce: u16,
    pub modded_hello: Option<ModdedHelloPacket>,
    pub client_version: Option<i32>,
    pub username: Option<String>,
    pub auth: Option<i32>,
    pub language: Option<i32>,
    pub chat_mode: Option<i8>,
    pub platform_id: Option<u8>,
    pub platform_name: Option<String>,
    pub protocol_version: Option<i8>,
    pub mod_count: Option<u32>
}

pub struct ModdedHelloPacket {
    pub protocol_version: i8,
    pub mod_count: u32
}

pub struct AcknowledgePacket {
    pub nonce: u16,
}

pub struct DisconnectPacket;

pub struct ReactorPacket;

pub struct ReliablePacket {
    pub nonce: u16,
    pub hazel_msg: Option<HazelMessage>,
}

impl Packet for DisconnectPacket {
    fn get_packet_id(&self) -> u8 {
        0x09
    }

    fn deserialize(&mut self, buffer: &mut Buffer) {
        todo!()
    }

    fn serialize(&self, buffer: &mut Buffer) -> Buffer {
        buffer.write_byte(0x01);
        let mut hazel_msg = HazelMessage::start_message(0);
        hazel_msg
            .payload()
            .write_byte(0x08);
        hazel_msg.payload().write_string("Hello".to_string());
        hazel_msg.end_message();
        buffer.combine(&mut hazel_msg.payload().array());
        println!("New Buffer: {:?}", convert(&buffer.array()[0..]));
        buffer.set_position(hazel_msg.payload().position() + buffer.position() + 2);
        return buffer.clone();
    }

    fn process_packet(&mut self, socket: &UdpSocket, user: &User) {
        todo!()
    }
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
                user.send_reliable_packet(socket, self.nonce, JoinedGame {
                    code: code_to_int("REDSUS".to_string()),
                    join_id: 1,
                    host_id: 1
                });
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
        println!("Authentication: {}", buffer.read_int_32());
        println!("Last Language: {}", buffer.read_uint_32());
        println!("Chat Mode Type: {}", buffer.read_byte());
        let mut hazel_msg = HazelMessage::read(buffer).unwrap();
        println!("Platform Name: {}", hazel_msg.payload().read_string());
        buffer.set_position(hazel_msg.payload().position() + buffer.position());
        buffer.read_string();
        buffer.read_uint_32();
        println!(
            "Remaining Buffer: {:?}",
            &buffer.array()[buffer.position()..]
        );
        if buffer.position() < buffer.size() {
            println!(
                "Remaining Buffer: {:?}",
                &buffer.array()[buffer.position()..]
            );
            let protocol_version = buffer.read_byte();
            let mod_count = buffer.read_packed_uint_32();
            println!("Mod Count: {}", mod_count);
            self.modded_hello = Some(ModdedHelloPacket {
                protocol_version, mod_count
            });
        }
    }

    fn serialize(&self, buffer: &mut Buffer) -> Buffer {
        // buffer.write_uint_16(self.nonce);
        // buffer.write_int_32(self.client_version.unwrap());
        // buffer.write_string(self.username.unwrap());
        return buffer.clone();
    }

    fn process_packet(&mut self, socket: &UdpSocket, user: &User) {
        user.send_ack(socket, self.nonce);
        user.send_packet(socket, None, DisconnectPacket {});
        // if self.modded_hello.is_some() {
        //     let modded_hello_packet = self.modded_hello.as_ref().unwrap();
        //     let modded_handshake = ModdedHandshake {
        //         protocol_version: modded_hello_packet.protocol_version,
        //         mod_count: modded_hello_packet.mod_count
        //     };
        //     user.send_reliable_packet(socket, self.nonce+1, modded_handshake);
        // }
    }
}
