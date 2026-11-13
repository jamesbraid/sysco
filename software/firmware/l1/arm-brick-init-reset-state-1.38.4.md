# ARM brick initialization and reset state in 1.38.4

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
A computed transfer in brickInit remains unresolved. Tables and return values cover only known edges, with the excluded transfer site listed below its entry. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/brick-init-reset-state-facts.txt) retain all tool sections.

## `softReset`

- `softReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4e44c to 0x4e4d8.
  Established. The entry comparison takes its bypass for `r0 == 0x5` and falls through on inequality
  (`beq` at 0x4e470). An indirect call has unknown target, three unknown candidates and `[0x2ae5d0]`
  in r3 (`mov` at 0x4e4b0). The byte store at `sp -0x15` carries `(r1 & 0xff)` (`strb` at 0x4e464).
  Physical reset behavior and r0 at the joined exit remain unknown (`ldmdb` at 0x4e4cc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4e45c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x4e464 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x4e468 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4e474 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4e480 | load | 4 | `abs 0x4e4d0` | `-` |
| `ldr` at 0x4e48c | load | 4 | `abs 0x4e4d4` | `-` |
| `ldr` at 0x4e490 | load | 4 | `abs 0x2ae5d0` | `-` |
| `ldr` at 0x4e49c | load | 4 | `abs 0x4e4d4` | `-` |
| `ldrb` at 0x4e4a0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x4e4a4 | load | 4 | `abs 0x2ae5d0` | `-` |
| `str` at 0x4e4b8 | store | 4 | `sp -0x1c` | `ret(unknown)` |
| `str` at 0x4e4c4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x4e4c8 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x4e4cc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4e4cc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4e4cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4e478 | `(unknown callee) 0x4e650` | `r0` | `r1` | `r2` | `r0` |
| `bl` at 0x4e488 | `(unknown callee) 0x2070c` | `0x10a5d0` | `ret(0x4e650)` | `?` | `ret(0x4e650)` |
| `mov` at 0x4e4b0 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `[0x2ae5d0]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4e470 | `r0 == 0x5` | `r0 != 0x5` |
| `beq` at 0x4e498 | `[0x2ae5d0] == 0x0` | `[0x2ae5d0] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4e4cc | `r0` | `?` |

## `brickInit`

- `brickInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4e7b0 to 0x4f090.
  Established. Word stores write `0x1` and later `0x2` at absolute `0x2ae4c0` (`str` at 0x4e7d8,
  `str` at 0x4ee60). A separate byte store sets `0x148249` to `0x2` (`strb` at 0x4e7e4). The call to
  `0xd1ae8` uses `0x0`, `0xff`, `sp-136`, `sp-136` (`bl` at 0x4e800); its nonzero result takes the
  branch and zero falls through (`bne` at 0x4e80c). The known-edge exit carries `ret(0x1b910)`
  (`ldmdb` at 0x4f028). Board-type encodings and behavior beyond the unresolved selector remain
  unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4e7c4 | store | 4 | `sp -0x94` | `0x1` |
