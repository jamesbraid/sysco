// generated from defs/l1_link_framing.toml by defs-gen - do not edit, change the source
// frame bytes and link context constants of the ColdFire L1 node and peer links
#![allow(dead_code)]

pub const FRAME_CHECK_RECEIVE: u32 = 0xf0b8; // established
pub const FRAME_CHECK_SEED: u32 = 0xffff; // established
pub const FRAME_ESCAPE: u32 = 0x7d; // established
pub const FRAME_ESCAPE_XOR: u32 = 0x20; // established
pub const FRAME_FLAG: u32 = 0x7e; // established
pub const NODE_BODY_CAP: u32 = 0x100; // inferred
pub const NODE_PAYLOAD_MAX: u32 = 0x80; // inferred
pub const NODE_UART_BLOCK_OFFSET: u32 = 0x30000080; // inferred
pub const PEER_BODY_OFFSET: u32 = 0x10; // established
pub const PEER_ESCAPE_TX_A: u32 = 0x11; // inferred
pub const PEER_ESCAPE_TX_B: u32 = 0x13; // inferred
pub const PEER_GROWTH_LIMIT: u32 = 0x410; // established
