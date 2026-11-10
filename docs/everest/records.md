# Environment, log and diagnostic records

Read from `root/6.5/usr/etc/sysctlrd` (IRIX 6.5 Foundation 1, SHA-256 `c92b4b43c67fc22e...`). Addresses are virtual addresses in that daemon. The frames that carry these records are in [daemon-wire.md](daemon-wire.md). Grades follow [../../AGENTS.md](../../AGENTS.md).

## The environment reply (item `E`)

`getenvinfo` (`0x10005028`, 3,568 bytes) asks item `E`, using as the sequence tag a one-byte counter kept in the daemon state at +52. It splits the reply text at semicolons (`;`, the string at `0x1001c840`) with `strtok`, converts each field and stores it in a 200-byte record. **Established** from the `strtok` calls at `0x1000511c` and `0x1000516c` and the field switch at `0x10005190`-`0x100051cc`.

The sequence tag cycles through the ASCII digits `0` to `9`: it is incremented after every attempt, successful or not, and reset to `0` (`0x30`) when it reaches `:` (`0x3a`). **Established** (`0x100050b0`-`0x100050d4`, `0x100050ec`-`0x10005108`).

The reply holds either 10 or 26 fields. After the loop the daemon compares the count: 10 stores 1 at record +0, 26 stores 2, anything else logs "Received short environmental response string (%d fields)." and stores 0. **Established** (`0x100051f0`-`0x10005228`, `0x10005610`, `0x1000561c`). The extra fields are the extremes and the controller's internal variables, which `printenvinfo` prints only when +0 is 2 or more (`0x10004954`-`0x1000495c`).

Fields are decimal text. Fields written below as *fixed* go through `decimal_to_fixed` (`0x10004e90`, 240 bytes) and are stored in hundredths; the others go through `atoi`. **Established** from the switch.

| Field | Record offset | Conversion | Meaning | Grade |
|---:|---:|---|---|---|
| 0 | +8 | fixed | supply voltage, 48.0 V supply | established order, inferred identity |
| 1 | +12 | fixed | 12.0 V supply | as above |
| 2 | +16 | fixed | 5.0 V supply | as above |
| 3 | +20 | fixed | 1.6 V supply | as above |
| 4 | +24 | fixed | -5.2 V supply | as above |
| 5 | +28 | fixed | -12.0 V supply | as above |
| 6 | +32 | fixed | system air temperature | established |
| 7 | +36 | integer | blower A speed in RPM | established |
| 8 | +40 | integer | blower B speed in RPM (printed only for the `T` panel type) | established |
| 9 | +44 | integer | D/A setting (printed only in verbose mode) | established |
| 10 | +56 | fixed | controller firmware revision in hundredths, 202 meaning 2.02 | established |
| 11 | +60 | fixed | highest air temperature since power-up | established |
| 12 | +64 | fixed | lowest air temperature since power-up | established |
| 13 | +68 | integer | highest blower A speed | established |
| 14 | +72 | integer | lowest blower A speed | established |
| 15 | +76 | integer | highest blower B speed | established |
| 16 | +80 | integer | lowest blower B speed | established |
| 17 to 20 | +84 to +96 | integer | firmware internal variables `B0` to `B3` | established |
| 21 to 24 | +100 to +112 | integer | firmware internal variables `F0` to `F3` | established |
| 25 | +116 | integer | count of invalid CPU commands | established |

The offsets of fields 0 to 5 are `8 + 4n`; fields 7 to 8 are `8 + 4n`; the rest are `16 + 4n` for 11 to 24 and +116 for 25 (the three `atoi` and `decimal_to_fixed` branches at `0x1000514c`-`0x10005534`). Each meaning comes from where `printenvinfo` (`0x10004780`, 1,428 bytes) prints that offset and which label it uses (strings at `0x1001a000`-`0x1001a9f8`): the voltage labels pair, in table order, the six offsets +8 to +28 with the labels `48.0V`, `12.0V`, `5.0V`, `1.6V`, `-5.2V`, `-12.0V` (the table `powersupply_names` at `0x1001c2a8`, six pointers). Which physical rail each field carries is **inferred** from that pairing. The temperature fields print through `print_dec` followed by the text " C" (`0x1001c808`) and the revision prints through the same helper.

Other record words, written by the daemon and not taken from the reply: +0 the format level above, +4 the panel type letter set by `getscale`, +48 the time of the fetch (`time`), +52 the sequence digit, +120 the number of retries (copied from state +1160), +124 the log-fetch failures (+1164), +128 the environment fetch failures (+1168), +132 a fourth counter (+1172, never printed by the routines read), +136 to +183 the voltage extremes (six pairs: the maximum at +136+8k, the minimum at +140+8k, for supply k), +184 and +192 two time stamps used by the blower message rate limit. **Established** from the stores at `0x10005230`-`0x10005278` and the loop at `0x1000527c`-`0x1000546c`.

`decimal_to_fixed` reads the whole part with `atoi`, multiplies it by 100, and adds a fraction taken from the text after the decimal point: a fraction of two or more characters is read as one integer ("5.123" gives 500 + 123), one character is read and multiplied by 10, none adds zero. When the whole part is negative the fraction is subtracted instead of added, so "-5.2" gives -520; a whole part of zero is not negative, so "-0.5" gives +50. **Established** (`0x10004ee0`-`0x10004f68`; the sign test is on the `atoi` result at `0x10004ef0`).

On a failed read (`getresult` returned non-zero) `getenvinfo` logs "Get env info failed." in verbose mode, increments the counters at state +900 and +1168, advances the tag, and returns -1. On a good read it zeroes +900. The man page says the daemon gives up after 20 failures in a row; the number 20 is **hint** (man page), the log text "Giving up on fetching environmental information." is at `0x1001b110`.

