# Router role: persistent setting `0x3c` (L1 1.44.0, packages 0, 1, 2, 3 and 5)

Which values the L1 accepts for the router role, where each is legal, what the console command is, what is stored, who uses the stored value, and what happens to an illegal one. Every package was read individually: the earlier notes carried package 1 to the others by analogy. This note supersedes the router-role paragraph of [l1-board-contract.md](../../devices/l1_board/README.md) section 9 where they differ (section 7 lists the differences).

Grades follow [../../AGENTS.md](../../AGENTS.md): **E** established, **I** inferred (reasoning stated), **U** unknown (static route in section 8).

## 0. Inputs and method

- Image: IRIX 6.5.30 `l1.bin`, L1 1.44.0, container sha256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`. Packages (body sha256): 0 `Base 1MB` `3c8a7aa93eb38502289cf0c2921ea91e6800228005bd8abd67425ce615f7b364`; 1 `Fuel/PE/O300 1MB` `9db44f3bb06ec90475035781766e1ca7e924f15a0ed215cf855c1f491275fdd1`; 2 `MIPS 2MB` `ac81f501f01c6789e756ab1f5ec69fadb1e4c04225511d541340831686e04e70`; 3 `Legacy 2MB` `d13e4959222bad3308e71ba9c854efae01770afb942a9e562212eaeb19fff6fa`; 5 `2MB` `5d4521de0baec59a9592cee0aa871cc67cd309c85f4dd303a290e937f49dbf8c`. Bodies load at `0xffe00000`; addresses are first-bank.
- Setting accessors per package (getter, setter): package 0 `0xffe59fbc`, `0xffe59184`; package 1 `0xffe59590`, `0xffe58758`; package 2 `0xffe7a420`, `0xffe795e8`; package 3 `0xffe6f994`, `0xffe6eb98`; package 5 `0xffe73be4`, `0xffe72de8`. Found by their shape (bound compare against the highest setting number, then a word jump table); each setter's call is `(key, value)` and each getter's `(key)`.
- **How the readers and writers were found.** (a) Every call to the getter or setter in the first bank was located twice, once in a linear 68040-mode listing and once by scanning the raw bytes for `jsr`, `bsr.w` at any alignment; the counts agree in every package (393, 408, 474, 385, 390 sites for packages 0, 1, 2, 3, 5). (b) For each site the key was resolved from the bytes immediately before the call (`moveq` or `move.l` immediate pushed, `pea.w`). Sites with an immediate key `0x3c` are the ones in section 3. (c) Sites whose key is computed were classified: a key read from a console-table word (no numeric table word anywhere in [command-table.tsv](../../software/firmware/l1/command-table.tsv) is 60), base plus a small index (`0x14 + n`, `0x58 + n`, `0x6c + n`, `0x71 + n`, `0x82 + n`, `0xa9 + n`, `0xaa + n`, loop counters from `0x80`), or a value taken from a received message (section 3, item 5). Only the base `0x14` family could reach `0x3c`, at index `0x28`; the type `C` port table of package 0 has indexes 0 to 8 (checked), other tables were not enumerated (I). (d) The settings-record pointer variable (`0x30000384`, `0x300003a8`, `0x30000984`, `0x30000b04`, `0x30000ac4` in packages 0, 1, 2, 3, 5) is referenced only from inside the getter and the setter, so nothing reads or writes the record by another path (E). (e) A scan for the getter or setter address stored as data finds none: every use is a direct call (E).
- Packages were compared by byte skeleton (address operands masked) where the routine is the same: the `router` command handler of packages 0, 2 and 3 is identical to package 1's, and the record-initialiser arm of packages 0, 1, 2, 3 and the R-brick release of packages 0 and 2 are identical.

## 1. Answer

| Question | Answer | Grade |
|---|---|---|
| What the setting is | Bits 7:4 of byte `+0x17` of the first settings record, so a 4-bit value. Same in all five packages (section 2). | E |
| Values legal at the console, packages 0 to 3 | `0xf` (`ordinary`, `ord`), `0xe` (`meta`), `0xc` (`repeater`, `rep`) | E |
| Values legal at the console, package 5 | the same three plus `0xa`, `0x6`, `0x2` for `meta1`, `meta2`, `meta3` | E |
| Console command | `router` (alias `rtr`): bare form prints the type; `router meta`, `router repeater`, `router ordinary` set it; package 5 adds `router meta1`, `meta2`, `meta3`. Offered only for types `1 2 6 R @ D` (package 5 also `.` and `-`). | E |
| What is stored | Exactly the table value (14, 12, 15, or in package 5 also 10, 6, 2) in the nibble; no other bits are changed. The command prints "router type changed, effective on power up or reset" and drives nothing. | E |
| What a start-up check accepts | Packages 0 to 3: `0xc`, `0xe`, `0xf`. Package 5: `0x2`, `0x6`, `0xa`, `0xc`, `0xe`, `0xf`. Anything else is replaced by `0xf` and the replacement is written back to the setting. | E |
| Who uses the value | (1) The R-brick release hook in packages 0 and 2 writes it to `ROUTER CONTROL` bits 7:4. (2) The record initialiser copies it into word `+0x2c` of the L1's own table entry, which is sent whole in the configuration reply. (3) The `router` status display. Nothing else reads it. | E for the sites, I for "nothing else" (section 4) |
| Illegal value | Never produced by the console. A value written some other way is repaired to `0xf` at the next start-up (types `6 @ D R`, `-`, `.`, `}` where applicable) or at the next R-brick release; the status display prints the raw value as "unknown (0x..)" until then. | E |
| Does a metarouter level reach any pin? | No image that can drive the pins accepts it. Package 5 stores and prints levels but has no release hook that writes `ROUTER CONTROL`; packages 0 and 2 have the hook but replace `0xa`, `0x6`, `0x2` by `0xf`. | E for both halves, I for "no other consumer" |

## 2. The storage (E, each package read on its own)

For key `0x3c` the getter's word table (index = key, scale 2) jumps to a stub that loads `0x14` into a register and joins a shared tail; the tail loads the record base pointer, reads the byte at `base + 0x14 + 3`, shifts right by 4 and masks `0xf`. The setter's table does the same for the write: the value is shifted left by 4, masked `0xf0`, merged into the byte under mask `0x0f`, the byte stored, then the checksum and mirror update runs as described in [settings-record.md](../../software/firmware/l1/settings-record.md). The setter performs no range check: any value stores its low four bits.

| Package | Getter stub for key `0x3c` | Setter stub | Getter tail | Setter tail | Record pointer |
|---|---|---|---|---|---|
| 0 | `0xffe5a406` | `0xffe596d0` | `0xffe5a848` | `0xffe59e58` | `0x30000384` |
| 1 | `0xffe599da` | `0xffe58ca4` | `0xffe59e1c` | `0xffe5942c` | `0x300003a8` |
| 2 | `0xffe7a86a` | `0xffe79b34` | `0xffe7acac` | `0xffe7a2bc` | `0x30000984` |
| 3 | `0xffe6fdca` | `0xffe6f0d0` | `0xffe7021a` | `0xffe6f878` | `0x30000b04` |
| 5 | `0xffe7401a` | `0xffe73320` | `0xffe7446a` | `0xffe73ac8` | `0x30000ac4` |

Keys `0x3d` and `0x3e` go to separate stubs reading byte `+0x17` bits 2:1 and bit 0 (the router port and service state); they do not touch the nibble (E, packages 0 to 5 by table).

## 3. Every site that names key `0x3c`

Each row below is a call with an immediate key `0x3c`. G is a getter call, S a setter call.

| Package | `router` command handler | Command sites | Start-up check (record initialiser) | R-brick release |
|---|---|---|---|---|
| 0 | `0xffe19f08` | G `0xffe1a026`, S `0xffe1a09e` | G `0xffe31594`, S `0xffe315ca` | routine `0xffe4a060`: G `0xffe4a078`, S `0xffe4a0a2` |
| 1 | `0xffe19f08` | G `0xffe1a026`, S `0xffe1a09e` | G `0xffe31594`, S `0xffe315ca` | none |
| 2 | `0xffe1f6e0` | G `0xffe1f7fe`, S `0xffe1f876` | G `0xffe36d6c`, S `0xffe36da2` | routine `0xffe69100`: G `0xffe69118`, S `0xffe69142` |
| 3 | `0xffe19458` | G `0xffe19576`, S `0xffe195ee` | G `0xffe313b2`, S `0xffe313e4` | none |
| 5 | `0xffe16664` | G `0xffe1678a`, S `0xffe1683e` and `0xffe1686e` | G `0xffe43ecc`, S `0xffe43f2c` | none |

There are no other immediate-key sites in any package (E). The five sites of package 5 differ from the earlier note's three; the two extra are the second setter in the command (levels) and the setter of the start-up check.

**1. The command (E).** The handler reads record word 12 of the command entry as the sub-command and word 13 as the value (word 11 is the type set; [command-table.tsv](../../software/firmware/l1/command-table.tsv) columns `word11` to `word13`). Packages 0 to 3: sub-command 0 prints; 1, 2, 3 store the value word, which the table fixes as 14 (`meta`), 12 (`repeater`), 15 (`ordinary`). Package 5: sub-command 0 prints; 1, 2 and 3 store the value word; sub-command 11 (word 12 of the `meta1|meta2|meta3` entry) takes the character at offset 4 of the typed word, subtracts `'0'`, and stores `((digit & 3) ^ 3) * 4 + 2`: `meta1` 10 (`0xa`), `meta2` 6 (`0x6`), `meta3` 2 (`0x2`); other sub-commands give the "unknown router type" error. The type check `set:126R@D` (package 5 `126R@D.-`) is the first thing the handler does. The status form reads the key, and in packages 0 to 3 compares it with 12, 14, 15 (names `repeater`, `meta`, `ordinary` at `0xffe26510`, `0xffe2650b`, `0xffe26502` in package 0), else prints `router type is unknown (0x%02x)`; package 5 uses a 16-entry jump table giving `ordinary` (15), `meta (no metarouter level set)` (14), `meta (level 1)` (10), `meta (level 2)` (6), `meta (level 3)` (2), `repeater` (12) and unknown for the other ten values (table `0xffe167a8`, strings from `0xffe223e4`). Before the type the same handler prints the service-mode banner (keys `0x3e` and `0x3f`) and a series or state word from key `0x3d`. The set path does not write any expander (no expander writer is reachable from the handler within eight calls of direct calls, package 0).

**2. The start-up check (E).** The record initialiser fills the L1's own table entry (table pointer `0x30000a78` in package 0; entry stride `0x70`). For the types listed below, the matching arm reads key `0x3c`, stores the raw value in entry word `+0x2c`, and, if the value is not in the accepted set, replaces entry word `+0x2c` by `0xf` and calls the setter with `(0x3c, 0xf)`. Accepted sets: `{0xc, 0xe, 0xf}` in packages 0 to 3 (compare ladder `moveq #-12` then three tests); `{0x2, 0x6, 0xa, 0xc, 0xe, 0xf}` in package 5 (a 16-entry jump table; package 5 writes the setting back only if the final value differs from the entry's). Which brick types use this arm, from the type jump table (index = type minus `0x23`, tables compared against the arm address): packages 0, 1, 2, 3: `6`, `@`, `D`, `R`; package 5: `-`, `.`, `6`, `@`, `D`, `R`, `}`. The same arm also clears the partition word (entry `+0x3c`) and writes setting `0x0e` := 0 for `R` and `D` (unconditionally in packages 0 to 3, only when the word is nonzero in package 5, which also does it for `-`).

