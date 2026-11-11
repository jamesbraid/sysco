/*
 * L1_BOARD_DECODE.h - board-level bit layouts and constants of the ColdFire L1 firmware
 *
 * Bit positions of the I/O expander bytes the L1 drives, the flash command
 * stream, the quad UART, the battery-backed clock window and the hardware
 * monitor start-up values. Window offsets are relative to the base of the
 * device's chip select. Expander addresses come in the 8-bit wire form and
 * the 7-bit form.
 *
 * Each ENUM or FIELD line carries the grade of the reading. Polarity grades
 * are given separately where the firmware only shows that a bit is written.
 */
#ifndef L1_BOARD_DECODE_H
#define L1_BOARD_DECODE_H

/* Processor-module control expanders, wire addresses 0x42 and 0x44 (7-bit 0x21 and 0x22). */
#define L1B_CNTL1_WIRE 0x42u /* established */
#define L1B_CNTL2_WIRE 0x44u /* established */

/* FIELD: CNTL1.reset_a 0:0 established */
#define L1B_CNTL1_RESET_A_SHIFT 0
#define L1B_CNTL1_RESET_A_MASK  0x1u
/* FIELD: CNTL1.reset_b 1:1 established */
#define L1B_CNTL1_RESET_B_SHIFT 1
#define L1B_CNTL1_RESET_B_MASK  0x1u
/* Non-maskable-interrupt line: idle high, driven low then high by the nmi command (polarity inferred). */
/* FIELD: CNTL1.nmi 6:6 established */
#define L1B_CNTL1_NMI_SHIFT 6
#define L1B_CNTL1_NMI_MASK  0x1u
/* Soft-reset line: idle low, set then cleared by the soft-reset command (polarity inferred). */
/* FIELD: CNTL1.soft_reset 7:7 established */
#define L1B_CNTL1_SOFT_RESET_SHIFT 7
#define L1B_CNTL1_SOFT_RESET_MASK  0x1u
#define L1B_CNTL1_START_VALUE 0x72u /* established: applied at cold start */

/* FIELD: CNTL2.reset_c 0:0 established */
#define L1B_CNTL2_RESET_C_SHIFT 0
#define L1B_CNTL2_RESET_C_MASK  0x1u
/* Strap or configuration bits written once per release; what they select is unknown. */
/* FIELD: CNTL2.strap 7:3 unknown */
#define L1B_CNTL2_STRAP_SHIFT 3
#define L1B_CNTL2_STRAP_MASK  0x1fu
#define L1B_CNTL2_START_VALUE 0x8au /* established: applied at cold start */

/* Bus-segment select expander: 7-bit address 0x27, one value per segment on the C-brick. */
#define L1B_BUS_SELECT_ADDR7 0x27u /* established */
#define L1B_BUS_SELECT_SEG1 0xffu /* established */
#define L1B_BUS_SELECT_SEG2 0x78u /* established */
#define L1B_BUS_SELECT_SEG3 0xb8u /* established */
#define L1B_BUS_SELECT_SEG4 0xd8u /* established */
#define L1B_BUS_SELECT_SEG5 0xe8u /* established */
#define L1B_BUS_SELECT_SEG6_FIRST 0xf0u /* established: segments 6 to 13 are 0xf0 to 0xf7 */
#define L1B_BUS_SELECT_SEG13_LAST 0xf7u /* established */

