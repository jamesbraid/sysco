# ARM controller commands and FRU identity in 1.44.0

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
Computed transfers leave unresolved edges in the margin, test and identity routines, identified below. Tables cover known control-flow edges only.
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/controller-command-fru-identity-facts.txt) retain all tool sections.

## `SMPCmdLibrary`

- `SMPCmdLibrary`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8d178 to 0x8e46c.
  Established. Included word stores write `ret(0x1943c)` to `[sp-28] +0x0` and clear `[[sp-28]] +0x0` (`str` at 0x8d210, `str` at 0x8d224).
  An indirect call with unknown target carries `sp-2664`, unknown, `sp-2664`, unknown (`mov` at 0x8e384).
  Command-library response ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8d18c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x8d190 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x8d194 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x8d19c | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0x8d1a4 | store | 4 | `sp -0xa6c` | `0x0` |
| `str` at 0x8d1ac | store | 4 | `sp -0xa70` | `0x0` |
| `str` at 0x8d1b4 | store | 4 | `sp -0xa74` | `0x0` |
| `str` at 0x8d1c4 | store | 4 | `sp -0xa88` | `ret(0x6a0f8)` |
| `ldr` at 0x8d1c8 | load | 4 | `sp -0xa88` | `-` |
| `ldr` at 0x8d1cc | load | 4 | `ret(0x6a0f8) +0x8` | `-` |
| `ldr` at 0x8d1dc | load | 4 | `sp -0xa88` | `-` |
| `ldr` at 0x8d1e0 | load | 4 | `ret(0x6a0f8) +0xc` | `-` |
| `str` at 0x8d1fc | store | 4 | `sp -0xa8c` | `?` |
| `ldr` at 0x8d200 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x8d210 | store | 4 | `[sp-28] +0x0` | `ret(0x1943c)` |
| `ldr` at 0x8d214 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x8d218 | store | 4 | `sp -0xa78` | `[[sp-28]]` |
| `ldr` at 0x8d21c | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x8d224 | store | 4 | `[[sp-28]] +0x0` | `0x0` |
| `ldr` at 0x8d22c | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x8d238 | store | 4 | `[sp-2680] +0x4` | `0x0` |
| `ldr` at 0x8d24c | load | 4 | `abs 0x8e0e0` | `-` |
| `ldr` at 0x8d254 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x8d258 | store | 4 | `sp -0xa68` | `[sp-32]` |
| `str` at 0x8d260 | store | 4 | `sp -0xac8` | `0x0` |
| `str` at 0x8d26c | store | 4 | `sp -0xac4` | `sp-2664` |
| `ldr` at 0x8d274 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d278 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x8d288 | store | 4 | `sp -0x2c` | `ret(0xaed30)` |
| `ldr` at 0x8d28c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x8d298 | load | 4 | `sp -0xa60` | `-` |
| `ldr` at 0x8d2a4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x8d2a8 | store | 4 | `sp -0xa98` | `ret(0xaed30)` |
| `ldr` at 0x8d2b0 | load | 4 | `abs 0x8e134` | `-` |
| `str` at 0x8d2c0 | store | 4 | `sp -0x24` | `ret(0x20cd0)` |
| `ldr` at 0x8d2c4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8d2d0 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x8d2d4 | load | 4 | `[sp-2680] +0x0` | `-` |
| `ldr` at 0x8d2d8 | load | 4 | `abs 0x8e0e4` | `-` |
| `str` at 0x8d2e4 | store | 4 | `sp -0xa98` | `?` |
| `ldr` at 0x8d2ec | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x8d2f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8d2f8 | load | 4 | `sp -0xa60` | `-` |
| `str` at 0x8d310 | store | 4 | `[sp-28] +0x0` | `ret(0x1943c)` |
| `ldr` at 0x8d314 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x8d318 | store | 4 | `sp -0xa78` | `[[sp-28]]` |
| `str` at 0x8d320 | store | 4 | `sp -0xa7c` | `0x0` |
| `ldr` at 0x8d324 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x8d32c | store | 4 | `[[sp-28]] +0x0` | `0x0` |
| `ldr` at 0x8d330 | load | 4 | `sp -0xa60` | `-` |
| `str` at 0x8d338 | store | 4 | `sp -0x450` | `[sp-2656]-1` |
| `ldr` at 0x8d33c | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x8d348 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x8d34c | load | 4 | `abs 0x8e0e8` | `-` |
| `ldr` at 0x8d360 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8d36c | load | 4 | `abs 0x8e0ec` | `-` |
| `str` at 0x8d370 | store | 4 | `sp -0xa90` | `0xffff003` |
| `ldr` at 0x8d374 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x8d378 | load | 4 | `abs 0x8e128` | `-` |
| `ldr` at 0x8d38c | load | 4 | `(unknown)` | `-` |
| `str` at 0x8d390 | store | 4 | `sp -0xa94` | `?` |
| `ldr` at 0x8d394 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x8d398 | load | 4 | `abs 0x8e0f0` | `-` |
| `ldr` at 0x8d3ac | load | 4 | `abs 0x8e0f4` | `-` |
| `ldrh` at 0x8d3b0 | load | 2 | `abs 0x29a6ba` | `-` |
| `str` at 0x8d3b4 | store | 4 | `(unknown)` | `[0x29a6ba]:2` |
| `ldr` at 0x8d3b8 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x8d3bc | load | 4 | `abs 0x8e0f8` | `-` |
| `str` at 0x8d3d4 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x8d3f0 | load | 4 | `sp -0xa64` | `-` |
| `str` at 0x8d40c | store | 4 | `sp -0xac8` | `0x1` |
| `ldr` at 0x8d414 | load | 4 | `sp -0xa64` | `-` |
| `str` at 0x8d42c | store | 4 | `sp -0xac8` | `0x2` |
| `ldr` at 0x8d430 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x8d434 | store | 4 | `sp -0xac4` | `[sp-76]` |
| `str` at 0x8d43c | store | 4 | `sp -0xac0` | `sp-72` |
| `ldr` at 0x8d440 | load | 4 | `abs 0x8e0f4` | `-` |
| `str` at 0x8d458 | store | 4 | `sp -0x30` | `ret(0xaa424)` |
| `ldr` at 0x8d45c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x8d46c | load | 4 | `sp -0xa94` | `-` |
| `ldr` at 0x8d480 | load | 4 | `abs 0x8e0fc` | `-` |
| `ldr` at 0x8d484 | load | 4 | `sp -0xa94` | `-` |
| `ldr` at 0x8d498 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8d4c0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x8d4c8 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8d4d0 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x8d4e4 | store | 4 | `(unknown)` | `ret(0x1943c)` |
| `ldr` at 0x8d4e8 | load | 4 | `abs 0x8e0f4` | `-` |
| `ldr` at 0x8d4ec | load | 4 | `abs 0x29a6b4` | `-` |
| `ldr` at 0x8d4f8 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8d500 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x8d508 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8d510 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8d518 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x8d520 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8d524 | load | 4 | `abs 0x8e100` | `-` |
| `ldr` at 0x8d52c | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8d534 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x8d53c | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8d544 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x8d54c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8d55c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8d564 | load | 4 | `sp -0xa94` | `-` |
| `ldr` at 0x8d57c | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8d584 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x8d58c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8d590 | load | 4 | `abs 0x8e12c` | `-` |
| `ldr` at 0x8d598 | load | 4 | `sp -0xa7c` | `-` |
| `str` at 0x8d5a0 | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x8d5a8 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x8d5b4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x8d5b8 | load | 4 | `sp -0xa6c` | `-` |
| `str` at 0x8d5c0 | store | 4 | `sp -0xa6c` | `[sp-2668]+1` |
| `ldr` at 0x8d5c4 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x8d5c8 | load | 4 | `abs 0x8e0f8` | `-` |
| `str` at 0x8d5e0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x8d5e8 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x8d5ec | load | 4 | `abs 0x8e128` | `-` |
| `ldr` at 0x8d600 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8d610 | load | 4 | `abs 0x8e0f4` | `-` |
| `ldr` at 0x8d614 | load | 4 | `abs 0x29a6b4` | `-` |
| `str` at 0x8d630 | store | 4 | `sp -0xa94` | `0x0` |
| `str` at 0x8d63c | store | 4 | `sp -0xac8` | `0x0` |
| `str` at 0x8d648 | store | 4 | `sp -0xac4` | `sp-2664` |
| `ldr` at 0x8d650 | load | 4 | `sp -0xa64` | `-` |
| `str` at 0x8d660 | store | 4 | `sp -0x2c` | `ret(0xaed30)` |
| `ldr` at 0x8d664 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x8d670 | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x8d678 | store | 4 | `sp -0xa74` | `[sp-2676]+1` |
| `ldr` at 0x8d67c | load | 4 | `sp -0xa94` | `-` |
| `ldr` at 0x8d688 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8d690 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x8d6a4 | store | 4 | `(unknown)` | `ret(0x1943c)` |
| `ldr` at 0x8d6a8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x8d6b4 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x8d6b8 | load | 4 | `abs 0x8e128` | `-` |
| `ldr` at 0x8d6cc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8d6d8 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8d6e0 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x8d6e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8d6f0 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8d6f8 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x8d700 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8d704 | load | 4 | `abs 0x8e104` | `-` |
| `ldr` at 0x8d70c | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8d714 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x8d71c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8d720 | load | 4 | `sp -0xa94` | `-` |
| `ldr` at 0x8d728 | load | 4 | `sp -0xa7c` | `-` |
| `str` at 0x8d730 | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x8d738 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x8d744 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x8d74c | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x8d750 | load | 4 | `abs 0x8e128` | `-` |
| `ldr` at 0x8d764 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8d770 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8d778 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x8d780 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8d790 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8d79c | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x8d7a8 | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x8d7ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8d7b0 | load | 4 | `abs 0x8e108` | `-` |
| `ldr` at 0x8d7b8 | load | 4 | `sp -0xa94` | `-` |
| `ldr` at 0x8d7c8 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8d7d0 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x8d7d8 | store | 4 | `(unknown)` | `ret(0x192bc)` |
| `ldr` at 0x8d7dc | load | 4 | `sp -0xa7c` | `-` |
| `str` at 0x8d7e4 | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x8d7ec | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x8d7f8 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x8d7fc | load | 4 | `sp -0xa94` | `-` |
| `ldr` at 0x8d804 | load | 4 | `sp -0xa6c` | `-` |
| `str` at 0x8d80c | store | 4 | `sp -0xa6c` | `[sp-2668]+1` |
| `ldr` at 0x8d810 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x8d814 | load | 4 | `abs 0x8e0f8` | `-` |
| `str` at 0x8d82c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x8d834 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x8d838 | load | 4 | `abs 0x8e0f8` | `-` |
| `str` at 0x8d850 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x8d86c | load | 4 | `sp -0xa64` | `-` |
| `str` at 0x8d888 | store | 4 | `sp -0xac8` | `0x1` |
| `ldr` at 0x8d890 | load | 4 | `sp -0xa64` | `-` |
| `ldr` at 0x8d8a4 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x8d8bc | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x8d8c0 | store | 4 | `sp -0xac8` | `[sp-76]` |
| `str` at 0x8d8c8 | store | 4 | `sp -0xac4` | `sp-72` |
| `ldr` at 0x8d8d4 | load | 4 | `sp -0x450` | `-` |
| `str` at 0x8d8e0 | store | 4 | `sp -0xac0` | `?` |
| `str` at 0x8d8e8 | store | 4 | `sp -0xabc` | `0x0` |
| `ldr` at 0x8d8ec | load | 4 | `abs 0x8e0f4` | `-` |
| `str` at 0x8d900 | store | 4 | `sp -0x24` | `ret(0xa9f4c)` |
| `ldr` at 0x8d904 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8d914 | load | 4 | `sp -0x65c` | `-` |
| `ldr` at 0x8d920 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x8d924 | load | 4 | `abs 0x8e0f8` | `-` |
| `str` at 0x8d93c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x8d940 | load | 4 | `sp -0xa6c` | `-` |
| `str` at 0x8d948 | store | 4 | `sp -0xa6c` | `[sp-2668]+1` |
| `ldr` at 0x8d950 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8d958 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x8d96c | store | 4 | `(unknown)` | `ret(0x1943c)` |
| `ldr` at 0x8d970 | load | 4 | `abs 0x8e0f4` | `-` |
| `ldr` at 0x8d974 | load | 4 | `abs 0x29a6b4` | `-` |
| `ldr` at 0x8d980 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8d988 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x8d990 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8d998 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8d9a0 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x8d9a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8d9ac | load | 4 | `abs 0x8e10c` | `-` |
| `ldr` at 0x8d9b4 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8d9bc | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x8d9c4 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8d9cc | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x8d9d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8d9e4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8d9ec | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x8d9f0 | load | 4 | `abs 0x8e128` | `-` |
| `ldr` at 0x8da04 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8da1c | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8da24 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x8da2c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8da30 | load | 4 | `abs 0x8e12c` | `-` |
| `ldr` at 0x8da38 | load | 4 | `sp -0xa7c` | `-` |
| `str` at 0x8da40 | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x8da48 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x8da54 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x8da58 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x8da5c | load | 4 | `abs 0x8e0f8` | `-` |
| `str` at 0x8da74 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x8da78 | load | 4 | `sp -0xa6c` | `-` |
| `str` at 0x8da80 | store | 4 | `sp -0xa6c` | `[sp-2668]+1` |
| `ldr` at 0x8da84 | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x8da8c | store | 4 | `sp -0xa74` | `[sp-2676]+1` |
| `ldr` at 0x8da90 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8da9c | load | 4 | `abs 0x8e110` | `-` |
| `ldr` at 0x8daa8 | load | 4 | `sp -0x450` | `-` |
| `str` at 0x8dab0 | store | 4 | `sp -0x450` | `[sp-1104]-1` |
| `ldr` at 0x8dab8 | load | 4 | `sp -0xa64` | `-` |
| `ldr` at 0x8dac0 | load | 4 | `abs 0x8e114` | `-` |
| `str` at 0x8dac4 | store | 4 | `sp -0x28` | `0x44c` |
| `ldr` at 0x8dac8 | load | 4 | `sp -0xa60` | `-` |
| `ldr` at 0x8dacc | load | 4 | `sp -0xa6c` | `-` |
| `str` at 0x8dadc | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x8dae8 | load | 4 | `abs 0x8e118` | `-` |
| `ldr` at 0x8daec | load | 4 | `abs 0x8e114` | `-` |
| `str` at 0x8daf8 | store | 4 | `sp -0x24` | `ret(0xa8b38)` |
| `ldr` at 0x8dafc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x8db00 | store | 4 | `sp -0xaa0` | `ret(0xa8b38)` |
| `ldr` at 0x8db04 | load | 4 | `sp -0xaa0` | `-` |
| `ldr` at 0x8db10 | load | 4 | `sp -0xaa0` | `-` |
| `ldr` at 0x8db1c | load | 4 | `sp -0xaa0` | `-` |
| `ldr` at 0x8db2c | load | 4 | `sp -0xaa0` | `-` |
| `str` at 0x8db40 | store | 4 | `sp -0x450` | `0x0` |
| `ldr` at 0x8db44 | load | 4 | `sp -0xa60` | `-` |
| `ldr` at 0x8db48 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x8db54 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x8db58 | load | 4 | `abs 0x8e0f8` | `-` |
| `ldr` at 0x8db6c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8db78 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8db80 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x8db94 | store | 4 | `(unknown)` | `ret(0x1943c)` |
| `ldr` at 0x8db98 | load | 4 | `abs 0x8e0f4` | `-` |
| `ldr` at 0x8db9c | load | 4 | `abs 0x29a6b4` | `-` |
| `ldr` at 0x8dba8 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8dbb0 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x8dbb8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8dbc0 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8dbc8 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x8dbd0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8dbd4 | load | 4 | `abs 0x8e11c` | `-` |
| `ldr` at 0x8dbdc | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8dbe4 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x8dbec | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8dbf4 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x8dbfc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8dc0c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8dc14 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x8dc18 | load | 4 | `abs 0x8e128` | `-` |
| `ldr` at 0x8dc2c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8dc44 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8dc4c | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x8dc54 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8dc58 | load | 4 | `abs 0x8e12c` | `-` |
| `ldr` at 0x8dc60 | load | 4 | `sp -0xa7c` | `-` |
| `str` at 0x8dc68 | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x8dc70 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x8dc7c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x8dc80 | load | 4 | `sp -0x450` | `-` |
| `str` at 0x8dc88 | store | 4 | `sp -0x450` | `?` |
| `ldr` at 0x8dc90 | load | 4 | `sp -0xa60` | `-` |
| `str` at 0x8dc94 | store | 4 | `sp -0xa6c` | `[sp-2656]` |
| `ldr` at 0x8dc9c | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x8dca0 | load | 1 | `[sp-48] +0x13` | `-` |
| `ldr` at 0x8dcac | load | 4 | `sp -0x30` | `-` |
| `str` at 0x8dcb8 | store | 4 | `sp -0xa94` | `ret(0xaadf4)` |
| `ldr` at 0x8dcbc | load | 4 | `sp -0xa94` | `-` |
| `ldr` at 0x8dcc0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x8dccc | load | 4 | `sp -0xa94` | `-` |
| `str` at 0x8dcd0 | store | 4 | `sp -0x28` | `ret(0xaadf4)` |
| `ldr` at 0x8dcd4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x8dce0 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x8dce4 | load | 1 | `[sp-48] +0x12` | `-` |
| `str` at 0x8dce8 | store | 4 | `sp -0xa80` | `[[sp-48]+18]:1` |
| `ldr` at 0x8dcec | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x8dcf0 | load | 1 | `[sp-48] +0x14` | `-` |
| `ldr` at 0x8dcf4 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x8dcf8 | load | 1 | `[sp-48] +0x21` | `-` |
| `str` at 0x8dd04 | store | 4 | `sp -0xa84` | `?` |
| `str` at 0x8dd0c | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x8dd14 | store | 4 | `sp -0x450` | `0x0` |
| `ldr` at 0x8dd18 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8dd24 | load | 4 | `sp -0xa60` | `-` |
| `ldr` at 0x8dd28 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x8dd34 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x8dd38 | load | 4 | `abs 0x8e0f0` | `-` |
| `ldr` at 0x8dd4c | load | 4 | `(unknown)` | `-` |
| `str` at 0x8dd50 | store | 4 | `sp -0xa9c` | `?` |
| `ldr` at 0x8dd54 | load | 4 | `sp -0xa80` | `-` |
| `ldr` at 0x8dd60 | load | 4 | `sp -0xa84` | `-` |
| `ldr` at 0x8dd68 | load | 4 | `sp -0xa9c` | `-` |
| `ldr` at 0x8dd78 | load | 4 | `sp -0xa84` | `-` |
| `ldr` at 0x8dd84 | load | 4 | `sp -0xa9c` | `-` |
| `ldr` at 0x8dd94 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x8dd98 | load | 4 | `abs 0x8e0f8` | `-` |
| `ldr` at 0x8ddac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8ddb8 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x8ddbc | load | 4 | `abs 0x8e0f8` | `-` |
| `str` at 0x8ddd4 | store | 4 | `(unknown)` | `0x0` |
| `str` at 0x8dddc | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x8dde0 | load | 4 | `sp -0x450` | `-` |
| `str` at 0x8dde8 | store | 4 | `sp -0x450` | `?` |
| `ldr` at 0x8ddf0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8ddfc | load | 4 | `sp -0x30` | `-` |
| `str` at 0x8de08 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x8de10 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8de1c | load | 4 | `abs 0x8e120` | `-` |
| `ldr` at 0x8de24 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8de2c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x8de38 | load | 4 | `sp -0x658` | `-` |
| `ldr` at 0x8de44 | load | 4 | `sp -0x654` | `-` |
| `ldr` at 0x8de48 | load | 4 | `abs 0x8e130` | `-` |
| `ldr` at 0x8de5c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x8de60 | store | 4 | `(unknown)` | `[sp-48]` |
| `ldr` at 0x8de64 | load | 4 | `sp -0x654` | `-` |
| `str` at 0x8de6c | store | 4 | `sp -0x654` | `[sp-1620]+1` |
| `ldr` at 0x8de70 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x8de74 | load | 1 | `[sp-48] +0x13` | `-` |
| `ldr` at 0x8de84 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x8de88 | load | 1 | `[sp-48] +0x24` | `-` |
| `ldr` at 0x8de98 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x8de9c | load | 4 | `[sp-48] +0x28` | `-` |
| `str` at 0x8dea0 | store | 4 | `sp -0xaa4` | `[[sp-48]+40]` |
| `ldr` at 0x8dea8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x8deb4 | store | 4 | `sp -0xaa4` | `ret(0xaadf4)` |
| `ldr` at 0x8deb8 | load | 4 | `sp -0xaa4` | `-` |
| `str` at 0x8debc | store | 4 | `sp -0x2c` | `[sp-2724]` |
| `ldr` at 0x8dec8 | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x8ded0 | store | 4 | `sp -0xa74` | `[sp-2676]+1` |
| `ldr` at 0x8ded4 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x8ded8 | load | 1 | `[sp-48] +0x20` | `-` |
| `ldr` at 0x8dee4 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8deec | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x8df00 | store | 4 | `(unknown)` | `ret(0x1943c)` |
| `ldr` at 0x8df04 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x8df10 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x8df14 | load | 4 | `[sp-48] +0x1c` | `-` |
| `ldr` at 0x8df20 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8df28 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x8df30 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8df38 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8df40 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x8df48 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8df4c | load | 4 | `abs 0x8e104` | `-` |
| `ldr` at 0x8df54 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x8df58 | load | 1 | `[sp-48] +0x2c` | `-` |
| `ldr` at 0x8df68 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x8df6c | load | 4 | `[sp-48] +0x30` | `-` |
| `str` at 0x8df70 | store | 4 | `sp -0xaa8` | `[[sp-48]+48]` |
| `ldr` at 0x8df78 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x8df84 | store | 4 | `sp -0xaa8` | `ret(0xaadf4)` |
| `ldr` at 0x8df88 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8df90 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x8df98 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8df9c | load | 4 | `sp -0xaa8` | `-` |
| `ldr` at 0x8dfa4 | load | 4 | `sp -0xa7c` | `-` |
| `str` at 0x8dfac | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x8dfb4 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x8dfc0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x8dfc8 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x8dfcc | load | 1 | `[sp-48] +0x20` | `-` |
| `ldr` at 0x8dfd8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x8dfdc | load | 4 | `[sp-48] +0x1c` | `-` |
| `ldr` at 0x8dfe8 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8dff0 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x8dff8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8e008 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8e014 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x8e020 | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x8e024 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8e028 | load | 4 | `abs 0x8e108` | `-` |
| `ldr` at 0x8e030 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x8e034 | load | 1 | `[sp-48] +0x2c` | `-` |
| `ldr` at 0x8e044 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x8e048 | load | 4 | `[sp-48] +0x30` | `-` |
| `str` at 0x8e04c | store | 4 | `sp -0xaac` | `[[sp-48]+48]` |
| `ldr` at 0x8e054 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x8e060 | store | 4 | `sp -0xaac` | `ret(0xaadf4)` |
| `ldr` at 0x8e064 | load | 4 | `sp -0xaac` | `-` |
| `ldr` at 0x8e074 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8e07c | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x8e084 | store | 4 | `(unknown)` | `ret(0x192bc)` |
| `ldr` at 0x8e088 | load | 4 | `sp -0xa7c` | `-` |
| `str` at 0x8e090 | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x8e098 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x8e0a4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x8e0ac | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x8e0b0 | load | 1 | `[sp-48] +0x20` | `-` |
| `ldr` at 0x8e0bc | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x8e0c0 | load | 1 | `[sp-48] +0x2c` | `-` |
| `ldr` at 0x8e0d0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x8e0d4 | load | 4 | `[sp-48] +0x30` | `-` |
| `str` at 0x8e0d8 | store | 4 | `sp -0xab0` | `[[sp-48]+48]` |
| `ldr` at 0x8e144 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x8e150 | store | 4 | `sp -0xab0` | `ret(0xaadf4)` |
| `ldr` at 0x8e154 | load | 4 | `sp -0xab0` | `-` |
| `ldr` at 0x8e16c | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8e178 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x8e180 | store | 4 | `(unknown)` | `ret(0x192bc)` |
| `str` at 0x8e188 | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x8e18c | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x8e190 | load | 1 | `[sp-48] +0x34` | `-` |
| `ldr` at 0x8e1a0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x8e1a4 | load | 4 | `[sp-48] +0x38` | `-` |
| `str` at 0x8e1a8 | store | 4 | `sp -0xab4` | `[[sp-48]+56]` |
| `ldr` at 0x8e1b0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x8e1bc | store | 4 | `sp -0xab4` | `ret(0xaadf4)` |
| `ldr` at 0x8e1c0 | load | 4 | `sp -0xab4` | `-` |
| `ldr` at 0x8e1d8 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8e1e4 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x8e1ec | store | 4 | `(unknown)` | `ret(0x192bc)` |
| `str` at 0x8e1f4 | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x8e1f8 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8e200 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x8e20c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x8e214 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8e21c | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x8e230 | store | 4 | `(unknown)` | `ret(0x1943c)` |
| `ldr` at 0x8e234 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x8e238 | load | 4 | `[sp-48] +0x1c` | `-` |
| `ldr` at 0x8e244 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8e24c | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x8e254 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8e25c | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8e264 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x8e26c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8e270 | load | 4 | `abs 0x8e124` | `-` |
| `ldr` at 0x8e278 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8e280 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x8e288 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8e290 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x8e298 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8e2a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8e2b0 | load | 4 | `sp -0x450` | `-` |
| `str` at 0x8e2b8 | store | 4 | `sp -0x450` | `[sp-1104]-1` |
| `ldr` at 0x8e2bc | load | 4 | `abs 0x8e128` | `-` |
| `ldr` at 0x8e2d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8e2e8 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8e2f0 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x8e2f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8e2fc | load | 4 | `abs 0x8e12c` | `-` |
| `ldr` at 0x8e304 | load | 4 | `sp -0xa7c` | `-` |
| `str` at 0x8e30c | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x8e314 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x8e320 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x8e324 | load | 4 | `sp -0x658` | `-` |
| `ldr` at 0x8e330 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x8e33c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x8e340 | load | 4 | `sp -0xa6c` | `-` |
| `str` at 0x8e348 | store | 4 | `sp -0xa6c` | `[sp-2668]+1` |
| `ldr` at 0x8e350 | load | 4 | `sp -0x658` | `-` |
| `ldr` at 0x8e364 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x8e36c | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x8e374 | load | 4 | `sp -0x658` | `-` |
| `ldr` at 0x8e388 | load | 4 | `sp -0xa7c` | `-` |
| `str` at 0x8e390 | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x8e398 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x8e3a4 | store | 4 | `(unknown)` | `0x0` |
| `str` at 0x8e3ac | store | 4 | `sp -0xa6c` | `0x0` |
| `ldr` at 0x8e3b0 | load | 4 | `sp -0x654` | `-` |
| `ldr` at 0x8e3b4 | load | 4 | `sp -0xa6c` | `-` |
| `ldr` at 0x8e3c0 | load | 4 | `sp -0xa6c` | `-` |
| `ldr` at 0x8e3c4 | load | 4 | `abs 0x8e130` | `-` |
| `ldr` at 0x8e3d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8e3e0 | load | 4 | `sp -0xa6c` | `-` |
| `str` at 0x8e3e8 | store | 4 | `sp -0xa6c` | `[sp-2668]+1` |
| `ldr` at 0x8e3f0 | load | 4 | `abs 0x8e134` | `-` |
| `str` at 0x8e3fc | store | 4 | `sp -0x24` | `ret(0x2105c)` |
| `ldr` at 0x8e400 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8e40c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8e418 | load | 4 | `abs 0x8e138` | `-` |
| `str` at 0x8e41c | store | 4 | `sp -0xac8` | `0x10a4c4` |
| `str` at 0x8e420 | store | 4 | `sp -0xac4` | `ret(0x1f8e0)` |
| `ldr` at 0x8e428 | load | 4 | `abs 0x8e13c` | `-` |
| `ldr` at 0x8e42c | load | 4 | `abs 0x8e140` | `-` |
| `ldr` at 0x8e438 | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x8e448 | store | 4 | `sp -0xab8` | `?` |
| `str` at 0x8e454 | store | 4 | `sp -0xab8` | `0x0` |
| `ldr` at 0x8e458 | load | 4 | `sp -0xab8` | `-` |
| `str` at 0x8e45c | store | 4 | `sp -0xa98` | `[sp-2744]` |
| `ldr` at 0x8e460 | load | 4 | `sp -0xa98` | `-` |
| `ldm` at 0x8e468 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x8e468 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x8e468 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x8e468 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8d1bc | `(unknown callee) 0x6a0f8` | `0x0` | `r1` | `r2` | `0x0` |
| `bl` at 0x8d208 | `(unknown callee) 0x1943c` | `0x8` | `?` | `?` | `?` |
| `bl` at 0x8d250 | `(unknown callee) 0x19748` | `sp-2664` | `0x0` | `0x618` | `sp-2664` |
| `bl` at 0x8d280 | `(unknown callee) 0xaed30` | `0x1` | `[sp-24]` | `[sp-2680]` | `0x0` |
| `bl` at 0x8d2b8 | `(unknown callee) 0x20cd0` | `0x29a7f0` | `0x0` | `?` | `[sp-2656]` |
| `bl` at 0x8d2dc | `(unknown callee) 0x19808` | `[[sp-2680]]` | `0x10a3b4` | `?` | `[sp-2680]` |
| `bl` at 0x8d2f0 | `(unknown callee) 0x1970c` | `[sp-2680]` | `?` | `?` | `ret(0x20cd0)` |
| `bl` at 0x8d308 | `(unknown callee) 0x1943c` | `?` | `?` | `?` | `?` |
| `bl` at 0x8d3ec | `(unknown callee) 0x5eac8` | `sp-76` | `sp-72` | `0x0` | `sp-76` |
| `bl` at 0x8d3f4 | `(unknown callee) 0x19538` | `[sp-2660]` | `?` | `?` | `?` |
| `bl` at 0x8d41c | `(unknown callee) 0x5ed50` | `sp-76` | `sp-72` | `[sp-2660]` | `ret(0x19538)` |
| `bl` at 0x8d450 | `(unknown callee) 0xaa424` | `0x29a6b4` | `sp-2704` | `0x0` | `0x0` |
| `bl` at 0x8d470 | `(unknown callee) 0x6f4d4` | `[sp-2708]` | `?` | `?` | `ret(0xaa424)` |
| `bl` at 0x8d4b0 | `(unknown callee) 0x21d50` | `?` | `sp-48` | `0x64` | `?` |
| `bl` at 0x8d4c4 | `(unknown callee) 0xa8b98` | `[sp-48]` | `?` | `?` | `ret(0x21d50)` |
| `bl` at 0x8d4dc | `(unknown callee) 0x1943c` | `0x50` | `?` | `?` | `[sp-2680]` |
| `bl` at 0x8d4f0 | `(unknown callee) 0x5ef20` | `[0x29a6b4]` | `?` | `?` | `0x29a6b4` |
| `bl` at 0x8d50c | `(unknown callee) 0x19808` | `?` | `ret(0x5ef20)` | `?` | `?` |
| `bl` at 0x8d528 | `(unknown callee) 0x195e0` | `?` | `0x10a3dc` | `?` | `?` |
| `bl` at 0x8d550 | `(unknown callee) 0x19538` | `?` | `?` | `?` | `?` |
| `bl` at 0x8d568 | `(unknown callee) 0x5ef20` | `[sp-2708]` | `?` | `ret(0x19538)` | `?` |
| `bl` at 0x8d578 | `(unknown callee) 0x19808` | `?` | `ret(0x5ef20)` | `?` | `ret(0x5ef20)` |
| `bl` at 0x8d594 | `(unknown callee) 0x195e0` | `?` | `0x107d30` | `?` | `?` |
| `bl` at 0x8d658 | `(unknown callee) 0xaed30` | `0x1` | `[sp-2660]` | `sp-2708` | `0x0` |
| `bl` at 0x8d69c | `(unknown callee) 0x1943c` | `0x50` | `?` | `?` | `[sp-2680]` |
| `bl` at 0x8d6d0 | `(unknown callee) 0x5ef20` | `?` | `sp-20` | `0xfffff5bc` | `?` |
| `bl` at 0x8d6ec | `(unknown callee) 0x19808` | `?` | `ret(0x5ef20)` | `?` | `?` |
| `bl` at 0x8d708 | `(unknown callee) 0x195e0` | `?` | `0x10a404` | `?` | `?` |
| `bl` at 0x8d724 | `(unknown callee) 0x195e0` | `?` | `[sp-2708]` | `?` | `?` |
| `bl` at 0x8d768 | `(unknown callee) 0x5ef20` | `?` | `sp-20` | `0xfffff5bc` | `?` |
| `bl` at 0x8d784 | `(unknown callee) 0x19808` | `?` | `ret(0x5ef20)` | `?` | `?` |
| `bl` at 0x8d7b4 | `(unknown callee) 0x195e0` | `?` | `0x10a408` | `[sp-2684]+1` | `?` |
| `bl` at 0x8d7bc | `(unknown callee) 0x192bc` | `[sp-2708]` | `?` | `?` | `?` |
| `bl` at 0x8d800 | `(unknown callee) 0x1970c` | `[sp-2708]` | `?` | `?` | `0x0` |
| `bl` at 0x8d868 | `(unknown callee) 0x5eac8` | `sp-76` | `sp-72` | `0x0` | `sp-76` |
| `bl` at 0x8d870 | `(unknown callee) 0x19538` | `[sp-2660]` | `?` | `?` | `?` |
| `bl` at 0x8d898 | `(unknown callee) 0x5ed50` | `sp-76` | `sp-72` | `[sp-2660]` | `ret(0x19538)` |
| `bl` at 0x8d8f8 | `(unknown callee) 0xa9f4c` | `0x29a6b4` | `?` | `sp-2700` | `0x0` |
| `bl` at 0x8d964 | `(unknown callee) 0x1943c` | `0x50` | `?` | `?` | `[sp-2680]` |
| `bl` at 0x8d978 | `(unknown callee) 0x5ef20` | `[0x29a6b4]` | `?` | `?` | `0x29a6b4` |
| `bl` at 0x8d994 | `(unknown callee) 0x19808` | `?` | `ret(0x5ef20)` | `?` | `?` |
| `bl` at 0x8d9b0 | `(unknown callee) 0x195e0` | `?` | `0x10a40c` | `?` | `?` |
| `bl` at 0x8d9d8 | `(unknown callee) 0x19538` | `?` | `?` | `?` | `?` |
| `bl` at 0x8da08 | `(unknown callee) 0x5ef20` | `?` | `sp-20` | `0xfffff5bc` | `?` |
| `bl` at 0x8da18 | `(unknown callee) 0x19808` | `?` | `ret(0x5ef20)` | `?` | `ret(0x5ef20)` |
| `bl` at 0x8da34 | `(unknown callee) 0x195e0` | `?` | `0x107d30` | `?` | `?` |
| `bl` at 0x8da94 | `(unknown callee) 0x1f8e0` | `[sp-36]` | `sp-20` | `?` | `[sp-2676]+1` |
| `bl` at 0x8daa4 | `(unknown callee) 0x1e958` | `0x10a434` | `ret(0x1f8e0)` | `?` | `ret(0x1f8e0)` |
| `bl` at 0x8dabc | `(unknown callee) 0x1970c` | `[sp-2660]` | `?` | `?` | `?` |
| `bl` at 0x8daf0 | `(unknown callee) 0xa8b38` | `sp-48` | `0x29a754` | `0x44c` | `sp-48` |
| `bl` at 0x8db8c | `(unknown callee) 0x1943c` | `0x50` | `sp-20` | `?` | `[sp-2680]` |
| `bl` at 0x8dba0 | `(unknown callee) 0x5ef20` | `[0x29a6b4]` | `?` | `?` | `0x29a6b4` |
| `bl` at 0x8dbbc | `(unknown callee) 0x19808` | `?` | `ret(0x5ef20)` | `?` | `?` |
| `bl` at 0x8dbd8 | `(unknown callee) 0x195e0` | `?` | `0x10a458` | `?` | `?` |
| `bl` at 0x8dc00 | `(unknown callee) 0x19538` | `?` | `?` | `?` | `?` |
| `bl` at 0x8dc30 | `(unknown callee) 0x5ef20` | `?` | `sp-20` | `0xfffff5bc` | `?` |
| `bl` at 0x8dc40 | `(unknown callee) 0x19808` | `?` | `ret(0x5ef20)` | `?` | `ret(0x5ef20)` |
| `bl` at 0x8dc5c | `(unknown callee) 0x195e0` | `?` | `0x107d30` | `?` | `?` |
| `bl` at 0x8dcb4 | `(unknown callee) 0xaadf4` | `[sp-48]` | `0x0` | `ret(0xa8b38)` | `[[sp-48]+19]:1` |
| `bl` at 0x8dcd8 | `(unknown callee) 0xa8b98` | `[sp-48]` | `?` | `ret(0xaadf4)` | `?` |
| `bl` at 0x8de00 | `(unknown callee) 0xa8b98` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0x8de14 | `(unknown callee) 0x1f8e0` | `[sp-36]` | `ret(0xa8b38)` | `ret(0xa8b38)` | `ret(0xa8b38)` |
| `bl` at 0x8de28 | `(unknown callee) 0x1e958` | `0x10a474` | `ret(0x1f8e0)` | `[sp-36]` | `ret(0x1f8e0)` |
| `bl` at 0x8deb0 | `(unknown callee) 0xaadf4` | `[sp-48]` | `0x0` | `?` | `?` |
| `bl` at 0x8def8 | `(unknown callee) 0x1943c` | `0x50` | `?` | `?` | `[sp-2680]` |
| `bl` at 0x8df18 | `(unknown callee) 0x5ef20` | `[[sp-48]+28]` | `?` | `?` | `[sp-48]` |
| `bl` at 0x8df34 | `(unknown callee) 0x19808` | `?` | `ret(0x5ef20)` | `?` | `?` |
| `bl` at 0x8df50 | `(unknown callee) 0x195e0` | `?` | `0x10a404` | `?` | `?` |
| `bl` at 0x8df80 | `(unknown callee) 0xaadf4` | `[sp-48]` | `0x1` | `?` | `?` |
| `bl` at 0x8dfa0 | `(unknown callee) 0x195e0` | `?` | `[sp-2728]` | `?` | `?` |
| `bl` at 0x8dfe0 | `(unknown callee) 0x5ef20` | `[[sp-48]+28]` | `?` | `?` | `[sp-48]` |
| `bl` at 0x8dffc | `(unknown callee) 0x19808` | `?` | `ret(0x5ef20)` | `?` | `?` |
| `bl` at 0x8e02c | `(unknown callee) 0x195e0` | `?` | `0x10a408` | `[sp-2684]+1` | `?` |
| `bl` at 0x8e05c | `(unknown callee) 0xaadf4` | `[sp-48]` | `0x1` | `?` | `?` |
| `bl` at 0x8e068 | `(unknown callee) 0x192bc` | `[sp-2732]` | `?` | `?` | `?` |
| `bl` at 0x8e14c | `(unknown callee) 0xaadf4` | `[sp-48]` | `0x1` | `?` | `?` |
| `bl` at 0x8e158 | `(unknown callee) 0x192bc` | `[sp-2736]` | `?` | `?` | `?` |
| `bl` at 0x8e1b8 | `(unknown callee) 0xaadf4` | `[sp-48]` | `0x2` | `[sp-2684]+1` | `?` |
| `bl` at 0x8e1c4 | `(unknown callee) 0x192bc` | `[sp-2740]` | `?` | `?` | `?` |
| `bl` at 0x8e228 | `(unknown callee) 0x1943c` | `0x50` | `?` | `?` | `[sp-2680]` |
| `bl` at 0x8e23c | `(unknown callee) 0x5ef20` | `[[sp-48]+28]` | `?` | `?` | `[sp-48]` |
| `bl` at 0x8e258 | `(unknown callee) 0x19808` | `?` | `ret(0x5ef20)` | `?` | `?` |
| `bl` at 0x8e274 | `(unknown callee) 0x195e0` | `?` | `0x10a49c` | `?` | `?` |
| `bl` at 0x8e29c | `(unknown callee) 0x19538` | `?` | `?` | `?` | `?` |
| `bl` at 0x8e2d4 | `(unknown callee) 0x5ef20` | `?` | `sp-20` | `0xfffff5bc` | `?` |
| `bl` at 0x8e2e4 | `(unknown callee) 0x19808` | `?` | `ret(0x5ef20)` | `?` | `ret(0x5ef20)` |
| `bl` at 0x8e300 | `(unknown callee) 0x195e0` | `?` | `0x107d30` | `?` | `?` |
| `bl` at 0x8e334 | `(unknown callee) 0xa8b98` | `[sp-48]` | `?` | `?` | `[sp-1624]` |
| `mov` at 0x8e384 | `(unknown callee) (unknown target)` | `sp-2664` | `?` | `sp-2664` | `?` |
| `bl` at 0x8e3dc | `(unknown callee) 0xa8b98` | `?` | `sp-20` | `0xfffff9c4` | `?` |
| `bl` at 0x8e3f4 | `(unknown callee) 0x2105c` | `0x29a7f0` | `?` | `?` | `?` |
| `bl` at 0x8e410 | `(unknown callee) 0x1f8e0` | `ret(0x2105c)` | `?` | `?` | `ret(0x2105c)` |
| `bl` at 0x8e434 | `(unknown callee) 0x200a0` | `0x9` | `0x10a2d8` | `0x1994` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x8d294 | `ret(0xaed30) != 0x0` | `ret(0xaed30) == 0x0` |
| `bne` at 0x8d2a0 | `[sp-2656] != 0x0` | `[sp-2656] == 0x0` |
| `beq` at 0x8d2cc | `ret(0x20cd0) == 0x0` | `ret(0x20cd0) != 0x0` |
| `blt` at 0x8d344 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x8d368 | `? != 0x1` | `? == 0x1` |
| `bne` at 0x8d464 | `ret(0xaa424) != 0x0` | `ret(0xaa424) == 0x0` |
| `beq` at 0x8d47c | `ret(0x6f4d4) == 0x0` | `ret(0x6f4d4) != 0x0` |
| `beq` at 0x8d4bc | `ret(0x21d50) == 0x0` | `ret(0x21d50) != 0x0` |
| `bne` at 0x8d628 | `? != ?` | `? == ?` |
| `beq` at 0x8d66c | `ret(0xaed30) == 0x0` | `ret(0xaed30) != 0x0` |
| `beq` at 0x8d684 | `[sp-2708] == 0x0` | `[sp-2708] != 0x0` |
| `beq` at 0x8d6b0 | `[sp-44] == 0x0` | `[sp-44] != 0x0` |
| `beq` at 0x8d90c | `ret(0xa9f4c) == 0x0` | `ret(0xa9f4c) != 0x0` |
| `bne` at 0x8d91c | `[sp-1628] != 0x1` | `[sp-1628] == 0x1` |
| `ble` at 0x8dad4 | `[sp-2656] <= [sp-2668] (signed, 32-bit)` | `[sp-2656] > [sp-2668] (signed, 32-bit)` |
| `beq` at 0x8db0c | `ret(0xa8b38) == 0x0` | `ret(0xa8b38) != 0x0` |
| `bgt` at 0x8db18 | `ret(0xa8b38) > 0x0 (signed, 32-bit)` | `ret(0xa8b38) <= 0x0 (signed, 32-bit)` |
| `beq` at 0x8db24 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x8db34 | `ret(0xa8b38) == 0x6e` | `ret(0xa8b38) != 0x6e` |
| `ble` at 0x8db50 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x8db74 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x8dca8 | `[[sp-48]+19]:1 != 0x13` | `[[sp-48]+19]:1 == 0x13` |
| `ble` at 0x8dcc8 | `ret(0xaadf4) <= [sp-40] (signed, 32-bit)` | `ret(0xaadf4) > [sp-40] (signed, 32-bit)` |
| `bne` at 0x8dd20 | `Z == 0` | `not (Z == 0)` |
| `ble` at 0x8dd30 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x8dd5c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x8dd70 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x8dd8c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x8ddb4 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x8ddf8 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x8de34 | `[sp-48] == 0x0` | `[sp-48] != 0x0` |
| `beq` at 0x8de40 | `[sp-1624] == 0x0` | `[sp-1624] != 0x0` |
| `beq` at 0x8de7c | `[[sp-48]+19]:1 == 0x0` | `[[sp-48]+19]:1 != 0x0` |
| `bne` at 0x8de94 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8dec4 | `[sp-2724] == 0x0` | `[sp-2724] != 0x0` |
| `bls` at 0x8dee0 | `[[sp-48]+32]:1 <= 0x1 (unsigned, 32-bit)` | `[[sp-48]+32]:1 > 0x1 (unsigned, 32-bit)` |
| `beq` at 0x8df0c | `[sp-44] == 0x0` | `[sp-44] != 0x0` |
| `bne` at 0x8df64 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8dfd4 | `[[sp-48]+32]:1 != 0x2` | `[[sp-48]+32]:1 == 0x2` |
| `bne` at 0x8e040 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8e0b8 | `[[sp-48]+32]:1 != 0x3` | `[[sp-48]+32]:1 == 0x3` |
| `bne` at 0x8e0cc | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8e19c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8e32c | `[sp-1624] != 0x0` | `[sp-1624] == 0x0` |
| `beq` at 0x8e358 | `[sp-1624] == 0x0` | `[sp-1624] != 0x0` |
| `ble` at 0x8e3bc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x8e408 | `ret(0x2105c) == 0x0` | `ret(0x2105c) != 0x0` |
| `beq` at 0x8e440 | `[sp-2676] == 0x0` | `[sp-2676] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x8e468 | `r0` | `?` |

## `smpcmd_intr_count`

- `smpcmd_intr_count`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa523c to 0xa55ac.
  Established. A word store writes `ret(0x1943c)` to `[sp-32] +0x0` (`str` at 0xa52b4).
  An included unnamed `0x197a8` call carries `[sp-48]`, `0x113778`, `ret(0xc1680)`, `ret(0xc1278)` (`bl` at 0xa5454).
  A nonzero `ret(0xca484)` takes `0xa54e8`, while zero falls through (`bne` at 0xa54b8).
  Interrupt-count report source meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa524c | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0xa5250 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0xa5260 | store | 4 | `sp -0x24` | `ret(0xbfecc)` |
| `str` at 0xa5270 | store | 4 | `sp -0x28` | `ret(0xca434)` |
| `str` at 0xa5278 | store | 4 | `sp -0x2c` | `ret(0xbbe84)` |
| `ldr` at 0xa527c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa5280 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa5284 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa528c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xa52b4 | store | 4 | `[sp-32] +0x0` | `ret(0x1943c)` |
| `ldr` at 0xa52b8 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0xa52bc | store | 4 | `sp -0x30` | `[[sp-32]]` |
| `ldr` at 0xa52c0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa52c4 | load | 4 | `abs 0xa5594` | `-` |
| `ldr` at 0xa52cc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa52dc | load | 4 | `sp -0x30` | `-` |
| `str` at 0xa52e4 | store | 4 | `sp -0x30` | `([sp-48] + ret(0x19538))` |
| `str` at 0xa52ec | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xa52f0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa52f4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa5300 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa5314 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa532c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xa5340 | store | 4 | `sp -0x38` | `ret(0xbbea4)` |
| `ldr` at 0xa5344 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa5348 | load | 4 | `abs 0xa5598` | `-` |
| `ldr` at 0xa5358 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa5368 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xa5370 | store | 4 | `sp -0x30` | `([sp-48] + ret(0x19538))` |
| `ldr` at 0xa5374 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xa537c | store | 4 | `sp -0x34` | `[sp-52]+1` |
| `str` at 0xa5388 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xa538c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa5390 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa539c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa53bc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa53d4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa53d8 | load | 4 | `abs 0xa559c` | `-` |
| `ldr` at 0xa53e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa53f4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa5418 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa5430 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa5448 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa544c | load | 4 | `abs 0xa55a0` | `-` |
| `ldr` at 0xa5458 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa5468 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xa5470 | store | 4 | `sp -0x30` | `([sp-48] + ret(0x19538))` |
| `ldr` at 0xa5474 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xa547c | store | 4 | `sp -0x34` | `[sp-52]+1` |
| `str` at 0xa5488 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xa548c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa5490 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa549c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa54bc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa54d4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa54d8 | load | 4 | `abs 0xa55a4` | `-` |
| `ldr` at 0xa54e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa54f4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa5518 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa5530 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa5548 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa554c | load | 4 | `abs 0xa55a8` | `-` |
| `ldr` at 0xa5558 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa5568 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xa5570 | store | 4 | `sp -0x30` | `([sp-48] + ret(0x19538))` |
| `ldr` at 0xa5574 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xa557c | store | 4 | `sp -0x34` | `[sp-52]+1` |
| `ldm` at 0xa5590 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xa5590 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xa5590 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa5590 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa5590 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa5258 | `(unknown callee) 0xbfecc` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0xa5268 | `(unknown callee) 0xca434` | `0x0` | `?` | `?` | `ret(0xbfecc)` |
| `bl` at 0xa5274 | `(unknown callee) 0xbbe84` | `ret(0xca434)` | `?` | `?` | `ret(0xca434)` |
| `bl` at 0xa52ac | `(unknown callee) 0x1943c` | `?` | `?` | `(([sp-36] + [sp-40]) + ret(0xbbe84))` | `?` |
| `bl` at 0xa52c8 | `(unknown callee) 0x19808` | `[[sp-32]]` | `0x113730` | `?` | `[[sp-32]]` |
| `bl` at 0xa52d0 | `(unknown callee) 0x19538` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0xa530c | `(unknown callee) 0xbbf34` | `?` | `?` | `?` | `?` |
| `bl` at 0xa5320 | `(unknown callee) 0xbbfc4` | `?` | `?` | `?` | `?` |
| `bl` at 0xa5338 | `(unknown callee) 0xbbea4` | `?` | `?` | `?` | `?` |
| `bl` at 0xa5354 | `(unknown callee) 0x197a8` | `[sp-48]` | `0x113740` | `ret(0xbbf34)` | `?` |
| `bl` at 0xa535c | `(unknown callee) 0x19538` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0xa53ac | `(unknown callee) 0xbfb1c` | `0x0` | `?` | `?` | `?` |
| `bl` at 0xa53cc | `(unknown callee) 0xc1680` | `0x0` | `?` | `?` | `?` |
| `bl` at 0xa53e0 | `(unknown callee) 0x197a8` | `[sp-48]` | `0x113758` | `ret(0xc1680)` | `ret(0xc1680)` |
| `bl` at 0xa5404 | `(unknown callee) 0xc1278` | `0x0` | `?` | `?` | `?` |
| `bl` at 0xa5428 | `(unknown callee) 0xc1680` | `0x0` | `?` | `?` | `?` |
| `bl` at 0xa5440 | `(unknown callee) 0xc1278` | `0x0` | `?` | `?` | `?` |
| `bl` at 0xa5454 | `(unknown callee) 0x197a8` | `[sp-48]` | `0x113778` | `ret(0xc1680)` | `ret(0xc1278)` |
| `bl` at 0xa545c | `(unknown callee) 0x19538` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0xa54ac | `(unknown callee) 0xca484` | `0x0` | `?` | `?` | `?` |
| `bl` at 0xa54cc | `(unknown callee) 0xcb12c` | `0x0` | `?` | `?` | `?` |
| `bl` at 0xa54e0 | `(unknown callee) 0x197a8` | `[sp-48]` | `0x113790` | `ret(0xcb12c)` | `ret(0xcb12c)` |
| `bl` at 0xa5504 | `(unknown callee) 0xca7e8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0xa5528 | `(unknown callee) 0xcb12c` | `0x0` | `?` | `?` | `?` |
| `bl` at 0xa5540 | `(unknown callee) 0xca7e8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0xa5554 | `(unknown callee) 0x197a8` | `[sp-48]` | `0x1137b0` | `ret(0xcb12c)` | `ret(0xca7e8)` |
| `bl` at 0xa555c | `(unknown callee) 0x19538` | `[sp-48]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xa52fc | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bge` at 0xa5398 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0xa53b8 | `ret(0xbfb1c) != 0x0` | `ret(0xbfb1c) == 0x0` |
| `bne` at 0xa53f0 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xa5410 | `ret(0xc1278) != 0x0` | `ret(0xc1278) == 0x0` |
| `bge` at 0xa5498 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0xa54b8 | `ret(0xca484) != 0x0` | `ret(0xca484) == 0x0` |
| `bne` at 0xa54f0 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xa5510 | `ret(0xca7e8) != 0x0` | `ret(0xca7e8) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa5590 | `r0` | `0x0` |

