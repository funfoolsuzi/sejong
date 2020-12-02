use super::Byte;

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub (crate) enum FinalConsonant {
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

impl FinalConsonant {
    pub fn add(&self, b: Byte) -> Self {
        match self {
            Self::G => {
                match b {
                    Byte::S => Self::GS,
                    _ => Self::Invalid,
                }
            },
            Self::N => {
                match b {
                    Byte::J => Self::NJ,
                    Byte::H => Self::NH,
                    _ => Self::Invalid,
                }
            },
            Self::L => {
                match b {
                    Byte::G => Self::LG,
                    Byte::M => Self::LM,
                    Byte::B => Self::LB,
                    Byte::S => Self::LS,
                    Byte::T => Self::LT,
                    Byte::P => Self::LP,
                    Byte::H => Self::LH,
                    _ => Self::Invalid,
                }
            },
            Self::B => {
                match b {
                    Byte::S => Self::BS,
                    _ => Self::Invalid,
                }
            }
            _ => Self::Invalid
        }
    }
}