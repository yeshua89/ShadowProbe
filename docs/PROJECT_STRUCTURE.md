# ShadowProbe Project Structure

```
ShadowProbe/
├── .github/              # GitHub Actions CI/CD
│   └── workflows/
│       └── rust.yml      # Automated testing
│
├── benches/              # Performance benchmarks
│   └── scan_benchmark.sh
│
├── crates/               # Rust workspace crates
│   ├── shadowprobe-core/       # Core types and traits
│   ├── shadowprobe-scanner/    # Scanners and crawlers
│   ├── shadowprobe-ai/         # AI analysis engine
│   ├── shadowprobe-report/     # Report generators
│   └── shadowprobe-cli/        # CLI application
│
├── docs/                 # Documentation
│   ├── README.md         # Docs index
│   ├── CHANGELOG.md      # Version history
│   ├── CONTRIBUTING.md   # Contribution guidelines
│   ├── ROADMAP.md        # Future plans
│   └── SECURITY.md       # Security policy
│
├── examples/             # Usage examples
│   ├── config/          # Example configurations
│   ├── targets/         # Test target info
│   └── usage/           # Usage scripts
│
├── scripts/              # Utility scripts
│   ├── install.sh       # Installation script
│   ├── quick-scan.sh    # Quick scan wrapper
│   ├── run-tests.sh     # Test runner
│   └── setup-dev.sh     # Dev environment setup
│
├── .dockerignore        # Docker ignore rules
├── .editorconfig        # Editor configuration
├── .gitignore           # Git ignore rules
├── Cargo.toml           # Workspace manifest
├── docker-compose.yml   # Docker services
├── Dockerfile           # Container definition
├── LICENSE              # MIT License
├── Makefile             # Build commands
└── README.md            # Main documentation
```

## Key Directories

### `/crates`
Modular Rust workspace with 5 crates following clean architecture.

### `/docs`
All project documentation centralized for easy navigation.

### `/examples`
Practical examples and configuration templates.

### `/scripts`
Automation and utility scripts for common tasks.

### `/benches`
Performance benchmarking tools.

## Clean Architecture Benefits

- ✅ Clear separation of concerns
- ✅ Easy to navigate and understand
- ✅ Modular and extensible
- ✅ Well-documented structure
- ✅ Professional organization
