#!/usr/bin/env bash
set -o nounset -o pipefail -o errexit

# Ensure there are no circular dependencies.

packages="
    rustup-toolchain
    rustdoc-json
    public-api
    cargo-public-api
"

for package in $packages; do
    cargo publish --dry-run --package $package
done
