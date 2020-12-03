use std::convert::TryFrom;

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Byte {
    TT = 0x45,  // E
    YAE = 0x4f, // O
    YE,         // P
    PP,         // Q
    KK = 0x52,  // R
    SS = 0x54,  // T
    JJ = 0x57,  // W
    M = 0x61,   // a
    YU,         // b
    CH,         // c
    NG,         // d
    D,          // e
    R,          // f
    H,          // g
    O,          // h
    YA,         // i
    EO,         // j
    A,          // k
    I,          // l
    EU,         // m
    U,          // n
    AE,         // o
    E,          // p
    B,          // q
    G,          // r
    N,          // s
    S,          // t
    YEO,        // u
    P,          // v
    J,          // w
    T,          // x
    YO,         // y
    K = 0x7a,   // z
}

impl TryFrom<u8> for Byte {
    type Error = u8;
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x45 | 0x4f | 0x50 | 0x51 | 0x52 | 0x54 | 0x57 | 0x61 | 0x62 | 0x63 | 0x64 | 0x65
            | 0x66 | 0x67 | 0x68 | 0x69 | 0x6a | 0x6b | 0x6c | 0x6d | 0x6e | 0x6f | 0x70 | 0x71
            | 0x72 | 0x73 | 0x74 | 0x75 | 0x76 | 0x77 | 0x78 | 0x79 | 0x7a => unsafe {
                Ok(std::mem::transmute(byte))
            },
            _ => Err(byte),
        }
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
            _ => Err(c),
        }
    }
}

impl Byte {
    pub fn is_consonant(&self) -> bool {
        match self {
            Self::TT
            | Self::PP
            | Self::KK
            | Self::SS
            | Self::JJ
            | Self::M
            | Self::CH
            | Self::NG
            | Self::D
            | Self::R
            | Self::H
            | Self::B
            | Self::G
            | Self::N
            | Self::S
            | Self::P
            | Self::J
            | Self::T
            | Self::K => true,
            _ => false,
        }
    }
    pub fn is_vowel(&self) -> bool {
        !self.is_consonant()
    }
}
