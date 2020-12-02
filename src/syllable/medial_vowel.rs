use super::Byte;

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub (crate) enum MedialVowel {
    A, // ㅏ
    AE, // ㅐ
    YA, // ㅑ
    YAE, // ㅒ
    EO, // ㅓ
    E, // ㅔ
    YEO, // ㅕ
    YE, // ㅖ
    O, // ㅗ
    WA, // ㅘ
    WAE, // ㅙ
    OE, // ㅚ
    YO, // ㅛ
    U, // ㅜ
    WO, // ㅝ
    WE, // ㅞ
    WI, // ㅟ
    YU, // ㅠ
    EU, // ㅡ
    YI, // ㅢ
    I, // ㅣ
    Invalid
}

impl From<Byte> for MedialVowel {
    fn from(b: Byte) -> Self {
        match b {
            Byte::A => Self::A,
            Byte::AE => Self::AE,
            Byte::YA => Self::YA,
            Byte::YAE => Self::YAE,
            Byte::EO => Self::EO,
            Byte::E => Self::E,
            Byte::YEO => Self::YEO,
            Byte::YE => Self::YE,
            Byte::O => Self::O,
            Byte::YO => Self::YO,
            Byte::U => Self::U,
            Byte::YU => Self::YU,
            Byte::EU => Self::EU,
            Byte::I => Self::I,
            _ => Self::Invalid,
        }
    }
}

impl Default for MedialVowel {
    fn default() -> Self {
        Self::A
    }
}

impl MedialVowel {
    pub fn add(&self, b: Byte) -> Self {
        match self {
            Self::O => {
                match b {
                    Byte::A => Self::WA,
                    Byte::AE => Self::WAE,
                    Byte::I => Self::OE,
                    _ => Self::Invalid,
                }
            },
            Self::U => {
                match b {
                    Byte::EO => Self::WO,
                    Byte::E => Self::WE,
                    Byte::I => Self::WI,
                    _ => Self::Invalid,
                }
            }
            Self::EU => {
                match b {
                    Byte::I => Self::YI,
                    _ => Self::Invalid,
                }
            }
            _ => Self::Invalid
        }
    }
}