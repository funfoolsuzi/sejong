use crate::byte::Byte;
use crate::syllable::Syllable;
use std::convert::TryInto;

const DEFAULT_BUFFER_CAP: usize = 100;

#[derive(Clone)]
pub struct Buffer(Vec<Syllable>);

impl Buffer {
    pub fn with_capacity(cap: usize) -> Self {
        Self(Vec::with_capacity(cap))
    }

    pub fn put<T>(&mut self, byte_candidate: T) -> Option<T>
    where
        T: TryInto<Byte, Error = T>,
    {
        match byte_candidate.try_into() {
            Ok(byte) => {
                self.put_byte(byte);
                None
            }
            Err(returned) => Some(returned),
        }
    }

    pub fn remove_last(&mut self) -> Option<()> {
        if let Some(last) = self.0.last_mut() {
            last.remove_last().or_else(|| self.0.pop().and(Some(())))
        } else {
            None
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
            if let Some(b) = last.put(b) {
                if let Ok(new_syl) = last.try_split_with_vowel(b) {
                    self.0.push(new_syl);
                    return;
                }
            } else {
                return;
            }
        }

        self.0.push(b.into());
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

    #[test]
    fn test_special_consonant() {
        let mut buffer = Buffer::default();
        buffer.put(Byte::N as u8);
        buffer.put(Byte::J as u8);

        assert_eq!("ㄴㅈ", buffer.to_string());
    }

    #[test]
    fn test_syllable_with_four_jamol() {
        let mut buffer = Buffer::default();
        buffer.put(Byte::NG as u8);
        buffer.put(Byte::EU as u8);
        buffer.put(Byte::N as u8);
        buffer.put(Byte::J as u8);
        buffer.put(Byte::NG as u8);
        buffer.put(Byte::EU as u8);
        buffer.put(Byte::N as u8);
        buffer.put(Byte::J as u8);

        assert_eq!("읁읁", buffer.to_string())
    }

    #[test]
    fn test_syllable_splitting_with_two_part_final_consonant() {
        let mut buffer = Buffer::default();
        buffer.put(Byte::NG as u8);
        buffer.put(Byte::EU as u8);
        buffer.put(Byte::N as u8);
        buffer.put(Byte::J as u8);
        buffer.put(Byte::EU as u8);
        buffer.put(Byte::N as u8);
        buffer.put(Byte::J as u8);

        assert_eq!("은즍", buffer.to_string())
    }

    #[test]
    fn test_pop() {
        let mut buffer = Buffer::default();
        assert!(buffer.remove_last().is_none());

        buffer.put(Byte::NG as u8);
        buffer.put(Byte::EU as u8);
        buffer.put(Byte::N as u8);
        buffer.put(Byte::J as u8);
        buffer.put(Byte::EU as u8);
        buffer.put(Byte::N as u8);
        buffer.put(Byte::J as u8);

        assert_eq!("은즍", buffer.to_string());

        assert!(buffer.remove_last().is_some());
        assert_eq!("은즌", buffer.to_string());
        assert!(buffer.remove_last().is_some());
        assert_eq!("은즈", buffer.to_string());
        assert!(buffer.remove_last().is_some());
        assert!(buffer.remove_last().is_some());
        assert_eq!("은", buffer.to_string());
        assert!(buffer.remove_last().is_some());
        assert_eq!("으", buffer.to_string());
    }
}
