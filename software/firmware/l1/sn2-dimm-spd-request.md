# The DIMM SPD read (request `0x0006`, selector 4) of the ARM L1 1.62.1, byte by byte

The node firmware sizes its memory by asking the L1 for the serial-presence-detect (SPD) bytes of each DIMM. This note gives the request and the reply as the Linux L1 serves them, so an emulated L1 can answer in a way the node firmware accepts. The general frame, the argument encoding and the other requests are in [sn2-prom-link-1.62.1.md](sn2-prom-link-1.62.1.md); this note adds the DIMM case and corrects one detail of the checksum there. Grades follow [AGENTS.md](../../../AGENTS.md).

Sources, disassembly only (nothing was run): the `Linux L1 image` (package 4) of `l1-1.62.1.bin`, its `/stand/sysco/bin/l1` (ARM, SHA-256 `22cb62871e447a394acbd662f4f49f3146580afec93839953b37a5354545bfc5`, [sn2-l1-1.62.1-packages.md](sn2-l1-1.62.1-packages.md)), addresses are virtual addresses with base `0x8000`; and, for the node's end of the exchange, the SAL_B image of SHub 1 PROM 10.54 (SAL 5.04), addresses with base `0xffa00000`.

## The request

**Established** (handler `L1ReqIRouter` code `0x0006`, `0xc9658` to `0xc9dd8`; node side `0xffa843f0` and its wrapper `0xffa84680`).

The node sends task 6, code `0x0006`, four integer arguments, in this order:

| Argument | Value for a DIMM | Meaning at the L1 |
|---|---|---|
| 1 | device id, 8 to 23 | table lookup (below) |
| 2 | selector 4 | the DIMM's raw SPD record |
| 3 | byte offset, 0 | start inside the 256-byte record |
| 4 | length, 48 | bytes wanted; the node never asks for more than 64 in one request, and a 48-byte read is one request |

The L1 handler replies -100 unless the request has more than four arguments counting the code (the L1's own record counts the code as argument 0, so four integers pass). Wire byte 7 of the request is then 4 (inferred: the node's builder keeps a word count of 8 (an integer is two words) that is not what goes on the wire, [node-link-sn1-vs-sn2.md](node-link-sn1-vs-sn2.md)).

## Device id to table entry

**Established** for module type `F` (the Opus TIO brick, which the earlier note takes to be the Altix 350 node), mapping routine `0xc7664`, jump table at `0xc7fe8`; the table `OpusTio_eepromCfg` (`0x16550c`, 22 entries of 12 bytes) read in full.

| Device id | Table entry | Name | EEPROM select byte | EEPROM address byte |
|---|---|---|---|---|
| 8, 9, 10, 11 | 6, 7, 8, 9 | DIMM0 L to DIMM3 L | `0x23` | `0xa8`, `0xaa`, `0xac`, `0xae` |
| 12, 13 | 10, 11 | DIMM4 L, DIMM5 L | `0x43` | `0xa8`, `0xaa` |
| 16, 17, 18, 19 | 12, 13, 14, 15 | DIMM0 R to DIMM3 R | `0x33` | `0xa8`, `0xaa`, `0xac`, `0xae` |
| 20, 21 | 16, 17 | DIMM4 R, DIMM5 R | `0x43` | `0xac`, `0xae` |
| 14, 15, 22, 23 | none | none | none | none |

Ids 8 to 13 map to entry id minus 2, ids 16 to 21 to id minus 4. Ids 0, 1, 4 to 7, 98 and 99 map to other entries (not DIMMs); every other id, including 14, 15, 22, 23 and anything above 99, maps to -1. Each DIMM entry is `select | address << 8 | 0x100 << 16`, type code 2 (the high halfword `0x100` is the size of the buffer, 256 bytes). The meaning of the select byte (an I2C multiplexer or bus choice) and of the address byte (`0xa8` is `0x54` in seven-bit form, the usual serial EEPROM base for a DIMM) is inferred from the values, not read from a bus routine.

The four DIMMs of the node's first bank are ids 8, 16, 9, 17, which are entries 6, 12, 7, 13.

## Where the bytes come from

The record for a DIMM is a 256-byte buffer the L1 reads from the DIMM's EEPROM (routine `0x12e5c0`, a 256-byte allocation, read from offset 0 through the EEPROM access routine `0x12c9dc` in pieces of at most 32 bytes). **Established** for the routine; **inferred** that this runs when the L1 builds or refreshes its EEPROM table and not once per request (the request handler only reads the stored record; the loop that calls the loader, `0x12c470`, was read, its trigger was not).

Two checks decide whether the record is published, both in `0x12e5c0`:

