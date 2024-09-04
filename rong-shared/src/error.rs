use bincode;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum GameError {
    #[error("IO error: {0}")]
    Io(String), // Changed from std::io::Error to String for serializability
    #[error("UTF-8 error: {0}")]
    Utf8(String), // Changed from std::str::Utf8Error to String
                  // Uncomment if needed in the future
                  // #[error("Game error: {0}")]
                  // Game(String),
}

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum ServerError {
    #[error("IO error: {0}")]
    Io(String),
    #[error("UTF-8 error: {0}")]
    Utf8(String),
    #[error("Player not found")]
    PlayerNotFound,
    #[error("Game full")]
    GameFull,
}

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum ClientError {
    #[error("IO error: {0}")]
    Io(String),
    #[error("UTF-8 error: {0}")]
    Utf8(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
}

impl From<std::io::Error> for ClientError {
    fn from(err: std::io::Error) -> Self {
        ClientError::Io(err.to_string())
    }
}

impl From<std::str::Utf8Error> for ClientError {
    fn from(err: std::str::Utf8Error) -> Self {
        ClientError::Utf8(err.to_string())
    }
}

impl From<bincode::Error> for ClientError {
    fn from(err: bincode::Error) -> Self {
        ClientError::Serialization(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, GameError>;

// Helper functions to convert from std errors to our serializable errors
impl From<std::io::Error> for GameError {
    fn from(err: std::io::Error) -> Self {
        GameError::Io(err.to_string())
    }
}

impl From<std::str::Utf8Error> for GameError {
    fn from(err: std::str::Utf8Error) -> Self {
        GameError::Utf8(err.to_string())
    }
}

impl From<std::io::Error> for ServerError {
    fn from(err: std::io::Error) -> Self {
        ServerError::Io(err.to_string())
    }
}

impl From<std::str::Utf8Error> for ServerError {
    fn from(err: std::str::Utf8Error) -> Self {
        ServerError::Utf8(err.to_string())
    }
}
