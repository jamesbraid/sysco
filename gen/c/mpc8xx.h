/* generated from defs/mpc8xx.toml by defs-gen - do not edit, change the source */
/* start-up values of the PowerPC communications controller on the L2 board */
#ifndef M8X_GEN_H
#define M8X_GEN_H

#include <stdint.h>


/*
 * Byte-wide peripheral on the fourth chip select. Established: the window decode above, the
 * 8-bit port, and that the boot code only writes it, in the order given. What the device is
 * stays unknown, so the three registers are named by offset. Whether the kernel or any
 * program uses the part after mapping the first 4 KiB is not settled.
 */
/* reg0: w8, space bytedev, reset unknown */
#define M8X_REG0_OFFSET                              0x0u /* established */

/* reg1: w8, space bytedev, reset unknown */
#define M8X_REG1_OFFSET                              0x1u /* established */

/* reg3: w8, space bytedev, reset unknown */
#define M8X_REG3_OFFSET                              0x3u /* established */

/*
 * First baud-rate generator, loaded by stage A of the console loader. Its place in the register
 * map is the public map's: the store goes through a base that sits 0x30 below the register, and
 * the same routine uses that base for the communications-processor command register and for
 * port B (read in l2.bin 1.44.0), so the position is inferred. Stage A computes the divisor
 * field as (clock word >> 4) / baud, doubled, with the enable bit or-ed in; with its clock word
 * of 0x32 and baud word 0x9600 the quotient is zero, so it stores only the enable bit. The
 * wrapper's clock word is 50,000,000, which gives the quotient 81 and the value 0x100a2.
 */
/* brgc1: rw32, space mpc8xx_brg, reset unknown */
#define M8X_BRGC1_OFFSET                             0x9f0u /* inferred */

/*
 * Clock and reset registers written by the reset code; the multiplication factor and
 * division fields were not decoded.
 */
/* sccr: rw32, space mpc8xx_clk, reset unknown */
#define M8X_SCCR_OFFSET                              0x280u /* established */

/* plprcr: rw32, space mpc8xx_clk, reset unknown */
#define M8X_PLPRCR_OFFSET                            0x284u /* established */

/*
 * Memory controller banks: the base and option register of each of the eight banks, written
 * from a 14-word table (banks 1 to 7) and, for bank 0, built and stored by the reset code. The
 * width, base, mask and timing fields of the two registers were not decoded here.
 */
/* br0: rw32, space mpc8xx_cs, reset unknown */
#define M8X_BR0_OFFSET                               0x100u /* established */

/* or0: rw32, space mpc8xx_cs, reset unknown */
#define M8X_OR0_OFFSET                               0x104u /* established */

/* br1: rw32, space mpc8xx_cs, reset unknown */
#define M8X_BR1_OFFSET                               0x108u /* established */

/* or1: rw32, space mpc8xx_cs, reset unknown */
#define M8X_OR1_OFFSET                               0x10cu /* established */

/* br2: rw32, space mpc8xx_cs, reset unknown */
#define M8X_BR2_OFFSET                               0x110u /* established */

/* or2: rw32, space mpc8xx_cs, reset unknown */
#define M8X_OR2_OFFSET                               0x114u /* established */

/* br3: rw32, space mpc8xx_cs, reset unknown */
#define M8X_BR3_OFFSET                               0x118u /* established */

/* or3: rw32, space mpc8xx_cs, reset unknown */
#define M8X_OR3_OFFSET                               0x11cu /* established */

/* br4: rw32, space mpc8xx_cs, reset unknown */
#define M8X_BR4_OFFSET                               0x120u /* established */

/* or4: rw32, space mpc8xx_cs, reset unknown */
#define M8X_OR4_OFFSET                               0x124u /* established */

/* br5: rw32, space mpc8xx_cs, reset unknown */
#define M8X_BR5_OFFSET                               0x128u /* established */

