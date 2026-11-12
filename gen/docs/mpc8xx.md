# start-up values of the PowerPC communications controller on the L2 board

<!-- generated from defs/mpc8xx.toml by defs-gen - do not edit, change the source -->

Start-up values of the PowerPC communications controller on the L2 board: the values the L2
reset code and its second stage write to the machine state register, the clock and reset
registers, the serial-management channel parameter page and the PCI bridge. Register offsets are
from the internal memory map base. The register and field names are the public Linux headers';
the firmware's own reading of each field was not decoded, so the layouts of the clock registers
and the bridge are not given. A value is established when the image writes it as stated.

Unit `mpc8xx`, macro prefix `M8X`.

Every item has a grade: established, inferred or unknown. A grade says how the item was derived from its source; it does not claim the item was measured on silicon.

## Address spaces

| Space | Offset unit | Description |
|---|---|---|
| bytedev | byte |  |
| mpc8xx_brg | byte |  |
| mpc8xx_clk | byte |  |
| mpc8xx_cs | byte |  |
| mpc8xx_pcib | byte |  |
| mpc8xx_siu | byte |  |
| mpc8xx_upm | byte |  |

## Registers

| Register | Offset | Access | Width | Count | Space | Reset | Grade | Source |
|---|---|---|---|---|---|---|---|---|
| `reg0` | 0x0 | w | 8 |  | bytedev | unknown | established |  |
| `reg1` | 0x1 | w | 8 |  | bytedev | unknown | established |  |
| `reg3` | 0x3 | w | 8 |  | bytedev | unknown | established |  |
| `brgc1` | 0x9f0 | rw | 32 |  | mpc8xx_brg | unknown | inferred |  |
| `sccr` | 0x280 | rw | 32 |  | mpc8xx_clk | unknown | established |  |
| `plprcr` | 0x284 | rw | 32 |  | mpc8xx_clk | unknown | established |  |
| `br0` | 0x100 | rw | 32 |  | mpc8xx_cs | unknown | established |  |
| `or0` | 0x104 | rw | 32 |  | mpc8xx_cs | unknown | established |  |
| `br1` | 0x108 | rw | 32 |  | mpc8xx_cs | unknown | established |  |
| `or1` | 0x10c | rw | 32 |  | mpc8xx_cs | unknown | established |  |
| `br2` | 0x110 | rw | 32 |  | mpc8xx_cs | unknown | established |  |
| `or2` | 0x114 | rw | 32 |  | mpc8xx_cs | unknown | established |  |
| `br3` | 0x118 | rw | 32 |  | mpc8xx_cs | unknown | established |  |
| `or3` | 0x11c | rw | 32 |  | mpc8xx_cs | unknown | established |  |
| `br4` | 0x120 | rw | 32 |  | mpc8xx_cs | unknown | established |  |
| `or4` | 0x124 | rw | 32 |  | mpc8xx_cs | unknown | established |  |
| `br5` | 0x128 | rw | 32 |  | mpc8xx_cs | unknown | established |  |
| `or5` | 0x12c | rw | 32 |  | mpc8xx_cs | unknown | established |  |
| `br6` | 0x130 | rw | 32 |  | mpc8xx_cs | unknown | established |  |
| `or6` | 0x134 | rw | 32 |  | mpc8xx_cs | unknown | established |  |
| `br7` | 0x138 | rw | 32 |  | mpc8xx_cs | unknown | established |  |
| `or7` | 0x13c | rw | 32 |  | mpc8xx_cs | unknown | established |  |
| `reg_004` | 0x4 | rw | 32 |  | mpc8xx_pcib | unknown | inferred |  |
| `reg_00c` | 0xc | rw | 32 |  | mpc8xx_pcib | unknown | inferred |  |
| `reg_010` | 0x10 | rw | 32 |  | mpc8xx_pcib | unknown | inferred |  |
| `reg_030` | 0x30 | rw | 32 |  | mpc8xx_pcib | unknown | inferred |  |
| `reg_03c` | 0x3c | rw | 32 |  | mpc8xx_pcib | unknown | inferred |  |
| `reg_100` | 0x100 | rw | 32 |  | mpc8xx_pcib | unknown | inferred |  |
| `reg_104` | 0x104 | rw | 32 |  | mpc8xx_pcib | unknown | inferred |  |
| `reg_110` | 0x110 | rw | 32 |  | mpc8xx_pcib | unknown | inferred |  |
| `reg_114` | 0x114 | rw | 32 |  | mpc8xx_pcib | unknown | inferred |  |
| `reg_13c` | 0x13c | rw | 32 |  | mpc8xx_pcib | unknown | inferred |  |
| `reg_140` | 0x140 | rw | 32 |  | mpc8xx_pcib | unknown | inferred |  |
| `reg_400` | 0x400 | rw | 32 |  | mpc8xx_pcib | unknown | inferred |  |
| `reg_404` | 0x404 | rw | 32 |  | mpc8xx_pcib | unknown | inferred |  |
| `reg_408` | 0x408 | rw | 32 |  | mpc8xx_pcib | unknown | inferred |  |
| `cfg_addr` | 0x500 | rw | 32 |  | mpc8xx_pcib | unknown | established |  |
| `cfg_data` | 0x504 | rw | 32 |  | mpc8xx_pcib | unknown | established |  |
| `reg_600` | 0x600 | rw | 32 |  | mpc8xx_pcib | unknown | inferred |  |
| `reg_604` | 0x604 | rw | 32 |  | mpc8xx_pcib | unknown | inferred |  |
| `reg_608` | 0x608 | rw | 32 |  | mpc8xx_pcib | unknown | inferred |  |
| `reg_800` | 0x800 | rw | 32 |  | mpc8xx_pcib | unknown | inferred |  |
| `reg_f00` | 0xf00 | rw | 32 |  | mpc8xx_pcib | unknown | inferred |  |
| `reg_f04` | 0xf04 | rw | 32 |  | mpc8xx_pcib | unknown | inferred |  |
| `reg_f10` | 0xf10 | rw | 32 |  | mpc8xx_pcib | unknown | inferred |  |
| `reg_f14` | 0xf14 | rw | 32 |  | mpc8xx_pcib | unknown | inferred |  |
| `reg_f80` | 0xf80 | rw | 32 |  | mpc8xx_pcib | unknown | inferred |  |
| `sypcr` | 0x4 | rw | 32 |  | mpc8xx_siu | unknown | established |  |
| `swsr` | 0xe | rw | 16 |  | mpc8xx_siu | unknown | established |  |
| `mar` | 0x164 | rw | 32 |  | mpc8xx_upm | unknown | established |  |
| `mcr` | 0x168 | rw | 32 |  | mpc8xx_upm | unknown | established |  |
| `mamr` | 0x170 | rw | 32 |  | mpc8xx_upm | unknown | established |  |
| `mptpr` | 0x17a | rw | 16 |  | mpc8xx_upm | unknown | established |  |
| `mdr` | 0x17c | rw | 32 |  | mpc8xx_upm | unknown | established |  |

