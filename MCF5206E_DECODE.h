/*
 * MCF5206E_DECODE.h - on-chip registers of the ColdFire MCF5206e as the L1 firmware uses them
 *
 * Offsets are from the module base register and are given as constants;
 * the registers are not modelled as a register block here. Only registers the L1 firmware
 * touches are listed; widths marked inferred follow the chip manual's layout
 * because the firmware access size was not separately read. The reset values
 * are not stated by any image, so they are unknown here.
 *
 * Grades follow the board-contract notes: established means the firmware
 * writes or tests the register as stated, inferred means the layout is the
 * public manual's and was not re-read in an image.
 */
#ifndef MCF5206E_DECODE_H
#define MCF5206E_DECODE_H

/* Reset status: one bit is tested at start-up to record the reset cause. */
#define SIM_OFFSET_RSR 0x40u /* rw8, established */
/* FIELD: RSR.cause5 5:5 established */
#define SIM_RSR_CAUSE5_SHIFT 5
#define SIM_RSR_CAUSE5_MASK  0x1u

/* Software watchdog control: write-once, armed once by the firmware. */
#define SIM_OFFSET_SYPCR 0x41u /* rw8, established */
/* FIELD: SYPCR.swe 7:7 established */
#define SIM_SYPCR_SWE_SHIFT 7
#define SIM_SYPCR_SWE_MASK  0x1u
/* The firmware arms the watchdog with the value 0xfe. */
/*
 * TESTVEC: 0xfe as SYPCR => swe=1
 */

#define SIM_OFFSET_SWIVR 0x42u /* rw8, established */

/* Watchdog service: the pair 0x55 then 0xaa is written to restart the count. */
#define SIM_OFFSET_SWSR 0x43u /* w8, established */
#define SIM_SWSR_SERVICE_FIRST  0x55u /* established */
#define SIM_SWSR_SERVICE_SECOND 0xaau /* established */

#define SIM_OFFSET_CSAR0 0x64u /* rw16, established */
#define SIM_OFFSET_CSMR0 0x68u /* rw32, established */
#define SIM_OFFSET_CSCR0 0x6eu /* rw16, established */
/* The firmware sizes the CS0 flash by writing the mask for 2 MB, then rewrites it
 * for 1 MB when the two halves of the window read back the same. */
/* FIELD: CSMR0.bam 31:16 established */
#define SIM_CSMR0_BAM_SHIFT 16
#define SIM_CSMR0_BAM_MASK  0xffffu
/* ENUM: CSMR0.bam 0x000f size_1mb established */
#define SIM_CSMR0_BAM_SIZE_1MB 0x000f
/* ENUM: CSMR0.bam 0x001f size_2mb established */
#define SIM_CSMR0_BAM_SIZE_2MB 0x001f
/*
 * TESTVEC: 0x001f0000 as CSMR0 => bam=0x1f
 */
/*
 * TESTVEC: 0x000f0000 as CSMR0 => bam=0xf
 */

#define SIM_OFFSET_CSAR1 0x70u /* rw16, established */
#define SIM_OFFSET_CSMR1 0x74u /* rw32, established */
#define SIM_OFFSET_CSCR1 0x7au /* rw16, established */
#define SIM_OFFSET_CSAR2 0x7cu /* rw16, established */
#define SIM_OFFSET_CSMR2 0x80u /* rw32, established */
#define SIM_OFFSET_CSCR2 0x86u /* rw16, established */
#define SIM_OFFSET_CSAR3 0x88u /* rw16, established */
#define SIM_OFFSET_CSMR3 0x8cu /* rw32, established */
#define SIM_OFFSET_CSCR3 0x92u /* rw16, established */
#define SIM_OFFSET_CSAR4 0x94u /* rw16, established */
#define SIM_OFFSET_CSMR4 0x98u /* rw32, established */
#define SIM_OFFSET_CSCR4 0x9eu /* rw16, established */
#define SIM_OFFSET_CSAR5 0xa0u /* rw16, established */
#define SIM_OFFSET_CSMR5 0xa4u /* rw32, established */
#define SIM_OFFSET_CSCR5 0xaau /* rw16, established */

