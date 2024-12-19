use failure::Fail;
// use std::io;

/// Error type for kvs.
#[derive(Fail, Debug)]
pub enum KvsError {
    /// Removing non-existent key error.
    #[fail(display = "Key not found")]
    KeyNotFound,
}

/// Result type for kvs.
pub type Result<T> = std::result::Result<T, KvsError>;