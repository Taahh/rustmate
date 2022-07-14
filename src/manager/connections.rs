use crate::manager::user::User;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Mutex;
use tokio::net::UdpSocket;

lazy_static! {
    pub static ref CONNECTIONS: Mutex<HashMap<SocketAddr, Option<User>>> = {
        let mut m = HashMap::new();
        return Mutex::new(m);
    };
}
