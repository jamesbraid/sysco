// generated from sysco 7df7db8526cb MMSC_CMD_DECODE.h - do not edit, regenerate in sysco
//!
//! Typed command table of the multi-module system controller firmware:
//! the dispatch position of each command word and the user authority levels.
//!
//! Grades on each item say what is known about it: `established` is what the
//! shipped driver software does, read in full; `inferred` is a reasoned
//! reading; `unknown` is not derived. None of them is a measurement of the
//! silicon.

#![allow(dead_code)]

/// Values of `index` in `Cmd` that the header names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CmdIndex {
    /// established.
    Auth,
    /// established.
    Authority,
    /// established.
    Bs,
    /// established.
    Cecho,
    /// established.
    Com,
    /// established.
    Cons,
    /// established.
    Console,
    /// established.
    Debug,
    /// established.
    Numracks,
    /// established.
    Dest,
    /// established.
    Direct,
    /// established.
    Downloader,
    /// established.
    Elsc,
    /// established.
    Emsg,
    /// established.
    End,
    /// established.
    Esc,
    /// established.
    Exit,
    /// established.
    Ffsc,
    /// established.
    Flash,
    /// established.
    GraphicsCommand,
    /// established.
    Help,
    /// established.
    Ipaddr,
    /// established.
    Kill,
    /// established.
    Log,
    /// established.
    Mem,
    /// established.
    Mmsc,
    /// established.
    Mmsg,
    /// established.
    Msc,
    /// established.
    NapTime,
    /// established.
    Nop,
    /// established.
    Options,
    /// established.
    Pager,
    /// established.
    Pas,
    /// established.
    Password,
    /// established.
    Printenv,
    /// established.
    Pwr,
    /// established.
    Rackid,
    /// established.
    Rat,
    /// established.
    RebootFfsc,
    /// established.
    ResetFfsc,
    /// established.
    ResetMmsc,
    /// established.
    ResetNvram,
    /// established.
    Rmsg,
    /// established.
    Scan,
    /// established.
    Setenv,
    /// established.
    Steal,
    /// established.
    Partlst,
    /// established.
    Partscan,
    /// established.
    Partget,
    /// established.
    Partnam,
    /// established.
    Partclr,
    /// established.
    Partset,
    /// established.
    Swapcons,
    /// established.
    TtyReinit,
    /// established.
    Unsetenv,
    /// established.
    Unsteal,
    /// established.
    Ver,
    /// established.
    Arbiter,
    /// established.
    Macaddr,
    /// A value the notes do not name.
    Other(u8),
}

impl CmdIndex {
    pub const fn from_bits(v: u32) -> Self {
        match v {
            0x0 => Self::Auth,
            0x1 => Self::Authority,
            0x2 => Self::Bs,
            0x3 => Self::Cecho,
            0x4 => Self::Com,
            0x5 => Self::Cons,
            0x6 => Self::Console,
            0x7 => Self::Debug,
            0x8 => Self::Numracks,
            0x9 => Self::Dest,
            0xa => Self::Direct,
            0xb => Self::Downloader,
            0xc => Self::Elsc,
            0xd => Self::Emsg,
            0xe => Self::End,
            0xf => Self::Esc,
            0x10 => Self::Exit,
            0x11 => Self::Ffsc,
            0x12 => Self::Flash,
            0x13 => Self::GraphicsCommand,
            0x14 => Self::Help,
            0x15 => Self::Ipaddr,
            0x16 => Self::Kill,
            0x17 => Self::Log,
            0x18 => Self::Mem,
            0x19 => Self::Mmsc,
            0x1a => Self::Mmsg,
            0x1b => Self::Msc,
            0x1c => Self::NapTime,
            0x1d => Self::Nop,
            0x1e => Self::Options,
            0x1f => Self::Pager,
            0x20 => Self::Pas,
            0x21 => Self::Password,
            0x22 => Self::Printenv,
            0x23 => Self::Pwr,
            0x24 => Self::Rackid,
            0x25 => Self::Rat,
            0x26 => Self::RebootFfsc,
            0x27 => Self::ResetFfsc,
            0x28 => Self::ResetMmsc,
            0x29 => Self::ResetNvram,
            0x2a => Self::Rmsg,
            0x2b => Self::Scan,
            0x2c => Self::Setenv,
            0x2d => Self::Steal,
            0x2e => Self::Partlst,
            0x2f => Self::Partscan,
            0x30 => Self::Partget,
            0x31 => Self::Partnam,
            0x32 => Self::Partclr,
            0x33 => Self::Partset,
            0x34 => Self::Swapcons,
            0x35 => Self::TtyReinit,
            0x36 => Self::Unsetenv,
            0x37 => Self::Unsteal,
            0x38 => Self::Ver,
            0x3a => Self::Arbiter,
            0x3b => Self::Macaddr,
            _ => Self::Other(v as u8),
        }
    }

