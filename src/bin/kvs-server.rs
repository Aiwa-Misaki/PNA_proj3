use clap::ArgAction::Version;
use clap::{Parser, Subcommand};
use kvs::engines::kvs::KvStore;
use slog::*;
use slog_async;
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
    let dir = env::current_dir()?;
    let mut kv = KvStore::open(dir.as_path()).unwrap();

    // parse command
    let cli = Cli::parse();

    let addr = cli.addr.unwrap();
    let engine = cli.engine.unwrap();


    // parse IP port
    let mut sp = addr.split(':');
    let ip = sp.next().unwrap();
    let port = sp.next().unwrap().parse::<u32>().unwrap();

    // init TCP listener
    let listener = TcpListener::bind(format!("{}:{}", ip, port))?;
    for stream in listener.incoming() {
        handle_connect(stream?, _log.clone())?
    }
    Ok(())
}

// handler for client request
fn handle_connect(stream: TcpStream, logger: slog::Logger) -> std::io::Result<()> {
    let client_addr = stream.peer_addr()?;


    Ok(())
}
