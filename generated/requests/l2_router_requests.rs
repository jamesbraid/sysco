// generated from sysco 50f318a26300 L2R_DECODE.h - do not edit, regenerate in sysco
//!
//! Request codes of the L2 controller router library.
//! Codes are unique within the library.
//!
//! Grades on each item say what is known about it: `established` is what the
//! shipped driver software does, read in full; `inferred` is a reasoned
//! reading; `unknown` is not derived. None of them is a measurement of the
//! silicon.

#![allow(dead_code)]

/// Values of `code` in `Cmd` that the header names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CmdCode {
    /// established.
    ChunkFirstFlagged,
    /// established.
    ChunkFirst,
    /// established.
    ChunkNext,
    /// A value the notes do not name.
    Other(u32),
}

impl CmdCode {
    pub const fn from_bits(v: u32) -> Self {
        match v {
            0x3001 => Self::ChunkFirstFlagged,
            0x3002 => Self::ChunkFirst,
            0x3003 => Self::ChunkNext,
            _ => Self::Other(v as u32),
        }
    }

    pub const fn bits(self) -> u32 {
        match self {
            Self::ChunkFirstFlagged => 0x3001,
            Self::ChunkFirst => 0x3002,
            Self::ChunkNext => 0x3003,
            Self::Other(v) => v as u32,
        }
    }
}

/// The `CMD` word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cmd(u32);

impl Cmd {
    pub const fn new(word: u32) -> Self {
        Self(word)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Bits 15:0, established.
    pub const fn code(self) -> CmdCode {
        CmdCode::from_bits(self.0 & 0xffff)
    }
}
