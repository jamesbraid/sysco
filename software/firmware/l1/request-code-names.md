# Request code names

The `L1P<n>_CMD_CODE_*` macros in the `*_DECODE.h` headers name the console request codes of each ColdFire package. This note says what each name's code does in package 1, the Origin 300 reference, so that a header name can be read against its handler. The handler addresses are first-bank body addresses in 6.5.30:l1#1 (L1 1.44.0, package 1). Each role is read from the handler row in [request-codes.tsv](request-codes.tsv) and restated in our own words; the grade is the one that row gives. The task number is the one the dispatcher uses for the code.

Packages 0 and 2 run the same handler for the 39 codes that package 1 shares with them, as [brick-request-handlers.md](brick-request-handlers.md) records. Packages 3 and 5 are not covered here.

## General request codes

- `L1P1_CMD_CODE_COMMAND_TASK_LOOP` (code 0x0000) **Established.** Any code that carries one text argument is handled here, and the reply is the status followed by the output lines as text. The handler is at 0xffe1ba5c, under task 3.

- `L1P1_CMD_CODE_GENERAL_REQ_READ_DEBUG_SWITCHES` (code 0x0001) **Established.** Replies 0 followed by one integer, the 16-bit non-volatile debug switches. The handler at 0xffe11fe8 builds that reply under task 6.

- `L1P1_CMD_CODE_GENERAL_REQ_RACK_BAY` (code 0x0002) **Established.** Replies 0 followed by the rack and bay packed as rack shifted left six bits, OR bay, taken from the non-volatile settings. The handler is at 0xffe11ffc.

- `L1P1_CMD_CODE_GENERAL_REQ_RACK_BAY_TYPE` (code 0x0003) **Established.** The same handler at 0xffe11ffc as code 0x0002, with the stored brick-type character added to the reply (0x34 for an IP45 brick).

- `L1P1_CMD_CODE_GENERAL_REQ_BRICK_SERIAL` (code 0x0004) **Established.** Replies 0 followed by the brick serial as text, and returns minus 104 when no serial is stored. The handler is at 0xffe122a8.

- `L1P1_CMD_CODE_GENERAL_REQ_FIRMWARE_REVISION` (code 0x0005) **Established.** Replies 0 followed by three integers parsed out of the firmware version string. The handler at 0xffe122da reads that string.

- `L1P1_CMD_CODE_GENERAL_REQ_EEPROM_READ` (code 0x0006) **Established.** Answers with status 0, the count of bytes left after the window, and a binary field. The handler at 0xffe12526 returns a negative status on its failure paths.

- `L1P1_CMD_CODE_GENERAL_REQ_EEPROM_FORMAT` (code 0x0007) **Established.** Its argument is a record kind, then eight more integers for kind 1 and two for kind 2. The handler is at 0xffe1236a.

- `L1P1_CMD_CODE_GENERAL_REQ_PORT_SPEEDS` (code 0x000a) **Established.** Replies 0 then two integers taken from the per-port speed settings. The handler is at 0xffe128f2.

- `L1P1_CMD_CODE_GENERAL_REQ_SYSTEM_SERIAL` (code 0x0008) **Established.** Replies 0 followed by the system serial as text, and returns minus 104 when that text is empty. The handler is at 0xffe12848.

- `L1P1_CMD_CODE_GENERAL_REQ_PARTITION_GET` (code 0x0009) **Established.** Replies with a status of 0 followed by one integer value; the handler at 0xffe128ca builds that two-field reply under task 6.

- `L1P1_CMD_CODE_GENERAL_REQ_TWO_BIT_SETTING` (code 0x000b) **Established.** Replies 0 then one integer read from setting 0x12, the partition-database flag: 1 means the partition comes from the operating system through the L2, and 0 means it comes from L1 non-volatile memory. The handler is at 0xffe128de.

- `L1P1_CMD_CODE_GENERAL_REQ_ATTACHED_PORTS_1_TO_4` (code 0x000c) **Established.** Replies 0 followed by four pairs, each a rack and bay packed as rack shifted left six bits, OR bay, with a brick type. The handler at 0xffe12090 builds the pairs.

- `L1P1_CMD_CODE_GENERAL_REQ_ATTACHED_PORTS_5_AND_10` (code 0x000d) **Established.** Replies 0 followed by two of the same rack-and-bay and type pairs, for ports 5 and 10. The handler is at 0xffe1220c.

- `L1P1_CMD_CODE_GENERAL_REQ_REQUESTING_PORT` (code 0x000e) **Established.** Replies 0 followed by one integer naming the link index the request arrived on. The handler at 0xffe12a4c returns that index.

- `L1P1_CMD_CODE_GENERAL_REQ_LAST_EVENT_REASONS` (code 0x000f) **Established.** Replies 0 followed by two integers from the lifecycle module. The handler is at 0xffe12a78.

- `L1P1_CMD_CODE_GENERAL_REQ_PROCESSOR_MASKS` (code 0x0010) **Established.** Replies 0, a word count, the processor count, then the processor mask pairs. The handler at 0xffe12acc returns minus 108 when there are no masks.

- `L1P1_CMD_CODE_GENERAL_REQ_SCAN_RECORD` (code 0x0011) **Established.** Replies 0 followed by a 64-byte binary field. The handler at 0xffe12c3c returns minus 106 with a status in its other case.

- `L1P1_CMD_CODE_GENERAL_REQ_WRITE_DEBUG_SWITCHES` (code 0x1001) **Established.** Takes one integer, writes it to the non-volatile debug switches and replies 0. The handler is at 0xffe12dfc.

- `L1P1_CMD_CODE_GENERAL_REQ_CONSOLE_SUBCHANNEL` (code 0x1002) **Established.** Takes one integer, sets the active console subchannel, and replies 0. The handler at 0xffe12e40 makes that change.

