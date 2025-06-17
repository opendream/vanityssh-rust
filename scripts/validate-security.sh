#!/bin/bash
# Security Configuration Validation Script
# Validates all security setup components for VanitySSH

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔒 VanitySSH Security Configuration Validation${NC}"
echo "=================================================="
echo

# Check if we're in the project root
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}❌ Error: Run this script from the project root directory${NC}"
    exit 1
fi

success_count=0
total_tests=0

# Test function
test_check() {
    local description="$1"
    local test_command="$2"
    total_tests=$((total_tests + 1))
    
    echo -n "Testing: $description... "
    
    if eval "$test_command" >/dev/null 2>&1; then
        echo -e "${GREEN}✅ PASS${NC}"
        success_count=$((success_count + 1))
        return 0
    else
        echo -e "${RED}❌ FAIL${NC}"
        return 1
    fi
}

# Test function with output
test_check_verbose() {
    local description="$1"
    local test_command="$2"
    total_tests=$((total_tests + 1))
    
    echo "Testing: $description"
    
    if eval "$test_command"; then
        echo -e "${GREEN}✅ PASS${NC}"
        success_count=$((success_count + 1))
        echo
        return 0
    else
        echo -e "${RED}❌ FAIL${NC}"
        echo
        return 1
    fi
}

echo -e "${YELLOW}📋 Security Files Validation${NC}"
echo "--------------------------------"

# Check security workflow files exist
test_check "Security workflow exists" "[ -f .github/workflows/security.yml ]"
test_check "CI workflow exists" "[ -f .github/workflows/ci.yml ]"
test_check "Dependabot config exists" "[ -f .github/dependabot.yml ]"
test_check "Auto-merge workflow exists" "[ -f .github/workflows/dependabot-auto-merge.yml ]"
test_check "Security policy exists" "[ -f SECURITY.md ]"
test_check "Security setup docs exist" "[ -f docs/SECURITY_SETUP.md ]"

echo
echo -e "${YELLOW}🔧 Workflow Configuration Validation${NC}"
echo "-------------------------------------"

# Validate workflow syntax (basic check)
test_check "Security workflow YAML syntax" "grep -q 'name: Security Scanning' .github/workflows/security.yml"
test_check "CodeQL job configured" "grep -q 'github/codeql-action' .github/workflows/security.yml"
test_check "SARIF upload configured" "grep -q 'upload-sarif' .github/workflows/security.yml"
test_check "Daily schedule configured" "grep -q 'cron.*0 2 \* \* \*' .github/workflows/security.yml"

echo
echo -e "${YELLOW}📦 Dependabot Configuration Validation${NC}"
echo "---------------------------------------"

# Validate Dependabot configuration
test_check "Dependabot version 2" "grep -q 'version: 2' .github/dependabot.yml"
test_check "Cargo ecosystem configured" "grep -q 'package-ecosystem: \"cargo\"' .github/dependabot.yml"
test_check "Thailand timezone configured" "grep -q 'timezone: \"Asia/Bangkok\"' .github/dependabot.yml"
test_check "Weekly schedule configured" "grep -q 'interval: \"weekly\"' .github/dependabot.yml"
test_check "Monday schedule configured" "grep -q 'day: \"monday\"' .github/dependabot.yml"
test_check "Crypto group configured" "grep -q 'crypto:' .github/dependabot.yml"

echo
echo -e "${YELLOW}🛡️ Auto-merge Workflow Validation${NC}"
echo "-----------------------------------"

# Validate auto-merge workflow
test_check "Auto-merge workflow name" "grep -q 'name: Dependabot Auto-Merge' .github/workflows/dependabot-auto-merge.yml"
test_check "Dependabot actor check" "grep -q 'github.actor == .dependabot\\[bot\\].' .github/workflows/dependabot-auto-merge.yml"
test_check "Crypto dependency detection" "grep -q 'isCryptoDep.*ed25519' .github/workflows/dependabot-auto-merge.yml"
test_check "Manual review logic" "grep -q 'Manual review required' .github/workflows/dependabot-auto-merge.yml"

