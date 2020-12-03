use super::{Byte, InitialConsonant};
use std::convert::{TryFrom, TryInto};

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub(crate) enum FinalConsonant {
    None,
    G,  // ㄱ
    KK, // ㄲ
    GS, // ㄳ
    N,  // ㄴ
    NJ, // ㄵ
    NH, // ㄶ
    D,  // ㄷ
    L,  // ㄹ
    LG, // ㄺ
    LM, // ㄻ
    LB, // ㄼ
    LS, // ㄽ
    LT, // ㄾ
    LP, // ㄿ
    LH, // ㅀ
    M,  // ㅁ
    B,  // ㅂ
    BS, // ㅄ
    S,  // ㅅ
    SS, // ㅆ
    NG, // ㅇ
    J,  // ㅈ
    CH, // ㅊ
    K,  // ㅋ
    T,  // ㅌ
    P,  // ㅍ
    H,  // ㅎ
}

impl TryFrom<Byte> for FinalConsonant {
    type Error = Byte;
    fn try_from(b: Byte) -> Result<Self, Self::Error> {
        match b {
            Byte::G => Ok(Self::G),
            Byte::KK => Ok(Self::KK),
            Byte::N => Ok(Self::N),
            Byte::D => Ok(Self::D),
            Byte::R => Ok(Self::L),
            Byte::M => Ok(Self::M),
            Byte::B => Ok(Self::B),
            Byte::S => Ok(Self::S),
            Byte::SS => Ok(Self::SS),
            Byte::NG => Ok(Self::NG),
            Byte::J => Ok(Self::J),
            Byte::CH => Ok(Self::CH),
            Byte::K => Ok(Self::K),
            Byte::T => Ok(Self::T),
            Byte::P => Ok(Self::P),
            Byte::H => Ok(Self::H),
            _ => Err(b),
        }
    }
}

impl TryFrom<(Self, Byte)> for FinalConsonant {
    type Error = (Self, Byte);
    fn try_from(input: (Self, Byte)) -> Result<Self, Self::Error> {
        match input.0 {
            Self::G => match input.1 {
                Byte::S => Ok(Self::GS),
                _ => Err(input),
            },
            Self::N => match input.1 {
                Byte::J => Ok(Self::NJ),
                Byte::H => Ok(Self::NH),
                _ => Err(input),
            },
            Self::L => match input.1 {
                Byte::G => Ok(Self::LG),
                Byte::M => Ok(Self::LM),
                Byte::B => Ok(Self::LB),
                Byte::S => Ok(Self::LS),
                Byte::T => Ok(Self::LT),
                Byte::P => Ok(Self::LP),
                Byte::H => Ok(Self::LH),
                _ => Err(input),
            },
            Self::B => match input.1 {
                Byte::S => Ok(Self::BS),
                _ => Err(input),
            },
            _ => Err(input),
        }
    }
}

impl Default for FinalConsonant {
    fn default() -> Self {
        Self::None
    }
}

impl TryInto<(Self, InitialConsonant)> for FinalConsonant {
    type Error = Self;
    fn try_into(self) -> Result<(Self, InitialConsonant), Self> {
        match self {
            Self::GS => Ok((Self::G, InitialConsonant::S)),
            Self::NJ => Ok((Self::N, InitialConsonant::J)),
            Self::NH => Ok((Self::N, InitialConsonant::H)),
            Self::LG => Ok((Self::L, InitialConsonant::G)),
            Self::LM => Ok((Self::L, InitialConsonant::M)),
            Self::LB => Ok((Self::L, InitialConsonant::B)),
            Self::LS => Ok((Self::L, InitialConsonant::S)),
            Self::LT => Ok((Self::L, InitialConsonant::T)),
            Self::LP => Ok((Self::L, InitialConsonant::P)),
            Self::LH => Ok((Self::L, InitialConsonant::H)),
            Self::BS => Ok((Self::B, InitialConsonant::S)),
            _ => Err(self),
        }
    }
}
