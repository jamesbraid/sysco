# ARM scan setup and command paths in 1.34.2

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
Indirect calls can have unknown targets while retaining their continuation and argument candidates. No unresolved computed transfers were reported for these routines. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/scan-setup-commands-facts.txt) retain all tool sections.

## `setup_tdrdata_shub`

- `setup_tdrdata_shub`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcf9e0 to 0xcffe8.
  Established. The call to `0x18920` has candidates `0x3c` followed by three unknown values (`bl` at
  0xcfa0c); its result is stored as a word at `[sp-24] +0x1c` (`str` at 0xcfa14). Calls to `0xce934`
  include `0x10bc4c`, `0x7c`, `0x1164b0`, unknown and later `0x10bcf8`, `0x63`, `0x116600`, unknown
  (`bl` at 0xcfa80, `bl` at 0xcff18). The last non-stack word store writes `[sp-32]+1` at
  `[sp-24] +0x18` (`str` at 0xcff68). r0 returns `[sp-36]` (`ldmdb` at 0xcff78). The register-data layout’s
  hardware meaning is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcf9f0 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xcf9f8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xcf9fc | load | 4 | `abs 0xcff7c` | `-` |
| `ldr` at 0xcfa04 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xcfa14 | store | 4 | `[sp-24] +0x1c` | `ret(0x18920)` |
| `ldr` at 0xcfa18 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xcfa28 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xcfa34 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xcfa38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcfa48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcfa4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcfa54 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xcfa60 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xcfa64 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcfa6c | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xcfa74 | load | 4 | `abs 0xcff80` | `-` |
| `ldr` at 0xcfa7c | load | 4 | `abs 0xcff84` | `-` |
| `ldr` at 0xcfa88 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcfa90 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcfa9c | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xcfaa4 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xcfaac | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xcfab0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcfac0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xcfac8 | load | 4 | `abs 0xcff88` | `-` |
| `ldr` at 0xcfad0 | load | 4 | `abs 0xcff8c` | `-` |
| `ldr` at 0xcfadc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcfae4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcfaf0 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xcfaf8 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xcfb00 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xcfb04 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcfb14 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xcfb1c | load | 4 | `abs 0xcff90` | `-` |
| `ldr` at 0xcfb24 | load | 4 | `abs 0xcff94` | `-` |
| `ldr` at 0xcfb30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcfb38 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcfb44 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xcfb4c | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xcfb54 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xcfb58 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcfb68 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xcfb70 | load | 4 | `abs 0xcff98` | `-` |
| `ldr` at 0xcfb78 | load | 4 | `abs 0xcff9c` | `-` |
| `ldr` at 0xcfb84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcfb8c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcfb98 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xcfba0 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xcfba8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xcfbac | load | 4 | `(unknown)` | `-` |
| `str` at 0xcfbbc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xcfbc4 | load | 4 | `abs 0xcffa0` | `-` |
| `ldr` at 0xcfbcc | load | 4 | `abs 0xcffa4` | `-` |
| `ldr` at 0xcfbd8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcfbe0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcfbec | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xcfbf4 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xcfbfc | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xcfc00 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcfc10 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xcfc18 | load | 4 | `abs 0xcffa8` | `-` |
| `ldr` at 0xcfc20 | load | 4 | `abs 0xcffac` | `-` |
| `ldr` at 0xcfc2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcfc34 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcfc40 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xcfc48 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xcfc50 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xcfc54 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcfc64 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xcfc6c | load | 4 | `abs 0xcffb0` | `-` |
| `ldr` at 0xcfc74 | load | 4 | `abs 0xcffb4` | `-` |
| `ldr` at 0xcfc80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcfc88 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcfc94 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xcfc9c | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xcfca4 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xcfca8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcfcb8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xcfcc0 | load | 4 | `abs 0xcffb8` | `-` |
| `ldr` at 0xcfcc8 | load | 4 | `abs 0xcffbc` | `-` |
| `ldr` at 0xcfcd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcfcdc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcfce8 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xcfcf0 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xcfcf8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xcfcfc | load | 4 | `(unknown)` | `-` |
| `str` at 0xcfd0c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xcfd14 | load | 4 | `abs 0xcffc0` | `-` |
| `ldr` at 0xcfd1c | load | 4 | `abs 0xcffbc` | `-` |
| `ldr` at 0xcfd28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcfd30 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcfd3c | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xcfd44 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xcfd4c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xcfd50 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcfd60 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xcfd68 | load | 4 | `abs 0xcffc4` | `-` |
| `ldr` at 0xcfd70 | load | 4 | `abs 0xcffc8` | `-` |
| `ldr` at 0xcfd7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcfd84 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcfd90 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xcfd98 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xcfda0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xcfda4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcfdb4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xcfdbc | load | 4 | `abs 0xcffcc` | `-` |
| `ldr` at 0xcfdc4 | load | 4 | `abs 0xcffc8` | `-` |
| `ldr` at 0xcfdd0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcfdd8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcfde4 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xcfdec | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xcfdf4 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xcfdf8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcfe08 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xcfe10 | load | 4 | `abs 0xcffd0` | `-` |
| `ldr` at 0xcfe18 | load | 4 | `abs 0xcffd4` | `-` |
| `ldr` at 0xcfe24 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcfe2c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcfe38 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xcfe40 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xcfe48 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xcfe4c | load | 4 | `(unknown)` | `-` |
| `str` at 0xcfe5c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xcfe64 | load | 4 | `abs 0xcffd8` | `-` |
| `ldr` at 0xcfe6c | load | 4 | `abs 0xcffd4` | `-` |
| `ldr` at 0xcfe78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcfe80 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcfe8c | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xcfe94 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xcfe9c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xcfea0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcfeb0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xcfeb8 | load | 4 | `abs 0xcffdc` | `-` |
| `ldr` at 0xcfec0 | load | 4 | `abs 0xcffe0` | `-` |
| `ldr` at 0xcfecc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcfed4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcfee0 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xcfee8 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xcfef0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xcfef4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcff04 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xcff0c | load | 4 | `abs 0xcffe4` | `-` |
| `ldr` at 0xcff14 | load | 4 | `abs 0xcffe0` | `-` |
| `ldr` at 0xcff20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcff28 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcff34 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xcff3c | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xcff44 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xcff48 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcff58 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xcff60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcff64 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xcff68 | store | 4 | `[sp-24] +0x18` | `[sp-32]+1` |
| `str` at 0xcff70 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xcff74 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xcff78 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xcff78 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcff78 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcff78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcfa00 | `(unknown callee) 0x20f14` | `0x10bc34` | `r1` | `r2` | `0x0` |
| `bl` at 0xcfa0c | `(unknown callee) 0x18920` | `0x3c` | `?` | `?` | `?` |
| `bl` at 0xcfa80 | `(unknown callee) 0xce934` | `0x10bc4c` | `0x7c` | `0x1164b0` | `?` |
| `bl` at 0xcfad4 | `(unknown callee) 0xce934` | `0x10bc58` | `0x7d` | `0x1164c4` | `?` |
| `bl` at 0xcfb28 | `(unknown callee) 0xce934` | `0x10bc64` | `0x7f` | `0x1164d8` | `?` |
| `bl` at 0xcfb7c | `(unknown callee) 0xce934` | `0x10bc6c` | `0x7e` | `0x1158e8` | `?` |
| `bl` at 0xcfbd0 | `(unknown callee) 0xce934` | `0x10bc74` | `0x7` | `0x11594c` | `?` |
| `bl` at 0xcfc24 | `(unknown callee) 0xce934` | `0x10bc7c` | `0x5` | `0x115978` | `?` |
| `bl` at 0xcfc78 | `(unknown callee) 0xce934` | `0x10bc88` | `0x6` | `0x115aa4` | `?` |
| `bl` at 0xcfccc | `(unknown callee) 0xce934` | `0x10bc90` | `0xa` | `0x115b18` | `?` |
| `bl` at 0xcfd20 | `(unknown callee) 0xce934` | `0x10bc9c` | `0xb` | `0x115b18` | `?` |
| `bl` at 0xcfd74 | `(unknown callee) 0xce934` | `0x10bca8` | `0x1a` | `0x1165ac` | `?` |
| `bl` at 0xcfdc8 | `(unknown callee) 0xce934` | `0x10bcb8` | `0x1b` | `0x1165ac` | `?` |
| `bl` at 0xcfe1c | `(unknown callee) 0xce934` | `0x10bcc8` | `0x60` | `0x115b6c` | `?` |
| `bl` at 0xcfe70 | `(unknown callee) 0xce934` | `0x10bcd8` | `0x62` | `0x115b6c` | `?` |
| `bl` at 0xcfec4 | `(unknown callee) 0xce934` | `0x10bce8` | `0x61` | `0x116600` | `?` |
| `bl` at 0xcff18 | `(unknown callee) 0xce934` | `0x10bcf8` | `0x63` | `0x116600` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcfa20 | `[[sp-24]+28] != 0x0` | `[[sp-24]+28] == 0x0` |
| `bls` at 0xcfa40 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xcfab8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xcfb0c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xcfb60 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xcfbb4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xcfc08 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xcfc5c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xcfcb0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xcfd04 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xcfd58 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xcfdac | `? != 0x0` | `? == 0x0` |
| `bne` at 0xcfe00 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xcfe54 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xcfea8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xcfefc | `? != 0x0` | `? == 0x0` |
| `bne` at 0xcff50 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcff78 | `r0` | `[sp-36]` |

## `setup_tdrdata_tio`

- `setup_tdrdata_tio`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcffe8 to 0xd06a8.
  Established. The numeric call to `0x18920` carries `0x44`, unknown, unknown, unknown (`bl` at
  0xd0014). A word store then puts `ret(0x18920)` at `[sp-24] +0x1c` (`str` at 0xd001c); its nonzero
  load takes the continuation edge, while zero falls through to a stack store whose value is
  unknown (`bne` at 0xd0028, `str` at 0xd0030). Later calls to `0xce934` include candidates
  `0x10c354`, `0x12`, `0x1164ec`, unknown and `0x10c364`, `0x11`, `0x116500`, unknown (`bl` at
  0xd02d4, `bl` at 0xd0328). The return reports `[sp-36]`, with the physical setup effect unproven
  (`ldmdb` at 0xd0628).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcfff8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xd0000 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xd0004 | load | 4 | `abs 0xd062c` | `-` |
