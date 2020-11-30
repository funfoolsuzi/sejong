
use super::Byte;

#[repr(u8)]
pub (super) enum InitialConsonant {
    G, // ㄱ
    KK, // ㄲ
    N, // ㄴ
    D, // ㄷ
    TT, // ㄸ
    R, // ㄹ
    M, // ㅁ
    B, // ㅂ
    PP, // ㅃ 
    S, // ㅅ
    SS, // ㅆ
    NG, // ㅇ
    J, // ㅈ
    JJ, // ㅉ
    CH, // ㅊ
    K, // ㅋ
    T, // ㅌ
    P, // ㅍ
    H, // ㅎ
    Invalid,
}

impl From<u8> for InitialConsonant {
    fn from(byte: u8) -> Self {
        if byte > Self::H as u8 {
            return Self::Invalid
        }
        unsafe { std::mem::transmute(byte) }
    }
}

impl From<Byte> for InitialConsonant {
    fn from(b: Byte) -> Self {
        match b {
            Byte::G => Self::G,
            Byte::KK => Self::KK,
            Byte::N => Self::N,
            Byte::D => Self::D,
            Byte::TT => Self::TT,
            Byte::R => Self::R,
            Byte::M => Self::M,
            Byte::B => Self::B,
            Byte::PP => Self::PP,
            Byte::S => Self::S,
            Byte::SS => Self::SS,
            Byte::NG => Self::NG,
            Byte::J => Self::J,
            Byte::JJ => Self::JJ,
            Byte::CH => Self::CH,
            Byte::K => Self::K,
            Byte::T => Self::T,
            Byte::P => Self::P,
            Byte::H => Self::H,
            _ => Self::Invalid,
        }
    }
}

impl Default for InitialConsonant {
    fn default() -> Self {
        Self::G
    }
}