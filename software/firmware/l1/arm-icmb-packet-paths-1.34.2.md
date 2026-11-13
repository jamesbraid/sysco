# ARM controller-message packet paths in 1.34.2

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

[Fact sheets and full listings](disasm/arm-1.34.2-package3/icmb-packet-paths-facts.txt) retain all tool sections.

## `IcmbMsgRqRs`

- `IcmbMsgRqRs`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x90884 to 0x90d14.
  Established. The branch skips the call below when `[r1+156] == 0xff` (`beq` at 0x908b0).
  On fall-through, the call targets `0x94264` with candidates `[r1+160]:1`, `[r1+168]:1`,
  `[r1+164]`, `[r1+156]` (`bl` at 0x90900); its result is stored as a word at entry `r1 +0x98`
  (`str` at 0x90904). Later calls include `0x94570` with `[[sp-24]+4]:2`, `[[sp-28]]:2`, `sp-184`,
  `[sp-188]:1` and `0x94648` with `[sp-28]+240`, `[sp-360]:1`, `sp-370`, `sp-352` (`bl` at 0x9098c,
  `bl` at 0x90b50). The latter nonzero result takes the continuation, while zero falls through
  to the iteration update (`bne` at 0x90b64). Message matching, timeout units and physical
  exchanges are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x90894 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x90898 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x908a0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x908a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x908a8 | load | 4 | `r1 +0x9c` | `-` |
