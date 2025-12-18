#!/bin/bash
# Basic scan example

echo "=== ShadowProbe Basic Scan Example ==="

# Simple scan
docker compose run --rm shadowprobe scan \
  --url https://example.com \
  -o results.json

echo "Scan complete! Results saved to results.json"
