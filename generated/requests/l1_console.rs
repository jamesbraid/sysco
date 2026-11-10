// generated from sysco c0c9fa1403a4 L1_CONSOLE_DECODE.h - do not edit, regenerate in sysco
//!
//! Entry layout and argument kinds of the console command tables of
//! the ColdFire L1 firmware.
//!
//! Grades on each item say what is known about it: `established` is what the
//! shipped driver software does, read in full; `inferred` is a reasoned
//! reading; `unknown` is not derived. None of them is a measurement of the
//! silicon.

#![allow(dead_code)]

/// established.
pub const ENTRY_SIZE: u32 = 68;
/// established.
pub const ENTRY_WORDS: u32 = 17;
/// established.
pub const PATTERN_WORDS: u32 = 10;
/// established.
pub const TABLES_MAX: u32 = 10;

/// Entry word `pattern`: read, 32-bit, 10 consecutive registers; reset unknown; established.
pub const REG_PATTERN: u32 = 0x0000;
pub const REG_PATTERN_WIDTH: u32 = 32;
pub const REG_PATTERN_COUNT: u32 = 10;
/// Entry word `handler`: read, 32-bit; reset unknown; established.
pub const REG_HANDLER: u32 = 0x0028;
pub const REG_HANDLER_WIDTH: u32 = 32;
/// Entry word `arg`: read, 32-bit, 4 consecutive registers; reset unknown; established.
pub const REG_ARG: u32 = 0x002c;
pub const REG_ARG_WIDTH: u32 = 32;
pub const REG_ARG_COUNT: u32 = 4;
/// Entry word `help`: read, 32-bit; reset unknown; established.
pub const REG_HELP: u32 = 0x003c;
pub const REG_HELP_WIDTH: u32 = 32;
/// Entry word `hidden`: read, 32-bit; reset unknown; established.
pub const REG_HIDDEN: u32 = 0x0040;
pub const REG_HIDDEN_WIDTH: u32 = 32;

/// Values of `kind` in `Token` that the header names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    /// established.
    End,
    /// established.
    Number,
    /// established.
    Decimal,
    /// established.
    Hex,
    /// established.
    Octal,
    /// established.
    Word,
    /// established.
    RestOfLine,
    /// established.
    PortL1,
    /// established.
    RackSlot,
    /// A value the notes do not name.
    Other(u8),
}

impl TokenKind {
    pub const fn from_bits(v: u32) -> Self {
        match v {
            0x0 => Self::End,
            0x1 => Self::Number,
            0x2 => Self::Decimal,
            0x3 => Self::Hex,
            0x4 => Self::Octal,
            0x5 => Self::Word,
            0x6 => Self::RestOfLine,
            0x8 => Self::PortL1,
            0xb => Self::RackSlot,
            _ => Self::Other(v as u8),
        }
    }

    pub const fn bits(self) -> u32 {
        match self {
            Self::End => 0x0,
            Self::Number => 0x1,
            Self::Decimal => 0x2,
            Self::Hex => 0x3,
            Self::Octal => 0x4,
            Self::Word => 0x5,
            Self::RestOfLine => 0x6,
            Self::PortL1 => 0x8,
            Self::RackSlot => 0xb,
            Self::Other(v) => v as u32,
        }
    }
}

/// The `TOKEN` word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Token(u32);

impl Token {
    pub const fn new(word: u32) -> Self {
        Self(word)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Bits 3:0, established.
    pub const fn kind(self) -> TokenKind {
        TokenKind::from_bits(self.0 & 0xf)
    }
}
