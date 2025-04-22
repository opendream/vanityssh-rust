// src/error.rs
// Updated: 2025-04-22 13:37:42 by kengggg

use thiserror::Error;

/// Custom result type
pub type Result<T> = std::result::Result<T, VanityError>;

/// Custom error type for the vanity key generator
#[derive(Error, Debug)]
pub enum VanityError {
    /// Error when regex pattern is invalid
    #[error("Invalid regex pattern: {0}")]
    InvalidRegex(String),

    /// Error when key generation fails
    #[error("Key generation failed: {0}")]
    KeyGenerationError(String),

    /// Error when encoding/decoding fails
    #[error("Encoding error: {0}")]
    EncodingError(String),

    /// Error when the format is invalid
    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    /// I/O error
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}