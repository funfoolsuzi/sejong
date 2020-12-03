mod buffer;
mod byte;
mod syllable;

pub use buffer::Buffer;
pub use byte::Byte;

pub fn default() -> Buffer {
    Buffer::default()
}

pub fn with_capacity(cap: usize) -> Buffer {
    Buffer::with_capacity(cap)
}