/* or5: rw32, space mpc8xx_cs, reset unknown */
#define M8X_OR5_OFFSET                               0x12cu /* established */

/* br6: rw32, space mpc8xx_cs, reset unknown */
#define M8X_BR6_OFFSET                               0x130u /* established */

/* or6: rw32, space mpc8xx_cs, reset unknown */
#define M8X_OR6_OFFSET                               0x134u /* established */

/* br7: rw32, space mpc8xx_cs, reset unknown */
#define M8X_BR7_OFFSET                               0x138u /* established */

/* or7: rw32, space mpc8xx_cs, reset unknown */
#define M8X_OR7_OFFSET                               0x13cu /* established */

/* reg_004: rw32, space mpc8xx_pcib, reset unknown */
#define M8X_REG_004_OFFSET                           0x4u /* inferred */

/* reg_00c: rw32, space mpc8xx_pcib, reset unknown */
#define M8X_REG_00C_OFFSET                           0xcu /* inferred */

/* reg_010: rw32, space mpc8xx_pcib, reset unknown */
#define M8X_REG_010_OFFSET                           0x10u /* inferred */

/* reg_030: rw32, space mpc8xx_pcib, reset unknown */
#define M8X_REG_030_OFFSET                           0x30u /* inferred */

/* reg_03c: rw32, space mpc8xx_pcib, reset unknown */
#define M8X_REG_03C_OFFSET                           0x3cu /* inferred */

/* reg_100: rw32, space mpc8xx_pcib, reset unknown */
#define M8X_REG_100_OFFSET                           0x100u /* inferred */

/* reg_104: rw32, space mpc8xx_pcib, reset unknown */
#define M8X_REG_104_OFFSET                           0x104u /* inferred */

/* reg_110: rw32, space mpc8xx_pcib, reset unknown */
#define M8X_REG_110_OFFSET                           0x110u /* inferred */

/* reg_114: rw32, space mpc8xx_pcib, reset unknown */
#define M8X_REG_114_OFFSET                           0x114u /* inferred */

/* reg_13c: rw32, space mpc8xx_pcib, reset unknown */
#define M8X_REG_13C_OFFSET                           0x13cu /* inferred */

/* reg_140: rw32, space mpc8xx_pcib, reset unknown */
#define M8X_REG_140_OFFSET                           0x140u /* inferred */

/* reg_400: rw32, space mpc8xx_pcib, reset unknown */
#define M8X_REG_400_OFFSET                           0x400u /* inferred */

/* reg_404: rw32, space mpc8xx_pcib, reset unknown */
#define M8X_REG_404_OFFSET                           0x404u /* inferred */

/* reg_408: rw32, space mpc8xx_pcib, reset unknown */
#define M8X_REG_408_OFFSET                           0x408u /* inferred */

/*
 * The bridge's configuration-access pair, used by the second stage to reach the devices behind
 * it. The address register is loaded first, then the data register is read or written at the
 * data offset plus the byte lane (byte, halfword and word accesses). The address word is built
 * from the bus number, the device and function byte and the register number with its low two
 * bits cleared, and bit 0 is set only when the bus number is not 0. The widths of the bus and
 * device-and-function arguments were not read, so those two field extents are inferred from
 * the usual layout; the register number mask and the type bit follow from the code.
 */
/* cfg_addr: rw32, space mpc8xx_pcib, reset unknown */
#define M8X_CFG_ADDR_OFFSET                          0x500u /* established */

/* cfg_data: rw32, space mpc8xx_pcib, reset unknown */
#define M8X_CFG_DATA_OFFSET                          0x504u /* established */

/* reg_600: rw32, space mpc8xx_pcib, reset unknown */
#define M8X_REG_600_OFFSET                           0x600u /* inferred */

/* reg_604: rw32, space mpc8xx_pcib, reset unknown */
#define M8X_REG_604_OFFSET                           0x604u /* inferred */