## `smpcmd_mgn`

- `smpcmd_mgn`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x89ac8 to 0x89f88.
  Established. A zero `ret(0x8156c)` takes `0x89b28`, while nonzero falls through (`beq` at 0x89b18).
  Included word stores write `ret(0x1943c)` to `[sp-28] +0x0` on known edges (`str` at 0x89b58, `str` at 0x89c60).
  The unnamed `0x364c4` call carries `0x0`, `sp-40`, unknown, `sp-40` (`bl` at 0x89b34).
  Margin command actions behind the computed transfer remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x89ad8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x89adc | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x89ae4 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x89aec | store | 4 | `sp -0x24` | `0x0` |
| `strb` at 0x89af8 | store | 1 | `sp -0x36` | `(ret(0x488cc) & 0xff)` |
| `ldr` at 0x89afc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x89b00 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0x89b04 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0x89b08 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x89b20 | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0x89b38 | store | 4 | `sp -0x2c` | `ret(0x364c4)` |
| `ldr` at 0x89b3c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x89b48 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x89b58 | store | 4 | `[sp-28] +0x0` | `ret(0x1943c)` |
| `ldr` at 0x89b5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x89b60 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x89b64 | load | 4 | `abs 0x89f70` | `-` |
| `str` at 0x89b70 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x89b78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x89b80 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0x89b88 | load | 4 | `[[sp-24]+112] +0x8` | `-` |
| `ldr` at 0x89b94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x89b9c | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0x89ba4 | load | 4 | `[[sp-24]+112] +0x8` | `-` |
| `str` at 0x89ba8 | store | 4 | `sp -0x24` | `[[[sp-24]+112]+8]` |
| `str` at 0x89bb0 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0x89bbc | store | 4 | `sp -0x44` | `0x0` |
| `str` at 0x89bc4 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x89bc8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x89bcc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x89bd8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x89bec | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x89bf4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x89c04 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x89c10 | store | 4 | `sp -0x48` | `[sp-72]+1` |
| `ldr` at 0x89c14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x89c18 | load | 4 | `[sp-24] +0x28` | `-` |
| `ldr` at 0x89c24 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x89c28 | store | 4 | `sp -0x24` | `[sp-68]` |
| `ldr` at 0x89c30 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x89c38 | store | 4 | `sp -0x44` | `[sp-68]+1` |
| `ldr` at 0x89c40 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x89c44 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x89c50 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x89c60 | store | 4 | `[sp-28] +0x0` | `ret(0x1943c)` |
| `ldr` at 0x89c64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x89c68 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x89c6c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x89c70 | load | 4 | `abs 0x89f74` | `-` |
| `ldr` at 0x89c74 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0x89c80 | store | 4 | `sp -0x4c` | `?` |
| `str` at 0x89c8c | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0x89c94 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x89ca0 | store | 4 | `sp -0x34` | `ret(0x39b0c)` |
| `ldr` at 0x89ca4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x89cac | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0x89cb4 | load | 4 | `[[sp-24]+112] +0x4` | `-` |
| `ldrls` at 0x89cbc | load | 4 | `(unknown)` | `-` |
| `str` at 0x89f04 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x89f5c | load | 4 | `sp -0x4c` | `-` |
| `ldm` at 0x89f64 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x89f64 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x89f64 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x89f64 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x89af0 | `(unknown callee) 0x488cc` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x89b0c | `(unknown callee) 0x8156c` | `[[[sp-24]+112]]` | `[sp-28]` | `?` | `[[sp-24]+112]` |
| `bl` at 0x89b34 | `(unknown callee) 0x364c4` | `0x0` | `sp-40` | `?` | `sp-40` |
| `bl` at 0x89b50 | `(unknown callee) 0x1943c` | `0x50` | `?` | `?` | `ret(0x364c4)` |
| `bl` at 0x89b68 | `(unknown callee) 0x19808` | `[[sp-28]]` | `0x108fd4` | `?` | `[sp-28]` |
| `bl` at 0x89c58 | `(unknown callee) 0x1943c` | `0x50` | `?` | `[sp-68]` | `[sp-44]` |
| `bl` at 0x89c78 | `(unknown callee) 0x197a8` | `[[sp-28]]` | `0x1092c8` | `[[sp-24]+40]` | `[sp-28]` |
| `bl` at 0x89c98 | `(unknown callee) 0x39b0c` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x89b18 | `ret(0x8156c) == 0x0` | `ret(0x8156c) != 0x0` |
| `bne` at 0x89b44 | `ret(0x364c4) != 0x0` | `ret(0x364c4) == 0x0` |
| `bne` at 0x89b90 | `[[[sp-24]+112]+8] != 0x21` | `[[[sp-24]+112]+8] == 0x21` |
| `bhs` at 0x89bd4 | `[sp-68] >= [sp-44] (unsigned, 32-bit)` | `[sp-68] < [sp-44] (unsigned, 32-bit)` |
| `bne` at 0x89bfc | `? != 0xff` | `? == 0xff` |
| `bne` at 0x89c20 | `[sp-72]+1 != [[sp-24]+40]` | `[sp-72]+1 == [[sp-24]+40]` |
| `bne` at 0x89c4c | `[sp-68] != [sp-44]` | `[sp-68] == [sp-44]` |
| `ldrls` at 0x89cbc | `[[[sp-24]+112]+4] <= 0x3 (unsigned, 32-bit)` | `[[[sp-24]+112]+4] > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x89f64 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x89cbc).

## `smpcmd_tst`

- `smpcmd_tst`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa4aa4 to 0xa505c.
  Established. An included byte store clears `[[sp-28]] +0x0` (`strb` at 0xa4ae4).
  The unnamed `0x1943c` call carries `0x50`, entry `r1`, entry `r2`, `0x1` (`bl` at 0xa4acc).
  The unsigned condition `[[[sp-24]+112]] <= 0x8` takes an unresolved target, while greater falls through (`ldrls` at 0xa4af8).
  Test command actions beyond known edges remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa4ab4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xa4ab8 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xa4ac0 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0xa4ac4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa4ad4 | store | 4 | `r1 +0x0` | `ret(0x1943c)` |
| `ldr` at 0xa4ad8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa4adc | load | 4 | `[sp-28] +0x0` | `-` |
| `strb` at 0xa4ae4 | store | 1 | `[[sp-28]] +0x0` | `0x0` |
| `ldr` at 0xa4ae8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4aec | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xa4af0 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldrls` at 0xa4af8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa5000 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xa5010 | load | 4 | `sp -0x4c` | `-` |
| `ldm` at 0xa5018 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xa5018 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa5018 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa5018 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa4acc | `(unknown callee) 0x1943c` | `0x50` | `r1` | `r2` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xa4af8 | `[[[sp-24]+112]] <= 0x8 (unsigned, 32-bit)` | `[[[sp-24]+112]] > 0x8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa5018 | `r0` | `[sp-76]` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xa4af8).

## `L1FRU_2m_Events`

- `L1FRU_2m_Events`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9a17c to 0x9ac00.
  Established. The unnamed `0xb3c24` call carries `0x29a960`, unknown, unknown, `[ret(0x6a0f8)+16]` (`bl` at 0x9a1c4).
  An included byte store clears `[sp-104] +0x7` (`strb` at 0x9a60c).
  The unnamed `0xb3dec` call carries `0x29a960`, `[sp-104]`, `[sp-28]`, `[sp-116]` (`bl` at 0x9a818).
  FRU event generation policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9a198 | store | 4 | `sp -0x98` | `ret(0x6a0f8)` |
| `ldr` at 0x9a19c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x9a1a0 | load | 4 | `ret(0x6a0f8) +0x8` | `-` |
| `str` at 0x9a1a4 | store | 4 | `sp -0x1c` | `[ret(0x6a0f8)+8]` |
| `ldr` at 0x9a1a8 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x9a1ac | load | 4 | `ret(0x6a0f8) +0xc` | `-` |
| `str` at 0x9a1b0 | store | 4 | `sp -0x74` | `[ret(0x6a0f8)+12]` |
| `ldr` at 0x9a1b4 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x9a1b8 | load | 4 | `ret(0x6a0f8) +0x10` | `-` |
| `str` at 0x9a1bc | store | 4 | `sp -0x78` | `[ret(0x6a0f8)+16]` |
| `ldr` at 0x9a1c0 | load | 4 | `abs 0x9abf4` | `-` |
| `strb` at 0x9a1cc | store | 1 | `sp -0x60` | `?` |
| `strb` at 0x9a1d4 | store | 1 | `sp -0x5f` | `?` |
| `strb` at 0x9a1dc | store | 1 | `sp -0x5e` | `?` |
| `strb` at 0x9a1e4 | store | 1 | `sp -0x5d` | `?` |
| `strb` at 0x9a1ec | store | 1 | `sp -0x5c` | `?` |
| `strb` at 0x9a1f4 | store | 1 | `sp -0x5b` | `?` |
| `strb` at 0x9a1fc | store | 1 | `sp -0x5a` | `?` |
| `strb` at 0x9a204 | store | 1 | `sp -0x59` | `?` |
| `ldr` at 0x9a208 | load | 4 | `abs 0x9abf8` | `-` |
| `ldr` at 0x9a20c | load | 4 | `abs 0x29a958` | `-` |
| `str` at 0x9a210 | store | 4 | `sp -0x80` | `[0x29a958]` |
| `str` at 0x9a218 | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0x9a21c | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x9a220 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x9a22c | load | 4 | `abs 0x9abf8` | `-` |
| `ldr` at 0x9a230 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x9a244 | load | 4 | `abs 0x29a95c` | `-` |
| `ldr` at 0x9a24c | load | 4 | `(unknown)` | `-` |
| `str` at 0x9a250 | store | 4 | `sp -0x84` | `?` |
| `ldr` at 0x9a254 | load | 4 | `abs 0x9abf8` | `-` |
| `ldr` at 0x9a258 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x9a26c | load | 4 | `abs 0x29a95c` | `-` |
| `ldr` at 0x9a274 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9a278 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x9a27c | load | 4 | `abs 0x9abf8` | `-` |
| `ldr` at 0x9a280 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x9a294 | load | 4 | `abs 0x29a95c` | `-` |
| `ldr` at 0x9a29c | load | 4 | `(unknown)` | `-` |
| `str` at 0x9a2a0 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x9a2a4 | load | 4 | `abs 0x9abf8` | `-` |
| `ldr` at 0x9a2a8 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x9a2bc | load | 4 | `abs 0x29a95c` | `-` |
| `ldr` at 0x9a2c4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9a2c8 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0x9a2cc | load | 4 | `abs 0x9abf8` | `-` |
| `ldr` at 0x9a2d0 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x9a2e4 | load | 4 | `abs 0x29a95c` | `-` |
| `ldr` at 0x9a2ec | load | 4 | `(unknown)` | `-` |
| `str` at 0x9a2f0 | store | 4 | `sp -0x94` | `?` |
| `ldr` at 0x9a2f4 | load | 4 | `abs 0x9abfc` | `-` |
| `str` at 0x9a2f8 | store | 4 | `sp -0x2c` | `0xffff` |
| `str` at 0x9a300 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x9a304 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9a308 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x9a314 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9a328 | load | 4 | `sp -0x88` | `-` |
| `ldrh` at 0x9a330 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x9a34c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9a354 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x9a35c | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0x9a360 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x9a374 | store | 4 | `sp -0x2c` | `ret(0xb0840)` |
| `ldr` at 0x9a37c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9a390 | load | 4 | `sp -0x88` | `-` |
| `ldrh` at 0x9a398 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x9a3b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9a3bc | load | 4 | `sp -0x94` | `-` |
| `str` at 0x9a3c4 | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0x9a3c8 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x9a3dc | store | 4 | `sp -0x2c` | `ret(0xb0840)` |
| `ldr` at 0x9a3e0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9a3e8 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `str` at 0x9a3f4 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x9a3f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9a3fc | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x9a408 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9a410 | load | 4 | `sp -0x90` | `-` |
| `ldrh` at 0x9a418 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x9a428 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x9a430 | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0x9a434 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9a43c | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x9a444 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a450 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9a458 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x9a460 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a46c | load | 4 | `sp -0x64` | `-` |
| `str` at 0x9a47c | store | 4 | `sp -0x50` | `ret(0xb0840)` |
| `ldr` at 0x9a480 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x9a48c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9a494 | load | 4 | `sp -0x90` | `-` |
| `ldrb` at 0x9a49c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x9a4b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9a4b8 | load | 4 | `sp -0x90` | `-` |
| `ldrb` at 0x9a4c0 | load | 1 | `(unknown)` | `-` |
| `str` at 0x9a4d8 | store | 4 | `sp -0x68` | `sp-96` |
| `ldr` at 0x9a4dc | load | 4 | `sp -0x94` | `-` |
| `str` at 0x9a4e0 | store | 4 | `sp -0xe0` | `[sp-148]` |
| `ldr` at 0x9a4e4 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x9a4e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9a4ec | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x9a4f0 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x9a4fc | store | 4 | `sp -0x4c` | `ret(0xb0b84)` |
| `ldr` at 0x9a500 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x9a504 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x9a510 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x9a518 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x9a520 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0x9a524 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x9a52c | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x9a534 | load | 1 | `[sp-104] +0x1` | `-` |
| `strb` at 0x9a538 | store | 1 | `[sp-104] +0x0` | `([[sp-104]+1]:1 & 0xff)` |
| `ldr` at 0x9a540 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x9a54c | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x9a554 | load | 1 | `[sp-104] +0x2` | `-` |
| `strb` at 0x9a558 | store | 1 | `[sp-104] +0x1` | `([[sp-104]+2]:1 & 0xff)` |
| `ldr` at 0x9a560 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x9a56c | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x9a574 | load | 1 | `[sp-104] +0x3` | `-` |
| `strb` at 0x9a578 | store | 1 | `[sp-104] +0x2` | `([[sp-104]+3]:1 & 0xff)` |
| `ldr` at 0x9a580 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x9a58c | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x9a594 | load | 1 | `[sp-104] +0x4` | `-` |
| `strb` at 0x9a598 | store | 1 | `[sp-104] +0x3` | `([[sp-104]+4]:1 & 0xff)` |
| `ldr` at 0x9a5a0 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x9a5ac | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x9a5b4 | load | 1 | `[sp-104] +0x5` | `-` |
| `strb` at 0x9a5b8 | store | 1 | `[sp-104] +0x4` | `([[sp-104]+5]:1 & 0xff)` |
| `ldr` at 0x9a5c0 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x9a5cc | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x9a5d4 | load | 1 | `[sp-104] +0x6` | `-` |
| `strb` at 0x9a5d8 | store | 1 | `[sp-104] +0x5` | `([[sp-104]+6]:1 & 0xff)` |
| `ldr` at 0x9a5e0 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x9a5ec | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x9a5f4 | load | 1 | `[sp-104] +0x7` | `-` |
| `strb` at 0x9a5f8 | store | 1 | `[sp-104] +0x6` | `([[sp-104]+7]:1 & 0xff)` |
| `ldr` at 0x9a600 | load | 4 | `sp -0x68` | `-` |
| `strb` at 0x9a60c | store | 1 | `[sp-104] +0x7` | `0x0` |
| `ldr` at 0x9a610 | load | 4 | `abs 0x9abfc` | `-` |
| `str` at 0x9a614 | store | 4 | `sp -0x28` | `0xffff` |
| `str` at 0x9a61c | store | 4 | `sp -0x30` | `0x3` |
| `str` at 0x9a624 | store | 4 | `sp -0x34` | `0x7` |
| `str` at 0x9a62c | store | 4 | `sp -0x38` | `0x3` |
| `str` at 0x9a634 | store | 4 | `sp -0x3c` | `0x7f` |
| `str` at 0x9a63c | store | 4 | `sp -0x40` | `0x7f` |
| `str` at 0x9a644 | store | 4 | `sp -0x44` | `0x7f` |
| `str` at 0x9a64c | store | 4 | `sp -0x48` | `0x7f` |
| `ldr` at 0x9a650 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9a658 | load | 4 | `sp -0x90` | `-` |
| `ldrh` at 0x9a660 | load | 2 | `(unknown)` | `-` |
| `str` at 0x9a674 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x9a678 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9a680 | load | 4 | `sp -0x90` | `-` |
| `ldrh` at 0x9a688 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x9a6a4 | load | 4 | `sp -0x88` | `-` |
| `ldrh` at 0x9a6ac | load | 2 | `(unknown)` | `-` |
| `str` at 0x9a6c0 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x9a6c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9a6cc | load | 4 | `sp -0x90` | `-` |
| `ldrh` at 0x9a6d4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x9a6f0 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x9a6f8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9a6fc | store | 4 | `sp -0x6c` | `?` |
| `ldr` at 0x9a700 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9a708 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x9a710 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9a714 | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0x9a718 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9a720 | load | 4 | `sp -0x90` | `-` |
| `ldrb` at 0x9a728 | load | 1 | `(unknown)` | `-` |
| `str` at 0x9a734 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x9a738 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x9a73c | store | 4 | `sp -0xe0` | `[sp-120]` |
| `ldr` at 0x9a740 | load | 4 | `abs 0x9abf8` | `-` |
| `ldr` at 0x9a744 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x9a758 | load | 4 | `abs 0x29a95c` | `-` |
| `ldr` at 0x9a760 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9a764 | store | 4 | `sp -0xdc` | `?` |
| `ldr` at 0x9a768 | load | 4 | `abs 0x9abf8` | `-` |
| `ldr` at 0x9a76c | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x9a780 | load | 4 | `abs 0x29a95c` | `-` |
| `ldr` at 0x9a788 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9a78c | store | 4 | `sp -0xd8` | `?` |
| `ldr` at 0x9a790 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9a794 | store | 4 | `sp -0xd4` | `[sp-24]` |
| `ldr` at 0x9a798 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9a79c | store | 4 | `sp -0xd0` | `[sp-32]` |
| `ldr` at 0x9a7a0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x9a7a4 | store | 4 | `sp -0xcc` | `[sp-44]` |
| `ldr` at 0x9a7a8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9a7ac | store | 4 | `sp -0xc8` | `[sp-40]` |
| `ldr` at 0x9a7b0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x9a7b4 | store | 4 | `sp -0xc4` | `[sp-48]` |
| `ldr` at 0x9a7b8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x9a7bc | store | 4 | `sp -0xc0` | `[sp-52]` |
| `ldr` at 0x9a7c0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x9a7c4 | store | 4 | `sp -0xbc` | `[sp-56]` |
| `ldr` at 0x9a7c8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x9a7cc | store | 4 | `sp -0xb8` | `[sp-60]` |
| `ldr` at 0x9a7d0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x9a7d4 | store | 4 | `sp -0xb4` | `[sp-64]` |
| `ldr` at 0x9a7d8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x9a7dc | store | 4 | `sp -0xb0` | `[sp-68]` |
| `ldr` at 0x9a7e0 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x9a7e4 | store | 4 | `sp -0xac` | `[sp-72]` |
| `ldr` at 0x9a7e8 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x9a7ec | store | 4 | `sp -0xa8` | `[sp-84]` |
| `ldr` at 0x9a7f0 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x9a7f4 | store | 4 | `sp -0xa4` | `[sp-88]` |
| `ldr` at 0x9a7f8 | load | 4 | `sp -0x6c` | `-` |
| `str` at 0x9a7fc | store | 4 | `sp -0xa0` | `[sp-108]` |
| `ldr` at 0x9a800 | load | 4 | `sp -0x70` | `-` |
| `str` at 0x9a804 | store | 4 | `sp -0x9c` | `[sp-112]` |
| `ldr` at 0x9a808 | load | 4 | `abs 0x9abf4` | `-` |
| `ldr` at 0x9a80c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x9a810 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9a814 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x9a820 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x9a82c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9a834 | load | 4 | `sp -0x90` | `-` |
| `ldrb` at 0x9a83c | load | 1 | `(unknown)` | `-` |
| `str` at 0x9a854 | store | 4 | `sp -0x68` | `sp-96` |
| `ldr` at 0x9a858 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x9a85c | store | 4 | `sp -0xe0` | `[sp-148]` |
| `ldr` at 0x9a860 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x9a864 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9a868 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x9a86c | load | 4 | `sp -0x88` | `-` |
| `str` at 0x9a878 | store | 4 | `sp -0x4c` | `ret(0xb0b84)` |
| `ldr` at 0x9a87c | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x9a880 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x9a88c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x9a894 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x9a89c | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0x9a8a0 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x9a8a4 | store | 4 | `sp -0xe0` | `[sp-136]` |
| `ldr` at 0x9a8a8 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x9a8ac | store | 4 | `sp -0xdc` | `[sp-148]` |
| `ldr` at 0x9a8b4 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x9a8b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9a8bc | load | 4 | `sp -0x84` | `-` |
| `str` at 0x9a8c8 | store | 4 | `sp -0x28` | `ret(0xb29b0)` |
| `ldr` at 0x9a8cc | load | 4 | `sp -0x88` | `-` |
| `str` at 0x9a8d0 | store | 4 | `sp -0xe0` | `[sp-136]` |
| `ldr` at 0x9a8d4 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x9a8d8 | store | 4 | `sp -0xdc` | `[sp-148]` |
| `ldr` at 0x9a8e0 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x9a8e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9a8e8 | load | 4 | `sp -0x84` | `-` |
| `str` at 0x9a8f4 | store | 4 | `sp -0x30` | `ret(0xb29b0)` |
| `ldr` at 0x9a8f8 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x9a8fc | store | 4 | `sp -0xe0` | `[sp-136]` |
| `ldr` at 0x9a900 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x9a904 | store | 4 | `sp -0xdc` | `[sp-148]` |
| `ldr` at 0x9a90c | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x9a910 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9a914 | load | 4 | `sp -0x84` | `-` |
| `str` at 0x9a920 | store | 4 | `sp -0x34` | `ret(0xb29b0)` |
| `ldr` at 0x9a924 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x9a928 | store | 4 | `sp -0xe0` | `[sp-136]` |
| `ldr` at 0x9a92c | load | 4 | `sp -0x94` | `-` |
| `str` at 0x9a930 | store | 4 | `sp -0xdc` | `[sp-148]` |
| `ldr` at 0x9a938 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x9a93c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9a940 | load | 4 | `sp -0x84` | `-` |
| `str` at 0x9a94c | store | 4 | `sp -0x38` | `ret(0xb29b0)` |
| `ldr` at 0x9a950 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x9a954 | store | 4 | `sp -0xe0` | `[sp-136]` |
| `ldr` at 0x9a958 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x9a95c | store | 4 | `sp -0xdc` | `[sp-148]` |
| `ldr` at 0x9a964 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x9a968 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9a96c | load | 4 | `sp -0x84` | `-` |
| `str` at 0x9a978 | store | 4 | `sp -0x3c` | `ret(0xb29b0)` |
| `ldr` at 0x9a97c | load | 4 | `sp -0x88` | `-` |
| `str` at 0x9a980 | store | 4 | `sp -0xe0` | `[sp-136]` |
| `ldr` at 0x9a984 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x9a988 | store | 4 | `sp -0xdc` | `[sp-148]` |
| `ldr` at 0x9a990 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x9a994 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9a998 | load | 4 | `sp -0x84` | `-` |
| `str` at 0x9a9a4 | store | 4 | `sp -0x40` | `ret(0xb29b0)` |
| `ldr` at 0x9a9a8 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x9a9ac | store | 4 | `sp -0xe0` | `[sp-136]` |
| `ldr` at 0x9a9b0 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x9a9b4 | store | 4 | `sp -0xdc` | `[sp-148]` |
| `ldr` at 0x9a9bc | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x9a9c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9a9c4 | load | 4 | `sp -0x84` | `-` |
| `str` at 0x9a9d0 | store | 4 | `sp -0x44` | `ret(0xb29b0)` |
| `ldr` at 0x9a9d4 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x9a9d8 | store | 4 | `sp -0xe0` | `[sp-136]` |
| `ldr` at 0x9a9dc | load | 4 | `sp -0x94` | `-` |
| `str` at 0x9a9e0 | store | 4 | `sp -0xdc` | `[sp-148]` |
| `ldr` at 0x9a9e8 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x9a9ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9a9f0 | load | 4 | `sp -0x84` | `-` |
| `str` at 0x9a9fc | store | 4 | `sp -0x48` | `ret(0xb29b0)` |
| `ldr` at 0x9aa00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9aa08 | load | 4 | `sp -0x90` | `-` |
| `ldrh` at 0x9aa10 | load | 2 | `(unknown)` | `-` |
| `str` at 0x9aa24 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x9aa28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9aa30 | load | 4 | `sp -0x90` | `-` |
| `ldrh` at 0x9aa38 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x9aa54 | load | 4 | `sp -0x88` | `-` |
| `ldrh` at 0x9aa5c | load | 2 | `(unknown)` | `-` |
| `str` at 0x9aa70 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x9aa74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9aa7c | load | 4 | `sp -0x90` | `-` |
| `ldrh` at 0x9aa84 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x9aaa0 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x9aaa8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9aaac | store | 4 | `sp -0x6c` | `?` |
| `ldr` at 0x9aab0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9aab8 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x9aac0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9aac4 | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0x9aac8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9aad0 | load | 4 | `sp -0x90` | `-` |
| `ldrb` at 0x9aad8 | load | 1 | `(unknown)` | `-` |
| `str` at 0x9aae4 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x9aae8 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x9aaec | store | 4 | `sp -0xe0` | `[sp-120]` |
| `ldr` at 0x9aaf0 | load | 4 | `abs 0x9abf8` | `-` |
| `ldr` at 0x9aaf4 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x9ab08 | load | 4 | `abs 0x29a95c` | `-` |
| `ldr` at 0x9ab10 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9ab14 | store | 4 | `sp -0xdc` | `?` |
| `ldr` at 0x9ab18 | load | 4 | `abs 0x9abf8` | `-` |
| `ldr` at 0x9ab1c | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x9ab30 | load | 4 | `abs 0x29a95c` | `-` |
| `ldr` at 0x9ab38 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9ab3c | store | 4 | `sp -0xd8` | `?` |
| `ldr` at 0x9ab40 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9ab44 | store | 4 | `sp -0xd4` | `[sp-24]` |
| `ldr` at 0x9ab48 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9ab4c | store | 4 | `sp -0xd0` | `[sp-32]` |
| `ldr` at 0x9ab50 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x9ab54 | store | 4 | `sp -0xcc` | `[sp-44]` |
| `ldr` at 0x9ab58 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9ab5c | store | 4 | `sp -0xc8` | `[sp-40]` |
| `ldr` at 0x9ab60 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x9ab64 | store | 4 | `sp -0xc4` | `[sp-48]` |
| `ldr` at 0x9ab68 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x9ab6c | store | 4 | `sp -0xc0` | `[sp-52]` |
| `ldr` at 0x9ab70 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x9ab74 | store | 4 | `sp -0xbc` | `[sp-56]` |
| `ldr` at 0x9ab78 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x9ab7c | store | 4 | `sp -0xb8` | `[sp-60]` |
| `ldr` at 0x9ab80 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x9ab84 | store | 4 | `sp -0xb4` | `[sp-64]` |
| `ldr` at 0x9ab88 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x9ab8c | store | 4 | `sp -0xb0` | `[sp-68]` |
| `ldr` at 0x9ab90 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x9ab94 | store | 4 | `sp -0xac` | `[sp-72]` |
| `ldr` at 0x9ab98 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x9ab9c | store | 4 | `sp -0xa8` | `[sp-84]` |
| `ldr` at 0x9aba0 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x9aba4 | store | 4 | `sp -0xa4` | `[sp-88]` |
| `ldr` at 0x9aba8 | load | 4 | `sp -0x6c` | `-` |
| `str` at 0x9abac | store | 4 | `sp -0xa0` | `[sp-108]` |
| `ldr` at 0x9abb0 | load | 4 | `sp -0x70` | `-` |
| `str` at 0x9abb4 | store | 4 | `sp -0x9c` | `[sp-112]` |
| `ldr` at 0x9abb8 | load | 4 | `abs 0x9abf4` | `-` |
| `ldr` at 0x9abbc | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x9abc0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9abc4 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x9abcc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9abd4 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x9abdc | load | 4 | `sp -0x7c` | `-` |
| `str` at 0x9abe4 | store | 4 | `sp -0x7c` | `[sp-124]+1` |
| `ldm` at 0x9abf0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9abf0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9abf0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9a190 | `(unknown callee) 0x6a0f8` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x9a1c4 | `(unknown callee) 0xb3c24` | `0x29a960` | `?` | `?` | `[ret(0x6a0f8)+16]` |
| `bl` at 0x9a36c | `(unknown callee) 0xb0840` | `?` | `0x2a` | `0x20` | `?` |
| `bl` at 0x9a3d4 | `(unknown callee) 0xb0840` | `?` | `0x15` | `0x8` | `?` |
| `bl` at 0x9a474 | `(unknown callee) 0xb0840` | `[sp-100]` | `?` | `?` | `?` |
| `bl` at 0x9a4f4 | `(unknown callee) 0xb0b84` | `[sp-144]` | `[sp-24]` | `[sp-132]` | `[sp-136]` |
| `bl` at 0x9a818 | `(unknown callee) 0xb3dec` | `0x29a960` | `[sp-104]` | `[sp-28]` | `[sp-116]` |
| `bl` at 0x9a870 | `(unknown callee) 0xb0b84` | `[sp-144]` | `[sp-24]` | `[sp-132]` | `[sp-136]` |
| `bl` at 0x9a8c0 | `(unknown callee) 0xb29b0` | `0x0` | `[sp-144]` | `[sp-24]` | `[sp-132]` |
| `bl` at 0x9a8ec | `(unknown callee) 0xb29b0` | `0x1` | `[sp-144]` | `[sp-24]` | `[sp-132]` |
| `bl` at 0x9a918 | `(unknown callee) 0xb29b0` | `0x2` | `[sp-144]` | `[sp-24]` | `[sp-132]` |
| `bl` at 0x9a944 | `(unknown callee) 0xb29b0` | `0x7` | `[sp-144]` | `[sp-24]` | `[sp-132]` |
| `bl` at 0x9a970 | `(unknown callee) 0xb29b0` | `0x3` | `[sp-144]` | `[sp-24]` | `[sp-132]` |
| `bl` at 0x9a99c | `(unknown callee) 0xb29b0` | `0x4` | `[sp-144]` | `[sp-24]` | `[sp-132]` |
| `bl` at 0x9a9c8 | `(unknown callee) 0xb29b0` | `0x5` | `[sp-144]` | `[sp-24]` | `[sp-132]` |
| `bl` at 0x9a9f4 | `(unknown callee) 0xb29b0` | `0x6` | `[sp-144]` | `[sp-24]` | `[sp-132]` |
| `bl` at 0x9abc8 | `(unknown callee) 0xb3dec` | `0x29a960` | `[sp-104]` | `[sp-28]` | `[sp-116]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x9a228 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bgt` at 0x9a310 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0x9a348 | `? != 0x2` | `? == 0x2` |
| `bne` at 0x9a3b0 | `? != 0x34` | `? == 0x34` |
| `bge` at 0x9a404 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x9a488 | `ret(0xb0840) == 0x0` | `ret(0xb0840) != 0x0` |
| `ble` at 0x9a4ac | `? <= 0x1b (signed, 32-bit)` | `? > 0x1b (signed, 32-bit)` |
| `bgt` at 0x9a4d0 | `? > 0x24 (signed, 32-bit)` | `? <= 0x24 (signed, 32-bit)` |
| `blo` at 0x9a50c | `[sp-132] < ret(0xb0b84) (unsigned, 32-bit)` | `[sp-132] >= ret(0xb0b84) (unsigned, 32-bit)` |
| `beq` at 0x9a828 | `ret(0xb0840) == 0x0` | `ret(0xb0840) != 0x0` |
| `beq` at 0x9a84c | `? == 0x0` | `? != 0x0` |
| `blo` at 0x9a888 | `[sp-132] < ret(0xb0b84) (unsigned, 32-bit)` | `[sp-132] >= ret(0xb0b84) (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9abf0 | `r0` | `?` |

## `FruFmtMultiRec`

- `FruFmtMultiRec`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa66e0 to 0xa6e1c.
  Established. A byte store clears entry `r3 +0x0` (`strb` at 0xa670c).
  A zero `[sp-28]` takes `0xa6734`, while nonzero falls through (`beq` at 0xa6720).
  An included unnamed `0xa1aa8` call carries `[sp-36]`, unknown, unknown, `[sp-36]` (`bl` at 0xa6768).
  FRU multirecord field units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa66f0 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0xa66f4 | store | 4 | `sp -0x24` | `r2` |
| `str` at 0xa66f8 | store | 4 | `sp -0x28` | `r3` |
| `strb` at 0xa6700 | store | 1 | `sp -0x1d` | `(r1 & 0xff)` |
| `ldr` at 0xa6704 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0xa670c | store | 1 | `r3 +0x0` | `0x0` |
| `ldr` at 0xa6710 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa6714 | store | 4 | `sp -0x90` | `[sp-28]` |
| `ldr` at 0xa6718 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xa6724 | load | 4 | `sp -0x90` | `-` |
| `ldrb` at 0xa6734 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xa6740 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6744 | load | 4 | `abs 0xa6dc0` | `-` |
| `ldr` at 0xa6750 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6754 | load | 4 | `abs 0xa6dc4` | `-` |
| `ldr` at 0xa675c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa6760 | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0xa6764 | load | 4 | `sp -0x2c` | `-` |
| `strh` at 0xa6770 | store | 2 | `sp -0x34` | `(ret(0xa1aa8) & 0xffff)` |
| `ldrh` at 0xa6778 | load | 2 | `sp -0x34` | `-` |
| `ldr` at 0xa6780 | load | 4 | `abs 0xa6dc8` | `-` |
| `ldr` at 0xa678c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6798 | load | 4 | `sp -0x2c` | `-` |
| `strh` at 0xa67ac | store | 2 | `sp -0x34` | `(ret(0xa1aa8) & 0xffff)` |
| `ldrh` at 0xa67b0 | load | 2 | `sp -0x34` | `-` |
| `ldr` at 0xa67b4 | load | 4 | `abs 0xa6dcc` | `-` |
| `ldrh` at 0xa67c4 | load | 2 | `sp -0x34` | `-` |
| `ldr` at 0xa67cc | load | 4 | `abs 0xa6dd0` | `-` |
| `ldr` at 0xa67d8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa67e4 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xa67e8 | load | 1 | `[sp-44] +0x4` | `-` |
| `ldr` at 0xa67f8 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xa67fc | load | 1 | `[sp-44] +0x4` | `-` |
| `ldr` at 0xa6800 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xa6804 | load | 1 | `[sp-44] +0x5` | `-` |
| `ldr` at 0xa680c | load | 4 | `abs 0xa6dd4` | `-` |
| `ldr` at 0xa681c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6828 | load | 4 | `sp -0x2c` | `-` |
| `strh` at 0xa683c | store | 2 | `sp -0x34` | `(ret(0xa1aa8) & 0xffff)` |
| `ldrh` at 0xa6840 | load | 2 | `sp -0x34` | `-` |
| `ldr` at 0xa684c | load | 4 | `sp -0x2c` | `-` |
| `strh` at 0xa6860 | store | 2 | `sp -0x36` | `(ret(0xa1aa8) & 0xffff)` |
| `ldrh` at 0xa6868 | load | 2 | `sp -0x34` | `-` |
| `ldr` at 0xa686c | load | 4 | `abs 0xa6dd8` | `-` |
| `ldrh` at 0xa6880 | load | 2 | `sp -0x36` | `-` |
| `ldr` at 0xa6884 | load | 4 | `abs 0xa6dd8` | `-` |
| `ldr` at 0xa689c | load | 4 | `abs 0xa6ddc` | `-` |
| `ldr` at 0xa68ac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa68b8 | load | 4 | `sp -0x2c` | `-` |
| `strh` at 0xa68cc | store | 2 | `sp -0x34` | `(ret(0xa1aa8) & 0xffff)` |
| `ldrh` at 0xa68d0 | load | 2 | `sp -0x34` | `-` |
| `ldr` at 0xa68dc | load | 4 | `sp -0x2c` | `-` |
| `strh` at 0xa68f0 | store | 2 | `sp -0x36` | `(ret(0xa1aa8) & 0xffff)` |
| `ldrh` at 0xa68f8 | load | 2 | `sp -0x34` | `-` |
| `ldr` at 0xa68fc | load | 4 | `abs 0xa6dd8` | `-` |
| `ldrh` at 0xa6910 | load | 2 | `sp -0x36` | `-` |
| `ldr` at 0xa6914 | load | 4 | `abs 0xa6dd8` | `-` |
| `ldr` at 0xa692c | load | 4 | `abs 0xa6de0` | `-` |
| `ldr` at 0xa693c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6948 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa694c | load | 4 | `abs 0xa6de4` | `-` |
| `ldr` at 0xa6958 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xa695c | load | 1 | `[sp-44] +0xe` | `-` |
| `ldr` at 0xa6960 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xa6964 | load | 1 | `[sp-44] +0xf` | `-` |
| `ldr` at 0xa696c | load | 4 | `abs 0xa6de8` | `-` |
| `ldr` at 0xa697c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa698c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xa6990 | load | 1 | `[sp-44] +0x10` | `-` |
| `ldr` at 0xa6998 | load | 4 | `abs 0xa6dec` | `-` |
| `ldr` at 0xa69a8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa69b4 | load | 4 | `sp -0x2c` | `-` |
| `strh` at 0xa69c8 | store | 2 | `sp -0x34` | `(ret(0xa1aa8) & 0xffff)` |
| `ldrh` at 0xa69d0 | load | 2 | `sp -0x34` | `-` |
| `ldrh` at 0xa69dc | load | 2 | `sp -0x34` | `-` |
| `ldr` at 0xa69f4 | load | 4 | `abs 0xa6df0` | `-` |
| `ldr` at 0xa6a00 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6a0c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xa6a10 | load | 1 | `[sp-44] +0x14` | `-` |
| `strb` at 0xa6a14 | store | 1 | `sp -0x31` | `([[sp-44]+20]:1 & 0xff)` |
| `ldrb` at 0xa6a18 | load | 1 | `sp -0x31` | `-` |
| `ldrb` at 0xa6a24 | load | 1 | `sp -0x31` | `-` |
| `ldrb` at 0xa6a38 | load | 1 | `sp -0x31` | `-` |
| `ldr` at 0xa6a54 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xa6a6c | store | 4 | `sp -0x98` | `ret(0xa1aa8)` |
| `ldr` at 0xa6a74 | load | 4 | `abs 0xa6df4` | `-` |
| `ldrb` at 0xa6a88 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xa6a94 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6a98 | load | 4 | `abs 0xa6df8` | `-` |
| `ldr` at 0xa6aa4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6aa8 | load | 4 | `abs 0xa6dfc` | `-` |
| `ldr` at 0xa6ab0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa6ab4 | store | 4 | `sp -0x30` | `[sp-36]` |
| `ldr` at 0xa6ab8 | load | 4 | `sp -0x30` | `-` |
| `strh` at 0xa6acc | store | 2 | `sp -0x38` | `(ret(0xa1aa8) & 0xffff)` |
| `ldrsh` at 0xa6ad4 | load | 2 | `sp -0x38` | `-` |
| `ldr` at 0xa6ad8 | load | 4 | `abs 0xa6dd8` | `-` |
| `ldrsh` at 0xa6af4 | load | 2 | `sp -0x38` | `-` |
| `ldr` at 0xa6af8 | load | 4 | `abs 0xa6dd8` | `-` |
| `ldr` at 0xa6b30 | load | 4 | `abs 0xa6e00` | `-` |
| `ldr` at 0xa6b40 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6b4c | load | 4 | `sp -0x30` | `-` |
| `ldrsb` at 0xa6b50 | load | 1 | `[sp-48] +0x0` | `-` |
| `ldr` at 0xa6b5c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6b60 | load | 4 | `abs 0xa6e04` | `-` |
| `ldr` at 0xa6b68 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6b6c | load | 4 | `abs 0xa6e08` | `-` |
| `ldr` at 0xa6b74 | load | 4 | `sp -0x30` | `-` |
| `strh` at 0xa6b88 | store | 2 | `sp -0x38` | `(ret(0xa1aa8) & 0xffff)` |
| `ldr` at 0xa6b8c | load | 4 | `sp -0x30` | `-` |
| `strh` at 0xa6ba0 | store | 2 | `sp -0x3a` | `(ret(0xa1aa8) & 0xffff)` |
| `ldrsh` at 0xa6ba8 | load | 2 | `sp -0x38` | `-` |
| `ldr` at 0xa6bac | load | 4 | `abs 0xa6dd8` | `-` |
| `ldrsh` at 0xa6bc8 | load | 2 | `sp -0x38` | `-` |
| `ldr` at 0xa6bcc | load | 4 | `abs 0xa6dd8` | `-` |
| `ldrsh` at 0xa6c04 | load | 2 | `sp -0x3a` | `-` |
| `ldr` at 0xa6c08 | load | 4 | `abs 0xa6dd8` | `-` |
| `str` at 0xa6c24 | store | 4 | `sp -0x98` | `?` |
| `ldrsh` at 0xa6c28 | load | 2 | `sp -0x3a` | `-` |
| `ldr` at 0xa6c2c | load | 4 | `abs 0xa6dd8` | `-` |
| `str` at 0xa6c64 | store | 4 | `sp -0x94` | `?` |
| `ldr` at 0xa6c68 | load | 4 | `abs 0xa6e0c` | `-` |
| `ldr` at 0xa6c7c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6c88 | load | 4 | `sp -0x30` | `-` |
| `strh` at 0xa6c9c | store | 2 | `sp -0x34` | `(ret(0xa1aa8) & 0xffff)` |
| `ldrh` at 0xa6ca4 | load | 2 | `sp -0x34` | `-` |
| `ldr` at 0xa6cac | load | 4 | `abs 0xa6e10` | `-` |
| `ldr` at 0xa6cb8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6cc4 | load | 4 | `sp -0x30` | `-` |
| `strh` at 0xa6cd8 | store | 2 | `sp -0x34` | `(ret(0xa1aa8) & 0xffff)` |
| `ldr` at 0xa6cdc | load | 4 | `sp -0x30` | `-` |
| `strh` at 0xa6cf0 | store | 2 | `sp -0x36` | `(ret(0xa1aa8) & 0xffff)` |
| `ldrh` at 0xa6cf8 | load | 2 | `sp -0x34` | `-` |
| `ldr` at 0xa6cfc | load | 4 | `abs 0xa6e14` | `-` |
| `ldrh` at 0xa6d10 | load | 2 | `sp -0x34` | `-` |
| `ldr` at 0xa6d14 | load | 4 | `abs 0xa6e14` | `-` |
| `ldrh` at 0xa6d44 | load | 2 | `sp -0x36` | `-` |
| `ldr` at 0xa6d48 | load | 4 | `abs 0xa6e14` | `-` |
| `str` at 0xa6d5c | store | 4 | `sp -0x98` | `?` |
| `ldrh` at 0xa6d60 | load | 2 | `sp -0x36` | `-` |
| `ldr` at 0xa6d64 | load | 4 | `abs 0xa6e14` | `-` |
| `str` at 0xa6d94 | store | 4 | `sp -0x94` | `?` |
| `ldr` at 0xa6d98 | load | 4 | `abs 0xa6e18` | `-` |
| `ldr` at 0xa6dac | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xa6dbc | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xa6dbc | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xa6dbc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa6dbc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa6dbc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa6748 | `(unknown callee) 0x19808` | `[sp-40]` | `0x113c1c` | `r3` | `[sp-29]:1` |
| `bl` at 0xa6758 | `(unknown callee) 0x19808` | `[sp-40]` | `0x113c30` | `r3` | `[sp-29]:1` |
| `bl` at 0xa6768 | `(unknown callee) 0xa1aa8` | `[sp-36]` | `?` | `?` | `[sp-36]` |
| `bl` at 0xa6784 | `(unknown callee) 0x197a8` | `sp-140` | `0x113c40` | `(ret(0xa1aa8) & 0xffff)` | `sp-140` |
| `bl` at 0xa6794 | `(unknown callee) 0x195e0` | `[sp-40]` | `sp-140` | `?` | `sp-140` |
| `bl` at 0xa67a4 | `(unknown callee) 0xa1aa8` | `[sp-44]+2` | `?` | `?` | `[sp-44]+2` |
| `bl` at 0xa67d0 | `(unknown callee) 0x197a8` | `sp-140` | `0x113c60` | `(ret(0xa1aa8) & 0xffff)` | `sp-140` |
| `bl` at 0xa67e0 | `(unknown callee) 0x195e0` | `[sp-40]` | `sp-140` | `?` | `sp-140` |
| `bl` at 0xa6814 | `(unknown callee) 0x197a8` | `sp-140` | `0x113c74` | `[[sp-44]+4]:1` | `[[sp-44]+5]:1` |
| `bl` at 0xa6824 | `(unknown callee) 0x195e0` | `[sp-40]` | `sp-140` | `?` | `sp-140` |
| `bl` at 0xa6834 | `(unknown callee) 0xa1aa8` | `[sp-44]+6` | `?` | `?` | `[sp-44]+6` |
| `bl` at 0xa6858 | `(unknown callee) 0xa1aa8` | `[sp-44]+8` | `?` | `?` | `[sp-44]+8` |
| `bl` at 0xa68a4 | `(unknown callee) 0x197a8` | `sp-140` | `0x113ca4` | `?` | `?` |
| `bl` at 0xa68b4 | `(unknown callee) 0x195e0` | `[sp-40]` | `sp-140` | `?` | `sp-140` |
| `bl` at 0xa68c4 | `(unknown callee) 0xa1aa8` | `[sp-44]+10` | `?` | `?` | `[sp-44]+10` |
| `bl` at 0xa68e8 | `(unknown callee) 0xa1aa8` | `[sp-44]+12` | `?` | `?` | `[sp-44]+12` |
| `bl` at 0xa6934 | `(unknown callee) 0x197a8` | `sp-140` | `0x113ccc` | `?` | `?` |
| `bl` at 0xa6944 | `(unknown callee) 0x195e0` | `[sp-40]` | `sp-140` | `?` | `sp-140` |
| `bl` at 0xa6950 | `(unknown callee) 0x195e0` | `[sp-40]` | `0x113cdc` | `?` | `?` |
| `bl` at 0xa6974 | `(unknown callee) 0x197a8` | `sp-140` | `0x113ce4` | `[[sp-44]+14]:1` | `[[sp-44]+15]:1` |
| `bl` at 0xa6984 | `(unknown callee) 0x195e0` | `[sp-40]` | `sp-140` | `?` | `sp-140` |
| `bl` at 0xa69a0 | `(unknown callee) 0x197a8` | `sp-140` | `0x113d08` | `[[sp-44]+16]:1` | `[[sp-44]+16]:1` |
| `bl` at 0xa69b0 | `(unknown callee) 0x195e0` | `[sp-40]` | `sp-140` | `?` | `sp-140` |
| `bl` at 0xa69c0 | `(unknown callee) 0xa1aa8` | `[sp-44]+18` | `?` | `?` | `[sp-44]+18` |
| `bl` at 0xa69f8 | `(unknown callee) 0x197a8` | `sp-140` | `0x113d2c` | `?` | `?` |
| `bl` at 0xa6a08 | `(unknown callee) 0x195e0` | `[sp-40]` | `sp-140` | `?` | `sp-140` |
| `bl` at 0xa6a30 | `(unknown callee) 0xa6e1c` | `?` | `?` | `?` | `?` |
| `bl` at 0xa6a4c | `(unknown callee) 0xa6e1c` | `?` | `?` | `?` | `?` |
| `bl` at 0xa6a60 | `(unknown callee) 0xa1aa8` | `[sp-44]+21` | `?` | `?` | `[sp-44]+21` |
| `bl` at 0xa6a80 | `(unknown callee) 0x197a8` | `sp-140` | `0x113d4c` | `ret(0xa6e1c)` | `ret(0xa6e1c)` |
| `bl` at 0xa6a9c | `(unknown callee) 0x19808` | `[sp-40]` | `0x113d70` | `[sp-28]` | `[sp-29]:1` |
| `bl` at 0xa6aac | `(unknown callee) 0x19808` | `[sp-40]` | `0x113d80` | `[sp-28]` | `[sp-29]:1` |
| `bl` at 0xa6ac4 | `(unknown callee) 0xa1aa8` | `[sp-36]+1` | `?` | `?` | `[sp-36]+1` |
| `bl` at 0xa6b38 | `(unknown callee) 0x197a8` | `sp-140` | `0x113d90` | `?` | `?` |
| `bl` at 0xa6b48 | `(unknown callee) 0x195e0` | `[sp-40]` | `sp-140` | `?` | `sp-140` |
| `bl` at 0xa6b64 | `(unknown callee) 0x195e0` | `[sp-40]` | `0x113db0` | `?` | `?` |
| `bl` at 0xa6b70 | `(unknown callee) 0x195e0` | `[sp-40]` | `0x113dbc` | `?` | `?` |
| `bl` at 0xa6b80 | `(unknown callee) 0xa1aa8` | `[sp-48]+3` | `?` | `?` | `[sp-48]+3` |
| `bl` at 0xa6b98 | `(unknown callee) 0xa1aa8` | `[sp-48]+5` | `?` | `?` | `[sp-48]+5` |
| `bl` at 0xa6c74 | `(unknown callee) 0x197a8` | `sp-140` | `0x113dc0` | `?` | `?` |
| `bl` at 0xa6c84 | `(unknown callee) 0x195e0` | `[sp-40]` | `sp-140` | `?` | `sp-140` |
| `bl` at 0xa6c94 | `(unknown callee) 0xa1aa8` | `[sp-48]+7` | `?` | `?` | `[sp-48]+7` |
| `bl` at 0xa6cb0 | `(unknown callee) 0x197a8` | `sp-140` | `0x113dfc` | `(ret(0xa1aa8) & 0xffff)` | `sp-140` |
| `bl` at 0xa6cc0 | `(unknown callee) 0x195e0` | `[sp-40]` | `sp-140` | `?` | `sp-140` |
| `bl` at 0xa6cd0 | `(unknown callee) 0xa1aa8` | `[sp-48]+9` | `?` | `?` | `[sp-48]+9` |
| `bl` at 0xa6ce8 | `(unknown callee) 0xa1aa8` | `[sp-48]+11` | `?` | `?` | `[sp-48]+11` |
| `bl` at 0xa6da4 | `(unknown callee) 0x197a8` | `sp-140` | `0x113e28` | `?` | `?` |
| `bl` at 0xa6db4 | `(unknown callee) 0x195e0` | `[sp-40]` | `sp-140` | `?` | `sp-140` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa6720 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `beq` at 0xa672c | `[sp-28] == 0x1` | `[sp-28] != 0x1` |
| `beq` at 0xa673c | `[sp-29]:1 == 0x18` | `[sp-29]:1 != 0x18` |
| `beq` at 0xa67bc | `(ret(0xa1aa8) & 0xffff) == 0xffff` | `(ret(0xa1aa8) & 0xffff) != 0xffff` |
| `beq` at 0xa67f0 | `[[sp-44]+4]:1 == 0xff` | `[[sp-44]+4]:1 != 0xff` |
| `beq` at 0xa6848 | `(ret(0xa1aa8) & 0xffff) == 0x0` | `(ret(0xa1aa8) & 0xffff) != 0x0` |
| `beq` at 0xa68d8 | `(ret(0xa1aa8) & 0xffff) == 0x0` | `(ret(0xa1aa8) & 0xffff) != 0x0` |
| `beq` at 0xa6a20 | `([[sp-44]+20]:1 & 0xff) == 0x0` | `([[sp-44]+20]:1 & 0xff) != 0x0` |
| `beq` at 0xa6a90 | `[sp-29]:1 == 0x10` | `[sp-29]:1 != 0x10` |
| `bge` at 0xa6b58 | `? >= 0x0 (signed, 32-bit)` | `? < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa6dbc | `r0` | `?` |

## `brickConvertType`

- `brickConvertType`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8050 to 0xb8484.
  Established. An included unnamed `0x197a8` call on known edges carries `0x133f34`, `0x11ea50`, entry `r0`, `r0-35` (`bl` at 0xb83ec).
  The return record preserves `0x133f34` (`ldm` at 0xb83f8).
  Brick type-to-name mappings behind the computed transfer remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb8060 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb8064 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0xb8070 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb83e0 | load | 4 | `abs 0xb83fc` | `-` |
| `ldr` at 0xb83e4 | load | 4 | `abs 0xb8480` | `-` |
| `ldr` at 0xb83e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb83f0 | load | 4 | `abs 0xb83fc` | `-` |
| `ldm` at 0xb83f8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb83f8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb83f8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb83f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb83ec | `(unknown callee) 0x197a8` | `0x133f34` | `0x11ea50` | `r0` | `r0-35` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xb8070 | `r0-35 <= 0x59 (unsigned, 32-bit)` | `r0-35 > 0x59 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb83f8 | `r0` | `0x133f34` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xb8070).

