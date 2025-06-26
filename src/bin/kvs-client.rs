use clap::{Parser, Subcommand};
use kvs::client::Client;
use kvs::common::{validate_address, OpType};
use kvs::error::KvsError;
use log::info;

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

fn main() -> Result<(), KvsError> {
    env_logger::init();

    let cli = Cli::parse();

    // validate arg
    let socket = validate_address(&cli.addr)?;
    let (ip, port) = (socket.ip(), socket.port());
    info!("config:{ip}:{port}");

    let mut client = Client::new(socket);

    // init a TcpStream
    match cli.command {
        Commands::Set { key, value } => {
            client.run_cmd(OpType::Set, key, value)?;
        }
        Commands::Get { key } => {
            client.run_cmd(OpType::Get, key, "".to_string())?;
        }
        Commands::Rm { key } => {
            client.run_cmd(OpType::Remove, key, "".to_string())?;
        }
    }
    Ok(())
}
