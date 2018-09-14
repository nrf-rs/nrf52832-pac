#!/usr/bin/env bash
set -x
set -e

cargo install --force --git https://github.com/wez/svd2rust --rev e0de96e90d6fd4c4d7207111fbe72bf2b531d593 svd2rust
cargo install --force --version 0.99.2 rustfmt-nightly
cargo install --force --version 0.3.0 form

for chip in nrf52832 ; do
  cd $chip
  rm -rf src
  mkdir src
  ls -l ../svd/
  svd2rust -i ../svd/$chip.svd --nightly
  form -i lib.rs -o src
  rm lib.rs
  cargo fmt
  rustfmt build.rs
  cd ..
done
