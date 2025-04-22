// src/error.rs
// Created: 2025-04-22 12:10:09 by kengggg

use thiserror::Error;

/// Custom error types for the vanity key generator
#[derive(Error, Debug)]
pub enum VanityError {
    #[error("Invalid regex pattern: {0}")]
    InvalidRegex(#[from] regex::Error),

    #[error("Key generation error: {0}")]
    KeyGenerationError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Result type alias using our custom error type
pub type Result<T> = std::result::Result<T, VanityError>;