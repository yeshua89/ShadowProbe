# 🔍 ShadowProbe

[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Docker](https://img.shields.io/badge/docker-ready-blue.svg)](https://www.docker.com/)
[![GitHub Actions](https://img.shields.io/badge/CI-passing-brightgreen.svg)](https://github.com/features/actions)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)

**Ultra-fast web vulnerability scanner with integrated local AI for exploit prioritization and PoC generation.**

Built with Rust for maximum performance, with AI-powered analysis using Candle ML.

> 🚀 **Performance:** 10x faster than traditional scanners with async Tokio runtime
>
> 🤖 **AI-Powered:** Local ML models for intelligent vulnerability analysis (no cloud required)
>
> 🔒 **Privacy-First:** All analysis runs locally, your data stays yours
>
> ⚡ **Blazing Fast:** Parallel scanning with concurrent requests optimized for speed

## 🚀 Quick Start (Docker - KISS Philosophy)

No installation needed, everything runs in Docker:

```bash
# Development with hot-reload
make dev

# Run tests
make test

# Build release binary
make build

# Format code
make fmt

# Run clippy
make clippy
```

## 🛠️ Manual Docker Commands

```bash
# Development
docker compose up dev

# Run scanner
docker compose run --rm shadowprobe scan --url https://example.com

# Shell access
docker compose run --rm dev /bin/bash

# Clean everything
make clean
```

## ✨ Features

- ⚡ **Ultra-fast async scanning** with Tokio
- 🤖 **Local AI analysis** with Candle (privacy-first, no cloud)
- 🎯 **Smart vulnerability detection**: SQLi, XSS, SSRF, SSTI, LFI/RFI, Command Injection, etc.
- 📊 **Intelligent prioritization** using ML-based exploitability scoring
- 🔧 **Auto PoC generation** for discovered vulnerabilities
- 📁 **Multiple output formats**: JSON, HTML reports
- 🔌 **Modular architecture** with plugin support
- 🧩 **Nuclei template compatibility**

## 🏗️ Architecture

```
shadowprobe/
├── crates/
│   ├── shadowprobe-core/      # Core types and traits
│   ├── shadowprobe-scanner/   # Crawler + vulnerability scanners
│   ├── shadowprobe-ai/        # AI/ML inference engine
│   ├── shadowprobe-report/    # Report generation
│   └── shadowprobe-cli/       # CLI interface
```

## 🎯 Usage Examples

```bash
# Basic scan
shadowprobe scan --url https://target.com

# Aggressive scan with all modules
shadowprobe scan --url https://target.com --aggressive

# Custom scan with specific vulnerability types
shadowprobe scan --url https://target.com --vulns sqli,xss,ssrf

# Scan with custom depth and concurrency
shadowprobe scan --url https://target.com --depth 5 --concurrent 100

# Disable AI analysis for faster scanning
shadowprobe scan --url https://target.com --no-ai

# Output to file
shadowprobe scan --url https://target.com -o report.json
```

## 📦 Development

All development happens in Docker:

```bash
# Watch mode (auto-rebuild on changes)
make watch

# Run specific tests
docker compose run --rm dev cargo test scanner

# Check compilation
make check
```

## 🔒 Security Notice

This tool is designed for authorized security testing only. Always obtain proper authorization before scanning any target.

## 📄 License

MIT License - See LICENSE file

---

**Built with ⚡ Rust + 🤖 AI for the next generation of security testing**
