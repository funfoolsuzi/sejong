use super::Byte;
use std::convert::TryFrom;

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub(crate) enum MedialVowel {
    A,   // ㅏ
    AE,  // ㅐ
    YA,  // ㅑ
    YAE, // ㅒ
    EO,  // ㅓ
    E,   // ㅔ
    YEO, // ㅕ
    YE,  // ㅖ
    O,   // ㅗ
    WA,  // ㅘ
    WAE, // ㅙ
    OE,  // ㅚ
    YO,  // ㅛ
    U,   // ㅜ
    WO,  // ㅝ
    WE,  // ㅞ
    WI,  // ㅟ
    YU,  // ㅠ
    EU,  // ㅡ
    YI,  // ㅢ
    I,   // ㅣ
}

impl TryFrom<Byte> for MedialVowel {
    type Error = Byte;
    fn try_from(b: Byte) -> Result<Self, Self::Error> {
        match b {
            Byte::A => Ok(Self::A),
            Byte::AE => Ok(Self::AE),
            Byte::YA => Ok(Self::YA),
            Byte::YAE => Ok(Self::YAE),
            Byte::EO => Ok(Self::EO),
            Byte::E => Ok(Self::E),
            Byte::YEO => Ok(Self::YEO),
            Byte::YE => Ok(Self::YE),
            Byte::O => Ok(Self::O),
            Byte::YO => Ok(Self::YO),
            Byte::U => Ok(Self::U),
            Byte::YU => Ok(Self::YU),
            Byte::EU => Ok(Self::EU),
            Byte::I => Ok(Self::I),
            _ => Err(b),
        }
    }
}

impl Default for MedialVowel {
    fn default() -> Self {
        Self::A
    }
}

impl Into<char> for MedialVowel {
    fn into(self) -> char {
        unsafe {
            let x = std::char::from_u32_unchecked(self as u32 + 0x314f);
            x
        }
    }
}

impl TryFrom<(Self, Byte)> for MedialVowel {
    type Error = (Self, Byte);
    fn try_from(input: (Self, Byte)) -> Result<Self, Self::Error> {
        match input.0 {
            Self::O => match input.1 {
                Byte::A => Ok(Self::WA),
                Byte::AE => Ok(Self::WAE),
                Byte::I => Ok(Self::OE),
                _ => Err(input),
            },
            Self::U => match input.1 {
                Byte::EO => Ok(Self::WO),
                Byte::E => Ok(Self::WE),
                Byte::I => Ok(Self::WI),
                _ => Err(input),
            },
            Self::EU => match input.1 {
                Byte::I => Ok(Self::YI),
                _ => Err(input),
            },
            _ => Err(input),
        }
    }
}

impl MedialVowel {
    pub fn try_remove_second_half(self) -> Option<Self> {
        match self {
            Self::WA | Self::WAE | Self::OE => Some(Self::O),
            Self::WO | Self::WE | Self::WI => Some(Self::U),
            Self::YI => Some(Self::EU),
            _ => None,
        }
    }
}
