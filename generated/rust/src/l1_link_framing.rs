// @generated from formats/l1_link_framing/format.toml by defs-gen - do not edit, change the source
// SPDX-License-Identifier: BSD-3-Clause
// defs-gen 5
// input sha256 8b0103c5a64b9a8b795c5f658d886fefa0545a0eedb8fc1a054643b74cc774a5
// frame bytes and link context constants of the ColdFire L1 node and peer links
#![allow(dead_code)]
#![allow(clippy::doc_markdown)]
#![cfg_attr(rustfmt, rustfmt_skip)]

pub const FRAME_CHECK_RECEIVE: u32 = 0xf0b8; // established
pub const FRAME_CHECK_SEED: u32 = 0xffff; // established
pub const FRAME_ESCAPE: u32 = 0x7d; // established
pub const FRAME_ESCAPE_XOR: u32 = 0x20; // established
pub const FRAME_FLAG: u32 = 0x7e; // established
pub const NODE_BODY_CAP: u32 = 0x100; // inferred
pub const PEER_BODY_OFFSET: u32 = 0x10; // established
pub const PEER_ESCAPE_TX_A: u32 = 0x11; // inferred
pub const PEER_ESCAPE_TX_B: u32 = 0x13; // inferred
pub const PEER_GROWTH_LIMIT: u32 = 0x410; // established
