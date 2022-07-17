use crate::connections::{get_users, CONNECTIONS};
use crate::manager::connections;
use crate::manager::states::UserState::Loading;
use crate::manager::user::User;
use crate::matchmaker::handler::{handle_host, handle_request};
use crate::protocol::packet::{DisconnectPacket, HelloPacket, Packet, PingPacket, ReliablePacket};
use crate::util::buffer::Buffer;
use crate::util::util::convert;
use axum::body::Body;
use axum::http::Request;
use axum::response::Response;
use axum::routing::{get, put};
use axum::{routing, Router};
use lazy_static::lazy_static;
use std::borrow::{Borrow, BorrowMut};
use std::error::Error;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use std::thread;
use tokio::net::UdpSocket;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
use tower::make::Shared;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::field::debug;
use tracing::{debug, error, info, Level};
use tracing_subscriber::FmtSubscriber;

#[path = "./manager/manager.rs"]
mod manager;

#[path = "./protocol/protocol.rs"]
mod protocol;

#[path = "./matchmaker/matchmaker.rs"]
mod matchmaker;

#[path = "./util/mod.rs"]
mod util;

#[path = "./util/structs/mod.rs"]
mod structs;

#[path = "./inner/mod.rs"]
mod inner;

lazy_static! {
    /*pub static ref CONNECTIONS: Mutex<HashMap<SocketAddr, Option<User>>> = {
        let mut m = HashMap::new();
        return Mutex::new(m);
    };*/
    pub static ref RUNTIME: Runtime = {
        return tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
    };
}

#[tokio::main]
async fn spawn_udp() {
    let addr = SocketAddr::from_str("127.0.0.1:22023").unwrap();
    // let http_addr = SocketAddr::from_str("127.0.0.1:8082").unwrap();
    let socket = UdpSocket::bind(&addr).await;
    info!("Created new thread for UDP Socket");
    info!("Server started, listening for udp connections on /127.0.0.1:22023");
    loop {
        let mut raw_buffer: [u8; 2048] = [0; 2048];

        let (length, data_address) = socket
            .as_ref()
            .unwrap()
            .recv_from(&mut raw_buffer)
            .await
            .unwrap();
        let spliced_buffer = &raw_buffer[..length];
        println!("{:?}", convert(spliced_buffer));

        let mut buffer = Buffer {
            position: 0,
            array: Vec::from(spliced_buffer),
        };
        let packet_type: u8 = buffer.read_u8();

        debug!("Received packet type {:?}", packet_type);

        unsafe {
            let mut map = CONNECTIONS.lock().await;
            if map.contains_key(&data_address) {
                let mut userRef = map.get(&data_address).unwrap().as_ref();
                let user = userRef.as_mut().unwrap();
                if packet_type == 8 {
                    let mut packet = HelloPacket {
                        nonce: buffer.read_u16(),
                        version: None,
                        username: None,
                        lastNonce: None,
                        lastLanguage: None,
                        chatMode: None,
                        platformData: None,
                        modded: false
                    };
                    packet.deserialize(&mut buffer);
                    packet.process(user, socket.as_ref().unwrap());
                } else if packet_type == 1 {
                    let mut packet = ReliablePacket {
                        nonce: buffer.read_u16(),
                        reliable_packet_id: None,
                        hazel_message: None,
                        buffer: buffer.clone(),
                    };
                    packet.deserialize(&mut buffer);
                    packet.process(user, socket.as_ref().unwrap());
                } else if packet_type == 12 {
                    let mut packet = PingPacket {
                        nonce: buffer.read_u16(),
                    };
                    packet.deserialize(&mut buffer);
                    packet.process(user, socket.as_ref().unwrap());
                } else if packet_type == 9 {
                    let mut packet = DisconnectPacket {
                        disconnect_type: None,
                        reason: None,
                    };
                    packet.deserialize(&mut buffer);
                    packet.process(user, socket.as_ref().unwrap());
                }
                info!("user: {:?}", user);
            } else {
                error!("RE INSERTING");
                let mut user = User::new(Loading, data_address);
                map.insert(data_address, Some(user));
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");

    tokio::task::spawn_blocking(|| {
        spawn_udp();
    })
    .await
    .expect("Thread panicked");

    /*tokio::spawn(async move {
        info!("New HTTP Server /127.0.0.1:8082");
        let service = Router::new()
            /*.route("/api/games", put(handle_host))
            .route("/", get(|r: Request<Body>| async move {
                println!("{:?}", r.uri())
            }))*/
            .layer(TraceLayer::new_for_http())
            .layer(ServiceBuilder::new()
                .layer(axum::middleware::from_fn(handle_request)));
        axum::Server::bind(&http_addr)
            .serve(service.into_make_service())
            .await
            .expect("HTTP Server Error");
    });*/

    loop {}

    Ok(())
}
