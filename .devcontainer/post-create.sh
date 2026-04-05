#!/bin/bash
set -e

echo "🚀 Setting up AI Trading Dashboard development environment..."

# Install system dependencies for Tauri
echo "📦 Installing system dependencies..."
sudo apt-get update
sudo apt-get install -y \
    libgtk-3-dev \
    libwebkit2gtk-4.0-dev \
    libappindicator3-dev \
    librsvg2-dev \
    patchelf \
    libssl-dev \
    pkg-config \
    libayatana-appindicator3-dev

# Install Rust targets
echo "🦀 Installing Rust targets..."
rustup target add wasm32-unknown-unknown
rustup target add x86_64-pc-windows-gnu 2>/dev/null || true

# Install cargo tools
echo "🛠️ Installing cargo tools..."
cargo install trunk 2>/dev/null || echo "trunk already installed"
cargo install tauri-cli 2>/dev/null || echo "tauri-cli already installed"
cargo install cargo-watch 2>/dev/null || echo "cargo-watch already installed"

# Setup git safe directory
git config --global --add safe.directory /workspaces/$(basename "$PWD") 2>/dev/null || true

echo "✅ Setup complete!"
echo ""
echo "To build the project:"
echo "  cd src-tauri"
echo "  cargo build --release"
echo ""
echo "To run development server:"
echo "  cd src-tauri && cargo run"
