use log::info;
use clap::Parser;
use kvs::engines::kvs::KvStore;
use kvs::error::KvsError;
use kvs::server::Server;
use kvs::{common, KvsEngine, SledKvsEngine};
use std::env;

#[derive(Parser)] // derive Parser trait
#[command(version)] // add -V --version option
struct Cli {
    #[arg(long, default_value = "127.0.0.1:4000")]
    addr: String,

    #[arg(long, default_value = "kvs")]
    engine: String,
}

fn main() -> Result<(), KvsError> {
    env_logger::init();
    let dir = env::current_dir()?;

    // parse command
    let cli = Cli::parse();

    let addr = cli.addr;
    let engine = cli.engine;

    // parse IP port
    let socket = common::validate_address(&addr).expect("invalid ip address");
    let (ip, port) = (socket.ip(), socket.port());
    info!("[kvs-server] config {ip}:{port}, engine {engine}");

    // init TCP listener
    let engine_instance: Box<dyn KvsEngine> = match engine.as_str() {
        "sled" => Box::new(SledKvsEngine::open(&dir)?),
        "kvs" => Box::new(KvStore::open(&dir)?),
        other => return Err(KvsError::UnknownEngineError(other.to_string())),
    };

    let mut server = Server::new(engine_instance, socket)?;

    server.run()?;

    Ok(())
}
