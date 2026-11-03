# ARM packet, interrupt and socket helpers in 1.38.4

Read from `6.5.28/usr/cpu/firmware/sysco/l1.bin`, package `6.5.28:l1#4` (version 1.38.4),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x53cfa0`.
Carrier SHA-256: `e5b65e5f28d7fba6013a8618f55c34bed7f60f7c6f46d7e9c5b8b7b2eabbe7b3`.
Package SHA-256: `c6693d19c6eb8c3fcf00cbb84d6974d1b5b17b930fb5375c1df2df20894edec9`.
Inner executable SHA-256: `f652c04c77cd7c53ca7ae6d6d6f581f11200efa211ff089bfba0d8bd4f718eb8`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1254136 bytes.
Segment SHA-256: `a373bb774d119f083e52d394fbd2701ebfbf35210301a40dcdb4ebdc39bf6bdc`. Analysed routine spans use the exact
ELF function symbols and sizes. They can include trailing literal words after a return.

`sp` denotes the entry stack pointer. Register expressions such as `r0` denote entry
values, and brackets denote loaded values. Decimal numbers inside expressions retain the
exact tool notation. Call candidates use the assumed ARM AAPCS
register locations r0 through r3 with unknown arity. Stack stores appear in the access tables.
Unknown values and predicates include unsupported instruction effects and merged paths.
Numeric call targets have unknown callee names in the flat-image analysis. Device roles
and pointee meanings remain unknown unless the body establishes them. Tables include all
explicit memory operands on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Separate loads at the same printed address need not observe the same value.
SocketMakeInetConnection has an unresolved computed selector. Accesses and return values cover only known control-flow edges. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/packet-interrupt-socket-helpers-facts.txt) retain all tool sections.

## `syscom_process_inbound_packet`

- `syscom_process_inbound_packet`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x841cc to 0x842a8.
  Established. The first branch takes `[sp-32] == 0x0` and falls through on nonzero (`beq` at
  0x84204). On its taken path, a word store writes `[0x1496f4]+1` at `0x1496f4` (`str` at 0x84228).
  A subsequent word store writes `([0x149700] + [sp-28])` to `0x149700` (`str` at 0x84240). The two
  counters' units and packet classification remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x841dc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x841e0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x841e4 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x841e8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x841ec | load | 1 | `r1 +0x0` | `-` |
| `str` at 0x841f8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x841fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x84208 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x84218 | load | 4 | `abs 0x84298` | `-` |
| `ldr` at 0x8421c | load | 4 | `abs 0x84298` | `-` |
| `ldr` at 0x84220 | load | 4 | `abs 0x1496f4` | `-` |
| `str` at 0x84228 | store | 4 | `abs 0x1496f4` | `[0x1496f4]+1` |
| `ldr` at 0x8422c | load | 4 | `abs 0x8429c` | `-` |
| `ldr` at 0x84230 | load | 4 | `abs 0x8429c` | `-` |
| `ldr` at 0x84234 | load | 4 | `abs 0x149700` | `-` |
| `ldr` at 0x84238 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x84240 | store | 4 | `abs 0x149700` | `([0x149700] + [sp-28])` |
| `ldr` at 0x84244 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x84248 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8424c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x84258 | load | 4 | `abs 0x842a0` | `-` |
| `ldr` at 0x8425c | load | 4 | `abs 0x842a0` | `-` |
| `ldr` at 0x84260 | load | 4 | `abs 0x1496fc` | `-` |
| `str` at 0x84268 | store | 4 | `abs 0x1496fc` | `[0x1496fc]+1` |
| `ldr` at 0x8426c | load | 4 | `abs 0x842a4` | `-` |
| `ldr` at 0x84270 | load | 4 | `abs 0x842a4` | `-` |
| `ldr` at 0x84274 | load | 4 | `abs 0x149708` | `-` |
| `ldr` at 0x84278 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x84280 | store | 4 | `abs 0x149708` | `([0x149708] + [sp-28])` |
| `ldr` at 0x84284 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x84288 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8428c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x84294 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x84294 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x84294 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x84250 | `(unknown callee) 0x83ac4` | `[sp-20]` | `[sp-24]` | `[sp-28]` | `([0x149700] + [sp-28])` |
| `bl` at 0x84290 | `(unknown callee) 0x83ea4` | `[sp-20]` | `[sp-24]` | `[sp-28]` | `([0x149708] + [sp-28])` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x84204 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `beq` at 0x84210 | `[sp-32] == 0x2` | `[sp-32] != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x84294 | `r0` | `?` |

## `syscom_process_outbound_packet`

