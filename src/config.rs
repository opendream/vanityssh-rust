// src/config.rs
use std::process;

/// Configuration options for the VanitySSH program
pub struct Config<'a> {
    pub pattern: &'a str,
    pub streaming: bool,
    pub case_sensitive: bool,
    pub comment: Option<&'a str>,
    pub threads: Option<usize>,
}

impl<'a> Config<'a> {
    /// Parse command-line arguments into a Config struct
    pub fn parse_args(args: &'a [String]) -> Self {
        if args.len() < 2 {
            Self::display_help();
            process::exit(1);
        }

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
                }
                "--case-sensitive" => {
                    case_sensitive = true;
                    i += 1;
                }
                "--comment" => {
                    if i + 1 < args.len() {
                        comment = Some(args[i + 1].as_str());
                        i += 2;
                    } else {
                        eprintln!("Error: --comment requires a value");
                        Self::display_help();
                        process::exit(1);
                    }
                }
                "--threads" => {
                    if i + 1 < args.len() {
                        match args[i + 1].parse::<usize>() {
                            Ok(n) if n > 0 => {
                                threads = Some(n);
                                i += 2;
                            }
                            _ => {
                                eprintln!("Error: --threads requires a positive integer");
                                Self::display_help();
                                process::exit(1);
                            }
                        }
                    } else {
                        eprintln!("Error: --threads requires a value");
                        Self::display_help();
                        process::exit(1);
                    }
                }
                "--help" => {
                    Self::display_help();
                    process::exit(0);
                }
                arg if arg.starts_with("--") => {
                    eprintln!("Error: Unknown option: {}", arg);
                    Self::display_help();
                    process::exit(1);
                }
                _ => {
                    if pattern.is_none() {
                        pattern = Some(args[i].as_str());
                    } else {
                        eprintln!("Error: Multiple patterns specified");
                        Self::display_help();
                        process::exit(1);
                    }
                    i += 1;
                }
            }
        }

        let pattern = match pattern {
            Some(p) => p,
            None => {
                eprintln!("Error: No pattern specified");
                Self::display_help();
                process::exit(1);
            }
        };

        Config {
            pattern,
            streaming,
            case_sensitive,
            comment,
            threads,
        }
    }

    /// Display usage information
    pub fn display_help() {
        println!("VanitySSH - Generate SSH keys with custom patterns");
        println!();
        println!("Usage: vanityssh-rust <pattern> [OPTIONS]");
        println!("  pattern         : Regex pattern to match against the generated keys");
        println!("  --streaming     : Continue generating keys after a match is found");
        println!("  --comment       : Add a comment to the SSH public key");
        println!("  --case-sensitive: Make pattern matching case-sensitive (default is case-insensitive)");
        println!("  --threads <N>   : Number of threads to use (default: number of CPU cores)");
        println!("  --help          : Display this help message");
    }
}

/// Print usage instructions - kept for backward compatibility with tests
pub fn print_usage(program_name: &str) {
    eprintln!("Usage: {} <pattern> [OPTIONS]", program_name);
    eprintln!("  pattern         : Regex pattern to match against the generated keys");
    eprintln!("  --streaming     : Continue generating keys after a match is found");
    eprintln!("  --comment       : Add a comment to the SSH public key");
    eprintln!(
        "  --case-sensitive: Make pattern matching case-sensitive (default is case-insensitive)"
    );
    eprintln!("  --threads <N>   : Number of threads to use (default: number of CPU cores)");
    eprintln!("  --help          : Display this help message");
}
