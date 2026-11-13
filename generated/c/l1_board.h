/* generated from devices/l1_board/registers.toml by defs-gen - do not edit, change the source */
/* board-level bit layouts and constants of the ColdFire L1 firmware */
#ifndef L1B_GEN_H
#define L1B_GEN_H

#include <stdint.h>


/*
 * Control-register codes of the MOVEC extension word and the values the firmware writes
 * at start-up. The register names are the manual's. Instruction cache control is written
 * twice: invalidate, then enable.
 */
/* word CACR: 32 bits */
#define L1B_CACR_WIDTH                               32 /* inferred */
#define L1B_CACR_CENB_SHIFT                          31 /* inferred */
#define L1B_CACR_CENB_MASK                           0x1u
#define L1B_CACR_CENB_INPLACE                        (L1B_CACR_CENB_MASK << L1B_CACR_CENB_SHIFT)
#define L1B_CACR_CINV_SHIFT                          24 /* inferred */
#define L1B_CACR_CINV_MASK                           0x1u
#define L1B_CACR_CINV_INPLACE                        (L1B_CACR_CINV_MASK << L1B_CACR_CINV_SHIFT)

/*
 * Battery-backed clock: control byte at the top of the window; BCD time bytes follow it.
 */
/* word CLOCK: 32 bits */
#define L1B_CLOCK_WIDTH                              32 /* established */
#define L1B_CLOCK_WRITE_ENABLE_SHIFT                 7 /* established */
#define L1B_CLOCK_WRITE_ENABLE_MASK                  0x1u
#define L1B_CLOCK_WRITE_ENABLE_INPLACE               (L1B_CLOCK_WRITE_ENABLE_MASK << L1B_CLOCK_WRITE_ENABLE_SHIFT)
#define L1B_CLOCK_READ_LATCH_SHIFT                   6 /* established */
#define L1B_CLOCK_READ_LATCH_MASK                    0x1u
#define L1B_CLOCK_READ_LATCH_INPLACE                 (L1B_CLOCK_READ_LATCH_MASK << L1B_CLOCK_READ_LATCH_SHIFT)
#define L1B_CLOCK_CENTURY_TENS_SHIFT                 4 /* established */
#define L1B_CLOCK_CENTURY_TENS_MASK                  0x3u
#define L1B_CLOCK_CENTURY_TENS_INPLACE               (L1B_CLOCK_CENTURY_TENS_MASK << L1B_CLOCK_CENTURY_TENS_SHIFT)
#define L1B_CLOCK_CENTURY_HUNDREDS_SHIFT             0 /* established */
#define L1B_CLOCK_CENTURY_HUNDREDS_MASK              0xfu
#define L1B_CLOCK_CENTURY_HUNDREDS_INPLACE           (L1B_CLOCK_CENTURY_HUNDREDS_MASK << L1B_CLOCK_CENTURY_HUNDREDS_SHIFT)

/*
 * Processor-module control expanders, wire addresses 0x42 and 0x44 (7-bit 0x21 and 0x22).
 */
/* word CNTL1: 32 bits */
#define L1B_CNTL1_WIDTH                              32 /* established */
#define L1B_CNTL1_SOFT_RESET_SHIFT                   7 /* established */
#define L1B_CNTL1_SOFT_RESET_MASK                    0x1u
#define L1B_CNTL1_SOFT_RESET_INPLACE                 (L1B_CNTL1_SOFT_RESET_MASK << L1B_CNTL1_SOFT_RESET_SHIFT)
#define L1B_CNTL1_NMI_SHIFT                          6 /* established */
#define L1B_CNTL1_NMI_MASK                           0x1u
#define L1B_CNTL1_NMI_INPLACE                        (L1B_CNTL1_NMI_MASK << L1B_CNTL1_NMI_SHIFT)
#define L1B_CNTL1_RESET_B_SHIFT                      1 /* established */
#define L1B_CNTL1_RESET_B_MASK                       0x1u
#define L1B_CNTL1_RESET_B_INPLACE                    (L1B_CNTL1_RESET_B_MASK << L1B_CNTL1_RESET_B_SHIFT)
#define L1B_CNTL1_RESET_A_SHIFT                      0 /* established */
#define L1B_CNTL1_RESET_A_MASK                       0x1u
#define L1B_CNTL1_RESET_A_INPLACE                    (L1B_CNTL1_RESET_A_MASK << L1B_CNTL1_RESET_A_SHIFT)

