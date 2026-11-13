# ARM JTAG scan and configuration paths in 1.44.0

Read from `6.5.30/usr/cpu/firmware/sysco/l1.bin`, package `6.5.30:l1#4` (version 1.44.0),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x53d154`.
Carrier SHA-256: `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
Package SHA-256: `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
Inner executable SHA-256: `8d492b1d00a2fa93aebb48890caf8cacc5a9747e79209e93b841378aabd75162`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1166540 bytes.
Segment SHA-256: `7f2caa3b9c656f2cc1caf5bdaf139038c25b4a58ac8ff8a26ea044818245a46d`. Analysed routine spans use the exact
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
No unresolved computed transfers were reported for these routines.
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/jtag-scan-configuration-facts.txt) retain all tool sections.

## `JTAG_Init`

- `JTAG_Init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd6e58 to 0xd702c.
  Established. Included halfword stores through `[0x29c464] +0x0` write `0x800`, `0x1000`, `0x800`, and `0x800` (`strh` at 0xd6f8c, `strh` at 0xd6f9c, `strh` at 0xd6fac, `strh` at 0xd6fbc).
  The unnamed `0x193b8` call carries `0x0`, `0x100`, `0x3`, `0x1` (`bl` at 0xd6f24).
  The absolute word at `0x297378` receives `0x1` (`str` at 0xd6fdc).
  Scan initialization I/O contract remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xd6e68 | load | 4 | `abs 0xd6fe8` | `-` |
| `ldr` at 0xd6e6c | load | 4 | `abs 0x297378` | `-` |
| `ldr` at 0xd6e78 | load | 4 | `abs 0xd6fec` | `-` |
| `str` at 0xd6e7c | store | 4 | `sp -0x24` | `0x121370` |
| `ldr` at 0xd6e84 | load | 4 | `abs 0xd6ff0` | `-` |
| `ldr` at 0xd6e88 | load | 4 | `abs 0xd6ff4` | `-` |
| `ldr` at 0xd6e94 | load | 4 | `abs 0xd6ff8` | `-` |
| `ldr` at 0xd6e98 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xd6ea4 | load | 4 | `abs 0xd6ffc` | `-` |
| `ldr` at 0xd6eac | load | 4 | `abs 0xd7000` | `-` |
| `ldr` at 0xd6eb0 | load | 4 | `abs 0xd7004` | `-` |
| `ldr` at 0xd6eb8 | load | 4 | `abs 0xd7008` | `-` |
| `ldr` at 0xd6ebc | load | 4 | `abs 0xd700c` | `-` |
| `str` at 0xd6ec8 | store | 4 | `sp -0x1c` | `ret(0x1967c)` |
| `ldr` at 0xd6ecc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd6ee4 | load | 4 | `abs 0xd7010` | `-` |
| `ldr` at 0xd6ee8 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0xd6eec | load | 4 | `abs 0xd7014` | `-` |
| `ldr` at 0xd6ef0 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xd6f00 | load | 4 | `abs 0xd7018` | `-` |
| `ldr` at 0xd6f04 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd6f08 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xd6f10 | store | 4 | `sp -0x20` | `0x10000000` |
| `str` at 0xd6f28 | store | 4 | `abs 0x29c4e8` | `ret(0x193b8)` |
| `ldr` at 0xd6f2c | load | 4 | `abs 0x29c4e8` | `-` |
| `ldr` at 0xd6f44 | load | 4 | `abs 0xd7010` | `-` |
| `ldr` at 0xd6f48 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0xd6f4c | load | 4 | `abs 0xd701c` | `-` |
| `ldr` at 0xd6f50 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xd6f6c | load | 4 | `abs 0xd7020` | `-` |
| `ldr` at 0xd6f70 | load | 4 | `abs 0xd7018` | `-` |
| `ldr` at 0xd6f74 | load | 4 | `abs 0x29c4e8` | `-` |
| `str` at 0xd6f7c | store | 4 | `abs 0x29c464` | `[0x29c4e8]+16` |
| `ldr` at 0xd6f80 | load | 4 | `abs 0xd7020` | `-` |
| `ldr` at 0xd6f84 | load | 4 | `abs 0x29c464` | `-` |
| `strh` at 0xd6f8c | store | 2 | `[0x29c464] +0x0` | `0x800` |
| `ldr` at 0xd6f90 | load | 4 | `abs 0xd7020` | `-` |
| `ldr` at 0xd6f94 | load | 4 | `abs 0x29c464` | `-` |
| `strh` at 0xd6f9c | store | 2 | `[0x29c464] +0x0` | `0x1000` |
| `ldr` at 0xd6fa0 | load | 4 | `abs 0xd7020` | `-` |
| `ldr` at 0xd6fa4 | load | 4 | `abs 0x29c464` | `-` |
| `strh` at 0xd6fac | store | 2 | `[0x29c464] +0x0` | `0x800` |
| `ldr` at 0xd6fb0 | load | 4 | `abs 0xd7020` | `-` |
| `ldr` at 0xd6fb4 | load | 4 | `abs 0x29c464` | `-` |
| `strh` at 0xd6fbc | store | 2 | `[0x29c464] +0x0` | `0x800` |
| `ldr` at 0xd6fc0 | load | 4 | `abs 0xd7024` | `-` |
| `ldr` at 0xd6fc4 | load | 4 | `abs 0xd7028` | `-` |
| `ldr` at 0xd6fd4 | load | 4 | `abs 0xd6fe8` | `-` |
| `str` at 0xd6fdc | store | 4 | `abs 0x297378` | `0x1` |
| `ldm` at 0xd6fe4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xd6fe4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd6fe4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd6fe4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd6e90 | `(unknown callee) 0x200a0` | `0x9` | `0x121388` | `0x102` | `0x0` |
| `bl` at 0xd6ea8 | `(unknown callee) 0x221f0` | `0x1213a0` | `?` | `?` | `[0x29737c]` |
| `bl` at 0xd6eb4 | `(unknown callee) 0xd0714` | `0xd9be0` | `0xd9978` | `?` | `?` |
| `bl` at 0xd6ec0 | `(unknown callee) 0x1967c` | `0x1213d4` | `0x1002` | `?` | `?` |
| `bl` at 0xd6ed8 | `(unknown callee) 0x193d0` | `ret(0x1967c)` | `?` | `?` | `ret(0x1967c)` |
| `bl` at 0xd6ef4 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0x1213e0` | `[ret(0x193d0)]` | `0x12eeb8` |
| `bl` at 0xd6efc | `(unknown callee) 0x196b8` | `0x1` | `?` | `?` | `?` |
| `bl` at 0xd6f24 | `(unknown callee) 0x193b8` | `0x0` | `0x100` | `0x3` | `0x1` |
| `bl` at 0xd6f38 | `(unknown callee) 0x193d0` | `ret(0x193b8)` | `?` | `?` | `[0x29c4e8]` |
| `bl` at 0xd6f54 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0x121400` | `[ret(0x193d0)]` | `0x12eeb8` |
| `bl` at 0xd6f5c | `(unknown callee) 0x196b8` | `0x1` | `?` | `?` | `?` |
| `bl` at 0xd6f68 | `(unknown callee) 0xd702c` | `0x1` | `0x0` | `?` | `?` |
| `bl` at 0xd6fd0 | `(unknown callee) 0x20bb4` | `0x29c4ec` | `0x12142c` | `0x1` | `0x9` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd6e74 | `[0x297378] != 0x1` | `[0x297378] == 0x1` |
| `beq` at 0xd6ea0 | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `bge` at 0xd6ed4 | `ret(0x1967c) >= 0x0 (signed, 32-bit)` | `ret(0x1967c) < 0x0 (signed, 32-bit)` |
| `bne` at 0xd6f34 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd6fe4 | `r0` | `ret(0x20bb4)` |

