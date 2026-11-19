# L1 1.44.0: what the settings window must hold at start, and what the firmware does with a blank one

[settings-record.md](settings-record.md) maps every setting number to a byte. This note answers the question behind it: can a window be written from nothing, which bytes does the start-up code insist on, and what does it make of a window that is blank or wrong. It reads the start-up routines in full and checks the C-brick package against the Origin 300 one.

Read from L1 1.44.0 of the IRIX 6.5.30 `l1.bin`. Package 1 (`Fuel/PE/O300 1MB`) addresses are quoted first, package 0 (`Base 1MB`, the C-brick body) in brackets. The routines were compared whole between the two packages with address operands masked: the record initialiser (`0xffe5a188`, package 0 `0xffe5abb4`), the header rebuild (`0xffe5a6a8`, `0xffe5b0d4`), the start-up entry (`0xffe5a758`, `0xffe5b184`) and the getter (`0xffe59590`, `0xffe59fbc`) are identical instruction for instruction; only the RAM state block moves (`0x300003a4` in package 1, `0x30000380` in package 0). The setter was not compared in package 0. Both packages call the start-up entry from the boot task at `0xffe10580`. Grades as in [../../AGENTS.md](../../../AGENTS.md): E established, I inferred, U unknown.

## Layout of the window (E)

The window is the 2 KB battery-backed RAM at `0x20000000`.

| Window offset | Size | Contents |
|---|---|---|
| `0x000` | 8 | header |
| `0x008` | `0x70` | settings record 1, primary |
| `0x078` | `0x70` | record 1, mirror |
| `0x0e8` | `0x24` | record 2, primary (event-log descriptor) |
| `0x10c` | `0x24` | record 2, mirror |
| `0x130` | `0xb4` | record 3, primary |
| `0x1e4` | `0xb4` | record 3, mirror |
| `0x298` to `0x7f7` | `0x560` | event-log text ring; the firmware does not read it at start |
| `0x7f8` to `0x7ff` | 8 | clock-calendar |

The record offsets in [settings-record.md](settings-record.md) are from the start of record 1 (window offset `0x008`), so its `+0x70` is window offset `0x078`. Every `+` offset below is from the start of record 1 unless it says "window".

## Integrity rules (E)

There is no magic number and no CRC. Every check is a byte sum.

- **Sum.** The check routine (`0xffe5b3a4`) adds the bytes of a region and the caller tests the low byte for zero. The routine that writes a checksum (`0xffe5b3cc`) adds the other bytes and stores the two's complement, so a region is valid when all its bytes, checksum byte included, add to 0 modulo 256.
- **Header** (window `0x000`, 8 bytes): sum zero, the low nibble of byte 0 non-zero (the writer stores 1) and byte 1 non-zero. Byte 1 is the offset of record 1 in units of 8 bytes; the writer stores 1. Byte 7 is the checksum. Bytes 2 to 6 are zero. The firmware accepts another non-zero offset byte, but every number here assumes 1.
- **Record 1** (`0x70` bytes): sum zero over all `0x70` bytes (checksum at `+0x6f`). The start-up path tests the sum only; the repair routine (step 4) also requires the low nibble of byte 0 to be non-zero. The current version nibble is `0xa`.
- **Record 2** (`0x24` bytes at `+0xe0`, checksum at `+0x103`): sum zero and byte 0 non-zero. Byte 0 is its version, 1.
- **Record 3** (`0xb4` bytes at `+0x128`, checksum at `+0x1db`): sum zero and byte 0 non-zero. Byte 0 is its version, 3.
- **Mirrors** are byte copies of the primaries, placed `0x70`, `0x24` and `0xb4` bytes after them.

## What start-up does (E, entry `0xffe5a758`)