/* word CNTL2: 32 bits */
#define L1B_CNTL2_WIDTH                              32 /* unknown */
#define L1B_CNTL2_STRAP_SHIFT                        3 /* unknown */
#define L1B_CNTL2_STRAP_MASK                         0x1fu
#define L1B_CNTL2_STRAP_INPLACE                      (L1B_CNTL2_STRAP_MASK << L1B_CNTL2_STRAP_SHIFT)
#define L1B_CNTL2_RESET_C_SHIFT                      0 /* established */
#define L1B_CNTL2_RESET_C_MASK                       0x1u
#define L1B_CNTL2_RESET_C_INPLACE                    (L1B_CNTL2_RESET_C_MASK << L1B_CNTL2_RESET_C_SHIFT)

/*
 * Front-panel expander: 7-bit address 0x20, low nibble input.
 */
/* word DISPLAY: 32 bits */
#define L1B_DISPLAY_WIDTH                            32 /* established */
#define L1B_DISPLAY_LED_6_SHIFT                      6 /* established */
#define L1B_DISPLAY_LED_6_MASK                       0x1u
#define L1B_DISPLAY_LED_6_INPLACE                    (L1B_DISPLAY_LED_6_MASK << L1B_DISPLAY_LED_6_SHIFT)
#define L1B_DISPLAY_LED_5_SHIFT                      5 /* established */
#define L1B_DISPLAY_LED_5_MASK                       0x1u
#define L1B_DISPLAY_LED_5_INPLACE                    (L1B_DISPLAY_LED_5_MASK << L1B_DISPLAY_LED_5_SHIFT)
#define L1B_DISPLAY_LED_4_SHIFT                      4 /* established */
#define L1B_DISPLAY_LED_4_MASK                       0x1u
#define L1B_DISPLAY_LED_4_INPLACE                    (L1B_DISPLAY_LED_4_MASK << L1B_DISPLAY_LED_4_SHIFT)

/* word FLASH_STATUS: 32 bits */
#define L1B_FLASH_STATUS_WIDTH                       32 /* established */
#define L1B_FLASH_STATUS_TOGGLE_SHIFT                6 /* established */
#define L1B_FLASH_STATUS_TOGGLE_MASK                 0x1u
#define L1B_FLASH_STATUS_TOGGLE_INPLACE              (L1B_FLASH_STATUS_TOGGLE_MASK << L1B_FLASH_STATUS_TOGGLE_SHIFT)
#define L1B_FLASH_STATUS_TIMEOUT_SHIFT               5 /* established */
#define L1B_FLASH_STATUS_TIMEOUT_MASK                0x1u
#define L1B_FLASH_STATUS_TIMEOUT_INPLACE             (L1B_FLASH_STATUS_TIMEOUT_MASK << L1B_FLASH_STATUS_TIMEOUT_SHIFT)

/*
 * Processor-module supply-good expanders: one per module, at the wire address of the first
 * control expander but on its own bus segment (segment 4 for module 0, segment 5 for module 1).
 * Reset hold sets both bits of the field; release clears both. The mask is not applied to
 * the written byte as an input mask.
 */
/* word PIMM_VCC: 32 bits */
#define L1B_PIMM_VCC_WIDTH                           32 /* established */
#define L1B_PIMM_VCC_OK_SHIFT                        6 /* established */
#define L1B_PIMM_VCC_OK_MASK                         0x3u
#define L1B_PIMM_VCC_OK_INPLACE                      (L1B_PIMM_VCC_OK_MASK << L1B_PIMM_VCC_OK_SHIFT)

