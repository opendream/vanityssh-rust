// src/thread_pool.rs
// Updated: 2025-04-22 15:45:00 by kengggg

use crate::error::Result;
use crate::keygen;
use crate::matcher::CompiledMatcher;
use crossbeam_channel::{bounded, Receiver};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;

/// Represents a match found by a worker thread
pub struct KeyMatch {
    pub public_key: String,
    pub private_key: String,
    pub attempts: u64,
    pub thread_id: usize,
}

/// Represents a status update from worker threads
pub struct StatusUpdate {
    pub attempts: u64,
}

/// Configuration for the thread pool
pub struct ThreadPoolConfig {
    pub pattern: String,
    pub thread_count: usize,
    pub case_sensitive: bool,
    pub streaming: bool,
    pub comment: Option<String>,
}

/// Handle for managing thread pool lifecycle
pub struct ThreadPoolHandle {
    thread_handles: Vec<thread::JoinHandle<()>>,
    terminate_flag: Arc<AtomicBool>,
}

impl ThreadPoolHandle {
    /// Signal all threads to terminate gracefully
    pub fn terminate(&self) {
        self.terminate_flag.store(true, Ordering::Relaxed);
    }

    /// Wait for all threads to complete and join them
    pub fn join(self) -> Result<()> {
        // Signal termination first
        self.terminate();

        // Wait for all threads to complete
        for handle in self.thread_handles {
            if handle.join().is_err() {
                // Thread panicked, but we continue joining others
                eprintln!("Warning: Worker thread panicked during shutdown");
            }
        }

        Ok(())
    }
}

/// Creates and manages a thread pool for generating and matching keys
/// Returns receivers and a handle to manage thread cleanup
pub fn run_thread_pool(
    config: ThreadPoolConfig,
) -> Result<(Receiver<KeyMatch>, Receiver<StatusUpdate>, ThreadPoolHandle)> {
    let thread_count = config.thread_count;
    let pattern = config.pattern;
    let case_sensitive = config.case_sensitive;
    let streaming = config.streaming;
    let comment = config.comment;

    // Set up communication channels
    let (match_sender, match_receiver) = bounded::<KeyMatch>(32);
    let (status_sender, status_receiver) = bounded::<StatusUpdate>(128);

    // Shared state for thread coordination
    let terminate = Arc::new(AtomicBool::new(false));
    let mut thread_handles = Vec::new();

    // Spawn worker threads
    for thread_id in 0..thread_count {
        let thread_match_sender = match_sender.clone();
        let thread_status_sender = status_sender.clone();
        let thread_pattern = pattern.clone();
        let thread_comment = comment.clone();
        let thread_terminate = Arc::clone(&terminate);

        let handle = thread::spawn(move || {
            let mut local_attempts: u64 = 0;
            let mut last_reported = 0;
            let batch_size = 50; // Report every 50 attempts

            // Pre-compile the regex pattern once per thread for maximum performance
            let compiled_matcher = match CompiledMatcher::new(&thread_pattern, case_sensitive) {
                Ok(matcher) => matcher,
                Err(_) => {
                    // Invalid regex pattern, exit thread
                    return;
                }
            };

            // Worker thread loop
            while !thread_terminate.load(Ordering::Relaxed) {
                // Generate a key pair
                local_attempts += 1;

                // Report progress regularly
                if local_attempts - last_reported >= batch_size {
                    // Send status update to main thread
                    let _ = thread_status_sender.send(StatusUpdate {
                        attempts: batch_size,
                    });
                    last_reported = local_attempts;
                }

                // Generate key
                if let Ok((public_key, private_key)) = match thread_comment {
                    Some(ref c) => keygen::generate_openssh_key_pair(Some(c)),
                    None => keygen::generate_openssh_key_pair(None),
                } {
                    // Check if it matches the pattern using pre-compiled matcher
                    match compiled_matcher.ssh_key_matches(&public_key) {
                        Ok(true) => {
                            // Found a match!
                            // Report any remaining attempts
                            let remaining = local_attempts - last_reported;
                            if remaining > 0 {
                                let _ = thread_status_sender.send(StatusUpdate {
                                    attempts: remaining,
                                });
                            }

                            let key_match = KeyMatch {
                                public_key,
                                private_key,
                                attempts: local_attempts,
                                thread_id,
                            };

                            // Send the match back to the main thread
                            if thread_match_sender.send(key_match).is_err() {
                                // Channel closed, exit thread
                                break;
                            }

                            // If not streaming, signal termination
                            if !streaming {
                                thread_terminate.store(true, Ordering::Relaxed);
                                break;
                            }
                        }
                        Ok(false) => {
                            // No match, continue
                        }
                        Err(_) => {
                            // Error matching, just continue
                        }
                    }
                }
            }

            // Final update for any remaining attempts
            let remaining = local_attempts - last_reported;
            if remaining > 0 {
                let _ = thread_status_sender.send(StatusUpdate {
                    attempts: remaining,
                });
            }
        });

        thread_handles.push(handle);
    }

    // Create thread pool handle for cleanup
    let pool_handle = ThreadPoolHandle {
        thread_handles,
        terminate_flag: Arc::clone(&terminate),
    };

    // Return the receiver channels and handle for the main thread
    Ok((match_receiver, status_receiver, pool_handle))
}

/// Signal all threads to terminate
pub fn terminate_all(flag: &AtomicBool) {
    flag.store(true, Ordering::Relaxed);
}