echo
echo -e "${YELLOW}🚀 Local Security Testing${NC}"
echo "-------------------------"

# Test cargo audit
if command -v cargo-audit >/dev/null 2>&1; then
    test_check_verbose "Cargo audit scan" "cargo audit"
else
    echo "Installing cargo-audit..."
    cargo install --locked cargo-audit
    test_check_verbose "Cargo audit scan (after install)" "cargo audit"
fi

# Test basic compilation
test_check_verbose "Security-conscious compilation" "cargo clippy --all-targets -- -D warnings"

# Test that required Rust features work
test_check "Ed25519 key generation test" "cargo test test_generate_key_pair"

echo
echo -e "${YELLOW}📖 Documentation Validation${NC}"
echo "-----------------------------"

# Check README security section
test_check "README has security badge" "grep -q 'Security.*badge' README.md"
test_check "README has security section" "grep -q '## Security' README.md"
test_check "README links to SECURITY.md" "grep -q 'SECURITY.md' README.md"

# Check SECURITY.md completeness
test_check "Security policy has contact info" "grep -q '@opendream.co.th' SECURITY.md"
test_check "Security policy has disclosure process" "grep -q 'Reporting a Vulnerability' SECURITY.md"
test_check "Security policy has threat model" "grep -q 'Threat Model' SECURITY.md"

echo
echo -e "${YELLOW}🔗 GitHub Integration Validation${NC}"
echo "-----------------------------------"

# Check if we can validate GitHub integration (requires gh CLI)
if command -v gh >/dev/null 2>&1; then
    test_check "GitHub CLI authenticated" "gh auth status"
    
    if gh auth status >/dev/null 2>&1; then
        test_check "Security workflow exists on GitHub" "gh workflow list | grep -q 'Security Scanning'"
        test_check "CI workflow exists on GitHub" "gh workflow list | grep -q 'CI'"
        
        # Check if workflows have run
        echo "Recent workflow runs:"
        gh run list --limit 5 --json displayTitle,status,conclusion,createdAt --template '{{range .}}{{.displayTitle}}: {{.status}} ({{.conclusion}}) - {{.createdAt}}{{"\n"}}{{end}}' 2>/dev/null || echo "Could not fetch workflow runs"
    else
        echo -e "${YELLOW}⚠️  GitHub CLI not authenticated - skipping GitHub integration tests${NC}"
    fi
else
    echo -e "${YELLOW}⚠️  GitHub CLI not installed - skipping GitHub integration tests${NC}"
fi

echo
echo -e "${YELLOW}📊 Security Metrics Validation${NC}"
echo "-------------------------------"

# Check key security metrics
echo "Current security status:"
echo "- Dependencies: $(cargo audit 2>/dev/null | grep -c 'Crate:' || echo '0') potential issues"
echo "- Last security scan: $(git log -1 --grep='security' --pretty=format:'%cr' 2>/dev/null || echo 'Not found')"
echo "- Security files: $(find . -name '*security*' -o -name 'SECURITY*' | wc -l) files"
echo "- Workflow files: $(find .github/workflows -name '*.yml' | wc -l) workflows"

echo
echo "=================================================="
echo -e "${BLUE}📈 Validation Summary${NC}"
echo "=================================================="

if [ $success_count -eq $total_tests ]; then
    echo -e "${GREEN}🎉 All security validations passed! ($success_count/$total_tests)${NC}"
    echo -e "${GREEN}✅ VanitySSH security configuration is properly set up${NC}"
    exit 0
else
    failed_count=$((total_tests - success_count))
    echo -e "${RED}⚠️  Some validations failed ($failed_count/$total_tests failed)${NC}"
    echo -e "${YELLOW}🔧 Please review and fix the failing tests above${NC}"
    exit 1
fi