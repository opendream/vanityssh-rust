// src/main.rs
// Updated: 2025-04-22 14:36:00 by kengggg

use std::env;
use std::process;
use ed25519_vanity_rust::{stream_openssh_keys_and_match_mt, error::Result};
use regex::Regex;

fn main() -> Result<()> {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage(&args[0]);
        process::exit(1);
    }

    // Process arguments flexibly
    let mut pattern = None;
    let mut streaming = false;
    let mut case_sensitive = false;
    let mut comment = None;
    let mut threads = None;
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
            "--threads" => {
                if i + 1 < args.len() {
                    match args[i + 1].parse::<usize>() {
                        Ok(n) if n > 0 => {
                            threads = Some(n);
                            i += 2;
                        },
                        _ => {
                            eprintln!("Error: --threads requires a positive integer");
                            print_usage(&args[0]);
                            process::exit(1);
                        }
                    }
                } else {
                    eprintln!("Error: --threads requires a value");
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

    // Validate the regex pattern before starting threads
    // This will catch and display invalid regex errors immediately
    match Regex::new(pattern) {
        Ok(_) => {}, // Pattern is valid, continue
        Err(e) => {
            eprintln!("Error: Invalid regex pattern: {}", e);
            process::exit(1);
        }
    }

    // Get CPU count for display
    let cpu_count = num_cpus::get();
    let thread_count = threads.unwrap_or(cpu_count);

    // Display thread info
    println!("Using {} thread{} (system has {} CPU{})",
        thread_count,
        if thread_count == 1 { "" } else { "s" },
        cpu_count,
        if cpu_count == 1 { "" } else { "s" }
    );

    // Generate keys and match against pattern with multi-threading
    match stream_openssh_keys_and_match_mt(
        pattern,
        streaming,
        comment,
        case_sensitive,
        threads
    ) {
        Ok(_) => Ok(()),
        Err(e) => {
            // Explicitly print the error to stderr before returning it
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}

fn print_usage(program_name: &str) {
    eprintln!("Usage: {} <pattern> [OPTIONS]", program_name);
    eprintln!("  pattern         : Regex pattern to match against the generated keys");
    eprintln!("  --streaming     : Continue generating keys after a match is found");
    eprintln!("  --comment       : Add a comment to the SSH public key");
    eprintln!("  --case-sensitive: Make pattern matching case-sensitive (default is case-insensitive)");
    eprintln!("  --threads <N>   : Number of threads to use (default: number of CPU cores)");
    eprintln!("  --help          : Display this help message");
}