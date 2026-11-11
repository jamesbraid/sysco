/*
 * MPC8XX_DECODE.h - start-up values of the PowerPC communications controller on the L2 board
 *
 * Values the L2 reset code and its second stage write: the machine state register, the
 * clock and reset registers, the serial-management channel parameter page and the
 * writes to the PCI bridge. Register offsets are from the internal memory map base.
 * The register and field names are the public Linux headers'; the firmware's own
 * reading of each field was not decoded, so the layouts of the clock registers and the
 * bridge are not given here.
 *
 * Each line carries the grade of the reading. A value is established when the image
 * writes it as stated.
 */
#ifndef MPC8XX_DECODE_H
#define MPC8XX_DECODE_H

/* Machine state register written once by the reset code: machine-check enable and
 * recoverable-interrupt set, translation off, supervisor mode. The bit positions are
 * the PowerPC architecture's. */
#define M8X_MSR_RESET_VALUE 0x00001002u /* established */
/* FIELD: MSR.ri 1:1 inferred */
#define M8X_MSR_RI_SHIFT 1
#define M8X_MSR_RI_MASK  0x1u
/* FIELD: MSR.me 12:12 inferred */
#define M8X_MSR_ME_SHIFT 12
#define M8X_MSR_ME_MASK  0x1u
/*
 * TESTVEC: 0x00001002 as MSR => ri=1, me=1
 */

/* Internal memory map base register value; every offset below is from it. */
#define M8X_IMMR_VALUE 0xfa200000u /* established */

/* Clock and reset registers written by the reset code; the multiplication factor and
 * division fields were not decoded. */
/* REG: CLK.sccr 0x280 rw32 reset=unknown established */
/* REG: CLK.plprcr 0x284 rw32 reset=unknown established */
#define M8X_SCCR_START_VALUE 0x62000000u /* established */
#define M8X_PLPRCR_START_VALUE 0x00005040u /* established */

/* Memory controller banks: the base and option register of each of the eight banks, written
 * from a 14-word table (banks 1 to 7) and, for bank 0, built and stored by the reset code. The
 * width, base, mask and timing fields of the two registers were not decoded here. */
/* REG: CS.br0 0x100 rw32 reset=unknown established */
/* REG: CS.or0 0x104 rw32 reset=unknown established */
/* REG: CS.br1 0x108 rw32 reset=unknown established */
/* REG: CS.or1 0x10c rw32 reset=unknown established */
/* REG: CS.br2 0x110 rw32 reset=unknown established */
/* REG: CS.or2 0x114 rw32 reset=unknown established */
/* REG: CS.br3 0x118 rw32 reset=unknown established */
/* REG: CS.or3 0x11c rw32 reset=unknown established */
/* REG: CS.br4 0x120 rw32 reset=unknown established */
/* REG: CS.or4 0x124 rw32 reset=unknown established */
/* REG: CS.br5 0x128 rw32 reset=unknown established */
/* REG: CS.or5 0x12c rw32 reset=unknown established */
/* REG: CS.br6 0x130 rw32 reset=unknown established */
/* REG: CS.or6 0x134 rw32 reset=unknown established */
/* REG: CS.br7 0x138 rw32 reset=unknown established */
/* REG: CS.or7 0x13c rw32 reset=unknown established */
#define M8X_BR0_START_VALUE 0xfe000001u /* established: flash window, 32-bit */
#define M8X_OR0_START_VALUE 0xff800940u /* established */
#define M8X_BR1_START_VALUE 0x00000081u /* established: DRAM bank */
#define M8X_OR1_START_VALUE 0xff000400u /* established */
#define M8X_BR2_START_VALUE 0x10000801u /* established: LCD window, 16-bit */
#define M8X_OR2_START_VALUE 0xfffe0938u /* established */
#define M8X_BR3_START_VALUE 0x00000000u /* established: bank unused */
#define M8X_OR3_START_VALUE 0x00000000u /* established */
#define M8X_BR4_START_VALUE 0xfa100401u /* established: byte-wide peripheral window, 8-bit */
#define M8X_OR4_START_VALUE 0xfffe0930u /* established */
#define M8X_BR5_START_VALUE 0x80000001u /* established: PCI memory window, 32-bit */
#define M8X_OR5_START_VALUE 0xa0000008u /* established */
#define M8X_BR6_START_VALUE 0xfa210001u /* established: PCI bridge register window, 32-bit */
#define M8X_OR6_START_VALUE 0xffff0108u /* established */
#define M8X_BR7_START_VALUE 0xfc000401u /* established: 8 MiB window, 8-bit */
#define M8X_OR7_START_VALUE 0xff800930u /* established */

