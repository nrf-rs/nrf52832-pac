#!/usr/bin/env bash
set -x
set -e

cargo install --force --git https://github.com/wez/svd2rust --rev e0de96e90d6fd4c4d7207111fbe72bf2b531d593 svd2rust
cargo install --force --version 0.99.2 rustfmt-nightly
cargo install --force --version 0.3.0 form

for chip in nrf52832 nrf52840 ; do
  cd $chip
  rm -rf src
  mkdir src
  ls -l ../svd/
  svd2rust -i ../svd/$chip.svd --nightly
  form -i lib.rs -o src
  rm lib.rs
  cargo fmt

  # workaround for https://github.com/rust-embedded/svd2rust/issues/232
  grep -v 'deny(warnings)' src/lib.rs > lib.rs
  mv lib.rs src/lib.rs

  rustfmt build.rs
  cd ..
done
