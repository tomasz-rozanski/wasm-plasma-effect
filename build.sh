#!/bin/bash

set -euo pipefail

TARGET=wasm32-unknown-unknown
BINARY=target/$TARGET/release/bare_metal_wasm.wasm

cargo build --target $TARGET --release

# NEW PART:
wasm-snip --snip-rust-fmt-code \
          --snip-rust-panicking-code \
          -o $BINARY \
          $BINARY

wasm-strip $BINARY
mkdir -p www/
wasm-opt -o www/bare_metal_wasm.wasm -Oz $BINARY
ls -lh www/bare_metal_wasm.wasm
