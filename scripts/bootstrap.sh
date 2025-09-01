#!/usr/bin/env bash
set -euo pipefail
echo "Installing rust components..."
rustup default stable
rustup component add rustfmt clippy
echo "Bootstrapped developer toolchain."
