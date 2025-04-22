// src/main.rs
// Updated: 2025-04-22 12:10:09 by kengggg

use ed25519_vanity_rust::stream_keys_and_match;
use std::env;
use std::process;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <regex-pattern> [--streaming]", args[0]);
        process::exit(1);
    }

    // Extract the regex pattern
    let pattern = &args[1];

    // Check if streaming mode is enabled
    let streaming = args.get(2).map_or(false, |arg| arg == "--streaming");

    // Start the key generation and matching process
    match stream_keys_and_match(pattern, streaming) {
        Ok(metrics) => {
            println!("\nFinal performance metrics:");
            println!("{}", metrics.to_string());
            process::exit(0);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}