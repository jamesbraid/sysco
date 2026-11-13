// generated from devices/mpc8xx/registers.toml by defs-gen - do not edit, change the source
// start-up values of the PowerPC communications controller on the L2 board
#![allow(dead_code)]

/// Byte-wide peripheral on the fourth chip select. Established: the window decode above, the 8-bit port, and that the boot code only writes it, in the or (established)
pub mod reg0 {
    pub const WIDTH: u32 = 8;
    pub const OFFSET: u64 = 0x0;
}
/// (established)
pub mod reg1 {
    pub const WIDTH: u32 = 8;
    pub const OFFSET: u64 = 0x1;
}
/// (established)
pub mod reg3 {
    pub const WIDTH: u32 = 8;
    pub const OFFSET: u64 = 0x3;
}
/// First baud-rate generator, loaded by stage A of the console loader. Its place in the register map is the public map's: the store goes through a base t (inferred)
pub mod brgc1 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x9f0;
}
/// Clock and reset registers written by the reset code; the multiplication factor and division fields were not decoded. (established)
pub mod sccr {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x280;
}
/// (established)
pub mod plprcr {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x284;
}
/// Memory controller banks: the base and option register of each of the eight banks, written from a 14-word table (banks 1 to 7) and, for bank 0, built a (established)
pub mod br0 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x100;
}
/// (established)
pub mod or0 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x104;
}
/// (established)
pub mod br1 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x108;
}
/// (established)
pub mod or1 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x10c;
}
/// (established)
pub mod br2 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x110;
}
/// (established)
pub mod or2 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x114;
}
/// (established)
pub mod br3 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x118;
}
/// (established)
pub mod or3 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x11c;
}
/// (established)
pub mod br4 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x120;
}
/// (established)
pub mod or4 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x124;
}
/// (established)
pub mod br5 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x128;
}
/// (established)
pub mod or5 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x12c;
}
/// (established)
pub mod br6 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x130;
}
/// (established)
pub mod or6 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x134;
}
/// (established)
pub mod br7 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x138;
}
/// (established)
pub mod or7 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x13c;
}
/// (inferred)
pub mod reg_004 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x4;
}
/// (inferred)
pub mod reg_00c {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0xc;
}
/// (inferred)
pub mod reg_010 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x10;
}
/// (inferred)
pub mod reg_030 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x30;
}
/// (inferred)
pub mod reg_03c {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x3c;
}
/// (inferred)
pub mod reg_100 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x100;
}
/// (inferred)
pub mod reg_104 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x104;
}
/// (inferred)
pub mod reg_110 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x110;
}
/// (inferred)
pub mod reg_114 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x114;
}
/// (inferred)
pub mod reg_13c {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x13c;
}
/// (inferred)
pub mod reg_140 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x140;
}
/// (inferred)
pub mod reg_400 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x400;
}
/// (inferred)
pub mod reg_404 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x404;
}
/// (inferred)
pub mod reg_408 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x408;
}
/// The bridge's configuration-access pair, used by the second stage to reach the devices behind it. The address register is loaded first, then the data r (established)
pub mod cfg_addr {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x500;
}
/// (established)
pub mod cfg_data {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x504;
}
/// (inferred)
pub mod reg_600 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x600;
}
/// (inferred)
pub mod reg_604 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x604;
}
/// (inferred)
pub mod reg_608 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x608;
}
/// (inferred)
pub mod reg_800 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x800;
}
/// (inferred)
pub mod reg_f00 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0xf00;
}
/// (inferred)
pub mod reg_f04 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0xf04;
}
/// (inferred)
pub mod reg_f10 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0xf10;
}
/// (inferred)
pub mod reg_f14 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0xf14;
}
/// (inferred)
pub mod reg_f80 {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0xf80;
}
/// System protection control and software service registers. The boot code stores one 32-bit word to the control register only on its failure path (image (established)
pub mod sypcr {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x4;
}
/// (established)
pub mod swsr {
    pub const WIDTH: u32 = 16;
    pub const OFFSET: u64 = 0xe;
}
/// Memory controller timing and the user-programmable machine A. The second stage writes the periodic timer prescaler and the machine mode register, load (established)
pub mod mar {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x164;
}
/// (established)
pub mod mcr {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x168;
}
/// (established)
pub mod mamr {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x170;
}
/// (established)
pub mod mptpr {
    pub const WIDTH: u32 = 16;
    pub const OFFSET: u64 = 0x17a;
}
/// (established)
pub mod mdr {
    pub const WIDTH: u32 = 32;
    pub const OFFSET: u64 = 0x17c;
}
/// Machine state register written once by the reset code: machine-check enable and recoverable-interrupt set, translation off, supervisor mode. The bit p (inferred)
pub mod msr {
    pub const WIDTH: u32 = 32;
    pub mod me { // bits 12:12, inferred
        pub const SHIFT: u32 = 12;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod ri { // bits 1:1, inferred
        pub const SHIFT: u32 = 1;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
}
/// (inferred)
pub mod pci_cfg_addr {
    pub const WIDTH: u32 = 32;
    pub mod bus { // bits 23:16, inferred
        pub const SHIFT: u32 = 16;
        pub const MASK: u32 = 0xff;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod devfn { // bits 15:8, inferred
        pub const SHIFT: u32 = 8;
        pub const MASK: u32 = 0xff;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod register { // bits 7:2, established
        pub const SHIFT: u32 = 2;
        pub const MASK: u32 = 0x3f;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod type1 { // bits 0:0, established
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
pub const BR0_START_VALUE: u32 = 0xfe000001; // established
pub const BR1_START_VALUE: u32 = 0x81; // established
pub const BR2_START_VALUE: u32 = 0x10000801; // established
pub const BR3_START_VALUE: u32 = 0x0; // established
pub const BR4_START_VALUE: u32 = 0xfa100401; // established
pub const BR5_START_VALUE: u32 = 0x80000001; // established
pub const BR6_START_VALUE: u32 = 0xfa210001; // established
pub const BR7_START_VALUE: u32 = 0xfc000401; // established
pub const BRGC1_STAGE_A_VALUE: u32 = 0x10000; // established
pub const BRGC1_WRAPPER_VALUE: u32 = 0x100a2; // established
pub const BYTEDEV_MAPPED_SIZE: u32 = 0x1000; // established
pub const BYTEDEV_REG0_FIRST: u32 = 0x0; // established
pub const BYTEDEV_REG0_SECOND: u32 = 0x80; // established
pub const BYTEDEV_REG1_FIRST: u32 = 0xe; // established
pub const BYTEDEV_REG1_LAST: u32 = 0xe; // established
pub const BYTEDEV_REG1_SECOND: u32 = 0xf; // established
pub const BYTEDEV_REG3_VALUE: u32 = 0x18; // established
pub const FLASH_CMD_AUTOSELECT: u32 = 0x900090; // established
pub const FLASH_CMD_ERASE_SETUP: u32 = 0x800080; // established
pub const FLASH_CMD_PROGRAM: u32 = 0xa000a0; // established
pub const FLASH_CMD_RESET: u32 = 0xf000f0; // established
pub const FLASH_CMD_SECTOR_ERASE: u32 = 0x300030; // established
pub const FLASH_STATUS_ERROR: u32 = 0x200020; // established
pub const FLASH_STATUS_TOGGLE: u32 = 0x400040; // established
pub const FLASH_UNLOCK_DATA_1: u32 = 0xaa00aa; // established
pub const FLASH_UNLOCK_DATA_2: u32 = 0x550055; // established
pub const FLASH_UNLOCK_OFFSET_1: u32 = 0x1554; // established
pub const FLASH_UNLOCK_OFFSET_2: u32 = 0xaa8; // established
pub const HC0_WINDOW_BASE: u32 = 0xc0000000; // established
pub const HC1_WINDOW_BASE: u32 = 0xc0001000; // established
pub const IMMR_VALUE: u32 = 0xfa200000; // established
pub const MAMR_START_VALUE: u32 = 0x18804000; // established
pub const MAR_START_VALUE: u32 = 0x88; // established
pub const MCR_RUN_FIRST: u32 = 0x8000202d; // established
pub const MCR_RUN_LAST: u32 = 0x80002013; // established
pub const MCR_RUN_SECOND: u32 = 0x80002030; // established
pub const MPTPR_START_VALUE: u32 = 0x200; // established
pub const MSR_RESET_VALUE: u32 = 0x1002; // established
pub const OR0_START_VALUE: u32 = 0xff800940; // established
pub const OR1_START_VALUE: u32 = 0xff000400; // established
pub const OR2_START_VALUE: u32 = 0xfffe0938; // established
pub const OR3_START_VALUE: u32 = 0x0; // established
pub const OR4_START_VALUE: u32 = 0xfffe0930; // established
pub const OR5_START_VALUE: u32 = 0xa0000008; // established
pub const OR6_START_VALUE: u32 = 0xffff0108; // established
pub const OR7_START_VALUE: u32 = 0xff800930; // established
pub const PCIB_INIT_004: u32 = 0xf9000147; // established
pub const PCIB_INIT_00C: u32 = 0x0; // established
pub const PCIB_INIT_010: u32 = 0x1000000; // established
pub const PCIB_INIT_030: u32 = 0x0; // established
pub const PCIB_INIT_03C: u32 = 0x100; // established
pub const PCIB_INIT_100: u32 = 0x88800080; // established
pub const PCIB_INIT_104: u32 = 0x80000000; // established
pub const PCIB_INIT_110: u32 = 0x0; // established
pub const PCIB_INIT_114: u32 = 0x0; // established
pub const PCIB_INIT_13C: u32 = 0x0; // established
pub const PCIB_INIT_140: u32 = 0x81000000; // established
pub const PCIB_INIT_400: u32 = 0x780008; // established
pub const PCIB_INIT_404: u32 = 0x0; // established
pub const PCIB_INIT_408: u32 = 0x0; // established
pub const PCIB_INIT_600: u32 = 0xeff0000f; // established
pub const PCIB_INIT_604: u32 = 0x0; // established
pub const PCIB_INIT_608: u32 = 0x0; // established
pub const PCIB_INIT_800: u32 = 0xc1013; // established
pub const PCIB_INIT_F00: u32 = 0x1000000; // established
pub const PCIB_INIT_F04: u32 = 0x800000d1; // established
pub const PCIB_INIT_F10: u32 = 0x0; // established
pub const PCIB_INIT_F14: u32 = 0xc00000d1; // established
pub const PCIB_INIT_F80: u32 = 0x81000000; // established
pub const PLPRCR_START_VALUE: u32 = 0x5040; // established
pub const SCCR_START_VALUE: u32 = 0x62000000; // established
pub const SMC1_PARAM_PAGE: u32 = 0x3e80; // established
pub const SWSR_KEY_FIRST: u32 = 0x556c; // established
pub const SWSR_KEY_SECOND: u32 = 0xaa39; // established
pub const SYPCR_FAILURE_PATH_VALUE: u32 = 0xffffff80; // established
pub const UPM_PROGRAM_WORDS: u32 = 0x40; // established
pub const VPD_BLOCK_SIZE: u32 = 0x100; // established
pub const VPD_CHAIN_START: u32 = 0xa; // established
pub const VPD_EEPROM_ADDR7: u32 = 0x52; // established
pub const VPD_EEPROM_ADDR8: u32 = 0xa4; // established
pub const VPD_TAG_END: u32 = 0xff; // established
pub const VPD_TAG_ETHERNET: u32 = 0x8; // established

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn msr_1002() {
        assert_eq!(msr::ri::get(0x1002), 1);
        assert_eq!(msr::me::get(0x1002), 1);
    }
}
