#!/bin/bash
set -e

# Script to extract strings from Slint files and update PO files

# Check for slint-tr-extractor
if ! command -v slint-tr-extractor &> /dev/null; then
  echo "Error: slint-tr-extractor is not installed."
  echo "Install it with: cargo install slint-tr-extractor"
  exit 1
fi

# Ensure we are in the frontend directory
if [ -d "ui" ] && [ -d "lang" ]; then
    : # We are likely in frontend dir
elif [ -d "frontend/ui" ]; then
    cd frontend
else
    echo "Error: Could not find 'ui' and 'lang' directories."
    echo "Run this script from the project root or frontend directory."
    exit 1
fi

echo "Extracting strings from .slint files..."
# Find all .slint files and run extractor
find ui -name "*.slint" | xargs slint-tr-extractor -o lang/rbibli.pot

echo "Template created at lang/rbibli.pot"

# Update .po files
if command -v msgmerge &> /dev/null; then
    for po_file in lang/*.po; do
        if [ -f "$po_file" ]; then
            echo "Updating $po_file..."
            msgmerge --update --backup=off "$po_file" lang/rbibli.pot
        fi
    done
    echo "PO files updated. Checking for fuzzy translations..."
    grep -c "#, fuzzy" lang/*.po || true
else
    echo "msgmerge not found. Skipping PO update."
    echo "Install gettext to enable automatic PO updates."
fi

echo "Done. Please review the updated .po files using Poedit or a text editor."
