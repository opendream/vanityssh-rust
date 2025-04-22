# Ed25519 Vanity Key Generator

Generate Ed25519 SSH key pairs whose public keys match a user-specified regex pattern. Supports multi-threaded key generation and OpenSSH-compatible output.

## Features
- Customizable regex pattern matching for public keys
- Multi-threaded execution for faster key generation
- OpenSSH-compatible key output
- Optional comment in the public key
- Streaming mode to continue searching for more matches
- Case-sensitive or case-insensitive matching

## Installation

Clone the repository and build with Cargo:

```sh
git clone https://github.com/yourusername/ed25519-vanity-rust.git
cd ed25519-vanity-rust
cargo build --release
```

## Usage

```
Usage: ed25519-vanity-rust <pattern> [OPTIONS]
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
./target/release/ed25519-vanity-rust '^abc'
```

#### Find a key with a case-sensitive pattern and add a comment:
```sh
./target/release/ed25519-vanity-rust 'MyPattern' --case-sensitive --comment "mykey@host"
```

#### Use 8 threads and keep searching for more matches (streaming):
```sh
./target/release/ed25519-vanity-rust 'test' --threads 8 --streaming
```

## Configuration Options

- `<pattern>`: **Required.** Regex pattern to match against the generated public keys.
- `--streaming`: Continue generating keys after a match is found. By default, the program stops after the first match.
- `--comment <comment>`: Add a comment to the SSH public key output.
- `--case-sensitive`: Make pattern matching case-sensitive. Default is case-insensitive.
- `--threads <N>`: Number of threads to use for key generation. Defaults to the number of CPU cores.
- `--help`: Show usage information.

## Output

On a successful match, the program prints the private and public keys in OpenSSH format. If `--streaming` is enabled, it continues searching and prints each new match.

## License

MIT
