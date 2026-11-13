/* generated from devices/mcf5206e/registers.toml by defs-gen - do not edit, change the source */
/* on-chip registers of the ColdFire MCF5206e as the L1 firmware uses them */
#ifndef SIM_GEN_H
#define SIM_GEN_H

#include <stdint.h>


/*
 * The firmware sizes the CS0 flash by writing the mask for 2 MB, then rewrites it
 * for 1 MB when the two halves of the window read back the same.
 */
/* word CSMR0: 32 bits */
#define SIM_CSMR0_WIDTH                              32 /* established */
#define SIM_CSMR0_BAM_SHIFT                          16 /* established */
#define SIM_CSMR0_BAM_MASK                           0xffffu
#define SIM_CSMR0_BAM_INPLACE                        (SIM_CSMR0_BAM_MASK << SIM_CSMR0_BAM_SHIFT)
#define SIM_CSMR0_BAM_SIZE_1MB                       15 /* established */
#define SIM_CSMR0_BAM_SIZE_2MB                       31 /* established */

/*
 * Chip-select 1 (battery-backed RAM and clock, 8-bit) and chip-select 2 (external RAM,
 * 16-bit) values the board set-up writes. The RAM mask is cut from 512 KB to 128 KB when
 * the firmware's alias probe fails. The port-size decode of each control value is the
 * manual's and was not re-read in an image.
 */
/* word CSMR2: 32 bits */
#define SIM_CSMR2_WIDTH                              32 /* established */
#define SIM_CSMR2_BAM_SHIFT                          16 /* established */
#define SIM_CSMR2_BAM_MASK                           0xffffu
#define SIM_CSMR2_BAM_INPLACE                        (SIM_CSMR2_BAM_MASK << SIM_CSMR2_BAM_SHIFT)
#define SIM_CSMR2_BAM_SIZE_128KB                     1 /* established */
#define SIM_CSMR2_BAM_SIZE_512KB                     7 /* established */

/*
 * M-Bus address register: the module's own seven-bit slave address, in bits 7 to 1. Bit 0 is
 * reserved. Reset value 0x00. Not the calling byte a master sends. Layout from the manual; the
 * firmware's own use is not re-read here.
 */
/* word MADR: 8 bits */
#define SIM_MADR_WIDTH                               8 /* inferred */
#define SIM_MADR_ADR_SHIFT                           1 /* inferred */
#define SIM_MADR_ADR_MASK                            0x7fu
#define SIM_MADR_ADR_INPLACE                         (SIM_MADR_ADR_MASK << SIM_MADR_ADR_SHIFT)

/*
 * M-Bus control register, reset value 0x00. MEN holds the module in reset while clear.
 * RSTA is write-one and always reads zero. Bits 1 and 0 are reserved. Other control bits have no
 * effect while MEN is clear.
 */
/* word MBCR: 8 bits */
#define SIM_MBCR_WIDTH                               8 /* inferred */
#define SIM_MBCR_MEN_SHIFT                           7 /* inferred */
#define SIM_MBCR_MEN_MASK                            0x1u
#define SIM_MBCR_MEN_INPLACE                         (SIM_MBCR_MEN_MASK << SIM_MBCR_MEN_SHIFT)
#define SIM_MBCR_MIEN_SHIFT                          6 /* inferred */
#define SIM_MBCR_MIEN_MASK                           0x1u
#define SIM_MBCR_MIEN_INPLACE                        (SIM_MBCR_MIEN_MASK << SIM_MBCR_MIEN_SHIFT)
#define SIM_MBCR_MSTA_SHIFT                          5 /* inferred */
#define SIM_MBCR_MSTA_MASK                           0x1u
#define SIM_MBCR_MSTA_INPLACE                        (SIM_MBCR_MSTA_MASK << SIM_MBCR_MSTA_SHIFT)
#define SIM_MBCR_MTX_SHIFT                           4 /* inferred */
#define SIM_MBCR_MTX_MASK                            0x1u
#define SIM_MBCR_MTX_INPLACE                         (SIM_MBCR_MTX_MASK << SIM_MBCR_MTX_SHIFT)
#define SIM_MBCR_TXAK_SHIFT                          3 /* inferred */
#define SIM_MBCR_TXAK_MASK                           0x1u
#define SIM_MBCR_TXAK_INPLACE                        (SIM_MBCR_TXAK_MASK << SIM_MBCR_TXAK_SHIFT)
#define SIM_MBCR_RSTA_SHIFT                          2 /* inferred */
#define SIM_MBCR_RSTA_MASK                           0x1u
#define SIM_MBCR_RSTA_INPLACE                        (SIM_MBCR_RSTA_MASK << SIM_MBCR_RSTA_SHIFT)

