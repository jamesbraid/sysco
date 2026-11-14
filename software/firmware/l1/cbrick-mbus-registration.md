# L1 1.44.0 C-brick (type C): two-wire bus population, bus select, identity and USB probe

What the C-brick personality asks of the L1's two-wire bus before and while it registers its board, read for the node-board controller of the C-brick (brick letter `C`, board name `IP35`). Everything is stated as bus traffic: an address, a value written, the reaction to an error.

Read from the IRIX 6.5.30 `l1.bin` (L1 1.44.0, sha256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`), ColdFire package 0 (`Base 1MB image`, body loaded at `0xffe00000`), first program copy. Package 2 (`MIPS 2MB image`) carries byte-identical descriptor arrays and an identical bus-select hook at `+0x6070` (tables and hook compared, hook differing only in 16 call-target bytes); package 1 has none of this board. Grades as in [../../../AGENTS.md](../../../AGENTS.md). The two-wire controller layer, the expander driver and the monitor driver are the same code as in package 1 and are described in [mbus.md](mbus.md), [ip45-io-expanders.md](ip45-io-expanders.md) and [ip45-monitor-registers.md](ip45-monitor-registers.md); this note gives the C-brick's tables and what it does with them. The array at `0xffe455dc` listed in the package 0 section of the expander note is this board's array.

## How the C-brick personality is selected (established)

The identity routine (`0xffe55604`) runs first in the boot task (`0xffe10570`, call at `0xffe105a0`). A table keyed by the type character picks the brick-operations initializer (`0xffe4439a`, jump table at `0xffe443bc`): `C` gets the IP35 one (`0xffe45504`), `G` and `R` their own, `&`, `I`, `N`, `P`, `X`, `Y` one I/O-brick initializer, and `?` a default one (`0xffe49e9c`). Every other character logs "brick type is not supported in this image" and gets nothing. The IP35 initializer stores the board init (`0xffe453f8`) at offset 4 of the operations record (`0x30001e30`), a USB count of 1 at offset `0x10` and a second count of 1 at `0x14`. So a node EEPROM that does not classify as `C` (or a stored fallback setting that does not) gives the default personality: none of the tables below is registered, no expander is written and no monitor is initialised.

Boot order after identity (`0xffe10570`): UART and USB window setup (`0xffe55548`, which runs the USB probe of section 5), then board init (`0xffe453f8`). Board init registers the bus-select hook and the tables, and its display probe is the first step that drives the bus. The PIMM probe, the DIMM probes, the start-up block `0xffe4a83c`, then monitor init and fan set-up follow; section 3 gives each step and its error reaction. A settings-driven port and name routine (`0xffe537a8`, not read in full) also runs in this span; where it falls among the probes was not traced. The network connection mode write (section 3, COM CONTROL) is a later step, in the main task.

## 1. The board identity EEPROM

Firmware record table (`0xffe456ec`, 11 records of 12 bytes, registered by `0xffe6ade0`; layout: byte 0 segment, byte 1 wire address in write form, halfword at 2 capacity, word at 4 kind, word at 8 name pointer). Established.

| Record | Name | Segment | Wire address | Capacity | Kind |
|---:|---|---:|---|---:|---|
| 0 | NODE | 0 | `0xa4` | 512 | board record |
| 1 | PIMM 0 | 4 | `0xa8` | 512 | board record |
| 2 | PIMM 1 | 5 | `0xa8` | 512 | board record |
| 3 to 10 | DIMM 0, 2, 4, 6, 1, 3, 5, 7 | 6 to 13 in that order | `0xa0` | 256 | memory SPD |

Four more fixed routes, used only by the identity routine and not in the registered table (selector `0xfc` to `0xff`, table at `0xffe6be3c`, resolver `0xffe6b0b8`): `0xff` is segment 0 wire `0xa4`, `0xfe` is `0xa8`, `0xfd` is `0xa0`, `0xfc` is `0xac`, all capacity 256, all kind board record. Record `0xff` is therefore the same device as NODE (record 0). Established.

**Addressing.** The reader (`0xffe6b18c`, read in full) sends one address byte per transaction. The upper offset bits go into the slave address: slave byte = table address + ((offset >> 7) and `0xe`); the low offset byte is the single address byte. A read never crosses a 256-byte boundary (it is split) and never exceeds 32 bytes. So the second 256 bytes of a 512-byte record are at wire `0xa6` for NODE and `0xaa` for the PIMM records, which answers the question of whether the second page sits at `0xa6`/`0xaa`: yes, in the firmware's addressing. An access whose offset plus length exceeds the table capacity logs "EEPROM length out of range" and returns 0 without bus traffic. Established. The capacity word is a 16-bit byte count (`0x0200`, `0x0100`), not a page count.

**Part.** The four route addresses `0xa0`, `0xa4`, `0xa8`, `0xac` are the four settings of two address pins in bits 3 and 2 of the slave byte, and the firmware puts the next offset bit (bit 8) into bit 1. A 512-byte device with one address byte, two address pins and a page-select bit in bit 1 of the slave byte is the organisation of the 24C04 family. Inferred: a 24C04-compatible device on NODE and the two PIMM records, and a 256-byte SPD-style device (24C02 compatible) on each DIMM route. No image names the part. The `0xe` mask would reach bits 3 to 1 for larger offsets; capacity 512 only ever reaches bit 1.

**Write protect.** Nothing in the image drives or reads a write-protect line: no expander descriptor is named for it, and no record has a writer (below). Unknown.

**Writes.** Package 0 has no EEPROM writer. The module literal of the writer (`eeprom_write.c`) is in packages 1 and 2 and absent from package 0, and the only callers of the register-write primitive (`0xffe6cf4c`) are the expander and monitor drivers (`0xffe65cbc`, `0xffe68494`, `0xffe69394`); none lies in the record code (`0xffe6a000` to `0xffe6c000`). Established for package 0: a C-brick running package 0 never writes any board or DIMM EEPROM at boot or later. In package 2 the writer is linked; its callers there belong to the MAC-address EEPROM initializer and the `eeprom write` console command (command text "not available for this brick" for other types), not to the C-brick board init; inferred, those callers not re-read for this note.

**Boot reads.** `0xffe4a83c` runs the full record load (`0xffe6bdfc`): for each of the 11 records the 8-byte common header, then each declared area. Before it, `0xffe4482c` probes records 3 to 10 with one 8-byte read at offset 0 (`0xffe6b2a4`). Identity (`0xffe55604`) loads `0xff`, and `0xfe`, `0xfd`, `0xfc` when still unknown. A probe that gets no acknowledge (status `-404`, `0xfffffe6c`) clears that record's presence flag silently; any other error logs at level 1 first. The same presence clear applies to every failed probe. No retry anywhere. Established.

## 2. Type classification from the identity records (established)

The loader (`0xffe6bd54`) clears a 120-byte record, then reads the common header and areas. The name the identity routine compares is the first string after the manufacturer in the **board information area** (area class 1), decoded by `0xffe6b458` (manufacturer length `(byte 6 of the area) and 0x3f`, then the product-name field with its own length `and 0x3f`, then serial number and part number). A header error, a bad area checksum or an absent area leaves the pointer empty and the comparison is skipped. Each compare is a prefix compare of the stated length (`0xffe7d444`, a counted string compare, case-sensitive; trailing characters do not matter).

Record `0xff` (route segment 0, wire `0xa4`), tried first, first match wins:

| Prefix (length) | Type |
|---|---|
| `IP35` (4) | `C` |
| `KCARL1` (6) | `G` |
| `IP34` (4) | `A` |
| `POWERON` (7) | `T` |
| `X2PWR` (5) | `Y` |
| `NIPWR` (5) | `&` |
| `NPWR` (4) | `N` |
| `RPWR` (4) | `R` |
| `TWPWRIN` (7) | `6` |
| `WS_INT_53` (9) | `9` |
| `2U_INT_53` (9) | `7` |

If still unknown, record `0xfe` (route wire `0xa8`):

| Prefix (length) | Type |
|---|---|
| `ROUTER` (6) | `R` |
| `PBRICK` (6) | `P` |
| `IBRICK` (6) | `I` |
| `XBRICK` (6) | `X` |
| `X2BRIC` (6, from `X2BRICK`) | `Y` |
| `PXBRIC` (6, from `PXBRICK`) | `#` |
| `IP45` (4) | `4` |
| `PEBRICK` (7) | `%`, and `PEBRICK5V` (9) also sets a sub-type byte |
| `2U_INT_53` (9) | `7` |

Records `0xfd` and `0xfc` are read only when the type is already `%` (from `0xfe`), and they never change it. A type that stays `?` after `0xfe` takes the stored setting with key 9, which supplies a character (accepted set: `# % & 0 3 4 5 6 7 8 9 = @ A B C D E F G H I L N O P Q R T V X Y` and `1`, `2`; a stored `4` is rejected back to `?`; anything else leaves `?`). Types `1` and `2` also consult a second stored setting, which was not decoded (unknown). So a C-brick is `C` when the board information area of the EEPROM at wire `0xa4` on segment 0 has a product name starting `IP35`; an I-brick is `I` when the EEPROM at `0xa8` has `IBRICK` (and the `0xa4` name matched nothing above); an R-brick is `R` from `RPWR` at `0xa4` or `ROUTER` at `0xa8`. The default name `NODE` is the log name of the route, not a compared string; a product name of `NODE` matches nothing in either table, so the type stays `?` and the setting fallback decides. The accepted-set decode is a jump table at `0xffe55a14` (55 entries, type minus `0x23`).

## 3. Devices on the bus, in boot order

Descriptors (`0xffe455dc`, 17 of 16 bytes; layout in [ip45-io-expanders.md](ip45-io-expanders.md)), registered by `0xffe65974`; all form 0, one data byte per access, no register byte. "Wire" is the write form; reads use that value with bit 0 set (so display control is written at `0x40` and read at `0x41`; the bus-select expander at `0x4e` and `0x4f`). The application writes `applied value or input mask`.

| Idx | Name | Seg | Wire | Input bits | Applied | Event bits | Hook |
|---:|---|---:|---|---|---|---|---|
| 0 | BUS SELECT | 0 | `0x4e` | `0x00` | `0xff` | `0x00` | no |
| 1 | LED0 LOW | 2 | `0x42` | `0xff` | `0xff` | `0x00` | no |
| 2 | LED0 HIGH | 2 | `0x44` | `0xff` | `0xff` | `0x00` | no |
| 3 | LED1 LOW | 2 | `0x46` | `0xff` | `0xff` | `0x00` | no |
| 4 | LED1 HIGH | 2 | `0x48` | `0xff` | `0xff` | `0x00` | no |
| 5 | COM CONTROL | 2 | `0x4c` | `0xf1` | `0xff` | `0x00` | yes |
| 6 | BEDROCK CNTL 1 | 3 | `0x42` | `0x00` | `0x72` | `0x00` | no |
| 7 | BEDROCK CNTL 2 | 3 | `0x44` | `0x00` | `0x8a` | `0x00` | no |
| 8 | PORT CONTROL | 3 | `0x46` | `0x55` | `0xdd` | `0x11` | yes |
| 9 | POWER CONTROL | 3 | `0x48` | `0xde` | `0xff` | `0xde` | yes |
| 10 | PIMM PSC | 3 | `0x4a` | `0xff` | `0xff` | `0x00` | no |
| 11 | FAN CONTROL | 3 | `0x4c` | `0x40` | `0xff` | `0x40` | yes |
| 12 | PIMM0 POWER | 4 | `0x44` | `0x36` | `0xff` | `0x36` | yes |
| 13 | PIMM0 VCC OK | 4 | `0x42` | `0x3c` | `0xff` | `0x00` | yes |
| 14 | PIMM1 POWER | 5 | `0x44` | `0x36` | `0xff` | `0x36` | yes |
| 15 | PIMM1 VCC OK | 5 | `0x42` | `0x3c` | `0xff` | `0x00` | yes |
| 16 | DISPLAY CONTROL | 0 | `0x40` | `0x0f` | `0xff` | `0x07` | yes |

Monitors (`0xffe45770`, 7 of 12 bytes: type, segment, wire, flags, halfword for register `0x47`), registered by `0xffe67bb8`; all type 1 (the voltage and temperature monitor of [ip45-monitor-registers.md](ip45-monitor-registers.md)). Established.

| Idx | Name | Seg | Wire | Flags | Register `0x47` |
|---:|---|---:|---|---|---|
| 0 | NODE 0 | 2 | `0x58` | 0 | `0xa0` |
| 1 | NODE 1 | 2 | `0x5a` | 0 | `0x20` |
| 2 | NODE 2 | 2 | `0x5c` | 1 | `0x00` |
| 3 | PIMM0 A | 4 | `0x5c` | 0 | `0x00` |
| 4 | PIMM0 B | 4 | `0x5e` | 0 | `0x00` |
| 5 | PIMM1 A | 5 | `0x5c` | 0 | `0x00` |
| 6 | PIMM1 B | 5 | `0x5e` | 0 | `0x00` |

### Boot traffic in order, with the reaction to an error

The identity reads (section 2 and section 1) and the USB probe (section 5) come first. Board init then drives the bus itself: its display probe is the first traffic after the identity reads and the USB probe, and each step below reports its own errors.

After any failed transaction the two-wire driver runs its bus-recovery routine before it returns the error. The routine clears the controller's control register, sets its clock divider and enables the controller again, then, only if the bus-busy flag is still set, clocks the bus up to 1000 times. It is not a retry: the failed transaction is not sent again, and the caller gets the error. A board whose bus never goes busy sees one control-register reinitialisation per failed access and nothing else. Established for the routine; the register names are inferred. The routine's address is not recorded in this note.

1. **Display probe (`0xffe51268`, called from board init with index 16).** Board init (`0xffe453f8`) registers the bus-select hook and the tables with no bus traffic; this probe is its first traffic. One read of DISPLAY CONTROL. On an error it logs "Error reading the display I/O expander" at level 1, clears the expander's enabled byte (later access returns -407 without traffic) and gives up the display; otherwise it records the index and the display text driver starts (not decoded). Established.
2. **PIMM probe (`0xffe4482c`).** One read of PIMM PSC (index 10, segment 3, wire `0x4a`/`0x4b`). On an error it logs "Error reading PIMM PSC status" at level 1 and treats the byte as `0xff`. The low nibble is the code for PIMM 0, the high nibble for PIMM 1. Accepted codes (`0xffe446cc`): 0 to 8, 10 and 15. Codes 9 and 11 to 14 log "Unknown PSC for PIMM" at level 1 and are left unrecorded; 14 is unknown, not accepted. Code `0xf` means absent: the firmware then disables that PIMM's supply entries and its two expanders (`PIMMn POWER`, `PIMMn VCC OK`, indexes 12 and 13, or 14 and 15, by clearing their enabled byte so later access fails with -407 and no bus traffic), its EEPROM record (record 1 or 2) and the monitor entries of step 5 below. If both are present with different codes and the pair is not {1, 6} (`0xffe44750`), the firmware logs "PIMM types are incompatible" at level 1 and marks PIMM 1 absent. A change from the PSC code stored in the settings (keys `0x56`, `0x57`) logs "PIMM type changed, setting the default voltage margins" at level 6 and resets the margins. Established. An absent PIMM, either one, disables monitor indexes 5 and 6, PIMM 1's pair (the call arguments are constants in the routine at `0xffe4497e`), so an absent PIMM 0 leaves its own two monitors (indexes 3 and 4) enabled: noted as read, reason unknown.
3. **DIMM probes (`0xffe4482c`, records 3 to 10).** One 8-byte read at offset 0 of each of records 3 to 10, see section 1. The node record and the two PIMM records (0 to 2) are not probed.
4. **Start-up block (`0xffe4a83c`).** Runs these in order:
   - a. **Bus-select reset (`0xffe6a914`).** Takes the bus lock for segment 0 and resets the bus-switch table entries; this board registers no entry (the table count stays 0), so no traffic. Inferred (registration absent in the board init; not searched for outside it).
   - b. **Cold-start test (`0xffe4a7b0`).** Reads a RAM word (`0x301019ac`, `0xffe566e8`); nothing in the body stores to it. If it is zero the routine returns -1 at once. If nonzero it reads every expander back (`0xffe66488`, a read error logs "could not read I/O exp" and returns -1) and checks supply, fan and display-light state; only if all agree does it return 0. A nonzero return runs step c; zero skips it. Established for the branches; that the word is zero after a power-on is inferred (it is only ever read).
   - c. **Apply (`0xffe6664c`, per descriptor `0xffe6657c`).** For each enabled descriptor: shadow = applied or input mask, one write of that byte to the wire address, then one read of the same address into the state record, change reporting enabled for the event bits. The first bus error for a descriptor skips its read and its reporting, stores nothing else, logs nothing, and the loop goes on to the next descriptor. No retry. The bytes written are therefore, in index order: BUS SELECT `0xff` (every downstream segment off), the four LED expanders and COM CONTROL `0xff` (segment 2), BEDROCK CNTL 1 `0x72`, BEDROCK CNTL 2 `0x8a`, PORT CONTROL `0xdd` (`0xdd or 0x55`), POWER CONTROL `0xff`, PIMM PSC `0xff`, FAN CONTROL `0xff` (segment 3), the four PIMM expanders `0xff` (segments 4 and 5) and DISPLAY CONTROL `0xff` (segment 0). Established (the routine and the table were read in full). Each descriptor is applied under the lock for its own segment, so the select write to `0x4e` precedes the first access on each of segments 2 to 5.
   - d. **Record load (`0xffe6bdfc`).** Section 1.
5. **Monitor init (`0xffe69ca0`).** For each enabled monitor, type 1 calls `0xffe686b4`, which writes registers in order: `0x40` = `0x80`, `0x43` = `0xdf`, `0x44` = `0x13`, `0x47` = the low byte of the halfword in the table, `0x40` = `0x03` unless bit 0 of the flags is set (NODE 2 only: left with `0x40` = `0x80`). Each write is one address byte, register, value. It stops at the first error for that monitor, logs "Error initializing the NAME monitor" at level 1 and moves on; the monitor stays enabled. No retry. Established.
6. **Fan and temperature set-up** (`0xffe4ae94`, `0xffe4b1f4`, `0xffe4e600`, `0xffe52244`): writes to fan-enable expander bits and monitor fan registers; an error logs at level 1 and continues. Not decoded value by value.

After boot the expanders are rescanned by a handler registered for indexes 0 to 15 (`0xffe66798`, registered by `0xffe6c15c`) and the monitors by one registered for 0 to 6 (`0xffe69a80`); both are interrupt handlers (the line is a ColdFire interrupt input, wiring unknown). A read error in a rescan logs "Error reading NAME I/O expander" at level 3 and the scan continues.

No path in the routines read above aborts the boot or retries a transaction. The bus-recovery routine after a failed transaction reinitialises the controller's control register, but it does not send the transaction again. Routines in step 9 and the display text driver were not read in full.

### Which devices the boot needs

- **Needed to become a C-brick:** the identity EEPROM at segment 0 wire `0xa4`, with a valid common header and board information area whose product name starts `IP35`; or a stored setting 9 that yields `C`. Without it the type is `?` or another type and none of the C-brick tables is used (section "How the C-brick personality is selected"). Established. The same EEPROM serves the NODE record.
- **Everything else can be absent without stopping boot:** each expander, monitor, EEPROM and DIMM read failure is logged and skipped (above). A missing expander still has its enabled byte set, so later set and clear calls on it fail with a bus error each time they are tried, and an absent PSC expander makes both PIMMs absent. The firmware does not need any of them to start the console task; whether the console prompt itself depends on a later routine was not examined.
- **The bus-select expander at `0x4e`:** its write error ends that locked transaction with the error code and no retry; segment 0 devices (NODE EEPROM, display expander, the bus-select expander itself) are unaffected. Established.

### Pin meanings (the bits the firmware names or tests)

Established only where a routine or table gives a literal; the labels are the firmware's own. Polarity is inferred unless stated.

- **BUS SELECT** (all outputs): section 4.
- **COM CONTROL** (`0x4c`, bits 1 to 3 outputs, bits 0 and 4 to 7 inputs): the network connection routine (`0xffe54250`, called from the main task at `0xffe108fe` when the mode variable is not 2, and from `0xffe544dc`; case `C` at `0xffe542de`) updates mask `0x0e` to value `0x0a` on index 5, which the error text calls USB mode, then sets its state byte to 2; an error logs "Error setting network connection to USB mode" at level 3 and the routine goes on. It then resets each USB device (one here) unless a second state word is set, logging "USB n reset error" at level 3 on failure. The other branch of the caller (`0xffe543bc`) is not read. Other types use other indexes and masks (`4`: index 0, same mask and value). Established for mask, value and the error path; the condition at the call site is not traced. Which pins are the inputs: unknown.
- **BEDROCK CNTL 1** (`0x42`, segment 3, all outputs, written `0x72`): mask `0x80` is the first reset line (set, wait ten task-clock units, clear: `0xffe445e8`), mask `0x40` the second (cleared, then set: `0xffe4463c`), masks `0x02` and `0x01` are cleared by the enable sequence (`0xffe44bd4`), together with `BEDROCK CNTL 2` mask `0x01`. Established for the sequences; polarity inferred from the order.
- **BEDROCK CNTL 2** (`0x44`, written `0x8a`, all outputs): mask `0x01` as above.
- **PORT CONTROL** (`0x46`, inputs `0x55`, outputs the others, written `0xdd`, events `0x11`): two ports, `NI` and `II` (the names in the port table at `0xffe459f4`, 2 entries registered by `0xffe4f454`). The change hook (`0xffe507c8`) hands the low nibble of the changed bits and of the value to the port state of the first port; the port table's first byte selects index 8. Which bit of which nibble is cable-present: inferred (inputs are bits 0 and 4, the event bits).
- **POWER CONTROL** (`0x48`, outputs bits 0 and 5, others inputs, events `0xde`): the supply records enable `DIMM` rails through masks `0x20` and `0x01` of this expander (record table at `0xffe457c4`, entries 3 and 4). Inferred from the record layout carried over from package 1.
- **PIMM PSC** (`0x4a`, all inputs): low and high nibble are the per-PIMM codes above.
- **FAN CONTROL** (`0x4c` on segment 3, bit 6 an input and event, other bits outputs): the supply records name mask `0x80` of it as an enable (entry 0). Inferred.
- **PIMMn POWER** (outputs bits 0, 3, 6, 7; inputs `0x36`): the supply records enable `PIMMn VCPU` with mask `0x01` and `PIMMn SRAM` with mask `0x08`. **PIMMn VCC OK** (inputs `0x3c`) is read; the bits are unnamed. Inferred.
- **DISPLAY CONTROL**: bits 0 to 3 inputs (events on bits 0 to 2), bits 4, 5, 6 the three LED outputs (LED 1 mask `0x10`, LED 2 `0x20`, LED 0 `0x40`; routines `0xffe51778`, `0xffe51830`, `0xffe518e4`), bit 7 unnamed. The state query reports 1 when the shadow bit is clear (a clear bit is the lit state, inferred). Established for the masks; the front-panel power button is bit 0 by analogy with package 1.

### Parts implied by the traffic (inferred)

- **Expanders:** a byte-wide quasi-bidirectional expander (the PCF8574 family): one data byte per access, no register select, input bits written as ones, wire addresses `0x40` to `0x4e` (seven-bit `0x20` to `0x27`, the PCF8574 range; the PCF8574A range `0x38` to `0x3f` is not used). A PCA9555 is excluded by the access form (no command byte). No image names the part.
- **Monitors:** a voltage and temperature monitor in the DS1780, LM78 and ADM9240 family (registers `0x20` to `0x27`, configuration `0x40`, interrupt masks `0x43` and `0x44`, divisor and VID `0x47`), seven-bit `0x2c` to `0x2f`; the driver also reads the SMBus alert response address (seven-bit `0x0c`, `0xffe67d44`). No identity register is checked.

## 4. The bus-select expander at wire `0x4e` (established)

The board registers one select hook (`0xffe452d0`, registered by `0xffe6c9fc`). The lock routine (`0xffe6cab4`) takes the bus semaphore and, for a non-zero segment, calls the hook; the release routine (`0xffe6ca24`) clears the remembered segment, so the hook runs again at the next locked access to any non-zero segment. An access to segment 0 writes nothing: the select byte left by the previous access stays in place, so a segment chosen earlier stays enabled. The hook writes the BUS SELECT expander (index 0) through the ordinary masked write (full mask, one byte); it skips the write when the shadow already holds the value. Segment 1 clears the shadow first, so its write always happens. A segment above 13 logs "Illegal IP35 brick I2C bus select" at level 9 and carries on to the write step with an unset value (an error path no descriptor uses). A bus error on the write ends the locked access with that error. There is no separate switch part on this board: the only select traffic is single writes to `0x4e`.

| Segment | Byte written to `0x4e` | Reading of the bits (inferred) |
|---:|---|---|
| 0 | none | direct bus |
| 1 | `0xff` | all downstream segments off |
| 2 | `0x78` | bit 7 low |
| 3 | `0xb8` | bit 6 low |
| 4 | `0xd8` | bit 5 low |
| 5 | `0xe8` | bit 4 low |
| 6 to 13 | `0xf0` to `0xf7` (segment minus 6) | bits 7 to 4 high, bit 3 low, bits 2 to 0 = segment minus 6 |

So segments 2 to 5 are four separately enabled downstream segments (one of bits 7 to 4 low, bit 3 high, bits 2 to 0 zero) and segments 6 to 13 are eight positions of a three-bit selector enabled by bit 3 low while the four enables are off. The expander is written `0xff` at start-up (segment 1's value), which switches everything off. Which devices sit behind which segment is in the tables above: segment 2 holds the LED and COM CONTROL expanders and the NODE 0 to 2 monitors; segment 3 the BEDROCK, PORT, POWER, PSC and FAN expanders; segments 4 and 5 the PIMM 0 and PIMM 1 power expanders, EEPROMs (`0xa8`) and monitors; segments 6 to 13 the eight DIMM SPD EEPROMs. The part behind bits 7 to 4 and the eight-way selector is unknown. The same table holds in package 2.

## 5. USB controller window (chip select 3)

The C-brick's USB count is 1 (operations record offset `0x10`), so only software slot 0 is set up, and slot 1 (chip select 5, `0x60000000`) is not. Established for the count and for the slot rule (`0xffe55548`, `0xffe55570`, `0xffe555b0`). Start-up (`0xffe55548`): MBAR+`0x88` = `0x4000`, MBAR+`0x8c` = 0, MBAR+`0x92` = `0x0953` (chip select 3 base `0x40000000`, mask 0, control `0x0953`; register identities and the 64 KB window size from the MCF5206E manual, inferred), then the probe for slot 0 at byte base `0x40000000`. The probe result is not tested by the caller (`0xffe555a6`); a failed probe leaves slot 0 unregistered and boot continues. Established.

Probe, in order (`0xffe6dcf8`, `0xffe6dcbc`, `0xffe70ab8`, `0xffe70be8`):
1. Write `0xb5` to offset `0x1b` (scratch), then read offset `0x18` twice; the second read is the revision. If it is `0x81` the firmware calls a RAM-held function pointer and stops (the target is not set anywhere in the image).
2. Otherwise read offset `0x18` again. Revision `0x10`, `0x11` or `0x13` is accepted without comment. Any other value with high nibble 1 is accepted with a level-4 log. Any other high nibble logs an error at level 1 and returns -1 (no controller registered).
3. Write each even value `i` from 0 to `0xfc` to offset `0x1b`, then read offset `0x1b` back (the firmware stores the complement first, a dead store) and require `i`; a mismatch logs at level 3 and returns -1.
4. On success register the slot (`0xffe6e968`) with the operation and interrupt callbacks. The reset callback (`0xffe70c30`) writes, in order: `0x11` = 0; with its flag argument set also `0x11` = `0x08`, a two-unit wait, `0x11` = 0 and `0x1c` = 0; then `0x1a` = 1; `0x14`, `0x15`, `0x12` = 0; `0x0f` = `0x08` then 0; `0x10` = 0; for each endpoint 0 to 7, `0x0a` = endpoint, `0x0b` = `0x3f` (endpoint 0) or 0, `0x03` = `0xa4`, `0x08` = `0xa4`; then `0x1a` = 0; `0x16` = 3; `0x17` = 0; `0x11` = `0x12`; with the flag also a two-unit wait and `0x1c` = 1. The boot-time reset call (`0xffe54370`, argument 0) is the only caller read; whether it passes a set flag to this routine was not traced.

Expected value: the firmware does not require one revision. `0x10`, `0x11` and `0x13` pass quietly, any other `0x1n` passes with a log line, `0x81` hands over to a driver the image does not contain (its pointer is never stored by the body), and every other value fails. A bus that floats (`0xff`) or reads zero fails step 2 and leaves the controller unregistered; the boot goes on. The later reset call (`0xffe54370`) logs "USB reset error" at level 3 if it returns an error; what it returns for an unregistered slot was not read. Established for the sequence; the register meanings are inferred from the public USS-820 manual and are in [usb-controller-registers.md](usb-controller-registers.md).

## Not read

- The display text driver and its busy handling.
- Fan, supply and temperature routines of step 9 value by value, and the supply-record layout for this board (carried over from package 1; the decode gives sensible expander indexes and masks).
- The pin meaning of every input bit; the parts behind the bus select; whether the console prompt depends on any device above.
- Who sets the RAM word at `0x301019ac`.
- The callers of the writer in package 2.
