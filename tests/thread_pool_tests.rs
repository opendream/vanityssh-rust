// tests/thread_pool_tests.rs
// Created: 2025-04-22 14:30:00 by kengggg

use ed25519_vanity_rust::thread_pool::{ThreadPoolConfig, run_thread_pool};
use std::time::Duration;

#[test]
fn test_thread_pool_basic() {
    // Create a thread pool with 2 threads
    let config = ThreadPoolConfig {
        pattern: ".*".to_string(),  // Match anything
        thread_count: 2,
        case_sensitive: false,
        streaming: false,
        comment: None,
    };

    // Run the thread pool
    let receiver = run_thread_pool(config).unwrap();

    // Wait for a match
    let key_match = receiver.recv_timeout(Duration::from_secs(10)).unwrap();

    // Verify we got a valid key match
    assert!(!key_match.public_key.is_empty());
    assert!(!key_match.private_key.is_empty());
    assert!(key_match.attempts > 0);
    assert!(key_match.thread_id < 2);  // Should be thread 0 or 1
}

#[test]
fn test_thread_pool_streaming() {
    // Create a thread pool with 2 threads in streaming mode
    let config = ThreadPoolConfig {
        pattern: ".*".to_string(),  // Match anything
        thread_count: 2,
        case_sensitive: false,
        streaming: true,  // Streaming mode
        comment: None,
    };

    // Run the thread pool
    let receiver = run_thread_pool(config).unwrap();

    // Get multiple matches
    let mut matches = 0;
    let start = std::time::Instant::now();

    // Get up to 3 matches or timeout after 10 seconds
    while matches < 3 && start.elapsed() < Duration::from_secs(10) {
        if let Ok(_) = receiver.recv_timeout(Duration::from_secs(1)) {
            matches += 1;
        }
    }

    // Should get at least one match
    assert!(matches > 0);
}