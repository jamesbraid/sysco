// generated from defs/mcf5206e.toml by defs-gen - do not edit, change the source
// on-chip registers of the ColdFire MCF5206e as the L1 firmware uses them
#![allow(dead_code)]

/// The firmware sizes the CS0 flash by writing the mask for 2 MB, then rewrites it for 1 MB when the two halves of the window read back the same. (established)
pub mod csmr0 {
    pub const WIDTH: u32 = 32;
    pub mod bam { // bits 31:16, established
        pub const SHIFT: u32 = 16;
        pub const MASK: u32 = 0xffff;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
        pub const SIZE_1MB: u32 = 15; // established
        pub const SIZE_2MB: u32 = 31; // established
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Value {
            Size1mb,
            Size2mb,
            Other(u32),
        }
        impl Value {
            pub const fn from_raw(v: u32) -> Self {
                match v {
                    15 => Self::Size1mb,
                    31 => Self::Size2mb,
                    _ => Self::Other(v),
                }
            }
            pub const fn raw(self) -> u32 {
                match self {
                    Self::Size1mb => 15,
                    Self::Size2mb => 31,
                    Self::Other(v) => v,
                }
            }
        }
    }
}
/// Chip-select 1 (battery-backed RAM and clock, 8-bit) and chip-select 2 (external RAM, 16-bit) values the board set-up writes. The RAM mask is cut from  (established)
pub mod csmr2 {
    pub const WIDTH: u32 = 32;
    pub mod bam { // bits 31:16, established
        pub const SHIFT: u32 = 16;
        pub const MASK: u32 = 0xffff;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
        pub const SIZE_128KB: u32 = 1; // established
        pub const SIZE_512KB: u32 = 7; // established
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Value {
            Size128kb,
            Size512kb,
            Other(u32),
        }
        impl Value {
            pub const fn from_raw(v: u32) -> Self {
                match v {
                    1 => Self::Size128kb,
                    7 => Self::Size512kb,
                    _ => Self::Other(v),
                }
            }
            pub const fn raw(self) -> u32 {
                match self {
                    Self::Size128kb => 1,
                    Self::Size512kb => 7,
                    Self::Other(v) => v,
                }
            }
        }
    }
}
/// M-Bus address register: the module's own seven-bit slave address, in bits 7 to 1. Bit 0 is reserved. Reset value 0x00. Not the calling byte a master s (inferred)
pub mod madr {
    pub const WIDTH: u32 = 8;
    pub mod adr { // bits 7:1, inferred
        pub const SHIFT: u32 = 1;
        pub const MASK: u32 = 0x7f;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
}
/// M-Bus control register, reset value 0x00. MEN holds the module in reset while clear. RSTA is write-one and always reads zero. Bits 1 and 0 are reserve (inferred)
pub mod mbcr {
    pub const WIDTH: u32 = 8;
    pub mod men { // bits 7:7, inferred
        pub const SHIFT: u32 = 7;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod mien { // bits 6:6, inferred
        pub const SHIFT: u32 = 6;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod msta { // bits 5:5, inferred
        pub const SHIFT: u32 = 5;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod mtx { // bits 4:4, inferred
        pub const SHIFT: u32 = 4;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod txak { // bits 3:3, inferred
        pub const SHIFT: u32 = 3;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod rsta { // bits 2:2, inferred
        pub const SHIFT: u32 = 2;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
}
/// M-Bus data register, reset value 0x00. A write starts the next transmit byte, most significant bit first. A read in receive mode returns the received  (inferred)
pub mod mbdr {
    pub const WIDTH: u32 = 8;
    pub mod data { // bits 7:0, inferred
        pub const SHIFT: u32 = 0;
        pub const MASK: u32 = 0xff;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
}
/// M-Bus module. The offsets are the manual's, as the firmware adds them to the module base. Reset value 0x81 (MCF and RXAK set). Software writes clear o (established)
pub mod mbsr {
    pub const WIDTH: u32 = 32;
    pub mod mcf { // bits 7:7, inferred
        pub const SHIFT: u32 = 7;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod maas { // bits 6:6, inferred
        pub const SHIFT: u32 = 6;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod bus_busy { // bits 5:5, established
        pub const SHIFT: u32 = 5;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod mal { // bits 4:4, inferred
        pub const SHIFT: u32 = 4;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod srw { // bits 2:2, inferred
        pub const SHIFT: u32 = 2;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod mif { // bits 1:1, inferred
        pub const SHIFT: u32 = 1;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod rxak { // bits 0:0, inferred
        pub const SHIFT: u32 = 0;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
}
/// M-Bus frequency divider register, reset value 0x00. MBC5 to MBC0 select a divider from the manual's table 13-2. The table value is a divider, not a bi (inferred)
pub mod mfdr {
    pub const WIDTH: u32 = 8;
    pub mod mbc { // bits 5:0, inferred
        pub const SHIFT: u32 = 0;
        pub const MASK: u32 = 0x3f;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
        pub const DIVIDE_28: u32 = 0; // inferred
        pub const DIVIDE_68: u32 = 7; // inferred
        pub const DIVIDE_480: u32 = 19; // inferred
        pub const DIVIDE_20: u32 = 32; // inferred
        pub const DIVIDE_2048: u32 = 63; // inferred
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Value {
            Divide28,
            Divide68,
            Divide480,
            Divide20,
            Divide2048,
            Other(u32),
        }
        impl Value {
            pub const fn from_raw(v: u32) -> Self {
                match v {
                    0 => Self::Divide28,
                    7 => Self::Divide68,
                    19 => Self::Divide480,
                    32 => Self::Divide20,
                    63 => Self::Divide2048,
                    _ => Self::Other(v),
                }
            }
            pub const fn raw(self) -> u32 {
                match self {
                    Self::Divide28 => 0,
                    Self::Divide68 => 7,
                    Self::Divide480 => 19,
                    Self::Divide20 => 32,
                    Self::Divide2048 => 63,
                    Self::Other(v) => v,
                }
            }
        }
    }
}
/// Parallel port: direction and data, used as the scan-chain interface. (established)
pub mod padat {
    pub const WIDTH: u32 = 32;
    pub mod tdi { // bits 7:7, established
        pub const SHIFT: u32 = 7;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod tck { // bits 6:6, established
        pub const SHIFT: u32 = 6;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod tms { // bits 5:5, established
        pub const SHIFT: u32 = 5;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod trst { // bits 4:4, established
        pub const SHIFT: u32 = 4;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod psi { // bits 3:3, established
        pub const SHIFT: u32 = 3;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod tdo { // bits 0:0, established
        pub const SHIFT: u32 = 0;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
}
/// Pin assignment: bit 7 is pulsed by the timer set-up, bits 5:4 are cleared by the scan set-up. (established)
pub mod par {
    pub const WIDTH: u32 = 32;
    pub mod timer_pulse { // bits 7:7, established
        pub const SHIFT: u32 = 7;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod scan_sel { // bits 5:4, established
        pub const SHIFT: u32 = 4;
        pub const MASK: u32 = 0x3;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
}
/// Reset status: one bit is tested at start-up to record the reset cause. (established)
pub mod rsr {
    pub const WIDTH: u32 = 32;
    pub mod cause5 { // bits 5:5, established
        pub const SHIFT: u32 = 5;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
}
/// Software watchdog control: write-once, armed once by the firmware. (established)
pub mod sypcr {
    pub const WIDTH: u32 = 32;
    pub mod swe { // bits 7:7, established
        pub const SHIFT: u32 = 7;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
}
pub const CS0_START_CSAR: u32 = 0xffe0; // established
pub const CS0_START_CSCR: u32 = 0x1183; // established
pub const CS1_START_CSAR: u32 = 0x2000; // established
pub const CS1_START_CSCR: u32 = 0x1143; // established
pub const CS1_START_CSMR: u32 = 0x0; // established
pub const CS2_START_CSAR: u32 = 0x3000; // established
pub const CS2_START_CSCR: u32 = 0x983; // established
pub const CS3_START_CSAR: u32 = 0x4000; // established
pub const CS3_START_CSCR: u32 = 0x953; // established
pub const CS3_START_CSMR: u32 = 0x0; // established
pub const CS4_START_CSAR: u32 = 0x5000; // established
pub const CS4_START_CSCR: u32 = 0x95f; // established
pub const CS4_START_CSMR: u32 = 0x0; // established
pub const DMCR_START_VALUE: u32 = 0x0; // established
pub const ICR10_START_VALUE: u32 = 0x80; // established
pub const ICR11_RESET_VALUE: u32 = 0x80; // inferred
pub const ICR11_START_VALUE: u32 = 0x99; // established
pub const ICR12_START_VALUE: u32 = 0x1a; // established
pub const ICR13_START_VALUE: u32 = 0x0; // established
pub const ICR14_START_VALUE: u32 = 0x9; // established
pub const ICR15_START_VALUE: u32 = 0xa; // established
pub const ICR1_START_VALUE: u32 = 0x84; // established
pub const ICR2_START_VALUE: u32 = 0x88; // established
pub const ICR3_START_VALUE: u32 = 0x80; // established
pub const ICR4_START_VALUE: u32 = 0x90; // established
pub const ICR5_START_VALUE: u32 = 0x80; // established
pub const ICR6_START_VALUE: u32 = 0x98; // established
pub const ICR7_START_VALUE: u32 = 0x9c; // established
pub const ICR8_START_VALUE: u32 = 0x1d; // established
pub const ICR9_START_VALUE: u32 = 0x94; // established
pub const IMR_START_VALUE: u32 = 0xfffe; // established
pub const MBSR_RESET_VALUE: u32 = 0x81; // inferred
pub const MFDR_INIT: u32 = 0x13; // established
pub const OFFSET_CSAR0: u32 = 0x64; // established
pub const OFFSET_CSAR1: u32 = 0x70; // established
pub const OFFSET_CSAR2: u32 = 0x7c; // established
pub const OFFSET_CSAR3: u32 = 0x88; // established
pub const OFFSET_CSAR4: u32 = 0x94; // established
pub const OFFSET_CSAR5: u32 = 0xa0; // established
pub const OFFSET_CSCR0: u32 = 0x6e; // established
pub const OFFSET_CSCR1: u32 = 0x7a; // established
pub const OFFSET_CSCR2: u32 = 0x86; // established
pub const OFFSET_CSCR3: u32 = 0x92; // established
pub const OFFSET_CSCR4: u32 = 0x9e; // established
pub const OFFSET_CSCR5: u32 = 0xaa; // established
pub const OFFSET_CSMR0: u32 = 0x68; // established
pub const OFFSET_CSMR1: u32 = 0x74; // established
pub const OFFSET_CSMR2: u32 = 0x80; // established
pub const OFFSET_CSMR3: u32 = 0x8c; // established
pub const OFFSET_CSMR4: u32 = 0x98; // established
pub const OFFSET_CSMR5: u32 = 0xa4; // established
pub const OFFSET_DMCR: u32 = 0xc6; // established
pub const OFFSET_ICR1: u32 = 0x14; // established
pub const OFFSET_ICR10: u32 = 0x1d; // established
pub const OFFSET_ICR11: u32 = 0x1e; // established
pub const OFFSET_ICR12: u32 = 0x1f; // established
pub const OFFSET_ICR13: u32 = 0x20; // established
pub const OFFSET_ICR14: u32 = 0x21; // established
pub const OFFSET_ICR15: u32 = 0x22; // established
pub const OFFSET_ICR2: u32 = 0x15; // established
pub const OFFSET_ICR3: u32 = 0x16; // established
pub const OFFSET_ICR4: u32 = 0x17; // established
pub const OFFSET_ICR5: u32 = 0x18; // established
pub const OFFSET_ICR6: u32 = 0x19; // established
pub const OFFSET_ICR7: u32 = 0x1a; // established
pub const OFFSET_ICR8: u32 = 0x1b; // established
pub const OFFSET_ICR9: u32 = 0x1c; // established
pub const OFFSET_IMR: u32 = 0x36; // established
pub const OFFSET_MADR: u32 = 0x1e0; // inferred
pub const OFFSET_MBCR: u32 = 0x1e8; // inferred
pub const OFFSET_MBDR: u32 = 0x1f0; // inferred
pub const OFFSET_MBSR: u32 = 0x1ec; // inferred
pub const OFFSET_MFDR: u32 = 0x1e4; // inferred
pub const OFFSET_PADAT: u32 = 0x1c9; // established
pub const OFFSET_PADDR: u32 = 0x1c5; // established
pub const OFFSET_PAR: u32 = 0xca; // established
pub const OFFSET_RSR: u32 = 0x40; // established
pub const OFFSET_SIMR: u32 = 0x3; // established
pub const OFFSET_SWIVR: u32 = 0x42; // established
pub const OFFSET_SWSR: u32 = 0x43; // established
pub const OFFSET_SYPCR: u32 = 0x41; // established
pub const OFFSET_TMR1: u32 = 0x100; // inferred
pub const OFFSET_TMR2: u32 = 0x120; // inferred
pub const OFFSET_TMR_AUX_A: u32 = 0x1b8; // inferred
pub const OFFSET_TMR_AUX_B: u32 = 0x1bc; // inferred
pub const OFFSET_TRR1: u32 = 0x104; // inferred
pub const OFFSET_TRR2: u32 = 0x124; // inferred
pub const OFFSET_UART1: u32 = 0x140; // established
pub const PADDR_SCAN_SETUP: u32 = 0xfe; // established
pub const SIMR_START_VALUE: u32 = 0xc0; // established
pub const SWSR_SERVICE_FIRST: u32 = 0x55; // established
pub const SWSR_SERVICE_SECOND: u32 = 0xaa; // established
pub const TMR1_INIT: u32 = 0x1d; // established
pub const TMR2_INIT: u32 = 0x2b; // established
pub const TRR1_TICK_REFERENCE: u32 = 0x752f; // established
pub const TRR2_REFERENCE: u32 = 0x1; // established
pub const UART1_VECTOR: u32 = 0x42; // established

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csmr0_f0000() {
        assert_eq!(csmr0::bam::get(0xf0000), 15);
    }

    #[test]
    fn csmr0_1f0000() {
        assert_eq!(csmr0::bam::get(0x1f0000), 31);
    }

    #[test]
    fn csmr2_10000() {
        assert_eq!(csmr2::bam::get(0x10000), 1);
    }

    #[test]
    fn csmr2_70000() {
        assert_eq!(csmr2::bam::get(0x70000), 7);
    }

    #[test]
    fn madr_58() {
        assert_eq!(madr::adr::get(0x58), 44);
    }

    #[test]
    fn mbcr_f0() {
        assert_eq!(mbcr::men::get(0xf0), 1);
        assert_eq!(mbcr::mien::get(0xf0), 1);
        assert_eq!(mbcr::msta::get(0xf0), 1);
        assert_eq!(mbcr::mtx::get(0xf0), 1);
        assert_eq!(mbcr::txak::get(0xf0), 0);
        assert_eq!(mbcr::rsta::get(0xf0), 0);
    }

    #[test]
    fn mbsr_81() {
        assert_eq!(mbsr::mcf::get(0x81), 1);
        assert_eq!(mbsr::rxak::get(0x81), 1);
    }

    #[test]
    fn mfdr_13() {
        assert_eq!(mfdr::mbc::get(0x13), 19);
    }

    #[test]
    fn sypcr_fe() {
        assert_eq!(sypcr::swe::get(0xfe), 1);
    }
}