## `JTAG_Lock`

- `JTAG_Lock`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd708c to 0xd70fc.
  Established. The unnamed `0x20cd0` call carries `0x29c4ec`, unknown, entry `r2`, entry `r3` (`bl` at 0xd70a4);
  its zero result takes `0xd70e8`, while nonzero falls through (`beq` at 0xd70b8).
  An included word store writes `ret(0x1f8e0)` to `sp -0x18` (`str` at 0xd70d0).
  Scan lock synchronization semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xd709c | load | 4 | `abs 0xd70f0` | `-` |
| `str` at 0xd70ac | store | 4 | `sp -0x14` | `ret(0x20cd0)` |
| `ldr` at 0xd70b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd70bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd70c8 | load | 4 | `abs 0xd70f4` | `-` |
| `str` at 0xd70cc | store | 4 | `sp -0x1c` | `0x121438` |
| `str` at 0xd70d0 | store | 4 | `sp -0x18` | `ret(0x1f8e0)` |
| `ldr` at 0xd70d8 | load | 4 | `abs 0xd70f8` | `-` |
| `ldm` at 0xd70ec | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd70ec | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd70ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd70a4 | `(unknown callee) 0x20cd0` | `0x29c4ec` | `?` | `r2` | `r3` |
| `bl` at 0xd70c0 | `(unknown callee) 0x1f8e0` | `ret(0x20cd0)` | `?` | `?` | `ret(0x20cd0)` |
| `bl` at 0xd70e4 | `(unknown callee) 0x200a0` | `0x9` | `0x121388` | `0x170` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd70b8 | `ret(0x20cd0) == 0x0` | `ret(0x20cd0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd70ec | `r0` | `?` |

## `JTAG_Unlock`

- `JTAG_Unlock`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd70fc to 0xd7168.
  Established. The unnamed `0x2105c` call carries `0x29c4ec` and entry `r1`, `r2`, `r3` (`bl` at 0xd7110).
  A zero result takes `0xd7154`, while nonzero falls through (`beq` at 0xd7124).
  The word at `sp -0x1c` receives `0x121458` (`str` at 0xd7138).
  Scan release result interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xd710c | load | 4 | `abs 0xd715c` | `-` |
| `str` at 0xd7118 | store | 4 | `sp -0x14` | `ret(0x2105c)` |
| `ldr` at 0xd711c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd7128 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd7134 | load | 4 | `abs 0xd7160` | `-` |
| `str` at 0xd7138 | store | 4 | `sp -0x1c` | `0x121458` |
| `str` at 0xd713c | store | 4 | `sp -0x18` | `ret(0x1f8e0)` |
| `ldr` at 0xd7144 | load | 4 | `abs 0xd7164` | `-` |
| `ldm` at 0xd7158 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd7158 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd7158 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd7110 | `(unknown callee) 0x2105c` | `0x29c4ec` | `r1` | `r2` | `r3` |
| `bl` at 0xd712c | `(unknown callee) 0x1f8e0` | `ret(0x2105c)` | `?` | `?` | `ret(0x2105c)` |
| `bl` at 0xd7150 | `(unknown callee) 0x200a0` | `0x9` | `0x121388` | `0x180` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd7124 | `ret(0x2105c) == 0x0` | `ret(0x2105c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd7158 | `r0` | `?` |

## `jmstr_reset`

- `jmstr_reset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd0754 to 0xd0814.
  Established. An included indirect call carries unknown, `0x20`, `sp-40`, `sp-40` (`mov` at 0xd07d0);
  its nonzero result takes `0xd07f8`, while zero falls through (`bne` at 0xd07dc).
  The word at `sp -0x2c` receives `0x1` on an included path (`str` at 0xd07fc).
  Scan reset command interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd0764 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xd0768 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xd076c | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xd0774 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xd077c | store | 4 | `sp -0x24` | `0x20` |
| `ldr` at 0xd0780 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd078c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd0794 | store | 4 | `sp -0x24` | `0x28` |
| `ldr` at 0xd0798 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd07a4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd07ac | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd07b0 | load | 4 | `abs 0xd080c` | `-` |
| `ldr` at 0xd07bc | load | 4 | `abs 0x29736c` | `-` |
| `ldr` at 0xd07c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd07e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd07e4 | load | 4 | `abs 0xd0810` | `-` |
| `str` at 0xd07f0 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0xd07fc | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xd0800 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0xd0808 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xd0808 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd0808 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd0808 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0xd07d0 | `(unknown callee) (unknown target)` | `?` | `0x20` | `sp-40` | `sp-40` |
| `bl` at 0xd07e8 | `(unknown callee) 0x197a8` | `[sp-32]` | `0x12108c` | `?` | `ret(unknown)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd0788 | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0xd07a0 | `r0 != 0x1` | `r0 == 0x1` |
| `bne` at 0xd07dc | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd0808 | `r0` | `?` |

## `jmstr_reset_and_idle`

- `jmstr_reset_and_idle`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd0814 to 0xd093c.
  Established. The word at `sp -0x24` receives `0x10` before a later included `0x18` assignment (`str` at 0xd08bc, `str` at 0xd08d4).
  The indirect call at `0xd08f8` carries unknown, `0x20`, `sp-40`, `sp-40` (`mov` at 0xd08f8);
  its nonzero result takes `0xd0920`, while zero falls through (`bne` at 0xd0904).
  Reset-to-idle scan behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd0824 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xd0828 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xd082c | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xd0834 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xd083c | store | 4 | `sp -0x24` | `0x20` |
| `ldr` at 0xd0840 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd084c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd0854 | store | 4 | `sp -0x24` | `0x28` |
| `ldr` at 0xd0858 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd0864 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd086c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd0870 | load | 4 | `abs 0xd0934` | `-` |
| `ldr` at 0xd087c | load | 4 | `abs 0x29736c` | `-` |
| `ldr` at 0xd0880 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd08a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd08a4 | load | 4 | `abs 0xd0938` | `-` |
| `str` at 0xd08b0 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0xd08bc | store | 4 | `sp -0x24` | `0x10` |
| `ldr` at 0xd08c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd08cc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd08d4 | store | 4 | `sp -0x24` | `0x18` |
| `ldr` at 0xd08d8 | load | 4 | `abs 0xd0934` | `-` |
| `ldr` at 0xd08e4 | load | 4 | `abs 0x29736c` | `-` |
| `ldr` at 0xd08e8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd0908 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd090c | load | 4 | `abs 0xd0938` | `-` |
| `str` at 0xd0918 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0xd0924 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xd0928 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0xd0930 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xd0930 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd0930 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd0930 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0xd0890 | `(unknown callee) (unknown target)` | `?` | `0x20` | `sp-40` | `sp-40` |
| `bl` at 0xd08a8 | `(unknown callee) 0x197a8` | `[sp-32]` | `0x12108c` | `?` | `ret(unknown)` |
| `mov` at 0xd08f8 | `(unknown callee) (unknown target)` | `?` | `0x20` | `sp-40` | `sp-40` |
| `bl` at 0xd0910 | `(unknown callee) 0x197a8` | `[sp-32]` | `0x12108c` | `?` | `ret(unknown)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd0848 | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0xd0860 | `r0 != 0x1` | `r0 == 0x1` |
| `bne` at 0xd089c | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |
| `beq` at 0xd08c8 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `bne` at 0xd0904 | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd0930 | `r0` | `?` |

## `JTAG_Comm_read`

- `JTAG_Comm_read`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdb53c to 0xdb6d4.
  Established. The unnamed `0xde978` call carries entry `r0`, `[sp-48]`, `sp-44`, `sp-44` (`bl` at 0xdb5a4);
  its zero result takes `0xdb5f0`, while nonzero falls through (`beq` at 0xdb5b8).
  An included byte store writes an unknown value through `([sp-24] + [sp-32]) +0x0` (`strb` at 0xdb680).
  Scan read data packing remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdb54c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdb550 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xdb554 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xdb558 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdb570 | store | 4 | `sp -0x30` | `0xc` |
| `str` at 0xdb57c | store | 4 | `sp -0x30` | `0x11` |
| `str` at 0xdb58c | store | 4 | `sp -0x3c` | `0x40` |
| `ldr` at 0xdb590 | load | 4 | `abs 0xdb6cc` | `-` |
| `str` at 0xdb594 | store | 4 | `sp -0x38` | `0x121a10` |
| `ldr` at 0xdb598 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdb59c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xdb5ac | store | 4 | `sp -0x24` | `ret(0xde978)` |
| `ldr` at 0xdb5b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdb5bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdb5c8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdb5d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdb5d8 | load | 4 | `abs 0xdb6d0` | `-` |
| `ldr` at 0xdb5e4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdb5e8 | store | 4 | `sp -0x34` | `?` |
| `str` at 0xdb608 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xdb60c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdb61c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdb620 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdb638 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xdb63c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xdb658 | load | 4 | `(unknown)` | `-` |
| `str` at 0xdb660 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xdb664 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdb668 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdb67c | load | 4 | `(unknown)` | `-` |
| `strb` at 0xdb680 | store | 1 | `([sp-24] + [sp-32]) +0x0` | `?` |
| `ldr` at 0xdb69c | load | 4 | `(unknown)` | `-` |
| `str` at 0xdb6a4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xdb6a8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdb6b0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `str` at 0xdb6bc | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xdb6c0 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0xdb6c8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdb6c8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdb6c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdb5a4 | `(unknown callee) 0xde978` | `r0` | `[sp-48]` | `sp-44` | `sp-44` |
| `bl` at 0xdb5cc | `(unknown callee) 0x1f8e0` | `ret(0xde978)` | `?` | `?` | `[sp-28]` |
| `bl` at 0xdb5e0 | `(unknown callee) 0x197a8` | `[sp-28]` | `0x121a20` | `ret(0x1f8e0)` | `ret(0x1f8e0)` |
| `bl` at 0xdb600 | `(unknown callee) 0xd6084` | `0x0` | `0x40` | `sp-44` | `sp-44` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdb568 | `? != 0x6` | `? == 0x6` |
| `beq` at 0xdb5b8 | `ret(0xde978) == 0x0` | `ret(0xde978) != 0x0` |
| `beq` at 0xdb5c4 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `bgt` at 0xdb614 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdb6c8 | `r0` | `?` |

## `JTAG_Comm_write`

- `JTAG_Comm_write`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdb6d4 to 0xdb868.
  Established. The unnamed `0xde978` call carries `[sp-20]`, `[sp-48]`, `sp-44`, `0x0` (`bl` at 0xdb800).
  A zero result takes `0xdb84c`, while nonzero falls through (`beq` at 0xdb814).
  The byte through `([sp-24] + [sp-32]) +0x4` is read (`ldrb` at 0xdb778).
  Scan write transport encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdb6e4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdb6e8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xdb6ec | store | 4 | `sp -0x1c` | `r2` |
| `stm` at 0xdb704 | store | 4 | `(unknown)` | `0x0` |
| `stm` at 0xdb704 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xdb708 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdb720 | store | 4 | `sp -0x30` | `0xd` |
| `str` at 0xdb72c | store | 4 | `sp -0x30` | `0x12` |
| `str` at 0xdb734 | store | 4 | `sp -0x20` | `0x3` |
| `ldr` at 0xdb738 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdb75c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdb768 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdb76c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xdb778 | load | 1 | `([sp-24] + [sp-32]) +0x4` | `-` |
| `str` at 0xdb780 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xdb79c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdb7a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdb7a8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xdb7b0 | load | 1 | `([sp-24] + [sp-32]) +0x0` | `-` |
| `str` at 0xdb7b8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xdb7bc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdb7c4 | store | 4 | `sp -0x20` | `[sp-32]-1` |
| `str` at 0xdb7e8 | store | 4 | `sp -0x3c` | `0x40` |
| `ldr` at 0xdb7ec | load | 4 | `abs 0xdb860` | `-` |
| `str` at 0xdb7f0 | store | 4 | `sp -0x38` | `0x121a4c` |
| `ldr` at 0xdb7f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdb7f8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xdb808 | store | 4 | `sp -0x24` | `ret(0xde978)` |
| `ldr` at 0xdb80c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdb818 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdb824 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdb830 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdb834 | load | 4 | `abs 0xdb864` | `-` |
| `ldr` at 0xdb840 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdb844 | store | 4 | `sp -0x34` | `?` |
| `str` at 0xdb850 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xdb854 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0xdb85c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdb85c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdb85c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdb7dc | `(unknown callee) 0xd6084` | `0x0` | `0x40` | `sp-44` | `sp-44` |
| `bl` at 0xdb800 | `(unknown callee) 0xde978` | `[sp-20]` | `[sp-48]` | `sp-44` | `0x0` |
| `bl` at 0xdb828 | `(unknown callee) 0x1f8e0` | `ret(0xde978)` | `?` | `?` | `[sp-28]` |
| `bl` at 0xdb83c | `(unknown callee) 0x197a8` | `[sp-28]` | `0x121a5c` | `ret(0x1f8e0)` | `ret(0x1f8e0)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdb718 | `? != 0x6` | `? == 0x6` |
| `blt` at 0xdb740 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xdb814 | `ret(0xde978) == 0x0` | `ret(0xde978) != 0x0` |
| `beq` at 0xdb820 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdb85c | `r0` | `?` |

## `JTAG_Cfg_Setup`

- `JTAG_Cfg_Setup`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdb4e8 to 0xdb53c.
  Established. Three included absolute word stores assign `0xdb090` to `0x12da64`, `0xda900` to `0x12da68`, and `0xdadd8` to `0x12da6c` (`str` at 0xdb4fc, `str` at 0xdb508, `str` at 0xdb514).
  The integer return record is `0x0` (`ldm` at 0xdb520).
  Configuration hook target roles remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xdb4f4 | load | 4 | `abs 0xdb524` | `-` |
| `ldr` at 0xdb4f8 | load | 4 | `abs 0xdb528` | `-` |
| `str` at 0xdb4fc | store | 4 | `abs 0x12da64` | `0xdb090` |
| `ldr` at 0xdb500 | load | 4 | `abs 0xdb52c` | `-` |
| `ldr` at 0xdb504 | load | 4 | `abs 0xdb530` | `-` |
| `str` at 0xdb508 | store | 4 | `abs 0x12da68` | `0xda900` |
| `ldr` at 0xdb50c | load | 4 | `abs 0xdb534` | `-` |
| `ldr` at 0xdb510 | load | 4 | `abs 0xdb538` | `-` |
| `str` at 0xdb514 | store | 4 | `abs 0x12da6c` | `0xdadd8` |
| `ldm` at 0xdb520 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdb520 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdb520 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdb520 | `r0` | `0x0` |

## `JTAG_Cfg_create`

- `JTAG_Cfg_create`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd9e0c to 0xda000.
  Established. Included absolute word stores clear `0x29738c` and assign `0x1` to `0x297388` (`str` at 0xd9edc, `str` at 0xd9ee8).
  A zero `[0x29737c]` takes `0xd9e54`, while nonzero falls through (`beq` at 0xd9e2c).
  The unnamed `0x1970c` call carries unknown, `0x29c540`, unknown, unknown (`bl` at 0xd9ec0).
  Configuration allocation lifecycle remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd9e1c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xd9e20 | load | 4 | `abs 0xd9fe8` | `-` |
| `ldr` at 0xd9e24 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xd9e30 | load | 4 | `abs 0xd9fec` | `-` |
| `str` at 0xd9e34 | store | 4 | `sp -0x24` | `0x1216e8` |
| `ldr` at 0xd9e38 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd9e3c | store | 4 | `sp -0x20` | `r0` |
| `ldr` at 0xd9e44 | load | 4 | `abs 0xd9ff0` | `-` |
| `str` at 0xd9e58 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xd9e5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd9e68 | load | 4 | `abs 0xd9ff4` | `-` |
| `ldr` at 0xd9e6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd9e8c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd9e98 | load | 4 | `abs 0xd9ff4` | `-` |
| `ldr` at 0xd9e9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd9ebc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd9ec4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd9ecc | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xd9ed4 | load | 4 | `abs 0xd9ff8` | `-` |
| `str` at 0xd9edc | store | 4 | `abs 0x29738c` | `0x0` |
| `ldr` at 0xd9ee0 | load | 4 | `abs 0xd9ffc` | `-` |
| `str` at 0xd9ee8 | store | 4 | `abs 0x297388` | `0x1` |
| `ldr` at 0xd9eec | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd9efc | store | 4 | `sp -0x1c` | `0x1` |
| `str` at 0xd9f08 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xd9f0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd9f10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9f1c | load | 4 | `abs 0xd9ff4` | `-` |
| `ldr` at 0xd9f20 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd9f3c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xd9f40 | load | 4 | `abs 0xd9ff4` | `-` |
| `ldr` at 0xd9f44 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd9f68 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xd9f6c | load | 4 | `abs 0xd9ff4` | `-` |
| `ldr` at 0xd9f70 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd9f94 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xd9f98 | load | 4 | `abs 0xd9ff4` | `-` |
| `ldr` at 0xd9f9c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd9fc0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xd9fc4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd9fcc | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0xd9fd8 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0xd9fdc | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xd9fe4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd9fe4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd9fe4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd9e50 | `(unknown callee) 0x200a0` | `0x6` | `0x121700` | `0x7b` | `0x0` |
| `bl` at 0xd9ec0 | `(unknown callee) 0x1970c` | `?` | `0x29c540` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd9e2c | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `bhi` at 0xd9e64 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0xd9e94 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xd9ef4 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bhs` at 0xd9f18 | `C == 1` | `not (C == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd9fe4 | `r0` | `0x1` |

## `JTAG_Cfg_info_init`

- `JTAG_Cfg_info_init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xda000 to 0xda374.
  Established. Included stores clear a halfword and two bytes at unknown effective targets (`strh` at 0xda244, `strb` at 0xda280, `strb` at 0xda2bc).
  Unsigned `[0x29737c] <= 0x2` takes `0xda054`, while greater values fall through (`bls` at 0xda024).
  The unnamed `0x1943c` call carries unknown, `0x29c540`, unknown, unknown (`bl` at 0xda1b4).
  Configuration record field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xda010 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xda014 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xda018 | load | 4 | `abs 0xda360` | `-` |
| `ldr` at 0xda01c | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xda028 | load | 4 | `abs 0xda364` | `-` |
| `str` at 0xda02c | store | 4 | `sp -0x30` | `0x12171c` |
| `ldr` at 0xda030 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xda034 | store | 4 | `sp -0x2c` | `r0` |
| `ldr` at 0xda038 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xda03c | store | 4 | `sp -0x28` | `r1` |
| `ldr` at 0xda044 | load | 4 | `abs 0xda368` | `-` |
| `ldr` at 0xda054 | load | 4 | `abs 0xda36c` | `-` |
| `ldr` at 0xda058 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xda078 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xda07c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xda088 | load | 4 | `abs 0xda36c` | `-` |
| `ldr` at 0xda08c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xda0b0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xda0b4 | load | 4 | `abs 0xda36c` | `-` |
| `ldr` at 0xda0b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xda0d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xda0e4 | load | 4 | `abs 0xda36c` | `-` |
| `ldr` at 0xda0e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xda108 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xda110 | load | 4 | `abs 0xda36c` | `-` |
| `ldr` at 0xda114 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xda138 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xda13c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xda14c | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0xda154 | load | 4 | `abs 0xda36c` | `-` |
| `ldr` at 0xda158 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xda178 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xda184 | load | 4 | `abs 0xda36c` | `-` |
| `ldr` at 0xda188 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xda1a8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xda1bc | store | 4 | `(unknown)` | `ret(0x1943c)` |
| `ldr` at 0xda1c0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xda1cc | load | 4 | `abs 0xda370` | `-` |
| `str` at 0xda1d0 | store | 4 | `sp -0x30` | `0x121744` |
| `ldr` at 0xda1d8 | load | 4 | `abs 0xda368` | `-` |
| `str` at 0xda1ec | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xda1f8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xda1fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xda200 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xda20c | load | 4 | `abs 0xda36c` | `-` |
| `ldr` at 0xda210 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xda230 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xda238 | load | 4 | `(unknown)` | `-` |
| `strh` at 0xda244 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0xda248 | load | 4 | `abs 0xda36c` | `-` |
| `ldr` at 0xda24c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xda26c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xda274 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xda280 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xda284 | load | 4 | `abs 0xda36c` | `-` |
| `ldr` at 0xda288 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xda2a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xda2b0 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xda2bc | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xda2c0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xda2c8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xda2d0 | load | 4 | `abs 0xda36c` | `-` |
| `ldr` at 0xda2d4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xda2f0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xda2f4 | load | 4 | `abs 0xda36c` | `-` |
| `ldr` at 0xda2f8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xda31c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xda320 | load | 4 | `abs 0xda36c` | `-` |
| `ldr` at 0xda324 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xda344 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xda348 | store | 4 | `(unknown)` | `[sp-28]` |
| `str` at 0xda350 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0xda354 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xda35c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xda35c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xda35c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xda35c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xda050 | `(unknown callee) 0x200a0` | `0x6` | `0x121700` | `0xa3` | `0x0` |
| `bl` at 0xda10c | `(unknown callee) 0x1970c` | `?` | `0x29c540` | `[sp-24]` | `?` |
| `bl` at 0xda1b4 | `(unknown callee) 0x1943c` | `?` | `0x29c540` | `?` | `?` |
| `bl` at 0xda1e4 | `(unknown callee) 0x200a0` | `0x3` | `0x121700` | `0xb7` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xda024 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xda084 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xda0e0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xda144 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xda180 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xda1c8 | `? != 0x0` | `? == 0x0` |
| `bhs` at 0xda208 | `C == 1` | `not (C == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xda35c | `r0` | `?` |

## `JTAG_Cfg_port_init`

- `JTAG_Cfg_port_init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xda374 to 0xda778.
  Established. The unnamed `0xda778` call carries `[sp-24]`, `0x29c540`, unknown, `[sp]` (`bl` at 0xda73c).
  A nonzero entry `r2` takes `0xda3bc`, while zero falls through (`bne` at 0xda39c).
  Included byte stores have unknown effective targets and values (`strb` at 0xda604, `strb` at 0xda650).
  Port configuration record layout remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xda384 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xda388 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xda38c | store | 4 | `sp -0x20` | `r2` |
| `strb` at 0xda390 | store | 1 | `sp -0x21` | `(r3 & 0xff)` |
| `ldr` at 0xda394 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xda3a0 | load | 4 | `abs 0xda75c` | `-` |
| `str` at 0xda3a4 | store | 4 | `sp -0x38` | `0x121774` |
| `ldr` at 0xda3ac | load | 4 | `abs 0xda760` | `-` |
| `ldr` at 0xda3bc | load | 4 | `abs 0xda764` | `-` |
| `ldr` at 0xda3c0 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xda3cc | load | 4 | `abs 0xda768` | `-` |
| `str` at 0xda3d0 | store | 4 | `sp -0x38` | `0x1217a0` |
| `ldr` at 0xda3d4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xda3d8 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xda3dc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xda3e0 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xda3e8 | load | 4 | `abs 0xda760` | `-` |
| `ldr` at 0xda3f8 | load | 4 | `abs 0xda76c` | `-` |
| `ldr` at 0xda3fc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xda418 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xda41c | load | 4 | `abs 0xda76c` | `-` |
| `ldr` at 0xda420 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xda440 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xda444 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xda450 | load | 4 | `abs 0xda76c` | `-` |
| `ldr` at 0xda454 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xda478 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xda47c | load | 4 | `abs 0xda76c` | `-` |
| `ldr` at 0xda480 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xda4a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xda4ac | load | 4 | `abs 0xda76c` | `-` |
| `ldr` at 0xda4b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xda4d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xda4d8 | load | 4 | `abs 0xda76c` | `-` |
| `ldr` at 0xda4dc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xda500 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xda504 | load | 4 | `abs 0xda76c` | `-` |
| `ldr` at 0xda508 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xda528 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xda534 | load | 4 | `abs 0xda76c` | `-` |
| `ldr` at 0xda538 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xda558 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xda56c | store | 4 | `(unknown)` | `ret(0x1943c)` |
| `ldr` at 0xda570 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xda57c | load | 4 | `abs 0xda770` | `-` |
| `str` at 0xda580 | store | 4 | `sp -0x38` | `0x121744` |
| `ldr` at 0xda588 | load | 4 | `abs 0xda760` | `-` |
| `ldr` at 0xda598 | load | 4 | `abs 0xda774` | `-` |
| `str` at 0xda59c | store | 4 | `sp -0x2c` | `0xfffff7cc` |
| `str` at 0xda5a8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xda5ac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xda5b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xda5bc | load | 4 | `abs 0xda76c` | `-` |
| `ldr` at 0xda5c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xda5e0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xda5e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xda5f0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xda5f8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xda600 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xda604 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xda608 | load | 4 | `abs 0xda76c` | `-` |
| `ldr` at 0xda60c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xda62c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xda634 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xda63c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xda644 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xda64c | load | 1 | `(unknown)` | `-` |
| `strb` at 0xda650 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xda654 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xda65c | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xda664 | load | 4 | `abs 0xda76c` | `-` |
| `ldr` at 0xda668 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xda684 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0xda688 | load | 4 | `abs 0xda76c` | `-` |
| `ldr` at 0xda68c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xda6b0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xda6b4 | load | 4 | `abs 0xda76c` | `-` |
| `ldr` at 0xda6b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xda6d8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xda6dc | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0xda6e0 | load | 4 | `abs 0xda76c` | `-` |
| `ldr` at 0xda6e4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xda704 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0xda708 | store | 1 | `(unknown)` | `([sp-33]:1 & 0xff)` |
| `ldr` at 0xda70c | load | 4 | `abs 0xda76c` | `-` |
| `ldr` at 0xda710 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xda730 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xda734 | store | 4 | `(unknown)` | `[sp]` |
| `ldr` at 0xda738 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xda740 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xda74c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xda750 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0xda758 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xda758 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xda758 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xda758 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xda3b8 | `(unknown callee) 0x200a0` | `0x9` | `0x121700` | `0xd4` | `0x0` |
| `bl` at 0xda3f4 | `(unknown callee) 0x200a0` | `0x6` | `0x121700` | `0xd7` | `0x0` |
| `bl` at 0xda4d4 | `(unknown callee) 0x1970c` | `?` | `0x29c540` | `[sp-24]` | `?` |
| `bl` at 0xda564 | `(unknown callee) 0x1943c` | `?` | `0x29c540` | `[sp-24]` | `?` |
| `bl` at 0xda594 | `(unknown callee) 0x200a0` | `0x3` | `0x121700` | `0xec` | `0x0` |
| `bl` at 0xda73c | `(unknown callee) 0xda778` | `[sp-24]` | `0x29c540` | `?` | `[sp]` |
| `bl` at 0xda744 | `(unknown callee) 0xdd46c` | `[sp-24]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xda39c | `r2 != 0x0` | `r2 == 0x0` |
| `bls` at 0xda3c8 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xda44c | `? == [sp-32]` | `? != [sp-32]` |
| `beq` at 0xda4a8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xda530 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xda578 | `? != 0x0` | `? == 0x0` |
| `bhs` at 0xda5b8 | `C == 1` | `not (C == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xda758 | `r0` | `?` |

## `JTAG_Cfg_calc_ir_vals`

- `JTAG_Cfg_calc_ir_vals`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xda778 to 0xda900.
  Established. An included halfword store has unknown effective target and value (`strh` at 0xda83c).
  Unsigned `[0x29737c] <= 0x3` takes `0xda8e0`, while larger values fall through (`bls` at 0xda8cc).
  The indirect call carries `0x4`, `0x29c540`, unknown, `[0x12da6c]` (`mov` at 0xda8dc).
  Scan instruction-length accumulation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xda788 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xda78c | load | 4 | `abs 0xda8e8` | `-` |
| `ldr` at 0xda790 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xda79c | load | 4 | `abs 0xda8ec` | `-` |
| `str` at 0xda7a0 | store | 4 | `sp -0x24` | `0x1217c8` |
| `ldr` at 0xda7a4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xda7a8 | store | 4 | `sp -0x20` | `r0` |
| `ldr` at 0xda7b0 | load | 4 | `abs 0xda8f0` | `-` |
| `ldr` at 0xda7b4 | load | 4 | `abs 0xda8f4` | `-` |
| `str` at 0xda7c4 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0xda7cc | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xda7d0 | load | 4 | `abs 0xda8f8` | `-` |
| `ldr` at 0xda7d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xda7f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xda7f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xda804 | load | 4 | `abs 0xda8f8` | `-` |
| `ldr` at 0xda808 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xda828 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xda830 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xda838 | load | 4 | `sp -0x1c` | `-` |
| `strh` at 0xda83c | store | 2 | `(unknown)` | `?` |
| `ldr` at 0xda840 | load | 4 | `abs 0xda8f8` | `-` |
| `ldr` at 0xda844 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xda864 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xda86c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xda874 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xda878 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xda880 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xda884 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xda88c | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0xda894 | load | 4 | `abs 0xda8f8` | `-` |
| `ldr` at 0xda898 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xda8b8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xda8bc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xda8c0 | load | 4 | `abs 0xda8e8` | `-` |
| `ldr` at 0xda8c4 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xda8d0 | load | 4 | `abs 0xda8fc` | `-` |
| `ldr` at 0xda8d4 | load | 4 | `abs 0x12da6c` | `-` |
| `ldm` at 0xda8e4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xda8e4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xda8e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xda7bc | `(unknown callee) 0x200a0` | `0x6` | `0x121700` | `0x113` | `0x0` |
| `mov` at 0xda8dc | `(unknown callee) (unknown target)` | `0x4` | `0x29c540` | `?` | `[0x12da6c]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xda798 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bls` at 0xda800 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bls` at 0xda8cc | `[0x29737c] <= 0x3 (unsigned, 32-bit)` | `[0x29737c] > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xda8e4 | `r0` | `?` |

## `JTAG_Cfg_set_ports_f`

- `JTAG_Cfg_set_ports_f`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xda900 to 0xdab04.
  Established. The word at absolute `0x29738c` receives `[sp-24]` (`str` at 0xdaa5c).
  Unsigned `[0x29737c] <= 0x3` takes `0xda954`, while larger values fall through (`bls` at 0xda924).
  The unnamed `0xd702c` call has four unknown candidates (`bl` at 0xdaa80).
  Scan port selection and cached state remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xda910 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xda914 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xda918 | load | 4 | `abs 0xdaadc` | `-` |
| `ldr` at 0xda91c | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xda928 | load | 4 | `abs 0xdaae0` | `-` |
| `str` at 0xda92c | store | 4 | `sp -0x38` | `0x1217e8` |
| `ldr` at 0xda930 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xda934 | store | 4 | `sp -0x34` | `r0` |
| `ldr` at 0xda938 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xda93c | store | 4 | `sp -0x30` | `r1` |
| `ldr` at 0xda944 | load | 4 | `abs 0xdaae4` | `-` |
| `ldr` at 0xda948 | load | 4 | `abs 0xdaae8` | `-` |
| `ldr` at 0xda954 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xda960 | load | 4 | `abs 0xdaaec` | `-` |
| `ldr` at 0xda964 | load | 4 | `abs 0x29738c` | `-` |
| `str` at 0xda968 | store | 4 | `sp -0x18` | `[0x29738c]` |
| `ldr` at 0xda96c | load | 4 | `abs 0xdaaf0` | `-` |
| `ldr` at 0xda970 | load | 4 | `abs 0x297388` | `-` |
| `str` at 0xda974 | store | 4 | `sp -0x14` | `[0x297388]` |
| `ldr` at 0xda97c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xda98c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xda994 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xda998 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xda9a0 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xda9a4 | load | 4 | `abs 0xdaaf0` | `-` |
| `ldr` at 0xda9a8 | load | 4 | `abs 0x297388` | `-` |
| `ldr` at 0xda9ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xda9b8 | load | 4 | `abs 0xdaaec` | `-` |
| `ldr` at 0xda9bc | load | 4 | `abs 0x29738c` | `-` |
| `ldr` at 0xda9c0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xda9d0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xda9d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xda9e4 | load | 4 | `abs 0xdaaf4` | `-` |
| `str` at 0xda9e8 | store | 4 | `sp -0x38` | `0x12181c` |
| `ldr` at 0xda9ec | load | 4 | `sp -0x18` | `-` |
| `str` at 0xda9f0 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xda9f4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xda9f8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xdaa00 | load | 4 | `abs 0xdaae4` | `-` |
| `ldr` at 0xdaa04 | load | 4 | `abs 0xdaaf8` | `-` |
| `ldr` at 0xdaa10 | load | 4 | `abs 0xdaafc` | `-` |
| `str` at 0xdaa14 | store | 4 | `sp -0x24` | `0xfffff7c3` |
| `ldr` at 0xdaa1c | load | 4 | `abs 0xdaaf0` | `-` |
| `str` at 0xdaa20 | store | 4 | `sp -0x28` | `0x297388` |
| `ldr` at 0xdaa24 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdaa28 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xdaa2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdaa3c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdaa44 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xdaa48 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdaa4c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xdaa50 | store | 4 | `abs 0x297388` | `?` |
| `ldr` at 0xdaa54 | load | 4 | `abs 0xdaaec` | `-` |
| `ldr` at 0xdaa58 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdaa5c | store | 4 | `abs 0x29738c` | `[sp-24]` |
| `ldr` at 0xdaa60 | load | 4 | `abs 0xdaaf0` | `-` |
| `ldr` at 0xdaa64 | load | 4 | `abs 0x297388` | `-` |
| `ldr` at 0xdaa6c | load | 4 | `abs 0xdaaec` | `-` |
| `ldr` at 0xdaa70 | load | 4 | `abs 0x29738c` | `-` |
| `ldr` at 0xdaa84 | load | 4 | `abs 0xdaadc` | `-` |
| `ldr` at 0xdaa88 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xdaa94 | load | 4 | `abs 0xdab00` | `-` |
| `str` at 0xdaa98 | store | 4 | `sp -0x38` | `0x12185c` |
| `ldr` at 0xdaa9c | load | 4 | `abs 0xdaaf0` | `-` |
| `ldr` at 0xdaaa0 | load | 4 | `abs 0x297388` | `-` |
| `str` at 0xdaaa4 | store | 4 | `sp -0x34` | `[0x297388]` |
| `ldr` at 0xdaaa8 | load | 4 | `abs 0xdaaec` | `-` |
| `ldr` at 0xdaaac | load | 4 | `abs 0x29738c` | `-` |
| `str` at 0xdaab0 | store | 4 | `sp -0x30` | `[0x29738c]` |
| `ldr` at 0xdaab8 | load | 4 | `abs 0xdaae4` | `-` |
| `str` at 0xdaacc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xdaad0 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xdaad8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdaad8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdaad8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xda950 | `(unknown callee) 0x200a0` | `0x6` | `0x121700` | `0x12a` | `0x0` |
| `bl` at 0xdaa0c | `(unknown callee) 0x200a0` | `0x3` | `0x121700` | `0x141` | `0x0` |
| `bl` at 0xdaa80 | `(unknown callee) 0xd702c` | `?` | `?` | `?` | `?` |
| `bl` at 0xdaac4 | `(unknown callee) 0x200a0` | `0x6` | `0x121700` | `0x160` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xda924 | `[0x29737c] <= 0x3 (unsigned, 32-bit)` | `[0x29737c] > 0x3 (unsigned, 32-bit)` |
| `bne` at 0xda95c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xda984 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bne` at 0xda9b4 | `[0x297388] != [sp-20]` | `[0x297388] == [sp-20]` |
| `bne` at 0xda9c8 | `[0x29738c] != [sp-24]` | `[0x29738c] == [sp-24]` |
| `bls` at 0xda9e0 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xdaa34 | `Z == 0` | `not (Z == 0)` |
| `bls` at 0xdaa90 | `[0x29737c] <= 0x3 (unsigned, 32-bit)` | `[0x29737c] > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdaad8 | `r0` | `?` |

## `JTAG_Cfg_find_chip`

- `JTAG_Cfg_find_chip`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdab04 to 0xdad98.
  Established. Unsigned `[0x29737c] <= 0x2` takes `0xdab70`, while larger values fall through (`bls` at 0xdab30).
  Included byte loads have unknown effective targets (`ldrb` at 0xdac84, `ldrb` at 0xdad28).
  The return record gives `[sp-44]` (`ldm` at 0xdad80).
  Chip lookup predicate meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdab14 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdab18 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xdab1c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xdab20 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xdab24 | load | 4 | `abs 0xdad84` | `-` |
| `ldr` at 0xdab28 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xdab34 | load | 4 | `abs 0xdad88` | `-` |
| `str` at 0xdab38 | store | 4 | `sp -0x40` | `0x121890` |
| `ldr` at 0xdab3c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdab40 | store | 4 | `sp -0x3c` | `r0` |
| `ldr` at 0xdab44 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdab48 | store | 4 | `sp -0x38` | `r1` |
| `ldr` at 0xdab4c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdab50 | store | 4 | `sp -0x34` | `r2` |
| `ldr` at 0xdab54 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdab58 | store | 4 | `sp -0x30` | `r3` |
| `ldr` at 0xdab60 | load | 4 | `abs 0xdad8c` | `-` |
| `ldr` at 0xdab64 | load | 4 | `abs 0xdad90` | `-` |
| `ldr` at 0xdab70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdab7c | load | 4 | `abs 0xdad94` | `-` |
| `ldr` at 0xdab80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdaba0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xdabb0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xdabb8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdabc4 | load | 4 | `abs 0xdad94` | `-` |
| `ldr` at 0xdabc8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdabe8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdabec | load | 4 | `(unknown)` | `-` |
| `str` at 0xdac00 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xdac08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdac14 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdac18 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xdac1c | load | 4 | `abs 0xdad94` | `-` |
| `ldr` at 0xdac20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdac40 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdac44 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdac50 | load | 4 | `abs 0xdad94` | `-` |
| `ldr` at 0xdac54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdac74 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdac7c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xdac84 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xdac88 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdac94 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdac98 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xdaca0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdaca8 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xdacb4 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xdacbc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xdacc0 | load | 4 | `abs 0xdad94` | `-` |
| `ldr` at 0xdacc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdace4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdace8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdacf4 | load | 4 | `abs 0xdad94` | `-` |
| `ldr` at 0xdacf8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdad18 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdad20 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xdad28 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xdad2c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdad38 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdad3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdad48 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdad4c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xdad54 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xdad5c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xdad60 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdad68 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xdad74 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xdad78 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0xdad80 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdad80 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdad80 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdab6c | `(unknown callee) 0x200a0` | `0x6` | `0x121700` | `0x17b` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xdab30 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bhi` at 0xdab78 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0xdaba8 | `? != 0x0` | `? == 0x0` |
| `blt` at 0xdabc0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bhs` at 0xdabf4 | `C == 1` | `not (C == 1)` |
| `bne` at 0xdac10 | `Z == 0` | `not (Z == 0)` |
| `bhs` at 0xdac4c | `C == 1` | `not (C == 1)` |
| `bne` at 0xdac90 | `Z == 0` | `not (Z == 0)` |
| `bhs` at 0xdacf0 | `C == 1` | `not (C == 1)` |
| `bne` at 0xdad34 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xdad44 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdad80 | `r0` | `[sp-44]` |

## `JTAG_Cfg_pid`

- `JTAG_Cfg_pid`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdad98 to 0xdadd8.
  Established. Entry `r0` is saved in the word at `sp -0x14` (`str` at 0xdada8).
  A later word load has unknown effective target (`ldr` at 0xdadc8).
  The integer return value is unknown (`ldm` at 0xdadd0).
  Configuration identifier lookup semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdada8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xdadac | load | 4 | `abs 0xdadd4` | `-` |
| `ldr` at 0xdadb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdadc8 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xdadd0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xdadd0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdadd0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdadd0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdadd0 | `r0` | `?` |
