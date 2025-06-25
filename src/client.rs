use std::net::{SocketAddr, TcpStream};

use crate::{
    common::{OpType, Request, Response},
    error::KvsError,
};

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new(addr: SocketAddr) -> Self {
        let stream = TcpStream::connect(addr).expect("failed to establish connection");
        Client { stream }
    }

    pub fn runCmd(&mut self, op: OpType, key: String, value: String) -> Result<(), KvsError> {
        let req = Request { op, key, value };
        serde_json::to_writer(&mut self.stream, &req)?;
        let resp: Response = serde_json::from_reader(&self.stream)?;
        match resp {
            Response::Ok(s) => match s {
                Some(value) => {}
                None => {}
            },
            Response::Success => {}
            Response::Err(e) => {}
        }
        Ok(())
    }

    // pub fn get(&self, key: String) -> Result<(), KvsError> {
    //     todo!()
    // }

    // pub fn set(&mut self, key: String, value: String) -> Result<(), KvsError> {
    //     todo!()
    // }

    // pub fn remove(&mut self, key: String, value: String) -> Result<(), KvsError> {
    //     todo!()
    // }
}
