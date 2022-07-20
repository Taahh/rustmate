use crate::connections::update_user;
use crate::manager::states::UserState;
use crate::protocol::packet::AcknowledgementPacket;
use crate::structs::structs::PlatformSpecificData;
use crate::util::hazel::HazelMessage;
use crate::util::inner::GameCode;
use crate::{convert, Buffer, DisconnectPacket, Packet, CONNECTIONS};
use std::mem::transmute;
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tracing::info;
use crate::inner::rooms::{room_exists, ROOMS};
use crate::util::util::to_string;
#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub struct User {
    pub state: UserState,
    pub socketAddr: SocketAddr,
    pub serverNonce: u16,
    pub username: Option<String>,
    pub player: Option<Player>,
    pub platformData: Option<PlatformSpecificData>,
}

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub struct Player {
    pub id: i32,
    pub game_code: GameCode,
}

impl User {
    pub fn new(state: UserState, addr: SocketAddr) -> Self {
        User {
            state,
            socketAddr: addr,
            serverNonce: 0,
            username: None,
            player: None,
            platformData: None,
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
        info!("EXISTING NONCE: {:?}", nonce);
        let addr = self.socketAddr;
        let username = self.username.as_ref().unwrap_or(&"N/A".to_string()).to_owned();
        let mut player = self.to_owned().player;
        tokio::spawn(async move {
            CONNECTIONS
                .lock()
                .await
                .get_mut(&addr)
                .unwrap()
                .as_mut()
                .unwrap()
                .serverNonce += 1;
            if player != None {
                println!("PLAYER WAS FOUND DURING NONCE {:?}", nonce);
                let player_actual = player.as_ref().unwrap().to_owned();
                let code = player_actual.game_code;
                if room_exists(code.to_owned()) {
                    println!("AND PLAYER ROOM EXISTS DURING NONCE {:?}", nonce);
                    ROOMS.lock().await.get_mut(&code).unwrap().as_mut().unwrap().players.get_mut(&player_actual.id).unwrap().as_mut().unwrap().serverNonce = nonce;
                }
            }
            info!("UPDATED NONCE TO {:?} FOR {:?}", nonce, username);
        });
        // info!("UPDATING USER RELIABLE: {:?}", user_option.as_ref().unwrap());
        buffer.write_u16(nonce);
        packet.serialize(&mut buffer);
        let length =
            futures::executor::block_on(socket.send_to(&buffer.array, self.socketAddr)).unwrap();
        info!(
            "Sending reliable packet to {:?} with length {:?} and buffer {:?}",
            self.username.as_ref().unwrap_or(&"not found".to_string()),
            length,
            to_string(convert(&buffer.array))
        );
    }

    pub fn forward_packet(&self, buffer: Buffer, socket: &UdpSocket) {
        let length =
            futures::executor::block_on(socket.send_to(&buffer.array, self.socketAddr)).unwrap();
        info!(
            "Forwarding packet to {:?} with length {:?} and buffer {:?}",
            self.username.as_ref().unwrap_or(&"not found".to_string()),
            length,
            convert(&buffer.array)
        );
    }

    pub fn send_disconnect(&self, disconnect_packet: DisconnectPacket, socket: &UdpSocket) {
        let mut buffer = Buffer {
            array: Vec::new(),
            position: 0,
        };
        buffer.write_i8(0x09);
        buffer.write_i8(1);
        disconnect_packet.serialize(&mut buffer);

        let length =
            futures::executor::block_on(socket.send_to(&buffer.array, self.socketAddr)).unwrap();
        info!("Sending disconnect packet with length {:?}", length);
    }

    pub fn assign_player(&mut self, player: Player) -> Option<User> {
        // let mut user = self.to_owned();
        // user.unwrap().player = Some(player);
        // update_user(user.unwrap());
        self.player = Some(player);
        info!("NEW USER: {:?}", self.to_owned());
        update_user(self.clone());
        return Some(self.to_owned());
    }
}
