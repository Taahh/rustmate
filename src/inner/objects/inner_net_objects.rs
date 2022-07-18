use crate::inner::rooms::{get_rooms, GameRoom, ROOMS};
use crate::inner::structs::player::PlayerInfo;
use crate::util::hazel::HazelMessage;
use crate::Buffer;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::DerefMut;
use tracing::info;

pub trait InnerNetObject {
    fn deserialize(&mut self, hazel_msg: &mut HazelMessage);
    fn process(&mut self, room: &mut GameRoom);
    fn serialize(&self, buffer: &mut Buffer);
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameData {
    pub net_id: u32,
    pub initial_spawn: bool,
    pub all_players: HashMap<u8, PlayerInfo>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VoteBanSystem {
    pub net_id: u32,
    pub initial_spawn: bool,
    pub votes: HashMap<i32, Vec<i32>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayerControl {
    pub net_id: u32,
    pub initial_spawn: bool,
    pub is_new: bool,
    pub player_id: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayerPhysics {
    pub net_id: u32,
    pub initial_spawn: bool,
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
            room.game_data.as_mut().unwrap().all_players.insert(
                id,
                PlayerInfo {
                    outfits: HashMap::new(),
                    level: 0,
                    disconnected: false,
                    dead: false,
                    player_control: Some(player_control),
                    player_physics: None,
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
        /*for (x, v) in room.game_data.as_mut().unwrap().all_players {
            if v.player_control != None {
                println!("Skipping");
                return;
            }
            if v.player_control.as_ref().unwrap().net_id =
        }*/
        /*println!("room: {:?}", room);
        if room.game_data.as_mut().unwrap().all_players.contains_key(&id) {
            player_control.initial_spawn = false;
            room.game_data.as_mut().unwrap().all_players.get_mut(&id).unwrap().player_control = Some(player_control);
        } else {
            room.game_data.as_mut().unwrap().all_players.insert(id, PlayerInfo {
                outfits: HashMap::new(),
                level: 0,
                disconnected: false,
                dead: false,
                player_control: Some(player_control)
            });
        }*/

        println!("UPDATED ROOM");
    }

    fn serialize(&self, buffer: &mut Buffer) {}
}
