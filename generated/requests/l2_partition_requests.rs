// generated from sysco 50f318a26300 L2P_DECODE.h - do not edit, regenerate in sysco
//!
//! Request codes of the L2 controller partition task image.
//! Codes are unique within the image.
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
    PartitionForBrick,
    /// established.
    PartitionEnvironment,
    /// established.
    DatabaseGet,
    /// established.
    DatabaseTime,
    /// established.
    DatabaseSet,
    /// established.
    ModeSet,
    /// inferred.
    ParkRequest,
    /// inferred.
    ReleaseParked,
    /// A value the notes do not name.
    Other(u32),
}

impl CmdCode {
    pub const fn from_bits(v: u32) -> Self {
        match v {
            0x1 => Self::PartitionForBrick,
            0x2 => Self::PartitionEnvironment,
            0x3 => Self::DatabaseGet,
            0x4 => Self::DatabaseTime,
            0x5 => Self::DatabaseSet,
            0x6 => Self::ModeSet,
            0x7 => Self::ParkRequest,
            0x8 => Self::ReleaseParked,
            _ => Self::Other(v as u32),
        }
    }

    pub const fn bits(self) -> u32 {
        match self {
            Self::PartitionForBrick => 0x1,
            Self::PartitionEnvironment => 0x2,
            Self::DatabaseGet => 0x3,
            Self::DatabaseTime => 0x4,
            Self::DatabaseSet => 0x5,
            Self::ModeSet => 0x6,
            Self::ParkRequest => 0x7,
            Self::ReleaseParked => 0x8,
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
