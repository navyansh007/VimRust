#!/bin/bash

# VimRust Installation Script
# This script builds and installs VimRust from source

set -e

echo "🦀 VimRust Installation Script"
echo "=============================="

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "✅ Rust found: $(rustc --version)"

# Check Rust version
RUST_VERSION=$(rustc --version | cut -d' ' -f2)
REQUIRED_VERSION="1.70.0"

if ! printf '%s\n%s\n' "$REQUIRED_VERSION" "$RUST_VERSION" | sort -V -C; then
    echo "❌ Rust version $RUST_VERSION is too old. Required: $REQUIRED_VERSION+"
    echo "   Please update Rust: rustup update"
    exit 1
fi

echo "✅ Rust version is compatible"

# Build VimRust
echo "🔨 Building VimRust..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed. Please check the error messages above."
    exit 1
fi

echo "✅ Build successful"

# Install to system (optional)
read -p "📦 Install VimRust to /usr/local/bin? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    if [ "$EUID" -ne 0 ]; then
        echo "🔐 Installing VimRust (requires sudo)..."
        sudo cp target/release/vimrust /usr/local/bin/
    else
        echo "🔐 Installing VimRust..."
        cp target/release/vimrust /usr/local/bin/
    fi
    
    if [ $? -eq 0 ]; then
        echo "✅ VimRust installed to /usr/local/bin/vimrust"
        echo "   You can now run 'vimrust' from anywhere!"
    else
        echo "❌ Installation failed"
        exit 1
    fi
else
    echo "📍 VimRust binary is available at: $(pwd)/target/release/vimrust"
    echo "   You can copy it manually or add it to your PATH"
fi

echo ""
echo "🎉 Installation complete!"
echo ""
echo "Usage:"
echo "  vimrust                 # Open empty editor"
echo "  vimrust filename.txt    # Open file"
echo "  vimrust --help          # Show help"
echo ""
echo "Documentation:"
echo "  📖 User Manual: $(pwd)/user-manual.md"
echo "  📋 Examples: $(pwd)/examples/"
echo "  🐛 Issues: https://github.com/yourusername/VimRust/issues"
echo ""
echo "Happy editing! 🚀"