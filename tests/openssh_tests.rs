// tests/openssh_tests.rs
// Updated: 2025-04-22 14:16:30 by kengggg
// Removed match_full parameter from tests

use std::fs;
use std::path::Path;
use vanityssh_rust::{keygen, matcher};

#[test]
fn test_openssh_key_format() {
    // Generate an OpenSSH key pair
    let (public_key, private_key) =
        keygen::generate_openssh_key_pair(Some("test@example.com")).unwrap();

    // Verify public key format
    assert!(public_key.starts_with("ssh-ed25519 "));
    assert!(public_key.ends_with("test@example.com"));

    // Verify private key format
    assert!(private_key.starts_with("-----BEGIN OPENSSH PRIVATE KEY-----"));
    assert!(private_key.ends_with("-----END OPENSSH PRIVATE KEY-----"));
}

#[test]
fn test_ssh_key_matching() {
    // Generate an OpenSSH key pair
    let (public_key, _) = keygen::generate_openssh_key_pair(None).unwrap();

    // Extract base64 part
    let parts: Vec<&str> = public_key.split_whitespace().collect();
    let base64_part = parts[1];

    // Test that matching works on the base64 part with a lowercase pattern
    // This should work with case-insensitive matching
    let lower_pattern = "a";
    assert!(matcher::ssh_key_matches_pattern(&public_key, lower_pattern, false).unwrap());

    // Test that case-sensitive matching works properly with an exact pattern
    let exact_prefix = &base64_part[0..5]; // Get exact prefix for case-sensitive match
    assert!(
        matcher::ssh_key_matches_pattern(&public_key, exact_prefix, true).unwrap(),
        "Case-sensitive matching should work with exact case"
    );

    // Test that case-sensitive matching fails with wrong case
    // This ensures that case sensitivity is working as expected
    let wrong_case = if exact_prefix.chars().next().unwrap().is_uppercase() {
        exact_prefix.to_lowercase()
    } else {
        exact_prefix.to_uppercase()
    };

    assert!(
        !matcher::ssh_key_matches_pattern(&public_key, &wrong_case, true).unwrap(),
        "Case-sensitive matching should fail with wrong case"
    );
}

#[test]
fn test_ssh_key_file_operations() {
    // Generate an OpenSSH key pair
    let (public_key, private_key) = keygen::generate_openssh_key_pair(None).unwrap();

    // Create test directory if it doesn't exist
    let test_dir = Path::new("./test_keys");
    if !test_dir.exists() {
        fs::create_dir(test_dir).unwrap();
    }

    // Write keys to files - use references (borrowed values) instead of moving the strings
    fs::write(test_dir.join("id_ed25519"), &private_key).unwrap();
    fs::write(test_dir.join("id_ed25519.pub"), &public_key).unwrap();

    // Read back and verify
    let read_pub = fs::read_to_string(test_dir.join("id_ed25519.pub")).unwrap();
    let read_priv = fs::read_to_string(test_dir.join("id_ed25519")).unwrap();

    assert_eq!(public_key, read_pub);
    assert_eq!(private_key, read_priv);

    // Clean up
    fs::remove_file(test_dir.join("id_ed25519")).unwrap();
    fs::remove_file(test_dir.join("id_ed25519.pub")).unwrap();
    fs::remove_dir(test_dir).unwrap();
}
