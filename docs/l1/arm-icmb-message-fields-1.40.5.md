# ARM ICMB message fields and builders in 1.40.5

Read from `6.5.29/usr/cpu/firmware/sysco/l1.bin`, package `6.5.29:l1#4` (version 1.40.5),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x53d448`.
Carrier SHA-256: `80f4a616601d187bc3839a2758f2e7f2d0db962ad7fd8f2d96d41bcd15bdf88c`.
Package SHA-256: `4ccd1a6e7054bc3cd56e962a60737d1c90339ffa93fcc8af0912b6c9c7d385dd`.
Inner executable SHA-256: `08ebc648bb298017c7e3d6049983f2a54a8fe56f1bb5738df88b82c38f75ac3a`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1257088 bytes.
Segment SHA-256: `dc7cc5d18fbdf816d328411825c9c01e26c3f8852f1012c7a642aea0767e2665`. Analysed routine spans use the exact
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
No unresolved computed transfers are reported. Unknown data values and instruction effects
remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/icmb-message-fields-facts.txt) retain all tool sections.

## `IcmbMsgRqRs`

- `IcmbMsgRqRs`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xab0d4 to 0xab564.
  Established. The word at `r1 +0x9c` is read (`ldr` at 0xab0f8). Equality with `0xff` takes
  `0xab158`, while inequality falls through (`beq` at 0xab100). The unnamed `0xaeab4` call carries
  `[r1+160]:1`, `[r1+168]:1`, `[r1+164]`, `[r1+156]` (`bl` at 0xab150). Message transport effects
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xab0e4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xab0e8 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xab0f0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xab0f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab0f8 | load | 4 | `r1 +0x9c` | `-` |
| `ldr` at 0xab104 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab108 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xab10c | load | 1 | `r1 +0xa0` | `-` |
| `ldr` at 0xab110 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xab114 | load | 1 | `r1 +0xa8` | `-` |
| `ldr` at 0xab118 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab11c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab120 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xab128 | store | 4 | `sp -0x1b4` | `r1+169` |
| `ldr` at 0xab12c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab130 | load | 4 | `r1 +0xec` | `-` |
| `str` at 0xab134 | store | 4 | `sp -0x1b0` | `[r1+236]` |
| `ldr` at 0xab138 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xab140 | store | 4 | `sp -0x1ac` | `r1+8` |
| `ldr` at 0xab148 | load | 4 | `r1 +0xa4` | `-` |
| `ldr` at 0xab14c | load | 4 | `r1 +0x9c` | `-` |
| `str` at 0xab154 | store | 4 | `r1 +0x98` | `ret(0xaeab4)` |
| `strb` at 0xab16c | store | 1 | `(unknown)` | `(ret(0xab9a4) & 0xff)` |
| `ldrb` at 0xab178 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xab17c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab180 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab188 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xab190 | store | 4 | `sp -0x1b4` | `sp-184` |
| `ldr` at 0xab198 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xab19c | load | 4 | `[sp-28] +0x98` | `-` |
| `str` at 0xab1a8 | store | 4 | `sp -0xbc` | `ret(0xaf1f8)` |
| `ldr` at 0xab1ac | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xab1b0 | load | 2 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xab1b4 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xab1b8 | load | 2 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0xab1c0 | load | 1 | `sp -0xbc` | `-` |
| `str` at 0xab1c8 | store | 4 | `sp -0x1b4` | `sp-340` |
| `str` at 0xab1e4 | store | 4 | `sp -0x158` | `ret(0xaedc0)` |
| `str` at 0xab1ec | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xab1f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab1f4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xab1f8 | load | 4 | `[sp-28] +0x18c` | `-` |
| `ldr` at 0xab204 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xab210 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xab224 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab228 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xab234 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab23c | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xab24c | store | 4 | `sp -0x19c` | `0x3` |
| `ldr` at 0xab250 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab258 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xab268 | store | 4 | `sp -0x170` | `0x0` |
| `ldr` at 0xab26c | load | 4 | `sp -0x170` | `-` |
| `ldr` at 0xab27c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab284 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xab28c | load | 4 | `sp -0x158` | `-` |
| `str` at 0xab298 | store | 4 | `sp -0x16c` | `ret(0xcad54)` |
| `ldr` at 0xab29c | load | 4 | `sp -0x16c` | `-` |
| `ldr` at 0xab2a0 | load | 4 | `abs 0xab558` | `-` |
| `ldr` at 0xab2b4 | load | 4 | `sp -0x170` | `-` |
| `str` at 0xab2bc | store | 4 | `sp -0x170` | `[sp-368]+1` |
| `ldr` at 0xab2c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab2c8 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xab2d8 | store | 4 | `sp -0x19c` | `0x0` |
| `ldr` at 0xab2dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab2e4 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xab2f0 | load | 4 | `sp -0x170` | `-` |
| `str` at 0xab304 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xab308 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab30c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xab310 | load | 4 | `[sp-28] +0x190` | `-` |
| `ldr` at 0xab31c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xab328 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab32c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab334 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xab348 | store | 4 | `sp -0x168` | `ret(0xcab94)` |
| `ldr` at 0xab34c | load | 4 | `sp -0x168` | `-` |
| `ldr` at 0xab350 | load | 4 | `abs 0xab55c` | `-` |
| `ldr` at 0xab360 | load | 4 | `sp -0x168` | `-` |
| `ldr` at 0xab370 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab378 | load | 4 | `abs 0xab560` | `-` |
| `ldrb` at 0xab380 | load | 1 | `sp -0x168` | `-` |
| `str` at 0xab3a8 | store | 4 | `sp -0x164` | `ret(0xaee98)` |
| `ldr` at 0xab3ac | load | 4 | `sp -0x164` | `-` |
| `ldr` at 0xab3bc | load | 4 | `sp -0x160` | `-` |
| `str` at 0xab3c8 | store | 4 | `sp -0x178` | `ret(0xaf000)` |
| `ldr` at 0xab3cc | load | 4 | `sp -0x178` | `-` |
| `ldr` at 0xab3e0 | load | 4 | `sp -0x160` | `-` |
| `strb` at 0xab3f0 | store | 1 | `(unknown)` | `(ret(0xaf08c) & 0xff)` |
| `ldr` at 0xab3f4 | load | 4 | `sp -0x160` | `-` |
| `str` at 0xab400 | store | 4 | `sp -0x180` | `ret(0xaf108)` |
| `ldr` at 0xab404 | load | 4 | `sp -0x160` | `-` |
| `str` at 0xab410 | store | 4 | `sp -0x184` | `ret(0xaf16c)` |
| `ldr` at 0xab414 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab418 | load | 4 | `sp -0x184` | `-` |
| `str` at 0xab41c | store | 4 | `[sp-28] +0x194` | `ret(0xaf16c)` |
| `ldr` at 0xab420 | load | 4 | `sp -0x184` | `-` |
| `ldr` at 0xab42c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab430 | load | 4 | `sp -0x180` | `-` |
| `ldr` at 0xab434 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0xab44c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xab454 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xab460 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab464 | load | 4 | `sp -0x160` | `-` |
| `str` at 0xab46c | store | 4 | `[sp-28] +0x188` | `ret(0xaf1a0)` |
| `ldr` at 0xab470 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab474 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xab480 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab484 | load | 4 | `[sp-28] +0x188` | `-` |
| `str` at 0xab490 | store | 4 | `sp -0x184` | `ret(0xaeca0)` |
| `ldr` at 0xab494 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab498 | load | 4 | `[sp-28] +0x188` | `-` |
| `str` at 0xab4a4 | store | 4 | `sp -0x180` | `ret(0xaec6c)` |
| `ldr` at 0xab4a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab4ac | load | 4 | `[sp-28] +0x188` | `-` |
| `str` at 0xab4b8 | store | 4 | `sp -0x188` | `ret(0xaec2c)` |
| `ldr` at 0xab4bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab4c0 | load | 4 | `[sp-28] +0x188` | `-` |
| `str` at 0xab4cc | store | 4 | `sp -0x18c` | `ret(0xaebec)` |
| `ldr` at 0xab4d0 | load | 4 | `sp -0x184` | `-` |
| `ldr` at 0xab4dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab4e0 | load | 4 | `sp -0x180` | `-` |
| `ldr` at 0xab4e4 | load | 4 | `[sp-28] +0x9c` | `-` |
| `ldr` at 0xab4f0 | load | 4 | `sp -0x188` | `-` |
| `ldr` at 0xab4fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab500 | load | 4 | `sp -0x18c` | `-` |
| `ldr` at 0xab504 | load | 4 | `[sp-28] +0xa4` | `-` |
| `str` at 0xab514 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xab518 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xab520 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xab528 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xab530 | store | 4 | `sp -0x24` | `?` |
| `ldrb` at 0xab540 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xab54c | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xab554 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xab554 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xab554 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xab554 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xab150 | `(unknown callee) 0xaeab4` | `[r1+160]:1` | `[r1+168]:1` | `[r1+164]` | `[r1+156]` |
| `bl` at 0xab15c | `(unknown callee) 0xab9a4` | `0x20` | `?` | `?` | `?` |
| `bl` at 0xab1a0 | `(unknown callee) 0xaf1f8` | `?` | `[[sp-28]+4]` | `[sp-28]+8` | `[[sp-28]+152]` |
| `bl` at 0xab1dc | `(unknown callee) 0xaedc0` | `[[sp-24]+4]:2` | `[[sp-28]]:2` | `sp-184` | `[sp-188]:1` |
| `bl` at 0xab220 | `(unknown callee) 0xac140` | `0x14` | `?` | `?` | `?` |
| `bl` at 0xab244 | `(unknown callee) 0xcade4` | `[[sp-24]+12]` | `sp-424` | `sp-424` | `[sp-24]` |
| `bl` at 0xab260 | `(unknown callee) 0xcae20` | `[[sp-24]+12]` | `sp-424` | `sp-424` | `[sp-24]` |
| `bl` at 0xab290 | `(unknown callee) 0xcad54` | `[[sp-24]+12]` | `sp-340` | `?` | `[sp-24]` |
| `bl` at 0xab2b0 | `(unknown callee) 0xac140` | `0x1` | `?` | `ret(0xcad54)` | `0xfffff698` |
| `bl` at 0xab2ec | `(unknown callee) 0xcae20` | `[[sp-24]+12]` | `sp-424` | `sp-424` | `[sp-24]` |
| `bl` at 0xab340 | `(unknown callee) 0xcab94` | `[[sp-24]+12]` | `[sp-28]+240` | `0x98` | `[sp-28]+240` |
| `bl` at 0xab3a0 | `(unknown callee) 0xaee98` | `[sp-28]+240` | `[sp-360]:1` | `sp-370` | `sp-352` |
| `bl` at 0xab3c0 | `(unknown callee) 0xaf000` | `[sp-352]` | `?` | `?` | `ret(0xaee98)` |
| `bl` at 0xab3e4 | `(unknown callee) 0xaf08c` | `[sp-352]` | `?` | `?` | `ret(0xaf000)` |
| `bl` at 0xab3f8 | `(unknown callee) 0xaf108` | `[sp-352]` | `sp-20` | `?` | `ret(0xaf08c)` |
| `bl` at 0xab408 | `(unknown callee) 0xaf16c` | `[sp-352]` | `?` | `?` | `ret(0xaf108)` |
| `bl` at 0xab468 | `(unknown callee) 0xaf1a0` | `[sp-352]` | `?` | `?` | `?` |
| `bl` at 0xab488 | `(unknown callee) 0xaeca0` | `[[sp-28]+392]` | `?` | `?` | `[sp-28]` |
| `bl` at 0xab49c | `(unknown callee) 0xaec6c` | `[[sp-28]+392]` | `?` | `?` | `[sp-28]` |
| `bl` at 0xab4b0 | `(unknown callee) 0xaec2c` | `[[sp-28]+392]` | `?` | `?` | `[sp-28]` |
| `bl` at 0xab4c4 | `(unknown callee) 0xaebec` | `[[sp-28]+392]` | `?` | `?` | `[sp-28]` |
| `bl` at 0xab548 | `(unknown callee) 0xab9a4` | `?` | `sp-20` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xab100 | `[r1+156] == 0xff` | `[r1+156] != 0xff` |
| `bge` at 0xab200 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0xab20c | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xab218 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xab230 | `[[sp-28]+4] != 0xb` | `[[sp-28]+4] == 0xb` |
| `ble` at 0xab274 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xab2a8 | `ret(0xcad54) != 0xfffff698` | `ret(0xcad54) == 0xfffff698` |
| `bne` at 0xab2d0 | `[[sp-28]+4] != 0xb` | `[[sp-28]+4] == 0xb` |
| `bne` at 0xab2f8 | `Z == 0` | `not (Z == 0)` |
| `bge` at 0xab318 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0xab324 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0xab358 | `ret(0xcab94) != 0xfffff69e` | `ret(0xcab94) == 0xfffff69e` |
| `bgt` at 0xab368 | `ret(0xcab94) > 0x0 (signed, 32-bit)` | `ret(0xcab94) <= 0x0 (signed, 32-bit)` |
| `bne` at 0xab3b4 | `ret(0xaee98) != 0x0` | `ret(0xaee98) == 0x0` |
| `beq` at 0xab3d4 | `ret(0xaf000) == 0x1` | `ret(0xaf000) != 0x1` |
| `bne` at 0xab428 | `[sp-388] != 0x0` | `[sp-388] == 0x0` |
| `bne` at 0xab43c | `[sp-384] != [[sp-28]+4]` | `[sp-384] == [[sp-28]+4]` |
| `bne` at 0xab45c | `? != ?` | `? == ?` |
| `bne` at 0xab47c | `[[sp-28]+4] != 0x20` | `[[sp-28]+4] == 0x20` |
| `bne` at 0xab4d8 | `[sp-388] != 0x0` | `[sp-388] == 0x0` |
| `bne` at 0xab4ec | `[sp-384] != [[sp-28]+156]` | `[sp-384] == [[sp-28]+156]` |
| `bne` at 0xab4f8 | `[sp-392] != 0x1` | `[sp-392] == 0x1` |
| `bne` at 0xab50c | `ret(0xaebec) != [[sp-28]+164]` | `ret(0xaebec) == [[sp-28]+164]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xab554 | `r0` | `[sp-32]` |