After a successful parse the daemon updates the six voltage extremes and prints new ones when the `-l` option is set (strings `0x1001ab60` and `0x1001ab88`). It also compares the highest blower speeds (fields 13 and 15) with 3000: a value above 3000 is logged as "Blower A maximum (%d) > %d RPM. Notification[%d]." (or the B form), subject to a rate limit decided by `should_log_message` (`0x10002314`, 212 bytes: it refreshes one time stamp with `update_timer`, computes the minutes since the other with `elapsed_minutes`, and returns non-zero on its first call and afterwards only when 60 minutes or more have passed, in which case it refreshes the stored stamp). **Established** for the 3000 threshold (`slti` against 3001 at `0x1000546c` and `0x10005478`); the rate-limit reading (first message always, then at most one per 60 minutes) is **inferred** from the call-counter test and the `slti 60` at `0x10002378`; the units of `elapsed_minutes` come from a floating-point scale constant that was not decoded.

`check_sysctlr_rev` (`0x10004f80`, 168 bytes) compares the revision (+56) with 202: if the record's format level is non-zero and the revision is below 202 it logs "System controller firmware is downrev -" and "  Rev is %d.%02d - should be at least 2.02" and returns 1. **Established.** A 10-field reply does not write +56, so a controller that answers with 10 fields is reported as down-revision unless the record held an earlier value.

## The log reply (item `L`)

`getlog` (`0x1000620c`, 364 bytes) takes the display state and a sequence tag. A tag of -1 means "unsolicited": the daemon uses the character `L` (`0x4c`) as the tag. It reads item `L` with `getresult`, and on success sends a set of item `K` to clear the controller's log (`0x10006368`). On failure it logs "Get log failed!" in verbose mode and adds one to the log-failure counter at state +1164; the code goes on to process whatever is in the buffer. **Established.**

The reply is a list of entries separated by semicolons (`;`, `0x3b`). `getlog` removes the last character of the text, then walks backwards from the end, so entries are logged from last to first, each by `decode_entry` (`0x10006088`, 388 bytes). An entry is a text, a comma (`0x2c`) and a time field. **Established** (`0x100062c0`-`0x10006330` and `0x10006088`).

`decode_entry` rebuilds the time into the text `MM/DD/YY HH:MM:SS`: byte 6 and 7 of the field become the first pair, 8 and 9 the second, 10 and 11 the third, then a space, then bytes 0 and 1, a colon, 2 and 3, a colon, 4 and 5 (stores at `0x100060fc`-`0x10006180`). It then logs `Event: <time>, <text>` at priority 5, except when the first 19 characters of the text are "INVALID CPU COMMAND". An entry with no time field is shown as "Unknown time" (`0x1001ac80`). **Established.** The field is twelve digits; since the daemon sets the controller clock with the order month, day, year, hour, minute, second (item `T`), the layout of the reply field is **inferred** to be hour, minute, second, month, day, year (that gives the printed `MM/DD/YY`); the daemon never prints it any other way.

The man page quotes the strings "SCLR DETECTED", "SYSTEM OFF", "SYSTEM ON" and "NMI" as examples of log text; they are not in the daemon, so they come from the controller (**hint**, man page).

## Board and diagnostic names (PROM configuration messages)

At startup `getpromerrs` (`0x10006494`, 180 bytes) calls `syssgi` with selector 102 and a 1,768-byte buffer, hands the buffer to `check_inventory` (`0x1000703c`, 524 bytes) and checks the serial-number text with `valid_serial` (`0x10006378`, 176 bytes: accepts a non-empty string of at most ten printable characters); an invalid serial prints "WARNING: The serial number on this system controller is not valid!" to stderr and the log. The messages that begin "*** Slot %d previously contained an %s board" and the following ones (strings from `0x1001bc60` to `0x1001bf60`) are printed by `check_inventory`. `check_inventory` and the record it reads are described in [inventory-record.md](inventory-record.md).

`evdiag_msg` (`0x10008f88`, 68 bytes) maps a diagnostic code byte to text: it scans the 83 eight-byte entries of `diagval_map` (`0x1001c2e0`) comparing the first byte of each entry, returns the text pointer in the second word on a match, and returns "Unknown diagnostic code" otherwise. **Established.** The table is [prom-diag-codes.tsv](prom-diag-codes.tsv). Each entry's first word is the code in its top byte (`0x01000000` for code `0x01`). The code `0xfd` appears twice; the first entry ("POD mode switch set or POD key pressed.") always wins.

`board_name` (`0x10008df4`, 264 bytes) maps a board type byte to text: `0x00` Empty, `0x11` IP19, `0x12` IP21, `0x13` IP25, `0x21` IO4, `0x31` MC3, `0xff` Unknown; `0x10`, `0x20` and `0x30` give "Unknown CPU-type", "Unknown IO-type" and "Unknown Memory-type"; any other value gives "Unrecognized boardtype". **Established** from the compare chain at `0x10008df4`-`0x10008ef4` and the strings at `0x1001c780`-`0x1001c8a0` and `0x1001c008`-`0x1001c048`.

`ioa_name` (`0x10008efc`, 140 bytes) names an I/O adapter from a type and a subtype: type `0x0d` is "S1 SCSI controller", `0x0e` is "EPC Peripheral controller", `0x0f` is "FCI Graphics adapter" with subtype `0x20`, "VME adapter" with subtype `0x11` and "Flat cable interconnect" otherwise, `0x00` is "Empty", and anything else is "Unknown adapter type". **Established** from `0x10008efc`-`0x10008f84`.
