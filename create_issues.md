# 📋 GitHub Issues Creation Guide

## Phase 1 Issues to Create

Since GitHub CLI (`gh`) is not available, please create these issues manually through the GitHub web interface:

### 1. Epic Issue: Phase 1 - Enhanced CI Foundation
**Title:** `🎯 Epic: Phase 1 - Enhanced CI Foundation`  
**Labels:** `epic`, `phase-1`, `ci/cd`, `priority-high`  
**Content:** Copy from `.github/ISSUE_TEMPLATE/phase1-epic.md`

### 2. Task 1: Security Scanning
**Title:** `🔒 Add Security Scanning (cargo audit + Dependabot)`  
**Labels:** `enhancement`, `security`, `phase-1`, `priority-high`, `ci/cd`  
**Content:** Copy from `.github/ISSUE_TEMPLATE/phase1-task1-security.md`

### 3. Task 2: Performance Benchmarking  
**Title:** `⚡ Implement Performance Benchmarking with Criterion.rs`  
**Labels:** `enhancement`, `performance`, `phase-1`, `priority-high`, `ci/cd`  
**Content:** Copy from `.github/ISSUE_TEMPLATE/phase1-task2-benchmarks.md`

### 4. Task 3: Enhanced Testing
**Title:** `🧪 Enhance Testing with Stress Tests and Integration Tests`  
**Labels:** `enhancement`, `testing`, `phase-1`, `priority-medium`, `ci/cd`  
**Content:** Copy from `.github/ISSUE_TEMPLATE/phase1-task3-testing.md`

### 5. Task 4: CI Optimization
**Title:** `🚀 Optimize CI Caching and Parallel Execution`  
**Labels:** `enhancement`, `ci/cd`, `phase-1`, `priority-medium`, `performance`  
**Content:** Copy from `.github/ISSUE_TEMPLATE/phase1-task4-optimization.md`

## How to Create Issues

1. Go to: https://github.com/opendream/vanityssh-rust/issues/new
2. Copy the title from above
3. Copy the content from the corresponding file in `.github/ISSUE_TEMPLATE/`
4. Add the specified labels
5. Create the issue

## Issue Linking

After creating all issues:
1. Update the Epic issue to link to the task issues (replace `#TBD` with actual issue numbers)
2. Reference the Epic issue number in each task issue

## Recommended Order

Create in this order for proper linking:
1. Epic issue first
2. Task issues 1-4
3. Update Epic with task issue numbers

---

## Alternative: GitHub CLI Installation

If you want to install GitHub CLI for future use:

```bash
# macOS with Homebrew
brew install gh

# Login
gh auth login

# Then you can create issues with:
gh issue create --title "Issue Title" --body-file issue-template.md --label "label1,label2"
```

---

*All issue templates are ready in `.github/ISSUE_TEMPLATE/` directory*