1. Check the header. If its sum is non-zero, or its version nibble or its offset byte is zero, rebuild it (`0xffe5a6a8`): zero the 8 bytes, store version 1, offset 1 and the checksum, set the record base pointer in the state block to window `8 * offset`, then run the record initialiser (`0xffe5a188`) at once, as step 4 describes: a record valid at its usual position is kept, so only a record with no valid copy comes from defaults.
2. Dispatch on the record 1 version nibble. Nibbles 0 and above 10 take the same path as `0xa`. A record 1 with nibble 0 and a good sum is accepted untouched when no repair runs; the repair routine rejects nibble 0, so a repair replaces such a record from its mirror or from defaults. Nibbles 1 to 9 run an upgrade chain (jump table at `0xffe5a858`, package 0 `0xffe5b284`) that rewrites older layouts in steps; it is decoded below, and a writer should still never produce those versions.
3. On the current path, flag a repair when record 1 or record 3 fails its sum. If record 3's byte 0 is not 3 it is rebuilt in place (the long at `+0x12c` and the `0xa8` bytes from `+0x130` zeroed, `+0x68` to `+0x6e` of record 1 zeroed, byte 0 set to 3, both checksums recomputed). If record 2's byte 0 is not 1 it is set to 1 and re-summed; if record 2 fails its sum a repair is flagged.
4. A flagged repair runs the record initialiser, which takes each record independently: a valid primary is kept; otherwise a valid mirror is copied over the primary; otherwise the record is rebuilt from defaults. With no repair flagged the primaries are copied over the mirrors. One valid copy of each record is therefore enough, but a writer should write both. A record 1 that a repair keeps, whose nibble is not `0xa`, is rewritten with nibble 4 and its brick serial (`+0x68` to `+0x6e`) cleared.
5. Fix-ups on the result: setting 17 (`+0x11`) below 2 becomes 2; setting 61 (`+0x17` bits 2:1) equal to 0 becomes 2.
6. Record 2 pointers: the word at `+0xe4` must equal window `0x298` (the end of the record area), `+0xe8` must equal `0x200007f8`, and `+0xec` and `+0xf0` must each lie in `[+0xe4, +0xe8)`. Any failure rebuilds record 2 (`0xffe59e94`): zero it, byte 0 = 1, `+0xe4` = `0x20000298`, `+0xe8` = `0x200007f8`, `+0xec` and `+0xf0` = `0x20000298`, checksum, mirror.

## The record 1 upgrade chain (package 0, E; package 1 has the same entry routine and its table at `0xffe5a858` was not re-read)

The table is eleven signed halfword offsets at `0xffe5b284` indexed by the version nibble (bits 27:24 of the first word of record 1): nibble 0 and nibble 10 go to the final check at `0xffe5b68c`, nibble *n* from 1 to 9 goes to arm *n*. Each arm checks, edits, writes version *n* + 1 and recomputes the checksum over `0x70` bytes into `+0x6f`, then falls through to the next arm, so a version 1 record runs all nine. The checked lengths are two words of the state block at `0x30000380`, loaded from the initialised data (copy source `0xffe7f114`): `+0x0c` = `0x70` (the current record) and `+0x18` = `0x54` (the length of versions 1 to 3). Arm 1 accepts a record whose byte sum over `0x54` bytes is zero; arms 2 and 3 accept zero over `0x54` or, failing that, over `0x70`; arms 4 to 9 accept only `0x70`. A failed check sets the repair flag and ends the chain at once (to `0xffe5b6a0`), so the repair of step 4 then runs. Offsets are from the start of record 1; setting numbers are those of [settings-record.md](settings-record.md).

| Arm (from version) | Entry | Edit | Settings touched |
|---:|---|---|---|
| 1 | `0xffe5b29a` | zero `+0x48` to `+0x67` (eight words); `+0x17` bits 7:4 := `0xf`; `+0x17` bits 2:1 := 2 | 70 to 77 zeroed; 60 (router role) := `0xf` (ordinary); 61 := 2 (the 3400 series, see below) |
| 2 | `0xffe5b364` | `+0x46` bits 5 and 4 cleared | the low bits of settings 42 and 43 |
| 3 | `0xffe5b3d6` | zero `+0x68` to `+0x6e` | the brick serial number (setting 15) |
| 4 | `0xffe5b444` | `+0x04` bit 4 set | 53 := 1 |
| 5 | `0xffe5b49e` | `+0x12` := 0 | 84 := 0 |
| 6 | `0xffe5b4ea` | `+0x11` := 0 | 17 := 0 |
| 7 | `0xffe5b536` | when setting 17 is 0 or 1, zero `+0x68` to `+0x6e`; then `+0x12` := 1 | 15 cleared; 84 := 1 |
| 8 | `0xffe5b5a8` | `+0x2e` bit 0 and `+0x2f` bits 7, 6, 5 cleared | the high bits of settings 42, 43, 46, 47 |
| 9 | `0xffe5b61a` | `+0x2e` bits 2:1 := 2 and bits 6:3 := `0xf` | 123 := 2; 124 := `0xf` |

