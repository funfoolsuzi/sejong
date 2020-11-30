use super::Byte;

#[repr(u8)]
pub (super) enum FinalConsonant {
    None,
    G, // ㄱ
    KK, // ㄲ
    GS, // ㄳ
    N, // ㄴ
    NJ, // ㄵ
    NH, // ㄶ
    D, // ㄷ
    L, // ㄹ
    LG, // ㄺ
    LM, // ㄻ
    LB, // ㄼ
    LS, // ㄽ
    LT, // ㄾ
    LP, // ㄿ
    LH, // ㅀ
    M, // ㅁ
    B, // ㅂ
    BS, // ㅄ
    S, // ㅅ
    SS, // ㅆ
    NG, // ㅇ
    J, // ㅈ
    CH, // ㅊ
    K, // ㅋ
    T, // ㅌ
    P, // ㅍ
    H, // ㅎ
    Invalid
}

impl From<u8> for  FinalConsonant {
    fn from(byte: u8) -> Self {
        if byte > Self::H as u8 {
            return Self::Invalid
        }
        unsafe { std::mem::transmute(byte) }
    }
}

impl From<Byte> for FinalConsonant {
    fn from(b: Byte) -> Self {
        match b {
            Byte::G => Self::G,
            Byte::KK => Self::KK,
            Byte::N => Self::N,
            Byte::D => Self::D,
            Byte::R => Self::L,
            Byte::M => Self::M,
            Byte::B => Self::B,
            Byte::S => Self::S,
            Byte::SS => Self::SS,
            Byte::NG => Self::NG,
            Byte::J => Self::J,
            Byte::CH => Self::CH,
            Byte::K => Self::K,
            Byte::T => Self::T,
            Byte::P => Self::P,
            Byte::H => Self::H,
            _ => Self::Invalid,
        }
    }
}

impl Default for FinalConsonant {
    fn default() -> Self {
        Self::None
    }
}