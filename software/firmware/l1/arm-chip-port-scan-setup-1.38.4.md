# ARM chip, port and scan setup in 1.38.4

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
No unresolved computed transfers were reported for these routines. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/chip-port-scan-setup-facts.txt) retain all tool sections.

## `JChipSetup`

- `JChipSetup`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe8ffc to 0xe9028.
  Established. The word store installs `0xea678` at `0x142b58` (`str` at 0xe9010); its unbranched
  exit carries r0 `0x0` (`ldmdb` at 0xe901c). There are no calls or conditional branches in the
  facts. The stored address alone does not establish an invoked callback or hardware effect.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xe9008 | load | 4 | `abs 0xe9020` | `-` |
| `ldr` at 0xe900c | load | 4 | `abs 0xe9024` | `-` |
| `str` at 0xe9010 | store | 4 | `abs 0x142b58` | `0xea678` |
| `ldmdb` at 0xe901c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe901c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe901c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe901c | `r0` | `0x0` |

## `JChipTargetSetup`

- `JChipTargetSetup`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe9028 to 0xe9080.
  Established. The first call reaches `0xeb27c` with entry r0, entry r1, entry r2, entry r3 (`bl` at
  0xe9040); its nonzero result takes the arm containing the call to `0xe9080`. Zero falls through
  (`bne` at 0xe9054). That second call carries `ret(0xeb27c)`, unknown, unknown, `ret(0xeb27c)`
  (`bl` at 0xe9068). The merged r0 return is unknown (`ldmdb` at 0xe9078). Target setup semantics
  remain unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe9038 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xe903c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe9048 | store | 4 | `sp -0x18` | `ret(0xeb27c)` |
