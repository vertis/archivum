#!/bin/bash
# Simple build script for archivum

set -e

# Default target is the host architecture
TARGET=${1:-$(rustc -vV | grep host | cut -d' ' -f2)}

echo "Building archivum for target: $TARGET"

# Install target if not already installed
rustup target add $TARGET

# Build the binary
cargo build --release --target $TARGET

# Output location
echo "Binary built at: target/$TARGET/release/archivum"