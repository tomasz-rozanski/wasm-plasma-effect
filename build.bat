@echo off

set TARGET=wasm32-unknown-unknown
set BINARY=target/%TARGET%/release/bare_metal_wasm.wasm

cargo build --target %TARGET% --release

REM NEW PART:
wasm-snip --snip-rust-fmt-code --snip-rust-panicking-code -o %BINARY% %BINARY%

wasm-strip %BINARY%
if not exist www mkdir www
wasm-opt -o www/bare_metal_wasm.wasm -Oz %BINARY%
REM dir www\bare_metal_wasm.wasm
wsl ls -lh ./www/bare_metal_wasm.wasm
