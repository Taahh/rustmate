use crate::inner::objects::inner_net_objects::{GameData, LobbyBehavior, VoteBanSystem};
use crate::util::inner::GameCode;
use crate::{Buffer, Packet, User};
use lazy_static::lazy_static;
use std::collections::{BTreeMap, HashMap};
use std::future::Future;
use std::marker::PhantomData;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::{Mutex, MutexGuard};
use tracing::info;

#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    NotStarted,
    WaitingForHost,
    InProgress
}

#[derive(Debug, Clone)]
pub struct GameRoom {
    pub state: GameState,
    pub code: GameCode,
    pub players: HashMap<i32, Option<User>>,
    pub game_data: Option<GameData>,
    pub vote_ban_system: Option<VoteBanSystem>,
    pub lobby_behavior: Option<LobbyBehavior>,
    pub host: i32,
    pub waiting_for_host: Vec<i32>
}

impl GameRoom {
    pub fn new(code: GameCode) -> Self {
        let code_option = Some(code);
        let inside = code_option.as_ref();
        let room = Some(GameRoom {
            state: GameState::NotStarted,
            code: inside.unwrap().clone(),
            players: HashMap::new(),
            game_data: None,
            vote_ban_system: None,
            lobby_behavior: None,
            host: -1,
            waiting_for_host: Vec::new()
        });
        get_rooms().insert(code_option.unwrap(), room.to_owned());
        return room.unwrap();
    }

    pub fn send_reliable_to_all(&self, packet: impl Packet + Clone, socket: &UdpSocket) {
        for x in self.players.values() {
            x.as_ref()
                .unwrap()
                .send_reliable_packet(packet.clone(), socket)
        }
    }

    pub fn send_reliable_to(&self, packet: impl Packet + Clone, socket: &UdpSocket, target: i32) {
        self.players.get(&target).as_ref().unwrap().as_ref().unwrap().send_reliable_packet(packet.clone(), socket);
    }

    pub fn send_reliable_to_all_but(
        &self,
        packet: impl Packet + Clone,
        socket: &UdpSocket,
        exclude: &[i32],
    ) {
        for x in self.players.values() {
            if x.as_ref().unwrap().player == None {
                continue;
            }
            if exclude.contains(&x.as_ref().unwrap().player.as_ref().unwrap().id) {
                continue;
            }
            info!("NOT SKIPPING: {:?}", x.as_ref().unwrap().username);
            x.as_ref()
                .unwrap()
                .send_reliable_packet(packet.clone(), socket)
        }
    }

    pub fn forward_packet_to_all(&self, buffer: Buffer, socket: &UdpSocket) {
        for x in self.players.values() {
            x.as_ref().unwrap().forward_packet(buffer.clone(), socket)
        }
    }

    pub fn forward_packet_to(&self, buffer: Buffer, socket: &UdpSocket, target: i32) {
        self.players.get(&target).as_ref().unwrap().as_ref().unwrap().forward_packet(buffer.clone(), socket)
    }

    pub fn forward_packet_to_all_but(&self, buffer: Buffer, socket: &UdpSocket, exclude: &[i32]) {
        for x in self.players.values() {
            if x.as_ref().unwrap().player == None {
                continue;
            }
            if exclude.contains(&x.as_ref().unwrap().player.as_ref().unwrap().id) {
                continue;
            }
            info!("NOT SKIPPING: {:?}", x.as_ref().unwrap().username);
            x.as_ref().unwrap().forward_packet(buffer.clone(), socket)
        }
    }
}

lazy_static! {
    pub static ref ROOMS: Mutex<HashMap<GameCode, Option<GameRoom>>> = {
        let mut m = HashMap::new();
        return Mutex::new(m);
    };
}

pub fn get_rooms() -> MutexGuard<'static, HashMap<GameCode, Option<GameRoom>>> {
    return futures::executor::block_on(ROOMS.lock());
}

pub fn room_exists(code: GameCode) -> bool {
    let map = futures::executor::block_on(ROOMS.lock());
    return map
        .keys()
        .any(|f| f.code_string.eq_ignore_ascii_case(&code.code_string));
}

pub fn update_room(room: GameRoom) {
    let room_clone = room.clone();
    tokio::spawn(async move {
        ROOMS.lock().await.insert(room.code, Some(room_clone));
    });
}

pub fn update_room_callback<T>(room: GameRoom, future: T)
where
    T: Future + Send + 'static,
    T::Output: Send + 'static,
{
    let room_clone = room.clone();
    tokio::spawn(async move {
        ROOMS.lock().await.insert(room.code, Some(room_clone));
        tokio::spawn(future);
    });
}