### Register `reg0`

Byte-wide peripheral on the fourth chip select. Established: the window decode above, the
8-bit port, and that the boot code only writes it, in the order given. What the device is
stays unknown, so the three registers are named by offset. Whether the kernel or any
program uses the part after mapping the first 4 KiB is not settled.

### Register `reg1`

### Register `reg3`

### Register `brgc1`

First baud-rate generator, loaded by stage A of the console loader. Its place in the register
map is the public map's: the store goes through a base that sits 0x30 below the register, and
the same routine uses that base for the communications-processor command register and for
port B (read in l2.bin 1.44.0), so the position is inferred. Stage A computes the divisor
field as (clock word >> 4) / baud, doubled, with the enable bit or-ed in; with its clock word
of 0x32 and baud word 0x9600 the quotient is zero, so it stores only the enable bit. The
wrapper's clock word is 50,000,000, which gives the quotient 81 and the value 0x100a2.

### Register `sccr`

Clock and reset registers written by the reset code; the multiplication factor and
division fields were not decoded.

### Register `plprcr`

### Register `br0`

Memory controller banks: the base and option register of each of the eight banks, written
from a 14-word table (banks 1 to 7) and, for bank 0, built and stored by the reset code. The
width, base, mask and timing fields of the two registers were not decoded here.

### Register `or0`

### Register `br1`

### Register `or1`

### Register `br2`

### Register `or2`

### Register `br3`

### Register `or3`

### Register `br4`

### Register `or4`

### Register `br5`

### Register `or5`

### Register `br6`

### Register `or6`

### Register `br7`

### Register `or7`

### Register `reg_004`

### Register `reg_00c`

### Register `reg_010`

### Register `reg_030`

### Register `reg_03c`

### Register `reg_100`

### Register `reg_104`

### Register `reg_110`

### Register `reg_114`

### Register `reg_13c`

### Register `reg_140`

### Register `reg_400`

### Register `reg_404`

### Register `reg_408`

### Register `cfg_addr`

