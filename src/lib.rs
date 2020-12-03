use wasm_bindgen::prelude::wasm_bindgen;

mod buffer;
mod byte;
mod syllable;

pub use buffer::Buffer;
pub use byte::Byte;

#[wasm_bindgen]
pub fn default() -> Buffer {
    Buffer::default()
}

#[wasm_bindgen]
pub fn with_capacity(cap: usize) -> Buffer {
    Buffer::with_capacity(cap)
}
