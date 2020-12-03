use super::*;
use std::convert::{TryFrom, TryInto};

#[derive(Clone, Copy, Debug)]
pub(crate) enum Syllable {
    Initial(InitialConsonant),
    Medial(InitialConsonant, MedialVowel),
    Final(InitialConsonant, MedialVowel, FinalConsonant),
    VowelOnly(MedialVowel),
}

impl Syllable {
    pub fn put(&mut self, byte: Byte) -> Option<Byte> {
        if let Some(new) = self.update(byte) {
            *self = new;
            None
        } else {
            Some(byte)
        }
    }
    pub fn try_split_with_vowel(&mut self, byte: Byte) -> Result<Self, Byte> {
        if !byte.is_vowel() {
            return Err(byte);
        }
        if let Self::Final(ic, mv, fc) = self {
            if let Ok(new_ic) = InitialConsonant::try_from(*fc) {
                let mut splitted = Self::Initial(new_ic);
                if let Some(byte) = splitted.put(byte) {
                    return Err(byte);
                }
                *self = Self::Medial(*ic, *mv);
                Ok(splitted)
            } else {
                let split_result: Result<(FinalConsonant, InitialConsonant), FinalConsonant> =
                    (*fc).try_into();
                if let Ok(consonants) = split_result {
                    let mut splitted = Self::Initial(consonants.1);
                    if let Some(byte) = splitted.put(byte) {
                        return Err(byte);
                    }
                    *self = Self::Final(*ic, *mv, consonants.0);
                    Ok(splitted)
                } else {
                    Err(byte)
                }
            }
        } else {
            Err(byte)
        }
    }

    pub fn remove_last(&mut self) -> Option<()> {
        match match self {
            Self::Medial(ic, mv) => match mv.try_remove_second_half() {
                Some(new_mv) => Some(Self::Medial(*ic, new_mv)),
                None => Some(Self::Initial(*ic)),
            },
            Self::Final(ic, mv, fc) => match fc.try_remove_second_half() {
                Some(new_fc) => Some(Self::Final(*ic, *mv, new_fc)),
                None => Some(Self::Medial(*ic, *mv)),
            },
            _ => None,
        } {
            Some(new) => {
                *self = new;
                Some(())
            }
            _ => None,
        }
    }

    fn update(&self, byte: Byte) -> Option<Self> {
        match self {
            Self::Initial(ic) => Self::handle_initial(ic, byte),
            Self::Medial(ic, mv) => Self::handle_medial(ic, mv, byte),
            Self::Final(ic, mv, fc) => Self::handle_final(ic, mv, fc, byte),
            Self::VowelOnly(_) => None,
        }
    }

    fn handle_initial(ic: &InitialConsonant, byte: Byte) -> Option<Self> {
        match MedialVowel::try_from(byte) {
            Ok(mv) => Some(Self::Medial(*ic, mv)),
            Err(_) => None,
        }
    }

    fn handle_medial(ic: &InitialConsonant, mv: &MedialVowel, byte: Byte) -> Option<Self> {
        if let Ok(added) = MedialVowel::try_from((*mv, byte)) {
            return Some(Self::Medial(*ic, added));
        }
        match FinalConsonant::try_from(byte) {
            Ok(fc) => Some(Self::Final(*ic, *mv, fc)),
            Err(_) => None,
        }
    }

    fn handle_final(
        ic: &InitialConsonant,
        mv: &MedialVowel,
        fc: &FinalConsonant,
        byte: Byte,
    ) -> Option<Self> {
        match FinalConsonant::try_from((*fc, byte)) {
            Ok(new) => Some(Self::Final(*ic, *mv, new)),
            Err(_) => None,
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
        }
    }
}

// the formula comes from this Wikipedia page:
// https://en.wikipedia.org/wiki/Korean_language_and_computers#Hangul_Syllables_block
fn calculate_syllable_u32(initial_consonant: u32, medial_vowel: u32, final_consonant: u32) -> char {
    unsafe {
        std::char::from_u32_unchecked(
            initial_consonant * 588 + medial_vowel * 28 + final_consonant + 44032,
        )
    }
}

impl From<Byte> for Syllable {
    fn from(b: Byte) -> Self {
        if b.is_consonant() {
            Self::Initial(InitialConsonant::try_from(b).unwrap())
        } else {
            Self::VowelOnly(MedialVowel::try_from(b).unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_char() {
        let mut syllable1 = Syllable::from(Byte::M);
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

    #[test]
    fn guard_from_byte() {
        let pairs = vec![
            (Byte::TT, 'ㄸ'),
            (Byte::YAE, 'ㅒ'),
            (Byte::YE, 'ㅖ'),
            (Byte::PP, 'ㅃ'),
            (Byte::KK, 'ㄲ'),
            (Byte::SS, 'ㅆ'),
            (Byte::JJ, 'ㅉ'),
            (Byte::M, 'ㅁ'),
            (Byte::YU, 'ㅠ'),
            (Byte::CH, 'ㅊ'),
            (Byte::NG, 'ㅇ'),
            (Byte::D, 'ㄷ'),
            (Byte::R, 'ㄹ'),
            (Byte::H, 'ㅎ'),
            (Byte::O, 'ㅗ'),
            (Byte::YA, 'ㅑ'),
            (Byte::EO, 'ㅓ'),
            (Byte::A, 'ㅏ'),
            (Byte::I, 'ㅣ'),
            (Byte::EU, 'ㅡ'),
            (Byte::U, 'ㅜ'),
            (Byte::AE, 'ㅐ'),
            (Byte::E, 'ㅔ'),
            (Byte::B, 'ㅂ'),
            (Byte::G, 'ㄱ'),
            (Byte::N, 'ㄴ'),
            (Byte::S, 'ㅅ'),
            (Byte::YEO, 'ㅕ'),
            (Byte::P, 'ㅍ'),
            (Byte::J, 'ㅈ'),
            (Byte::T, 'ㅌ'),
            (Byte::YO, 'ㅛ'),
            (Byte::K, 'ㅋ'),
        ];
        for pair in pairs {
            let c: char = Syllable::from(pair.0).into();
            assert_eq!(c, pair.1)
        }
    }
}
