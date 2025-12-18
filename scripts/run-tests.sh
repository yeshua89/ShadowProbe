#!/bin/bash
# Run all tests with coverage

set -e

echo "🧪 Running ShadowProbe Test Suite..."
echo ""

# Run unit tests
echo "📦 Running unit tests..."
docker compose run --rm dev cargo test --all --verbose

echo ""
echo "🎨 Checking code formatting..."
docker compose run --rm dev cargo fmt --all -- --check

echo ""
echo "📎 Running Clippy lints..."
docker compose run --rm dev cargo clippy --all-targets --all-features -- -D warnings

echo ""
echo "✅ All tests passed!"
