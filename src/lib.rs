#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/remarks.markdown"))]
mod error;
pub mod front;
pub use error::{Error, ErrorKind, Result};
