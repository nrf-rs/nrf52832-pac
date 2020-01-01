#!/usr/bin/env bash
set -x
set -e

# NOTE: Last executed using Rust 1.40.0

cargo install --force --version 0.16.1 svd2rust
cargo install --force --version 0.7.0  form
rustup component add rustfmt

rm -rf src
mkdir src
svd2rust -i ./nrf52832.svd
form -i lib.rs -o src
rm lib.rs
cargo fmt
rustfmt build.rs
