# Sejong Buffer

[![Crate](https://img.shields.io/crates/v/sejong.svg)](https://crates.io/crates/sejong)
[![NPM](https://img.shields.io/npm/v/sejong-buffer)](https://www.npmjs.com/package/sejong-buffer)
[![API](https://docs.rs/sejong/badge.svg)](https://docs.rs/sejong)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Sejong Buffer is a buffer that receives ASCII bytes from standard English keyboard and sends out UTF-32 Hangul string. This buffer allows deletion by Jamo.

This rust library can be compiled as a WASM library or as part of another rust program.

# USE

## To use as a NPM package

Install: `npm install sejong-buffer`

In apps that built with webpack:

```js
import('sejong-buffer').then(buffer => {
    buffer.put('d');
    buffer.put('k');
    buffer.put('s');
    buffer.put('s');
    buffer.put('u');
    buffer.put('d');

    console.assert(buffer.to_string() === '안녕');

    buffer.put('s');
    console.assert(buffer.to_string() === '안녕ㄴ');

    buffer.pop();
    console.assert(buffer.out() === '안녕');
    console.assert(buffer.out() === '');
});
```

## To use in Rust program

Install: `cargo install sejong`

```rust
use sejong::{Buffer, Byte};
let mut buf = Buffer::default();
buf.put(Byte::NG as u8);
buf.put(Byte::A as u8);
buf.put(Byte::N as u8);
buf.put(Byte::N as u8);
buf.put(Byte::YEO as u8);
buf.put(Byte::NG as u8);

assert_eq!(buf.to_string(), "안녕");

buf.put(Byte::N as u8);
assert_eq!(buf.to_string(), "안녕ㄴ");

buf.pop();
assert_eq!(buf.out(), "안녕");
assert_eq!(buf.out(), "");
```

# BUILD

## WASM

1. Install [wasm-pack](https://rustwasm.github.io/):
    ` curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh `
2. Build the wasm npm package:
    `wasm-pack build --release -- --features wasm`

## rust lib

No additional step. Just use as regular rust library.
