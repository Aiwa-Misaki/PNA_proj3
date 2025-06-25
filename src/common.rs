use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

use crate::error::KvsError;

pub fn validate_address(addr: &str) -> Result<SocketAddr, KvsError> {
    let socket: SocketAddr = addr.parse().map_err(KvsError::AddrError)?;
    Ok(socket)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum OpType {
    Get,
    Set,
    Remove,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Request {
    pub op: OpType,
    pub key: String,
    pub value: String
}

// for client-server communication
#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Ok(Option<String>),
    Success,
    Err(String),
}
