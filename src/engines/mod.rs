use crate::error::Result;

pub trait KvsEngine {
    fn set(&mut self, key: String, value: String) -> Result<()>;

    /// get map[key]; if key not in map, then None
    fn get(&mut self, key: String) -> Result<Option<String>>;

    /// remove k-v pair from map
    fn remove(&mut self, key: String) -> Result<()>;
}

pub mod kvs;
pub use kvs::KvStore;
pub mod sled;
pub use sled::SledKvsEngine;
