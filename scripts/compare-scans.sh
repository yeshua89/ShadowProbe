#!/bin/bash
# Compare two scan results to see differences

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <scan1.json> <scan2.json>"
    exit 1
fi

SCAN1="$1"
SCAN2="$2"

if [ ! -f "$SCAN1" ] || [ ! -f "$SCAN2" ]; then
    echo "Error: One or both files not found"
    exit 1
fi

echo "🔍 ShadowProbe Scan Comparison"
echo "=============================="
echo ""
echo "Scan 1: $SCAN1"
echo "Scan 2: $SCAN2"
echo ""

# Extract vulnerability counts
VULNS1=$(jq '.vulnerabilities | length' "$SCAN1" 2>/dev/null || echo "0")
VULNS2=$(jq '.vulnerabilities | length' "$SCAN2" 2>/dev/null || echo "0")

echo "📊 Summary:"
echo "  Scan 1: $VULNS1 vulnerabilities"
echo "  Scan 2: $VULNS2 vulnerabilities"
echo "  Difference: $((VULNS2 - VULNS1))"
echo ""

# Extract severity breakdown
echo "🎯 Severity Breakdown:"
echo ""

for severity in Critical High Medium Low; do
    COUNT1=$(jq "[.vulnerabilities[] | select(.severity == \"$severity\")] | length" "$SCAN1" 2>/dev/null || echo "0")
    COUNT2=$(jq "[.vulnerabilities[] | select(.severity == \"$severity\")] | length" "$SCAN2" 2>/dev/null || echo "0")
    DIFF=$((COUNT2 - COUNT1))

    if [ $DIFF -gt 0 ]; then
        SYMBOL="↑"
        COLOR="\033[0;31m"
    elif [ $DIFF -lt 0 ]; then
        SYMBOL="↓"
        COLOR="\033[0;32m"
    else
        SYMBOL="="
        COLOR="\033[0;37m"
    fi

    printf "  %-10s: %3d → %3d ${COLOR}[%s%d]${NC}\n" "$severity" "$COUNT1" "$COUNT2" "$SYMBOL" "${DIFF#-}"
done

echo ""

# Extract new vulnerabilities
echo "🆕 New Vulnerabilities in Scan 2:"
jq -r '.vulnerabilities[] | "\(.vuln_type) - \(.severity) - \(.url)"' "$SCAN2" > /tmp/scan2_vulns.txt 2>/dev/null
jq -r '.vulnerabilities[] | "\(.vuln_type) - \(.severity) - \(.url)"' "$SCAN1" > /tmp/scan1_vulns.txt 2>/dev/null

NEW_VULNS=$(comm -13 <(sort /tmp/scan1_vulns.txt) <(sort /tmp/scan2_vulns.txt) | wc -l)
echo "  Total: $NEW_VULNS new findings"
echo ""

if [ "$NEW_VULNS" -gt 0 ]; then
    comm -13 <(sort /tmp/scan1_vulns.txt) <(sort /tmp/scan2_vulns.txt) | head -10 | while read line; do
        echo "  • $line"
    done
fi

# Fixed vulnerabilities
echo ""
echo "✅ Fixed Vulnerabilities (in Scan 1 but not Scan 2):"
FIXED=$(comm -23 <(sort /tmp/scan1_vulns.txt) <(sort /tmp/scan2_vulns.txt) | wc -l)
echo "  Total: $FIXED fixed"
echo ""

if [ "$FIXED" -gt 0 ]; then
    comm -23 <(sort /tmp/scan1_vulns.txt) <(sort /tmp/scan2_vulns.txt) | head -10 | while read line; do
        echo "  • $line"
    done
fi

# Cleanup
rm -f /tmp/scan1_vulns.txt /tmp/scan2_vulns.txt

echo ""
echo "=============================="