/*
 * M-Bus data register, reset value 0x00. A write starts the next transmit byte, most
 * significant bit first. A read in receive mode returns the received byte and starts receipt of the
 * next one.
 */
/* word MBDR: 8 bits */
#define SIM_MBDR_WIDTH                               8 /* inferred */
#define SIM_MBDR_DATA_SHIFT                          0 /* inferred */
#define SIM_MBDR_DATA_MASK                           0xffu
#define SIM_MBDR_DATA_INPLACE                        (SIM_MBDR_DATA_MASK << SIM_MBDR_DATA_SHIFT)

/*
 * M-Bus module. The offsets are the manual's, as the firmware adds them to the module base.
 * Reset value 0x81 (MCF and RXAK set). Software writes clear only MAL and MIF, by writing zero;
 * the other bits report hardware state.
 */
/* word MBSR: 32 bits */
#define SIM_MBSR_WIDTH                               32 /* established */
#define SIM_MBSR_MCF_SHIFT                           7 /* inferred */
#define SIM_MBSR_MCF_MASK                            0x1u
#define SIM_MBSR_MCF_INPLACE                         (SIM_MBSR_MCF_MASK << SIM_MBSR_MCF_SHIFT)
#define SIM_MBSR_MAAS_SHIFT                          6 /* inferred */
#define SIM_MBSR_MAAS_MASK                           0x1u
#define SIM_MBSR_MAAS_INPLACE                        (SIM_MBSR_MAAS_MASK << SIM_MBSR_MAAS_SHIFT)
#define SIM_MBSR_BUS_BUSY_SHIFT                      5 /* established */
#define SIM_MBSR_BUS_BUSY_MASK                       0x1u
#define SIM_MBSR_BUS_BUSY_INPLACE                    (SIM_MBSR_BUS_BUSY_MASK << SIM_MBSR_BUS_BUSY_SHIFT)
#define SIM_MBSR_MAL_SHIFT                           4 /* inferred */
#define SIM_MBSR_MAL_MASK                            0x1u
#define SIM_MBSR_MAL_INPLACE                         (SIM_MBSR_MAL_MASK << SIM_MBSR_MAL_SHIFT)
#define SIM_MBSR_SRW_SHIFT                           2 /* inferred */
#define SIM_MBSR_SRW_MASK                            0x1u
#define SIM_MBSR_SRW_INPLACE                         (SIM_MBSR_SRW_MASK << SIM_MBSR_SRW_SHIFT)
#define SIM_MBSR_MIF_SHIFT                           1 /* inferred */
#define SIM_MBSR_MIF_MASK                            0x1u
#define SIM_MBSR_MIF_INPLACE                         (SIM_MBSR_MIF_MASK << SIM_MBSR_MIF_SHIFT)
#define SIM_MBSR_RXAK_SHIFT                          0 /* inferred */
#define SIM_MBSR_RXAK_MASK                           0x1u
#define SIM_MBSR_RXAK_INPLACE                        (SIM_MBSR_RXAK_MASK << SIM_MBSR_RXAK_SHIFT)

