# ARM L1 I/O-expander interrupt and dispatch paths in 1.34.2

Read from `6.5.27/usr/cpu/firmware/sysco/l1.bin`, package `6.5.27:l1#3` (version 1.34.2),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x3b0c1c`.
Carrier SHA-256: `45c872a661d4a4ad20a4cb482fe2aee921ee9ae52784382be125b3d7b7836572`.
Package SHA-256: `d8561deedcf6bae5bef085e0420886534c72bf6c59c54b742d086b9998264128`.
Inner executable SHA-256: `65c32185ef6f284717fd705e751157059ffed1b028bd6195f92eac73749012d3`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1069932 bytes.
Segment SHA-256: `c92d999ce63930093f87780073854fbb74f634d99098b88b7830a55634c9de6c`. Analysed routine spans use the exact
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
An unresolved computed transfer can leave additional effects and return paths unknown.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/io-expander-interrupt-facts.txt) retain all tool sections.

## `ioExpWriteValue`

- `ioExpWriteValue`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7b78 to 0xb7bb4.
  Established. The call to 0xb8348 has r0 candidate `(r0 & 0xff)` (`bl` at 0xb7b98). A byte load at
  its result +0x1 supplies the return expression `[ret(0xb8348)+1]:1` (`ldrb` at 0xb7ba8, `ldmdb` at
  0xb7bb0). The byte's physical write-value meaning is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb7b8c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xb7b90 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb7ba0 | store | 4 | `sp -0x18` | `ret(0xb8348)` |
| `ldr` at 0xb7ba4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb7ba8 | load | 1 | `ret(0xb8348) +0x1` | `-` |
| `ldmdb` at 0xb7bb0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb7bb0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb7bb0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7b98 | `(unknown callee) 0xb8348` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb7bb0 | `r0` | `[ret(0xb8348)+1]:1` |

## `ioExpIntCount`

- `ioExpIntCount`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7bb4 to 0xb7bf0.
  Established. Entry r0's masked byte is saved at sp -0x11 before the call to 0xb8348 (`strb` at
  0xb7bc8, `bl` at 0xb7bd4). The word read from that call result +0x8 reaches the return in r0 as
  `[ret(0xb8348)+8]` (`ldr` at 0xb7be4, `ldmdb` at 0xb7bec); its interpretation is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb7bc8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xb7bcc | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb7bdc | store | 4 | `sp -0x18` | `ret(0xb8348)` |
| `ldr` at 0xb7be0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7be4 | load | 4 | `ret(0xb8348) +0x8` | `-` |
| `ldmdb` at 0xb7bec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb7bec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb7bec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7bd4 | `(unknown callee) 0xb8348` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb7bec | `r0` | `[ret(0xb8348)+8]` |

## `ioExpIntMask`

- `ioExpIntMask`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7bf0 to 0xb7c2c.
  Established. A byte save at sp -0x11 retains `(r0 & 0xff)` (`strb` at 0xb7c04). The call to
  0xb8348 uses that expression as its first candidate (`bl` at 0xb7c10). The following byte load at
  the call result +0x3 becomes `[ret(0xb8348)+3]:1` at return (`ldrb` at 0xb7c20, `ldmdb` at
  0xb7c28). The interrupt-mask encoding remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb7c04 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xb7c08 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb7c18 | store | 4 | `sp -0x18` | `ret(0xb8348)` |
| `ldr` at 0xb7c1c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb7c20 | load | 1 | `ret(0xb8348) +0x3` | `-` |
| `ldmdb` at 0xb7c28 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb7c28 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb7c28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7c10 | `(unknown callee) 0xb8348` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb7c28 | `r0` | `[ret(0xb8348)+3]:1` |

## `ioExpIntDisable`

- `ioExpIntDisable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7c2c to 0xb7c74.
  Established. Two byte stores preserve masked entry r0 and r1 at sp -0x11 and sp -0x12 (`strb` at
  0xb7c44, `strb` at 0xb7c4c). The call to 0xb880c supplies candidates `(r0 & 0xff)`, `(r1 & 0xff)`,
  0x0 and `(r0 & 0xff)` (`bl` at 0xb7c64). That call's result reaches the return in r0 (`ldmdb` at
  0xb7c70). The physical interrupt-disable effect is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb7c44 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xb7c4c | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldrb` at 0xb7c50 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xb7c54 | load | 1 | `sp -0x12` | `-` |
| `ldmdb` at 0xb7c70 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb7c70 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb7c70 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7c64 | `(unknown callee) 0xb880c` | `(r0 & 0xff)` | `(r1 & 0xff)` | `0x0` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb7c70 | `r0` | `ret(0xb880c)` |

## `ioExpIntEnable`

- `ioExpIntEnable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7c74 to 0xb7cbc.
  Established. The first and second call candidates at 0xb880c are `(r0 & 0xff)` and `(r1 & 0xff)`,
  with third candidate 0x1 (`bl` at 0xb7cac). The byte saves at sp -0x11 and sp -0x12 retain those
  entry expressions (`strb` at 0xb7c8c, `strb` at 0xb7c94); its return carries `ret(0xb880c)`
  (`ldmdb` at 0xb7cb8). No interrupt-enable hardware meaning is established here.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb7c8c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xb7c94 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldrb` at 0xb7c98 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xb7c9c | load | 1 | `sp -0x12` | `-` |
| `ldmdb` at 0xb7cb8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb7cb8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb7cb8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7cac | `(unknown callee) 0xb880c` | `(r0 & 0xff)` | `(r1 & 0xff)` | `0x1` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb7cb8 | `r0` | `ret(0xb880c)` |

## `ioExpIntSet`

- `ioExpIntSet`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7cbc to 0xb7d04.
  Established. Entry r2 is saved as a word at sp -0x18 (`str` at 0xb7cd0). The call to 0xb880c has
  candidates `(r0 & 0xff)`, `(r1 & 0xff)`, entry r2 and `(r0 & 0xff)` (`bl` at 0xb7cf4). The return
  preserves the call result in r0 (`ldmdb` at 0xb7d00). The third candidate's operation or mask
  meaning remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb7cd0 | store | 4 | `sp -0x18` | `r2` |
| `strb` at 0xb7cd4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xb7cdc | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldrb` at 0xb7ce0 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xb7ce4 | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0xb7cf0 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xb7d00 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb7d00 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb7d00 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7cf4 | `(unknown callee) 0xb880c` | `(r0 & 0xff)` | `(r1 & 0xff)` | `r2` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb7d00 | `r0` | `ret(0xb880c)` |

