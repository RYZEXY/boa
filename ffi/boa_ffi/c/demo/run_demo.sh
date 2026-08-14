#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../../.." && pwd)"
BIN="$SCRIPT_DIR/demo"

case "$(uname -s)" in
  Darwin) LIB_NAME="libboa_ffi.dylib" ;;
  *)      LIB_NAME="libboa_ffi.so" ;;
esac
LIB_PATH="$REPO_ROOT/target/debug/$LIB_NAME"

cd "$REPO_ROOT"
cargo build -p boa_ffi

gcc -std=c11 -I "$REPO_ROOT/ffi/boa_ffi/c/generated" \
  "$SCRIPT_DIR/main.c" \
  "$LIB_PATH" \
  -Wl,-rpath,"$REPO_ROOT/target/debug" \
  -o "$BIN"

"$BIN"
