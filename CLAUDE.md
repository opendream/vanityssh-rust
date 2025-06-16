# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

VanitySSH is a Rust command-line tool for generating SSH Ed25519 key pairs with custom patterns. It uses multi-threading to efficiently search for SSH keys whose public key base64 representation matches a user-specified regex pattern.

## Common Commands

### Build and Run

```bash
# Build debug version
cargo build

# Build release version (recommended for performance)
cargo build --release

# Run with debug build
cargo run -- <pattern> [OPTIONS]

# Run with release build
./target/release/vanityssh-rust <pattern> [OPTIONS]
```

### Development Commands

```bash
# Run tests
cargo test

# Check code formatting
cargo fmt --check

# Apply code formatting
cargo fmt

# Run linter
cargo clippy --all-targets -- -D warnings

# Run all CI checks locally
cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test
```

## Architecture

### Core Components

- **`src/main.rs`**: CLI entry point and orchestration
- **`src/keygen.rs`**: Ed25519 key generation (raw hex and OpenSSH formats)
- **`src/matcher.rs`**: Regex pattern matching against base64 public keys
- **`src/thread_pool.rs`**: Multi-threaded key generation with work-stealing
- **`src/ssh/`**: OpenSSH key format encoding/decoding
- **`src/config.rs`**: Command-line argument parsing
- **`src/error.rs`**: Custom error types

### Key Architecture Patterns

1. **Multi-threading**: Uses `crossbeam-channel` for communication between worker threads and main thread
2. **Work-stealing**: Each thread generates keys independently, first to find match wins
3. **Streaming mode**: Can continue searching for additional matches after first success
4. **Progress tracking**: Real-time performance metrics using `indicatif`

### Data Flow

1. CLI args parsed → `ThreadPoolConfig` created
2. Worker threads spawn, each running independent key generation loops
3. Generated keys tested against regex pattern in base64 format
4. Matches sent back to main thread via channels
5. Results displayed with performance metrics

## Key Dependencies

- `ed25519-dalek`: Cryptographic key generation
- `regex`: Pattern matching
- `crossbeam-channel`: Inter-thread communication
- `indicatif`: Progress bars and metrics
- `base64`: SSH key encoding

## Testing

The test suite covers:

- Key generation functionality
- Pattern matching logic
- CLI integration tests
- Thread pool behavior
- OpenSSH format compliance

Integration tests use `assert_cmd` and `predicates` crates for CLI testing.
