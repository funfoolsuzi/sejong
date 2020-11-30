
const DEFAULT_BUFFER_CAP: usize = 100;

pub struct Buffer(Vec<Buffer>);

impl Buffer {
    pub fn with_capacity(cap: usize) -> Self {
        Self(Vec::with_capacity(cap))
    }

    
}

impl Default for Buffer {
    fn default() -> Self {
        Self::with_capacity(DEFAULT_BUFFER_CAP)
    }
}
