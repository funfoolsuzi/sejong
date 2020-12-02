
use super::*;

#[derive(Clone, Copy, Debug)]
pub (crate) enum Syllable {
    Initial(InitialConsonant),
    Medial(InitialConsonant, MedialVowel),
    Final(InitialConsonant, MedialVowel, FinalConsonant),
    VowelOnly(MedialVowel),
    FinalConsonantOnly(FinalConsonant),
}

impl Syllable {
    pub fn new(byte: Byte) -> Option<Self> {
        let ic = InitialConsonant::from(byte);
        if ic != InitialConsonant::Invalid {
            return Some(Self::Initial(ic))
        }
        let vo = MedialVowel::from(byte);
        if vo != MedialVowel::Invalid {
            return Some(Self::VowelOnly(vo))
        }
        let fc = FinalConsonant::from(byte);
        if fc != FinalConsonant::Invalid {
            return Some(Self::FinalConsonantOnly(fc))
        }
        None
    }

    pub fn put(&mut self, byte: Byte) -> Option<Byte> {
        match match self {
            Self::Initial(ic) => Self::handle_initial(ic, byte),
            Self::Medial(ic, mv) => Self::handle_medial(ic, mv, byte),
            Self::Final(ic, mv, fc) => Self::handle_final(ic, mv, fc, byte),
            Self::VowelOnly(_) | Self::FinalConsonantOnly(_) => None,
        } {
            Some(new) => {
                *self = new;
                None
            },
            None => Some(byte),
        }
    }
    
    pub fn try_split_with_vowel(&mut self, byte: Byte) -> Result<Self, Byte> {
        if !byte.is_vowel() {
            return Err(byte)
        }
        match self {
            Self::Final(ic, mv, fc) => {
                match InitialConsonant::from(*fc) {
                    InitialConsonant::Invalid => Err(byte),
                    new_ic => {
                        let mut splitted = Self::Initial(new_ic);
                        if let Some(byte) = splitted.put(byte) {
                            return Err(byte)
                        } 
                        *self = Self::Medial(*ic, *mv);
                        Ok(splitted)
                    }
                }
            },
            _ => Err(byte),
        }
        
    }

    fn handle_initial(ic: &InitialConsonant, byte: Byte) -> Option<Self> {
        let mv = MedialVowel::from(byte);
        match mv {
            MedialVowel::Invalid => None,
            _ => Some(Self::Medial(*ic, mv))
        }
    }

    fn handle_medial(ic: &InitialConsonant, mv: &MedialVowel, byte: Byte) -> Option<Self> {
        let added = mv.add(byte);
        if added != MedialVowel::Invalid {
            return Some(Self::Medial(*ic, added))
        }
        let fc = FinalConsonant::from(byte);
        match ic {
            InitialConsonant::Invalid => None,
            _ => Some(Self::Final(*ic, *mv, fc))
        }
    }

    fn handle_final(ic: &InitialConsonant, mv: &MedialVowel, fc: &FinalConsonant, byte: Byte) -> Option<Self> {
        match fc.add(byte) {
            FinalConsonant::Invalid => None,
            added => Some(Self::Final(*ic, *mv, added)),
        }
    }
}


impl Into<char> for Syllable {
    fn into(self) -> char {
        match self {
            Self::Initial(ic) => ic.into(),
            Self::Medial(ic, mv) => calculate_syllable_u32(ic as u32, mv as u32, 0),
            Self::Final(ic, mv, fc) => calculate_syllable_u32(ic as u32, mv as u32, fc as u32),
            Self::VowelOnly(v) => v.into(),
            _ => '\0', // TODO: needs to handle other case
        }
    }
}

// the formula comes from this Wikipedia page: 
// https://en.wikipedia.org/wiki/Korean_language_and_computers#Hangul_Syllables_block
fn calculate_syllable_u32(initial_consonant: u32, medial_vowel: u32, final_consonant: u32) -> char {
    unsafe {
        std::char::from_u32_unchecked(initial_consonant * 588 + medial_vowel * 28 + final_consonant + 44032)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_char() {
        let mut syllable1 = Syllable::new(Byte::M).unwrap();
        syllable1.put(Byte::A);
        syllable1.put(Byte::N);
        
        let char1: char = syllable1.into();
        assert_eq!(char1, '만');
    }

    #[test]
    fn size_of_syllable() {
        let size = std::mem::size_of::<Syllable>();
        assert_eq!(4, size);
    }
}