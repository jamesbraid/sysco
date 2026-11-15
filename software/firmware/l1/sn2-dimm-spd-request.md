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

Ids 8 to 13 map to entry id minus 2, ids 16 to 21 to id minus 4. Every other id, including 14, 15, 22, 23 and anything above 99, maps to -1. Each DIMM entry is `select | address << 8 | 0x100 << 16`, type code 2 (the high halfword `0x100` is the size of the buffer, 256 bytes). The meaning of the select byte (an I2C multiplexer or bus choice) and of the address byte (`0xa8` is `0x54` in seven-bit form, the usual serial EEPROM base for a DIMM) is inferred from the values, not read from a bus routine.

The four DIMMs of the node's first bank are ids 8, 16, 9, 17, which are entries 6, 12, 7, 13.

## Where the bytes come from

The record for a DIMM is a 256-byte buffer the L1 reads from the DIMM's EEPROM (routine `0x12e5c0`, a 256-byte allocation, read from offset 0 through the EEPROM access routine `0x12c9dc` in pieces of at most 32 bytes). **Established** for the routine; **inferred** that this runs when the L1 builds or refreshes its EEPROM table and not once per request (the request handler only reads the stored record; the loop that calls the loader, `0x12c470`, was read, its trigger was not).

Two checks decide whether the record is published, both in `0x12e5c0`:

1. The EEPROM read must succeed. If not, the record gets error code 2, its buffer is freed, and requests for it reply -107.
2. **Checksum** (established): the low byte of the sum of bytes 0 to 62 (63 bytes) must equal byte 63. If not, the record gets error code 3, the log line is `SPD checksum error`, the buffer is freed, and requests reply -107. This replaces the sentence in the earlier note that puts the compare at index 62: the loop runs the counter from 0 up to and including `0x3e` and then compares the byte after the 63 summed, which is byte 63.

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
- the frame writer drops an argument that would push the frame past 127 bytes (the check is on the running length before the raw tag, so the largest raw block after the status and the integer is 116 bytes, giving a 128-byte payload). The dropped argument is still counted in the count byte (inferred from the loop tail at `0xc4550`). A request of 48 bytes never reaches this.

The handler adds no clamp of its own at 64; the 64-byte limit is the node's.

## Error replies

**Established.** An error reply is a normal frame with the status in bytes 1 to 4 and a count of 0: 6 bytes, `0x20 | subchannel`, the status big-endian (`0xffffff98` for -104), `0x00`. Statuses for a DIMM request, in the order tested:

| Status | When |
|---|---|
| -100 (`0xffffff9c`) | request has four or fewer arguments counting the code |
| -104 (`0xffffff98`) | the table entry is not present: this covers every id that maps to -1 (the mapped index is then `0xff` as a byte, which is at or above the entry count), ids 14, 15, 22, 23, and a DIMM slot with its presence flag clear. Also: a selector other than 4 on a DIMM entry, because the record type is 2 |
| -105 (`0xffffff97`) | selector above 4; offset at or above 256. A table index at or above the entry count would also give -105 but cannot occur after the presence test, which uses the same count (inferred from reading both) |
| -107 (`0xffffff95`) | the entry is present, selector 4, but the record failed to load: EEPROM read error or bad checksum |

Separately, a request the L1 cannot route gets the nine-byte transport error (status `0xffffffff` then a four-byte code), described in [sn2-prom-link-1.62.1.md](sn2-prom-link-1.62.1.md).

On the node's side every nonzero status makes the DIMM read fail: `0xffa843f0` turns -107 into return code 3 and everything else into 1, and the caller only tests for zero, so the slot counts as empty either way.

## Not settled

- What sets the presence flag, and when the EEPROM table is built.
- Whether the select byte in the table entry is an I2C multiplexer channel.
- Whether a record that exists with a null buffer (never produced by the loader) would send no reply: the handler returns without a reply in that case (inferred from the count word test, not triggered by any input).
- The same table for module types other than `F`.
