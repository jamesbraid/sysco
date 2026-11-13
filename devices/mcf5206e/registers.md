<!-- generated from devices/mcf5206e/registers.toml by defs-gen - do not edit, change the source -->

# on-chip registers of the ColdFire MCF5206e as the L1 firmware uses them

On-chip registers of the ColdFire MCF5206e that the L1 firmware uses. Offsets are from the
module base register and are given as constants; the registers are not modelled as a register
block. Only registers the L1 firmware touches are listed. Widths marked inferred follow the chip
manual's layout because the firmware access size was not separately read. The reset values are
not stated by any image, so they are unknown. Established means the firmware writes or tests the
register as stated; inferred means the layout is the public manual's and was not re-read in an
image.

Unit `mcf5206e`, macro prefix `SIM`.

Every item has a grade: established, inferred or unknown. A grade says how the item was derived from its source; it does not claim the item was measured on silicon.

## Words

| Word | Width | Grade | Source |
|---|---|---|---|
| `CSMR0` | 32 | established |  |
| `CSMR2` | 32 | established |  |
| `MADR` | 8 | inferred |  |
| `MBCR` | 8 | inferred |  |
| `MBDR` | 8 | inferred |  |
| `MBSR` | 32 | established |  |
| `MFDR` | 8 | inferred |  |
| `PADAT` | 32 | established |  |
| `PAR` | 32 | established |  |
| `RSR` | 32 | established |  |
| `SYPCR` | 32 | established |  |

### Word `CSMR0`

The firmware sizes the CS0 flash by writing the mask for 2 MB, then rewrites it
for 1 MB when the two halves of the window read back the same.

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 31:16 | `bam` | established | size_1mb=15, size_2mb=31 |  |  |

### Word `CSMR2`

Chip-select 1 (battery-backed RAM and clock, 8-bit) and chip-select 2 (external RAM,
16-bit) values the board set-up writes. The RAM mask is cut from 512 KB to 128 KB when
the firmware's alias probe fails. The port-size decode of each control value is the
manual's and was not re-read in an image.

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 31:16 | `bam` | established | size_128kb=1, size_512kb=7 |  |  |

### Word `MADR`

M-Bus address register: the module's own seven-bit slave address, in bits 7 to 1. Bit 0 is
reserved. Reset value 0x00. Not the calling byte a master sends. Layout from the manual; the
firmware's own use is not re-read here.

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 7:1 | `adr` | inferred |  |  |  |

### Word `MBCR`

M-Bus control register, reset value 0x00. MEN holds the module in reset while clear.
RSTA is write-one and always reads zero. Bits 1 and 0 are reserved. Other control bits have no
effect while MEN is clear.

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 7 | `men` | inferred |  |  |  |
| 6 | `mien` | inferred |  |  |  |
| 5 | `msta` | inferred |  |  |  |
| 4 | `mtx` | inferred |  |  |  |
| 3 | `txak` | inferred |  |  |  |
| 2 | `rsta` | inferred |  |  | write one for repeated START; reads zero |

### Word `MBDR`

M-Bus data register, reset value 0x00. A write starts the next transmit byte, most
significant bit first. A read in receive mode returns the received byte and starts receipt of the
next one.

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 7:0 | `data` | inferred |  |  |  |

### Word `MBSR`

M-Bus module. The offsets are the manual's, as the firmware adds them to the module base.
Reset value 0x81 (MCF and RXAK set). Software writes clear only MAL and MIF, by writing zero;
the other bits report hardware state.

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 7 | `mcf` | inferred |  |  | byte transfer complete; clears during a transfer, sets on the ninth clock |
| 6 | `maas` | inferred |  |  | addressed as slave; an MBCR write clears it |
| 5 | `bus_busy` | established |  |  |  |
| 4 | `mal` | inferred |  |  | arbitration lost; software clears it by writing zero |
| 2 | `srw` | inferred |  |  | slave-mode direction of the completed address match |
| 1 | `mif` | inferred |  |  | interrupt flag; software clears it by writing zero |
| 0 | `rxak` | inferred |  |  | received acknowledge; one means NACK |

### Word `MFDR`

M-Bus frequency divider register, reset value 0x00. MBC5 to MBC0 select a divider
from the manual's table 13-2. The table value is a divider, not a bit rate. The errata says the
SCL frequency is the CPU frequency divided by the selected divider, and that the selection must
keep the M-Bus below 100 kHz. The rate in hertz is unknown until the controller clock is known.
Bits 7 and 6 are reserved; their readback is unknown.

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 5:0 | `mbc` | inferred | divide_28=0, divide_68=7, divide_480=19, divide_20=32, divide_2048=63 |  | selector into the manual's divider table, not the divider itself |