| `ldr` at 0xd000c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd001c | store | 4 | `[sp-24] +0x1c` | `ret(0x18920)` |
| `ldr` at 0xd0020 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd0030 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xd003c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xd0040 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd0050 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd0054 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd005c | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd0068 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xd006c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd0074 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xd007c | load | 4 | `abs 0xd0630` | `-` |
| `ldr` at 0xd0084 | load | 4 | `abs 0xd0634` | `-` |
| `ldr` at 0xd0090 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd0098 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd00a4 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd00ac | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xd00b4 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xd00b8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd00c8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd00d0 | load | 4 | `abs 0xd0638` | `-` |
| `ldr` at 0xd00d8 | load | 4 | `abs 0xd063c` | `-` |
| `ldr` at 0xd00e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd00ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd00f8 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd0100 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xd0108 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xd010c | load | 4 | `(unknown)` | `-` |
| `str` at 0xd011c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd0124 | load | 4 | `abs 0xd0640` | `-` |
| `ldr` at 0xd012c | load | 4 | `abs 0xd0644` | `-` |
| `ldr` at 0xd0138 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd0140 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd014c | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd0154 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xd015c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xd0160 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd0170 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd0178 | load | 4 | `abs 0xd0648` | `-` |
| `ldr` at 0xd0180 | load | 4 | `abs 0xd064c` | `-` |
| `ldr` at 0xd018c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd0194 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd01a0 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd01a8 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xd01b0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xd01b4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd01c4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd01cc | load | 4 | `abs 0xd0650` | `-` |
| `ldr` at 0xd01d4 | load | 4 | `abs 0xd0654` | `-` |
| `ldr` at 0xd01e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd01e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd01f4 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd01fc | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xd0204 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xd0208 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd0218 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd0220 | load | 4 | `abs 0xd0658` | `-` |
| `ldr` at 0xd0228 | load | 4 | `abs 0xd065c` | `-` |
| `ldr` at 0xd0234 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd023c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd0248 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd0250 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xd0258 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xd025c | load | 4 | `(unknown)` | `-` |
| `str` at 0xd026c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd0274 | load | 4 | `abs 0xd0660` | `-` |
| `ldr` at 0xd027c | load | 4 | `abs 0xd0664` | `-` |
| `ldr` at 0xd0288 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd0290 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd029c | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd02a4 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xd02ac | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xd02b0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd02c0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd02c8 | load | 4 | `abs 0xd0668` | `-` |
| `ldr` at 0xd02d0 | load | 4 | `abs 0xd066c` | `-` |
| `ldr` at 0xd02dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd02e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd02f0 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd02f8 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xd0300 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xd0304 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd0314 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd031c | load | 4 | `abs 0xd0670` | `-` |
| `ldr` at 0xd0324 | load | 4 | `abs 0xd0674` | `-` |
| `ldr` at 0xd0330 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd0338 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd0344 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd034c | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xd0354 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xd0358 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd0368 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd0370 | load | 4 | `abs 0xd0678` | `-` |
| `ldr` at 0xd0378 | load | 4 | `abs 0xd067c` | `-` |
| `ldr` at 0xd0384 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd038c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd0398 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd03a0 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xd03a8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xd03ac | load | 4 | `(unknown)` | `-` |
| `str` at 0xd03bc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd03c4 | load | 4 | `abs 0xd0680` | `-` |
| `ldr` at 0xd03cc | load | 4 | `abs 0xd067c` | `-` |
| `ldr` at 0xd03d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd03e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd03ec | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd03f4 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xd03fc | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xd0400 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd0410 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd0418 | load | 4 | `abs 0xd0684` | `-` |
| `ldr` at 0xd0420 | load | 4 | `abs 0xd0688` | `-` |
| `ldr` at 0xd042c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd0434 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd0440 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd0448 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xd0450 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xd0454 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd0464 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd046c | load | 4 | `abs 0xd068c` | `-` |
| `ldr` at 0xd0474 | load | 4 | `abs 0xd0688` | `-` |
| `ldr` at 0xd0480 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd0488 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd0494 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd049c | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xd04a4 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xd04a8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd04b8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd04c0 | load | 4 | `abs 0xd0690` | `-` |
| `ldr` at 0xd04c8 | load | 4 | `abs 0xd0694` | `-` |
| `ldr` at 0xd04d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd04dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd04e8 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd04f0 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xd04f8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xd04fc | load | 4 | `(unknown)` | `-` |
| `str` at 0xd050c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd0514 | load | 4 | `abs 0xd0698` | `-` |
| `ldr` at 0xd051c | load | 4 | `abs 0xd0694` | `-` |
| `ldr` at 0xd0528 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd0530 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd053c | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd0544 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xd054c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xd0550 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd0560 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd0568 | load | 4 | `abs 0xd069c` | `-` |
| `ldr` at 0xd0570 | load | 4 | `abs 0xd06a0` | `-` |
| `ldr` at 0xd057c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd0584 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd0590 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd0598 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xd05a0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xd05a4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd05b4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd05bc | load | 4 | `abs 0xd06a4` | `-` |
| `ldr` at 0xd05c4 | load | 4 | `abs 0xd06a0` | `-` |
| `ldr` at 0xd05d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd05d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd05e4 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd05ec | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xd05f4 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xd05f8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd0608 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd0610 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd0614 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd0618 | store | 4 | `[sp-24] +0x18` | `[sp-32]+1` |
| `str` at 0xd0620 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xd0624 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xd0628 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd0628 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd0628 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd0628 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd0008 | `(unknown callee) 0x20f14` | `0x10c2f8` | `r1` | `r2` | `0x0` |
| `bl` at 0xd0014 | `(unknown callee) 0x18920` | `0x44` | `?` | `?` | `?` |
| `bl` at 0xd0088 | `(unknown callee) 0xce934` | `0x10c310` | `0x7c` | `0x1164b0` | `?` |
| `bl` at 0xd00dc | `(unknown callee) 0xce934` | `0x10c31c` | `0x7d` | `0x1164c4` | `?` |
| `bl` at 0xd0130 | `(unknown callee) 0xce934` | `0x10c328` | `0x7f` | `0x1164d8` | `?` |
| `bl` at 0xd0184 | `(unknown callee) 0xce934` | `0x10c330` | `0x7e` | `0x115bf4` | `?` |
| `bl` at 0xd01d8 | `(unknown callee) 0xce934` | `0x10c338` | `0x7` | `0x115c80` | `?` |
| `bl` at 0xd022c | `(unknown callee) 0xce934` | `0x10c340` | `0x5` | `0x115cac` | `?` |
| `bl` at 0xd0280 | `(unknown callee) 0xce934` | `0x10c34c` | `0x6` | `0x115df8` | `?` |
| `bl` at 0xd02d4 | `(unknown callee) 0xce934` | `0x10c354` | `0x12` | `0x1164ec` | `?` |
| `bl` at 0xd0328 | `(unknown callee) 0xce934` | `0x10c364` | `0x11` | `0x116500` | `?` |
| `bl` at 0xd037c | `(unknown callee) 0xce934` | `0x10c374` | `0xa` | `0x115e94` | `?` |
| `bl` at 0xd03d0 | `(unknown callee) 0xce934` | `0x10c380` | `0xb` | `0x115e94` | `?` |
| `bl` at 0xd0424 | `(unknown callee) 0xce934` | `0x10c38c` | `0x1a` | `0x1165ac` | `?` |
| `bl` at 0xd0478 | `(unknown callee) 0xce934` | `0x10c39c` | `0x1b` | `0x1165ac` | `?` |
| `bl` at 0xd04cc | `(unknown callee) 0xce934` | `0x10c3ac` | `0x60` | `0x115f04` | `?` |
| `bl` at 0xd0520 | `(unknown callee) 0xce934` | `0x10c3bc` | `0x62` | `0x115f04` | `?` |
| `bl` at 0xd0574 | `(unknown callee) 0xce934` | `0x10c3cc` | `0x61` | `0x116600` | `?` |
| `bl` at 0xd05c8 | `(unknown callee) 0xce934` | `0x10c3dc` | `0x63` | `0x116600` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd0028 | `[[sp-24]+28] != 0x0` | `[[sp-24]+28] == 0x0` |
| `bls` at 0xd0048 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xd00c0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd0114 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd0168 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd01bc | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd0210 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd0264 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd02b8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd030c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd0360 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd03b4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd0408 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd045c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd04b0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd0504 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd0558 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd05ac | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd0600 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd0628 | `r0` | `[sp-36]` |

## `setup_tdrdata_nl4r`

