use crate::{error::KvsError, KvsEngine};
use std::path::PathBuf;
extern crate sled as sled_crate;

pub struct SledKvsEngine {
    db: sled_crate::Db,
}

impl KvsEngine for SledKvsEngine {
    fn set(&mut self, key: String, value: String) -> Result<(), KvsError> {
        match self.db.insert(key.as_bytes(), value.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => Err(KvsError::SledError(e)),
        }
    }
    fn get(&self, key: String) -> Result<Option<String>, KvsError> {
        match self.db.get(key.as_bytes()) {
            Ok(res) => match res {
                Some(r) => Ok(Some(
                    String::from_utf8(r.to_vec()).map_err(KvsError::VecError)?,
                )),
                None => Ok(None),
            },
            Err(e) => Err(KvsError::SledError(e)),
        }
    }
    fn remove(&mut self, key: String) -> Result<(), KvsError> {
        match self.db.get(key.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => Err(KvsError::SledError(e)),
        }
    }
}

impl SledKvsEngine {
    pub fn open(path: &PathBuf) -> Result<Self, KvsError> {
        Ok(SledKvsEngine {
            db: sled_crate::open(path)?,
        })
    }
}
