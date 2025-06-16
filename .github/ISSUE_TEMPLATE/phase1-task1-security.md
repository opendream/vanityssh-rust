# 🔒 Add Security Scanning (cargo audit + Dependabot)

## Overview
Implement comprehensive security scanning to automatically detect vulnerabilities in dependencies and ensure VanitySSH maintains zero security issues.

## Tasks

### 1. Cargo Audit Integration
- [ ] Add `cargo audit` to CI pipeline
- [ ] Configure audit to fail CI on vulnerabilities
- [ ] Add audit job to existing workflow
- [ ] Test with known vulnerable dependency

### 2. Dependabot Configuration
- [ ] Create `.github/dependabot.yml` configuration
- [ ] Configure for Cargo ecosystem updates
- [ ] Set update frequency (weekly)
- [ ] Configure auto-merge for patch updates

### 3. Security Workflow
- [ ] Create dedicated `security.yml` workflow
- [ ] Add SARIF upload for security results
- [ ] Configure security notifications
- [ ] Add security badge to README

## Acceptance Criteria
- [ ] CI fails when vulnerabilities are detected
- [ ] Dependabot automatically creates update PRs
- [ ] Security scan results are visible in GitHub Security tab
- [ ] Documentation updated with security practices

## Implementation Details

### Cargo Audit Job
```yaml
- name: Security audit
  run: |
    cargo install --locked cargo-audit
    cargo audit
```

### Dependabot Config
```yaml
version: 2
updates:
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "weekly"
```

## Testing
- [ ] Test with vulnerable dependency
- [ ] Verify Dependabot PR creation
- [ ] Confirm security tab integration
- [ ] Test CI failure scenarios

## Timeline
**Estimate:** 1-2 days  
**Priority:** High  
**Phase:** 1

## Labels
`enhancement`, `security`, `phase-1`, `priority-high`, `ci/cd`

## Dependencies
None - can start immediately

---
*Part of Phase 1: Enhanced CI Foundation*