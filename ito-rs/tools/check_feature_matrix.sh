#!/usr/bin/env bash
# Exercise Ito's independent shipping and experimental CLI feature sets.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

SHIPPING_PACKAGES=(
    -p ito-common
    -p ito-config
    -p ito-core
    -p ito-domain
    -p ito-logging
    -p ito-templates
    -p ito-web
)

run_case() {
    local name="$1"
    shift

    echo "==> feature matrix: $name"
    cargo check -p ito-cli "$@"
    # `assert_cmd::cargo_bin!` resolves the shared target/debug/ito path. Build
    # the requested feature case explicitly so a previous all-feature command
    # cannot leak an experimental binary into default-lane integration tests.
    cargo build -p ito-cli "$@"
    cargo test -p ito-cli "$@"
    cargo clippy -p ito-cli --all-targets "$@" -- \
        -D warnings \
        -D clippy::dbg_macro \
        -D clippy::todo \
        -D clippy::unimplemented
}

run_case default "${SHIPPING_PACKAGES[@]}"
run_case backend-only --no-default-features --features backend
run_case coordination-only --no-default-features --features coordination-branch
run_case all-features --all-features

python3 ito-rs/tools/check_release_features.py
