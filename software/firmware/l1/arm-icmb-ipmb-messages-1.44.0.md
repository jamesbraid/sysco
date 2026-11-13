# ARM ICMB and IPMB message paths in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package4/icmb-ipmb-messages-facts.txt) retain all tool sections.

## `IcmbMsgRqRs`

- `IcmbMsgRqRs`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9dc3c to 0x9e0c8.
  Established. Equality `[r1+156] == 0xff` takes `0x9dcc0`, while inequality falls through (`beq` at 0x9dc68).
  The unnamed `0xa15c8` call carries `[r1+160]:1`, `[r1+168]:1`, `[r1+164]`, `[r1+156]` (`bl` at 0x9dcb8).
  A word assignment stores `ret(0xa15c8)` at entry `r1 +0x98` (`str` at 0x9dcbc).
  ICMB request/response matching semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9dc4c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x9dc50 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x9dc58 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x9dc5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9dc60 | load | 4 | `r1 +0x9c` | `-` |
| `ldr` at 0x9dc6c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9dc70 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x9dc74 | load | 1 | `r1 +0xa0` | `-` |
| `ldr` at 0x9dc78 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x9dc7c | load | 1 | `r1 +0xa8` | `-` |
| `ldr` at 0x9dc80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9dc84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9dc88 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x9dc90 | store | 4 | `sp -0x1b4` | `r1+169` |
| `ldr` at 0x9dc94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9dc98 | load | 4 | `r1 +0xec` | `-` |
| `str` at 0x9dc9c | store | 4 | `sp -0x1b0` | `[r1+236]` |
| `ldr` at 0x9dca0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x9dca8 | store | 4 | `sp -0x1ac` | `r1+8` |
| `ldr` at 0x9dcb0 | load | 4 | `r1 +0xa4` | `-` |
| `ldr` at 0x9dcb4 | load | 4 | `r1 +0x9c` | `-` |
| `str` at 0x9dcbc | store | 4 | `r1 +0x98` | `ret(0xa15c8)` |
| `strb` at 0x9dcd4 | store | 1 | `(unknown)` | `(ret(0x9e530) & 0xff)` |
| `ldrb` at 0x9dce0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x9dce4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9dce8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9dcf0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x9dcf8 | store | 4 | `sp -0x1b4` | `sp-184` |
| `ldr` at 0x9dd00 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x9dd08 | load | 4 | `[sp-28] +0x98` | `-` |
| `str` at 0x9dd14 | store | 4 | `sp -0xbc` | `ret(0xa1d68)` |
| `ldr` at 0x9dd18 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x9dd1c | load | 2 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x9dd20 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x9dd24 | load | 2 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x9dd2c | load | 4 | `sp -0xbc` | `-` |
| `str` at 0x9dd38 | store | 4 | `sp -0x1b4` | `sp-340` |
| `str` at 0x9dd50 | store | 4 | `sp -0x158` | `ret(0xa18fc)` |
| `str` at 0x9dd58 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x9dd5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9dd60 | load | 4 | `[sp-28] +0x18c` | `-` |
| `ldr` at 0x9dd64 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9dd70 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9dd7c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9dd90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9dd94 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x9dda0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9dda8 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x9ddb8 | store | 4 | `sp -0x19c` | `0x3` |
| `ldr` at 0x9ddbc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9ddc4 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x9ddd4 | store | 4 | `sp -0x170` | `0x0` |
| `ldr` at 0x9ddd8 | load | 4 | `sp -0x170` | `-` |
| `ldr` at 0x9dde4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9ddec | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x9ddf4 | load | 4 | `sp -0x158` | `-` |
| `str` at 0x9de00 | store | 4 | `sp -0x16c` | `ret(0xbd9c0)` |
| `ldr` at 0x9de04 | load | 4 | `sp -0x16c` | `-` |
| `ldr` at 0x9de08 | load | 4 | `abs 0x9e0c0` | `-` |
| `ldr` at 0x9de1c | load | 4 | `sp -0x170` | `-` |
| `str` at 0x9de24 | store | 4 | `sp -0x170` | `[sp-368]+1` |
| `ldr` at 0x9de2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9de30 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0x9de40 | store | 4 | `sp -0x19c` | `0x0` |
| `ldr` at 0x9de44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9de4c | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x9de58 | load | 4 | `sp -0x170` | `-` |
| `str` at 0x9de6c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x9de70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9de74 | load | 4 | `[sp-28] +0x190` | `-` |
| `ldr` at 0x9de78 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9de84 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9de90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9de94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9de9c | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x9deb0 | store | 4 | `sp -0x168` | `ret(0xbd808)` |
| `ldr` at 0x9deb4 | load | 4 | `sp -0x168` | `-` |
| `ldr` at 0x9deb8 | load | 4 | `abs 0x9e0c4` | `-` |
| `ldr` at 0x9dec8 | load | 4 | `sp -0x168` | `-` |
| `ldr` at 0x9ded8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9dee0 | load | 4 | `sp -0x168` | `-` |
| `str` at 0x9df0c | store | 4 | `sp -0x164` | `ret(0xa19d8)` |
| `ldr` at 0x9df10 | load | 4 | `sp -0x164` | `-` |
| `ldr` at 0x9df20 | load | 4 | `sp -0x160` | `-` |
| `str` at 0x9df2c | store | 4 | `sp -0x178` | `ret(0xa1b4c)` |
| `ldr` at 0x9df30 | load | 4 | `sp -0x178` | `-` |
| `ldr` at 0x9df44 | load | 4 | `sp -0x160` | `-` |
| `strb` at 0x9df54 | store | 1 | `(unknown)` | `(ret(0xa1be4) & 0xff)` |
| `ldr` at 0x9df58 | load | 4 | `sp -0x160` | `-` |
| `str` at 0x9df64 | store | 4 | `sp -0x180` | `ret(0xa1c68)` |
| `ldr` at 0x9df68 | load | 4 | `sp -0x160` | `-` |
| `str` at 0x9df74 | store | 4 | `sp -0x184` | `ret(0xa1cd4)` |
| `ldr` at 0x9df78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9df7c | load | 4 | `sp -0x184` | `-` |
| `str` at 0x9df80 | store | 4 | `[sp-28] +0x194` | `ret(0xa1cd4)` |
| `ldr` at 0x9df84 | load | 4 | `sp -0x184` | `-` |
| `ldr` at 0x9df90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9df94 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x9df98 | load | 4 | `sp -0x180` | `-` |
| `ldrb` at 0x9dfb0 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x9dfb8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x9dfc4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9dfc8 | load | 4 | `sp -0x160` | `-` |
| `str` at 0x9dfd0 | store | 4 | `[sp-28] +0x188` | `ret(0xa1d0c)` |
| `ldr` at 0x9dfd4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9dfd8 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x9dfe4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9dfe8 | load | 4 | `[sp-28] +0x188` | `-` |
| `str` at 0x9dff4 | store | 4 | `sp -0x184` | `ret(0xa17d0)` |
| `ldr` at 0x9dff8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9dffc | load | 4 | `[sp-28] +0x188` | `-` |
| `str` at 0x9e008 | store | 4 | `sp -0x180` | `ret(0xa1798)` |
| `ldr` at 0x9e00c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9e010 | load | 4 | `[sp-28] +0x188` | `-` |
| `str` at 0x9e01c | store | 4 | `sp -0x188` | `ret(0xa1754)` |
| `ldr` at 0x9e020 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9e024 | load | 4 | `[sp-28] +0x188` | `-` |
| `str` at 0x9e030 | store | 4 | `sp -0x18c` | `ret(0xa1710)` |
| `ldr` at 0x9e034 | load | 4 | `sp -0x184` | `-` |
| `ldr` at 0x9e040 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9e044 | load | 4 | `[sp-28] +0x9c` | `-` |
| `ldr` at 0x9e048 | load | 4 | `sp -0x180` | `-` |
| `ldr` at 0x9e054 | load | 4 | `sp -0x188` | `-` |
| `ldr` at 0x9e060 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9e064 | load | 4 | `[sp-28] +0xa4` | `-` |
| `ldr` at 0x9e068 | load | 4 | `sp -0x18c` | `-` |
| `str` at 0x9e078 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x9e07c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9e084 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x9e08c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9e094 | store | 4 | `sp -0x24` | `?` |
| `ldrb` at 0x9e0a4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x9e0b0 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x9e0bc | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x9e0bc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9e0bc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9e0bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9dcb8 | `(unknown callee) 0xa15c8` | `[r1+160]:1` | `[r1+168]:1` | `[r1+164]` | `[r1+156]` |
| `bl` at 0x9dcc4 | `(unknown callee) 0x9e530` | `0x20` | `?` | `?` | `?` |
| `bl` at 0x9dd0c | `(unknown callee) 0xa1d68` | `?` | `[[sp-28]+4]` | `[sp-28]+8` | `[[sp-28]+152]` |
| `bl` at 0x9dd48 | `(unknown callee) 0xa18fc` | `[[sp-24]+4]:2` | `[[sp-28]]:2` | `sp-184` | `?` |
| `bl` at 0x9dd8c | `(unknown callee) 0x9e0c8` | `0x14` | `?` | `[[sp-28]+396]` | `?` |
| `bl` at 0x9ddb0 | `(unknown callee) 0xbda54` | `[[sp-24]+12]` | `sp-424` | `sp-424` | `[sp-24]` |
| `bl` at 0x9ddcc | `(unknown callee) 0xbda94` | `[[sp-24]+12]` | `sp-424` | `sp-424` | `[sp-24]` |
| `bl` at 0x9ddf8 | `(unknown callee) 0xbd9c0` | `[[sp-24]+12]` | `sp-340` | `?` | `[sp-24]` |
| `bl` at 0x9de18 | `(unknown callee) 0x9e0c8` | `0x1` | `?` | `ret(0xbd9c0)` | `0xfffff698` |
| `bl` at 0x9de54 | `(unknown callee) 0xbda94` | `[[sp-24]+12]` | `sp-424` | `sp-424` | `[sp-24]` |
| `bl` at 0x9dea8 | `(unknown callee) 0xbd808` | `[[sp-24]+12]` | `[sp-28]+240` | `0x98` | `[sp-28]+240` |
| `bl` at 0x9df04 | `(unknown callee) 0xa19d8` | `[sp-28]+240` | `?` | `sp-370` | `sp-352` |
| `bl` at 0x9df24 | `(unknown callee) 0xa1b4c` | `[sp-352]` | `?` | `?` | `ret(0xa19d8)` |
| `bl` at 0x9df48 | `(unknown callee) 0xa1be4` | `[sp-352]` | `?` | `?` | `ret(0xa1b4c)` |
| `bl` at 0x9df5c | `(unknown callee) 0xa1c68` | `[sp-352]` | `?` | `sp-20` | `ret(0xa1be4)` |
| `bl` at 0x9df6c | `(unknown callee) 0xa1cd4` | `[sp-352]` | `?` | `?` | `ret(0xa1c68)` |
| `bl` at 0x9dfcc | `(unknown callee) 0xa1d0c` | `[sp-352]` | `?` | `?` | `?` |
| `bl` at 0x9dfec | `(unknown callee) 0xa17d0` | `[[sp-28]+392]` | `?` | `?` | `[sp-28]` |
| `bl` at 0x9e000 | `(unknown callee) 0xa1798` | `[[sp-28]+392]` | `?` | `?` | `[sp-28]` |
| `bl` at 0x9e014 | `(unknown callee) 0xa1754` | `[[sp-28]+392]` | `?` | `?` | `[sp-28]` |
| `bl` at 0x9e028 | `(unknown callee) 0xa1710` | `[[sp-28]+392]` | `?` | `?` | `[sp-28]` |
| `bl` at 0x9e0ac | `(unknown callee) 0x9e530` | `?` | `sp-20` | `[[sp-28]+396]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x9dc68 | `[r1+156] == 0xff` | `[r1+156] != 0xff` |
| `ble` at 0x9dd6c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x9dd78 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x9dd84 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x9dd9c | `[[sp-28]+4] != 0xb` | `[[sp-28]+4] == 0xb` |
| `bgt` at 0x9dde0 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0x9de10 | `ret(0xbd9c0) != 0xfffff698` | `ret(0xbd9c0) == 0xfffff698` |
| `bne` at 0x9de38 | `[[sp-28]+4] != 0xb` | `[[sp-28]+4] == 0xb` |
| `bne` at 0x9de60 | `Z == 0` | `not (Z == 0)` |
| `ble` at 0x9de80 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x9de8c | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0x9dec0 | `ret(0xbd808) != 0xfffff69e` | `ret(0xbd808) == 0xfffff69e` |
| `bgt` at 0x9ded0 | `ret(0xbd808) > 0x0 (signed, 32-bit)` | `ret(0xbd808) <= 0x0 (signed, 32-bit)` |
| `bne` at 0x9df18 | `ret(0xa19d8) != 0x0` | `ret(0xa19d8) == 0x0` |
| `beq` at 0x9df38 | `ret(0xa1b4c) == 0x1` | `ret(0xa1b4c) != 0x1` |
| `bne` at 0x9df8c | `[sp-388] != 0x0` | `[sp-388] == 0x0` |
| `bne` at 0x9dfa0 | `[[sp-28]+4] != [sp-384]` | `[[sp-28]+4] == [sp-384]` |
| `bne` at 0x9dfc0 | `? != ?` | `? == ?` |
| `bne` at 0x9dfe0 | `[[sp-28]+4] != 0x20` | `[[sp-28]+4] == 0x20` |
| `bne` at 0x9e03c | `[sp-388] != 0x0` | `[sp-388] == 0x0` |
| `bne` at 0x9e050 | `[[sp-28]+156] != [sp-384]` | `[[sp-28]+156] == [sp-384]` |
| `bne` at 0x9e05c | `[sp-392] != 0x1` | `[sp-392] == 0x1` |
| `bne` at 0x9e070 | `[[sp-28]+164] != ret(0xa1710)` | `[[sp-28]+164] == ret(0xa1710)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9e0bc | `r0` | `[sp-32]` |