    pub const fn bits(self) -> u32 {
        match self {
            Self::Auth => 0x0,
            Self::Authority => 0x1,
            Self::Bs => 0x2,
            Self::Cecho => 0x3,
            Self::Com => 0x4,
            Self::Cons => 0x5,
            Self::Console => 0x6,
            Self::Debug => 0x7,
            Self::Numracks => 0x8,
            Self::Dest => 0x9,
            Self::Direct => 0xa,
            Self::Downloader => 0xb,
            Self::Elsc => 0xc,
            Self::Emsg => 0xd,
            Self::End => 0xe,
            Self::Esc => 0xf,
            Self::Exit => 0x10,
            Self::Ffsc => 0x11,
            Self::Flash => 0x12,
            Self::GraphicsCommand => 0x13,
            Self::Help => 0x14,
            Self::Ipaddr => 0x15,
            Self::Kill => 0x16,
            Self::Log => 0x17,
            Self::Mem => 0x18,
            Self::Mmsc => 0x19,
            Self::Mmsg => 0x1a,
            Self::Msc => 0x1b,
            Self::NapTime => 0x1c,
            Self::Nop => 0x1d,
            Self::Options => 0x1e,
            Self::Pager => 0x1f,
            Self::Pas => 0x20,
            Self::Password => 0x21,
            Self::Printenv => 0x22,
            Self::Pwr => 0x23,
            Self::Rackid => 0x24,
            Self::Rat => 0x25,
            Self::RebootFfsc => 0x26,
            Self::ResetFfsc => 0x27,
            Self::ResetMmsc => 0x28,
            Self::ResetNvram => 0x29,
            Self::Rmsg => 0x2a,
            Self::Scan => 0x2b,
            Self::Setenv => 0x2c,
            Self::Steal => 0x2d,
            Self::Partlst => 0x2e,
            Self::Partscan => 0x2f,
            Self::Partget => 0x30,
            Self::Partnam => 0x31,
            Self::Partclr => 0x32,
            Self::Partset => 0x33,
            Self::Swapcons => 0x34,
            Self::TtyReinit => 0x35,
            Self::Unsetenv => 0x36,
            Self::Unsteal => 0x37,
            Self::Ver => 0x38,
            Self::Arbiter => 0x3a,
            Self::Macaddr => 0x3b,
            Self::Other(v) => v as u32,
        }
    }
}

/// Values of `level` in `Auth` that the header names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthLevel {
    /// established.
    Basic,
    /// established.
    Supervisor,
    /// established.
    Service,
    /// A value the notes do not name.
    Other(u32),
}

impl AuthLevel {
    pub const fn from_bits(v: u32) -> Self {
        match v {
            0x64 => Self::Basic,
            0x1f4 => Self::Supervisor,
            0x3e8 => Self::Service,
            _ => Self::Other(v as u32),
        }
    }

    pub const fn bits(self) -> u32 {
        match self {
            Self::Basic => 0x64,
            Self::Supervisor => 0x1f4,
            Self::Service => 0x3e8,
            Self::Other(v) => v as u32,
        }
    }
}

/// Values of `level` in `Gate` that the header names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GateLevel {
    /// established.
    Above499,
    /// established.
    Above999,
    /// A value the notes do not name.
    Other(u32),
}

impl GateLevel {
    pub const fn from_bits(v: u32) -> Self {
        match v {
            0x1f3 => Self::Above499,
            0x3e7 => Self::Above999,
            _ => Self::Other(v as u32),
        }
    }

    pub const fn bits(self) -> u32 {
        match self {
            Self::Above499 => 0x1f3,
            Self::Above999 => 0x3e7,
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

    /// Bits 7:0, established.
    pub const fn index(self) -> CmdIndex {
        CmdIndex::from_bits(self.0 & 0xff)
    }
}

/// The `AUTH` word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Auth(u32);

impl Auth {
    pub const fn new(word: u32) -> Self {
        Self(word)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Bits 15:0, established.
    pub const fn level(self) -> AuthLevel {
        AuthLevel::from_bits(self.0 & 0xffff)
    }
}

/// The `GATE` word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Gate(u32);

impl Gate {
    pub const fn new(word: u32) -> Self {
        Self(word)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Bits 15:0, established.
    pub const fn level(self) -> GateLevel {
        GateLevel::from_bits(self.0 & 0xffff)
    }
}
