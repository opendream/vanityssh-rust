# 📋 Create Professional Packaging with Checksums and Documentation

## Overview
Develop professional packaging system with compressed archives, security checksums, installation scripts, and comprehensive documentation for easy distribution and installation.

## Tasks

### 1. Archive Creation
- [ ] Generate platform-specific archives (.tar.gz, .zip)
- [ ] Include documentation and installation scripts
- [ ] Create universal macOS binaries
- [ ] Optimize archive sizes

### 2. Security and Verification
- [ ] Generate SHA256 checksums for all assets
- [ ] Create combined checksum file (SHA256SUMS)
- [ ] Add GPG signing support (future)
- [ ] Verification scripts for users

### 3. Installation Scripts
- [ ] Unix installation script (install.sh)
- [ ] Windows PowerShell installer (install.ps1)
- [ ] Homebrew formula preparation
- [ ] Package manager metadata

### 4. Documentation Package
- [ ] Installation guides per platform
- [ ] Usage examples and tutorials
- [ ] Changelog generation
- [ ] License and attribution files

## Acceptance Criteria
- [ ] Professional archive structure with consistent naming
- [ ] SHA256 checksums for all binaries and archives
- [ ] Installation scripts work on clean systems
- [ ] Documentation is comprehensive and user-friendly
- [ ] Universal macOS binaries support both Intel and Apple Silicon

## Implementation Details

### Archive Structure
```
vanityssh-rust-v1.0.0-{platform}/
├── bin/
│   └── vanityssh-rust[.exe]
├── docs/
│   ├── README.md
│   ├── CHANGELOG.md
│   ├── LICENSE
│   └── USAGE.md
├── install/
│   ├── install.sh (Unix)
│   └── install.ps1 (Windows)
└── SHA256SUMS
```

### Packaging Workflow
```yaml
- name: Create platform archives
  run: |
    VERSION=${{ needs.version.outputs.version_number }}
    
    for platform in linux-x86_64 linux-x86_64-musl linux-arm64 macos-x86_64 macos-arm64 windows-x86_64; do
      mkdir -p "pkg/vanityssh-rust-v${VERSION}-${platform}"
      
      # Copy binary
      if [[ $platform == windows-* ]]; then
        cp "binaries/vanityssh-rust-${platform}.exe" "pkg/vanityssh-rust-v${VERSION}-${platform}/bin/"
      else
        cp "binaries/vanityssh-rust-${platform}" "pkg/vanityssh-rust-v${VERSION}-${platform}/bin/"
      fi
      
      # Copy documentation
      cp -r docs/* "pkg/vanityssh-rust-v${VERSION}-${platform}/docs/"
      cp -r install/* "pkg/vanityssh-rust-v${VERSION}-${platform}/install/"
      
      # Create archive
      if [[ $platform == windows-* ]]; then
        cd pkg && zip -r "vanityssh-rust-v${VERSION}-${platform}.zip" "vanityssh-rust-v${VERSION}-${platform}/" && cd ..
      else
        tar -czf "pkg/vanityssh-rust-v${VERSION}-${platform}.tar.gz" -C pkg "vanityssh-rust-v${VERSION}-${platform}/"
      fi
    done
```

### Universal macOS Binary
```yaml
- name: Create macOS universal binary
  if: matrix.os == 'macos-latest'
  run: |
    mkdir -p universal/bin
    lipo -create \
      target/x86_64-apple-darwin/release/vanityssh-rust \
      target/aarch64-apple-darwin/release/vanityssh-rust \
      -output universal/bin/vanityssh-rust
    
    # Verify universal binary
    lipo -info universal/bin/vanityssh-rust
    file universal/bin/vanityssh-rust
```

### Checksum Generation
```yaml
- name: Generate checksums
  run: |
    cd pkg
    
    # Individual checksums
    for file in vanityssh-rust-v*; do
      if [ -f "$file" ]; then
        shasum -a 256 "$file" > "${file}.sha256"
      fi
    done
    
    # Combined checksum file
    shasum -a 256 vanityssh-rust-v* > SHA256SUMS
    
    # Verify checksums
    shasum -a 256 -c SHA256SUMS
```

## Installation Scripts

