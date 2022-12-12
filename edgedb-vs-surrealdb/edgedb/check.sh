#!/usr/bin/env bash
set -euo pipefail

cargo fmt
cargo clippy --tests
cargo test -- --test-threads=1 # table cleanup only works when tests are run sequentially
