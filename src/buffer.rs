use crate::byte::Byte;
use crate::syllable::Syllable;
use std::convert::TryInto;

const DEFAULT_BUFFER_CAP: usize = 100;

/// It is simply a vector of `Syllable`(private struct).
/// See its methods to find examples.
#[derive(Clone)]
pub struct Buffer(Vec<Syllable>);

impl Buffer {
    /// Initialize the buffer with a capacity. Remember the capacity is for 
    /// `Syllable`(private struct).
    /// It is a capacity for input like `u8` and `char`.
    pub fn with_capacity(cap: usize) -> Self {
        Self(Vec::with_capacity(cap))
    }

    /// Put a byte('u8') or a 'char' into the buffer. A valid byte_candidate should be
    /// meets all of the following requirements:
    ///     1. a valid English letter in ASCII chart.
    ///     2. this English letter must has a corresponding valid modern Hangul Jamo as
    /// in a standard Korean 2-set(QWERT) keyboard.
    /// When a byte_candidate is accepted by the buffer, this will return None.
    /// When a byte_candidate can't be matched with a valid modern Hangul Jamo, this
    /// will return `Some(byte_candidate)`.
    ///
    /// # Example
    /// ```
    /// use sejong::{Buffer, Byte};
    /// let mut buf = Buffer::default();
    /// assert!(buf.put(Byte::NG as u8).is_none());
    /// assert!(buf.put('k').is_none());
    /// assert_eq!(buf.put('M').unwrap(), 'M');
    /// assert_eq!(buf.to_string(), "아");
    /// ```
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

    /// Removes the last single Jamo put. Returns `Some(())` when it succeeds.
    /// Returns `None` when it fails. It fails when buffer is empty.
    ///
    /// # Example
    /// ```
    /// use sejong::{Buffer, Byte};
    /// let mut buf = Buffer::default();
    /// buf.put(Byte::NG as u8);
    /// buf.put(Byte::A as u8);
    /// buf.put(Byte::N as u8);
    /// assert_eq!(buf.to_string(), "안");
    /// assert_eq!(buf.pop().unwrap(), ());
    /// assert_eq!(buf.to_string(), "아");
    /// assert_eq!(buf.pop().unwrap(), ());
    /// assert_eq!(buf.pop().unwrap(), ());
    /// assert_eq!(buf.to_string(), "");
    /// assert!(buf.pop().is_none());
    /// ```
    pub fn pop(&mut self) -> Option<()> {
        if let Some(last) = self.0.last_mut() {
            last.remove_last().or_else(|| self.0.pop().and(Some(())))
        } else {
            None
        }
    }

    /// Output the buffer as a UTF-32 string. Calling this method clears the buffer.
    /// If buffer needs to be preserved, use [`Buffer::to_string`].
    ///
    /// # Example
    /// ```
    /// use sejong::{Buffer, Byte};
    /// let mut buf = Buffer::default();
    /// buf.put(Byte::NG as u8);
    /// buf.put(Byte::A as u8);
    /// buf.put(Byte::N as u8);
    /// assert_eq!(buf.out(), "안");
    /// assert_eq!(buf.to_string(), "");
    /// ```
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

    /// Output the buffer as a UTF-32 string. It is very similar with [`Buffer::out`].
    /// But `to_string()` doesn't clear the buffer. It always reflect the
    /// current state of the buffer.
    ///
    /// # Example
    /// ```
    /// use sejong::{Buffer, Byte};
    /// let mut buf = Buffer::default();
    /// buf.put(Byte::NG as u8);
    /// buf.put(Byte::A as u8);
    /// buf.put(Byte::N as u8);
    /// assert_eq!(buf.to_string(), "안");
    /// assert_eq!(buf.to_string(), "안");
    /// ```
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
        assert!(buffer.pop().is_none());

        buffer.put(Byte::NG as u8);
        buffer.put(Byte::EU as u8);
        buffer.put(Byte::N as u8);
        buffer.put(Byte::J as u8);
        buffer.put(Byte::EU as u8);
        buffer.put(Byte::N as u8);
        buffer.put(Byte::J as u8);

        assert_eq!("은즍", buffer.to_string());

        assert!(buffer.pop().is_some());
        assert_eq!("은즌", buffer.to_string());
        assert!(buffer.pop().is_some());
        assert_eq!("은즈", buffer.to_string());
        assert!(buffer.pop().is_some());
        assert!(buffer.pop().is_some());
        assert_eq!("은", buffer.to_string());
        assert!(buffer.pop().is_some());
        assert_eq!("으", buffer.to_string());
    }
}
