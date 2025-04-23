// src/lib.rs
// Updated: 2025-04-22 15:50:00 by kengggg

pub mod error;
pub mod keygen;
pub mod matcher;
pub mod ssh;
pub mod thread_pool;

use crate::error::Result;
use crate::thread_pool::{run_thread_pool, ThreadPoolConfig};
use chrono::Local;
use crossbeam_channel::select;
use indicatif::{ProgressBar, ProgressStyle};
use std::fmt;
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
    let mut pb = ProgressBar::new_spinner();
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
    let (match_receiver, status_receiver) = run_thread_pool(config)?;

    // Track attempts and matches
    let mut total_attempts: u64 = 0;
    let mut matches_found: u64 = 0;

    // Performance metrics to return
    let mut metrics = PerformanceMetrics::new();

    // Enable steady spinner tick
    pb.enable_steady_tick(Duration::from_millis(100));

    loop {
        // Use crossbeam's select! to handle multiple channels
        select! {
            // Handle key matches
            recv(match_receiver) -> msg => {
                if let Ok(key_match) = msg {
                    // Update counters with the match information
                    total_attempts += key_match.attempts;
                    matches_found += 1;

                    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
                    let elapsed = start_time.elapsed();

                    // Update metrics
                    metrics.update(total_attempts, matches_found, elapsed);

                    // Clear progress spinner when reporting a match
                    pb.finish_and_clear();

                    // Report the match
                    println!(
                        "\n[{}] Match found after {} attempts by thread {}!",
                        timestamp, key_match.attempts, key_match.thread_id
                    );
                    println!("Public Key:  {}", key_match.public_key);
                    println!("Private Key:\n{}", key_match.private_key);
                    println!("Performance: {}", metrics);

                    // If not in streaming mode, exit
                    if !streaming {
                        return Ok(metrics);
                    }

                    // In streaming mode, we need to completely recreate the progress bar
                    // rather than just reinitializing it
                    pb = ProgressBar::new_spinner();
                    pb.set_style(
                        ProgressStyle::default_spinner()
                            .template("{spinner:.green} {msg}")
                            .unwrap(),
                    );
                    pb.enable_steady_tick(Duration::from_millis(100));

                    // Reset update timer to ensure immediate refresh
                    last_update = Instant::now().checked_sub(update_interval * 2).unwrap_or(Instant::now());

                    // Force an immediate update of the progress display with a clear message
                    // that indicates we're continuing the search
                    let now = Instant::now();
                    let elapsed = now.duration_since(start_time);
                    metrics.update(total_attempts, matches_found, elapsed);

                    // Add a newline before continuing to ensure progress bar appears on its own line
                    println!("\nContinuing search for more matches...");

                    pb.set_message(format!("Attempts: {} | Matches: {} | Duration: {:.2}s | Speed: {:.2} keys/sec (Threads: {})",
                        total_attempts, matches_found, elapsed.as_secs_f64(), metrics.keys_per_second, thread_count));
                } else {
                    // Channel closed, exit
                    break;
                }
            },

            // Handle status updates
            recv(status_receiver) -> msg => {
                if let Ok(status) = msg {
                    // Update attempt counter
                    total_attempts += status.attempts;

                    // Refresh display if update interval has passed
                    let now = Instant::now();
                    if now.duration_since(last_update) >= update_interval {
                        let elapsed = now.duration_since(start_time);
                        metrics.update(total_attempts, matches_found, elapsed);
                        pb.set_message(format!("Attempts: {} | Matches: {} | Duration: {:.2}s | Speed: {:.2} keys/sec (Threads: {})",
                            total_attempts, matches_found, elapsed.as_secs_f64(), metrics.keys_per_second, thread_count));
                        last_update = now;
                    }
                }
            },

            // Handle timeout to update display even if no status updates received
            default(update_interval) => {
                let now = Instant::now();
                let elapsed = now.duration_since(start_time);
                metrics.update(total_attempts, matches_found, elapsed);
                pb.set_message(format!("Attempts: {} | Matches: {} | Duration: {:.2}s | Speed: {:.2} keys/sec (Threads: {})",
                    total_attempts, matches_found, elapsed.as_secs_f64(), metrics.keys_per_second, thread_count));
                last_update = now;
            }
        }
    }

    pb.finish_and_clear();

    // Final update to metrics
    let elapsed = start_time.elapsed();
    metrics.update(total_attempts, matches_found, elapsed);

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