/*
 * M-Bus frequency divider register, reset value 0x00. MBC5 to MBC0 select a divider
 * from the manual's table 13-2. The table value is a divider, not a bit rate. The errata says the
 * SCL frequency is the CPU frequency divided by the selected divider, and that the selection must
 * keep the M-Bus below 100 kHz. The rate in hertz is unknown until the controller clock is known.
 * Bits 7 and 6 are reserved; their readback is unknown.
 */
/* word MFDR: 8 bits */
#define SIM_MFDR_WIDTH                               8 /* inferred */
#define SIM_MFDR_MBC_SHIFT                           0 /* inferred */
#define SIM_MFDR_MBC_MASK                            0x3fu
#define SIM_MFDR_MBC_INPLACE                         (SIM_MFDR_MBC_MASK << SIM_MFDR_MBC_SHIFT)
#define SIM_MFDR_MBC_DIVIDE_28                       0 /* inferred */
#define SIM_MFDR_MBC_DIVIDE_68                       7 /* inferred */
#define SIM_MFDR_MBC_DIVIDE_480                      19 /* inferred */
#define SIM_MFDR_MBC_DIVIDE_20                       32 /* inferred */
#define SIM_MFDR_MBC_DIVIDE_2048                     63 /* inferred */

/*
 * Parallel port: direction and data, used as the scan-chain interface.
 */
/* word PADAT: 32 bits */
#define SIM_PADAT_WIDTH                              32 /* established */
#define SIM_PADAT_TDI_SHIFT                          7 /* established */
#define SIM_PADAT_TDI_MASK                           0x1u
#define SIM_PADAT_TDI_INPLACE                        (SIM_PADAT_TDI_MASK << SIM_PADAT_TDI_SHIFT)
#define SIM_PADAT_TCK_SHIFT                          6 /* established */
#define SIM_PADAT_TCK_MASK                           0x1u
#define SIM_PADAT_TCK_INPLACE                        (SIM_PADAT_TCK_MASK << SIM_PADAT_TCK_SHIFT)
#define SIM_PADAT_TMS_SHIFT                          5 /* established */
#define SIM_PADAT_TMS_MASK                           0x1u
#define SIM_PADAT_TMS_INPLACE                        (SIM_PADAT_TMS_MASK << SIM_PADAT_TMS_SHIFT)
#define SIM_PADAT_TRST_SHIFT                         4 /* established */
#define SIM_PADAT_TRST_MASK                          0x1u
#define SIM_PADAT_TRST_INPLACE                       (SIM_PADAT_TRST_MASK << SIM_PADAT_TRST_SHIFT)
#define SIM_PADAT_PSI_SHIFT                          3 /* established */
#define SIM_PADAT_PSI_MASK                           0x1u
#define SIM_PADAT_PSI_INPLACE                        (SIM_PADAT_PSI_MASK << SIM_PADAT_PSI_SHIFT)
#define SIM_PADAT_TDO_SHIFT                          0 /* established */
#define SIM_PADAT_TDO_MASK                           0x1u
#define SIM_PADAT_TDO_INPLACE                        (SIM_PADAT_TDO_MASK << SIM_PADAT_TDO_SHIFT)

/*
 * Pin assignment: bit 7 is pulsed by the timer set-up, bits 5:4 are cleared by the scan set-up.
 */
/* word PAR: 32 bits */
#define SIM_PAR_WIDTH                                32 /* established */
#define SIM_PAR_TIMER_PULSE_SHIFT                    7 /* established */
#define SIM_PAR_TIMER_PULSE_MASK                     0x1u
#define SIM_PAR_TIMER_PULSE_INPLACE                  (SIM_PAR_TIMER_PULSE_MASK << SIM_PAR_TIMER_PULSE_SHIFT)
#define SIM_PAR_SCAN_SEL_SHIFT                       4 /* established */
#define SIM_PAR_SCAN_SEL_MASK                        0x3u
#define SIM_PAR_SCAN_SEL_INPLACE                     (SIM_PAR_SCAN_SEL_MASK << SIM_PAR_SCAN_SEL_SHIFT)

