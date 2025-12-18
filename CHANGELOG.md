# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Command Injection scanner with timing analysis
- XXE (XML External Entity) vulnerability scanner
- CORS misconfiguration detector
- Open Redirect scanner with multiple bypass techniques
- Advanced WAF evasion techniques (encoding, obfuscation, polyglots)
- Scan profile system (fast, balanced, deep, stealth modes)
- Configuration management with YAML support
- Comprehensive test suite for all modules
- Benchmark suite for performance testing
- Example configurations for various use cases
- CLI enhancements: profile selection, WAF evasion mode, high-severity filter
- Security policy and responsible disclosure guidelines
- Installation and quick-scan utility scripts
- Enhanced documentation with all scanner details

### Changed
- Improved README with 9+ scanner features and new examples
- Enhanced CLI with more options and flexibility
- Better payload coverage for open redirect detection

## [0.1.0] - 2025-01-XX

### Added
- Initial release of ShadowProbe
- Modular workspace architecture with 5 crates:
  - `shadowprobe-core`: Core types and error handling
  - `shadowprobe-scanner`: HTTP client, crawler, and scanners
  - `shadowprobe-ai`: AI-powered vulnerability analysis
  - `shadowprobe-report`: Multi-format reporting
  - `shadowprobe-cli`: Command-line interface
- Async HTTP client built on Tokio and Reqwest
- Parallel web crawler with configurable depth and concurrency
- Vulnerability scanners:
  - SQL Injection (error-based, boolean-based, union-based)
  - Cross-Site Scripting (reflected, stored)
  - Server-Side Request Forgery (including cloud metadata)
  - Local File Inclusion (path traversal)
  - Server-Side Template Injection (Jinja2, Twig, etc.)
- AI analysis engine with confidence scoring
- Automated PoC generation for vulnerabilities
- Reporting formats:
  - JSON (machine-readable)
  - HTML (visual reports with styling)
  - Console (colored terminal output)
- Scan profiles:
  - Fast (quick reconnaissance)
  - Balanced (default, moderate coverage)
  - Deep (comprehensive testing)
  - Stealth (evade detection)
- Docker development environment
- GitHub Actions CI/CD pipeline
- Comprehensive documentation
- MIT License

### Security
- Privacy-first design: all AI analysis runs locally
- No telemetry or data collection
- Intentionally vulnerable app testing only

### Performance
- Async/await with Tokio for concurrent requests
- Connection pooling and reuse
- Efficient HTML parsing with Scraper
- Parallel endpoint scanning

### Developer Experience
- Full Docker support (no local Rust installation needed)
- Hot-reload development mode
- Makefile for common tasks
- Pre-configured CI/CD
- Comprehensive examples

## [0.0.1] - 2025-01-XX

### Added
- Project initialization
- Basic project structure

---

## Release Notes

### v0.1.0 - First Public Release

This is the first public release of ShadowProbe! 🎉

**Highlights:**
- ⚡ Ultra-fast scanning with Rust and async I/O
- 🤖 Local AI analysis (privacy-first, no cloud)
- 🔍 5 vulnerability types supported
- 🐳 Full Docker support
- 📊 Beautiful HTML reports

**For Bug Bounty Hunters:**
- Fast reconnaissance with parallel crawling
- Intelligent vulnerability prioritization
- Auto-generated PoCs
- Configurable scan profiles

**For Security Teams:**
- Privacy-first design
- Comprehensive reporting
- Easy integration with CI/CD
- Extensible architecture

**Getting Started:**
```bash
git clone https://github.com/yourusername/shadowprobe
cd shadowprobe
make dev
```

See [README.md](README.md) for full documentation.

### Known Limitations

- AI model is rule-based (ML integration coming in v0.2.0)
- Limited authentication support
- No Nuclei template support yet (planned for v0.3.0)

### Upgrading

This is the first release, no upgrade path needed.

---

[Unreleased]: https://github.com/yourusername/shadowprobe/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/shadowprobe/releases/tag/v0.1.0
[0.0.1]: https://github.com/yourusername/shadowprobe/releases/tag/v0.0.1
