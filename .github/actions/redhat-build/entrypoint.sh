#!/bin/sh -l

echo "Building for Red Hat Linux"

echo "Installing Rust environment"
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

echo "Installing dependencies"
rustup target add x86_64-unknown-linux-gnu

echo "Building the project"
RUSTFLAGS='-C link-arg=-s' cargo build --release --target x86_64-unknown-linux-gnu

echo "Copy the gnu binary to a different name for release generation"
cp target/x86_64-unknown-linux-gnu/release/displayMouse target/x86_64-unknown-linux-gnu/release/displayMouse-gnu

time=$(date)
echo "time=$time" >> $GITHUB_OUTPUT