/*
 * R-brick router expanders.
 */
/* word ROUTER: 32 bits */
#define L1B_ROUTER_WIDTH                             32 /* established */
#define L1B_ROUTER_ROUTER_TYPE_SHIFT                 4 /* established */
#define L1B_ROUTER_ROUTER_TYPE_MASK                  0xfu
#define L1B_ROUTER_ROUTER_TYPE_INPLACE               (L1B_ROUTER_ROUTER_TYPE_MASK << L1B_ROUTER_ROUTER_TYPE_SHIFT)

/*
 * Router packet destination word, as the L2 router and the L1 target list both build and
 * read it for brick-directed packets (any destination form other than the L2 form and the
 * node form): the rack takes 10 bits and the slot 6.
 */
/* word ROUTER_DEST: 32 bits */
#define L1B_ROUTER_DEST_WIDTH                        32 /* established */
#define L1B_ROUTER_DEST_RACK_SHIFT                   18 /* established */
#define L1B_ROUTER_DEST_RACK_MASK                    0x3ffu
#define L1B_ROUTER_DEST_RACK_INPLACE                 (L1B_ROUTER_DEST_RACK_MASK << L1B_ROUTER_DEST_RACK_SHIFT)
#define L1B_ROUTER_DEST_SLOT_SHIFT                   12 /* established */
#define L1B_ROUTER_DEST_SLOT_MASK                    0x3fu
#define L1B_ROUTER_DEST_SLOT_INPLACE                 (L1B_ROUTER_DEST_SLOT_MASK << L1B_ROUTER_DEST_SLOT_SHIFT)

/* word RPOWER: 32 bits */
#define L1B_RPOWER_WIDTH                             32 /* established */
#define L1B_RPOWER_ROUTER_POWER_SHIFT                3 /* established */
#define L1B_RPOWER_ROUTER_POWER_MASK                 0x1u
#define L1B_RPOWER_ROUTER_POWER_INPLACE              (L1B_RPOWER_ROUTER_POWER_MASK << L1B_RPOWER_ROUTER_POWER_SHIFT)

/*
 * I-brick bridge control expander: expander index 5, 7-bit address 0x24.
 */
/* word XBRIDGE: 32 bits */
#define L1B_XBRIDGE_WIDTH                            32 /* established */
#define L1B_XBRIDGE_CLOCK_SELECT_SHIFT               5 /* established */
#define L1B_XBRIDGE_CLOCK_SELECT_MASK                0x1u
#define L1B_XBRIDGE_CLOCK_SELECT_INPLACE             (L1B_XBRIDGE_CLOCK_SELECT_MASK << L1B_XBRIDGE_CLOCK_SELECT_SHIFT)
#define L1B_XBRIDGE_HOST_ENABLE_B_SHIFT              4 /* established */
#define L1B_XBRIDGE_HOST_ENABLE_B_MASK               0x1u
#define L1B_XBRIDGE_HOST_ENABLE_B_INPLACE            (L1B_XBRIDGE_HOST_ENABLE_B_MASK << L1B_XBRIDGE_HOST_ENABLE_B_SHIFT)
#define L1B_XBRIDGE_RESET_3_SHIFT                    3 /* established */
#define L1B_XBRIDGE_RESET_3_MASK                     0x1u
#define L1B_XBRIDGE_RESET_3_INPLACE                  (L1B_XBRIDGE_RESET_3_MASK << L1B_XBRIDGE_RESET_3_SHIFT)
#define L1B_XBRIDGE_HOST_ENABLE_A_SHIFT              2 /* established */
#define L1B_XBRIDGE_HOST_ENABLE_A_MASK               0x1u
#define L1B_XBRIDGE_HOST_ENABLE_A_INPLACE            (L1B_XBRIDGE_HOST_ENABLE_A_MASK << L1B_XBRIDGE_HOST_ENABLE_A_SHIFT)
#define L1B_XBRIDGE_RESET_1_SHIFT                    1 /* established */
#define L1B_XBRIDGE_RESET_1_MASK                     0x1u
#define L1B_XBRIDGE_RESET_1_INPLACE                  (L1B_XBRIDGE_RESET_1_MASK << L1B_XBRIDGE_RESET_1_SHIFT)
#define L1B_XBRIDGE_RESET_0_SHIFT                    0 /* established */
#define L1B_XBRIDGE_RESET_0_MASK                     0x1u
#define L1B_XBRIDGE_RESET_0_INPLACE                  (L1B_XBRIDGE_RESET_0_MASK << L1B_XBRIDGE_RESET_0_SHIFT)

