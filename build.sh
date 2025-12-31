#!/bin/bash

echo "==================================="
echo "Room Booking System - Build Script"
echo "==================================="
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed!"
    echo "Please install Rust from: https://rustup.rs/"
    echo ""
    echo "Installation command:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "✓ Rust found: $(cargo --version)"
echo ""

# Build the project
echo "Building project..."
cargo build --release

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Build successful!"
    echo ""
    echo "Run the application with:"
    echo "  cargo run --release"
    echo ""
    echo "Or run the binary directly:"
    echo "  ./target/release/room-booking-system"
else
    echo ""
    echo "❌ Build failed!"
    exit 1
fi