/* I-brick bridge control expander: expander index 5, 7-bit address 0x24. */
#define L1B_XBRIDGE_ADDR7 0x24u /* established */
/* FIELD: XBRIDGE.reset_0 0:0 established */
#define L1B_XBRIDGE_RESET_0_SHIFT 0
#define L1B_XBRIDGE_RESET_0_MASK  0x1u
/* FIELD: XBRIDGE.reset_1 1:1 established */
#define L1B_XBRIDGE_RESET_1_SHIFT 1
#define L1B_XBRIDGE_RESET_1_MASK  0x1u
/* FIELD: XBRIDGE.host_enable_a 2:2 established */
#define L1B_XBRIDGE_HOST_ENABLE_A_SHIFT 2
#define L1B_XBRIDGE_HOST_ENABLE_A_MASK  0x1u
/* FIELD: XBRIDGE.reset_3 3:3 established */
#define L1B_XBRIDGE_RESET_3_SHIFT 3
#define L1B_XBRIDGE_RESET_3_MASK  0x1u
/* FIELD: XBRIDGE.host_enable_b 4:4 established */
#define L1B_XBRIDGE_HOST_ENABLE_B_SHIFT 4
#define L1B_XBRIDGE_HOST_ENABLE_B_MASK  0x1u
/* FIELD: XBRIDGE.clock_select 5:5 established */
#define L1B_XBRIDGE_CLOCK_SELECT_SHIFT 5
#define L1B_XBRIDGE_CLOCK_SELECT_MASK  0x1u
#define L1B_XBRIDGE_START_VALUE 0xd4u /* established */

/* R-brick router expanders. */
#define L1B_ROUTER_CONTROL_ADDR7 0x23u /* established: expander index 2 */
/* FIELD: ROUTER.router_type 7:4 established */
#define L1B_ROUTER_ROUTER_TYPE_SHIFT 4
#define L1B_ROUTER_ROUTER_TYPE_MASK  0xfu
#define L1B_ROUTER_RESET_BITS_MASK 0x0du /* established: bits stepped through hold and release */
#define L1B_ROUTER_HOLD_VALUE 0x01u /* established: written under that mask by hold */
#define L1B_ROUTER_START_VALUE 0xf1u /* established */
/* FIELD: RPOWER.router_power 3:3 established */
#define L1B_RPOWER_ROUTER_POWER_SHIFT 3
#define L1B_RPOWER_ROUTER_POWER_MASK  0x1u

/* Front-panel expander: 7-bit address 0x20, low nibble input. */
#define L1B_DISPLAY_CONTROL_ADDR7 0x20u /* established */
#define L1B_DISPLAY_INPUT_MASK 0x0fu /* established */
#define L1B_DISPLAY_CHANGE_MASK 0x07u /* established: bits whose changes are reported */
/* FIELD: DISPLAY.led_4 4:4 established */
#define L1B_DISPLAY_LED_4_SHIFT 4
#define L1B_DISPLAY_LED_4_MASK  0x1u
/* FIELD: DISPLAY.led_5 5:5 established */
#define L1B_DISPLAY_LED_5_SHIFT 5
#define L1B_DISPLAY_LED_5_MASK  0x1u
/* FIELD: DISPLAY.led_6 6:6 established */
#define L1B_DISPLAY_LED_6_SHIFT 6
#define L1B_DISPLAY_LED_6_MASK  0x1u

/* Flash on the first chip select: 16-bit port, command writes at these halfword-byte offsets. */
#define L1B_FLASH_UNLOCK_ADDR_1 0xaaau /* established */
#define L1B_FLASH_UNLOCK_ADDR_2 0x554u /* established */
#define L1B_FLASH_UNLOCK_DATA_1 0xaau /* established */
#define L1B_FLASH_UNLOCK_DATA_2 0x55u /* established */
#define L1B_FLASH_CMD_RESET 0xf0u /* established */
#define L1B_FLASH_CMD_PROGRAM 0xa0u /* established */
#define L1B_FLASH_CMD_ERASE_SETUP 0x80u /* established: followed by the unlock pair and sector erase */
#define L1B_FLASH_CMD_SECTOR_ERASE 0x30u /* established */
#define L1B_FLASH_CMD_ERASE_SUSPEND 0xb0u /* established */
#define L1B_FLASH_CMD_ERASE_RESUME 0x30u /* established */
#define L1B_FLASH_CMD_AUTOSELECT 0x90u /* established */
/* Completion is read by comparing two successive halfword reads. */
/* FIELD: FLASH_STATUS.timeout 5:5 established */
#define L1B_FLASH_STATUS_TIMEOUT_SHIFT 5
#define L1B_FLASH_STATUS_TIMEOUT_MASK  0x1u
/* FIELD: FLASH_STATUS.toggle 6:6 established */
#define L1B_FLASH_STATUS_TOGGLE_SHIFT 6
#define L1B_FLASH_STATUS_TOGGLE_MASK  0x1u

