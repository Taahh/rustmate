use crate::protocol::reliable_packets::{HostGamePacket, JoinGamePacket};
use crate::structs::structs::PlatformSpecificData;
use crate::util::hazel::HazelMessage;
use crate::{Buffer, User};
use tokio::net::UdpSocket;
use tracing::info;
use tracing::log::{debug, log};

pub trait Packet {
    fn deserialize(&mut self, buffer: &mut Buffer);
    fn serialize(self, buffer: &mut Buffer);

    fn process(self, user: &mut &User, socket: &UdpSocket);
}

#[derive(Debug)]
pub struct AcknowledgementPacket {
    pub nonce: u16,
}

#[derive(Debug)]
pub struct HelloPacket {
    pub nonce: u16,
    pub version: Option<i32>,
    pub username: Option<String>,
    pub lastNonce: Option<u32>,
    pub lastLanguage: Option<u32>,
    pub chatMode: Option<i8>,
    pub platformData: Option<PlatformSpecificData>,
}

#[derive(Debug)]
pub struct ReliablePacket {
    pub nonce: u16,
    pub reliable_packet_id: Option<i8>,
    pub hazel_message: Option<HazelMessage>,
}

#[derive(Debug)]
pub struct PingPacket {
    pub nonce: u16,
}

impl Packet for AcknowledgementPacket {
    fn deserialize(&mut self, buffer: &mut Buffer) {}

    fn serialize(self, buffer: &mut Buffer) {
        buffer.write_u16(self.nonce);
        buffer.write_u8(0xff);
    }

    fn process(self, user: &mut &User, socket: &UdpSocket) {}
}

impl Packet for HelloPacket {
    fn deserialize(&mut self, buffer: &mut Buffer) {
        buffer.read_i8();
        self.version = Some(buffer.read_i32());
        self.username = Some(buffer.read_string());
        self.lastNonce = Some(buffer.read_u32());
        self.lastLanguage = Some(buffer.read_u32());
        self.chatMode = Some(buffer.read_i8());
        let mut platformData = HazelMessage::read_message(buffer);
        self.platformData = Some(PlatformSpecificData {
            platform: platformData.tag,
            platformName: platformData.buffer.read_string(),
        });
        buffer.read_string();
        buffer.read_u32();
    }

    fn serialize(self, buffer: &mut Buffer) {}

    fn process(self, user: &mut &User, socket: &UdpSocket) {
        info!("hello packet {:?}", self);
        user.send_ack(self.nonce, socket);
    }
}

impl Packet for ReliablePacket {
    fn deserialize(&mut self, buffer: &mut Buffer) {
        let pos = buffer.position;
        info!("Hazel: {:?}", &buffer.array[pos..]);
        let reliable_hazel = HazelMessage::read_message(buffer);
        let reliable_packet_id = reliable_hazel.tag;
        self.reliable_packet_id = Some(reliable_packet_id);
        self.hazel_message = Some(reliable_hazel);
        let pos = buffer.position;
        info!("Hazel: {:?}", &buffer.array[pos..]);
    }

    fn serialize(self, buffer: &mut Buffer) {}

    fn process(self, user: &mut &User, socket: &UdpSocket) {
        user.send_ack(self.nonce, socket);
        info!(
            "Handling reliable packet {:?}",
            self.reliable_packet_id.unwrap()
        );
        let id = self.reliable_packet_id.unwrap();
        if id == 0 {
            info!("Reliable Host Game Packet");
            let mut packet = HostGamePacket { code: None };
            packet.deserialize(&mut self.hazel_message.unwrap().buffer);
            packet.process(user, socket);
        } else if id == 1 {
            info!("Reliable Join Game Packet");
            let mut join_game = JoinGamePacket { code: None };
            join_game.deserialize(&mut self.hazel_message.unwrap().buffer);
            join_game.process(user, socket);
        }
    }
}

impl Packet for PingPacket {
    fn deserialize(&mut self, buffer: &mut Buffer) {}

    fn serialize(self, buffer: &mut Buffer) {}

    fn process(self, user: &mut &User, socket: &UdpSocket) {
        user.send_ack(self.nonce, socket);
    }
}