The bridge's configuration-access pair, used by the second stage to reach the devices behind
it. The address register is loaded first, then the data register is read or written at the
data offset plus the byte lane (byte, halfword and word accesses). The address word is built
from the bus number, the device and function byte and the register number with its low two
bits cleared, and bit 0 is set only when the bus number is not 0. The widths of the bus and
device-and-function arguments were not read, so those two field extents are inferred from
the usual layout; the register number mask and the type bit follow from the code.

### Register `cfg_data`

### Register `reg_600`

### Register `reg_604`

### Register `reg_608`

### Register `reg_800`

### Register `reg_f00`

### Register `reg_f04`

### Register `reg_f10`

### Register `reg_f14`

### Register `reg_f80`

### Register `sypcr`

System protection control and software service registers. The boot code stores one 32-bit
word to the control register only on its failure path (image check failed, or the second
stage returned); a normal boot never writes it. The value keeps the high bits set and
clears the low seven. Both widths are those of the stores.

### Register `swsr`

### Register `mar`

Memory controller timing and the user-programmable machine A. The second stage writes the
periodic timer prescaler and the machine mode register, loads 64 array words through the
data register (each followed by a store of its index to the command register), then issues
run commands: one at array address 0x2d, eight at 0x30, then the address register is set
and one run at 0x13 is issued. The meaning of the machine mode fields was not decoded.

### Register `mcr`

### Register `mamr`

### Register `mptpr`

### Register `mdr`

## Words

| Word | Width | Grade | Source |
|---|---|---|---|
| `MSR` | 32 | inferred |  |
| `PCI_CFG_ADDR` | 32 | inferred |  |

### Word `MSR`

Machine state register written once by the reset code: machine-check enable and
recoverable-interrupt set, translation off, supervisor mode. The bit positions are
the PowerPC architecture's.

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 12 | `me` | inferred |  |  |  |
| 1 | `ri` | inferred |  |  |  |

### Word `PCI_CFG_ADDR`

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 23:16 | `bus` | inferred |  |  |  |
| 15:8 | `devfn` | inferred |  |  |  |
| 7:2 | `register` | established |  |  |  |
| 0 | `type1` | established |  |  |  |

## Constants

