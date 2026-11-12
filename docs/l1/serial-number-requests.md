# Serial numbers on the L1: requests 0x0004 and 0x0008, the formats they carry, and who consumes them

Question. What does an IP53 or IP59 compute brick's L1 answer to the brick-serial request (code 4) and the
system-serial request (code 8)? What do the IP35 PROM and the IRIX kernel do with those strings (length, character
classes, a letter prefix, a checksum)? What do real Origin 350 and Tezro units print?

Short answer.

- Request 8 returns the stored system serial number (SSN) verbatim; request 4 returns the brick serial number (BSN),
  which on the IP53/IP59 bricks is a verbatim copy of the serial text in the first board EEPROM record, the interface
  board's. Neither reply is checked, padded or checksummed. An empty value is the error `-104`.
- The SSN format is enforced only where it is written or used to power up, not where it is read: eight characters, a
  letter, one of `0` to `3`, then six digits. The letter is fixed per brick family (`M` for the 2U Origin 350 profile,
  `P` for the workstation Tezro profile). There is no checksum.
- The BSN is a six-character board serial: three uppercase letters then three digits (all 54 distinct barcodes of
  IP53, IP59, interface and IO9 boards in the retained captures fit, first letter `M`, `N` or `R`). It carries no
  checksum the firmware tests; the only test on the EEPROM field is its type byte.
- The PROM and the kernel treat both strings as opaque text. The kernel copies at most twelve bytes and turns bytes 4
  to 11 into a 32-bit host identifier by reading them as hexadecimal digits.

Sources. L1 firmware 1.44.0 as shipped with IRIX 6.5.30 (`l1.bin`, container sha256
`c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`): package 1 (`Fuel/PE/O300 1MB image`) and package 2
(`MIPS 2MB image`, the compute-brick package for IP35, IP53 and IP59), ColdFire, body address = 0xffe00000 + offset.
IP35 PROM SGI Version 6.210 (unpacked segment sha256 `554924380189a03a512156af2faa38344902f251b49fd5c80c253c23ecaff27b`).
IRIX 6.5.30 IP35 boot kernel objects `kernel.o`, `l1_command.o` and `eeprom.o` (symbol names, `.text` offsets).
Published: SGI L1 and L2 Controller Software User's Guide, 007-3938-006, printed pages 93 and 94 (the `serial`
command) and 131 (L2 `serial set`, example `L0000010`). Published console captures of `serial all`, `hinv -v` and PROM
inventories from Origin 350, Onyx 350, rack and tower Tezro and Origin 300 systems, described generically below.

## The two L1 requests

Established (package 2 read at the handlers; package 1 handler shape matches).

| Code | Handler (package 2) | Reply | Grade |
|---|---|---|---|
| `0x0004` brick serial | `0xffe145f8` | For brick types `6` and `@` the per-node serial routine `0xffe76048`; for every other type the brick-serial routine `0xffe7616c`. A null result is error `-104`; otherwise status 0 and the text. | established |
| `0x0008` system serial | `0xffe14b98` | If a hook is installed at `0x3000281c + 0xe0` it supplies the string, otherwise persistent setting `0xd`. An empty string is error `-104` (`moveq #$98` at `0xffe14bd4`); otherwise status 0 and the string with its terminator. | established |

The SSN is stored in the persistent record as eight characters plus a terminator (setting `0xd`). Established in
package 1: the setter at `0xffe58a86` clears nine bytes and truncates a longer input at eight characters
(`0xffe58aa6` to `0xffe58ab8`). The package 2 setter was not re-read (inferred the same).

Brick-serial routine (`0xffe7616c`, package 2). Established. The cached result lives at `0x300008cc + 4`. For most
brick types, including `7` (the 2U interface board `2U_INT_53`) and `9` (the workstation board `WS_INT_53`), the
routine selects board record 0, and when that record is a board record (kind 1) with a serial pointer at record
`+0x50` it returns a copy of that string (`strdup` at `0xffea682c`). Records are 0x78 bytes apart. Type `6` selects
record 1. Types `1` and `2` build `"%3.3s%1d%02d"` from a prefix and two counters (`0xffe76226` to `0xffe76392`) and type
`T` returns the constant `TTT000` (`0xffe7650e`); neither applies to IP53 or IP59.

Record 0 is the first board listed by `serial all`. On the 2U profile that is `INTERFACE` (`2U_INT_53`). Matches the
captures: in every capture where the line is not redacted (7 of 7) the `Local Brick Serial Number` equals the
serial of the `INTERFACE` row; three posts show a one-digit difference between the two lines that is consistent with
the poster masking digits (e.g. `MYE999` against `MYE900`).