## `brickConvertString`

- `brickConvertString`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7c7c to 0xb8050.
  Established. Inequality `ret(0x19538) != 0x1` takes `0xb7dc4`, while equality falls through (`bne` at 0xb7ca8).
  Included word stores write `0x3f` and `0x43` to `sp -0x18` on known edges (`str` at 0xb7c94, `str` at 0xb7de0).
  Brick string conversion cases behind the transfer remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb7c8c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb7c94 | store | 4 | `sp -0x18` | `0x3f` |
| `ldr` at 0xb7c98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7cac | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb7cb0 | load | 1 | `[sp-20] +0x0` | `-` |
| `ldrls` at 0xb7cc8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb7dc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7dc8 | load | 4 | `abs 0xb8010` | `-` |
| `str` at 0xb7de0 | store | 4 | `sp -0x18` | `0x43` |
| `ldr` at 0xb7de8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7dec | load | 4 | `abs 0xb8014` | `-` |
| `str` at 0xb7e04 | store | 4 | `sp -0x18` | `0x33` |
| `ldr` at 0xb7e0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7e10 | load | 4 | `abs 0xb8018` | `-` |
| `str` at 0xb7e28 | store | 4 | `sp -0x18` | `0x34` |
| `ldr` at 0xb7e30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7e34 | load | 4 | `abs 0xb801c` | `-` |
| `str` at 0xb7e4c | store | 4 | `sp -0x18` | `0x35` |
| `ldr` at 0xb7e54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7e58 | load | 4 | `abs 0xb8020` | `-` |
| `str` at 0xb7e70 | store | 4 | `sp -0x18` | `0x36` |
| `ldr` at 0xb7e78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7e7c | load | 4 | `abs 0xb8024` | `-` |
| `str` at 0xb7e94 | store | 4 | `sp -0x18` | `0x40` |
| `ldr` at 0xb7e9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7ea0 | load | 4 | `abs 0xb8028` | `-` |
| `str` at 0xb7eb8 | store | 4 | `sp -0x18` | `0x4f` |
| `ldr` at 0xb7ec0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7ec4 | load | 4 | `abs 0xb802c` | `-` |
| `str` at 0xb7edc | store | 4 | `sp -0x18` | `0x42` |
| `ldr` at 0xb7ee4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7ee8 | load | 4 | `abs 0xb8030` | `-` |
| `str` at 0xb7f00 | store | 4 | `sp -0x18` | `0x4c` |
| `ldr` at 0xb7f08 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7f0c | load | 4 | `abs 0xb8034` | `-` |
| `str` at 0xb7f24 | store | 4 | `sp -0x18` | `0x5a` |
| `ldr` at 0xb7f2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7f30 | load | 4 | `abs 0xb8038` | `-` |
| `str` at 0xb7f48 | store | 4 | `sp -0x18` | `0x7c` |
| `ldr` at 0xb7f50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7f54 | load | 4 | `abs 0xb803c` | `-` |
| `str` at 0xb7f6c | store | 4 | `sp -0x18` | `0x44` |
| `ldr` at 0xb7f74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7f78 | load | 4 | `abs 0xb8040` | `-` |
| `str` at 0xb7f90 | store | 4 | `sp -0x18` | `0x53` |
| `ldr` at 0xb7f98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7f9c | load | 4 | `abs 0xb8044` | `-` |
| `str` at 0xb7fb4 | store | 4 | `sp -0x18` | `0x31` |
| `ldr` at 0xb7fbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7fc0 | load | 4 | `abs 0xb8048` | `-` |
| `str` at 0xb7fd8 | store | 4 | `sp -0x18` | `0x32` |
| `ldr` at 0xb7fe0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7fe4 | load | 4 | `abs 0xb804c` | `-` |
| `str` at 0xb7ffc | store | 4 | `sp -0x18` | `0x54` |
| `ldr` at 0xb8000 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xb800c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb800c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb800c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7c9c | `(unknown callee) 0x19538` | `r0` | `r1` | `r2` | `0x3f` |
| `bl` at 0xb7cb8 | `(unknown callee) 0x195c8` | `[[sp-20]]:1` | `?` | `?` | `[[sp-20]]:1` |
| `bl` at 0xb7dcc | `(unknown callee) 0x19310` | `[sp-20]` | `0x11e884` | `?` | `ret(0x19538)` |
| `bl` at 0xb7df0 | `(unknown callee) 0x19310` | `[sp-20]` | `0x11e88c` | `?` | `ret(0x19310)` |
| `bl` at 0xb7e14 | `(unknown callee) 0x19310` | `[sp-20]` | `0x11e894` | `?` | `ret(0x19310)` |
| `bl` at 0xb7e38 | `(unknown callee) 0x19310` | `[sp-20]` | `0x11e89c` | `?` | `ret(0x19310)` |
| `bl` at 0xb7e5c | `(unknown callee) 0x19310` | `[sp-20]` | `0x11e8a4` | `?` | `ret(0x19310)` |
| `bl` at 0xb7e80 | `(unknown callee) 0x19310` | `[sp-20]` | `0x11e8ac` | `?` | `ret(0x19310)` |
| `bl` at 0xb7ea4 | `(unknown callee) 0x19310` | `[sp-20]` | `0x11e8b4` | `?` | `ret(0x19310)` |
| `bl` at 0xb7ec8 | `(unknown callee) 0x19310` | `[sp-20]` | `0x11e8bc` | `?` | `ret(0x19310)` |
| `bl` at 0xb7eec | `(unknown callee) 0x19310` | `[sp-20]` | `0x11e8c4` | `?` | `ret(0x19310)` |
| `bl` at 0xb7f10 | `(unknown callee) 0x19310` | `[sp-20]` | `0x11e8cc` | `?` | `ret(0x19310)` |
| `bl` at 0xb7f34 | `(unknown callee) 0x19310` | `[sp-20]` | `0x11e8d4` | `?` | `ret(0x19310)` |
| `bl` at 0xb7f58 | `(unknown callee) 0x19310` | `[sp-20]` | `0x11e8dc` | `?` | `ret(0x19310)` |
| `bl` at 0xb7f7c | `(unknown callee) 0x19310` | `[sp-20]` | `0x11e8e4` | `?` | `ret(0x19310)` |
| `bl` at 0xb7fa0 | `(unknown callee) 0x19310` | `[sp-20]` | `0x11e8ec` | `?` | `ret(0x19310)` |
| `bl` at 0xb7fc4 | `(unknown callee) 0x19310` | `[sp-20]` | `0x11e8f0` | `?` | `ret(0x19310)` |
| `bl` at 0xb7fe8 | `(unknown callee) 0x19310` | `[sp-20]` | `0x11e8f4` | `?` | `ret(0x19310)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb7ca8 | `ret(0x19538) != 0x1` | `ret(0x19538) == 0x1` |
| `ldrls` at 0xb7cc8 | `ret(0x195c8)-35 <= 0x36 (unsigned, 32-bit)` | `ret(0x195c8)-35 > 0x36 (unsigned, 32-bit)` |
| `bne` at 0xb7dd8 | `ret(0x19310) != 0x0` | `ret(0x19310) == 0x0` |
| `bne` at 0xb7dfc | `ret(0x19310) != 0x0` | `ret(0x19310) == 0x0` |
| `bne` at 0xb7e20 | `ret(0x19310) != 0x0` | `ret(0x19310) == 0x0` |
| `bne` at 0xb7e44 | `ret(0x19310) != 0x0` | `ret(0x19310) == 0x0` |
| `bne` at 0xb7e68 | `ret(0x19310) != 0x0` | `ret(0x19310) == 0x0` |
| `bne` at 0xb7e8c | `ret(0x19310) != 0x0` | `ret(0x19310) == 0x0` |
| `bne` at 0xb7eb0 | `ret(0x19310) != 0x0` | `ret(0x19310) == 0x0` |
| `bne` at 0xb7ed4 | `ret(0x19310) != 0x0` | `ret(0x19310) == 0x0` |
| `bne` at 0xb7ef8 | `ret(0x19310) != 0x0` | `ret(0x19310) == 0x0` |
| `bne` at 0xb7f1c | `ret(0x19310) != 0x0` | `ret(0x19310) == 0x0` |
| `bne` at 0xb7f40 | `ret(0x19310) != 0x0` | `ret(0x19310) == 0x0` |
| `bne` at 0xb7f64 | `ret(0x19310) != 0x0` | `ret(0x19310) == 0x0` |
| `bne` at 0xb7f88 | `ret(0x19310) != 0x0` | `ret(0x19310) == 0x0` |
| `bne` at 0xb7fac | `ret(0x19310) != 0x0` | `ret(0x19310) == 0x0` |
| `bne` at 0xb7fd0 | `ret(0x19310) != 0x0` | `ret(0x19310) == 0x0` |
| `bne` at 0xb7ff4 | `ret(0x19310) != 0x0` | `ret(0x19310) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb800c | `r0` | `[sp-24]` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xb7cc8).

