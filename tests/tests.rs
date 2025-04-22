// tests/tests.rs
// Updated: 2025-04-22 12:12:57 by kengggg

use ed25519_vanity_rust::keygen;
use ed25519_vanity_rust::matcher;
use ed25519_vanity_rust::stream_keys_and_match;
use ed25519_vanity_rust::error::VanityError;
use std::time::Duration;

#[test]
fn test_generate_key_pair() {
    let result = keygen::generate_key_pair();
    assert!(result.is_ok(), "Key generation should succeed");

    let (public_key, private_key) = result.unwrap();

    // Ensure the public key is a valid hex string of length 64
    assert_eq!(public_key.len(), 64, "Public key length is incorrect");
    assert!(hex::decode(&public_key).is_ok(), "Public key is not valid hex");

    // Ensure the private key is a valid hex string of length 64
    assert_eq!(private_key.len(), 64, "Private key length is incorrect");
    assert!(hex::decode(&private_key).is_ok(), "Private key is not valid hex");
}

#[test]
fn test_matches_pattern_valid() {
    let public_key = "aabbccddeeff00112233445566778899aabbccddeeff00112233445566778899";

    // Test exact match
    let result = matcher::matches_pattern(&public_key, "aabbcc");
    assert!(result.is_ok(), "Pattern matching should succeed");
    assert!(result.unwrap(), "Should match the pattern 'aabbcc'");

    // Test case-insensitive match
    let result = matcher::matches_pattern(&public_key, "AABBCC");
    assert!(result.is_ok(), "Pattern matching should succeed");
    assert!(result.unwrap(), "Should match the case-insensitive pattern 'AABBCC'");

    // Test regex match
    let result = matcher::matches_pattern(&public_key, "aabb.*99");
    assert!(result.is_ok(), "Pattern matching should succeed");
    assert!(result.unwrap(), "Should match the regex pattern 'aabb.*99'");

    // Test non-match
    let result = matcher::matches_pattern(&public_key, "zzzz");
    assert!(result.is_ok(), "Pattern matching should succeed");
    assert!(!result.unwrap(), "Should not match the pattern 'zzzz'");
}

#[test]
fn test_matches_pattern_invalid_regex() {
    let public_key = "aabbccddeeff00112233445566778899aabbccddeeff00112233445566778899";

    // Test invalid regex
    let result = matcher::matches_pattern(&public_key, "[");
    assert!(result.is_err(), "Invalid regex should return an error");

    // Check the error is the expected type
    match result {
        Err(VanityError::InvalidRegex(_)) => assert!(true),
        _ => panic!("Expected InvalidRegex error"),
    }
}

#[test]
fn test_stream_keys_with_simple_pattern() {
    // Use a pattern that will match nearly immediately (".*")
    let result = stream_keys_and_match(".*", false);
    assert!(result.is_ok(), "Stream keys should succeed with valid pattern");

    let metrics = result.unwrap();
    assert!(metrics.attempts > 0, "Should have at least one attempt");
    assert_eq!(metrics.matches_found, 1, "Should have found exactly one match");
    assert!(metrics.duration > Duration::from_nanos(0), "Duration should be positive");
    assert!(metrics.keys_per_second > 0.0, "Keys per second should be positive");
}

#[test]
fn test_performance_metrics_calculation() {
    // Import the PerformanceMetrics struct for this test
    use ed25519_vanity_rust::PerformanceMetrics;

    let mut metrics = PerformanceMetrics::new();

    // Initialize with some values
    metrics.update(100, 2, Duration::from_secs(5));

    // Test calculations
    assert_eq!(metrics.attempts, 100);
    assert_eq!(metrics.matches_found, 2);
    assert_eq!(metrics.duration, Duration::from_secs(5));
    assert_eq!(metrics.keys_per_second, 20.0); // 100 keys in 5 seconds = 20 keys/sec

    // Test string representation
    let metrics_string = metrics.to_string();
    assert!(metrics_string.contains("Attempts: 100"));
    assert!(metrics_string.contains("Matches: 2"));
    assert!(metrics_string.contains("20.00 keys/sec"));
}