#!/bin/bash
# Development environment setup script

echo "🔧 Setting up ShadowProbe development environment..."

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed. Please install Docker first."
    exit 1
fi

if ! command -v docker-compose &> /dev/null && ! docker compose version &> /dev/null; then
    echo "❌ Docker Compose is not installed. Please install Docker Compose first."
    exit 1
fi

echo "✅ Docker is installed"

# Create output directory
mkdir -p output
echo "✅ Created output directory"

# Build development container
echo "🐳 Building development container..."
docker compose build dev

echo "✅ Development environment ready!"
echo ""
echo "Quick start commands:"
echo "  make dev      - Start development environment"
echo "  make test     - Run tests"
echo "  make build    - Build release binary"
echo "  make shell    - Open development shell"
echo ""
