# Security Setup Guide

This document explains the comprehensive security setup implemented in VanitySSH.

## Overview

VanitySSH implements a multi-layered security approach:

1. **Automated Security Scanning** - Daily vulnerability scans
2. **Dependency Management** - Automated security updates  
3. **Static Analysis** - CodeQL analysis for vulnerabilities
4. **Security Reporting** - SARIF integration with GitHub Security tab
5. **Security Policy** - Responsible disclosure process

## Security Workflows

### 1. Main CI Security (`ci.yml`)
- **Trigger**: Every push and PR
- **Scope**: Multi-platform security audit (Linux, macOS, Windows)
- **Tool**: `cargo-audit`
- **Action**: Fails CI if vulnerabilities found

```yaml
- name: Run security audit
  run: cargo audit
```

### 2. Advanced Security Workflow (`security.yml`)
- **Trigger**: Push, PR, and daily at 2 AM UTC (10 AM Thailand time)
- **Scope**: Comprehensive security analysis
- **Tools**: `cargo-audit` + CodeQL
- **Reporting**: SARIF format to GitHub Security tab

#### Components:

**Cargo Audit Job:**
- Scans for known vulnerabilities in dependencies
- Converts results to SARIF format
- Uploads to GitHub Security tab
- Fails if vulnerabilities found

**CodeQL Analysis:**
- Advanced static analysis for Rust code
- Detects security vulnerabilities and coding errors
- Integrates with GitHub Security tab
- Runs on push/PR (not on scheduled runs)

**Security Report:**
- Aggregates results from all security jobs  
- Creates summary in GitHub Actions
- Links to detailed Security tab results

### 3. Dependabot (`dependabot.yml`)
- **Schedule**: Weekly on Mondays at 09:00 Asia/Bangkok
- **Scope**: All Cargo dependencies
- **Grouping**: Smart dependency grouping by category
- **Auto-merge**: Safe patch updates with manual review for crypto

## Security Features

### Cryptographic Security
- **Ed25519 Only**: Most secure elliptic curve algorithm
- **OS Random**: Uses OS cryptographically secure RNG (`OsRng`)
- **No Key Storage**: Keys never stored during generation
- **Memory Safety**: Rust prevents memory corruption vulnerabilities

### Dependency Security
```yaml
# Weekly security updates
schedule:
  interval: "weekly"
  day: "monday" 
  time: "09:00"
  timezone: "Asia/Bangkok"

# Smart grouping
groups:
  crypto:        # Cryptography deps - manual review required
  dev-tools:     # Development tools  
  utilities:     # Build utilities
  threading:     # Performance deps
```

### Automated Security Response
```yaml
# Auto-merge logic for different security update types:
- Patch updates (non-crypto): Auto-merge after CI passes
- Patch updates (crypto): Manual review required  
- Minor/Major updates: Manual review required
```

## Security Monitoring

### GitHub Security Tab Integration

All security findings are automatically uploaded to GitHub's Security tab:

1. **Vulnerabilities**: From cargo-audit scans
2. **Code Scanning**: From CodeQL analysis  
3. **Dependabot**: Dependency update alerts
4. **Secret Scanning**: GitHub's secret detection

### Daily Security Reports

Automated daily scans provide:
- Vulnerability count and details
- New security advisories
- Dependency status updates
- Security trend analysis

### Security Badges

README displays real-time security status:
```markdown
[![Security](https://github.com/opendream/vanityssh-rust/actions/workflows/security.yml/badge.svg)](https://github.com/opendream/vanityssh-rust/actions/workflows/security.yml)
```

## Security Policy Enforcement

### Pull Request Requirements
All PRs must pass:
- ✅ Cargo audit (no vulnerabilities)
- ✅ CodeQL analysis (no security issues)
- ✅ All existing tests
- ✅ Clippy lints

### Crypto Dependency Updates
Special handling for cryptographic dependencies:
- Manual review required for ALL updates
- Security team approval needed
- Extended testing period
- Rollback plan documented

### Security Issue Response

**Critical Security Issues (0-day, RCE, etc.):**
- Response: Within 24 hours
- Fix: Within 7 days
- Disclosure: Coordinated disclosure

**High Security Issues:**
- Response: Within 72 hours  
- Fix: Within 30 days
- Public disclosure after fix

## Configuration Files

### 1. Security Workflow (`.github/workflows/security.yml`)
```yaml
# Runs on:
- push (main branch)
- pull_request  
- schedule (daily 2 AM UTC)

# Jobs:
- cargo-audit: Dependency vulnerability scanning
- advanced-security: CodeQL static analysis  
- security-report: Results aggregation
```

### 2. Dependabot Config (`.github/dependabot.yml`)
```yaml
# Features:
- Weekly updates on Monday 09:00 Thailand time
- 5 concurrent PR limit
- Smart dependency grouping
- Security-focused labeling
```

### 3. Auto-merge Workflow (`.github/workflows/dependabot-auto-merge.yml`)
```yaml
# Logic:
- Auto-merge: Patch updates (non-crypto) after CI passes
- Manual review: Crypto deps, minor/major updates
- Safety gates: All CI must pass before merge
```

## Validation Checklist

### ✅ Security Scanning
- [x] Daily cargo-audit scans configured
- [x] CodeQL analysis enabled
- [x] SARIF reporting to Security tab
- [x] CI fails on vulnerabilities

### ✅ Dependency Management  
- [x] Dependabot weekly updates
- [x] Thailand timezone configured
- [x] Smart dependency grouping
- [x] Auto-merge with safety gates

### ✅ Security Documentation
- [x] SECURITY.md policy created
- [x] README security section added
- [x] Security badges displayed
- [x] Responsible disclosure process

### ✅ Integration Testing
- [x] Security workflows syntax validated
- [x] Badge links functional
- [x] GitHub Security tab populated
- [x] Auto-merge logic tested

## Troubleshooting

### Common Issues

**Security workflow fails:**
```bash
# Check workflow logs
gh run list --workflow=security.yml
gh run view [RUN_ID]
```

**SARIF upload fails:**
- Ensure `security-events: write` permission
- Check SARIF file format validity
- Verify CodeQL action version

**Dependabot not creating PRs:**
- Check dependabot.yml syntax
- Verify timezone format
- Check PR limits not exceeded

### Testing Security Setup

**Test cargo-audit locally:**
```bash
cargo install --locked cargo-audit
cargo audit
```

**Test CodeQL locally:**
```bash
# Install CodeQL CLI
codeql database create --language=rust ./codeql-db
codeql database analyze ./codeql-db --format=sarif-latest --output=results.sarif
```

**Validate Dependabot config:**
```bash
# Use GitHub's dependabot CLI (if available)
dependabot validate .github/dependabot.yml
```

## Security Metrics

Track security effectiveness:

### Key Performance Indicators (KPIs)
- **MTTD** (Mean Time To Detection): < 24 hours
- **MTTR** (Mean Time To Resolution): < 7 days (critical), < 30 days (high)
- **Vulnerability Backlog**: Zero critical/high vulnerabilities
- **Dependency Freshness**: < 30 days behind latest secure versions

### Monthly Security Review
- Review all security findings
- Update threat model
- Assess new attack vectors
- Update security policies

---

**Document Version**: 1.0  
**Last Updated**: 2025-06-17  
**Next Review**: 2025-07-17