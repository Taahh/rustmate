use crate::connections::update_user;
use crate::game_data::game_data::{DataData, GameData, SpawnData};
use crate::inner::rooms::{
    get_rooms, room_exists, update_room, update_room_callback, GameRoom, ROOMS,
};
use crate::manager::user::Player;
use crate::structs::structs::{GameOptionsData, PlatformSpecificData};
use crate::util::hazel::HazelMessage;
use crate::util::inner::GameCode;
use crate::{convert, get_users, Buffer, DisconnectPacket, Packet, User, CONNECTIONS, RUNTIME};
use rand::distributions::{Alphanumeric, DistString, Standard};
use rand::Rng;
use std::borrow::BorrowMut;
use std::sync::Arc;
use std::thread;
use tokio::net::UdpSocket;
use tracing::log::{debug, log};
use tracing::{error, info};
use tracing_subscriber::registry::Data;
use crate::util::util::send_spawn_message;

pub struct HostGamePacket {
    pub code: Option<GameCode>,
}

#[derive(Clone)]
pub struct JoinGamePacket {
    pub code: Option<GameCode>,
    pub joining: Option<User>,
    pub host: Option<i32>,
    pub room: Option<GameRoom>,
}

pub struct JoinedGamePacket {
    pub room: GameRoom,
    pub user: User,
}

#[derive(Clone)]
pub struct GameDataPacket {
    pub code: Option<GameCode>,
    pub buffer: Buffer,
    pub reliable: bool
}

#[derive(Clone)]
pub struct GameDataToPacket {
    pub code: Option<GameCode>,
    pub target: i32,
    pub buffer: Buffer,
}

#[derive(Clone)]
pub struct StartGamePacket {
    pub code: Option<GameCode>,
    pub buffer: Buffer,
}

pub struct ReactorHandshakePacket;

impl Packet for ReactorHandshakePacket {
    fn deserialize(&mut self, buffer: &mut Buffer) {}

    fn serialize(self, buffer: &mut Buffer) {
        let mut hazel_message = HazelMessage::start_message(255);
        hazel_message.buffer.write_i8(0);
        hazel_message.buffer.write_string("Hydrogen".to_string());
        hazel_message.buffer.write_string("0.0.1".to_string());
        hazel_message.buffer.write_packed_u32(0);
        hazel_message.end_message();
        hazel_message.copy_to(buffer);
    }

    fn process(self, user: &mut &User, socket: &UdpSocket) {}
}

impl Packet for HostGamePacket {
    fn deserialize(&mut self, buffer: &mut Buffer) {
        let pos = buffer.position;
        info!("Hazel: {:?}", &buffer.array[pos..]);
        let settings = GameOptionsData::deserialize(buffer);
        info!("Game Options: {:?}", settings);
        info!("Crossplay Flags: {:?}", buffer.read_i32_le());
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
        /*user.send_disconnect(DisconnectPacket {
            reason: Some("Hi".to_string()),
            disconnect_type: Some(0x08)
        }, socket);*/

        GameRoom::new(self.code.as_ref().unwrap().clone());
        user.send_reliable_packet(self, socket);
    }
}

impl Packet for JoinGamePacket {
    fn deserialize(&mut self, buffer: &mut Buffer) {
        self.code = Some(GameCode::new_code_int(buffer.read_i32_le()));
    }

    fn serialize(self, buffer: &mut Buffer) {
        // info!("join game serialization");
        let mut hazel_message = HazelMessage::start_message(0x01);
        hazel_message
            .buffer
            .write_i32_le(self.room.as_ref().unwrap().code.code_int);
        let user = self.joining.as_ref().unwrap().to_owned();
        let player = user.player.as_ref().unwrap().to_owned();
        hazel_message.buffer.write_i32_le(player.id);
        // info!("player id: {:?}", player.id);
        hazel_message
            .buffer
            .write_i32_le(self.room.as_ref().unwrap().host);
        // info!("room host id: {:?}", self.room.as_ref().unwrap().host);
        hazel_message
            .buffer
            .write_string(user.username.as_ref().unwrap().to_string());
        // info!(
        //     "username: {:?}",
        //     user.username.as_ref().unwrap().to_string()
        // );
        user.platformData
            .as_ref()
            .unwrap()
            .serialize(&mut hazel_message.buffer);
        hazel_message.buffer.write_packed_u32(0);
        hazel_message.buffer.write_string("".to_string());
        hazel_message.buffer.write_string("".to_string());
        hazel_message.end_message();
        hazel_message.copy_to(buffer);
    }