## `IcmbIpmbRqBuild`

- `IcmbIpmbRqBuild`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaeab4 to 0xaeb44.
  Established. The byte assignment at `[sp+8] +0x0` writes `((r0 & 0xff) & 0xff)` (`strb` at
  0xaeae0). The unnamed `0x1b394` call carries `[sp+8]+3`, `[sp]`, `[sp+4]`, `[sp+8]+3` (`bl` at
  0xaeb30). The return reports `[sp+4]+3` (`ldmdb` at 0xaeb40). Request field interpretation remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaeac4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xaeac8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xaeacc | store | 4 | `sp -0x20` | `r3` |
| `strb` at 0xaead4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xaead8 | load | 4 | `sp +0x8` | `-` |
| `ldrb` at 0xaeadc | load | 1 | `sp -0x11` | `-` |
| `strb` at 0xaeae0 | store | 1 | `[sp+8] +0x0` | `((r0 & 0xff) & 0xff)` |
| `ldr` at 0xaeae8 | load | 4 | `sp +0x8` | `-` |
| `ldrb` at 0xaeaf0 | load | 1 | `sp -0x1c` | `-` |
| `ldrb` at 0xaeafc | load | 1 | `sp -0x18` | `-` |
| `strb` at 0xaeb04 | store | 1 | `[sp+8] +0x1` | `?` |
| `ldr` at 0xaeb0c | load | 4 | `sp +0x8` | `-` |
| `ldrb` at 0xaeb14 | load | 1 | `sp -0x20` | `-` |
| `strb` at 0xaeb18 | store | 1 | `[sp+8] +0x2` | `([sp-32]:1 & 0xff)` |
| `ldr` at 0xaeb1c | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xaeb28 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xaeb2c | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xaeb34 | load | 4 | `sp +0x4` | `-` |
| `ldmdb` at 0xaeb40 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaeb40 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaeb40 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaeb30 | `(unknown callee) 0x1b394` | `[sp+8]+3` | `[sp]` | `[sp+4]` | `[sp+8]+3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaeb40 | `r0` | `[sp+4]+3` |

## `IcmbIpmbRsBuild`

- `IcmbIpmbRsBuild`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaeb44 to 0xaebec.
  Established. The byte at `[sp+12] +0x3` receives `([sp]:1 & 0xff)` (`strb` at 0xaebc0). The
  unnamed `0x1b394` call uses `[sp+12]+4`, `[sp+4]`, `[sp+8]`, `[sp+12]+4` (`bl` at 0xaebd8). The
  return reports `[sp+8]+4` (`ldmdb` at 0xaebe8). Response field semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaeb54 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xaeb58 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xaeb5c | store | 4 | `sp -0x20` | `r3` |
| `strb` at 0xaeb64 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xaeb68 | load | 4 | `sp +0xc` | `-` |
| `ldrb` at 0xaeb6c | load | 1 | `sp -0x11` | `-` |
| `strb` at 0xaeb70 | store | 1 | `[sp+12] +0x0` | `((r0 & 0xff) & 0xff)` |
| `ldr` at 0xaeb78 | load | 4 | `sp +0xc` | `-` |
| `ldrb` at 0xaeb80 | load | 1 | `sp -0x1c` | `-` |
| `ldrb` at 0xaeb8c | load | 1 | `sp -0x18` | `-` |
| `strb` at 0xaeb98 | store | 1 | `[sp+12] +0x1` | `?` |
| `ldr` at 0xaeba0 | load | 4 | `sp +0xc` | `-` |
| `ldrb` at 0xaeba8 | load | 1 | `sp -0x20` | `-` |
| `strb` at 0xaebac | store | 1 | `[sp+12] +0x2` | `([sp-32]:1 & 0xff)` |
| `ldr` at 0xaebb4 | load | 4 | `sp +0xc` | `-` |
| `ldrb` at 0xaebbc | load | 1 | `sp +0x0` | `-` |
| `strb` at 0xaebc0 | store | 1 | `[sp+12] +0x3` | `([sp]:1 & 0xff)` |
| `ldr` at 0xaebc4 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xaebd0 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xaebd4 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xaebdc | load | 4 | `sp +0x8` | `-` |
| `ldmdb` at 0xaebe8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaebe8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaebe8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaebd8 | `(unknown callee) 0x1b394` | `[sp+12]+4` | `[sp+4]` | `[sp+8]` | `[sp+12]+4` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaebe8 | `r0` | `[sp+8]+4` |

## `IcmbIpmbGetNetFn`

- `IcmbIpmbGetNetFn`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaebec to 0xaec2c.
  Established. The byte read uses `r0 +0x1` (`ldrb` at 0xaec0c). A word assignment at `sp -0x18` has
  an unknown value (`str` at 0xaec1c). The reported return value is `[sp-24]` (`ldmdb` at 0xaec28).
  Network-function decoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaebfc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xaec04 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xaec0c | load | 1 | `r0 +0x1` | `-` |
| `str` at 0xaec1c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xaec20 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xaec28 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaec28 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaec28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaec28 | `r0` | `[sp-24]` |

## `IcmbIpmbGetRqRsId`

- `IcmbIpmbGetRqRsId`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaec2c to 0xaec6c.
  Established. A byte is loaded from `r0 +0x1` (`ldrb` at 0xaec4c). The resulting word assignment at
  `sp -0x18` is unknown (`str` at 0xaec5c). The return exposes `[sp-24]` (`ldmdb` at 0xaec68).
  Request/response identifier encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaec3c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xaec44 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xaec4c | load | 1 | `r0 +0x1` | `-` |
| `str` at 0xaec5c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xaec60 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xaec68 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaec68 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaec68 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaec68 | `r0` | `[sp-24]` |

## `IcmbIpmbGetCmd`

- `IcmbIpmbGetCmd`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaec6c to 0xaeca0.
  Established. The byte read at `r0 +0x2` supplies `[r0+2]:1` to the word at `sp -0x18` (`ldrb` at
  0xaec8c, `str` at 0xaec90). That same byte expression is the reported return value (`ldmdb` at
  0xaec9c). Command semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaec7c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xaec84 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xaec8c | load | 1 | `r0 +0x2` | `-` |
| `str` at 0xaec90 | store | 4 | `sp -0x18` | `[r0+2]:1` |
| `ldr` at 0xaec94 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xaec9c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaec9c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaec9c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaec9c | `r0` | `[r0+2]:1` |

## `IcmbIpmbGetCmpCode`

- `IcmbIpmbGetCmpCode`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaeca0 to 0xaecd4.
  Established. The byte at `r0 +0x3` is loaded (`ldrb` at 0xaecc0); its expression `[r0+3]:1` is
  written as a word at `sp -0x18` (`str` at 0xaecc4). The return reports `[r0+3]:1` (`ldmdb` at
  0xaecd0). Completion-code meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaecb0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xaecb8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xaecc0 | load | 1 | `r0 +0x3` | `-` |
| `str` at 0xaecc4 | store | 4 | `sp -0x18` | `[r0+3]:1` |
| `ldr` at 0xaecc8 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xaecd0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaecd0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaecd0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaecd0 | `r0` | `[r0+3]:1` |

## `IcmbIpmbGetData`

- `IcmbIpmbGetData`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaecd4 to 0xaed2c.
  Established. The unnamed `0xaec2c` call carries `r0`, `r1`, `r2`, `r3` (`bl` at 0xaecec); its
  nonzero result takes `0xaed14`, while zero falls through (`bne` at 0xaed00). The two word
  assignments at `sp -0x1c` write `[sp-20]+3` and `[sp-20]+4` (`str` at 0xaed0c, `str` at 0xaed1c).
  Data contents remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaece4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xaece8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaecf4 | store | 4 | `sp -0x18` | `ret(0xaec2c)` |
| `ldr` at 0xaecf8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaed04 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaed0c | store | 4 | `sp -0x1c` | `[sp-20]+3` |
| `ldr` at 0xaed14 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaed1c | store | 4 | `sp -0x1c` | `[sp-20]+4` |
| `ldr` at 0xaed20 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xaed28 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaed28 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaed28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaecec | `(unknown callee) 0xaec2c` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xaed00 | `ret(0xaec2c) != 0x0` | `ret(0xaec2c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaed28 | `r0` | `[sp-28]` |

## `IcmbCvtToU16`

- `IcmbCvtToU16`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaef64 to 0xaefbc.
  Established. Byte reads use `r0 +0x1` and `r0 +0x0` (`ldrb` at 0xaef84, `ldrb` at 0xaef9c). The
  first halfword assignment at `sp -0x16` writes `([r0+1]:1 & 0xffff)` (`strh` at 0xaef88). The
  return reports `[sp-22]:2` (`ldmdb` at 0xaefb8). The final conversion value remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaef74 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xaef7c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xaef84 | load | 1 | `r0 +0x1` | `-` |
| `strh` at 0xaef88 | store | 2 | `sp -0x16` | `([r0+1]:1 & 0xffff)` |
| `ldrh` at 0xaef8c | load | 2 | `sp -0x16` | `-` |
| `strh` at 0xaef94 | store | 2 | `sp -0x16` | `?` |
| `ldr` at 0xaef98 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xaef9c | load | 1 | `r0 +0x0` | `-` |
| `ldrh` at 0xaefa4 | load | 2 | `sp -0x16` | `-` |
| `strh` at 0xaefac | store | 2 | `sp -0x16` | `?` |
| `ldrh` at 0xaefb0 | load | 2 | `sp -0x16` | `-` |
| `ldmdb` at 0xaefb8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaefb8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaefb8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaefb8 | `r0` | `[sp-22]:2` |

## `IcmbCvtToMsg`

- `IcmbCvtToMsg`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaefbc to 0xaf000.
  Established. A halfword at `sp -0x16` receives `(r1 & 0xffff)` (`strh` at 0xaefd4). The byte
  assignment through `r0 +0x0` writes `([sp-22]:1 & 0xff)` (`strb` at 0xaefe0). The byte store at
  `[sp-20] +0x1` has an unknown value (`strb` at 0xaeff8). Message representation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaefcc | store | 4 | `sp -0x14` | `r0` |
| `strh` at 0xaefd4 | store | 2 | `sp -0x16` | `(r1 & 0xffff)` |
| `ldr` at 0xaefd8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xaefdc | load | 1 | `sp -0x16` | `-` |
| `strb` at 0xaefe0 | store | 1 | `r0 +0x0` | `([sp-22]:1 & 0xff)` |
| `ldr` at 0xaefe8 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xaeff0 | load | 2 | `sp -0x16` | `-` |
| `strb` at 0xaeff8 | store | 1 | `[sp-20] +0x1` | `?` |
| `ldmdb` at 0xaeffc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaeffc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaeffc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaeffc | `r0` | `r0` |

## `IcmbMsgGetMsgType`

- `IcmbMsgGetMsgType`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaf000 to 0xaf038.
  Established. The byte load reads through `r0 +0x0` (`ldrb` at 0xaf018). The word assignment at
  `sp -0x18` has an unknown value (`str` at 0xaf028). The reported return is `[sp-24]` (`ldmdb` at
  0xaf034). Message-type encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaf010 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xaf014 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xaf018 | load | 1 | `r0 +0x0` | `-` |
| `str` at 0xaf028 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xaf02c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xaf034 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaf034 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaf034 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaf034 | `r0` | `[sp-24]` |

## `IcmbMsgGetSeqNbr`

- `IcmbMsgGetSeqNbr`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaf08c to 0xaf0bc.
  Established. The byte read uses `r0 +0x0` (`ldrb` at 0xaf0a4). The assignment at `sp -0x15` has an
  unknown byte value (`strb` at 0xaf0ac). The return reports `[sp-21]:1` (`ldmdb` at 0xaf0b8).
  Sequence-number extraction remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaf09c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xaf0a0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xaf0a4 | load | 1 | `r0 +0x0` | `-` |
| `strb` at 0xaf0ac | store | 1 | `sp -0x15` | `?` |
| `ldrb` at 0xaf0b0 | load | 1 | `sp -0x15` | `-` |
| `ldmdb` at 0xaf0b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaf0b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaf0b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaf0b8 | `r0` | `[sp-21]:1` |

## `IcmbMsgGetCmd`

- `IcmbMsgGetCmd`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaf108 to 0xaf13c.
  Established. The byte at `r0 +0x1` is read and its expression `[r0+1]:1` is assigned as a word at
  `sp -0x18` (`ldrb` at 0xaf128, `str` at 0xaf12c). The return reports `[r0+1]:1` (`ldmdb` at
  0xaf138). Message command meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaf118 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xaf120 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xaf128 | load | 1 | `r0 +0x1` | `-` |
| `str` at 0xaf12c | store | 4 | `sp -0x18` | `[r0+1]:1` |
| `ldr` at 0xaf130 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xaf138 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaf138 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaf138 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaf138 | `r0` | `[r0+1]:1` |

## `IcmbMsgGetCmpCode`

- `IcmbMsgGetCmpCode`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaf16c to 0xaf1a0.
  Established. The read through `r0 +0x2` is one byte wide (`ldrb` at 0xaf18c); its expression
  `[r0+2]:1` is stored as a word at `sp -0x18` (`str` at 0xaf190). The return exposes `[r0+2]:1`
  (`ldmdb` at 0xaf19c). Message completion semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaf17c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xaf184 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xaf18c | load | 1 | `r0 +0x2` | `-` |
| `str` at 0xaf190 | store | 4 | `sp -0x18` | `[r0+2]:1` |
| `ldr` at 0xaf194 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xaf19c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaf19c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaf19c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaf19c | `r0` | `[r0+2]:1` |

## `IcmbMsgRqBuild`

- `IcmbMsgRqBuild`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaf1f8 to 0xaf274.
  Established. The word assignment at `sp -0x24` writes `r3+2` (`str` at 0xaf224). The unnamed
  `0xaf038` call carries `[sp]`, `0x0`, `r2`, `r3+2` (`bl` at 0xaf230). The unnamed `0x1b394` call
  carries `[sp]+2`, `[sp-28]`, `[sp-32]`, `[sp]+2` (`bl` at 0xaf264). Request payload meanings
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaf208 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xaf20c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xaf210 | store | 4 | `sp -0x20` | `r3` |
| `strb` at 0xaf218 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xaf21c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xaf224 | store | 4 | `sp -0x24` | `r3+2` |
| `ldr` at 0xaf228 | load | 4 | `sp +0x0` | `-` |
| `ldrb` at 0xaf234 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xaf238 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xaf244 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xaf248 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaf250 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xaf25c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaf260 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xaf268 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xaf270 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaf270 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaf270 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaf230 | `(unknown callee) 0xaf038` | `[sp]` | `0x0` | `r2` | `r3+2` |
| `bl` at 0xaf240 | `(unknown callee) 0xaf0bc` | `[sp]` | `[sp-17]:1` | `?` | `[sp-17]:1` |
| `bl` at 0xaf24c | `(unknown callee) 0xaf13c` | `[sp]` | `[sp-24]` | `?` | `?` |
| `bl` at 0xaf264 | `(unknown callee) 0x1b394` | `[sp]+2` | `[sp-28]` | `[sp-32]` | `[sp]+2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaf270 | `r0` | `[sp-36]` |