## `ioExpInt`

- `ioExpInt`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7d04 to 0xb7eb8.
  Established. A word at sp -0x24 begins at zero (`str` at 0xb7d2c). The loop's `bls` predicate has
  unknown operands (`bls` at 0xb7d44). Later `beq` sites select paths for `[[sp-24]]:1` equal to
  0x2, 0x0 and 0x3 (`beq` at 0xb7dbc, `beq` at 0xb7dd4, `beq` at 0xb7de4). Calls on those paths
  include 0xb8a40, 0xba214 and 0xad294, whose results appear separately in word stores added to
  `[sp-36]` (`bl` at 0xb7dfc, `bl` at 0xb7e24, `bl` at 0xb7e4c, `str` at 0xb7e0c, `str` at 0xb7e34,
  `str` at 0xb7e5c). The aggregate's interrupt meaning and iteration count remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb7d1c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xb7d24 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `str` at 0xb7d2c | store | 4 | `sp -0x24` | `0x0` |
| `ldrb` at 0xb7d30 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0xb7d34 | store | 1 | `sp -0x1d` | `((r0 & 0xff) & 0xff)` |
| `ldrb` at 0xb7d38 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0xb7d3c | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0xb7d4c | load | 4 | `abs 0xb7ea8` | `-` |
| `ldrb` at 0xb7d50 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xb7d58 | load | 4 | `abs 0x284180` | `-` |
| `str` at 0xb7d60 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xb7d64 | load | 4 | `abs 0xb7eac` | `-` |
| `ldrb` at 0xb7d68 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xb7d7c | load | 4 | `abs 0x284160` | `-` |
| `str` at 0xb7d84 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xb7d88 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7d8c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xb7d98 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb7d9c | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xb7da8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb7dac | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0xb7db0 | store | 4 | `sp -0x28` | `[[sp-24]]:1` |
| `ldr` at 0xb7db4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb7dc0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb7dcc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb7ddc | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xb7dec | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xb7df0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7df4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb7e04 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb7e0c | store | 4 | `sp -0x24` | `([sp-36] + ret(0xb8a40))` |
| `ldrb` at 0xb7e14 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xb7e18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7e1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb7e2c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb7e34 | store | 4 | `sp -0x24` | `([sp-36] + ret(0xba214))` |
| `ldrb` at 0xb7e3c | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xb7e40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7e44 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb7e54 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb7e5c | store | 4 | `sp -0x24` | `([sp-36] + ret(0xad294))` |
| `ldr` at 0xb7e64 | load | 4 | `abs 0xb7eb0` | `-` |
| `str` at 0xb7e68 | store | 4 | `sp -0x30` | `0x10924c` |
| `ldr` at 0xb7e6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7e70 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xb7e74 | store | 4 | `sp -0x2c` | `[[sp-24]+12]` |
| `ldr` at 0xb7e7c | load | 4 | `abs 0xb7eb4` | `-` |
| `ldrb` at 0xb7e8c | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0xb7e94 | store | 1 | `sp -0x1d` | `?` |
| `ldr` at 0xb7e9c | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xb7ea4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb7ea4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb7ea4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7dfc | `(unknown callee) 0xb8a40` | `[sp-24]` | `[sp-28]` | `?` | `?` |
| `bl` at 0xb7e24 | `(unknown callee) 0xba214` | `[sp-24]` | `[sp-28]` | `?` | `?` |
| `bl` at 0xb7e4c | `(unknown callee) 0xad294` | `[sp-24]` | `[sp-28]` | `?` | `?` |
| `bl` at 0xb7e88 | `(unknown callee) 0x1eed0` | `0x1` | `0x10916c` | `0x25c` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xb7d44 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0xb7d94 | `[[sp-24]+8] == 0x0` | `[[sp-24]+8] != 0x0` |
| `beq` at 0xb7da4 | `[[sp-28]]:1 == 0x0` | `[[sp-28]]:1 != 0x0` |
| `beq` at 0xb7dbc | `[[sp-24]]:1 == 0x2` | `[[sp-24]]:1 != 0x2` |
| `bgt` at 0xb7dc8 | `[[sp-24]]:1 > 0x2 (signed, 32-bit)` | `[[sp-24]]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0xb7dd4 | `[[sp-24]]:1 == 0x0` | `[[sp-24]]:1 != 0x0` |
| `beq` at 0xb7de4 | `[[sp-24]]:1 == 0x3` | `[[sp-24]]:1 != 0x3` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb7ea4 | `r0` | `?` |

## `ioExpPortInt`

- `ioExpPortInt`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7eb8 to 0xb8014.
  Established. Four byte saves retain masked entry r0 through r3 at sp -0x11 through sp -0x14
  (`strb` at 0xb7ed0, `strb` at 0xb7ed8, `strb` at 0xb7ee0, `strb` at 0xb7ee8). Three `bne` sites
  each take their Z == 0 path to the loop increment, bypassing the call to 0xb8014 (`bne` at
  0xb7f58, `bne` at 0xb7f6c, `bne` at 0xb7f80, `bl` at 0xb7f9c). That call has three unknown
  candidates followed by `(r3 & 0xff)` (`bl` at 0xb7f9c). A separate path calls 0x1eed0 with
  candidates 0x1, 0x10916c, 0x27c and 0x0 (`bl` at 0xb7fec). The port-match and interrupt-routing
  semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb7ed0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xb7ed8 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0xb7ee0 | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `strb` at 0xb7ee8 | store | 1 | `sp -0x14` | `(r3 & 0xff)` |
| `strb` at 0xb7ef0 | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0xb7ef4 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xb7ef8 | load | 4 | `abs 0xb8000` | `-` |
| `ldr` at 0xb7efc | load | 4 | `abs 0x2841a0` | `-` |
| `ldr` at 0xb7f0c | load | 4 | `abs 0xb8004` | `-` |
| `ldrb` at 0xb7f10 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xb7f18 | load | 4 | `abs 0x284180` | `-` |
| `str` at 0xb7f20 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xb7f24 | load | 4 | `abs 0xb8008` | `-` |
| `ldrb` at 0xb7f28 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xb7f3c | load | 4 | `abs 0x284160` | `-` |
| `str` at 0xb7f44 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xb7f48 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb7f4c | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xb7f50 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb7f5c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb7f60 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0xb7f64 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb7f70 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb7f74 | load | 1 | `sp -0x13` | `-` |
| `ldrb` at 0xb7f78 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xb7f84 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0xb7f88 | load | 1 | `sp -0x14` | `-` |
| `ldr` at 0xb7f8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7f90 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb7fa4 | store | 4 | `sp -0x24` | `ret(0xb8014)` |
| `ldrb` at 0xb7fac | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0xb7fb4 | store | 1 | `sp -0x1d` | `?` |
| `ldr` at 0xb7fbc | load | 4 | `abs 0xb800c` | `-` |
| `str` at 0xb7fc0 | store | 4 | `sp -0x34` | `0x109278` |
| `ldrb` at 0xb7fc4 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb7fc8 | store | 4 | `sp -0x30` | `(r0 & 0xff)` |
| `ldrb` at 0xb7fcc | load | 1 | `sp -0x12` | `-` |
| `str` at 0xb7fd0 | store | 4 | `sp -0x2c` | `(r1 & 0xff)` |
| `ldrb` at 0xb7fd4 | load | 1 | `sp -0x13` | `-` |
| `str` at 0xb7fd8 | store | 4 | `sp -0x28` | `(r2 & 0xff)` |
| `ldr` at 0xb7fe0 | load | 4 | `abs 0xb8010` | `-` |
| `str` at 0xb7ff4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb7ff8 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xb7ffc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb7ffc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb7ffc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7f9c | `(unknown callee) 0xb8014` | `?` | `?` | `?` | `(r3 & 0xff)` |
| `bl` at 0xb7fec | `(unknown callee) 0x1eed0` | `0x1` | `0x10916c` | `0x27c` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xb7f04 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xb7f58 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xb7f6c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xb7f80 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb7ffc | `r0` | `?` |

## `ioExpIntHandler`

- `ioExpIntHandler`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8014 to 0xb81fc.
  Established. A word at `[sp-24] +0x8` receives `[[sp-24]+8]+1` (`str` at 0xb80e8). The indirect
  call has candidates `([sp-25]:1, unknown, [[sp-24]+2]:1, [[sp-24]+2]:1)` (`mov` at 0xb8120), with
  an unknown target. The initial equality branch is taken when `[r0+8] == 0`, and falls through
  otherwise (`beq` at 0xb804c). The exit retains an unknown value (`ldmdb` at 0xb81dc). Interrupt
  ownership and handler effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb8024 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb8028 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xb8034 | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `strb` at 0xb803c | store | 1 | `sp -0x1a` | `(r3 & 0xff)` |
| `ldr` at 0xb8040 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8044 | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0xb8050 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8054 | load | 1 | `sp -0x1a` | `-` |
| `ldrb` at 0xb8058 | load | 1 | `r1 +0x3` | `-` |
| `ldr` at 0xb806c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8074 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb8084 | store | 4 | `sp -0x20` | `ret(0xb86d4)` |
| `ldr` at 0xb8088 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb8094 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb80a0 | load | 4 | `abs 0xb81e0` | `-` |
| `str` at 0xb80a4 | store | 4 | `sp -0x44` | `0x1092b0` |
| `ldr` at 0xb80a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb80ac | load | 4 | `[sp-20] +0xc` | `-` |
| `str` at 0xb80b0 | store | 4 | `sp -0x40` | `[[sp-20]+12]` |
| `str` at 0xb80b4 | store | 4 | `sp -0x3c` | `ret(0x1ea50)` |
| `ldr` at 0xb80bc | load | 4 | `abs 0xb81e4` | `-` |
| `ldr` at 0xb80c0 | load | 4 | `abs 0xb81e8` | `-` |
| `str` at 0xb80d0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb80d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb80dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb80e0 | load | 4 | `[sp-24] +0x8` | `-` |
| `str` at 0xb80e8 | store | 4 | `[sp-24] +0x8` | `[[sp-24]+8]+1` |
| `ldr` at 0xb80ec | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb80f0 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xb80f4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb80f8 | load | 1 | `sp -0x1a` | `-` |
| `ldrb` at 0xb80fc | load | 1 | `[sp-24] +0x3` | `-` |
| `ldr` at 0xb8108 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb810c | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xb8110 | load | 4 | `[sp-20] +0x8` | `-` |
| `str` at 0xb8128 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0xb8130 | load | 4 | `abs 0xb81ec` | `-` |
| `str` at 0xb8134 | store | 4 | `sp -0x44` | `0x1092d4` |
| `ldr` at 0xb8138 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb813c | load | 4 | `r0 +0xc` | `-` |
| `str` at 0xb8140 | store | 4 | `sp -0x40` | `[r0+12]` |
| `ldrb` at 0xb8144 | load | 1 | `sp -0x1a` | `-` |
| `str` at 0xb8148 | store | 4 | `sp -0x3c` | `(r3 & 0xff)` |
| `ldr` at 0xb814c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8150 | load | 1 | `r1 +0x3` | `-` |
| `str` at 0xb8154 | store | 4 | `sp -0x38` | `[r1+3]:1` |
| `ldr` at 0xb8158 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb815c | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0xb8168 | load | 4 | `abs 0xb81f0` | `-` |
| `str` at 0xb816c | store | 4 | `sp -0x28` | `0x109318` |
| `ldr` at 0xb8174 | load | 4 | `abs 0xb81f4` | `-` |
| `str` at 0xb8178 | store | 4 | `sp -0x28` | `0x10931c` |
| `ldr` at 0xb817c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb8180 | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldr` at 0xb8188 | load | 4 | `abs 0xb81e4` | `-` |
| `ldr` at 0xb818c | load | 4 | `abs 0xb81f8` | `-` |
| `ldrb` at 0xb8198 | load | 1 | `sp -0x19` | `-` |
| `str` at 0xb819c | store | 4 | `sp -0x2c` | `[sp-25]:1` |
| `ldr` at 0xb81a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb81a4 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldrb` at 0xb81b0 | load | 1 | `sp -0x1a` | `-` |
| `str` at 0xb81b4 | store | 4 | `sp -0x30` | `[sp-26]:1` |
| `str` at 0xb81c0 | store | 4 | `sp -0x30` | `0xff` |
| `ldr` at 0xb81c4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb81c8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xb81d4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb81d8 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xb81dc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb81dc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb81dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb807c | `(unknown callee) 0xb86d4` | `r0` | `r1+2` | `(r3 & 0xff)` | `r1+2` |
| `bl` at 0xb8098 | `(unknown callee) 0x1ea50` | `ret(0xb86d4)` | `?` | `?` | `ret(0xb86d4)` |
| `bl` at 0xb80c8 | `(unknown callee) 0x1eed0` | `0x3` | `0x10916c` | `0x28f` | `0x0` |
| `mov` at 0xb8120 | `(unknown callee) (unknown target)` | `[sp-25]:1` | `?` | `[[sp-24]+2]:1` | `[[sp-24]+2]:1` |
| `bl` at 0xb8194 | `(unknown callee) 0x1eed0` | `0x1` | `0x10916c` | `0x299` | `0x0` |
| `bl` at 0xb81cc | `(unknown callee) 0xb7c2c` | `[sp-25]:1` | `[sp-48]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb804c | `[r0+8] == 0x0` | `[r0+8] != 0x0` |
| `beq` at 0xb8068 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xb8090 | `ret(0xb86d4) == 0x0` | `ret(0xb86d4) != 0x0` |
| `beq` at 0xb8164 | `[r0+8] == 0x0` | `[r0+8] != 0x0` |
| `beq` at 0xb81ac | `[[sp-20]+8] == 0x0` | `[[sp-20]+8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb81dc | `r0` | `?` |

## `ioExpIgnoreInt`

- `ioExpIgnoreInt`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb81fc to 0xb8268.
  Established. The outgoing word stores at sp -0x20, sp -0x1c and sp -0x18 contain `(r0 & 0xff)`,
  `(r1 & 0xff)` and `(r2 & 0xff)` (`str` at 0xb8230, `str` at 0xb8238, `str` at 0xb8240). The call
  to 0x1eed0 uses candidates 0x6, 0x10916c, 0x2ab and 0x0 (`bl` at 0xb8254); its result reaches r0
  at return (`ldmdb` at 0xb8258). The interrupt-ignore role suggested by the name remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb8210 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xb8218 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0xb8220 | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `ldr` at 0xb8224 | load | 4 | `abs 0xb825c` | `-` |
| `str` at 0xb8228 | store | 4 | `sp -0x24` | `0x10932c` |
| `ldrb` at 0xb822c | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb8230 | store | 4 | `sp -0x20` | `(r0 & 0xff)` |
| `ldrb` at 0xb8234 | load | 1 | `sp -0x12` | `-` |
| `str` at 0xb8238 | store | 4 | `sp -0x1c` | `(r1 & 0xff)` |
| `ldrb` at 0xb823c | load | 1 | `sp -0x13` | `-` |
| `str` at 0xb8240 | store | 4 | `sp -0x18` | `(r2 & 0xff)` |
| `ldr` at 0xb8248 | load | 4 | `abs 0xb8260` | `-` |
| `ldr` at 0xb824c | load | 4 | `abs 0xb8264` | `-` |
| `ldmdb` at 0xb8258 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb8258 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb8258 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb8254 | `(unknown callee) 0x1eed0` | `0x6` | `0x10916c` | `0x2ab` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb8258 | `r0` | `ret(0x1eed0)` |

## `ioExpName`

- `ioExpName`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8268 to 0xb82c8.
  Established. The word read at `ret(0xb82c8) +0xc` is saved at sp -0x1c (`ldr` at 0xb8298, `str` at
  0xb829c). A second load from the same effective location decides `bne`, whose nonzero taken path
  skips the fallback store of 0x10936c (`ldr` at 0xb82a4, `bne` at 0xb82ac, `str` at 0xb82b4). The
  two loads need not agree. The pointed value's name-string meaning is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb827c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xb8280 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb8290 | store | 4 | `sp -0x18` | `ret(0xb82c8)` |
| `ldr` at 0xb8294 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8298 | load | 4 | `ret(0xb82c8) +0xc` | `-` |
| `str` at 0xb829c | store | 4 | `sp -0x1c` | `[ret(0xb82c8)+12]` |
| `ldr` at 0xb82a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb82a4 | load | 4 | `ret(0xb82c8) +0xc` | `-` |
| `ldr` at 0xb82b0 | load | 4 | `abs 0xb82c4` | `-` |
| `str` at 0xb82b4 | store | 4 | `sp -0x1c` | `0x10936c` |
| `ldr` at 0xb82b8 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xb82c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb82c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb82c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb8288 | `(unknown callee) 0xb82c8` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb82ac | `[ret(0xb82c8)+12] != 0x0` | `[ret(0xb82c8)+12] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb82c0 | `r0` | `?` |