- `setup_tdrdata_nl4r`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd06a8 to 0xd0e10.
  Established. An initial stack word receives `0x10ce84` at `sp -0x44` (`str` at 0xd06c8). The call
  to `0x18920` uses `0xa8` and three unknown candidates, with its result stored at `[sp-24] +0x1c`
  (`bl` at 0xd06dc, `str` at 0xd06e4). Later `0xce934` calls include `sp-64`, `[sp-36]:2`,
  `0x116404`, `sp-64` and `sp-64`, `[sp-36]:2`, `0x116600`, `sp-64` (`bl` at 0xd0ae4, `bl` at
  0xd0d18). The final non-stack word store writes `[sp-32]+1` at `[sp-24] +0x18` (`str` at 0xd0d84).
  The reported r0 return is `[sp-72]` (`ldmdb` at 0xd0d94). The selected data’s device semantics
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd06b8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xd06c0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xd06c4 | load | 4 | `abs 0xd0d98` | `-` |
| `str` at 0xd06c8 | store | 4 | `sp -0x44` | `0x10ce84` |
| `ldr` at 0xd06cc | load | 4 | `abs 0xd0d9c` | `-` |
| `ldr` at 0xd06d4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd06e4 | store | 4 | `[sp-24] +0x1c` | `ret(0x18920)` |
| `ldr` at 0xd06e8 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd06f8 | store | 4 | `sp -0x48` | `?` |
| `str` at 0xd0704 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xd0708 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd0718 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd071c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd0724 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd0730 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xd0734 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd073c | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xd0744 | load | 4 | `abs 0xd0da0` | `-` |
| `ldr` at 0xd074c | load | 4 | `abs 0xd0da4` | `-` |
| `ldr` at 0xd0758 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd0760 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd076c | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd0774 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xd077c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xd0780 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd0790 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xd0798 | load | 4 | `abs 0xd0da8` | `-` |
| `ldr` at 0xd07a0 | load | 4 | `abs 0xd0dac` | `-` |
| `ldr` at 0xd07ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd07b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd07c0 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd07c8 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xd07d0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xd07d4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd07e4 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xd07ec | load | 4 | `abs 0xd0db0` | `-` |
| `ldr` at 0xd07f4 | load | 4 | `abs 0xd0db4` | `-` |
| `ldr` at 0xd0800 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd0808 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd0814 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd081c | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xd0824 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xd0828 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd0838 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xd0840 | load | 4 | `abs 0xd0db8` | `-` |
| `ldr` at 0xd0848 | load | 4 | `abs 0xd0dbc` | `-` |
| `ldr` at 0xd0854 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd085c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd0868 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd0870 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xd0878 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xd087c | load | 4 | `(unknown)` | `-` |
| `str` at 0xd088c | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xd0894 | load | 4 | `abs 0xd0dc0` | `-` |
| `ldr` at 0xd089c | load | 4 | `abs 0xd0dc4` | `-` |
| `ldr` at 0xd08a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd08b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd08bc | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd08c4 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xd08cc | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xd08d0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd08e0 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xd08e8 | load | 4 | `abs 0xd0dc8` | `-` |
| `ldr` at 0xd08f0 | load | 4 | `abs 0xd0dcc` | `-` |
| `ldr` at 0xd08fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd0904 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd0910 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd0918 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xd0920 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xd0924 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd0934 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xd093c | load | 4 | `abs 0xd0dd0` | `-` |
| `ldr` at 0xd0944 | load | 4 | `abs 0xd0dd4` | `-` |
| `ldr` at 0xd0950 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd0958 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd0964 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd096c | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xd0974 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xd0978 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd0988 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xd0990 | load | 4 | `abs 0xd0dd8` | `-` |
| `ldr` at 0xd0998 | load | 4 | `abs 0xd0ddc` | `-` |
| `ldr` at 0xd09a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd09ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd09b8 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd09c0 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xd09c8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xd09cc | load | 4 | `(unknown)` | `-` |
| `str` at 0xd09dc | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xd09e4 | load | 4 | `abs 0xd0de0` | `-` |
| `ldr` at 0xd09ec | load | 4 | `abs 0xd0de4` | `-` |
| `ldr` at 0xd09f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd0a00 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd0a0c | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd0a14 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xd0a1c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xd0a20 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd0a30 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xd0a38 | load | 4 | `abs 0xd0de8` | `-` |
| `ldr` at 0xd0a40 | load | 4 | `abs 0xd0dec` | `-` |
| `ldr` at 0xd0a4c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd0a54 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd0a60 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd0a68 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xd0a70 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xd0a74 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd0a84 | store | 4 | `sp -0x48` | `?` |
| `str` at 0xd0a90 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xd0a98 | store | 4 | `sp -0x24` | `0x10` |
| `ldr` at 0xd0a9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd0ab0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd0ab4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd0abc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xd0ac4 | load | 4 | `abs 0xd0df0` | `-` |
| `ldrh` at 0xd0ad4 | load | 2 | `sp -0x24` | `-` |
| `ldr` at 0xd0ae0 | load | 4 | `abs 0xd0df4` | `-` |
| `ldr` at 0xd0aec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd0af4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd0b00 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd0b08 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xd0b10 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xd0b14 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd0b24 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xd0b2c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd0b34 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xd0b38 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd0b40 | store | 4 | `sp -0x24` | `[sp-36]+2` |
| `str` at 0xd0b4c | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xd0b54 | store | 4 | `sp -0x24` | `0x11` |
| `ldr` at 0xd0b58 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd0b6c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd0b70 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd0b78 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xd0b80 | load | 4 | `abs 0xd0df8` | `-` |
| `ldrh` at 0xd0b90 | load | 2 | `sp -0x24` | `-` |
| `ldr` at 0xd0b9c | load | 4 | `abs 0xd0dfc` | `-` |
| `ldr` at 0xd0ba8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd0bb0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd0bbc | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd0bc4 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xd0bcc | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xd0bd0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd0be0 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xd0be8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd0bf0 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xd0bf4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd0bfc | store | 4 | `sp -0x24` | `[sp-36]+2` |
| `str` at 0xd0c08 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xd0c10 | store | 4 | `sp -0x24` | `0x60` |
| `ldr` at 0xd0c14 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd0c28 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd0c2c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd0c34 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xd0c3c | load | 4 | `abs 0xd0e00` | `-` |
| `ldrh` at 0xd0c4c | load | 2 | `sp -0x24` | `-` |
| `ldr` at 0xd0c58 | load | 4 | `abs 0xd0e04` | `-` |
| `ldr` at 0xd0c64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd0c6c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd0c78 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd0c80 | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xd0c88 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xd0c8c | load | 4 | `(unknown)` | `-` |
| `str` at 0xd0c9c | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xd0ca4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd0cac | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xd0cb0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd0cb8 | store | 4 | `sp -0x24` | `[sp-36]+2` |
| `str` at 0xd0cc4 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xd0ccc | store | 4 | `sp -0x24` | `0x61` |
| `ldr` at 0xd0cd0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd0ce4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd0ce8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd0cf0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xd0cf8 | load | 4 | `abs 0xd0e08` | `-` |
| `ldrh` at 0xd0d08 | load | 2 | `sp -0x24` | `-` |
| `ldr` at 0xd0d14 | load | 4 | `abs 0xd0e0c` | `-` |
| `ldr` at 0xd0d20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd0d28 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd0d34 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xd0d3c | store | 4 | `(unknown)` | `ret(0xce934)` |
| `str` at 0xd0d44 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xd0d48 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd0d58 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xd0d60 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd0d68 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xd0d6c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd0d74 | store | 4 | `sp -0x24` | `[sp-36]+2` |
| `ldr` at 0xd0d7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd0d80 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd0d84 | store | 4 | `[sp-24] +0x18` | `[sp-32]+1` |
| `str` at 0xd0d8c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xd0d90 | load | 4 | `sp -0x48` | `-` |
| `ldmdb` at 0xd0d94 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd0d94 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd0d94 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd0d94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd06d0 | `(unknown callee) 0x20f14` | `0x10ce90` | `r1` | `r2` | `0x10ce84` |
| `bl` at 0xd06dc | `(unknown callee) 0x18920` | `0xa8` | `?` | `?` | `?` |
| `bl` at 0xd0750 | `(unknown callee) 0xce934` | `0x10cea8` | `0x7c` | `0x1164b0` | `?` |
| `bl` at 0xd07a4 | `(unknown callee) 0xce934` | `0x10ceb4` | `0x7d` | `0x1164c4` | `?` |
| `bl` at 0xd07f8 | `(unknown callee) 0xce934` | `0x10cec0` | `0x7f` | `0x1164d8` | `?` |
| `bl` at 0xd084c | `(unknown callee) 0xce934` | `0x10cec8` | `0x7e` | `0x115f20` | `?` |
| `bl` at 0xd08a0 | `(unknown callee) 0xce934` | `0x10ced0` | `0x7` | `0x115f6c` | `?` |
| `bl` at 0xd08f4 | `(unknown callee) 0xce934` | `0x10ced8` | `0x5` | `0x115f98` | `?` |
| `bl` at 0xd0948 | `(unknown callee) 0xce934` | `0x10cee4` | `0x6` | `0x1162a4` | `?` |
| `bl` at 0xd099c | `(unknown callee) 0xce934` | `0x10ceec` | `0xd` | `0x1164ec` | `?` |
| `bl` at 0xd09f0 | `(unknown callee) 0xce934` | `0x10cefc` | `0xc` | `0x116500` | `?` |
| `bl` at 0xd0a44 | `(unknown callee) 0xce934` | `0x10cf0c` | `0x4b` | `0x116618` | `?` |
| `bl` at 0xd0acc | `(unknown callee) 0x18c74` | `sp-64` | `0x10cf18` | `?` | `?` |
| `bl` at 0xd0ae4 | `(unknown callee) 0xce934` | `sp-64` | `[sp-36]:2` | `0x116404` | `sp-64` |
| `bl` at 0xd0b88 | `(unknown callee) 0x18c74` | `sp-64` | `0x10cf20` | `?` | `?` |
| `bl` at 0xd0ba0 | `(unknown callee) 0xce934` | `sp-64` | `[sp-36]:2` | `0x1165ac` | `sp-64` |
| `bl` at 0xd0c44 | `(unknown callee) 0x18c74` | `sp-64` | `0x10cf2c` | `?` | `?` |
| `bl` at 0xd0c5c | `(unknown callee) 0xce934` | `sp-64` | `[sp-36]:2` | `0x11649c` | `sp-64` |
| `bl` at 0xd0d00 | `(unknown callee) 0x18c74` | `sp-64` | `0x10cf3c` | `?` | `?` |
| `bl` at 0xd0d18 | `(unknown callee) 0xce934` | `sp-64` | `[sp-36]:2` | `0x116600` | `sp-64` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd06f0 | `[[sp-24]+28] != 0x0` | `[[sp-24]+28] == 0x0` |
| `bls` at 0xd0710 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xd0788 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd07dc | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd0830 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd0884 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd08d8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd092c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd0980 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd09d4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd0a28 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd0a7c | `? != 0x0` | `? == 0x0` |
| `bls` at 0xd0aa4 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xd0b1c | `? != 0x0` | `? == 0x0` |
| `bls` at 0xd0b60 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xd0bd8 | `? != 0x0` | `? == 0x0` |
| `bls` at 0xd0c1c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xd0c94 | `? != 0x0` | `? == 0x0` |
| `bls` at 0xd0cd8 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xd0d50 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd0d94 | `r0` | `[sp-72]` |

## `llpcmd_to_tdr`