Because arm 6 sets setting 17 to 0 and arm 7 clears the serial when it is below 2, every record that enters below version 7 loses its brick serial; only a version 7 or 8 record that already holds a setting 17 of 2 or more keeps it. Arm 1 also repairs the router role and series for old records, so an upgraded R-brick comes out as an ordinary router of the 3400 series. After arm 9 and for nibbles 0 and 10 the common tail checks the sum over `0x70` bytes (failure sets the repair flag), then record 3 and record 2 as in steps 3 and 6.

## Record 3 (E for the layout and the rebuild, I for what the entries mean)

Record 3 is `0xb4` bytes at window `0x130`: byte 0 the version (3), bytes `+1` to `+3` three bytes read as settings 170 to 175 (six nibbles), byte `+4` a slot index (setting 81, 0 to 23), bytes `+5` to `+7` a 24-bit slot flag word (setting 82), then 24 entries of 7 bytes at `+8` (the setting 83 address), `+0xb3` the checksum. An entry is three fields from a three-word argument: the low 16 bits of word 1, the low 16 bits of word 2 and the low 24 bits of word 3. The routines are in package 0: add `0xffe53424`, find `0xffe53514` and the checker that wraps them `0xffe535c8`. The log texts around them (`0xffe53f03` onward) name a public key in the board EEPROM, an authenticator printed as three eight-digit hex words, "Authenticator is invalid", "Authenticator has expired" and "Key has been disable by clock setting change". So the table is the history of authenticators the L1 has accepted, in a ring of 24, with a flag per slot; the checker rejects a repeat whose slot flag is set, rejects an authenticator whose date value, compared with a value derived from the L1 clock, is negative ("CLOCK ERROR") or above 2000 (`0x7d0`, "expired"; the unit was not worked out), and otherwise records it in the slot at the index, clears that slot's flag and advances the index modulo 24. The date test and the use of the flag words were read only as far as the branches; the settings 170 to 175 were not traced. I.

**Rebuild rule (E).** At start-up a record 3 whose byte 0 is not 3 is rebuilt in place without the repair routine: its first word is zeroed except the version, the `0xa8` entry bytes are zeroed, record 1's brick serial (`+0x68` to `+0x6e`) is zeroed, byte 0 becomes 3 and both checksums are recomputed (`0xffe5b6d0` to `0xffe5b752`). A record 3 whose byte 0 is 3 is only sum-checked. This is the "record 3 clears the serial" rule of the writing recipe below, and the serial is the only thing outside record 3 that the rebuild touches.

## Blank, erased or garbage windows (E)

A window of all zeros fails the header (offset byte 0), and all `0xff` fails it on the sum (eight bytes of `0xff` leave `0xf8`), so both are rebuilt. The header rebuild runs the record initialiser (step 4); a blank window has no valid copy of any record, so every record comes from defaults. Nothing stops the boot. Random contents behave the same way record by record: a record that passes the repair check at its usual position is kept.

Default content of record 1 after a rebuild (every byte not listed is zero):

| Byte | Value | Setting and meaning |
|---|---|---|
| `+0x00` | `0x0a` | version nibble; rack 0 (word bits 23:14) |
| `+0x02` | `0x00` | bay 0 |
| `+0x04` | `0x70` | bits 6:4 set: settings 52 and 53 are 1, setting 50 is 0 |
| `+0x11` | `0x02` | setting 17 (the initialiser stores 0, the fix-up raises it) |
| `+0x1f` | `0x00` | partition 0 |
| `+0x20` | `0x05` | setting 5 |
| `+0x21` | `0x0a` | setting 6 = 1, setting 8 = 1 |
| `+0x22` | `0x00` | brick type unset |
| `+0x17` | see below | bits 7:4 = `0xf` (setting 60, the router role nibble) |
| `+0x2e` | see below | settings 123 and 124 |
| `+0x6f` | checksum | |
| `+0x70` | mirror | copy of `+0x00` to `+0x6f` |

Record 3 is all zero except byte 0 = 3 (and its checksum). Record 2 is as in the pointer rule above.

**The clock decides three defaults.** The initialiser calls the clock reader (`0xffe5e32c`) once. The reader sets the latch (bit 6 of the control byte at window `0x7f8`) for the duration of the read and clears it again, decodes the BCD registers at window `0x7f8` to `0x7ff`, and returns -1 when every calendar field is zero or the year is outside 1930 to 2500. The control byte holds the thousands digit of the year in bits 5:4 and the hundreds digit in bits 3:0. An all-`0xff` window reads as year 4665, outside the range, so it gets the blank-clock defaults. A blank window reads as -1.