- `syscom_process_outbound_packet`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x857e4 to 0x85860.
  Established. A word save places `[r0+22]:1` at `sp -0x18` (`str` at 0x85800). The first comparison
  takes `[r0+22]:1 == 0x1` and falls through otherwise (`beq` at 0x8580c). One call targets
  `0x84518` with entry r0, r1, r2 and `[r0+22]:1` (`bl` at 0x85840). The transport selection and
  merged return are unknown (`ldmdb` at 0x8585c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x857f4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x857f8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x857fc | load | 1 | `r0 +0x16` | `-` |
| `str` at 0x85800 | store | 4 | `sp -0x18` | `[r0+22]:1` |
| `ldr` at 0x85804 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x85810 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8581c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8582c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8583c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x85848 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x85854 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x8585c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8585c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8585c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x85840 | `(unknown callee) 0x84518` | `r0` | `r1` | `r2` | `[r0+22]:1` |
| `bl` at 0x8584c | `(unknown callee) 0x84994` | `r0` | `r1` | `r2` | `[r0+22]:1` |
| `bl` at 0x85858 | `(unknown callee) 0x85074` | `r0` | `r1` | `r2` | `[r0+22]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x8580c | `[r0+22]:1 == 0x1` | `[r0+22]:1 != 0x1` |
| `bgt` at 0x85818 | `[r0+22]:1 > 0x1 (signed, 32-bit)` | `[r0+22]:1 <= 0x1 (signed, 32-bit)` |
| `beq` at 0x85824 | `[r0+22]:1 == 0x0` | `[r0+22]:1 != 0x0` |
| `beq` at 0x85834 | `[r0+22]:1 == 0x2` | `[r0+22]:1 != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8585c | `r0` | `?` |

## `extIntRegister`

- `extIntRegister`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc8698 to 0xc87f4.
  Established. Three byte stores save `(r1 & 0xff)`, `(r2 & 0xff)` and `(r3 & 0xff)` at `sp -0x15`,
  `sp -0x16` and `sp -0x17` (`strb` at 0xc86b4, `strb` at 0xc86bc, `strb` at 0xc86c4). The first
  branch takes `(r1 & 0xff) < [0x2ad194]:1 (unsigned, 32-bit)` and falls through at or above the
  bound (`blo` at 0xc86d8); its callback record geometry and hardware interrupt mapping remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc86a8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xc86b4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xc86bc | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0xc86c4 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldr` at 0xc86c8 | load | 4 | `abs 0xc87e0` | `-` |
| `ldrb` at 0xc86cc | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xc86d0 | load | 1 | `abs 0x2ad194` | `-` |
| `ldr` at 0xc86dc | load | 4 | `abs 0xc87e4` | `-` |
| `str` at 0xc86e0 | store | 4 | `sp -0x28` | `0x133184` |
| `ldrb` at 0xc86e4 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xc86e8 | store | 4 | `sp -0x24` | `(r1 & 0xff)` |
| `ldr` at 0xc86f0 | load | 4 | `abs 0xc87e8` | `-` |
| `ldr` at 0xc8700 | load | 4 | `abs 0xc87ec` | `-` |
| `ldrb` at 0xc8704 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc8720 | load | 4 | `abs 0x2ad190` | `-` |
| `str` at 0xc8728 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xc872c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc8730 | load | 1 | `[sp-32] +0x28` | `-` |
| `ldr` at 0xc873c | load | 4 | `abs 0xc87f0` | `-` |
| `str` at 0xc8740 | store | 4 | `sp -0x28` | `0x1331c8` |
| `ldr` at 0xc8744 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc8748 | load | 1 | `[sp-32] +0x28` | `-` |
| `str` at 0xc874c | store | 4 | `sp -0x24` | `[[sp-32]+40]:1` |
| `ldr` at 0xc8754 | load | 4 | `abs 0xc87e8` | `-` |
| `ldrb` at 0xc8764 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc8780 | load | 4 | `abs 0xc87ec` | `-` |
| `ldr` at 0xc8784 | load | 4 | `abs 0x2ad190` | `-` |
| `ldrb` at 0xc878c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc8794 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc87a0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xc87a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc87a8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc87ac | store | 4 | `[sp-28] +0x0` | `?` |
| `ldr` at 0xc87b0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc87b4 | load | 1 | `sp -0x16` | `-` |
| `strb` at 0xc87b8 | store | 1 | `[sp-28] +0x4` | `([sp-22]:1 & 0xff)` |
| `ldr` at 0xc87bc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc87c0 | load | 1 | `sp -0x17` | `-` |
| `strb` at 0xc87c4 | store | 1 | `[sp-28] +0x5` | `([sp-23]:1 & 0xff)` |
| `ldr` at 0xc87c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc87cc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc87d0 | load | 1 | `[sp-32] +0x28` | `-` |
| `strb` at 0xc87d8 | store | 1 | `[sp-32] +0x28` | `([[sp-32]+40]:1+1 & 0xff)` |
| `ldmdb` at 0xc87dc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc87dc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc87dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc86fc | `(unknown callee) 0x21e28` | `0x9` | `0x133148` | `0x8c` | `0x0` |
| `bl` at 0xc8760 | `(unknown callee) 0x21e28` | `0x9` | `0x133148` | `0x94` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xc86d8 | `(r1 & 0xff) < [0x2ad194]:1 (unsigned, 32-bit)` | `(r1 & 0xff) >= [0x2ad194]:1 (unsigned, 32-bit)` |
| `bls` at 0xc8738 | `[[sp-32]+40]:1 <= 0x2 (unsigned, 32-bit)` | `[[sp-32]+40]:1 > 0x2 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc87dc | `r0` | `?` |

## `extIntIgnoreSpurious`

- `extIntIgnoreSpurious`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc8be4 to 0xc8cac.
  Established. A byte store records `(r0 & 0xff)` at `sp -0x11` (`strb` at 0xc8bfc). The unsigned
  lower-than branch takes `(r0 & 0xff) < [0x2ad194]:1 (unsigned, 32-bit)` and falls through at or
  above it (`blo` at 0xc8c10). The final byte store has an unknown target and value (`strb` at
  0xc8c94). The spurious-interrupt policy remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc8bf8 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xc8bfc | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xc8c00 | load | 4 | `abs 0xc8c9c` | `-` |
| `ldrb` at 0xc8c04 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xc8c08 | load | 1 | `abs 0x2ad194` | `-` |
| `ldr` at 0xc8c14 | load | 4 | `abs 0xc8ca0` | `-` |
| `str` at 0xc8c18 | store | 4 | `sp -0x20` | `0x1332e0` |
| `ldrb` at 0xc8c1c | load | 1 | `sp -0x11` | `-` |
| `str` at 0xc8c20 | store | 4 | `sp -0x1c` | `(r0 & 0xff)` |
| `ldr` at 0xc8c28 | load | 4 | `abs 0xc8ca4` | `-` |
| `ldrb` at 0xc8c38 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xc8c54 | load | 4 | `abs 0xc8ca8` | `-` |
| `ldr` at 0xc8c58 | load | 4 | `abs 0x2ad190` | `-` |
| `ldrb` at 0xc8c60 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xc8c7c | load | 4 | `abs 0xc8ca8` | `-` |
| `ldr` at 0xc8c80 | load | 4 | `abs 0x2ad190` | `-` |
| `ldrb` at 0xc8c88 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xc8c8c | load | 1 | `sp -0x18` | `-` |
| `strb` at 0xc8c94 | store | 1 | `(unknown)` | `?` |
| `ldmdb` at 0xc8c98 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc8c98 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc8c98 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc8c34 | `(unknown callee) 0x21e28` | `0x9` | `0x133148` | `0xfb` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xc8c10 | `(r0 & 0xff) < [0x2ad194]:1 (unsigned, 32-bit)` | `(r0 & 0xff) >= [0x2ad194]:1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc8c98 | `r0` | `?` |

## `AthenaCommPollInit`

- `AthenaCommPollInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2ce84 to 0x2cee0.
  Established. The stack receives word values `0x0`, `0x800` and `0xa` at `sp -0x28`, `sp -0x24` and
  `sp -0x20` (`str` at 0x2ce98, `str` at 0x2cea0, `str` at 0x2cea8). The call to `0x223f0` carries
  `0x147ed0`, `0x103914`, `0x2cee0` and `0x1` (`bl` at 0x2cecc); its result is the recorded return
  (`ldmdb` at 0x2ced0). Polling ownership and interval units remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2ce98 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x2cea0 | store | 4 | `sp -0x24` | `0x800` |
| `str` at 0x2cea8 | store | 4 | `sp -0x20` | `0xa` |
| `str` at 0x2ceb0 | store | 4 | `sp -0x1c` | `0xa` |
| `str` at 0x2ceb8 | store | 4 | `sp -0x18` | `0x9` |
| `ldr` at 0x2cebc | load | 4 | `abs 0x2ced4` | `-` |
| `ldr` at 0x2cec0 | load | 4 | `abs 0x2ced8` | `-` |
| `ldr` at 0x2cec4 | load | 4 | `abs 0x2cedc` | `-` |
| `ldmdb` at 0x2ced0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2ced0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2ced0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2cecc | `(unknown callee) 0x223f0` | `0x147ed0` | `0x103914` | `0x2cee0` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2ced0 | `r0` | `ret(0x223f0)` |

## `athenaComm`

- `athenaComm`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2cee0 to 0x2cf94.
  Established. A zero result from the call to `0x53bf0` takes the loop branch and nonzero falls
  through (`bl` at 0x2cefc, `beq` at 0x2cf08). The next call to `0xd6b9c` passes `0x0`, `0x9`,
  `sp-25` and `sp-25` (`bl` at 0x2cf1c); its truncated result is saved as a byte at `sp -0x1a`
  (`strb` at 0x2cf24). No exit is recorded on the known edges. The peripheral and error policy
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2cef0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x2cef4 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x2cf24 | store | 1 | `sp -0x1a` | `(ret(0xd6b9c) & 0xff)` |
| `ldrb` at 0x2cf28 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0x2cf38 | load | 4 | `abs 0x2cf88` | `-` |
| `str` at 0x2cf3c | store | 4 | `sp -0x20` | `0x103920` |
| `ldr` at 0x2cf44 | load | 4 | `abs 0x2cf8c` | `-` |
| `ldr` at 0x2cf48 | load | 4 | `abs 0x2cf90` | `-` |
| `ldrb` at 0x2cf54 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0x2cf64 | load | 1 | `sp -0x19` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2cefc | `(unknown callee) 0x53bf0` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x2cf1c | `(unknown callee) 0xd6b9c` | `0x0` | `0x9` | `sp-25` | `sp-25` |
| `bl` at 0x2cf50 | `(unknown callee) 0x21e28` | `0x1` | `0x1037bc` | `0x8c5` | `0x0` |
| `bl` at 0x2cf74 | `(unknown callee) 0x53d84` | `0x0` | `0x9` | `0x2` | `[sp-25]:1` |
| `bl` at 0x2cf80 | `(unknown callee) 0x2d260` | `0xfa` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x2cf08 | `ret(0x53bf0) == 0x0` | `ret(0x53bf0) != 0x0` |
| `beq` at 0x2cf34 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x2cf60 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|

## `jchip_dump_io`

- `jchip_dump_io`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeb734 to 0xeb810.
  Established. The opening call to `0x1b49c` uses `sp-72`, `0x136720`, entry r0 and entry r1 (`bl`
  at 0xeb768). A later call to `0xf7858` has `0x0`, `sp-72`, unknown and `[sp-32]` as candidates
  (`bl` at 0xeb7a8). The return records `ret(0xf7858)` (`ldmdb` at 0xeb804). The dumped I/O object
  and output destination remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xeb744 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xeb748 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xeb74c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xeb750 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xeb75c | load | 4 | `abs 0xeb808` | `-` |
| `ldr` at 0xeb760 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb764 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb770 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeb784 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeb7a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeb7b4 | load | 4 | `abs 0xeb80c` | `-` |
| `ldr` at 0xeb7b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb7bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb7c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeb7dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeb7fc | load | 4 | `sp +0x0` | `-` |
| `ldmdb` at 0xeb804 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xeb804 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xeb804 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeb768 | `(unknown callee) 0x1b49c` | `sp-72` | `0x136720` | `r0` | `r1` |
| `bl` at 0xeb7a8 | `(unknown callee) 0xf7858` | `0x0` | `sp-72` | `?` | `[sp-32]` |
| `bl` at 0xeb7c0 | `(unknown callee) 0x1b49c` | `sp-72` | `0x13672c` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xeb800 | `(unknown callee) 0xf7858` | `0x0` | `sp-72` | `?` | `[sp]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xeb804 | `r0` | `ret(0xf7858)` |

## `SocketExportInetConnection`

- `SocketExportInetConnection`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb83d8 to 0xb85a8.
  Established. A word store places `0x1` at `sp -0x2c` (`str` at 0xb83f0). The first call to
  `0x1b4b4` carries `0x2`, `0x1`, `0x0` and `0x1` (`bl` at 0xb8400); its signed nonnegative result
  takes the branch, while a negative result falls through (`bge` at 0xb8414). A later halfword store
  records `0x2` at `sp -0x24` (`strh` at 0xb84b0). Socket-family and export semantics remain unknown
  in the flat analysis.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb83e8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb83f0 | store | 4 | `sp -0x2c` | `0x1` |
| `str` at 0xb8408 | store | 4 | `sp -0x28` | `ret(0x1b4b4)` |
| `ldr` at 0xb840c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb8420 | load | 4 | `abs 0xb858c` | `-` |
| `ldr` at 0xb8424 | load | 4 | `abs 0xb8590` | `-` |
| `ldr` at 0xb8428 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xb842c | load | 4 | `abs 0xb8594` | `-` |
| `str` at 0xb8438 | store | 4 | `sp -0x30` | `?` |
| `str` at 0xb8448 | store | 4 | `sp -0x38` | `0x4` |
| `ldr` at 0xb844c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb8470 | load | 4 | `abs 0xb858c` | `-` |
| `ldr` at 0xb8474 | load | 4 | `abs 0xb8598` | `-` |
| `ldr` at 0xb8478 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xb847c | load | 4 | `abs 0xb859c` | `-` |
| `ldr` at 0xb8484 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb8490 | store | 4 | `sp -0x30` | `?` |
| `strh` at 0xb84b0 | store | 2 | `sp -0x24` | `0x2` |
| `ldrh` at 0xb84b4 | load | 2 | `sp -0x14` | `-` |
| `strh` at 0xb84c4 | store | 2 | `sp -0x22` | `(ret(0x1b424) & 0xffff)` |
| `str` at 0xb84cc | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xb84d4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb84fc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb8500 | store | 4 | `sp -0x38` | `[sp-20]` |
| `ldr` at 0xb8504 | load | 4 | `abs 0xb8594` | `-` |
| `str` at 0xb8508 | store | 4 | `sp -0x34` | `0x128590` |
| `ldr` at 0xb850c | load | 4 | `abs 0xb858c` | `-` |
| `ldr` at 0xb8510 | load | 4 | `abs 0xb85a0` | `-` |
| `ldr` at 0xb8514 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xb8518 | load | 4 | `abs 0xb8594` | `-` |
| `ldr` at 0xb8520 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb852c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xb8534 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb8554 | load | 4 | `abs 0xb858c` | `-` |
| `ldr` at 0xb8558 | load | 4 | `abs 0xb85a4` | `-` |
| `ldr` at 0xb855c | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xb8560 | load | 4 | `abs 0xb8594` | `-` |
| `ldr` at 0xb8568 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb8574 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xb857c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb8580 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xb8584 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xb8588 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb8588 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb8588 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb8400 | `(unknown callee) 0x1b4b4` | `0x2` | `0x1` | `0x0` | `0x1` |
| `bl` at 0xb8418 | `(unknown callee) 0x1b0b8` | `ret(0x1b4b4)` | `?` | `?` | `ret(0x1b4b4)` |
| `bl` at 0xb8430 | `(unknown callee) 0x212dc` | `0xfffffe70` | `0x128588` | `[ret(0x1b0b8)]` | `0x128590` |
| `bl` at 0xb8458 | `(unknown callee) 0x1b1c0` | `ret(0x1b4b4)` | `0x1` | `0x2` | `sp-44` |
| `bl` at 0xb8468 | `(unknown callee) 0x1b0b8` | `ret(0x1b1c0)` | `?` | `?` | `ret(0x1b1c0)` |
| `bl` at 0xb8480 | `(unknown callee) 0x212dc` | `0xfffffe70` | `0x128594` | `[ret(0x1b0b8)]` | `0x1285a0` |
| `bl` at 0xb8488 | `(unknown callee) 0x1b004` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xb84a8 | `(unknown callee) 0x1b43c` | `sp-36` | `0x0` | `0x10` | `sp-36` |
| `bl` at 0xb84bc | `(unknown callee) 0x1b424` | `[sp-20]:2` | `?` | `?` | `[sp-20]:2` |
| `bl` at 0xb84e0 | `(unknown callee) 0x1b2e0` | `[sp-40]` | `sp-36` | `0x10` | `sp-36` |
| `bl` at 0xb84f0 | `(unknown callee) 0x1b0b8` | `ret(0x1b2e0)` | `?` | `?` | `ret(0x1b2e0)` |
| `bl` at 0xb851c | `(unknown callee) 0x21340` | `0xfffffe70` | `0x1285b0` | `[ret(0x1b0b8)]` | `0x128590` |
| `bl` at 0xb8524 | `(unknown callee) 0x1b004` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xb853c | `(unknown callee) 0x1b118` | `[sp-40]` | `0x5` | `?` | `ret(0x1b2e0)` |
| `bl` at 0xb854c | `(unknown callee) 0x1b0b8` | `ret(0x1b118)` | `?` | `?` | `ret(0x1b118)` |
| `bl` at 0xb8564 | `(unknown callee) 0x212dc` | `0xfffffe70` | `0x1285b8` | `[ret(0x1b0b8)]` | `0x128590` |
| `bl` at 0xb856c | `(unknown callee) 0x1b004` | `[sp-40]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xb8414 | `ret(0x1b4b4) >= 0x0 (signed, 32-bit)` | `ret(0x1b4b4) < 0x0 (signed, 32-bit)` |
| `bge` at 0xb8464 | `ret(0x1b1c0) >= 0x0 (signed, 32-bit)` | `ret(0x1b1c0) < 0x0 (signed, 32-bit)` |
| `bge` at 0xb84ec | `ret(0x1b2e0) >= 0x0 (signed, 32-bit)` | `ret(0x1b2e0) < 0x0 (signed, 32-bit)` |
| `bge` at 0xb8548 | `ret(0x1b118) >= 0x0 (signed, 32-bit)` | `ret(0x1b118) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb8588 | `r0` | `[sp-48]` |

## `SocketExportUnixConnection`

- `SocketExportUnixConnection`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb85a8 to 0xb86fc.
  Established. The call to `0x1b4b4` has candidates `0x1`, `0x1`, `0x0` and entry r3 (`bl` at
  0xb85c8). A signed nonnegative result takes the first branch and a negative result falls through
  (`bge` at 0xb85dc). The word save at `sp -0x8c` contains `ret(0xb95a0)` (`str` at 0xb861c). The
  address representation and external export action remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb85b8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb85d0 | store | 4 | `sp -0x88` | `ret(0x1b4b4)` |
| `ldr` at 0xb85d4 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xb85ec | load | 4 | `abs 0xb86e8` | `-` |
| `ldr` at 0xb85f0 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xb85f4 | load | 4 | `abs 0xb86ec` | `-` |
| `str` at 0xb8600 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xb8610 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb861c | store | 4 | `sp -0x8c` | `ret(0xb95a0)` |
| `ldr` at 0xb8634 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xb863c | load | 4 | `sp -0x8c` | `-` |
| `str` at 0xb8664 | store | 4 | `sp -0x94` | `sp-130` |
| `ldr` at 0xb866c | load | 4 | `abs 0xb86f0` | `-` |
| `ldr` at 0xb8670 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xb8674 | load | 4 | `abs 0xb86f4` | `-` |
| `ldr` at 0xb867c | load | 4 | `sp -0x88` | `-` |
| `str` at 0xb8688 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xb8690 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xb86b4 | load | 4 | `abs 0xb86f8` | `-` |
| `ldr` at 0xb86b8 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xb86bc | load | 4 | `abs 0xb86ec` | `-` |
| `ldr` at 0xb86c4 | load | 4 | `sp -0x88` | `-` |
| `str` at 0xb86d0 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xb86d8 | load | 4 | `sp -0x88` | `-` |
| `str` at 0xb86dc | store | 4 | `sp -0x90` | `[sp-136]` |
| `ldr` at 0xb86e0 | load | 4 | `sp -0x90` | `-` |
| `ldmdb` at 0xb86e4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb86e4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb86e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb85c8 | `(unknown callee) 0x1b4b4` | `0x1` | `0x1` | `0x0` | `r3` |
| `bl` at 0xb85e0 | `(unknown callee) 0x1b0b8` | `ret(0x1b4b4)` | `?` | `?` | `ret(0x1b4b4)` |
| `bl` at 0xb85f8 | `(unknown callee) 0x212dc` | `?` | `0x128588` | `[ret(0x1b0b8)]` | `0x128590` |
| `bl` at 0xb8614 | `(unknown callee) 0xb95a0` | `sp-132` | `[sp-20]` | `?` | `sp-132` |
| `bl` at 0xb862c | `(unknown callee) 0x1b040` | `sp-130` | `?` | `?` | `sp-130` |
| `bl` at 0xb8640 | `(unknown callee) 0x1b2e0` | `[sp-136]` | `sp-132` | `[sp-140]` | `sp-132` |
| `bl` at 0xb8650 | `(unknown callee) 0x1b0b8` | `ret(0x1b2e0)` | `?` | `?` | `ret(0x1b2e0)` |
| `bl` at 0xb8678 | `(unknown callee) 0x212dc` | `?` | `0x1285b0` | `[ret(0x1b0b8)]` | `0x1285c0` |
| `bl` at 0xb8680 | `(unknown callee) 0x1b004` | `[sp-136]` | `?` | `?` | `?` |
| `bl` at 0xb8698 | `(unknown callee) 0x1b118` | `[sp-136]` | `0x5` | `?` | `ret(0x1b2e0)` |
| `bl` at 0xb86a8 | `(unknown callee) 0x1b0b8` | `ret(0x1b118)` | `?` | `?` | `ret(0x1b118)` |
| `bl` at 0xb86c0 | `(unknown callee) 0x212dc` | `?` | `0x1285b8` | `[ret(0x1b0b8)]` | `0x128590` |
| `bl` at 0xb86c8 | `(unknown callee) 0x1b004` | `[sp-136]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xb85dc | `ret(0x1b4b4) >= 0x0 (signed, 32-bit)` | `ret(0x1b4b4) < 0x0 (signed, 32-bit)` |
| `bge` at 0xb864c | `ret(0x1b2e0) >= 0x0 (signed, 32-bit)` | `ret(0x1b2e0) < 0x0 (signed, 32-bit)` |
| `bge` at 0xb86a4 | `ret(0x1b118) >= 0x0 (signed, 32-bit)` | `ret(0x1b118) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb86e4 | `r0` | `[sp-144]` |

## `SocketAcceptInetConnection`

- `SocketAcceptInetConnection`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb86fc to 0xb87e8.
  Established. A stack word is set to `0x10` at `sp -0x30` (`str` at 0xb8718). The call to `0x1b0c4`
  carries entry r0, `sp-44`, `sp-48` and `sp-44` (`bl` at 0xb872c). The result test takes
  nonnegative values and falls through on negative values, using a signed comparison (`bge` at
  0xb8740). The accepted object and final r0 value remain unknown (`ldmdb` at 0xb87d0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb870c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb8710 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb8718 | store | 4 | `sp -0x30` | `0x10` |
| `ldr` at 0xb8724 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb8734 | store | 4 | `sp -0x1c` | `ret(0x1b0c4)` |
| `ldr` at 0xb8738 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb874c | load | 4 | `abs 0xb87d4` | `-` |
| `ldr` at 0xb8750 | load | 4 | `abs 0xb87d8` | `-` |
| `ldr` at 0xb8754 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xb8758 | load | 4 | `abs 0xb87dc` | `-` |
| `str` at 0xb8764 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xb876c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8778 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb879c | load | 4 | `abs 0xb87d4` | `-` |
| `ldr` at 0xb87a0 | load | 4 | `abs 0xb87e0` | `-` |
| `ldr` at 0xb87a4 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xb87a8 | load | 4 | `abs 0xb87e4` | `-` |
| `ldr` at 0xb87b0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb87bc | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xb87c4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb87c8 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xb87cc | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0xb87d0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb87d0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb87d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb872c | `(unknown callee) 0x1b0c4` | `r0` | `sp-44` | `sp-48` | `sp-44` |
| `bl` at 0xb8744 | `(unknown callee) 0x1b0b8` | `ret(0x1b0c4)` | `?` | `?` | `ret(0x1b0c4)` |
| `bl` at 0xb875c | `(unknown callee) 0x212dc` | `0xfffffe6e` | `0x1285c4` | `[ret(0x1b0b8)]` | `0x128590` |
| `bl` at 0xb8784 | `(unknown callee) 0x1b2f8` | `ret(0x1b0c4)` | `0x4` | `0x800` | `[sp-24]` |
| `bl` at 0xb8794 | `(unknown callee) 0x1b0b8` | `ret(0x1b2f8)` | `?` | `?` | `ret(0x1b2f8)` |
| `bl` at 0xb87ac | `(unknown callee) 0x212dc` | `0xfffffe6e` | `0x1285cc` | `[ret(0x1b0b8)]` | `0x1285d4` |
| `bl` at 0xb87b4 | `(unknown callee) 0x1b004` | `[sp-28]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xb8740 | `ret(0x1b0c4) >= 0x0 (signed, 32-bit)` | `ret(0x1b0c4) < 0x0 (signed, 32-bit)` |
| `beq` at 0xb8774 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0xb8790 | `ret(0x1b2f8) == 0x0` | `ret(0x1b2f8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb87d0 | `r0` | `?` |

## `SocketAcceptUnixConnection`

- `SocketAcceptUnixConnection`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb87e8 to 0xb88d4.
  Established. A word store supplies `0x70` at `sp -0x90` (`str` at 0xb8804). The opening call to
  `0x1b0c4` passes entry r0, `sp-140`, `sp-144` and `sp-140` (`bl` at 0xb8818). Signed nonnegative
  results take the branch and negative results fall through (`bge` at 0xb882c). The meaning of the
  larger stack object and acceptance result remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb87f8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb87fc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb8804 | store | 4 | `sp -0x90` | `0x70` |
| `ldr` at 0xb8810 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb8820 | store | 4 | `sp -0x1c` | `ret(0x1b0c4)` |
| `ldr` at 0xb8824 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8838 | load | 4 | `abs 0xb88c0` | `-` |
| `ldr` at 0xb883c | load | 4 | `abs 0xb88c4` | `-` |
| `ldr` at 0xb8840 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xb8844 | load | 4 | `abs 0xb88c8` | `-` |
| `str` at 0xb8850 | store | 4 | `sp -0x94` | `?` |
| `ldr` at 0xb8858 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8864 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8888 | load | 4 | `abs 0xb88c0` | `-` |
| `ldr` at 0xb888c | load | 4 | `abs 0xb88cc` | `-` |
| `ldr` at 0xb8890 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xb8894 | load | 4 | `abs 0xb88d0` | `-` |
| `ldr` at 0xb889c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb88a8 | store | 4 | `sp -0x94` | `?` |
| `ldr` at 0xb88b0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb88b4 | store | 4 | `sp -0x94` | `?` |
| `ldr` at 0xb88b8 | load | 4 | `sp -0x94` | `-` |
| `ldmdb` at 0xb88bc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb88bc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb88bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb8818 | `(unknown callee) 0x1b0c4` | `r0` | `sp-140` | `sp-144` | `sp-140` |
| `bl` at 0xb8830 | `(unknown callee) 0x1b0b8` | `ret(0x1b0c4)` | `?` | `?` | `ret(0x1b0c4)` |
| `bl` at 0xb8848 | `(unknown callee) 0x212dc` | `0xfffffe6d` | `0x1285c4` | `[ret(0x1b0b8)]` | `0x128590` |
| `bl` at 0xb8870 | `(unknown callee) 0x1b2f8` | `ret(0x1b0c4)` | `0x4` | `0x800` | `[sp-24]` |
| `bl` at 0xb8880 | `(unknown callee) 0x1b0b8` | `ret(0x1b2f8)` | `?` | `?` | `ret(0x1b2f8)` |
| `bl` at 0xb8898 | `(unknown callee) 0x212dc` | `0xfffffe6d` | `0x1285cc` | `[ret(0x1b0b8)]` | `0x1285d4` |
| `bl` at 0xb88a0 | `(unknown callee) 0x1b004` | `[sp-28]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xb882c | `ret(0x1b0c4) >= 0x0 (signed, 32-bit)` | `ret(0x1b0c4) < 0x0 (signed, 32-bit)` |
| `beq` at 0xb8860 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0xb887c | `ret(0x1b2f8) == 0x0` | `ret(0x1b2f8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb88bc | `r0` | `?` |

## `SocketNodelayMode`

- `SocketNodelayMode`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb88d4 to 0xb8960.
  Established. Zero entry r1 takes the initial branch and nonzero falls through (`beq` at 0xb88f4).
  The call to `0x1b2f8` uses entry r0, `0x4`, `0x800` and entry r1 (`bl` at 0xb8904). A later word
  store clears `sp -0x1c` (`str` at 0xb8948). The flag meaning and final merged value are unknown
  (`ldmdb` at 0xb8950).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb88e4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb88e8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xb88ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb88f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb891c | load | 4 | `abs 0xb8954` | `-` |
| `ldr` at 0xb8920 | load | 4 | `abs 0xb8958` | `-` |
| `ldr` at 0xb8924 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xb8928 | load | 4 | `abs 0xb895c` | `-` |
| `ldr` at 0xb8930 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb893c | store | 4 | `sp -0x1c` | `?` |
| `str` at 0xb8948 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xb894c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xb8950 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb8950 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb8950 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb8904 | `(unknown callee) 0x1b2f8` | `r0` | `0x4` | `0x800` | `r1` |
| `bl` at 0xb8914 | `(unknown callee) 0x1b0b8` | `ret(0x1b2f8)` | `?` | `?` | `ret(0x1b2f8)` |
| `bl` at 0xb892c | `(unknown callee) 0x212dc` | `0xfffffe6c` | `0x1285cc` | `[ret(0x1b0b8)]` | `0x1285d4` |
| `bl` at 0xb8934 | `(unknown callee) 0x1b004` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb88f4 | `r1 == 0x0` | `r1 != 0x0` |
| `beq` at 0xb8910 | `ret(0x1b2f8) == 0x0` | `ret(0x1b2f8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb8950 | `r0` | `?` |

## `SocketMakeInetConnection`

- `SocketMakeInetConnection`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8960 to 0xb8b58.
  Established. The call to `0xb9128` carries entry r0, `sp-48`, entry r2 and `sp-48` (`bl` at
  0xb8988); its zero result takes the branch and nonzero falls through (`beq` at 0xb8994). A later
  word store saves `ret(0x1b4b4)` at `sp -0x34` (`str` at 0xb89dc). The computed transfer leaves
  unresolved edges (`ldrls` at 0xb8a98), so the connection outcome is unknown beyond the recorded
  paths.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb8970 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb8974 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb8978 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xb8980 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb899c | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xb89a0 | load | 4 | `abs 0xb8b44` | `-` |
| `str` at 0xb89a4 | store | 4 | `sp -0x40` | `0x128590` |
| `ldr` at 0xb89a8 | load | 4 | `abs 0xb8b48` | `-` |
| `ldr` at 0xb89ac | load | 4 | `abs 0xb8b44` | `-` |
| `ldr` at 0xb89b4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb89c0 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xb89dc | store | 4 | `sp -0x34` | `ret(0x1b4b4)` |
| `ldr` at 0xb89e0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb89f4 | load | 4 | `abs 0xb8b4c` | `-` |
| `ldr` at 0xb89f8 | load | 4 | `abs 0xb8b50` | `-` |
| `ldr` at 0xb89fc | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xb8a00 | load | 4 | `abs 0xb8b44` | `-` |
| `str` at 0xb8a0c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xb8a14 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb8a18 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb8a24 | store | 4 | `sp -0x38` | `ret(0xb88d4)` |
| `ldr` at 0xb8a28 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xb8a34 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xb8a38 | store | 4 | `sp -0x3c` | `ret(0xb88d4)` |
| `strh` at 0xb8a44 | store | 2 | `sp -0x2c` | `0x2` |
| `ldrh` at 0xb8a48 | load | 2 | `sp -0x18` | `-` |
| `strh` at 0xb8a58 | store | 2 | `sp -0x2a` | `(ret(0x1b424) & 0xffff)` |
| `ldr` at 0xb8a5c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xb8a60 | store | 4 | `sp -0x28` | `[sp-48]` |
| `ldr` at 0xb8a68 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb8a8c | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldrls` at 0xb8a98 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb8afc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb8b00 | store | 4 | `sp -0x44` | `[sp-24]` |
| `ldr` at 0xb8b04 | load | 4 | `abs 0xb8b44` | `-` |
| `str` at 0xb8b08 | store | 4 | `sp -0x40` | `0x128590` |
| `ldr` at 0xb8b0c | load | 4 | `abs 0xb8b4c` | `-` |
| `ldr` at 0xb8b10 | load | 4 | `abs 0xb8b54` | `-` |
| `ldr` at 0xb8b14 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xb8b18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8b20 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xb8b2c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xb8b34 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xb8b38 | store | 4 | `sp -0x3c` | `[sp-52]` |
| `ldr` at 0xb8b3c | load | 4 | `sp -0x3c` | `-` |
| `ldmdb` at 0xb8b40 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb8b40 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb8b40 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb8988 | `(unknown callee) 0xb9128` | `r0` | `sp-48` | `r2` | `sp-48` |
| `bl` at 0xb89b8 | `(unknown callee) 0x21340` | `0xfffffe6a` | `0x128590` | `0x0` | `[sp-20]` |
| `bl` at 0xb89d4 | `(unknown callee) 0x1b4b4` | `0x2` | `0x1` | `0x0` | `ret(0xb9128)` |
| `bl` at 0xb89ec | `(unknown callee) 0x1b0b8` | `ret(0x1b4b4)` | `?` | `?` | `ret(0x1b4b4)` |
| `bl` at 0xb8a04 | `(unknown callee) 0x212dc` | `0xfffffe6c` | `0x128588` | `[ret(0x1b0b8)]` | `0x128590` |
| `bl` at 0xb8a1c | `(unknown callee) 0xb88d4` | `ret(0x1b4b4)` | `[sp-28]` | `?` | `ret(0x1b4b4)` |
| `bl` at 0xb8a50 | `(unknown callee) 0x1b424` | `[sp-24]:2` | `?` | `?` | `[sp-24]:2` |
| `bl` at 0xb8a74 | `(unknown callee) 0x1b448` | `[sp-52]` | `sp-44` | `0x10` | `sp-44` |
| `bl` at 0xb8a84 | `(unknown callee) 0x1b0b8` | `ret(0x1b448)` | `?` | `?` | `ret(0x1b448)` |
| `bl` at 0xb8af0 | `(unknown callee) 0x1b0b8` | `ret(0x1b0b8)` | `?` | `?` | `[ret(0x1b0b8)]-101` |
| `bl` at 0xb8b1c | `(unknown callee) 0x21340` | `0xfffffe6c` | `0x1285e0` | `[ret(0x1b0b8)]` | `[sp-20]` |
| `bl` at 0xb8b24 | `(unknown callee) 0x1b004` | `[sp-52]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb8994 | `ret(0xb9128) == 0x0` | `ret(0xb9128) != 0x0` |
| `bge` at 0xb89e8 | `ret(0x1b4b4) >= 0x0 (signed, 32-bit)` | `ret(0x1b4b4) < 0x0 (signed, 32-bit)` |
| `beq` at 0xb8a30 | `ret(0xb88d4) == 0x0` | `ret(0xb88d4) != 0x0` |
| `beq` at 0xb8a80 | `ret(0x1b448) == 0x0` | `ret(0x1b448) != 0x0` |
| `ldrls` at 0xb8a98 | `[ret(0x1b0b8)]-101 <= 0xe (unsigned, 32-bit)` | `[ret(0x1b0b8)]-101 > 0xe (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb8b40 | `r0` | `[sp-60]` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xb8a98).

## `SocketMakeUnixConnection`

- `SocketMakeUnixConnection`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8b58 to 0xb8d10.
  Established. A call to `0x1b4b4` receives `0x1`, `0x1`, `0x0` and entry r3 (`bl` at 0xb8b7c); its
  signed nonnegative result takes the branch and negative values fall through (`bge` at 0xb8b90).
  The word save at `sp -0x90` records `ret(0xb95a0)` (`str` at 0xb8c28). The connection format
  remains unknown, as does the meaning of the later error classification.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb8b68 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb8b6c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb8b84 | store | 4 | `sp -0x8c` | `ret(0x1b4b4)` |
| `ldr` at 0xb8b88 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xb8ba0 | load | 4 | `abs 0xb8cf8` | `-` |
| `ldr` at 0xb8ba4 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xb8ba8 | load | 4 | `abs 0xb8cfc` | `-` |
| `str` at 0xb8bb4 | store | 4 | `sp -0x94` | `?` |
| `ldr` at 0xb8bbc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8bc8 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xb8bf0 | load | 4 | `abs 0xb8d00` | `-` |
| `ldr` at 0xb8bf4 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xb8bf8 | load | 4 | `abs 0xb8d04` | `-` |
| `ldr` at 0xb8c00 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0xb8c0c | store | 4 | `sp -0x94` | `?` |
| `ldr` at 0xb8c1c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb8c28 | store | 4 | `sp -0x90` | `ret(0xb95a0)` |
| `ldr` at 0xb8c30 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xb8c38 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xb8c54 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `str` at 0xb8c58 | store | 4 | `sp -0x98` | `[ret(0x1b0b8)]` |
| `ldr` at 0xb8c5c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0xb8c68 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0xb8c74 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0xb8c84 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0xb8c94 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0xb8ca0 | store | 4 | `sp -0x94` | `?` |
| `str` at 0xb8cbc | store | 4 | `sp -0x9c` | `sp-134` |
| `ldr` at 0xb8cc4 | load | 4 | `abs 0xb8d08` | `-` |
| `ldr` at 0xb8cc8 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xb8ccc | load | 4 | `abs 0xb8d0c` | `-` |
| `ldr` at 0xb8cd4 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0xb8ce0 | store | 4 | `sp -0x94` | `?` |
| `ldr` at 0xb8ce8 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0xb8cec | store | 4 | `sp -0x94` | `[sp-140]` |
| `ldr` at 0xb8cf0 | load | 4 | `sp -0x94` | `-` |
| `ldmdb` at 0xb8cf4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb8cf4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb8cf4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb8b7c | `(unknown callee) 0x1b4b4` | `0x1` | `0x1` | `0x0` | `r3` |
| `bl` at 0xb8b94 | `(unknown callee) 0x1b0b8` | `ret(0x1b4b4)` | `?` | `?` | `ret(0x1b4b4)` |
| `bl` at 0xb8bac | `(unknown callee) 0x212dc` | `?` | `0x128588` | `[ret(0x1b0b8)]` | `0x128590` |
| `bl` at 0xb8bd4 | `(unknown callee) 0x1b2f8` | `ret(0x1b4b4)` | `0x4` | `0x800` | `[sp-24]` |
| `bl` at 0xb8be4 | `(unknown callee) 0x1b0b8` | `ret(0x1b2f8)` | `?` | `?` | `ret(0x1b2f8)` |
| `bl` at 0xb8bfc | `(unknown callee) 0x212dc` | `?` | `0x1285cc` | `[ret(0x1b0b8)]` | `0x1285d4` |
| `bl` at 0xb8c04 | `(unknown callee) 0x1b004` | `[sp-140]` | `?` | `?` | `?` |
| `bl` at 0xb8c20 | `(unknown callee) 0xb95a0` | `sp-136` | `[sp-20]` | `?` | `sp-136` |
| `bl` at 0xb8c3c | `(unknown callee) 0x1b448` | `[sp-140]` | `sp-136` | `ret(0xb95a0)` | `sp-136` |
| `bl` at 0xb8c4c | `(unknown callee) 0x1b0b8` | `ret(0x1b448)` | `?` | `?` | `ret(0x1b448)` |
| `bl` at 0xb8c98 | `(unknown callee) 0x1b004` | `[sp-140]` | `?` | `?` | `[ret(0x1b0b8)]` |
| `bl` at 0xb8ca8 | `(unknown callee) 0x1b0b8` | `ret(0x1b0b8)` | `?` | `?` | `[ret(0x1b0b8)]` |
| `bl` at 0xb8cd0 | `(unknown callee) 0x212dc` | `?` | `0x1285e0` | `[ret(0x1b0b8)]` | `0x1285c0` |
| `bl` at 0xb8cd8 | `(unknown callee) 0x1b004` | `[sp-140]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xb8b90 | `ret(0x1b4b4) >= 0x0 (signed, 32-bit)` | `ret(0x1b4b4) < 0x0 (signed, 32-bit)` |
| `beq` at 0xb8bc4 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0xb8be0 | `ret(0x1b2f8) == 0x0` | `ret(0x1b2f8) != 0x0` |
| `beq` at 0xb8c48 | `ret(0x1b448) == 0x0` | `ret(0x1b448) != 0x0` |
| `bgt` at 0xb8c64 | `[ret(0x1b0b8)] > 0x6f (signed, 32-bit)` | `[ret(0x1b0b8)] <= 0x6f (signed, 32-bit)` |
| `bge` at 0xb8c70 | `[ret(0x1b0b8)] >= 0x6e (signed, 32-bit)` | `[ret(0x1b0b8)] < 0x6e (signed, 32-bit)` |
| `beq` at 0xb8c7c | `[ret(0x1b0b8)] == 0x2` | `[ret(0x1b0b8)] != 0x2` |
| `beq` at 0xb8c8c | `[ret(0x1b0b8)] == 0x73` | `[ret(0x1b0b8)] != 0x73` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb8cf4 | `r0` | `[sp-148]` |

## `SocketGetPeerAddr`

- `SocketGetPeerAddr`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb91e4 to 0xb9274.
  Established. The stack word at `sp -0x2c` is initialised to `0x10` (`str` at 0xb9200). A call to
  `0x1b1d8` carries entry r0, `sp-40`, `sp-44` and `sp-40` (`bl` at 0xb9214). Signed nonnegative
  results take the branch and negative results fall through (`bge` at 0xb9220). On the taken path, a
  word store writes `[sp-36]` to `[sp-24] +0x0` (`str` at 0xb9254). The peer-address representation
  remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb91f4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb91f8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb9200 | store | 4 | `sp -0x2c` | `0x10` |
| `ldr` at 0xb920c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb922c | load | 4 | `abs 0xb9268` | `-` |
| `ldr` at 0xb9230 | load | 4 | `abs 0xb926c` | `-` |
| `ldr` at 0xb9234 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xb9238 | load | 4 | `abs 0xb9270` | `-` |
| `str` at 0xb9244 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xb924c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb9250 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb9254 | store | 4 | `[sp-24] +0x0` | `[sp-36]` |
| `str` at 0xb925c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xb9260 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xb9264 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb9264 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb9264 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb9214 | `(unknown callee) 0x1b1d8` | `r0` | `sp-40` | `sp-44` | `sp-40` |
| `bl` at 0xb9224 | `(unknown callee) 0x1b0b8` | `ret(0x1b1d8)` | `?` | `?` | `ret(0x1b1d8)` |
| `bl` at 0xb923c | `(unknown callee) 0x212dc` | `0xfffffe69` | `0x128698` | `[ret(0x1b0b8)]` | `0x128590` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xb9220 | `ret(0x1b1d8) >= 0x0 (signed, 32-bit)` | `ret(0x1b1d8) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb9264 | `r0` | `?` |
