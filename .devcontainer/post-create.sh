#!/bin/bash
set -e

echo "🚀 Setting up AI Trading Dashboard..."

# Install Rust if not present
if ! command -v cargo &> /dev/null; then
    echo "🦀 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Install targets
rustup target add wasm32-unknown-unknown

# Install system dependencies for Tauri
echo "📦 Installing system dependencies..."
sudo apt-get update
sudo apt-get install -y \
    libgtk-3-dev \
    libwebkit2gtk-4.0-dev \
    libappindicator3-dev \
    librsvg2-dev \
    libsoup2.4-dev \
    libjavascriptcoregtk-4.0-dev \
    pkg-config \
    libssl-dev

# Install cargo tools
cargo install trunk 2>/dev/null || true
cargo install tauri-cli 2>/dev/null || true

echo "✅ Setup complete!"
echo ""
echo "Run: cd src-tauri && cargo build --release"