### Unix Installation Script (install.sh)
```bash
#!/bin/bash
set -euo pipefail

REPO="opendream/vanityssh-rust"
VERSION=${1:-latest}

# Detect platform
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case $ARCH in
    x86_64) ARCH="x86_64" ;;
    arm64|aarch64) ARCH="arm64" ;;
    *) echo "Unsupported architecture: $ARCH" && exit 1 ;;
esac

# Download URL
if [ "$VERSION" = "latest" ]; then
    URL="https://github.com/$REPO/releases/latest/download/vanityssh-rust-${OS}-${ARCH}.tar.gz"
else
    URL="https://github.com/$REPO/releases/download/v${VERSION}/vanityssh-rust-v${VERSION}-${OS}-${ARCH}.tar.gz"
fi

# Installation
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
TEMP_DIR=$(mktemp -d)

echo "Downloading VanitySSH..."
curl -sL "$URL" | tar -xz -C "$TEMP_DIR"

echo "Installing to $INSTALL_DIR..."
sudo cp "$TEMP_DIR"/*/bin/vanityssh-rust "$INSTALL_DIR/"
sudo chmod +x "$INSTALL_DIR/vanityssh-rust"

echo "VanitySSH installed successfully!"
echo "Run 'vanityssh-rust --help' to get started."

# Cleanup
rm -rf "$TEMP_DIR"
```

### Windows Installation Script (install.ps1)
```powershell
param(
    [string]$Version = "latest",
    [string]$InstallDir = "$env:USERPROFILE\bin"
)

$ErrorActionPreference = "Stop"

$repo = "opendream/vanityssh-rust"
$arch = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "x86" }

# Download URL
if ($Version -eq "latest") {
    $url = "https://github.com/$repo/releases/latest/download/vanityssh-rust-windows-$arch.zip"
} else {
    $url = "https://github.com/$repo/releases/download/v$Version/vanityssh-rust-v$Version-windows-$arch.zip"
}

# Create install directory
if (!(Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

# Download and extract
$tempFile = [System.IO.Path]::GetTempFileName() + ".zip"
Write-Host "Downloading VanitySSH..."
Invoke-WebRequest -Uri $url -OutFile $tempFile

$tempDir = [System.IO.Path]::GetTempPath() + [System.Guid]::NewGuid()
Expand-Archive -Path $tempFile -DestinationPath $tempDir

# Install
Copy-Item "$tempDir\*\bin\vanityssh-rust.exe" -Destination "$InstallDir\"

# Add to PATH if not already there
$currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($currentPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("PATH", "$currentPath;$InstallDir", "User")
    Write-Host "Added $InstallDir to PATH"
}

Write-Host "VanitySSH installed successfully!"
Write-Host "Restart your terminal and run 'vanityssh-rust --help' to get started."

# Cleanup
Remove-Item $tempFile, $tempDir -Recurse -Force
```

## Documentation Package

### README.md (Release)
```markdown
# VanitySSH v{VERSION}

Generate SSH keys with custom patterns in their public key representation.

## Quick Start

### Installation
- **Unix/Linux/macOS:** `./install/install.sh`
- **Windows:** `./install/install.ps1`

### Usage
```bash
# Find a key starting with 'abc'
vanityssh-rust '^abc'

# Case-sensitive pattern with comment
vanityssh-rust 'MyPattern' --case-sensitive --comment "mykey@host"
```

## Platform Support
- Linux x86_64 (GNU and MUSL)
- Linux ARM64
- macOS x86_64 (Intel)
- macOS ARM64 (Apple Silicon)
- Windows x86_64

## Verification
Verify download integrity:
```bash
shasum -a 256 -c SHA256SUMS
```
```

### Platform-Specific Notes
- **Linux MUSL:** Static binary, no dependencies required
- **macOS Universal:** Works on both Intel and Apple Silicon
- **Windows:** No Visual C++ Redistributables required

## Homebrew Formula Preparation
```ruby
class VanitysshRust < Formula
  desc "Generate SSH keys with custom patterns"
  homepage "https://github.com/opendream/vanityssh-rust"
  version "{VERSION}"
  
  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/opendream/vanityssh-rust/releases/download/v{VERSION}/vanityssh-rust-v{VERSION}-macos-universal.tar.gz"
      sha256 "{MACOS_SHA256}"
    else
      url "https://github.com/opendream/vanityssh-rust/releases/download/v{VERSION}/vanityssh-rust-v{VERSION}-macos-universal.tar.gz"
      sha256 "{MACOS_SHA256}"
    end
  end
  
  on_linux do
    url "https://github.com/opendream/vanityssh-rust/releases/download/v{VERSION}/vanityssh-rust-v{VERSION}-linux-x86_64-musl.tar.gz"
    sha256 "{LINUX_SHA256}"
  end
  
  def install
    bin.install "bin/vanityssh-rust"
  end
end
```

## Timeline
**Estimate:** 2-3 days  
**Priority:** High  
**Phase:** 3

## Labels
`enhancement`, `packaging`, `phase-3`, `priority-high`, `distribution`

## Dependencies
- Automated release workflow implementation
- Multi-platform builds from Phase 2

---
*Part of Phase 3: Release Automation - Professional distribution and user experience*