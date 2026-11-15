# The node-to-L1 link on SN1 and SN2, side by side: the ColdFire L1 1.44.0 against the ARM Linux L1 1.62.1

The Origin 300 and Origin 3000 nodes (SN1, Bedrock chip, MIPS) talk to a ColdFire L1; the Altix nodes (SN2, SHub chip, IA-64) talk to an ARM Linux L1. Both ends of both links use the same byte framing and request vocabulary, but not the same everything. This note lists, item by item, what is identical, what differs (with the exact difference) and what is unknown, so that nobody has to assume identity. Grades follow [AGENTS.md](../../../AGENTS.md): **established** (the whole routine was read), **inferred** (reasoning stated), **unknown**. "Same" and "different" are always about the bytes of the cited routines.

Sources, all disassembled, nothing run:

- **ColdFire L1.** L1 1.44.0, package 1 (`Fuel/PE/O300 1MB image`), the committed listing [disasm/mcf5206e-1.44.0-fuel-pe-o300/](disasm/mcf5206e-1.44.0-fuel-pe-o300/), addresses `0xffe...`. Packages 3 and 5 (the SHUB-aware ones) by the existing comparison notes ([personalities.md](personalities.md), [brick-request-handlers.md](brick-request-handlers.md)).
- **ARM L1.** The `l1` program of package 4 of L1 1.62.1 (SHA-256 `22cb62871e447a394acbd662f4f49f3146580afec93839953b37a5354545bfc5`), as in [sn2-prom-link-1.62.1.md](sn2-prom-link-1.62.1.md); addresses `0x8000`-based, routine names from its symbol table.
- **SN1 node firmware.** The IP35 PROM 6.210 payload (1,476,264 bytes, SHA-256 `554924380189a03a512156af2faa38344902f251b49fd5c80c253c23ecaff27b`, the image cut from `ip35prom.img` after its 4,096-byte header, as in [serial-number-requests.md](serial-number-requests.md)). Its code is MIPS64 big-endian; `jal` targets carry the base `0x0fc00000`, so this note names routines by payload offset (`+0xec04`).
- **SN2 node firmware.** The SAL runtime of snprom 10.54 (SAL 5.04) exactly as [sn2-prom-link-1.62.1.md](sn2-prom-link-1.62.1.md) and its two reviews read it. Not re-read here; every SN2 node-side statement below is quoted from that page and carries its grade.

## Summary table

| # | Item | Verdict | Grade |
|---|---|---|---|
| 1 | Transport to the node chip | **different**: SN1 is a quad-UART channel; SN2 is a serial channel of the ARM CPU or, for Opus TIO and NL4R, the chip's scan chain | established for the L1 ends; unknown for the SN1 node end and the ARM channels' line rate |
| 2 | Frame flag, escape, FCS, no `ff 03` prefix | **identical** on all three readable ends | established |
| 2 | Escaped byte set on the node link | **identical**: `0x7e`, `0x7d` only; the flow-control bytes and `0x0d` are not escaped | established (L1 sides and SN1 PROM) |
| 2 | Maximum payload | 127 bytes on every side; **different** in what counts toward it | established for ColdFire L1 and SN1 PROM; unknown for the ARM L1 |
| 3 | Class and index byte, address word, request code, argument count, task numbers | **identical** | established |
| 3 | Which indexes the receiver accepts | **different** on the SN1 PROM (it filters); SN2 node not read | established for the SN1 PROM |
| 4 | Argument types and reply layout (status word, count, arguments, nine-byte error form) | **identical** | established |
| 4 | Status codes | **identical** set and meanings (`-100`, `-101`, `-102`, `-104` to `-108`) | established |
| 5 | Request codes accepted | **different**: 39 codes in the Origin 300 package, 55 in the ARM L1; exact lists below | established |
| 5 | The boot-time codes | shapes **identical** for `0x0001`, `0x0003`, `0x0006`, `0x0009`, `0x0010`; **different** in source and value for `0x000e`; `0x0013` and `0x0015` absent from the Origin 300 package | mostly established |
| 5 | `0x0006` device ids | **different** | established for both tables |
| 6 | Brick type letters | same single character, upper case on the wire; **different** letters per family | established |
| 7 | `0x0001` reply | **identical** shape: status 0 and the stored 16-bit setting 3; the value is data, not a constant | established |

