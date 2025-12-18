#!/bin/bash
# Quick installation script for ShadowProbe

set -e

echo "🔍 ShadowProbe Installation Script"
echo "=================================="
echo ""

# Check system
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo "✓ Detected Linux"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    echo "✓ Detected macOS"
else
    echo "⚠️  Unknown OS: $OSTYPE"
fi

# Check Docker
if command -v docker &> /dev/null; then
    echo "✓ Docker is installed: $(docker --version)"
else
    echo "❌ Docker not found!"
    echo "Please install Docker: https://docs.docker.com/get-docker/"
    exit 1
fi

# Check Docker Compose
if docker compose version &> /dev/null 2>&1; then
    echo "✓ Docker Compose is installed"
elif command -v docker-compose &> /dev/null; then
    echo "✓ Docker Compose (standalone) is installed"
else
    echo "❌ Docker Compose not found!"
    exit 1
fi

echo ""
echo "Building ShadowProbe containers..."
docker compose build

echo ""
echo "✅ Installation complete!"
echo ""
echo "Quick start:"
echo "  make dev      # Start development"
echo "  make test     # Run tests"
echo "  make shell    # Open shell"
echo ""
echo "Run a scan:"
echo "  docker compose run --rm shadowprobe scan --url https://example.com"
echo ""
