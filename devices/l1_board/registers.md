<!-- generated from devices/l1_board/registers.toml by defs-gen - do not edit, change the source -->

# board-level bit layouts and constants of the ColdFire L1 firmware

Board-level bit layouts and constants of the ColdFire L1 firmware: bit positions of the I/O
expander bytes the L1 drives, the flash command stream, the quad UART, the battery-backed clock
window and the hardware monitor start-up values. Window offsets are relative to the base of the
device's chip select. Expander addresses come in the 8-bit wire form and the 7-bit form. Each
field carries the grade of the reading; polarity grades are given separately where the firmware
only shows that a bit is written.

Unit `l1_board`, macro prefix `L1B`.

Every item has a grade: established, inferred or unknown. A grade says how the item was derived from its source; it does not claim the item was measured on silicon.

## Words

| Word | Width | Grade | Source |
|---|---|---|---|
| `CACR` | 32 | inferred |  |
| `CLOCK` | 32 | established |  |
| `CNTL1` | 32 | established |  |
| `CNTL2` | 32 | unknown |  |
| `DISPLAY` | 32 | established |  |
| `FLASH_STATUS` | 32 | established |  |
| `PIMM_VCC` | 32 | established |  |
| `ROUTER` | 32 | established |  |
| `ROUTER_DEST` | 32 | established |  |
| `RPOWER` | 32 | established |  |
| `XBRIDGE` | 32 | established |  |

### Word `CACR`

Control-register codes of the MOVEC extension word and the values the firmware writes
at start-up. The register names are the manual's. Instruction cache control is written
twice: invalidate, then enable.

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 31 | `cenb` | inferred |  |  |  |
| 24 | `cinv` | inferred |  |  |  |

### Word `CLOCK`

Battery-backed clock: control byte at the top of the window; BCD time bytes follow it.

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 7 | `write_enable` | established |  |  |  |
| 6 | `read_latch` | established |  |  |  |
| 5:4 | `century_tens` | established |  |  |  |
| 3:0 | `century_hundreds` | established |  |  |  |

### Word `CNTL1`

Processor-module control expanders, wire addresses 0x42 and 0x44 (7-bit 0x21 and 0x22).

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 7 | `soft_reset` | established |  |  | Soft-reset line: idle low, set then cleared by the soft-reset command (polarity inferred). |
| 6 | `nmi` | established |  |  | Non-maskable-interrupt line: idle high, driven low then high by the nmi command (polarity inferred). |
| 1 | `reset_b` | established |  |  |  |
| 0 | `reset_a` | established |  |  |  |

### Word `CNTL2`

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 7:3 | `strap` | unknown |  |  | Strap or configuration bits written once per release; what they select is unknown. |
| 0 | `reset_c` | established |  |  |  |

### Word `DISPLAY`

Front-panel expander: 7-bit address 0x20, low nibble input.

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 6 | `led_6` | established |  |  |  |
| 5 | `led_5` | established |  |  |  |
| 4 | `led_4` | established |  |  |  |

### Word `FLASH_STATUS`

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 6 | `toggle` | established |  |  |  |
| 5 | `timeout` | established |  |  | Completion is read by comparing two successive halfword reads. |

### Word `PIMM_VCC`

Processor-module supply-good expanders: one per module, at the wire address of the first
control expander but on its own bus segment (segment 4 for module 0, segment 5 for module 1).
Reset hold sets both bits of the field; release clears both. The mask is not applied to
the written byte as an input mask.

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 7:6 | `ok` | established |  |  |  |

### Word `ROUTER`

R-brick router expanders.

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 7:4 | `router_type` | established |  |  |  |

### Word `ROUTER_DEST`

Router packet destination word, as the L2 router and the L1 target list both build and
read it for brick-directed packets (any destination form other than the L2 form and the
node form): the rack takes 10 bits and the slot 6.

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 27:18 | `rack` | established |  |  |  |
| 17:12 | `slot` | established |  |  |  |

### Word `RPOWER`

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 3 | `router_power` | established |  |  |  |

### Word `XBRIDGE`

I-brick bridge control expander: expander index 5, 7-bit address 0x24.

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 5 | `clock_select` | established |  |  |  |
| 4 | `host_enable_b` | established |  |  |  |
| 3 | `reset_3` | established |  |  |  |
| 2 | `host_enable_a` | established |  |  |  |
| 1 | `reset_1` | established |  |  |  |
| 0 | `reset_0` | established |  |  |  |

