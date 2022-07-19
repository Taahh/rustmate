use tokio::net::UdpSocket;
use tracing::info;
use crate::inner::rooms::GameRoom;
use crate::{Buffer, CONNECTIONS, User};
use crate::inner::objects::inner_net_objects::InnerNetObject;
use crate::util::hazel::HazelMessage;

pub fn convert(array: &[u8]) -> Vec<String> {
    let mut arr: Vec<String> = Vec::new();
    for x in array {
        // arr.push(format!("{:#04X?}", x));
        arr.push(format!("{:02X?}", x));
    }
    return arr;
}

pub fn to_string(array: Vec<String>) -> String {
    let mut s = "".to_string();
    for x in array {
        s += &*format!(" {}", x);
    }
    return s;
}

pub fn convert_only_two(array: &[u8]) -> Vec<String> {
    let mut arr: Vec<String> = Vec::new();
    for x in array {
        arr.push(format!("{:02X?}", x));
    }
    return arr;
}

pub fn sid_greater_than(newSid: u16, prevSid: u16) -> bool {
    let num = (prevSid + 32767);
    if prevSid < num {
        if newSid > prevSid {
            return newSid <= num;
        }
        return false;
    }
    if newSid <= prevSid {
        return newSid <= num;
    }
    return true;
}

pub fn send_spawn_message(user: &mut User, socket: &UdpSocket, room: GameRoom) {
    let mut buffer = Buffer {
        array: Vec::new(),
        position: 0,
    };
    buffer.write_u8(0x01);
    // CONNECTIONS.lock().unwrap().get(&self.socketAddr).unwrap().serverNonce += 1;

    let user = room.players.get(&room.host).as_mut().unwrap().to_owned().to_owned().unwrap();

    let nonce = user.serverNonce + 1;
    let addr = user.socketAddr;
    tokio::spawn(async move {
        CONNECTIONS
            .lock()
            .await
            .get_mut(&addr)
            .unwrap()
            .as_mut()
            .unwrap()
            .serverNonce += 1;
    });
    // info!("UPDATING USER RELIABLE: {:?}", user_option.as_ref().unwrap());
    buffer.write_u16(nonce);

    let mut game_data_to = HazelMessage::start_message(0x06);
    game_data_to.buffer.write_i32_le(room.code.code_int);
    game_data_to.buffer.write_packed_i32(user.player.as_ref().unwrap().id);

    if room.lobby_behavior != None {
        println!("SENDING USER LOBBY SPAWN MSG");
        let lobby_Behavior = room.lobby_behavior.as_ref().unwrap();
        let mut spawn_msg = HazelMessage::start_message(0x04);
        spawn_msg.buffer.write_packed_u32(0x02);
        println!("owner id: {:?}", lobby_Behavior.owner_id);
        spawn_msg.buffer.write_packed_i32(lobby_Behavior.owner_id);
        spawn_msg.buffer.write_i8(0);

        spawn_msg.buffer.write_packed_i32(1);

        spawn_msg.buffer.write_packed_u32(lobby_Behavior.net_id);

        let mut lobby_behavior_msg = HazelMessage::start_message(0x01);
        lobby_Behavior.serialize(&mut lobby_behavior_msg.buffer);
        lobby_behavior_msg.end_message();
        lobby_behavior_msg.copy_to(&mut spawn_msg.buffer);

        spawn_msg.end_message();
        spawn_msg.copy_to(&mut game_data_to.buffer);
    }

    /*if room.game_data != None {
        println!("SENDING USER LOBBY SPAWN MSG");
        let game_data = room.game_data.as_ref().unwrap();
        let vote_ban_system = room.vote_ban_system.as_ref().unwrap();
        let mut spawn_msg = HazelMessage::start_message(0x04);
        spawn_msg.buffer.write_packed_u32(0x03);
        println!("owner id: {:?}", game_data.owner_id);
        spawn_msg.buffer.write_packed_i32(game_data.owner_id);
        spawn_msg.buffer.write_i8(0);

        spawn_msg.buffer.write_packed_i32(2);

        spawn_msg.buffer.write_packed_u32(game_data.net_id);

        let mut game_data_msg = HazelMessage::start_message(0x01);
        game_data.serialize(&mut game_data_msg.buffer);
        game_data_msg.end_message();
        game_data_msg.copy_to(&mut spawn_msg.buffer);


        spawn_msg.buffer.write_packed_u32(vote_ban_system.net_id);

        let mut vote_ban_system_msg = HazelMessage::start_message(0x01);
        vote_ban_system.serialize(&mut vote_ban_system_msg.buffer);
        vote_ban_system_msg.end_message();
        vote_ban_system_msg.copy_to(&mut spawn_msg.buffer);

        spawn_msg.end_message();
        spawn_msg.copy_to(&mut game_data_to.buffer);

        for (k, v) in room.players {
            let player = v.as_ref().unwrap().to_owned().player.as_ref().unwrap().to_owned();
        }
    }*/


    game_data_to.end_message();
    game_data_to.copy_to(&mut buffer);

    let length =
        futures::executor::block_on(socket.send_to(&buffer.array, user.socketAddr)).unwrap();
    info!(
            "Sending spawn packet to {:?} with length {:?} and buffer {:?}",
            user.username.as_ref().unwrap_or(&"not found".to_string()),
            length,
            convert_only_two(&buffer.array)
        );
}