    fn process(self, user: &mut &User, socket: &UdpSocket) {
        if !room_exists(self.code.as_ref().unwrap().to_owned()) {
            info!("Room not found");
            return;
        }
        let mut map = get_rooms();
        let mut room = map
            .get(self.code.as_ref().unwrap())
            .unwrap()
            .as_ref()
            .unwrap()
            .to_owned();
        let next_length = room.players.len() + 1;
        let player_option = Some(Player {
            id: next_length as i32,
            game_code: room.clone().code,
        });

        let mut newRoom = false;

        if room.players.is_empty() {
            room.host = player_option.as_ref().unwrap().to_owned().id;
            newRoom = true;
        }

        let mut user_mut = user.to_owned();
        user_mut.player = player_option.clone();
        let user_option = Some(user_mut.clone());
        let addr = user_option.as_ref().unwrap().socketAddr;
        tokio::spawn(async move {
            CONNECTIONS
                .lock()
                .await
                .get_mut(&addr)
                .unwrap()
                .as_mut()
                .unwrap()
                .player = user_mut.player;
        });

        room.players
            .insert(player_option.clone().unwrap().id, user_option.clone());
        println!("raaaaoom: {:?}", room);
        let room_clone = Some(room.clone());
        map.insert(
            room_clone.as_ref().unwrap().code.clone(),
            room_clone.clone(),
        );

        let code = self.code.as_ref().unwrap().to_owned();
        if !newRoom {
            let mut packet = self.clone();
            packet.host = Some(room_clone.as_ref().unwrap().host);
            packet.joining = user_option.clone();
            packet.room = room_clone.clone();
            println!("new room: {:?}", room_clone);
            room_clone.as_ref().unwrap().send_reliable_to_all_but(
                packet,
                socket,
                &[player_option.as_ref().unwrap().id],
            );
        }
        user.send_reliable_packet(
            JoinedGamePacket {
                room: map.get(&code).unwrap().as_ref().unwrap().to_owned(),
                user: user_option.as_ref().unwrap().to_owned(),
            },
            socket,
        );
        let address = user.socketAddr;
        tokio::spawn(async move {
            let mut users = CONNECTIONS.lock().await;
            let uzer = users.get_mut(&address).as_mut().unwrap().to_owned().unwrap();
            if uzer.player != None {
                let player = uzer.player.as_ref().unwrap().to_owned();
                ROOMS.lock().await.get_mut(&code).unwrap().as_mut().unwrap().players.get_mut(&player.id).unwrap().as_mut().unwrap().serverNonce = uzer.serverNonce;
            }
        });
    }
}

impl Packet for JoinedGamePacket {
    fn deserialize(&mut self, buffer: &mut Buffer) {}

    fn serialize(self, buffer: &mut Buffer) {
        info!("joined game for {:?}", self.user);
        let socketAddr = self.user.socketAddr;
        let mut hazel_message = HazelMessage::start_message(0x07);
        let room = self.room;
        hazel_message.buffer.write_i32_le(room.code.code_int);
        hazel_message
            .buffer
            .write_i32_le(self.user.player.as_ref().unwrap().id);
        // info!("JOINED GAME PLAYER ID {:?}", self.user.player.as_ref().unwrap().id);
        hazel_message.buffer.write_i32_le(room.host);
        if room.players.len() - 1 <= 0 {
            hazel_message.buffer.write_packed_i32(0);
        } else {
            hazel_message
                .buffer
                .write_packed_i32((room.players.len() - 1) as i32);
            for (x, v) in room.players {
                let user = v.as_ref().unwrap().to_owned();
                if user.socketAddr.eq(&self.user.socketAddr) {
                    continue;
                }
                info!(
                    "ADDING {:?} TO JOINED GAME PACKET",
                    user.username.as_ref().unwrap()
                );
                hazel_message.buffer.write_packed_i32(x);
                hazel_message
                    .buffer
                    .write_string(user.username.as_ref().unwrap().to_owned());
                user.platformData
                    .unwrap()
                    .serialize(&mut hazel_message.buffer);
                hazel_message.buffer.write_packed_u32(0);
                hazel_message.buffer.write_string("".to_string());
                hazel_message.buffer.write_string("".to_string());
            }
        }
        hazel_message.end_message();
        hazel_message.copy_to(buffer);
    }

