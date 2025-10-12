#!/bin/bash
set -e

echo "Installing Windsurf rules..."

# Create .windsurf/rules directory
mkdir -p .windsurf/rules

# Copy all rule files
cp windsurf-config/rules/*.md .windsurf/rules/

echo "✅ Rules installed to .windsurf/rules/"
echo ""
echo "Next steps:"
echo "1. Open Windsurf"
echo "2. Click Customizations icon (top-right)"
echo "3. Navigate to Rules panel"
echo "4. Configure activation modes for each rule"
echo ""
echo "See windsurf-config/README.md for activation mode details"
