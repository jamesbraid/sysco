// generated from defs/l2_partition_requests.toml by defs-gen - do not edit, change the source
// request codes of the L2 partition task image
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
        pub const PARTITION_FOR_BRICK: u32 = 1; // established
        pub const PARTITION_ENVIRONMENT: u32 = 2; // established
        pub const DATABASE_GET: u32 = 3; // established
        pub const DATABASE_TIME: u32 = 4; // established
        pub const DATABASE_SET: u32 = 5; // established
        pub const MODE_SET: u32 = 6; // established
        pub const PARK_REQUEST: u32 = 7; // inferred
        pub const RELEASE_PARKED: u32 = 8; // inferred
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Value {
            PartitionForBrick,
            PartitionEnvironment,
            DatabaseGet,
            DatabaseTime,
            DatabaseSet,
            ModeSet,
            ParkRequest,
            ReleaseParked,
            Other(u32),
        }
        impl Value {
            pub const fn from_raw(v: u32) -> Self {
                match v {
                    1 => Self::PartitionForBrick,
                    2 => Self::PartitionEnvironment,
                    3 => Self::DatabaseGet,
                    4 => Self::DatabaseTime,
                    5 => Self::DatabaseSet,
                    6 => Self::ModeSet,
                    7 => Self::ParkRequest,
                    8 => Self::ReleaseParked,
                    _ => Self::Other(v),
                }
            }
            pub const fn raw(self) -> u32 {
                match self {
                    Self::PartitionForBrick => 1,
                    Self::PartitionEnvironment => 2,
                    Self::DatabaseGet => 3,
                    Self::DatabaseTime => 4,
                    Self::DatabaseSet => 5,
                    Self::ModeSet => 6,
                    Self::ParkRequest => 7,
                    Self::ReleaseParked => 8,
                    Self::Other(v) => v,
                }
            }
        }
    }
}
