use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::{SocketAddr, TcpListener, TcpStream},
};

use log::info;

use crate::{
    common::{self},
    error::KvsError,
    KvsEngine,
};

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

    fn send_response(
        &mut self,
        stream: &TcpStream,
        resp: common::Response,
    ) -> Result<(), KvsError> {
        info!(
            "[server] sending response {:?} to {}",
            resp,
            stream.peer_addr()?
        );
        let json = serde_json::to_string(&resp)?;
        let mut writer = BufWriter::new(stream);
        writer.write_all(json.as_bytes())?;
        writer.write_all(b"\n")?;
        writer.flush()?;
        info!(
            "[server] sent response to {} successfully",
            stream.peer_addr()?
        );
        Ok(())
    }

    fn read_request(&mut self, stream: &TcpStream) -> Result<common::Request, KvsError> {
        info!("[server] reading request from {}", stream.peer_addr()?);
        let mut reader = BufReader::new(stream);
        let mut buf = String::new();
        match reader.read_line(&mut buf) {
            Ok(_) => {}
            Err(e) => return Err(KvsError::IOError(e)),
        };
        let req: common::Request = serde_json::from_str(buf.as_str())?;
        info!(
            "[server] read request from {} successfully, req={:?}",
            stream.peer_addr()?,
            req
        );
        Ok(req)
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
        info!("[server] new connection from {client_addr}");

        let req = self.read_request(&stream)?;
        info!("[server] received request {:?}", req);

        let resp: common::Response;

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
        info!("[server] finished handling request, resp {:?}", resp);
        self.send_response(&stream, resp)?;
        Ok(())
    }
}
