use crate::manager::user::User;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tokio::sync::{Mutex, MutexGuard, OwnedMutexGuard};
use tracing::log::info;

lazy_static! {
    pub static ref CONNECTIONS: Mutex<HashMap<SocketAddr, Option<User>>> = {
        let mut m = HashMap::new();
        return Mutex::new(m);
    };
}

pub fn update_user(user: User) {
    let user_clone = user.clone();
    tokio::spawn(async move {
        get_users().insert(user.socketAddr, Some(user_clone));
        println!("updated");
    });
}

pub fn get_users() -> MutexGuard<'static, HashMap<SocketAddr, Option<User>>> {
    return futures::executor::block_on(CONNECTIONS.lock());
}
