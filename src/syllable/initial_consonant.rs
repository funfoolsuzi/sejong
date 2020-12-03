use super::{Byte, FinalConsonant};
use std::convert::TryFrom;

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub(crate) enum InitialConsonant {
    G,  // ㄱ
    KK, // ㄲ
    N,  // ㄴ
    D,  // ㄷ
    TT, // ㄸ
    R,  // ㄹ
    M,  // ㅁ
    B,  // ㅂ
    PP, // ㅃ
    S,  // ㅅ
    SS, // ㅆ
    NG, // ㅇ
    J,  // ㅈ
    JJ, // ㅉ
    CH, // ㅊ
    K,  // ㅋ
    T,  // ㅌ
    P,  // ㅍ
    H,  // ㅎ
}

impl TryFrom<Byte> for InitialConsonant {
    type Error = Byte;
    fn try_from(b: Byte) -> Result<Self, Self::Error> {
        match b {
            Byte::G => Ok(Self::G),
            Byte::KK => Ok(Self::KK),
            Byte::N => Ok(Self::N),
            Byte::D => Ok(Self::D),
            Byte::TT => Ok(Self::TT),
            Byte::R => Ok(Self::R),
            Byte::M => Ok(Self::M),
            Byte::B => Ok(Self::B),
            Byte::PP => Ok(Self::PP),
            Byte::S => Ok(Self::S),
            Byte::SS => Ok(Self::SS),
            Byte::NG => Ok(Self::NG),
            Byte::J => Ok(Self::J),
            Byte::JJ => Ok(Self::JJ),
            Byte::CH => Ok(Self::CH),
            Byte::K => Ok(Self::K),
            Byte::T => Ok(Self::T),
            Byte::P => Ok(Self::P),
            Byte::H => Ok(Self::H),
            _ => Err(b),
        }
    }
}

impl TryFrom<FinalConsonant> for InitialConsonant {
    type Error = FinalConsonant;
    fn try_from(fc: FinalConsonant) -> Result<Self, Self::Error> {
        match fc {
            FinalConsonant::G => Ok(Self::G),
            FinalConsonant::KK => Ok(Self::KK),
            FinalConsonant::N => Ok(Self::N),
            FinalConsonant::D => Ok(Self::D),
            FinalConsonant::L => Ok(Self::R),
            FinalConsonant::M => Ok(Self::M),
            FinalConsonant::B => Ok(Self::B),
            FinalConsonant::S => Ok(Self::S),
            FinalConsonant::SS => Ok(Self::SS),
            FinalConsonant::NG => Ok(Self::NG),
            FinalConsonant::J => Ok(Self::J),
            FinalConsonant::CH => Ok(Self::CH),
            FinalConsonant::K => Ok(Self::K),
            FinalConsonant::T => Ok(Self::T),
            FinalConsonant::P => Ok(Self::P),
            FinalConsonant::H => Ok(Self::H),
            _ => Err(fc),
        }
    }
}

impl Into<char> for InitialConsonant {
    fn into(self) -> char {
        // Hangul Compability Jamo reference:
        // https://en.wikipedia.org/wiki/Hangul_Compatibility_Jamo
        match self {
            Self::G => 'ㄱ',
            Self::KK => 'ㄲ',
            Self::N => 'ㄴ',
            Self::D => 'ㄷ',
            Self::TT => 'ㄸ',
            Self::R => 'ㄹ',
            Self::M => 'ㅁ',
            Self::B => 'ㅂ',
            Self::PP => 'ㅃ',
            Self::S => 'ㅅ',
            Self::SS => 'ㅆ',
            Self::NG => 'ㅇ',
            Self::J => 'ㅈ',
            Self::JJ => 'ㅉ',
            Self::CH => 'ㅊ',
            Self::K => 'ㅋ',
            Self::T => 'ㅌ',
            Self::P => 'ㅍ',
            Self::H => 'ㅎ',
        }
    }
}

impl Default for InitialConsonant {
    fn default() -> Self {
        Self::G
    }
}