| `ldr` at 0x908b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x908b8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x908bc | load | 1 | `r1 +0xa0` | `-` |
| `ldr` at 0x908c0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x908c4 | load | 1 | `r1 +0xa8` | `-` |
| `ldr` at 0x908c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x908cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x908d0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x908d8 | store | 4 | `sp -0x1b4` | `r1+169` |
| `ldr` at 0x908dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x908e0 | load | 4 | `r1 +0xec` | `-` |
| `str` at 0x908e4 | store | 4 | `sp -0x1b0` | `[r1+236]` |
| `ldr` at 0x908e8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x908f0 | store | 4 | `sp -0x1ac` | `r1+8` |
| `ldr` at 0x908f8 | load | 4 | `r1 +0xa4` | `-` |
| `ldr` at 0x908fc | load | 4 | `r1 +0x9c` | `-` |
| `str` at 0x90904 | store | 4 | `r1 +0x98` | `ret(0x94264)` |
| `strb` at 0x9091c | store | 1 | `(unknown)` | `(ret(0x91154) & 0xff)` |
| `ldrb` at 0x90928 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x9092c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x90930 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x90938 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x90940 | store | 4 | `sp -0x1b4` | `sp-184` |
| `ldr` at 0x90948 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x9094c | load | 4 | `[sp-28] +0x98` | `-` |
| `str` at 0x90958 | store | 4 | `sp -0xbc` | `ret(0x949a8)` |
| `ldr` at 0x9095c | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x90960 | load | 2 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x90964 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x90968 | load | 2 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x90970 | load | 1 | `sp -0xbc` | `-` |
| `str` at 0x90978 | store | 4 | `sp -0x1b4` | `sp-340` |
| `str` at 0x90994 | store | 4 | `sp -0x158` | `ret(0x94570)` |
| `str` at 0x9099c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x909a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x909a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x909a8 | load | 4 | `[sp-28] +0x18c` | `-` |
| `ldr` at 0x909b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x909c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x909d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x909d8 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x909e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x909ec | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x909fc | store | 4 | `sp -0x19c` | `0x3` |
| `ldr` at 0x90a00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x90a08 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x90a18 | store | 4 | `sp -0x170` | `0x0` |
| `ldr` at 0x90a1c | load | 4 | `sp -0x170` | `-` |
| `ldr` at 0x90a2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x90a34 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x90a3c | load | 4 | `sp -0x158` | `-` |
| `str` at 0x90a48 | store | 4 | `sp -0x16c` | `ret(0xaded0)` |
| `ldr` at 0x90a4c | load | 4 | `sp -0x16c` | `-` |
| `ldr` at 0x90a50 | load | 4 | `abs 0x90d08` | `-` |
| `ldr` at 0x90a64 | load | 4 | `sp -0x170` | `-` |
| `str` at 0x90a6c | store | 4 | `sp -0x170` | `[sp-368]+1` |
| `ldr` at 0x90a74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x90a78 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0x90a88 | store | 4 | `sp -0x19c` | `0x0` |
| `ldr` at 0x90a8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x90a94 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x90aa0 | load | 4 | `sp -0x170` | `-` |
| `str` at 0x90ab4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x90ab8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x90abc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x90ac0 | load | 4 | `[sp-28] +0x190` | `-` |
| `ldr` at 0x90acc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x90ad8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x90adc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x90ae4 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x90af8 | store | 4 | `sp -0x168` | `ret(0xadd10)` |
| `ldr` at 0x90afc | load | 4 | `sp -0x168` | `-` |
| `ldr` at 0x90b00 | load | 4 | `abs 0x90d0c` | `-` |
| `ldr` at 0x90b10 | load | 4 | `sp -0x168` | `-` |
| `ldr` at 0x90b20 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x90b28 | load | 4 | `abs 0x90d10` | `-` |
| `ldrb` at 0x90b30 | load | 1 | `sp -0x168` | `-` |
| `str` at 0x90b58 | store | 4 | `sp -0x164` | `ret(0x94648)` |
| `ldr` at 0x90b5c | load | 4 | `sp -0x164` | `-` |
| `ldr` at 0x90b6c | load | 4 | `sp -0x160` | `-` |
| `str` at 0x90b78 | store | 4 | `sp -0x178` | `ret(0x947b0)` |
| `ldr` at 0x90b7c | load | 4 | `sp -0x178` | `-` |
| `ldr` at 0x90b90 | load | 4 | `sp -0x160` | `-` |
| `strb` at 0x90ba0 | store | 1 | `(unknown)` | `(ret(0x9483c) & 0xff)` |
| `ldr` at 0x90ba4 | load | 4 | `sp -0x160` | `-` |
| `str` at 0x90bb0 | store | 4 | `sp -0x180` | `ret(0x948b8)` |
| `ldr` at 0x90bb4 | load | 4 | `sp -0x160` | `-` |
| `str` at 0x90bc0 | store | 4 | `sp -0x184` | `ret(0x9491c)` |
| `ldr` at 0x90bc4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x90bc8 | load | 4 | `sp -0x184` | `-` |
| `str` at 0x90bcc | store | 4 | `[sp-28] +0x194` | `ret(0x9491c)` |
| `ldr` at 0x90bd0 | load | 4 | `sp -0x184` | `-` |
| `ldr` at 0x90bdc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x90be0 | load | 4 | `sp -0x180` | `-` |
| `ldr` at 0x90be4 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x90bfc | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x90c04 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x90c10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x90c14 | load | 4 | `sp -0x160` | `-` |
| `str` at 0x90c1c | store | 4 | `[sp-28] +0x188` | `ret(0x94950)` |
| `ldr` at 0x90c20 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x90c24 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x90c30 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x90c34 | load | 4 | `[sp-28] +0x188` | `-` |
| `str` at 0x90c40 | store | 4 | `sp -0x184` | `ret(0x94450)` |
| `ldr` at 0x90c44 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x90c48 | load | 4 | `[sp-28] +0x188` | `-` |
| `str` at 0x90c54 | store | 4 | `sp -0x180` | `ret(0x9441c)` |
| `ldr` at 0x90c58 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x90c5c | load | 4 | `[sp-28] +0x188` | `-` |
| `str` at 0x90c68 | store | 4 | `sp -0x188` | `ret(0x943dc)` |
| `ldr` at 0x90c6c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x90c70 | load | 4 | `[sp-28] +0x188` | `-` |
| `str` at 0x90c7c | store | 4 | `sp -0x18c` | `ret(0x9439c)` |
| `ldr` at 0x90c80 | load | 4 | `sp -0x184` | `-` |
| `ldr` at 0x90c8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x90c90 | load | 4 | `sp -0x180` | `-` |
| `ldr` at 0x90c94 | load | 4 | `[sp-28] +0x9c` | `-` |
| `ldr` at 0x90ca0 | load | 4 | `sp -0x188` | `-` |
| `ldr` at 0x90cac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x90cb0 | load | 4 | `sp -0x18c` | `-` |
| `ldr` at 0x90cb4 | load | 4 | `[sp-28] +0xa4` | `-` |
| `str` at 0x90cc4 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x90cc8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x90cd0 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x90cd8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x90ce0 | store | 4 | `sp -0x24` | `?` |
| `ldrb` at 0x90cf0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x90cfc | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x90d04 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x90d04 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x90d04 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x90d04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x90900 | `(unknown callee) 0x94264` | `[r1+160]:1` | `[r1+168]:1` | `[r1+164]` | `[r1+156]` |
| `bl` at 0x9090c | `(unknown callee) 0x91154` | `0x20` | `?` | `?` | `?` |
| `bl` at 0x90950 | `(unknown callee) 0x949a8` | `?` | `[[sp-28]+4]` | `[sp-28]+8` | `[[sp-28]+152]` |
| `bl` at 0x9098c | `(unknown callee) 0x94570` | `[[sp-24]+4]:2` | `[[sp-28]]:2` | `sp-184` | `[sp-188]:1` |
| `bl` at 0x909d0 | `(unknown callee) 0x918f0` | `0x14` | `?` | `?` | `?` |
| `bl` at 0x909f4 | `(unknown callee) 0xadf60` | `[[sp-24]+12]` | `sp-424` | `sp-424` | `[sp-24]` |
| `bl` at 0x90a10 | `(unknown callee) 0xadf9c` | `[[sp-24]+12]` | `sp-424` | `sp-424` | `[sp-24]` |
| `bl` at 0x90a40 | `(unknown callee) 0xaded0` | `[[sp-24]+12]` | `sp-340` | `?` | `[sp-24]` |
| `bl` at 0x90a60 | `(unknown callee) 0x918f0` | `0x1` | `?` | `ret(0xaded0)` | `0xfffff698` |
| `bl` at 0x90a9c | `(unknown callee) 0xadf9c` | `[[sp-24]+12]` | `sp-424` | `sp-424` | `[sp-24]` |
| `bl` at 0x90af0 | `(unknown callee) 0xadd10` | `[[sp-24]+12]` | `[sp-28]+240` | `0x98` | `[sp-28]+240` |
| `bl` at 0x90b50 | `(unknown callee) 0x94648` | `[sp-28]+240` | `[sp-360]:1` | `sp-370` | `sp-352` |
| `bl` at 0x90b70 | `(unknown callee) 0x947b0` | `[sp-352]` | `?` | `?` | `ret(0x94648)` |
| `bl` at 0x90b94 | `(unknown callee) 0x9483c` | `[sp-352]` | `?` | `?` | `ret(0x947b0)` |
| `bl` at 0x90ba8 | `(unknown callee) 0x948b8` | `[sp-352]` | `sp-20` | `?` | `ret(0x9483c)` |
| `bl` at 0x90bb8 | `(unknown callee) 0x9491c` | `[sp-352]` | `?` | `?` | `ret(0x948b8)` |
| `bl` at 0x90c18 | `(unknown callee) 0x94950` | `[sp-352]` | `?` | `?` | `?` |
| `bl` at 0x90c38 | `(unknown callee) 0x94450` | `[[sp-28]+392]` | `?` | `?` | `[sp-28]` |
| `bl` at 0x90c4c | `(unknown callee) 0x9441c` | `[[sp-28]+392]` | `?` | `?` | `[sp-28]` |
| `bl` at 0x90c60 | `(unknown callee) 0x943dc` | `[[sp-28]+392]` | `?` | `?` | `[sp-28]` |
| `bl` at 0x90c74 | `(unknown callee) 0x9439c` | `[[sp-28]+392]` | `?` | `?` | `[sp-28]` |
| `bl` at 0x90cf8 | `(unknown callee) 0x91154` | `?` | `sp-20` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x908b0 | `[r1+156] == 0xff` | `[r1+156] != 0xff` |
| `bge` at 0x909b0 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x909bc | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x909c8 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x909e0 | `[[sp-28]+4] != 0xb` | `[[sp-28]+4] == 0xb` |
| `ble` at 0x90a24 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x90a58 | `ret(0xaded0) != 0xfffff698` | `ret(0xaded0) == 0xfffff698` |
| `bne` at 0x90a80 | `[[sp-28]+4] != 0xb` | `[[sp-28]+4] == 0xb` |
| `bne` at 0x90aa8 | `Z == 0` | `not (Z == 0)` |
| `bge` at 0x90ac8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x90ad4 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0x90b08 | `ret(0xadd10) != 0xfffff69e` | `ret(0xadd10) == 0xfffff69e` |
| `bgt` at 0x90b18 | `ret(0xadd10) > 0x0 (signed, 32-bit)` | `ret(0xadd10) <= 0x0 (signed, 32-bit)` |
| `bne` at 0x90b64 | `ret(0x94648) != 0x0` | `ret(0x94648) == 0x0` |
| `beq` at 0x90b84 | `ret(0x947b0) == 0x1` | `ret(0x947b0) != 0x1` |
| `bne` at 0x90bd8 | `[sp-388] != 0x0` | `[sp-388] == 0x0` |
| `bne` at 0x90bec | `[sp-384] != [[sp-28]+4]` | `[sp-384] == [[sp-28]+4]` |
| `bne` at 0x90c0c | `? != ?` | `? == ?` |
| `bne` at 0x90c2c | `[[sp-28]+4] != 0x20` | `[[sp-28]+4] == 0x20` |
| `bne` at 0x90c88 | `[sp-388] != 0x0` | `[sp-388] == 0x0` |
| `bne` at 0x90c9c | `[sp-384] != [[sp-28]+156]` | `[sp-384] == [[sp-28]+156]` |
| `bne` at 0x90ca8 | `[sp-392] != 0x1` | `[sp-392] == 0x1` |
| `bne` at 0x90cbc | `ret(0x9439c) != [[sp-28]+164]` | `ret(0x9439c) == [[sp-28]+164]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x90d04 | `r0` | `[sp-32]` |

## `IcmbIpmbRqBuild`

- `IcmbIpmbRqBuild`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x94264 to 0x942f4.
  Established. Byte stores at `[sp+8] +0x0` and `[sp+8] +0x2` write `((r0 & 0xff) & 0xff)` and
  `([sp-32]:1 & 0xff)` (`strb` at 0x94290, `strb` at 0x942c8). The intervening byte store at `[sp+8]
  +0x1` has unknown value (`strb` at 0x942b4). The only call targets `0x18ae8` with `[sp+8]+3`,
  `[sp]`, `[sp+4]`, `[sp+8]+3` (`bl` at 0x942e0). The return reports `[sp+4]+3` (`ldmdb` at
  0x942f0). The byte fields’ protocol meanings are unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x94274 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x94278 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x9427c | store | 4 | `sp -0x20` | `r3` |
| `strb` at 0x94284 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x94288 | load | 4 | `sp +0x8` | `-` |
| `ldrb` at 0x9428c | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x94290 | store | 1 | `[sp+8] +0x0` | `((r0 & 0xff) & 0xff)` |
| `ldr` at 0x94298 | load | 4 | `sp +0x8` | `-` |
| `ldrb` at 0x942a0 | load | 1 | `sp -0x1c` | `-` |
| `ldrb` at 0x942ac | load | 1 | `sp -0x18` | `-` |
| `strb` at 0x942b4 | store | 1 | `[sp+8] +0x1` | `?` |
| `ldr` at 0x942bc | load | 4 | `sp +0x8` | `-` |
| `ldrb` at 0x942c4 | load | 1 | `sp -0x20` | `-` |
| `strb` at 0x942c8 | store | 1 | `[sp+8] +0x2` | `([sp-32]:1 & 0xff)` |
| `ldr` at 0x942cc | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x942d8 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x942dc | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x942e4 | load | 4 | `sp +0x4` | `-` |
| `ldmdb` at 0x942f0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x942f0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x942f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x942e0 | `(unknown callee) 0x18ae8` | `[sp+8]+3` | `[sp]` | `[sp+4]` | `[sp+8]+3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x942f0 | `r0` | `[sp+4]+3` |

