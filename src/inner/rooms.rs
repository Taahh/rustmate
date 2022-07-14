use crate::util::inner::GameCode;
use crate::User;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct GameRoom {
    code: GameCode,
    players: HashMap<i8, Option<User>>,
}

impl GameRoom {
    pub fn new(code: GameCode) -> Self {
        let code_option = Some(code);
        let inside = code_option.as_ref();
        let room = Some(GameRoom {
            code: inside.unwrap().clone(),
            players: HashMap::new(),
        });
        ROOMS
            .lock()
            .unwrap()
            .insert(code_option.unwrap(), room.clone());
        return room.unwrap();
    }
}

lazy_static! {
    pub static ref ROOMS: Mutex<HashMap<GameCode, Option<GameRoom>>> = {
        let mut m = HashMap::new();
        return Mutex::new(m);
    };
}

pub fn room_exists(code: GameCode) -> bool {
    return ROOMS
        .lock()
        .unwrap()
        .keys()
        .any(|f| f.code_string.eq_ignore_ascii_case(&code.code_string));
}
