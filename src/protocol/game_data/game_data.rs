use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use tracing::info;
use crate::{Buffer, inner};
use crate::inner::objects::inner_net_objects::{InnerNetObject, PlayerControl};
use crate::inner::rooms::{GameRoom, get_rooms, ROOMS};
use crate::util::hazel::HazelMessage;

type InnerType = &'static (dyn InnerNetObject + Sync);

#[derive(Clone)]
pub struct SpawnData {
    pub game_data: Option<inner::objects::inner_net_objects::GameData>,
    pub vote_ban_system: Option<inner::objects::inner_net_objects::VoteBanSystem>,
    pub player_control: Option<PlayerControl>,
}

#[derive(Clone)]
pub struct DataData {
    pub net_id: u32,
    pub hazel_msg: HazelMessage,
}

pub trait GameData {
    // type InnerType;
    fn deserialize(&mut self, hazel_msg: &mut HazelMessage);
    fn process(&mut self, room: &mut GameRoom);
    fn serialize(&self, buffer: &mut Buffer);
}

impl GameData for SpawnData {
    // type InnerType = impl InnerNetObject;

    fn deserialize(&mut self, hazel_msg: &mut HazelMessage) {
        let spawn_id = hazel_msg.buffer.read_packed_uint_32();
        let owner_id = hazel_msg.buffer.read_packed_int_32();
        let flags = hazel_msg.buffer.read_i8_le();
        let components = hazel_msg.buffer.read_packed_int_32();
        println!("-----------------------------------------");
        for i in 0..components {
            let net_id = hazel_msg.buffer.read_packed_uint_32();
            println!("net id: {:?}", net_id);
            let hazel_inner = HazelMessage::read_message(&mut hazel_msg.buffer);
            if hazel_inner != None {
                println!("length: {:?}", hazel_inner.as_ref().unwrap().length);
                println!("tag: {:?}", hazel_inner.as_ref().unwrap().tag);
                match spawn_id {
                    3 => {
                        println!("Spawning InnerNetObject: GameData");
                        if i == 0 {
                            let mut game_data = inner::objects::inner_net_objects::GameData {
                                net_id,
                                initial_spawn: true,
                                all_players: HashMap::new(),
                            };
                            info!("Game Data Deserialized Net ID: {:?}", game_data.net_id);
                            if hazel_inner.as_ref().unwrap().length > 0 {
                                game_data.deserialize(&mut hazel_inner.unwrap());
                            }
                            self.game_data = Some(game_data);
                        } else {
                            let mut vote_ban_system = inner::objects::inner_net_objects::VoteBanSystem {
                                net_id,
                                initial_spawn: true,
                                votes: HashMap::new(),
                            };
                            info!("Vote Ban System Deserialized Net ID: {:?}", vote_ban_system.net_id);
                            if hazel_inner.as_ref().unwrap().length > 0 {
                                vote_ban_system.deserialize(&mut hazel_inner.unwrap());
                            }
                            self.vote_ban_system = Some(vote_ban_system);
                        }
                    }
                    4 => {
                        if i == 0 {
                            let mut player_control = PlayerControl {
                                net_id,
                                initial_spawn: true,
                                is_new: false,
                                player_id: 0,
                            };
                            info!("Player Control Deserialized Net ID: {:?}", player_control.net_id);
                            if hazel_inner.as_ref().unwrap().length > 0 {
                                player_control.deserialize(&mut hazel_inner.unwrap());
                            }
                            self.player_control = Some(player_control);
                        }
                    }
                    _ => {}
                }
            }
            println!("-----------");
        }

        println!("Spawn ID {:?}; Owner ID {:?}; Flags {:?}; Components {:?}", spawn_id, owner_id, flags, components);
        println!("-----------------------------------------");
    }

    fn process(&mut self, room: &mut GameRoom) {
        if self.game_data != None {
            self.game_data.as_ref().unwrap().to_owned().process(room);
        }
        if self.vote_ban_system != None {
            self.vote_ban_system.as_ref().unwrap().to_owned().process(room);
        }
        if self.player_control != None {
            self.player_control.as_ref().unwrap().to_owned().process(room);
        }
    }

    fn serialize(&self, buffer: &mut Buffer) {}
}

unsafe impl Sync for SpawnData {}

impl GameData for DataData {
    fn deserialize(&mut self, hazel_msg: &mut HazelMessage) {
        let net_id = hazel_msg.buffer.read_packed_uint_32();
        self.net_id = net_id;
        self.hazel_msg = hazel_msg.to_owned();
        println!("Net ID: {:?}", net_id);
    }

    fn process(&mut self, room: &mut GameRoom) {
        let code = room.to_owned().code;
        let mut hazel_msg = self.clone().hazel_msg;
        let net_id = self.net_id;
        tokio::spawn(async move {
            let mut rooms = get_rooms();
            let mut room = rooms.get_mut(&code).unwrap().as_mut().unwrap();
            println!("room: {:?}", room);
            if net_id == room.game_data.as_ref().unwrap().net_id {
                println!("NET ID FROM DATA MATCHES GAME DATA");
                let mut game_data = room.game_data.as_ref().unwrap().to_owned();
                if hazel_msg.length > 0 {
                    game_data.deserialize(&mut hazel_msg);
                }
                game_data.process(room);
            } else {
                let players = room.game_data.as_ref().unwrap().to_owned().all_players;
                for (k, mut v) in players {
                    if v.player_control != None {
                        let player_control = v.player_control.as_mut().unwrap();
                        if player_control.net_id == net_id {
                            info!("UPDATING PLAYER CONTROL");
                            if hazel_msg.length > 0 {
                                player_control.deserialize(&mut hazel_msg);
                            }
                            player_control.process(room);
                        }
                    }
                }
                /*room.game_data.as_ref().unwrap().all_players.values().filter(|p| p.player_control != None && p.player_control.as_ref().unwrap().net_id == net_id).for_each(|p| {
                    info!("UPDATING PLAYER CONTROL");
                    let player_control = room.game_data.as_mut().unwrap().all_players.get_mut(&p.player_control.as_ref().unwrap().player_id).unwrap().player_control.as_mut().unwrap();
                    if hazel_msg.length > 0 {
                        player_control.deserialize(&mut hazel_msg);
                    }
                    player_control.process(&mut room);
                });*/
            }
            /*if net_id == room.vote_ban_system.as_ref().unwrap().net_id {
                println!("NET ID FROM DATA MATCHES VOTE BAN SYSTEM");
                let mut vote_ban_system = room.vote_ban_system.as_ref().unwrap().to_owned();
                if hazel_msg.length > 0 {
                    vote_ban_system.deserialize(&mut hazel_msg);
                }
                vote_ban_system.process(&mut room);
            }*/
        });
    }

    fn serialize(&self, buffer: &mut Buffer) {}
}