| `ldr` at 0xe904c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9058 | load | 4 | `abs 0xe907c` | `-` |
| `str` at 0xe905c | store | 4 | `sp -0x1c` | `0xfffff7c6` |
| `ldr` at 0xe9064 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe9070 | store | 4 | `sp -0x1c` | `ret(0xe9080)` |
| `ldr` at 0xe9074 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xe9078 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe9078 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe9078 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe9040 | `(unknown callee) 0xeb27c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xe9068 | `(unknown callee) 0xe9080` | `ret(0xeb27c)` | `?` | `?` | `ret(0xeb27c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe9054 | `ret(0xeb27c) != 0x0` | `ret(0xeb27c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe9078 | `r0` | `?` |

## `JChipBrickSetup`

- `JChipBrickSetup`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe9080 to 0xe9190.
  Established. The two indirect calls have unknown targets, with candidates unknown, `0x0`,
  `0x2b3a78`, unknown and unknown, `[[sp-20]+25]:1`, `[0x142b50]`, `[[sp-20]+25]:1`, respectively
  (`mov` at 0xe9110, `mov` at 0xe916c). The first callback-result branch takes its continuation for
  `ret(unknown) == 0x0` and falls through on inequality (`beq` at 0xe9124). A signed byte read
  addresses entry `r0 +0x21` (`ldrsb` at 0xe9098). Board selection, callback roles and the common r0
  result remain unknown (`ldmdb` at 0xe917c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe9090 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xe9094 | load | 4 | `sp -0x14` | `-` |
| `ldrsb` at 0xe9098 | load | 1 | `r0 +0x21` | `-` |
| `ldr` at 0xe90a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe90a8 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldrh` at 0xe90ac | load | 2 | `[[sp-20]] +0xe` | `-` |
| `ldr` at 0xe90bc | load | 4 | `abs 0xe9180` | `-` |
| `ldr` at 0xe90c0 | load | 4 | `abs 0x2ae5ec` | `-` |
| `ldr` at 0xe90cc | load | 4 | `abs 0xe9180` | `-` |
| `ldr` at 0xe90d0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xe90d4 | load | 1 | `[sp-20] +0x19` | `-` |
| `ldr` at 0xe90dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe90e0 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xe90e4 | load | 4 | `[[sp-20]] +0x0` | `-` |
| `ldr` at 0xe90f0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xe90f4 | load | 1 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xe90fc | load | 4 | `abs 0x2ae5ec` | `-` |
| `ldr` at 0xe9108 | load | 4 | `abs 0xe9184` | `-` |
| `str` at 0xe9118 | store | 4 | `sp -0x18` | `ret(unknown)` |
| `ldr` at 0xe911c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9128 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe912c | store | 4 | `sp -0x1c` | `ret(unknown)` |
| `ldr` at 0xe9134 | load | 4 | `abs 0xe9188` | `-` |
| `str` at 0xe9138 | store | 4 | `sp -0x1c` | `0xfffff7c5` |
| `ldr` at 0xe9140 | load | 4 | `abs 0xe918c` | `-` |
| `ldr` at 0xe9144 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xe9148 | load | 1 | `[sp-20] +0x19` | `-` |
| `ldr` at 0xe9154 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xe9158 | load | 1 | `[sp-20] +0x19` | `-` |
| `ldr` at 0xe915c | load | 4 | `abs 0x142b50` | `-` |
| `str` at 0xe9174 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xe9178 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xe917c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe917c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe917c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0xe9110 | `(unknown callee) (unknown target)` | `?` | `0x0` | `0x2b3a78` | `?` |
| `mov` at 0xe916c | `(unknown callee) (unknown target)` | `?` | `[[sp-20]+25]:1` | `[0x142b50]` | `[[sp-20]+25]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe90a0 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xe90b8 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xe90c8 | `[0x2ae5ec] == 0x0` | `[0x2ae5ec] != 0x0` |
| `beq` at 0xe9124 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe917c | `r0` | `?` |

## `JChipPortInit`

- `JChipPortInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe9190 to 0xe92b4.
  Established. The recursive call reaches `0xe9190` with `0x2ad280`, `0x1`, entry r2, entry r1 (`bl`
  at 0xe91c8). A separate call to `0x1b3c4` has entry r1, `0x10`, entry r2, unknown (`bl` at
  0xe91e4); its result is written as a word at `0x2ad38c` (`str` at 0xe91ec). Another word store
  places `[sp-28]` at `0x2ad390` (`str` at 0xe9210). Port-record ownership and the joined r0 value
  are unresolved (`ldmdb` at 0xe92a0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe91a0 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe91a4 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xe91a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe91b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe91c0 | load | 4 | `abs 0xe92a4` | `-` |
| `str` at 0xe91d0 | store | 4 | `sp -0x28` | `ret(0xe9190)` |
| `ldr` at 0xe91d8 | load | 4 | `abs 0xe92a8` | `-` |
| `ldr` at 0xe91dc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe91ec | store | 4 | `abs 0x2ad38c` | `ret(0x1b3c4)` |
| `ldr` at 0xe91f0 | load | 4 | `abs 0x2ad38c` | `-` |
| `ldr` at 0xe91fc | load | 4 | `abs 0xe92ac` | `-` |
| `str` at 0xe9200 | store | 4 | `sp -0x28` | `0xfffff7cc` |
| `ldr` at 0xe9208 | load | 4 | `abs 0xe92b0` | `-` |
| `ldr` at 0xe920c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe9210 | store | 4 | `abs 0x2ad390` | `[sp-28]` |
| `str` at 0xe9218 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xe921c | load | 4 | `abs 0xe92b0` | `-` |
| `ldr` at 0xe9220 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe9224 | load | 4 | `abs 0x2ad390` | `-` |
| `ldr` at 0xe9234 | load | 4 | `abs 0xe92a8` | `-` |
| `ldr` at 0xe9238 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe9240 | load | 4 | `abs 0x2ad38c` | `-` |
| `str` at 0xe9248 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe924c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe9250 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe9258 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9260 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe9264 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe9268 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe926c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe9274 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe927c | load | 4 | `(unknown)` | `-` |
| `str` at 0xe9280 | store | 4 | `[sp-36] +0x4` | `?` |
| `ldr` at 0xe9284 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe928c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `str` at 0xe9298 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe929c | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xe92a0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe92a0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe92a0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe92a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe91c8 | `(unknown callee) 0xe9190` | `0x2ad280` | `0x1` | `r2` | `r1` |
| `bl` at 0xe91e4 | `(unknown callee) 0x1b3c4` | `r1` | `0x10` | `r2` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe91b0 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xe91bc | `r1 != 0x0` | `r1 == 0x0` |
| `bne` at 0xe91f8 | `[0x2ad38c] != 0x0` | `[0x2ad38c] == 0x0` |
| `blt` at 0xe922c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe92a0 | `r0` | `?` |

## `JChipInit`

- `JChipInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe92b4 to 0xe974c.
  Established. The byte store at `[sp-40] +0x20` writes `0x0` (`strb` at 0xe9428). A later store
  there writes `([[sp-40]+32]:1+1 & 0xff)` (`strb` at 0xe96d8). The indirect call carries `[sp-40]`,
  unknown, unknown, `[[[sp-40]]+16]` to an unknown target (`mov` at 0xe9560); its signed result
  takes the continuation when at least `0x0` and falls through when negative (`bge` at 0xe956c).
  Chip identity encoding and the final r0 remain unproved (`ldmdb` at 0xe9710).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe92c4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe92c8 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xe92cc | load | 4 | `abs 0xe9714` | `-` |
| `ldr` at 0xe92d0 | load | 4 | `abs 0x2ad390` | `-` |
| `ldr` at 0xe92dc | load | 4 | `abs 0xe9718` | `-` |
| `str` at 0xe92e0 | store | 4 | `sp -0x50` | `0x136320` |
| `ldr` at 0xe92e8 | load | 4 | `abs 0xe971c` | `-` |
| `ldr` at 0xe92ec | load | 4 | `abs 0xe9720` | `-` |
| `ldr` at 0xe92f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9304 | load | 4 | `abs 0xe9714` | `-` |
| `ldr` at 0xe9308 | load | 4 | `abs 0x2ad390` | `-` |
| `ldr` at 0xe9314 | load | 4 | `abs 0xe9724` | `-` |
| `str` at 0xe9318 | store | 4 | `sp -0x30` | `0xfffff7c7` |
| `ldr` at 0xe9320 | load | 4 | `abs 0xe9728` | `-` |
| `ldr` at 0xe9324 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9334 | load | 4 | `abs 0x2ad38c` | `-` |
| `str` at 0xe933c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe9340 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe9344 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe9348 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe934c | load | 4 | `[sp-36] +0xc` | `-` |
| `ldr` at 0xe936c | load | 4 | `[sp-36] +0x8` | `-` |
| `str` at 0xe937c | store | 4 | `[sp-36] +0x8` | `ret(0x1b2c8)` |
| `ldr` at 0xe9380 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0xe938c | load | 4 | `abs 0xe972c` | `-` |
| `str` at 0xe9390 | store | 4 | `sp -0x30` | `0xfffff7cc` |
| `ldr` at 0xe9398 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe939c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe93a0 | load | 4 | `[sp-36] +0xc` | `-` |
| `ldr` at 0xe93bc | load | 4 | `[sp-36] +0x8` | `-` |
| `str` at 0xe93c4 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe93c8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe93cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe93d0 | load | 4 | `[sp-36] +0xc` | `-` |
| `str` at 0xe93d8 | store | 4 | `[sp-36] +0xc` | `[[sp-36]+12]+1` |
| `ldr` at 0xe93dc | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xe93e8 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0xe93f0 | store | 1 | `[sp-40] +0x18` | `?` |
| `ldr` at 0xe93f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe93f8 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xe9410 | store | 1 | `[sp-40] +0x19` | `?` |
| `ldr` at 0xe9414 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe9418 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe941c | store | 4 | `[sp-40] +0x1c` | `[sp-28]` |
| `ldr` at 0xe9420 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0xe9428 | store | 1 | `[sp-40] +0x20` | `0x0` |
| `ldr` at 0xe942c | load | 4 | `sp -0x28` | `-` |
| `strb` at 0xe9434 | store | 1 | `[sp-40] +0x21` | `?` |
| `ldr` at 0xe9438 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe943c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe9450 | store | 4 | `[sp-40] +0x0` | `ret(0xeb1d0)` |
| `ldr` at 0xe9454 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0xe9460 | load | 4 | `abs 0xe9730` | `-` |
| `str` at 0xe9464 | store | 4 | `sp -0x50` | `0x136364` |
| `ldr` at 0xe9468 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe9474 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xe947c | load | 4 | `abs 0xe971c` | `-` |
| `ldr` at 0xe948c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe9490 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xe949c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe94a0 | load | 4 | `[sp-36] +0x4` | `-` |
| `ldr` at 0xe94b0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe94b8 | store | 4 | `sp -0x34` | `[sp-40]+4` |
| `ldr` at 0xe94bc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe94c0 | load | 4 | `[sp-36] +0x4` | `-` |
| `str` at 0xe94c4 | store | 4 | `sp -0x38` | `[[sp-36]+4]` |
| `ldr` at 0xe94c8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe94cc | load | 4 | `[sp-36] +0x4` | `-` |
| `ldr` at 0xe94d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe94dc | load | 4 | `[sp-36] +0x0` | `-` |
| `str` at 0xe94e0 | store | 4 | `sp -0x38` | `[[sp-36]]` |
| `ldr` at 0xe94e4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe94e8 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0xe94ec | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xe94f0 | load | 1 | `[sp-40] +0x18` | `-` |
| `str` at 0xe94f4 | store | 4 | `sp -0x50` | `[[sp-40]+24]:1` |
| `ldr` at 0xe94f8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe94fc | load | 4 | `abs 0xe9734` | `-` |
| `ldr` at 0xe9500 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe9504 | load | 4 | `[[sp-40]] +0x8` | `-` |
| `ldr` at 0xe9510 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe9518 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe951c | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0xe9520 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xe9524 | load | 1 | `[sp-40] +0x18` | `-` |
| `ldr` at 0xe952c | load | 4 | `abs 0xe9738` | `-` |
| `ldr` at 0xe9530 | load | 4 | `[[sp-40]] +0x8` | `-` |
| `ldr` at 0xe9538 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe953c | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0xe9540 | load | 4 | `[[sp-40]] +0x10` | `-` |
| `ldr` at 0xe954c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe9550 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0xe9554 | load | 4 | `[[sp-40]] +0x10` | `-` |
| `ldr` at 0xe9558 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe9570 | load | 4 | `abs 0xe973c` | `-` |
| `str` at 0xe9574 | store | 4 | `sp -0x50` | `0x1363a0` |
| `ldr` at 0xe9578 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe9584 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xe958c | load | 4 | `abs 0xe971c` | `-` |
| `ldr` at 0xe9590 | load | 4 | `abs 0xe9740` | `-` |
| `str` at 0xe95a0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xe95a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe95a8 | load | 4 | `[sp-36] +0xc` | `-` |
| `ldr` at 0xe95b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe95c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe95c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe95e0 | load | 4 | `[sp-36] +0x8` | `-` |
| `str` at 0xe95e8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe95ec | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe95f0 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xe95f4 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xe95f8 | load | 1 | `[sp-40] +0x19` | `-` |
| `ldr` at 0xe9604 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe9608 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe960c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe9610 | load | 4 | `[sp-40] +0x1c` | `-` |
| `ldr` at 0xe961c | load | 4 | `abs 0xe9744` | `-` |
| `str` at 0xe9620 | store | 4 | `sp -0x50` | `0x1363d8` |
| `ldr` at 0xe9624 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xe9628 | load | 1 | `[sp-40] +0x19` | `-` |
| `str` at 0xe962c | store | 4 | `sp -0x4c` | `[[sp-40]+25]:1` |
| `ldr` at 0xe9630 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe9634 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xe9638 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe963c | load | 4 | `[sp-40] +0x1c` | `-` |
| `str` at 0xe9640 | store | 4 | `sp -0x44` | `[[sp-40]+28]` |
| `ldr` at 0xe9644 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe9648 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe964c | load | 4 | `(unknown)` | `-` |
| `str` at 0xe9650 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xe9654 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe9658 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0xe965c | load | 4 | `[[sp-40]] +0x0` | `-` |
| `str` at 0xe9660 | store | 4 | `sp -0x3c` | `[[[sp-40]]]` |
| `ldr` at 0xe9668 | load | 4 | `abs 0xe971c` | `-` |
| `ldr` at 0xe966c | load | 4 | `abs 0xe9748` | `-` |
| `ldr` at 0xe9678 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe967c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xe9680 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xe9684 | load | 1 | `[sp-40] +0x19` | `-` |
| `ldr` at 0xe9690 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe9694 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe9698 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe969c | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0xe96a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe96a4 | load | 4 | `[[sp-40]] +0x0` | `-` |
| `ldr` at 0xe96b0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe96b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe96b8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe96bc | load | 4 | `[sp-40] +0x1c` | `-` |
| `ldr` at 0xe96c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe96cc | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xe96d0 | load | 1 | `[sp-40] +0x20` | `-` |
| `strb` at 0xe96d8 | store | 1 | `[sp-40] +0x20` | `([[sp-40]+32]:1+1 & 0xff)` |
| `ldr` at 0xe96e0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe96e4 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xe96e8 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xe96f0 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe96f4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe96fc | store | 4 | `sp -0x20` | `?` |
| `str` at 0xe9708 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xe970c | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xe9710 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe9710 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe9710 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe9710 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe92f4 | `(unknown callee) 0x21e28` | `0x9` | `0x13634c` | `0x12d` | `0x0` |
| `bl` at 0xe9374 | `(unknown callee) 0x1b2c8` | `[[sp-36]+8]` | `?` | `[[sp-36]+12]` | `?` |
| `bl` at 0xe944c | `(unknown callee) 0xeb1d0` | `?` | `?` | `[sp-40]` | `?` |
| `bl` at 0xe9488 | `(unknown callee) 0x21e28` | `0x9` | `0x13634c` | `0x144` | `0x0` |
| `bl` at 0xe9508 | `(unknown callee) 0x1b49c` | `[sp-40]+4` | `0x136390` | `?` | `[[[sp-40]]+8]` |
| `bl` at 0xe9534 | `(unknown callee) 0x1b49c` | `[sp-40]+4` | `0x136398` | `[[[sp-40]]+8]` | `[[sp-40]+24]:1` |
| `mov` at 0xe9560 | `(unknown callee) (unknown target)` | `[sp-40]` | `?` | `?` | `[[[sp-40]]+16]` |
| `bl` at 0xe9598 | `(unknown callee) 0x21e28` | `0x9` | `0x13634c` | `0x152` | `0x0` |
| `bl` at 0xe9674 | `(unknown callee) 0x21e28` | `0x9` | `0x13634c` | `0x163` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe92d8 | `[0x2ad390] != 0x0` | `[0x2ad390] == 0x0` |
| `blt` at 0xe9310 | `? < [0x2ad390] (signed, 32-bit)` | `? >= [0x2ad390] (signed, 32-bit)` |
| `bne` at 0xe9388 | `[[sp-36]+8] != 0x0` | `[[sp-36]+8] == 0x0` |
| `bne` at 0xe945c | `[[sp-40]] != 0x0` | `[[sp-40]] == 0x0` |
| `bne` at 0xe9498 | `[[sp-36]] != 0x0` | `[[sp-36]] == 0x0` |
| `bne` at 0xe94a8 | `[[sp-36]+4] != 0x0` | `[[sp-36]+4] == 0x0` |
| `bne` at 0xe94d4 | `[[sp-36]+4] != 0x0` | `[[sp-36]+4] == 0x0` |
| `beq` at 0xe9548 | `[[[sp-40]]+16] == 0x0` | `[[[sp-40]]+16] != 0x0` |
| `bge` at 0xe956c | `ret(unknown) >= 0x0 (signed, 32-bit)` | `ret(unknown) < 0x0 (signed, 32-bit)` |
| `blt` at 0xe95b8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xe9600 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xe9618 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xe968c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xe96ac | `Z == 0` | `not (Z == 0)` |
| `bhs` at 0xe96c4 | `C == 1` | `not (C == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe9710 | `r0` | `?` |

## `JChipUpdatePort`

- `JChipUpdatePort`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe974c to 0xe9b0c.
  Established. The entry comparison takes the work arm for signed `r0 < [0x2ad390]` and falls
  through for greater or equal (`blt` at 0xe9770). The call to `0xe6e08` uses `[[sp-52]+25]:1`,
  unknown, `[[sp-52]+32]:1`, `[[[sp-52]]]` (`bl` at 0xe9a5c); its result is stored as `(ret(0xe6e08)
  & 0xff)` at `[sp-52] +0x21` (`strb` at 0xe9a64). Port compatibility rules and the r0 return remain
  unknown (`ldmdb` at 0xe9aec).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe975c | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xe9760 | load | 4 | `abs 0xe9af0` | `-` |
| `ldr` at 0xe9764 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9768 | load | 4 | `abs 0x2ad390` | `-` |
| `ldr` at 0xe9774 | load | 4 | `abs 0xe9af4` | `-` |
| `str` at 0xe9778 | store | 4 | `sp -0x44` | `0xfffff7c5` |
| `ldr` at 0xe9780 | load | 4 | `abs 0xe9af8` | `-` |
| `ldr` at 0xe9784 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe978c | load | 4 | `abs 0x2ad38c` | `-` |
| `str` at 0xe9794 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0xe979c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xe97a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe97a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe97a8 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0xe97b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe97bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe97d8 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0xe97e0 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe97e4 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xe97e8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xe97fc | load | 4 | `abs 0xe9afc` | `-` |
| `str` at 0xe9804 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe9808 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe980c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe9814 | load | 4 | `sp -0x38` | `-` |
| `ldrb` at 0xe9818 | load | 1 | `[sp-56] +0x10` | `-` |
| `ldr` at 0xe9824 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xe982c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe9834 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe9838 | load | 4 | `[sp-56] +0x14` | `-` |
| `ldr` at 0xe9844 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe9848 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe9860 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe9864 | load | 4 | `[sp-56] +0x14` | `-` |
| `ldr` at 0xe9874 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xe987c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe9884 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe9888 | load | 4 | `[sp-56] +0x14` | `-` |
| `ldr` at 0xe9894 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe9898 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe98a8 | load | 4 | `abs 0xe9b00` | `-` |
| `ldr` at 0xe98b4 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xe98bc | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe98c4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe98c8 | load | 4 | `[sp-56] +0x8` | `-` |
| `ldr` at 0xe98d4 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xe98dc | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe98e0 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xe98e4 | load | 1 | `[sp-52] +0x20` | `-` |
| `str` at 0xe98e8 | store | 4 | `sp -0x2c` | `[[sp-52]+32]:1` |
| `str` at 0xe98f0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe98f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe98f8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe98fc | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0xe990c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe9910 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe992c | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0xe9934 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xe9938 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe993c | load | 4 | `(unknown)` | `-` |
| `str` at 0xe9940 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xe9944 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe9948 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xe994c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xe9950 | load | 1 | `[sp-52] +0x19` | `-` |
| `ldr` at 0xe995c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe9960 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xe9964 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xe9968 | load | 1 | `[sp-52] +0x20` | `-` |
| `ldr` at 0xe9974 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe9978 | load | 4 | `[sp-52] +0x1c` | `-` |
| `ldr` at 0xe9980 | load | 4 | `sp -0x38` | `-` |
| `ldrb` at 0xe9984 | load | 1 | `[sp-56] +0x10` | `-` |
| `str` at 0xe9994 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe999c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe99a0 | load | 4 | `[sp-56] +0x14` | `-` |
| `ldr` at 0xe99ac | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe99b0 | load | 4 | `[sp-52] +0x1c` | `-` |
| `ldr` at 0xe99c8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe99cc | load | 4 | `[sp-56] +0x14` | `-` |
| `str` at 0xe99e0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe99e8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe99ec | load | 4 | `[sp-56] +0x14` | `-` |
| `ldr` at 0xe99f8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe99fc | load | 4 | `[sp-52] +0x1c` | `-` |
| `ldr` at 0xe9a0c | load | 4 | `abs 0xe9b00` | `-` |
| `str` at 0xe9a1c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe9a20 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe9a28 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe9a30 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe9a34 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xe9a38 | load | 1 | `[sp-52] +0x19` | `-` |
| `ldr` at 0xe9a3c | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xe9a40 | load | 1 | `[sp-52] +0x20` | `-` |
| `ldr` at 0xe9a44 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe9a48 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0xe9a58 | load | 4 | `[[sp-52]] +0x0` | `-` |
| `strb` at 0xe9a64 | store | 1 | `[sp-52] +0x21` | `(ret(0xe6e08) & 0xff)` |
| `ldr` at 0xe9a68 | load | 4 | `abs 0xe9b04` | `-` |
| `ldr` at 0xe9a6c | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xe9a78 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe9a7c | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0xe9a80 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xe9a84 | load | 1 | `[sp-52] +0x18` | `-` |
| `ldr` at 0xe9a88 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xe9a8c | load | 1 | `[sp-52] +0x19` | `-` |
| `str` at 0xe9a90 | store | 4 | `sp -0x54` | `[[sp-52]+25]:1` |
| `ldr` at 0xe9a94 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xe9a98 | load | 1 | `[sp-52] +0x20` | `-` |
| `str` at 0xe9a9c | store | 4 | `sp -0x50` | `[[sp-52]+32]:1` |
| `ldr` at 0xe9aa0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe9aa4 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0xe9aa8 | load | 4 | `[[sp-52]] +0x0` | `-` |
| `str` at 0xe9aac | store | 4 | `sp -0x4c` | `[[[sp-52]]]` |
| `ldr` at 0xe9ab0 | load | 4 | `sp -0x34` | `-` |
| `ldrsb` at 0xe9ab4 | load | 1 | `[sp-52] +0x21` | `-` |
| `str` at 0xe9ab8 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xe9abc | load | 4 | `abs 0xe9b08` | `-` |
| `ldr` at 0xe9ac0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9ac4 | load | 4 | `[[sp-52]] +0x8` | `-` |
| `ldr` at 0xe9ad0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe9ad8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `str` at 0xe9ae4 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xe9ae8 | load | 4 | `sp -0x44` | `-` |
| `ldmdb` at 0xe9aec | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe9aec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe9aec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe9aec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe9a5c | `(unknown callee) 0xe6e08` | `[[sp-52]+25]:1` | `?` | `[[sp-52]+32]:1` | `[[[sp-52]]]` |
| `bl` at 0xe9acc | `(unknown callee) 0x23f2c` | `0x136440` | `[sp-24]` | `[[[sp-52]]+8]` | `[[sp-52]+24]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xe9770 | `r0 < [0x2ad390] (signed, 32-bit)` | `r0 >= [0x2ad390] (signed, 32-bit)` |
| `blt` at 0xe97b0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xe9820 | `? == [[sp-56]+16]:1` | `? != [[sp-56]+16]:1` |
| `beq` at 0xe9840 | `[[sp-56]+20] == 0x0` | `[[sp-56]+20] != 0x0` |
| `bne` at 0xe9870 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe9890 | `[[sp-56]+20] != 0x0` | `[[sp-56]+20] == 0x0` |
| `beq` at 0xe98b0 | `? == 0xffff` | `? != 0xffff` |
| `bne` at 0xe98d0 | `[[sp-56]+8] != 0x0` | `[[sp-56]+8] == 0x0` |
| `blt` at 0xe9904 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xe9958 | `Z == 0` | `not (Z == 0)` |
| `blo` at 0xe9970 | `C == 0` | `not (C == 0)` |
| `beq` at 0xe998c | `? == [[sp-56]+16]:1` | `? != [[sp-56]+16]:1` |
| `beq` at 0xe99a8 | `[[sp-56]+20] == 0x0` | `[[sp-56]+20] != 0x0` |
| `bne` at 0xe99d8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe99f4 | `[[sp-56]+20] != 0x0` | `[[sp-56]+20] == 0x0` |
| `beq` at 0xe9a14 | `? == 0xffff` | `? != 0xffff` |
| `bls` at 0xe9a74 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe9aec | `r0` | `?` |

## `JChipGetType`

- `JChipGetType`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeb1d0 to 0xeb27c.
  Established. A word read addresses `0x142b68` (`ldr` at 0xeb1f4). The equality test takes its
  branch when an unknown value is not equal to `[sp-20]` and falls through on equality (`bne` at
  0xeb22c). Separate stack-word stores at `sp -0x1c` have unknown value and `0x0`, respectively
  (`str` at 0xeb24c, `str` at 0xeb268). The facts report no calls and leave r0 unresolved at the
  common exit (`ldmdb` at 0xeb270). A complete type-table interpretation is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xeb1e0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xeb1e8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xeb1ec | load | 4 | `abs 0xeb274` | `-` |
| `ldr` at 0xeb1f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb1f4 | load | 4 | `abs 0x142b68` | `-` |
| `ldr` at 0xeb204 | load | 4 | `abs 0xeb278` | `-` |
| `ldr` at 0xeb208 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb220 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeb224 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb230 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb244 | load | 4 | `abs 0xeb278` | `-` |
| `str` at 0xeb24c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xeb254 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xeb25c | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0xeb268 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xeb26c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xeb270 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xeb270 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xeb270 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xeb1fc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xeb22c | `? != [sp-20]` | `? == [sp-20]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xeb270 | `r0` | `?` |

## `JChipGetChip`

- `JChipGetChip`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeb27c to 0xeb3bc.
  Established. The stack byte at `sp -0x21` receives an unknown value (`strb` at 0xeb2d0). Two later
  tests take their branches for `Z == 0` and fall through otherwise. Their compared operands are not
  resolved by the facts (`bne` at 0xeb32c, `bne` at 0xeb364). The terminal stack-word store writes
  `0x0` at `sp -0x28` (`str` at 0xeb3ac). No calls are reported. Lookup encoding and the joined r0
  result remain unknown (`ldmdb` at 0xeb3b4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xeb28c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xeb290 | load | 4 | `abs 0xeb3b8` | `-` |
| `ldr` at 0xeb294 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb2a4 | load | 4 | `abs 0x2ad38c` | `-` |
| `str` at 0xeb2ac | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xeb2b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xeb2bc | store | 4 | `sp -0x20` | `?` |
| `ldrb` at 0xeb2c8 | load | 1 | `sp -0x14` | `-` |
| `strb` at 0xeb2d0 | store | 1 | `sp -0x21` | `?` |
| `str` at 0xeb2d8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xeb2dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeb2e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb2e4 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0xeb2f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeb2f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb314 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xeb31c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeb320 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeb324 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeb330 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeb334 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb350 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0xeb358 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xeb35c | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xeb368 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeb36c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb388 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0xeb390 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xeb398 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xeb3a0 | store | 4 | `sp -0x18` | `?` |
| `str` at 0xeb3ac | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xeb3b0 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xeb3b4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xeb3b4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xeb3b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xeb2ec | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xeb32c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xeb364 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xeb3b4 | `r0` | `?` |

## `JChipGetCfgTarget`

- `JChipGetCfgTarget`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeb47c to 0xeb644.
  Established. Entry-byte saves write `(r0 & 0xff)` at `sp -0x11` and `(r1 & 0xff)` at `sp -0x12`
  (`strb` at 0xeb494, `strb` at 0xeb49c). The mismatch branch is taken when an unknown value is not
  equal to `(r1 & 0xff)` and falls through on equality (`bne` at 0xeb514). The only call targets
  `0x21e28` with `0x9`, `0x13634c`, `0x40d`, `0x0` (`bl` at 0xeb5d8); its exit reports r0 `[sp-44]`
  (`ldmdb` at 0xeb62c). Configuration-target encoding remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xeb494 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xeb49c | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldr` at 0xeb4a0 | load | 4 | `abs 0xeb630` | `-` |
| `ldrb` at 0xeb4a4 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xeb4ac | load | 4 | `abs 0x2ad38c` | `-` |
| `str` at 0xeb4b4 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xeb4bc | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xeb4c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeb4c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeb4c8 | load | 4 | `[sp-36] +0xc` | `-` |
| `ldr` at 0xeb4d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeb4dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeb4f8 | load | 4 | `[sp-36] +0x8` | `-` |
| `str` at 0xeb500 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xeb504 | load | 4 | `sp -0x28` | `-` |
| `ldrsb` at 0xeb508 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xeb50c | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0xeb518 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeb51c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeb520 | load | 4 | `abs 0xeb634` | `-` |
| `ldrb` at 0xeb524 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xeb544 | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0xeb54c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xeb554 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xeb558 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeb564 | load | 4 | `abs 0xeb638` | `-` |
| `str` at 0xeb568 | store | 4 | `sp -0x40` | `0x1366bc` |
| `ldr` at 0xeb56c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeb570 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeb574 | load | 4 | `(unknown)` | `-` |
| `str` at 0xeb578 | store | 4 | `sp -0x3c` | `?` |
| `ldrb` at 0xeb57c | load | 1 | `sp -0x11` | `-` |
| `str` at 0xeb580 | store | 4 | `sp -0x38` | `(r0 & 0xff)` |
| `ldrb` at 0xeb584 | load | 1 | `sp -0x12` | `-` |
| `str` at 0xeb588 | store | 4 | `sp -0x34` | `(r1 & 0xff)` |
| `ldr` at 0xeb58c | load | 4 | `abs 0xeb634` | `-` |
| `ldrb` at 0xeb590 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xeb5b0 | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0xeb5b8 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xeb5c0 | load | 1 | `(unknown)` | `-` |
| `str` at 0xeb5c4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xeb5cc | load | 4 | `abs 0xeb63c` | `-` |
| `ldr` at 0xeb5d0 | load | 4 | `abs 0xeb640` | `-` |
| `ldr` at 0xeb5dc | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xeb5e0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xeb5e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeb5ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeb5f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeb5fc | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xeb600 | load | 1 | `(unknown)` | `-` |
| `str` at 0xeb608 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xeb610 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xeb618 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0xeb624 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xeb628 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xeb62c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xeb62c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xeb62c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeb5d8 | `(unknown callee) 0x21e28` | `0x9` | `0x13634c` | `0x40d` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xeb4d0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xeb514 | `? != (r1 & 0xff)` | `? == (r1 & 0xff)` |
| `beq` at 0xeb560 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xeb62c | `r0` | `[sp-44]` |

## `build_ir_access_chip`

- `build_ir_access_chip`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeb3bc to 0xeb47c.
  Established. The unsigned loop test takes its body for `[0x2ad288] < [sp-36]` and falls through
  for greater or equal (`blo` at 0xeb41c). The word store at `0x2ad288` first writes `0x0`. A later
  store there writes `[0x2ad288]+1` (`str` at 0xeb408, `str` at 0xeb450). The sole call to `0xe2174`
  has `0x0`, `[sp-32]`, `[sp-24]`, `[sp]` (`bl` at 0xeb470). The return propagates `ret(0xe2174)`
  (`ldmdb` at 0xeb474). Instruction-register bit layout is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xeb3cc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xeb3d0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xeb3d4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xeb3d8 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xeb3dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb3f0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xeb3fc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xeb400 | load | 4 | `abs 0xeb478` | `-` |
| `str` at 0xeb408 | store | 4 | `abs 0x2ad288` | `0x0` |
| `ldr` at 0xeb40c | load | 4 | `abs 0xeb478` | `-` |
| `ldr` at 0xeb410 | load | 4 | `abs 0x2ad288` | `-` |
| `ldr` at 0xeb414 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeb424 | load | 4 | `abs 0xeb478` | `-` |
| `ldr` at 0xeb428 | load | 4 | `abs 0x2ad288` | `-` |
| `ldr` at 0xeb430 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xeb43c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xeb440 | load | 4 | `abs 0xeb478` | `-` |
| `ldr` at 0xeb444 | load | 4 | `abs 0xeb478` | `-` |
| `ldr` at 0xeb448 | load | 4 | `abs 0x2ad288` | `-` |
| `str` at 0xeb450 | store | 4 | `abs 0x2ad288` | `[0x2ad288]+1` |
| `ldr` at 0xeb458 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xeb45c | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0xeb464 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeb468 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb46c | load | 4 | `sp +0x0` | `-` |
| `ldmdb` at 0xeb474 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xeb474 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xeb474 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeb470 | `(unknown callee) 0xe2174` | `0x0` | `[sp-32]` | `[sp-24]` | `[sp]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xeb41c | `[0x2ad288] < [sp-36] (unsigned, 32-bit)` | `[0x2ad288] >= [sp-36] (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xeb474 | `r0` | `ret(0xe2174)` |

## `build_dr_access_chip`

- `build_dr_access_chip`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeb644 to 0xeb734.
  Established. The initial word store writes `(r0 + r2)-1` at `[sp] +0x0` (`str` at 0xeb678); its
  next branch takes the bypass for `[sp-32] == [sp+4]` and falls through on inequality (`beq` at
  0xeb688). Fall-through includes a word store of `0x0` at an unknown address (`str` at 0xeb6f4).
  The call to `0xe2174` has `0x0`, `[sp-32]`, `[sp-24]`, `[sp+4]` (`bl` at 0xeb728). Data-register
  encoding and the merged r0 value remain unresolved (`ldmdb` at 0xeb72c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xeb654 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xeb658 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xeb65c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xeb660 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xeb664 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xeb668 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb66c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xeb678 | store | 4 | `[sp] +0x0` | `(r0 + r2)-1` |
| `ldr` at 0xeb67c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeb680 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xeb68c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xeb690 | load | 4 | `[sp] +0x0` | `-` |
| `ldr` at 0xeb6a4 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xeb6a8 | load | 4 | `[sp] +0x0` | `-` |
| `str` at 0xeb6b4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xeb6b8 | load | 4 | `abs 0xeb730` | `-` |
| `str` at 0xeb6c0 | store | 4 | `abs 0x2ad288` | `0x0` |
| `ldr` at 0xeb6c4 | load | 4 | `abs 0xeb730` | `-` |
| `ldr` at 0xeb6c8 | load | 4 | `abs 0x2ad288` | `-` |
| `ldr` at 0xeb6cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeb6dc | load | 4 | `abs 0xeb730` | `-` |
| `ldr` at 0xeb6e0 | load | 4 | `abs 0x2ad288` | `-` |
| `ldr` at 0xeb6e8 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xeb6f4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xeb6f8 | load | 4 | `abs 0xeb730` | `-` |
| `ldr` at 0xeb6fc | load | 4 | `abs 0xeb730` | `-` |
| `ldr` at 0xeb700 | load | 4 | `abs 0x2ad288` | `-` |
| `str` at 0xeb708 | store | 4 | `abs 0x2ad288` | `[0x2ad288]+1` |
| `ldr` at 0xeb710 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xeb714 | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0xeb71c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeb720 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb724 | load | 4 | `sp +0x4` | `-` |
| `ldmdb` at 0xeb72c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xeb72c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xeb72c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeb728 | `(unknown callee) 0xe2174` | `0x0` | `[sp-32]` | `[sp-24]` | `[sp+4]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xeb688 | `[sp-32] == [sp+4]` | `[sp-32] != [sp+4]` |
| `blo` at 0xeb6d4 | `[0x2ad288] < [sp-36] (unsigned, 32-bit)` | `[0x2ad288] >= [sp-36] (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xeb72c | `r0` | `?` |

## `JTAG_LLP_Setup`

- `JTAG_LLP_Setup`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf36e0 to 0xf3720.
  Established. Two word stores install `0xf4250` at `0x142b5c` and `0xf3d60` at `0x142b60`,
  respectively (`str` at 0xf36f4, `str` at 0xf3700). The exit supplies r0 `0x0` (`ldmdb` at
  0xf370c). The fact sections contain neither calls nor conditional branches. Invocations of those
  addresses and physical link effects are outside this body.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xf36ec | load | 4 | `abs 0xf3710` | `-` |
| `ldr` at 0xf36f0 | load | 4 | `abs 0xf3714` | `-` |
| `str` at 0xf36f4 | store | 4 | `abs 0x142b5c` | `0xf4250` |
| `ldr` at 0xf36f8 | load | 4 | `abs 0xf3718` | `-` |
| `ldr` at 0xf36fc | load | 4 | `abs 0xf371c` | `-` |
| `str` at 0xf3700 | store | 4 | `abs 0x142b60` | `0xf3d60` |
| `ldmdb` at 0xf370c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf370c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf370c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf370c | `r0` | `0x0` |

## `smpcmd_scan_reset`

- `smpcmd_scan_reset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe7c58 to 0xe7d98.
  Established. The two calls to `0xdc9a8` have `0x0`, `[sp-32]`, `[[sp-28]]`, `[sp-28]` and `0x1`,
  `[sp-32]`, `[[sp-28]]`, `[sp-28]`, respectively (`bl` at 0xe7cec, `bl` at 0xe7d40). Each following
  result test takes its continuation on nonzero and falls through on zero (`bne` at 0xe7cf8, `bne`
  at 0xe7d4c). The common return reports r0 `0x0` (`ldmdb` at 0xe7d88). Reset-line identity and
  physical assertion remain unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe7c68 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe7c6c | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xe7c70 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe7c80 | store | 4 | `r1 +0x0` | `ret(0x1b124)` |
| `ldr` at 0xe7c84 | load | 4 | `abs 0xe7d8c` | `-` |
| `ldr` at 0xe7c88 | load | 4 | `abs 0x2ad258` | `-` |
| `ldr` at 0xe7c94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7c98 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe7c9c | load | 4 | `abs 0xe7d90` | `-` |
| `str` at 0xe7ca8 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xe7cc0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xe7cc4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7cc8 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xe7ccc | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0xe7cdc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7ce4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe7ce8 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe7cfc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7d00 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe7d04 | load | 4 | `abs 0xe7d90` | `-` |
| `str` at 0xe7d10 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe7d18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7d1c | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xe7d20 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0xe7d30 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7d38 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe7d3c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe7d50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7d54 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe7d58 | load | 4 | `abs 0xe7d90` | `-` |
| `str` at 0xe7d64 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe7d6c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7d70 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe7d74 | load | 4 | `abs 0xe7d94` | `-` |
| `str` at 0xe7d80 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe7d84 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xe7d88 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe7d88 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe7d88 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe7d88 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe7c78 | `(unknown callee) 0x1b124` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0xe7ca0 | `(unknown callee) 0x1b2d4` | `[[sp-28]]` | `0x135ce8` | `?` | `[sp-28]` |
| `bl` at 0xe7cb0 | `(unknown callee) 0xe58a0` | `ret(0x1b124)` | `?` | `?` | `[0x2ad258]` |
| `bl` at 0xe7cec | `(unknown callee) 0xdc9a8` | `0x0` | `[sp-32]` | `[[sp-28]]` | `[sp-28]` |
| `bl` at 0xe7d08 | `(unknown callee) 0x1b2d4` | `[[sp-28]]` | `0x135ce8` | `?` | `[sp-28]` |
| `bl` at 0xe7d40 | `(unknown callee) 0xdc9a8` | `0x1` | `[sp-32]` | `[[sp-28]]` | `[sp-28]` |
| `bl` at 0xe7d5c | `(unknown callee) 0x1b2d4` | `[[sp-28]]` | `0x135ce8` | `?` | `[sp-28]` |
| `bl` at 0xe7d78 | `(unknown callee) 0x1b49c` | `[[sp-28]]` | `0x135cec` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe7c90 | `[0x2ad258] != 0x0` | `[0x2ad258] == 0x0` |
| `beq` at 0xe7cd8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xe7cf8 | `ret(0xdc9a8) != 0x0` | `ret(0xdc9a8) == 0x0` |
| `beq` at 0xe7d2c | `? == 0x0` | `? != 0x0` |
| `bne` at 0xe7d4c | `ret(0xdc9a8) != 0x0` | `ret(0xdc9a8) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe7d88 | `r0` | `0x0` |

## `smpcmd_scan_sel`

- `smpcmd_scan_sel`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe7d98 to 0xe7f7c.
  Established. Two unsigned bounds branches take their continuations for `[[sp-24]+40] <= 0xff` and
  `[[sp-24]+44] <= 0xffff`, respectively, and fall through for greater (`bls` at 0xe7e0c, `bls` at
  0xe7e44). The call to `0xdd4e4` has unknown, `[[sp-24]+40]`, `[[sp-24]+44]`, `0x0` (`bl` at
  0xe7f10); its nonzero result takes the next continuation and zero falls through (`bne` at
  0xe7f1c). The return carries r0 `0x0` (`ldmdb` at 0xe7f60). Physical scan-selection semantics are
  unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe7da8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe7dac | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xe7db0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe7dc0 | store | 4 | `r1 +0x0` | `ret(0x1b124)` |
| `ldr` at 0xe7dc4 | load | 4 | `abs 0xe7f64` | `-` |
| `ldr` at 0xe7dc8 | load | 4 | `abs 0x2ad258` | `-` |
| `ldr` at 0xe7dd4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7dd8 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe7ddc | load | 4 | `abs 0xe7f68` | `-` |
| `str` at 0xe7de8 | store | 4 | `sp -0xd8` | `0x0` |
| `str` at 0xe7df4 | store | 4 | `sp -0x20` | `0x58` |
| `ldr` at 0xe7df8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7dfc | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe7e00 | store | 4 | `sp -0x24` | `[[sp-24]+40]` |
| `ldr` at 0xe7e04 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe7e10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7e14 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe7e18 | load | 4 | `abs 0xe7f6c` | `-` |
| `str` at 0xe7e24 | store | 4 | `sp -0xd8` | `0x0` |
| `ldr` at 0xe7e2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7e30 | load | 4 | `[sp-24] +0x2c` | `-` |
| `str` at 0xe7e34 | store | 4 | `sp -0x2c` | `[[sp-24]+44]` |
| `ldr` at 0xe7e38 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe7e3c | load | 4 | `abs 0xe7f70` | `-` |
| `ldr` at 0xe7e48 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7e4c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe7e50 | load | 4 | `abs 0xe7f74` | `-` |
| `str` at 0xe7e5c | store | 4 | `sp -0xd8` | `0x0` |
| `str` at 0xe7e68 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0xe7e70 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xe7e74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7e78 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xe7e7c | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0xe7e8c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe7e94 | store | 4 | `sp -0x20` | `0x5a` |
| `ldr` at 0xe7e98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7e9c | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0xe7ea0 | store | 4 | `sp -0x30` | `[[sp-24]+48]` |
| `ldr` at 0xe7ea4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7ea8 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xe7eac | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0xe7ebc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe7ec4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xe7ec8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7ecc | load | 4 | `[sp-24] +0x34` | `-` |
| `str` at 0xe7ed0 | store | 4 | `sp -0x34` | `[[sp-24]+52]` |
| `ldr` at 0xe7ed4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe7ed8 | store | 4 | `sp -0xec` | `?` |
| `ldr` at 0xe7edc | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe7ee0 | store | 4 | `sp -0xe8` | `?` |
| `str` at 0xe7ee8 | store | 4 | `sp -0xe4` | `sp-56` |
| `str` at 0xe7ef0 | store | 4 | `sp -0xe0` | `sp-40` |
| `ldr` at 0xe7ef4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7ef8 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xe7efc | store | 4 | `sp -0xdc` | `[[sp-28]]` |
| `ldr` at 0xe7f00 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe7f04 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe7f08 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe7f20 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7f24 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe7f28 | load | 4 | `abs 0xe7f68` | `-` |
| `str` at 0xe7f34 | store | 4 | `sp -0xd8` | `0x0` |
| `ldr` at 0xe7f3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7f40 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe7f44 | load | 4 | `abs 0xe7f78` | `-` |
| `ldr` at 0xe7f48 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe7f4c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe7f58 | store | 4 | `sp -0xd8` | `0x0` |
| `ldr` at 0xe7f5c | load | 4 | `sp -0xd8` | `-` |
| `ldmdb` at 0xe7f60 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe7f60 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe7f60 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe7f60 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe7db8 | `(unknown callee) 0x1b124` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0xe7de0 | `(unknown callee) 0x1b2d4` | `[[sp-28]]` | `0x135ce8` | `?` | `[sp-28]` |
| `bl` at 0xe7e1c | `(unknown callee) 0x1b49c` | `[[sp-28]]` | `0x135d10` | `?` | `[sp-28]` |
| `bl` at 0xe7e54 | `(unknown callee) 0x1b49c` | `[[sp-28]]` | `0x135d44` | `[[sp-24]+44]` | `[sp-28]` |
| `bl` at 0xe7f10 | `(unknown callee) 0xdd4e4` | `?` | `[[sp-24]+40]` | `[[sp-24]+44]` | `0x0` |
| `bl` at 0xe7f2c | `(unknown callee) 0x1b2d4` | `[[sp-28]]` | `0x135ce8` | `?` | `[sp-28]` |
| `bl` at 0xe7f50 | `(unknown callee) 0x1b49c` | `[[sp-28]]` | `0x135d6c` | `[sp-56]` | `[sp-40]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe7dd0 | `[0x2ad258] != 0x0` | `[0x2ad258] == 0x0` |
| `bls` at 0xe7e0c | `[[sp-24]+40] <= 0xff (unsigned, 32-bit)` | `[[sp-24]+40] > 0xff (unsigned, 32-bit)` |
| `bls` at 0xe7e44 | `[[sp-24]+44] <= 0xffff (unsigned, 32-bit)` | `[[sp-24]+44] > 0xffff (unsigned, 32-bit)` |
| `beq` at 0xe7e88 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xe7eb8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xe7f1c | `ret(0xdd4e4) != 0x0` | `ret(0xdd4e4) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe7f60 | `r0` | `0x0` |

## `smpcmd_scan_port`

- `smpcmd_scan_port`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe8010 to 0xe8128.
  Established. The branch takes its bypass for `[[[sp-24]+112]] == 0x0` and falls through on nonzero
  (`beq` at 0xe8078). The unknown-target callback carries `[sp-32]`, `[sp-36]`, unknown,
  `[0x142b50]` (`mov` at 0xe80dc). A later call reaches `0x1b49c` with `[[sp-28]]`, `0x135d9c`,
  `[0x2ad268]`, `[0x2ad26c]` (`bl` at 0xe80fc). The exit reports r0 `0x0` (`ldmdb` at 0xe810c). Port
  routing and callback effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe8020 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe8024 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xe8028 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe8038 | store | 4 | `r1 +0x0` | `ret(0x1b124)` |
| `ldr` at 0xe803c | load | 4 | `abs 0xe8110` | `-` |
| `ldr` at 0xe8040 | load | 4 | `abs 0x2ad258` | `-` |
| `ldr` at 0xe804c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8050 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe8054 | load | 4 | `abs 0xe8114` | `-` |
| `str` at 0xe8060 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe8068 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe806c | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xe8070 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0xe807c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8080 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xe8084 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0xe8090 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8094 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe8098 | store | 4 | `sp -0x20` | `[[sp-24]+40]` |
| `ldr` at 0xe809c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe80a0 | load | 4 | `[sp-24] +0x2c` | `-` |
| `str` at 0xe80a4 | store | 4 | `sp -0x24` | `[[sp-24]+44]` |
| `ldr` at 0xe80ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe80b0 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe80b4 | store | 4 | `sp -0x24` | `[[sp-24]+40]` |
| `ldr` at 0xe80bc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe80c4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xe80c8 | load | 4 | `abs 0xe8118` | `-` |
| `ldr` at 0xe80cc | load | 4 | `abs 0x142b50` | `-` |
| `ldr` at 0xe80d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe80d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe80e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe80e4 | load | 4 | `abs 0xe811c` | `-` |
| `ldr` at 0xe80e8 | load | 4 | `abs 0xe8120` | `-` |
| `ldr` at 0xe80ec | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe80f0 | load | 4 | `abs 0xe8124` | `-` |
| `ldr` at 0xe80f4 | load | 4 | `abs 0x2ad268` | `-` |
| `ldr` at 0xe80f8 | load | 4 | `abs 0x2ad26c` | `-` |
| `str` at 0xe8104 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe8108 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xe810c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe810c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe810c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe810c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe8030 | `(unknown callee) 0x1b124` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0xe8058 | `(unknown callee) 0x1b508` | `[[sp-28]]` | `0x135ce8` | `?` | `[sp-28]` |
| `mov` at 0xe80dc | `(unknown callee) (unknown target)` | `[sp-32]` | `[sp-36]` | `?` | `[0x142b50]` |
| `bl` at 0xe80fc | `(unknown callee) 0x1b49c` | `[[sp-28]]` | `0x135d9c` | `[0x2ad268]` | `[0x2ad26c]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe8048 | `[0x2ad258] != 0x0` | `[0x2ad258] == 0x0` |
| `beq` at 0xe8078 | `[[[sp-24]+112]] == 0x0` | `[[[sp-24]+112]] != 0x0` |
| `bne` at 0xe808c | `[[[sp-24]+112]] != 0x2` | `[[[sp-24]+112]] == 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe810c | `r0` | `0x0` |
