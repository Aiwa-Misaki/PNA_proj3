use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::{SocketAddr, TcpStream},
};

use log::info;

use crate::{
    common::{OpType, Request, Response},
    error::KvsError,
};

pub struct Client {
    stream: TcpStream,
}

impl Client {
    fn send_request(&mut self, req: Request) -> Result<(), KvsError> {
        info!("[client] sending request {:?}", req);
        let json = serde_json::to_string(&req)?;
        let mut writer = BufWriter::new(&self.stream);
        writer.write_all(json.as_bytes())?;
        writer.write_all(b"\n")?;
        writer.flush()?;
        info!("[client] send req success");
        Ok(())
    }

    fn read_response(&mut self) -> Result<Response, KvsError> {
        info!("[client] reading response from server");
        let mut reader = BufReader::new(&self.stream);
        let mut buf = String::new();
        match reader.read_line(&mut buf) {
            Ok(_) => {}
            Err(e) => return Err(KvsError::IOError(e)),
        };
        let resp: Response = serde_json::from_str(buf.as_str())?;
        info!("[client] read resp {:?} success", resp);
        Ok(resp)
    }

    pub fn new(addr: SocketAddr) -> Self {
        let stream = TcpStream::connect(addr).expect("failed to establish connection");
        Client { stream }
    }

    pub fn run_cmd(&mut self, op: OpType, key: String, value: String) -> Result<(), KvsError> {
        let req = Request { op, key, value };
        self.send_request(req)?;
        let resp = self.read_response()?;
        info!("got resp from server {:?}", resp);
        match resp {
            Response::Ok(s) => match s {
                Some(value) => {
                    println!("{}", value);
                }
                None => {
                    println!("Key not found")
                }
            },
            Response::Success => {}
            Response::Err(e) => {
                eprintln!("{}", e);
            }
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
