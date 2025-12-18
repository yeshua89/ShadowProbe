#!/bin/bash
# Advanced scan with custom options

echo "=== ShadowProbe Advanced Scan ==="

# Aggressive scan with HTML report
docker compose run --rm shadowprobe scan \
  --url https://target.com \
  --depth 5 \
  --concurrent 100 \
  --timeout 15 \
  --aggressive \
  --user-agent "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36" \
  -o results.json \
  --html report.html

echo "Advanced scan complete!"
echo "JSON: results.json"
echo "HTML: report.html"
