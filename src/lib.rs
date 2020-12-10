#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

mod buffer;
mod byte;
mod syllable;
pub use buffer::Buffer;
pub use byte::Byte;

#[cfg(feature = "wasm")]
#[macro_use]
extern crate lazy_static;

#[cfg(feature = "wasm")]
lazy_static! {
    static ref BUFFER: std::sync::Mutex<Buffer> = std::sync::Mutex::new(Buffer::default());
}

#[cfg(feature = "wasm")]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn put(c: char) -> Option<String> {
    let mut b = BUFFER.lock().unwrap();
    match b.put(c) {
        None => Some(b.to_string()),
        _ => None,
    }
}

#[cfg(feature = "wasm")]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn pop() -> Option<String> {
    let mut b = BUFFER.lock().unwrap();
    b.remove_last().map(|_| b.to_string())
}

#[cfg(feature = "wasm")]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn out() -> String {
    let mut b = BUFFER.lock().unwrap();
    b.out()
}