    fn process(self, user: &mut &User, socket: &UdpSocket) {}
}

impl Packet for GameDataPacket {
    fn deserialize(&mut self, buffer: &mut Buffer) {
        self.code = Some(GameCode::new_code_int(buffer.read_i32()));
        info!("Got Game Code: {:?}", self.code);

        let mut hazel_buffer = buffer.clone();

        let room = &mut get_rooms()
            .get_mut(self.code.as_ref().unwrap())
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap()
            .to_owned();

        while hazel_buffer.position < hazel_buffer.array.len() {
            let mut hazel_msg_option = HazelMessage::read_message(buffer);
            if hazel_msg_option == None {
                break;
            }
            let mut msg = hazel_msg_option.as_mut().unwrap();
            // hazel_buffer = hazel_msg_option.as_r.unwrap().buffer;
            hazel_buffer = msg.to_owned().buffer;
            info!("GAME DATA TAG: {:?}", msg.tag);
            match msg.tag {
                0x04 => {
                    // let hazel_clone = hazel_msg.clone().to_owned();
                    // hazel_buffer = hazel_clone.buffer;
                    let mut spawn_data = SpawnData {
                        game_data: None,
                        vote_ban_system: None,
                        player_control: None,
                        lobby_behavior: None,
                    };
                    spawn_data.deserialize(msg);
                    spawn_data.process(room);
                }
                0x01 => {
                    // let hazel_clone = &mut hazel_msg.clone();
                    let mut data_data = DataData {
                        net_id: 0,
                        hazel_msg: HazelMessage {
                            length: 0,
                            tag: 0,
                            buffer: Buffer {
                                position: 0,
                                array: vec![],
                            },
                        },
                    };
                    data_data.deserialize(msg);
                    data_data.process(room);
                }
                _ => {}
            }
        }
    }

    fn serialize(self, buffer: &mut Buffer) {
        let mut arr = self.buffer;
        arr.position = 3;
        buffer.write_u8_arr(&arr.array[arr.position..]);
    }

    fn process(self, user: &mut &User, socket: &UdpSocket) {
        if !room_exists(self.code.as_ref().unwrap().to_owned()) {
            info!("Room not found");
            return;
        }

        // info!("game data user: {:?}", user);

        if user.player == None {
            error!("User for some reason lacks a player object!");
            return;
        }

        let mut room = Some(get_rooms()
            .get(&user.player.as_ref().unwrap().game_code)
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap()
            .to_owned());

        // info!("HELLO GAME DATA");

        let mut packet = self.to_owned();

        // let addr = user.socketAddr;
        if packet.reliable {
            room.as_mut().unwrap().send_reliable_to_all(
                packet,
                socket, /*, &[user.player.as_ref().unwrap().id]*/
            );
        } else {
            let mut buffer = Buffer {
                position: 0,
                array: Vec::new()
            };
            buffer.write_u8(0);
            // packet.serialize(&mut buffer);
            let mut buffer_two = self.buffer.clone();
            buffer_two.position = 1;
            buffer.write_u8_arr(&buffer_two.array[buffer_two.position..]);
            room.as_mut().unwrap().forward_packet_to_all(
                buffer,
                socket, /*, &[user.player.as_ref().unwrap().id]*/
            );
        }
        // get_users();
        // let room = room.clone();
        let id = user.player.as_ref().unwrap().id;
        if id != room.as_ref().unwrap().host {
            // get_users();
            let room = room.as_ref().unwrap().to_owned();
            let mut user = room.players.get(&id).as_ref().unwrap().to_owned().to_owned();
            let user_mut = user.as_mut().unwrap();
            // send_spawn_message(user_mut, socket, room);
        }
        /*
        let socketAddr = user.socketAddr;
        // gameRoom.unwrap().unwrap().players.*/
    }
}

