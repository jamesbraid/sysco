/*
 * L1P1_DECODE.h - console request codes of ColdFire L1 package 1
 *
 * Console request codes of one ColdFire L1 firmware package.
 * Codes are unique within the package.
 *
 * Each ENUM line is one request code. The comment above it gives the
 * argument shape as read in the image. Grades follow the source tables.
 */
#ifndef L1P1_DECODE_H
#define L1P1_DECODE_H

/* FIELD: CMD.code 15:0 established */
#define L1P1_CMD_CODE_SHIFT 0
#define L1P1_CMD_CODE_MASK 0xffffu

/* argument shape: any code with one text argument; reply is the status and the output lines as text */
/* ENUM: CMD.code 0x0000 command_task_loop established */
#define L1P1_CMD_CODE_COMMAND_TASK_LOOP 0x0000

/* argument shape: reply 0 then one int, the 16-bit non-volatile switches */
/* ENUM: CMD.code 0x0001 general_req_read_debug_switches established */
#define L1P1_CMD_CODE_GENERAL_REQ_READ_DEBUG_SWITCHES 0x0001

/* argument shape: reply 0 then int rack<<6|bay from non-volatile settings */
/* ENUM: CMD.code 0x0002 general_req_rack_bay established */
#define L1P1_CMD_CODE_GENERAL_REQ_RACK_BAY 0x0002

/* argument shape: same entry as 0x0002; adds the stored brick-type character (0x34 for ip45) */
/* ENUM: CMD.code 0x0003 general_req_rack_bay_type established */
#define L1P1_CMD_CODE_GENERAL_REQ_RACK_BAY_TYPE 0x0003

/* argument shape: reply 0 then text; minus 104 when absent */
/* ENUM: CMD.code 0x0004 general_req_brick_serial established */
#define L1P1_CMD_CODE_GENERAL_REQ_BRICK_SERIAL 0x0004

/* argument shape: reply 0 then three ints parsed from the version string */
/* ENUM: CMD.code 0x0005 general_req_firmware_revision established */
#define L1P1_CMD_CODE_GENERAL_REQ_FIRMWARE_REVISION 0x0005

/* argument shape: status 0, bytes left after the window, then a binary field; areas 0 to 4; minus 104, 105, 107 conditions */
/* ENUM: CMD.code 0x0006 general_req_eeprom_read established */
#define L1P1_CMD_CODE_GENERAL_REQ_EEPROM_READ 0x0006

/* argument shape: record kind, then eight more ints for kind 1 and two for kind 2 */
/* ENUM: CMD.code 0x0007 general_req_eeprom_format established */
#define L1P1_CMD_CODE_GENERAL_REQ_EEPROM_FORMAT 0x0007

/* argument shape: reply 0 then text; minus 104 when empty */
/* ENUM: CMD.code 0x0008 general_req_system_serial established */
#define L1P1_CMD_CODE_GENERAL_REQ_SYSTEM_SERIAL 0x0008

/* argument shape: reply 0 then int */
/* ENUM: CMD.code 0x0009 general_req_partition_get established */
#define L1P1_CMD_CODE_GENERAL_REQ_PARTITION_GET 0x0009

/* argument shape: reply 0 then two ints from per-port speed settings */
/* ENUM: CMD.code 0x000a general_req_port_speeds established */
#define L1P1_CMD_CODE_GENERAL_REQ_PORT_SPEEDS 0x000a

/* argument shape: reply 0 then int; meaning of the field unknown */
/* ENUM: CMD.code 0x000b general_req_two_bit_setting established */
#define L1P1_CMD_CODE_GENERAL_REQ_TWO_BIT_SETTING 0x000b

/* argument shape: reply 0 then four (rack<<6|bay, type) pairs */
/* ENUM: CMD.code 0x000c general_req_attached_ports_1_to_4 established */
#define L1P1_CMD_CODE_GENERAL_REQ_ATTACHED_PORTS_1_TO_4 0x000c

