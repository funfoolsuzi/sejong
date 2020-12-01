use crate::syllable::Syllable;
use crate::byte::Byte;

const DEFAULT_BUFFER_CAP: usize = 100;

#[derive(Clone)]
pub struct Buffer(Vec<Syllable>);

impl Buffer {
    pub fn with_capacity(cap: usize) -> Self {
        Self(Vec::with_capacity(cap))
    }

    pub fn put(&mut self, b: Byte) {
        if let Some(last) = self.0.last_mut() {
            match last.put(b) {
                Some(b) => {
                    match last.try_split_with_vowel(b) {
                        Ok(new_syl) => {
                            self.0.push(new_syl);
                            return
                        },
                        _ => {}
                    }
                },
                _ => return,
            };
        }

        if let Some(syl) = Syllable::new_with_first(b) {
            self.0.push(syl);
        }
    }

    pub fn out(&mut self) -> String {
        let mut result = String::with_capacity(self.0.len());
        self.0.reverse();
        loop {
            match self.0.pop() {
                Some(syl) => result.push(syl.into()),
                None => break,
            }
        }
        result
    }
}

impl Into<String> for Buffer {
    fn into(self) -> String {
        let mut result = String::with_capacity(self.0.len());
        for syl in self.0 {
            result.push(syl.into());
        }
        result
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::with_capacity(DEFAULT_BUFFER_CAP)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer() {
        let mut buffer = Buffer::default();
        buffer.put(Byte::NG);
        buffer.put(Byte::A);
        buffer.put(Byte::N);
        buffer.put(Byte::N);
        buffer.put(Byte::YEO);
        buffer.put(Byte::NG);
        buffer.put(Byte::H);
        buffer.put(Byte::A);
        buffer.put(Byte::S);
        buffer.put(Byte::E);
        buffer.put(Byte::NG);
        buffer.put(Byte::YEO);

        assert_eq!(5, buffer.0.len());

        let word: String = buffer.clone().into();
        assert_eq!("안녕하세여", word);

        assert_eq!("안녕하세여", buffer.out())
    }
}