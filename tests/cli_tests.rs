// tests/cli_tests.rs
// Updated: 2025-04-22 12:28:57 by kengggg

use assert_cmd::Command;
use predicates::prelude::*;
use std::time::Duration;

#[test]
fn test_cli_no_args() {
    // Run the command with no arguments
    let mut cmd = Command::cargo_bin("ed25519-vanity-rust").unwrap();

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_cli_with_simple_pattern() {
    // Run the command with a simple pattern that should match quickly
    let mut cmd = Command::cargo_bin("ed25519-vanity-rust").unwrap();

    cmd.arg(".*")
        .timeout(Duration::from_secs(10)) // Now using the Duration import
        .assert()
        .success()
        .stdout(predicate::str::contains("Match found after"))
        .stdout(predicate::str::contains("Public Key:"))
        .stdout(predicate::str::contains("Private Key:"));
}

#[test]
fn test_cli_with_invalid_regex() {
    // Run the command with an invalid regex pattern
    let mut cmd = Command::cargo_bin("ed25519-vanity-rust").unwrap();

    cmd.arg("[")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error:"))
        .stderr(predicate::str::contains("Invalid regex pattern:"));
}