/* argument shape: reply 0 then two (rack<<6|bay, type) pairs */
/* ENUM: CMD.code 0x000d general_req_attached_ports_5_and_10 established */
#define L1P1_CMD_CODE_GENERAL_REQ_ATTACHED_PORTS_5_AND_10 0x000d

/* argument shape: reply 0 then int: the link index the request arrived on */
/* ENUM: CMD.code 0x000e general_req_requesting_port established */
#define L1P1_CMD_CODE_GENERAL_REQ_REQUESTING_PORT 0x000e

/* argument shape: reply 0 then two ints from the lifecycle module */
/* ENUM: CMD.code 0x000f general_req_last_event_reasons established */
#define L1P1_CMD_CODE_GENERAL_REQ_LAST_EVENT_REASONS 0x000f

/* argument shape: reply 0, word count, processor count, mask pairs; minus 108 when none */
/* ENUM: CMD.code 0x0010 general_req_processor_masks established */
#define L1P1_CMD_CODE_GENERAL_REQ_PROCESSOR_MASKS 0x0010

/* argument shape: reply 0 then a 64-byte binary field, or minus 106 and a status */
/* ENUM: CMD.code 0x0011 general_req_scan_record established */
#define L1P1_CMD_CODE_GENERAL_REQ_SCAN_RECORD 0x0011

/* argument shape: first int at most 4; reply lists the flash images */
/* ENUM: CMD.code 0x0072 flash_req_status inferred */
#define L1P1_CMD_CODE_FLASH_REQ_STATUS 0x0072

/* argument shape: flash operation, not read */
/* ENUM: CMD.code 0x0073 flash_req_0x73 inferred */
#define L1P1_CMD_CODE_FLASH_REQ_0X73 0x0073

/* argument shape: flash operation, not read */
/* ENUM: CMD.code 0x0074 flash_req_0x74 inferred */
#define L1P1_CMD_CODE_FLASH_REQ_0X74 0x0074

/* argument shape: exactly one argument */
/* ENUM: CMD.code 0x0092 power_bay_flash_req_0x92 inferred */
#define L1P1_CMD_CODE_POWER_BAY_FLASH_REQ_0X92 0x0092

/* argument shape: exactly one argument */
/* ENUM: CMD.code 0x0093 power_bay_flash_req_0x93 inferred */
#define L1P1_CMD_CODE_POWER_BAY_FLASH_REQ_0X93 0x0093

/* argument shape: exactly one argument */
/* ENUM: CMD.code 0x0094 power_bay_flash_req_0x94 inferred */
#define L1P1_CMD_CODE_POWER_BAY_FLASH_REQ_0X94 0x0094

/* argument shape: one int; writes the non-volatile switches; reply 0 */
/* ENUM: CMD.code 0x1001 general_req_write_debug_switches established */
#define L1P1_CMD_CODE_GENERAL_REQ_WRITE_DEBUG_SWITCHES 0x1001

/* argument shape: one int; sets the active console subchannel; reply 0 */
/* ENUM: CMD.code 0x1002 general_req_console_subchannel established */
#define L1P1_CMD_CODE_GENERAL_REQ_CONSOLE_SUBCHANNEL 0x1002

/* argument shape: records the local module as console owner; reply 0 */
/* ENUM: CMD.code 0x1003 general_req_console_volunteer established */
#define L1P1_CMD_CODE_GENERAL_REQ_CONSOLE_VOLUNTEER 0x1003

/* argument shape: one text; writes display line 2; minus 102 if not text */
/* ENUM: CMD.code 0x1004 general_req_display_line_1 established */
#define L1P1_CMD_CODE_GENERAL_REQ_DISPLAY_LINE_1 0x1004

/* argument shape: same entry as 0x1004 */
/* ENUM: CMD.code 0x1005 general_req_display_line_2 established */
#define L1P1_CMD_CODE_GENERAL_REQ_DISPLAY_LINE_2 0x1005

/* argument shape: one int; non-volatile write; reply 0 */
/* ENUM: CMD.code 0x1006 general_req_partition_set established */
#define L1P1_CMD_CODE_GENERAL_REQ_PARTITION_SET 0x1006

