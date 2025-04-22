// tests/cli_tests.rs
// Updated: 2025-04-22 14:21:00 by kengggg
// Fixed test timeouts and output verification

use assert_cmd::Command;
use predicates::prelude::*;
use std::time::Duration;

#[test]
fn test_cli_no_args() {
    let mut cmd = Command::cargo_bin("ed25519-vanity-rust").unwrap();

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_cli_with_simple_pattern() {
    let mut cmd = Command::cargo_bin("ed25519-vanity-rust").unwrap();

    cmd.arg(".*")
        .timeout(Duration::from_secs(10))
        .assert()
        .success()
        .stdout(predicate::str::contains("Match found"));
}

#[test]
fn test_cli_with_invalid_regex() {
    let mut cmd = Command::cargo_bin("ed25519-vanity-rust").unwrap();

    cmd.arg("[")  // Invalid regex pattern (unclosed character class)
        .timeout(Duration::from_secs(5))
        .assert()
        .failure()
        .stderr(predicate::str::contains("InvalidRegex"));
}

#[test]
fn test_cli_with_openssh_format() {
    let mut cmd = Command::cargo_bin("ed25519-vanity-rust").unwrap();

    cmd.arg(".*")
        .arg("--comment")
        .arg("test@example.com")
        .timeout(Duration::from_secs(10))
        .assert()
        .success()
        .stdout(predicate::str::contains("ssh-ed25519"))
        .stdout(predicate::str::contains("BEGIN OPENSSH PRIVATE KEY"))
        .stdout(predicate::str::contains("test@example.com"));
}

#[test]
fn test_cli_with_case_sensitive_option() {
    let mut cmd = Command::cargo_bin("ed25519-vanity-rust").unwrap();

    cmd.arg(".*")
        .arg("--case-sensitive")
        .timeout(Duration::from_secs(10))
        .assert()
        .success()
        .stdout(predicate::str::contains("Match found"));
}

// FIXED TEST: For --streaming option, we kill the process after 2 seconds
// and just verify that it started successfully
#[test]
fn test_cli_with_streaming_option() {
    let mut cmd = Command::cargo_bin("ed25519-vanity-rust").unwrap();

    // We need a pattern that will match quickly and frequently
    let output = cmd
        .arg(".*")  // Match anything
        .arg("--streaming")
        .timeout(Duration::from_secs(2))  // Short timeout - process will be killed
        .output()
        .expect("Failed to execute command");

    // In streaming mode, we expect the process to be killed by timeout
    // So we just check if the output contains a match before being killed
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Match found"), "Should find at least one match before timeout");
}

// Test for combination of options
#[test]
fn test_cli_with_option_combinations() {
    let mut cmd = Command::cargo_bin("ed25519-vanity-rust").unwrap();

    // Combine multiple options
    cmd.arg(".*")
        .arg("--case-sensitive")
        .arg("--comment")
        .arg("test@example.com")
        .timeout(Duration::from_secs(10))
        .assert()
        .success()
        .stdout(predicate::str::contains("Match found"))
        .stdout(predicate::str::contains("test@example.com"));
}

// FIXED TEST: Test option order independence with proper kill handling
#[test]
fn test_cli_option_order_independence() {
    let mut cmd = Command::cargo_bin("ed25519-vanity-rust").unwrap();

    // Put options in a different order
    cmd.arg("--comment")
        .arg("test@example.com")
        .arg("--case-sensitive")
        .arg(".*")  // Pattern at the end
        .timeout(Duration::from_secs(10))
        .assert()
        .success()
        .stdout(predicate::str::contains("Match found"))
        .stdout(predicate::str::contains("test@example.com"));
}

// NEW TEST: Test the help option
#[test]
fn test_cli_help_option() {
    let mut cmd = Command::cargo_bin("ed25519-vanity-rust").unwrap();

    cmd.arg("--help")
        .assert()
        .success()
        .stderr(predicate::str::contains("Usage:"))
        .stderr(predicate::str::contains("pattern"));
}