## `IcmbIpmbRsBuild`

- `IcmbIpmbRsBuild`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x942f4 to 0x9439c.
  Established. Four byte stores address `[sp+12] +0x0`, `+0x1`, `+0x2`, `+0x3` (`strb` at 0x94320,
  `strb` at 0x94348, `strb` at 0x9435c, `strb` at 0x94370). Their values are `((r0 & 0xff) & 0xff)`,
  unknown, `([sp-32]:1 & 0xff)`, `([sp]:1 & 0xff)`, respectively (`strb` at 0x94320, `strb` at
  0x94348, `strb` at 0x9435c, `strb` at 0x94370). The call to `0x18ae8` carries `[sp+12]+4`,
  `[sp+4]`, `[sp+8]`, `[sp+12]+4` (`bl` at 0x94388). r0 returns `[sp+8]+4` (`ldmdb` at 0x94398).
  Response-field semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x94304 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x94308 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x9430c | store | 4 | `sp -0x20` | `r3` |
| `strb` at 0x94314 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x94318 | load | 4 | `sp +0xc` | `-` |
| `ldrb` at 0x9431c | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x94320 | store | 1 | `[sp+12] +0x0` | `((r0 & 0xff) & 0xff)` |
| `ldr` at 0x94328 | load | 4 | `sp +0xc` | `-` |
| `ldrb` at 0x94330 | load | 1 | `sp -0x1c` | `-` |
| `ldrb` at 0x9433c | load | 1 | `sp -0x18` | `-` |
| `strb` at 0x94348 | store | 1 | `[sp+12] +0x1` | `?` |
| `ldr` at 0x94350 | load | 4 | `sp +0xc` | `-` |
| `ldrb` at 0x94358 | load | 1 | `sp -0x20` | `-` |
| `strb` at 0x9435c | store | 1 | `[sp+12] +0x2` | `([sp-32]:1 & 0xff)` |
| `ldr` at 0x94364 | load | 4 | `sp +0xc` | `-` |
| `ldrb` at 0x9436c | load | 1 | `sp +0x0` | `-` |
| `strb` at 0x94370 | store | 1 | `[sp+12] +0x3` | `([sp]:1 & 0xff)` |
| `ldr` at 0x94374 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x94380 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x94384 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x9438c | load | 4 | `sp +0x8` | `-` |
| `ldmdb` at 0x94398 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x94398 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x94398 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x94388 | `(unknown callee) 0x18ae8` | `[sp+12]+4` | `[sp+4]` | `[sp+8]` | `[sp+12]+4` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x94398 | `r0` | `[sp+8]+4` |

