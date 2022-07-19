use crate::inner::rooms::{get_rooms, GameRoom, ROOMS};
use crate::inner::structs::player::PlayerInfo;
use crate::util::hazel::HazelMessage;
use crate::Buffer;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::DerefMut;
use tracing::info;
use crate::util::util::sid_greater_than;
use crate::util::vector::Vector2;

pub trait InnerNetObject {
    fn deserialize(&mut self, hazel_msg: &mut HazelMessage);
    fn process(&mut self, room: &mut GameRoom);
    fn serialize(&self, buffer: &mut Buffer);
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameData {
    pub net_id: u32,
    pub owner_id: i32,
    pub initial_spawn: bool,
    pub all_players: HashMap<u8, PlayerInfo>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VoteBanSystem {
    pub net_id: u32,
    pub owner_id: i32,
    pub initial_spawn: bool,
    pub votes: HashMap<i32, Vec<i32>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayerControl {
    pub net_id: u32,
    pub owner_id: i32,
    pub initial_spawn: bool,
    pub is_new: bool,
    pub player_id: u8,
    pub player_physics: Option<PlayerPhysics>,
    pub custom_network_transform: Option<CustomNetworkTransform>
}

#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub struct LobbyBehavior {
    pub net_id: u32,
    pub owner_id: i32,
    pub initial_spawn: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub struct PlayerPhysics {
    pub net_id: u32,
    pub initial_spawn: bool,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct CustomNetworkTransform {
    pub net_id: u32,
    pub initial_spawn: bool,
    pub last_sequence_id: u16,
    pub position: Vector2,
    pub velocity: Vector2
}

impl InnerNetObject for GameData {
    fn deserialize(&mut self, hazel_msg: &mut HazelMessage) {
        info!("hazel length: {:?}", hazel_msg.length);
        while hazel_msg.buffer.position < hazel_msg.length as usize {
            let mut hazel_msg_option = HazelMessage::read_message(&mut hazel_msg.buffer);
            if hazel_msg_option == None {
                break;
            }
            let mut hazel_msg_actual = hazel_msg_option.unwrap();
            info!("Player Tag: {:?}", hazel_msg_actual.tag);
            let tag = &hazel_msg_actual.tag;
            if self.all_players.contains_key(tag) {
                self.all_players
                    .get_mut(tag)
                    .unwrap()
                    .deserialize(&mut hazel_msg_actual.buffer);
            } else {
                let mut info = PlayerInfo::new();
                info.deserialize(&mut hazel_msg_actual.buffer);
                self.all_players.insert(*tag, info);
            }
        }
    }

    fn process(&mut self, room: &mut GameRoom) {
        // room.game_data = self.to_owned();
        let mut game_data = self.to_owned();
        let code = room.to_owned().code;
        // get_rooms().get_mut(&code).unwrap().as_mut().unwrap().game_data = Some(game_data);

        if self.initial_spawn {
            game_data.initial_spawn = false;
            get_rooms()
                .get_mut(&code)
                .unwrap()
                .as_mut()
                .unwrap()
                .game_data = Some(game_data);
        } else {
            tokio::spawn(async move {
                ROOMS
                    .lock()
                    .await
                    .get_mut(&code)
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .game_data = Some(game_data);
                println!("game data done");
            });
        }
    }

    fn serialize(&self, buffer: &mut Buffer) {}
}

impl InnerNetObject for VoteBanSystem {
    fn deserialize(&mut self, hazel_msg: &mut HazelMessage) {
        let vote_count = hazel_msg.buffer.read_i8();
        for i in 0..vote_count {
            let key = hazel_msg.buffer.read_i32();
            if !self.votes.contains_key(&key) {
                self.votes.insert(key, Vec::with_capacity(3));
            }
            for i in 0..3 {
                self.votes.get_mut(&key).unwrap()[i] = hazel_msg.buffer.read_packed_int_32();
            }
        }
    }

    fn process(&mut self, room: &mut GameRoom) {
        // room.game_data = self.to_owned();
        let mut vote_ban_system = self.to_owned();
        let code = room.to_owned().code;
        if self.initial_spawn {
            vote_ban_system.initial_spawn = false;
            get_rooms()
                .get_mut(&code)
                .unwrap()
                .as_mut()
                .unwrap()
                .vote_ban_system = Some(vote_ban_system);
        } else {
            tokio::spawn(async move {
                ROOMS
                    .lock()
                    .await
                    .get_mut(&code)
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .vote_ban_system = Some(vote_ban_system);
            });
        }
    }

    fn serialize(&self, buffer: &mut Buffer) {}
}

impl InnerNetObject for PlayerControl {
    fn deserialize(&mut self, hazel_msg: &mut HazelMessage) {
        if self.initial_spawn {
            self.is_new = hazel_msg.buffer.read_bool();
        }
        self.player_id = hazel_msg.buffer.read_u8();
    }

    fn process(&mut self, room: &mut GameRoom) {
        let mut player_control = self.to_owned();
        let id = player_control.player_id;
        println!("control id: {:?}", id);
        let code = room.to_owned().code;
        let mut rooms = get_rooms();
        let room = rooms.get_mut(&code).unwrap().as_mut().unwrap();
        println!("room: {:?}", room);
        if room
            .game_data
            .as_mut()
            .unwrap()
            .all_players
            .contains_key(&id)
        {
            player_control.initial_spawn = false;
            room.game_data
                .as_mut()
                .unwrap()
                .all_players
                .get_mut(&id)
                .unwrap()
                .player_control = Some(player_control);
        } else {
            player_control.initial_spawn = false;
            room.game_data.as_mut().unwrap().all_players.insert(
                id,
                PlayerInfo {
                    outfits: HashMap::new(),
                    level: 0,
                    disconnected: false,
                    dead: false,
                    player_control: Some(player_control),
                },
            );
        }

        println!("UPDATED ROOM");
    }

    fn serialize(&self, buffer: &mut Buffer) {}
}

impl InnerNetObject for PlayerPhysics {
    fn deserialize(&mut self, hazel_msg: &mut HazelMessage) {}

    fn process(&mut self, room: &mut GameRoom) {
        let mut player_physics = self.to_owned();
        let code = room.to_owned().code;
        let mut rooms = get_rooms();
        let room = rooms.get_mut(&code).unwrap().as_mut().unwrap();
        let game_data = room.game_data.as_mut().unwrap();
        for (k, v) in game_data.to_owned().all_players {
            if v.player_control.as_ref().unwrap().player_physics != None && v.player_control.as_ref().unwrap().player_physics.as_ref().unwrap().net_id == self.net_id {
                player_physics.initial_spawn = false;
                game_data.all_players.get_mut(&v.player_control.as_ref().unwrap().player_id).unwrap().player_control.as_mut().unwrap().player_physics = Some(player_physics);
            }
        }

        println!("UPDATED ROOM");
    }

    fn serialize(&self, buffer: &mut Buffer) {}
}

impl InnerNetObject for CustomNetworkTransform {
    fn deserialize(&mut self, hazel_msg: &mut HazelMessage) {
        if self.initial_spawn {
            self.last_sequence_id = hazel_msg.buffer.read_u16();
            self.position = Vector2::read_vector2(&mut hazel_msg.buffer);
            self.velocity = Vector2::read_vector2(&mut hazel_msg.buffer);
        } else {
            let newSid = hazel_msg.buffer.read_u16();
            if !sid_greater_than(newSid, self.last_sequence_id) {
                return;
            }
            self.last_sequence_id = newSid;
            self.position = Vector2::read_vector2(&mut hazel_msg.buffer);
            self.velocity = Vector2::read_vector2(&mut hazel_msg.buffer);
        }
    }

    fn process(&mut self, room: &mut GameRoom) {
        let mut custom_network_transform = self.to_owned();
        let net_id = self.net_id;
        let code = room.to_owned().code;
        tokio::spawn(async move {
            let mut rooms = get_rooms();
            let room = rooms.get_mut(&code).unwrap().as_mut().unwrap();
            let game_data = room.game_data.as_mut().unwrap();
            for (k, v) in game_data.to_owned().all_players {
                if v.player_control.as_ref().unwrap().player_physics != None && v.player_control.as_ref().unwrap().custom_network_transform.as_ref().unwrap().net_id == net_id {
                    custom_network_transform.initial_spawn = false;
                    game_data.all_players.get_mut(&v.player_control.as_ref().unwrap().player_id).unwrap().player_control.as_mut().unwrap().custom_network_transform = Some(custom_network_transform);
                }
            }
        });

        println!("UPDATED ROOM");
    }

    fn serialize(&self, buffer: &mut Buffer) {}
}

impl InnerNetObject for LobbyBehavior {
    fn deserialize(&mut self, hazel_msg: &mut HazelMessage) {}

    fn process(&mut self, room: &mut GameRoom) {
        let mut lobby_behavior = self.to_owned();
        let code = room.to_owned().code;
        if self.initial_spawn {
            lobby_behavior.initial_spawn = false;
            get_rooms()
                .get_mut(&code)
                .unwrap()
                .as_mut()
                .unwrap()
                .lobby_behavior = Some(lobby_behavior);
        } else {
            tokio::spawn(async move {
                ROOMS
                    .lock()
                    .await
                    .get_mut(&code)
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .lobby_behavior = Some(lobby_behavior);
            });
        }
    }

    fn serialize(&self, buffer: &mut Buffer) {}
}
