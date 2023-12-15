use std::fmt::Display;

/// [https://blog.mark-stevens.co.uk/2017/02/manchester-baby-ssem-emulator/]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OpCode {
    JMP = 0b000,
    JRP = 0b001,
    LDN = 0b010,
    STO = 0b011,
    SUB = 0b100,
    SUB2 = 0b101,
    CMP = 0b110,
    STOP = 0b111,
}

impl From<i32> for OpCode {
    fn from(value: i32) -> Self {
        match ((value & 0x0000E000) >> 13) as u8 {
            0b000 => Self::JMP,
            0b001 => Self::JRP,
            0b010 => Self::LDN,
            0b011 => Self::STO,
            0b100 => Self::SUB,
            0b101 => Self::SUB2,
            0b110 => Self::CMP,
            0b111 => Self::STOP,
            _ => unreachable!(),
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::JMP => "JMP",
                Self::JRP => "JRP",
                Self::LDN => "LDN",
                Self::STO => "STO",
                Self::SUB => "SUB",
                Self::SUB2 => "SUB2",
                Self::CMP => "CMP",
                Self::STOP => "STOP",
            }
        )
    }
}
