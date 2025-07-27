#!/bin/bash
set -euo pipefail
export HOME=/root

# Install Rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y -t wasm32-unknown-unknown --profile minimal

cargo install --locked trunk

# Build project with trunk
trunk build --release