## `ioExpGetCfgIndex`

- `ioExpGetCfgIndex`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb82c8 to 0xb8348.
  Established. The masked entry byte at sp -0x11 is compared with the word from 0x2841a0 (`strb` at
  0xb82dc, `ldr` at 0xb82e8, `cmp` at 0xb82ec). `blt` takes the signed `(r0 & 0xff) < [0x2841a0]`
  path past the call to 0x1eed0 (`blt` at 0xb82f0, `bl` at 0xb8314). The later word load from
  0x284180 precedes a return with unknown r0 value (`ldr` at 0xb8324, `ldmdb` at 0xb8330). The
  returned index or pointer interpretation remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb82dc | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xb82e0 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb82e4 | load | 4 | `abs 0xb8334` | `-` |
| `ldr` at 0xb82e8 | load | 4 | `abs 0x2841a0` | `-` |
| `ldr` at 0xb82f4 | load | 4 | `abs 0xb8338` | `-` |
| `str` at 0xb82f8 | store | 4 | `sp -0x1c` | `0x109374` |
| `ldrb` at 0xb82fc | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb8300 | store | 4 | `sp -0x18` | `(r0 & 0xff)` |
| `ldr` at 0xb8308 | load | 4 | `abs 0xb833c` | `-` |
| `ldr` at 0xb830c | load | 4 | `abs 0xb8340` | `-` |
| `ldr` at 0xb8318 | load | 4 | `abs 0xb8344` | `-` |
| `ldrb` at 0xb831c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb8324 | load | 4 | `abs 0x284180` | `-` |
| `ldmdb` at 0xb8330 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb8330 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb8330 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb8314 | `(unknown callee) 0x1eed0` | `0x9` | `0x10916c` | `0x2ca` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xb82f0 | `(r0 & 0xff) < [0x2841a0] (signed, 32-bit)` | `(r0 & 0xff) >= [0x2841a0] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb8330 | `r0` | `?` |

## `ioExpGetValIndex`

- `ioExpGetValIndex`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8348 to 0xb83d4.
  Established. `blt` takes the path past the call to 0x1eed0 for `(r0 & 0xff) < [0x2841a0]` signed
  (`blt` at 0xb8370, `bl` at 0xb8394). The later word load from 0x284160 is followed by an unknown
  r0 value at return (`ldr` at 0xb83b0, `ldmdb` at 0xb83bc). The call path's outgoing word at sp
  -0x18 retains `(r0 & 0xff)` (`str` at 0xb8380). The value-index interpretation is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb835c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xb8360 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb8364 | load | 4 | `abs 0xb83c0` | `-` |
| `ldr` at 0xb8368 | load | 4 | `abs 0x2841a0` | `-` |
| `ldr` at 0xb8374 | load | 4 | `abs 0xb83c4` | `-` |
| `str` at 0xb8378 | store | 4 | `sp -0x1c` | `0x109374` |
| `ldrb` at 0xb837c | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb8380 | store | 4 | `sp -0x18` | `(r0 & 0xff)` |
| `ldr` at 0xb8388 | load | 4 | `abs 0xb83c8` | `-` |
| `ldr` at 0xb838c | load | 4 | `abs 0xb83cc` | `-` |
| `ldr` at 0xb8398 | load | 4 | `abs 0xb83d0` | `-` |
| `ldrb` at 0xb839c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb83b0 | load | 4 | `abs 0x284160` | `-` |
| `ldmdb` at 0xb83bc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb83bc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb83bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb8394 | `(unknown callee) 0x1eed0` | `0x9` | `0x10916c` | `0x2d6` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xb8370 | `(r0 & 0xff) < [0x2841a0] (signed, 32-bit)` | `(r0 & 0xff) >= [0x2841a0] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb83bc | `r0` | `?` |

