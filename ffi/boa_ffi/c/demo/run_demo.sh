#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="/mnt/c/Users/sunfr/projects/boa"
MANIFEST="$REPO_ROOT/ffi/boa_ffi/Cargo.toml"
BIN="$PWD/demo"

cd "$REPO_ROOT"
cargo build -p boa_ffi --target x86_64-unknown-linux-gnu

gcc -std=c11 -I "$REPO_ROOT/ffi/boa_ffi/c/generated" \
  "$REPO_ROOT/ffi/boa_ffi/c/demo/main.c" \
  "$REPO_ROOT/target/x86_64-unknown-linux-gnu/debug/libboa_ffi.so" \
  -Wl,-rpath,"$REPO_ROOT/target/x86_64-unknown-linux-gnu/debug" \
  -o "$BIN"

"$BIN"