## 1. Transport

| Aspect | SN1, ColdFire L1 1.44.0 | SN2, ARM L1 1.62.1 | Verdict |
|---|---|---|---|
| Node channel at the L1 | channel 3 of the external quad UART, software port 6; client class 3 ([quad-uart-and-serial-links.md](quad-uart-and-serial-links.md)) | a "SysCom" channel opened as `SCC-<n>` (`uartOpen`, called from the routine `init` at `0xc0134`), or, for the Opus TIO and NL4R router families, the node chip's scan chain: `JTAG_UART_write` and `JTAG_UART_read` on the test data registers `WR_SYSCOM` and `RD_SYSCOM` ([sn2-prom-link-1.62.1.md](sn2-prom-link-1.62.1.md)) | different; established |
| Line rate and handshake | 8N1, divisor 13 on a 12 MHz clock (57,692 actual, 57,600 nominal), enhanced-feature value `0xd0` (automatic RTS and CTS); the firmware itself never tests CTS ([node-link-cts-flow-control.md](node-link-cts-flow-control.md)) | not read. `uartOpen` is called with a flags word of `2`, or `0x2000 | 2` for some brick types (`{` among them, `0xc00c4` to `0xc0128`); the rate and handshake were not traced (`uartSetConfig` and its callers) | unknown for ARM; a scan-chain channel has no rate |
| Node end | Bedrock UART; register block and wire not read. In the PROM, the byte reader at `+0xd748` calls a function pointer in the port record (`+0x20`) | the SHub UART at `0x8000000ffed40000 | nasid << 38` with 8-byte register spacing, and a separate TIO UART for Opus TIO ([sn2-prom-link-1.62.1.md](sn2-prom-link-1.62.1.md)) | unknown for SN1 (route: the port record's function pointers in the PROM); established for SN2 |
| Raw (unframed) channels | none read on the node link | channel mode 0 is raw: its bytes become one string argument of a class-2 event on subchannel 4; modes 1 and 3 are framed | different; established |

So an L1 model for SN2 cannot reuse the SN1 transport; the two share the byte stream above it.

## 2. Framing

Read at: ColdFire L1 frame writer `ppp_frame_send` (`0xffe623b4`) and context set-up (`0xffe62330`); ARM `PPPInit` (`0x10de48`), `PPPWrite` (`0x10df80`), `PPPRead`; SN1 PROM frame writer (`+0xec04`) and receive state machine (around `+0xe100` to `+0xe700`); SN2 SAL as quoted.

| Aspect | ColdFire L1 | ARM L1 | SN1 PROM | SN2 SAL (quoted) |
|---|---|---|---|---|
| Flag | `0x7e` at both ends of the frame (`0xffe623e4`) | `0x7e` (`PPPWrite`, `0x10dfc8`) | `0x7e` at both ends (`+0xec3c`, `+0xed4c`) | `0x7e` |
| Escape | `0x7d`, byte xor `0x20` (`0xffe62436`, `0xffe62444`) | same (`0x10e0b0`, `0x10e0d4`) | same (`+0xecc8`, `+0xecdc`) | same |
| Always escaped | `0x7e`, `0x7d` | `0x7e`, `0x7d` (`PPPWrite` compares `0x7d`, `0x7e` by the range test at `0x10e044`) | `0x7e`, `0x7d` | `0x7e`, `0x7d` tested |
| Optionally escaped | `0x11`, `0x13` by context word `+0x20`; `0x0d` by `+0x24` | `0x11`, `0x13` by context word `+0x28`; `0x0d` by `+0x2c` | none: no test for `0x11`, `0x13` or `0x0d` exists in the writer | not read |
| Node-link setting | both off: the node-link context call at `0xffe346ce` to `0xffe34706` passes zero for both flags, offset 0, no growth, 256-byte buffers | both off: the SysCom channel call in `config` at `0xbfd10` passes zero for both (`sp[4]`, `sp[8]`), buffer `0x100` | not applicable | not read |
| Setting on the peer links and other contexts | peers: flags 1 and 0 (flow-control bytes escaped, `0x0d` not), offset `0x10`, limit `0x410` (`0xffe2e07a`) | the context at `0x9b0f0` (not a SysCom channel): flags 0 and 1, offset `0x10`, limit `0x410` | not applicable | not applicable |
| FCS | initial `0xffff`, reflected polynomial `0x8408`, complemented, low byte first (`0xffe623ee`, `0xffe624aa` to `0xffe624ca`); receiver tests `0xf0b8` (`0xffe6276c`) | same; `PPPRead` tests `0xf0b8`; table at `0x198134` | same: init `0xffff` (`+0xec4c`), table-driven update at `+0xd700` (`fcs = table[(fcs ^ byte) & 0xff] ^ (fcs >> 8)`), complement (`+0xed0c`), low byte first, residue `0xf0b8` (`+0xe7e4`); the 256-entry table at payload `+0x153558` equals the generated reflected `0x8408` table | table equal; residue constant present; complement and byte order not read |
| Address and control prefix (`ff 03`) | none: the flag is followed directly by the header byte | none | none | none |
| Maximum payload | a reply stops growing when its length (class byte included) plus the next element would reach 128 bytes (`0xffe35d52`, `0xffe35dbe`): at most 127 bytes with the class byte | "the whole payload at most 127 bytes", overflow flag; whether the class byte counts was not re-read | the writer copies at most `0x7f` payload bytes after the separate header byte (`+0xec58` to `+0xec70`); the receiver's ring buffer is 128 bytes (index mask `0x7f`) | clamp at 127 (`l1sc_fill_sendbuf`) |
| Receive buffer | 256 bytes (node link) | `0x100` | 128-byte ring per channel | not read |

Verdicts. **Identical** (established): flag, escape and xor, FCS including residue, no `ff 03` prefix, and the escaped set on the node link. The writers *can* differ per context in both L1 programs (the flow-control bytes on the peer links of the ColdFire L1), so the two L1 libraries are the same design with the same options; but nothing on a node link turns an option on. **Different** (established): the ColdFire L1 counts the class byte inside its 127 bytes, the SN1 PROM does not count its header byte inside the 127 it copies. The consequence is at most one byte of payload and only matters for a message near the limit. For the ARM L1 and the SN2 SAL the class byte's accounting is **unknown**.

Chunking: both L1s reserve request codes `0x3001` to `0x3003` for multi-frame messages ([chunked-request-frames.md](chunked-request-frames.md): segment count in byte 7, data length in bytes 8 and 9, errors `-1623` to `-1625`). The ARM L1's layout and error codes were not read (unknown). The same unresolved puzzle exists on both: the parser sends `0x3001` to `0x3003` to the assembler before the request chain, yet the ColdFire packages 3 and 5 and the ARM L1 each have chain handlers for `0x3001` (and, for package 5 and ARM, `0x3002` to `0x3004`); how a request reaches those handlers is not established on either (unknown).

## 3. Header

**Identical, established.** A request frame is: byte 0 class in bits 7:5 and index in bits 4:0; bytes 1 to 4 a big-endian address word; bytes 5 and 6 the big-endian request code; byte 7 the argument count; arguments from byte 8. ColdFire `request_frame_parse` (`0xffe35634`) reads exactly these fields (address word `0xffe3567a` to `0xffe356a0`, code `0xffe356a4` to `0xffe356ac`, count `0xffe3574e` when the frame is at least 8 bytes, arguments from the next byte); the ARM page records the same offsets from `syscom_process_inbound_request`. The class test is the same: class 0 request, class 2 event or console data, anything else not processed (`frame_classify`, `0xffe35864`; ARM `syscom_process_inbound_packet`). A response is class 1: byte 0 is `0x20` plus the subchannel index, bytes 1 to 4 the status, no address word, no echoed code (ColdFire `response_frame_serialize`, `0xffe35c08`, `0xffe35c76`; ARM `syscom_process_outbound_response`, `0xc3f64`).

**Tags and subchannels, identical (established for both writers).** The reply's subchannel index is recovered from a 7-bit tag in bits 5 to 11 of the request's source word; the ColdFire dispatcher builds the tag from the arrival link and the request's index (`0xffe35708`, tag at `0xffe3571c`), the ARM L1 derives it with `SysComSetTasknum`. The ARM page infers that the index returns unchanged; the ColdFire reply uses `tag_subchannel` (`0xffe3498c`) the same way.

**Task numbers, identical (established).** Destination task is the low five bits of the address word when bit 31 is clear: general requests are task 6, text commands task 3, loopback 18, node link 5 ([sn2-prom-link-1.62.1.md](sn2-prom-link-1.62.1.md)). The destination base is the same on both nodes: the SN1 PROM opens its destination with the word `0x0ffff000` (at every site read: `+0xee28`, `+0x24cfc`, `+0x4e5c8`, `+0x535b0`, `+0x65fc4`; the two `0x000b` sites differ, below) and ORs in task 6 (every builder call has `t0 = 6`); the SN2 SAL opens `0x0ffff000` too ([sn2-prom-link-1.62.1.md](sn2-prom-link-1.62.1.md)).

**Different, established for the SN1 PROM only.** What the node accepts coming back. Its receive state machine (`+0xe1a4` to `+0xe1e0`) takes a frame header as valid only if (class 0 and index below 5) or (class 1 or 2 and index 5 to `0x17`); index `0x18` and up is rejected whatever the class. The SN1 PROM sends requests with class 0 and the index of its destination handle (`+0x534f8`), and console output as class 2: the header byte is `0x40` ORed with the low byte of a read of the doubleword at `0x9200000001000020` (`+0xd2ec`, `+0xd304`; what that register is, is unknown). For the SN2 node, `l1sc_receive` compares byte 0 against `0x44` (class 2, index 4: inferred from two immediates, not read in full); index 4 with class 2 would fail the SN1 PROM's filter. So an L1 reply or console frame that satisfies one node may be dropped by the other; the safe range for responses is class 1, index 5 to `0x17`. Whether the SN2 side has any filter is unknown.

## 4. Arguments and status

**Identical, established.** Argument types: `0x00` then a 32-bit big-endian integer; `0x01` then a NUL-terminated string; `0x80 | n` then `n` raw bytes (up to 127). The ColdFire request parser (`0xffe3575a` to `0xffe3580c`) dispatches on type 0 (integer, five bytes), type 1 (string, length by `strlen` plus the terminator) and any other value as raw with the length masked by `0x7f`. A reply is: byte 0 `0x20 | subchannel`; bytes 1 to 4 the status; byte 5 the count of arguments after the status; arguments (`0xffe35c88` to `0xffe35cc8` for the status and count, `0xffe35d4e` and later for each argument, which writes `0x00` plus four bytes, `0x80 | n` plus bytes, or `0x01` plus a string). The error form is the same on both: status `0xffffffff` followed by the error code in four bytes, nine bytes in all, with no count (ColdFire `0xffe35cd0` to `0xffe35d12`; ARM page).

**Different in meaning, same in bytes: the node's argument count.** The SN1 PROM's request builder (`+0x63e78`, called from 27 sites) takes the argument list as `(type, value)` word pairs and counts words: an integer or string is 2, a raw block 3 (established from the builder loop at `+0x63f44` to `+0x640d4`). Its call sites therefore pass 8 for the four integers of `0x0006`, 2 for one string or one integer, 4 for two integers. The SN2 page's "Arguments" column (8, 2, 2, 12, 4) uses the same numbers, and they equal the ARM handlers' needs (four integers for `0x0006`, one string for `0x1005`, six integers for `0x200d`), so that column counts words, not arguments (inferred; the SN2 builder was not re-read). The byte 7 count on the wire counts arguments.

**Status codes, identical (established).** The ColdFire and the ARM request routers return the same set: `-100` (too few arguments), `-101` (no handler), `-102` (not text), `-104` (absent or empty), `-105` (out of range), `-106` (hook or lifecycle failure), `-107` (record parse failure), `-108` (not available on this brick type). Literal counts in the ARM `L1ReqIRouter` (`0xc8388` to `0xcf268`): `-100` 22 times, `-101` 2, `-102` 1 (the display request), `-104` 10, `-105` 7, `-106` 17, `-107` 5, `-108` 1 (`0xca62c`, the processor-mask request, when no processor exists: the same condition as the ColdFire handler). `-103` is not returned by either; it appears only in the SN1 PROM's own display routine for text over 12 characters, which never sends. The ColdFire L1's chunk errors `-1623` to `-1626` have no counterpart read in the ARM L1.

## 5. Request codes

### Which codes each L1 accepts

Counted by running the compare chains, as in [sn2-prom-link-1.62.1.md](sn2-prom-link-1.62.1.md) and [personalities.md](personalities.md) (established):

- **Origin 300 package (package 1), 39 codes:** `0x0001` to `0x0011`, `0x1001` to `0x100c`, `0x2001` to `0x2008`, `0x4000`, `0x4002`. Packages 0 and 2 have the same stream.
- **ARM L1 1.62.1, 55 codes.**
- **In the ARM L1 and not in package 1 (21):** `0x0012`, `0x0013`, `0x0015`, `0x0016`, `0x0017`, `0x2009` to `0x200f`, `0x3001` to `0x3004`, `0x4001`, `0x4003`, `0x4004`, `0x4005`, `0x5000`.
- **In package 1 and not in the ARM L1 (5):** `0x000f`, `0x1002`, `0x1003`, `0x2007`, `0x2008`.
- **SHUB-aware ColdFire packages.** Package 5 (44 codes) adds to package 1 `0x0012`, `0x0013`, `0x200d`, `0x200e`, `0x3001` to `0x3004`, `0x4001`, `0x4004` and drops `0x000f`, `0x1002`, `0x1003`, `0x2007`, `0x2008`: the ARM L1 accepts every package 5 code. Left in the ARM L1 and not in package 5: `0x0015`, `0x0016`, `0x0017`, `0x2009` to `0x200c`, `0x200f`, `0x4003`, `0x4005`, `0x5000`. Package 3 (42 codes) is the same except that it keeps `0x000f` and `0x1003` and lacks `0x3002` to `0x3004`, `0x4004`.
- So the ARM L1 is a superset of the SHUB-aware ColdFire package 5; `0x0015` (sharing mode) is the only code the node sends at boot that no ColdFire package handles.

### What the node sends

SN1, from the IP35 PROM (every call of the request builder `+0x63e78`, decoded for task, code and argument words; all with task 6). Established.

| Code | Words | Sites (payload offsets) | ColdFire request |
|---|---|---|---|
| `0x0001` | 0 | `+0x28d5c` | debug switches |
| `0x0003` | 0 | `+0x24d2c`, `+0x44c00`, `+0x4e600`, `+0x4e734` | rack, bay, type |
| `0x0004` | 0 | `+0x24f54`, `+0x5776c`, `+0x65ff0` | brick serial |
| `0x0005` | 0 | `+0x63064` | firmware revision |
| `0x0006` | 8 | `+0x1cef8`, `+0x1d58c` | board EEPROM read |
| `0x0008` | 0 | `+0x49d98` | system serial |
| `0x0009` | 0 | `+0x572f4`, `+0x5e7d4` | partition |
| `0x000a` | 0 | `+0x4e0fc`, `+0x4e210` | port speeds |
| `0x000b` | 0 | `+0x573f8`, `+0x5e8c4` | partition-database flag; sent to a destination built from a node value shifted left 18 bits, not `0x0ffff000` (not read further) |
| `0x000c` | 0 | `+0x4d3a0` | attached ports 1 to 4 |
| `0x000e` | 0 | `+0xee58` | the arrival link |
| `0x1004 + n` | 2 | `+0x44aa8` | display line, one string |
| `0x1006` | 2 | `+0x535ec`, `+0x536fc` | partition set, one integer |
| `0x1009` | 4 | `+0x68d94` | panel control, two integers |
| `0x100a` | 0 | `+0x68a3c` | event subchannel clear |
| `0x100c` | 4 | `+0x68eb0` | per-processor setting, two integers |
| `0x2006` | 0 | `+0x62cf4` | lifecycle action |

The IP35 PROM never builds `0x0002`, `0x0007`, `0x0010`, `0x0011`, `0x0012`, `0x0013`, `0x0015`, `0x1001`, `0x1007`, `0x100b`, `0x2001` to `0x2005`, `0x200d`, `0x200e` or any `0x3xxx` and `0x4xxx` code through this builder (established for the builder; another builder of the same PROM, at `+0x53ae0`, sends the L2 partition fetch to task 10 and is not part of this table). Who sends `0x100b` to a ColdFire L1 is not found in this PROM (unknown; the open question below).

SN2, from the SAL runtime ([sn2-prom-link-1.62.1.md](sn2-prom-link-1.62.1.md), established): `0x0001`, `0x0003`, `0x0006`, `0x0008`, `0x0009`, `0x000a`, `0x000e`, `0x0010`, `0x0012`, `0x0013`, `0x0015`, `0x1005`, `0x1007`, `0x100a`, `0x200d`, `0x200e`, `0x200f`, `0x2004`, the text command to task 3, and (SAL_B) `0x100b`.

Common to both nodes: `0x0001`, `0x0003`, `0x0006`, `0x0008`, `0x0009`, `0x000a`, `0x000e`, `0x100a`, and the display request (the SN1 PROM adds a line index to `0x1004`, `+0x44a90`; the earlier reading in [brick-request-handlers.md](brick-request-handlers.md) found it sends only `0x1005`; the SN2 SAL sends `0x1005`). SN1 only: `0x0004`, `0x0005`, `0x000b`, `0x000c`, `0x1006`, `0x1009`, `0x100c`, `0x2006`. SN2 only: `0x0010`, `0x0012`, `0x0013`, `0x0015`, `0x1007`, `0x100b`, `0x200d`, `0x200e`, `0x200f`, `0x2004`.

### The boot-time codes on the L1 side

"CF" is the Origin 300 package (package 1) unless another package is named.

| Code | ColdFire reply | ARM reply | Verdict |
|---|---|---|---|
| `0x0001` | status 0; one integer, setting 3 (`0xffe11fe8`) | status 0; one integer, setting 3 (`0xc8b74`) | identical shape and setting index; established |
| `0x0003` | status 0; `rack << 6 | bay` from settings 1 and 2 (`0xffe11ffc`); then the type character. In package 1 type `8` reports `7` and `=` reports `#` when the low byte of the slot value is zero; packages 3 and 5 keep only the `=` rule | status 0; the same integer from settings 1 and 2; then the type character; `=` reports `#` unless a per-slot flag is set (`0xc8bb8`) | identical shape and settings; the `=` rule is the same in the SHUB-aware packages; package 1 adds the `8` rule; established |
| `0x0006` | needs five words including the code; status 0, bytes left, raw bytes (below) | same (below) | identical shape; different id tables; established |
| `0x0009` | status 0; one integer, setting 14 (`0xffe128ca`) | status 0; setting 14 | identical; established |
| `0x000e` | status 0; one integer, the arrival link index computed by the dispatcher (`0xffe12a4c`) | status 0; one integer, the requesting slab (`SysComGetSlab` of the request tag; `0xca214`) | same shape, differently defined value; whether they agree for a node on its own channel is unknown (inferred 0 for the ARM single-node case) |
| `0x0010` | status 0; word count; processor count; one pair of masks per word; `-108` when no processor (`0xffe12acc`). Package 5 returns a single pair chosen by the port tag for types `,` `}` `.` `$` | the same two shapes, selected by type `,` `}` `.` `$` or more than one slot: single pair is status 0, 1, a count, presence mask, enable mask; otherwise status 0, node count, processor count, then per node presence and enable (`0xca5f4`, `0xcaa14` to `0xcab54`); `-108` at `0xca62c` | identical shapes; same order of the two counts (the ARM log text "%d CPUs, %d nodes" fixes which is which); that the first count is the same quantity as the ColdFire word count was not compared; shapes established |
| `0x0013` | absent from packages 0, 1 and 2; packages 3 and 5: status 0, integer 0, the type character | for types other than `,` `}` `.` `$` with one slot: status 0, integer 0, the type character; other types return the slot and state, `-106` on a failed state check | identical for the single-slot case; the multi-slot form has no ColdFire counterpart |
| `0x0015` | no handler in any ColdFire package | status 0; setting `0xcc` masked to 7 bits | ARM only |
| `0x1005` | one string; `-100` below two words, `-102` for a non-string | one string; `-100` if the count is 1 or less; `-102` | identical (the `-102` literal at `0xcaf04` is in the display handler) |
| `0x100b` | exactly three integers, no reply on success (`0xffe131bc`); wrong count `-100` | exactly three integers, no reply on success; wrong count `-100` | identical; established |

### `0x0006` in detail

Argument layout, **identical (established):** the request carries four integers after the code: device id, area selector, byte offset, length. The ColdFire handler (`0xffe12526`) needs the count byte at frame `+0x20` to be at least 5 (`-100` otherwise, `0xffe12528` to `0xffe12532`), maps the device id through a hook with the sender tag, reads selector, offset and length from words 2, 3 and 4. The SN1 PROM passes (device id, selector, running offset, length of at most 64) from a stack frame (`+0x1ce6c` to `+0x1cef8`: the length starts at `0x40`); the SN2 node asks for at most 64 bytes per request too.

Checks and reply, **identical in order and result (established for ColdFire, from the ARM page for ARM):** record not present `-104`; record index at or above the count `-105`; selector above 4 `-105` (the ColdFire default arm loads `0x97` at `0xffe127c4`; the earlier note "otherwise the chain ends in `-104`" in [brick-request-handlers.md](brick-request-handlers.md) is about the present test, not the selector); offset at or above the area size `-105` (`0xffe127e4`); then status 0, the number of bytes of the area left after this window (the area size less offset less length, floored at 0), then the raw bytes, omitted when the length is zero (`0xffe127fc` to `0xffe12844`). The selectors are 0 (8-byte common header), 1, 2 and 3 (IPMI areas) and 4 (memory module serial-presence bytes) on both: the ColdFire dispatcher has the same five arms (`0xffe12620` to `0xffe12642`, header size 8 at `0xffe12674`); that the area-to-selector assignment matches the ARM one is inferred, not compared arm by arm.

Device id to record, **different (established for both):**

- ColdFire: the hook at `0x30000990` (package 1; per-package address) maps the id by brick type. For types `0`, `7`, `8`, `9`, `E`, `Q`, `V` it returns the id unchanged except that 0 and 4 swap; every other type except `5`, `6` and `O` returns it unchanged. For the 2U and workstation profiles that makes id 0 the node board and id 4 the interface board ([ip53-eeprom-record-tables-1.44.0.md](ip53-eeprom-record-tables-1.44.0.md)).
- ARM, brick type `F`: id 0 is INTERFACE, id 1 NODE, ids 4 to 7 the four CPU entries, ids 8 to 13 and 16 to 21 the twelve DIMMs, id 98 the riser, id 99 IO10; every other id is "not present" (`0xc7664`, [sn2-prom-link-1.62.1.md](sn2-prom-link-1.62.1.md)).

So an id means the node board on one and the interface board on the other (id 0), and the DIMM ids are not the same. Which ids the SN1 PROM sends is **unknown**: its callers reach the shared routine through two further levels (`+0x50244`, `+0x502ec`, with selectors 2 and 1) whose id arguments were not traced. The ColdFire hook for types `5`, `6`, `O` and `C` (the IP35 compute brick) was not read for `C` beyond "unchanged".

## 6. Identity

- **Type character on the wire.** One byte, upper case, the stored value of `brickGetType` on the ARM L1 (`0x327c0`, no case change; `0xc83b4` to `0xc83c0` copy it into the request context) and the stored type character on the ColdFire L1. Established. The Linux kernel's `l1.h` uses lower case for the same letters ([sn2-brick-packages.md](sn2-brick-packages.md)); neither L1's handler folds, so any fold is on the node side (unknown).
- **Letters differ by family (established).** ColdFire package 1: `4` (Origin 300, IP45), `%4A?` string. Package 2 server profile `7`, workstation `9`, compute brick `C` and the rest of [personalities.md](personalities.md). The ARM L1 reports `F` for the Opus TIO brick and the families named in [sn2-brick-packages.md](sn2-brick-packages.md). `=` (IX) reports `#` (PX) in both families under the conditions in the table above.
- **Module id.** `rack << 6 | bay` from non-volatile settings 1 and 2 on both (`0xffe11ffc` and `0xc8bb8`; setting numbers identical). The serial request `0x0008` returns setting 13 (`0xd`) or a per-brick hook on both ([serial-number-requests.md](serial-number-requests.md): `0xffe12848`; ARM `0xc9ddc`); `-104` when empty.
- **Settings numbering is shared (established):** setting 1 rack, 2 bay, 3 debug switches, `0xd` system serial, `0xe` partition, `0x12` partition-database flag, `0x13` event subchannel; the ColdFire handlers `0xffe128ca`, `0xffe128de` and the ARM handlers read the same indexes. Whether the stored widths and layouts are the same is unknown.

## 7. The `0x0001` conflict

The SN2 pages say the SAL 5.04 single-node skip needs the debug switches to carry `0x800` (bit 11) while the ARM L1 1.62.1 answers 3. The binaries show:

- The ARM handler (`0xc8b74`) is: `nvramGet(3)`, reply status 0 and that value as one integer. It contains no constant. Setting 3 is the 16-bit field made from bits 8 to 23 of the word at offset `0x1c` of the NVRAM image (`0x303b4` to `0x303cc`, established, read in full). So the reply is whatever is stored: 3 is a stored value, not the handler's answer, and bit 11 is present only if the stored word has it.
- The ColdFire handler (`0xffe11fe8`, packages 0, 1 and 2 byte-identical in this handler) is the same: setting 3 through the settings reader at `0xffe59590`, status 0, one integer. Established. Packages 3 and 5 run the same instruction stream ([brick-request-handlers.md](brick-request-handlers.md)). The setter `0x1001` writes the same setting on both.
- What the stored value is on a fresh or factory-reset controller was not found. The ARM `nvramInit` (`0x2c5ac`) writes defaults for settings `0x11` and `0x3d` only in the part read; whether a blank image holds zero for setting 3 is **unknown** (route: the blank-image branch of `nvramInit` and the factory image the flash carries).
- That the SAL tests bit 11 is as recorded in the SN2 pages (not re-read). Nothing in the L1 handlers decides the conflict: the two statements hold at once if the unit's stored debug switches differ from what the SAL wants, and the firmware cannot say which unit was observed. The ARM console's command string `debug switches set to 0x%04x` writes the same setting.

## Open items

Listed in [../../../questions/README.md](../../../questions/README.md) under "SN1 and SN2 node link".

- The SN1 node end: Bedrock UART registers and wire, from the port record's function pointers in the PROM.
- The ARM L1's line rate and handshake for a wire channel (`uartSetConfig`), and the mode of each Opus TIO family channel.
- The device ids the SN1 PROM sends in `0x0006`.
- Who sends `0x100b` to a ColdFire L1; the destination form of the PROM's `0x000b`.
- The SN2 node's receive filter against the SN1 PROM's.
- Whether the class byte counts toward 127 on the ARM L1 and the SN2 SAL.
- The ARM L1's chunk frame layout and errors, and the stored default of setting 3.
