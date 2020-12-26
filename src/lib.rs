
//! Sejong [`Buffer`] takes English letters([`Byte`]) that appears on standard
//! keyboard and convert them to corresponding Hangul Jamos as
//! in standard Korean 2-set keyboard. It can output complete Hangul
//! Syllables as a UTF-32 string. It also allows deletion by Hangul
//! Jamo.
//! 
//! # Example
//! ```
//! use sejong::{Buffer, Byte};
//! let mut buf = Buffer::default();
//! buf.put(Byte::NG as u8);
//! buf.put(Byte::A as u8);
//! buf.put(Byte::N as u8);
//! buf.put(Byte::N as u8);
//! buf.put(Byte::YEO as u8);
//! buf.put(Byte::NG as u8);
//! 
//! assert_eq!(buf.to_string(), "안녕");
//! 
//! buf.put(Byte::N as u8);
//! assert_eq!(buf.to_string(), "안녕ㄴ");
//! 
//! buf.pop();
//! assert_eq!(buf.out(), "안녕");
//! assert_eq!(buf.out(), "");
//! ```

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

/// This is a simple wrapper for [`Buffer::put`]. 
/// When used as a WASM module, this lib instantiate a global
/// [`Buffer`] and this method is using the global instance.
#[cfg(any(feature = "wasm", doc))]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn put(c: char) -> Option<String> {
    let mut b = BUFFER.lock().unwrap();
    match b.put(c) {
        None => Some(b.to_string()),
        _ => None,
    }
}

/// This is a simple wrapper for [`Buffer::pop`]. 
/// When used as a WASM module, this lib instantiate a global
/// [`Buffer`] and this method is using the global instance.
#[cfg(any(feature = "wasm", doc))]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn pop() -> Option<String> {
    let mut b = BUFFER.lock().unwrap();
    b.pop().map(|_| b.to_string())
}

/// This is a simple wrapper for [`Buffer::out`]. 
/// When used as a WASM module, this lib instantiate a global
/// [`Buffer`] and this method is using the global instance.
#[cfg(any(feature = "wasm", doc))]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn out() -> String {
    let mut b = BUFFER.lock().unwrap();
    b.out()
}