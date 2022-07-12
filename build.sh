#!/bin/bash
cargo build --target wasm32-unknown-unknown --release
wasm-gc target/wasm32-unknown-unknown/release/minesweeper_rust_wasm.wasm
cp target/wasm32-unknown-unknown/release/minesweeper_rust_wasm.wasm .

