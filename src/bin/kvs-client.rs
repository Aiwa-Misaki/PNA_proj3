use clap::builder::{Str, TypedValueParser};
use clap::{Parser, Subcommand};
use kvs::common;
use log::{info, warn};
use std::env;
use std::io::prelude::*;
use std::io::{BufRead, Write};
use std::net::SocketAddr;
use std::net::TcpStream;

#[derive(Parser)]
#[command(version)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[clap(long, default_value = "127.0.0.1:4000", global = true)]
    addr: String,
}

#[derive(Subcommand)]
enum Commands {
    Set { key: String, value: String },
    Get { key: String },
    Rm { key: String },
}

fn main() {
    env_logger::init();

    let cli = Cli::parse();
    let dir = env::current_dir().unwrap();

    // validate arg
    let socket = common::validate_address(&cli.addr).unwrap_or_else(|_| {
        panic!(
            "connect addr invalid, should be ip:PORT, got {}",
            cli.addr.clone()
        )
    });
    let (ip, port) = (socket.ip(), socket.port());
    info!("config:{ip}:{port}");

    // init a TcpStream
    let mut stream = TcpStream::connect(cli.addr).expect("failed to connect to given addr");
    stream.write(b"op={cli.command},key={1},value={2}");
}