## `IcmbIpmbRqBuild`

- `IcmbIpmbRqBuild`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa15c8 to 0xa1660.
  Established. Included byte assignments put `((r0 & 0xff) & 0xff)` at `[sp+8] +0x0` and `([sp-32] & 0xff)` at `[sp+8] +0x2` (`strb` at 0xa15f4, `strb` at 0xa1630).
  The unnamed `0x19610` call carries `[sp+8]+3`, `[sp]`, `[sp+4]`, `[sp+8]+3` (`bl` at 0xa1648).
  IPMB request field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa15d8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa15dc | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xa15e0 | store | 4 | `sp -0x20` | `r3` |
| `strb` at 0xa15e8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xa15ec | load | 4 | `sp +0x8` | `-` |
| `ldrb` at 0xa15f0 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0xa15f4 | store | 1 | `[sp+8] +0x0` | `((r0 & 0xff) & 0xff)` |
| `ldr` at 0xa15fc | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xa1604 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa160c | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xa161c | store | 1 | `[sp+8] +0x1` | `?` |
| `ldr` at 0xa1624 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xa162c | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xa1630 | store | 1 | `[sp+8] +0x2` | `([sp-32] & 0xff)` |
| `ldr` at 0xa1634 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xa1640 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xa1644 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xa164c | load | 4 | `sp +0x4` | `-` |
| `ldm` at 0xa165c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa165c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa165c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa1648 | `(unknown callee) 0x19610` | `[sp+8]+3` | `[sp]` | `[sp+4]` | `[sp+8]+3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa165c | `r0` | `[sp+4]+3` |

## `IcmbIpmbRsBuild`

- `IcmbIpmbRsBuild`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa1660 to 0xa1710.
  Established. The byte at `[sp+12] +0x3` receives `([sp] & 0xff)` (`strb` at 0xa16e0).
  The unnamed `0x19610` call carries `[sp+12]+4`, `[sp+4]`, `[sp+8]`, `[sp+12]+4` (`bl` at 0xa16f8);
  its own integer return candidate is `[sp+8]+4` (`ldm` at 0xa170c).
  IPMB response payload semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa1670 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa1674 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xa1678 | store | 4 | `sp -0x20` | `r3` |
| `strb` at 0xa1680 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xa1684 | load | 4 | `sp +0xc` | `-` |
| `ldrb` at 0xa1688 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0xa168c | store | 1 | `[sp+12] +0x0` | `((r0 & 0xff) & 0xff)` |
| `ldr` at 0xa1694 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xa169c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa16a4 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xa16b8 | store | 1 | `[sp+12] +0x1` | `?` |
| `ldr` at 0xa16c0 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xa16c8 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xa16cc | store | 1 | `[sp+12] +0x2` | `([sp-32] & 0xff)` |
| `ldr` at 0xa16d4 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xa16dc | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xa16e0 | store | 1 | `[sp+12] +0x3` | `([sp] & 0xff)` |
| `ldr` at 0xa16e4 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xa16f0 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xa16f4 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xa16fc | load | 4 | `sp +0x8` | `-` |
| `ldm` at 0xa170c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa170c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa170c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa16f8 | `(unknown callee) 0x19610` | `[sp+12]+4` | `[sp+4]` | `[sp+8]` | `[sp+12]+4` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa170c | `r0` | `[sp+8]+4` |

## `IcmbIpmbGetNetFn`

- `IcmbIpmbGetNetFn`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa1710 to 0xa1754.
  Established. A byte load reads entry `r0 +0x1` (`ldrb` at 0xa1730).
  The word at `sp -0x18` receives an unknown value (`str` at 0xa1740).
  The integer return record contains `[sp-24]` (`ldm` at 0xa1750).
  IPMB network-function encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa1720 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa1728 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa1730 | load | 1 | `r0 +0x1` | `-` |
| `str` at 0xa1740 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xa1744 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xa1750 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa1750 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa1750 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa1750 | `r0` | `[sp-24]` |

## `IcmbIpmbGetRqRsId`

- `IcmbIpmbGetRqRsId`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa1754 to 0xa1798.
  Established. The byte at entry `r0 +0x1` is read (`ldrb` at 0xa1774).
  A word store assigns an unknown value to `sp -0x18` (`str` at 0xa1784);
  its integer return candidate is `[sp-24]` (`ldm` at 0xa1794).
  Request/response bit meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa1764 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa176c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa1774 | load | 1 | `r0 +0x1` | `-` |
| `str` at 0xa1784 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xa1788 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xa1794 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa1794 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa1794 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa1794 | `r0` | `[sp-24]` |

## `IcmbIpmbGetCmd`

- `IcmbIpmbGetCmd`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa1798 to 0xa17d0.
  Established. A byte read fetches entry `r0 +0x2` (`ldrb` at 0xa17b8).
  The word at `sp -0x18` receives `[r0+2]:1` (`str` at 0xa17bc).
  The same expression is the integer return candidate (`ldm` at 0xa17cc).
  IPMB command numbering remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa17a8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa17b0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa17b8 | load | 1 | `r0 +0x2` | `-` |
| `str` at 0xa17bc | store | 4 | `sp -0x18` | `[r0+2]:1` |
| `ldr` at 0xa17c0 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xa17cc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa17cc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa17cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa17cc | `r0` | `[r0+2]:1` |

## `IcmbIpmbGetCmpCode`

- `IcmbIpmbGetCmpCode`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa17d0 to 0xa1808.
  Established. An included byte load reads entry `r0 +0x3` (`ldrb` at 0xa17f0);
  its loaded expression `[r0+3]:1` is stored at `sp -0x18` (`str` at 0xa17f4).
  The integer return record preserves that expression (`ldm` at 0xa1804).
  IPMB completion code meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa17e0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa17e8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa17f0 | load | 1 | `r0 +0x3` | `-` |
| `str` at 0xa17f4 | store | 4 | `sp -0x18` | `[r0+3]:1` |
| `ldr` at 0xa17f8 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xa1804 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa1804 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa1804 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa1804 | `r0` | `[r0+3]:1` |

## `IcmbIpmbGetData`

- `IcmbIpmbGetData`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa1808 to 0xa1864.
  Established. Nonzero `ret(0xa1754)` takes `0xa1848`, while zero falls through (`bne` at 0xa1834).
  Included word assignments place `[sp-20]+3` and `[sp-20]+4` at `sp -0x1c` on separate paths (`str` at 0xa1840, `str` at 0xa1850).
  The unnamed `0xa1754` call carries entry `r0`, `r1`, `r2`, `r3` (`bl` at 0xa1820).
  Payload pointer selection remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa1818 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa181c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa1828 | store | 4 | `sp -0x18` | `ret(0xa1754)` |
| `ldr` at 0xa182c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa1838 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa1840 | store | 4 | `sp -0x1c` | `[sp-20]+3` |
| `ldr` at 0xa1848 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa1850 | store | 4 | `sp -0x1c` | `[sp-20]+4` |
| `ldr` at 0xa1854 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xa1860 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa1860 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa1860 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa1820 | `(unknown callee) 0xa1754` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa1834 | `ret(0xa1754) != 0x0` | `ret(0xa1754) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa1860 | `r0` | `[sp-28]` |

