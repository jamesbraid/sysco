// @generated from devices/pcf8584/registers.toml by defs-gen - do not edit, change the source
// SPDX-License-Identifier: BSD-3-Clause
// defs-gen 5
// input sha256 26d3da7fa19f4c7dc8f9862789b781ef6985e363caad44774a9252704a5c1eaa
// control bits and clock-control fields of the PCF8584 I2C-bus controller
#![allow(dead_code)]
#![allow(clippy::doc_markdown)]
#![cfg_attr(rustfmt, rustfmt_skip)]

pub const CLOCK_OSC_12MHZ: u32 = 0x1c; // inferred
pub const CLOCK_OSC_3MHZ: u32 = 0x0; // inferred
pub const CLOCK_OSC_4_43MHZ: u32 = 0x10; // inferred
pub const CLOCK_OSC_6MHZ: u32 = 0x14; // inferred
pub const CLOCK_OSC_8MHZ: u32 = 0x18; // inferred
pub const CLOCK_SCL_11KHZ: u32 = 0x2; // inferred
pub const CLOCK_SCL_1_5KHZ: u32 = 0x3; // inferred
pub const CLOCK_SCL_45KHZ: u32 = 0x1; // inferred
pub const CLOCK_SCL_90KHZ: u32 = 0x0; // inferred
pub const CTRL_ES1: u32 = 0x20; // inferred
pub const CTRL_ESO: u32 = 0x40; // inferred
pub const CTRL_PIN: u32 = 0x80; // inferred
pub const OWN_ADDRESS_MASK: u32 = 0x7f; // inferred
