/*
 * L1P2_DECODE.h - console request codes of ColdFire L1 package 2
 *
 * Console request codes of one ColdFire L1 firmware package.
 * Codes are unique within the package.
 *
 * Each ENUM line is one request code. The comment above it gives the
 * argument shape as read in the image. Grades follow the source tables.
 */
#ifndef L1P2_DECODE_H
#define L1P2_DECODE_H

/* FIELD: CMD.code 15:0 established */
#define L1P2_CMD_CODE_SHIFT 0
#define L1P2_CMD_CODE_MASK 0xffffu

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x0001 general_req_read_debug_switches established */
#define L1P2_CMD_CODE_GENERAL_REQ_READ_DEBUG_SWITCHES 0x0001

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x0002 general_req_rack_bay established */
#define L1P2_CMD_CODE_GENERAL_REQ_RACK_BAY 0x0002

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x0003 general_req_rack_bay_type established */
#define L1P2_CMD_CODE_GENERAL_REQ_RACK_BAY_TYPE 0x0003

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x0004 general_req_brick_serial established */
#define L1P2_CMD_CODE_GENERAL_REQ_BRICK_SERIAL 0x0004

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x0005 general_req_firmware_revision established */
#define L1P2_CMD_CODE_GENERAL_REQ_FIRMWARE_REVISION 0x0005

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x0006 general_req_eeprom_read established */
#define L1P2_CMD_CODE_GENERAL_REQ_EEPROM_READ 0x0006

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x0007 general_req_eeprom_format established */
#define L1P2_CMD_CODE_GENERAL_REQ_EEPROM_FORMAT 0x0007

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x0008 general_req_system_serial established */
#define L1P2_CMD_CODE_GENERAL_REQ_SYSTEM_SERIAL 0x0008

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x0009 general_req_partition_get established */
#define L1P2_CMD_CODE_GENERAL_REQ_PARTITION_GET 0x0009

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x000a general_req_port_speeds established */
#define L1P2_CMD_CODE_GENERAL_REQ_PORT_SPEEDS 0x000a

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x000b general_req_two_bit_setting established */
#define L1P2_CMD_CODE_GENERAL_REQ_TWO_BIT_SETTING 0x000b

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x000c general_req_attached_ports_1_to_4 established */
#define L1P2_CMD_CODE_GENERAL_REQ_ATTACHED_PORTS_1_TO_4 0x000c

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x000d general_req_attached_ports_5_and_10 established */
#define L1P2_CMD_CODE_GENERAL_REQ_ATTACHED_PORTS_5_AND_10 0x000d

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x000e general_req_requesting_port established */
#define L1P2_CMD_CODE_GENERAL_REQ_REQUESTING_PORT 0x000e

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x000f general_req_last_event_reasons established */
#define L1P2_CMD_CODE_GENERAL_REQ_LAST_EVENT_REASONS 0x000f

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x0010 general_req_processor_masks established */
#define L1P2_CMD_CODE_GENERAL_REQ_PROCESSOR_MASKS 0x0010

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x0011 general_req_scan_record established */
#define L1P2_CMD_CODE_GENERAL_REQ_SCAN_RECORD 0x0011

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x1001 general_req_write_debug_switches established */
#define L1P2_CMD_CODE_GENERAL_REQ_WRITE_DEBUG_SWITCHES 0x1001

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x1002 general_req_console_subchannel established */
#define L1P2_CMD_CODE_GENERAL_REQ_CONSOLE_SUBCHANNEL 0x1002

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x1003 general_req_console_volunteer established */
#define L1P2_CMD_CODE_GENERAL_REQ_CONSOLE_VOLUNTEER 0x1003

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x1004 general_req_display_line_1 established */
#define L1P2_CMD_CODE_GENERAL_REQ_DISPLAY_LINE_1 0x1004

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x1005 general_req_display_line_2 established */
#define L1P2_CMD_CODE_GENERAL_REQ_DISPLAY_LINE_2 0x1005

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x1006 general_req_partition_set established */
#define L1P2_CMD_CODE_GENERAL_REQ_PARTITION_SET 0x1006

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x1007 general_req_event_subchannel_set established */
#define L1P2_CMD_CODE_GENERAL_REQ_EVENT_SUBCHANNEL_SET 0x1007

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x1008 general_req_two_bit_setting_set established */
#define L1P2_CMD_CODE_GENERAL_REQ_TWO_BIT_SETTING_SET 0x1008

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x1009 general_req_panel_control established */
#define L1P2_CMD_CODE_GENERAL_REQ_PANEL_CONTROL 0x1009

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x100a general_req_event_subchannel_clear established */
#define L1P2_CMD_CODE_GENERAL_REQ_EVENT_SUBCHANNEL_CLEAR 0x100a

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x100b general_req_prom_version_report established */
#define L1P2_CMD_CODE_GENERAL_REQ_PROM_VERSION_REPORT 0x100b

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x100c general_req_per_processor_setting established */
#define L1P2_CMD_CODE_GENERAL_REQ_PER_PROCESSOR_SETTING 0x100c

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x2001 general_req_system_reset established */
#define L1P2_CMD_CODE_GENERAL_REQ_SYSTEM_RESET 0x2001

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x2002 general_req_pci_operation_a established */
#define L1P2_CMD_CODE_GENERAL_REQ_PCI_OPERATION_A 0x2002

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x2003 general_req_pci_operation_b established */
#define L1P2_CMD_CODE_GENERAL_REQ_PCI_OPERATION_B 0x2003

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x2004 general_req_pci_operation_c established */
#define L1P2_CMD_CODE_GENERAL_REQ_PCI_OPERATION_C 0x2004

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x2005 general_req_power_down established */
#define L1P2_CMD_CODE_GENERAL_REQ_POWER_DOWN 0x2005

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x2006 general_req_lifecycle_third_action established */
#define L1P2_CMD_CODE_GENERAL_REQ_LIFECYCLE_THIRD_ACTION 0x2006

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x2007 general_req_expander_read established */
#define L1P2_CMD_CODE_GENERAL_REQ_EXPANDER_READ 0x2007

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x2008 general_req_expander_write established */
#define L1P2_CMD_CODE_GENERAL_REQ_EXPANDER_WRITE 0x2008

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x4000 general_req_inventory established */
#define L1P2_CMD_CODE_GENERAL_REQ_INVENTORY 0x4000

/* argument shape: same instruction stream as the Origin 300 handler; only call targets, string pointers and moved RAM words differ */
/* ENUM: CMD.code 0x4002 general_req_controller_reboot established */
#define L1P2_CMD_CODE_GENERAL_REQ_CONTROLLER_REBOOT 0x4002

#endif