- `L1P1_CMD_CODE_GENERAL_REQ_CONSOLE_VOLUNTEER` (code 0x1003) **Established.** Records the local module as the console owner and replies 0. The handler is at 0xffe12e9c.

- `L1P1_CMD_CODE_GENERAL_REQ_DISPLAY_LINE_1` (code 0x1004) **Established.** Takes one text argument and writes it to display line 2, replying 0. The handler at 0xffe12ec2 returns minus 102 when the argument is not text.

- `L1P1_CMD_CODE_GENERAL_REQ_DISPLAY_LINE_2` (code 0x1005) **Established.** The same handler at 0xffe12ec2 as code 0x1004, so the two codes share the display-line path.

- `L1P1_CMD_CODE_GENERAL_REQ_PARTITION_SET` (code 0x1006) **Established.** Takes one integer, stores it in non-volatile memory and replies 0. The handler is at 0xffe12f20.

- `L1P1_CMD_CODE_GENERAL_REQ_TWO_BIT_SETTING_SET` (code 0x1008) **Established.** Takes one integer, writes it to the partition-database flag that code 0x000b reads back, and replies 0. The handler is at 0xffe12f7e.

- `L1P1_CMD_CODE_GENERAL_REQ_EVENT_SUBCHANNEL_SET` (code 0x1007) **Established.** Takes one integer, keeps it in non-volatile memory, and replies 0. The handler is at 0xffe12fd8.

- `L1P1_CMD_CODE_GENERAL_REQ_EVENT_SUBCHANNEL_CLEAR` (code 0x100a) **Established.** Sends no reply. It clears the event subchannel and unpersists it. The handler is at 0xffe131b2.

- `L1P1_CMD_CODE_GENERAL_REQ_PANEL_CONTROL` (code 0x1009) **Established.** Takes two integers, accepts a selector from 0 to 2 and returns minus 105 for any other value, then replies 0. The handler at 0xffe13060 holds that check.

- `L1P1_CMD_CODE_GENERAL_REQ_PROM_VERSION_REPORT` (code 0x100b) **Established.** Accepts exactly three integers and sends no reply on success. The handler is at 0xffe131bc.

- `L1P1_CMD_CODE_GENERAL_REQ_PER_PROCESSOR_SETTING` (code 0x100c) **Established.** Accepts exactly two integers and sends no reply on success. The handler at 0xffe132ca takes the two values.

- `L1P1_CMD_CODE_GENERAL_REQ_SYSTEM_RESET` (code 0x2001) **Established.** Sends no reply on success. On failure it returns minus 106 with a status. The handler is at 0xffe1335a.

- `L1P1_CMD_CODE_GENERAL_REQ_PCI_OPERATION_A` (code 0x2002) **Established.** Takes two integers, a bus and a slot, passes them to the board hook and sends no reply on success. The handler at 0xffe1336c makes the hook call.

- `L1P1_CMD_CODE_GENERAL_REQ_PCI_OPERATION_B` (code 0x2003) **Established.** The same shape as code 0x2002: a bus and a slot passed to the board hook, with no reply on success. The handler is at 0xffe13406.

- `L1P1_CMD_CODE_GENERAL_REQ_PCI_OPERATION_C` (code 0x2004) **Established.** The same shape as codes 0x2002 and 0x2003, with its own handler at 0xffe134bc.

- `L1P1_CMD_CODE_GENERAL_REQ_POWER_DOWN` (code 0x2005) **Established.** Sends no reply on success, and the table gives no other effect. The handler is at 0xffe13570.

- `L1P1_CMD_CODE_GENERAL_REQ_LIFECYCLE_THIRD_ACTION` (code 0x2006) **Established.** Sends no reply on success. The handler at 0xffe1359c runs a third lifecycle action, and the table leaves the identity of that action open.

- `L1P1_CMD_CODE_GENERAL_REQ_EXPANDER_READ` (code 0x2007) **Established.** Takes no arguments and returns minus 108 on an Origin 300 brick. The handler is at 0xffe135de.

- `L1P1_CMD_CODE_GENERAL_REQ_EXPANDER_WRITE` (code 0x2008) **Established.** Takes exactly two integers and returns minus 108 on an Origin 300 brick. The handler is at 0xffe136bc.

- `L1P1_CMD_CODE_GENERAL_REQ_INVENTORY` (code 0x4000) **Established.** Replies 0 followed by one text field for each inventory record, and returns minus 104 when there are none. The handler is at 0xffe137e8.

- `L1P1_CMD_CODE_GENERAL_REQ_CONTROLLER_REBOOT` (code 0x4002) **Established.** Sends no reply. It logs the request and restarts the controller. The handler at 0xffe13d76 does both.

## Flash request codes

- `L1P1_CMD_CODE_FLASH_REQ_STATUS` (code 0x0072) **Inferred.** Takes a first integer of at most 4, and the reply lists the flash images. The handler at 0xffe3c166 is task 16.

- `L1P1_CMD_CODE_FLASH_REQ_0X73` (code 0x0073) **Established.** Takes an offset and data, programs halfwords after a range check, and returns an integer result. On error the reply adds the failing address, with minus 0x514 for a range error and minus 0x515 for a verify error. The handler at 0xffe3c4c2 does the programming.

- `L1P1_CMD_CODE_FLASH_REQ_0X74` (code 0x0074) **Established.** Takes an offset and a length, erases the overlapping sectors other than the protected span, and returns an integer result. On error the reply adds the failing address, with minus 0x514 for a range or protection error and minus 0x517 for an erase failure. The handler at 0xffe3c41e does the erase.
