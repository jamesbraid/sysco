# ARM command, protocol and scan helpers in 1.38.4

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
Computed selectors in smpcmd_mgn and smpcmd_tst remain unresolved. Tables and return values cover only known edges, with unresolved sites listed below that entry. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/command-protocol-scan-helpers-facts.txt) retain all tool sections.

## `SMPCmdResponseDelay`

- `SMPCmdResponseDelay`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8d5dc to 0x8d6a4.
  Established. The first call targets `0x6907c` with candidates `sp-36`, `sp-52`, entry r0 and
  `sp-36` (`bl` at 0x8d60c). The branch takes `[sp-24] == 0x0` and falls through on nonzero (`beq`
  at 0x8d618). A word store places `0x1` at `sp -0x3c` (`str` at 0x8d638). The role of the delay and
  its units remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8d5ec | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x8d5f0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x8d5f4 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x8d608 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8d610 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d61c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8d638 | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0x8d640 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d64c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x8d654 | store | 4 | `sp -0x3c` | `sp-52` |
| `ldr` at 0x8d658 | load | 4 | `abs 0x8d69c` | `-` |
| `ldr` at 0x8d65c | load | 4 | `abs 0x8d6a0` | `-` |
| `ldr` at 0x8d660 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x8d664 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x8d670 | store | 4 | `sp -0x20` | `ret(0xb6bdc)` |
| `ldr` at 0x8d674 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x8d684 | store | 4 | `sp -0x38` | `?` |
| `str` at 0x8d690 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x8d694 | load | 4 | `sp -0x38` | `-` |
| `ldmdb` at 0x8d698 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8d698 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8d698 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8d60c | `(unknown callee) 0x6907c` | `sp-36` | `sp-52` | `r0` | `sp-36` |
| `bl` at 0x8d620 | `(unknown callee) 0x1b22c` | `[sp-24]` | `?` | `?` | `[sp-24]` |
| `bl` at 0x8d648 | `(unknown callee) 0x692f8` | `sp-36` | `sp-52` | `[sp-24]` | `ret(0x1b22c)` |
| `bl` at 0x8d668 | `(unknown callee) 0xb6bdc` | `0x2b1820` | `0xfffffd95` | `[[sp-28]]` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x8d618 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0x8d67c | `ret(0xb6bdc) == 0x0` | `ret(0xb6bdc) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8d698 | `r0` | `[sp-56]` |

## `SMPCmdNoResponse`

- `SMPCmdNoResponse`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8db5c to 0x8dba8.
  Established. A call to `0x1aff8` passes entry r0, `0x114e94`, entry r2 and entry r3 (`bl` at
  0x8db78). Nonzero takes the branch and zero falls through (`bne` at 0x8db84). The taken arm writes
  `0x0` to `sp -0x18` (`str` at 0x8db98). The fall-through arm writes `0x1` to the same target
  (`str` at 0x8db8c). This establishes
  a local result selection, while the callee name and external meaning remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8db6c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x8db70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8db74 | load | 4 | `abs 0x8dba4` | `-` |
| `str` at 0x8db8c | store | 4 | `sp -0x18` | `0x1` |
| `str` at 0x8db98 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x8db9c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x8dba0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8dba0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8dba0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8db78 | `(unknown callee) 0x1aff8` | `r0` | `0x114e94` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x8db84 | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8dba0 | `r0` | `[sp-24]` |

## `SMPCmdPostProcess`

- `SMPCmdPostProcess`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8dba8 to 0x8dc68.
  Established. A word read uses absolute address `0x149990` (`ldr` at 0x8dbec); its zero value takes
  the branch and nonzero falls through (`beq` at 0x8dbf4). Later word operations read `0x1499a8` and
  store `[0x1499a8]` at `[sp-24] +0x410` (`ldr` at 0x8dc38, `str` at 0x8dc3c). The exit value is
  unknown (`ldmdb` at 0x8dc4c), as is the purpose of the copied field.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8dbb8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x8dbbc | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x8dbc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8dbcc | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0x8dbd0 | load | 4 | `abs 0x8dc50` | `-` |
| `ldr` at 0x8dbe8 | load | 4 | `abs 0x8dc54` | `-` |
| `ldr` at 0x8dbec | load | 4 | `abs 0x149990` | `-` |
| `ldr` at 0x8dc00 | load | 4 | `abs 0x8dc58` | `-` |
| `ldr` at 0x8dc1c | load | 4 | `abs 0x8dc5c` | `-` |
| `ldr` at 0x8dc30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8dc34 | load | 4 | `abs 0x8dc60` | `-` |
| `ldr` at 0x8dc38 | load | 4 | `abs 0x1499a8` | `-` |
| `str` at 0x8dc3c | store | 4 | `[sp-24] +0x410` | `[0x1499a8]` |
| `ldr` at 0x8dc40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8dc44 | load | 4 | `abs 0x8dc64` | `-` |
| `str` at 0x8dc48 | store | 4 | `[sp-20] +0x0` | `0x11aa58` |
| `ldmdb` at 0x8dc4c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8dc4c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8dc4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8dbd8 | `(unknown callee) 0x1b3e8` | `[r0]` | `0x11aa40` | `sp-56` | `sp-88` |
| `bl` at 0x8dc04 | `(unknown callee) 0x1aff8` | `sp-56` | `0x11aa4c` | `?` | `sp-56` |
| `bl` at 0x8dc20 | `(unknown callee) 0x1aff8` | `sp-88` | `0x11aa50` | `?` | `sp-88` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x8dbe4 | `ret(0x1b3e8) != 0x2` | `ret(0x1b3e8) == 0x2` |
| `beq` at 0x8dbf4 | `[0x149990] == 0x0` | `[0x149990] != 0x0` |
| `bne` at 0x8dc10 | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |
| `bne` at 0x8dc2c | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8dc4c | `r0` | `?` |

## `smpcmd_mgn`

- `smpcmd_mgn`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x95800 to 0x95d0c.
  Established. Two stack word stores initialise `sp -0x20` and `sp -0x24` to `0x0` (`str` at
  0x9581c, `str` at 0x95824). The call to `0x3c80c` has candidates `0x0`, `sp-40`, unknown and
  `sp-40` (`bl` at 0x9586c). A nonzero result takes the branch and zero falls through (`bne` at
  0x9587c). The unresolved selector limits the recorded effects to known edges (`ldrls` at 0x959f8).
  The physical meaning of the command remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x95810 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x95814 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x9581c | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x95824 | store | 4 | `sp -0x24` | `0x0` |
| `strb` at 0x95830 | store | 1 | `sp -0x36` | `(ret(0x4f090) & 0xff)` |
| `ldr` at 0x95834 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x95838 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0x9583c | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0x95840 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x95858 | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0x95870 | store | 4 | `sp -0x2c` | `ret(0x3c80c)` |
| `ldr` at 0x95874 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x95880 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x95890 | store | 4 | `[sp-28] +0x0` | `ret(0x1b124)` |
| `ldr` at 0x95894 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x95898 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x9589c | load | 4 | `abs 0x95cf4` | `-` |
| `str` at 0x958a8 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x958b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x958b8 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0x958c0 | load | 4 | `[[sp-24]+112] +0x8` | `-` |
| `ldr` at 0x958cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x958d4 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0x958dc | load | 4 | `[[sp-24]+112] +0x8` | `-` |
| `str` at 0x958e0 | store | 4 | `sp -0x24` | `[[[sp-24]+112]+8]` |
| `str` at 0x958e8 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0x958f4 | store | 4 | `sp -0x44` | `0x0` |
| `str` at 0x958fc | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x95900 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x95904 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x95914 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x95928 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x95930 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x95940 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x9594c | store | 4 | `sp -0x48` | `[sp-72]+1` |
| `ldr` at 0x95950 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x95954 | load | 4 | `[sp-24] +0x28` | `-` |
| `ldr` at 0x95960 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x95964 | store | 4 | `sp -0x24` | `[sp-68]` |
| `ldr` at 0x9596c | load | 4 | `sp -0x44` | `-` |
| `str` at 0x95974 | store | 4 | `sp -0x44` | `[sp-68]+1` |
| `ldr` at 0x9597c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x95980 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9598c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x9599c | store | 4 | `[sp-28] +0x0` | `ret(0x1b124)` |
| `ldr` at 0x959a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x959a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x959a8 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x959ac | load | 4 | `abs 0x95cf8` | `-` |
| `ldr` at 0x959b0 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0x959bc | store | 4 | `sp -0x4c` | `?` |
| `str` at 0x959c8 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0x959d0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x959dc | store | 4 | `sp -0x34` | `ret(0x3fff0)` |
| `ldr` at 0x959e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x959e8 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0x959f0 | load | 4 | `[[sp-24]+112] +0x4` | `-` |
| `ldrls` at 0x959f8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x95c84 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x95cdc | load | 4 | `sp -0x4c` | `-` |
| `ldmdb` at 0x95ce0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x95ce0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x95ce0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x95ce0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x95828 | `(unknown callee) 0x4f090` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x95844 | `(unknown callee) 0x8d2c8` | `[[[sp-24]+112]]` | `[sp-28]` | `?` | `[[sp-24]+112]` |
| `bl` at 0x9586c | `(unknown callee) 0x3c80c` | `0x0` | `sp-40` | `?` | `sp-40` |
| `bl` at 0x95888 | `(unknown callee) 0x1b124` | `0x50` | `?` | `?` | `ret(0x3c80c)` |
| `bl` at 0x958a0 | `(unknown callee) 0x1b508` | `[[sp-28]]` | `0x11c588` | `?` | `[sp-28]` |
| `bl` at 0x95994 | `(unknown callee) 0x1b124` | `0x50` | `?` | `[sp-68]` | `[sp-44]` |
| `bl` at 0x959b4 | `(unknown callee) 0x1b49c` | `[[sp-28]]` | `0x11c874` | `[[sp-24]+40]` | `[sp-28]` |
| `bl` at 0x959d4 | `(unknown callee) 0x3fff0` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x95850 | `ret(0x8d2c8) == 0x0` | `ret(0x8d2c8) != 0x0` |
| `bne` at 0x9587c | `ret(0x3c80c) != 0x0` | `ret(0x3c80c) == 0x0` |
| `bne` at 0x958c8 | `[[[sp-24]+112]+8] != 0x21` | `[[[sp-24]+112]+8] == 0x21` |
| `blo` at 0x9590c | `[sp-68] < [sp-44] (unsigned, 32-bit)` | `[sp-68] >= [sp-44] (unsigned, 32-bit)` |
| `bne` at 0x95938 | `? != 0xff` | `? == 0xff` |
| `bne` at 0x9595c | `[sp-72]+1 != [[sp-24]+40]` | `[sp-72]+1 == [[sp-24]+40]` |
| `bne` at 0x95988 | `[sp-68] != [sp-44]` | `[sp-68] == [sp-44]` |
| `ldrls` at 0x959f8 | `[[[sp-24]+112]+4] <= 0x3 (unsigned, 32-bit)` | `[[[sp-24]+112]+4] > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x95ce0 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x959f8).

## `SMPCmdInit`

