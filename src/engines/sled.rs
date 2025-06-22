use crate::{error::KvsError, KvsEngine};
use sled;
use std::path::{Path, PathBuf};

pub struct SledKvsEngine {}

impl KvsEngine for SledKvsEngine {
    fn set(&mut self, key: String, value: String) -> Result<(), KvsError> {
        todo!()
    }
    fn get(&mut self, key: String) -> Result<Option<String>, KvsError> {
        todo!()
    }
    fn remove(&mut self, key: String) -> Result<(), KvsError> {
        todo!()
    }
}

impl SledKvsEngine {
    pub fn open(path: &Path) -> Result<Self, KvsError> {
        todo!()
    }
}
