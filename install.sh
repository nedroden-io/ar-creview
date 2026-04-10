#!/usr/bin/env bash
set -euo pipefail

BIN_DIR="$HOME/bin"
BINARY_NAME="ar-creview"

echo "Building $BINARY_NAME (release)..."
cargo build --release

mkdir -p "$BIN_DIR"

echo "Installing $BINARY_NAME to $BIN_DIR..."
cp "target/release/$BINARY_NAME" "$BIN_DIR/$BINARY_NAME"
chmod +x "$BIN_DIR/$BINARY_NAME"

echo ""
echo "Done! $BINARY_NAME installed to $BIN_DIR/$BINARY_NAME"

if ! echo "$PATH" | grep -q "$BIN_DIR"; then
    echo ""
    echo "Note: $BIN_DIR is not in your PATH."
    echo "Add this to your ~/.zshrc:"
    echo "  export PATH=\"\$HOME/bin:\$PATH\""
fi