/* Interrupt wiring. */
#define L1B_TIMER1_VECTOR 29u /* established: external level 5 */
#define L1B_TIMER1_LEVEL 5u /* established */
#define L1B_UART_VECTOR 0x42u /* established: on-chip UART */
#define L1B_UART_LEVEL 6u /* established */
#define L1B_UART_BAUD 19200u /* established */

/* Quad UART: four channels, each a 16550-style register set; the part is unknown. */
#define L1B_QUART_CHANNEL0 0x00u /* established: byte offset in the chip-select window */
#define L1B_QUART_CHANNEL1 0x08u /* established */
#define L1B_QUART_CHANNEL2 0x10u /* established */
#define L1B_QUART_CHANNEL3 0x18u /* established */

/* Battery-backed clock: control byte at the top of the window; BCD time bytes follow it. */
#define L1B_CLOCK_CONTROL_INDEX 0x7f8u /* established */
/* FIELD: CLOCK.century_hundreds 3:0 established */
#define L1B_CLOCK_CENTURY_HUNDREDS_SHIFT 0
#define L1B_CLOCK_CENTURY_HUNDREDS_MASK  0xfu
/* FIELD: CLOCK.century_tens 5:4 established */
#define L1B_CLOCK_CENTURY_TENS_SHIFT 4
#define L1B_CLOCK_CENTURY_TENS_MASK  0x3u
/* FIELD: CLOCK.read_latch 6:6 established */
#define L1B_CLOCK_READ_LATCH_SHIFT 6
#define L1B_CLOCK_READ_LATCH_MASK  0x1u
/* FIELD: CLOCK.write_enable 7:7 established */
#define L1B_CLOCK_WRITE_ENABLE_SHIFT 7
#define L1B_CLOCK_WRITE_ENABLE_MASK  0x1u

/* Hardware monitor start-up writes (register index, value); the part marking is unknown. */
#define L1B_MONITOR_INIT_REG_40 0x80u /* established: register 0x40 */
#define L1B_MONITOR_INIT_REG_43 0xdfu /* established: register 0x43 */
#define L1B_MONITOR_INIT_REG_44 0x13u /* established: register 0x44 */

/* ColdFire MOVEC register-to-control opcode. */
#define L1B_MOVEC_RN_RC_OPCODE 0x4e7bu /* established */
/* Control-register codes of the MOVEC extension word and the values the firmware writes
 * at start-up. The register names are the manual's. Instruction cache control is written
 * twice: invalidate, then enable. */
#define L1B_MOVEC_CODE_CACR 0x002u /* established */
#define L1B_MOVEC_CODE_VBR 0x801u /* established */
#define L1B_MOVEC_CODE_RAMBAR 0xc04u /* established */
#define L1B_MOVEC_CODE_MBAR 0xc0fu /* established */
#define L1B_RAMBAR_START_VALUE 0x30100001u /* established: on-chip SRAM base with the valid bit */
#define L1B_MBAR_START_VALUE 0x10000001u /* established: peripheral block base with the valid bit */
#define L1B_VBR_START_VALUE 0x30100000u /* established: vector table at the SRAM base */
/* FIELD: CACR.cinv 24:24 inferred */
#define L1B_CACR_CINV_SHIFT 24
#define L1B_CACR_CINV_MASK  0x1u
/* FIELD: CACR.cenb 31:31 inferred */
#define L1B_CACR_CENB_SHIFT 31
#define L1B_CACR_CENB_MASK  0x1u
#define L1B_CACR_FIRST_WRITE 0x01000000u /* established: invalidate */
#define L1B_CACR_SECOND_WRITE 0x80000000u /* established: enable */

/* Rack numbers in L1 setting 1 and the identity word are 10 bits wide. */
#define L1B_RACK_WIDTH_BITS 10u /* established */
#define L1B_RACK_MAX 1023u /* established */

#endif
