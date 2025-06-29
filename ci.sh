#!/bin/bash

set -euxo pipefail

cd model
cargo clippy -- --deny warnings

cd ../out

VARIANTS=("m4")
# TESTS=()

for VARIANT in "${VARIANTS[@]}"; do
    cargo build --features "$VARIANT"
    # for TEST in "${TESTS[@]}"; do
    #     cargo build --test "$TEST" --features "$VARIANT"
    # done

    cargo clippy --features "$VARIANT" -- --deny warnings
    # for TEST in "${TESTS[@]}"; do
    #     cargo clippy --test "$TEST" --features "$VARIANT" -- --deny warnings
    # done
done