- `llpcmd_to_tdr`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd1aec to 0xd1f44.
  Established. The calls all target `0xcb020`, beginning with candidates `[sp-20]`, `[sp-24]`,
  `0x0`, `sp-36` (`bl` at 0xd1b40). Later calls include the same first, second and fourth candidates
  with third values `0xf` and `0xc` (`bl` at 0xd1e94, `bl` at 0xd1f3c). Word stores include
  `[[sp-28]+20]` and `[[sp-28]+40]`, both at unknown effective targets (`str` at 0xd1d8c, `str` at
  0xd1f28). No conditional branches are reported. The return reports `ret(0xcb020)`, and the numeric
  selectors’ hardware meanings are unknown (`ldmdb` at 0xd1f40).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd1afc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd1b00 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xd1b04 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xd1b18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1b20 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xd1b2c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd1b34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd1b38 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd1b54 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xd1b68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1b70 | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0xd1b7c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd1b98 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd1ba8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1bb0 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0xd1bc0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd1bc8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd1bcc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1be8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1bf0 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xd1bfc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd1c04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd1c08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1c24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1c2c | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0xd1c34 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd1c3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd1c40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1c5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1c64 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xd1c70 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd1c78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd1c7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1c98 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1ca0 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xd1ca8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd1cb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd1cb4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1cd0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1cd8 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0xd1ce4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd1cec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd1cf0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1d0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1d14 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0xd1d20 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd1d28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd1d2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1d48 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1d50 | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0xd1d58 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd1d60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd1d64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1d80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1d88 | load | 4 | `[sp-28] +0x14` | `-` |
| `str` at 0xd1d8c | store | 4 | `(unknown)` | `[[sp-28]+20]` |
| `ldr` at 0xd1d94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd1d98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1db4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1dbc | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0xd1dc8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd1dd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd1dd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1df0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1df8 | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0xd1e04 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd1e0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd1e10 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1e2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1e34 | load | 4 | `[sp-28] +0x18` | `-` |
| `str` at 0xd1e44 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd1e4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd1e50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1e6c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1e74 | load | 4 | `[sp-28] +0x18` | `-` |
| `str` at 0xd1e80 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd1e88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd1e8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1ea8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1eb0 | load | 4 | `[sp-28] +0x1c` | `-` |
| `str` at 0xd1eb4 | store | 4 | `(unknown)` | `[[sp-28]+28]` |
| `ldr` at 0xd1ec8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1ed0 | load | 4 | `[sp-28] +0x20` | `-` |
| `str` at 0xd1ed4 | store | 4 | `(unknown)` | `[[sp-28]+32]` |
| `ldr` at 0xd1edc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd1ee0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1efc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1f04 | load | 4 | `[sp-28] +0x24` | `-` |
| `str` at 0xd1f08 | store | 4 | `(unknown)` | `[[sp-28]+36]` |
| `ldr` at 0xd1f1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1f24 | load | 4 | `[sp-28] +0x28` | `-` |
| `str` at 0xd1f28 | store | 4 | `(unknown)` | `[[sp-28]+40]` |
| `ldr` at 0xd1f30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd1f34 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xd1f40 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd1f40 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd1f40 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd1b40 | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]` | `0x0` | `sp-36` |
| `bl` at 0xd1bd4 | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]` | `0x1` | `sp-36` |
| `bl` at 0xd1c10 | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]` | `0x2` | `sp-36` |
| `bl` at 0xd1c48 | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]` | `0x3` | `sp-36` |
| `bl` at 0xd1c84 | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]` | `0x4` | `sp-36` |
| `bl` at 0xd1cbc | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]` | `0x5` | `sp-36` |
| `bl` at 0xd1cf8 | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]` | `0x6` | `sp-36` |
| `bl` at 0xd1d34 | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]` | `0x7` | `sp-36` |
| `bl` at 0xd1d6c | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]` | `0x9` | `sp-36` |
| `bl` at 0xd1da0 | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]` | `0x8` | `sp-36` |
| `bl` at 0xd1ddc | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]` | `0xa` | `sp-36` |
| `bl` at 0xd1e18 | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]` | `0xb` | `sp-36` |
| `bl` at 0xd1e58 | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]` | `0xe` | `sp-36` |
| `bl` at 0xd1e94 | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]` | `0xf` | `sp-36` |
| `bl` at 0xd1ee8 | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]` | `0xd` | `sp-36` |
| `bl` at 0xd1f3c | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]` | `0xc` | `sp-36` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd1f40 | `r0` | `ret(0xcb020)` |

## `llpcmd_from_tdr`

- `llpcmd_from_tdr`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd1f44 to 0xd24c4.
  Established. Every call targets `0xcab20`. The first carries entry r0, entry r1, `0x0`, `sp-36`,
  while the last carries `[sp-20]`, `[sp-24]`, `0xc`, `sp-36` (`bl` at 0xd1f70, `bl` at 0xd247c).
  Word stores at `[sp-28] +0x4` and `[sp-28] +0x10` have unknown values (`str` at 0xd1fb0, `str` at
  0xd2014). Another word store writes `[sp-40]` at `[sp-28] +0xc` (`str` at 0xd2044). The return
  reports `sp-16` in r0 (`ldmdb` at 0xd24c0). No conditional branches are reported, and field
  meanings are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd1f54 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd1f58 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xd1f5c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xd1f64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd1f68 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1f78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1f8c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd1f9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1fa4 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xd1fb0 | store | 4 | `[sp-28] +0x4` | `?` |
| `ldr` at 0xd1fb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd1fbc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd1fd4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd1fe0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xd1fe8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1ff0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd2000 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd2008 | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0xd2014 | store | 4 | `[sp-28] +0x10` | `?` |
| `ldr` at 0xd2024 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd2030 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xd2038 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd2040 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd2044 | store | 4 | `[sp-28] +0xc` | `[sp-40]` |
| `ldr` at 0xd204c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd2050 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd2060 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd2074 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd2084 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd208c | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xd2098 | store | 4 | `[sp-28] +0x4` | `?` |
| `ldr` at 0xd20a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd20a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd20b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd20c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd20d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd20dc | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0xd20e8 | store | 4 | `[sp-28] +0x8` | `?` |
| `ldr` at 0xd20f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd20f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd2104 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd2118 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd2128 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd2130 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xd213c | store | 4 | `[sp-28] +0x4` | `?` |
| `ldr` at 0xd2144 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd2148 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd2158 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd216c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd2178 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd2180 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xd218c | store | 4 | `[sp-28] +0x4` | `?` |
| `ldr` at 0xd2194 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd2198 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd21a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd21bc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd21cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd21d4 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0xd21e0 | store | 4 | `[sp-28] +0x8` | `?` |
| `ldr` at 0xd21e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd21ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd21fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd2210 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd2220 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd2228 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0xd2234 | store | 4 | `[sp-28] +0x8` | `?` |
| `ldr` at 0xd223c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd2240 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd2250 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd2264 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd2270 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd2278 | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0xd2284 | store | 4 | `[sp-28] +0x10` | `?` |
| `ldr` at 0xd228c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd2290 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd22a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd22b4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd22b8 | store | 4 | `[sp-28] +0x14` | `?` |
| `ldr` at 0xd22c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd22c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd22d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd22e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd22f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd2300 | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0xd230c | store | 4 | `[sp-28] +0x10` | `?` |
| `ldr` at 0xd2314 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd2318 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd2328 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd233c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd234c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd2354 | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0xd2360 | store | 4 | `[sp-28] +0x10` | `?` |
| `ldr` at 0xd2368 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd236c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd237c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd2388 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd2390 | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0xd23a8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd23bc | store | 4 | `[sp-28] +0x18` | `?` |
| `ldr` at 0xd23c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd23c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd23d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd23e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd23ec | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0xd2404 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd2414 | store | 4 | `[sp-28] +0x18` | `?` |
| `ldr` at 0xd241c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd2420 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd2430 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd2444 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd2448 | store | 4 | `[sp-28] +0x1c` | `?` |
| `ldr` at 0xd2450 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd2464 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd2468 | store | 4 | `[sp-28] +0x20` | `?` |
| `ldr` at 0xd2470 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd2474 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd2484 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd2498 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd249c | store | 4 | `[sp-28] +0x24` | `?` |
| `ldr` at 0xd24a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd24b8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd24bc | store | 4 | `[sp-28] +0x28` | `?` |
| `ldmdb` at 0xd24c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd24c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd24c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd1f70 | `(unknown callee) 0xcab20` | `r0` | `r1` | `0x0` | `sp-36` |
| `bl` at 0xd1fc4 | `(unknown callee) 0xcab20` | `[sp-20]` | `[sp-24]` | `0x1` | `sp-36` |
| `bl` at 0xd2058 | `(unknown callee) 0xcab20` | `[sp-20]` | `[sp-24]` | `0x2` | `sp-36` |
| `bl` at 0xd20ac | `(unknown callee) 0xcab20` | `[sp-20]` | `[sp-24]` | `0x3` | `sp-36` |
| `bl` at 0xd20fc | `(unknown callee) 0xcab20` | `[sp-20]` | `[sp-24]` | `0x4` | `sp-36` |
| `bl` at 0xd2150 | `(unknown callee) 0xcab20` | `[sp-20]` | `[sp-24]` | `0x5` | `sp-36` |
| `bl` at 0xd21a0 | `(unknown callee) 0xcab20` | `[sp-20]` | `[sp-24]` | `0x6` | `sp-36` |
| `bl` at 0xd21f4 | `(unknown callee) 0xcab20` | `[sp-20]` | `[sp-24]` | `0x7` | `sp-36` |
| `bl` at 0xd2248 | `(unknown callee) 0xcab20` | `[sp-20]` | `[sp-24]` | `0x9` | `sp-36` |
| `bl` at 0xd2298 | `(unknown callee) 0xcab20` | `[sp-20]` | `[sp-24]` | `0x8` | `sp-36` |
| `bl` at 0xd22cc | `(unknown callee) 0xcab20` | `[sp-20]` | `[sp-24]` | `0xa` | `sp-36` |
| `bl` at 0xd2320 | `(unknown callee) 0xcab20` | `[sp-20]` | `[sp-24]` | `0xb` | `sp-36` |
| `bl` at 0xd2374 | `(unknown callee) 0xcab20` | `[sp-20]` | `[sp-24]` | `0xe` | `sp-36` |
| `bl` at 0xd23d0 | `(unknown callee) 0xcab20` | `[sp-20]` | `[sp-24]` | `0xf` | `sp-36` |
| `bl` at 0xd2428 | `(unknown callee) 0xcab20` | `[sp-20]` | `[sp-24]` | `0xd` | `sp-36` |
| `bl` at 0xd247c | `(unknown callee) 0xcab20` | `[sp-20]` | `[sp-24]` | `0xc` | `sp-36` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd24c0 | `r0` | `sp-16` |