| Constant | Value | Grade | Source | Description |
|---|---|---|---|---|
| `BR0_START_VALUE` | 0xfe000001 | established |  | flash window, 32-bit |
| `BR1_START_VALUE` | 0x81 | established |  | DRAM bank |
| `BR2_START_VALUE` | 0x10000801 | established |  | LCD window, 16-bit |
| `BR3_START_VALUE` | 0x0 | established |  | bank unused |
| `BR4_START_VALUE` | 0xfa100401 | established |  | byte-wide peripheral window, 8-bit |
| `BR5_START_VALUE` | 0x80000001 | established |  | PCI memory window, 32-bit |
| `BR6_START_VALUE` | 0xfa210001 | established |  | PCI bridge register window, 32-bit |
| `BR7_START_VALUE` | 0xfc000401 | established |  | 8 MiB window, 8-bit |
| `BRGC1_STAGE_A_VALUE` | 0x10000 | established |  |  |
| `BRGC1_WRAPPER_VALUE` | 0x100a2 | established |  |  |
| `BYTEDEV_MAPPED_SIZE` | 0x1000 | established |  | size the kernel maps; the chip select decodes 128 KiB |
| `BYTEDEV_REG0_FIRST` | 0x0 | established |  |  |
| `BYTEDEV_REG0_SECOND` | 0x80 | established |  | after a 500 ms wait |
| `BYTEDEV_REG1_FIRST` | 0xe | established |  |  |
| `BYTEDEV_REG1_LAST` | 0xe | established |  | after a second 500 ms wait |
| `BYTEDEV_REG1_SECOND` | 0xf | established |  |  |
| `BYTEDEV_REG3_VALUE` | 0x18 | established |  |  |
| `FLASH_CMD_AUTOSELECT` | 0x900090 | established |  |  |
| `FLASH_CMD_ERASE_SETUP` | 0x800080 | established |  | followed by the unlock pair |
| `FLASH_CMD_PROGRAM` | 0xa000a0 | established |  |  |
| `FLASH_CMD_RESET` | 0xf000f0 | established |  |  |
| `FLASH_CMD_SECTOR_ERASE` | 0x300030 | established |  | written at the sector address |
| `FLASH_STATUS_ERROR` | 0x200020 | established |  | one bit per device |
| `FLASH_STATUS_TOGGLE` | 0x400040 | established |  | one bit per device |
| `FLASH_UNLOCK_DATA_1` | 0xaa00aa | established |  |  |
| `FLASH_UNLOCK_DATA_2` | 0x550055 | established |  |  |
| `FLASH_UNLOCK_OFFSET_1` | 0x1554 | established |  |  |
| `FLASH_UNLOCK_OFFSET_2` | 0xaa8 | established |  |  |
| `HC0_WINDOW_BASE` | 0xc0000000 | established |  | argument for device 4 |
| `HC1_WINDOW_BASE` | 0xc0001000 | established |  | argument for device 5 |
| `IMMR_VALUE` | 0xfa200000 | established |  |  |
| `MAMR_START_VALUE` | 0x18804000 | established |  |  |
| `MAR_START_VALUE` | 0x88 | established |  |  |
| `MCR_RUN_FIRST` | 0x8000202d | established |  | one store |
| `MCR_RUN_LAST` | 0x80002013 | established |  | one store, after the address register |
| `MCR_RUN_SECOND` | 0x80002030 | established |  | eight stores |
| `MPTPR_START_VALUE` | 0x200 | established |  |  |
| `MSR_RESET_VALUE` | 0x1002 | established |  |  |
| `OR0_START_VALUE` | 0xff800940 | established |  |  |
| `OR1_START_VALUE` | 0xff000400 | established |  |  |
| `OR2_START_VALUE` | 0xfffe0938 | established |  |  |
| `OR3_START_VALUE` | 0x0 | established |  |  |
| `OR4_START_VALUE` | 0xfffe0930 | established |  |  |
| `OR5_START_VALUE` | 0xa0000008 | established |  |  |
| `OR6_START_VALUE` | 0xffff0108 | established |  |  |
| `OR7_START_VALUE` | 0xff800930 | established |  |  |
| `PCIB_INIT_004` | 0xf9000147 | established |  |  |
| `PCIB_INIT_00C` | 0x0 | established |  |  |
| `PCIB_INIT_010` | 0x1000000 | established |  |  |
| `PCIB_INIT_030` | 0x0 | established |  |  |
| `PCIB_INIT_03C` | 0x100 | established |  |  |
| `PCIB_INIT_100` | 0x88800080 | established |  |  |
| `PCIB_INIT_104` | 0x80000000 | established |  |  |
| `PCIB_INIT_110` | 0x0 | established |  |  |
| `PCIB_INIT_114` | 0x0 | established |  |  |
| `PCIB_INIT_13C` | 0x0 | established |  |  |
| `PCIB_INIT_140` | 0x81000000 | established |  |  |
| `PCIB_INIT_400` | 0x780008 | established |  |  |
| `PCIB_INIT_404` | 0x0 | established |  |  |
| `PCIB_INIT_408` | 0x0 | established |  |  |
| `PCIB_INIT_600` | 0xeff0000f | established |  |  |
| `PCIB_INIT_604` | 0x0 | established |  |  |
| `PCIB_INIT_608` | 0x0 | established |  |  |
| `PCIB_INIT_800` | 0xc1013 | established |  |  |
| `PCIB_INIT_F00` | 0x1000000 | established |  |  |
| `PCIB_INIT_F04` | 0x800000d1 | established |  |  |
| `PCIB_INIT_F10` | 0x0 | established |  |  |
| `PCIB_INIT_F14` | 0xc00000d1 | established |  |  |
| `PCIB_INIT_F80` | 0x81000000 | established |  |  |
| `PLPRCR_START_VALUE` | 0x5040 | established |  |  |
| `SCCR_START_VALUE` | 0x62000000 | established |  |  |
| `SMC1_PARAM_PAGE` | 0x3e80 | established |  |  |
| `SWSR_KEY_FIRST` | 0x556c | established |  |  |
| `SWSR_KEY_SECOND` | 0xaa39 | established |  |  |
| `SYPCR_FAILURE_PATH_VALUE` | 0xffffff80 | established |  | failure path only |
| `UPM_PROGRAM_WORDS` | 0x40 | established |  |  |
| `VPD_BLOCK_SIZE` | 0x100 | established |  |  |
| `VPD_CHAIN_START` | 0xa | established |  |  |
| `VPD_EEPROM_ADDR7` | 0x52 | established |  |  |
| `VPD_EEPROM_ADDR8` | 0xa4 | established |  |  |
| `VPD_TAG_END` | 0xff | established |  |  |
| `VPD_TAG_ETHERNET` | 0x8 | established |  |  |

## Test vectors

A vector is a value recorded from a source. An example only shows the layout.

| Target | Value | Kind | Decoded |
|---|---|---|---|
| `MSR` | 0x1002 | vector | ri=1, me=1 |
