use crate::error::Result;
use crate::KvsEngine;
use sled;
use std::path::PathBuf;

pub struct SledKvsEngine {}

impl KvsEngine for SledKvsEngine {
    fn set(&self, key: String, value: String) -> Result<()> {
        todo!()
    }
    fn get(&self, key: String) -> Result<Option<String>> {
        todo!()
    }
    fn remove(&self, key: String) -> Result<()> {
        todo!()
    }
}

impl SledKvsEngine {
    pub fn new(db: sled::Db) -> SledKvsEngine {
        todo!()
    }
}

pub fn open(sled_path: &str) -> PathBuf {
    todo!()
}