**3. The R-brick release (E, packages 0 and 2, same instructions).** Routine `0xffe4a060` (package 2 `0xffe69100`) runs once per row of the brick's reset table (`R` has one row, bytes `00 02 00 08` at `0xffe4a5d0`, package 2 `0xffe69670`). Byte 0 of the row is the role if nonzero (it is 0 in the `R` row), else key `0x3c` is read. The low byte is compared with `0xc`, `0xe`, `0xf`; anything else becomes `0xf` and is stored back with the setter. Then the nibble is written to the expander named by row byte 1 (index 2, `ROUTER CONTROL`) under mask `0xf0`: `0xffe65e84(2, 0xf0, nibble << 4)`. After that the routine runs the supply-state routine, and only when its result is nonzero it waits 10 ticks, sets the row's `POWER CONTROL` bit (index 0, mask `0x08`), waits 10 ticks and steps `ROUTER CONTROL` bits `0x0d` through `0x09`, `0x0d`, `0x0c` with 10-tick waits. The expander: index 2 of the R-brick table (package 0 `0xffe4a3bc`, package 2 `0xffe6945c`), 16-byte record with wire address `0x46` (7-bit `0x23`), applied value `0xf1`, name `ROUTER CONTROL`. The hold hook of the R-brick (`0xffe49fe0`) writes only `ROUTER CONTROL` bits `0x0d` := `0x01` and clears the `POWER CONTROL` bit; bits 7:4 are left alone. The descriptor has no soft-reset or NMI hook. So bits 7:4 change at release only (E for the L1's own paths; the router-port console family reaches the port-down routine, which writes whichever expander the port record names, and its records were not decoded, U3).

**4. Table word `+0x2c` (E for the store and the copy, I for the reader).** The configuration reply builder (package 0 `0xffe325ec`, data append at `0xffe328ce`) appends `N * 0x70` bytes of the table verbatim, so the validated nibble leaves the L1 as the word at record offset `0x2c`. The L2 side is described in [l2/config-request-reply.md](../../software/firmware/l2/config-request-reply.md): word `+0x2c` becomes brick byte `+0x0f`, "not used by the route search". No code in any ColdFire package reads word `+0x2c` of the table entry: every displacement-`0x2c` load in the five first banks was classified; the loads that remain are the second command argument (`$2c` of the command context), flags of received messages, or fields of other structures (I; a search by displacement cannot exclude an indirect copy).

**5. A writer that bypasses the checks (E for the code, U for who uses it).** The task-14 event handler has a code `0x66` arm (package 0 `0xffe33c50`, called at `0xffe34012`, body read in full in package 0 only). A setter call of the same shape (key and value from the message) is present in package 1 (`0xffe33cd0`), package 2 (`0xffe394a8`), package 3 (`0xffe340b8`) and package 5 (`0xffe46940`); those bodies were not read (I that they match). If an integer argument equals 1 it takes the next two integers as key and value and calls the setter with them: no key filter, no value filter. Inside the L1 the senders of code `0x66` (builder `0xffe33060`, one call site `0xffe33494`) are three sites (package 0 `0xffe109ba`, `0xffe1a7d8`, `0xffe1ad3a`) that pass keys `0x80` and `0x7f`, never `0x3c`. So whether anything can send `(0x3c, value)` to an L1 (a peer L1, the L2, a host tool) is not known (U2); if one does, the nibble stores unvalidated and the next start-up or R-brick release repairs it.

## 4. Illegal values and cross-package behaviour (E, from sections 2 and 3)

- A value `0xa`, `0x6` or `0x2` stored by a package 5 L1 is treated as illegal by packages 0 to 3: the start-up check (types `6 @ D R`) and the R-brick release replace it by `0xf` and write that back. A brick running package 0 or 2 therefore never drives a metarouter level to the pins. I for "the settings window survives an image change": the window format and offsets are the same, but replacing one image by another was not traced.
- A value other than the accepted set read by the status display is printed with `%02x` as `unknown (0x..)`.
- Value `0xd` (and the other illegal nibbles) cannot come from the console in any package.

## 5. Pins (E for the write, I for meaning)

Only the R-brick release drives pins from the role: `ROUTER CONTROL` bits 7:4 receive the accepted nibble in packages 0 and 2. Cold start applies `0xf1`, so a never-written router starts as ordinary. The nibble bits: `0xf` `1111` ordinary, `0xe` `1110` meta, `0xc` `1100` repeater; bits 7:6 are set in all three, bit 5 is clear only for repeater, bit 4 is clear for meta and repeater. Which chip input each bit reaches, and polarity, is not in the image (U1). Package 5 has no hook that writes the nibble anywhere, although it accepts six values (I: from the five call sites and the absence of an R-brick release in its table; its other router personalities, `-` and `.`, were not read for pin writes).

## 6. Ledger of earlier claims

| Claim (source) | Status |
|---|---|
| Setting `0x3c` is byte `+0x17` bits 7:4 (settings-record row 60; board contract section 9) | re-read: confirmed in packages 0, 1, 2, 3, 5, getter and setter |
| Accepted values `0xf`, `0xe`, `0xc`; anything else becomes `0xf` and is stored (board contract; L2 conversation finding 11) | re-read: confirmed for the start-up check in packages 0, 1, 2, 3 and for the R release in 0 and 2 individually; package 5 differs (six values) |
| Package 5 decodes `0xa`/`0x6`/`0x2` as levels 1/2/3 with `meta1`/`meta2`/`meta3` | re-read: confirmed (command, display and start-up check) |
| Packages 0, 2, 3 "decode like package 1 (inferred)" (L2 conversation finding 11) | re-read: upgraded to established for the command handler, the display and the start-up check (skeleton-identical) |
| The nibble reaches `ROUTER CONTROL` bits 7:4 only at reset release on the R-brick (finding 11; section 9) | re-read: confirmed for packages 0 and 2 by scan; package 5 has no writer |
| Package 5 call sites `0xffe16786`, `0xffe1683a`, `0xffe43ec8` (board contract) | re-read: those are the `moveq` addresses of three of five sites; the sites are `0xffe1678a`, `0xffe1683e`, `0xffe1686e`, `0xffe43ecc`, `0xffe43f2c` |
| "One more encoded `0x3c` call in each of packages 3 and 5 is a size argument to another routine" (board contract section 9) | not reproduced: no further call with key `0x3c` was found in either package; the extra site in package 5 is the second setter above, package 3 has none beyond the four |
| Partition clear for `R` and `D` only (board contract) | re-read: confirmed for packages 0 to 3; package 5 also clears for `-` and only when the word is nonzero |
| Other consumers (table word `+0x2c`, L2 brick byte `+0x0f`, display) unknown (U22 to U24) | partly settled: the table word travels in the configuration reply (E) and no L1 reader exists (I); the L2 reader was not re-read (not re-read, grade as the L2 note) |
| `router` command is offered for `1 2 6 R @ D` only | re-read: confirmed for packages 0 to 3; package 5 adds `.` and `-` |

## 7. Corrections to earlier notes

1. [l1-board-contract.md](../../devices/l1_board/README.md) section 9, router paragraph: the list of package 5 call sites is incomplete (two sites missing) and the statement about size arguments in packages 3 and 5 is not supported. Section 10 of that note is correct for package 0 but omits the code `0x66` writer.
2. [l2/l1-conversation.md](../../software/firmware/l2/l1-conversation.md) section 12 finding 11: packages 0, 2, 3 are established, not inferred, and package 5 has no pin writer for its levels.
3. The L1 board note's statement that the partition clear applies "for `R` and `D` only" holds for packages 0 to 3 only.

## 8. Open items and the static route for each

| # | Item | Why open | Route |
|---|---|---|---|
| U1 | Chip inputs behind `ROUTER CONTROL` bits 7:4, polarity, and what each role does | image labels the expander only | the router-board and routing-chip hardware descriptions; in the boot firmware image the read of the router-chip control register and the role names it prints |
| U2 | Whether any peer, L2 or host tool sends event code `0x66` with key `0x3c` | senders found inside the L1 pass other keys | search the L2 image and the host tools for a builder of task-14 events with code `0x66`; the three L1 senders' callers |
| U3 | Whether the `router 6 port`, `8 port`, `service` family writes `ROUTER CONTROL` bits 7:4 through the port-down routine | R-brick port records not decoded | decode the R-brick port table behind `0xffe4a460` and the expander index and mask of each record |
| U4 | Pin writers in package 5's own router personalities (`-`, `.`, `}`) | not read | list stores to expanders in the descriptor initialisers reached by the type table at `0xffe43dce`; their hold and release hooks |
| U5 | Who reads the role from L2 brick byte `+0x0f` (configuration display, boot firmware) | route search does not; display not read | the L2 `config` printer; the boot firmware's use of the configuration reply word at record offset `0x2c` |
| U6 | Whether the settings window survives an image replacement with its role nibble intact | updater not traced for the window | the host updater's flash map against the window address |