## `IcmbIpmbGetNetFn`

- `IcmbIpmbGetNetFn`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9439c to 0x943dc.
  Established. A byte load reads entry `r0 +0x1` (`ldrb` at 0x943bc). The later word store at `sp
  -0x18` has unknown value (`str` at 0x943cc). The return reports `[sp-24]` (`ldmdb` at 0x943d8). No
  calls or conditional branches are reported, and the extracted value’s network-function
  interpretation is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x943ac | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x943b4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x943bc | load | 1 | `r0 +0x1` | `-` |
| `str` at 0x943cc | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x943d0 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x943d8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x943d8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x943d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x943d8 | `r0` | `[sp-24]` |

## `IcmbIpmbGetRqRsId`

- `IcmbIpmbGetRqRsId`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x943dc to 0x9441c.
  Established. The byte read addresses entry `r0 +0x1` (`ldrb` at 0x943fc). A word store at `sp
  -0x18` receives unknown value (`str` at 0x9440c). r0 returns `[sp-24]` (`ldmdb` at 0x94418). The
  body reports no calls or conditional branches. A request/response identity mapping is not
  established by these facts.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x943ec | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x943f4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x943fc | load | 1 | `r0 +0x1` | `-` |
| `str` at 0x9440c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x94410 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x94418 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x94418 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x94418 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x94418 | `r0` | `[sp-24]` |

