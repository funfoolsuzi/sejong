
use super::*;

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Status {
    Empty,
    First,
    Second,
    Complete,
}

impl Default for Status {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Default)]
pub struct Syllable {
    initial_consonant: InitialConsonant,
    medial_vowel: MedialVowel,
    final_consonant: FinalConsonant,
    status: Status,
}

impl Into<char> for Syllable {
    fn into(self) -> char {
        unsafe {
            std::char::from_u32_unchecked(
                (self.initial_consonant as u32) * 588 +
                (self.medial_vowel as u32) * 28 +
                (self.final_consonant as u32) +
                44032
            )
        }
    }
}

impl Syllable {
    pub fn new_with_first(byte: u8) -> Option<Self> {
        let initial = InitialConsonant::from(byte);
        match initial {
            InitialConsonant::Invalid => None,
            _ => Some(Self {
                initial_consonant: initial,
                medial_vowel: MedialVowel::default(),
                final_consonant: FinalConsonant::default(),
                status: Status::First,
            })
        }
    }

    pub fn put(&mut self, byte: Byte) -> Option<Status> {
        match self.status {
            Status::Empty => self.put_initial_consonant(byte),
            Status::First => self.put_medial_vowel(byte),
            Status::Second => self.put_final_consonant(byte),
            Status::Complete => None,
        }
    }

    fn put_initial_consonant(&mut self, byte: Byte) -> Option<Status> {
        let ic = InitialConsonant::from(byte);
        match ic {
            InitialConsonant::Invalid => None,
            _ => {
                self.initial_consonant = ic;
                self.status = Status::First;
                Some(self.status)
            }
        }
    }

    fn put_medial_vowel(&mut self, byte: Byte) -> Option<Status> {
        let mv = MedialVowel::from(byte);
        match mv {
            MedialVowel::Invalid => None,
            _ => {
                self.medial_vowel = mv;
                self.status = Status::Second;
                Some(self.status)
            }
        }
    }

    fn put_final_consonant(&mut self, byte: Byte) -> Option<Status> {
        let fc = FinalConsonant::from(byte);
        match fc {
            FinalConsonant::Invalid => None,
            _ => {
                self.final_consonant = fc;
                self.status = Status::Complete;
                Some(self.status)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_char() {
        let mut syllable1 = Syllable::default();
        syllable1.put(Byte::M);
        syllable1.put(Byte::A);
        syllable1.put(Byte::N);
        
        let char1: char = syllable1.into();
        assert_eq!(char1, '만');
    }
}