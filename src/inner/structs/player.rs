use crate::inner::objects::inner_net_objects::{PlayerControl, PlayerPhysics};
use crate::Buffer;
use std::collections::HashMap;
use std::mem::transmute;
use tracing::log::info;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum PlayerOutfitType {
    Default = 0,
    Shapeshifted = 1,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayerInfo {
    pub outfits: HashMap<PlayerOutfitType, PlayerOutfit>,
    pub level: u32,
    pub disconnected: bool,
    pub dead: bool,
    pub player_control: Option<PlayerControl>
}

// #[derive(Clone, Debug)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PlayerOutfit {
    pub preCensorName: String,
    pub color_id: i32,
    pub hat_id: String,
    pub pet_id: String,
    pub skin_id: String,
    pub visor_id: String,
    pub name_plate_id: String,
}

impl PlayerOutfit {
    pub fn deserialize(buffer: &mut Buffer) -> Self {
        PlayerOutfit {
            preCensorName: buffer.read_string(),
            color_id: buffer.read_packed_int_32(),
            hat_id: buffer.read_string(),
            pet_id: buffer.read_string(),
            skin_id: buffer.read_string(),
            visor_id: buffer.read_string(),
            name_plate_id: buffer.read_string(),
        }
    }

    pub fn serialize(&self, buffer: &mut Buffer) {
        let me = self.clone();
        buffer.write_string(me.preCensorName);
        buffer.write_packed_i32(me.color_id);
        buffer.write_string(me.hat_id);
        buffer.write_string(me.pet_id);
        buffer.write_string(me.skin_id);
        buffer.write_string(me.visor_id);
        buffer.write_string(me.name_plate_id);
    }
}

impl PlayerInfo {
    pub fn new() -> Self {
        PlayerInfo {
            outfits: HashMap::new(),
            level: 0,
            disconnected: false,
            dead: false,
            player_control: None
        }
    }
    pub fn deserialize(&mut self, buffer: &mut Buffer) {
        let byte = buffer.read_i8_le();
        println!("outfits: {:?}", byte);
        self.outfits.clear();
        for i in 0..byte {
            let outfit_type: PlayerOutfitType = unsafe { transmute(buffer.read_i8()) };
            println!("outfit_type: {:?}", outfit_type);
            println!("buffer: {:?}", buffer);
            let outfit = PlayerOutfit::deserialize(buffer);
            println!("OUTFIT: {:?}", outfit);
            self.outfits.insert(outfit_type, outfit);
        }
        self.level = buffer.read_packed_uint_32();
        let flags = buffer.read_i8_le();
        self.disconnected = (flags & 1) != 0;
        self.dead = (flags & 4) != 0;
        let tasks = buffer.read_i8();
        for i in 0..tasks {
            buffer.read_packed_uint_32();
            buffer.read_bool();
        }
        buffer.read_string();
        buffer.read_string();
    }
}
