#!/usr/bin/env bash

cargo install --force --version 0.13.1 svd2rust &&
cargo install --force --version 0.99.2 rustfmt-nightly &&
cargo install --force --version 0.3.0 form &&

rm -rf src && mkdir src &&
svd2rust -i nrf52.svd &&
form -i lib.rs -o src && rm lib.rs &&
cargo fmt &&
rustfmt build.rs
