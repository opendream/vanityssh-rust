// tests/cli_tests.rs
// Updated 2025-04-22 15:10:00 by kengggg

use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use std::time::Duration;

#[test]
fn test_cli_no_args() {
    let mut cmd = Command::cargo_bin("vanityssh-rust").unwrap();

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_cli_with_simple_pattern() {
    let mut cmd = Command::cargo_bin("vanityssh-rust").unwrap();

    // Setting a timeout isn't directly available on Command
    // We'll use a simpler approach of just running the command
    cmd.arg(".*")
        .assert()
        .success()
        .stdout(predicate::str::contains("Match found"));
}

#[test]
fn test_cli_with_invalid_regex() {
    let mut cmd = Command::cargo_bin("vanityssh-rust").unwrap();

    cmd.arg("[") // Invalid regex pattern (unclosed character class)
        .assert()
        .failure()
        // Check for specific error message about invalid regex
        .stderr(predicate::str::contains("Invalid regex"));
}

#[test]
fn test_cli_with_openssh_format() {
    let mut cmd = Command::cargo_bin("vanityssh-rust").unwrap();

    cmd.arg(".*")
        .arg("--comment")
        .arg("test@example.com")
        .assert()
        .success()
        .stdout(predicate::str::contains("ssh-ed25519"))
        .stdout(predicate::str::contains("BEGIN OPENSSH PRIVATE KEY"))
        .stdout(predicate::str::contains("test@example.com"));
}

#[test]
fn test_cli_with_case_sensitive_option() {
    let mut cmd = Command::cargo_bin("vanityssh-rust").unwrap();

    cmd.arg(".*")
        .arg("--case-sensitive")
        .assert()
        .success()
        .stdout(predicate::str::contains("Match found"));
}

// For --streaming option, we'll use with_kill_on_drop or similar if available
// otherwise we just test that it starts correctly
#[test]
#[ignore] // Ignoring this test since it can run indefinitely
fn test_cli_with_streaming_option() {
    let mut cmd = Command::cargo_bin("vanityssh-rust").unwrap();

    // We need a pattern that will match quickly and frequently
    let output = cmd
        .arg(".*") // Match anything
        .arg("--streaming")
        .output()
        .expect("Failed to execute command");

    // Check if at least one match was found before the command was terminated
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Match found"),
        "Should find at least one match"
    );
}

// Test for combination of options
#[test]
fn test_cli_with_option_combinations() {
    let mut cmd = Command::cargo_bin("vanityssh-rust").unwrap();

    // Combine multiple options
    cmd.arg(".*")
        .arg("--case-sensitive")
        .arg("--comment")
        .arg("test@example.com")
        .assert()
        .success()
        .stdout(predicate::str::contains("Match found"))
        .stdout(predicate::str::contains("test@example.com"));
}

// Test option order independence
#[test]
fn test_cli_option_order_independence() {
    let mut cmd = Command::cargo_bin("vanityssh-rust").unwrap();

    // Put options in a different order
    cmd.arg("--comment")
        .arg("test@example.com")
        .arg("--case-sensitive")
        .arg(".*") // Pattern at the end
        .assert()
        .success()
        .stdout(predicate::str::contains("Match found"))
        .stdout(predicate::str::contains("test@example.com"));
}

// Test the help option
#[test]
fn test_cli_help_option() {
    let mut cmd = Command::cargo_bin("vanityssh-rust").unwrap();

    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("VanitySSH"))
        .stdout(predicate::str::contains(
            "Usage: vanityssh-rust <pattern> [OPTIONS]",
        ))
        .stdout(predicate::str::contains("pattern"));
}

#[test]
fn test_cli_with_threads_option() {
    let mut cmd = Command::cargo_bin("vanityssh-rust").unwrap();

    cmd.arg(".*")
        .arg("--threads")
        .arg("2") // Use 2 threads explicitly
        .assert()
        .success()
        .stdout(predicate::str::contains("Using 2 threads"))
        .stdout(predicate::str::contains("Match found"));
}

// Test multiple options including threads
#[test]
fn test_cli_with_threads_and_other_options() {
    let mut cmd = Command::cargo_bin("vanityssh-rust").unwrap();

    cmd.arg(".*")
        .arg("--threads")
        .arg("2")
        .arg("--comment")
        .arg("test@example.com")
        .assert()
        .success()
        .stdout(predicate::str::contains("Using 2 threads"))
        .stdout(predicate::str::contains("Match found"))
        .stdout(predicate::str::contains("test@example.com"));
}
