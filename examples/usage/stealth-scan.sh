#!/bin/bash
# Stealth scan to evade detection

echo "=== ShadowProbe Stealth Scan ==="

# Slow and careful scan
docker compose run --rm shadowprobe scan \
  --url https://target.com \
  --depth 3 \
  --concurrent 5 \
  --timeout 20 \
  --user-agent "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36" \
  -o stealth-results.json

echo "Stealth scan complete!"
