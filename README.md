# VanitySSH

[![Build Status](https://github.com/opendream/vanityssh-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/opendream/vanityssh-rust/actions/workflows/ci.yml)

Generate SSH key pairs whose public keys match a user-specified regex pattern. This tool creates ed25519 SSH keys with OpenSSH-compatible formatting and searches for keys that match custom patterns in their base64-encoded representation.

## Features
- Customizable regex pattern matching for public keys
- Multi-threaded execution for faster key generation (20-50x speedup on multi-core systems)
- OpenSSH-compatible key output ready for immediate use
- Optional comment in the public key
- Streaming mode to continue searching for more matches
- Case-sensitive or case-insensitive matching
- Real-time performance metrics display

## Installation

Clone the repository and build with Cargo:

```sh
git clone https://github.com/opendream/vanityssh-rust.git
cd vanityssh-rust
cargo build --release
```

## Usage

```
Usage: vanityssh-rust <pattern> [OPTIONS]
  pattern         : Regex pattern to match against the generated keys
  --streaming     : Continue generating keys after a match is found
  --comment       : Add a comment to the SSH public key
  --case-sensitive: Make pattern matching case-sensitive (default is case-insensitive)
  --threads <N>   : Number of threads to use (default: number of CPU cores)
  --help          : Display this help message
```

### Examples

#### Find a key whose public key starts with 'abc':
```sh
./target/release/vanityssh-rust '^abc'
```

#### Find a key with a case-sensitive pattern and add a comment:
```sh
./target/release/vanityssh-rust 'MyPattern' --case-sensitive --comment "mykey@host"
```

#### Use 8 threads and keep searching for more matches (streaming):
```sh
./target/release/vanityssh-rust 'test' --threads 8 --streaming
```

## Understanding the Output

When a matching key is found, VanitySSH outputs:
- The timestamp when the match was found
- The number of key attempts before finding the match
- Which thread found the match
- The public key in standard OpenSSH format
- The private key in OpenSSH format
- Current performance metrics

Example output:
```
[2023-05-15 14:32:21] Match found after 5432 attempts by thread 3!
Public Key:  ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIHcMBrUGjR1/j9AcddUky9vLQKsvdvFe+wFg/q8j3+MD vanityssh-key
Private Key:
-----BEGIN OPENSSH PRIVATE KEY-----
...private key content...
-----END OPENSSH PRIVATE KEY-----
Performance: 15.21 keys/sec, 5432 attempts, 1 matches, elapsed: 357.1 sec
```

## Pattern Matching Details

The pattern is applied to the base64-encoded portion of the OpenSSH public key. By default, the matching is case-insensitive, which can be changed with the `--case-sensitive` option.

Some examples of patterns:
- `^abc`: Keys starting with "abc"
- `xyz$`: Keys ending with "xyz"
- `[0-9]{4}`: Keys containing four consecutive digits
- `(foo|bar)`: Keys containing either "foo" or "bar"

Keep in mind that more complex or specific patterns will take longer to match.

## Performance Considerations

- Performance is measured in keys generated per second
- Multi-threading provides significant speedup on multi-core systems
- Key generation is CPU-intensive; expect high CPU usage
- The more specific your pattern, the longer it will take to find a match
- Use `--threads` to control CPU utilization if needed

## Using Generated Keys

The generated keys are standard OpenSSH Ed25519 keys that can be used immediately:

1. Save the private key to a file (e.g., `id_ed25519`)
2. Save the public key to a file (e.g., `id_ed25519.pub`)
3. Set appropriate permissions: `chmod 600 id_ed25519`
4. Use the key as you would any SSH key

## Troubleshooting

### Common Issues

- **Slow key generation**: Try increasing the thread count or simplifying your pattern
- **No matches found**: Your pattern might be too specific; try a simpler pattern
- **High CPU usage**: This is normal; reduce thread count if needed
- **Compile errors**: Ensure you have the latest Rust toolchain and required dependencies

## License

MIT
