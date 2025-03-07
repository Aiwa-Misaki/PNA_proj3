use clap::ArgAction::Version;
use clap::{Parser, Subcommand};
use kvs::engines::kvs::KvStore;
use slog::*;
use slog_async;
use std::env;
use std::net::{TcpListener, TcpStream};

#[derive(Parser)]
#[command(version)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(long, default_value = "127.0.0.1:4000")]
    addr: Option<String>,

    #[arg(long, default_value = "kvs")]
    engine: Option<String>,
}

fn main() -> std::io::Result<()> {
    let dir = env::current_dir()?;
    let mut kv = KvStore::open(dir.as_path()).unwrap();

    // init logger
    let decorator = slog_term::TermDecorator::new().stderr().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let _log = slog::Logger::root(drain, o!());

    // parse command
    let cli = Cli::parse();

    let addr = cli.addr.unwrap();
    let engine = cli.engine.unwrap();

    // print ip, engine and version on start
    info!(_log.clone(), "Application started";
        "listen ip address" => format!("{:?}", addr),
        "storage engine" => format!("{:?}", engine),
    "clt version" => format!("v{}", clap::crate_version!()));

    // parse IP port
    let mut sp = addr.split(':');
    let ip = sp.next().unwrap();
    let port = sp.next().unwrap().parse::<u32>().unwrap();
    info!(_log.clone(), "ip {}, port {}", ip, port);

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

    info!(logger, "new connection from {}", client_addr);

    Ok(())
}
