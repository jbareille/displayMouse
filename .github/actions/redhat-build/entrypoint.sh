#!/bin/sh -l

echo "Building for Red Hat Linux"

echo "Installing Rust environment"
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"

echo "Installing dependencies"
rustup target add x86_64-unknown-linux-gnu

echo "Building the project"
cd /github/workspace || exit

ls -l

RUSTFLAGS='-C link-arg=-s' cargo build --release --target x86_64-unknown-linux-gnu

echo "Copy the gnu binary to a different name for release generation"
cp target/x86_64-unknown-linux-gnu/release/displayMouse displayMouse-rh810

echo "Cleaning up"
cargo clean

time=$(date)
echo "time=$time" >> $GITHUB_OUTPUT