## `IcmbIpmbGetCmd`

- `IcmbIpmbGetCmd`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9441c to 0x94450.
  Established. A byte load at entry `r0 +0x2` supplies word `[r0+2]:1` to `sp -0x18` (`ldrb` at
  0x9443c, `str` at 0x94440). That value reaches the r0 return (`ldmdb` at 0x9444c). No call or
  conditional branch is reported. The byte’s command-code interpretation remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9442c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x94434 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x9443c | load | 1 | `r0 +0x2` | `-` |
| `str` at 0x94440 | store | 4 | `sp -0x18` | `[r0+2]:1` |
| `ldr` at 0x94444 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x9444c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x9444c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x9444c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x9444c | `r0` | `[r0+2]:1` |

## `IcmbIpmbGetCmpCode`

- `IcmbIpmbGetCmpCode`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x94450 to 0x94484.
  Established. The byte at entry `r0 +0x3` is read and stored as word `[r0+3]:1` at `sp -0x18`
  (`ldrb` at 0x94470, `str` at 0x94474). The return reports `[r0+3]:1` in r0 (`ldmdb` at 0x94480).
  Calls and conditional branches are absent from the facts. Completion-status semantics are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x94460 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x94468 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x94470 | load | 1 | `r0 +0x3` | `-` |
| `str` at 0x94474 | store | 4 | `sp -0x18` | `[r0+3]:1` |
| `ldr` at 0x94478 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x94480 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x94480 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x94480 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x94480 | `r0` | `[r0+3]:1` |

## `IcmbIpmbGetData`

- `IcmbIpmbGetData`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x94484 to 0x944dc.
  Established. The only call targets `0x943dc` with all four entry candidates (`bl` at 0x9449c); its
  nonzero result takes the path to word `[sp-20]+4` at `sp -0x1c`, while zero falls through
  to `[sp-20]+3` there (`bne` at 0x944b0, `str` at 0x944cc, `str` at 0x944bc). The return
  reports `[sp-28]` (`ldmdb` at 0x944d8). The selected data’s protocol role is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x94494 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x94498 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x944a4 | store | 4 | `sp -0x18` | `ret(0x943dc)` |
