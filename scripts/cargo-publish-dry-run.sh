#!/usr/bin/env bash
set -o nounset -o pipefail -o errexit

pids=()
for package in rustup-toolchain rustdoc-json public-api cargo-public-api; do
    cargo publish --dry-run -p $package &
    pids+=($!)
done

for pid in "${pids[@]}"; do
    wait $pid || { echo "ERROR: cargo publish --dry-run failed"; exit 1; }
done
