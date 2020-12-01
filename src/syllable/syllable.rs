
use super::*;

#[derive(Clone, PartialEq)]
#[repr(u8)]
pub enum Status {
    Empty,
    Initial,
    Medial,
    Final,
}

impl Default for Status {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Default, Clone)]
pub struct Syllable {
    initial_consonant: InitialConsonant,
    medial_vowel: MedialVowel,
    final_consonant: FinalConsonant,
    status: Status,
}

impl Into<char> for Syllable {
    fn into(self) -> char {
        // the formula comes from this Wikipedia page: 
        // https://en.wikipedia.org/wiki/Korean_language_and_computers#Hangul_Syllables_block
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
    pub fn new_with_first(byte: Byte) -> Option<Self> {
        let initial = InitialConsonant::from(byte);
        match initial {
            InitialConsonant::Invalid => None,
            _ => Some(Self {
                initial_consonant: initial,
                medial_vowel: MedialVowel::default(),
                final_consonant: FinalConsonant::default(),
                status: Status::Initial,
            })
        }
    }

    pub fn put(&mut self, byte: Byte) -> Option<Byte> {
        match self.status {
            Status::Empty => self.put_initial_consonant(byte),
            Status::Initial => self.put_medial_vowel(byte),
            Status::Medial => {
                let added = self.medial_vowel.add(byte);
                match added {
                    MedialVowel::Invalid => self.put_final_consonant(byte),
                    _ => {
                        self.medial_vowel = added;
                        None
                    }
                }
            },
            Status::Final => {
                let added = self.final_consonant.add(byte);
                match added {
                    FinalConsonant::Invalid => Some(byte),
                    _ => {
                        self.final_consonant = added;
                        None
                    }
                }
            },
        }
    }
    
    pub fn try_split_with_vowel(&mut self, byte: Byte) -> Result<Self, Byte> {
        if byte.is_vowel() && self.status == Status::Final {
            let ic: InitialConsonant = self.final_consonant.clone().into();
            match ic {
                InitialConsonant::Invalid => Err(byte),
                _ => {
                    let mut splitted = Self::default();
                    splitted.initial_consonant = ic;
                    splitted.status = Status::Initial;
                    if let Some(byte) = splitted.put(byte) {
                        return Err(byte)
                    } 
                    self.final_consonant = FinalConsonant::None;
                    self.status = Status::Medial;
                    Ok(splitted)
                }
            }
        } else {
            Err(byte)
        }
        
    }

    fn put_initial_consonant(&mut self, byte: Byte) -> Option<Byte> {
        let ic = InitialConsonant::from(byte);
        match ic {
            InitialConsonant::Invalid => Some(byte),
            _ => {
                self.initial_consonant = ic;
                self.status = Status::Initial;
                None
            }
        }
    }

    fn put_medial_vowel(&mut self, byte: Byte) -> Option<Byte> {
        let mv = MedialVowel::from(byte);
        match mv {
            MedialVowel::Invalid => Some(byte),
            _ => {
                self.medial_vowel = mv;
                self.status = Status::Medial;
                None
            }
        }
    }

    fn put_final_consonant(&mut self, byte: Byte) -> Option<Byte> {
        let fc = FinalConsonant::from(byte);
        match fc {
            FinalConsonant::Invalid => Some(byte),
            _ => {
                self.final_consonant = fc;
                self.status = Status::Final;
                None
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