## `IcmbMsgGetMsgType`

- `IcmbMsgGetMsgType`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa1b4c to 0xa1b88.
  Established. A byte load reads entry `r0 +0x0` (`ldrb` at 0xa1b64).
  An included word store writes an unknown value at `sp -0x18` (`str` at 0xa1b74);
  its integer return record is `[sp-24]` (`ldm` at 0xa1b84).
  ICMB message type encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa1b5c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa1b60 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa1b64 | load | 1 | `r0 +0x0` | `-` |
| `str` at 0xa1b74 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xa1b78 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xa1b84 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa1b84 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa1b84 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa1b84 | `r0` | `[sp-24]` |

## `IcmbMsgGetSeqNbr`

- `IcmbMsgGetSeqNbr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa1be4 to 0xa1c18.
  Established. An included byte read fetches entry `r0 +0x0` (`ldrb` at 0xa1bfc).
  A byte assignment puts an unknown value at `sp -0x15` (`strb` at 0xa1c04).
  The integer return candidate is `[sp-21]:1` (`ldm` at 0xa1c14).
  Sequence number interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa1bf4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa1bf8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa1bfc | load | 1 | `r0 +0x0` | `-` |
| `strb` at 0xa1c04 | store | 1 | `sp -0x15` | `?` |
| `ldrb` at 0xa1c08 | load | 1 | `sp -0x15` | `-` |
| `ldm` at 0xa1c14 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa1c14 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa1c14 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa1c14 | `r0` | `[sp-21]:1` |

## `IcmbMsgGetCmd`

- `IcmbMsgGetCmd`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa1c68 to 0xa1ca0.
  Established. The byte at entry `r0 +0x1` is read (`ldrb` at 0xa1c88).
  A word store saves `[r0+1]:1` at `sp -0x18` (`str` at 0xa1c8c).
  The integer return candidate retains `[r0+1]:1` (`ldm` at 0xa1c9c).
  ICMB command catalogue remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa1c78 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa1c80 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa1c88 | load | 1 | `r0 +0x1` | `-` |
| `str` at 0xa1c8c | store | 4 | `sp -0x18` | `[r0+1]:1` |
| `ldr` at 0xa1c90 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xa1c9c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa1c9c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa1c9c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa1c9c | `r0` | `[r0+1]:1` |

## `IcmbMsgGetCmpCode`

- `IcmbMsgGetCmpCode`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa1cd4 to 0xa1d0c.
  Established. A byte read fetches entry `r0 +0x2` (`ldrb` at 0xa1cf4).
  A word assignment writes `[r0+2]:1` to `sp -0x18` (`str` at 0xa1cf8);
  its integer return record is `[r0+2]:1` (`ldm` at 0xa1d08).
  ICMB completion status meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa1ce4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa1cec | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa1cf4 | load | 1 | `r0 +0x2` | `-` |
| `str` at 0xa1cf8 | store | 4 | `sp -0x18` | `[r0+2]:1` |
| `ldr` at 0xa1cfc | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xa1d08 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa1d08 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa1d08 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa1d08 | `r0` | `[r0+2]:1` |

## `IcmbMsgGetData`

- `IcmbMsgGetData`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa1d0c to 0xa1d68.
  Established. The unnamed `0xa1b4c` call carries entry `r0`, `r1`, `r2`, `r3` (`bl` at 0xa1d24);
  its nonzero result takes `0xa1d4c`, while zero falls through (`bne` at 0xa1d38).
  Included word assignments put `[sp-20]+2` and `[sp-20]+3` at `sp -0x1c` on separate paths (`str` at 0xa1d44, `str` at 0xa1d54).
  Message payload boundaries remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa1d1c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa1d20 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa1d2c | store | 4 | `sp -0x18` | `ret(0xa1b4c)` |
| `ldr` at 0xa1d30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa1d3c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa1d44 | store | 4 | `sp -0x1c` | `[sp-20]+2` |
| `ldr` at 0xa1d4c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa1d54 | store | 4 | `sp -0x1c` | `[sp-20]+3` |
| `ldr` at 0xa1d58 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xa1d64 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa1d64 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa1d64 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa1d24 | `(unknown callee) 0xa1b4c` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa1d38 | `ret(0xa1b4c) != 0x0` | `ret(0xa1b4c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa1d64 | `r0` | `[sp-28]` |

## `IcmbPktBuild`

- `IcmbPktBuild`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa18fc to 0xa19d8.
  Established. A byte assignment clears `[sp] +0x4` (`strb` at 0xa1960).
  The unnamed `0x19610` call carries `[sp]+5`, `[sp-24]`, `[sp-25]:1`, `[sp]+5` (`bl` at 0xa1978).
  The byte at `sp -0x1a` receives `((r3 & 0xff)+7 & 0xff)` (`strb` at 0xa1924).
  Packet length and checksum roles remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa190c | store | 4 | `sp -0x18` | `r2` |
| `strh` at 0xa1910 | store | 2 | `sp -0x12` | `(r0 & 0xffff)` |
| `strh` at 0xa1914 | store | 2 | `sp -0x14` | `(r1 & 0xffff)` |
| `strb` at 0xa1918 | store | 1 | `sp -0x19` | `(r3 & 0xff)` |
| `ldrb` at 0xa191c | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xa1924 | store | 1 | `sp -0x1a` | `((r3 & 0xff)+7 & 0xff)` |
| `ldrh` at 0xa1928 | load | 2 | `sp -0x14` | `-` |
| `ldr` at 0xa192c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xa1938 | load | 4 | `sp +0x0` | `-` |
| `ldrh` at 0xa1940 | load | 2 | `sp -0x12` | `-` |
| `ldr` at 0xa1954 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xa1960 | store | 1 | `[sp] +0x4` | `0x0` |
| `ldr` at 0xa1964 | load | 4 | `sp +0x0` | `-` |
| `ldrb` at 0xa196c | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xa1974 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa197c | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xa1980 | load | 4 | `sp +0x0` | `-` |
| `strh` at 0xa1990 | store | 2 | `sp -0x1c` | `(ret(0xa1864) & 0xffff)` |
| `ldrb` at 0xa1994 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xa1998 | load | 4 | `sp +0x0` | `-` |
| `ldrh` at 0xa19ac | load | 2 | `sp -0x1c` | `-` |
| `ldrb` at 0xa19c8 | load | 1 | `sp -0x1a` | `-` |
| `ldm` at 0xa19d4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa19d4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa19d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa1934 | `(unknown callee) 0xa1b04` | `[sp]` | `(r1 & 0xffff)` | `r2` | `(r1 & 0xffff)` |
| `bl` at 0xa194c | `(unknown callee) 0xa1b04` | `[sp]+2` | `[sp-18]:2` | `[sp-18]:2` | `[sp]+2` |
| `bl` at 0xa1978 | `(unknown callee) 0x19610` | `[sp]+5` | `[sp-24]` | `[sp-25]:1` | `[sp]+5` |
| `bl` at 0xa1988 | `(unknown callee) 0xa1864` | `[sp]` | `[sp-26]:1` | `?` | `[sp-26]:1` |
| `bl` at 0xa19c4 | `(unknown callee) 0xa1b04` | `([sp-26]:1 + [sp])-2` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa19d4 | `r0` | `[sp-26]:1` |

## `IcmbPktDecode`

- `IcmbPktDecode`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa19d8 to 0xa1aa8.
  Established. An included halfword store writes `(ret(0xa1aa8) & 0xffff)` at `[sp-28] +0x0` (`strh` at 0xa1a70).
  A word assignment stores `[sp-20]+5` at `[sp-32] +0x0` (`str` at 0xa1a8c).
  Equality `([sp-36]:2 + (ret(0xa1aa8) & 0xffff)) == 0xffff` takes `0xa1a58`, while inequality falls through (`beq` at 0xa1a48).
  Packet validation arithmetic meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa19e8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa19ec | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xa19f0 | store | 4 | `sp -0x20` | `r3` |
| `strb` at 0xa19f8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xa19fc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xa1a00 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xa1a10 | store | 2 | `sp -0x24` | `(ret(0xa1864) & 0xffff)` |
| `ldrb` at 0xa1a14 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xa1a18 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xa1a30 | store | 2 | `sp -0x26` | `(ret(0xa1aa8) & 0xffff)` |
| `ldrh` at 0xa1a34 | load | 2 | `sp -0x24` | `-` |
| `ldrh` at 0xa1a38 | load | 2 | `sp -0x26` | `-` |
| `ldr` at 0xa1a40 | load | 4 | `abs 0xa1aa4` | `-` |
| `str` at 0xa1a50 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xa1a58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa1a6c | load | 4 | `sp -0x1c` | `-` |
| `strh` at 0xa1a70 | store | 2 | `[sp-28] +0x0` | `(ret(0xa1aa8) & 0xffff)` |
| `ldrb` at 0xa1a74 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0xa1a7c | store | 1 | `sp -0x21` | `([sp-21]:1-7 & 0xff)` |
| `ldr` at 0xa1a80 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa1a84 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa1a8c | store | 4 | `[sp-32] +0x0` | `[sp-20]+5` |
| `ldrb` at 0xa1a90 | load | 1 | `sp -0x21` | `-` |
| `str` at 0xa1a94 | store | 4 | `sp -0x2c` | `[sp-33]:1` |
| `ldr` at 0xa1a98 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0xa1aa0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa1aa0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa1aa0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa1a08 | `(unknown callee) 0xa1864` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xa1a28 | `(unknown callee) 0xa1aa8` | `([sp-21]:1 + [sp-20])-2` | `?` | `[sp-21]:1` | `([sp-21]:1 + [sp-20])-2` |
| `bl` at 0xa1a64 | `(unknown callee) 0xa1aa8` | `[sp-20]+2` | `?` | `([sp-36]:2 + (ret(0xa1aa8) & 0xffff))` | `[sp-20]+2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa1a48 | `([sp-36]:2 + (ret(0xa1aa8) & 0xffff)) == 0xffff` | `([sp-36]:2 + (ret(0xa1aa8) & 0xffff)) != 0xffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa1aa0 | `r0` | `?` |
