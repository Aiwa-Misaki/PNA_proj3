use clap::{Parser, Subcommand};
use kvs::engines::kvs::KvStore;
use std::env;

#[derive(Parser)]
#[command(version)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(long,default_value = "127.0.0.1:4000")]
    addr: Option<String>,

    #[arg(long,default_value = "kvs")]
    engine: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let dir = env::current_dir().unwrap();
    let mut kv = KvStore::open(dir.as_path()).unwrap();

    println!("addr: {:?}, engine: {:?}", &cli.addr, cli.engine);
}
