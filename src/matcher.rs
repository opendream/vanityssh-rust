// src/matcher.rs
// Updated: 2025-04-22 12:10:09 by kengggg

use regex::Regex;
use crate::error::{Result, VanityError};

/// Checks if the given public key matches the provided regex pattern.
/// The matching is case-insensitive.
pub fn matches_pattern(public_key: &str, pattern: &str) -> Result<bool> {
    // Compile the regex pattern with case-insensitive matching
    let regex = Regex::new(&format!("(?i){}", pattern))
        .map_err(|e| VanityError::InvalidRegex(e))?;

    // Check if the public key matches the pattern
    Ok(regex.is_match(public_key))
}