/* reg_608: rw32, space mpc8xx_pcib, reset unknown */
#define M8X_REG_608_OFFSET                           0x608u /* inferred */

/* reg_800: rw32, space mpc8xx_pcib, reset unknown */
#define M8X_REG_800_OFFSET                           0x800u /* inferred */

/* reg_f00: rw32, space mpc8xx_pcib, reset unknown */
#define M8X_REG_F00_OFFSET                           0xf00u /* inferred */

/* reg_f04: rw32, space mpc8xx_pcib, reset unknown */
#define M8X_REG_F04_OFFSET                           0xf04u /* inferred */

/* reg_f10: rw32, space mpc8xx_pcib, reset unknown */
#define M8X_REG_F10_OFFSET                           0xf10u /* inferred */

/* reg_f14: rw32, space mpc8xx_pcib, reset unknown */
#define M8X_REG_F14_OFFSET                           0xf14u /* inferred */

/* reg_f80: rw32, space mpc8xx_pcib, reset unknown */
#define M8X_REG_F80_OFFSET                           0xf80u /* inferred */

/*
 * System protection control and software service registers. The boot code stores one 32-bit
 * word to the control register only on its failure path (image check failed, or the second
 * stage returned); a normal boot never writes it. The value keeps the high bits set and
 * clears the low seven. Both widths are those of the stores.
 */
/* sypcr: rw32, space mpc8xx_siu, reset unknown */
#define M8X_SYPCR_OFFSET                             0x4u /* established */

/* swsr: rw16, space mpc8xx_siu, reset unknown */
#define M8X_SWSR_OFFSET                              0xeu /* established */

/*
 * Memory controller timing and the user-programmable machine A. The second stage writes the
 * periodic timer prescaler and the machine mode register, loads 64 array words through the
 * data register (each followed by a store of its index to the command register), then issues
 * run commands: one at array address 0x2d, eight at 0x30, then the address register is set
 * and one run at 0x13 is issued. The meaning of the machine mode fields was not decoded.
 */
/* mar: rw32, space mpc8xx_upm, reset unknown */
#define M8X_MAR_OFFSET                               0x164u /* established */

/* mcr: rw32, space mpc8xx_upm, reset unknown */
#define M8X_MCR_OFFSET                               0x168u /* established */

/* mamr: rw32, space mpc8xx_upm, reset unknown */
#define M8X_MAMR_OFFSET                              0x170u /* established */

/* mptpr: rw16, space mpc8xx_upm, reset unknown */
#define M8X_MPTPR_OFFSET                             0x17au /* established */

/* mdr: rw32, space mpc8xx_upm, reset unknown */
#define M8X_MDR_OFFSET                               0x17cu /* established */

/*
 * Machine state register written once by the reset code: machine-check enable and
 * recoverable-interrupt set, translation off, supervisor mode. The bit positions are
 * the PowerPC architecture's.
 */
/* word MSR: 32 bits */
#define M8X_MSR_WIDTH                                32 /* inferred */
#define M8X_MSR_ME_SHIFT                             12 /* inferred */
#define M8X_MSR_ME_MASK                              0x1u
#define M8X_MSR_ME_INPLACE                           (M8X_MSR_ME_MASK << M8X_MSR_ME_SHIFT)
#define M8X_MSR_RI_SHIFT                             1 /* inferred */
#define M8X_MSR_RI_MASK                              0x1u
#define M8X_MSR_RI_INPLACE                           (M8X_MSR_RI_MASK << M8X_MSR_RI_SHIFT)

