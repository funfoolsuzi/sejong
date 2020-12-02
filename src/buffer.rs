use crate::syllable::Syllable;
use crate::byte::Byte;
use std::convert::TryInto;

const DEFAULT_BUFFER_CAP: usize = 100;

#[derive(Clone)]
pub struct Buffer(Vec<Syllable>);

impl Buffer {
    pub fn with_capacity(cap: usize) -> Self {
        Self(Vec::with_capacity(cap))
    }

    pub fn put<T>(&mut self, byte_candidate: T) -> Option<T> where T:TryInto<Byte, Error = T> {
        match byte_candidate.try_into() {
            Ok(byte) => {
                self.put_byte(byte);
                None
            },
            Err(returned) => Some(returned),
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

    pub fn to_string(&self) -> String {
        let mut result = String::with_capacity(self.0.len());
        for syl in &self.0 {
            result.push((*syl).into());
        }
        result
    }

    fn put_byte(&mut self, b: Byte) {
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

        if let Some(syl) = Syllable::new(b) {
            self.0.push(syl);
        }
    }
}

impl Into<String> for Buffer {
    fn into(self) -> String {
        self.to_string()
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
        buffer.put(Byte::NG as u8);
        buffer.put(Byte::A as u8);
        buffer.put(Byte::N as u8);
        buffer.put(Byte::N as u8);
        buffer.put(Byte::YEO as u8);
        buffer.put(Byte::NG as u8);
        buffer.put(Byte::H as u8);
        buffer.put(Byte::A as u8);
        buffer.put(Byte::S as u8);
        buffer.put(Byte::E as u8);
        buffer.put(Byte::NG as u8);
        buffer.put(Byte::YEO as u8);

        assert_eq!(5, buffer.0.len());

        let word: String = buffer.clone().into();
        assert_eq!("안녕하세여", word);

        assert_eq!("안녕하세여", buffer.out())
    }

    #[test]
    fn test_consecutive_consonants() {
        let mut buffer = Buffer::default();
        buffer.put(Byte::NG as u8);
        buffer.put(Byte::NG as u8);
        buffer.put(Byte::NG as u8);
        buffer.put(Byte::G as u8);
        buffer.put(Byte::K as u8);

        assert_eq!("ㅇㅇㅇㄱㅋ", buffer.to_string())
    }

    #[test]
    fn test_consecutive_vowels() {
        let mut buffer = Buffer::default();
        buffer.put(Byte::YA as u8);
        buffer.put(Byte::YA as u8);
        buffer.put(Byte::A as u8);
        buffer.put(Byte::A as u8);
        buffer.put(Byte::EU as u8);

        assert_eq!("ㅑㅑㅏㅏㅡ", buffer.to_string());
    }
}