- `SMPCmdInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x98c1c to 0x98eb4.
  Established. The initial word load reads `0x1497e0` (`ldr` at 0x98c30). Zero takes the branch and
  nonzero falls through (`beq` at 0x98c38). The call to `0xb553c` carries `0x2b1820`, `0x3`, `0x0`
  and unknown (`bl` at 0x98c64). The return records `ret(0xbb764)` (`ldmdb` at 0x98e48). These
  effects do not establish which external resource is initialised, so that role remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x98c2c | load | 4 | `abs 0x98e4c` | `-` |
| `ldr` at 0x98c30 | load | 4 | `abs 0x1497e0` | `-` |
| `ldr` at 0x98c3c | load | 4 | `abs 0x98e50` | `-` |
| `str` at 0x98c40 | store | 4 | `sp -0x2c` | `0x11d860` |
| `ldr` at 0x98c48 | load | 4 | `abs 0x98e54` | `-` |
| `ldr` at 0x98c4c | load | 4 | `abs 0x98e58` | `-` |
| `ldr` at 0x98c58 | load | 4 | `abs 0x98e5c` | `-` |
| `ldr` at 0x98c68 | load | 4 | `abs 0x98e60` | `-` |
| `ldr` at 0x98c6c | load | 4 | `abs 0x98e64` | `-` |
| `str` at 0x98c7c | store | 4 | `sp -0x14` | `ret(0xb5654)` |
| `ldr` at 0x98c80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x98c8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x98c98 | load | 4 | `abs 0x98e68` | `-` |
| `str` at 0x98c9c | store | 4 | `sp -0x2c` | `0x11d8a8` |
| `str` at 0x98ca0 | store | 4 | `sp -0x28` | `ret(0x21674)` |
| `ldr` at 0x98ca8 | load | 4 | `abs 0x98e54` | `-` |
| `ldr` at 0x98cac | load | 4 | `abs 0x98e6c` | `-` |
| `ldr` at 0x98cb8 | load | 4 | `abs 0x98e5c` | `-` |
| `ldr` at 0x98cbc | load | 4 | `abs 0x98e70` | `-` |
| `ldr` at 0x98cc4 | load | 4 | `abs 0x98e60` | `-` |
| `str` at 0x98cd0 | store | 4 | `sp -0x14` | `ret(0xb4220)` |
| `ldr` at 0x98cd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x98ce0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x98cec | load | 4 | `abs 0x98e74` | `-` |
| `str` at 0x98cf0 | store | 4 | `sp -0x2c` | `0x11d8cc` |
| `str` at 0x98cf4 | store | 4 | `sp -0x28` | `ret(0x21674)` |
| `ldr` at 0x98cfc | load | 4 | `abs 0x98e54` | `-` |
| `ldr` at 0x98d00 | load | 4 | `abs 0x98e78` | `-` |
| `ldr` at 0x98d0c | load | 4 | `abs 0x98e7c` | `-` |
| `ldr` at 0x98d10 | load | 4 | `abs 0x98e80` | `-` |
| `str` at 0x98d20 | store | 4 | `sp -0x14` | `ret(0xb5654)` |
| `ldr` at 0x98d24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x98d30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x98d3c | load | 4 | `abs 0x98e84` | `-` |
| `str` at 0x98d40 | store | 4 | `sp -0x2c` | `0x11d8fc` |
| `str` at 0x98d44 | store | 4 | `sp -0x28` | `ret(0x21674)` |
| `ldr` at 0x98d4c | load | 4 | `abs 0x98e54` | `-` |
| `ldr` at 0x98d50 | load | 4 | `abs 0x98e88` | `-` |
| `ldr` at 0x98d5c | load | 4 | `abs 0x98e5c` | `-` |
| `ldr` at 0x98d60 | load | 4 | `abs 0x98e70` | `-` |
| `ldr` at 0x98d68 | load | 4 | `abs 0x98e7c` | `-` |
| `str` at 0x98d74 | store | 4 | `sp -0x14` | `ret(0xb4220)` |
| `ldr` at 0x98d78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x98d84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x98d90 | load | 4 | `abs 0x98e8c` | `-` |
| `str` at 0x98d94 | store | 4 | `sp -0x2c` | `0x11d920` |
| `str` at 0x98d98 | store | 4 | `sp -0x28` | `ret(0x21674)` |
| `ldr` at 0x98da0 | load | 4 | `abs 0x98e54` | `-` |
| `ldr` at 0x98da4 | load | 4 | `abs 0x98e90` | `-` |
| `ldr` at 0x98db0 | load | 4 | `abs 0x98e94` | `-` |
| `ldr` at 0x98db4 | load | 4 | `abs 0x98e98` | `-` |
| `ldr` at 0x98dcc | load | 4 | `abs 0x98e9c` | `-` |
| `str` at 0x98ddc | store | 4 | `sp -0x18` | `0x1800` |
| `str` at 0x98de8 | store | 4 | `sp -0x18` | `0x1000` |
| `str` at 0x98df0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x98df4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x98df8 | store | 4 | `sp -0x28` | `[sp-24]` |
| `str` at 0x98e00 | store | 4 | `sp -0x24` | `0x14` |
| `str` at 0x98e08 | store | 4 | `sp -0x20` | `0xa` |
| `str` at 0x98e10 | store | 4 | `sp -0x1c` | `0x9` |
| `ldr` at 0x98e14 | load | 4 | `abs 0x98ea0` | `-` |
| `ldr` at 0x98e18 | load | 4 | `abs 0x98ea4` | `-` |
| `ldr` at 0x98e1c | load | 4 | `abs 0x98ea8` | `-` |
| `ldr` at 0x98e28 | load | 4 | `abs 0x98eac` | `-` |
| `ldr` at 0x98e2c | load | 4 | `abs 0x98eb0` | `-` |
| `ldr` at 0x98e34 | load | 4 | `abs 0x98e4c` | `-` |
| `ldr` at 0x98e38 | load | 4 | `abs 0x98e4c` | `-` |
| `ldr` at 0x98e3c | load | 4 | `abs 0x1497e0` | `-` |
| `str` at 0x98e44 | store | 4 | `abs 0x1497e0` | `[0x1497e0]+1` |
| `ldmdb` at 0x98e48 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x98e48 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x98e48 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x98c54 | `(unknown callee) 0x21e28` | `0x9` | `0x11d884` | `0x17a3` | `0x0` |
| `bl` at 0x98c64 | `(unknown callee) 0xb553c` | `0x2b1820` | `0x3` | `0x0` | `?` |
| `bl` at 0x98c74 | `(unknown callee) 0xb5654` | `0x2b1828` | `0x11d89c` | `0xa` | `?` |
| `bl` at 0x98c90 | `(unknown callee) 0x21674` | `ret(0xb5654)` | `?` | `?` | `ret(0xb5654)` |
| `bl` at 0x98cb4 | `(unknown callee) 0x21e28` | `0x9` | `0x11d884` | `0x17b0` | `0x0` |
| `bl` at 0x98cc8 | `(unknown callee) 0xb4220` | `0x2b1820` | `0xffff003` | `0x1` | `0x2b1828` |
| `bl` at 0x98ce4 | `(unknown callee) 0x21674` | `ret(0xb4220)` | `?` | `?` | `ret(0xb4220)` |
| `bl` at 0x98d08 | `(unknown callee) 0x21e28` | `0x9` | `0x11d884` | `0x17b8` | `0x0` |
| `bl` at 0x98d18 | `(unknown callee) 0xb5654` | `0x2b18c0` | `0x11d8f0` | `0xa` | `?` |
| `bl` at 0x98d34 | `(unknown callee) 0x21674` | `ret(0xb5654)` | `?` | `?` | `ret(0xb5654)` |
| `bl` at 0x98d58 | `(unknown callee) 0x21e28` | `0x9` | `0x11d884` | `0x17bf` | `0x0` |
| `bl` at 0x98d6c | `(unknown callee) 0xb4220` | `0x2b1820` | `0xffff003` | `0x2` | `0x2b18c0` |
| `bl` at 0x98d88 | `(unknown callee) 0x21674` | `ret(0xb4220)` | `?` | `?` | `ret(0xb4220)` |
| `bl` at 0x98dac | `(unknown callee) 0x21e28` | `0x9` | `0x11d884` | `0x17c7` | `0x0` |
| `bl` at 0x98dc0 | `(unknown callee) 0x22920` | `0x2b195c` | `0x11d948` | `0x1` | `0x9` |
| `bl` at 0x98dc4 | `(unknown callee) 0x9a500` | `ret(0x22920)` | `?` | `?` | `?` |
| `bl` at 0x98e24 | `(unknown callee) 0x223f0` | `0x2b1958` | `0x11d954` | `0x9a1b8` | `0x0` |
| `bl` at 0x98e30 | `(unknown callee) 0xbb764` | `0x115770` | `0x131` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x98c38 | `[0x1497e0] == 0x0` | `[0x1497e0] != 0x0` |
| `beq` at 0x98c88 | `ret(0xb5654) == 0x0` | `ret(0xb5654) != 0x0` |
| `beq` at 0x98cdc | `ret(0xb4220) == 0x0` | `ret(0xb4220) != 0x0` |
| `beq` at 0x98d2c | `ret(0xb5654) == 0x0` | `ret(0xb5654) != 0x0` |
| `beq` at 0x98d80 | `ret(0xb4220) == 0x0` | `ret(0xb4220) != 0x0` |
| `bls` at 0x98dd4 | `ret(0x9a500) <= 0x7ffff (unsigned, 32-bit)` | `ret(0x9a500) > 0x7ffff (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x98e48 | `r0` | `ret(0xbb764)` |

## `SMPCmdLibrary`

- `SMPCmdLibrary`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x98eb4 to 0x9a1b8.
  Established. After saving entry registers, this body clears stack words at `sp -0x30`, `sp
  -0xa6c`, `sp -0xa70` and `sp -0xa74` (`str` at 0x98ed8, `str` at 0x98ee0, `str` at 0x98ee8, `str`
  at 0x98ef0). A call to `0x1b124` has candidates `0x8`, unknown, unknown and unknown (`bl` at
  0x98f44). The first recorded decision takes a nonzero `ret(0xbb804)` and falls through on zero
  (`bne` at 0x98fd0); its role remains unknown beyond the accesses and calls below.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x98ec8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x98ecc | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x98ed0 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x98ed8 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0x98ee0 | store | 4 | `sp -0xa6c` | `0x0` |
| `str` at 0x98ee8 | store | 4 | `sp -0xa70` | `0x0` |
| `str` at 0x98ef0 | store | 4 | `sp -0xa74` | `0x0` |
| `str` at 0x98f00 | store | 4 | `sp -0xa88` | `ret(0x74908)` |
| `ldr` at 0x98f04 | load | 4 | `sp -0xa88` | `-` |
| `ldr` at 0x98f08 | load | 4 | `ret(0x74908) +0x8` | `-` |
| `ldr` at 0x98f18 | load | 4 | `sp -0xa88` | `-` |
| `ldr` at 0x98f1c | load | 4 | `ret(0x74908) +0xc` | `-` |
| `str` at 0x98f38 | store | 4 | `sp -0xa8c` | `?` |
| `ldr` at 0x98f3c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x98f4c | store | 4 | `[sp-28] +0x0` | `ret(0x1b124)` |
| `ldr` at 0x98f50 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x98f54 | store | 4 | `sp -0xa78` | `[[sp-28]]` |
| `ldr` at 0x98f58 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x98f60 | store | 4 | `[[sp-28]] +0x0` | `0x0` |
| `ldr` at 0x98f68 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x98f74 | store | 4 | `[sp-2680] +0x4` | `0x0` |
| `ldr` at 0x98f88 | load | 4 | `abs 0x99e2c` | `-` |
| `ldr` at 0x98f90 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x98f94 | store | 4 | `sp -0xa68` | `[sp-32]` |
| `str` at 0x98f9c | store | 4 | `sp -0xac8` | `0x0` |
| `str` at 0x98fa8 | store | 4 | `sp -0xac4` | `sp-2664` |
| `ldr` at 0x98fb0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x98fb4 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x98fc4 | store | 4 | `sp -0x2c` | `ret(0xbb804)` |
| `ldr` at 0x98fc8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x98fd4 | load | 4 | `sp -0xa60` | `-` |
| `ldr` at 0x98fe0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x98fe4 | store | 4 | `sp -0xa98` | `ret(0xbb804)` |
| `ldr` at 0x98fec | load | 4 | `abs 0x99e80` | `-` |
| `str` at 0x98ffc | store | 4 | `sp -0x24` | `ret(0x22a38)` |
| `ldr` at 0x99000 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9900c | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99010 | load | 4 | `[sp-2680] +0x0` | `-` |
| `ldr` at 0x99014 | load | 4 | `abs 0x99e30` | `-` |
| `str` at 0x99020 | store | 4 | `sp -0xa98` | `?` |
| `ldr` at 0x99028 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99030 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x99034 | load | 4 | `sp -0xa60` | `-` |
| `str` at 0x9904c | store | 4 | `[sp-28] +0x0` | `ret(0x1b124)` |
| `ldr` at 0x99050 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x99054 | store | 4 | `sp -0xa78` | `[[sp-28]]` |
| `str` at 0x9905c | store | 4 | `sp -0xa7c` | `0x0` |
| `ldr` at 0x99060 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x99068 | store | 4 | `[[sp-28]] +0x0` | `0x0` |
| `ldr` at 0x9906c | load | 4 | `sp -0xa60` | `-` |
| `str` at 0x99074 | store | 4 | `sp -0x450` | `[sp-2656]-1` |
| `ldr` at 0x99078 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x99088 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x9908c | load | 4 | `abs 0x99e34` | `-` |
| `ldr` at 0x990a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x990ac | load | 4 | `abs 0x99e38` | `-` |
| `str` at 0x990b0 | store | 4 | `sp -0xa90` | `0xffff003` |
| `ldr` at 0x990b4 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x990b8 | load | 4 | `abs 0x99e74` | `-` |
| `ldr` at 0x990cc | load | 4 | `(unknown)` | `-` |
| `str` at 0x990d0 | store | 4 | `sp -0xa94` | `?` |
| `ldr` at 0x990d4 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x990d8 | load | 4 | `abs 0x99e3c` | `-` |
| `ldr` at 0x990ec | load | 4 | `abs 0x99e40` | `-` |
| `ldrh` at 0x990f0 | load | 2 | `abs 0x2b1826` | `-` |
| `str` at 0x990f4 | store | 4 | `(unknown)` | `[0x2b1826]:2` |
| `ldr` at 0x990f8 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x990fc | load | 4 | `abs 0x99e44` | `-` |
| `str` at 0x99114 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x99130 | load | 4 | `sp -0xa64` | `-` |
| `str` at 0x9914c | store | 4 | `sp -0xac8` | `0x1` |
| `ldr` at 0x99154 | load | 4 | `sp -0xa64` | `-` |
| `str` at 0x9916c | store | 4 | `sp -0xac8` | `0x2` |
| `ldr` at 0x99170 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x99174 | store | 4 | `sp -0xac4` | `[sp-76]` |
| `str` at 0x9917c | store | 4 | `sp -0xac0` | `sp-72` |
| `ldr` at 0x99180 | load | 4 | `abs 0x99e40` | `-` |
| `str` at 0x99198 | store | 4 | `sp -0x30` | `ret(0xb6f64)` |
| `ldr` at 0x9919c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x991ac | load | 4 | `sp -0xa94` | `-` |
| `ldr` at 0x991c0 | load | 4 | `abs 0x99e48` | `-` |
| `ldr` at 0x991c4 | load | 4 | `sp -0xa94` | `-` |
| `ldr` at 0x991d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99200 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x99208 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99210 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x99224 | store | 4 | `(unknown)` | `ret(0x1b124)` |
| `ldr` at 0x99228 | load | 4 | `abs 0x99e40` | `-` |
| `ldr` at 0x9922c | load | 4 | `abs 0x2b1820` | `-` |
| `ldr` at 0x99238 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99240 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99248 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99250 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99258 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99260 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99264 | load | 4 | `abs 0x99e4c` | `-` |
| `ldr` at 0x9926c | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99274 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x9927c | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99284 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x9928c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9929c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x992a4 | load | 4 | `sp -0xa94` | `-` |
| `ldr` at 0x992bc | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x992c4 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x992cc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x992d0 | load | 4 | `abs 0x99e78` | `-` |
| `ldr` at 0x992d8 | load | 4 | `sp -0xa7c` | `-` |
| `str` at 0x992e0 | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x992e8 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x992f4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x992f8 | load | 4 | `sp -0xa6c` | `-` |
| `str` at 0x99300 | store | 4 | `sp -0xa6c` | `[sp-2668]+1` |
| `ldr` at 0x99304 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x99308 | load | 4 | `abs 0x99e44` | `-` |
| `str` at 0x99320 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x99328 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x9932c | load | 4 | `abs 0x99e74` | `-` |
| `ldr` at 0x99340 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99350 | load | 4 | `abs 0x99e40` | `-` |
| `ldr` at 0x99354 | load | 4 | `abs 0x2b1820` | `-` |
| `str` at 0x99370 | store | 4 | `sp -0xa94` | `0x0` |
| `str` at 0x9937c | store | 4 | `sp -0xac8` | `0x0` |
| `str` at 0x99388 | store | 4 | `sp -0xac4` | `sp-2664` |
| `ldr` at 0x99390 | load | 4 | `sp -0xa64` | `-` |
| `str` at 0x993a0 | store | 4 | `sp -0x2c` | `ret(0xbb804)` |
| `ldr` at 0x993a4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x993b0 | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x993b8 | store | 4 | `sp -0xa74` | `[sp-2676]+1` |
| `ldr` at 0x993bc | load | 4 | `sp -0xa94` | `-` |
| `ldr` at 0x993c8 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x993d0 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x993e4 | store | 4 | `(unknown)` | `ret(0x1b124)` |
| `ldr` at 0x993e8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x993f4 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x993f8 | load | 4 | `abs 0x99e74` | `-` |
| `ldr` at 0x9940c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99418 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99420 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99428 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99430 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99438 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99440 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99444 | load | 4 | `abs 0x99e50` | `-` |
| `ldr` at 0x9944c | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99454 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x9945c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99460 | load | 4 | `sp -0xa94` | `-` |
| `ldr` at 0x99468 | load | 4 | `sp -0xa7c` | `-` |
| `str` at 0x99470 | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x99478 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x99484 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x9948c | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x99490 | load | 4 | `abs 0x99e74` | `-` |
| `ldr` at 0x994a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x994b0 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x994b8 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x994c0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x994d0 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x994dc | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x994e8 | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x994ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x994f0 | load | 4 | `abs 0x99e54` | `-` |
| `ldr` at 0x994f8 | load | 4 | `sp -0xa94` | `-` |
| `ldr` at 0x99508 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99510 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x99518 | store | 4 | `(unknown)` | `ret(0x1afa4)` |
| `ldr` at 0x9951c | load | 4 | `sp -0xa7c` | `-` |
| `str` at 0x99524 | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x9952c | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x99538 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x9953c | load | 4 | `sp -0xa94` | `-` |
| `ldr` at 0x99544 | load | 4 | `sp -0xa6c` | `-` |
| `str` at 0x9954c | store | 4 | `sp -0xa6c` | `[sp-2668]+1` |
| `ldr` at 0x99550 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x99554 | load | 4 | `abs 0x99e44` | `-` |
| `str` at 0x9956c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x99574 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x99578 | load | 4 | `abs 0x99e44` | `-` |
| `str` at 0x99590 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x995ac | load | 4 | `sp -0xa64` | `-` |
| `str` at 0x995c8 | store | 4 | `sp -0xac8` | `0x1` |
| `ldr` at 0x995d0 | load | 4 | `sp -0xa64` | `-` |
| `ldr` at 0x995e4 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x995fc | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x99600 | store | 4 | `sp -0xac8` | `[sp-76]` |
| `str` at 0x99608 | store | 4 | `sp -0xac4` | `sp-72` |
| `ldr` at 0x99614 | load | 4 | `sp -0x450` | `-` |
| `str` at 0x99620 | store | 4 | `sp -0xac0` | `?` |
| `str` at 0x99628 | store | 4 | `sp -0xabc` | `0x0` |
| `ldr` at 0x9962c | load | 4 | `abs 0x99e40` | `-` |
| `str` at 0x99640 | store | 4 | `sp -0x24` | `ret(0xb6a9c)` |
| `ldr` at 0x99644 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x99654 | load | 4 | `sp -0x65c` | `-` |
| `ldr` at 0x99660 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x99664 | load | 4 | `abs 0x99e44` | `-` |
| `str` at 0x9967c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x99680 | load | 4 | `sp -0xa6c` | `-` |
| `str` at 0x99688 | store | 4 | `sp -0xa6c` | `[sp-2668]+1` |
| `ldr` at 0x99690 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99698 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x996ac | store | 4 | `(unknown)` | `ret(0x1b124)` |
| `ldr` at 0x996b0 | load | 4 | `abs 0x99e40` | `-` |
| `ldr` at 0x996b4 | load | 4 | `abs 0x2b1820` | `-` |
| `ldr` at 0x996c0 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x996c8 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x996d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x996d8 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x996e0 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x996e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x996ec | load | 4 | `abs 0x99e58` | `-` |
| `ldr` at 0x996f4 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x996fc | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99704 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x9970c | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99714 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99724 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9972c | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x99730 | load | 4 | `abs 0x99e74` | `-` |
| `ldr` at 0x99744 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9975c | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99764 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x9976c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99770 | load | 4 | `abs 0x99e78` | `-` |
| `ldr` at 0x99778 | load | 4 | `sp -0xa7c` | `-` |
| `str` at 0x99780 | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x99788 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x99794 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x99798 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x9979c | load | 4 | `abs 0x99e44` | `-` |
| `str` at 0x997b4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x997b8 | load | 4 | `sp -0xa6c` | `-` |
| `str` at 0x997c0 | store | 4 | `sp -0xa6c` | `[sp-2668]+1` |
| `ldr` at 0x997c4 | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x997cc | store | 4 | `sp -0xa74` | `[sp-2676]+1` |
| `ldr` at 0x997d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x997dc | load | 4 | `abs 0x99e5c` | `-` |
| `ldr` at 0x997e8 | load | 4 | `sp -0x450` | `-` |
| `str` at 0x997f0 | store | 4 | `sp -0x450` | `[sp-1104]-1` |
| `ldr` at 0x997f8 | load | 4 | `sp -0xa64` | `-` |
| `ldr` at 0x99800 | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x99804 | store | 4 | `sp -0x28` | `0x44c` |
| `ldr` at 0x99808 | load | 4 | `sp -0xa6c` | `-` |
| `ldr` at 0x9980c | load | 4 | `sp -0xa60` | `-` |
| `str` at 0x99820 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x9982c | load | 4 | `abs 0x99e64` | `-` |
| `ldr` at 0x99830 | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x9983c | store | 4 | `sp -0x24` | `ret(0xb5698)` |
| `ldr` at 0x99840 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x99844 | store | 4 | `sp -0xaa0` | `ret(0xb5698)` |
| `ldr` at 0x99848 | load | 4 | `sp -0xaa0` | `-` |
| `ldr` at 0x99854 | load | 4 | `sp -0xaa0` | `-` |
| `ldr` at 0x99860 | load | 4 | `sp -0xaa0` | `-` |
| `ldr` at 0x99870 | load | 4 | `sp -0xaa0` | `-` |
| `str` at 0x99884 | store | 4 | `sp -0x450` | `0x0` |
| `ldr` at 0x99888 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x9988c | load | 4 | `sp -0xa60` | `-` |
| `ldr` at 0x9989c | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x998a0 | load | 4 | `abs 0x99e44` | `-` |
| `ldr` at 0x998b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x998c0 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x998c8 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x998dc | store | 4 | `(unknown)` | `ret(0x1b124)` |
| `ldr` at 0x998e0 | load | 4 | `abs 0x99e40` | `-` |
| `ldr` at 0x998e4 | load | 4 | `abs 0x2b1820` | `-` |
| `ldr` at 0x998f0 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x998f8 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99900 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99908 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99910 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99918 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9991c | load | 4 | `abs 0x99e68` | `-` |
| `ldr` at 0x99924 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x9992c | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99934 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x9993c | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99944 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99954 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9995c | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x99960 | load | 4 | `abs 0x99e74` | `-` |
| `ldr` at 0x99974 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9998c | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99994 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x9999c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x999a0 | load | 4 | `abs 0x99e78` | `-` |
| `ldr` at 0x999a8 | load | 4 | `sp -0xa7c` | `-` |
| `str` at 0x999b0 | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x999b8 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x999c4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x999c8 | load | 4 | `sp -0x450` | `-` |
| `str` at 0x999d0 | store | 4 | `sp -0x450` | `?` |
| `ldr` at 0x999d8 | load | 4 | `sp -0xa60` | `-` |
| `str` at 0x999dc | store | 4 | `sp -0xa6c` | `[sp-2656]` |
| `ldr` at 0x999e4 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x999e8 | load | 1 | `[sp-48] +0x13` | `-` |
| `ldr` at 0x999f4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x99a00 | store | 4 | `sp -0xa94` | `ret(0xb7944)` |
| `ldr` at 0x99a04 | load | 4 | `sp -0xa94` | `-` |
| `ldr` at 0x99a08 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x99a14 | load | 4 | `sp -0xa94` | `-` |
| `str` at 0x99a18 | store | 4 | `sp -0x28` | `ret(0xb7944)` |
| `ldr` at 0x99a1c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x99a28 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x99a2c | load | 1 | `[sp-48] +0x12` | `-` |
| `str` at 0x99a30 | store | 4 | `sp -0xa80` | `[[sp-48]+18]:1` |
| `ldr` at 0x99a34 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x99a38 | load | 1 | `[sp-48] +0x14` | `-` |
| `ldr` at 0x99a3c | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x99a40 | load | 1 | `[sp-48] +0x21` | `-` |
| `str` at 0x99a4c | store | 4 | `sp -0xa84` | `?` |
| `str` at 0x99a54 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x99a5c | store | 4 | `sp -0x450` | `0x0` |
| `ldr` at 0x99a60 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x99a6c | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x99a70 | load | 4 | `sp -0xa60` | `-` |
| `ldr` at 0x99a80 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x99a84 | load | 4 | `abs 0x99e3c` | `-` |
| `ldr` at 0x99a98 | load | 4 | `(unknown)` | `-` |
| `str` at 0x99a9c | store | 4 | `sp -0xa9c` | `?` |
| `ldr` at 0x99aa0 | load | 4 | `sp -0xa80` | `-` |
| `ldr` at 0x99aac | load | 4 | `sp -0xa84` | `-` |
| `ldr` at 0x99ab4 | load | 4 | `sp -0xa9c` | `-` |
| `ldr` at 0x99ac4 | load | 4 | `sp -0xa84` | `-` |
| `ldr` at 0x99ad0 | load | 4 | `sp -0xa9c` | `-` |
| `ldr` at 0x99ae0 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x99ae4 | load | 4 | `abs 0x99e44` | `-` |
| `ldr` at 0x99af8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99b04 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x99b08 | load | 4 | `abs 0x99e44` | `-` |
| `str` at 0x99b20 | store | 4 | `(unknown)` | `0x0` |
| `str` at 0x99b28 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x99b2c | load | 4 | `sp -0x450` | `-` |
| `str` at 0x99b34 | store | 4 | `sp -0x450` | `?` |
| `ldr` at 0x99b3c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x99b48 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x99b54 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x99b5c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x99b68 | load | 4 | `abs 0x99e6c` | `-` |
| `ldr` at 0x99b70 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x99b78 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x99b84 | load | 4 | `sp -0x658` | `-` |
| `ldr` at 0x99b90 | load | 4 | `sp -0x654` | `-` |
| `ldr` at 0x99b94 | load | 4 | `abs 0x99e7c` | `-` |
| `ldr` at 0x99ba8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x99bac | store | 4 | `(unknown)` | `[sp-48]` |
| `ldr` at 0x99bb0 | load | 4 | `sp -0x654` | `-` |
| `str` at 0x99bb8 | store | 4 | `sp -0x654` | `[sp-1620]+1` |
| `ldr` at 0x99bbc | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x99bc0 | load | 1 | `[sp-48] +0x13` | `-` |
| `ldr` at 0x99bd0 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x99bd4 | load | 1 | `[sp-48] +0x24` | `-` |
| `ldr` at 0x99be4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x99be8 | load | 4 | `[sp-48] +0x28` | `-` |
| `str` at 0x99bec | store | 4 | `sp -0xaa4` | `[[sp-48]+40]` |
| `ldr` at 0x99bf4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x99c00 | store | 4 | `sp -0xaa4` | `ret(0xb7944)` |
| `ldr` at 0x99c04 | load | 4 | `sp -0xaa4` | `-` |
| `str` at 0x99c08 | store | 4 | `sp -0x2c` | `[sp-2724]` |
| `ldr` at 0x99c14 | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x99c1c | store | 4 | `sp -0xa74` | `[sp-2676]+1` |
| `ldr` at 0x99c20 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x99c24 | load | 1 | `[sp-48] +0x20` | `-` |
| `ldr` at 0x99c30 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99c38 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x99c4c | store | 4 | `(unknown)` | `ret(0x1b124)` |
| `ldr` at 0x99c50 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x99c5c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x99c60 | load | 4 | `[sp-48] +0x1c` | `-` |
| `ldr` at 0x99c6c | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99c74 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99c7c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99c84 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99c8c | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99c94 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99c98 | load | 4 | `abs 0x99e50` | `-` |
| `ldr` at 0x99ca0 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x99ca4 | load | 1 | `[sp-48] +0x2c` | `-` |
| `ldr` at 0x99cb4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x99cb8 | load | 4 | `[sp-48] +0x30` | `-` |
| `str` at 0x99cbc | store | 4 | `sp -0xaa8` | `[[sp-48]+48]` |
| `ldr` at 0x99cc4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x99cd0 | store | 4 | `sp -0xaa8` | `ret(0xb7944)` |
| `ldr` at 0x99cd4 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99cdc | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99ce4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99ce8 | load | 4 | `sp -0xaa8` | `-` |
| `ldr` at 0x99cf0 | load | 4 | `sp -0xa7c` | `-` |
| `str` at 0x99cf8 | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x99d00 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x99d0c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x99d14 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x99d18 | load | 1 | `[sp-48] +0x20` | `-` |
| `ldr` at 0x99d24 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x99d28 | load | 4 | `[sp-48] +0x1c` | `-` |
| `ldr` at 0x99d34 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99d3c | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99d44 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99d54 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99d60 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x99d6c | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x99d70 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99d74 | load | 4 | `abs 0x99e54` | `-` |
| `ldr` at 0x99d7c | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x99d80 | load | 1 | `[sp-48] +0x2c` | `-` |
| `ldr` at 0x99d90 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x99d94 | load | 4 | `[sp-48] +0x30` | `-` |
| `str` at 0x99d98 | store | 4 | `sp -0xaac` | `[[sp-48]+48]` |
| `ldr` at 0x99da0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x99dac | store | 4 | `sp -0xaac` | `ret(0xb7944)` |
| `ldr` at 0x99db0 | load | 4 | `sp -0xaac` | `-` |
| `ldr` at 0x99dc0 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99dc8 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x99dd0 | store | 4 | `(unknown)` | `ret(0x1afa4)` |
| `ldr` at 0x99dd4 | load | 4 | `sp -0xa7c` | `-` |
| `str` at 0x99ddc | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x99de4 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x99df0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x99df8 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x99dfc | load | 1 | `[sp-48] +0x20` | `-` |
| `ldr` at 0x99e08 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x99e0c | load | 1 | `[sp-48] +0x2c` | `-` |
| `ldr` at 0x99e1c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x99e20 | load | 4 | `[sp-48] +0x30` | `-` |
| `str` at 0x99e24 | store | 4 | `sp -0xab0` | `[[sp-48]+48]` |
| `ldr` at 0x99e90 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x99e9c | store | 4 | `sp -0xab0` | `ret(0xb7944)` |
| `ldr` at 0x99ea0 | load | 4 | `sp -0xab0` | `-` |
| `ldr` at 0x99eb8 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99ec4 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x99ecc | store | 4 | `(unknown)` | `ret(0x1afa4)` |
| `str` at 0x99ed4 | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x99ed8 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x99edc | load | 1 | `[sp-48] +0x34` | `-` |
| `ldr` at 0x99eec | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x99ef0 | load | 4 | `[sp-48] +0x38` | `-` |
| `str` at 0x99ef4 | store | 4 | `sp -0xab4` | `[[sp-48]+56]` |
| `ldr` at 0x99efc | load | 4 | `sp -0x30` | `-` |
| `str` at 0x99f08 | store | 4 | `sp -0xab4` | `ret(0xb7944)` |
| `ldr` at 0x99f0c | load | 4 | `sp -0xab4` | `-` |
| `ldr` at 0x99f24 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99f30 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x99f38 | store | 4 | `(unknown)` | `ret(0x1afa4)` |
| `str` at 0x99f40 | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x99f44 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99f4c | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x99f58 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x99f60 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99f68 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x99f7c | store | 4 | `(unknown)` | `ret(0x1b124)` |
| `ldr` at 0x99f80 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x99f84 | load | 4 | `[sp-48] +0x1c` | `-` |
| `ldr` at 0x99f90 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99f98 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99fa0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99fa8 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99fb0 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99fb8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99fbc | load | 4 | `abs 0x99e70` | `-` |
| `ldr` at 0x99fc4 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99fcc | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99fd4 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99fdc | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99fe4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99ff4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99ffc | load | 4 | `sp -0x450` | `-` |
| `str` at 0x9a004 | store | 4 | `sp -0x450` | `[sp-1104]-1` |
| `ldr` at 0x9a008 | load | 4 | `abs 0x99e74` | `-` |
| `ldr` at 0x9a01c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a034 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x9a03c | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x9a044 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a048 | load | 4 | `abs 0x99e78` | `-` |
| `ldr` at 0x9a050 | load | 4 | `sp -0xa7c` | `-` |
| `str` at 0x9a058 | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x9a060 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x9a06c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x9a070 | load | 4 | `sp -0x658` | `-` |
| `ldr` at 0x9a07c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x9a088 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x9a08c | load | 4 | `sp -0xa6c` | `-` |
| `str` at 0x9a094 | store | 4 | `sp -0xa6c` | `[sp-2668]+1` |
| `ldr` at 0x9a09c | load | 4 | `sp -0x658` | `-` |
| `ldr` at 0x9a0b0 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x9a0b8 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x9a0c0 | load | 4 | `sp -0x658` | `-` |
| `ldr` at 0x9a0d4 | load | 4 | `sp -0xa7c` | `-` |
| `str` at 0x9a0dc | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x9a0e4 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x9a0f0 | store | 4 | `(unknown)` | `0x0` |
| `str` at 0x9a0f8 | store | 4 | `sp -0xa6c` | `0x0` |
| `ldr` at 0x9a0fc | load | 4 | `sp -0xa6c` | `-` |
| `ldr` at 0x9a100 | load | 4 | `sp -0x654` | `-` |
| `ldr` at 0x9a110 | load | 4 | `sp -0xa6c` | `-` |
| `ldr` at 0x9a114 | load | 4 | `abs 0x99e7c` | `-` |
| `ldr` at 0x9a128 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a130 | load | 4 | `sp -0xa6c` | `-` |
| `str` at 0x9a138 | store | 4 | `sp -0xa6c` | `[sp-2668]+1` |
| `ldr` at 0x9a140 | load | 4 | `abs 0x99e80` | `-` |
| `str` at 0x9a14c | store | 4 | `sp -0x24` | `ret(0x22dc0)` |
| `ldr` at 0x9a150 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9a15c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9a168 | load | 4 | `abs 0x99e84` | `-` |
| `str` at 0x9a16c | store | 4 | `sp -0xac8` | `0x11da70` |
| `str` at 0x9a170 | store | 4 | `sp -0xac4` | `ret(0x21674)` |
| `ldr` at 0x9a178 | load | 4 | `abs 0x99e88` | `-` |
| `ldr` at 0x9a17c | load | 4 | `abs 0x99e8c` | `-` |
| `ldr` at 0x9a188 | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x9a198 | store | 4 | `sp -0xab8` | `?` |
| `str` at 0x9a1a4 | store | 4 | `sp -0xab8` | `0x0` |
| `ldr` at 0x9a1a8 | load | 4 | `sp -0xab8` | `-` |
| `str` at 0x9a1ac | store | 4 | `sp -0xa98` | `[sp-2744]` |
| `ldr` at 0x9a1b0 | load | 4 | `sp -0xa98` | `-` |
| `ldmdb` at 0x9a1b4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x9a1b4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x9a1b4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x9a1b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x98ef8 | `(unknown callee) 0x74908` | `0x0` | `r1` | `r2` | `0x0` |
| `bl` at 0x98f44 | `(unknown callee) 0x1b124` | `0x8` | `?` | `?` | `?` |
| `bl` at 0x98f8c | `(unknown callee) 0x1b43c` | `sp-2664` | `0x0` | `0x618` | `sp-2664` |
| `bl` at 0x98fbc | `(unknown callee) 0xbb804` | `0x1` | `[sp-24]` | `[sp-2680]` | `0x0` |
| `bl` at 0x98ff4 | `(unknown callee) 0x22a38` | `0x2b195c` | `0x0` | `?` | `[sp-2656]` |
| `bl` at 0x99018 | `(unknown callee) 0x1b508` | `[[sp-2680]]` | `0x11d960` | `?` | `[sp-2680]` |
| `bl` at 0x9902c | `(unknown callee) 0x1b400` | `[sp-2680]` | `?` | `?` | `ret(0x22a38)` |
| `bl` at 0x99044 | `(unknown callee) 0x1b124` | `?` | `?` | `?` | `?` |
| `bl` at 0x9912c | `(unknown callee) 0x6907c` | `sp-76` | `sp-72` | `0x0` | `sp-76` |
| `bl` at 0x99134 | `(unknown callee) 0x1b22c` | `[sp-2660]` | `?` | `?` | `?` |
| `bl` at 0x9915c | `(unknown callee) 0x692f8` | `sp-76` | `sp-72` | `[sp-2660]` | `ret(0x1b22c)` |
| `bl` at 0x99190 | `(unknown callee) 0xb6f64` | `0x2b1820` | `sp-2704` | `0x0` | `0x0` |
| `bl` at 0x991b0 | `(unknown callee) 0x79dcc` | `[sp-2708]` | `?` | `?` | `ret(0xb6f64)` |
| `bl` at 0x991f0 | `(unknown callee) 0x23aa0` | `?` | `sp-48` | `0x64` | `?` |
| `bl` at 0x99204 | `(unknown callee) 0xb56f4` | `[sp-48]` | `?` | `?` | `ret(0x23aa0)` |
| `bl` at 0x9921c | `(unknown callee) 0x1b124` | `0x50` | `?` | `?` | `[sp-2680]` |
| `bl` at 0x99230 | `(unknown callee) 0x694c4` | `[0x2b1820]` | `?` | `?` | `0x2b1820` |
| `bl` at 0x9924c | `(unknown callee) 0x1b508` | `?` | `ret(0x694c4)` | `?` | `?` |
| `bl` at 0x99268 | `(unknown callee) 0x1b2d4` | `?` | `0x11d988` | `?` | `?` |
| `bl` at 0x99290 | `(unknown callee) 0x1b22c` | `?` | `?` | `?` | `?` |
| `bl` at 0x992a8 | `(unknown callee) 0x694c4` | `[sp-2708]` | `?` | `ret(0x1b22c)` | `?` |
| `bl` at 0x992b8 | `(unknown callee) 0x1b508` | `?` | `ret(0x694c4)` | `?` | `ret(0x694c4)` |
| `bl` at 0x992d4 | `(unknown callee) 0x1b2d4` | `?` | `0x11b2f8` | `?` | `?` |
| `bl` at 0x99398 | `(unknown callee) 0xbb804` | `0x1` | `[sp-2660]` | `sp-2708` | `0x0` |
| `bl` at 0x993dc | `(unknown callee) 0x1b124` | `0x50` | `?` | `?` | `[sp-2680]` |
| `bl` at 0x99410 | `(unknown callee) 0x694c4` | `?` | `sp-20` | `0xfffff5bc` | `?` |
| `bl` at 0x9942c | `(unknown callee) 0x1b508` | `?` | `ret(0x694c4)` | `?` | `?` |
| `bl` at 0x99448 | `(unknown callee) 0x1b2d4` | `?` | `0x11d9b0` | `?` | `?` |
| `bl` at 0x99464 | `(unknown callee) 0x1b2d4` | `?` | `[sp-2708]` | `?` | `?` |
| `bl` at 0x994a8 | `(unknown callee) 0x694c4` | `?` | `sp-20` | `0xfffff5bc` | `?` |
| `bl` at 0x994c4 | `(unknown callee) 0x1b508` | `?` | `ret(0x694c4)` | `?` | `?` |
| `bl` at 0x994f4 | `(unknown callee) 0x1b2d4` | `?` | `0x11d9b4` | `[sp-2684]+1` | `?` |
| `bl` at 0x994fc | `(unknown callee) 0x1afa4` | `[sp-2708]` | `?` | `?` | `?` |
| `bl` at 0x99540 | `(unknown callee) 0x1b400` | `[sp-2708]` | `?` | `?` | `0x0` |
| `bl` at 0x995a8 | `(unknown callee) 0x6907c` | `sp-76` | `sp-72` | `0x0` | `sp-76` |
| `bl` at 0x995b0 | `(unknown callee) 0x1b22c` | `[sp-2660]` | `?` | `?` | `?` |
| `bl` at 0x995d8 | `(unknown callee) 0x692f8` | `sp-76` | `sp-72` | `[sp-2660]` | `ret(0x1b22c)` |
| `bl` at 0x99638 | `(unknown callee) 0xb6a9c` | `0x2b1820` | `?` | `sp-2700` | `0x0` |
| `bl` at 0x996a4 | `(unknown callee) 0x1b124` | `0x50` | `?` | `?` | `[sp-2680]` |
| `bl` at 0x996b8 | `(unknown callee) 0x694c4` | `[0x2b1820]` | `?` | `?` | `0x2b1820` |
| `bl` at 0x996d4 | `(unknown callee) 0x1b508` | `?` | `ret(0x694c4)` | `?` | `?` |
| `bl` at 0x996f0 | `(unknown callee) 0x1b2d4` | `?` | `0x11d9b8` | `?` | `?` |
| `bl` at 0x99718 | `(unknown callee) 0x1b22c` | `?` | `?` | `?` | `?` |
| `bl` at 0x99748 | `(unknown callee) 0x694c4` | `?` | `sp-20` | `0xfffff5bc` | `?` |
| `bl` at 0x99758 | `(unknown callee) 0x1b508` | `?` | `ret(0x694c4)` | `?` | `ret(0x694c4)` |
| `bl` at 0x99774 | `(unknown callee) 0x1b2d4` | `?` | `0x11b2f8` | `?` | `?` |
| `bl` at 0x997d4 | `(unknown callee) 0x21674` | `[sp-36]` | `sp-20` | `?` | `[sp-2676]+1` |
| `bl` at 0x997e4 | `(unknown callee) 0x2070c` | `0x11d9e0` | `ret(0x21674)` | `?` | `ret(0x21674)` |
| `bl` at 0x997fc | `(unknown callee) 0x1b400` | `[sp-2660]` | `?` | `?` | `?` |
| `bl` at 0x99834 | `(unknown callee) 0xb5698` | `sp-48` | `0x2b18c0` | `0x44c` | `sp-48` |
| `bl` at 0x998d4 | `(unknown callee) 0x1b124` | `0x50` | `sp-20` | `?` | `[sp-2680]` |
| `bl` at 0x998e8 | `(unknown callee) 0x694c4` | `[0x2b1820]` | `?` | `?` | `0x2b1820` |
| `bl` at 0x99904 | `(unknown callee) 0x1b508` | `?` | `ret(0x694c4)` | `?` | `?` |
| `bl` at 0x99920 | `(unknown callee) 0x1b2d4` | `?` | `0x11da04` | `?` | `?` |
| `bl` at 0x99948 | `(unknown callee) 0x1b22c` | `?` | `?` | `?` | `?` |
| `bl` at 0x99978 | `(unknown callee) 0x694c4` | `?` | `sp-20` | `0xfffff5bc` | `?` |
| `bl` at 0x99988 | `(unknown callee) 0x1b508` | `?` | `ret(0x694c4)` | `?` | `ret(0x694c4)` |
| `bl` at 0x999a4 | `(unknown callee) 0x1b2d4` | `?` | `0x11b2f8` | `?` | `?` |
| `bl` at 0x999fc | `(unknown callee) 0xb7944` | `[sp-48]` | `0x0` | `ret(0xb5698)` | `[[sp-48]+19]:1` |
| `bl` at 0x99a20 | `(unknown callee) 0xb56f4` | `[sp-48]` | `?` | `ret(0xb7944)` | `?` |
| `bl` at 0x99b4c | `(unknown callee) 0xb56f4` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0x99b60 | `(unknown callee) 0x21674` | `[sp-36]` | `ret(0xb5698)` | `ret(0xb5698)` | `ret(0xb5698)` |
| `bl` at 0x99b74 | `(unknown callee) 0x2070c` | `0x11da20` | `ret(0x21674)` | `[sp-36]` | `ret(0x21674)` |
| `bl` at 0x99bfc | `(unknown callee) 0xb7944` | `[sp-48]` | `0x0` | `?` | `?` |
| `bl` at 0x99c44 | `(unknown callee) 0x1b124` | `0x50` | `?` | `?` | `[sp-2680]` |
| `bl` at 0x99c64 | `(unknown callee) 0x694c4` | `[[sp-48]+28]` | `?` | `?` | `[sp-48]` |
| `bl` at 0x99c80 | `(unknown callee) 0x1b508` | `?` | `ret(0x694c4)` | `?` | `?` |
| `bl` at 0x99c9c | `(unknown callee) 0x1b2d4` | `?` | `0x11d9b0` | `?` | `?` |
| `bl` at 0x99ccc | `(unknown callee) 0xb7944` | `[sp-48]` | `0x1` | `?` | `?` |
| `bl` at 0x99cec | `(unknown callee) 0x1b2d4` | `?` | `[sp-2728]` | `?` | `?` |
| `bl` at 0x99d2c | `(unknown callee) 0x694c4` | `[[sp-48]+28]` | `?` | `?` | `[sp-48]` |
| `bl` at 0x99d48 | `(unknown callee) 0x1b508` | `?` | `ret(0x694c4)` | `?` | `?` |
| `bl` at 0x99d78 | `(unknown callee) 0x1b2d4` | `?` | `0x11d9b4` | `[sp-2684]+1` | `?` |
| `bl` at 0x99da8 | `(unknown callee) 0xb7944` | `[sp-48]` | `0x1` | `?` | `?` |
| `bl` at 0x99db4 | `(unknown callee) 0x1afa4` | `[sp-2732]` | `?` | `?` | `?` |
| `bl` at 0x99e98 | `(unknown callee) 0xb7944` | `[sp-48]` | `0x1` | `?` | `?` |
| `bl` at 0x99ea4 | `(unknown callee) 0x1afa4` | `[sp-2736]` | `?` | `?` | `?` |
| `bl` at 0x99f04 | `(unknown callee) 0xb7944` | `[sp-48]` | `0x2` | `[sp-2684]+1` | `?` |
| `bl` at 0x99f10 | `(unknown callee) 0x1afa4` | `[sp-2740]` | `?` | `?` | `?` |
| `bl` at 0x99f74 | `(unknown callee) 0x1b124` | `0x50` | `?` | `?` | `[sp-2680]` |
| `bl` at 0x99f88 | `(unknown callee) 0x694c4` | `[[sp-48]+28]` | `?` | `?` | `[sp-48]` |
| `bl` at 0x99fa4 | `(unknown callee) 0x1b508` | `?` | `ret(0x694c4)` | `?` | `?` |
| `bl` at 0x99fc0 | `(unknown callee) 0x1b2d4` | `?` | `0x11da48` | `?` | `?` |
| `bl` at 0x99fe8 | `(unknown callee) 0x1b22c` | `?` | `?` | `?` | `?` |
| `bl` at 0x9a020 | `(unknown callee) 0x694c4` | `?` | `sp-20` | `0xfffff5bc` | `?` |
| `bl` at 0x9a030 | `(unknown callee) 0x1b508` | `?` | `ret(0x694c4)` | `?` | `ret(0x694c4)` |
| `bl` at 0x9a04c | `(unknown callee) 0x1b2d4` | `?` | `0x11b2f8` | `?` | `?` |
| `bl` at 0x9a080 | `(unknown callee) 0xb56f4` | `[sp-48]` | `?` | `?` | `[sp-1624]` |
| `mov` at 0x9a0d0 | `(unknown callee) (unknown target)` | `sp-2664` | `?` | `sp-2664` | `?` |
| `bl` at 0x9a12c | `(unknown callee) 0xb56f4` | `?` | `sp-20` | `0xfffff9c4` | `?` |
| `bl` at 0x9a144 | `(unknown callee) 0x22dc0` | `0x2b195c` | `?` | `?` | `?` |
| `bl` at 0x9a160 | `(unknown callee) 0x21674` | `ret(0x22dc0)` | `?` | `?` | `ret(0x22dc0)` |
| `bl` at 0x9a184 | `(unknown callee) 0x21e28` | `0x9` | `0x11d884` | `0x1995` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x98fd0 | `ret(0xbb804) != 0x0` | `ret(0xbb804) == 0x0` |
| `bne` at 0x98fdc | `[sp-2656] != 0x0` | `[sp-2656] == 0x0` |
| `beq` at 0x99008 | `ret(0x22a38) == 0x0` | `ret(0x22a38) != 0x0` |
| `bge` at 0x99080 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x990a8 | `? != 0x1` | `? == 0x1` |
| `bne` at 0x991a4 | `ret(0xb6f64) != 0x0` | `ret(0xb6f64) == 0x0` |
| `beq` at 0x991bc | `ret(0x79dcc) == 0x0` | `ret(0x79dcc) != 0x0` |
| `beq` at 0x991fc | `ret(0x23aa0) == 0x0` | `ret(0x23aa0) != 0x0` |
| `bne` at 0x99368 | `? != ?` | `? == ?` |
| `beq` at 0x993ac | `ret(0xbb804) == 0x0` | `ret(0xbb804) != 0x0` |
| `beq` at 0x993c4 | `[sp-2708] == 0x0` | `[sp-2708] != 0x0` |
| `beq` at 0x993f0 | `[sp-44] == 0x0` | `[sp-44] != 0x0` |
| `beq` at 0x9964c | `ret(0xb6a9c) == 0x0` | `ret(0xb6a9c) != 0x0` |
| `bne` at 0x9965c | `[sp-1628] != 0x1` | `[sp-1628] == 0x1` |
| `blt` at 0x99814 | `[sp-2668] < [sp-2656] (signed, 32-bit)` | `[sp-2668] >= [sp-2656] (signed, 32-bit)` |
| `beq` at 0x99850 | `ret(0xb5698) == 0x0` | `ret(0xb5698) != 0x0` |
| `bgt` at 0x9985c | `ret(0xb5698) > 0x0 (signed, 32-bit)` | `ret(0xb5698) <= 0x0 (signed, 32-bit)` |
| `beq` at 0x99868 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x99878 | `ret(0xb5698) == 0x6e` | `ret(0xb5698) != 0x6e` |
| `blt` at 0x99894 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x998bc | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x999f0 | `[[sp-48]+19]:1 != 0x13` | `[[sp-48]+19]:1 == 0x13` |
| `ble` at 0x99a10 | `ret(0xb7944) <= [sp-40] (signed, 32-bit)` | `ret(0xb7944) > [sp-40] (signed, 32-bit)` |
| `bne` at 0x99a68 | `Z == 0` | `not (Z == 0)` |
| `blt` at 0x99a78 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x99aa8 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x99abc | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x99ad8 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x99b00 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x99b44 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x99b80 | `[sp-48] == 0x0` | `[sp-48] != 0x0` |
| `beq` at 0x99b8c | `[sp-1624] == 0x0` | `[sp-1624] != 0x0` |
| `beq` at 0x99bc8 | `[[sp-48]+19]:1 == 0x0` | `[[sp-48]+19]:1 != 0x0` |
| `bne` at 0x99be0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x99c10 | `[sp-2724] == 0x0` | `[sp-2724] != 0x0` |
| `bls` at 0x99c2c | `[[sp-48]+32]:1 <= 0x1 (unsigned, 32-bit)` | `[[sp-48]+32]:1 > 0x1 (unsigned, 32-bit)` |
| `beq` at 0x99c58 | `[sp-44] == 0x0` | `[sp-44] != 0x0` |
| `bne` at 0x99cb0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x99d20 | `[[sp-48]+32]:1 != 0x2` | `[[sp-48]+32]:1 == 0x2` |
| `bne` at 0x99d8c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x99e04 | `[[sp-48]+32]:1 != 0x3` | `[[sp-48]+32]:1 == 0x3` |
| `bne` at 0x99e18 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x99ee8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x9a078 | `[sp-1624] != 0x0` | `[sp-1624] == 0x0` |
| `beq` at 0x9a0a4 | `[sp-1624] == 0x0` | `[sp-1624] != 0x0` |
| `blt` at 0x9a108 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x9a158 | `ret(0x22dc0) == 0x0` | `ret(0x22dc0) != 0x0` |
| `beq` at 0x9a190 | `[sp-2676] == 0x0` | `[sp-2676] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x9a1b4 | `r0` | `?` |

## `SMPCmd_2m_Init`

- `SMPCmd_2m_Init`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa2ff8 to 0xa3040.
  Established. Following a call to `0xbb764` with `0x11fe54`, `0xba`, entry r2 and entry r3 (`bl` at
  0xa300c), word stores install `0xa03fc` at `0x1497e4` and `0xa09f8` at `0x1497e8` (`str` at
  0xa3018, `str` at 0xa3024). The recorded exit carries `ret(0xbb764)` (`ldmdb` at 0xa3028). What
  consumes the two installed values remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xa3004 | load | 4 | `abs 0xa302c` | `-` |
| `ldr` at 0xa3010 | load | 4 | `abs 0xa3030` | `-` |
| `ldr` at 0xa3014 | load | 4 | `abs 0xa3034` | `-` |
| `str` at 0xa3018 | store | 4 | `abs 0x1497e4` | `0xa03fc` |
| `ldr` at 0xa301c | load | 4 | `abs 0xa3038` | `-` |
| `ldr` at 0xa3020 | load | 4 | `abs 0xa303c` | `-` |
| `str` at 0xa3024 | store | 4 | `abs 0x1497e8` | `0xa09f8` |
| `ldmdb` at 0xa3028 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa3028 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa3028 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa300c | `(unknown callee) 0xbb764` | `0x11fe54` | `0xba` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa3028 | `r0` | `ret(0xbb764)` |

## `smpcmd_tst`

- `smpcmd_tst`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb1650 to 0xb1bdc.
  Established. The first candidate is `0x50` at the call to `0x1b124`, followed by entry r1,
  entry r2 and `0x1` (`bl` at 0xb1678). A word store writes `ret(0x1b124)` to `r1 +0x0`, then a byte
  store clears `[[sp-28]] +0x0` (`str` at 0xb1680, `strb` at 0xb1690). The selector takes
  `[[[sp-24]+112]] <= 0x8 (unsigned, 32-bit)` and falls through above that bound (`ldrls` at
  0xb16a4); its taken targets are unresolved, and the command effect there is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb1660 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xb1664 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xb166c | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0xb1670 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb1680 | store | 4 | `r1 +0x0` | `ret(0x1b124)` |
| `ldr` at 0xb1684 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1688 | load | 4 | `[sp-28] +0x0` | `-` |
| `strb` at 0xb1690 | store | 1 | `[[sp-28]] +0x0` | `0x0` |
| `ldr` at 0xb1694 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb1698 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xb169c | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldrls` at 0xb16a4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb1b84 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xb1b94 | load | 4 | `sp -0x4c` | `-` |
| `ldmdb` at 0xb1b98 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xb1b98 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb1b98 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb1b98 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb1678 | `(unknown callee) 0x1b124` | `0x50` | `r1` | `r2` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xb16a4 | `[[[sp-24]+112]] <= 0x8 (unsigned, 32-bit)` | `[[[sp-24]+112]] > 0x8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb1b98 | `r0` | `[sp-76]` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xb16a4).

## `smpcmd_ioexp`

- `smpcmd_ioexp`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb1bdc to 0xb1dac.
  Established. A word store records `ret(0xd68dc)` at `sp -0x2c` (`str` at 0xb1c00). The later call
  to `0x1b124` has unknown candidates r0 and r1, `ret(0xd68dc)` in r2 and unknown r3 (`bl` at
  0xb1c24). The resulting word is written to `[sp-28] +0x0` (`str` at 0xb1c2c). The loop decision
  has unknown operands (`blt` at 0xb1c74), and the physical role of the reported data is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb1bec | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xb1bf0 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xb1c00 | store | 4 | `sp -0x2c` | `ret(0xd68dc)` |
| `ldr` at 0xb1c04 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1c08 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb1c2c | store | 4 | `[sp-28] +0x0` | `ret(0x1b124)` |
| `ldr` at 0xb1c30 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xb1c34 | store | 4 | `sp -0x28` | `[[sp-28]]` |
| `ldr` at 0xb1c38 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb1c3c | load | 4 | `abs 0xb1d98` | `-` |
| `ldr` at 0xb1c44 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb1c54 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb1c5c | store | 4 | `sp -0x28` | `([sp-40] + ret(0x1b22c))` |
| `str` at 0xb1c64 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb1c68 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb1c6c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xb1c7c | load | 1 | `sp -0x24` | `-` |
| `ldr` at 0xb1c90 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb1c94 | load | 4 | `abs 0xb1d9c` | `-` |
| `ldr` at 0xb1c98 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb1ca0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb1cb0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb1cb8 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x1b22c))` |
| `ldrb` at 0xb1cbc | load | 1 | `sp -0x24` | `-` |
| `ldr` at 0xb1cd8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb1cdc | load | 4 | `abs 0xb1da0` | `-` |
| `ldrb` at 0xb1ce8 | load | 1 | `sp -0x24` | `-` |
| `str` at 0xb1d00 | store | 4 | `sp -0x20` | `ret(0xd6b9c)` |
| `ldr` at 0xb1d04 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1d10 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1d1c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb1d20 | load | 4 | `abs 0xb1da4` | `-` |
| `ldrb` at 0xb1d30 | load | 1 | `sp -0x24` | `-` |
| `ldrb` at 0xb1d48 | load | 1 | `sp -0x2d` | `-` |
| `ldr` at 0xb1d4c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb1d50 | load | 4 | `abs 0xb1da8` | `-` |
| `ldr` at 0xb1d60 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb1d70 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb1d78 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x1b22c))` |
| `ldr` at 0xb1d7c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb1d84 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldmdb` at 0xb1d94 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xb1d94 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb1d94 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb1d94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb1bf8 | `(unknown callee) 0xd68dc` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0xb1c24 | `(unknown callee) 0x1b124` | `?` | `?` | `ret(0xd68dc)` | `?` |
| `bl` at 0xb1c40 | `(unknown callee) 0x1b49c` | `[[sp-28]]` | `0x127918` | `?` | `[[sp-28]]` |
| `bl` at 0xb1c48 | `(unknown callee) 0x1b22c` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xb1c88 | `(unknown callee) 0xd7558` | `0x0` | `[sp-36]:1` | `?` | `[sp-36]:1` |
| `bl` at 0xb1c9c | `(unknown callee) 0x1b49c` | `[sp-40]` | `0x127964` | `[sp-36]` | `ret(0xd7558)` |
| `bl` at 0xb1ca4 | `(unknown callee) 0x1b22c` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xb1cc8 | `(unknown callee) 0xd6928` | `0x0` | `[sp-36]:1` | `ret(0x1b22c)` | `[sp-36]:1` |
| `bl` at 0xb1ce0 | `(unknown callee) 0x1b49c` | `[sp-40]` | `0x127974` | `?` | `ret(0xd6928)` |
| `bl` at 0xb1cf8 | `(unknown callee) 0xd6b9c` | `0x0` | `[sp-36]:1` | `sp-45` | `[sp-36]:1` |
| `bl` at 0xb1d14 | `(unknown callee) 0x219a8` | `ret(0xd6b9c)` | `?` | `?` | `ret(0xd6b9c)` |
| `bl` at 0xb1d28 | `(unknown callee) 0x1b49c` | `[sp-40]` | `0x127984` | `ret(0x219a8)` | `ret(0x219a8)` |
| `bl` at 0xb1d3c | `(unknown callee) 0xd6cb4` | `0x0` | `[sp-36]:1` | `?` | `[sp-36]:1` |
| `bl` at 0xb1d5c | `(unknown callee) 0x1b49c` | `[sp-40]` | `0x1279a0` | `[sp-45]:1` | `?` |
| `bl` at 0xb1d64 | `(unknown callee) 0x1b22c` | `[sp-40]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xb1c74 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xb1cd4 | `ret(0xd6928) != 0x0` | `ret(0xd6928) == 0x0` |
| `beq` at 0xb1d0c | `ret(0xd6b9c) == 0x0` | `ret(0xd6b9c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb1d94 | `r0` | `0x0` |

## `smpcmd_intr_count`

- `smpcmd_intr_count`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb1dac to 0xb20f0.
  Established. Three call results are saved as words: `ret(0xcc81c)` at `sp -0x24`, `ret(0xd68dc)`
  at `sp -0x28` and `ret(0xc88d8)` at `sp -0x2c` (`str` at 0xb1dd0, `str` at 0xb1de0, `str` at
  0xb1de8). Another word store publishes `ret(0x1b124)` through `[sp-32] +0x0` (`str` at 0xb1e24).
  The early loop comparisons retain unknown operands (`blt` at 0xb1e6c, `blt` at 0xb1f00). The
  counter sources and their hardware meaning remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb1dbc | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0xb1dc0 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0xb1dd0 | store | 4 | `sp -0x24` | `ret(0xcc81c)` |
| `str` at 0xb1de0 | store | 4 | `sp -0x28` | `ret(0xd68dc)` |
| `str` at 0xb1de8 | store | 4 | `sp -0x2c` | `ret(0xc88d8)` |
| `ldr` at 0xb1dec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1df0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb1df4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb1dfc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb1e24 | store | 4 | `[sp-32] +0x0` | `ret(0x1b124)` |
| `ldr` at 0xb1e28 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0xb1e2c | store | 4 | `sp -0x30` | `[[sp-32]]` |
| `ldr` at 0xb1e30 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb1e34 | load | 4 | `abs 0xb20d8` | `-` |
| `ldr` at 0xb1e3c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb1e4c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xb1e54 | store | 4 | `sp -0x30` | `([sp-48] + ret(0x1b22c))` |
| `str` at 0xb1e5c | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xb1e60 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb1e64 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xb1e74 | load | 1 | `sp -0x34` | `-` |
| `ldrb` at 0xb1e84 | load | 1 | `sp -0x34` | `-` |
| `ldrb` at 0xb1e98 | load | 1 | `sp -0x34` | `-` |
| `str` at 0xb1ea8 | store | 4 | `sp -0x38` | `ret(0xc88f8)` |
| `ldr` at 0xb1eac | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb1eb0 | load | 4 | `abs 0xb20dc` | `-` |
| `ldr` at 0xb1ec0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb1ed0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xb1ed8 | store | 4 | `sp -0x30` | `([sp-48] + ret(0x1b22c))` |
| `ldr` at 0xb1edc | load | 4 | `sp -0x34` | `-` |
| `str` at 0xb1ee4 | store | 4 | `sp -0x34` | `[sp-52]+1` |
| `str` at 0xb1ef0 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xb1ef4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb1ef8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xb1f08 | load | 1 | `sp -0x34` | `-` |
| `ldrb` at 0xb1f24 | load | 1 | `sp -0x34` | `-` |
| `ldr` at 0xb1f38 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb1f3c | load | 4 | `abs 0xb20e0` | `-` |
| `ldr` at 0xb1f4c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb1f58 | load | 1 | `sp -0x34` | `-` |
| `ldrb` at 0xb1f78 | load | 1 | `sp -0x34` | `-` |
| `ldrb` at 0xb1f8c | load | 1 | `sp -0x34` | `-` |
| `ldr` at 0xb1fa0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb1fa4 | load | 4 | `abs 0xb20e4` | `-` |
| `ldr` at 0xb1fb0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb1fc0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xb1fc8 | store | 4 | `sp -0x30` | `([sp-48] + ret(0x1b22c))` |
| `ldr` at 0xb1fcc | load | 4 | `sp -0x34` | `-` |
| `str` at 0xb1fd4 | store | 4 | `sp -0x34` | `[sp-52]+1` |
| `str` at 0xb1fe0 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xb1fe4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb1fe8 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xb1ff8 | load | 1 | `sp -0x34` | `-` |
| `ldrb` at 0xb2014 | load | 1 | `sp -0x34` | `-` |
| `ldr` at 0xb2028 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb202c | load | 4 | `abs 0xb20e8` | `-` |
| `ldr` at 0xb203c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb2048 | load | 1 | `sp -0x34` | `-` |
| `ldrb` at 0xb2068 | load | 1 | `sp -0x34` | `-` |
| `ldrb` at 0xb207c | load | 1 | `sp -0x34` | `-` |
| `ldr` at 0xb2090 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb2094 | load | 4 | `abs 0xb20ec` | `-` |
| `ldr` at 0xb20a0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb20b0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xb20b8 | store | 4 | `sp -0x30` | `([sp-48] + ret(0x1b22c))` |
| `ldr` at 0xb20bc | load | 4 | `sp -0x34` | `-` |
| `str` at 0xb20c4 | store | 4 | `sp -0x34` | `[sp-52]+1` |
| `ldmdb` at 0xb20d4 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xb20d4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xb20d4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb20d4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb20d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb1dc8 | `(unknown callee) 0xcc81c` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0xb1dd8 | `(unknown callee) 0xd68dc` | `0x0` | `?` | `?` | `ret(0xcc81c)` |
| `bl` at 0xb1de4 | `(unknown callee) 0xc88d8` | `ret(0xd68dc)` | `?` | `?` | `ret(0xd68dc)` |
| `bl` at 0xb1e1c | `(unknown callee) 0x1b124` | `?` | `?` | `(([sp-36] + [sp-40]) + ret(0xc88d8))` | `?` |
| `bl` at 0xb1e38 | `(unknown callee) 0x1b508` | `[[sp-32]]` | `0x1279b4` | `?` | `[[sp-32]]` |
| `bl` at 0xb1e40 | `(unknown callee) 0x1b22c` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0xb1e7c | `(unknown callee) 0xc898c` | `[sp-52]:1` | `?` | `?` | `[sp-52]:1` |
| `bl` at 0xb1e8c | `(unknown callee) 0xc8a20` | `[sp-52]:1` | `?` | `?` | `[sp-52]:1` |
| `bl` at 0xb1ea0 | `(unknown callee) 0xc88f8` | `[sp-52]:1` | `?` | `?` | `[sp-52]:1` |
| `bl` at 0xb1ebc | `(unknown callee) 0x1b49c` | `[sp-48]` | `0x1279c4` | `ret(0xc898c)` | `?` |
| `bl` at 0xb1ec4 | `(unknown callee) 0x1b22c` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0xb1f14 | `(unknown callee) 0xcc494` | `0x0` | `[sp-52]:1` | `?` | `[sp-52]:1` |
| `bl` at 0xb1f30 | `(unknown callee) 0xcdf64` | `0x0` | `[sp-52]:1` | `?` | `[sp-52]:1` |
| `bl` at 0xb1f44 | `(unknown callee) 0x1b49c` | `[sp-48]` | `0x1279dc` | `ret(0xcdf64)` | `ret(0xcdf64)` |
| `bl` at 0xb1f64 | `(unknown callee) 0xcdb7c` | `0x0` | `[sp-52]:1` | `?` | `[sp-52]:1` |
| `bl` at 0xb1f84 | `(unknown callee) 0xcdf64` | `0x0` | `[sp-52]:1` | `?` | `[sp-52]:1` |
| `bl` at 0xb1f98 | `(unknown callee) 0xcdb7c` | `0x0` | `[sp-52]:1` | `?` | `[sp-52]:1` |
| `bl` at 0xb1fac | `(unknown callee) 0x1b49c` | `[sp-48]` | `0x1279fc` | `ret(0xcdf64)` | `ret(0xcdb7c)` |
| `bl` at 0xb1fb4 | `(unknown callee) 0x1b22c` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0xb2004 | `(unknown callee) 0xd6928` | `0x0` | `[sp-52]:1` | `?` | `[sp-52]:1` |
| `bl` at 0xb2020 | `(unknown callee) 0xd7558` | `0x0` | `[sp-52]:1` | `?` | `[sp-52]:1` |
| `bl` at 0xb2034 | `(unknown callee) 0x1b49c` | `[sp-48]` | `0x127a14` | `ret(0xd7558)` | `ret(0xd7558)` |
| `bl` at 0xb2054 | `(unknown callee) 0xd6c70` | `0x0` | `[sp-52]:1` | `?` | `[sp-52]:1` |
| `bl` at 0xb2074 | `(unknown callee) 0xd7558` | `0x0` | `[sp-52]:1` | `?` | `[sp-52]:1` |
| `bl` at 0xb2088 | `(unknown callee) 0xd6c70` | `0x0` | `[sp-52]:1` | `?` | `[sp-52]:1` |
| `bl` at 0xb209c | `(unknown callee) 0x1b49c` | `[sp-48]` | `0x127a34` | `ret(0xd7558)` | `ret(0xd6c70)` |
| `bl` at 0xb20a4 | `(unknown callee) 0x1b22c` | `[sp-48]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xb1e6c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0xb1f00 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xb1f20 | `ret(0xcc494) != 0x0` | `ret(0xcc494) == 0x0` |
| `bne` at 0xb1f54 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xb1f70 | `ret(0xcdb7c) != 0x0` | `ret(0xcdb7c) == 0x0` |
| `blt` at 0xb1ff0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xb2010 | `ret(0xd6928) != 0x0` | `ret(0xd6928) == 0x0` |
| `bne` at 0xb2044 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xb2060 | `ret(0xd6c70) != 0x0` | `ret(0xd6c70) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb20d4 | `r0` | `0x0` |

## `charioPort`

- `charioPort`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x23d74 to 0x23d94.
  Established. The non-stack word read is at absolute address `0x145584` (`ldr` at 0x23d84); its
  value reaches the recorded r0 exit as `[0x145584]` (`ldmdb` at 0x23d8c). The port identity and
  interpretation of that word are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x23d80 | load | 4 | `abs 0x23d90` | `-` |
| `ldr` at 0x23d84 | load | 4 | `abs 0x145584` | `-` |
| `ldmdb` at 0x23d8c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x23d8c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x23d8c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x23d8c | `r0` | `[0x145584]` |

## `PPPWrite`

- `PPPWrite`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xba8cc to 0xbade0.
  Established. The initial word copy moves `[r0+16]` into `r0 +0x18` (`str` at 0xba900). A byte
  store emits `0x7e` at `[[sp-20]+24] +0x0` (`strb` at 0xba918). The first loop guard takes `[sp] !=
  0x0` and falls through on zero (`bne` at 0xba940). An indirect call has candidates `[sp-28]`,
  `[[sp-20]+16]`, `[[sp-20]+28]` and `[sp-20]` (`mov` at 0xbaaf8); its target and the physical
  transport remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xba8dc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xba8e0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xba8e4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xba8e8 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xba8ec | load | 4 | `sp +0x0` | `-` |
| `str` at 0xba8f0 | store | 4 | `sp -0x30` | `[sp]` |
| `ldr` at 0xba8f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba8f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba8fc | load | 4 | `r0 +0x10` | `-` |
| `str` at 0xba900 | store | 4 | `r0 +0x18` | `[r0+16]` |
| `ldr` at 0xba904 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba90c | load | 4 | `[sp-20] +0x18` | `-` |
| `strb` at 0xba918 | store | 1 | `[[sp-20]+24] +0x0` | `0x7e` |
| `str` at 0xba920 | store | 4 | `[sp-20] +0x18` | `[[sp-20]+24]+1` |
| `ldr` at 0xba924 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xba92c | store | 4 | `[sp-20] +0x1c` | `0x1` |
| `strh` at 0xba934 | store | 2 | `sp -0x26` | `?` |
| `ldr` at 0xba938 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xba94c | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xba950 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xba954 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0xba958 | store | 4 | `sp -0x3c` | `[[sp-32]]:1` |
| `ldr` at 0xba95c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xba968 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xba974 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xba980 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xba990 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xba9a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba9a8 | load | 4 | `[sp-20] +0x24` | `-` |
| `str` at 0xba9b8 | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0xba9c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba9c4 | load | 4 | `[sp-20] +0x20` | `-` |
| `str` at 0xba9d4 | store | 4 | `sp -0x34` | `0x1` |
| `str` at 0xba9e0 | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0xba9e4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xba9f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba9f8 | load | 4 | `[sp-20] +0x18` | `-` |
| `strb` at 0xbaa04 | store | 1 | `[[sp-20]+24] +0x0` | `0x7d` |
| `str` at 0xbaa0c | store | 4 | `[sp-20] +0x18` | `[[sp-20]+24]+1` |
| `ldr` at 0xbaa10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbaa18 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xbaa1c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xbaa20 | load | 1 | `[sp-32] +0x0` | `-` |
| `strb` at 0xbaa2c | store | 1 | `[[sp-20]+24] +0x0` | `?` |
| `str` at 0xbaa34 | store | 4 | `[sp-20] +0x18` | `[[sp-20]+24]+1` |
| `ldr` at 0xbaa38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbaa3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbaa40 | load | 4 | `[sp-20] +0x1c` | `-` |
| `str` at 0xbaa48 | store | 4 | `[sp-20] +0x1c` | `[[sp-20]+28]+2` |
| `ldr` at 0xbaa50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbaa58 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xbaa5c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xbaa64 | load | 1 | `[sp-32] +0x0` | `-` |
| `strb` at 0xbaa68 | store | 1 | `[[sp-20]+24] +0x0` | `([[sp-32]]:1 & 0xff)` |
| `str` at 0xbaa70 | store | 4 | `[sp-20] +0x18` | `[[sp-20]+24]+1` |
| `ldr` at 0xbaa74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbaa78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbaa7c | load | 4 | `[sp-20] +0x1c` | `-` |
| `str` at 0xbaa84 | store | 4 | `[sp-20] +0x1c` | `[[sp-20]+28]+1` |
| `ldr` at 0xbaa88 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xbaa8c | load | 1 | `[sp-32] +0x0` | `-` |
| `ldrh` at 0xbaa90 | load | 2 | `sp -0x26` | `-` |
| `strh` at 0xbaaa4 | store | 2 | `sp -0x26` | `(ret(0xba6c4) & 0xffff)` |
| `ldr` at 0xbaaa8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbaab0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xbaab4 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbaabc | store | 4 | `sp +0x0` | `[sp]-1` |
| `ldr` at 0xbaac0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbaac4 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0xbaacc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbaad0 | load | 4 | `[sp-20] +0x14` | `-` |
| `ldr` at 0xbaadc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbaae0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbaae4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbaae8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbaaec | load | 4 | `[sp-20] +0x10` | `-` |
| `ldr` at 0xbaaf0 | load | 4 | `[sp-20] +0x1c` | `-` |
| `str` at 0xbab00 | store | 4 | `sp -0x24` | `ret(unknown)` |
| `ldr` at 0xbab04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbab08 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbab0c | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0xbab18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbab1c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbab24 | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0xbab2c | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xbab34 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xbab38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbab3c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbab40 | load | 4 | `[sp-20] +0x10` | `-` |
| `ldr` at 0xbab44 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0xbab4c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbab50 | store | 4 | `sp -0x38` | `[sp-36]` |
| `ldr` at 0xbab58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbab5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbab60 | load | 4 | `[sp-20] +0x10` | `-` |
| `str` at 0xbab64 | store | 4 | `[sp-20] +0x18` | `[[sp-20]+16]` |
| `ldr` at 0xbab68 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbab70 | store | 4 | `[sp-20] +0x1c` | `0x0` |
| `ldrh` at 0xbab78 | load | 2 | `sp -0x26` | `-` |
| `strh` at 0xbab80 | store | 2 | `sp -0x26` | `?` |
| `ldrb` at 0xbab84 | load | 1 | `sp -0x26` | `-` |
| `strb` at 0xbab88 | store | 1 | `sp -0x2c` | `([sp-38]:1 & 0xff)` |
| `ldrh` at 0xbab8c | load | 2 | `sp -0x26` | `-` |
| `strb` at 0xbab94 | store | 1 | `sp -0x2b` | `?` |
| `strh` at 0xbab9c | store | 2 | `sp -0x26` | `0x0` |
| `ldrh` at 0xbaba0 | load | 2 | `sp -0x26` | `-` |
| `str` at 0xbabb4 | store | 4 | `sp -0x34` | `0x0` |
| `ldrh` at 0xbabb8 | load | 2 | `sp -0x26` | `-` |
| `ldrb` at 0xbabcc | load | 1 | `(unknown)` | `-` |
| `str` at 0xbabd0 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xbabd4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbabe0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbabec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbabf8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbac08 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbac1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbac20 | load | 4 | `[sp-20] +0x24` | `-` |
| `str` at 0xbac30 | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0xbac38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbac3c | load | 4 | `[sp-20] +0x20` | `-` |
| `str` at 0xbac4c | store | 4 | `sp -0x34` | `0x1` |
| `str` at 0xbac58 | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0xbac5c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xbac68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbac70 | load | 4 | `[sp-20] +0x18` | `-` |
| `strb` at 0xbac7c | store | 1 | `[[sp-20]+24] +0x0` | `0x7d` |
| `str` at 0xbac84 | store | 4 | `[sp-20] +0x18` | `[[sp-20]+24]+1` |
| `ldr` at 0xbac88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbac90 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldrh` at 0xbac94 | load | 2 | `sp -0x26` | `-` |
| `ldrb` at 0xbaca8 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xbacb4 | store | 1 | `[[sp-20]+24] +0x0` | `?` |
| `str` at 0xbacbc | store | 4 | `[sp-20] +0x18` | `[[sp-20]+24]+1` |
| `ldr` at 0xbacc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbacc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbacc8 | load | 4 | `[sp-20] +0x1c` | `-` |
| `str` at 0xbacd0 | store | 4 | `[sp-20] +0x1c` | `[[sp-20]+28]+2` |
| `ldr` at 0xbacd8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbace0 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldrh` at 0xbace4 | load | 2 | `sp -0x26` | `-` |
| `ldrb` at 0xbacfc | load | 1 | `(unknown)` | `-` |
| `strb` at 0xbad00 | store | 1 | `[[sp-20]+24] +0x0` | `?` |
| `str` at 0xbad08 | store | 4 | `[sp-20] +0x18` | `[[sp-20]+24]+1` |
| `ldr` at 0xbad0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbad10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbad14 | load | 4 | `[sp-20] +0x1c` | `-` |
| `str` at 0xbad1c | store | 4 | `[sp-20] +0x1c` | `[[sp-20]+28]+1` |
| `ldrh` at 0xbad20 | load | 2 | `sp -0x26` | `-` |
| `strh` at 0xbad28 | store | 2 | `sp -0x26` | `([sp-38]:2+1 & 0xffff)` |
| `ldr` at 0xbad30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbad34 | load | 4 | `[sp-20] +0x18` | `-` |
| `strb` at 0xbad3c | store | 1 | `[[sp-20]+24] +0x0` | `0x7e` |
| `ldr` at 0xbad40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbad44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbad48 | load | 4 | `[sp-20] +0x1c` | `-` |
| `str` at 0xbad50 | store | 4 | `[sp-20] +0x1c` | `[[sp-20]+28]+1` |
| `ldr` at 0xbad54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbad58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbad5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbad60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbad64 | load | 4 | `[sp-20] +0x10` | `-` |
| `ldr` at 0xbad68 | load | 4 | `[sp-20] +0x1c` | `-` |
| `str` at 0xbad78 | store | 4 | `sp -0x24` | `ret(unknown)` |
| `ldr` at 0xbad7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbad80 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbad84 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0xbad90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbad94 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbad9c | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0xbada4 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xbadac | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xbadb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbadb4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbadb8 | load | 4 | `[sp-20] +0x10` | `-` |
| `ldr` at 0xbadbc | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0xbadc4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbadc8 | store | 4 | `sp -0x38` | `[sp-36]` |
| `ldr` at 0xbadd0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xbadd4 | store | 4 | `sp -0x38` | `[sp-48]` |
| `ldr` at 0xbadd8 | load | 4 | `sp -0x38` | `-` |
| `ldmdb` at 0xbaddc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xbaddc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xbaddc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbaa9c | `(unknown callee) 0xba6c4` | `[[sp-32]]:1` | `[sp-38]:2` | `[sp-38]:2` | `[[sp-32]]:1` |
| `mov` at 0xbaaf8 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-20]+16]` | `[[sp-20]+28]` | `[sp-20]` |
| `bl` at 0xbab48 | `(unknown callee) 0xba71c` | `[sp-20]` | `ret(unknown)` | `[[sp-20]+16]` | `[[sp-20]+28]` |
| `mov` at 0xbad70 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-20]+16]` | `[[sp-20]+28]` | `[sp-20]` |
| `bl` at 0xbadc0 | `(unknown callee) 0xba71c` | `[sp-20]` | `ret(unknown)` | `[[sp-20]+16]` | `[[sp-20]+28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xba940 | `[sp] != 0x0` | `[sp] == 0x0` |
| `beq` at 0xba964 | `[[sp-32]]:1 == 0x13` | `[[sp-32]]:1 != 0x13` |
| `bgt` at 0xba970 | `[[sp-32]]:1 > 0x13 (signed, 32-bit)` | `[[sp-32]]:1 <= 0x13 (signed, 32-bit)` |
| `beq` at 0xba97c | `[[sp-32]]:1 == 0xd` | `[[sp-32]]:1 != 0xd` |
| `beq` at 0xba988 | `[[sp-32]]:1 == 0x11` | `[[sp-32]]:1 != 0x11` |
| `bhi` at 0xba99c | `[[sp-32]]:1-125 > 0x1 (unsigned, 32-bit)` | `[[sp-32]]:1-125 <= 0x1 (unsigned, 32-bit)` |
| `beq` at 0xba9b0 | `[[sp-20]+36] == 0x0` | `[[sp-20]+36] != 0x0` |
| `beq` at 0xba9cc | `[[sp-20]+32] == 0x0` | `[[sp-20]+32] != 0x0` |
| `beq` at 0xba9ec | `Z == 1` | `not (Z == 1)` |
| `blo` at 0xbaad8 | `[[sp-20]+28]+5 < [[sp-20]+20] (unsigned, 32-bit)` | `[[sp-20]+28]+5 >= [[sp-20]+20] (unsigned, 32-bit)` |
| `beq` at 0xbab14 | `ret(unknown) == [[sp-20]+28]` | `ret(unknown) != [[sp-20]+28]` |
| `bls` at 0xbaba8 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0xbabdc | `[sp-64] == 0x13` | `[sp-64] != 0x13` |
| `bgt` at 0xbabe8 | `[sp-64] > 0x13 (signed, 32-bit)` | `[sp-64] <= 0x13 (signed, 32-bit)` |
| `beq` at 0xbabf4 | `[sp-64] == 0xd` | `[sp-64] != 0xd` |
| `beq` at 0xbac00 | `[sp-64] == 0x11` | `[sp-64] != 0x11` |
| `bhi` at 0xbac14 | `[sp-64]-125 > 0x1 (unsigned, 32-bit)` | `[sp-64]-125 <= 0x1 (unsigned, 32-bit)` |
| `beq` at 0xbac28 | `[[sp-20]+36] == 0x0` | `[[sp-20]+36] != 0x0` |
| `beq` at 0xbac44 | `[[sp-20]+32] == 0x0` | `[[sp-20]+32] != 0x0` |
| `beq` at 0xbac64 | `[sp-52] == 0x0` | `[sp-52] != 0x0` |
| `beq` at 0xbad8c | `ret(unknown) == [[sp-20]+28]` | `ret(unknown) != [[sp-20]+28]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xbaddc | `r0` | `?` |