/* word PCI_CFG_ADDR: 32 bits */
#define M8X_PCI_CFG_ADDR_WIDTH                       32 /* established */
#define M8X_PCI_CFG_ADDR_BUS_SHIFT                   16 /* inferred */
#define M8X_PCI_CFG_ADDR_BUS_MASK                    0xffu
#define M8X_PCI_CFG_ADDR_BUS_INPLACE                 (M8X_PCI_CFG_ADDR_BUS_MASK << M8X_PCI_CFG_ADDR_BUS_SHIFT)
#define M8X_PCI_CFG_ADDR_DEVFN_SHIFT                 8 /* inferred */
#define M8X_PCI_CFG_ADDR_DEVFN_MASK                  0xffu
#define M8X_PCI_CFG_ADDR_DEVFN_INPLACE               (M8X_PCI_CFG_ADDR_DEVFN_MASK << M8X_PCI_CFG_ADDR_DEVFN_SHIFT)
#define M8X_PCI_CFG_ADDR_REGISTER_SHIFT              2 /* established */
#define M8X_PCI_CFG_ADDR_REGISTER_MASK               0x3fu
#define M8X_PCI_CFG_ADDR_REGISTER_INPLACE            (M8X_PCI_CFG_ADDR_REGISTER_MASK << M8X_PCI_CFG_ADDR_REGISTER_SHIFT)
#define M8X_PCI_CFG_ADDR_TYPE1_SHIFT                 0 /* established */
#define M8X_PCI_CFG_ADDR_TYPE1_MASK                  0x1u
#define M8X_PCI_CFG_ADDR_TYPE1_INPLACE               (M8X_PCI_CFG_ADDR_TYPE1_MASK << M8X_PCI_CFG_ADDR_TYPE1_SHIFT)

