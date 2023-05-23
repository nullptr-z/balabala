#!/bin/bash
cargo watch -w 'src' -s "wasm-pack build --target nodejs --out-dir wasm/pkg"

