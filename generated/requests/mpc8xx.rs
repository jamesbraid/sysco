// generated from sysco 7b8c98aed437 MPC8XX_DECODE.h - do not edit, regenerate in sysco
//!
//! Start-up values of the PowerPC communications controller on the L2
//! board: machine state, clock registers, serial channel page, PCI bridge.
//!
//! Grades on each item say what is known about it: `established` is what the
//! shipped driver software does, read in full; `inferred` is a reasoned
//! reading; `unknown` is not derived. None of them is a measurement of the
//! silicon.

#![allow(dead_code)]

/// established.
pub const MSR_RESET_VALUE: u32 = 0x1002;
/// established.
pub const IMMR_VALUE: u32 = 0xfa200000;
/// established.
pub const SCCR_START_VALUE: u32 = 0x62000000;
/// established.
pub const PLPRCR_START_VALUE: u32 = 0x5040;
/// established: flash window, 32-bit.
pub const BR0_START_VALUE: u32 = 0xfe000001;
/// established.
pub const OR0_START_VALUE: u32 = 0xff800940;
/// established: DRAM bank.
pub const BR1_START_VALUE: u32 = 129;
/// established.
pub const OR1_START_VALUE: u32 = 0xff000400;
/// established: LCD window, 16-bit.
pub const BR2_START_VALUE: u32 = 0x10000801;
/// established.
pub const OR2_START_VALUE: u32 = 0xfffe0938;
/// established: bank unused.
pub const BR3_START_VALUE: u32 = 0;
/// established.
pub const OR3_START_VALUE: u32 = 0;
/// established: byte-wide peripheral window, 8-bit.
pub const BR4_START_VALUE: u32 = 0xfa100401;
/// established.
pub const OR4_START_VALUE: u32 = 0xfffe0930;
/// established: PCI memory window, 32-bit.
pub const BR5_START_VALUE: u32 = 0x80000001;
/// established.
pub const OR5_START_VALUE: u32 = 0xa0000008;
/// established: PCI bridge register window, 32-bit.
pub const BR6_START_VALUE: u32 = 0xfa210001;
/// established.
pub const OR6_START_VALUE: u32 = 0xffff0108;
/// established: 8 MiB window, 8-bit.
pub const BR7_START_VALUE: u32 = 0xfc000401;
/// established.
pub const OR7_START_VALUE: u32 = 0xff800930;
/// established.
pub const BRGC1_STAGE_A_VALUE: u32 = 0x10000;
/// established.
pub const BRGC1_WRAPPER_VALUE: u32 = 0x100a2;
/// established: argument for device 4.
pub const HC0_WINDOW_BASE: u32 = 0xc0000000;
/// established: argument for device 5.
pub const HC1_WINDOW_BASE: u32 = 0xc0001000;
/// established.
pub const SMC1_PARAM_PAGE: u32 = 0x3e80;
/// established.
pub const PCIB_INIT_004: u32 = 0xf9000147;
/// established.
pub const PCIB_INIT_00C: u32 = 0;
/// established.
pub const PCIB_INIT_010: u32 = 0x1000000;
/// established.
pub const PCIB_INIT_030: u32 = 0;
/// established.
pub const PCIB_INIT_03C: u32 = 0x100;
/// established.
pub const PCIB_INIT_100: u32 = 0x88800080;
/// established.
pub const PCIB_INIT_104: u32 = 0x80000000;
/// established.
pub const PCIB_INIT_110: u32 = 0;
/// established.
pub const PCIB_INIT_114: u32 = 0;
/// established.
pub const PCIB_INIT_13C: u32 = 0;
/// established.
pub const PCIB_INIT_140: u32 = 0x81000000;
/// established.
pub const PCIB_INIT_400: u32 = 0x780008;
/// established.
pub const PCIB_INIT_404: u32 = 0;
/// established.
pub const PCIB_INIT_408: u32 = 0;
/// established.
pub const PCIB_INIT_600: u32 = 0xeff0000f;
/// established.
pub const PCIB_INIT_604: u32 = 0;
/// established.
pub const PCIB_INIT_608: u32 = 0;
/// established.
pub const PCIB_INIT_800: u32 = 0xc1013;
/// established.
pub const PCIB_INIT_F00: u32 = 0x1000000;
/// established.
pub const PCIB_INIT_F04: u32 = 0x800000d1;
/// established.
pub const PCIB_INIT_F10: u32 = 0;
/// established.
pub const PCIB_INIT_F14: u32 = 0xc00000d1;
/// established.
pub const PCIB_INIT_F80: u32 = 0x81000000;