| `ldr` at 0x4e7d0 | load | 4 | `abs 0x4f02c` | `-` |
| `str` at 0x4e7d8 | store | 4 | `abs 0x2ae4c0` | `0x1` |
| `ldr` at 0x4e7dc | load | 4 | `abs 0x4f030` | `-` |
| `strb` at 0x4e7e4 | store | 1 | `abs 0x148249` | `0x2` |
| `strb` at 0x4e7ec | store | 1 | `sp -0x8e` | `?` |
| `ldr` at 0x4e810 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x4e814 | store | 4 | `sp -0x8c` | `[sp-60]` |
| `ldr` at 0x4e820 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4e824 | load | 4 | `abs 0x4f034` | `-` |
| `strb` at 0x4e840 | store | 1 | `sp -0x8e` | `?` |
| `str` at 0x4e848 | store | 4 | `sp -0x94` | `0x0` |
| `ldr` at 0x4e850 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4e854 | load | 4 | `abs 0x4f038` | `-` |
| `ldr` at 0x4e86c | load | 4 | `abs 0x4f03c` | `-` |
| `strb` at 0x4e874 | store | 1 | `abs 0x14279c` | `0x4c` |
| `ldr` at 0x4e878 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x4e87c | load | 4 | `abs 0x4f040` | `-` |
| `ldr` at 0x4e894 | load | 4 | `abs 0x4f044` | `-` |
| `strb` at 0x4e89c | store | 1 | `abs 0x148248` | `0x7` |
| `ldr` at 0x4e8a4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4e8a8 | load | 4 | `abs 0x4f048` | `-` |
| `ldr` at 0x4e8ec | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x4e8f0 | store | 4 | `sp -0x8c` | `[sp-60]` |
| `ldr` at 0x4e8fc | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4e900 | load | 4 | `abs 0x4f04c` | `-` |
| `ldr` at 0x4e918 | load | 4 | `abs 0x4f03c` | `-` |
| `strb` at 0x4e920 | store | 1 | `abs 0x14279c` | `0x5a` |
| `ldr` at 0x4e928 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4e92c | load | 4 | `abs 0x4f050` | `-` |
| `ldr` at 0x4e944 | load | 4 | `abs 0x4f03c` | `-` |
| `strb` at 0x4e94c | store | 1 | `abs 0x14279c` | `0x7c` |
| `ldr` at 0x4e954 | load | 4 | `abs 0x4f054` | `-` |
| `ldr` at 0x4e958 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4e984 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x4e988 | store | 4 | `sp -0x8c` | `[sp-60]` |
| `ldr` at 0x4e994 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4e998 | load | 4 | `abs 0x4f04c` | `-` |
| `ldr` at 0x4e9b0 | load | 4 | `abs 0x4f03c` | `-` |
| `strb` at 0x4e9b8 | store | 1 | `abs 0x14279c` | `0x5a` |
| `ldr` at 0x4e9c0 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4e9c4 | load | 4 | `abs 0x4f050` | `-` |
| `ldr` at 0x4e9dc | load | 4 | `abs 0x4f03c` | `-` |
| `strb` at 0x4e9e4 | store | 1 | `abs 0x14279c` | `0x7c` |
| `ldr` at 0x4e9ec | load | 4 | `abs 0x4f054` | `-` |
| `ldr` at 0x4e9f0 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4e9fc | load | 4 | `abs 0x4f03c` | `-` |
| `strb` at 0x4ea04 | store | 1 | `abs 0x14279c` | `0x5a` |
| `ldr` at 0x4ea0c | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4ea10 | load | 4 | `abs 0x4f058` | `-` |
| `strb` at 0x4ea2c | store | 1 | `sp -0x8e` | `?` |
| `ldr` at 0x4ea34 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4ea38 | load | 4 | `abs 0x4f05c` | `-` |
| `strb` at 0x4ea54 | store | 1 | `sp -0x8e` | `?` |
| `ldr` at 0x4ea64 | load | 4 | `abs 0x4f03c` | `-` |
| `ldrb` at 0x4ea68 | load | 1 | `abs 0x14279c` | `-` |
| `ldrb` at 0x4ea74 | load | 1 | `sp -0x8e` | `-` |
| `strb` at 0x4ea78 | store | 1 | `sp -0x8d` | `([sp-142]:1 & 0xff)` |
| `strb` at 0x4ea80 | store | 1 | `sp -0x8e` | `?` |
| `ldrb` at 0x4ea84 | load | 1 | `sp -0x8d` | `-` |
| `ldr` at 0x4eaa4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x4eaa8 | store | 4 | `sp -0x8c` | `[sp-60]` |
| `strb` at 0x4eab8 | store | 1 | `sp -0x95` | `?` |
| `ldr` at 0x4eabc | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4eac0 | load | 4 | `abs 0x4f060` | `-` |
| `ldr` at 0x4ead8 | load | 4 | `abs 0x4f03c` | `-` |
| `strb` at 0x4eae0 | store | 1 | `abs 0x14279c` | `0x44` |
| `ldr` at 0x4eae8 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4eaec | load | 4 | `abs 0x4f064` | `-` |
| `ldr` at 0x4eb04 | load | 4 | `abs 0x4f03c` | `-` |
| `strb` at 0x4eb0c | store | 1 | `abs 0x14279c` | `0x40` |
| `ldr` at 0x4eb14 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4eb18 | load | 4 | `abs 0x4f068` | `-` |
| `ldr` at 0x4eb30 | load | 4 | `abs 0x4f03c` | `-` |
| `strb` at 0x4eb38 | store | 1 | `abs 0x14279c` | `0x42` |
| `ldr` at 0x4eb40 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4eb44 | load | 4 | `abs 0x4f06c` | `-` |
| `ldr` at 0x4eb5c | load | 4 | `abs 0x4f03c` | `-` |
| `str` at 0x4eb60 | store | 4 | `sp -0x9c` | `0x14279c` |
| `ldr` at 0x4eb70 | load | 4 | `abs 0x4f070` | `-` |
| `strb` at 0x4eb88 | store | 1 | `sp -0x9d` | `0x48` |
| `strb` at 0x4eb94 | store | 1 | `sp -0x9d` | `0x46` |
| `ldrb` at 0x4eb98 | load | 1 | `sp -0x9d` | `-` |
| `ldr` at 0x4eb9c | load | 4 | `sp -0x9c` | `-` |
| `strb` at 0x4eba0 | store | 1 | `[sp-156] +0x0` | `([sp-157]:1 & 0xff)` |
| `ldr` at 0x4eba8 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4ebac | load | 4 | `abs 0x4f074` | `-` |
| `ldr` at 0x4ebc4 | load | 4 | `abs 0x4f03c` | `-` |
| `strb` at 0x4ebcc | store | 1 | `abs 0x14279c` | `0x53` |
| `ldr` at 0x4ebd4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4ebd8 | load | 4 | `abs 0x4f078` | `-` |
| `ldr` at 0x4ebf0 | load | 4 | `abs 0x4f03c` | `-` |
| `strb` at 0x4ebf8 | store | 1 | `abs 0x14279c` | `0x4d` |
| `ldr` at 0x4ec08 | load | 4 | `abs 0x4f03c` | `-` |
| `ldrb` at 0x4ec0c | load | 1 | `abs 0x14279c` | `-` |
| `ldrb` at 0x4ec18 | load | 1 | `sp -0x8e` | `-` |
| `strb` at 0x4ec1c | store | 1 | `sp -0x8d` | `([sp-142]:1 & 0xff)` |
| `strb` at 0x4ec24 | store | 1 | `sp -0x8e` | `?` |
| `ldrb` at 0x4ec28 | load | 1 | `sp -0x8d` | `-` |
| `ldr` at 0x4ec48 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x4ec4c | store | 4 | `sp -0x8c` | `[sp-60]` |
| `ldr` at 0x4ec58 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4ec5c | load | 4 | `abs 0x4f07c` | `-` |
| `ldr` at 0x4ec74 | load | 4 | `abs 0x4f03c` | `-` |
| `strb` at 0x4ec7c | store | 1 | `abs 0x14279c` | `0x5e` |
| `ldr` at 0x4ec84 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4ec88 | load | 4 | `abs 0x4f080` | `-` |
| `ldr` at 0x4eca0 | load | 4 | `abs 0x4f03c` | `-` |
| `strb` at 0x4eca8 | store | 1 | `abs 0x14279c` | `0x35` |
| `ldr` at 0x4ecb8 | load | 4 | `abs 0x4f03c` | `-` |
| `ldrb` at 0x4ecbc | load | 1 | `abs 0x14279c` | `-` |
| `ldrb` at 0x4ecc8 | load | 1 | `sp -0x8e` | `-` |
| `strb` at 0x4eccc | store | 1 | `sp -0x8d` | `([sp-142]:1 & 0xff)` |
| `strb` at 0x4ecd4 | store | 1 | `sp -0x8e` | `?` |
| `ldrb` at 0x4ecd8 | load | 1 | `sp -0x8d` | `-` |
| `ldr` at 0x4ecf8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x4ecfc | store | 4 | `sp -0x8c` | `[sp-60]` |
| `ldr` at 0x4ed08 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4ed0c | load | 4 | `abs 0x4f084` | `-` |
| `ldr` at 0x4ed24 | load | 4 | `abs 0x4f03c` | `-` |
| `strb` at 0x4ed2c | store | 1 | `abs 0x14279c` | `0x4a` |
| `ldr` at 0x4ed3c | load | 4 | `abs 0x4f03c` | `-` |
| `ldrb` at 0x4ed40 | load | 1 | `abs 0x14279c` | `-` |
| `ldrb` at 0x4ed4c | load | 1 | `sp -0x8e` | `-` |
| `strb` at 0x4ed50 | store | 1 | `sp -0x8d` | `([sp-142]:1 & 0xff)` |
| `strb` at 0x4ed58 | store | 1 | `sp -0x8e` | `?` |
| `ldrb` at 0x4ed5c | load | 1 | `sp -0x8d` | `-` |
| `ldr` at 0x4ed7c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x4ed80 | store | 4 | `sp -0x8c` | `[sp-60]` |
| `ldr` at 0x4ed8c | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4ed90 | load | 4 | `abs 0x4f084` | `-` |
| `ldr` at 0x4eda8 | load | 4 | `abs 0x4f03c` | `-` |
| `strb` at 0x4edb0 | store | 1 | `abs 0x14279c` | `0x4a` |
| `ldr` at 0x4edc0 | load | 4 | `abs 0x4f03c` | `-` |
| `ldrb` at 0x4edc4 | load | 1 | `abs 0x14279c` | `-` |
| `ldrb` at 0x4edd0 | load | 1 | `sp -0x8e` | `-` |
| `strb` at 0x4edd4 | store | 1 | `sp -0x8d` | `([sp-142]:1 & 0xff)` |
| `ldrb` at 0x4edd8 | load | 1 | `sp -0x8d` | `-` |
| `ldr` at 0x4edf8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x4edfc | store | 4 | `sp -0x8c` | `[sp-60]` |
| `ldr` at 0x4ee08 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4ee0c | load | 4 | `abs 0x4f088` | `-` |
| `ldr` at 0x4ee24 | load | 4 | `abs 0x4f03c` | `-` |
| `strb` at 0x4ee2c | store | 1 | `abs 0x14279c` | `0x2b` |
| `ldr` at 0x4ee3c | load | 4 | `abs 0x4f03c` | `-` |
| `ldrb` at 0x4ee40 | load | 1 | `abs 0x14279c` | `-` |
| `ldr` at 0x4ee4c | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x4ee58 | load | 4 | `abs 0x4f02c` | `-` |
| `str` at 0x4ee60 | store | 4 | `abs 0x2ae4c0` | `0x2` |
| `strb` at 0x4ee70 | store | 1 | `sp -0x95` | `(ret(0x1f1dc) & 0xff)` |
| `ldrb` at 0x4ee74 | load | 1 | `sp -0x95` | `-` |
| `ldrls` at 0x4ee84 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4f000 | load | 4 | `abs 0x4f03c` | `-` |
| `ldrb` at 0x4f004 | load | 1 | `abs 0x14279c` | `-` |
| `ldr` at 0x4f014 | load | 4 | `abs 0x4f03c` | `-` |
| `ldrb` at 0x4f018 | load | 1 | `abs 0x14279c` | `-` |
| `ldr` at 0x4f01c | load | 4 | `abs 0x4f08c` | `-` |
| `ldmdb` at 0x4f028 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4f028 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4f028 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4e7cc | `(unknown callee) 0xc75ac` | `0x6` | `r1` | `r2` | `0x1` |
| `bl` at 0x4e800 | `(unknown callee) 0xd1ae8` | `0x0` | `0xff` | `sp-136` | `sp-136` |
| `bl` at 0x4e82c | `(unknown callee) 0x1b250` | `[sp-60]` | `0x10a624` | `0x9` | `[sp-60]` |
| `bl` at 0x4e85c | `(unknown callee) 0x1b250` | `[sp-140]` | `0x10a630` | `0xd` | `ret(0x1b250)` |
| `bl` at 0x4e884 | `(unknown callee) 0x1b250` | `[sp-52]` | `0x10a640` | `0xc` | `0x4c` |
| `bl` at 0x4e8b0 | `(unknown callee) 0x1b250` | `[sp-140]` | `0x10a650` | `0xd` | `ret(0x1b250)` |
| `bl` at 0x4e8c8 | `(unknown callee) 0xd1c14` | `sp-136` | `?` | `?` | `sp-136` |
| `bl` at 0x4e8dc | `(unknown callee) 0xd1ae8` | `0x0` | `0xfe` | `sp-136` | `sp-136` |
| `bl` at 0x4e908 | `(unknown callee) 0x1b250` | `[sp-60]` | `0x10a660` | `0xd` | `[sp-60]` |
| `bl` at 0x4e934 | `(unknown callee) 0x1b250` | `[sp-140]` | `0x10a670` | `0x4` | `ret(0x1b250)` |
| `bl` at 0x4e95c | `(unknown callee) 0x2070c` | `0x10a678` | `[sp-140]` | `?` | `ret(0x1b250)` |
| `bl` at 0x4e974 | `(unknown callee) 0xd1ae8` | `0x0` | `0xfd` | `sp-136` | `sp-136` |
| `bl` at 0x4e9a0 | `(unknown callee) 0x1b250` | `[sp-60]` | `0x10a660` | `0xd` | `[sp-60]` |
| `bl` at 0x4e9cc | `(unknown callee) 0x1b250` | `[sp-140]` | `0x10a670` | `0x4` | `ret(0x1b250)` |
| `bl` at 0x4e9f4 | `(unknown callee) 0x2070c` | `0x10a678` | `[sp-140]` | `?` | `ret(0x1b250)` |
| `bl` at 0x4ea18 | `(unknown callee) 0x1b250` | `[sp-140]` | `0x10a694` | `0x8` | `ret(0x1b250)` |
| `bl` at 0x4ea40 | `(unknown callee) 0x1b250` | `[sp-140]` | `0x10a6a0` | `0xd` | `ret(0x1b250)` |
| `bl` at 0x4ea60 | `(unknown callee) 0xd1c14` | `sp-136` | `?` | `?` | `sp-136` |
| `bl` at 0x4ea94 | `(unknown callee) 0xd1ae8` | `0x0` | `[sp-141]:1` | `sp-136` | `[sp-141]:1` |
| `bl` at 0x4eac8 | `(unknown callee) 0x1b250` | `[sp-140]` | `0x10a6b0` | `0x7` | `?` |
| `bl` at 0x4eaf4 | `(unknown callee) 0x1b250` | `[sp-140]` | `0x10a6b8` | `0xa` | `ret(0x1b250)` |
| `bl` at 0x4eb20 | `(unknown callee) 0x1b250` | `[sp-140]` | `0x10a6c4` | `0xd` | `ret(0x1b250)` |
| `bl` at 0x4eb4c | `(unknown callee) 0x1b250` | `[sp-140]` | `0x10a6d4` | `0xe` | `ret(0x1b250)` |
| `bl` at 0x4eb74 | `(unknown callee) 0x4f5a0` | `sp-149` | `0x1` | `0x10a6e4` | `sp-149` |
| `bl` at 0x4ebb4 | `(unknown callee) 0x1b250` | `[sp-140]` | `0x10a6ec` | `0x8` | `ret(0x1b250)` |
| `bl` at 0x4ebe0 | `(unknown callee) 0x1b250` | `[sp-140]` | `0x10a6f8` | `0x4` | `ret(0x1b250)` |
| `bl` at 0x4ec04 | `(unknown callee) 0xd1c14` | `sp-136` | `?` | `?` | `sp-136` |
| `bl` at 0x4ec38 | `(unknown callee) 0xd1ae8` | `0x0` | `[sp-141]:1` | `sp-136` | `[sp-141]:1` |
| `bl` at 0x4ec64 | `(unknown callee) 0x1b250` | `[sp-60]` | `0x10a700` | `0x7` | `[sp-60]` |
| `bl` at 0x4ec90 | `(unknown callee) 0x1b250` | `[sp-140]` | `0x10a708` | `0x4` | `ret(0x1b250)` |
| `bl` at 0x4ecb4 | `(unknown callee) 0xd1c14` | `sp-136` | `?` | `?` | `sp-136` |
| `bl` at 0x4ece8 | `(unknown callee) 0xd1ae8` | `0x0` | `[sp-141]:1` | `sp-136` | `[sp-141]:1` |
| `bl` at 0x4ed14 | `(unknown callee) 0x1b250` | `[sp-60]` | `0x10a710` | `0xa` | `[sp-60]` |
| `bl` at 0x4ed38 | `(unknown callee) 0xd1c14` | `sp-136` | `?` | `?` | `sp-136` |
| `bl` at 0x4ed6c | `(unknown callee) 0xd1ae8` | `0x0` | `[sp-141]:1` | `sp-136` | `[sp-141]:1` |
| `bl` at 0x4ed98 | `(unknown callee) 0x1b250` | `[sp-60]` | `0x10a710` | `0xa` | `[sp-60]` |
| `bl` at 0x4edbc | `(unknown callee) 0xd1c14` | `sp-136` | `?` | `?` | `sp-136` |
| `bl` at 0x4ede8 | `(unknown callee) 0xd1ae8` | `0x0` | `([sp-142]:1 & 0xff)` | `sp-136` | `([sp-142]:1 & 0xff)` |
| `bl` at 0x4ee14 | `(unknown callee) 0x1b250` | `[sp-60]` | `0x10a71c` | `0xc` | `[sp-60]` |
| `bl` at 0x4ee38 | `(unknown callee) 0xd1c14` | `sp-136` | `?` | `?` | `sp-136` |
| `bl` at 0x4ee68 | `(unknown callee) 0x1f1dc` | `0x9` | `?` | `0x2ae4c0` | `0x2` |
| `bl` at 0x4f010 | `(unknown callee) 0x1d38c` | `0x9` | `[0x14279c]:1` | `?` | `[0x14279c]:1` |
| `bl` at 0x4f024 | `(unknown callee) 0x1b910` | `0x2ae500` | `[0x14279c]:1` | `?` | `[0x14279c]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x4e80c | `ret(0xd1ae8) != 0x0` | `ret(0xd1ae8) == 0x0` |
| `beq` at 0x4e81c | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `bne` at 0x4e838 | `ret(0x1b250) != 0x0` | `ret(0x1b250) == 0x0` |
| `bne` at 0x4e868 | `ret(0x1b250) != 0x0` | `ret(0x1b250) == 0x0` |
| `bne` at 0x4e890 | `ret(0x1b250) != 0x0` | `ret(0x1b250) == 0x0` |
| `bne` at 0x4e8bc | `ret(0x1b250) != 0x0` | `ret(0x1b250) == 0x0` |
| `bne` at 0x4e8e8 | `ret(0xd1ae8) != 0x0` | `ret(0xd1ae8) == 0x0` |
| `beq` at 0x4e8f8 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `bne` at 0x4e914 | `ret(0x1b250) != 0x0` | `ret(0x1b250) == 0x0` |
| `bne` at 0x4e940 | `ret(0x1b250) != 0x0` | `ret(0x1b250) == 0x0` |
| `bne` at 0x4e980 | `ret(0xd1ae8) != 0x0` | `ret(0xd1ae8) == 0x0` |
| `beq` at 0x4e990 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `bne` at 0x4e9ac | `ret(0x1b250) != 0x0` | `ret(0x1b250) == 0x0` |
| `bne` at 0x4e9d8 | `ret(0x1b250) != 0x0` | `ret(0x1b250) == 0x0` |
| `bne` at 0x4ea24 | `ret(0x1b250) != 0x0` | `ret(0x1b250) == 0x0` |
| `bne` at 0x4ea4c | `ret(0x1b250) != 0x0` | `ret(0x1b250) == 0x0` |
| `bne` at 0x4ea70 | `[0x14279c]:1 != 0x3f` | `[0x14279c]:1 == 0x3f` |
| `bne` at 0x4eaa0 | `ret(0xd1ae8) != 0x0` | `ret(0xd1ae8) == 0x0` |
| `beq` at 0x4eab0 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `bne` at 0x4ead4 | `ret(0x1b250) != 0x0` | `ret(0x1b250) == 0x0` |
| `bne` at 0x4eb00 | `ret(0x1b250) != 0x0` | `ret(0x1b250) == 0x0` |
| `bne` at 0x4eb2c | `ret(0x1b250) != 0x0` | `ret(0x1b250) == 0x0` |
| `bne` at 0x4eb58 | `ret(0x1b250) != 0x0` | `ret(0x1b250) == 0x0` |
| `beq` at 0x4eb80 | `ret(0x4f5a0) == 0x0` | `ret(0x4f5a0) != 0x0` |
| `bne` at 0x4ebc0 | `ret(0x1b250) != 0x0` | `ret(0x1b250) == 0x0` |
| `bne` at 0x4ebec | `ret(0x1b250) != 0x0` | `ret(0x1b250) == 0x0` |
| `bne` at 0x4ec14 | `[0x14279c]:1 != 0x3f` | `[0x14279c]:1 == 0x3f` |
| `bne` at 0x4ec44 | `ret(0xd1ae8) != 0x0` | `ret(0xd1ae8) == 0x0` |
| `beq` at 0x4ec54 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `bne` at 0x4ec70 | `ret(0x1b250) != 0x0` | `ret(0x1b250) == 0x0` |
| `bne` at 0x4ec9c | `ret(0x1b250) != 0x0` | `ret(0x1b250) == 0x0` |
| `bne` at 0x4ecc4 | `[0x14279c]:1 != 0x3f` | `[0x14279c]:1 == 0x3f` |
| `bne` at 0x4ecf4 | `ret(0xd1ae8) != 0x0` | `ret(0xd1ae8) == 0x0` |
| `beq` at 0x4ed04 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `bne` at 0x4ed20 | `ret(0x1b250) != 0x0` | `ret(0x1b250) == 0x0` |
| `bne` at 0x4ed48 | `[0x14279c]:1 != 0x3f` | `[0x14279c]:1 == 0x3f` |
| `bne` at 0x4ed78 | `ret(0xd1ae8) != 0x0` | `ret(0xd1ae8) == 0x0` |
| `beq` at 0x4ed88 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `bne` at 0x4eda4 | `ret(0x1b250) != 0x0` | `ret(0x1b250) == 0x0` |
| `bne` at 0x4edcc | `[0x14279c]:1 != 0x3f` | `[0x14279c]:1 == 0x3f` |
| `bne` at 0x4edf4 | `ret(0xd1ae8) != 0x0` | `ret(0xd1ae8) == 0x0` |
| `beq` at 0x4ee04 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `bne` at 0x4ee20 | `ret(0x1b250) != 0x0` | `ret(0x1b250) == 0x0` |
| `bne` at 0x4ee48 | `[0x14279c]:1 != 0x3f` | `[0x14279c]:1 == 0x3f` |
| `beq` at 0x4ee54 | `[sp-148] == 0x0` | `[sp-148] != 0x0` |
| `ldrls` at 0x4ee84 | `? <= 0x59 (unsigned, 32-bit)` | `? > 0x59 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4f028 | `r0` | `ret(0x1b910)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x4ee84).

## `brickGetType`

- `brickGetType`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4f090 to 0x4f0b0.
  Established. The byte read addresses absolute `0x14279c` (`ldrb` at 0x4f0a0); its loaded
  expression `[0x14279c]:1` is the exit r0 value (`ldmdb` at 0x4f0a8). No call or conditional branch
  is reported. The physical meaning of the byte remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x4f09c | load | 4 | `abs 0x4f0ac` | `-` |
| `ldrb` at 0x4f0a0 | load | 1 | `abs 0x14279c` | `-` |
| `ldmdb` at 0x4f0a8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4f0a8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4f0a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4f0a8 | `r0` | `[0x14279c]:1` |

## `brickSetType`

- `brickSetType`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4f0b0 to 0x4f0dc.
  Established. The two byte stores carry `(r0 & 0xff)` at `sp -0x11` and `((r0 & 0xff) & 0xff)` at
  absolute `0x14279c` (`strb` at 0x4f0c4, `strb` at 0x4f0d0). Entry r0 remains the reported return
  register value (`ldmdb` at 0x4f0d4). The byte encoding has no established hardware interpretation.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4f0c4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x4f0c8 | load | 4 | `abs 0x4f0d8` | `-` |
| `ldrb` at 0x4f0cc | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x4f0d0 | store | 1 | `abs 0x14279c` | `((r0 & 0xff) & 0xff)` |
| `ldmdb` at 0x4f0d4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4f0d4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4f0d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4f0d4 | `r0` | `r0` |

## `brickGetSubType`

- `brickGetSubType`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4f0dc to 0x4f0fc.
  Established. Absolute `0x148248` supplies a byte (`ldrb` at 0x4f0ec). The exit reports that loaded
  expression as `[0x148248]:1` (`ldmdb` at 0x4f0f4). Calls and conditional branches are absent from
  the tool facts. Subtype semantics remain unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x4f0e8 | load | 4 | `abs 0x4f0f8` | `-` |
| `ldrb` at 0x4f0ec | load | 1 | `abs 0x148248` | `-` |
| `ldmdb` at 0x4f0f4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4f0f4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4f0f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4f0f4 | `r0` | `[0x148248]:1` |

## `brickSetSubType`

- `brickSetSubType`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4f0fc to 0x4f128.
  Established. A byte store first saves `(r0 & 0xff)` at `sp -0x11` (`strb` at 0x4f110). The later
  byte store writes `((r0 & 0xff) & 0xff)` at absolute `0x148248` (`strb` at 0x4f11c). The exit
  preserves entry r0 (`ldmdb` at 0x4f120). The stored value has unknown physical meaning.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4f110 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x4f114 | load | 4 | `abs 0x4f124` | `-` |
| `ldrb` at 0x4f118 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x4f11c | store | 1 | `abs 0x148248` | `((r0 & 0xff) & 0xff)` |
| `ldmdb` at 0x4f120 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4f120 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4f120 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4f120 | `r0` | `r0` |

## `brickGetFlashType`

- `brickGetFlashType`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4f128 to 0x4f148.
  Established. The sole explicit data-byte load reads absolute `0x148249` (`ldrb` at 0x4f138). The
  tool carries `[0x148249]:1` into r0 at the return (`ldmdb` at 0x4f140). Flash-part identification
  cannot be established from this byte alone.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x4f134 | load | 4 | `abs 0x4f144` | `-` |
| `ldrb` at 0x4f138 | load | 1 | `abs 0x148249` | `-` |
| `ldmdb` at 0x4f140 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4f140 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4f140 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4f140 | `r0` | `[0x148249]:1` |

## `brickSlotLabel`

- `brickSlotLabel`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4f564 to 0x4f5a0.
  Established. A byte store clears absolute `0x148250` (`strb` at 0x4f58c). The returned r0 is
  `0x148250` (`ldmdb` at 0x4f598). No calls or deciding branches appear in the facts. Label contents
  and ownership remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4f574 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4f578 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x4f57c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x4f580 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x4f584 | load | 4 | `abs 0x4f59c` | `-` |
| `strb` at 0x4f58c | store | 1 | `abs 0x148250` | `0x0` |
| `ldr` at 0x4f590 | load | 4 | `abs 0x4f59c` | `-` |
| `ldmdb` at 0x4f598 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4f598 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4f598 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4f598 | `r0` | `0x148250` |

## `displayReset`

- `displayReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x51db0 to 0x51fbc.
  Established. Two byte stores write `0x1` and `0x0` at absolute `0x148322` (`strb` at 0x51e44,
  `strb` at 0x51ee4). The call to `0xd6a1c` has `0x0`, `[0x1427a0]:1`, `0x70`, `0x0` (`bl` at
  0x51e18); its zero result takes the continuation while nonzero falls through (`beq` at 0x51e2c). A
  later call to the same target uses `[sp-21]:1` as the fourth candidate (`bl` at 0x51f08). Display
  signal polarity and the merged return are unresolved (`ldmdb` at 0x51f88).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x51dc0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x51dc4 | load | 4 | `abs 0x51f8c` | `-` |
