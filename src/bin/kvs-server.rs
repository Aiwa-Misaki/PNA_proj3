use clap::ArgAction::Version;
use clap::{Parser, Subcommand};
use kvs::common;
use kvs::engines::kvs::KvStore;
use log::{info, log};
use std::env;
use std::net::{TcpListener, TcpStream};

#[derive(Parser)] // derive Parser trait
#[command(version)] // add -V --version option
struct Cli {
    #[arg(long, default_value = "127.0.0.1:4000")]
    addr: String,

    #[arg(long, default_value = "kvs")]
    engine: String,
}

fn main() -> std::io::Result<()> {
    env_logger::init();
    let dir = env::current_dir()?;
    let mut kv = KvStore::open(dir.as_path()).unwrap();

    // parse command
    let cli = Cli::parse();

    let addr = cli.addr;
    let engine = cli.engine;

    // parse IP port
    let socket = common::validate_address(&addr).expect("invalid ip address");
    let (ip, port) = (socket.ip(), socket.port());
    info!("config {ip}:{port}, engine {engine}");

    // init TCP listener
    Ok(())
}

