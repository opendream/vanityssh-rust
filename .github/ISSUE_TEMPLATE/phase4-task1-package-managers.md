# 📦 Set up Package Manager Distribution (Homebrew, Chocolatey, crates.io)

## Overview
Integrate VanitySSH with major package managers to provide easy installation across all platforms, with automated publishing and updates triggered by releases.

## Tasks

### 1. Homebrew Integration (macOS/Linux)
- [ ] Create Homebrew formula
- [ ] Submit to homebrew-core or create tap
- [ ] Set up automated formula updates
- [ ] Test installation on macOS and Linux

### 2. Chocolatey Integration (Windows)
- [ ] Create Chocolatey package definition
- [ ] Submit to Chocolatey Community Repository
- [ ] Set up automated package publishing
- [ ] Test installation on Windows systems

### 3. crates.io Integration (Rust Ecosystem)
- [ ] Prepare package for crates.io
- [ ] Set up automated publishing
- [ ] Configure package metadata
- [ ] Test cargo install process

### 4. Container Distribution (Docker)
- [ ] Create multi-arch Docker images
- [ ] Publish to Docker Hub
- [ ] Set up automated image builds
- [ ] Create usage documentation

## Acceptance Criteria
- [ ] `brew install vanityssh-rust` works on macOS and Linux
- [ ] `choco install vanityssh-rust` works on Windows
- [ ] `cargo install vanityssh-rust` installs latest version
- [ ] Docker images available for x86_64 and ARM64
- [ ] All package managers auto-update on new releases

## Implementation Details

### Homebrew Formula
```ruby
class VanitysshRust < Formula
  desc "Generate SSH keys with custom patterns"
  homepage "https://github.com/opendream/vanityssh-rust"
  url "https://github.com/opendream/vanityssh-rust/releases/download/v{VERSION}/vanityssh-rust-v{VERSION}-macos-universal.tar.gz"
  sha256 "{SHA256}"
  license "MIT"
  version "{VERSION}"

  depends_on macos: ">= :big_sur"

  def install
    bin.install "bin/vanityssh-rust"
    
    # Install documentation
    doc.install "docs/README.md"
    doc.install "docs/USAGE.md"
  end

  test do
    assert_match "VanitySSH", shell_output("#{bin}/vanityssh-rust --help")
    
    # Test basic functionality
    system "#{bin}/vanityssh-rust", "--help"
  end
end
```

### Chocolatey Package (chocolateyinstall.ps1)
```powershell
$ErrorActionPreference = 'Stop'

$packageName = 'vanityssh-rust'
$version = '{VERSION}'
$url64 = "https://github.com/opendream/vanityssh-rust/releases/download/v$version/vanityssh-rust-v$version-windows-x86_64.zip"
$checksum64 = '{CHECKSUM64}'

$packageArgs = @{
  packageName   = $packageName
  url64bit      = $url64
  checksum64    = $checksum64
  checksumType64= 'sha256'
  unzipLocation = Split-Path $MyInvocation.MyCommand.Definition
}

Install-ChocolateyZipPackage @packageArgs

# Add to PATH
$binPath = Join-Path (Split-Path $MyInvocation.MyCommand.Definition) "bin"
Install-ChocolateyPath $binPath
```

### Chocolatey Package Metadata (vanityssh-rust.nuspec)
```xml
<?xml version="1.0" encoding="utf-8"?>
<package xmlns="http://schemas.microsoft.com/packaging/2015/06/nuspec.xsd">
  <metadata>
    <id>vanityssh-rust</id>
    <version>{VERSION}</version>
    <title>VanitySSH</title>
    <authors>kengggg</authors>
    <projectUrl>https://github.com/opendream/vanityssh-rust</projectUrl>
    <licenseUrl>https://github.com/opendream/vanityssh-rust/blob/main/LICENSE</licenseUrl>
    <requireLicenseAcceptance>false</requireLicenseAcceptance>
    <description>Generate SSH Ed25519 key pairs whose public keys match a user-specified regex pattern. Multi-threaded for high performance.</description>
    <summary>SSH vanity key generator with regex pattern matching</summary>
    <tags>ssh ed25519 key-generation cryptography security cli</tags>
    <copyright>MIT License</copyright>
  </metadata>
  <files>
    <file src="tools\**" target="tools" />
  </files>
</package>
```