## `jmstr_set_functions`

- `jmstr_set_functions`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdc96c to 0xdc9a8.
  Established. Two word stores set `0x2ad248` to entry r0 and `0x2ad24c` to `[sp-24]` (`str` at
  0xdc98c, `str` at 0xdc998). The recorded return preserves entry r0 (`ldmdb` at 0xdc99c). These
  assignments establish storage of the supplied values, while their use as functions remains unknown
  in this body.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdc97c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdc980 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xdc984 | load | 4 | `abs 0xdc9a0` | `-` |
| `ldr` at 0xdc988 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdc98c | store | 4 | `abs 0x2ad248` | `r0` |
| `ldr` at 0xdc990 | load | 4 | `abs 0xdc9a4` | `-` |
| `ldr` at 0xdc994 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdc998 | store | 4 | `abs 0x2ad24c` | `[sp-24]` |
| `ldmdb` at 0xdc99c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdc99c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdc99c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdc99c | `r0` | `r0` |

## `readsection`

- `readsection`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe1aa8 to 0xe1b2c.
  Established. The call to `0x1b160` carries entry r0, `0x1`, entry r1 and entry r2 (`bl` at
  0xe1ad4). A result unequal to `[sp-24]` takes the branch and equality falls through (`bne` at
  0xe1ae8). The later byte load has an unknown effective target (`ldrb` at 0xe1b00), and its
  deciding branch has an unknown operand (`bne` at 0xe1b08). The section format and callee identity
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe1ab8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe1abc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe1ac0 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xe1ac4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe1acc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe1ad0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe1ad8 | store | 4 | `sp -0x20` | `ret(0x1b160)` |
| `ldr` at 0xe1adc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe1ae0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe1af0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe1af4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xe1b00 | load | 1 | `(unknown)` | `-` |
| `str` at 0xe1b14 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xe1b20 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe1b24 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xe1b28 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe1b28 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe1b28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe1ad4 | `(unknown callee) 0x1b160` | `r0` | `0x1` | `r1` | `r2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe1ae8 | `ret(0x1b160) != [sp-24]` | `ret(0x1b160) == [sp-24]` |
| `bne` at 0xe1b08 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe1b28 | `r0` | `[sp-36]` |

## `bitfile_read_hdr_section`

- `bitfile_read_hdr_section`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf9f58 to 0xfa0a0.
  Established. The opening call to `0x1b160` uses `sp-40`, `0x1`, `0x1` and entry r0 (`bl` at
  0xf9f90). Equality with `[sp-32]` takes the following branch and inequality falls through (`beq`
  at 0xf9fa8). A later call uses `sp-40`, `0x1`, `0x2` and `[sp-20]` (`bl` at 0xfa008). A zero byte
  is written to an unknown effective target (`strb` at 0xf9fd0). The header format remains unknown
  beyond the reads and decisions listed below.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf9f68 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf9f6c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf9f70 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xf9f78 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xf9f88 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf9f8c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xf9f98 | store | 4 | `sp -0x24` | `ret(0x1b160)` |
| `ldr` at 0xf9f9c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf9fa0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf9fb0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xf9fbc | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xf9fd0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xf9fd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf9fe0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xf9fe4 | load | 1 | `sp -0x28` | `-` |
| `strb` at 0xf9fe8 | store | 1 | `[sp-24] +0x0` | `([sp-40]:1 & 0xff)` |
| `str` at 0xf9ff0 | store | 4 | `sp -0x20` | `0x2` |
| `ldr` at 0xfa000 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xfa004 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xfa010 | store | 4 | `sp -0x24` | `ret(0x1b160)` |
| `ldr` at 0xfa014 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xfa018 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xfa028 | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0xfa030 | load | 1 | `sp -0x28` | `-` |
| `ldrb` at 0xfa038 | load | 1 | `sp -0x27` | `-` |
| `str` at 0xfa040 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xfa044 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfa04c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xfa050 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xfa05c | store | 4 | `sp -0x24` | `ret(0x1b160)` |
| `ldr` at 0xfa060 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xfa064 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xfa074 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xfa07c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfa080 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xfa08c | store | 1 | `([sp-28] + [sp-32]) +0x0` | `0x0` |
| `str` at 0xfa094 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xfa098 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xfa09c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xfa09c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xfa09c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf9f90 | `(unknown callee) 0x1b160` | `sp-40` | `0x1` | `0x1` | `r0` |
| `bl` at 0xfa008 | `(unknown callee) 0x1b160` | `sp-40` | `0x1` | `0x2` | `[sp-20]` |
| `bl` at 0xfa054 | `(unknown callee) 0x1b160` | `[sp-28]` | `0x1` | `[sp-32]` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xf9fa8 | `ret(0x1b160) == [sp-32]` | `ret(0x1b160) != [sp-32]` |
| `beq` at 0xf9fdc | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0xfa020 | `ret(0x1b160) == [sp-32]` | `ret(0x1b160) != [sp-32]` |
| `beq` at 0xfa06c | `ret(0x1b160) == [sp-32]` | `ret(0x1b160) != [sp-32]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xfa09c | `r0` | `[sp-44]` |
