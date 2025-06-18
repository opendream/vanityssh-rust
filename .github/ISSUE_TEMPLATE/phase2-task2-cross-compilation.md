# 🔄 Implement Comprehensive Cross-Compilation Setup

## Overview
Set up robust cross-compilation infrastructure to build VanitySSH for all target platforms from any host, with static linking and platform-specific optimizations.

## Tasks

### 1. Cross-Compilation Infrastructure
- [ ] Set up `cross` tool for containerized cross-compilation
- [ ] Configure Docker-based cross-compilation environments
- [ ] Add all target platforms to compilation matrix
- [ ] Set up static linking for Linux targets

### 2. Target Platform Configuration
- [ ] **Linux x86_64 GNU** - Standard glibc linking
- [ ] **Linux x86_64 MUSL** - Static, no dependencies
- [ ] **Linux ARM64** - ARM server support
- [ ] **Windows x86_64** - MSVC and GNU toolchains
- [ ] **Windows ARM64** - ARM laptop support
- [ ] **macOS x86_64** - Intel Mac support
- [ ] **macOS ARM64** - Apple Silicon support

### 3. Static Linking Setup
- [ ] Configure MUSL static builds for Linux
- [ ] Static CRT linking for Windows
- [ ] Minimize dependencies for all platforms
- [ ] Test binary portability across systems

### 4. Build Optimization
- [ ] Platform-specific compiler optimizations
- [ ] Binary size optimization
- [ ] Performance tuning per architecture
- [ ] Strip debugging symbols for release

## Acceptance Criteria
- [ ] All target platforms build successfully
- [ ] Linux MUSL binaries run without dependencies
- [ ] Windows binaries work without Visual C++ Redistributables
- [ ] Cross-compilation completes in reasonable time (<10 minutes)
- [ ] Binaries are optimized for their target platforms

## Implementation Details

### Cross.toml Configuration
```toml
[build]
default-target = "x86_64-unknown-linux-gnu"

[target.x86_64-unknown-linux-musl]
image = "ghcr.io/cross-rs/x86_64-unknown-linux-musl:edge"

[target.aarch64-unknown-linux-gnu]
image = "ghcr.io/cross-rs/aarch64-unknown-linux-gnu:edge"

[target.x86_64-pc-windows-gnu]
image = "ghcr.io/cross-rs/x86_64-pc-windows-gnu:edge"

[target.aarch64-pc-windows-msvc]
image = "ghcr.io/cross-rs/aarch64-pc-windows-msvc:edge"
```

### Target Installation
```bash
# Install all required targets
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-musl
rustup target add aarch64-unknown-linux-gnu
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-pc-windows-msvc
rustup target add aarch64-pc-windows-msvc
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
```

### CI Matrix Configuration
```yaml
strategy:
  matrix:
    include:
      # Linux targets
      - target: x86_64-unknown-linux-gnu
        os: ubuntu-latest
        use-cross: false
      - target: x86_64-unknown-linux-musl
        os: ubuntu-latest
        use-cross: true
      - target: aarch64-unknown-linux-gnu
        os: ubuntu-latest
        use-cross: true
      
      # Windows targets
      - target: x86_64-pc-windows-msvc
        os: windows-latest
        use-cross: false
      - target: x86_64-pc-windows-gnu
        os: ubuntu-latest
        use-cross: true
      - target: aarch64-pc-windows-msvc
        os: windows-latest
        use-cross: true
      
      # macOS targets
      - target: x86_64-apple-darwin
        os: macos-latest
        use-cross: false
      - target: aarch64-apple-darwin
        os: macos-latest
        use-cross: false
```

### Build Script
```bash
#!/bin/bash
set -euo pipefail

TARGET=${1:-x86_64-unknown-linux-gnu}
USE_CROSS=${2:-false}

if [ "$USE_CROSS" = "true" ]; then
    cross build --release --target "$TARGET"
else
    cargo build --release --target "$TARGET"
fi

# Strip debug symbols
strip "target/$TARGET/release/vanityssh-rust" 2>/dev/null || true

# Verify binary
file "target/$TARGET/release/vanityssh-rust"
ls -lah "target/$TARGET/release/vanityssh-rust"
```

## Target Platform Details

### Linux MUSL (Static)
```toml
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"
rustflags = ["-C", "target-feature=+crt-static"]
```

### Windows Static
```toml
[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "target-feature=+crt-static"]
```

### Platform-Specific Optimizations
```toml
# Intel/AMD optimizations
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "target-cpu=x86-64-v2"]

# ARM64 optimizations  
[target.aarch64-unknown-linux-gnu]
rustflags = ["-C", "target-cpu=cortex-a72"]

# Apple Silicon optimizations
[target.aarch64-apple-darwin]
rustflags = ["-C", "target-cpu=apple-m1"]
```

## Testing Strategy

### 1. Build Verification
- [ ] All targets compile successfully
- [ ] No missing dependencies on target systems
- [ ] Binaries execute on clean target environments

### 2. Portability Testing
- [ ] Linux MUSL on various distributions
- [ ] Windows without VC++ Redistributables
- [ ] macOS across different versions
- [ ] ARM systems (Raspberry Pi, ARM servers)

### 3. Performance Validation
- [ ] Platform-specific optimizations effective
- [ ] No performance regression from cross-compilation
- [ ] Binary size reasonable for all targets

## Binary Size Optimization
```toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true         # Link-time optimization
codegen-units = 1  # Single codegen unit
panic = "abort"    # Smaller binaries
strip = true       # Remove debug symbols
```

## Platform Matrix Summary
| Platform | Architecture | Static | Priority |
|----------|-------------|--------|----------|
| Linux GNU | x86_64 | No | High |
| Linux MUSL | x86_64 | Yes | High |
| Linux | ARM64 | Yes | Medium |
| Windows MSVC | x86_64 | Yes | High |
| Windows GNU | x86_64 | Yes | Medium |
| Windows | ARM64 | Yes | Low |
| macOS | x86_64 | No | High |
| macOS | ARM64 | No | High |

## Timeline
**Estimate:** 4-5 days  
**Priority:** High  
**Phase:** 2

## Labels
`enhancement`, `ci/cd`, `phase-2`, `priority-high`, `cross-compilation`, `architecture`

## Dependencies
- Phase 1 completion (CI optimization)
- ARM64 macOS support implementation
- Docker/containerization support in CI

---
*Part of Phase 2: Multi-Architecture Support - Foundation for multi-platform distribution*