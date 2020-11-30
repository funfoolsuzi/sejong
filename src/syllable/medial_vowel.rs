use super::Byte;

#[repr(u8)]
pub (super) enum MedialVowel {
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

impl From<u8> for MedialVowel {
    fn from(byte: u8) -> Self {
        if byte > Self::I as u8 {
            return Self::Invalid
        }
        unsafe { std::mem::transmute(byte) }
    }
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