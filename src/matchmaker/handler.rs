use crate::matchmaker::{HostServer, JsonResponse, UserTokenRequestData};
use axum::body::Body;
use axum::http::{HeaderMap, Request};
use axum::middleware::Next;
use axum::{extract, Json};
use std::fmt::Error;
use std::mem::transmute;
use std::net::Ipv4Addr;
use std::ops::Shl;
use std::ptr::null;
use std::str::FromStr;
use tracing::info;

pub async fn handle_request(request: Request<Body>, next: Next<Body>) -> Json<impl JsonResponse> {
    todo!("idk how to do this");
    info!("Request: {:?}", request.method());
    info!("URI: {:?}", request.uri());
    handle_host(request)
}

pub fn handle_host(request: Request<Body>) -> Json<HostServer> {
    info!("Headers: {:?}", request.body());
    return Json(HostServer {
        ip: convert_ip(Ipv4Addr::from_str("127.0.0.1").unwrap()),
        port: 22023,
    });
}

pub fn handle_user(
    extract::Json(payload): extract::Json<UserTokenRequestData>,
) -> Json<UserTokenRequestData> {
    info!("extracted: {:?}", payload);
    Json(payload)
}

#[deny(arithmetic_overflow)]
fn convert_ip(ip: Ipv4Addr) -> u32 {
    let octets = ip.octets();
    let ip = unsafe { transmute::<[u8; 4], u32>(octets) };
    return ip;
}
