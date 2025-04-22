// src/lib.rs
// Updated: 2025-04-22 14:27:00 by kengggg

pub mod error;
pub mod keygen;
pub mod matcher;
pub mod ssh;
pub mod thread_pool; // New module

use crate::error::Result;
use crate::thread_pool::{run_thread_pool, ThreadPoolConfig};
use chrono::Local;
use indicatif::{ProgressBar, ProgressStyle};
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Performance metrics for key generation
pub struct PerformanceMetrics {
    pub attempts: u64,
    pub matches_found: u64,
    pub duration: Duration,
    pub keys_per_second: f64,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for PerformanceMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Attempts: {} | Matches: {} | Duration: {:.2}s | Speed: {:.2} keys/sec",
            self.attempts,
            self.matches_found,
            self.duration.as_secs_f64(),
            self.keys_per_second
        )
    }
}

impl PerformanceMetrics {
    /// Creates a new metrics instance
    pub fn new() -> Self {
        PerformanceMetrics {
            attempts: 0,
            matches_found: 0,
            duration: Duration::from_secs(0),
            keys_per_second: 0.0,
        }
    }

    /// Updates the metrics based on current performance
    pub fn update(&mut self, attempts: u64, matches_found: u64, duration: Duration) {
        self.attempts = attempts;
        self.matches_found = matches_found;
        self.duration = duration;

        // Calculate keys per second
        let seconds = duration.as_secs_f64();
        if seconds > 0.0 {
            self.keys_per_second = attempts as f64 / seconds;
        }
    }
}

/// Continuously generates random ed25519 key pairs in OpenSSH format
/// and matches the public key against a regex pattern.
/// This is the multi-threaded version of the key generation function.
///
/// # Arguments
///
/// * `pattern` - The regex pattern to match against
/// * `streaming` - Whether to continue after finding a match
/// * `comment` - Optional comment to add to the SSH key
/// * `case_sensitive` - Whether to perform case-sensitive matching
/// * `threads` - Number of worker threads to use (default: number of CPU cores)
///
/// # Returns
///
/// Performance metrics for the operation
pub fn stream_openssh_keys_and_match_mt(
    pattern: &str,
    streaming: bool,
    comment: Option<&str>,
    case_sensitive: bool,
    threads: Option<usize>,
) -> Result<PerformanceMetrics> {
    // Determine thread count: use provided value or CPU count
    let thread_count = threads.unwrap_or_else(num_cpus::get);

    // Setup progress bar
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );

    // Performance tracking
    let start_time = Instant::now();
    let mut last_update = Instant::now();
    let update_interval = Duration::from_millis(500);

    // Create thread pool configuration
    let config = ThreadPoolConfig {
        pattern: pattern.to_string(),
        thread_count,
        case_sensitive,
        streaming,
        comment: comment.map(|s| s.to_string()),
    };

    // Start the thread pool
    let receiver = run_thread_pool(config)?;

    // Shared counters for tracking attempts and matches
    let total_attempts = Arc::new(AtomicU64::new(0));
    let matches_found = Arc::new(AtomicU64::new(0));

    // Performance metrics to return
    let mut metrics = PerformanceMetrics::new();

    // Main thread listens for matches and updates progress
    loop {
        // Check for key matches with a timeout
        match receiver.recv_timeout(update_interval) {
            Ok(key_match) => {
                // Update total attempts with the latest from the successful thread
                // This is not perfectly accurate but gives a reasonable estimate
                let attempts = total_attempts.load(Ordering::Relaxed) + key_match.attempts;
                total_attempts.store(attempts, Ordering::Relaxed);

                let matches = matches_found.fetch_add(1, Ordering::Relaxed) + 1;
                let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");

                // Update metrics
                let elapsed = start_time.elapsed();
                metrics.update(attempts, matches, elapsed);

                // Clear progress spinner when reporting a match
                pb.finish_and_clear();

                // Report the match
                println!(
                    "\n[{}] Match found after {} attempts by thread {}!",
                    timestamp, attempts, key_match.thread_id
                );
                println!("Public Key:  {}", key_match.public_key);
                println!("Private Key:\n{}", key_match.private_key);
                println!("Performance: {}", metrics);

                // If not in streaming mode, exit
                if (!streaming) {
                    break;
                }

                // Re-initialize progress bar if continuing
                pb.set_style(
                    ProgressStyle::default_spinner()
                        .template("{spinner:.green} {msg}")
                        .unwrap(),
                );
                pb.enable_steady_tick(Duration::from_millis(100));
            }
            Err(_) => {
                // Timeout occurred (no match yet), update progress
                let now = Instant::now();
                if now.duration_since(last_update) > update_interval {
                    let elapsed = now.duration_since(start_time);
                    let attempts = total_attempts.load(Ordering::Relaxed);
                    let matches = matches_found.load(Ordering::Relaxed);

                    metrics.update(attempts, matches, elapsed);
                    pb.set_message(format!("{} (Threads: {})", metrics, thread_count));

                    last_update = now;
                }
            }
        }
    }

    pb.finish_and_clear();

    // Final update to metrics
    let elapsed = start_time.elapsed();
    let attempts = total_attempts.load(Ordering::Relaxed);
    let matches = matches_found.load(Ordering::Relaxed);
    metrics.update(attempts, matches, elapsed);

    Ok(metrics)
}

// Keep the original single-threaded function for backward compatibility
pub fn stream_openssh_keys_and_match(
    pattern: &str,
    streaming: bool,
    comment: Option<&str>,
    case_sensitive: bool,
) -> Result<PerformanceMetrics> {
    // By default, use the multi-threaded version with 1 thread
    stream_openssh_keys_and_match_mt(pattern, streaming, comment, case_sensitive, Some(1))
}

// Original stream_keys_and_match for backward compatibility
pub fn stream_keys_and_match(
    pattern: &str,
    streaming: bool,
    case_sensitive: bool,
) -> Result<PerformanceMetrics> {
    // Call the multi-threaded version with 1 thread
    stream_openssh_keys_and_match_mt(pattern, streaming, None, case_sensitive, Some(1))
}

// For test helper function
#[cfg(test)]
pub fn generate_n_keys(n: u64) -> Result<PerformanceMetrics> {
    let mut metrics = PerformanceMetrics::new();
    let start = Instant::now();

    for _ in 0..n {
        let _ = keygen::generate_key_pair()?;
    }

    let duration = start.elapsed();
    metrics.update(n, 0, duration);
    Ok(metrics)
}