| Clock reader | `+0x17` | `+0x2e` |
|---|---|---|
| -1 (blank or invalid) | `0xfa` (bits 2:1 = 01, bit 3 set) | `0x7e` |
| valid date | `0xf4` (bits 2:1 = 10, bit 3 clear) | `0x02` |

Bit 3 of `+0x17` is setting `0x50`. The power-up code skips the brick serial-number check when it is non-zero (`0xffe53108`, after the test for brick type `A`), so a window rebuilt with no valid clock disables that check and one rebuilt with a valid clock enables it. That consequence is inferred: the check was read only as far as its early exits (see [power-reset-ibrick-rbrick.md](power-reset-ibrick-rbrick.md)). The routine also logs "Brick Serial Number not set in EEPROM" and returns success when the board identity record supplies no brick serial number (`0xffe55d08` returns null; E for the branch, `0xffe53122` to `0xffe53142` and the return at `0xffe53416`). Settings 123 and 124 (`+0x2e`) were not traced to their readers (U).

## Writing a window from nothing

Because the firmware repairs whatever fails, a writer that wants given rack, bay, partition and brick type produces an image the firmware accepts without repair:

1. Header `01 01 00 00 00 00 00 fe`.
2. Record 1 at window `0x008` with the defaults above plus the chosen fields, and the checksum at `+0x6f` set so the 112 bytes sum to 0 modulo 256; its mirror at window `0x078`.
3. Record 2 (36 bytes) at window `0x0e8`: byte 0 = 1; bytes `+0x04` to `+0x07` = `20 00 02 98`; `+0x08` to `+0x0b` = `20 00 07 f8`; `+0x0c` to `+0x0f` and `+0x10` to `+0x13` = `20 00 02 98`; checksum at `+0x23`. Mirror at window `0x10c`.
4. Record 3 (180 bytes) at window `0x130`: byte 0 = 3, checksum at `+0xb3` (window `0x1e3`). Mirror at window `0x1e4`. A record 3 whose byte 0 is not 3 zeroes record 1's brick serial (`+0x68` to `+0x6e`), so write record 3 whenever record 1 holds a serial.
5. A clock that reads a valid date at window `0x7f8` to `0x7ff`, if the valid-date defaults are wanted.

The chosen fields, relative to record 1:

| Field | Where | Width | Boot use |
|---|---|---|---|
| rack | big-endian word at `+0`, bits 23:14 (byte 1 and the top two bits of byte 2) | 10 bits | The boot task builds the 16-bit router address as `(rack & 0x3ff) << 6 \| (bay & 0x3f)` (`0xffe105d8` to `0xffe10602`) and gives it to the router (`0xffe78464`). Rack and bay are also copied into the first module record (`0xffe3140a` to `0xffe3143e`). E |
| bay | byte `+2`, bits 5:0 | 6 bits | as above. E |
| partition | byte `+0x1f` | 8 bits | copied into the first module record; a value of `0xff` or more is replaced by 0 and written back to the setting (`0xffe31446` to `0xffe3146a`). E |
| brick type | byte `+0x22` | 1 character | read as setting 9 by the identity routine only when the board identification records name no brick; see [personalities.md](personalities.md) and [cbrick-mbus-registration.md](cbrick-mbus-registration.md). The accepted characters include `C` (`0x43`). E for the location and the fall-back role |
| brick sub-type | byte `+0x47` | 1 character | setting 16, kept as the pseudo-type only when it is one of the 31 accepted letters. E |
| auto power-up | byte `+0x04` bit 7 | 1 bit | setting `0x32`; the automatic power-up task is created only for the listed brick types and only when this is non-zero (`0xffe30044`). Blank: 0. E |

Rack, bay and partition have no range check at start beyond the field widths and the partition rule above. The brick and system serial numbers are settings 15 (`+0x68`, seven bytes) and 13 (`+8`); both are blank after a rebuild.

## What is not settled

- The readers of settings 123 and 124 (the chain sets them to 2 and `0xf`), the meaning of settings 170 to 175 and the date arithmetic of the record 3 checker.
- Package 1's table at `0xffe5a858` (the chain was read in package 0).
- Whether the setter in package 0 matches package 1's (the getter and the start-up code do).
- No case was found where the firmware compares a primary with its mirror when both pass: the primary wins at the repair step and the mirror is overwritten (E from the copy direction).
