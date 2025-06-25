use std::net::{SocketAddr, TcpListener, TcpStream};

use log::info;

use crate::{common, error::KvsError, KvsEngine};

pub struct Server {
    engine: Box<dyn KvsEngine>,
    listener: TcpListener,
}

impl Server {
    /// Init server with engine. Estab TCP connection.
    ///
    /// * `engine`: specific engine instance, kvstore or sled
    /// * `addr`: socket addr to conn to
    pub fn new(engine: Box<dyn KvsEngine>, addr: SocketAddr) -> Result<Self, KvsError> {
        let listener = TcpListener::bind(addr).map_err(KvsError::IOError)?;
        Ok(Server { engine, listener })
    }

    /// Start handling connection.
    pub fn run(&mut self) -> Result<(), KvsError> {
        let stream = self.listener.try_clone()?;
        for s in stream.incoming() {
            self.handle_connect(s?)?;
        }
        Ok(())
    }
    /// Handle connection.
    ///
    /// 1) do request desirializing.
    /// 2) call engine api to handle request.
    /// 3) do response serializing.
    ///
    /// * `stream`: stream from client
    fn handle_connect(&mut self, stream: TcpStream) -> Result<(), KvsError> {
        let client_addr = stream
            .peer_addr()
            .expect("fail to get addr of incoming request");
        info!("receiving connection from {client_addr}");

        let req: common::Request = serde_json::from_reader(&stream)?;

        let mut resp: common::Response;

        match req.op {
            common::OpType::Set => {
                let res = self.engine.set(req.key, req.value);
                match res {
                    Ok(()) => {
                        resp = common::Response::Success;
                    }
                    Err(e) => {
                        resp = common::Response::Err(e.to_string());
                    }
                }
            }
            common::OpType::Remove => {
                let res = self.engine.remove(req.key);
                match res {
                    Ok(()) => {
                        resp = common::Response::Success;
                    }
                    Err(e) => {
                        resp = common::Response::Err(e.to_string());
                    }
                }
            }
            common::OpType::Get => {
                let res = self.engine.get(req.key);
                match res {
                    Ok(s) => {
                        resp = common::Response::Ok(s);
                    }
                    Err(e) => {
                        resp = common::Response::Err(e.to_string());
                    }
                }
            }
        }
        serde_json::to_writer(&stream, &resp)?;
        Ok(())
    }
}
