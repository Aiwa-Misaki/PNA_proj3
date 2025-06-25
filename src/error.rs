use std::{net::AddrParseError, string::FromUtf8Error};

use thiserror::Error;

#[derive(Error, Debug)]
/// err
pub enum KvsError {
    #[error("IO error: {0}")]
    /// error related to file IO
    IOError(#[from] std::io::Error),

    #[error("Key not found")]
    /// error related to remove key
    RmKeyError(String),

    #[error("serialization error: {0}")]
    /// error related to serialize & deserialize
    SerializeError(#[from] serde_json::Error),

    #[error("sled error: {0}")]
    SledError(#[from] sled::Error),

    #[error("utf8 conversion error: {0}")]
    VecError(#[from] FromUtf8Error),

    #[error("Unknown engine error: {0}")]
    UnknownEngineError(String),

    #[error("Invalid addr error: {0}")]
    AddrError(AddrParseError),

}