/* First baud-rate generator, loaded by stage A of the console loader. Its place in the register
 * map is the public map's: the store goes through a base that sits 0x30 below the register, and
 * the same routine uses that base for the communications-processor command register and for
 * port B (read in l2.bin 1.44.0), so the position is inferred. Stage A computes the divisor
 * field as (clock word >> 4) / baud, doubled, with the enable bit or-ed in; with its clock word
 * of 0x32 and baud word 0x9600 the quotient is zero, so it stores only the enable bit. The
 * wrapper's clock word is 50,000,000, which gives the quotient 81 and the value 0x100a2. */
/* REG: BRG.brgc1 0x9f0 rw32 reset=unknown inferred */
#define M8X_BRGC1_STAGE_A_VALUE 0x00010000u /* established */
#define M8X_BRGC1_WRAPPER_VALUE 0x000100a2u /* established */

/* Register windows the wrapper assigns to its two host-controller devices, as the base address
 * argument of the device set-up call (l2.bin 1.44.0, wrapper). Both fall
 * inside the second image of the PCI memory window of bank 5. */
#define M8X_HC0_WINDOW_BASE 0xc0000000u /* established: argument for device 4 */
#define M8X_HC1_WINDOW_BASE 0xc0001000u /* established: argument for device 5 */

/* Parameter page of the first serial-management channel, programmed by an initialiser
 * that is not on the boot path. */
#define M8X_SMC1_PARAM_PAGE 0x3e80u /* established */

/* PCI bridge registers the second stage writes, and the value each receives. The bridge's
 * manual is not held, so the meaning of every field is unknown; the access width was not
 * read, so the register lines are inferred. */
/* The bridge's configuration-access pair, used by the second stage to reach the devices behind
 * it. The address register is loaded first, then the data register is read or written at the
 * data offset plus the byte lane (byte, halfword and word accesses). The address word is built
 * from the bus number, the device and function byte and the register number with its low two
 * bits cleared, and bit 0 is set only when the bus number is not 0. The widths of the bus and
 * device-and-function arguments were not read, so those two field extents are inferred from
 * the usual layout; the register number mask and the type bit follow from the code. */
