/*
 * L1P3_DECODE.h - console request codes of ColdFire L1 package 3
 *
 * Console request codes of one ColdFire L1 firmware package.
 * Codes are unique within the package.
 *
 * Each ENUM line is one request code. The comment above it gives the
 * argument shape as read in the image. Grades follow the source tables.
 */
#ifndef L1P3_DECODE_H
#define L1P3_DECODE_H

/* FIELD: CMD.code 15:0 established */
#define L1P3_CMD_CODE_SHIFT 0
#define L1P3_CMD_CODE_MASK 0xffffu

/* argument shape: reply 0 then rack<<6|bay; type 0x3d reports 0x23 when the slot low byte is zero; no type 0x38 case */
/* ENUM: CMD.code 0x0002 general_req_rack_bay established */
#define L1P3_CMD_CODE_GENERAL_REQ_RACK_BAY 0x0002

/* argument shape: same entry as 0x0002; adds the reported type character */
/* ENUM: CMD.code 0x0003 general_req_rack_bay_type established */
#define L1P3_CMD_CODE_GENERAL_REQ_RACK_BAY_TYPE 0x0003

/* argument shape: reply 0 then text; type 0x40 uses the per-node serial routine, others the brick serial; minus 104 when null */
/* ENUM: CMD.code 0x0004 general_req_brick_serial established */
#define L1P3_CMD_CODE_GENERAL_REQ_BRICK_SERIAL 0x0004

/* argument shape: five or more words; record number from a local type map instead of a hook word; status 0, bytes left, then a binary field */
/* ENUM: CMD.code 0x0006 general_req_eeprom_read established */
#define L1P3_CMD_CODE_GENERAL_REQ_EEPROM_READ 0x0006

/* argument shape: two or more words; record number from a local type map instead of a hook word; record kind, then eight more ints for kind 1 and two for kind 2 */
/* ENUM: CMD.code 0x0007 general_req_eeprom_format established */
#define L1P3_CMD_CODE_GENERAL_REQ_EEPROM_FORMAT 0x0007

/* argument shape: descriptor word at a fixed location else setting 0xd; reply 0 then text; minus 104 when empty */
/* ENUM: CMD.code 0x0008 general_req_system_serial established */
#define L1P3_CMD_CODE_GENERAL_REQ_SYSTEM_SERIAL 0x0008

/* argument shape: type table sends types 03456789@ABCFLMOQ| to the port-record path, others read setting 0x2a; reply 0 then two ints */
/* ENUM: CMD.code 0x000a general_req_port_speeds established */
#define L1P3_CMD_CODE_GENERAL_REQ_PORT_SPEEDS 0x000a

/* argument shape: reply 0 then four (rack<<6|bay, type) pairs; same stream as the Origin 300 handler */
/* ENUM: CMD.code 0x000c general_req_attached_ports_1_to_4 established */
#define L1P3_CMD_CODE_GENERAL_REQ_ATTACHED_PORTS_1_TO_4 0x000c

/* argument shape: reply 0 then two pairs; same stream as the Origin 300 handler */
/* ENUM: CMD.code 0x000d general_req_attached_ports_5_and_10 established */
#define L1P3_CMD_CODE_GENERAL_REQ_ATTACHED_PORTS_5_AND_10 0x000d

/* argument shape: reply 0 then the requesting link index; same stream as the Origin 300 handler */
/* ENUM: CMD.code 0x000e general_req_requesting_port established */
#define L1P3_CMD_CODE_GENERAL_REQ_REQUESTING_PORT 0x000e

/* argument shape: reply 0 then two lifecycle words; same stream as the Origin 300 handler */
/* ENUM: CMD.code 0x000f general_req_last_event_reasons established */
#define L1P3_CMD_CODE_GENERAL_REQ_LAST_EVENT_REASONS 0x000f

/* argument shape: mask words from descriptor a fixed offset and shift a fixed offset; reply 0, word count, processor count, mask pairs */
/* ENUM: CMD.code 0x0010 general_req_processor_masks established */
#define L1P3_CMD_CODE_GENERAL_REQ_PROCESSOR_MASKS 0x0010

/* argument shape: reply 0 then a 64-byte binary field or minus 106; relocation of the Origin 300 body */
/* ENUM: CMD.code 0x0011 general_req_scan_record established */
#define L1P3_CMD_CODE_GENERAL_REQ_SCAN_RECORD 0x0011

/* argument shape: reply 0, int 0, then the brick-type character as an int */
/* ENUM: CMD.code 0x0013 general_req_type_char established */
#define L1P3_CMD_CODE_GENERAL_REQ_TYPE_CHAR 0x0013

/* argument shape: one int; writes setting 3 through a different setting writer; reply 0 */
/* ENUM: CMD.code 0x1001 general_req_write_debug_switches established */
#define L1P3_CMD_CODE_GENERAL_REQ_WRITE_DEBUG_SWITCHES 0x1001

/* argument shape: one int; sets the console subchannel; same stream as the Origin 300 handler */
/* ENUM: CMD.code 0x1002 general_req_console_subchannel established */
#define L1P3_CMD_CODE_GENERAL_REQ_CONSOLE_SUBCHANNEL 0x1002

/* argument shape: records the local module as console owner; same stream as the Origin 300 handler */
/* ENUM: CMD.code 0x1003 general_req_console_volunteer established */
#define L1P3_CMD_CODE_GENERAL_REQ_CONSOLE_VOLUNTEER 0x1003

