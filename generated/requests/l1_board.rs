// generated from sysco 2eeac980c961 L1_BOARD_DECODE.h - do not edit, regenerate in sysco
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
