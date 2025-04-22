// src/matcher.rs
// Updated: 2025-04-22 13:39:21 by kengggg

use regex::Regex;
use crate::error::{Result, VanityError};
use crate::ssh::public_key::extract_ssh_key_data;

/// Checks if a string matches a regex pattern.
pub fn matches_pattern(key: &str, pattern: &str) -> Result<bool> {
    // Compile the regex pattern
    let regex = match Regex::new(pattern) {
        Ok(r) => r,
        Err(e) => return Err(VanityError::InvalidRegex(e.to_string())),
    };

    // Check if the key matches the pattern
    Ok(regex.is_match(key))
}

/// Checks if an SSH public key matches a regex pattern.
/// This function can match either against the full SSH key string
/// or just the base64-encoded portion, depending on the full_match parameter.
pub fn ssh_key_matches_pattern(ssh_key: &str, pattern: &str, full_match: bool) -> Result<bool> {
    if full_match {
        // Match against the full SSH key string
        matches_pattern(ssh_key, pattern)
    } else {
        // Extract the base64 part and match against that
        let base64_part = extract_ssh_key_data(ssh_key)?;
        matches_pattern(&base64_part, pattern)
    }
}