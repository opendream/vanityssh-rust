// src/main.rs
use ed25519_vanity_rust::{error::Result, stream_openssh_keys_and_match_mt};
use num_cpus;
use std::env;

mod config;
mod validation;

use config::Config;
use validation::{display_thread_info, validate_pattern, validate_threads};

/// Entry point for the Ed25519 vanity key generation application
///
/// This application generates Ed25519 key pairs until it finds one
/// whose public key matches a specified regex pattern.
///
/// The application flow is:
/// 1. Parse command-line arguments
/// 2. Validate inputs
/// 3. Configure and display thread usage
/// 4. Generate keys and search for matches
/// 5. Display results
fn main() -> Result<()> {
    // Parse arguments
    let args: Vec<String> = env::args().collect();
    let config = Config::parse_args(&args);

    // Validate inputs
    validate_pattern(config.pattern);
    let cpu_count = num_cpus::get();
    let thread_count = validate_threads(config.threads, cpu_count);

    // Display configuration
    display_thread_info(thread_count, cpu_count);

    // Run the core functionality
    match stream_openssh_keys_and_match_mt(
        config.pattern,
        config.streaming,
        config.comment,
        config.case_sensitive,
        Some(thread_count),
    ) {
        Ok(metrics) => {
            // Format and display performance metrics
            println!("\nKey generation completed successfully!");
            println!("----------------------------------------");
            println!("{}", metrics.to_string());
            Ok(())
        }
        Err(e) => {
            eprintln!("\nError during key generation: {}", e);
            Err(e)
        }
    }
}
