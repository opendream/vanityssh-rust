# 🤖 Implement Automated Release Workflow (Tag-Triggered)

## Overview
Create a comprehensive automated release workflow that triggers on git tags, builds all platforms, runs quality checks, and publishes professional releases with proper versioning.

## Tasks

### 1. Release Trigger Setup
- [ ] Configure tag-based release triggers (`v*.*.*`)
- [ ] Validate semantic versioning format
- [ ] Support pre-release tags (`-alpha`, `-beta`, `-rc`)
- [ ] Implement manual release trigger option

### 2. Quality Gates
- [ ] Require all CI checks to pass before release
- [ ] Run security audit before release
- [ ] Validate performance benchmarks
- [ ] Check cross-platform build success

### 3. Version Management
- [ ] Extract version from git tag
- [ ] Update Cargo.toml version automatically
- [ ] Validate version consistency
- [ ] Generate version metadata

### 4. Release Process Orchestration
- [ ] Build all platform targets
- [ ] Generate release artifacts
- [ ] Create GitHub release
- [ ] Upload all assets
- [ ] Generate release notes

## Acceptance Criteria
- [ ] `git tag v1.0.0 && git push --tags` triggers full release
- [ ] All platform binaries included in release
- [ ] Release fails if any quality gate fails
- [ ] Pre-release tags create draft releases
- [ ] Release notes generated automatically

## Implementation Details

### Release Workflow Trigger
```yaml
name: Release

on:
  push:
    tags:
      - 'v*.*.*'
  workflow_dispatch:
    inputs:
      tag:
        description: 'Release tag (e.g., v1.0.0)'
        required: true
        type: string

env:
  CARGO_TERM_COLOR: always
```

### Version Extraction
```yaml
- name: Extract version
  id: version
  run: |
    if [ "${{ github.event_name }}" = "workflow_dispatch" ]; then
      VERSION="${{ github.event.inputs.tag }}"
    else
      VERSION=${GITHUB_REF#refs/tags/}
    fi
    
    # Validate semver format
    if [[ ! $VERSION =~ ^v[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9-]+)?$ ]]; then
      echo "Invalid version format: $VERSION"
      exit 1
    fi
    
    echo "version=$VERSION" >> $GITHUB_OUTPUT
    echo "version_number=${VERSION#v}" >> $GITHUB_OUTPUT
```

### Quality Gates
```yaml
quality-checks:
  runs-on: ubuntu-latest
  steps:
    - name: Checkout
      uses: actions/checkout@v4
    
    - name: Wait for CI
      uses: fountainhead/action-wait-for-check@v1.2.0
      with:
        token: ${{ secrets.GITHUB_TOKEN }}
        checkName: "CI"
        timeoutSeconds: 1800
    
    - name: Security audit
      run: |
        cargo install --locked cargo-audit
        cargo audit
    
    - name: Performance check
      run: |
        cargo bench --bench key_generation -- --test
```

### Multi-Platform Build Matrix
```yaml
build:
  needs: quality-checks
  strategy:
    matrix:
      include:
        - target: x86_64-unknown-linux-gnu
          os: ubuntu-latest
          name: linux-x86_64
        - target: x86_64-unknown-linux-musl
          os: ubuntu-latest
          name: linux-x86_64-musl
          use-cross: true
        - target: aarch64-unknown-linux-gnu
          os: ubuntu-latest
          name: linux-arm64
          use-cross: true
        - target: x86_64-apple-darwin
          os: macos-latest
          name: macos-x86_64
        - target: aarch64-apple-darwin
          os: macos-latest
          name: macos-arm64
        - target: x86_64-pc-windows-msvc
          os: windows-latest
          name: windows-x86_64
```

### Build and Package
```yaml
- name: Build release binary
  run: |
    if [ "${{ matrix.use-cross }}" = "true" ]; then
      cargo install cross --git https://github.com/cross-rs/cross
      cross build --release --target ${{ matrix.target }}
    else
      cargo build --release --target ${{ matrix.target }}
    fi

- name: Prepare binary
  run: |
    cd target/${{ matrix.target }}/release
    if [ "${{ runner.os }}" = "Windows" ]; then
      BINARY="vanityssh-rust.exe"
    else
      BINARY="vanityssh-rust"
    fi
    
    # Strip binary if possible
    strip "$BINARY" 2>/dev/null || true
    
    # Rename for release
    mv "$BINARY" "vanityssh-rust-${{ matrix.name }}"
    
    # Generate checksum
    if [ "${{ runner.os }}" = "Windows" ]; then
      certutil -hashfile "vanityssh-rust-${{ matrix.name }}" SHA256 > "vanityssh-rust-${{ matrix.name }}.sha256"
    else
      shasum -a 256 "vanityssh-rust-${{ matrix.name }}" > "vanityssh-rust-${{ matrix.name }}.sha256"
    fi
```

### Release Creation
```yaml
release:
  needs: build
  runs-on: ubuntu-latest
  steps:
    - name: Download all artifacts
      uses: actions/download-artifact@v4
    
    - name: Generate changelog
      id: changelog
      uses: mikepenz/release-changelog-builder-action@v4
      with:
        configuration: ".github/changelog-config.json"
        toTag: ${{ steps.version.outputs.version }}
      env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
    
    - name: Create release
      uses: softprops/action-gh-release@v2
      with:
        tag_name: ${{ steps.version.outputs.version }}
        name: VanitySSH ${{ steps.version.outputs.version }}
        body: ${{ steps.changelog.outputs.changelog }}
        draft: ${{ contains(steps.version.outputs.version, '-') }}
        prerelease: ${{ contains(steps.version.outputs.version, '-') }}
        files: |
          artifacts/**/*
        generate_release_notes: true
      env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

## Changelog Configuration
```json
{
  "template": "#{{CHANGELOG}}\n\n**Full Changelog**: #{{UNCATEGORIZED}}",
  "pr_template": "- #{{TITLE}} by @#{{AUTHOR}} in #{{URL}}",
  "categories": [
    {
      "title": "## 🚀 Features",
      "labels": ["enhancement", "feature"]
    },
    {
      "title": "## 🐛 Bug Fixes", 
      "labels": ["bug", "fix"]
    },
    {
      "title": "## 📚 Documentation",
      "labels": ["documentation"]
    },
    {
      "title": "## 🔧 Maintenance",
      "labels": ["maintenance", "dependencies"]
    }
  ]
}
```

## Release Process Flow
1. **Trigger:** Git tag push or manual dispatch
2. **Validation:** Version format and quality gates
3. **Build:** All platform targets in parallel
4. **Package:** Binaries with checksums
5. **Release:** GitHub release with all assets
6. **Notify:** Success/failure notifications

## Pre-release Support
- **Alpha:** `v1.0.0-alpha.1` → Draft release
- **Beta:** `v1.0.0-beta.1` → Pre-release
- **RC:** `v1.0.0-rc.1` → Pre-release
- **Stable:** `v1.0.0` → Full release

## Error Handling
- Rollback on build failures
- Cleanup incomplete releases
- Retry mechanisms for transient failures
- Clear error reporting

## Timeline
**Estimate:** 3-4 days  
**Priority:** High  
**Phase:** 3

## Labels
`enhancement`, `ci/cd`, `phase-3`, `priority-high`, `release-automation`

## Dependencies
- Phase 2 completion (multi-platform builds)
- All quality gates from Phase 1

---
*Part of Phase 3: Release Automation - Core infrastructure for professional releases*