#define M8X_BR0_START_VALUE                          0xfe000001u /* established */
#define M8X_BR1_START_VALUE                          0x81u /* established */
#define M8X_BR2_START_VALUE                          0x10000801u /* established */
#define M8X_BR3_START_VALUE                          0x0u /* established */
#define M8X_BR4_START_VALUE                          0xfa100401u /* established */
#define M8X_BR5_START_VALUE                          0x80000001u /* established */
#define M8X_BR6_START_VALUE                          0xfa210001u /* established */
#define M8X_BR7_START_VALUE                          0xfc000401u /* established */
#define M8X_BRGC1_STAGE_A_VALUE                      0x10000u /* established */
#define M8X_BRGC1_WRAPPER_VALUE                      0x100a2u /* established */
#define M8X_BYTEDEV_MAPPED_SIZE                      0x1000u /* established */
#define M8X_BYTEDEV_REG0_FIRST                       0x0u /* established */
#define M8X_BYTEDEV_REG0_SECOND                      0x80u /* established */
#define M8X_BYTEDEV_REG1_FIRST                       0xeu /* established */
#define M8X_BYTEDEV_REG1_LAST                        0xeu /* established */
#define M8X_BYTEDEV_REG1_SECOND                      0xfu /* established */
#define M8X_BYTEDEV_REG3_VALUE                       0x18u /* established */
#define M8X_FLASH_CMD_AUTOSELECT                     0x900090u /* established */
#define M8X_FLASH_CMD_ERASE_SETUP                    0x800080u /* established */
#define M8X_FLASH_CMD_PROGRAM                        0xa000a0u /* established */
#define M8X_FLASH_CMD_RESET                          0xf000f0u /* established */
#define M8X_FLASH_CMD_SECTOR_ERASE                   0x300030u /* established */
#define M8X_FLASH_STATUS_ERROR                       0x200020u /* established */
#define M8X_FLASH_STATUS_TOGGLE                      0x400040u /* established */
#define M8X_FLASH_UNLOCK_DATA_1                      0xaa00aau /* established */
#define M8X_FLASH_UNLOCK_DATA_2                      0x550055u /* established */
#define M8X_FLASH_UNLOCK_OFFSET_1                    0x1554u /* established */
#define M8X_FLASH_UNLOCK_OFFSET_2                    0xaa8u /* established */
#define M8X_HC0_WINDOW_BASE                          0xc0000000u /* established */
#define M8X_HC1_WINDOW_BASE                          0xc0001000u /* established */
#define M8X_IMMR_VALUE                               0xfa200000u /* established */
#define M8X_MAMR_START_VALUE                         0x18804000u /* established */
#define M8X_MAR_START_VALUE                          0x88u /* established */
#define M8X_MCR_RUN_FIRST                            0x8000202du /* established */
#define M8X_MCR_RUN_LAST                             0x80002013u /* established */
#define M8X_MCR_RUN_SECOND                           0x80002030u /* established */
#define M8X_MPTPR_START_VALUE                        0x200u /* established */
#define M8X_MSR_RESET_VALUE                          0x1002u /* established */
#define M8X_OR0_START_VALUE                          0xff800940u /* established */
#define M8X_OR1_START_VALUE                          0xff000400u /* established */
#define M8X_OR2_START_VALUE                          0xfffe0938u /* established */
#define M8X_OR3_START_VALUE                          0x0u /* established */
#define M8X_OR4_START_VALUE                          0xfffe0930u /* established */
#define M8X_OR5_START_VALUE                          0xa0000008u /* established */
#define M8X_OR6_START_VALUE                          0xffff0108u /* established */
#define M8X_OR7_START_VALUE                          0xff800930u /* established */
#define M8X_PCIB_INIT_004                            0xf9000147u /* established */
#define M8X_PCIB_INIT_00C                            0x0u /* established */
#define M8X_PCIB_INIT_010                            0x1000000u /* established */
#define M8X_PCIB_INIT_030                            0x0u /* established */
#define M8X_PCIB_INIT_03C                            0x100u /* established */
#define M8X_PCIB_INIT_100                            0x88800080u /* established */
#define M8X_PCIB_INIT_104                            0x80000000u /* established */
#define M8X_PCIB_INIT_110                            0x0u /* established */
#define M8X_PCIB_INIT_114                            0x0u /* established */
#define M8X_PCIB_INIT_13C                            0x0u /* established */
#define M8X_PCIB_INIT_140                            0x81000000u /* established */
#define M8X_PCIB_INIT_400                            0x780008u /* established */
#define M8X_PCIB_INIT_404                            0x0u /* established */
#define M8X_PCIB_INIT_408                            0x0u /* established */
#define M8X_PCIB_INIT_600                            0xeff0000fu /* established */
#define M8X_PCIB_INIT_604                            0x0u /* established */
#define M8X_PCIB_INIT_608                            0x0u /* established */
#define M8X_PCIB_INIT_800                            0xc1013u /* established */
#define M8X_PCIB_INIT_F00                            0x1000000u /* established */
#define M8X_PCIB_INIT_F04                            0x800000d1u /* established */
#define M8X_PCIB_INIT_F10                            0x0u /* established */
#define M8X_PCIB_INIT_F14                            0xc00000d1u /* established */
#define M8X_PCIB_INIT_F80                            0x81000000u /* established */
#define M8X_PLPRCR_START_VALUE                       0x5040u /* established */
#define M8X_SCCR_START_VALUE                         0x62000000u /* established */
#define M8X_SMC1_PARAM_PAGE                          0x3e80u /* established */
#define M8X_SWSR_KEY_FIRST                           0x556cu /* established */
#define M8X_SWSR_KEY_SECOND                          0xaa39u /* established */
#define M8X_SYPCR_FAILURE_PATH_VALUE                 0xffffff80u /* established */
#define M8X_UPM_PROGRAM_WORDS                        0x40u /* established */
#define M8X_VPD_BLOCK_SIZE                           0x100u /* established */
#define M8X_VPD_CHAIN_START                          0xau /* established */
#define M8X_VPD_EEPROM_ADDR7                         0x52u /* established */
#define M8X_VPD_EEPROM_ADDR8                         0xa4u /* established */
#define M8X_VPD_TAG_END                              0xffu /* established */
#define M8X_VPD_TAG_ETHERNET                         0x8u /* established */

#endif /* M8X_GEN_H */