/// Register `sccr`: read/write, 32-bit; reset unknown; established.
pub const REG_SCCR: u32 = 0x0280;
pub const REG_SCCR_WIDTH: u32 = 32;
/// Register `plprcr`: read/write, 32-bit; reset unknown; established.
pub const REG_PLPRCR: u32 = 0x0284;
pub const REG_PLPRCR_WIDTH: u32 = 32;
/// Register `br0`: read/write, 32-bit; reset unknown; established.
pub const REG_BR0: u32 = 0x0100;
pub const REG_BR0_WIDTH: u32 = 32;
/// Register `or0`: read/write, 32-bit; reset unknown; established.
pub const REG_OR0: u32 = 0x0104;
pub const REG_OR0_WIDTH: u32 = 32;
/// Register `br1`: read/write, 32-bit; reset unknown; established.
pub const REG_BR1: u32 = 0x0108;
pub const REG_BR1_WIDTH: u32 = 32;
/// Register `or1`: read/write, 32-bit; reset unknown; established.
pub const REG_OR1: u32 = 0x010c;
pub const REG_OR1_WIDTH: u32 = 32;
/// Register `br2`: read/write, 32-bit; reset unknown; established.
pub const REG_BR2: u32 = 0x0110;
pub const REG_BR2_WIDTH: u32 = 32;
/// Register `or2`: read/write, 32-bit; reset unknown; established.
pub const REG_OR2: u32 = 0x0114;
pub const REG_OR2_WIDTH: u32 = 32;
/// Register `br3`: read/write, 32-bit; reset unknown; established.
pub const REG_BR3: u32 = 0x0118;
pub const REG_BR3_WIDTH: u32 = 32;
/// Register `or3`: read/write, 32-bit; reset unknown; established.
pub const REG_OR3: u32 = 0x011c;
pub const REG_OR3_WIDTH: u32 = 32;
/// Register `br4`: read/write, 32-bit; reset unknown; established.
pub const REG_BR4: u32 = 0x0120;
pub const REG_BR4_WIDTH: u32 = 32;
/// Register `or4`: read/write, 32-bit; reset unknown; established.
pub const REG_OR4: u32 = 0x0124;
pub const REG_OR4_WIDTH: u32 = 32;
/// Register `br5`: read/write, 32-bit; reset unknown; established.
pub const REG_BR5: u32 = 0x0128;
pub const REG_BR5_WIDTH: u32 = 32;
/// Register `or5`: read/write, 32-bit; reset unknown; established.
pub const REG_OR5: u32 = 0x012c;
pub const REG_OR5_WIDTH: u32 = 32;
/// Register `br6`: read/write, 32-bit; reset unknown; established.
pub const REG_BR6: u32 = 0x0130;
pub const REG_BR6_WIDTH: u32 = 32;
/// Register `or6`: read/write, 32-bit; reset unknown; established.
pub const REG_OR6: u32 = 0x0134;
pub const REG_OR6_WIDTH: u32 = 32;
/// Register `br7`: read/write, 32-bit; reset unknown; established.
pub const REG_BR7: u32 = 0x0138;
pub const REG_BR7_WIDTH: u32 = 32;
/// Register `or7`: read/write, 32-bit; reset unknown; established.
pub const REG_OR7: u32 = 0x013c;
pub const REG_OR7_WIDTH: u32 = 32;
/// Register `brgc1`: read/write, 32-bit; reset unknown; inferred.
pub const REG_BRGC1: u32 = 0x09f0;
pub const REG_BRGC1_WIDTH: u32 = 32;
/// Register `cfg_addr`: read/write, 32-bit; reset unknown; established.
pub const REG_CFG_ADDR: u32 = 0x0500;
pub const REG_CFG_ADDR_WIDTH: u32 = 32;
/// Register `cfg_data`: read/write, 32-bit; reset unknown; established.
pub const REG_CFG_DATA: u32 = 0x0504;
pub const REG_CFG_DATA_WIDTH: u32 = 32;
/// Register `reg_004`: read/write, 32-bit; reset unknown; inferred.
pub const REG_REG_004: u32 = 0x0004;
pub const REG_REG_004_WIDTH: u32 = 32;
/// Register `reg_00c`: read/write, 32-bit; reset unknown; inferred.
pub const REG_REG_00C: u32 = 0x000c;
pub const REG_REG_00C_WIDTH: u32 = 32;
/// Register `reg_010`: read/write, 32-bit; reset unknown; inferred.
pub const REG_REG_010: u32 = 0x0010;
pub const REG_REG_010_WIDTH: u32 = 32;
/// Register `reg_030`: read/write, 32-bit; reset unknown; inferred.
pub const REG_REG_030: u32 = 0x0030;
pub const REG_REG_030_WIDTH: u32 = 32;
/// Register `reg_03c`: read/write, 32-bit; reset unknown; inferred.
pub const REG_REG_03C: u32 = 0x003c;
pub const REG_REG_03C_WIDTH: u32 = 32;
/// Register `reg_100`: read/write, 32-bit; reset unknown; inferred.
pub const REG_REG_100: u32 = 0x0100;
pub const REG_REG_100_WIDTH: u32 = 32;
/// Register `reg_104`: read/write, 32-bit; reset unknown; inferred.
pub const REG_REG_104: u32 = 0x0104;
pub const REG_REG_104_WIDTH: u32 = 32;
/// Register `reg_110`: read/write, 32-bit; reset unknown; inferred.
pub const REG_REG_110: u32 = 0x0110;
pub const REG_REG_110_WIDTH: u32 = 32;
/// Register `reg_114`: read/write, 32-bit; reset unknown; inferred.
pub const REG_REG_114: u32 = 0x0114;
pub const REG_REG_114_WIDTH: u32 = 32;
/// Register `reg_13c`: read/write, 32-bit; reset unknown; inferred.
pub const REG_REG_13C: u32 = 0x013c;
pub const REG_REG_13C_WIDTH: u32 = 32;
/// Register `reg_140`: read/write, 32-bit; reset unknown; inferred.
pub const REG_REG_140: u32 = 0x0140;
pub const REG_REG_140_WIDTH: u32 = 32;
/// Register `reg_400`: read/write, 32-bit; reset unknown; inferred.
pub const REG_REG_400: u32 = 0x0400;
pub const REG_REG_400_WIDTH: u32 = 32;
/// Register `reg_404`: read/write, 32-bit; reset unknown; inferred.
pub const REG_REG_404: u32 = 0x0404;
pub const REG_REG_404_WIDTH: u32 = 32;
/// Register `reg_408`: read/write, 32-bit; reset unknown; inferred.
pub const REG_REG_408: u32 = 0x0408;
pub const REG_REG_408_WIDTH: u32 = 32;
/// Register `reg_600`: read/write, 32-bit; reset unknown; inferred.
pub const REG_REG_600: u32 = 0x0600;
pub const REG_REG_600_WIDTH: u32 = 32;
/// Register `reg_604`: read/write, 32-bit; reset unknown; inferred.
pub const REG_REG_604: u32 = 0x0604;
pub const REG_REG_604_WIDTH: u32 = 32;
/// Register `reg_608`: read/write, 32-bit; reset unknown; inferred.
pub const REG_REG_608: u32 = 0x0608;
pub const REG_REG_608_WIDTH: u32 = 32;
/// Register `reg_800`: read/write, 32-bit; reset unknown; inferred.
pub const REG_REG_800: u32 = 0x0800;
pub const REG_REG_800_WIDTH: u32 = 32;
/// Register `reg_f00`: read/write, 32-bit; reset unknown; inferred.
pub const REG_REG_F00: u32 = 0x0f00;
pub const REG_REG_F00_WIDTH: u32 = 32;
/// Register `reg_f04`: read/write, 32-bit; reset unknown; inferred.
pub const REG_REG_F04: u32 = 0x0f04;
pub const REG_REG_F04_WIDTH: u32 = 32;
/// Register `reg_f10`: read/write, 32-bit; reset unknown; inferred.
pub const REG_REG_F10: u32 = 0x0f10;
pub const REG_REG_F10_WIDTH: u32 = 32;
/// Register `reg_f14`: read/write, 32-bit; reset unknown; inferred.
pub const REG_REG_F14: u32 = 0x0f14;
pub const REG_REG_F14_WIDTH: u32 = 32;
/// Register `reg_f80`: read/write, 32-bit; reset unknown; inferred.
pub const REG_REG_F80: u32 = 0x0f80;
pub const REG_REG_F80_WIDTH: u32 = 32;

/// The `MSR` word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Msr(u32);

impl Msr {
    pub const fn new(word: u32) -> Self {
        Self(word)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Bit 1, inferred.
    pub const fn ri(self) -> bool {
        (self.0 >> 1) & 1 != 0
    }

    /// Bit 12, inferred.
    pub const fn me(self) -> bool {
        (self.0 >> 12) & 1 != 0
    }
}

/// The `PCI_CFG_ADDR` word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PciCfgAddr(u32);

impl PciCfgAddr {
    pub const fn new(word: u32) -> Self {
        Self(word)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Bits 23:16, inferred.
    pub const fn bus(self) -> u32 {
        (self.0 >> 16) & 0xff
    }

    /// Bits 15:8, inferred.
    pub const fn devfn(self) -> u32 {
        (self.0 >> 8) & 0xff
    }

    /// Bits 7:2, established.
    pub const fn register(self) -> u32 {
        (self.0 >> 2) & 0x3f
    }

    /// Bit 0, established.
    pub const fn type1(self) -> bool {
        self.0 & 1 != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testvec_msr_1002() {
        let w = Msr::new(0x1002);
        assert_eq!(w.ri() as u32, 1, "ri");
        assert_eq!(w.me() as u32, 1, "me");
    }
}