| `ldr` at 0x944a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x944b4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x944bc | store | 4 | `sp -0x1c` | `[sp-20]+3` |
| `ldr` at 0x944c4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x944cc | store | 4 | `sp -0x1c` | `[sp-20]+4` |
| `ldr` at 0x944d0 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x944d8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x944d8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x944d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9449c | `(unknown callee) 0x943dc` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x944b0 | `ret(0x943dc) != 0x0` | `ret(0x943dc) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x944d8 | `r0` | `[sp-28]` |

## `IcmbPktBuild`

- `IcmbPktBuild`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x94570 to 0x94648.
  Established. A byte store clears `[sp] +0x4` (`strb` at 0x945d4). The call to `0x18ae8` carries
  `[sp]+5`, `[sp-24]`, `[sp-25]:1`, `[sp]+5` (`bl` at 0x945ec). The later call to `0x944dc` has
  `[sp]`, `[sp-26]:1`, unknown, `[sp-26]:1` (`bl` at 0x945fc); its result is saved as halfword
  `(ret(0x944dc) & 0xffff)` at `sp -0x1c` (`strh` at 0x94604). The return reports `[sp-26]:1`, with
  packet-layout and checksum meaning unproven (`ldmdb` at 0x94644).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x94580 | store | 4 | `sp -0x18` | `r2` |
| `strh` at 0x94584 | store | 2 | `sp -0x12` | `(r0 & 0xffff)` |
| `strh` at 0x94588 | store | 2 | `sp -0x14` | `(r1 & 0xffff)` |
| `strb` at 0x9458c | store | 1 | `sp -0x19` | `(r3 & 0xff)` |
| `ldrb` at 0x94590 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0x94598 | store | 1 | `sp -0x1a` | `((r3 & 0xff)+7 & 0xff)` |
| `ldrh` at 0x9459c | load | 2 | `sp -0x14` | `-` |
| `ldr` at 0x945a0 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x945ac | load | 4 | `sp +0x0` | `-` |
| `ldrh` at 0x945b4 | load | 2 | `sp -0x12` | `-` |
| `ldr` at 0x945c8 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0x945d4 | store | 1 | `[sp] +0x4` | `0x0` |
| `ldr` at 0x945d8 | load | 4 | `sp +0x0` | `-` |
| `ldrb` at 0x945e0 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x945e8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x945f0 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0x945f4 | load | 4 | `sp +0x0` | `-` |
| `strh` at 0x94604 | store | 2 | `sp -0x1c` | `(ret(0x944dc) & 0xffff)` |
| `ldrb` at 0x94608 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0x9460c | load | 4 | `sp +0x0` | `-` |
| `ldrh` at 0x94620 | load | 2 | `sp -0x1c` | `-` |
| `ldrb` at 0x9463c | load | 1 | `sp -0x1a` | `-` |
| `ldmdb` at 0x94644 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x94644 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x94644 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x945a8 | `(unknown callee) 0x9476c` | `[sp]` | `(r1 & 0xffff)` | `r2` | `(r1 & 0xffff)` |
| `bl` at 0x945c0 | `(unknown callee) 0x9476c` | `[sp]+2` | `[sp-18]:2` | `[sp-18]:2` | `[sp]+2` |
| `bl` at 0x945ec | `(unknown callee) 0x18ae8` | `[sp]+5` | `[sp-24]` | `[sp-25]:1` | `[sp]+5` |
| `bl` at 0x945fc | `(unknown callee) 0x944dc` | `[sp]` | `[sp-26]:1` | `?` | `[sp-26]:1` |
| `bl` at 0x94638 | `(unknown callee) 0x9476c` | `([sp-26]:1 + [sp])-2` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x94644 | `r0` | `[sp-26]:1` |

## `IcmbPktDecode`

- `IcmbPktDecode`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x94648 to 0x94714.
  Established. The deciding branch takes its continuation for `([sp-36]:2 + (ret(0x94714) & 0xffff))
  == 0xffff` and falls through on inequality (`beq` at 0x946b8). The continuation includes a
  halfword store of `(ret(0x94714) & 0xffff)` at `[sp-28] +0x0` and a word store of `[sp-20]+5` at
  `[sp-32] +0x0` (`strh` at 0x946e0, `str` at 0x946fc). The final stack word at `sp -0x2c` can
  receive `[sp-33]:1` (`str` at 0x94704). The acceptance rule’s protocol interpretation and merged
  return remain unknown (`ldmdb` at 0x9470c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x94658 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9465c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x94660 | store | 4 | `sp -0x20` | `r3` |
| `strb` at 0x94668 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x9466c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x94670 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0x94680 | store | 2 | `sp -0x24` | `(ret(0x944dc) & 0xffff)` |
| `ldrb` at 0x94684 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x94688 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0x946a0 | store | 2 | `sp -0x26` | `(ret(0x94714) & 0xffff)` |
| `ldrh` at 0x946a4 | load | 2 | `sp -0x24` | `-` |
| `ldrh` at 0x946a8 | load | 2 | `sp -0x26` | `-` |
| `ldr` at 0x946b0 | load | 4 | `abs 0x94710` | `-` |
| `str` at 0x946c0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x946c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x946dc | load | 4 | `sp -0x1c` | `-` |
| `strh` at 0x946e0 | store | 2 | `[sp-28] +0x0` | `(ret(0x94714) & 0xffff)` |
| `ldrb` at 0x946e4 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0x946ec | store | 1 | `sp -0x21` | `([sp-21]:1-7 & 0xff)` |
| `ldr` at 0x946f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x946f4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x946fc | store | 4 | `[sp-32] +0x0` | `[sp-20]+5` |
| `ldrb` at 0x94700 | load | 1 | `sp -0x21` | `-` |
| `str` at 0x94704 | store | 4 | `sp -0x2c` | `[sp-33]:1` |
| `ldr` at 0x94708 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x9470c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x9470c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x9470c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x94678 | `(unknown callee) 0x944dc` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x94698 | `(unknown callee) 0x94714` | `([sp-21]:1 + [sp-20])-2` | `?` | `[sp-21]:1` | `([sp-21]:1 + [sp-20])-2` |
| `bl` at 0x946d4 | `(unknown callee) 0x94714` | `[sp-20]+2` | `?` | `([sp-36]:2 + (ret(0x94714) & 0xffff))` | `[sp-20]+2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x946b8 | `([sp-36]:2 + (ret(0x94714) & 0xffff)) == 0xffff` | `([sp-36]:2 + (ret(0x94714) & 0xffff)) != 0xffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x9470c | `r0` | `?` |

## `IcmbCvtToU16`

- `IcmbCvtToU16`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x94714 to 0x9476c.
  Established. The two byte reads address entry `r0 +0x1` and `r0 +0x0` (`ldrb` at 0x94734, `ldrb`
  at 0x9474c). The first halfword store at `sp -0x16` writes `([r0+1]:1 & 0xffff)`, while two later
  halfword stores there have unknown values (`strh` at 0x94738, `strh` at 0x94744, `strh` at
  0x9475c). The return reports `[sp-22]:2` (`ldmdb` at 0x94768). No calls are reported. A complete
  integer-encoding rule remains unknown in the analysis.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x94724 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x9472c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x94734 | load | 1 | `r0 +0x1` | `-` |
| `strh` at 0x94738 | store | 2 | `sp -0x16` | `([r0+1]:1 & 0xffff)` |
| `ldrh` at 0x9473c | load | 2 | `sp -0x16` | `-` |
| `strh` at 0x94744 | store | 2 | `sp -0x16` | `?` |
| `ldr` at 0x94748 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x9474c | load | 1 | `r0 +0x0` | `-` |
| `ldrh` at 0x94754 | load | 2 | `sp -0x16` | `-` |
| `strh` at 0x9475c | store | 2 | `sp -0x16` | `?` |
| `ldrh` at 0x94760 | load | 2 | `sp -0x16` | `-` |
| `ldmdb` at 0x94768 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x94768 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x94768 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x94768 | `r0` | `[sp-22]:2` |

## `IcmbCvtToMsg`

- `IcmbCvtToMsg`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9476c to 0x947b0.
  Established. A halfword store saves `(r1 & 0xffff)` at `sp -0x16` (`strh` at 0x94784). Byte stores
  write `([sp-22]:1 & 0xff)` at entry `r0 +0x0` and unknown value at `[sp-20] +0x1` (`strb` at
  0x94790, `strb` at 0x947a8). Entry r0 remains the reported return value (`ldmdb` at 0x947ac). No
  calls or conditional branches are reported, and the field’s byte order is unproven by these facts.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9477c | store | 4 | `sp -0x14` | `r0` |
| `strh` at 0x94784 | store | 2 | `sp -0x16` | `(r1 & 0xffff)` |
| `ldr` at 0x94788 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x9478c | load | 1 | `sp -0x16` | `-` |
| `strb` at 0x94790 | store | 1 | `r0 +0x0` | `([sp-22]:1 & 0xff)` |
| `ldr` at 0x94798 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x947a0 | load | 2 | `sp -0x16` | `-` |
| `strb` at 0x947a8 | store | 1 | `[sp-20] +0x1` | `?` |
| `ldmdb` at 0x947ac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x947ac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x947ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x947ac | `r0` | `r0` |

## `IcmbMsgRqBuild`

- `IcmbMsgRqBuild`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x949a8 to 0x94a24.
  Established. The first call targets `0x947e8` with `[sp]`, `0x0`, entry r2 and `r3+2` (`bl` at
  0x949e0). A later call to `0x948ec` carries `[sp]`, `[sp-24]`, unknown, unknown (`bl` at 0x949fc).
  The final call to `0x18ae8` uses `[sp]+2`, `[sp-28]`, `[sp-32]`, `[sp]+2` (`bl` at 0x94a14). Word
  `r3+2` was saved at `sp -0x24`, whose loaded value reaches the return (`str` at 0x949d4, `ldmdb`
  at 0x94a20). Request-message interpretation is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x949b8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x949bc | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x949c0 | store | 4 | `sp -0x20` | `r3` |
| `strb` at 0x949c8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x949cc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x949d4 | store | 4 | `sp -0x24` | `r3+2` |
| `ldr` at 0x949d8 | load | 4 | `sp +0x0` | `-` |
| `ldrb` at 0x949e4 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x949e8 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x949f4 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x949f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x94a00 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x94a0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x94a10 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x94a18 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x94a20 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x94a20 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x94a20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x949e0 | `(unknown callee) 0x947e8` | `[sp]` | `0x0` | `r2` | `r3+2` |
| `bl` at 0x949f0 | `(unknown callee) 0x9486c` | `[sp]` | `[sp-17]:1` | `?` | `[sp-17]:1` |
| `bl` at 0x949fc | `(unknown callee) 0x948ec` | `[sp]` | `[sp-24]` | `?` | `?` |
| `bl` at 0x94a14 | `(unknown callee) 0x18ae8` | `[sp]+2` | `[sp-28]` | `[sp-32]` | `[sp]+2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x94a20 | `r0` | `[sp-36]` |