Width of the brick serial (established for the decoder's code, L1 1.44.0, package 2). The board-area decoder `0xffe8d18c` skips the manufacturer field (its length byte is at area offset 6), then copies each following text field into a heap block of (type and length byte & `0x3f`) + 1 bytes and terminates it: the second field is stored at record `+0x4c` (`0xffe8d1f8` to `0xffe8d224`) and the third, the serial that the brick-serial routine returns, at `+0x50` (`0xffe8d23e` to `0xffe8d246`, copied by the same steps). The width is the field's own six-bit length, 0 to 63 characters, with no fixed buffer and no truncation in the decoder; the instructions read do not test the two type bits. Whether the type bits are tested before the decoder runs was not read here.

## Formats

### System serial number (SSN)

Established, package 1 (`Fuel/PE/O300`), validator `0xffe2f968` called from the `serial set` branch; constants read
today. Applies when a required first letter is set (the byte at `0x30001f20 + 0xc`; zero means no requirement and the
validator accepts anything):

1. length exactly 8 (`strlen - 8`, `0xffe2f984` to `0xffe2f98e`);
2. the first character equals the required letter (the lookup at `0xffe2fa26` to `0xffe2fa3c`);
3. characters 2 to 8 are not `9999999` and not `3999999` (those suffixes mark a disabled brick; compared at
   `0xffe2fa42` to `0xffe2fa6e`);
4. character 2 is `0` to `3` (`0xffe2fa74` to `0xffe2fa8c`);
5. characters 3 to 8 are digits per the C library character table (`0xffe2fa90` to `0xffe2faf4`).

The failure text is `ERROR: invalid system serial number '%s'` then `System serial number must be [LMN]0000000
through [LMN]3999999` (package 2 strings at `0x2a33a` and `0x2a364`; the validator is narrower than that text: one
letter per family, and the exact suffix `3999999` is refused). Package 2's strings name `L0000000` as the cleared value
(`0x2a430`, `0x2a466`) and `L3999999` as the value that disables a brick (`0x287c1`); the stores themselves were read
only in package 1 in an earlier pass.

Pre-power check, package 1 `0xffe52d18`, called from the power-up path. Established (constants and return codes read
today): skipped when the brick type is `A` or setting `0x50` is nonzero; with a required letter the first character
must equal it, otherwise it must lie between `L` (0x4c) and `R` (0x52); character 2 is `0` to `3`; characters 3 to 8
are digits; the length is not tested (only eight characters are examined). Failures: BSN mismatch `-2100`
(`0xfffff7cc`), SSN differing from the reference value learned from attached bricks `-2101`, disabled suffix `-2102`,
bad format `-2103`, no assignment (equal reference and suffix `0000000`) `-2104`. Each makes power-up fail before
the hardware is touched. The reference-value rules ("learn" from attached C-brick or L2) were read in package 1 in an
earlier pass and are not repeated here.

Package 2 forms (established, L1 1.44.0, package 2, read in full). The `serial set` validator `0xffe35140` returns 1 at once when the required-letter byte is zero (`0xffe35152`). Otherwise it needs length exactly 8 (`0xffe3515c` to `0xffe35164`), then a first character found by a string search (`0xffea7210`) in a local string that starts with the required letter, characters 2 to 8 not equal to `9999999` or `3999999` (strings at `0xffe3574d`, `0xffe35755`, compared at `0xffe3521a` to `0xffe35242`), character 2 from `0` to `3` (`0xffe3524c` to `0xffe35264`) and characters 3 to 8 digits by the character table (`0xffe35268` to `0xffe352cc`). That is the package 1 shape. Two differences: for brick type `7` the local string is extended with `P` when the byte at `0x300008ce` is 3 (`0xffe3518e` to `0xffe351b0`), so a type 7 module in that state also accepts `P` as first letter; and for type `8` an input that starts with `P` and whose other seven characters are not `0000000` runs an extra pair of calls (`0xffe795e8` with 0x7a, `0xffe76e68`) whose roles were not read. The routine stores only the first byte of the local string, so the search also scans whatever the stack holds after it (observed from the instruction sequence, not run).

The pre-power check `0xffe7352c` has the package 1 form: it is skipped for brick type `A` (`0xffe73560`) and when setting `0x50` is nonzero (`0xffe73568` to `0xffe73576`); compares six characters of the brick serial with the first board record's serial (`-2100`, `0xffe735e2`); rejects the `9999999` and `3999999` suffixes (`-2102`, `0xffe73628`); with the required byte nonzero demands first character equal to it, otherwise first character from `L` to `R` (`0xffe7364e` to `0xffe73662`); character 2 from `0` to `3` and characters 3 to 8 digits (`0xffe73664` to `0xffe736f6`); any of these failing returns `-2103` (`0xffe73772`); a differing reference serial returns `-2101` (`0xffe7381c`). The `-2104` case was not read.

Required first letter by brick family. The profile set-up functions store the letter into the shared structure.
Package 2: the dispatch at `0xffea38f6` to `0xffea3970` sends type `7` to `0xffe5f858`, which stores `M` at
`0xffe5f890`, and type `9` to `0xffe61bc0`, which stores `P` at `0xffe61bd8` (E for the stores and the dispatch). The
validator and the pre-power check in package 2 read that same structure field. Established (L1 1.44.0, package 2): the structure is at `0x3000281c` (the type dispatch is called with that address, `0xffe75fae`) and its required-letter byte at `+0xc` is `0x30002828`; the validator `0xffe35140` tests it at `0xffe35152` and the pre-power check `0xffe7352c` at `0xffe73632`. The profile stores above (`0xffe5f890`, `0xffe61bd8`) write `+0xc` of the structure passed in.
Package 1 stores `M` for the Origin 300 node board (`IP45`).
Earlier reading, not repeated: `L` for the original `IP35` C-brick profile.

Real values. Origin 350 and Onyx 350 2U modules print `M` plus seven digits (`M2100629`, `M2008888`, `M2100887`,
`M1234567` set by an owner); an Onyx 350/InfiniteReality-class system with an `IP57` node prints `R2003602`. The
user guide's own example is `L0000010`. No retained capture shows a tower Tezro's SSN. Unknown: whether tower Tezros
ship with a `P` serial; the `P` letter is established only as the constant the workstation profile stores.

### Brick serial number (BSN) and board serials

Established. The board EEPROM's serial is a field of a type-and-length encoded board information area. The PROM's
reader of that area (`0x1fc505a4` to `0x1fc505e4`) requires the type bits `0xc0` (8-bit text) and takes the length
from the low six bits, otherwise it logs "bad serial number format in EEPROM" and skips the record. The L1 passes the
text on without further tests. Neither examines a character class or a checksum.

Real values, inferred as the format of production boards: 54 of 54 distinct barcodes printed for `IP53_4CPU` (12),
`IP59_4CPU` (8), `2U_INT_53` (17), `WS_INT_53` (2) and `IO9` (15) boards in the retained `hinv -v` and `serial all`
captures match three uppercase letters then three digits (first letter `M` 18, `N` 19, `R` 17). Tower Tezro captures
print `NER934` and `NET662` (`IP53_4CPU` boards) and `NBY982` and `NKX689` (`WS_INT_53`). Power-supply serials in the
same listings are longer (`XPD0323001887`). `R`-prefixed serials appear on `IP59_4CPU` revisions -C and -D and on
`2U_INT_53` revision -D, `M` and `N` on the earlier revisions. What the letters mean in the factory is not in any
shipped image (unknown).

## What the PROM does with them

Established (PROM 6.210).

- System serial: the request builder `0x1fc49cf8` sends code 8; its only caller (`0x1fc4aa8c`) copies the reply
  into a stack buffer and sets it empty on failure. The result is passed through `0x1fc49c90`, which adds `0x6e`
  to each of ten bytes in a fixed interleaved order and stores them at offset `0x38` of the record being built. No
  format test.
- Board serial text: the formatter at `0x1fc505a0` to `0x1fc506fc` writes `Serial:<text>` into the node record string.
- The PROM also calls the brick-serial request for the primary node identity unless the `OverrideNIC` variable is
  set (read earlier; not re-traced). The strings "Warning: found invalid serial number, possible missing or
  invalid NIC" and "Could not find serial num struct" belong to code in the BaseIO image, which was not read for this
  note (unknown).

## What the IRIX kernel does with them

Established (IRIX 6.5.30 IP35 boot kernel; symbol names and `.text` offsets).

- `iobrick_get_sys_snum` (`l1_command.o` `+0x2730`) opens a system-controller channel, builds a general request with
  function 6 and code 8, and passes the reply text through `sc_interpret_resp`; any failure becomes `-100`.
- `get_kmod_sys_snum` (`kernel.o` `+0x251e4`) fills a buffer for one module. If the board identification register
  class (bits 15:12) is `0x4000` it copies up to twelve bytes of the cached reply with no test; for other classes it
  decodes the ten-byte shuffled-and-offset form with `decode_str_serial` (`+0x24a74`, subtracts `0x6e` per byte, the
  inverse of the PROM's transform). `encode_str_serial` (`+0x249f8`) is the writer.
- `getsysid` (`+0x690c`) zeroes 64 bytes and calls it; `getsysid_snum` (`+0x6978`) reads bytes 4 to 11 of that
  buffer and builds a 32-bit value by shifting in one hexadecimal digit per byte (`0`-`9`, `a`-`f`, `A`-`F`; any other
  byte contributes zero). No letter, length or checksum test. Inferred consequence: with an eight-character SSN in
  bytes 0 to 7 and zeros after it, only characters 5 to 8 are hexadecimal digits in bytes 4 to 7, so `M2100887` would
  give `0x08870000`. The consumer of the 32-bit value was not traced here.
- `eeprom.o` carries `fake_a_serial_number` (`+0x1900`), the routine that fabricates a serial for hardware without an
  EEPROM; it is the reason the PROM's board record can hold text that no EEPROM supplied.
- No kernel path in `l1_command.o` sends request code 4; the brick serial reaches IRIX through the PROM-built board
  records only (inferred from the absence of a caller in the three objects).

## Not settled

- Whether a tower Tezro's SSN starts with `P`. Route: a `serial all` or `hinv` capture from one, or a package 2 profile that stores a different letter. The package 2 validator and pre-power check were read (above); for brick type `9` they require `P`.
