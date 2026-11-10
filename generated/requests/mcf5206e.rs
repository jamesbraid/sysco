// generated from sysco 76d848c380c5 MCF5206E_DECODE.h - do not edit, regenerate in sysco
//!
//! On-chip registers of the ColdFire MCF5206e that the L1 firmware uses.
//! Offsets are from the module base register.
//!
//! Grades on each item say what is known about it: `established` is what the
//! shipped driver software does, read in full; `inferred` is a reasoned
//! reading; `unknown` is not derived. None of them is a measurement of the
//! silicon.

#![allow(dead_code)]

/// rw8, established.
pub const OFFSET_RSR: u32 = 64;
/// rw8, established.
pub const OFFSET_SYPCR: u32 = 65;
/// rw8, established.
pub const OFFSET_SWIVR: u32 = 66;
/// w8, established.
pub const OFFSET_SWSR: u32 = 67;
/// established.
pub const SWSR_SERVICE_FIRST: u32 = 85;
/// established.
pub const SWSR_SERVICE_SECOND: u32 = 170;
/// rw16, established.
pub const OFFSET_CSAR0: u32 = 100;
/// rw32, established.
pub const OFFSET_CSMR0: u32 = 104;
/// rw16, established.
pub const OFFSET_CSCR0: u32 = 110;
/// rw16, established.
pub const OFFSET_CSAR1: u32 = 112;
/// rw32, established.
pub const OFFSET_CSMR1: u32 = 116;
/// rw16, established.
pub const OFFSET_CSCR1: u32 = 122;
/// rw16, established.
pub const OFFSET_CSAR2: u32 = 124;
/// rw32, established.
pub const OFFSET_CSMR2: u32 = 128;
/// rw16, established.
pub const OFFSET_CSCR2: u32 = 134;
/// rw16, established.
pub const OFFSET_CSAR3: u32 = 136;
/// rw32, established.
pub const OFFSET_CSMR3: u32 = 140;
/// rw16, established.
pub const OFFSET_CSCR3: u32 = 146;
/// rw16, established.
pub const OFFSET_CSAR4: u32 = 148;
/// rw32, established.
pub const OFFSET_CSMR4: u32 = 152;
/// rw16, established.
pub const OFFSET_CSCR4: u32 = 158;
/// rw16, established.
pub const OFFSET_CSAR5: u32 = 160;
/// rw32, established.
pub const OFFSET_CSMR5: u32 = 164;
/// rw16, established.
pub const OFFSET_CSCR5: u32 = 170;
/// rw16, established.
pub const OFFSET_PAR: u32 = 202;
/// rw16, inferred.
pub const OFFSET_TMR1: u32 = 0x100;
/// rw16, inferred.
pub const OFFSET_TRR1: u32 = 0x104;
/// rw16, inferred.
pub const OFFSET_TMR2: u32 = 0x120;
/// rw16, inferred.
pub const OFFSET_TRR2: u32 = 0x124;
/// established: written to the timer 1 reference.
pub const TRR1_TICK_REFERENCE: u32 = 0x752f;
/// established: value written; field meaning inferred from the manual.
pub const TMR1_INIT: u32 = 29;
/// established.
pub const TRR2_REFERENCE: u32 = 1;
/// established: value written; field meaning inferred.
pub const TMR2_INIT: u32 = 43;
/// w8, inferred.
pub const OFFSET_TMR_AUX_A: u32 = 0x1b8;
/// w8, inferred.
pub const OFFSET_TMR_AUX_B: u32 = 0x1bc;
/// established.
pub const OFFSET_UART1: u32 = 0x140;
/// rw8, established.
pub const OFFSET_PADDR: u32 = 0x1c5;
/// rw8, established.
pub const OFFSET_PADAT: u32 = 0x1c9;
/// established.
pub const PADDR_SCAN_SETUP: u32 = 254;
/// rw8, inferred.
pub const OFFSET_MADR: u32 = 0x1e0;
/// rw8, inferred.
pub const OFFSET_MFDR: u32 = 0x1e4;
/// rw8, inferred.
pub const OFFSET_MBCR: u32 = 0x1e8;
/// rw8, inferred.
pub const OFFSET_MBSR: u32 = 0x1ec;
/// rw8, inferred.
pub const OFFSET_MBDR: u32 = 0x1f0;
/// established: written at bring-up, 100 kHz at the inferred 48 MHz clock.
pub const MFDR_INIT: u32 = 19;

