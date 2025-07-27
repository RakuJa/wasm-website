#!/bin/bash
set -euo pipefail
export HOME=/root

# Install Rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y -t wasm32-unknown-unknown --profile minimal
source "$HOME/.cargo/env"

# Install trunk using binstall
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
cargo binstall --targets x86_64-unknown-linux-musl -y trunk

# Build project with trunk
trunk build --release