## Constants

| Constant | Value | Grade | Source | Description |
|---|---|---|---|---|
| `BANK_RECORD_WORD0` | 0x40030 | established |  | first word of each bank's 48-byte record: record type 4, length 0x30; the same in all ten records |
| `BUS_SELECT_ADDR7` | 0x27 | established |  |  |
| `BUS_SELECT_SEG1` | 0xff | established |  |  |
| `BUS_SELECT_SEG13_LAST` | 0xf7 | established |  |  |
| `BUS_SELECT_SEG2` | 0x78 | established |  |  |
| `BUS_SELECT_SEG3` | 0xb8 | established |  |  |
| `BUS_SELECT_SEG4` | 0xd8 | established |  |  |
| `BUS_SELECT_SEG5` | 0xe8 | established |  |  |
| `BUS_SELECT_SEG6_FIRST` | 0xf0 | established |  | segments 6 to 13 are 0xf0 to 0xf7 |
| `CACR_FIRST_WRITE` | 0x1000000 | established |  | invalidate |
| `CACR_SECOND_WRITE` | 0x80000000 | established |  | enable |
| `CLOCK_CONTROL_INDEX` | 0x7f8 | established |  |  |
| `CNTL1_INPUT_MASK_CBRICK` | 0x0 | established |  |  |
| `CNTL1_INPUT_MASK_O300` | 0x20 | established |  |  |
| `CNTL1_START_VALUE` | 0x72 | established |  | applied at cold start |
| `CNTL1_WIRE` | 0x42 | established |  |  |
| `CNTL2_INPUT_MASK_CBRICK` | 0x0 | established |  |  |
| `CNTL2_INPUT_MASK_O300` | 0x2 | established |  |  |
| `CNTL2_START_VALUE` | 0x8a | established |  | applied at cold start |
| `CNTL2_WIRE` | 0x44 | established |  |  |
| `DISPLAY_CHANGE_MASK` | 0x7 | established |  | bits whose changes are reported |
| `DISPLAY_CONTROL_ADDR7` | 0x20 | established |  |  |
| `DISPLAY_INPUT_MASK` | 0xf | established |  |  |
| `EEPROM_CX_NODE_ADDR7` | 0x56 | established |  | Cx-brick node board records, one per node on bus selectors 2 to 5 (wire 0xac) |
| `EEPROM_CX_RECORD_COUNT` | 0x26 | established |  | records the Cx-brick personality registers |
| `EEPROM_CX_SPD_CAPACITY` | 0xa0 | inferred |  | length the importing note gives for each SPD record the Cx-brick personality serves; the registration table shows a capacity of 0x100 for these entries and no instruction showing 0xa0 as a served length was found, so this is unconfirmed |
| `EEPROM_DIMM_ADDR7` | 0x50 | established |  | DIMM 0 to 7 on segments 6 to 13 |
| `EEPROM_DIMM_CAPACITY` | 0x100 | established |  |  |
| `EEPROM_KIND_BOARD` | 0x1 | established |  |  |
| `EEPROM_KIND_MEMORY` | 0x2 | established |  |  |
| `EEPROM_LOGIC_ADDR7` | 0x54 | established |  | I-brick and R-brick logic board |
| `EEPROM_LOGIC_CAPACITY` | 0x200 | established |  |  |
| `EEPROM_NODE_ADDR7` | 0x52 | established |  | C-brick node board, segment 0 |
| `EEPROM_NODE_CAPACITY` | 0x200 | established |  |  |
| `EEPROM_PIMM_ADDR7` | 0x54 | established |  | PIMM 0 on segment 4, PIMM 1 on segment 5 |
| `EEPROM_PIMM_CAPACITY` | 0x200 | established |  |  |
| `EEPROM_POWER_ADDR7` | 0x52 | established |  | I-brick and R-brick power board |
| `EEPROM_POWER_CAPACITY` | 0x200 | established |  |  |
| `FLASH_CMD_AUTOSELECT` | 0x90 | established |  |  |
| `FLASH_CMD_ERASE_RESUME` | 0x30 | established |  |  |
| `FLASH_CMD_ERASE_SETUP` | 0x80 | established |  | followed by the unlock pair and sector erase |
| `FLASH_CMD_ERASE_SUSPEND` | 0xb0 | established |  |  |
| `FLASH_CMD_PROGRAM` | 0xa0 | established |  |  |
| `FLASH_CMD_RESET` | 0xf0 | established |  |  |
| `FLASH_CMD_SECTOR_ERASE` | 0x30 | established |  |  |
| `FLASH_HELPER_ERR_PROGRAM` | 0xfffffaeb | established |  | returned by the write helper on a program failure; package 1 |
| `FLASH_HELPER_ERR_RANGE` | 0xfffffaec | established |  | returned by the erase helper and the write helper when the span lies outside the flash limits block held in RAM, and by the erase helper when a sector range overlaps a second range held in the same block; package 1 |
| `FLASH_HELPER_ERR_VERIFY` | 0xfffffaea | established |  | returned by the write helper on a verify mismatch; package 1 |
| `FLASH_UNLOCK_ADDR_1` | 0xaaa | established |  |  |
| `FLASH_UNLOCK_ADDR_2` | 0x554 | established |  |  |
| `FLASH_UNLOCK_DATA_1` | 0xaa | established |  |  |
| `FLASH_UNLOCK_DATA_2` | 0x55 | established |  |  |
| `IBRICK_FAN_SPEED_ADDR7` | 0x39 | established |  |  |
| `IBRICK_PCI_BUS1_SLOT1_ADDR7` | 0x25 | established |  |  |
| `IBRICK_PCI_BUS1_SLOT2_ADDR7` | 0x26 | established |  |  |
| `IBRICK_PCI_BUS1_SLOT3_ADDR7` | 0x27 | established |  |  |
| `IBRICK_PCI_BUS1_SLOT4_ADDR7` | 0x21 | established |  |  |
| `IBRICK_PCI_BUS2_SLOT1_ADDR7` | 0x3e | established |  |  |
| `IBRICK_PCI_BUS2_SLOT2_ADDR7` | 0x3f | established |  |  |
| `IBRICK_PORT_CONTROL_ADDR7` | 0x23 | established |  |  |
| `IBRICK_POWER_ENABLE_ADDR7` | 0x38 | established |  |  |
| `IBRICK_POWER_STATUS1_ADDR7` | 0x3a | established |  |  |
| `IBRICK_POWER_STATUS2_ADDR7` | 0x3b | established |  |  |
| `IBRICK_SUPPLY_12V_BIAS_ENABLE` | 0x4 | established |  |  |
| `IBRICK_SUPPLY_12V_ENABLE` | 0x40 | established |  |  |
| `IBRICK_SUPPLY_2V5_ENABLE` | 0x8 | established |  |  |
| `IBRICK_SUPPLY_3V3_ENABLE` | 0x10 | established |  |  |
| `IBRICK_SUPPLY_48V_ENABLE` | 0x1 | established |  |  |
| `IBRICK_SUPPLY_5V_ENABLE` | 0x20 | established |  |  |
| `IBRICK_SUPPLY_M12V_ENABLE` | 0x80 | established |  |  |
| `MBAR_START_VALUE` | 0x10000001 | established |  | peripheral block base with the valid bit |
| `MONITOR_ADDR7_0` | 0x2c | established |  |  |
| `MONITOR_ADDR7_1` | 0x2d | established |  |  |
| `MONITOR_ADDR7_2` | 0x2e | established |  |  |
| `MONITOR_ADDR7_3` | 0x2f | established |  |  |
| `MONITOR_FLAG_NO_START` | 0x1 | established |  |  |
| `MONITOR_INIT_REG_40` | 0x80 | established |  | register 0x40 |
| `MONITOR_INIT_REG_43` | 0xdf | established |  | register 0x43 |
| `MONITOR_INIT_REG_44` | 0x13 | established |  | register 0x44 |
| `MONITOR_INIT_REG_47_ADDR7_0` | 0xa0 | established |  |  |
| `MONITOR_INIT_REG_47_ADDR7_1` | 0x20 | established |  |  |
| `MONITOR_INIT_REG_47_OTHER` | 0x0 | established |  |  |
| `MOVEC_CODE_CACR` | 0x2 | established |  |  |
| `MOVEC_CODE_MBAR` | 0xc0f | established |  |  |
| `MOVEC_CODE_RAMBAR` | 0xc04 | established |  |  |
| `MOVEC_CODE_VBR` | 0x801 | established |  |  |
| `MOVEC_RN_RC_OPCODE` | 0x4e7b | established |  |  |
| `PCI_EXPANSION_TYPE` | 0x25 | established |  | type character the Origin 300 PCI expansion chassis returns in general request 0x0003; the compute chassis returns 0x34 |
| `PIMM_VCC_OK_WIRE` | 0x42 | established |  |  |
| `QUART_CHANNEL0` | 0x0 | established |  | byte offset in the chip-select window |
| `QUART_CHANNEL1` | 0x8 | established |  |  |
| `QUART_CHANNEL2` | 0x10 | established |  |  |
| `QUART_CHANNEL3` | 0x18 | established |  |  |
| `QUART_FIFO_CONTROL_TRIGGER_A` | 0xf | established |  |  |
| `QUART_FIFO_CONTROL_TRIGGER_B` | 0x8f | established |  |  |
| `QUART_LINE_CONTROL_8E1` | 0x1b | established |  |  |
| `QUART_LINE_CONTROL_8N1` | 0x3 | established |  |  |
| `QUART_LINE_CONTROL_8O1` | 0xb | established |  |  |
| `QUART_MODEM_CONTROL_DTR_RTS` | 0x3 | established |  | normal use |
| `QUART_MODEM_CONTROL_LOOPBACK` | 0x10 | established |  | only when the channel loop-back flag is set |
| `QUART_REG_FIFO_CONTROL` | 0x2 | established |  | write; interrupt identification on read |
| `QUART_REG_LINE_CONTROL` | 0x3 | established |  |  |
| `QUART_REG_MODEM_CONTROL` | 0x4 | established |  |  |
| `RACK_MAX` | 0x3ff | established |  |  |
| `RACK_WIDTH_BITS` | 0xa | established |  |  |
| `RAMBAR_START_VALUE` | 0x30100001 | established |  | on-chip SRAM base with the valid bit |
| `RBRICK_PORT_BC_CONTROL_ADDR7` | 0x25 | established |  |  |
| `RBRICK_PORT_DE_CONTROL_ADDR7` | 0x26 | established |  |  |
| `RBRICK_PORT_FG_CONTROL_ADDR7` | 0x27 | established |  |  |
| `RBRICK_PORT_HA_CONTROL_ADDR7` | 0x24 | established |  |  |
| `RBRICK_POWER_CONTROL_ADDR7` | 0x39 | established |  |  |
| `RBRICK_SUPPLY_2V5_ENABLE` | 0x4 | established |  |  |
| `RBRICK_SUPPLY_48V_ENABLE` | 0x1 | established |  |  |
| `RBRICK_WIDGET_PRESENT_ADDR7` | 0x22 | established |  |  |
| `RETAINED_RECORD_BASE` | 0x30101990 | established |  | retained record in the on-chip SRAM; the loader is shared by the same shape in all five packages |
| `RETAINED_RECORD_MARKER` | 0xa1b2c3d4 | established |  | first word of the retained record |
| `RETAINED_RECORD_SIZE` | 0x4d8 | established |  |  |
| `ROUTER_CONTROL_ADDR7` | 0x23 | established |  | expander index 2 |
| `ROUTER_HOLD_VALUE` | 0x1 | established |  | written under that mask by hold |
| `ROUTER_RESET_BITS_MASK` | 0xd | established |  | bits stepped through hold and release |
| `ROUTER_START_VALUE` | 0xf1 | established |  |  |
| `TIMER1_LEVEL` | 0x5 | established |  |  |
| `TIMER1_VECTOR` | 0x1d | established |  | external level 5 |
| `UART_BAUD` | 0x4b00 | established |  |  |
| `UART_LEVEL` | 0x6 | established |  |  |
| `UART_VECTOR` | 0x42 | established |  | on-chip UART |
| `VBR_START_VALUE` | 0x30100000 | established |  | vector table at the SRAM base |
| `XBRIDGE_ADDR7` | 0x24 | established |  |  |
| `XBRIDGE_START_VALUE` | 0xd4 | established |  |  |
