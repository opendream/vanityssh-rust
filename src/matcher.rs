// src/matcher.rs
// Updated: 2025-04-22 14:15:00 by kengggg

use regex::Regex;
use crate::error::{Result, VanityError};
use crate::ssh::public_key::extract_ssh_key_data;

/// Checks if a string matches a regex pattern.
///
/// If case_sensitive is false, the pattern is treated as case-insensitive.
pub fn matches_pattern(key: &str, pattern: &str, case_sensitive: bool) -> Result<bool> {
    // Handle case sensitivity properly:
    // 1. If we want case-insensitive matching, add (?i) if not already there
    // 2. If we want case-sensitive matching, ensure (?i) is not present
    let effective_pattern = if case_sensitive {
        // Remove (?i) prefix if it exists for case-sensitive matching
        if pattern.starts_with("(?i)") {
            pattern[4..].to_string()
        } else {
            pattern.to_string()
        }
    } else {
        // Add (?i) prefix if it doesn't exist for case-insensitive matching
        if pattern.starts_with("(?i)") {
            pattern.to_string()
        } else {
            format!("(?i){}", pattern)
        }
    };

    // Compile the regex pattern
    let regex = match Regex::new(&effective_pattern) {
        Ok(r) => r,
        Err(e) => return Err(VanityError::InvalidRegex(e.to_string())),
    };

    // Check if the key matches the pattern
    Ok(regex.is_match(key))
}

/// Checks if an SSH public key matches a regex pattern.
/// The function extracts the base64-encoded part of the key and matches against that.
///
/// If case_sensitive is false, the pattern is treated as case-insensitive.
pub fn ssh_key_matches_pattern(ssh_key: &str, pattern: &str, case_sensitive: bool) -> Result<bool> {
    // Extract the base64 part and match against that
    let base64_part = extract_ssh_key_data(ssh_key)?;
    matches_pattern(&base64_part, pattern, case_sensitive)
}