/* REG: PCIB.cfg_addr 0x500 rw32 reset=unknown established */
/* REG: PCIB.cfg_data 0x504 rw32 reset=unknown established */
/* FIELD: PCI_CFG_ADDR.bus 23:16 inferred */
#define M8X_PCI_CFG_ADDR_BUS_SHIFT 16
#define M8X_PCI_CFG_ADDR_BUS_MASK  0xffu
/* FIELD: PCI_CFG_ADDR.devfn 15:8 inferred */
#define M8X_PCI_CFG_ADDR_DEVFN_SHIFT 8
#define M8X_PCI_CFG_ADDR_DEVFN_MASK  0xffu
/* FIELD: PCI_CFG_ADDR.register 7:2 established */
#define M8X_PCI_CFG_ADDR_REGISTER_SHIFT 2
#define M8X_PCI_CFG_ADDR_REGISTER_MASK  0x3fu
/* FIELD: PCI_CFG_ADDR.type1 0:0 established */
#define M8X_PCI_CFG_ADDR_TYPE1_SHIFT 0
#define M8X_PCI_CFG_ADDR_TYPE1_MASK  0x1u
/* REG: PCIB.reg_004 0x004 rw32 reset=unknown inferred */
/* REG: PCIB.reg_00c 0x00c rw32 reset=unknown inferred */
/* REG: PCIB.reg_010 0x010 rw32 reset=unknown inferred */
/* REG: PCIB.reg_030 0x030 rw32 reset=unknown inferred */
/* REG: PCIB.reg_03c 0x03c rw32 reset=unknown inferred */
/* REG: PCIB.reg_100 0x100 rw32 reset=unknown inferred */
/* REG: PCIB.reg_104 0x104 rw32 reset=unknown inferred */
/* REG: PCIB.reg_110 0x110 rw32 reset=unknown inferred */
/* REG: PCIB.reg_114 0x114 rw32 reset=unknown inferred */
/* REG: PCIB.reg_13c 0x13c rw32 reset=unknown inferred */
/* REG: PCIB.reg_140 0x140 rw32 reset=unknown inferred */
/* REG: PCIB.reg_400 0x400 rw32 reset=unknown inferred */
/* REG: PCIB.reg_404 0x404 rw32 reset=unknown inferred */
/* REG: PCIB.reg_408 0x408 rw32 reset=unknown inferred */
/* REG: PCIB.reg_600 0x600 rw32 reset=unknown inferred */
/* REG: PCIB.reg_604 0x604 rw32 reset=unknown inferred */
/* REG: PCIB.reg_608 0x608 rw32 reset=unknown inferred */
/* REG: PCIB.reg_800 0x800 rw32 reset=unknown inferred */
/* REG: PCIB.reg_f00 0xf00 rw32 reset=unknown inferred */
/* REG: PCIB.reg_f04 0xf04 rw32 reset=unknown inferred */
/* REG: PCIB.reg_f10 0xf10 rw32 reset=unknown inferred */
/* REG: PCIB.reg_f14 0xf14 rw32 reset=unknown inferred */
/* REG: PCIB.reg_f80 0xf80 rw32 reset=unknown inferred */
#define M8X_PCIB_INIT_004 0xf9000147u /* established */
#define M8X_PCIB_INIT_00C 0x00000000u /* established */
#define M8X_PCIB_INIT_010 0x01000000u /* established */
#define M8X_PCIB_INIT_030 0x00000000u /* established */
#define M8X_PCIB_INIT_03C 0x00000100u /* established */
#define M8X_PCIB_INIT_100 0x88800080u /* established */
#define M8X_PCIB_INIT_104 0x80000000u /* established */
#define M8X_PCIB_INIT_110 0x00000000u /* established */
#define M8X_PCIB_INIT_114 0x00000000u /* established */
#define M8X_PCIB_INIT_13C 0x00000000u /* established */
#define M8X_PCIB_INIT_140 0x81000000u /* established */
#define M8X_PCIB_INIT_400 0x00780008u /* established */
#define M8X_PCIB_INIT_404 0x00000000u /* established */
#define M8X_PCIB_INIT_408 0x00000000u /* established */
#define M8X_PCIB_INIT_600 0xeff0000fu /* established */
#define M8X_PCIB_INIT_604 0x00000000u /* established */
#define M8X_PCIB_INIT_608 0x00000000u /* established */
#define M8X_PCIB_INIT_800 0x000c1013u /* established */
#define M8X_PCIB_INIT_F00 0x01000000u /* established */
#define M8X_PCIB_INIT_F04 0x800000d1u /* established */
#define M8X_PCIB_INIT_F10 0x00000000u /* established */
#define M8X_PCIB_INIT_F14 0xc00000d1u /* established */
#define M8X_PCIB_INIT_F80 0x81000000u /* established */

#endif