1. The EEPROM read must succeed. If not, the record gets error code 2, its buffer is freed, and requests for it reply -107.
2. **Checksum** (established): the low byte of the sum of bytes 0 to 62 (63 bytes) must equal byte 63. If not, the record gets error code 3, the log line is `SPD checksum error`, the buffer is freed, and requests reply -107. This replaces the sentence in the earlier note that puts the compare at index 62: the loop runs the counter from 0 up to and including `0x3e` and then compares the byte after the 63 summed, which is byte 63. Micron's 512 MB PC3200 UDIMM datasheets follow the same rule: byte 63 is `0xC1` in MT8VDDT6464A Table 17 (Rev. E, p. 24) and `0xD3` in MT9VDDT6472A Table 21 (Rev. B, p. 26), each the low byte of the sum of bytes 0 to 62.

An emulated L1 therefore needs a 256-byte image per DIMM with a valid checksum in byte 63. Bytes 64 to 255 are served only if the node asks for them; the node asks for 0 to 47.

Whether a DIMM is "present" is a one-byte flag per table entry (`eepromIsPresent`, `0x12c960`: the flag array of the slot's table, indexed by the entry number, zero when the number is at or above the entry count). What sets the flag, and the 256-byte record's trigger above, were not traced: unknown (probably a bus probe).

## The reply

**Established** (`L1ReqIRouter` handler body `0xc9658` to `0xc9dd8`; helpers `0x9df34`, `0x9df94`, `0x9e03c`; frame writer `syscom_process_outbound_response`, `0xc3f64`, read for the integer, raw-block and overflow paths). Offsets are in the frame's PPP payload as the L1 sends it.

Success for id 8, selector 4, offset 0, length 48 (60 bytes):

| Byte | Value |
|---|---|
| 0 | `0x20` plus the request's subchannel index |
| 1 to 4 | status 0, big-endian |
| 5 | argument count after the status: 2 |
| 6 | `0x00`, the integer tag |
| 7 to 10 | the bytes left after this chunk, big-endian: record size minus offset minus length, floored at 0 (`0x100 - 0 - 48 = 208 = 0x000000d0`) |
| 11 | `0xb0`: the raw-block tag, `0x80 \| 48` |
| 12 to 59 | SPD bytes 0 to 47 |

The node's parser (`0xffa82270`) reads the same layout after dropping byte 0: status in bytes 0 to 3, the count in byte 4, then the arguments, and it requires the count to be exactly 2 for this call (any other count gives -33).

Length handling, in the order the handler applies it:

- offset at or above the record size (256): status -105, no data;
- offset plus length above 256: length is cut to `256 - offset` and the "bytes left" integer is 0;
- length 0: the raw argument is left out and the count is 1;
- the frame writer drops an argument that would push the frame past 127 bytes (the check is on the running length before the raw tag, so the largest raw block after the status and the integer is 116 bytes, giving a 128-byte payload). The dropped argument is still counted in the count byte (established: the loop tail at `0xc4550` increments the count after the overflow jump). A request of 48 bytes never reaches this.

The handler adds no clamp of its own at 64; the 64-byte limit is the node's.

## Error replies

**Established.** An error reply is a normal frame with the status in bytes 1 to 4 and a count of 0: 6 bytes, `0x20 | subchannel`, the status big-endian (`0xffffff98` for -104), `0x00`. Statuses for a DIMM request, in the order tested:

| Status | When |
|---|---|
| -100 (`0xffffff9c`) | request has four or fewer arguments counting the code |
| -104 (`0xffffff98`) | the table entry is not present: this covers every id that maps to -1 (the mapped index is then `0xff` as a byte, which is at or above the entry count), ids 14, 15, 22, 23, and a DIMM slot with its presence flag clear. Also: a selector other than 4 on a DIMM entry, because the record type is 2 |
| -105 (`0xffffff97`) | selector above 4; offset at or above 256. A table index at or above the entry count would also give -105 but cannot occur after the presence test, which uses the same count (established: `0x12c960` returns 0 for an index at or above it, and the handler masks the index to a byte first) |
| -107 (`0xffffff95`) | the entry is present, selector 4, but the record failed to load: EEPROM read error or bad checksum |

Separately, a request the L1 cannot route gets the nine-byte transport error (status `0xffffffff` then a four-byte code), described in [sn2-prom-link-1.62.1.md](sn2-prom-link-1.62.1.md).

On the node's side every nonzero status makes the DIMM read fail: `0xffa843f0` turns -107 into return code 3 and everything else into 1, and the caller only tests for zero, so the slot counts as empty either way.

## Not settled

- What sets the presence flag, and when the EEPROM table is built.
- Whether the select byte in the table entry is an I2C multiplexer channel.
- Whether a record that exists with a null buffer (never produced by the loader) would send no reply: the handler returns without a reply in that case (inferred from the count word test, not triggered by any input).
- The same table for module types other than `F`.

## Review

Independent review of this page as committed in sysco `08df38c`. The reviewer wrote none of it. Method: package 4 of `l1-1.62.1.bin` was carved (container offset `0x7ecf04`, length `0x4b80f4`, sha256 `8c964136...`), its CramFS read with the repository's CramFS reader, and `/stand/sysco/bin/l1` (sha256 `22cb6287...`, matches) decoded with the retools ARM mode (Capstone), base `0x8000`. The routines read in full were `0xc9658` to `0xc9dd8`, `0xc7664` with its jump table, `0x12e5c0`, `0x12c908`, `0x12c960`, the frame helpers `0x9df34`, `0x9df94`, `0x9e03c` and the frame writer `0xc3f64`. The node side (`0xffa843f0`, `0xffa82270`, `0xffa84680`) was read in the 10.54 SAL_B image to check the reader. No firmware bytes or listings are kept.

**Confirmed.**

- Handler order: argument count at `+0x20` of 4 or fewer gives -100 (the slot count includes the code, so code plus four integers is 5); presence flag clear gives -104 (the index is masked to a byte there, so -1 reads as 255); index at or above the count gives -105; selector above 4 gives -105; selector 4 needs record type 2 (-104 otherwise), a nonzero error code gives -107, a null buffer sends the status already stored. Statuses `0xffffff9c`, `0xffffff98`, `0xffffff97`, `0xffffff95`.
- Length rules: offset at or above the record size (unsigned) gives -105; offset plus length above it cuts the length to size minus offset and sets "left" to 0, otherwise left is size - offset - length (208 for the 48-byte read); length 0 omits the raw argument.
- Frame (`0xc3f64`): byte 0 `0x20 | index`; bytes 1 to 4 the status; byte 5 the count, incremented for each argument including one dropped at overflow; integer is tag 0 and four bytes big-endian (guard: running length plus 5 must not exceed 127); raw is tag `0x80 | n` then n bytes (guard: running length plus n must not exceed 127, so 116 after the status and integer, a 128-byte payload). The 60-byte success frame and the 6-byte error frame are as the page lays out.
- "Left" is 256 - offset - length, and the node needs it only because its parser demands an integer and a raw block (count exactly 2); for a single 48-byte request the node never uses the value.
- Device id table: `OpusTio_eepromCfg` entries 6 to 17 match the select, address and size words and the names; the type-F jump table at `0xc7fe8` maps 8 to 13 to id minus 2 and 16 to 21 to id minus 4 (see the fix below).
- Loader `0x12e5c0`: 256-byte buffer, error code 2 on a read failure and 3 on a checksum failure (both free the buffer), 4 if the allocation fails; the checksum loop runs the counter from 0 to `0x3e` and compares byte 63. The older text in the link page ("compare at index 62") was in item 4 of that page's section on the EEPROM request; `08df38c` corrects it and the correction is right. The ColdFire L1's SPD check (`mbus.md`) agrees with the byte 63 rule.
- The example checksum in the chips page: its eight non-zero bytes sum to `0x250`, so byte 63 is `0x50` if all other bytes of 0 to 62 are 0.

**Mechanical fixes made in this review (this page).**

1. The id sentence said every other id, including anything above 99, maps to -1. Ids 0, 1, 4 to 7, 98 and 99 map to other table entries; they are not DIMMs.
2. Two "inferred" grades are now established: the dropped argument is counted (loop tail `0xc4550`) and the -105 index case cannot follow the presence test (`0x12c960` returns 0 above the count).

**Findings not fixed.**

1. Byte 0 of the reply is `0x20 | c119c(...)`, where the helper maps the request's subchannel through a per-module-type table (`0xc10bc`, `0xc11d4`); the page's "the request's subchannel index" holds if that mapping is the identity, which was not shown (the link page grades it inferred).
2. The link page still says the -100 test is on a count of zero; for this handler it is a count of 4 or fewer. That page is outside this review.
3. The page's note that wire byte 7 of the request is 4 was not re-checked.
4. The eeprom record trigger and the presence flag writers remain unread.

**Not checked.** The EEPROM access routine `0x12c9dc` beyond its 32-byte piece size, the request receive path, other module types.

**Cleared scope.** The request arguments, the id-to-entry map for type F, the handler checks and statuses, the reply frame byte by byte, the length cut and the 116-byte limit, the checksum rule and its correction of the link page. Not cleared: the subchannel mapping in byte 0, and whether the loader runs on request.
