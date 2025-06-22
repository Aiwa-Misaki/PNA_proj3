use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

pub fn validate_address(addr: &str) -> Result<SocketAddr, String> {
    let socket: SocketAddr = addr.parse().expect("invalid ip address");
    Ok(socket)
}

// for client-server communication
#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Ok(Option<String>),
    Success,
    Err(String),
}
