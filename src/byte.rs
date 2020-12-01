use std::convert::TryFrom;

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Byte {
    BackSpace = 0x8,
    Space = 0x20,

    TT = 0x45, // E
    YAE = 0x4f, // O
    YE, // P
    PP, // Q
    KK, // R
    SS = 0x54, // T
    JJ = 0x57, // W
    
    M = 0x61, // a
    YU, // b
    CH, // c
    NG, // d
    D, // e
    R, // f
    H, // g
    O, // h
    YA, // i
    EO, // j
    A, // k
    I, // l
    EU, // m
    U, // n
    AE, // o
    E, // p
    B, // q
    G, // r
    N, // s
    S, // t
    YEO, // u
    P, // v
    J, // w
    T, // x
    YO, // y
    K = 0x7a, // z
}

impl TryFrom<u8> for Byte {
    type Error = u8;
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        if byte == 0x8 ||
        byte == 0x20 || byte == 0x45 ||
        (byte >= 0x4f && byte <=0x54) ||
        byte == 0x57 ||
        (byte >= 0x61 && byte <= 0x7a) {
            return unsafe { Ok(std::mem::transmute(byte)) }
        }
        return Err(byte);
    }
}


impl TryFrom<char> for Byte {
    type Error = char;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        if c.len_utf8() != 1 {
            return Err(c);
        }
        let mut b = [0u8];
        c.encode_utf8(&mut b);
        match Byte::try_from(b[0]) {
            Ok(byte) => Ok(byte),
            _ => Err(c)
        }
    }
}

impl Byte {
    pub fn is_vowel(&self) -> bool {
        match self {
            Self::YAE | Self::YE | Self::YU | Self::O | Self::YA |
            Self::EO | Self::A | Self::I | Self::EU | Self::U |
            Self::AE | Self::E | Self::YEO | Self::YO => true,
            _ => false
        }
    }
    pub fn is_consonant(&self) -> bool {
        match self {
            Self::TT | Self::PP | Self::KK | Self::SS | Self::JJ |
            Self::M | Self::CH | Self::NG | Self::D | Self::R |
            Self::H | Self::B | Self::G | Self::N | Self::S | Self::P |
            Self::J | Self::T | Self::K => true,
            _ => false,
        }
    }
}
