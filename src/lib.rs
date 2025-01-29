//! kvStore crate
//!
//! This crate provides a simple key-value store implementation.
// #![deny(missing_docs)]
mod client;
mod common;
pub mod engines;
pub mod error;
mod server;

pub use engines::{KvStore, KvsEngine, SledKvsEngine};

pub use error::Result;