// generated from formats/l2_router_requests/format.toml by defs-gen - do not edit, change the source
// request codes of the L2 router library
#![allow(dead_code)]

/// (established)
pub mod cmd {
    pub const WIDTH: u32 = 32;
    pub mod code { // bits 15:0, established
        pub const SHIFT: u32 = 0;
        pub const MASK: u32 = 0xffff;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
        pub const CHUNK_FIRST_FLAGGED: u32 = 12289; // established
        pub const CHUNK_FIRST: u32 = 12290; // established
        pub const CHUNK_NEXT: u32 = 12291; // established
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Value {
            ChunkFirstFlagged,
            ChunkFirst,
            ChunkNext,
            Other(u32),
        }
        impl Value {
            pub const fn from_raw(v: u32) -> Self {
                match v {
                    12289 => Self::ChunkFirstFlagged,
                    12290 => Self::ChunkFirst,
                    12291 => Self::ChunkNext,
                    _ => Self::Other(v),
                }
            }
            pub const fn raw(self) -> u32 {
                match self {
                    Self::ChunkFirstFlagged => 12289,
                    Self::ChunkFirst => 12290,
                    Self::ChunkNext => 12291,
                    Self::Other(v) => v,
                }
            }
        }
    }
}
