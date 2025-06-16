// src/matcher.rs
// Updated: 2025-04-22 14:15:00 by kengggg

use crate::error::{Result, VanityError};
use crate::ssh::public_key::extract_ssh_key_data;
use regex::Regex;

/// Pre-compiled regex matcher for high-performance matching
pub struct CompiledMatcher {
    regex: Regex,
}

impl CompiledMatcher {
    /// Create a new compiled matcher from a pattern
    pub fn new(pattern: &str, case_sensitive: bool) -> Result<Self> {
        let effective_pattern = if case_sensitive {
            // Remove (?i) prefix if it exists for case-sensitive matching
            if let Some(stripped) = pattern.strip_prefix("(?i)") {
                stripped.to_string()
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

        let regex =
            Regex::new(&effective_pattern).map_err(|e| VanityError::InvalidRegex(e.to_string()))?;

        Ok(CompiledMatcher { regex })
    }

    /// Check if a string matches the compiled pattern
    pub fn is_match(&self, text: &str) -> bool {
        self.regex.is_match(text)
    }

    /// Check if an SSH public key matches the compiled pattern
    pub fn ssh_key_matches(&self, ssh_key: &str) -> Result<bool> {
        let base64_part = extract_ssh_key_data(ssh_key)?;
        Ok(self.is_match(&base64_part))
    }
}

/// Checks if a string matches a regex pattern.
///
/// **WARNING**: This function compiles the regex on every call and should only be used
/// for one-off matching. For repeated matching, use `CompiledMatcher` instead.
///
/// If case_sensitive is false, the pattern is treated as case-insensitive.
pub fn matches_pattern(key: &str, pattern: &str, case_sensitive: bool) -> Result<bool> {
    let matcher = CompiledMatcher::new(pattern, case_sensitive)?;
    Ok(matcher.is_match(key))
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
