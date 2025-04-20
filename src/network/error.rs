use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum NetworkError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    
    #[error("Connection error: {0}")]
    ConnectionError(String),
    
    #[error("Authentication error: {0}")]
    AuthError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Server error: {0}")]
    ServerError(String),
    
    #[error("Client error: {0}")]
    ClientError(String),
    
    #[error("Timeout error")]
    Timeout,
}

pub type NetworkResult<T> = Result<T, NetworkError>;