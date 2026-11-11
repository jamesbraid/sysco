// generated from sysco 3bf6ffcfecb1 L1_BOARD_DECODE.h - do not edit, regenerate in sysco
//!
//! Board-level bit layouts and constants of the ColdFire L1 firmware:
//! expander bytes, flash commands, quad UART, clock window.
//!
//! Grades on each item say what is known about it: `established` is what the
//! shipped driver software does, read in full; `inferred` is a reasoned
//! reading; `unknown` is not derived. None of them is a measurement of the
//! silicon.

#![allow(dead_code)]

/// established.
pub const CNTL1_WIRE: u32 = 66;
/// established.
pub const CNTL2_WIRE: u32 = 68;
/// established: applied at cold start.
pub const CNTL1_START_VALUE: u32 = 114;
/// established: applied at cold start.
pub const CNTL2_START_VALUE: u32 = 138;
/// established.
pub const PIMM_VCC_OK_WIRE: u32 = 66;
/// established.
pub const CNTL1_INPUT_MASK_CBRICK: u32 = 0;
/// established.
pub const CNTL2_INPUT_MASK_CBRICK: u32 = 0;
/// established.
pub const CNTL1_INPUT_MASK_O300: u32 = 32;
/// established.
pub const CNTL2_INPUT_MASK_O300: u32 = 2;
/// established.
pub const BUS_SELECT_ADDR7: u32 = 39;
/// established.
pub const BUS_SELECT_SEG1: u32 = 255;
/// established.
pub const BUS_SELECT_SEG2: u32 = 120;
/// established.
pub const BUS_SELECT_SEG3: u32 = 184;
/// established.
pub const BUS_SELECT_SEG4: u32 = 216;
/// established.
pub const BUS_SELECT_SEG5: u32 = 232;
/// established: segments 6 to 13 are 0xf0 to 0xf7.
pub const BUS_SELECT_SEG6_FIRST: u32 = 240;
/// established.
pub const BUS_SELECT_SEG13_LAST: u32 = 247;
/// established.
pub const XBRIDGE_ADDR7: u32 = 36;
/// established.
pub const XBRIDGE_START_VALUE: u32 = 212;
/// established: expander index 2.
pub const ROUTER_CONTROL_ADDR7: u32 = 35;
/// established: written under that mask by hold.
pub const ROUTER_HOLD_VALUE: u32 = 1;
/// established.
pub const ROUTER_START_VALUE: u32 = 241;
/// established.
pub const DISPLAY_CONTROL_ADDR7: u32 = 32;
/// established.
pub const FLASH_UNLOCK_ADDR_1: u32 = 0xaaa;
/// established.
pub const FLASH_UNLOCK_ADDR_2: u32 = 0x554;
/// established.
pub const FLASH_UNLOCK_DATA_1: u32 = 170;
/// established.
pub const FLASH_UNLOCK_DATA_2: u32 = 85;
/// established.
pub const FLASH_CMD_RESET: u32 = 240;
/// established.
pub const FLASH_CMD_PROGRAM: u32 = 160;
/// established: followed by the unlock pair and sector erase.
pub const FLASH_CMD_ERASE_SETUP: u32 = 128;
/// established.
pub const FLASH_CMD_SECTOR_ERASE: u32 = 48;
/// established.
pub const FLASH_CMD_ERASE_SUSPEND: u32 = 176;
/// established.
pub const FLASH_CMD_ERASE_RESUME: u32 = 48;
/// established.
pub const FLASH_CMD_AUTOSELECT: u32 = 144;
/// established: external level 5.
pub const TIMER1_VECTOR: u32 = 29;
/// established.
pub const TIMER1_LEVEL: u32 = 5;
/// established: on-chip UART.
pub const UART_VECTOR: u32 = 66;
/// established.
pub const UART_LEVEL: u32 = 6;
/// established.
pub const UART_BAUD: u32 = 0x4b00;
/// established: byte offset in the chip-select window.
pub const QUART_CHANNEL0: u32 = 0;
/// established.
pub const QUART_CHANNEL1: u32 = 8;
/// established.
pub const QUART_CHANNEL2: u32 = 16;
/// established.
pub const QUART_CHANNEL3: u32 = 24;
/// established: write; interrupt identification on read.
pub const QUART_REG_FIFO_CONTROL: u32 = 2;
/// established.
pub const QUART_REG_LINE_CONTROL: u32 = 3;
/// established.
pub const QUART_REG_MODEM_CONTROL: u32 = 4;
/// established.
pub const QUART_FIFO_CONTROL_TRIGGER_A: u32 = 15;
/// established.
pub const QUART_FIFO_CONTROL_TRIGGER_B: u32 = 143;
/// established.
pub const QUART_LINE_CONTROL_8N1: u32 = 3;
/// established.
pub const QUART_LINE_CONTROL_8E1: u32 = 27;
/// established.
pub const QUART_LINE_CONTROL_8O1: u32 = 11;
/// established: normal use.
pub const QUART_MODEM_CONTROL_DTR_RTS: u32 = 3;
/// established: only when the channel loop-back flag is set.
pub const QUART_MODEM_CONTROL_LOOPBACK: u32 = 16;
/// established.
pub const CLOCK_CONTROL_INDEX: u32 = 0x7f8;
/// established: register 0x40.
pub const MONITOR_INIT_REG_40: u32 = 128;
/// established: register 0x43.
pub const MONITOR_INIT_REG_43: u32 = 223;
/// established: register 0x44.
pub const MONITOR_INIT_REG_44: u32 = 19;
/// established.
pub const MOVEC_RN_RC_OPCODE: u32 = 0x4e7b;
/// established.
pub const MOVEC_CODE_CACR: u32 = 2;
/// established.
pub const MOVEC_CODE_VBR: u32 = 0x801;
/// established.
pub const MOVEC_CODE_RAMBAR: u32 = 0xc04;
/// established.
pub const MOVEC_CODE_MBAR: u32 = 0xc0f;
/// established: on-chip SRAM base with the valid bit.
pub const RAMBAR_START_VALUE: u32 = 0x30100001;
/// established: peripheral block base with the valid bit.
pub const MBAR_START_VALUE: u32 = 0x10000001;
/// established: vector table at the SRAM base.
pub const VBR_START_VALUE: u32 = 0x30100000;
/// established: invalidate.
pub const CACR_FIRST_WRITE: u32 = 0x1000000;
/// established: enable.
pub const CACR_SECOND_WRITE: u32 = 0x80000000;
/// established.
pub const IBRICK_POWER_ENABLE_ADDR7: u32 = 56;
/// established.
pub const IBRICK_FAN_SPEED_ADDR7: u32 = 57;
/// established.
pub const IBRICK_POWER_STATUS1_ADDR7: u32 = 58;
/// established.
pub const IBRICK_POWER_STATUS2_ADDR7: u32 = 59;
/// established.
pub const IBRICK_PORT_CONTROL_ADDR7: u32 = 35;
/// established.
pub const IBRICK_PCI_BUS1_SLOT1_ADDR7: u32 = 37;
/// established.
pub const IBRICK_PCI_BUS1_SLOT2_ADDR7: u32 = 38;
/// established.
pub const IBRICK_PCI_BUS1_SLOT3_ADDR7: u32 = 39;
/// established.
pub const IBRICK_PCI_BUS1_SLOT4_ADDR7: u32 = 33;
/// established.
pub const IBRICK_PCI_BUS2_SLOT1_ADDR7: u32 = 62;
/// established.
pub const IBRICK_PCI_BUS2_SLOT2_ADDR7: u32 = 63;
/// established.
pub const RBRICK_POWER_CONTROL_ADDR7: u32 = 57;
/// established.
pub const RBRICK_WIDGET_PRESENT_ADDR7: u32 = 34;
/// established.
pub const RBRICK_PORT_HA_CONTROL_ADDR7: u32 = 36;
/// established.
pub const RBRICK_PORT_BC_CONTROL_ADDR7: u32 = 37;
/// established.
pub const RBRICK_PORT_DE_CONTROL_ADDR7: u32 = 38;
/// established.
pub const RBRICK_PORT_FG_CONTROL_ADDR7: u32 = 39;
/// established: C-brick node board, segment 0.
pub const EEPROM_NODE_ADDR7: u32 = 82;
/// established.
pub const EEPROM_NODE_CAPACITY: u32 = 0x200;
/// established: PIMM 0 on segment 4, PIMM 1 on segment 5.
pub const EEPROM_PIMM_ADDR7: u32 = 84;
/// established.
pub const EEPROM_PIMM_CAPACITY: u32 = 0x200;
/// established: DIMM 0 to 7 on segments 6 to 13.
pub const EEPROM_DIMM_ADDR7: u32 = 80;
/// established.
pub const EEPROM_DIMM_CAPACITY: u32 = 0x100;
/// established: I-brick and R-brick power board.
pub const EEPROM_POWER_ADDR7: u32 = 82;
/// established.
pub const EEPROM_POWER_CAPACITY: u32 = 0x200;
/// established: I-brick and R-brick logic board.
pub const EEPROM_LOGIC_ADDR7: u32 = 84;
/// established.
pub const EEPROM_LOGIC_CAPACITY: u32 = 0x200;
/// established.
pub const EEPROM_KIND_BOARD: u32 = 1;
/// established.
pub const EEPROM_KIND_MEMORY: u32 = 2;
/// established.
pub const MONITOR_ADDR7_0: u32 = 44;
/// established.
pub const MONITOR_ADDR7_1: u32 = 45;
/// established.
pub const MONITOR_ADDR7_2: u32 = 46;
/// established.
pub const MONITOR_ADDR7_3: u32 = 47;
/// established.
pub const MONITOR_INIT_REG_47_ADDR7_0: u32 = 160;
/// established.
pub const MONITOR_INIT_REG_47_ADDR7_1: u32 = 32;
/// established.
pub const MONITOR_INIT_REG_47_OTHER: u32 = 0;
/// established.
pub const MONITOR_FLAG_NO_START: u32 = 1;
/// established.
pub const IBRICK_SUPPLY_48V_ENABLE: u32 = 1;
/// established.
pub const IBRICK_SUPPLY_12V_BIAS_ENABLE: u32 = 4;
/// established.
pub const IBRICK_SUPPLY_2V5_ENABLE: u32 = 8;
/// established.
pub const IBRICK_SUPPLY_3V3_ENABLE: u32 = 16;
/// established.
pub const IBRICK_SUPPLY_5V_ENABLE: u32 = 32;
/// established.
pub const IBRICK_SUPPLY_12V_ENABLE: u32 = 64;
/// established.
pub const IBRICK_SUPPLY_M12V_ENABLE: u32 = 128;
/// established.
pub const RBRICK_SUPPLY_48V_ENABLE: u32 = 1;
/// established.
pub const RBRICK_SUPPLY_2V5_ENABLE: u32 = 4;
/// established.
pub const RACK_WIDTH_BITS: u32 = 10;
/// established.
pub const RACK_MAX: u32 = 0x3ff;