## `ioExpWriteExp`

- `ioExpWriteExp`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb83d4 to 0xb86d4.
  Established. Two byte stores at `[sp-24] +0x1` have unknown values before a computed transfer
  (`strb` at 0xb84b4, `strb` at 0xb84cc, `ldrls` at 0xb84dc). The `ldrls` taken condition is
  `[[sp-20]]:1 <= 0x3 (unsigned, 32-bit)`, with an unresolved taken edge (`ldrls` at 0xb84dc); its
  known fall-through reaches 0x1eed0 with candidates 0x1, 0x10916c, 0x31e and 0x0 (`bl` at 0xb86a4).
  That path saves 0x0 at sp -0x24 and returns r0 value 0x0 (`str` at 0xb86ac, `ldmdb` at 0xb86b4).
  The physical write protocol and additional dispatch effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb83e4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb83e8 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xb83f4 | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `strb` at 0xb83fc | store | 1 | `sp -0x1a` | `(r3 & 0xff)` |
| `ldr` at 0xb8400 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb8404 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0xb8408 | load | 1 | `r0 +0x4` | `-` |
| `ldr` at 0xb841c | load | 4 | `abs 0xb86b8` | `-` |
| `str` at 0xb8420 | store | 4 | `sp -0x34` | `0x109394` |
| `ldr` at 0xb8424 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8428 | load | 4 | `r0 +0xc` | `-` |
| `str` at 0xb842c | store | 4 | `sp -0x30` | `[r0+12]` |
| `ldrb` at 0xb8430 | load | 1 | `sp -0x19` | `-` |
| `str` at 0xb8434 | store | 4 | `sp -0x2c` | `(r2 & 0xff)` |
| `ldr` at 0xb843c | load | 4 | `abs 0xb86bc` | `-` |
| `ldr` at 0xb8440 | load | 4 | `abs 0xb86c0` | `-` |
| `ldrb` at 0xb844c | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0xb8454 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xb8464 | load | 4 | `abs 0xb86c4` | `-` |
| `str` at 0xb8468 | store | 4 | `sp -0x34` | `0x1093c8` |
| `ldr` at 0xb846c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8470 | load | 4 | `[sp-20] +0xc` | `-` |
| `str` at 0xb8474 | store | 4 | `sp -0x30` | `[[sp-20]+12]` |
| `ldrb` at 0xb8478 | load | 1 | `sp -0x19` | `-` |
| `str` at 0xb847c | store | 4 | `sp -0x2c` | `[sp-25]:1` |
| `ldrb` at 0xb8480 | load | 1 | `sp -0x1a` | `-` |
| `str` at 0xb8484 | store | 4 | `sp -0x28` | `[sp-26]:1` |
| `ldr` at 0xb848c | load | 4 | `abs 0xb86bc` | `-` |
| `ldr` at 0xb8490 | load | 4 | `abs 0xb86c8` | `-` |
| `ldr` at 0xb849c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb84a0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb84a4 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0xb84ac | load | 1 | `[sp-24] +0x1` | `-` |
| `strb` at 0xb84b4 | store | 1 | `[sp-24] +0x1` | `?` |
| `ldr` at 0xb84b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb84bc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb84c0 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldrb` at 0xb84c4 | load | 1 | `sp -0x1a` | `-` |
| `strb` at 0xb84cc | store | 1 | `[sp-24] +0x1` | `?` |
| `ldr` at 0xb84d0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb84d4 | load | 1 | `[sp-20] +0x0` | `-` |
| `ldrls` at 0xb84dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb8674 | load | 4 | `abs 0xb86cc` | `-` |
| `str` at 0xb8678 | store | 4 | `sp -0x34` | `0x1093fc` |
| `ldr` at 0xb867c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb8680 | load | 1 | `[sp-20] +0x0` | `-` |
| `str` at 0xb8684 | store | 4 | `sp -0x30` | `[[sp-20]]:1` |
| `ldr` at 0xb8688 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb868c | load | 4 | `[sp-20] +0xc` | `-` |
| `str` at 0xb8690 | store | 4 | `sp -0x2c` | `[[sp-20]+12]` |
| `ldr` at 0xb8698 | load | 4 | `abs 0xb86bc` | `-` |
| `ldr` at 0xb869c | load | 4 | `abs 0xb86d0` | `-` |
| `str` at 0xb86ac | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb86b0 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xb86b4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb86b4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb86b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb8448 | `(unknown callee) 0x1eed0` | `0x9` | `0x10916c` | `0x2ed` | `0x0` |
| `bl` at 0xb8498 | `(unknown callee) 0x1eed0` | `0x9` | `0x10916c` | `0x2f1` | `0x0` |
| `bl` at 0xb86a4 | `(unknown callee) 0x1eed0` | `0x1` | `0x10916c` | `0x31e` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb8418 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xb8460 | `? == 0x0` | `? != 0x0` |
| `ldrls` at 0xb84dc | `[[sp-20]]:1 <= 0x3 (unsigned, 32-bit)` | `[[sp-20]]:1 > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb86b4 | `r0` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xb84dc).

