// src/lib.rs
// Updated: 2025-04-22 14:15:30 by kengggg
// Removed match_full parameter from stream_openssh_keys_and_match

pub mod keygen;
pub mod matcher;
pub mod error;
pub mod ssh;

use std::time::{Duration, Instant};
use indicatif::{ProgressBar, ProgressStyle};
use chrono::Local;
use crate::error::Result;

/// Performance metrics for key generation
pub struct PerformanceMetrics {
    pub attempts: u64,
    pub matches_found: u64,
    pub duration: Duration,
    pub keys_per_second: f64,
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

    /// Returns a formatted string representation of the metrics
    pub fn to_string(&self) -> String {
        format!(
            "Attempts: {} | Matches: {} | Duration: {:.2}s | Speed: {:.2} keys/sec",
            self.attempts,
            self.matches_found,
            self.duration.as_secs_f64(),
            self.keys_per_second
        )
    }
}

/// Continuously generates random ed25519 key pairs in OpenSSH format
/// and matches the public key against a regex pattern.
/// If `streaming` is `true`, it continues generating keys even after a match is found.
/// Returns performance metrics for the operation.
///
/// The `comment` parameter will be used in the SSH public key if provided.
/// The `case_sensitive` parameter determines whether the pattern matching is case sensitive.
pub fn stream_openssh_keys_and_match(
    pattern: &str,
    streaming: bool,
    comment: Option<&str>,
    case_sensitive: bool
) -> Result<PerformanceMetrics> {
    // Initialize progress bar
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap()
    );

    // Performance tracking
    let start_time = Instant::now();
    let mut last_update = Instant::now();
    let update_interval = Duration::from_millis(500);

    // Counters
    let mut count: u64 = 0;
    let mut matches_found: u64 = 0;
    let mut metrics = PerformanceMetrics::new();

    loop {
        count += 1;

        // Generate a random OpenSSH key pair
        let (public_key, private_key) = keygen::generate_openssh_key_pair(comment)?;

        // Update progress bar periodically
        let now = Instant::now();
        if now.duration_since(last_update) > update_interval {
            let elapsed = now.duration_since(start_time);
            metrics.update(count, matches_found, elapsed);

            pb.set_message(format!("{}", metrics.to_string()));
            last_update = now;
        }

        // Check if the key matches the pattern
        match matcher::ssh_key_matches_pattern(&public_key, pattern, case_sensitive) {
            Ok(true) => {
                matches_found += 1;
                let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");

                // Update metrics
                let elapsed = now.duration_since(start_time);
                metrics.update(count, matches_found, elapsed);

                // Clear progress spinner when reporting a match
                pb.finish_and_clear();

                println!("\n[{}] Match found after {} attempts!", timestamp, count);
                println!("Public Key:  {}", public_key);
                println!("Private Key:\n{}", private_key);
                println!("Performance: {}", metrics.to_string());

                // Re-initialize progress bar if in streaming mode
                if streaming {
                    pb.set_style(
                        ProgressStyle::default_spinner()
                            .template("{spinner:.green} {msg}")
                            .unwrap()
                    );
                    pb.enable_steady_tick(Duration::from_millis(100));
                } else {
                    // Break the loop if not in streaming mode
                    break;
                }
            }
            Ok(false) => {
                // Do nothing, continue generating keys
            }
            Err(e) => {
                pb.finish_and_clear();
                return Err(e);
            }
        }
    }

    pb.finish_and_clear();

    // Final update to metrics
    let elapsed = start_time.elapsed();
    metrics.update(count, matches_found, elapsed);

    Ok(metrics)
}

// Keep the original function for compatibility, but simplify parameters
pub fn stream_keys_and_match(pattern: &str, streaming: bool, case_sensitive: bool) -> Result<PerformanceMetrics> {
    // Initialize progress bar
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap()
    );

    // Performance tracking
    let start_time = Instant::now();
    let mut last_update = Instant::now();
    let update_interval = Duration::from_millis(500);

    // Counters
    let mut count: u64 = 0;
    let mut matches_found: u64 = 0;
    let mut metrics = PerformanceMetrics::new();

    loop {
        count += 1;

        // Generate a random key pair
        let (public_key, private_key) = keygen::generate_key_pair()?;

        // Update progress bar periodically
        let now = Instant::now();
        if now.duration_since(last_update) > update_interval {
            let elapsed = now.duration_since(start_time);
            metrics.update(count, matches_found, elapsed);

            pb.set_message(format!("{}", metrics.to_string()));
            last_update = now;
        }

        // Check if the key matches the pattern
        match matcher::matches_pattern(&public_key, pattern, case_sensitive) {
            Ok(true) => {
                matches_found += 1;
                let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");

                // Update metrics
                let elapsed = now.duration_since(start_time);
                metrics.update(count, matches_found, elapsed);

                // Clear progress spinner when reporting a match
                pb.finish_and_clear();

                println!("\n[{}] Match found after {} attempts!", timestamp, count);
                println!("Public Key:  {}", public_key);
                println!("Private Key: {}", private_key);
                println!("Performance: {}", metrics.to_string());

                // Re-initialize progress bar if in streaming mode
                if streaming {
                    pb.set_style(
                        ProgressStyle::default_spinner()
                            .template("{spinner:.green} {msg}")
                            .unwrap()
                    );
                    pb.enable_steady_tick(Duration::from_millis(100));
                } else {
                    // Break the loop if not in streaming mode
                    break;
                }
            }
            Ok(false) => {
                // Do nothing, continue generating keys
            }
            Err(e) => {
                pb.finish_and_clear();
                return Err(e);
            }
        }
    }

    pb.finish_and_clear();

    // Final update to metrics
    let elapsed = start_time.elapsed();
    metrics.update(count, matches_found, elapsed);

    Ok(metrics)
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