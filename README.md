# Sejong Buffer

[![Crate](https://img.shields.io/crates/v/sejong.svg)](https://crates.io/crates/sejong)
[![API](https://docs.rs/sejong/badge.svg)](https://docs.rs/sejong)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Sejong Buffer is a buffer that receives ASCII bytes from standard English keyboard and sends out UTF-32 Hangul string. This buffer allows deletion by Jamo.

This rust library can be compiled as a WASM library or as part of another rust program.

# BUILD

## WASM

1. Install [wasm-pack](https://rustwasm.github.io/):
    ` curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh `
2. Build the wasm npm package:
    `wasm-pack build -- --features wasm`

## rust lib

No additional step. Just use as regular rust library.

# USE


