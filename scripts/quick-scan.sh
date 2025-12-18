#!/bin/bash
# Quick scan wrapper script

if [ -z "$1" ]; then
    echo "Usage: $0 <target-url> [profile]"
    echo "Profiles: fast, balanced (default), deep, stealth"
    exit 1
fi

TARGET="$1"
PROFILE="${2:-balanced}"

echo "🔍 ShadowProbe Quick Scan"
echo "Target: $TARGET"
echo "Profile: $PROFILE"
echo ""

case $PROFILE in
    fast)
        DEPTH=2
        CONCURRENT=100
        ;;
    balanced)
        DEPTH=3
        CONCURRENT=50
        ;;
    deep)
        DEPTH=5
        CONCURRENT=30
        ;;
    stealth)
        DEPTH=3
        CONCURRENT=5
        ;;
    *)
        echo "Unknown profile: $PROFILE"
        exit 1
        ;;
esac

OUTPUT="scan-$(date +%Y%m%d-%H%M%S).json"
HTML="report-$(date +%Y%m%d-%H%M%S).html"

docker compose run --rm shadowprobe scan \
    --url "$TARGET" \
    --depth "$DEPTH" \
    --concurrent "$CONCURRENT" \
    -o "$OUTPUT" \
    --html "$HTML"

echo ""
echo "✅ Scan complete!"
echo "Results: $OUTPUT"
echo "Report: $HTML"
