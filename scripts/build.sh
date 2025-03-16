#!/bin/bash
# Simple build script for archivum

set -e

# Default target is the host architecture
TARGET=${1:-$(rustc -vV | grep host | cut -d' ' -f2)}

# Check if we want to use musl for Linux targets
USE_MUSL=${2:-false}

# If we're building for Linux and USE_MUSL is true, switch to musl target
if [[ "$TARGET" == *"linux"* ]] && [[ "$USE_MUSL" == "true" ]]; then
  # Convert to musl target
  TARGET=$(echo $TARGET | sed 's/unknown-linux-gnu/unknown-linux-musl/')
  echo "Using musl target for better portability: $TARGET"
fi

echo "Building archivum for target: $TARGET"

# Install target if not already installed
rustup target add $TARGET

# Check if we need to use cross for musl targets
if [[ "$TARGET" == *"linux-musl"* ]]; then
  echo "Using cross for musl target"
  # Install cross if not already installed
  if ! command -v cross &> /dev/null; then
    echo "Installing cross..."
    cargo install cross
  fi
  # Build using cross
  cross build --release --target $TARGET
else
  # Build using cargo
  cargo build --release --target $TARGET
fi

# Output location
echo "Binary built at: target/$TARGET/release/archivum"

echo ""
echo "Note: For Linux builds, consider using musl targets for better portability:"
echo "  ./scripts/build.sh x86_64-unknown-linux-gnu true"
echo "This creates statically linked binaries that work across different Linux distributions."