## `ipmiCheckCSum`

- `ipmiCheckCSum`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1ed68 to 0x1eddc.
  Established. The stack byte `sp -0x19` starts at `0x0` and later receives unknown
  value (`strb` at 0x1ed84, `strb` at 0x1edc0). The loop branch retains `Z == 0` as its taken
  predicate and falls through otherwise (`bne` at 0x1eda0). Word stores at `sp -0x18` and `sp -0x14`
  have unknown values within that loop (`str` at 0x1ed98, `str` at 0x1edc8). No calls are reported.
  Checksum arithmetic and the returned r0 value remain unknown in these facts (`ldmdb` at 0x1edd8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1ed78 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x1ed7c | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x1ed84 | store | 1 | `sp -0x19` | `0x0` |
| `ldr` at 0x1ed8c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x1ed98 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x1edac | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x1edb4 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0x1edb8 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x1edc0 | store | 1 | `sp -0x19` | `?` |
| `str` at 0x1edc8 | store | 4 | `sp -0x14` | `?` |
| `ldrb` at 0x1edd0 | load | 1 | `sp -0x19` | `-` |
| `ldmdb` at 0x1edd8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x1edd8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x1edd8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x1eda0 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x1edd8 | `r0` | `?` |

## `ipmiCalcCSum`

- `ipmiCalcCSum`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1eddc to 0x1ee54.
  Established. A byte store clears `sp -0x19` (`strb` at 0x1edf8). The loop branch takes its edge
  for `Z == 0` and falls through for `not (Z == 0)` (`bne` at 0x1ee0c). Subsequent stores at `sp
  -0x19` and `sp -0x14` have unknown byte and word values, respectively (`strb` at 0x1ee2c, `str` at
  0x1ee34). The return reports unknown r0 (`ldmdb` at 0x1ee50). No calls are reported, and
  checksum-generation semantics are not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1edec | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x1edf0 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x1edf8 | store | 1 | `sp -0x19` | `0x0` |
| `ldr` at 0x1edfc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x1ee04 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x1ee18 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x1ee20 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0x1ee24 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x1ee2c | store | 1 | `sp -0x19` | `?` |
| `str` at 0x1ee34 | store | 4 | `sp -0x14` | `?` |
| `ldrb` at 0x1ee3c | load | 1 | `sp -0x19` | `-` |
| `ldmdb` at 0x1ee50 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x1ee50 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x1ee50 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x1ee0c | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x1ee50 | `r0` | `?` |
