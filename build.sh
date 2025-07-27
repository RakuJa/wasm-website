#!/bin/bash


# Install Rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y -t wasm32-unknown-unknown --profile minimal

. "$HOME/.cargo/env"

cargo install --locked trunk

# Build project with trunk
trunk build --release