/// The `CNTL1` word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cntl1(u32);

impl Cntl1 {
    pub const fn new(word: u32) -> Self {
        Self(word)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Bit 0, established.
    pub const fn reset_a(self) -> bool {
        self.0 & 1 != 0
    }

    /// Bit 1, established.
    pub const fn reset_b(self) -> bool {
        (self.0 >> 1) & 1 != 0
    }

    /// Bit 6, established.
    pub const fn nmi(self) -> bool {
        (self.0 >> 6) & 1 != 0
    }

    /// Bit 7, established.
    pub const fn soft_reset(self) -> bool {
        (self.0 >> 7) & 1 != 0
    }
}

/// The `CNTL2` word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cntl2(u32);

impl Cntl2 {
    pub const fn new(word: u32) -> Self {
        Self(word)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Bit 0, established.
    pub const fn reset_c(self) -> bool {
        self.0 & 1 != 0
    }

    /// Bits 7:3, unknown.
    pub const fn strap(self) -> u32 {
        (self.0 >> 3) & 0x1f
    }
}

/// The `PIMM_VCC` word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PimmVcc(u32);

impl PimmVcc {
    pub const fn new(word: u32) -> Self {
        Self(word)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Bits 7:6, established.
    pub const fn ok(self) -> u32 {
        (self.0 >> 6) & 0x3
    }
}

/// The `XBRIDGE` word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Xbridge(u32);

impl Xbridge {
    pub const fn new(word: u32) -> Self {
        Self(word)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Bit 0, established.
    pub const fn reset_0(self) -> bool {
        self.0 & 1 != 0
    }

    /// Bit 1, established.
    pub const fn reset_1(self) -> bool {
        (self.0 >> 1) & 1 != 0
    }

    /// Bit 2, established.
    pub const fn host_enable_a(self) -> bool {
        (self.0 >> 2) & 1 != 0
    }

    /// Bit 3, established.
    pub const fn reset_3(self) -> bool {
        (self.0 >> 3) & 1 != 0
    }

    /// Bit 4, established.
    pub const fn host_enable_b(self) -> bool {
        (self.0 >> 4) & 1 != 0
    }

    /// Bit 5, established.
    pub const fn clock_select(self) -> bool {
        (self.0 >> 5) & 1 != 0
    }
}

/// The `ROUTER` word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Router(u32);

impl Router {
    pub const fn new(word: u32) -> Self {
        Self(word)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Bits 7:4, established.
    pub const fn router_type(self) -> u32 {
        (self.0 >> 4) & 0xf
    }
}

/// The `RPOWER` word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rpower(u32);

impl Rpower {
    pub const fn new(word: u32) -> Self {
        Self(word)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Bit 3, established.
    pub const fn router_power(self) -> bool {
        (self.0 >> 3) & 1 != 0
    }
}

/// The `DISPLAY` word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Display(u32);

impl Display {
    pub const fn new(word: u32) -> Self {
        Self(word)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Bit 4, established.
    pub const fn led_4(self) -> bool {
        (self.0 >> 4) & 1 != 0
    }

    /// Bit 5, established.
    pub const fn led_5(self) -> bool {
        (self.0 >> 5) & 1 != 0
    }

    /// Bit 6, established.
    pub const fn led_6(self) -> bool {
        (self.0 >> 6) & 1 != 0
    }
}

/// The `FLASH_STATUS` word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlashStatus(u32);

impl FlashStatus {
    pub const fn new(word: u32) -> Self {
        Self(word)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Bit 5, established.
    pub const fn timeout(self) -> bool {
        (self.0 >> 5) & 1 != 0
    }

    /// Bit 6, established.
    pub const fn toggle(self) -> bool {
        (self.0 >> 6) & 1 != 0
    }
}

/// The `CLOCK` word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Clock(u32);

impl Clock {
    pub const fn new(word: u32) -> Self {
        Self(word)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Bits 3:0, established.
    pub const fn century_hundreds(self) -> u32 {
        self.0 & 0xf
    }

    /// Bits 5:4, established.
    pub const fn century_tens(self) -> u32 {
        (self.0 >> 4) & 0x3
    }

    /// Bit 6, established.
    pub const fn read_latch(self) -> bool {
        (self.0 >> 6) & 1 != 0
    }

    /// Bit 7, established.
    pub const fn write_enable(self) -> bool {
        (self.0 >> 7) & 1 != 0
    }
}

/// The `CACR` word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cacr(u32);

impl Cacr {
    pub const fn new(word: u32) -> Self {
        Self(word)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Bit 24, inferred.
    pub const fn cinv(self) -> bool {
        (self.0 >> 24) & 1 != 0
    }

    /// Bit 31, inferred.
    pub const fn cenb(self) -> bool {
        (self.0 >> 31) & 1 != 0
    }
}

/// The `ROUTER_DEST` word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RouterDest(u32);

impl RouterDest {
    pub const fn new(word: u32) -> Self {
        Self(word)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Bits 17:12, established.
    pub const fn slot(self) -> u32 {
        (self.0 >> 12) & 0x3f
    }

    /// Bits 27:18, established.
    pub const fn rack(self) -> u32 {
        (self.0 >> 18) & 0x3ff
    }
}
