// generated from defs/l1_board.toml by defs-gen - do not edit, change the source
// board-level bit layouts and constants of the ColdFire L1 firmware
#![allow(dead_code)]

/// Control-register codes of the MOVEC extension word and the values the firmware writes at start-up. The register names are the manual's. Instruction ca (inferred)
pub mod cacr {
    pub const WIDTH: u32 = 32;
    pub mod cenb { // bits 31:31, inferred
        pub const SHIFT: u32 = 31;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod cinv { // bits 24:24, inferred
        pub const SHIFT: u32 = 24;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
}
/// Battery-backed clock: control byte at the top of the window; BCD time bytes follow it. (established)
pub mod clock {
    pub const WIDTH: u32 = 32;
    pub mod write_enable { // bits 7:7, established
        pub const SHIFT: u32 = 7;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod read_latch { // bits 6:6, established
        pub const SHIFT: u32 = 6;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod century_tens { // bits 5:4, established
        pub const SHIFT: u32 = 4;
        pub const MASK: u32 = 0x3;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod century_hundreds { // bits 3:0, established
        pub const SHIFT: u32 = 0;
        pub const MASK: u32 = 0xf;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
}
/// Processor-module control expanders, wire addresses 0x42 and 0x44 (7-bit 0x21 and 0x22). (established)
pub mod cntl1 {
    pub const WIDTH: u32 = 32;
    pub mod soft_reset { // bits 7:7, established
        pub const SHIFT: u32 = 7;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod nmi { // bits 6:6, established
        pub const SHIFT: u32 = 6;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod reset_b { // bits 1:1, established
        pub const SHIFT: u32 = 1;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod reset_a { // bits 0:0, established
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
/// (unknown)
pub mod cntl2 {
    pub const WIDTH: u32 = 32;
    pub mod strap { // bits 7:3, unknown
        pub const SHIFT: u32 = 3;
        pub const MASK: u32 = 0x1f;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod reset_c { // bits 0:0, established
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
/// Front-panel expander: 7-bit address 0x20, low nibble input. (established)
pub mod display {
    pub const WIDTH: u32 = 32;
    pub mod led_6 { // bits 6:6, established
        pub const SHIFT: u32 = 6;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod led_5 { // bits 5:5, established
        pub const SHIFT: u32 = 5;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod led_4 { // bits 4:4, established
        pub const SHIFT: u32 = 4;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
}
/// (established)
pub mod flash_status {
    pub const WIDTH: u32 = 32;
    pub mod toggle { // bits 6:6, established
        pub const SHIFT: u32 = 6;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod timeout { // bits 5:5, established
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
/// Processor-module supply-good expanders: one per module, at the wire address of the first control expander but on its own bus segment (segment 4 for mo (established)
pub mod pimm_vcc {
    pub const WIDTH: u32 = 32;
    pub mod ok { // bits 7:6, established
        pub const SHIFT: u32 = 6;
        pub const MASK: u32 = 0x3;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
}
/// R-brick router expanders. (established)
pub mod router {
    pub const WIDTH: u32 = 32;
    pub mod router_type { // bits 7:4, established
        pub const SHIFT: u32 = 4;
        pub const MASK: u32 = 0xf;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
}
/// Router packet destination word, as the L2 router and the L1 target list both build and read it for brick-directed packets (any destination form other  (established)
pub mod router_dest {
    pub const WIDTH: u32 = 32;
    pub mod rack { // bits 27:18, established
        pub const SHIFT: u32 = 18;
        pub const MASK: u32 = 0x3ff;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod slot { // bits 17:12, established
        pub const SHIFT: u32 = 12;
        pub const MASK: u32 = 0x3f;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
}
/// (established)
pub mod rpower {
    pub const WIDTH: u32 = 32;
    pub mod router_power { // bits 3:3, established
        pub const SHIFT: u32 = 3;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
}
/// I-brick bridge control expander: expander index 5, 7-bit address 0x24. (established)
pub mod xbridge {
    pub const WIDTH: u32 = 32;
    pub mod clock_select { // bits 5:5, established
        pub const SHIFT: u32 = 5;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod host_enable_b { // bits 4:4, established
        pub const SHIFT: u32 = 4;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod reset_3 { // bits 3:3, established
        pub const SHIFT: u32 = 3;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod host_enable_a { // bits 2:2, established
        pub const SHIFT: u32 = 2;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod reset_1 { // bits 1:1, established
        pub const SHIFT: u32 = 1;
        pub const MASK: u32 = 0x1;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
    }
    pub mod reset_0 { // bits 0:0, established
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
pub const BUS_SELECT_ADDR7: u32 = 0x27; // established
pub const BUS_SELECT_SEG1: u32 = 0xff; // established
pub const BUS_SELECT_SEG13_LAST: u32 = 0xf7; // established
pub const BUS_SELECT_SEG2: u32 = 0x78; // established
pub const BUS_SELECT_SEG3: u32 = 0xb8; // established
pub const BUS_SELECT_SEG4: u32 = 0xd8; // established
pub const BUS_SELECT_SEG5: u32 = 0xe8; // established
pub const BUS_SELECT_SEG6_FIRST: u32 = 0xf0; // established
pub const CACR_FIRST_WRITE: u32 = 0x1000000; // established
pub const CACR_SECOND_WRITE: u32 = 0x80000000; // established
pub const CLOCK_CONTROL_INDEX: u32 = 0x7f8; // established
pub const CNTL1_INPUT_MASK_CBRICK: u32 = 0x0; // established
pub const CNTL1_INPUT_MASK_O300: u32 = 0x20; // established
pub const CNTL1_START_VALUE: u32 = 0x72; // established
pub const CNTL1_WIRE: u32 = 0x42; // established
pub const CNTL2_INPUT_MASK_CBRICK: u32 = 0x0; // established
pub const CNTL2_INPUT_MASK_O300: u32 = 0x2; // established
pub const CNTL2_START_VALUE: u32 = 0x8a; // established
pub const CNTL2_WIRE: u32 = 0x44; // established
pub const DISPLAY_CHANGE_MASK: u32 = 0x7; // established
pub const DISPLAY_CONTROL_ADDR7: u32 = 0x20; // established
pub const DISPLAY_INPUT_MASK: u32 = 0xf; // established
pub const EEPROM_DIMM_ADDR7: u32 = 0x50; // established
pub const EEPROM_DIMM_CAPACITY: u32 = 0x100; // established
pub const EEPROM_KIND_BOARD: u32 = 0x1; // established
pub const EEPROM_KIND_MEMORY: u32 = 0x2; // established
pub const EEPROM_LOGIC_ADDR7: u32 = 0x54; // established
pub const EEPROM_LOGIC_CAPACITY: u32 = 0x200; // established
pub const EEPROM_NODE_ADDR7: u32 = 0x52; // established
pub const EEPROM_NODE_CAPACITY: u32 = 0x200; // established
pub const EEPROM_PIMM_ADDR7: u32 = 0x54; // established
pub const EEPROM_PIMM_CAPACITY: u32 = 0x200; // established
pub const EEPROM_POWER_ADDR7: u32 = 0x52; // established
pub const EEPROM_POWER_CAPACITY: u32 = 0x200; // established
pub const FLASH_CMD_AUTOSELECT: u32 = 0x90; // established
pub const FLASH_CMD_ERASE_RESUME: u32 = 0x30; // established
pub const FLASH_CMD_ERASE_SETUP: u32 = 0x80; // established
pub const FLASH_CMD_ERASE_SUSPEND: u32 = 0xb0; // established
pub const FLASH_CMD_PROGRAM: u32 = 0xa0; // established
pub const FLASH_CMD_RESET: u32 = 0xf0; // established
pub const FLASH_CMD_SECTOR_ERASE: u32 = 0x30; // established
pub const FLASH_UNLOCK_ADDR_1: u32 = 0xaaa; // established
pub const FLASH_UNLOCK_ADDR_2: u32 = 0x554; // established
pub const FLASH_UNLOCK_DATA_1: u32 = 0xaa; // established
pub const FLASH_UNLOCK_DATA_2: u32 = 0x55; // established
pub const IBRICK_FAN_SPEED_ADDR7: u32 = 0x39; // established
pub const IBRICK_PCI_BUS1_SLOT1_ADDR7: u32 = 0x25; // established
pub const IBRICK_PCI_BUS1_SLOT2_ADDR7: u32 = 0x26; // established
pub const IBRICK_PCI_BUS1_SLOT3_ADDR7: u32 = 0x27; // established
pub const IBRICK_PCI_BUS1_SLOT4_ADDR7: u32 = 0x21; // established
pub const IBRICK_PCI_BUS2_SLOT1_ADDR7: u32 = 0x3e; // established
pub const IBRICK_PCI_BUS2_SLOT2_ADDR7: u32 = 0x3f; // established
pub const IBRICK_PORT_CONTROL_ADDR7: u32 = 0x23; // established
pub const IBRICK_POWER_ENABLE_ADDR7: u32 = 0x38; // established
pub const IBRICK_POWER_STATUS1_ADDR7: u32 = 0x3a; // established
pub const IBRICK_POWER_STATUS2_ADDR7: u32 = 0x3b; // established
pub const IBRICK_SUPPLY_12V_BIAS_ENABLE: u32 = 0x4; // established
pub const IBRICK_SUPPLY_12V_ENABLE: u32 = 0x40; // established
pub const IBRICK_SUPPLY_2V5_ENABLE: u32 = 0x8; // established
pub const IBRICK_SUPPLY_3V3_ENABLE: u32 = 0x10; // established
pub const IBRICK_SUPPLY_48V_ENABLE: u32 = 0x1; // established
pub const IBRICK_SUPPLY_5V_ENABLE: u32 = 0x20; // established
pub const IBRICK_SUPPLY_M12V_ENABLE: u32 = 0x80; // established
pub const MBAR_START_VALUE: u32 = 0x10000001; // established
pub const MONITOR_ADDR7_0: u32 = 0x2c; // established
pub const MONITOR_ADDR7_1: u32 = 0x2d; // established
pub const MONITOR_ADDR7_2: u32 = 0x2e; // established
pub const MONITOR_ADDR7_3: u32 = 0x2f; // established
pub const MONITOR_FLAG_NO_START: u32 = 0x1; // established
pub const MONITOR_INIT_REG_40: u32 = 0x80; // established
pub const MONITOR_INIT_REG_43: u32 = 0xdf; // established
pub const MONITOR_INIT_REG_44: u32 = 0x13; // established
pub const MONITOR_INIT_REG_47_ADDR7_0: u32 = 0xa0; // established
pub const MONITOR_INIT_REG_47_ADDR7_1: u32 = 0x20; // established
pub const MONITOR_INIT_REG_47_OTHER: u32 = 0x0; // established
pub const MOVEC_CODE_CACR: u32 = 0x2; // established
pub const MOVEC_CODE_MBAR: u32 = 0xc0f; // established
pub const MOVEC_CODE_RAMBAR: u32 = 0xc04; // established
pub const MOVEC_CODE_VBR: u32 = 0x801; // established
pub const MOVEC_RN_RC_OPCODE: u32 = 0x4e7b; // established
pub const PIMM_VCC_OK_WIRE: u32 = 0x42; // established
pub const QUART_CHANNEL0: u32 = 0x0; // established
pub const QUART_CHANNEL1: u32 = 0x8; // established
pub const QUART_CHANNEL2: u32 = 0x10; // established
pub const QUART_CHANNEL3: u32 = 0x18; // established
pub const QUART_FIFO_CONTROL_TRIGGER_A: u32 = 0xf; // established
pub const QUART_FIFO_CONTROL_TRIGGER_B: u32 = 0x8f; // established
pub const QUART_LINE_CONTROL_8E1: u32 = 0x1b; // established
pub const QUART_LINE_CONTROL_8N1: u32 = 0x3; // established
pub const QUART_LINE_CONTROL_8O1: u32 = 0xb; // established
pub const QUART_MODEM_CONTROL_DTR_RTS: u32 = 0x3; // established
pub const QUART_MODEM_CONTROL_LOOPBACK: u32 = 0x10; // established
pub const QUART_REG_FIFO_CONTROL: u32 = 0x2; // established
pub const QUART_REG_LINE_CONTROL: u32 = 0x3; // established
pub const QUART_REG_MODEM_CONTROL: u32 = 0x4; // established
pub const RACK_MAX: u32 = 0x3ff; // established
pub const RACK_WIDTH_BITS: u32 = 0xa; // established
pub const RAMBAR_START_VALUE: u32 = 0x30100001; // established
pub const RBRICK_PORT_BC_CONTROL_ADDR7: u32 = 0x25; // established
pub const RBRICK_PORT_DE_CONTROL_ADDR7: u32 = 0x26; // established
pub const RBRICK_PORT_FG_CONTROL_ADDR7: u32 = 0x27; // established
pub const RBRICK_PORT_HA_CONTROL_ADDR7: u32 = 0x24; // established
pub const RBRICK_POWER_CONTROL_ADDR7: u32 = 0x39; // established
pub const RBRICK_SUPPLY_2V5_ENABLE: u32 = 0x4; // established
pub const RBRICK_SUPPLY_48V_ENABLE: u32 = 0x1; // established
pub const RBRICK_WIDGET_PRESENT_ADDR7: u32 = 0x22; // established
pub const ROUTER_CONTROL_ADDR7: u32 = 0x23; // established
pub const ROUTER_HOLD_VALUE: u32 = 0x1; // established
pub const ROUTER_RESET_BITS_MASK: u32 = 0xd; // established
pub const ROUTER_START_VALUE: u32 = 0xf1; // established
pub const TIMER1_LEVEL: u32 = 0x5; // established
pub const TIMER1_VECTOR: u32 = 0x1d; // established
pub const UART_BAUD: u32 = 0x4b00; // established
pub const UART_LEVEL: u32 = 0x6; // established
pub const UART_VECTOR: u32 = 0x42; // established
pub const VBR_START_VALUE: u32 = 0x30100000; // established
pub const XBRIDGE_ADDR7: u32 = 0x24; // established
pub const XBRIDGE_START_VALUE: u32 = 0xd4; // established