/* Pin assignment: bit 7 is pulsed by the timer set-up, bits 5:4 are cleared by the scan set-up. */
#define SIM_OFFSET_PAR 0xcau /* rw16, established */
/* FIELD: PAR.timer_pulse 7:7 established */
#define SIM_PAR_TIMER_PULSE_SHIFT 7
#define SIM_PAR_TIMER_PULSE_MASK  0x1u
/* FIELD: PAR.scan_sel 5:4 established */
#define SIM_PAR_SCAN_SEL_SHIFT 4
#define SIM_PAR_SCAN_SEL_MASK  0x3u

/* Timer 1 and timer 2: mode and reference registers, written in that order. */
#define SIM_OFFSET_TMR1 0x100u /* rw16, inferred */
#define SIM_OFFSET_TRR1 0x104u /* rw16, inferred */
#define SIM_OFFSET_TMR2 0x120u /* rw16, inferred */
#define SIM_OFFSET_TRR2 0x124u /* rw16, inferred */
#define SIM_TRR1_TICK_REFERENCE 0x752fu /* established: written to the timer 1 reference */
#define SIM_TMR1_INIT 0x1du /* established: value written; field meaning inferred from the manual */
#define SIM_TRR2_REFERENCE 1u /* established */
#define SIM_TMR2_INIT 0x2bu /* established: value written; field meaning inferred */

/* Byte registers written to 1 by the timer set-up; the firmware never reads them back. */
#define SIM_OFFSET_TMR_AUX_A 0x1b8u /* w8, inferred */
#define SIM_OFFSET_TMR_AUX_B 0x1bcu /* w8, inferred */

/* On-chip UART: the opened block; the second block (0x180) is never opened. */
#define SIM_OFFSET_UART1 0x140u /* established */

/* Parallel port: direction and data, used as the scan-chain interface. */
#define SIM_OFFSET_PADDR 0x1c5u /* rw8, established */
#define SIM_OFFSET_PADAT 0x1c9u /* rw8, established */
/* FIELD: PADAT.tdo 0:0 established */
#define SIM_PADAT_TDO_SHIFT 0
#define SIM_PADAT_TDO_MASK  0x1u
/* FIELD: PADAT.psi 3:3 established */
#define SIM_PADAT_PSI_SHIFT 3
#define SIM_PADAT_PSI_MASK  0x1u
/* FIELD: PADAT.trst 4:4 established */
#define SIM_PADAT_TRST_SHIFT 4
#define SIM_PADAT_TRST_MASK  0x1u
/* FIELD: PADAT.tms 5:5 established */
#define SIM_PADAT_TMS_SHIFT 5
#define SIM_PADAT_TMS_MASK  0x1u
/* FIELD: PADAT.tck 6:6 established */
#define SIM_PADAT_TCK_SHIFT 6
#define SIM_PADAT_TCK_MASK  0x1u
/* FIELD: PADAT.tdi 7:7 established */
#define SIM_PADAT_TDI_SHIFT 7
#define SIM_PADAT_TDI_MASK  0x1u
/* Scan set-up writes the direction byte 0xfe: bit 0 input, bits 7:1 outputs. */
#define SIM_PADDR_SCAN_SETUP 0xfeu /* established */

/* M-Bus module. The offsets are the manual's, as the firmware adds them to the module base. */
#define SIM_OFFSET_MADR 0x1e0u /* rw8, inferred */
#define SIM_OFFSET_MFDR 0x1e4u /* rw8, inferred */
#define SIM_OFFSET_MBCR 0x1e8u /* rw8, inferred */
#define SIM_OFFSET_MBSR 0x1ecu /* rw8, inferred */
/* FIELD: MBSR.bus_busy 5:5 established */
#define SIM_MBSR_BUS_BUSY_SHIFT 5
#define SIM_MBSR_BUS_BUSY_MASK  0x1u
#define SIM_OFFSET_MBDR 0x1f0u /* rw8, inferred */
#define SIM_MFDR_INIT 0x13u /* established: written at bring-up, 100 kHz at the inferred 48 MHz clock */

#endif
