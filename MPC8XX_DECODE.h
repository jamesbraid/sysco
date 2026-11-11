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

/* Parameter page of the first serial-management channel, programmed by an initialiser
 * that is not on the boot path. */
#define M8X_SMC1_PARAM_PAGE 0x3e80u /* established */

/* PCI bridge registers the second stage writes, and the value each receives. The bridge's
 * manual is not held, so the meaning of every field is unknown; the access width was not
 * read, so the register lines are inferred. */
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
