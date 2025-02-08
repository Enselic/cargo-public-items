#!/usr/bin/env bash
set -o nounset -o errexit -o pipefail -o xtrace

for workspace in . rustdoc-json rustup-toolchain; do
    $1 $workspace
done
