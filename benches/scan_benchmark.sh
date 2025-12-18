#!/bin/bash
# Benchmark script for ShadowProbe performance testing

set -e

echo "================================"
echo "ShadowProbe Performance Benchmark"
echo "================================"
echo ""

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Test target (intentionally vulnerable app)
TARGET="http://testphp.vulnweb.com"

echo -e "${CYAN}Target:${NC} $TARGET"
echo -e "${CYAN}Testing different scan profiles...${NC}"
echo ""

# Function to run benchmark
run_benchmark() {
    local profile=$1
    local depth=$2
    local concurrent=$3

    echo -e "${YELLOW}Testing: $profile profile${NC}"
    echo "  Depth: $depth, Concurrent: $concurrent"

    start_time=$(date +%s)

    docker compose run --rm shadowprobe scan \
        --url "$TARGET" \
        --depth "$depth" \
        --concurrent "$concurrent" \
        --timeout 10 \
        -o "bench_${profile}.json" \
        2>&1 | grep -E "(Discovered|Scanned|found)" || true

    end_time=$(date +%s)
    duration=$((end_time - start_time))

    echo -e "  ${GREEN}Duration: ${duration}s${NC}"
    echo ""

    # Extract results
    if [ -f "bench_${profile}.json" ]; then
        vulns=$(jq '.vulnerabilities | length' "bench_${profile}.json" 2>/dev/null || echo "0")
        endpoints=$(jq '.endpoints_discovered | length' "bench_${profile}.json" 2>/dev/null || echo "0")

        echo "  Results:"
        echo "    - Vulnerabilities: $vulns"
        echo "    - Endpoints: $endpoints"
        echo "    - Speed: $(echo "scale=2; $endpoints / $duration" | bc 2>/dev/null || echo "N/A") endpoints/sec"
        echo ""

        rm -f "bench_${profile}.json"
    fi
}

# Run benchmarks
echo "=== Fast Profile ===""
run_benchmark "fast" 2 100

echo "=== Balanced Profile ==="
run_benchmark "balanced" 3 50

echo "=== Deep Profile ==="
run_benchmark "deep" 5 30

echo "================================"
echo -e "${GREEN}Benchmark Complete!${NC}"
echo "================================"
