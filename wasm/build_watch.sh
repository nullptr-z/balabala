#!/bin/bash
cargo watch -s "wasm-pack build --target nodejs --out-dir wasm/pkg"