/* argument shape: one text; display write; minus 102 if not text; same stream as the Origin 300 handler */
/* ENUM: CMD.code 0x1004 general_req_display_line_1 established */
#define L1P3_CMD_CODE_GENERAL_REQ_DISPLAY_LINE_1 0x1004

/* argument shape: same entry as 0x1004 */
/* ENUM: CMD.code 0x1005 general_req_display_line_2 established */
#define L1P3_CMD_CODE_GENERAL_REQ_DISPLAY_LINE_2 0x1005

/* argument shape: one int; writes setting 0xe then updates the module record; reply 0 */
/* ENUM: CMD.code 0x1006 general_req_partition_set established */
#define L1P3_CMD_CODE_GENERAL_REQ_PARTITION_SET 0x1006

/* argument shape: one int; builds a tag, stores it at a fixed location of the state block and writes setting 0x13; reply 0 */
/* ENUM: CMD.code 0x1007 general_req_event_subchannel_set established */
#define L1P3_CMD_CODE_GENERAL_REQ_EVENT_SUBCHANNEL_SET 0x1007

/* argument shape: one int; writes setting 0x12; reply 0 */
/* ENUM: CMD.code 0x1008 general_req_two_bit_setting_set established */
#define L1P3_CMD_CODE_GENERAL_REQ_TWO_BIT_SETTING_SET 0x1008

/* argument shape: two ints; selector 0 to 2 else minus 105; reply 0; same stream as the Origin 300 handler */
/* ENUM: CMD.code 0x1009 general_req_panel_control established */
#define L1P3_CMD_CODE_GENERAL_REQ_PANEL_CONTROL 0x1009

/* argument shape: no reply; clears and unpersists the event subchannel */
/* ENUM: CMD.code 0x100a general_req_event_subchannel_clear established */
#define L1P3_CMD_CODE_GENERAL_REQ_EVENT_SUBCHANNEL_CLEAR 0x100a

/* argument shape: exactly three ints; stores the packed PROM version in the runtime slot and prints it; no reply on success */
/* ENUM: CMD.code 0x100b general_req_prom_version_report established */
#define L1P3_CMD_CODE_GENERAL_REQ_PROM_VERSION_REPORT 0x100b

/* argument shape: exactly two ints; per-processor setter; no reply on success */
/* ENUM: CMD.code 0x100c general_req_per_processor_setting established */
#define L1P3_CMD_CODE_GENERAL_REQ_PER_PROCESSOR_SETTING 0x100c

/* argument shape: no reply on success; reset with reason 3 */
/* ENUM: CMD.code 0x2001 general_req_system_reset established */
#define L1P3_CMD_CODE_GENERAL_REQ_SYSTEM_RESET 0x2001

/* argument shape: three or more words; hook at offset 8 of block a fixed address with a third zero argument; reply is the hook result */
/* ENUM: CMD.code 0x2002 general_req_pci_operation_a established */
#define L1P3_CMD_CODE_GENERAL_REQ_PCI_OPERATION_A 0x2002

/* argument shape: three or more words; hook at offset 0xc of block a fixed address; minus 106 on failure */
/* ENUM: CMD.code 0x2003 general_req_pci_operation_b established */
#define L1P3_CMD_CODE_GENERAL_REQ_PCI_OPERATION_B 0x2003

/* argument shape: three or more words; hook at offset 0x10 of block a fixed address; minus 106 on failure */
/* ENUM: CMD.code 0x2004 general_req_pci_operation_c established */
#define L1P3_CMD_CODE_GENERAL_REQ_PCI_OPERATION_C 0x2004

/* argument shape: no reply on success; power down with reason 3; minus 106 on failure */
/* ENUM: CMD.code 0x2005 general_req_power_down established */
#define L1P3_CMD_CODE_GENERAL_REQ_POWER_DOWN 0x2005

/* argument shape: no reply on success; minus 106 on failure and no integer reply */
/* ENUM: CMD.code 0x2006 general_req_lifecycle_third_action established */
#define L1P3_CMD_CODE_GENERAL_REQ_LIFECYCLE_THIRD_ACTION 0x2006

/* argument shape: seven or more words; builds and runs a date -u line from arguments 1 to 6 (year, month, day, hour, minute, second) */
/* ENUM: CMD.code 0x200d general_req_set_clock established */
#define L1P3_CMD_CODE_GENERAL_REQ_SET_CLOCK 0x200d

/* argument shape: reply 0 then year, month, day, hour, minute, second as ints */
/* ENUM: CMD.code 0x200e general_req_get_clock established */
#define L1P3_CMD_CODE_GENERAL_REQ_GET_CLOCK 0x200e

/* argument shape: three or more words; listed brick types only; calls the hook word at a fixed location with (0, arg1, arg2, 0); minus 106 when the result is not positive */
/* ENUM: CMD.code 0x3001 general_req_hook_3001 established */
#define L1P3_CMD_CODE_GENERAL_REQ_HOOK_3001 0x3001

/* argument shape: reply 0 then one text field per record; same formats as the Origin 300 handler */
/* ENUM: CMD.code 0x4000 general_req_inventory established */
#define L1P3_CMD_CODE_GENERAL_REQ_INVENTORY 0x4000

/* argument shape: reply 0 then the SHUB workaround flag byte as an int */
/* ENUM: CMD.code 0x4001 general_req_war_flag established */
#define L1P3_CMD_CODE_GENERAL_REQ_WAR_FLAG 0x4001

/* argument shape: no reply; logs and restarts the controller */
/* ENUM: CMD.code 0x4002 general_req_controller_reboot established */
#define L1P3_CMD_CODE_GENERAL_REQ_CONTROLLER_REBOOT 0x4002

#endif
