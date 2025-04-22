// src/thread_pool.rs
// Updated: 2025-04-22 14:32:00 by kengggg

use std::sync::{Arc, atomic::{AtomicBool, AtomicU64, Ordering}};
use std::thread;
use crossbeam_channel::{bounded, Receiver}; // Removed unused Sender import
use crate::error::Result;
use crate::keygen;
use crate::matcher;

/// Represents a match found by a worker thread
pub struct KeyMatch {
    pub public_key: String,
    pub private_key: String,
    pub attempts: u64,
    pub thread_id: usize,
}

/// Configuration for the thread pool
pub struct ThreadPoolConfig {
    pub pattern: String,
    pub thread_count: usize,
    pub case_sensitive: bool,
    pub streaming: bool,
    pub comment: Option<String>,
}

/// Creates and manages a thread pool for generating and matching keys
pub fn run_thread_pool(config: ThreadPoolConfig) -> Result<Receiver<KeyMatch>> {
    let thread_count = config.thread_count;
    let pattern = config.pattern;
    let case_sensitive = config.case_sensitive;
    let streaming = config.streaming;
    let comment = config.comment;

    // Set up communication channels
    let (sender, receiver) = bounded::<KeyMatch>(32);

    // Shared state
    let total_attempts = Arc::new(AtomicU64::new(0));
    let terminate = Arc::new(AtomicBool::new(false));

    // Spawn worker threads
    for thread_id in 0..thread_count {
        let thread_sender = sender.clone();
        let thread_pattern = pattern.clone();
        let thread_comment = comment.clone();
        let thread_attempts = Arc::clone(&total_attempts);
        let thread_terminate = Arc::clone(&terminate);

        thread::spawn(move || {
            let mut local_attempts: u64 = 0;

            // Worker thread loop
            while !thread_terminate.load(Ordering::Relaxed) {
                // Generate a key pair
                local_attempts += 1;

                if local_attempts % 1000 == 0 {
                    // Update shared attempt counter occasionally to reduce contention
                    thread_attempts.fetch_add(1000, Ordering::Relaxed);
                }

                // Generate key
                if let Ok((public_key, private_key)) = match thread_comment {
                    Some(ref c) => keygen::generate_openssh_key_pair(Some(c)),
                    None => keygen::generate_openssh_key_pair(None),
                } {
                    // Check if it matches the pattern
                    match matcher::ssh_key_matches_pattern(&public_key, &thread_pattern, case_sensitive) {
                        Ok(true) => {
                            // Found a match!
                            let key_match = KeyMatch {
                                public_key,
                                private_key,
                                attempts: local_attempts,
                                thread_id,
                            };

                            // Send the match back to the main thread
                            if thread_sender.send(key_match).is_err() {
                                // Channel closed, exit thread
                                break;
                            }

                            // If not streaming, signal termination
                            if !streaming {
                                thread_terminate.store(true, Ordering::Relaxed);
                                break;
                            }
                        },
                        Ok(false) => {
                            // No match, continue
                        },
                        Err(_) => {
                            // Error matching, just continue
                        }
                    }
                }
            }

            // Final update of the attempt counter
            let remaining = local_attempts % 1000;
            if remaining > 0 {
                thread_attempts.fetch_add(remaining, Ordering::Relaxed);
            }
        });
    }

    // Return the receiver channel for the main thread to listen on
    Ok(receiver)
}

/// Get the total number of attempts from all threads
pub fn get_total_attempts(counter: &AtomicU64) -> u64 {
    counter.load(Ordering::Relaxed)
}

/// Signal all threads to terminate
pub fn terminate_all(flag: &AtomicBool) {
    flag.store(true, Ordering::Relaxed);
}