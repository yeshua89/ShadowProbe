# Contributing to ShadowProbe

We love your input! We want to make contributing to ShadowProbe as easy and transparent as possible.

## Development Process

We use Docker for all development to maintain consistency:

1. Fork the repo
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Make your changes in Docker:
   ```bash
   make dev  # Start development environment
   ```
4. Write tests for your changes
5. Ensure all tests pass: `make test`
6. Format your code: `make fmt`
7. Run linter: `make clippy`
8. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
9. Push to the branch (`git push origin feature/AmazingFeature`)
10. Open a Pull Request

## Code Style

- Follow Rust standard formatting (enforced by `cargo fmt`)
- Pass all Clippy lints
- Write documentation for public APIs
- Add tests for new functionality

## Adding New Vulnerability Scanners

To add a new scanner:

1. Create a new file in `crates/shadowprobe-scanner/src/scanners/`
2. Implement the `VulnerabilityScanner` trait
3. Add payloads to `crates/shadowprobe-scanner/src/payloads.rs`
4. Register scanner in `crates/shadowprobe-scanner/src/scanners/mod.rs`
5. Add tests

## Pull Request Process

1. Update the README.md with details of changes if applicable
2. Update the documentation
3. The PR will be merged once you have approval from maintainers

## Any Questions?

Feel free to open an issue for discussion!