#define L1B_BANK_RECORD_WORD0                        0x40030u /* established */
#define L1B_BUS_SELECT_ADDR7                         0x27u /* established */
#define L1B_BUS_SELECT_SEG1                          0xffu /* established */
#define L1B_BUS_SELECT_SEG13_LAST                    0xf7u /* established */
#define L1B_BUS_SELECT_SEG2                          0x78u /* established */
#define L1B_BUS_SELECT_SEG3                          0xb8u /* established */
#define L1B_BUS_SELECT_SEG4                          0xd8u /* established */
#define L1B_BUS_SELECT_SEG5                          0xe8u /* established */
#define L1B_BUS_SELECT_SEG6_FIRST                    0xf0u /* established */
#define L1B_CACR_FIRST_WRITE                         0x1000000u /* established */
#define L1B_CACR_SECOND_WRITE                        0x80000000u /* established */
#define L1B_CLOCK_CONTROL_INDEX                      0x7f8u /* established */
#define L1B_CNTL1_INPUT_MASK_CBRICK                  0x0u /* established */
#define L1B_CNTL1_INPUT_MASK_O300                    0x20u /* established */
#define L1B_CNTL1_START_VALUE                        0x72u /* established */
#define L1B_CNTL1_WIRE                               0x42u /* established */
#define L1B_CNTL2_INPUT_MASK_CBRICK                  0x0u /* established */
#define L1B_CNTL2_INPUT_MASK_O300                    0x2u /* established */
#define L1B_CNTL2_START_VALUE                        0x8au /* established */
#define L1B_CNTL2_WIRE                               0x44u /* established */
#define L1B_DISPLAY_CHANGE_MASK                      0x7u /* established */
#define L1B_DISPLAY_CONTROL_ADDR7                    0x20u /* established */
#define L1B_DISPLAY_INPUT_MASK                       0xfu /* established */
#define L1B_EEPROM_CX_NODE_ADDR7                     0x56u /* established */
#define L1B_EEPROM_CX_RECORD_COUNT                   0x26u /* established */
#define L1B_EEPROM_CX_SPD_CAPACITY                   0xa0u /* established */
#define L1B_EEPROM_DIMM_ADDR7                        0x50u /* established */
#define L1B_EEPROM_DIMM_CAPACITY                     0x100u /* established */
#define L1B_EEPROM_KIND_BOARD                        0x1u /* established */
#define L1B_EEPROM_KIND_MEMORY                       0x2u /* established */
#define L1B_EEPROM_LOGIC_ADDR7                       0x54u /* established */
#define L1B_EEPROM_LOGIC_CAPACITY                    0x200u /* established */
#define L1B_EEPROM_NODE_ADDR7                        0x52u /* established */
#define L1B_EEPROM_NODE_CAPACITY                     0x200u /* established */
#define L1B_EEPROM_PIMM_ADDR7                        0x54u /* established */
#define L1B_EEPROM_PIMM_CAPACITY                     0x200u /* established */
#define L1B_EEPROM_POWER_ADDR7                       0x52u /* established */
#define L1B_EEPROM_POWER_CAPACITY                    0x200u /* established */
#define L1B_FLASH_CMD_AUTOSELECT                     0x90u /* established */
#define L1B_FLASH_CMD_ERASE_RESUME                   0x30u /* established */
#define L1B_FLASH_CMD_ERASE_SETUP                    0x80u /* established */
#define L1B_FLASH_CMD_ERASE_SUSPEND                  0xb0u /* established */
#define L1B_FLASH_CMD_PROGRAM                        0xa0u /* established */
#define L1B_FLASH_CMD_RESET                          0xf0u /* established */
#define L1B_FLASH_CMD_SECTOR_ERASE                   0x30u /* established */
#define L1B_FLASH_HELPER_ERR_PROGRAM                 0xfffffaebu /* established */
#define L1B_FLASH_HELPER_ERR_RANGE                   0xfffffaecu /* established */
#define L1B_FLASH_HELPER_ERR_VERIFY                  0xfffffaeau /* established */
#define L1B_FLASH_UNLOCK_ADDR_1                      0xaaau /* established */
#define L1B_FLASH_UNLOCK_ADDR_2                      0x554u /* established */
#define L1B_FLASH_UNLOCK_DATA_1                      0xaau /* established */
#define L1B_FLASH_UNLOCK_DATA_2                      0x55u /* established */
#define L1B_IBRICK_FAN_SPEED_ADDR7                   0x39u /* established */
#define L1B_IBRICK_PCI_BUS1_SLOT1_ADDR7              0x25u /* established */
#define L1B_IBRICK_PCI_BUS1_SLOT2_ADDR7              0x26u /* established */
#define L1B_IBRICK_PCI_BUS1_SLOT3_ADDR7              0x27u /* established */
#define L1B_IBRICK_PCI_BUS1_SLOT4_ADDR7              0x21u /* established */
#define L1B_IBRICK_PCI_BUS2_SLOT1_ADDR7              0x3eu /* established */
#define L1B_IBRICK_PCI_BUS2_SLOT2_ADDR7              0x3fu /* established */
#define L1B_IBRICK_PORT_CONTROL_ADDR7                0x23u /* established */
#define L1B_IBRICK_POWER_ENABLE_ADDR7                0x38u /* established */
#define L1B_IBRICK_POWER_STATUS1_ADDR7               0x3au /* established */
#define L1B_IBRICK_POWER_STATUS2_ADDR7               0x3bu /* established */
#define L1B_IBRICK_SUPPLY_12V_BIAS_ENABLE            0x4u /* established */
#define L1B_IBRICK_SUPPLY_12V_ENABLE                 0x40u /* established */
#define L1B_IBRICK_SUPPLY_2V5_ENABLE                 0x8u /* established */
#define L1B_IBRICK_SUPPLY_3V3_ENABLE                 0x10u /* established */
#define L1B_IBRICK_SUPPLY_48V_ENABLE                 0x1u /* established */
#define L1B_IBRICK_SUPPLY_5V_ENABLE                  0x20u /* established */
#define L1B_IBRICK_SUPPLY_M12V_ENABLE                0x80u /* established */
#define L1B_MBAR_START_VALUE                         0x10000001u /* established */
#define L1B_MONITOR_ADDR7_0                          0x2cu /* established */
#define L1B_MONITOR_ADDR7_1                          0x2du /* established */
#define L1B_MONITOR_ADDR7_2                          0x2eu /* established */
#define L1B_MONITOR_ADDR7_3                          0x2fu /* established */
#define L1B_MONITOR_FLAG_NO_START                    0x1u /* established */
#define L1B_MONITOR_INIT_REG_40                      0x80u /* established */
#define L1B_MONITOR_INIT_REG_43                      0xdfu /* established */
#define L1B_MONITOR_INIT_REG_44                      0x13u /* established */
#define L1B_MONITOR_INIT_REG_47_ADDR7_0              0xa0u /* established */
#define L1B_MONITOR_INIT_REG_47_ADDR7_1              0x20u /* established */
#define L1B_MONITOR_INIT_REG_47_OTHER                0x0u /* established */
#define L1B_MOVEC_CODE_CACR                          0x2u /* established */
#define L1B_MOVEC_CODE_MBAR                          0xc0fu /* established */
#define L1B_MOVEC_CODE_RAMBAR                        0xc04u /* established */
#define L1B_MOVEC_CODE_VBR                           0x801u /* established */
#define L1B_MOVEC_RN_RC_OPCODE                       0x4e7bu /* established */
#define L1B_PCI_EXPANSION_TYPE                       0x25u /* established */
#define L1B_PIMM_VCC_OK_WIRE                         0x42u /* established */
#define L1B_QUART_CHANNEL0                           0x0u /* established */
#define L1B_QUART_CHANNEL1                           0x8u /* established */
#define L1B_QUART_CHANNEL2                           0x10u /* established */
#define L1B_QUART_CHANNEL3                           0x18u /* established */
#define L1B_QUART_FIFO_CONTROL_TRIGGER_A             0xfu /* established */
#define L1B_QUART_FIFO_CONTROL_TRIGGER_B             0x8fu /* established */
#define L1B_QUART_LINE_CONTROL_8E1                   0x1bu /* established */
#define L1B_QUART_LINE_CONTROL_8N1                   0x3u /* established */
#define L1B_QUART_LINE_CONTROL_8O1                   0xbu /* established */
#define L1B_QUART_MODEM_CONTROL_DTR_RTS              0x3u /* established */
#define L1B_QUART_MODEM_CONTROL_LOOPBACK             0x10u /* established */
#define L1B_QUART_REG_FIFO_CONTROL                   0x2u /* established */
#define L1B_QUART_REG_LINE_CONTROL                   0x3u /* established */
#define L1B_QUART_REG_MODEM_CONTROL                  0x4u /* established */
#define L1B_RACK_MAX                                 0x3ffu /* established */
#define L1B_RACK_WIDTH_BITS                          0xau /* established */
#define L1B_RAMBAR_START_VALUE                       0x30100001u /* established */
#define L1B_RBRICK_PORT_BC_CONTROL_ADDR7             0x25u /* established */
#define L1B_RBRICK_PORT_DE_CONTROL_ADDR7             0x26u /* established */
#define L1B_RBRICK_PORT_FG_CONTROL_ADDR7             0x27u /* established */
#define L1B_RBRICK_PORT_HA_CONTROL_ADDR7             0x24u /* established */
#define L1B_RBRICK_POWER_CONTROL_ADDR7               0x39u /* established */
#define L1B_RBRICK_SUPPLY_2V5_ENABLE                 0x4u /* established */
#define L1B_RBRICK_SUPPLY_48V_ENABLE                 0x1u /* established */
#define L1B_RBRICK_WIDGET_PRESENT_ADDR7              0x22u /* established */
#define L1B_RETAINED_RECORD_BASE                     0x30101990u /* established */
#define L1B_RETAINED_RECORD_MARKER                   0xa1b2c3d4u /* established */
#define L1B_RETAINED_RECORD_SIZE                     0x4d8u /* established */
#define L1B_ROUTER_CONTROL_ADDR7                     0x23u /* established */
#define L1B_ROUTER_HOLD_VALUE                        0x1u /* established */
#define L1B_ROUTER_RESET_BITS_MASK                   0xdu /* established */
#define L1B_ROUTER_START_VALUE                       0xf1u /* established */
#define L1B_TIMER1_LEVEL                             0x5u /* established */
#define L1B_TIMER1_VECTOR                            0x1du /* established */
#define L1B_UART_BAUD                                0x4b00u /* established */
#define L1B_UART_LEVEL                               0x6u /* established */
#define L1B_UART_VECTOR                              0x42u /* established */
#define L1B_VBR_START_VALUE                          0x30100000u /* established */
#define L1B_XBRIDGE_ADDR7                            0x24u /* established */
#define L1B_XBRIDGE_START_VALUE                      0xd4u /* established */

#endif /* L1B_GEN_H */
