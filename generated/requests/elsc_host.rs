// generated from sysco 80298431ff24 ELSC_DECODE.h - do not edit, regenerate in sysco
//!
//! Two-wire device addresses the host driver of the Origin 2000 module
//! controller uses: the request mailbox and the non-volatile store.
//!
//! Grades on each item say what is known about it: `established` is what the
//! shipped driver software does, read in full; `inferred` is a reasoned
//! reading; `unknown` is not derived. None of them is a measurement of the
//! silicon.

#![allow(dead_code)]

/// established: the argument in the call; 7-bit form inferred.
pub const MAILBOX_SLAVE: u32 = 8;
/// established.
pub const NVRAM_SLAVE_BASE: u32 = 80;
/// established: address bits 10:8 select the page.
pub const NVRAM_PAGE_LOW_BIT: u32 = 8;
/// established.
pub const NVRAM_PAGE_BITS: u32 = 7;
/// established: addresses 0 to 0x7ff.
pub const NVRAM_PAGES: u32 = 8;
/// established for the call; meaning unknown.
pub const UST_SLAVE: u32 = 32;
