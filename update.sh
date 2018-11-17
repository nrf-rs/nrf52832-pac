#!/usr/bin/env bash
set -x
set -e

# NOTE: Last executed using rust/cargo 1.31.0-beta.4

cargo install \
    --force \
    --git https://github.com/rust-embedded/svd2rust \
    --rev b6d2c9a8076198d8579a9fe2525605991ef7a424 \
    svd2rust

rustup component add rustfmt-preview
cargo install --force --version 0.4.0 form

rm -rf src
mkdir src
svd2rust -i ./nrf52832.svd
form -i lib.rs -o src
rm lib.rs
cargo fmt
rustfmt build.rs
