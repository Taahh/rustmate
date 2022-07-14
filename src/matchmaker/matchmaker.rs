use serde::{Deserialize, Serialize};

pub mod handler;

pub trait JsonResponse: Serialize {}

#[derive(Serialize, Deserialize)]
pub struct HostServer {
    pub ip: u32,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserTokenRequestData {
    pub puid: String,
    pub username: String,
    pub clientVersion: i32,
    pub language: i8,
}

impl JsonResponse for HostServer {}
impl JsonResponse for UserTokenRequestData {}
