use crate::error::KvsError;

pub trait KvsEngine {
    fn set(&mut self, key: String, value: String) -> Result<(), KvsError>;

    fn get(&self, key: String) -> Result<Option<String>, KvsError>;

    fn remove(&mut self, key: String) -> Result<(), KvsError>;
}

pub mod kvs;
pub use kvs::KvStore;
pub mod sled;
pub use sled::SledKvsEngine;
