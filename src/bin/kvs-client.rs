use clap::builder::{Str, TypedValueParser};
use clap::{Parser, Subcommand};
use kvs::engines::KvStore;
use kvs::error::KvsError;
use slog::{info, o, Drain};
use std::env;
use std::io::{BufRead, Write};
use std::net::TcpStream;

#[derive(Parser)]
#[command(version)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[clap(long, default_value = "127.0.0.1:4000", global = true)]
    addr: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    Set { key: String, value: String },
    Get { key: String },
    Rm { key: String },
}

fn main() {
    // init logger
    let decorator = slog_term::TermDecorator::new().stderr().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let _log = slog::Logger::root(drain, o!());

    let cli = Cli::parse();
    let dir = env::current_dir().unwrap();
    let mut kv = KvStore::open(dir.as_path()).unwrap();

    // validate arg
    match validate_address(cli.addr.clone(), _log) {
        Ok(()) => {}
        Err(e) => {
            println!("invalid ip address: {}, should be IP:PORT", e);
            Err(e);
        }
    }
    // init a TcpStream
    let mut stream = TcpStream::connect(cli.addr.unwrap()).expect("connect failed");
    //stream.write("114514".as_bytes()).expect("write failed");

    // parse command
    match &cli.command {
        Commands::Set { key, value } => {
            let result = kv.set(key, value);
        }
        Commands::Get { key } => {
            let result = kv.get(key).unwrap();
            if result.is_none() {
                println!("Key not found");
            } else {
                println!("{}", result.unwrap());
            }
        }
        Commands::Rm { key } => {
            kv.remove(key.clone()).unwrap();
        }
    }
}

/// Validate if addr is valid IP address.
///
/// * `addr`: string to check
/// * `logger`: logger instance
fn validate_address(addr: Option<String>, logger: slog::Logger) -> Result<(), String> {
    match addr {
        None => {
            info!(logger, "no address specified, using 127.0.0.1:4000");
        }
        Some(addr_str) => {
            let mut sp = addr_str.split(':');
            let ip = sp.next().unwrap();
            let port = sp.next().unwrap().parse::<u16>().unwrap();

            info!(logger, "connecting to {}:{}", ip, port);
        }
    }

    Ok(())
}
