/*
This is the Error module. It contains custom error types for the game.

The module defines:
- GameError: An enum of possible error types that can occur in the game
- Result: A type alias for std::result::Result with GameError as the error type

This centralized error handling makes it easier to manage and propagate errors
throughout the application.
*/

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),
    // Note: Uncomment the following variants if they become necessary in the future
    //#[error("Game error: {0}")]
    //Game(String),
}

pub type Result<T> = std::result::Result<T, GameError>;
