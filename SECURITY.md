# Security Policy

## Supported Versions

We actively support the following versions of VanitySSH with security updates:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Security Features

VanitySSH implements several security measures:

### 🔒 Cryptographic Security
- **Ed25519 Keys Only**: Uses only Ed25519 cryptographic keys, considered one of the most secure elliptic curve algorithms
- **Secure Random Generation**: Uses OS-provided cryptographically secure random number generation (`OsRng`)
- **No Key Storage**: Generated keys are never stored on disk during the generation process
- **Memory Safety**: Written in Rust, providing memory safety guarantees

### 🛡️ Dependency Security
- **Automated Scanning**: All dependencies are automatically scanned for known vulnerabilities using `cargo-audit`
- **Regular Updates**: Dependabot automatically creates pull requests for security updates
- **Minimal Dependencies**: We maintain a minimal dependency footprint to reduce attack surface
- **Cryptographic Dependencies Review**: All cryptography-related dependencies require manual review before merging

### 🔍 Code Security
- **Static Analysis**: CodeQL analysis runs on all pull requests and scheduled scans
- **SARIF Reporting**: Security findings are reported in SARIF format for GitHub Security tab integration
- **Daily Security Scans**: Automated security scans run daily to catch new vulnerabilities quickly

## Reporting a Vulnerability

We take security seriously. If you discover a security vulnerability in VanitySSH, please report it responsibly:

### 🚨 For Critical Security Issues
**DO NOT** create a public GitHub issue for security vulnerabilities.

Instead, please:

1. **Email us privately**: Send details to [keng@opendream.co.th](mailto:keng@opendream.co.th)
2. **Include**:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact assessment
   - Suggested fix (if you have one)

### 📋 What to Expect
- **Acknowledgment**: We'll acknowledge receipt within 24 hours
- **Initial Assessment**: We'll provide an initial assessment within 72 hours
- **Regular Updates**: We'll keep you informed of progress at least weekly
- **Resolution Timeline**: We aim to fix critical issues within 7 days, high severity within 30 days

### 🏆 Recognition
We believe in recognizing security researchers who help make VanitySSH safer:
- **Hall of Fame**: Security researchers who report valid vulnerabilities will be listed in our security hall of fame (with permission)
- **Attribution**: We'll credit you in release notes and security advisories (unless you prefer anonymity)

## Security Best Practices for Users

### 🔐 Key Generation Security
- **Isolated Environment**: Generate keys on a secure, isolated machine when possible
- **Verify Randomness**: Ensure your system has sufficient entropy before key generation
- **Secure Storage**: Store generated private keys securely using appropriate key management practices
- **Regular Rotation**: Consider rotating SSH keys periodically

### 🌐 Network Security
- **Secure Channels**: Only transfer private keys over secure, encrypted channels
- **Key Distribution**: Use secure methods for distributing public keys
- **Access Control**: Implement proper access controls for systems using generated keys

### 🔄 Update Security
- **Stay Current**: Always use the latest version of VanitySSH
- **Monitor Advisories**: Subscribe to our security advisories for updates
- **Dependency Updates**: Keep your Rust toolchain and system dependencies updated

## Security Architecture

### Threat Model
VanitySSH's threat model considers:

1. **Cryptographic Threats**:
   - Weak random number generation
   - Implementation flaws in cryptographic algorithms
   - Side-channel attacks during key generation

2. **Supply Chain Threats**:
   - Compromised dependencies
   - Malicious code injection
   - Build environment tampering

3. **Runtime Threats**:
   - Memory corruption vulnerabilities
   - Information disclosure
   - Denial of service attacks

### Mitigations
- **Secure Coding**: Rust's memory safety prevents many common vulnerabilities
- **Dependency Scanning**: Automated vulnerability scanning of all dependencies
- **Minimal Privileges**: Application runs with minimal required permissions
- **Input Validation**: All user inputs are validated and sanitized
- **Secure Defaults**: All security-relevant settings use secure defaults

## Security Testing

We employ multiple layers of security testing:

### 🧪 Automated Testing
- **Unit Tests**: Core cryptographic functions have comprehensive test coverage
- **Integration Tests**: End-to-end security scenarios are tested
- **Fuzz Testing**: Key generation and parsing functions are fuzz tested
- **Static Analysis**: Multiple static analysis tools scan for vulnerabilities

### 🔍 Manual Testing
- **Code Reviews**: All security-relevant code receives thorough manual review
- **Cryptographic Review**: Cryptographic implementations are reviewed by security experts
- **Threat Modeling**: Regular threat modeling exercises identify new attack vectors

## Compliance and Standards

VanitySSH follows industry security standards:

- **NIST Guidelines**: Key generation follows NIST SP 800-57 recommendations
- **RFC Standards**: SSH key formats comply with relevant RFC specifications
- **Secure Development**: Development follows secure coding best practices

## Contact Information

For security-related questions or concerns:

- **Security Email**: [keng@opendream.co.th](mailto:keng@opendream.co.th)
- **General Issues**: [GitHub Issues](https://github.com/opendream/vanityssh-rust/issues) (for non-security bugs only)
- **Discussions**: [GitHub Discussions](https://github.com/opendream/vanityssh-rust/discussions)

---

**Last Updated**: 2025-06-17  
**Next Review**: 2025-12-17

*This security policy is reviewed and updated every 6 months or when significant changes occur.*