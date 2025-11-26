#!/bin/bash
# Build script for multi-page Trunk site

set -e

# Build other pages first (they overwrite dist/index.html, so we rename them)
for page in blog.html quotes.html blog-building-this-site.html; do
    if [ -f "$page" ]; then
        echo "Building $page..."
        trunk build "$page" --release
        mv dist/index.html "dist/$page"
    fi
done

# Build main index.html last so it stays as dist/index.html
echo "Building index.html..."
trunk build --release

echo "Build complete! Output in dist/"

