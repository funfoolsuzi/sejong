
mod buffer;
mod syllable;
mod byte;

pub use byte::Byte;
pub use buffer::Buffer;

pub fn default() -> Buffer {
    Buffer::default()
}

pub fn with_capacity(cap: usize) -> Buffer {
    Buffer::with_capacity(cap)
}