| `ldrb` at 0x51dc8 | load | 1 | `abs 0x1482a0` | `-` |
| `ldr` at 0x51dd4 | load | 4 | `abs 0x51f90` | `-` |
| `str` at 0x51dd8 | store | 4 | `sp -0x20` | `0xfffff8f7` |
| `ldr` at 0x51de0 | load | 4 | `abs 0x51f94` | `-` |
| `ldrb` at 0x51de4 | load | 1 | `abs 0x1427a0` | `-` |
| `strb` at 0x51dfc | store | 1 | `sp -0x15` | `?` |
| `ldr` at 0x51e00 | load | 4 | `abs 0x51f94` | `-` |
| `ldrb` at 0x51e04 | load | 1 | `abs 0x1427a0` | `-` |
| `str` at 0x51e20 | store | 4 | `sp -0x1c` | `ret(0xd6a1c)` |
| `ldr` at 0x51e24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x51e30 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x51e34 | store | 4 | `sp -0x20` | `ret(0xd6a1c)` |
| `ldr` at 0x51e3c | load | 4 | `abs 0x51f98` | `-` |
| `strb` at 0x51e44 | store | 1 | `abs 0x148322` | `0x1` |
| `ldr` at 0x51e48 | load | 4 | `abs 0x51f9c` | `-` |
| `ldr` at 0x51e4c | load | 4 | `abs 0x51f94` | `-` |
| `ldrb` at 0x51e50 | load | 1 | `abs 0x1427a0` | `-` |
| `ldr` at 0x51e54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51e70 | load | 4 | `abs 0x1482b4` | `-` |
| `str` at 0x51e84 | store | 4 | `sp -0x1c` | `ret(unknown)` |
| `ldr` at 0x51e88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x51e94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x51ea0 | load | 4 | `abs 0x51fa0` | `-` |
| `str` at 0x51ea4 | store | 4 | `sp -0x28` | `0x10afc4` |
| `str` at 0x51ea8 | store | 4 | `sp -0x24` | `ret(0x219a8)` |
| `ldr` at 0x51eb0 | load | 4 | `abs 0x51fa4` | `-` |
| `ldr` at 0x51eb4 | load | 4 | `abs 0x51fa8` | `-` |
| `ldr` at 0x51ec0 | load | 4 | `abs 0x51f9c` | `-` |
| `ldr` at 0x51ec4 | load | 4 | `abs 0x1482c0` | `-` |
| `ldr` at 0x51ec8 | load | 4 | `abs 0x51fac` | `-` |
| `ldr` at 0x51ecc | load | 4 | `abs 0x51fb0` | `-` |
| `ldr` at 0x51edc | load | 4 | `abs 0x51f98` | `-` |
| `strb` at 0x51ee4 | store | 1 | `abs 0x148322` | `0x0` |
| `ldr` at 0x51eec | load | 4 | `abs 0x51f94` | `-` |
| `ldrb` at 0x51ef0 | load | 1 | `abs 0x1427a0` | `-` |
| `ldrb` at 0x51ef4 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x51f10 | store | 4 | `sp -0x1c` | `ret(0xd6a1c)` |
| `ldr` at 0x51f14 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x51f20 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x51f24 | store | 4 | `sp -0x20` | `ret(0xd6a1c)` |
| `ldr` at 0x51f2c | load | 4 | `abs 0x51fb4` | `-` |
| `ldrb` at 0x51f30 | load | 1 | `abs 0x1482c8` | `-` |
| `ldr` at 0x51f40 | load | 4 | `abs 0x51fb4` | `-` |
| `str` at 0x51f50 | store | 4 | `sp -0x1c` | `ret(0x51580)` |
| `ldr` at 0x51f54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x51f60 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x51f64 | store | 4 | `sp -0x20` | `ret(0x51580)` |
| `ldr` at 0x51f70 | load | 4 | `abs 0x51fb8` | `-` |
| `str` at 0x51f80 | store | 4 | `sp -0x20` | `ret(0x51580)` |
| `ldr` at 0x51f84 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x51f88 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x51f88 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x51f88 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x51df0 | `(unknown callee) 0xd6c2c` | `0x0` | `[0x1427a0]:1` | `r2` | `[0x1427a0]:1` |
| `bl` at 0x51e18 | `(unknown callee) 0xd6a1c` | `0x0` | `[0x1427a0]:1` | `0x70` | `0x0` |
| `mov` at 0x51e7c | `(unknown callee) (unknown target)` | `[0x1427a0]:1` | `?` | `[0x1482b4]` | `?` |
| `bl` at 0x51e98 | `(unknown callee) 0x219a8` | `ret(unknown)` | `?` | `?` | `ret(unknown)` |
| `bl` at 0x51ebc | `(unknown callee) 0x21e28` | `0x3` | `0x10af4c` | `0x23f` | `0x0` |
| `mov` at 0x51ed8 | `(unknown callee) (unknown target)` | `0x148323` | `0x10af2c` | `0x4` | `[0x1482c0]` |
| `bl` at 0x51ee8 | `(unknown callee) 0x51a6c` | `ret(unknown)` | `?` | `0x148322` | `0x0` |
| `bl` at 0x51f08 | `(unknown callee) 0xd6a1c` | `0x0` | `[0x1427a0]:1` | `0x70` | `[sp-21]:1` |
| `bl` at 0x51f48 | `(unknown callee) 0x51580` | `0x31` | `0x1482c8` | `0x0` | `[0x1482c8]:1` |
| `bl` at 0x51f78 | `(unknown callee) 0x51580` | `0x32` | `0x10b020` | `0x1` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x51dd0 | `[0x1482a0]:1 != 0x0` | `[0x1482a0]:1 == 0x0` |
| `beq` at 0x51e2c | `ret(0xd6a1c) == 0x0` | `ret(0xd6a1c) != 0x0` |
| `beq` at 0x51e90 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x51f1c | `ret(0xd6a1c) == 0x0` | `ret(0xd6a1c) != 0x0` |
| `beq` at 0x51f38 | `[0x1482c8]:1 == 0x0` | `[0x1482c8]:1 != 0x0` |
| `beq` at 0x51f5c | `ret(0x51580) == 0x0` | `ret(0x51580) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x51f88 | `r0` | `?` |

## `pcixBusInReset`

- `pcixBusInReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x58814 to 0x589c8.
  Established. The first test takes its continuation for `[0x148360] != 0x0` and falls through on
  zero (`bne` at 0x5883c). Calls include `0x5b754` with `[sp-20]`, `[sp-36]`, `sp-44`, `sp-44` (`bl`
  at 0x58908). That result takes the next continuation on zero and falls through on nonzero (`beq`
  at 0x5891c). Three later byte reads have unknown effective addresses (`ldrb` at 0x58938, `ldrb` at
  0x58958, `ldrb` at 0x58978). Physical reset-state bits and final r0 remain unknown (`ldmdb` at
  0x589b4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x58824 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x5882c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x58830 | load | 4 | `abs 0x589b8` | `-` |
| `ldr` at 0x58834 | load | 4 | `abs 0x148360` | `-` |
| `ldr` at 0x58840 | load | 4 | `abs 0x589bc` | `-` |
| `str` at 0x58844 | store | 4 | `sp -0x38` | `0xfffff703` |
| `ldr` at 0x5884c | load | 4 | `abs 0x589c0` | `-` |
| `ldr` at 0x58850 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58864 | load | 4 | `abs 0x14835c` | `-` |
| `str` at 0x5886c | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x58870 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x58874 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58884 | store | 4 | `sp -0x20` | `ret(0x5b130)` |
| `ldr` at 0x58888 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x58894 | load | 4 | `abs 0x589bc` | `-` |
| `str` at 0x58898 | store | 4 | `sp -0x38` | `0xfffff703` |
| `str` at 0x588a4 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x588a8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x588ac | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x588b0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x588c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x588c4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x588c8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x588cc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x588d8 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0x588e0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x588ec | load | 4 | `abs 0x589c4` | `-` |
| `str` at 0x588f0 | store | 4 | `sp -0x38` | `0xfffff702` |
| `ldr` at 0x588fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58900 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x58910 | store | 4 | `sp -0x30` | `ret(0x5b754)` |
| `ldr` at 0x58914 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x58920 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x58924 | store | 4 | `sp -0x38` | `ret(0x5b754)` |
| `ldrb` at 0x58938 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x58958 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x58978 | load | 1 | `(unknown)` | `-` |
| `str` at 0x58990 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x58998 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x589a0 | store | 4 | `sp -0x34` | `[sp-52]+1` |
| `str` at 0x589ac | store | 4 | `sp -0x38` | `0x1` |
| `ldr` at 0x589b0 | load | 4 | `sp -0x38` | `-` |
| `ldmdb` at 0x589b4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x589b4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x589b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5887c | `(unknown callee) 0x5b130` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x58908 | `(unknown callee) 0x5b754` | `[sp-20]` | `[sp-36]` | `sp-44` | `sp-44` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5883c | `[0x148360] != 0x0` | `[0x148360] == 0x0` |
| `bne` at 0x58890 | `ret(0x5b130) != 0x0` | `ret(0x5b130) == 0x0` |
| `blt` at 0x588b8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x588e8 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x5891c | `ret(0x5b754) == 0x0` | `ret(0x5b754) != 0x0` |
| `beq` at 0x58948 | `? == 0x3` | `? != 0x3` |
| `bne` at 0x58968 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x58988 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x589b4 | `r0` | `?` |

## `SMPConsoleReset`

- `SMPConsoleReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x68018 to 0x68070.
  Established. The call to `0x5e8d4` has unknown, `0x0`, unknown, unknown (`bl` at 0x68050). The
  deciding loop branch retains only a `le flags condition (operands unknown)` predicate and its
  negation for fall-through (`ble` at 0x68038). After the loop, `0x67d30` receives `0x0` and three
  unknown candidates (`bl` at 0x68068); its return expression reaches the exit (`ldmdb` at 0x6806c).
  Console-channel numbering remains unestablished.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6802c | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x68030 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68040 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68054 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6805c | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldmdb` at 0x6806c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6806c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6806c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x68050 | `(unknown callee) 0x5e8d4` | `?` | `0x0` | `?` | `?` |
| `bl` at 0x68068 | `(unknown callee) 0x67d30` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x68038 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6806c | `r0` | `ret(0x67d30)` |

## `L1CfgReset`

- `L1CfgReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6b1e0 to 0x6b2b0.
  Established. A word store clears absolute `0x1489b8` (`str` at 0x6b1f8). The opening call to
  `0x1b43c` uses `[0x1489c0]`, `0x0`, unknown, unknown (`bl` at 0x6b224). A later call to `0x70e7c`
  carries `0x1489c0`, `0x1489b8`, `0x1489bc`, `[[sp-24]+8]`, and its result reaches the exit (`bl`
  at 0x6b298, `ldmdb` at 0x6b29c). Record format and reset ownership remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x6b1f0 | load | 4 | `abs 0x6b2a0` | `-` |
| `str` at 0x6b1f8 | store | 4 | `abs 0x1489b8` | `0x0` |
| `ldr` at 0x6b1fc | load | 4 | `abs 0x6b2a0` | `-` |
| `ldr` at 0x6b200 | load | 4 | `abs 0x6b2a0` | `-` |
| `ldr` at 0x6b204 | load | 4 | `abs 0x1489bc` | `-` |
| `ldr` at 0x6b218 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6b228 | load | 4 | `abs 0x6b2a0` | `-` |
| `str` at 0x6b22c | store | 4 | `sp -0x18` | `0x14892c` |
| `ldr` at 0x6b230 | load | 4 | `abs 0x6b2a0` | `-` |
| `ldr` at 0x6b234 | load | 4 | `abs 0x148938` | `-` |
| `str` at 0x6b238 | store | 4 | `sp -0x30` | `[0x148938]` |
| `ldr` at 0x6b23c | load | 4 | `abs 0x6b2a0` | `-` |
| `ldr` at 0x6b240 | load | 4 | `abs 0x14893c` | `-` |
| `str` at 0x6b244 | store | 4 | `sp -0x2c` | `[0x14893c]` |
| `ldr` at 0x6b248 | load | 4 | `abs 0x6b2a0` | `-` |
| `ldr` at 0x6b24c | load | 4 | `abs 0x148984` | `-` |
| `str` at 0x6b25c | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x6b268 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x6b26c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6b270 | store | 4 | `sp -0x28` | `[sp-28]` |
| `str` at 0x6b278 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x6b280 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x6b284 | load | 4 | `abs 0x6b2a4` | `-` |
| `ldr` at 0x6b288 | load | 4 | `abs 0x6b2a8` | `-` |
| `ldr` at 0x6b28c | load | 4 | `abs 0x6b2ac` | `-` |
| `ldr` at 0x6b290 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6b294 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldmdb` at 0x6b29c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6b29c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6b29c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6b224 | `(unknown callee) 0x1b43c` | `[0x1489c0]` | `0x0` | `?` | `?` |
| `bl` at 0x6b298 | `(unknown callee) 0x70e7c` | `0x1489c0` | `0x1489b8` | `0x1489bc` | `[[sp-24]+8]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x6b254 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6b29c | `r0` | `ret(0x70e7c)` |

## `L1FRU_2m_Reset`

- `L1FRU_2m_Reset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa69b8 to 0xa6a1c.
  Established. Word stores clear absolute `0x2b1b00` and `0x2b1b04` (`str` at 0xa69e8, `str` at
  0xa6a10). Each zero test takes its bypass and falls through for nonzero (`beq` at 0xa69d0, `beq`
  at 0xa69f8). The first call to `0x1b400` carries `[0x2b1b00]`, entry r1, entry r2, `0x2b1ae0`
  (`bl` at 0xa69dc). Resource meanings and the common r0 value remain unresolved (`ldmdb` at
  0xa6a14).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xa69c4 | load | 4 | `abs 0xa6a18` | `-` |
| `ldr` at 0xa69c8 | load | 4 | `abs 0x2b1b00` | `-` |
| `ldr` at 0xa69d4 | load | 4 | `abs 0xa6a18` | `-` |
| `ldr` at 0xa69d8 | load | 4 | `abs 0x2b1b00` | `-` |
| `ldr` at 0xa69e0 | load | 4 | `abs 0xa6a18` | `-` |
| `str` at 0xa69e8 | store | 4 | `abs 0x2b1b00` | `0x0` |
| `ldr` at 0xa69ec | load | 4 | `abs 0xa6a18` | `-` |
| `ldr` at 0xa69f0 | load | 4 | `abs 0x2b1b04` | `-` |
| `ldr` at 0xa69fc | load | 4 | `abs 0xa6a18` | `-` |
| `ldr` at 0xa6a00 | load | 4 | `abs 0x2b1b04` | `-` |
| `ldr` at 0xa6a08 | load | 4 | `abs 0xa6a18` | `-` |
| `str` at 0xa6a10 | store | 4 | `abs 0x2b1b04` | `0x0` |
| `ldmdb` at 0xa6a14 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa6a14 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa6a14 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa69dc | `(unknown callee) 0x1b400` | `[0x2b1b00]` | `r1` | `r2` | `0x2b1ae0` |
| `bl` at 0xa6a04 | `(unknown callee) 0x1b400` | `[0x2b1b04]` | `?` | `0x2b1ae0` | `0x2b1ae0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa69d0 | `[0x2b1b00] == 0x0` | `[0x2b1b00] != 0x0` |
| `beq` at 0xa69f8 | `[0x2b1b04] == 0x0` | `[0x2b1b04] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa6a14 | `r0` | `?` |

## `PBaySdrWarmReset`

- `PBaySdrWarmReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xade0c to 0xadef0.
  Established. The byte store at `sp -0x10c` writes `([0x1499ff]:1 & 0xff)` (`strb` at 0xade60). A
  second byte store clears `sp -0x104` (`strb` at 0xade74). The call to `0xaa7dc` has entry r0,
  `sp-428`, `sp-260`, `sp-428` (`bl` at 0xade8c); its zero result takes the branch and nonzero falls
  through (`beq` at 0xadea0). The exit reports `[sp-440]` (`ldmdb` at 0xadedc). Message fields and
  actual warm-reset behavior remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xade1c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xade20 | load | 4 | `abs 0xadee0` | `-` |
| `ldr` at 0xade24 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xade28 | load | 2 | `r0 +0x6` | `-` |
| `strh` at 0xade30 | store | 2 | `sp -0x1ac` | `([r0+6]:2 & 0xffff)` |
| `str` at 0xade38 | store | 4 | `sp -0x1a8` | `0x20` |
| `str` at 0xade40 | store | 4 | `sp -0x20` | `0x3` |
| `str` at 0xade48 | store | 4 | `sp -0x1c` | `0x3` |
| `str` at 0xade50 | store | 4 | `sp -0x110` | `0x3` |
| `ldr` at 0xade54 | load | 4 | `abs 0xadee4` | `-` |
| `ldrb` at 0xade58 | load | 1 | `abs 0x1499ff` | `-` |
| `strb` at 0xade60 | store | 1 | `sp -0x10c` | `([0x1499ff]:1 & 0xff)` |
| `str` at 0xade68 | store | 4 | `sp -0x108` | `0x6` |
| `strb` at 0xade74 | store | 1 | `sp -0x104` | `0x0` |
| `str` at 0xade7c | store | 4 | `sp -0xc0` | `0x0` |
| `ldr` at 0xade84 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xade94 | store | 4 | `sp -0x1b0` | `ret(0xaa7dc)` |
| `ldr` at 0xade98 | load | 4 | `sp -0x1b0` | `-` |
| `ldr` at 0xadea4 | load | 4 | `abs 0xadee8` | `-` |
| `str` at 0xadea8 | store | 4 | `sp -0x1b4` | `0x127304` |
| `ldr` at 0xadeb0 | load | 4 | `abs 0xadeec` | `-` |
| `str` at 0xadeb4 | store | 4 | `sp -0x1b4` | `0x127308` |
| `str` at 0xadec4 | store | 4 | `sp -0x1b8` | `ret(0x1b124)` |
| `ldr` at 0xadec8 | load | 4 | `sp -0x1b8` | `-` |
| `ldr` at 0xadecc | load | 4 | `sp -0x1b4` | `-` |
| `ldr` at 0xaded4 | load | 4 | `sp -0x1b8` | `-` |
| `ldmdb` at 0xadedc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xadedc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xadedc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xade8c | `(unknown callee) 0xaa7dc` | `r0` | `sp-428` | `sp-260` | `sp-428` |
| `bl` at 0xadebc | `(unknown callee) 0x1b124` | `0xc` | `?` | `?` | `?` |
| `bl` at 0xaded0 | `(unknown callee) 0x1b508` | `ret(0x1b124)` | `[sp-436]` | `?` | `ret(0x1b124)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xadea0 | `ret(0xaa7dc) == 0x0` | `ret(0xaa7dc) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xadedc | `r0` | `[sp-440]` |

## `SysComReset`

- `SysComReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x826fc to 0x82878.
  Established. Calls to `0xba86c`, `0xca51c` and `0xba86c` each carry unknown, `0x3`, unknown,
  unknown (`bl` at 0x827e0, `bl` at 0x8281c, `bl` at 0x8285c). The loop branch has an unresolved `lt
  flags condition (operands unknown)` predicate (`blt` at 0x82724). Later zero tests retain unknown
  operands (`bne` at 0x82764, `beq` at 0x827a0). Channel selection and r0 at the exit are not
  established (`ldmdb` at 0x82870).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x82710 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x82714 | load | 4 | `abs 0x82874` | `-` |
| `ldr` at 0x82718 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8271c | load | 4 | `abs 0x2b1730` | `-` |
| `ldr` at 0x8272c | load | 4 | `abs 0x82874` | `-` |
| `ldr` at 0x82730 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x82754 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x8275c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x82768 | load | 4 | `abs 0x82874` | `-` |
| `ldr` at 0x8276c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x82790 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x82798 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x827a4 | load | 4 | `abs 0x82874` | `-` |
| `ldr` at 0x827a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x827cc | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x827e4 | load | 4 | `abs 0x82874` | `-` |
| `ldr` at 0x827e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8280c | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x82814 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x82820 | load | 4 | `abs 0x82874` | `-` |
| `ldr` at 0x82824 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x82848 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x82860 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x82868 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldmdb` at 0x82870 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x82870 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x82870 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x827e0 | `(unknown callee) 0xba86c` | `?` | `0x3` | `?` | `?` |
| `bl` at 0x8281c | `(unknown callee) 0xca51c` | `?` | `0x3` | `?` | `?` |
| `bl` at 0x8285c | `(unknown callee) 0xba86c` | `?` | `0x3` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x82724 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x82764 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x827a0 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x82870 | `r0` | `?` |