/* argument shape: one int; persisted in non-volatile memory; reply 0 */
/* ENUM: CMD.code 0x1007 general_req_event_subchannel_set established */
#define L1P1_CMD_CODE_GENERAL_REQ_EVENT_SUBCHANNEL_SET 0x1007

/* argument shape: one int; writes the field read by 0x000b; reply 0 */
/* ENUM: CMD.code 0x1008 general_req_two_bit_setting_set established */
#define L1P1_CMD_CODE_GENERAL_REQ_TWO_BIT_SETTING_SET 0x1008

/* argument shape: two ints; selector 0 to 2 else minus 105; reply 0 */
/* ENUM: CMD.code 0x1009 general_req_panel_control established */
#define L1P1_CMD_CODE_GENERAL_REQ_PANEL_CONTROL 0x1009

/* argument shape: no reply; clears and unpersists the event subchannel */
/* ENUM: CMD.code 0x100a general_req_event_subchannel_clear established */
#define L1P1_CMD_CODE_GENERAL_REQ_EVENT_SUBCHANNEL_CLEAR 0x100a

/* argument shape: exactly three ints; no reply on success */
/* ENUM: CMD.code 0x100b general_req_prom_version_report established */
#define L1P1_CMD_CODE_GENERAL_REQ_PROM_VERSION_REPORT 0x100b

/* argument shape: exactly two ints; no reply on success */
/* ENUM: CMD.code 0x100c general_req_per_processor_setting established */
#define L1P1_CMD_CODE_GENERAL_REQ_PER_PROCESSOR_SETTING 0x100c

/* argument shape: no reply on success; minus 106 and status on failure */
/* ENUM: CMD.code 0x2001 general_req_system_reset established */
#define L1P1_CMD_CODE_GENERAL_REQ_SYSTEM_RESET 0x2001

/* argument shape: two ints bus and slot; board hook; no reply on success */
/* ENUM: CMD.code 0x2002 general_req_pci_operation_a established */
#define L1P1_CMD_CODE_GENERAL_REQ_PCI_OPERATION_A 0x2002

/* argument shape: two ints bus and slot; board hook; no reply on success */
/* ENUM: CMD.code 0x2003 general_req_pci_operation_b established */
#define L1P1_CMD_CODE_GENERAL_REQ_PCI_OPERATION_B 0x2003

/* argument shape: two ints bus and slot; board hook; no reply on success */
/* ENUM: CMD.code 0x2004 general_req_pci_operation_c established */
#define L1P1_CMD_CODE_GENERAL_REQ_PCI_OPERATION_C 0x2004

/* argument shape: no reply on success */
/* ENUM: CMD.code 0x2005 general_req_power_down established */
#define L1P1_CMD_CODE_GENERAL_REQ_POWER_DOWN 0x2005

/* argument shape: no reply on success; which action is not established */
/* ENUM: CMD.code 0x2006 general_req_lifecycle_third_action established */
#define L1P1_CMD_CODE_GENERAL_REQ_LIFECYCLE_THIRD_ACTION 0x2006

/* argument shape: no arguments; minus 108 on an Origin 300 */
/* ENUM: CMD.code 0x2007 general_req_expander_read established */
#define L1P1_CMD_CODE_GENERAL_REQ_EXPANDER_READ 0x2007

/* argument shape: exactly two ints; minus 108 on an Origin 300 */
/* ENUM: CMD.code 0x2008 general_req_expander_write established */
#define L1P1_CMD_CODE_GENERAL_REQ_EXPANDER_WRITE 0x2008

/* argument shape: reply 0 then one text field per inventory record; minus 104 when empty */
/* ENUM: CMD.code 0x4000 general_req_inventory established */
#define L1P1_CMD_CODE_GENERAL_REQ_INVENTORY 0x4000

/* argument shape: no reply; logs and restarts the controller */
/* ENUM: CMD.code 0x4002 general_req_controller_reboot established */
#define L1P1_CMD_CODE_GENERAL_REQ_CONTROLLER_REBOOT 0x4002

#endif
