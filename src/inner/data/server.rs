use crate::networking::buffer::Buffer;

#[derive(Debug)]
pub struct GameOptionsData {
    length: i8,
    version: i8,
    max_players: i8,
    keywords: u32,
    map: i8,
    player_speed_mod: f32,
    crew_light_mod: f32,
    imp_light_mod: f32,
    kill_cooldown: f32,
    common_tasks: i8,
    long_tasks: i8,
    short_tasks: i8,
    meetings: u32,
    imposters: i8,
    kill_distance: i8,
    discussion_time: u32,
    voting_time: u32,
    defaults: bool,
    emergency_cooldown: i8,
    ejections: bool,
    visual_tasks: bool,
    anonymous_voting: bool,
    task_bar: i8,
}

impl GameOptionsData {
    pub fn read(buffer: &mut Buffer) -> Self {
        let mut options = Self {
            length: buffer.read_byte(),
            version: buffer.read_byte(),
            max_players: buffer.read_byte(),
            keywords: buffer.read_uint_32(),
            map: buffer.read_byte(),
            player_speed_mod: buffer.read_float(),
            crew_light_mod: buffer.read_float(),
            imp_light_mod: buffer.read_float(),
            kill_cooldown: buffer.read_float(),
            common_tasks: buffer.read_byte(),
            long_tasks: buffer.read_byte(),
            short_tasks: buffer.read_byte(),
            meetings: buffer.read_uint_32(),
            imposters: buffer.read_byte(),
            kill_distance: buffer.read_byte(),
            discussion_time: buffer.read_uint_32(),
            voting_time: buffer.read_uint_32(),
            defaults: buffer.read_bool(),
            emergency_cooldown: buffer.read_byte(),
            ejections: true,
            visual_tasks: true,
            anonymous_voting: false,
            task_bar: 0,
        };

        if options.version > 2 {
            options.set_ejections(buffer.read_bool());
            options.set_visual_tasks(buffer.read_bool());
        }
        if options.version > 3 {
            options.set_anonymous_voting(buffer.read_bool());
            options.set_task_bar(buffer.read_byte());
        }

        //TODO: role settings

        return options;
    }

    pub fn length(&self) -> i8 {
        self.length
    }
    pub fn version(&self) -> i8 {
        self.version
    }
    pub fn max_players(&self) -> i8 {
        self.max_players
    }
    pub fn keywords(&self) -> u32 {
        self.keywords
    }
    pub fn map(&self) -> i8 {
        self.map
    }
    pub fn player_speed_mod(&self) -> f32 {
        self.player_speed_mod
    }
    pub fn crew_light_mod(&self) -> f32 {
        self.crew_light_mod
    }
    pub fn imp_light_mod(&self) -> f32 {
        self.imp_light_mod
    }
    pub fn kill_cooldown(&self) -> f32 {
        self.kill_cooldown
    }
    pub fn common_tasks(&self) -> i8 {
        self.common_tasks
    }
    pub fn long_tasks(&self) -> i8 {
        self.long_tasks
    }
    pub fn short_tasks(&self) -> i8 {
        self.short_tasks
    }
    pub fn meetings(&self) -> u32 {
        self.meetings
    }
    pub fn imposters(&self) -> i8 {
        self.imposters
    }
    pub fn kill_distance(&self) -> i8 {
        self.kill_distance
    }
    pub fn discussion_time(&self) -> u32 {
        self.discussion_time
    }
    pub fn voting_time(&self) -> u32 {
        self.voting_time
    }
    pub fn defaults(&self) -> bool {
        self.defaults
    }
    pub fn emergency_cooldown(&self) -> i8 {
        self.emergency_cooldown
    }
    pub fn ejections(&self) -> bool {
        self.ejections
    }
    pub fn visual_tasks(&self) -> bool {
        self.visual_tasks
    }
    pub fn anonymous_voting(&self) -> bool {
        self.anonymous_voting
    }
    pub fn task_bar(&self) -> i8 {
        self.task_bar
    }
    pub fn set_length(&mut self, length: i8) {
        self.length = length;
    }
    pub fn set_version(&mut self, version: i8) {
        self.version = version;
    }
    pub fn set_max_players(&mut self, max_players: i8) {
        self.max_players = max_players;
    }
    pub fn set_keywords(&mut self, keywords: u32) {
        self.keywords = keywords;
    }
    pub fn set_map(&mut self, map: i8) {
        self.map = map;
    }
    pub fn set_player_speed_mod(&mut self, player_speed_mod: f32) {
        self.player_speed_mod = player_speed_mod;
    }
    pub fn set_crew_light_mod(&mut self, crew_light_mod: f32) {
        self.crew_light_mod = crew_light_mod;
    }
    pub fn set_imp_light_mod(&mut self, imp_light_mod: f32) {
        self.imp_light_mod = imp_light_mod;
    }
    pub fn set_kill_cooldown(&mut self, kill_cooldown: f32) {
        self.kill_cooldown = kill_cooldown;
    }
    pub fn set_common_tasks(&mut self, common_tasks: i8) {
        self.common_tasks = common_tasks;
    }
    pub fn set_long_tasks(&mut self, long_tasks: i8) {
        self.long_tasks = long_tasks;
    }
    pub fn set_short_tasks(&mut self, short_tasks: i8) {
        self.short_tasks = short_tasks;
    }
    pub fn set_meetings(&mut self, meetings: u32) {
        self.meetings = meetings;
    }
    pub fn set_imposters(&mut self, imposters: i8) {
        self.imposters = imposters;
    }
    pub fn set_kill_distance(&mut self, kill_distance: i8) {
        self.kill_distance = kill_distance;
    }
    pub fn set_discussion_time(&mut self, discussion_time: u32) {
        self.discussion_time = discussion_time;
    }
    pub fn set_voting_time(&mut self, voting_time: u32) {
        self.voting_time = voting_time;
    }
    pub fn set_defaults(&mut self, defaults: bool) {
        self.defaults = defaults;
    }
    pub fn set_emergency_cooldown(&mut self, emergency_cooldown: i8) {
        self.emergency_cooldown = emergency_cooldown;
    }
    pub fn set_ejections(&mut self, ejections: bool) {
        self.ejections = ejections;
    }
    pub fn set_visual_tasks(&mut self, visual_tasks: bool) {
        self.visual_tasks = visual_tasks;
    }
    pub fn set_anonymous_voting(&mut self, anonymous_voting: bool) {
        self.anonymous_voting = anonymous_voting;
    }
    pub fn set_task_bar(&mut self, task_bar: i8) {
        self.task_bar = task_bar;
    }
}
