use crate::util::inner::GameCode;
use crate::{Packet, User};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::future::Future;
use tokio::net::UdpSocket;
use tokio::sync::{Mutex, MutexGuard};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct GameRoom {
    pub code: GameCode,
    pub players: HashMap<i32, Option<User>>,
    pub host: i32,
}

impl GameRoom {
    pub fn new(code: GameCode) -> Self {
        let code_option = Some(code);
        let inside = code_option.as_ref();
        let room = Some(GameRoom {
            code: inside.unwrap().clone(),
            players: HashMap::new(),
            host: -1,
        });
        /*ROOMS
        .lock()
        .unwrap()
        .insert(code_option.unwrap(), room.clone());*/
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
            x.as_ref()
                .unwrap()
                .send_reliable_packet(packet.clone(), socket)
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