### Word `PADAT`

Parallel port: direction and data, used as the scan-chain interface.

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 7 | `tdi` | established |  |  |  |
| 6 | `tck` | established |  |  |  |
| 5 | `tms` | established |  |  |  |
| 4 | `trst` | established |  |  |  |
| 3 | `psi` | established |  |  |  |
| 0 | `tdo` | established |  |  |  |

### Word `PAR`

Pin assignment: bit 7 is pulsed by the timer set-up, bits 5:4 are cleared by the scan set-up.

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 7 | `timer_pulse` | established |  |  |  |
| 5:4 | `scan_sel` | established |  |  |  |

### Word `RSR`

Reset status: one bit is tested at start-up to record the reset cause.

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 5 | `cause5` | established |  |  |  |

### Word `SYPCR`

Software watchdog control: write-once, armed once by the firmware.

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 7 | `swe` | established |  |  |  |

## Constants

| Constant | Value | Grade | Source | Description |
|---|---|---|---|---|
| `CS0_START_CSAR` | 0xffe0 | established |  |  |
| `CS0_START_CSCR` | 0x1183 | established |  |  |
| `CS1_START_CSAR` | 0x2000 | established |  |  |
| `CS1_START_CSCR` | 0x1143 | established |  |  |
| `CS1_START_CSMR` | 0x0 | established |  |  |
| `CS2_START_CSAR` | 0x3000 | established |  |  |
| `CS2_START_CSCR` | 0x983 | established |  |  |
| `CS3_START_CSAR` | 0x4000 | established |  |  |
| `CS3_START_CSCR` | 0x953 | established |  |  |
| `CS3_START_CSMR` | 0x0 | established |  |  |
| `CS4_START_CSAR` | 0x5000 | established |  |  |
| `CS4_START_CSCR` | 0x95f | established |  |  |
| `CS4_START_CSMR` | 0x0 | established |  |  |
| `DMCR_START_VALUE` | 0x0 | established |  |  |
| `ICR10_START_VALUE` | 0x80 | established |  |  |
| `ICR11_RESET_VALUE` | 0x80 | inferred |  | hardware reset from the manual's chapter 8; autovector bit forced to one |
| `ICR11_START_VALUE` | 0x99 | established |  |  |
| `ICR12_START_VALUE` | 0x1a | established |  |  |
| `ICR13_START_VALUE` | 0x0 | established |  | cleared, UART 2 unused |
| `ICR14_START_VALUE` | 0x9 | established |  |  |
| `ICR15_START_VALUE` | 0xa | established |  |  |
| `ICR1_START_VALUE` | 0x84 | established |  |  |
| `ICR2_START_VALUE` | 0x88 | established |  |  |
| `ICR3_START_VALUE` | 0x80 | established |  |  |
| `ICR4_START_VALUE` | 0x90 | established |  |  |
| `ICR5_START_VALUE` | 0x80 | established |  |  |
| `ICR6_START_VALUE` | 0x98 | established |  |  |
| `ICR7_START_VALUE` | 0x9c | established |  |  |
| `ICR8_START_VALUE` | 0x1d | established |  |  |
| `ICR9_START_VALUE` | 0x94 | established |  |  |
| `IMR_START_VALUE` | 0xfffe | established |  |  |
| `MBSR_RESET_VALUE` | 0x81 | inferred |  | hardware reset from the manual's register diagram |
| `MFDR_INIT` | 0x13 | established |  | written at bring-up, 100 kHz at the inferred 48 MHz clock |
| `OFFSET_CSAR0` | 0x64 | established |  | rw16 |
| `OFFSET_CSAR1` | 0x70 | established |  | rw16 |
| `OFFSET_CSAR2` | 0x7c | established |  | rw16 |
| `OFFSET_CSAR3` | 0x88 | established |  | rw16 |
| `OFFSET_CSAR4` | 0x94 | established |  | rw16 |
| `OFFSET_CSAR5` | 0xa0 | established |  | rw16 |
| `OFFSET_CSCR0` | 0x6e | established |  | rw16 |
| `OFFSET_CSCR1` | 0x7a | established |  | rw16 |
| `OFFSET_CSCR2` | 0x86 | established |  | rw16 |
| `OFFSET_CSCR3` | 0x92 | established |  | rw16 |
| `OFFSET_CSCR4` | 0x9e | established |  | rw16 |
| `OFFSET_CSCR5` | 0xaa | established |  | rw16 |
| `OFFSET_CSMR0` | 0x68 | established |  | rw32 |
| `OFFSET_CSMR1` | 0x74 | established |  | rw32 |
| `OFFSET_CSMR2` | 0x80 | established |  | rw32 |
| `OFFSET_CSMR3` | 0x8c | established |  | rw32 |
| `OFFSET_CSMR4` | 0x98 | established |  | rw32 |
| `OFFSET_CSMR5` | 0xa4 | established |  | rw32 |
| `OFFSET_DMCR` | 0xc6 | established |  | rw16 |
| `OFFSET_ICR1` | 0x14 | established |  | rw8 |
| `OFFSET_ICR10` | 0x1d | established |  | rw8 |
| `OFFSET_ICR11` | 0x1e | established |  | rw8 |
| `OFFSET_ICR12` | 0x1f | established |  | rw8 |
| `OFFSET_ICR13` | 0x20 | established |  | rw8 |
| `OFFSET_ICR14` | 0x21 | established |  | rw8 |
| `OFFSET_ICR15` | 0x22 | established |  | rw8 |
| `OFFSET_ICR2` | 0x15 | established |  | rw8 |
| `OFFSET_ICR3` | 0x16 | established |  | rw8 |
| `OFFSET_ICR4` | 0x17 | established |  | rw8 |
| `OFFSET_ICR5` | 0x18 | established |  | rw8 |
| `OFFSET_ICR6` | 0x19 | established |  | rw8 |
| `OFFSET_ICR7` | 0x1a | established |  | rw8 |
| `OFFSET_ICR8` | 0x1b | established |  | rw8 |
| `OFFSET_ICR9` | 0x1c | established |  | rw8 |
| `OFFSET_IMR` | 0x36 | established |  | rw16 |
| `OFFSET_MADR` | 0x1e0 | inferred |  | rw8 |
| `OFFSET_MBCR` | 0x1e8 | inferred |  | rw8 |
| `OFFSET_MBDR` | 0x1f0 | inferred |  | rw8 |
| `OFFSET_MBSR` | 0x1ec | inferred |  | rw8 |
| `OFFSET_MFDR` | 0x1e4 | inferred |  | rw8 |
| `OFFSET_PADAT` | 0x1c9 | established |  | rw8 |
| `OFFSET_PADDR` | 0x1c5 | established |  | rw8 |
| `OFFSET_PAR` | 0xca | established |  | rw16 |
| `OFFSET_RSR` | 0x40 | established |  | rw8 |
| `OFFSET_SIMR` | 0x3 | established |  | rw8 |
| `OFFSET_SWIVR` | 0x42 | established |  | rw8 |
| `OFFSET_SWSR` | 0x43 | established |  | w8 |
| `OFFSET_SYPCR` | 0x41 | established |  | rw8 |
| `OFFSET_TMR1` | 0x100 | inferred |  | rw16 |
| `OFFSET_TMR2` | 0x120 | inferred |  | rw16 |
| `OFFSET_TMR_AUX_A` | 0x1b8 | inferred |  | w8 |
| `OFFSET_TMR_AUX_B` | 0x1bc | inferred |  | w8 |
| `OFFSET_TRR1` | 0x104 | inferred |  | rw16 |
| `OFFSET_TRR2` | 0x124 | inferred |  | rw16 |
| `OFFSET_UART1` | 0x140 | established |  |  |
| `PADDR_SCAN_SETUP` | 0xfe | established |  |  |
| `SIMR_START_VALUE` | 0xc0 | established |  |  |
| `SWSR_SERVICE_FIRST` | 0x55 | established |  |  |
| `SWSR_SERVICE_SECOND` | 0xaa | established |  |  |
| `TMR1_INIT` | 0x1d | established |  | value written; field meaning inferred from the manual |
| `TMR2_INIT` | 0x2b | established |  | value written; field meaning inferred |
| `TRR1_TICK_REFERENCE` | 0x752f | established |  | written to the timer 1 reference |
| `TRR2_REFERENCE` | 0x1 | established |  |  |
| `UART1_VECTOR` | 0x42 | established |  | argument to the UART 1 open, not a register offset |

## Test vectors

A vector is a value recorded from a source. An example only shows the layout.

| Target | Value | Kind | Decoded |
|---|---|---|---|
| `CSMR0` | 0xf0000 | vector | bam=15 |
| `CSMR0` | 0x1f0000 | vector | bam=31 |
| `CSMR2` | 0x10000 | vector | bam=1 |
| `CSMR2` | 0x70000 | vector | bam=7 |
| `MADR` | 0x58 | vector | adr=44 |
| `MBCR` | 0xf0 | vector | men=1, mien=1, msta=1, mtx=1, txak=0, rsta=0 |
| `MBSR` | 0x81 | vector | mcf=1, rxak=1 |
| `MFDR` | 0x13 | vector | mbc=19 |
| `SYPCR` | 0xfe | vector | swe=1 |
