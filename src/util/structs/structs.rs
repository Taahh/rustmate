use crate::structs::enums::RoleType;
use crate::util::buffer::Buffer;
use crate::util::hazel::HazelMessage;
use std::collections::HashMap;
use tracing::log::info;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct PlatformSpecificData {
    pub platform: u8,
    pub platformName: String,
}

#[derive(Debug)]
pub struct RoleRate {
    pub max_count: i32,
    pub chance: i32,
}

#[derive(Debug)]
pub struct GameOptionsData {
    pub version: i8,
    pub max_players: i8,
    pub keywords: u32,
    pub map: i8,
    pub speed_mod: f32,
    pub crew_light: f32,
    pub imposter_light: f32,
    pub kill_cooldown: f32,
    pub common_tasks: i8,
    pub long_tasks: i8,
    pub short_tasks: i8,
    pub emergency_meetings: i32,
    pub imposters: i8,
    pub kill_distance: i8,
    pub discussion_time: i32,
    pub voting_time: i32,
    pub default: bool,
    pub emergency_cooldown: Option<i8>,
    pub confirm_ejects: Option<bool>,
    pub visual_tasks: Option<bool>,
    pub anonymous_votes: Option<bool>,
    pub taskbar_mode: Option<i8>,
    pub roleOptions: Option<RoleOptionsData>,
}

#[derive(Debug)]
pub struct RoleOptionsData {
    pub roles: i32,
    pub roleRates: HashMap<RoleType, RoleRate>,
    pub shapeshifter_leave_skin: bool,
    pub shapeshifter_cooldown: i8,
    pub shapeshifter_duration: i8,
    pub scientist_cooldown: i8,
    pub guardian_angel_cooldown: i8,
    pub engineer_cooldown: i8,
    pub engineer_max_vent_time: i8,
    pub scientist_battery_charge: i8,
    pub protection_duration_seconds: i8,
    pub imposters_see_protect: bool,
}

impl PlatformSpecificData {
    pub fn serialize(&self, buffer: &mut Buffer) {
        let mut hazel_msg = HazelMessage::start_message(self.platform);
        let platformName = &self.platformName;
        hazel_msg.buffer.write_string(platformName.to_string());
        hazel_msg.end_message();
        hazel_msg.copy_to(buffer);
    }
}

impl GameOptionsData {
    pub fn deserialize(buffer: &mut Buffer) -> Self {
        buffer.read_i8();
        let version = buffer.read_i8();
        let max_players = buffer.read_i8();
        let keywords = buffer.read_u32();
        let map = buffer.read_i8();
        let speed_mod = buffer.read_f32();
        let crew_light = buffer.read_f32();
        let imposter_light = buffer.read_f32();
        let kill_cooldown = buffer.read_f32();
        let common_tasks = buffer.read_i8();
        let long_tasks = buffer.read_i8();
        let short_tasks = buffer.read_i8();
        let emergency_meetings = buffer.read_i32();
        let imposters = buffer.read_i8();
        let kill_distance = buffer.read_i8();
        let discussion_time = buffer.read_i32();
        let voting_time = buffer.read_i32();
        let default = buffer.read_bool();
        let mut emergency_cooldown: Option<i8> = None;
        let mut confirm_ejects: Option<bool> = None;
        let mut visual_tasks: Option<bool> = None;
        let mut anonymous_votes: Option<bool> = None;
        let mut taskbar_mode: Option<i8> = None;
        let mut roleOptions: Option<RoleOptionsData> = None;
        println!("Version: {:?}", version);
        if version > 1 {
            emergency_cooldown = Some(buffer.read_i8());
        };
        if version > 2 {
            confirm_ejects = Some(buffer.read_bool());
            visual_tasks = Some(buffer.read_bool());
        };
        if version > 3 {
            anonymous_votes = Some(buffer.read_bool());
            taskbar_mode = Some(buffer.read_i8());
        };
        if version > 4 {
            roleOptions = Some(RoleOptionsData::deserialize(buffer));
        };

        GameOptionsData {
            version,
            max_players,
            keywords,
            map,
            speed_mod,
            crew_light,
            imposter_light,
            kill_cooldown,
            common_tasks,
            long_tasks,
            short_tasks,
            emergency_cooldown,
            imposters,
            kill_distance,
            discussion_time,
            voting_time,
            default,
            emergency_meetings,
            confirm_ejects,
            visual_tasks,
            anonymous_votes,
            taskbar_mode,
            roleOptions,
        }
    }
}

impl RoleOptionsData {
    pub fn deserialize(buffer: &mut Buffer) -> Self {
        let length = buffer.read_i32();
        let mut roleRates: HashMap<RoleType, RoleRate> = HashMap::new();
        for i in 0..length {
            let role_type: RoleType = unsafe { std::mem::transmute(buffer.read_i16()) };
            let roleRate = RoleRate {
                max_count: buffer.read_i8() as i32,
                chance: buffer.read_i8() as i32,
            };
            roleRates.insert(role_type, roleRate);
        }
        RoleOptionsData {
            roles: length,
            roleRates,
            shapeshifter_leave_skin: buffer.read_bool(),
            shapeshifter_cooldown: buffer.read_i8(),
            shapeshifter_duration: buffer.read_i8(),
            scientist_cooldown: buffer.read_i8(),
            guardian_angel_cooldown: buffer.read_i8(),
            engineer_cooldown: buffer.read_i8(),
            engineer_max_vent_time: buffer.read_i8(),
            scientist_battery_charge: buffer.read_i8(),
            protection_duration_seconds: buffer.read_i8(),
            imposters_see_protect: buffer.read_bool(),
        }
    }
}
