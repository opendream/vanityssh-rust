// tests/cli_tests.rs
// Updated: 2025-04-22 14:07:45 by kengggg

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
        .stdout(predicate::str::contains("BEGIN OPENSSH PRIVATE KEY"));
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