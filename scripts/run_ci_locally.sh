#!/usr/bin/env bash
set -e
rustup component add rustfmt clippy
cargo fmt --all -- --check
cargo clippy --all -- -D warnings
cargo test --all
