// tests/tests.rs
// Updated: 2025-04-22 13:57:30 by kengggg

use ed25519_vanity_rust::{keygen, matcher, PerformanceMetrics, stream_keys_and_match};
use std::time::Duration;

#[test]
fn test_generate_key_pair() {
    let result = keygen::generate_key_pair();
    assert!(result.is_ok());

    let (public_key, private_key) = result.unwrap();

    // Check that the keys are hexadecimal strings of the expected length
    assert_eq!(public_key.len(), 64); // 32 bytes = 64 hex chars
    assert_eq!(private_key.len(), 64); // 32 bytes = 64 hex chars

    // Check that the keys contain only valid hex characters
    assert!(public_key.chars().all(|c| c.is_digit(16)));
    assert!(private_key.chars().all(|c| c.is_digit(16)));
}

#[test]
fn test_matches_pattern_valid() {
    // Test with a simple hex pattern that should match
    let hex_key = "aabbccddeeff00112233445566778899";

    // Use case-insensitive pattern with (?i) prefix
    let result = matcher::matches_pattern(hex_key, "(?i)AABBCC");

    assert!(result.is_ok(), "Should return Ok result");
    assert!(result.unwrap(), "Should match the case-insensitive pattern 'AABBCC'");

    // Test with a pattern that shouldn't match
    let result = matcher::matches_pattern(hex_key, "xyz");

    assert!(result.is_ok(), "Should return Ok result");
    assert!(!result.unwrap(), "Should not match the pattern 'xyz'");
}

#[test]
fn test_matches_pattern_invalid_regex() {
    // Test with an invalid regex pattern
    let hex_key = "aabbccddeeff00112233445566778899";
    let result = matcher::matches_pattern(hex_key, "["); // Unclosed character class

    assert!(result.is_err(), "Should return an error for invalid regex");
}

#[test]
fn test_performance_metrics_calculation() {
    let mut metrics = PerformanceMetrics::new();

    // Test initial state
    assert_eq!(metrics.attempts, 0);
    assert_eq!(metrics.matches_found, 0);
    assert_eq!(metrics.duration, Duration::from_secs(0));
    assert_eq!(metrics.keys_per_second, 0.0);

    // Update metrics
    metrics.update(100, 1, Duration::from_secs(2));

    assert_eq!(metrics.attempts, 100);
    assert_eq!(metrics.matches_found, 1);
    assert_eq!(metrics.duration, Duration::from_secs(2));
    assert_eq!(metrics.keys_per_second, 50.0); // 100 keys / 2 seconds

    // Validate string format
    let metrics_str = metrics.to_string();
    assert!(metrics_str.contains("Attempts: 100"));
    assert!(metrics_str.contains("Matches: 1"));
    assert!(metrics_str.contains("Speed: 50.00 keys/sec"));
}

#[test]
fn test_stream_keys_with_simple_pattern() {
    // Use a pattern that should match any key (will match very quickly)
    let result = stream_keys_and_match(".", false);

    assert!(result.is_ok());

    let metrics = result.unwrap();
    assert_eq!(metrics.matches_found, 1);
    assert!(metrics.attempts > 0);
    assert!(metrics.keys_per_second > 0.0);
}