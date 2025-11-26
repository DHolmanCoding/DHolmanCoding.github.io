#!/bin/bash
# Build script for SPA with client-side routing

set -e

echo "Building site..."
trunk build --release

# Copy index.html to 404.html for GitHub Pages SPA routing
cp dist/index.html dist/404.html

echo "Build complete! Output in dist/"