## `ioExpReadExp`

- `ioExpReadExp`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb86d4 to 0xb880c.
  Established. A byte load at entry r0 +0x0 decides the conditional computed transfer (`ldrb` at
  0xb86f0, `ldrls` at 0xb86f8). The `ldrls` taken condition is `[r0]:1 <= 0x3 (unsigned, 32-bit)`,
  and its taken edge is unresolved (`ldrls` at 0xb86f8). Known fall-through stores `[r0]:1` and
  `[r0+12]` in outgoing word slots at sp -0x24 and sp -0x20 (`str` at 0xb87cc, `str` at 0xb87d8). It
  then calls 0x1eed0 with candidates 0x1, 0x10916c, 0x33e and 0x0 (`bl` at 0xb87ec). The known
  return has r0 value 0x0, while additional transfer effects and the read protocol remain unknown
  (`ldmdb` at 0xb87fc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb86e4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb86e8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xb86ec | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb86f0 | load | 1 | `r0 +0x0` | `-` |
| `ldrls` at 0xb86f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb87bc | load | 4 | `abs 0xb8800` | `-` |
| `str` at 0xb87c0 | store | 4 | `sp -0x28` | `0x1093fc` |
| `ldr` at 0xb87c4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb87c8 | load | 1 | `r0 +0x0` | `-` |
| `str` at 0xb87cc | store | 4 | `sp -0x24` | `[r0]:1` |
| `ldr` at 0xb87d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb87d4 | load | 4 | `r0 +0xc` | `-` |
| `str` at 0xb87d8 | store | 4 | `sp -0x20` | `[r0+12]` |
| `ldr` at 0xb87e0 | load | 4 | `abs 0xb8804` | `-` |
| `ldr` at 0xb87e4 | load | 4 | `abs 0xb8808` | `-` |
| `str` at 0xb87f4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xb87f8 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xb87fc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb87fc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb87fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb87ec | `(unknown callee) 0x1eed0` | `0x1` | `0x10916c` | `0x33e` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xb86f8 | `[r0]:1 <= 0x3 (unsigned, 32-bit)` | `[r0]:1 > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb87fc | `r0` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xb86f8).
