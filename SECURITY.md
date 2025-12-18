# Security Policy

## Reporting Security Vulnerabilities

We take the security of ShadowProbe seriously. If you discover a security vulnerability, please report it responsibly.

### How to Report

**Please DO NOT create a public GitHub issue for security vulnerabilities.**

Instead, please email security details to: [your-email@example.com]

Include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

We will acknowledge your email within 48 hours and provide a more detailed response within 7 days.

## Responsible Use

ShadowProbe is a penetration testing tool designed for **authorized security testing only**.

### ✅ Acceptable Use

- Testing systems you own
- Testing with explicit written permission
- Bug bounty programs (following their rules)
- Educational purposes in controlled environments
- Security research on test applications (DVWA, WebGoat, etc.)

### ❌ Prohibited Use

- Scanning systems without authorization
- Attacking production systems without permission
- Denial of Service (DoS) attacks
- Any illegal activities
- Violating computer fraud laws
- Exceeding authorized access

## Legal Disclaimer

Users of ShadowProbe must comply with all applicable laws and regulations. Unauthorized access to computer systems is illegal in most jurisdictions.

By using this tool, you agree to:
1. Only scan targets you have permission to test
2. Follow responsible disclosure practices
3. Not use the tool for malicious purposes
4. Comply with all applicable laws

## Security Best Practices

When using ShadowProbe:

1. **Rate Limiting**: Use the `--profile stealth` option for production systems
2. **Authorization**: Always obtain written permission before scanning
3. **Scope**: Stay within the defined scope of your engagement
4. **Privacy**: Use `--no-ai` if you're concerned about data processing
5. **Data Handling**: Securely store and delete scan results after use

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

## Known Limitations

- This is an early version (0.1.0) and may have bugs
- AI analysis is rule-based (ML models coming in future versions)
- Some scanners may produce false positives
- Not all vulnerability types are covered yet

## Security Features

ShadowProbe includes several security features:

- **Local-only AI**: All analysis runs locally, no data sent to cloud services
- **No telemetry**: We don't collect usage data
- **Docker isolation**: Running in Docker provides additional isolation
- **Open source**: All code is public and auditable

## Acknowledgments

We appreciate security researchers who responsibly disclose vulnerabilities.

Hall of Fame:
- (Your name could be here!)

## Contact

For security concerns: security@shadowprobe.dev (replace with real email)
For general questions: See CONTRIBUTING.md

---

**Remember: With great power comes great responsibility. Use ShadowProbe ethically and legally.**
