#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
mod error;
pub use error::{Error, ErrorKind, Result};
