# ARM byte flags and scan routines in 1.34.2

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
No unresolved computed transfers were reported for these routines. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/vrm-scan-status-facts.txt) retain all tool sections.

## `vrmInit`

- `vrmInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2daac to 0x2db8c.
  Established. Absolute word stores put `ret(0x18920)` at `0x119cd0`, `[sp-24]` at `0x119ccc`, and
  `[sp-28]` at `0x119cd4` (`str` at 0x2db18, `str` at 0x2db60, `str` at 0x2db6c). The call to
  `0x18c20` carries `[0x119cd0]`, `0x1`, `[sp-28]`, `0x119cd0` (`bl` at 0x2db54); its result reaches
  the return (`ldmdb` at 0x2db70). The three slots’ relationship to physical regulators is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2dabc | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x2dac0 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x2dac4 | load | 4 | `abs 0x2db74` | `-` |
| `ldr` at 0x2dac8 | load | 4 | `abs 0x119ccc` | `-` |
| `ldr` at 0x2dad4 | load | 4 | `abs 0x2db78` | `-` |
| `str` at 0x2dad8 | store | 4 | `sp -0x28` | `0xdbc64` |
| `ldr` at 0x2dadc | load | 4 | `abs 0x2db74` | `-` |
| `ldr` at 0x2dae0 | load | 4 | `abs 0x119ccc` | `-` |
| `str` at 0x2dae4 | store | 4 | `sp -0x24` | `[0x119ccc]` |
| `ldr` at 0x2dae8 | load | 4 | `abs 0x2db7c` | `-` |
| `ldr` at 0x2daec | load | 4 | `abs 0x119cd4` | `-` |
| `str` at 0x2daf0 | store | 4 | `sp -0x20` | `[0x119cd4]` |
| `ldr` at 0x2daf8 | load | 4 | `abs 0x2db80` | `-` |
| `ldr` at 0x2db08 | load | 4 | `abs 0x2db84` | `-` |
| `ldr` at 0x2db0c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2db18 | store | 4 | `abs 0x119cd0` | `ret(0x18920)` |
| `ldr` at 0x2db1c | load | 4 | `abs 0x119cd0` | `-` |
| `ldr` at 0x2db28 | load | 4 | `abs 0x2db88` | `-` |
| `str` at 0x2db2c | store | 4 | `sp -0x28` | `0xdbca0` |
| `ldr` at 0x2db34 | load | 4 | `abs 0x2db80` | `-` |
| `ldr` at 0x2db44 | load | 4 | `abs 0x2db84` | `-` |
| `ldr` at 0x2db48 | load | 4 | `abs 0x119cd0` | `-` |
| `ldr` at 0x2db50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2db58 | load | 4 | `abs 0x2db74` | `-` |
| `ldr` at 0x2db5c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2db60 | store | 4 | `abs 0x119ccc` | `[sp-24]` |
| `ldr` at 0x2db64 | load | 4 | `abs 0x2db7c` | `-` |
| `ldr` at 0x2db68 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2db6c | store | 4 | `abs 0x119cd4` | `[sp-28]` |
| `ldmdb` at 0x2db70 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x2db70 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2db70 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2db70 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2db04 | `(unknown callee) 0x1eed0` | `0x9` | `0xdbc8c` | `0x2e` | `0x0` |
| `bl` at 0x2db10 | `(unknown callee) 0x18920` | `?` | `?` | `?` | `?` |
| `bl` at 0x2db40 | `(unknown callee) 0x1eed0` | `0x9` | `0xdbc8c` | `0x36` | `0x0` |
| `bl` at 0x2db54 | `(unknown callee) 0x18c20` | `[0x119cd0]` | `0x1` | `[sp-28]` | `0x119cd0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x2dad0 | `[0x119ccc] == 0x0` | `[0x119ccc] != 0x0` |
| `bne` at 0x2db24 | `[0x119cd0] != 0x0` | `[0x119cd0] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2db70 | `r0` | `ret(0x18c20)` |

## `vrmDisable`

- `vrmDisable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2db8c to 0x2dc0c.
  Established. The deciding signed comparison takes its edge for `(r0 & 0xff) >= [0x119cd4]` and
  falls through for a smaller value (`bge` at 0x2dbb4). The fall-through path stores byte `0x0` at
  `((r0 & 0xff) + [0x119cd0]) +0x0` (`strb` at 0x2dbcc). The other path calls `0x1eed0` with `0x6`,
  `0xdbc8c`, `0x4b`, `0x0` (`bl` at 0x2dbf4). A physical disable operation and the merged return
  remain unknown (`ldmdb` at 0x2dbf8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x2dba0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x2dba4 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x2dba8 | load | 4 | `abs 0x2dbfc` | `-` |
| `ldr` at 0x2dbac | load | 4 | `abs 0x119cd4` | `-` |
| `ldrb` at 0x2dbb8 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x2dbbc | load | 4 | `abs 0x2dc00` | `-` |
| `ldr` at 0x2dbc0 | load | 4 | `abs 0x119cd0` | `-` |
| `strb` at 0x2dbcc | store | 1 | `((r0 & 0xff) + [0x119cd0]) +0x0` | `0x0` |
| `ldr` at 0x2dbd4 | load | 4 | `abs 0x2dc04` | `-` |
| `str` at 0x2dbd8 | store | 4 | `sp -0x1c` | `0xdbcbc` |
| `ldrb` at 0x2dbdc | load | 1 | `sp -0x11` | `-` |
| `str` at 0x2dbe0 | store | 4 | `sp -0x18` | `(r0 & 0xff)` |
| `ldr` at 0x2dbe8 | load | 4 | `abs 0x2dc08` | `-` |
| `ldmdb` at 0x2dbf8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2dbf8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2dbf8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2dbf4 | `(unknown callee) 0x1eed0` | `0x6` | `0xdbc8c` | `0x4b` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x2dbb4 | `(r0 & 0xff) >= [0x119cd4] (signed, 32-bit)` | `(r0 & 0xff) < [0x119cd4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2dbf8 | `r0` | `?` |

## `vrmEnable`

- `vrmEnable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2dc0c to 0x2dc8c.
  Established. The indexed byte store writes `0x1` at `((r0 & 0xff) + [0x119cd0]) +0x0` (`strb` at
  0x2dc4c). It lies on the signed-less-than fall-through of the test against `[0x119cd4]`, while
  greater-or-equal takes the other edge (`bge` at 0x2dc34). The only call targets `0x1eed0` with
  candidates `0x6`, `0xdbc8c`, `0x57`, `0x0` (`bl` at 0x2dc74). The byte’s physical enable meaning
  is unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x2dc20 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x2dc24 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x2dc28 | load | 4 | `abs 0x2dc7c` | `-` |
| `ldr` at 0x2dc2c | load | 4 | `abs 0x119cd4` | `-` |
| `ldrb` at 0x2dc38 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x2dc3c | load | 4 | `abs 0x2dc80` | `-` |
| `ldr` at 0x2dc40 | load | 4 | `abs 0x119cd0` | `-` |
| `strb` at 0x2dc4c | store | 1 | `((r0 & 0xff) + [0x119cd0]) +0x0` | `0x1` |
| `ldr` at 0x2dc54 | load | 4 | `abs 0x2dc84` | `-` |
| `str` at 0x2dc58 | store | 4 | `sp -0x1c` | `0xdbcd4` |
| `ldrb` at 0x2dc5c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x2dc60 | store | 4 | `sp -0x18` | `(r0 & 0xff)` |
| `ldr` at 0x2dc68 | load | 4 | `abs 0x2dc88` | `-` |
| `ldmdb` at 0x2dc78 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2dc78 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2dc78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2dc74 | `(unknown callee) 0x1eed0` | `0x6` | `0xdbc8c` | `0x57` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x2dc34 | `(r0 & 0xff) >= [0x119cd4] (signed, 32-bit)` | `(r0 & 0xff) < [0x119cd4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2dc78 | `r0` | `?` |

## `vrmGetCfg`

- `vrmGetCfg`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2dcb8 to 0x2dcf4.
  Established. A word store copies `[0x119ccc]` to entry `r0 +0x0` (`str` at 0x2dcd8). A separate
  word load reads absolute `0x119cd4`, whose value reaches the r0 return (`ldr` at 0x2dce0, `ldmdb`
  at 0x2dce8). No calls or conditional branches are reported. The two values’ units
  and layout are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2dcc8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2dccc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2dcd0 | load | 4 | `abs 0x2dcec` | `-` |
| `ldr` at 0x2dcd4 | load | 4 | `abs 0x119ccc` | `-` |
| `str` at 0x2dcd8 | store | 4 | `r0 +0x0` | `[0x119ccc]` |
| `ldr` at 0x2dcdc | load | 4 | `abs 0x2dcf0` | `-` |
| `ldr` at 0x2dce0 | load | 4 | `abs 0x119cd4` | `-` |
| `ldmdb` at 0x2dce8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2dce8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2dce8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2dce8 | `r0` | `[0x119cd4]` |

## `vrmIntEnable`

- `vrmIntEnable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2dcf4 to 0x2de10.
  Established. The first explicit store writes byte `0x1` at absolute `0x119cd8` (`strb` at
  0x2dd0c). The call to `0xb7c74` has four unknown candidates (`bl` at 0x2dd88). A zero result takes
  the bypass while nonzero falls through to calls to `0xb8268` and `0x1ea50` (`beq` at 0x2dd9c,
  `bl` at 0x2ddac, `bl` at 0x2ddb8). The signed loop branch retains unknown operands, following its
  edge for its less-than flags predicate and falling through otherwise (`blt` at 0x2dd28). An
  interrupt-enable effect is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x2dd04 | load | 4 | `abs 0x2ddf8` | `-` |
| `strb` at 0x2dd0c | store | 1 | `abs 0x119cd8` | `0x1` |
| `str` at 0x2dd14 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x2dd18 | load | 4 | `abs 0x2ddfc` | `-` |
| `ldr` at 0x2dd1c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2dd20 | load | 4 | `abs 0x119cd4` | `-` |
| `ldr` at 0x2dd30 | load | 4 | `abs 0x2de00` | `-` |
| `ldr` at 0x2dd34 | load | 4 | `abs 0x119cd0` | `-` |
| `ldr` at 0x2dd38 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x2dd40 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x2dd4c | load | 4 | `abs 0x2de04` | `-` |
| `ldr` at 0x2dd50 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2dd58 | load | 4 | `abs 0x119ccc` | `-` |
| `str` at 0x2dd60 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x2dd64 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2dd68 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x2dd6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2dd70 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2dd74 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x2dd78 | load | 1 | `(unknown)` | `-` |
| `str` at 0x2dd90 | store | 4 | `sp -0x1c` | `ret(0xb7c74)` |
| `ldr` at 0x2dd94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2dda0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2dda4 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x2ddb4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2ddc0 | load | 4 | `abs 0x2de08` | `-` |
| `str` at 0x2ddc4 | store | 4 | `sp -0x2c` | `0xdbcec` |
| `str` at 0x2ddc8 | store | 4 | `sp -0x28` | `ret(0xb8268)` |
| `str` at 0x2ddcc | store | 4 | `sp -0x24` | `ret(0x1ea50)` |
| `ldr` at 0x2ddd4 | load | 4 | `abs 0x2de0c` | `-` |
| `ldr` at 0x2dde4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x2ddec | store | 4 | `sp -0x20` | `?` |
| `ldmdb` at 0x2ddf4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x2ddf4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2ddf4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2ddf4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2dd88 | `(unknown callee) 0xb7c74` | `?` | `?` | `?` | `?` |
| `bl` at 0x2ddac | `(unknown callee) 0xb8268` | `[[sp-24]]:1` | `?` | `?` | `[[sp-24]]:1` |
| `bl` at 0x2ddb8 | `(unknown callee) 0x1ea50` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x2dde0 | `(unknown callee) 0x1eed0` | `0x3` | `0xdbc8c` | `0x8f` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x2dd28 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x2dd48 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x2dd9c | `ret(0xb7c74) == 0x0` | `ret(0xb7c74) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2ddf4 | `r0` | `?` |

## `vrmIntDisable`

- `vrmIntDisable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2de10 to 0x2df68.
  Established. An absolute byte store clears `0x119cd8` (`strb` at 0x2de28). Calls include `0xb78b4`
  with unknown, `0x119ccc`, `[0x119cd0]`, unknown and `0xb7c2c` with four unknown candidates (`bl`
  at 0x2dea8, `bl` at 0x2dee0). A zero result from the latter takes the bypass, while nonzero falls
  through to later calls (`beq` at 0x2def4). Stack word `sp -0x1c` retains `ret(0xb7c2c)` (`str`
  at 0x2dee8). Interrupt masking at a device remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x2de20 | load | 4 | `abs 0x2df50` | `-` |
| `strb` at 0x2de28 | store | 1 | `abs 0x119cd8` | `0x0` |
| `str` at 0x2de30 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x2de34 | load | 4 | `abs 0x2df54` | `-` |
| `ldr` at 0x2de38 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2de3c | load | 4 | `abs 0x119cd4` | `-` |
| `ldr` at 0x2de4c | load | 4 | `abs 0x2df58` | `-` |
| `ldr` at 0x2de50 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2de58 | load | 4 | `abs 0x119ccc` | `-` |
| `str` at 0x2de60 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x2de64 | load | 4 | `abs 0x2df5c` | `-` |
| `ldr` at 0x2de68 | load | 4 | `abs 0x119cd0` | `-` |
| `ldr` at 0x2de6c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x2de74 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x2de80 | load | 4 | `abs 0x2df5c` | `-` |
| `ldr` at 0x2de84 | load | 4 | `abs 0x119cd0` | `-` |
| `ldr` at 0x2de88 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x2de90 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x2de9c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2dea0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x2debc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2dec0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x2dec4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2dec8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2decc | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x2ded0 | load | 1 | `(unknown)` | `-` |
| `str` at 0x2dee8 | store | 4 | `sp -0x1c` | `ret(0xb7c2c)` |
| `ldr` at 0x2deec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2def8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2defc | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x2df0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2df18 | load | 4 | `abs 0x2df60` | `-` |
| `str` at 0x2df1c | store | 4 | `sp -0x2c` | `0xdbd24` |
| `str` at 0x2df20 | store | 4 | `sp -0x28` | `ret(0xb8268)` |
| `str` at 0x2df24 | store | 4 | `sp -0x24` | `ret(0x1ea50)` |
| `ldr` at 0x2df2c | load | 4 | `abs 0x2df64` | `-` |
| `ldr` at 0x2df3c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x2df44 | store | 4 | `sp -0x20` | `?` |
| `ldmdb` at 0x2df4c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x2df4c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2df4c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2df4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2dea8 | `(unknown callee) 0xb78b4` | `?` | `0x119ccc` | `[0x119cd0]` | `?` |
| `bl` at 0x2dee0 | `(unknown callee) 0xb7c2c` | `?` | `?` | `?` | `?` |
| `bl` at 0x2df04 | `(unknown callee) 0xb8268` | `[[sp-24]]:1` | `?` | `?` | `[[sp-24]]:1` |
| `bl` at 0x2df10 | `(unknown callee) 0x1ea50` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x2df38 | `(unknown callee) 0x1eed0` | `0x3` | `0xdbc8c` | `0xa6` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x2de44 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x2de7c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x2de98 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x2deb4 | `ret(0xb78b4) != 0x0` | `ret(0xb78b4) == 0x0` |
| `beq` at 0x2def4 | `ret(0xb7c2c) == 0x0` | `ret(0xb7c2c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2df4c | `r0` | `?` |

## `vrmEnabled`

- `vrmEnabled`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2df68 to 0x2df88.
  Established. A byte load reads absolute `0x119cd8` (`ldrb` at 0x2df78). The body moves that value
  to r0 and returns `[0x119cd8]:1` (`mov` at 0x2df7c, `ldmdb` at 0x2df80). No calls, explicit stores
  or conditional branches are reported. The returned byte’s physical status meaning is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x2df74 | load | 4 | `abs 0x2df84` | `-` |
| `ldrb` at 0x2df78 | load | 1 | `abs 0x119cd8` | `-` |
| `ldmdb` at 0x2df80 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2df80 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2df80 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2df80 | `r0` | `[0x119cd8]:1` |

## `vrmPresent`

- `vrmPresent`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2e090 to 0x2e174.
  Established. The initial signed comparison takes its edge for `(r0 & 0xff) >= [0x119cd4]` and
  falls through below it (`bge` at 0x2e0b8). A later call to `0xb7af8` uses `[[sp-24]]:1`, `sp-18`,
  `sp-18`, `[[sp-24]]:1` (`bl` at 0x2e120); its zero result takes the continuation and nonzero falls
  through to a stack word store of unknown value (`beq` at 0x2e12c, `str` at 0x2e134). The call
  to `0x2e00c` carries `[sp-24]`, `[sp-18]:1`, unknown, `[sp-18]:1` (`bl` at 0x2e148). Presence-bit
  semantics are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x2e0a4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x2e0a8 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x2e0ac | load | 4 | `abs 0x2e168` | `-` |
| `ldr` at 0x2e0b0 | load | 4 | `abs 0x119cd4` | `-` |
| `ldrb` at 0x2e0bc | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x2e0c0 | load | 4 | `abs 0x2e16c` | `-` |
| `ldr` at 0x2e0c4 | load | 4 | `abs 0x119cd0` | `-` |
| `ldrb` at 0x2e0cc | load | 1 | `((r0 & 0xff) + [0x119cd0]) +0x0` | `-` |
| `ldr` at 0x2e0d8 | load | 4 | `abs 0x2e170` | `-` |
| `ldrb` at 0x2e0dc | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x2e0e4 | load | 4 | `abs 0x119ccc` | `-` |
| `str` at 0x2e0ec | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x2e0f0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2e0f4 | load | 1 | `[sp-24] +0x3` | `-` |
| `str` at 0x2e104 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x2e10c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2e110 | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0x2e134 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x2e13c | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0x2e140 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2e150 | store | 4 | `sp -0x1c` | `ret(0x2e00c)` |
| `str` at 0x2e15c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x2e160 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x2e164 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2e164 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2e164 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2e120 | `(unknown callee) 0xb7af8` | `[[sp-24]]:1` | `sp-18` | `sp-18` | `[[sp-24]]:1` |
| `bl` at 0x2e148 | `(unknown callee) 0x2e00c` | `[sp-24]` | `[sp-18]:1` | `?` | `[sp-18]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x2e0b8 | `(r0 & 0xff) >= [0x119cd4] (signed, 32-bit)` | `(r0 & 0xff) < [0x119cd4] (signed, 32-bit)` |
| `beq` at 0x2e0d4 | `[((r0 & 0xff) + [0x119cd0])]:1 == 0x0` | `[((r0 & 0xff) + [0x119cd0])]:1 != 0x0` |
| `bne` at 0x2e0fc | `[[sp-24]+3]:1 != 0x0` | `[[sp-24]+3]:1 == 0x0` |
| `beq` at 0x2e12c | `ret(0xb7af8) == 0x0` | `ret(0xb7af8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2e164 | `r0` | `[sp-28]` |

## `vrmOK`

- `vrmOK`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2e174 to 0x2e328.
  Established. The call to `0x2e090` has candidates `(r0 & 0xff)`, entry r1, `(r0 & 0xff)`, `(r0 &
  0xff)` (`bl` at 0x2e1c4); its nonzero result takes the continuation and zero falls through to
  word `0x0` at `sp -0x1c` (`bne` at 0x2e1d0, `str` at 0x2e1d8). A later call to `0xd6930` uses
  `ret(0x3d538)`, `0x42400000`, unknown, `ret(0x3d538)` (`bl` at 0x2e244). The two stores selecting
  `0x0` and `0x1` at `sp -0x1c` occur on separate later paths (`str` at 0x2e2d8, `str` at 0x2e2e4).
  The returned status’s device interpretation remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x2e188 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x2e18c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x2e190 | load | 4 | `abs 0x2e318` | `-` |
| `ldr` at 0x2e194 | load | 4 | `abs 0x119cd4` | `-` |
| `ldrb` at 0x2e1a0 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x2e1a4 | load | 4 | `abs 0x2e31c` | `-` |
| `ldr` at 0x2e1a8 | load | 4 | `abs 0x119cd0` | `-` |
| `ldrb` at 0x2e1b0 | load | 1 | `((r0 & 0xff) + [0x119cd0]) +0x0` | `-` |
| `ldrb` at 0x2e1bc | load | 1 | `sp -0x11` | `-` |
| `str` at 0x2e1d8 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x2e1e0 | load | 4 | `abs 0x2e320` | `-` |
| `ldrb` at 0x2e1e4 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x2e1ec | load | 4 | `abs 0x119ccc` | `-` |
| `str` at 0x2e1f4 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x2e1f8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2e1fc | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0x2e220 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x2e228 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2e22c | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0x2e240 | load | 4 | `abs 0x2e324` | `-` |
| `ldrb` at 0x2e258 | load | 1 | `sp -0x12` | `-` |
| `strb` at 0x2e260 | store | 1 | `sp -0x12` | `?` |
| `ldr` at 0x2e264 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2e268 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0x2e278 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2e27c | load | 1 | `[sp-24] +0x4` | `-` |
| `ldrb` at 0x2e288 | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0x2e28c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2e2a8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2e2ac | load | 1 | `[sp-24] +0x3` | `-` |
| `ldrb` at 0x2e2b8 | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0x2e2bc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2e2d8 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x2e2e4 | store | 4 | `sp -0x1c` | `0x1` |
| `ldrb` at 0x2e2ec | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0x2e2f0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2e300 | store | 4 | `sp -0x1c` | `ret(0x2df88)` |
| `str` at 0x2e30c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x2e310 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x2e314 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2e314 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2e314 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2e1c4 | `(unknown callee) 0x2e090` | `(r0 & 0xff)` | `r1` | `(r0 & 0xff)` | `(r0 & 0xff)` |
| `bl` at 0x2e20c | `(unknown callee) 0xb7af8` | `[[sp-24]]:1` | `sp-18` | `sp-18` | `[[sp-24]]:1` |
| `bl` at 0x2e234 | `(unknown callee) 0x3d538` | `[[sp-24]+1]:1` | `?` | `?` | `[[sp-24]+1]:1` |
| `bl` at 0x2e244 | `(unknown callee) 0xd6930` | `ret(0x3d538)` | `0x42400000` | `?` | `ret(0x3d538)` |
| `bl` at 0x2e294 | `(unknown callee) 0x2df88` | `[sp-24]` | `[sp-18]:1` | `?` | `[sp-18]:1` |
| `bl` at 0x2e2c4 | `(unknown callee) 0x2e00c` | `[sp-24]` | `[sp-18]:1` | `?` | `[sp-18]:1` |
| `bl` at 0x2e2f8 | `(unknown callee) 0x2df88` | `[sp-24]` | `[sp-18]:1` | `?` | `[sp-18]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x2e19c | `(r0 & 0xff) >= [0x119cd4] (signed, 32-bit)` | `(r0 & 0xff) < [0x119cd4] (signed, 32-bit)` |
| `beq` at 0x2e1b8 | `[((r0 & 0xff) + [0x119cd0])]:1 == 0x0` | `[((r0 & 0xff) + [0x119cd0])]:1 != 0x0` |
| `bne` at 0x2e1d0 | `ret(0x2e090) != 0x0` | `ret(0x2e090) == 0x0` |
| `beq` at 0x2e218 | `ret(0xb7af8) == 0x0` | `ret(0xb7af8) != 0x0` |
| `beq` at 0x2e250 | `ret(0xd6930) == 0x0` | `ret(0xd6930) != 0x0` |
| `beq` at 0x2e274 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x2e284 | `[[sp-24]+4]:1 == 0x0` | `[[sp-24]+4]:1 != 0x0` |
| `bne` at 0x2e2a0 | `ret(0x2df88) != 0x0` | `ret(0x2df88) == 0x0` |
| `beq` at 0x2e2b4 | `[[sp-24]+3]:1 == 0x0` | `[[sp-24]+3]:1 != 0x0` |
| `bne` at 0x2e2d0 | `ret(0x2e00c) != 0x0` | `ret(0x2e00c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2e314 | `r0` | `[sp-28]` |

## `vrmAllPresent`

- `vrmAllPresent`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2e328 to 0x2e40c.
  Established. Stack word `sp -0x14` starts at `0x1` and can later receive `0x0` (`str` at 0x2e33c,
  `str` at 0x2e3dc). The call to `0x2e090` has candidates `[sp-24]:1`, unknown, unknown, `[sp-24]:1`
  (`bl` at 0x2e368); its nonzero result takes the next-iteration path and zero falls through to
  the later calls (`bne` at 0x2e374). The final call to `0x1eed0` carries `0x1`, `0xdbc8c`, `0x110`,
  `0x0` (`bl` at 0x2e3d4). The aggregate status meaning is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2e33c | store | 4 | `sp -0x14` | `0x1` |
| `str` at 0x2e344 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x2e348 | load | 4 | `abs 0x2e3fc` | `-` |
| `ldr` at 0x2e34c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2e350 | load | 4 | `abs 0x119cd4` | `-` |
| `ldrb` at 0x2e360 | load | 1 | `sp -0x18` | `-` |
| `ldr` at 0x2e378 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2e380 | load | 4 | `abs 0x2e400` | `-` |
| `ldr` at 0x2e384 | load | 4 | `abs 0x119ccc` | `-` |
| `ldrb` at 0x2e38c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x2e39c | load | 4 | `abs 0x2e404` | `-` |
| `str` at 0x2e3a0 | store | 4 | `sp -0x24` | `0xdbd5c` |
| `str` at 0x2e3a4 | store | 4 | `sp -0x20` | `ret(0x3d108)` |
| `ldr` at 0x2e3a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2e3b0 | load | 4 | `abs 0x2e400` | `-` |
| `ldr` at 0x2e3b4 | load | 4 | `abs 0x119ccc` | `-` |
| `ldrb` at 0x2e3bc | load | 1 | `(unknown)` | `-` |
| `str` at 0x2e3c0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x2e3c8 | load | 4 | `abs 0x2e408` | `-` |
| `str` at 0x2e3dc | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x2e3e0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2e3e8 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x2e3f0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x2e3f8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2e3f8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2e3f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2e368 | `(unknown callee) 0x2e090` | `[sp-24]:1` | `?` | `?` | `[sp-24]:1` |
| `bl` at 0x2e394 | `(unknown callee) 0x3d108` | `?` | `?` | `?` | `?` |
| `bl` at 0x2e3d4 | `(unknown callee) 0x1eed0` | `0x1` | `0xdbc8c` | `0x110` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x2e358 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x2e374 | `ret(0x2e090) != 0x0` | `ret(0x2e090) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2e3f8 | `r0` | `?` |

## `vrmAllOK`

- `vrmAllOK`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2e40c to 0x2e538.
  Established. The call to `0x2e174` carries `[sp-24]:1`, unknown, unknown, `[sp-24]:1` (`bl` at
  0x2e44c). A nonzero result takes the next-iteration edge and zero falls through (`bne` at
  0x2e458). The later call to `0xd6930` uses `ret(0x3d538)`, `0x42400000`, unknown, `ret(0x3d538)`
  (`bl` at 0x2e488); its nonzero result takes the edge to the remaining calls, while zero falls
  through to the iteration update (`bne` at 0x2e494). The result slot can receive word `0x0` at
  `sp -0x14`, but the aggregate device interpretation is unproven (`str` at 0x2e500).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2e420 | store | 4 | `sp -0x14` | `0x1` |
| `str` at 0x2e428 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x2e42c | load | 4 | `abs 0x2e520` | `-` |
| `ldr` at 0x2e430 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2e434 | load | 4 | `abs 0x119cd4` | `-` |
| `ldrb` at 0x2e444 | load | 1 | `sp -0x18` | `-` |
| `ldr` at 0x2e45c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2e464 | load | 4 | `abs 0x2e524` | `-` |
| `ldr` at 0x2e468 | load | 4 | `abs 0x119ccc` | `-` |
| `ldrb` at 0x2e470 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x2e484 | load | 4 | `abs 0x2e528` | `-` |
| `ldr` at 0x2e49c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2e4a4 | load | 4 | `abs 0x2e524` | `-` |
| `ldr` at 0x2e4a8 | load | 4 | `abs 0x119ccc` | `-` |
| `ldrb` at 0x2e4b0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x2e4c0 | load | 4 | `abs 0x2e52c` | `-` |
| `str` at 0x2e4c4 | store | 4 | `sp -0x28` | `0xdbd80` |
| `str` at 0x2e4c8 | store | 4 | `sp -0x24` | `ret(0x3d108)` |
| `ldr` at 0x2e4cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2e4d4 | load | 4 | `abs 0x2e524` | `-` |
| `ldr` at 0x2e4d8 | load | 4 | `abs 0x119ccc` | `-` |
| `ldrb` at 0x2e4e0 | load | 1 | `(unknown)` | `-` |
| `str` at 0x2e4e4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x2e4ec | load | 4 | `abs 0x2e530` | `-` |
| `ldr` at 0x2e4f0 | load | 4 | `abs 0x2e534` | `-` |
| `str` at 0x2e500 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x2e504 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2e50c | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x2e514 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x2e51c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2e51c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2e51c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2e44c | `(unknown callee) 0x2e174` | `[sp-24]:1` | `?` | `?` | `[sp-24]:1` |
| `bl` at 0x2e478 | `(unknown callee) 0x3d538` | `?` | `?` | `?` | `?` |
| `bl` at 0x2e488 | `(unknown callee) 0xd6930` | `ret(0x3d538)` | `0x42400000` | `?` | `ret(0x3d538)` |
| `bl` at 0x2e4b8 | `(unknown callee) 0x3d108` | `?` | `?` | `?` | `?` |
| `bl` at 0x2e4f8 | `(unknown callee) 0x1eed0` | `0x1` | `0xdbc8c` | `0x12a` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x2e43c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x2e458 | `ret(0x2e174) != 0x0` | `ret(0x2e174) == 0x0` |
| `bne` at 0x2e494 | `ret(0xd6930) != 0x0` | `ret(0xd6930) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2e51c | `r0` | `?` |

## `vrmInt`

- `vrmInt`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2e538 to 0x2e858.
  Established. The entry r0, r1 and r2 bytes are saved as `(r0 & 0xff)`, `(r1 & 0xff)`, `(r2 &
  0xff)` at `sp -0x11`, `sp -0x12`, `sp -0x13` (`strb` at 0x2e54c, `strb` at 0x2e554, `strb` at
  0x2e55c). Calls to `0x636f8` include `0x0`, `0x3402`, `0x1a3`, `sp-88` and `0x0`, `0x3401`,
  `0x1a2`, `sp-88` on separate paths (`bl` at 0x2e63c, `bl` at 0x2e698). Another uses `0x0`,
  `0x3500`, `0x1a4`, `0xdbdc8` (`bl` at 0x2e728). Preceding stack word stores provide `0x1` and
  `0x10` at `sp -0x64` and `sp -0x60` (`str` at 0x2e70c, `str` at 0x2e714). The call purposes and
  interrupt response are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x2e54c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x2e554 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0x2e55c | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `str` at 0x2e564 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0x2e568 | load | 4 | `abs 0x2e828` | `-` |
| `ldr` at 0x2e56c | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x2e570 | load | 4 | `abs 0x119cd4` | `-` |
| `ldr` at 0x2e580 | load | 4 | `abs 0x2e82c` | `-` |
| `ldr` at 0x2e584 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x2e58c | load | 4 | `abs 0x119ccc` | `-` |
| `str` at 0x2e594 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x2e598 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2e59c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x2e5a0 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x2e5ac | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0x2e5b0 | load | 1 | `sp -0x13` | `-` |
| `ldr` at 0x2e5b8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2e5c0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x2e5d4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2e5d8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x2e5e8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2e5ec | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x2e600 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2e604 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0x2e60c | load | 4 | `abs 0x2e830` | `-` |
| `str` at 0x2e620 | store | 4 | `sp -0x64` | `0x1` |
| `str` at 0x2e628 | store | 4 | `sp -0x60` | `0x10` |
| `ldr` at 0x2e630 | load | 4 | `abs 0x2e834` | `-` |
| `ldr` at 0x2e634 | load | 4 | `abs 0x2e838` | `-` |
| `ldr` at 0x2e644 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2e648 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x2e65c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2e660 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0x2e668 | load | 4 | `abs 0x2e83c` | `-` |
| `str` at 0x2e67c | store | 4 | `sp -0x64` | `0x1` |
| `str` at 0x2e684 | store | 4 | `sp -0x60` | `0x10` |
| `ldr` at 0x2e68c | load | 4 | `abs 0x2e840` | `-` |
| `ldr` at 0x2e690 | load | 4 | `abs 0x2e844` | `-` |
| `ldr` at 0x2e6a0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2e6a4 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0x2e6a8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x2e6bc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2e6c0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x2e6d4 | load | 4 | `abs 0x2e848` | `-` |
| `ldrb` at 0x2e6ec | load | 1 | `sp -0x13` | `-` |
| `ldr` at 0x2e6f4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2e6f8 | load | 1 | `[sp-24] +0x4` | `-` |
| `str` at 0x2e70c | store | 4 | `sp -0x64` | `0x1` |
| `str` at 0x2e714 | store | 4 | `sp -0x60` | `0x10` |
| `ldr` at 0x2e724 | load | 4 | `abs 0x2e84c` | `-` |
| `ldr` at 0x2e730 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2e734 | load | 1 | `sp -0x13` | `-` |
| `ldrb` at 0x2e738 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x2e74c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2e750 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0x2e760 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2e764 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0x2e778 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2e77c | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0x2e784 | load | 4 | `abs 0x2e850` | `-` |
| `str` at 0x2e798 | store | 4 | `sp -0x64` | `0x1` |
| `str` at 0x2e7a0 | store | 4 | `sp -0x60` | `0x10` |
| `ldr` at 0x2e7a8 | load | 4 | `abs 0x2e834` | `-` |
| `ldr` at 0x2e7ac | load | 4 | `abs 0x2e838` | `-` |
| `ldr` at 0x2e7bc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2e7c0 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0x2e7d4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2e7d8 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0x2e7e0 | load | 4 | `abs 0x2e854` | `-` |
| `str` at 0x2e7f4 | store | 4 | `sp -0x64` | `0x1` |
| `str` at 0x2e7fc | store | 4 | `sp -0x60` | `0x10` |
| `ldr` at 0x2e804 | load | 4 | `abs 0x2e834` | `-` |
| `ldr` at 0x2e808 | load | 4 | `abs 0x2e838` | `-` |
| `ldr` at 0x2e814 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x2e81c | store | 4 | `sp -0x5c` | `?` |
| `ldmdb` at 0x2e824 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2e824 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2e824 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2e5f4 | `(unknown callee) 0x3d108` | `?` | `?` | `?` | `?` |
| `bl` at 0x2e614 | `(unknown callee) 0x18c74` | `sp-88` | `0xdbda0` | `ret(0x3d108)` | `[[sp-24]+2]:1` |
| `bl` at 0x2e63c | `(unknown callee) 0x636f8` | `0x0` | `0x3402` | `0x1a3` | `sp-88` |
| `bl` at 0x2e650 | `(unknown callee) 0x3d108` | `?` | `?` | `?` | `?` |
| `bl` at 0x2e670 | `(unknown callee) 0x18c74` | `sp-88` | `0xdbd5c` | `ret(0x3d108)` | `[[sp-24]+2]:1` |
| `bl` at 0x2e698 | `(unknown callee) 0x636f8` | `0x0` | `0x3401` | `0x1a2` | `sp-88` |
| `bl` at 0x2e6c8 | `(unknown callee) 0x3d538` | `?` | `?` | `?` | `?` |
| `bl` at 0x2e6d8 | `(unknown callee) 0xd6930` | `ret(0x3d538)` | `0x42400000` | `?` | `ret(0x3d538)` |
| `bl` at 0x2e728 | `(unknown callee) 0x636f8` | `0x0` | `0x3500` | `0x1a4` | `0xdbdc8` |
| `bl` at 0x2e76c | `(unknown callee) 0x3d108` | `[[sp-24]+1]:1` | `?` | `[sp-19]:1` | `[[sp-24]+1]:1` |
| `bl` at 0x2e78c | `(unknown callee) 0x18c74` | `sp-88` | `0xdbde0` | `ret(0x3d108)` | `[[sp-24]+2]:1` |
| `bl` at 0x2e7b4 | `(unknown callee) 0x636f8` | `0x0` | `0x3402` | `0x1a3` | `sp-88` |
| `bl` at 0x2e7c8 | `(unknown callee) 0x3d108` | `[[sp-24]+1]:1` | `?` | `[sp-19]:1` | `[[sp-24]+1]:1` |
| `bl` at 0x2e7e8 | `(unknown callee) 0x18c74` | `sp-88` | `0xdbd80` | `ret(0x3d108)` | `[[sp-24]+2]:1` |
| `bl` at 0x2e810 | `(unknown callee) 0x636f8` | `0x0` | `0x3402` | `0x1a3` | `sp-88` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x2e578 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x2e5a8 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x2e5d0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x2e5e4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x2e6b8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x2e6e4 | `ret(0xd6930) == 0x0` | `ret(0xd6930) != 0x0` |
| `beq` at 0x2e704 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x2e748 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x2e75c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2e824 | `r0` | `?` |

## `i2cTest`

- `i2cTest`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x97fdc to 0x98264.
  Established. Four opening calls target `0xb6498`, `0xb7858`, `0xb4594`, `0xaff10`, with first
  candidates `sp-28`, `sp-36`, `sp-44`, `sp-52`, respectively (`bl` at 0x98000, `bl` at 0x98014,
  `bl` at 0x98028, `bl` at 0x9803c). Later stack words at `sp -0x3c` include `([sp-60] +
  ret(0x98264))` and `([sp-60] + ret(0xb150c))` (`str` at 0x980c4, `str` at 0x98218). The final test
  takes its continuation for `[sp-60] == 0x0` and falls through on inequality (`beq` at 0x98234).
  The reported r0 return and physical bus-test coverage are unknown (`ldmdb` at 0x98260).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x97fec | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x97ff4 | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0x98008 | store | 4 | `sp -0x18` | `ret(0xb6498)` |
| `str` at 0x9801c | store | 4 | `sp -0x20` | `ret(0xb7858)` |
| `str` at 0x98030 | store | 4 | `sp -0x28` | `ret(0xb4594)` |
| `str` at 0x98044 | store | 4 | `sp -0x30` | `ret(0xaff10)` |
| `str` at 0x9804c | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x98050 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x98054 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x98068 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x9806c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x98070 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x98080 | load | 1 | `sp -0x40` | `-` |
| `ldr` at 0x98098 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x980a0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x980a8 | load | 1 | `sp -0x40` | `-` |
| `ldr` at 0x980bc | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x980c4 | store | 4 | `sp -0x3c` | `([sp-60] + ret(0x98264))` |
| `ldr` at 0x980c8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x980d0 | store | 4 | `sp -0x40` | `[sp-64]+1` |
| `str` at 0x980dc | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x980e0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x980e4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x980f4 | load | 1 | `sp -0x40` | `-` |
| `ldr` at 0x9810c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x98114 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x9811c | load | 1 | `sp -0x40` | `-` |
| `ldr` at 0x98130 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x98138 | store | 4 | `sp -0x3c` | `([sp-60] + ret(0x9838c))` |
| `ldr` at 0x9813c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x98144 | store | 4 | `sp -0x40` | `[sp-64]+1` |
| `str` at 0x98150 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x98154 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x98158 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x98168 | load | 1 | `sp -0x40` | `-` |
| `ldr` at 0x98180 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x98194 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x9819c | load | 1 | `sp -0x40` | `-` |
| `ldr` at 0x981b0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x981b8 | store | 4 | `sp -0x3c` | `([sp-60] + ret(0x984b8))` |
| `ldr` at 0x981bc | load | 4 | `sp -0x40` | `-` |
| `str` at 0x981c4 | store | 4 | `sp -0x40` | `[sp-64]+1` |
| `str` at 0x981d0 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x981d4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x981d8 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x981e8 | load | 1 | `sp -0x40` | `-` |
| `ldrb` at 0x98200 | load | 1 | `sp -0x40` | `-` |
| `ldr` at 0x98210 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x98218 | store | 4 | `sp -0x3c` | `([sp-60] + ret(0xb150c))` |
| `ldr` at 0x9821c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x98224 | store | 4 | `sp -0x40` | `[sp-64]+1` |
| `ldr` at 0x9822c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x9823c | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x98244 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x9824c | store | 4 | `sp -0x38` | `?` |
| `str` at 0x98258 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x9825c | load | 4 | `sp -0x44` | `-` |
| `ldmdb` at 0x98260 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x98260 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x98260 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x98000 | `(unknown callee) 0xb6498` | `sp-28` | `r1` | `r2` | `sp-28` |
| `bl` at 0x98014 | `(unknown callee) 0xb7858` | `sp-36` | `?` | `?` | `sp-36` |
| `bl` at 0x98028 | `(unknown callee) 0xb4594` | `sp-44` | `?` | `?` | `sp-44` |
| `bl` at 0x9803c | `(unknown callee) 0xaff10` | `sp-52` | `?` | `?` | `sp-52` |
| `bl` at 0x98088 | `(unknown callee) 0xb64f4` | `[sp-64]:1` | `?` | `?` | `[sp-64]:1` |
| `bl` at 0x980b4 | `(unknown callee) 0x98264` | `?` | `[sp-64]:1` | `[sp-64]:1` | `?` |
| `bl` at 0x980fc | `(unknown callee) 0xb78b4` | `[sp-64]:1` | `?` | `?` | `[sp-64]:1` |
| `bl` at 0x98128 | `(unknown callee) 0x9838c` | `?` | `[sp-64]:1` | `[sp-64]:1` | `?` |
| `bl` at 0x98170 | `(unknown callee) 0xb460c` | `[sp-64]:1` | `?` | `?` | `[sp-64]:1` |
| `bl` at 0x981a8 | `(unknown callee) 0x984b8` | `?` | `[sp-64]:1` | `[sp-64]:1` | `?` |
| `bl` at 0x981f0 | `(unknown callee) 0xaff6c` | `[sp-64]:1` | `?` | `?` | `[sp-64]:1` |
| `bl` at 0x98208 | `(unknown callee) 0xb150c` | `[sp-64]:1` | `?` | `?` | `[sp-64]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x9805c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x98078 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x98094 | `ret(0xb64f4) == 0x0` | `ret(0xb64f4) != 0x0` |
| `blt` at 0x980ec | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x98108 | `ret(0xb78b4) == 0x0` | `ret(0xb78b4) != 0x0` |
| `blt` at 0x98160 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x9817c | `ret(0xb460c) == 0x0` | `ret(0xb460c) != 0x0` |
| `blt` at 0x981e0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x981fc | `ret(0xaff6c) == 0x0` | `ret(0xaff6c) != 0x0` |
| `beq` at 0x98234 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x98260 | `r0` | `?` |

## `nl4r_setup_llp`

- `nl4r_setup_llp`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x342c4 to 0x3472c.
  Established. The calls to `0xcb020` include `[sp-20]`, `[sp-24]+10`, `0x1f`, `sp-32` and later
  `[sp-20]`, `[sp-24]+18`, `0x2`, `sp-32` (`bl` at 0x34378, `bl` at 0x34614). Stack word `sp -0x20`
  receives `0x3f` and later `0x190` (`str` at 0x344b4, `str` at 0x345ac). The final test takes its
  alternate path for `[sp-28] == 0x0` and falls through otherwise (`beq` at 0x34670). Both paths
  reach further `0xcb020` calls with second candidate `[sp-24]+26` (`bl` at 0x34698, `bl` at
  0x346ec). r0 returns `0x0` (`ldmdb` at 0x34720). The numeric fields’ physical link behavior is
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x342d4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x342d8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x342dc | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x342e0 | load | 4 | `abs 0x34724` | `-` |
| `ldr` at 0x342e4 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x342f0 | load | 4 | `abs 0x34728` | `-` |
| `ldr` at 0x342f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x342f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x34300 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x34310 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x34314 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x34320 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x34334 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x34340 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x34358 | store | 4 | `sp -0x20` | `0x18` |
| `ldr` at 0x3435c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x34368 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3437c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x34388 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3439c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x343a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x343bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x343c8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x343e4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x343e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x343f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x34408 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x34414 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3442c | store | 4 | `sp -0x20` | `0xf` |
| `ldr` at 0x34430 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3443c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x34450 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3445c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x34470 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3447c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x34490 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3449c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x344b4 | store | 4 | `sp -0x20` | `0x3f` |
| `ldr` at 0x344b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x344c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x344d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x344e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x344f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x34504 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x34518 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x34524 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3453c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x34540 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3454c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x34560 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3456c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x34584 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x34588 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x34594 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x345ac | store | 4 | `sp -0x20` | `0x190` |
| `ldr` at 0x345b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x345bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x345d0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x345e0 | store | 4 | `sp -0x28` | `0x3` |
| `str` at 0x345ec | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x345f0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x345f4 | store | 4 | `sp -0x20` | `[sp-40]` |
| `ldr` at 0x345f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x34604 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3461c | store | 4 | `sp -0x20` | `0x2` |
| `ldr` at 0x34620 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3462c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x34644 | store | 4 | `sp -0x20` | `0x2` |
| `ldr` at 0x34648 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x34654 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x34668 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x34678 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x3467c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x34688 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x346a0 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x346a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x346b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x346cc | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x346d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x346dc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x346f4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x346f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x34704 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x34720 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x34720 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x34720 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x342fc | `(unknown callee) 0x20f14` | `0xde994` | `r0` | `r1` | `[0x27eb98]` |
| `bl` at 0x34330 | `(unknown callee) 0xcb020` | `?` | `?` | `0x0` | `sp-32` |
| `bl` at 0x34350 | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]+10` | `0x3` | `sp-32` |
| `bl` at 0x34378 | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]+10` | `0x1f` | `sp-32` |
| `bl` at 0x34398 | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]+10` | `0x20` | `sp-32` |
| `bl` at 0x343b8 | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]+10` | `0x21` | `sp-32` |
| `bl` at 0x343d8 | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]+10` | `0x22` | `sp-32` |
| `bl` at 0x34404 | `(unknown callee) 0xcb020` | `?` | `?` | `0x0` | `sp-32` |
| `bl` at 0x34424 | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]+10` | `0x3` | `sp-32` |
| `bl` at 0x3444c | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]+10` | `0x1f` | `sp-32` |
| `bl` at 0x3446c | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]+10` | `0x20` | `sp-32` |
| `bl` at 0x3448c | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]+10` | `0x21` | `sp-32` |
| `bl` at 0x344ac | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]+10` | `0x22` | `sp-32` |
| `bl` at 0x344d4 | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]+10` | `0x23` | `sp-32` |
| `bl` at 0x344f4 | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]+10` | `0x24` | `sp-32` |
| `bl` at 0x34514 | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]+10` | `0x25` | `sp-32` |
| `bl` at 0x34534 | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]+10` | `0x26` | `sp-32` |
| `bl` at 0x3455c | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]+10` | `0x2` | `sp-32` |
| `bl` at 0x3457c | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]+10` | `0x4` | `sp-32` |
| `bl` at 0x345a4 | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]+10` | `0x5` | `sp-32` |
| `bl` at 0x345cc | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]+10` | `0x6` | `sp-32` |
| `bl` at 0x34614 | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]+18` | `0x2` | `sp-32` |
| `bl` at 0x3463c | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]+18` | `0x4` | `sp-32` |
| `bl` at 0x34664 | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]+18` | `0x5` | `sp-32` |
| `bl` at 0x34698 | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]+26` | `0x0` | `sp-32` |
| `bl` at 0x346c0 | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]+26` | `0x4` | `sp-32` |
| `bl` at 0x346ec | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]+26` | `0x0` | `sp-32` |
| `bl` at 0x34714 | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]+26` | `0x4` | `sp-32` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x342ec | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `beq` at 0x34308 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x345d8 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `beq` at 0x34670 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x34720 | `r0` | `0x0` |

## `smpcmd_scan_ids`

- `smpcmd_scan_ids`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc7ecc to 0xc83cc.
  Established. The unsigned limit test takes its continuation for `[[sp-24]+40] <= 0x40` and falls
  through above it (`bls` at 0xc7f38). A later call to `0xbc8b4` uses `0x1`, `0x0`, `0x1`, `[sp-40]`
  (`bl` at 0xc812c). The call to `0xc1ab4` carries `[sp-60]`, `[sp-68]`, `0x0`, `sp-52` (`bl` at
  0xc8204). Stack word `sp -0x3c` is advanced to `[sp-60]+32` afterward (`str` at 0xc8210). The
  reported r0 return is `0x0` (`ldmdb` at 0xc838c). The calls’ physical effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc7edc | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xc7ee0 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xc7ee4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc7ef4 | store | 4 | `r1 +0x0` | `ret(0x18920)` |
| `ldr` at 0xc7ef8 | load | 4 | `abs 0xc8390` | `-` |
| `ldr` at 0xc7efc | load | 4 | `abs 0x27eb94` | `-` |
| `ldr` at 0xc7f08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc7f0c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xc7f10 | load | 4 | `abs 0xc8394` | `-` |
| `str` at 0xc7f1c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xc7f24 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc7f28 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xc7f2c | store | 4 | `sp -0x20` | `[[sp-24]+40]` |
| `ldr` at 0xc7f30 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc7f3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc7f40 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xc7f44 | load | 4 | `abs 0xc8398` | `-` |
| `str` at 0xc7f54 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xc7f5c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc7f68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc7f6c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xc7f70 | load | 4 | `abs 0xc839c` | `-` |
| `str` at 0xc7f7c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xc7f84 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc7f8c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xc7f90 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc7fa4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xc7fb0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xc7fb4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc7fc8 | store | 4 | `sp -0x40` | `ret(0x18920)` |
| `ldr` at 0xc7fcc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc7fd8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc7fdc | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xc7fe0 | load | 4 | `abs 0xc83a0` | `-` |
| `str` at 0xc7fec | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xc7ff4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc8008 | store | 4 | `sp -0x44` | `ret(0x18920)` |
| `ldr` at 0xc800c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xc8018 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc801c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xc8020 | load | 4 | `abs 0xc83a0` | `-` |
| `ldr` at 0xc8028 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xc8034 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xc8040 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xc8044 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc8048 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc8058 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc8060 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xc806c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xc8070 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc8078 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xc8084 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xc8088 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc8090 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xc80a8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xc80ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc80b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc80b8 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xc80cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc80d0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xc80d4 | load | 4 | `abs 0xc8394` | `-` |
| `ldr` at 0xc80dc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc80e4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xc80f0 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xc80f8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xc80fc | store | 4 | `sp -0x58` | `[sp-56]` |
| `ldr` at 0xc8100 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xc8104 | store | 4 | `sp -0x54` | `[sp-64]` |
| `ldr` at 0xc8108 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xc810c | store | 4 | `sp -0x50` | `[sp-68]` |
| `ldr` at 0xc8110 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc8114 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xc8118 | store | 4 | `sp -0x4c` | `[[sp-28]]` |
| `ldr` at 0xc8128 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc813c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc8140 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xc8144 | load | 4 | `abs 0xc8394` | `-` |
| `ldr` at 0xc814c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc8154 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xc8160 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xc8168 | load | 4 | `abs 0xc83a4` | `-` |
| `ldr` at 0xc8170 | load | 4 | `abs 0xc83a8` | `-` |
| `ldr` at 0xc8178 | load | 4 | `abs 0xc83ac` | `-` |
| `ldr` at 0xc8180 | load | 4 | `abs 0xc83b0` | `-` |
| `str` at 0xc818c | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0xc8194 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xc8198 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc819c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc81ac | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc81b0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xc81c4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xc81cc | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldr` at 0xc81d0 | load | 4 | `abs 0xc83b4` | `-` |
| `ldr` at 0xc81d4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc81e4 | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0xc81f0 | store | 4 | `sp -0x58` | `0x20` |
| `ldr` at 0xc81f4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc81f8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xc8208 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xc8210 | store | 4 | `sp -0x3c` | `[sp-60]+32` |
| `ldr` at 0xc8214 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc8218 | load | 4 | `abs 0xc83b8` | `-` |
| `ldr` at 0xc8224 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc822c | load | 4 | `abs 0xc83b8` | `-` |
| `str` at 0xc8238 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xc823c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc8240 | load | 4 | `abs 0xc83bc` | `-` |
| `ldr` at 0xc824c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc8254 | load | 4 | `abs 0xc83bc` | `-` |
| `str` at 0xc8260 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xc8264 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc8268 | load | 4 | `abs 0xc83c0` | `-` |
| `ldr` at 0xc8274 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc827c | load | 4 | `abs 0xc83c0` | `-` |
| `str` at 0xc8288 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xc828c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc829c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xc82b0 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xc82b4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc82c4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xc82d8 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xc82e0 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xc82ec | load | 1 | `sp -0x30` | `-` |
| `ldr` at 0xc82f4 | load | 4 | `sp -0x30` | `-` |
| `ldrh` at 0xc8304 | load | 2 | `sp -0x2e` | `-` |
| `str` at 0xc831c | store | 4 | `sp -0x58` | `?` |
| `ldrb` at 0xc8320 | load | 1 | `sp -0x2d` | `-` |
| `str` at 0xc832c | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xc8330 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xc8334 | store | 4 | `sp -0x50` | `[sp-52]` |
| `ldr` at 0xc8338 | load | 4 | `abs 0xc83c4` | `-` |
| `ldr` at 0xc833c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc8348 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc8350 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xc8358 | load | 4 | `abs 0xc8394` | `-` |
| `ldr` at 0xc8360 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc8364 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xc8368 | load | 4 | `abs 0xc83c8` | `-` |
| `ldr` at 0xc8370 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc8378 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xc8384 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xc8388 | load | 4 | `sp -0x48` | `-` |
| `ldmdb` at 0xc838c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc838c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc838c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc838c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc7eec | `(unknown callee) 0x18920` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0xc7f14 | `(unknown callee) 0x18cd4` | `[[sp-28]]` | `0x10a3f0` | `?` | `[sp-28]` |
| `bl` at 0xc7f4c | `(unknown callee) 0x18c74` | `[[sp-28]]` | `0x10a4a4` | `0x40` | `[sp-28]` |
| `bl` at 0xc7f74 | `(unknown callee) 0x18c74` | `[[sp-28]]` | `0x10a4cc` | `?` | `[sp-28]` |
| `bl` at 0xc7fc0 | `(unknown callee) 0x18920` | `?` | `?` | `?` | `?` |
| `bl` at 0xc7fe4 | `(unknown callee) 0x18c74` | `[[sp-28]]` | `0x10a4f0` | `?` | `[sp-28]` |
| `bl` at 0xc8000 | `(unknown callee) 0x18920` | `?` | `?` | `?` | `?` |
| `bl` at 0xc8024 | `(unknown callee) 0x18c74` | `[[sp-28]]` | `0x10a4f0` | `?` | `[sp-28]` |
| `bl` at 0xc802c | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xc8098 | `(unknown callee) 0xc4f18` | `ret(0x18920)` | `?` | `?` | `[sp-36]` |
| `bl` at 0xc80bc | `(unknown callee) 0xbc308` | `0x1` | `[sp-40]` | `[[sp-28]]` | `[sp-28]` |
| `bl` at 0xc80d8 | `(unknown callee) 0x18ab8` | `[[sp-28]]` | `0x10a3f0` | `?` | `[sp-28]` |
| `bl` at 0xc80e0 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xc80e8 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xc812c | `(unknown callee) 0xbc8b4` | `0x1` | `0x0` | `0x1` | `[sp-40]` |
| `bl` at 0xc8148 | `(unknown callee) 0x18ab8` | `[[sp-28]]` | `0x10a3f0` | `?` | `[sp-28]` |
| `bl` at 0xc8150 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xc8158 | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xc816c | `(unknown callee) 0x20f14` | `0x10a51c` | `?` | `?` | `ret(0xbc8b4)` |
| `bl` at 0xc8174 | `(unknown callee) 0x20f14` | `0x10a520` | `?` | `?` | `?` |
| `bl` at 0xc817c | `(unknown callee) 0x20f14` | `0x10a538` | `?` | `?` | `?` |
| `bl` at 0xc8184 | `(unknown callee) 0x20f14` | `0x10a568` | `?` | `?` | `?` |
| `bl` at 0xc81b4 | `(unknown callee) 0xc162c` | `?` | `[sp-68]` | `?` | `[sp-32]` |
| `bl` at 0xc81d8 | `(unknown callee) 0x20f14` | `0x10a598` | `[sp-44]` | `?` | `[sp-60]+1` |
| `bl` at 0xc8204 | `(unknown callee) 0xc1ab4` | `[sp-60]` | `[sp-68]` | `0x0` | `sp-52` |
| `bl` at 0xc82e8 | `(unknown callee) 0xc8610` | `[sp-52]` | `sp-48` | `?` | `sp-48` |
| `bl` at 0xc8344 | `(unknown callee) 0x20f14` | `0x10a5c4` | `[sp-44]` | `?` | `?` |
| `bl` at 0xc835c | `(unknown callee) 0x20f14` | `0x10a3f0` | `?` | `?` | `[sp-32]` |
| `bl` at 0xc836c | `(unknown callee) 0x18c74` | `[[sp-28]]` | `0x10a5fc` | `?` | `[sp-28]` |
| `bl` at 0xc8374 | `(unknown callee) 0x18be4` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xc837c | `(unknown callee) 0x18be4` | `[sp-68]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc7f04 | `[0x27eb94] != 0x0` | `[0x27eb94] == 0x0` |
| `bls` at 0xc7f38 | `[[sp-24]+40] <= 0x40 (unsigned, 32-bit)` | `[[sp-24]+40] > 0x40 (unsigned, 32-bit)` |
| `bne` at 0xc7f64 | `[[sp-24]+40] != 0x0` | `[[sp-24]+40] == 0x0` |
| `bne` at 0xc7fd4 | `ret(0x18920) != 0x0` | `ret(0x18920) == 0x0` |
| `bne` at 0xc8014 | `ret(0x18920) != 0x0` | `ret(0x18920) == 0x0` |
| `blo` at 0xc8050 | `C == 0` | `not (C == 0)` |
| `bne` at 0xc80c8 | `ret(0xbc308) != 0x0` | `ret(0xbc308) == 0x0` |
| `bne` at 0xc8138 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `blo` at 0xc81a4 | `C == 0` | `not (C == 0)` |
| `bne` at 0xc81c0 | `ret(0xc162c) != 0x0` | `ret(0xc162c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc838c | `r0` | `0x0` |
