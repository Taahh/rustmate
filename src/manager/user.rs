use crate::manager::states::UserState;
use crate::protocol::packet::AcknowledgementPacket;
use crate::util::hazel::HazelMessage;
use crate::{convert, Buffer, Packet, CONNECTIONS};
use std::mem::transmute;
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tracing::info;

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
pub struct User {
    pub state: UserState,
    pub socketAddr: SocketAddr,
    pub serverNonce: u16,
}

impl User {
    pub fn new(state: UserState, addr: SocketAddr) -> Self {
        User {
            state,
            socketAddr: addr,
            serverNonce: 0,
        }
    }

    pub fn send_ack(&self, nonce: u16, socket: &UdpSocket) {
        let mut buffer = Buffer {
            array: Vec::new(),
            position: 0,
        };
        buffer.write_u8(0x0a);
        let acknowledgementPacket = AcknowledgementPacket { nonce };
        acknowledgementPacket.serialize(&mut buffer);
        info!("Buffer: {:?}", buffer);
        futures::executor::block_on(socket.send_to(&buffer.array, self.socketAddr)).unwrap();
        info!("Sending ack for nonce {:?}", nonce);
    }

    pub fn send_reliable_packet(&self, packet: impl Packet, socket: &UdpSocket) {
        let mut buffer = Buffer {
            array: Vec::new(),
            position: 0,
        };
        buffer.write_u8(0x01);
        // CONNECTIONS.lock().unwrap().get(&self.socketAddr).unwrap().serverNonce += 1;
        let nonce = self.serverNonce + 1;
        let addr = self.socketAddr;
        let state = self.state;
        tokio::spawn(async move {
            CONNECTIONS.lock().unwrap().insert(
                addr,
                Some(User {
                    state,
                    socketAddr: addr,
                    serverNonce: nonce,
                }),
            );
        });

        buffer.write_u16(nonce);
        packet.serialize(&mut buffer);
        let length =
            futures::executor::block_on(socket.send_to(&buffer.array, self.socketAddr)).unwrap();
        info!(
            "Sending reliable packet with length {:?} and buffer {:?}",
            length,
            convert(&buffer.array)
        );
    }

    pub fn send_disconnect(&self, message: String, socket: &UdpSocket) {
        let mut buffer = Buffer {
            array: Vec::new(),
            position: 0,
        };
        buffer.write_i8(0x09);
        buffer.write_i8(1);
        let mut hazel_message = HazelMessage::start_message(0x00);
        hazel_message.buffer.write_i8(0x08);
        hazel_message.buffer.write_string(message);
        println!("Disconnect: {:?}", hazel_message.buffer);
        hazel_message.end_message();
        println!("Disconnect: {:?}", hazel_message.buffer);
        hazel_message.copy_to(&mut buffer);
        println!("Disconnect: {:?}", buffer);
        let length =
            futures::executor::block_on(socket.send_to(&buffer.array, self.socketAddr)).unwrap();
        info!("Sending disconnect packet with length {:?}", length);
    }
}
