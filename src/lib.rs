#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

mod buffer;
mod byte;
mod syllable;
pub use buffer::Buffer;
pub use byte::Byte;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn default() -> Buffer {
    Buffer::default()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn with_capacity(cap: usize) -> Buffer {
    Buffer::with_capacity(cap)
}
