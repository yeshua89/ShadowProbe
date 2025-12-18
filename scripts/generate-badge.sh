#!/bin/bash
# Generate a badge/shield for scan results

if [ -z "$1" ]; then
    echo "Usage: $0 <scan-result.json>"
    exit 1
fi

SCAN_FILE="$1"

if [ ! -f "$SCAN_FILE" ]; then
    echo "Error: File not found: $SCAN_FILE"
    exit 1
fi

# Extract statistics
TOTAL=$(jq '.vulnerabilities | length' "$SCAN_FILE" 2>/dev/null || echo "0")
CRITICAL=$(jq '[.vulnerabilities[] | select(.severity == "Critical")] | length' "$SCAN_FILE" 2>/dev/null || echo "0")
HIGH=$(jq '[.vulnerabilities[] | select(.severity == "High")] | length' "$SCAN_FILE" 2>/dev/null || echo "0")
MEDIUM=$(jq '[.vulnerabilities[] | select(.severity == "Medium")] | length' "$SCAN_FILE" 2>/dev/null || echo "0")
LOW=$(jq '[.vulnerabilities[] | select(.severity == "Low")] | length' "$SCAN_FILE" 2>/dev/null || echo "0")

# Determine color
if [ "$CRITICAL" -gt 0 ] || [ "$HIGH" -gt 0 ]; then
    COLOR="red"
    STATUS="VULNERABLE"
elif [ "$MEDIUM" -gt 0 ]; then
    COLOR="orange"
    STATUS="WARNING"
elif [ "$LOW" -gt 0 ]; then
    COLOR="yellow"
    STATUS="MINOR"
else
    COLOR="green"
    STATUS="SECURE"
fi

echo ""
echo "🏅 Scan Badge Information"
echo "========================"
echo ""
echo "Status: $STATUS"
echo "Color: $COLOR"
echo ""
echo "Vulnerabilities:"
echo "  Critical: $CRITICAL"
echo "  High: $HIGH"
echo "  Medium: $MEDIUM"
echo "  Low: $LOW"
echo "  Total: $TOTAL"
echo ""
echo "Shield.io Badge URL:"
echo "https://img.shields.io/badge/security-$STATUS-$COLOR"
echo ""
echo "Markdown:"
echo "![Security: $STATUS](https://img.shields.io/badge/security-$STATUS-$COLOR)"
echo ""