/// Values of `bam` in `Csmr0` that the header names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Csmr0Bam {
    /// established.
    Size1mb,
    /// established.
    Size2mb,
    /// A value the notes do not name.
    Other(u32),
}

impl Csmr0Bam {
    pub const fn from_bits(v: u32) -> Self {
        match v {
            0xf => Self::Size1mb,
            0x1f => Self::Size2mb,
            _ => Self::Other(v as u32),
        }
    }

    pub const fn bits(self) -> u32 {
        match self {
            Self::Size1mb => 0xf,
            Self::Size2mb => 0x1f,
            Self::Other(v) => v as u32,
        }
    }
}

/// The `RSR` word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rsr(u32);

impl Rsr {
    pub const fn new(word: u32) -> Self {
        Self(word)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Bit 5, established.
    pub const fn cause5(self) -> bool {
        (self.0 >> 5) & 1 != 0
    }
}

/// The `SYPCR` word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sypcr(u32);

impl Sypcr {
    pub const fn new(word: u32) -> Self {
        Self(word)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Bit 7, established.
    pub const fn swe(self) -> bool {
        (self.0 >> 7) & 1 != 0
    }
}

/// The `CSMR0` word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Csmr0(u32);

impl Csmr0 {
    pub const fn new(word: u32) -> Self {
        Self(word)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Bits 31:16, established.
    pub const fn bam(self) -> Csmr0Bam {
        Csmr0Bam::from_bits(self.0 >> 16)
    }
}

/// The `PAR` word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Par(u32);

impl Par {
    pub const fn new(word: u32) -> Self {
        Self(word)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Bit 7, established.
    pub const fn timer_pulse(self) -> bool {
        (self.0 >> 7) & 1 != 0
    }

    /// Bits 5:4, established.
    pub const fn scan_sel(self) -> u32 {
        (self.0 >> 4) & 0x3
    }
}

/// The `PADAT` word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Padat(u32);

impl Padat {
    pub const fn new(word: u32) -> Self {
        Self(word)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Bit 0, established.
    pub const fn tdo(self) -> bool {
        self.0 & 1 != 0
    }

    /// Bit 3, established.
    pub const fn psi(self) -> bool {
        (self.0 >> 3) & 1 != 0
    }

    /// Bit 4, established.
    pub const fn trst(self) -> bool {
        (self.0 >> 4) & 1 != 0
    }

    /// Bit 5, established.
    pub const fn tms(self) -> bool {
        (self.0 >> 5) & 1 != 0
    }

    /// Bit 6, established.
    pub const fn tck(self) -> bool {
        (self.0 >> 6) & 1 != 0
    }

    /// Bit 7, established.
    pub const fn tdi(self) -> bool {
        (self.0 >> 7) & 1 != 0
    }
}

/// The `MBSR` word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mbsr(u32);

impl Mbsr {
    pub const fn new(word: u32) -> Self {
        Self(word)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Bit 5, established.
    pub const fn bus_busy(self) -> bool {
        (self.0 >> 5) & 1 != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testvec_sypcr_fe() {
        let w = Sypcr::new(0xfe);
        assert_eq!(w.swe() as u32, 1, "swe");
    }

    #[test]
    fn testvec_csmr0_1f0000() {
        let w = Csmr0::new(0x1f0000);
        assert_eq!(w.bam().bits(), 31, "bam");
    }

    #[test]
    fn testvec_csmr0_f0000() {
        let w = Csmr0::new(0xf0000);
        assert_eq!(w.bam().bits(), 15, "bam");
    }
}
