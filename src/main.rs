// src/main.rs
// Updated: 2025-04-22 14:20:00 by kengggg
// Improved argument parsing to support flexible ordering

use std::env;
use std::process;
use ed25519_vanity_rust::{stream_openssh_keys_and_match, error::Result};

fn main() -> Result<()> {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage(&args[0]);
        process::exit(1);
    }

    // Process arguments more flexibly
    let mut pattern = None;
    let mut streaming = false;
    let mut case_sensitive = false;
    let mut comment = None;
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "--streaming" => {
                streaming = true;
                i += 1;
            },
            "--case-sensitive" => {
                case_sensitive = true;
                i += 1;
            },
            "--comment" => {
                if i + 1 < args.len() {
                    comment = Some(args[i + 1].as_str());
                    i += 2;
                } else {
                    eprintln!("Error: --comment requires a value");
                    print_usage(&args[0]);
                    process::exit(1);
                }
            },
            "--help" => {
                print_usage(&args[0]);
                process::exit(0);
            },
            arg if arg.starts_with("--") => {
                eprintln!("Error: Unknown option: {}", arg);
                print_usage(&args[0]);
                process::exit(1);
            },
            _ => {
                // If not an option, treat as pattern
                if pattern.is_none() {
                    pattern = Some(args[i].as_str());
                } else {
                    eprintln!("Error: Multiple patterns specified");
                    print_usage(&args[0]);
                    process::exit(1);
                }
                i += 1;
            }
        }
    }

    // Ensure we have a pattern
    let pattern = match pattern {
        Some(p) => p,
        None => {
            eprintln!("Error: No pattern specified");
            print_usage(&args[0]);
            process::exit(1);
        }
    };

    // Generate keys and match against pattern
    let _ = stream_openssh_keys_and_match(pattern, streaming, comment, case_sensitive)?;

    Ok(())
}

fn print_usage(program_name: &str) {
    eprintln!("Usage: {} <pattern> [OPTIONS]", program_name);
    eprintln!("  pattern        : Regex pattern to match against the generated keys");
    eprintln!("  --streaming    : Continue generating keys after a match is found");
    eprintln!("  --comment      : Add a comment to the SSH public key");
    eprintln!("  --case-sensitive: Make pattern matching case-sensitive (default is case-insensitive)");
    eprintln!("  --help         : Display this help message");
}