## `nl3r_LLP_SC_f`

- `nl3r_LLP_SC_f`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd24c4 to 0xd2ab0.
  Established. The initial call to `0xbce44` uses `0x10`, `0x0`, `0x1`, `0x0` (`bl` at 0xd2510); its
  nonzero result takes the continuation and zero falls through to word `0xfffff7c9` at `sp
  -0x3c` (`bne` at 0xd251c, `str` at 0xd2524). The stores include words `0x68000000` and
  `0xe8000000` at unknown targets on separate paths (`str` at 0xd25b0, `str` at 0xd25c8). The first
  call to `0xd2c30` carries `0x0`, `sp-40`, `[sp-44]`, `sp-40` (`bl` at 0xd27a4). The physical link
  operation and merged return are unknown (`ldmdb` at 0xd2aa0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd24d4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd24dc | store | 4 | `sp -0x50` | `0x0` |
| `str` at 0xd24e4 | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0xd24ec | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xd24f4 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xd24f8 | load | 4 | `abs 0xd2aa4` | `-` |
| `str` at 0xd24fc | store | 4 | `sp -0x40` | `0x2842a8` |
| `ldr` at 0xd2520 | load | 4 | `abs 0xd2aa8` | `-` |
| `str` at 0xd2524 | store | 4 | `sp -0x3c` | `0xfffff7c9` |
| `ldr` at 0xd252c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd2534 | store | 4 | `sp -0x2c` | `[sp-20]+16` |
| `ldr` at 0xd2538 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd253c | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xd2548 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd254c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd2550 | load | 4 | `[sp-20] +0x10` | `-` |
| `str` at 0xd2560 | store | 4 | `sp -0x34` | `?` |
| `str` at 0xd2568 | store | 4 | `sp -0x18` | `0x5` |
| `ldr` at 0xd256c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd2578 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd2584 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd2590 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xd25b0 | store | 4 | `(unknown)` | `0x68000000` |
| `str` at 0xd25c8 | store | 4 | `(unknown)` | `0xe8000000` |
| `ldr` at 0xd25cc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd25d0 | store | 4 | `sp -0x50` | `[sp-24]` |
| `str` at 0xd25d8 | store | 4 | `sp -0x4c` | `sp-32` |
| `str` at 0xd25e0 | store | 4 | `sp -0x48` | `sp-40` |
| `ldr` at 0xd25e4 | load | 4 | `abs 0xd2aa4` | `-` |
| `str` at 0xd25e8 | store | 4 | `sp -0x44` | `0x2842a8` |
| `ldr` at 0xd260c | load | 4 | `abs 0xd2aac` | `-` |
| `str` at 0xd2610 | store | 4 | `sp -0x3c` | `0xfffff7ca` |
| `str` at 0xd2620 | store | 4 | `sp -0x50` | `sp-32` |
| `ldr` at 0xd2624 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd2628 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd2634 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd2640 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd2644 | store | 4 | `sp -0x50` | `[sp-24]` |
| `str` at 0xd264c | store | 4 | `sp -0x4c` | `sp-32` |
| `str` at 0xd2654 | store | 4 | `sp -0x48` | `sp-40` |
| `ldr` at 0xd2658 | load | 4 | `abs 0xd2aa4` | `-` |
| `str` at 0xd265c | store | 4 | `sp -0x44` | `0x2842a8` |
| `ldr` at 0xd2680 | load | 4 | `abs 0xd2aac` | `-` |
| `str` at 0xd2684 | store | 4 | `sp -0x3c` | `0xfffff7ca` |
| `ldr` at 0xd268c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd269c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd26a0 | store | 4 | `sp -0x50` | `[sp-24]` |
| `str` at 0xd26a8 | store | 4 | `sp -0x4c` | `sp-32` |
| `str` at 0xd26b0 | store | 4 | `sp -0x48` | `sp-40` |
| `ldr` at 0xd26b4 | load | 4 | `abs 0xd2aa4` | `-` |
| `str` at 0xd26b8 | store | 4 | `sp -0x44` | `0x2842a8` |
| `ldr` at 0xd26dc | load | 4 | `abs 0xd2aac` | `-` |
| `str` at 0xd26e0 | store | 4 | `sp -0x3c` | `0xfffff7ca` |
| `str` at 0xd26ec | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xd26fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd270c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd271c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd2720 | store | 4 | `sp -0x50` | `[sp-24]` |
| `str` at 0xd2728 | store | 4 | `sp -0x4c` | `sp-32` |
| `str` at 0xd2730 | store | 4 | `sp -0x48` | `sp-40` |
| `ldr` at 0xd2734 | load | 4 | `abs 0xd2aa4` | `-` |
| `str` at 0xd2738 | store | 4 | `sp -0x44` | `0x2842a8` |
| `ldr` at 0xd275c | load | 4 | `abs 0xd2aac` | `-` |
| `str` at 0xd2760 | store | 4 | `sp -0x3c` | `0xfffff7ca` |
| `ldr` at 0xd2768 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xd2770 | store | 4 | `sp -0x38` | `[sp-56]+1` |
| `ldr` at 0xd2784 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd27a0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd27a8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd27ac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd27b0 | load | 4 | `[sp-44] +0x0` | `-` |
| `str` at 0xd27c0 | store | 4 | `[sp-44] +0x0` | `?` |
| `ldr` at 0xd27c4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd27c8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd27cc | load | 4 | `[sp-44] +0x0` | `-` |
| `ldr` at 0xd27d8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xd27ec | store | 4 | `[sp-44] +0x0` | `?` |
| `str` at 0xd27f4 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xd27fc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd2818 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd2840 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd2848 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd2868 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd2870 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd2874 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd2878 | store | 4 | `sp -0x50` | `[sp-24]` |
| `str` at 0xd2880 | store | 4 | `sp -0x4c` | `sp-32` |
| `str` at 0xd2888 | store | 4 | `sp -0x48` | `sp-40` |
| `ldr` at 0xd288c | load | 4 | `abs 0xd2aa4` | `-` |
| `str` at 0xd2890 | store | 4 | `sp -0x44` | `0x2842a8` |
| `ldr` at 0xd28b4 | load | 4 | `abs 0xd2aac` | `-` |
| `str` at 0xd28b8 | store | 4 | `sp -0x3c` | `0xfffff7ca` |
| `ldr` at 0xd28c0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd28e8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd28f0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd28f4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd28f8 | store | 4 | `sp -0x50` | `[sp-24]` |
| `str` at 0xd2900 | store | 4 | `sp -0x4c` | `sp-32` |
| `str` at 0xd2908 | store | 4 | `sp -0x48` | `sp-40` |
| `ldr` at 0xd290c | load | 4 | `abs 0xd2aa4` | `-` |
| `str` at 0xd2910 | store | 4 | `sp -0x44` | `0x2842a8` |
| `ldr` at 0xd2934 | load | 4 | `abs 0xd2aac` | `-` |
| `str` at 0xd2938 | store | 4 | `sp -0x3c` | `0xfffff7ca` |
| `str` at 0xd2944 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xd2954 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd2964 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd2974 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd2978 | store | 4 | `sp -0x50` | `[sp-24]` |
| `str` at 0xd2980 | store | 4 | `sp -0x4c` | `sp-32` |
| `str` at 0xd2988 | store | 4 | `sp -0x48` | `sp-40` |
| `ldr` at 0xd298c | load | 4 | `abs 0xd2aa4` | `-` |
| `str` at 0xd2990 | store | 4 | `sp -0x44` | `0x2842a8` |
| `ldr` at 0xd29b4 | load | 4 | `abs 0xd2aac` | `-` |
| `str` at 0xd29b8 | store | 4 | `sp -0x3c` | `0xfffff7ca` |
| `ldr` at 0xd29c0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xd29c8 | store | 4 | `sp -0x38` | `[sp-56]+1` |
| `ldr` at 0xd29dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd29f8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd2a00 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd2a04 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd2a08 | load | 4 | `[sp-44] +0x0` | `-` |
| `str` at 0xd2a18 | store | 4 | `[sp-44] +0x0` | `?` |
| `ldr` at 0xd2a1c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd2a20 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd2a24 | load | 4 | `[sp-44] +0x0` | `-` |
| `ldr` at 0xd2a30 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xd2a44 | store | 4 | `[sp-44] +0x0` | `?` |
| `str` at 0xd2a4c | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xd2a60 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd2a68 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd2a6c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd2a70 | load | 4 | `[sp-44] +0x0` | `-` |
| `ldr` at 0xd2a7c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xd2a90 | store | 4 | `[sp-44] +0x0` | `?` |
| `str` at 0xd2a98 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xd2a9c | load | 4 | `sp -0x3c` | `-` |
| `ldmdb` at 0xd2aa0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd2aa0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd2aa0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd2510 | `(unknown callee) 0xbce44` | `0x10` | `0x0` | `0x1` | `0x0` |
| `bl` at 0xd25fc | `(unknown callee) 0xbc8b4` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0xd2630 | `(unknown callee) 0xd2ab0` | `[sp-52]` | `[sp-44]` | `0x0` | `sp-24` |
| `bl` at 0xd2670 | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xd26cc | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xd274c | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xd27a4 | `(unknown callee) 0xd2c30` | `0x0` | `sp-40` | `[sp-44]` | `sp-40` |
| `bl` at 0xd28a4 | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xd2924 | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xd29a4 | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xd29fc | `(unknown callee) 0xd2c30` | `0x0` | `sp-40` | `[sp-44]` | `sp-40` |
| `bl` at 0xd2a64 | `(unknown callee) 0xd2c30` | `0x0` | `sp-40` | `[sp-44]` | `sp-40` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd251c | `ret(0xbce44) != 0x0` | `ret(0xbce44) == 0x0` |
| `beq` at 0xd2574 | `[sp-52] == 0x0` | `[sp-52] != 0x0` |
| `beq` at 0xd2580 | `[sp-52] == 0x1` | `[sp-52] != 0x1` |
| `beq` at 0xd258c | `[sp-52] == 0x2` | `[sp-52] != 0x2` |
| `beq` at 0xd2598 | `[sp-52] == 0x3` | `[sp-52] != 0x3` |
| `bne` at 0xd2608 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `beq` at 0xd263c | `[sp-48] == 0x2` | `[sp-48] != 0x2` |
| `bne` at 0xd267c | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `beq` at 0xd2698 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xd26d8 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `beq` at 0xd2708 | `? == 0x0` | `? != 0x0` |
| `bls` at 0xd2714 | `[sp-56] <= 0x3 (unsigned, 32-bit)` | `[sp-56] > 0x3 (unsigned, 32-bit)` |
| `bne` at 0xd2758 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `beq` at 0xd2790 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd2808 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd2824 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xd28b0 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `beq` at 0xd28cc | `? == 0x0` | `? != 0x0` |
| `bne` at 0xd2930 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bne` at 0xd2960 | `? != 0x0` | `? == 0x0` |
| `bls` at 0xd296c | `[sp-56] <= 0x3 (unsigned, 32-bit)` | `[sp-56] > 0x3 (unsigned, 32-bit)` |
| `bne` at 0xd29b0 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bne` at 0xd29e8 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd2aa0 | `r0` | `?` |

## `rtr_build_llptest`

- `rtr_build_llptest`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd2ab0 to 0xd2c30.
  Established. The opening call to `0xd50fc` carries entry r1, `sp-40`, entry r2, `sp-40` (`bl` at
  0xd2adc). Equality tests on `[sp-20]` take their edges for `0x1`, `0x5`, `0x3` or `0x7`,
  respectively, and fall through on inequality (`beq` at 0xd2ae8, `beq` at 0xd2af4, `beq` at
  0xd2b00, `beq` at 0xd2b0c). A word store writes `0x3c` at `[sp-32] +0x0` (`str` at 0xd2be8). The
  final call targets `0xc15b4` with `[sp-28]+59`, `[sp]`, unknown, `[sp-28]+59` (`bl` at 0xd2c28).
  That call’s result reaches the return, with the test encoding’s device interpretation unknown
  (`ldmdb` at 0xd2c2c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd2ac0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd2ac4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xd2ac8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xd2acc | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xd2ad4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd2ae0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd2aec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd2af8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd2b04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd2b2c | load | 4 | `(unknown)` | `-` |
| `str` at 0xd2b34 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd2b54 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd2b5c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd2b60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd2b6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd2b78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd2b84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd2bac | load | 4 | `(unknown)` | `-` |
| `str` at 0xd2bb4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd2bd4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd2bdc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd2be0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd2be8 | store | 4 | `[sp-32] +0x0` | `0x3c` |
| `ldr` at 0xd2bec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd2bf0 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xd2bfc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd2c08 | store | 4 | `sp -0x2c` | `0x3a` |
| `ldr` at 0xd2c10 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xd2c18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd2c24 | load | 4 | `sp +0x0` | `-` |
| `ldmdb` at 0xd2c2c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd2c2c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd2c2c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd2adc | `(unknown callee) 0xd50fc` | `r1` | `sp-40` | `r2` | `sp-40` |
| `bl` at 0xd2bf4 | `(unknown callee) 0xc15b4` | `[sp-28]` | `[sp]` | `[sp-32]` | `0x3c` |
| `bl` at 0xd2c14 | `(unknown callee) 0xc1ab4` | `0x0` | `sp-40` | `[sp-28]+1` | `[sp]` |
| `bl` at 0xd2c28 | `(unknown callee) 0xc15b4` | `[sp-28]+59` | `[sp]` | `?` | `[sp-28]+59` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd2ae8 | `[sp-20] == 0x1` | `[sp-20] != 0x1` |
| `beq` at 0xd2af4 | `[sp-20] == 0x5` | `[sp-20] != 0x5` |
| `beq` at 0xd2b00 | `[sp-20] == 0x3` | `[sp-20] != 0x3` |
| `beq` at 0xd2b0c | `[sp-20] == 0x7` | `[sp-20] != 0x7` |
| `beq` at 0xd2b68 | `[sp-20] == 0x2` | `[sp-20] != 0x2` |
| `beq` at 0xd2b74 | `[sp-20] == 0x6` | `[sp-20] != 0x6` |
| `beq` at 0xd2b80 | `[sp-20] == 0x3` | `[sp-20] != 0x3` |
| `beq` at 0xd2b8c | `[sp-20] == 0x7` | `[sp-20] != 0x7` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd2c2c | `r0` | `ret(0xc15b4)` |

## `rtr_extract_llptest`

- `rtr_extract_llptest`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd2c30 to 0xd2d34.
  Established. Two explicit word stores clear unknown targets (`str` at 0xd2c5c, `str` at 0xd2c70).
  The call to `0xc1ab4` has candidates `[sp-20]+1`, `[sp-24]`, `0x0`, `sp-36` (`bl` at 0xd2c98). The
  next call targets `0xd5364` with `sp-36`, `[sp-28]`, unknown, `sp-36` (`bl` at 0xd2ca8). The final
  word store has unknown value at `[sp-28] +0x0` (`str` at 0xd2d2c). r0 returns `ret(0xd5364)`
  (`ldmdb` at 0xd2d30). The extracted bits’ hardware status meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd2c40 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd2c44 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xd2c48 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xd2c5c | store | 4 | `(unknown)` | `0x0` |
| `str` at 0xd2c70 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xd2c74 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd2c84 | store | 4 | `sp -0x2c` | `0x40` |
| `ldr` at 0xd2c8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd2ca4 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xd2cb0 | store | 1 | `sp -0x25` | `0x0` |
| `ldr` at 0xd2cc0 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xd2cd0 | load | 1 | `sp -0x25` | `-` |
| `strb` at 0xd2cd8 | store | 1 | `sp -0x25` | `?` |
| `ldr` at 0xd2ce8 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xd2cf8 | load | 1 | `sp -0x25` | `-` |
| `strb` at 0xd2d00 | store | 1 | `sp -0x25` | `?` |
| `ldr` at 0xd2d04 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd2d08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd2d0c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0xd2d18 | load | 1 | `sp -0x25` | `-` |
| `str` at 0xd2d2c | store | 4 | `[sp-28] +0x0` | `?` |
| `ldmdb` at 0xd2d30 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd2d30 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd2d30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd2c98 | `(unknown callee) 0xc1ab4` | `[sp-20]+1` | `[sp-24]` | `0x0` | `sp-36` |
| `bl` at 0xd2ca8 | `(unknown callee) 0xd5364` | `sp-36` | `[sp-28]` | `?` | `sp-36` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd2ccc | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd2cf4 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd2d30 | `r0` | `ret(0xd5364)` |

## `llpcmd_build_old_fmt`

- `llpcmd_build_old_fmt`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd50fc to 0xd5364.
  Established. Word stores clear entry `r1 +0x0` and `[sp-24] +0x4` (`str` at 0xd511c, `str` at
  0xd5130). Later stores at `[sp-24] +0x0` and `[sp-24] +0x4` include unknown values (`str` at
  0xd5164, `str` at 0xd534c). The first deciding branch takes its edge for `? == 0x0` and falls
  through for inequality (`beq` at 0xd5150). The only call targets `0xc1c9c` with `0x0`, `0x40`,
  `[sp-24]`, unknown (`bl` at 0xd535c). The return reports `ret(0xc1c9c)` (`ldmdb` at 0xd5360). The
  legacy representation’s protocol meaning is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd510c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd5110 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xd5114 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd511c | store | 4 | `r1 +0x0` | `0x0` |
| `ldr` at 0xd5124 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd5130 | store | 4 | `[sp-24] +0x4` | `0x0` |
| `ldr` at 0xd5138 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd5140 | load | 4 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xd5154 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5158 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd515c | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xd5164 | store | 4 | `[sp-24] +0x0` | `?` |
| `ldr` at 0xd516c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd5174 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xd5188 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd518c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5190 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xd5198 | store | 4 | `[sp-24] +0x0` | `?` |
| `ldr` at 0xd51a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd51a8 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xd51bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd51c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd51c4 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xd51cc | store | 4 | `[sp-24] +0x0` | `?` |
| `ldr` at 0xd51d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd51d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd51dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd51e4 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xd51f4 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xd51fc | store | 4 | `[sp-24] +0x0` | `?` |
| `ldr` at 0xd5204 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd520c | load | 4 | `[sp-20] +0x10` | `-` |
| `ldr` at 0xd5220 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5224 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5228 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xd5230 | store | 4 | `[sp-24] +0x0` | `?` |
| `ldr` at 0xd5238 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd5240 | load | 4 | `[sp-20] +0x10` | `-` |
| `ldr` at 0xd5254 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5258 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd525c | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xd5264 | store | 4 | `[sp-24] +0x0` | `?` |
| `ldr` at 0xd5268 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd526c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5274 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd527c | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0xd528c | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xd5294 | store | 4 | `[sp-24] +0x0` | `?` |
| `ldr` at 0xd529c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd52a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd52b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd52bc | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0xd52c8 | load | 4 | `[sp-24] +0x4` | `-` |
| `str` at 0xd52d0 | store | 4 | `[sp-24] +0x4` | `?` |
| `ldr` at 0xd52d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd52e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd52f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd52f8 | load | 4 | `[sp-20] +0x10` | `-` |
| `ldr` at 0xd5308 | load | 4 | `[sp-24] +0x4` | `-` |
| `str` at 0xd5310 | store | 4 | `[sp-24] +0x4` | `?` |
| `ldr` at 0xd5318 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5324 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5330 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd5338 | load | 4 | `[sp-20] +0xc` | `-` |
| `ldr` at 0xd5344 | load | 4 | `[sp-24] +0x4` | `-` |
| `str` at 0xd534c | store | 4 | `[sp-24] +0x4` | `?` |
| `ldr` at 0xd5358 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xd5360 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd5360 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd5360 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd535c | `(unknown callee) 0xc1c9c` | `0x0` | `0x40` | `[sp-24]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd5150 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd5184 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd51b8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd521c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd5250 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd5360 | `r0` | `ret(0xc1c9c)` |

## `llpcmd_extract_old_fmt`

- `llpcmd_extract_old_fmt`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd5364 to 0xd5604.
  Established. The only call targets `0xc1c9c` with candidates `0x0`, `0x40`, entry r0, entry r3
  (`bl` at 0xd5388). Word stores include zero at an unknown target, `[sp-28]` at `[sp-24] +0x1c`,
  and an unknown value at `[sp-24] +0xc` (`str` at 0xd53b8, `str` at 0xd5584, `str` at 0xd55fc). The
  first loop test retains a less-or-equal flags predicate with unknown operands, following its edge
  when that predicate holds and falling through otherwise (`bls` at 0xd539c). The return reports
  `[sp-24]+16` (`ldmdb` at 0xd5600). The reconstructed fields’ hardware use is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd5374 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd5378 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xd5384 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd5390 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xd5394 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd53a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd53ac | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd53b8 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xd53bc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd53c4 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xd53cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd53d0 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xd53e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd53f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd53f8 | load | 4 | `[sp-24] +0x4` | `-` |
| `str` at 0xd5404 | store | 4 | `[sp-24] +0x4` | `?` |
| `ldr` at 0xd5408 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd540c | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xd5420 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd542c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5434 | load | 4 | `[sp-24] +0x8` | `-` |
| `str` at 0xd5440 | store | 4 | `[sp-24] +0x8` | `?` |
| `ldr` at 0xd5444 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd5448 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xd545c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5468 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5470 | load | 4 | `[sp-24] +0x8` | `-` |
| `str` at 0xd547c | store | 4 | `[sp-24] +0x8` | `?` |
| `ldr` at 0xd5480 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd5484 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xd5490 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xd5498 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd54a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd54b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd54b8 | load | 4 | `[sp-24] +0x8` | `-` |
| `str` at 0xd54c4 | store | 4 | `[sp-24] +0x8` | `?` |
| `ldr` at 0xd54c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd54cc | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xd54e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd54ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd54f4 | load | 4 | `[sp-24] +0x10` | `-` |
| `str` at 0xd5500 | store | 4 | `[sp-24] +0x10` | `?` |
| `ldr` at 0xd5504 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd5508 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xd551c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5528 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5530 | load | 4 | `[sp-24] +0x10` | `-` |
| `str` at 0xd553c | store | 4 | `[sp-24] +0x10` | `?` |
| `ldr` at 0xd5540 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd5544 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xd5558 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd5560 | load | 4 | `[sp-20] +0x4` | `-` |
| `str` at 0xd5570 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xd5578 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5580 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd5584 | store | 4 | `[sp-24] +0x1c` | `[sp-28]` |
| `ldr` at 0xd558c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd5594 | load | 4 | `[sp-20] +0x4` | `-` |
| `str` at 0xd55a0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xd55a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd55b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd55c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd55c8 | load | 4 | `[sp-24] +0x10` | `-` |
| `str` at 0xd55d4 | store | 4 | `[sp-24] +0x10` | `?` |
| `ldr` at 0xd55dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd55e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd55f0 | load | 4 | `[sp-20] +0x4` | `-` |
| `str` at 0xd55fc | store | 4 | `[sp-24] +0xc` | `?` |
| `ldmdb` at 0xd5600 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd5600 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd5600 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd5388 | `(unknown callee) 0xc1c9c` | `0x0` | `0x40` | `r0` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xd539c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0xd53dc | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd5418 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd5454 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd54d8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd5514 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd5600 | `r0` | `[sp-24]+16` |

## `smpcmd_scan_reset`

- `smpcmd_scan_reset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc7b14 to 0xc7c54.
  Established. The call to `0x18920` has `0x50`, entry r1, entry r2 and entry r3, with its result
  stored as a word at entry `r1 +0x0` (`bl` at 0xc7b34, `str` at 0xc7b3c). A nonzero word at
  absolute `0x27eb94` takes the continuation edge and zero falls through (`ldr` at 0xc7b44, `bne` at
  0xc7b4c). Two calls to `0xbc308` carry `0x0` or `0x1` first, followed by `[sp-32]`, `[[sp-28]]`,
  `[sp-28]` (`bl` at 0xc7ba8, `bl` at 0xc7bfc). r0 returns `0x0` (`ldmdb` at 0xc7c44). A physical
  reset operation is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc7b24 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xc7b28 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xc7b2c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc7b3c | store | 4 | `r1 +0x0` | `ret(0x18920)` |
| `ldr` at 0xc7b40 | load | 4 | `abs 0xc7c48` | `-` |
| `ldr` at 0xc7b44 | load | 4 | `abs 0x27eb94` | `-` |
| `ldr` at 0xc7b50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc7b54 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xc7b58 | load | 4 | `abs 0xc7c4c` | `-` |
| `str` at 0xc7b64 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xc7b7c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xc7b80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc7b84 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xc7b88 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0xc7b98 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc7ba0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc7ba4 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xc7bb8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc7bbc | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xc7bc0 | load | 4 | `abs 0xc7c4c` | `-` |
| `str` at 0xc7bcc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xc7bd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc7bd8 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xc7bdc | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0xc7bec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc7bf4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc7bf8 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xc7c0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc7c10 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xc7c14 | load | 4 | `abs 0xc7c4c` | `-` |
| `str` at 0xc7c20 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xc7c28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc7c2c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xc7c30 | load | 4 | `abs 0xc7c50` | `-` |
| `str` at 0xc7c3c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xc7c40 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xc7c44 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc7c44 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc7c44 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc7c44 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc7b34 | `(unknown callee) 0x18920` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0xc7b5c | `(unknown callee) 0x18ab8` | `[[sp-28]]` | `0x10a3f0` | `?` | `[sp-28]` |
| `bl` at 0xc7b6c | `(unknown callee) 0xc4f18` | `ret(0x18920)` | `?` | `?` | `[0x27eb94]` |
| `bl` at 0xc7ba8 | `(unknown callee) 0xbc308` | `0x0` | `[sp-32]` | `[[sp-28]]` | `[sp-28]` |
| `bl` at 0xc7bc4 | `(unknown callee) 0x18ab8` | `[[sp-28]]` | `0x10a3f0` | `?` | `[sp-28]` |
| `bl` at 0xc7bfc | `(unknown callee) 0xbc308` | `0x1` | `[sp-32]` | `[[sp-28]]` | `[sp-28]` |
| `bl` at 0xc7c18 | `(unknown callee) 0x18ab8` | `[[sp-28]]` | `0x10a3f0` | `?` | `[sp-28]` |
| `bl` at 0xc7c34 | `(unknown callee) 0x18c74` | `[[sp-28]]` | `0x10a3f4` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc7b4c | `[0x27eb94] != 0x0` | `[0x27eb94] == 0x0` |
| `beq` at 0xc7b94 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xc7bb4 | `ret(0xbc308) != 0x0` | `ret(0xbc308) == 0x0` |
| `beq` at 0xc7be8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xc7c08 | `ret(0xbc308) != 0x0` | `ret(0xbc308) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc7c44 | `r0` | `0x0` |

## `smpcmd_scan_sel`

- `smpcmd_scan_sel`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc7c54 to 0xc7e38.
  Established. An unsigned comparison takes the edge for `[[sp-24]+40] <= 0xff` and falls through
  above it (`bls` at 0xc7cc8). A second takes the edge for `[[sp-24]+44] <= 0xffff` and falls
  through above it (`bls` at 0xc7d00). The call to `0xbce44` has candidates unknown, `[[sp-24]+40]`,
  `[[sp-24]+44]`, `0x0` (`bl` at 0xc7dcc). Stack word stores write `sp-56` and `sp-40` at `sp -0xe4` and
  `sp -0xe0` (`str` at 0xc7da4, `str` at 0xc7dac). The return reports `0x0` (`ldmdb` at 0xc7e1c).
  The selector values’ physical meanings are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc7c64 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xc7c68 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xc7c6c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc7c7c | store | 4 | `r1 +0x0` | `ret(0x18920)` |
| `ldr` at 0xc7c80 | load | 4 | `abs 0xc7e20` | `-` |
| `ldr` at 0xc7c84 | load | 4 | `abs 0x27eb94` | `-` |
| `ldr` at 0xc7c90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc7c94 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xc7c98 | load | 4 | `abs 0xc7e24` | `-` |
| `str` at 0xc7ca4 | store | 4 | `sp -0xd8` | `0x0` |
| `str` at 0xc7cb0 | store | 4 | `sp -0x20` | `0x58` |
| `ldr` at 0xc7cb4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc7cb8 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xc7cbc | store | 4 | `sp -0x24` | `[[sp-24]+40]` |
| `ldr` at 0xc7cc0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc7ccc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc7cd0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xc7cd4 | load | 4 | `abs 0xc7e28` | `-` |
| `str` at 0xc7ce0 | store | 4 | `sp -0xd8` | `0x0` |
| `ldr` at 0xc7ce8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc7cec | load | 4 | `[sp-24] +0x2c` | `-` |
| `str` at 0xc7cf0 | store | 4 | `sp -0x2c` | `[[sp-24]+44]` |
| `ldr` at 0xc7cf4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc7cf8 | load | 4 | `abs 0xc7e2c` | `-` |
| `ldr` at 0xc7d04 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc7d08 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xc7d0c | load | 4 | `abs 0xc7e30` | `-` |
| `str` at 0xc7d18 | store | 4 | `sp -0xd8` | `0x0` |
| `str` at 0xc7d24 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0xc7d2c | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xc7d30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc7d34 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xc7d38 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0xc7d48 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc7d50 | store | 4 | `sp -0x20` | `0x5a` |
| `ldr` at 0xc7d54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc7d58 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0xc7d5c | store | 4 | `sp -0x30` | `[[sp-24]+48]` |
| `ldr` at 0xc7d60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc7d64 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xc7d68 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0xc7d78 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc7d80 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xc7d84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc7d88 | load | 4 | `[sp-24] +0x34` | `-` |
| `str` at 0xc7d8c | store | 4 | `sp -0x34` | `[[sp-24]+52]` |
| `ldr` at 0xc7d90 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xc7d94 | store | 4 | `sp -0xec` | `?` |
| `ldr` at 0xc7d98 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xc7d9c | store | 4 | `sp -0xe8` | `?` |
| `str` at 0xc7da4 | store | 4 | `sp -0xe4` | `sp-56` |
| `str` at 0xc7dac | store | 4 | `sp -0xe0` | `sp-40` |
| `ldr` at 0xc7db0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc7db4 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xc7db8 | store | 4 | `sp -0xdc` | `[[sp-28]]` |
| `ldr` at 0xc7dbc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc7dc0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc7dc4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc7ddc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc7de0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xc7de4 | load | 4 | `abs 0xc7e24` | `-` |
| `str` at 0xc7df0 | store | 4 | `sp -0xd8` | `0x0` |
| `ldr` at 0xc7df8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc7dfc | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xc7e00 | load | 4 | `abs 0xc7e34` | `-` |
| `ldr` at 0xc7e04 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc7e08 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc7e14 | store | 4 | `sp -0xd8` | `0x0` |
| `ldr` at 0xc7e18 | load | 4 | `sp -0xd8` | `-` |
| `ldmdb` at 0xc7e1c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc7e1c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc7e1c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc7e1c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc7c74 | `(unknown callee) 0x18920` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0xc7c9c | `(unknown callee) 0x18ab8` | `[[sp-28]]` | `0x10a3f0` | `?` | `[sp-28]` |
| `bl` at 0xc7cd8 | `(unknown callee) 0x18c74` | `[[sp-28]]` | `0x10a418` | `?` | `[sp-28]` |
| `bl` at 0xc7d10 | `(unknown callee) 0x18c74` | `[[sp-28]]` | `0x10a44c` | `[[sp-24]+44]` | `[sp-28]` |
| `bl` at 0xc7dcc | `(unknown callee) 0xbce44` | `?` | `[[sp-24]+40]` | `[[sp-24]+44]` | `0x0` |
| `bl` at 0xc7de8 | `(unknown callee) 0x18ab8` | `[[sp-28]]` | `0x10a3f0` | `?` | `[sp-28]` |
| `bl` at 0xc7e0c | `(unknown callee) 0x18c74` | `[[sp-28]]` | `0x10a474` | `[sp-56]` | `[sp-40]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc7c8c | `[0x27eb94] != 0x0` | `[0x27eb94] == 0x0` |
| `bls` at 0xc7cc8 | `[[sp-24]+40] <= 0xff (unsigned, 32-bit)` | `[[sp-24]+40] > 0xff (unsigned, 32-bit)` |
| `bls` at 0xc7d00 | `[[sp-24]+44] <= 0xffff (unsigned, 32-bit)` | `[[sp-24]+44] > 0xffff (unsigned, 32-bit)` |
| `beq` at 0xc7d44 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xc7d74 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xc7dd8 | `ret(0xbce44) != 0x0` | `ret(0xbce44) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc7e1c | `r0` | `0x0` |

## `smpcmd_scan_config`

- `smpcmd_scan_config`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc7e38 to 0xc7ecc.
  Established. A word store places `ret(0x18920)` at entry `r1 +0x0` (`str` at 0xc7e60). A nonzero
  word at absolute `0x27eb94` takes the edge to the indirect call, while zero falls through
  to `0x18cd4` (`ldr` at 0xc7e68, `bne` at 0xc7e70, `bl` at 0xc7e80). The indirect call has
  unknown target and candidates `ret(0x18920)`, unknown, unknown, `[0x115850]` (`mov` at 0xc7e9c).
  The return reports `0x0` (`ldmdb` at 0xc7ebc). The indirect callee’s physical configuration effect
  is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc7e48 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xc7e4c | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xc7e50 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc7e60 | store | 4 | `r1 +0x0` | `ret(0x18920)` |
| `ldr` at 0xc7e64 | load | 4 | `abs 0xc7ec0` | `-` |
| `ldr` at 0xc7e68 | load | 4 | `abs 0x27eb94` | `-` |
| `ldr` at 0xc7e74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc7e78 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xc7e7c | load | 4 | `abs 0xc7ec4` | `-` |
| `str` at 0xc7e88 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xc7e90 | load | 4 | `abs 0xc7ec8` | `-` |
| `ldr` at 0xc7e94 | load | 4 | `abs 0x115850` | `-` |
| `ldr` at 0xc7ea0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc7ea4 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xc7ea8 | load | 4 | `abs 0xc7ec4` | `-` |
| `str` at 0xc7eb4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xc7eb8 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xc7ebc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc7ebc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc7ebc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc7ebc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc7e58 | `(unknown callee) 0x18920` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0xc7e80 | `(unknown callee) 0x18cd4` | `[[sp-28]]` | `0x10a3f0` | `?` | `[sp-28]` |
| `mov` at 0xc7e9c | `(unknown callee) (unknown target)` | `ret(0x18920)` | `?` | `?` | `[0x115850]` |
| `bl` at 0xc7eac | `(unknown callee) 0x18cd4` | `[[sp-28]]` | `0x10a3f0` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc7e70 | `[0x27eb94] != 0x0` | `[0x27eb94] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc7ebc | `r0` | `0x0` |

## `smpcmd_scan_debug`

- `smpcmd_scan_debug`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc83cc to 0xc8470.
  Established. The unsigned bound test takes its edge for `[[sp-24]+40] <= 0x4` and falls through
  above that value (`bls` at 0xc8420). The taken path stores `[[sp-24]+40]` as a word at absolute
  `0x27eb98` (`str` at 0xc843c). The last call targets `0x18c74` with candidates `[[sp-28]]`,
  `0x10a664`, `[0x27eb98]`, `[sp-28]` (`bl` at 0xc8454). r0 receives `0x0` and returns it (`mov` at
  0xc8458, `mov` at 0xc845c, `ldmdb` at 0xc8460). The absolute value’s external meaning remains
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc83dc | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xc83e0 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xc83e4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc83f4 | store | 4 | `r1 +0x0` | `ret(0x18920)` |
| `ldr` at 0xc83f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc83fc | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xc8400 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0xc840c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc8410 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xc8414 | store | 4 | `sp -0x20` | `[[sp-24]+40]` |
| `ldr` at 0xc8418 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc8424 | load | 4 | `abs 0xc8464` | `-` |
| `ldr` at 0xc8434 | load | 4 | `abs 0xc8468` | `-` |
| `ldr` at 0xc8438 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc843c | store | 4 | `abs 0x27eb98` | `[[sp-24]+40]` |
| `ldr` at 0xc8440 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc8444 | load | 4 | `abs 0xc8468` | `-` |
| `ldr` at 0xc8448 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xc844c | load | 4 | `abs 0xc846c` | `-` |
| `ldr` at 0xc8450 | load | 4 | `abs 0x27eb98` | `-` |
| `ldmdb` at 0xc8460 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc8460 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc8460 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc8460 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc83ec | `(unknown callee) 0x18920` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0xc842c | `(unknown callee) 0x20f14` | `0x10a628` | `0x4` | `?` | `[[sp-24]+40]` |
| `bl` at 0xc8454 | `(unknown callee) 0x18c74` | `[[sp-28]]` | `0x10a664` | `[0x27eb98]` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc8408 | `[[[sp-24]+112]] != 0x1` | `[[[sp-24]+112]] == 0x1` |
| `bls` at 0xc8420 | `[[sp-24]+40] <= 0x4 (unsigned, 32-bit)` | `[[sp-24]+40] > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc8460 | `r0` | `0x0` |

## `smpcmd_scan_set`

- `smpcmd_scan_set`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc8470 to 0xc8568.
  Established. The first call to `0x18920` has `0x50`, entry r1, entry r2 and entry r3 (`bl` at
  0xc8490). The result is stored as a word at entry `r1 +0x0` (`str` at 0xc8498). The later zero
  test takes the bypass for `[[[sp-24]+112]] == 0x0` and falls through otherwise (`beq` at 0xc84d8).
  A call to `0xc526c` carries `[sp-36]` and three unknown candidates (`bl` at 0xc8540). The last
  call targets `0xc503c` with `[[sp-28]]`, unknown, unknown, `[sp-28]` (`bl` at 0xc854c). A physical
  signal update is unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc8480 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xc8484 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xc8488 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc8498 | store | 4 | `r1 +0x0` | `ret(0x18920)` |
| `ldr` at 0xc849c | load | 4 | `abs 0xc8560` | `-` |
| `ldr` at 0xc84a0 | load | 4 | `abs 0x27eb94` | `-` |
| `ldr` at 0xc84ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc84b0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xc84b4 | load | 4 | `abs 0xc8564` | `-` |
| `str` at 0xc84c0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xc84c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc84cc | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xc84d0 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `str` at 0xc84e4 | store | 4 | `sp -0x24` | `ret(0xc4f18)` |
| `ldr` at 0xc84e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc84f0 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xc84f8 | load | 4 | `[[sp-24]+112] +0x4` | `-` |
| `ldr` at 0xc8504 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc8508 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xc850c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc8510 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `str` at 0xc8518 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xc8520 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc8524 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xc8528 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0xc8530 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc8538 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xc853c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc8544 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc8548 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xc8554 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xc8558 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xc855c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc855c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc855c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc855c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc8490 | `(unknown callee) 0x18920` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0xc84b8 | `(unknown callee) 0x18ab8` | `[[sp-28]]` | `0x10a3f0` | `?` | `[sp-28]` |
| `bl` at 0xc84dc | `(unknown callee) 0xc4f18` | `ret(0x18920)` | `?` | `?` | `[[[sp-24]+112]]` |
| `bl` at 0xc8540 | `(unknown callee) 0xc526c` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0xc854c | `(unknown callee) 0xc503c` | `[[sp-28]]` | `?` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc84a8 | `[0x27eb94] != 0x0` | `[0x27eb94] == 0x0` |
| `beq` at 0xc84d8 | `[[[sp-24]+112]] == 0x0` | `[[[sp-24]+112]] != 0x0` |
| `beq` at 0xc8500 | `[[[sp-24]+112]+4] == 0x0` | `[[[sp-24]+112]+4] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc855c | `r0` | `0x0` |
