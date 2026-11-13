// generated from formats/l1_console/format.toml by defs-gen - do not edit, change the source
// entry layout and argument kinds of the L1 console command tables
#![allow(dead_code)]

/// Up to ten pattern words, the handler, four selector words, the help text and the hidden flag; 68 bytes in all. (established)
pub mod pattern {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x0;
    pub const COUNT: u32 = 10;
    pub const STRIDE: u64 = 4;
}
/// (established)
pub mod handler {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x28;
}
/// (established)
pub mod arg {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x2c;
    pub const COUNT: u32 = 4;
    pub const STRIDE: u64 = 4;
}
/// (established)
pub mod help {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x3c;
}
/// (established)
pub mod hidden {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x40;
}
/// (established)
pub mod token {
    pub const WIDTH: u32 = 32;
    pub mod kind { // bits 3:0, established
        pub const SHIFT: u32 = 0;
        pub const MASK: u32 = 0xf;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
        pub const END: u32 = 0; // established
        pub const NUMBER: u32 = 1; // established
        pub const DECIMAL: u32 = 2; // established
        pub const HEX: u32 = 3; // established
        pub const OCTAL: u32 = 4; // established
        pub const WORD: u32 = 5; // established
        pub const REST_OF_LINE: u32 = 6; // established
        pub const PORT_L1: u32 = 8; // established
        pub const RACK_SLOT: u32 = 11; // established
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Value {
            End,
            Number,
            Decimal,
            Hex,
            Octal,
            Word,
            RestOfLine,
            PortL1,
            RackSlot,
            Other(u32),
        }
        impl Value {
            pub const fn from_raw(v: u32) -> Self {
                match v {
                    0 => Self::End,
                    1 => Self::Number,
                    2 => Self::Decimal,
                    3 => Self::Hex,
                    4 => Self::Octal,
                    5 => Self::Word,
                    6 => Self::RestOfLine,
                    8 => Self::PortL1,
                    11 => Self::RackSlot,
                    _ => Self::Other(v),
                }
            }
            pub const fn raw(self) -> u32 {
                match self {
                    Self::End => 0,
                    Self::Number => 1,
                    Self::Decimal => 2,
                    Self::Hex => 3,
                    Self::Octal => 4,
                    Self::Word => 5,
                    Self::RestOfLine => 6,
                    Self::PortL1 => 8,
                    Self::RackSlot => 11,
                    Self::Other(v) => v,
                }
            }
        }
    }
}
pub const ENTRY_SIZE: u32 = 0x44; // established
pub const ENTRY_WORDS: u32 = 0x11; // established
pub const PATTERN_WORDS: u32 = 0xa; // established
pub const TABLES_MAX: u32 = 0xa; // established
