/* generated from formats/l1_pkg5_requests/format.toml by defs-gen - do not edit, change the source */
/* console request codes of ColdFire L1 package 5 */
#ifndef L1P5_GEN_H
#define L1P5_GEN_H

#include <stdint.h>


/* word CMD: 32 bits */
#define L1P5_CMD_WIDTH                               32 /* established */
#define L1P5_CMD_CODE_SHIFT                          0 /* established */
#define L1P5_CMD_CODE_MASK                           0xffffu
#define L1P5_CMD_CODE_INPLACE                        (L1P5_CMD_CODE_MASK << L1P5_CMD_CODE_SHIFT)
#define L1P5_CMD_CODE_GENERAL_REQ_RACK_BAY           2 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_RACK_BAY_TYPE      3 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_BRICK_SERIAL       4 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_EEPROM_READ        6 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_EEPROM_FORMAT      7 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_SYSTEM_SERIAL      8 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_PORT_SPEEDS        10 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_ATTACHED_PORTS_1_TO_4 12 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_ATTACHED_PORTS_5_AND_10 13 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_REQUESTING_PORT    14 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_PROCESSOR_MASKS    16 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_SCAN_RECORD        17 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_TYPE_CHAR          19 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_WRITE_DEBUG_SWITCHES 4097 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_DISPLAY_LINE_1     4100 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_DISPLAY_LINE_2     4101 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_PARTITION_SET      4102 /* inferred */
#define L1P5_CMD_CODE_GENERAL_REQ_EVENT_SUBCHANNEL_SET 4103 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_TWO_BIT_SETTING_SET 4104 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_PANEL_CONTROL      4105 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_EVENT_SUBCHANNEL_CLEAR 4106 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_PROM_VERSION_REPORT 4107 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_PER_PROCESSOR_SETTING 4108 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_SYSTEM_RESET       8193 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_PCI_OPERATION_A    8194 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_PCI_OPERATION_B    8195 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_PCI_OPERATION_C    8196 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_POWER_DOWN         8197 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_LIFECYCLE_THIRD_ACTION 8198 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_SET_CLOCK          8205 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_GET_CLOCK          8206 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_HOOK_3001          12289 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_FRU_3002           12290 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_FRU_3003           12291 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_FRU_3004           12292 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_INVENTORY          16384 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_WAR_FLAG           16385 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_CONTROLLER_REBOOT  16386 /* established */
#define L1P5_CMD_CODE_GENERAL_REQ_DIMM_TEXT          16388 /* established */

#endif /* L1P5_GEN_H */
