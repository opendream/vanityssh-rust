// src/validation.rs
use regex::Regex;
use std::process;

/// Validation utilities for command-line arguments and application inputs
///
/// This module centralizes input validation logic to keep the main application
/// code cleaner and more focused on the primary business logic.

/// Validate the regex pattern and exit on failure
///
/// This function attempts to compile the provided regex pattern and exits
/// the application with an error message if compilation fails.
///
/// # Arguments
///
/// * `pattern` - The regex pattern to validate
pub fn validate_pattern(pattern: &str) {
    match Regex::new(pattern) {
        Ok(_) => {} // Pattern is valid, continue
        Err(e) => {
            eprintln!("Error: Invalid regex pattern: {}", e);
            process::exit(1);
        }
    }
}

/// Validate the thread count or use the default CPU count
///
/// Returns the user-specified thread count if provided and valid,
/// otherwise returns the default value.
///
/// # Arguments
///
/// * `threads` - Optional user-specified thread count
/// * `default` - Default thread count to use if none specified
///
/// # Returns
///
/// The number of threads to use
pub fn validate_threads(threads: Option<usize>, default: usize) -> usize {
    threads.unwrap_or(default)
}

/// Display thread information to the user
///
/// Formats and prints a message explaining how many threads will be used
/// and how many CPUs are available in the system.
///
/// # Arguments
///
/// * `thread_count` - The number of threads that will be used
/// * `cpu_count` - The number of CPUs available in the system
pub fn display_thread_info(thread_count: usize, cpu_count: usize) {
    println!(
        "Using {} thread{} (system has {} CPU{})",
        thread_count,
        if thread_count == 1 { "" } else { "s" },
        cpu_count,
        if cpu_count == 1 { "" } else { "s" }
    );
}
