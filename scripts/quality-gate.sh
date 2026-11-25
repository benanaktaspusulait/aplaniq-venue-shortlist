#!/bin/sh
set -eu

case "$#:${1-}" in
    '0:') major=false ;;
    '1:--major') major=true ;;
    *)
        printf 'Usage: %s [--major]\n' "$0" >&2
        exit 2
        ;;
esac

cd "$(dirname "$0")/.."

cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo check --locked --all-targets --all-features
cargo test --locked --all-features

if "$major"; then
    cargo build --release --locked
    cargo test --release --locked
    RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps --all-features

    pinned=$(awk -F '"' '/^channel[[:space:]]*=/ {print $2; exit}' rust-toolchain.toml)
    msrv=$(awk -F '"' '/^rust-version[[:space:]]*=/ {print $2; exit}' Cargo.toml)
    : "${pinned:?Missing compiler pin}" "${msrv:?Missing minimum Rust version}"
    if [ "$msrv" != "$pinned" ]; then
        cargo "+$msrv" check --locked --all-targets --all-features
        cargo "+$msrv" test --locked --all-features
    fi
fi