/*
 * Reset status: one bit is tested at start-up to record the reset cause.
 */
/* word RSR: 32 bits */
#define SIM_RSR_WIDTH                                32 /* established */
#define SIM_RSR_CAUSE5_SHIFT                         5 /* established */
#define SIM_RSR_CAUSE5_MASK                          0x1u
#define SIM_RSR_CAUSE5_INPLACE                       (SIM_RSR_CAUSE5_MASK << SIM_RSR_CAUSE5_SHIFT)

/*
 * Software watchdog control: write-once, armed once by the firmware.
 */
/* word SYPCR: 32 bits */
#define SIM_SYPCR_WIDTH                              32 /* established */
#define SIM_SYPCR_SWE_SHIFT                          7 /* established */
#define SIM_SYPCR_SWE_MASK                           0x1u
#define SIM_SYPCR_SWE_INPLACE                        (SIM_SYPCR_SWE_MASK << SIM_SYPCR_SWE_SHIFT)

#define SIM_CS0_START_CSAR                           0xffe0u /* established */
#define SIM_CS0_START_CSCR                           0x1183u /* established */
#define SIM_CS1_START_CSAR                           0x2000u /* established */
#define SIM_CS1_START_CSCR                           0x1143u /* established */
#define SIM_CS1_START_CSMR                           0x0u /* established */
#define SIM_CS2_START_CSAR                           0x3000u /* established */
#define SIM_CS2_START_CSCR                           0x983u /* established */
#define SIM_CS3_START_CSAR                           0x4000u /* established */
#define SIM_CS3_START_CSCR                           0x953u /* established */
#define SIM_CS3_START_CSMR                           0x0u /* established */
#define SIM_CS4_START_CSAR                           0x5000u /* established */
#define SIM_CS4_START_CSCR                           0x95fu /* established */
#define SIM_CS4_START_CSMR                           0x0u /* established */
#define SIM_DMCR_START_VALUE                         0x0u /* established */
#define SIM_ICR10_START_VALUE                        0x80u /* established */
#define SIM_ICR11_RESET_VALUE                        0x80u /* inferred */
#define SIM_ICR11_START_VALUE                        0x99u /* established */
#define SIM_ICR12_START_VALUE                        0x1au /* established */
#define SIM_ICR13_START_VALUE                        0x0u /* established */
#define SIM_ICR14_START_VALUE                        0x9u /* established */
#define SIM_ICR15_START_VALUE                        0xau /* established */
#define SIM_ICR1_START_VALUE                         0x84u /* established */
#define SIM_ICR2_START_VALUE                         0x88u /* established */
#define SIM_ICR3_START_VALUE                         0x80u /* established */
#define SIM_ICR4_START_VALUE                         0x90u /* established */
#define SIM_ICR5_START_VALUE                         0x80u /* established */
#define SIM_ICR6_START_VALUE                         0x98u /* established */
#define SIM_ICR7_START_VALUE                         0x9cu /* established */
#define SIM_ICR8_START_VALUE                         0x1du /* established */
#define SIM_ICR9_START_VALUE                         0x94u /* established */
#define SIM_IMR_START_VALUE                          0xfffeu /* established */
#define SIM_MBSR_RESET_VALUE                         0x81u /* inferred */
#define SIM_MFDR_INIT                                0x13u /* established */
#define SIM_OFFSET_CSAR0                             0x64u /* established */
#define SIM_OFFSET_CSAR1                             0x70u /* established */
#define SIM_OFFSET_CSAR2                             0x7cu /* established */
#define SIM_OFFSET_CSAR3                             0x88u /* established */
#define SIM_OFFSET_CSAR4                             0x94u /* established */
#define SIM_OFFSET_CSAR5                             0xa0u /* established */
#define SIM_OFFSET_CSCR0                             0x6eu /* established */
#define SIM_OFFSET_CSCR1                             0x7au /* established */
#define SIM_OFFSET_CSCR2                             0x86u /* established */
#define SIM_OFFSET_CSCR3                             0x92u /* established */
#define SIM_OFFSET_CSCR4                             0x9eu /* established */
#define SIM_OFFSET_CSCR5                             0xaau /* established */
#define SIM_OFFSET_CSMR0                             0x68u /* established */
#define SIM_OFFSET_CSMR1                             0x74u /* established */
#define SIM_OFFSET_CSMR2                             0x80u /* established */
#define SIM_OFFSET_CSMR3                             0x8cu /* established */
#define SIM_OFFSET_CSMR4                             0x98u /* established */
#define SIM_OFFSET_CSMR5                             0xa4u /* established */
#define SIM_OFFSET_DMCR                              0xc6u /* established */
#define SIM_OFFSET_ICR1                              0x14u /* established */
#define SIM_OFFSET_ICR10                             0x1du /* established */
#define SIM_OFFSET_ICR11                             0x1eu /* established */
#define SIM_OFFSET_ICR12                             0x1fu /* established */
#define SIM_OFFSET_ICR13                             0x20u /* established */
#define SIM_OFFSET_ICR14                             0x21u /* established */
#define SIM_OFFSET_ICR15                             0x22u /* established */
#define SIM_OFFSET_ICR2                              0x15u /* established */
#define SIM_OFFSET_ICR3                              0x16u /* established */
#define SIM_OFFSET_ICR4                              0x17u /* established */
#define SIM_OFFSET_ICR5                              0x18u /* established */
#define SIM_OFFSET_ICR6                              0x19u /* established */
#define SIM_OFFSET_ICR7                              0x1au /* established */
#define SIM_OFFSET_ICR8                              0x1bu /* established */
#define SIM_OFFSET_ICR9                              0x1cu /* established */
#define SIM_OFFSET_IMR                               0x36u /* established */
#define SIM_OFFSET_MADR                              0x1e0u /* inferred */
#define SIM_OFFSET_MBCR                              0x1e8u /* inferred */
#define SIM_OFFSET_MBDR                              0x1f0u /* inferred */
#define SIM_OFFSET_MBSR                              0x1ecu /* inferred */
#define SIM_OFFSET_MFDR                              0x1e4u /* inferred */
#define SIM_OFFSET_PADAT                             0x1c9u /* established */
#define SIM_OFFSET_PADDR                             0x1c5u /* established */
#define SIM_OFFSET_PAR                               0xcau /* established */
#define SIM_OFFSET_RSR                               0x40u /* established */
#define SIM_OFFSET_SIMR                              0x3u /* established */
#define SIM_OFFSET_SWIVR                             0x42u /* established */
#define SIM_OFFSET_SWSR                              0x43u /* established */
#define SIM_OFFSET_SYPCR                             0x41u /* established */
#define SIM_OFFSET_TMR1                              0x100u /* inferred */
#define SIM_OFFSET_TMR2                              0x120u /* inferred */
#define SIM_OFFSET_TMR_AUX_A                         0x1b8u /* inferred */
#define SIM_OFFSET_TMR_AUX_B                         0x1bcu /* inferred */
#define SIM_OFFSET_TRR1                              0x104u /* inferred */
#define SIM_OFFSET_TRR2                              0x124u /* inferred */
#define SIM_OFFSET_UART1                             0x140u /* established */
#define SIM_PADDR_SCAN_SETUP                         0xfeu /* established */
#define SIM_SIMR_START_VALUE                         0xc0u /* established */
#define SIM_SWSR_SERVICE_FIRST                       0x55u /* established */
#define SIM_SWSR_SERVICE_SECOND                      0xaau /* established */
#define SIM_TMR1_INIT                                0x1du /* established */
#define SIM_TMR2_INIT                                0x2bu /* established */
#define SIM_TRR1_TICK_REFERENCE                      0x752fu /* established */
#define SIM_TRR2_REFERENCE                           0x1u /* established */
#define SIM_UART1_VECTOR                             0x42u /* established */

#endif /* SIM_GEN_H */