impl Packet for GameDataToPacket {
    fn deserialize(&mut self, buffer: &mut Buffer) {
        self.code = Some(GameCode::new_code_int(buffer.read_i32()));
        info!("Got Game Code: {:?}", self.code);
        self.target = buffer.read_packed_int_32();

        let mut hazel_buffer = buffer.clone();

        let room = &mut get_rooms()
            .get_mut(self.code.as_ref().unwrap())
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap()
            .to_owned();

        while hazel_buffer.position < hazel_buffer.array.len() {
            let mut hazel_msg_option = HazelMessage::read_message(buffer);
            if hazel_msg_option == None {
                break;
            }
            let mut msg = hazel_msg_option.as_mut().unwrap();
            // hazel_buffer = hazel_msg_option.as_r.unwrap().buffer;
            hazel_buffer = msg.to_owned().buffer;
            info!("GAME DATA TAG: {:?}", msg.tag);
            match msg.tag {
                0x04 => {
                    // let hazel_clone = hazel_msg.clone().to_owned();
                    // hazel_buffer = hazel_clone.buffer;
                    let mut spawn_data = SpawnData {
                        game_data: None,
                        vote_ban_system: None,
                        player_control: None,
                        lobby_behavior: None,
                    };
                    spawn_data.deserialize(msg);
                    spawn_data.process(room);
                }
                0x01 => {
                    // let hazel_clone = &mut hazel_msg.clone();
                    let mut data_data = DataData {
                        net_id: 0,
                        hazel_msg: HazelMessage {
                            length: 0,
                            tag: 0,
                            buffer: Buffer {
                                position: 0,
                                array: vec![],
                            },
                        },
                    };
                    data_data.deserialize(msg);
                    data_data.process(room);
                }
                _ => {}
            }
        }
    }

    fn serialize(self, buffer: &mut Buffer) {
        let mut arr = self.buffer;
        arr.position = 3;
        buffer.write_u8_arr(&arr.array[arr.position..]);
    }

    fn process(self, user: &mut &User, socket: &UdpSocket) {
        if !room_exists(self.code.as_ref().unwrap().to_owned()) {
            info!("Room not found");
            return;
        }

        if user.player == None {
            error!("User for some reason lacks a player object!");
            return;
        }

        let mut room = Some(get_rooms()
            .get(&user.player.as_ref().unwrap().game_code)
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap()
            .to_owned());

        // info!("HELLO GAME DATA");

        info!("game data to process");
        // let addr = user.socketAddr;
        let packet = self.to_owned();
        room.as_mut().unwrap().send_reliable_to(
            packet,
            socket, /*, &[user.player.as_ref().unwrap().id]*/
            self.target
        );
        /*
        let socketAddr = user.socketAddr;
        // gameRoom.unwrap().unwrap().players.*/
    }
}


impl Packet for StartGamePacket {
    fn deserialize(&mut self, buffer: &mut Buffer) {
        self.code = Some(GameCode::new_code_int(buffer.read_i32()));
    }

    fn serialize(self, buffer: &mut Buffer) {
        let mut arr = self.buffer;
        arr.position = 3;
        buffer.write_u8_arr(&arr.array[arr.position..]);
    }

    fn process(self, user: &mut &User, socket: &UdpSocket) {
        if !room_exists(self.code.as_ref().unwrap().to_owned()) {
            info!("Room not found");
            return;
        }

        if user.player == None {
            error!("User for some reason lacks a player object!");
            return;
        }

        let mut room = Some(get_rooms()
            .get(&user.player.as_ref().unwrap().game_code)
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap()
            .to_owned());

        // info!("HELLO GAME DATA");

        info!("game data to process");
        // let addr = user.socketAddr;
        let packet = self.to_owned();
        room.as_mut().unwrap().send_reliable_to_all(
            packet,
            socket
        );
        /*
        let socketAddr = user.socketAddr;
        // gameRoom.unwrap().unwrap().players.*/
    }
}