## `brickTypeToChar`

- `brickTypeToChar`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8484 to 0xb87b8.
  Established. The unsigned condition `r0-35 <= 0x59` takes an unresolved target, while greater falls through (`ldrls` at 0xb84a4).
  The unnamed `0x19400` call on known edges carries entry `r0`, entry `r1`, entry `r2`, `r0-35` (`bl` at 0xb87a4).
  Brick type character mappings behind the transfer remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb8494 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb8498 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0xb84a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb87a0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb87b4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb87b4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb87b4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb87b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb87a4 | `(unknown callee) 0x19400` | `r0` | `r1` | `r2` | `r0-35` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xb84a4 | `r0-35 <= 0x59 (unsigned, 32-bit)` | `r0-35 > 0x59 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb87b4 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xb84a4).

## `IcmbMsgRqBuild`

- `IcmbMsgRqBuild`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa1d68 to 0xa1de8.
  Established. The unnamed `0x19610` call carries `[sp]+2`, `[sp-28]`, `[sp-32]`, `[sp]+2` (`bl` at 0xa1dd4).
  A word assignment preserves `r3+2` at `sp -0x24` (`str` at 0xa1d94).
  The integer return candidate is `[sp-36]` (`ldm` at 0xa1de4).
  ICMB request payload capacity remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa1d78 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa1d7c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xa1d80 | store | 4 | `sp -0x20` | `r3` |
| `strb` at 0xa1d88 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xa1d8c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa1d94 | store | 4 | `sp -0x24` | `r3+2` |
| `ldr` at 0xa1d98 | load | 4 | `sp +0x0` | `-` |
| `ldrb` at 0xa1da4 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xa1da8 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xa1db4 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xa1db8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa1dc0 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xa1dcc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa1dd0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa1dd8 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xa1de4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa1de4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa1de4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa1da0 | `(unknown callee) 0xa1b88` | `[sp]` | `0x0` | `r2` | `r3+2` |
| `bl` at 0xa1db0 | `(unknown callee) 0xa1c18` | `[sp]` | `[sp-17]:1` | `?` | `[sp-17]:1` |
| `bl` at 0xa1dbc | `(unknown callee) 0xa1ca0` | `[sp]` | `[sp-24]` | `?` | `?` |
| `bl` at 0xa1dd4 | `(unknown callee) 0x19610` | `[sp]+2` | `[sp-28]` | `[sp-32]` | `[sp]+2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa1de4 | `r0` | `[sp-36]` |

