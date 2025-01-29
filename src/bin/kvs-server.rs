use clap::{Parser, Subcommand};
use std::env;
use kvs::engines::kvs::KvStore;

#[derive(Parser)]
#[command(version)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(long)]
    addr: Option<String>,

    #[arg(long)]
    engine: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let dir = env::current_dir().unwrap();
    let mut kv = KvStore::open(dir.as_path()).unwrap();

    println!("addr: {:?}, engine: {:?}", &cli.addr, cli.engine);
}
