// src/main.rs
// Updated: 2025-04-22 14:07:00 by kengggg

use std::env;
use std::process;
use ed25519_vanity_rust::{stream_openssh_keys_and_match, error::Result};

fn main() -> Result<()> {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <pattern> [--streaming] [--comment <comment>] [--match-full] [--case-sensitive]", args[0]);
        eprintln!("  pattern        : Regex pattern to match against the generated keys");
        eprintln!("  --streaming    : Continue generating keys after a match is found");
        eprintln!("  --comment      : Add a comment to the SSH public key");
        eprintln!("  --match-full   : Match against the full SSH key (default is to match only the base64 part)");
        eprintln!("  --case-sensitive: Make pattern matching case-sensitive (default is case-insensitive)");
        process::exit(1);
    }

    // Get the pattern from the first argument
    let pattern = &args[1];

    // Check for options
    let streaming = args.iter().any(|arg| arg == "--streaming");
    let match_full = args.iter().any(|arg| arg == "--match-full");
    let case_sensitive = args.iter().any(|arg| arg == "--case-sensitive");

    // Get comment if specified
    let comment = args.iter().position(|arg| arg == "--comment")
        .and_then(|pos| if pos + 1 < args.len() { Some(args[pos + 1].as_str()) } else { None });

    // Generate keys and match against pattern
    let _ = stream_openssh_keys_and_match(pattern, streaming, comment, match_full, case_sensitive)?;

    Ok(())
}