## `charioPort`

- `charioPort`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x22030 to 0x22050.
  Established. A word read addresses `0x12f674` (`ldr` at 0x22040).
  The return record preserves `[0x12f674]` (`ldm` at 0x22048).
  Character-I/O port identity remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x2203c | load | 4 | `abs 0x2204c` | `-` |
| `ldr` at 0x22040 | load | 4 | `abs 0x12f674` | `-` |
| `ldm` at 0x22048 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x22048 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x22048 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x22048 | `r0` | `[0x12f674]` |

## `promIceEnabled`

- `promIceEnabled`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3e9a0 to 0x3eab8.
  Established. The unnamed `0xca688` call carries `0x0`, `[[sp-24]]:1`, `[[sp-24]+1]:1`, `[[sp-24]+1]:1` (`bl` at 0x3ea14).
  A zero `ret(0xca688)` takes `0x3ea44`, while nonzero falls through (`beq` at 0x3ea30).
  PROM enable-status coding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x3e9b4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x3e9b8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x3e9d0 | store | 4 | `sp -0x1c` | `ret(0x3edc8)` |
| `ldr` at 0x3e9d4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3e9e0 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x3e9f8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3e9fc | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x3ea00 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3ea04 | load | 1 | `[sp-24] +0x1` | `-` |
| `str` at 0x3ea1c | store | 4 | `sp -0x1c` | `ret(0xca688)` |
| `ldr` at 0x3ea20 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3ea24 | store | 4 | `sp -0x24` | `ret(0xca688)` |
| `ldr` at 0x3ea28 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3ea34 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3ea48 | store | 4 | `sp -0x20` | `0x1` |
| `str` at 0x3ea54 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x3ea5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3ea68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3ea74 | load | 4 | `abs 0x3eab0` | `-` |
| `str` at 0x3ea78 | store | 4 | `sp -0x30` | `0xf64d4` |
| `ldrb` at 0x3ea7c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x3ea80 | store | 4 | `sp -0x2c` | `[sp-17]:1` |
| `str` at 0x3ea84 | store | 4 | `sp -0x28` | `ret(0x1fc1c)` |
| `ldr` at 0x3ea8c | load | 4 | `abs 0x3eab4` | `-` |
| `str` at 0x3eaa0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x3eaa4 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x3eaac | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3eaac | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3eaac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3e9c8 | `(unknown callee) 0x3edc8` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `bl` at 0x3e9e8 | `(unknown callee) 0x3e93c` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x3ea14 | `(unknown callee) 0xca688` | `0x0` | `[[sp-24]]:1` | `[[sp-24]+1]:1` | `[[sp-24]+1]:1` |
| `bl` at 0x3ea6c | `(unknown callee) 0x1fc1c` | `?` | `?` | `?` | `?` |
| `bl` at 0x3ea98 | `(unknown callee) 0x200a0` | `0x1` | `0xf6480` | `0xa9` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3e9dc | `ret(0x3edc8) != 0x0` | `ret(0x3edc8) == 0x0` |
| `beq` at 0x3e9f4 | `ret(0x3e93c) == 0x0` | `ret(0x3e93c) != 0x0` |
| `beq` at 0x3ea30 | `ret(0xca688) == 0x0` | `ret(0xca688) != 0x0` |
| `beq` at 0x3ea3c | `ret(0xca688) == 0x1` | `ret(0xca688) != 0x1` |
| `beq` at 0x3ea64 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3eaac | `r0` | `?` |

