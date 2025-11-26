#!/bin/bash
# Build script for multi-page Trunk site

set -e

# Build main index.html first
echo "Building index.html..."
trunk build --release

# Build other pages
for page in blog.html quotes.html blog-building-this-site.html; do
    if [ -f "$page" ]; then
        echo "Building $page..."
        trunk build "$page" --release
    fi
done

echo "Build complete! Output in dist/"