### crates.io Configuration (Cargo.toml updates)
```toml
[package]
name = "vanityssh-rust"
version = "0.1.0"
edition = "2021"
description = "Generate SSH Ed25519 key pairs whose public keys match a user-specified regex pattern"
authors = ["kengggg"]
license = "MIT"
repository = "https://github.com/opendream/vanityssh-rust"
homepage = "https://github.com/opendream/vanityssh-rust"
documentation = "https://docs.rs/vanityssh-rust"
readme = "README.md"
keywords = ["ssh", "ed25519", "key-generation", "vanity", "cryptography"]
categories = ["command-line-utilities", "cryptography"]
exclude = [
    ".github/",
    "tests/",
    "benches/",
    "*.md",
    "Dockerfile*"
]

[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
```

### Docker Multi-Arch Setup
```dockerfile
# Dockerfile
FROM scratch
COPY vanityssh-rust /usr/local/bin/vanityssh-rust
ENTRYPOINT ["/usr/local/bin/vanityssh-rust"]
```

### Docker Build Workflow
```yaml
- name: Build Docker images
  run: |
    # Build for multiple architectures
    docker buildx create --name multiarch --use
    
    docker buildx build \
      --platform linux/amd64,linux/arm64 \
      --tag opendream/vanityssh-rust:${{ steps.version.outputs.version_number }} \
      --tag opendream/vanityssh-rust:latest \
      --push \
      .
```

## Automated Publishing Workflows

### Homebrew Auto-Update
```yaml
homebrew-update:
  needs: release
  runs-on: ubuntu-latest
  steps:
    - name: Update Homebrew formula
      uses: mislav/bump-homebrew-formula-action@v3
      with:
        formula-name: vanityssh-rust
        formula-path: Formula/vanityssh-rust.rb
        homebrew-tap: opendream/homebrew-tap
        download-url: https://github.com/opendream/vanityssh-rust/releases/download/v${{ needs.release.outputs.version }}/vanityssh-rust-v${{ needs.release.outputs.version }}-macos-universal.tar.gz
        commit-message: "vanityssh-rust ${{ needs.release.outputs.version }}"
      env:
        COMMITTER_TOKEN: ${{ secrets.HOMEBREW_TOKEN }}
```

### Chocolatey Auto-Publish
```yaml
chocolatey-publish:
  needs: release
  runs-on: windows-latest
  steps:
    - name: Download package template
      run: |
        # Download and prepare Chocolatey package
        
    - name: Update package files
      run: |
        # Update version and checksums in package files
        
    - name: Publish to Chocolatey
      run: |
        choco apikey --key ${{ secrets.CHOCOLATEY_API_KEY }} --source https://push.chocolatey.org/
        choco push vanityssh-rust.${{ needs.release.outputs.version_number }}.nupkg --source https://push.chocolatey.org/
```

### crates.io Auto-Publish
```yaml
crates-publish:
  needs: release
  runs-on: ubuntu-latest
  steps:
    - name: Publish to crates.io
      run: |
        cargo login ${{ secrets.CARGO_REGISTRY_TOKEN }}
        cargo publish
```

## Testing Strategy

### Package Installation Testing
```yaml
test-packages:
  strategy:
    matrix:
      include:
        - os: macos-latest
          package-manager: homebrew
          install-cmd: "brew install vanityssh-rust"
        - os: windows-latest
          package-manager: chocolatey
          install-cmd: "choco install vanityssh-rust"
        - os: ubuntu-latest
          package-manager: cargo
          install-cmd: "cargo install vanityssh-rust"
  
  runs-on: ${{ matrix.os }}
  steps:
    - name: Test package installation
      run: |
        ${{ matrix.install-cmd }}
        vanityssh-rust --help
        vanityssh-rust '^test' --help
```

## Package Manager Comparison

| Package Manager | Pros | Cons | Maintenance |
|-----------------|------|------|-------------|
| **Homebrew** | Popular on macOS/Linux, good tooling | Manual formula updates | Medium |
| **Chocolatey** | Windows standard, automated | Windows-only | Medium |
| **crates.io** | Rust native, automatic | Rust-only audience | Low |
| **Docker** | Cross-platform, CI-friendly | Requires Docker knowledge | Low |

## Distribution Statistics Tracking
- Download counts from GitHub releases
- Package manager install statistics
- Platform usage analytics
- User feedback from different channels

## Timeline
**Estimate:** 5-6 days  
**Priority:** Medium  
**Phase:** 4

## Labels
`enhancement`, `distribution`, `phase-4`, `priority-medium`, `package-management`

## Dependencies
- Phase 3 completion (automated releases)
- Release asset generation working
- Professional packaging implemented

---
*Part of Phase 4: Distribution & Package Management - Final step for ecosystem integration*