## `ramLogReset`

- `ramLogReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xacde4 to 0xace2c.
  Established. A nonzero entry `r0` takes `0xace08`, while zero falls through (`bne` at 0xace00).
  Word stores copy `[r0+8]` to entry `r0 +0x10` and `[[sp-20]+8]` to `[sp-20] +0x14` (`str` at 0xace14, `str` at 0xace24).
  RAM-log cursor meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xacdf4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xacdf8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xace08 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xace0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xace10 | load | 4 | `r0 +0x8` | `-` |
| `str` at 0xace14 | store | 4 | `r0 +0x10` | `[r0+8]` |
| `ldr` at 0xace18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xace1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xace20 | load | 4 | `[sp-20] +0x8` | `-` |
| `str` at 0xace24 | store | 4 | `[sp-20] +0x14` | `[[sp-20]+8]` |
| `ldm` at 0xace28 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xace28 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xace28 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xace28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xace00 | `r0 != 0x0` | `r0 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xace28 | `r0` | `r0` |

## `exceptionBootComplete`

- `exceptionBootComplete`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1efe0 to 0x1eff8.
  Established. The integer return candidate is `0x0` (`ldm` at 0x1eff4).
  Boot-completion role beyond the stack and return sequence remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldm` at 0x1eff4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1eff4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1eff4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1eff4 | `r0` | `0x0` |

## `exceptionInit`

- `exceptionInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1ee64 to 0x1eeb0.
  Established. A word store writes an unknown value to `0x12ef68` (`str` at 0x1ee94).
  The unnamed `0x1946c` call carries entry `r0`, entry `r1`, entry `r2`, `0xeb9c0` (`bl` at 0x1ee80).
  Exception initialization flag semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x1ee74 | load | 4 | `abs 0x1eea8` | `-` |
| `str` at 0x1ee78 | store | 4 | `sp -0x1c` | `0xeb9c0` |
| `ldr` at 0x1ee7c | load | 4 | `abs 0x1eeac` | `-` |
| `str` at 0x1ee94 | store | 4 | `abs 0x12ef68` | `?` |
| `ldm` at 0x1eea4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x1eea4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1eea4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1eea4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1ee80 | `(unknown callee) 0x1946c` | `r0` | `r1` | `r2` | `0xeb9c0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1eea4 | `r0` | `0x0` |
