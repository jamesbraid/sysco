# ARM power sequencing and monitor helpers in 1.44.0

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
Computed transfers remain unresolved in powerUp (`ldrls` at 0x369a4, `ldrls` at 0x36c98)
and monitorVoltage (`ldrls` at 0xc1098, `ldrls` at 0xc1138). Their tables cover known edges only.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/power-sequence-monitor-facts.txt) retain all tool sections.

## `powerUpSlot`

- `powerUpSlot`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x365bc to 0x368c4.
  Established. Included byte assignments through `[sp-44] +0xc`, `+0xd` and `+0xe` write `0x0`,
  `0x1` and `0x1` respectively (`strb` at 0x3664c, `strb` at 0x36828, `strb` at 0x3666c). A later
  byte assignment through `[sp-44] +0xe` writes zero (`strb` at 0x36888). The indirect call has
  unknown target and candidates `[sp-20]`, unknown, unknown, `[[sp-44]+16]` (`mov` at 0x367e0). Slot
  power-up effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x365cc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x365d0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x365d4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x365dc | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x365e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x365f4 | load | 4 | `abs 0x368b4` | `-` |
| `ldr` at 0x365f8 | load | 4 | `abs 0x13219c` | `-` |
| `str` at 0x36600 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x36604 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3660c | store | 4 | `sp -0x34` | `ret(0x48054)` |
| `ldr` at 0x36610 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3661c | load | 4 | `abs 0x368b8` | `-` |
| `str` at 0x36620 | store | 4 | `sp -0x38` | `0xf575c` |
| `ldr` at 0x36628 | load | 4 | `abs 0x368bc` | `-` |
| `str` at 0x3662c | store | 4 | `sp -0x38` | `0xf5760` |
| `ldr` at 0x36630 | load | 4 | `abs 0x368c0` | `-` |
| `ldr` at 0x36634 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x36638 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3663c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x36644 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0x3664c | store | 1 | `[sp-44] +0xc` | `0x0` |
| `str` at 0x3665c | store | 4 | `sp -0x20` | `ret(0x47ba4)` |
| `ldr` at 0x36664 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0x3666c | store | 1 | `[sp-44] +0xe` | `0x1` |
| `str` at 0x36674 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x36678 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3667c | load | 4 | `[sp-44] +0x8` | `-` |
| `ldr` at 0x36680 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3668c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x36690 | load | 4 | `[sp-44] +0x4` | `-` |
| `ldr` at 0x36694 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x3669c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x366a8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x366ac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x366c0 | load | 4 | `[sp-44] +0x0` | `-` |
| `ldrb` at 0x366c8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x366d4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x366d8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x366ec | load | 4 | `[sp-44] +0x0` | `-` |
| `ldr` at 0x366f4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x36704 | store | 4 | `sp -0x24` | `ret(0x3a1a0)` |
| `ldr` at 0x36708 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x36714 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x36724 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0x3672c | store | 1 | `[sp-44] +0xe` | `0x0` |
| `ldr` at 0x36730 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x36734 | store | 4 | `sp -0x30` | `[sp-36]` |
| `ldr` at 0x3673c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x36740 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x36754 | load | 4 | `[sp-44] +0x0` | `-` |
| `ldrh` at 0x3675c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x36768 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3676c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x36780 | load | 4 | `[sp-44] +0x0` | `-` |
| `ldrh` at 0x36788 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x367a0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x367a8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x367b8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x367bc | load | 4 | `[sp-44] +0x10` | `-` |
| `str` at 0x367cc | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0x367d0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x367d4 | load | 4 | `[sp-44] +0x10` | `-` |
| `ldr` at 0x367d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x367f0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x36800 | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0x36804 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x36808 | store | 4 | `sp -0x24` | `[sp-60]` |
| `ldr` at 0x36814 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x36818 | store | 4 | `sp -0x20` | `[sp-60]` |
| `ldr` at 0x36820 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0x36828 | store | 1 | `[sp-44] +0xd` | `0x1` |
| `ldr` at 0x3682c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x36844 | store | 4 | `sp -0x24` | `ret(0x47ad8)` |
| `ldr` at 0x36848 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x36854 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x36860 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x36864 | store | 4 | `sp -0x20` | `ret(0x47ad8)` |
| `ldr` at 0x36868 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x36870 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x36878 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x36880 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0x36888 | store | 1 | `[sp-44] +0xe` | `0x0` |
| `ldr` at 0x3688c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x368a0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x368a4 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0x368a8 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x368b0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x368b0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x368b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x36608 | `(unknown callee) 0x48054` | `r2` | `r1` | `?` | `?` |
| `bl` at 0x36640 | `(unknown callee) 0x1e958` | `0xf5768` | `[sp-20]` | `[sp-56]` | `ret(0x48054)` |
| `bl` at 0x36654 | `(unknown callee) 0x47ba4` | `0x5` | `?` | `[sp-44]` | `0x0` |
| `bl` at 0x36660 | `(unknown callee) 0x76f1c` | `ret(0x47ba4)` | `?` | `?` | `ret(0x47ba4)` |
| `bl` at 0x366fc | `(unknown callee) 0x3a1a0` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x3671c | `(unknown callee) 0x36f6c` | `[sp-20]` | `0x5` | `?` | `ret(0x3a1a0)` |
| `bl` at 0x36720 | `(unknown callee) 0x76f34` | `ret(0x36f6c)` | `?` | `?` | `?` |
| `bl` at 0x36790 | `(unknown callee) 0x368c4` | `?` | `[sp-44]` | `?` | `?` |
| `bl` at 0x3679c | `(unknown callee) 0x368c4` | `0x19` | `[sp-44]` | `?` | `?` |
| `bl` at 0x367b4 | `(unknown callee) 0x368c4` | `0x64` | `?` | `[[sp-44]+8]` | `?` |
| `mov` at 0x367e0 | `(unknown callee) (unknown target)` | `[sp-20]` | `?` | `?` | `[[sp-44]+16]` |
| `bl` at 0x3681c | `(unknown callee) 0x76f34` | `?` | `?` | `?` | `?` |
| `bl` at 0x3683c | `(unknown callee) 0x47ad8` | `0x5` | `?` | `[sp-44]` | `[sp-24]` |
| `bl` at 0x3686c | `(unknown callee) 0x255ac` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x36874 | `(unknown callee) 0x25744` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x3687c | `(unknown callee) 0x24da8` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x3689c | `(unknown callee) 0x7701c` | `0x0` | `?` | `[sp-44]` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x36618 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `ble` at 0x36688 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x366a4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x366d0 | `? == 0xff` | `? != 0xff` |
| `beq` at 0x36710 | `ret(0x3a1a0) == 0x0` | `ret(0x3a1a0) != 0x0` |
| `beq` at 0x36764 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x367c4 | `[[sp-44]+16] == 0x0` | `[[sp-44]+16] != 0x0` |
| `beq` at 0x367ec | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x367f8 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x36810 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `beq` at 0x36834 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0x36850 | `ret(0x47ad8) == 0x0` | `ret(0x47ad8) != 0x0` |
| `bne` at 0x3685c | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x36894 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x368b0 | `r0` | `?` |

## `powerUp`

- `powerUp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x368ec to 0x36f6c.
  Established. The byte through `[sp-72] +0xc` receives zero (`strb` at 0x36a9c). Included byte
  assignments through `[sp-72] +0xe` write `0x1` and `0x0` in separate paths (`strb` at 0x36abc,
  `strb` at 0x36b60). The unnamed `0x3b9cc` call carries `0x0`, `0xff`, `0x3`, unknown (`bl` at 0x36c44). Power-up sequencing policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x368fc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x36900 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x36908 | store | 4 | `sp -0x20` | `0x0` |
| `strb` at 0x36914 | store | 1 | `sp -0x35` | `(ret(0x488cc) & 0xff)` |
| `ldr` at 0x36918 | load | 4 | `abs 0x36f2c` | `-` |
| `ldr` at 0x3691c | load | 4 | `abs 0x13219c` | `-` |
| `str` at 0x36920 | store | 4 | `sp -0x48` | `[0x13219c]` |
| `ldr` at 0x36928 | load | 4 | `abs 0x36f30` | `-` |
| `ldr` at 0x3692c | load | 4 | `abs 0x297684` | `-` |
| `ldr` at 0x36938 | load | 4 | `abs 0x36f30` | `-` |
| `ldr` at 0x3693c | load | 4 | `abs 0x297684` | `-` |
| `ldr` at 0x36954 | load | 4 | `abs 0x36f34` | `-` |
| `str` at 0x36958 | store | 4 | `sp -0x64` | `0xf577c` |
| `ldr` at 0x36960 | load | 4 | `abs 0x36f38` | `-` |
| `ldr` at 0x36964 | load | 4 | `abs 0x36f3c` | `-` |
| `str` at 0x36974 | store | 4 | `sp -0x50` | `0x0` |
| `str` at 0x36984 | store | 4 | `sp -0x28` | `ret(0x49c2c)` |
| `ldr` at 0x36988 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x36994 | load | 4 | `sp -0x28` | `-` |
| `ldrls` at 0x369a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x36a04 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x36a10 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x36a14 | store | 4 | `sp -0x50` | `[sp-40]` |
| `ldr` at 0x36a1c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x36a24 | store | 4 | `sp -0x54` | `ret(0x48054)` |
| `ldr` at 0x36a28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x36a34 | load | 4 | `abs 0x36f54` | `-` |
| `str` at 0x36a38 | store | 4 | `sp -0x58` | `0xf575c` |
| `ldr` at 0x36a40 | load | 4 | `abs 0x36f58` | `-` |
| `str` at 0x36a44 | store | 4 | `sp -0x58` | `0xf5760` |
| `ldr` at 0x36a48 | load | 4 | `abs 0x36f5c` | `-` |
| `ldr` at 0x36a4c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x36a50 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x36a64 | store | 4 | `sp -0x24` | `ret(0x23348)` |
| `str` at 0x36a78 | store | 4 | `sp -0x24` | `ret(0x4abf0)` |
| `ldr` at 0x36a80 | load | 4 | `abs 0x36f60` | `-` |
| `str` at 0x36a90 | store | 4 | `sp -0x24` | `ret(0x22e24)` |
| `ldr` at 0x36a94 | load | 4 | `sp -0x48` | `-` |
| `strb` at 0x36a9c | store | 1 | `[sp-72] +0xc` | `0x0` |
| `str` at 0x36aac | store | 4 | `sp -0x20` | `ret(0x47ba4)` |
| `ldr` at 0x36ab4 | load | 4 | `sp -0x48` | `-` |
| `strb` at 0x36abc | store | 1 | `[sp-72] +0xe` | `0x1` |
| `str` at 0x36ac4 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x36ac8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x36acc | load | 4 | `[sp-72] +0x8` | `-` |
| `ldr` at 0x36ad0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x36adc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x36ae0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x36af4 | load | 4 | `[sp-72] +0x0` | `-` |
| `str` at 0x36afc | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x36b00 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x36b04 | load | 4 | `[sp-72] +0x4` | `-` |
| `ldr` at 0x36b08 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x36b10 | load | 1 | `([[sp-72]+4] + [sp-44]) +0x0` | `-` |
| `ldr` at 0x36b1c | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x36b20 | load | 1 | `[sp-76] +0x2` | `-` |
| `ldr` at 0x36b30 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x36b3c | store | 4 | `sp -0x24` | `ret(0x3a1a0)` |
| `ldr` at 0x36b40 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x36b58 | load | 4 | `sp -0x48` | `-` |
| `strb` at 0x36b60 | store | 1 | `[sp-72] +0xe` | `0x0` |
| `ldr` at 0x36b64 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x36b68 | store | 4 | `sp -0x50` | `[sp-36]` |
| `ldr` at 0x36b70 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x36b74 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x36b88 | load | 4 | `[sp-72] +0x0` | `-` |
| `ldrh` at 0x36b90 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x36b9c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x36ba0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x36bb4 | load | 4 | `[sp-72] +0x0` | `-` |
| `ldrh` at 0x36bbc | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x36bd4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x36bdc | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x36bec | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x36bf0 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x36bfc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x36c00 | load | 4 | `[sp-72] +0x10` | `-` |
| `str` at 0x36c14 | store | 4 | `sp -0x24` | `ret(unknown)` |
| `ldr` at 0x36c18 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x36c24 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x36c30 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x36c34 | store | 4 | `sp -0x20` | `ret(unknown)` |
| `str` at 0x36c4c | store | 4 | `sp -0x24` | `ret(0x3b9cc)` |
| `ldr` at 0x36c50 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x36c5c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x36c68 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x36c6c | store | 4 | `sp -0x20` | `ret(0x3b9cc)` |
| `ldr` at 0x36c74 | load | 4 | `sp -0x48` | `-` |
| `strb` at 0x36c7c | store | 1 | `[sp-72] +0xd` | `0x1` |
| `ldr` at 0x36c80 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x36c8c | load | 1 | `sp -0x35` | `-` |
| `ldrls` at 0x36c98 | load | 4 | `(unknown)` | `-` |
| `str` at 0x36de8 | store | 4 | `sp -0x24` | `ret(0x47ad8)` |
| `ldr` at 0x36dec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x36df8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x36e04 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x36e08 | store | 4 | `sp -0x20` | `ret(0x47ad8)` |
| `ldr` at 0x36e0c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x36e24 | load | 4 | `abs 0x36f64` | `-` |
| `str` at 0x36e34 | store | 4 | `sp -0x24` | `ret(0x22e24)` |
| `ldr` at 0x36e40 | load | 4 | `abs 0x36f68` | `-` |
| `str` at 0x36e50 | store | 4 | `sp -0x24` | `ret(0x22e24)` |
| `ldrb` at 0x36e6c | load | 1 | `sp -0x35` | `-` |
| `str` at 0x36e70 | store | 4 | `sp -0x5c` | `[sp-53]:1` |
| `ldr` at 0x36e74 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x36e80 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x36e9c | store | 4 | `sp -0x24` | `ret(0x47ad8)` |
| `ldr` at 0x36ea0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x36eb8 | store | 4 | `sp -0x24` | `ret(0x47ba4)` |
| `ldr` at 0x36ebc | load | 4 | `abs 0x36f30` | `-` |
| `ldr` at 0x36ec0 | load | 4 | `abs 0x297604` | `-` |
| `ldr` at 0x36ecc | load | 4 | `abs 0x36f30` | `-` |
| `ldr` at 0x36ed0 | load | 4 | `abs 0x297604` | `-` |
| `ldr` at 0x36edc | load | 4 | `sp -0x48` | `-` |
| `strb` at 0x36ee4 | store | 1 | `[sp-72] +0xe` | `0x0` |
| `ldr` at 0x36ee8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x36efc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x36f00 | store | 4 | `sp -0x60` | `[sp-32]` |
| `ldr` at 0x36f04 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x36f10 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x36f14 | store | 4 | `sp -0x60` | `[sp-40]` |
| `ldr` at 0x36f18 | load | 4 | `sp -0x60` | `-` |
| `str` at 0x36f1c | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x36f20 | load | 4 | `sp -0x50` | `-` |
| `ldm` at 0x36f28 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x36f28 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x36f28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3690c | `(unknown callee) 0x488cc` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x36924 | `(unknown callee) 0x23314` | `ret(0x488cc)` | `?` | `?` | `[0x13219c]` |
| `mov` at 0x36944 | `(unknown callee) (unknown target)` | `ret(0x23314)` | `?` | `?` | `[0x297684]` |
| `bl` at 0x3696c | `(unknown callee) 0x200a0` | `0x6` | `0xf565c` | `0x2d5` | `0x0` |
| `bl` at 0x3697c | `(unknown callee) 0x49c2c` | `?` | `?` | `?` | `?` |
| `bl` at 0x369fc | `(unknown callee) 0x23348` | `0x2` | `?` | `?` | `ret(0x49c2c)+1704` |
| `bl` at 0x36a0c | `(unknown callee) 0x22e24` | `0x32` | `[sp-76]` | `0x2` | `?` |
| `bl` at 0x36a20 | `(unknown callee) 0x48054` | `[sp-24]` | `?` | `?` | `ret(0x49c2c)` |
| `bl` at 0x36a54 | `(unknown callee) 0x1e958` | `0xf5834` | `[sp-88]` | `ret(0x48054)` | `?` |
| `bl` at 0x36a5c | `(unknown callee) 0x23348` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x36a70 | `(unknown callee) 0x4abf0` | `0x1` | `0x0` | `?` | `ret(0x23348)` |
| `bl` at 0x36a88 | `(unknown callee) 0x22e24` | `0x32` | `0xf5844` | `0x1` | `ret(0x4abf0)` |
| `bl` at 0x36aa4 | `(unknown callee) 0x47ba4` | `0x5` | `?` | `[sp-72]` | `0x0` |
| `bl` at 0x36ab0 | `(unknown callee) 0x76f1c` | `ret(0x47ba4)` | `?` | `?` | `ret(0x47ba4)` |
| `bl` at 0x36b34 | `(unknown callee) 0x3a1a0` | `0x0` | `[sp-76]` | `[[sp-72]+4]` | `[[sp-76]+2]:1` |
| `bl` at 0x36b50 | `(unknown callee) 0x37174` | `0x5` | `?` | `?` | `ret(0x3a1a0)` |
| `bl` at 0x36b54 | `(unknown callee) 0x76f34` | `ret(0x37174)` | `?` | `?` | `?` |
| `bl` at 0x36bc4 | `(unknown callee) 0x368c4` | `?` | `[sp-72]` | `?` | `?` |
| `bl` at 0x36bd0 | `(unknown callee) 0x368c4` | `0x19` | `[sp-72]` | `?` | `?` |
| `bl` at 0x36be8 | `(unknown callee) 0x368c4` | `0x64` | `?` | `[[sp-72]+8]` | `?` |
| `mov` at 0x36c0c | `(unknown callee) (unknown target)` | `0x0` | `?` | `?` | `[[sp-72]+16]` |
| `bl` at 0x36c44 | `(unknown callee) 0x3b9cc` | `0x0` | `0xff` | `0x3` | `?` |
| `bl` at 0x36c70 | `(unknown callee) 0x76f34` | `ret(0x3b9cc)` | `?` | `?` | `?` |
| `bl` at 0x36de0 | `(unknown callee) 0x47ad8` | `0x5` | `?` | `[sp-72]` | `[sp-53]:1-48` |
| `bl` at 0x36e1c | `(unknown callee) 0x6a754` | `0x1` | `?` | `?` | `[sp-32]` |
| `bl` at 0x36e2c | `(unknown callee) 0x22e24` | `0x32` | `0xf5850` | `0x0` | `?` |
| `bl` at 0x36e48 | `(unknown callee) 0x22e24` | `0x32` | `0xf585c` | `0x2` | `[sp-32]` |
| `bl` at 0x36e58 | `(unknown callee) 0x255ac` | `0x0` | `?` | `?` | `ret(0x22e24)` |
| `bl` at 0x36e60 | `(unknown callee) 0x25744` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x36e68 | `(unknown callee) 0x24da8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x36e94 | `(unknown callee) 0x47ad8` | `0x5` | `?` | `?` | `[sp-53]:1` |
| `bl` at 0x36eb0 | `(unknown callee) 0x47ba4` | `0x5` | `?` | `?` | `[sp-20]` |
| `mov` at 0x36ed8 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `[0x297604]` |
| `bl` at 0x36ef8 | `(unknown callee) 0x7701c` | `0x0` | `?` | `[sp-72]` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x36934 | `[0x297684] == 0x0` | `[0x297684] != 0x0` |
| `beq` at 0x36950 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x36990 | `ret(0x49c2c) == 0x0` | `ret(0x49c2c) != 0x0` |
| `ldrls` at 0x369a4 | `ret(0x49c2c)+1704 <= 0x4 (unsigned, 32-bit)` | `ret(0x49c2c)+1704 > 0x4 (unsigned, 32-bit)` |
| `beq` at 0x36a30 | `[sp-20] == 0x0` | `[sp-20] != 0x0` |
| `ble` at 0x36ad8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x36b18 | `[([[sp-72]+4] + [sp-44])]:1 == 0x0` | `[([[sp-72]+4] + [sp-44])]:1 != 0x0` |
| `beq` at 0x36b28 | `[[sp-76]+2]:1 == 0xff` | `[[sp-76]+2]:1 != 0xff` |
| `beq` at 0x36b48 | `ret(0x3a1a0) == 0x0` | `ret(0x3a1a0) != 0x0` |
| `beq` at 0x36b98 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x36bf8 | `[[sp-72]+16] == 0x0` | `[[sp-72]+16] != 0x0` |
| `beq` at 0x36c20 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x36c2c | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x36c58 | `ret(0x3b9cc) == 0x0` | `ret(0x3b9cc) != 0x0` |
| `bne` at 0x36c64 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x36c88 | `[sp-20] == 0x0` | `[sp-20] != 0x0` |
| `ldrls` at 0x36c98 | `[sp-53]:1-48 <= 0x4c (unsigned, 32-bit)` | `[sp-53]:1-48 > 0x4c (unsigned, 32-bit)` |
| `beq` at 0x36df4 | `ret(0x47ad8) == 0x0` | `ret(0x47ad8) != 0x0` |
| `bne` at 0x36e00 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0x36e14 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x36e7c | `[sp-53]:1 == 0x23` | `[sp-53]:1 != 0x23` |
| `beq` at 0x36e88 | `[sp-53]:1 == 0x3d` | `[sp-53]:1 != 0x3d` |
| `bne` at 0x36ea8 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `beq` at 0x36ec8 | `[0x297604] == 0x0` | `[0x297604] != 0x0` |
| `beq` at 0x36ef0 | `[sp-20] == 0x0` | `[sp-20] != 0x0` |
| `bne` at 0x36f0c | `[sp-32] != 0x0` | `[sp-32] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x36f28 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x369a4).

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x36c98).

## `powerDownSlot`

- `powerDownSlot`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x36f6c to 0x37174.
  Established. The unnamed `0x47ba4` call carries `0x5` and three unknown candidates (`bl` at 0x36fdc). The indirect call has unknown target and candidates `[sp-20]`, unknown, unknown,
  `[[sp-120]+20]` (`mov` at 0x37018). The unnamed `0x3a358` call carries `[sp-20]`, `[sp-124]`,
  unknown, `0x0` (`bl` at 0x37110). Slot shutdown behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x36f7c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x36f80 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x36f88 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x36f8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x36fa0 | load | 4 | `abs 0x3716c` | `-` |
| `ldr` at 0x36fa4 | load | 4 | `abs 0x13219c` | `-` |
| `str` at 0x36fac | store | 4 | `sp -0x78` | `?` |
| `ldr` at 0x36fb0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x36fbc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x36fc8 | load | 4 | `abs 0x37170` | `-` |
| `ldr` at 0x36fcc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x36fe4 | store | 4 | `sp -0x1c` | `ret(0x47ba4)` |
| `ldr` at 0x36fe8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x36ff0 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x36ff4 | load | 4 | `[sp-120] +0x14` | `-` |
| `str` at 0x37004 | store | 4 | `sp -0x80` | `0x0` |
| `ldr` at 0x37008 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x3700c | load | 4 | `[sp-120] +0x14` | `-` |
| `ldr` at 0x37010 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x37028 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x37038 | store | 4 | `sp -0x80` | `0x1` |
| `ldr` at 0x3703c | load | 4 | `sp -0x80` | `-` |
| `str` at 0x37040 | store | 4 | `sp -0x20` | `[sp-128]` |
| `ldr` at 0x3704c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x37050 | store | 4 | `sp -0x1c` | `[sp-128]` |
| `ldr` at 0x37054 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x37058 | load | 4 | `[sp-120] +0x8` | `-` |
| `str` at 0x37060 | store | 4 | `sp -0x24` | `[[sp-120]+8]-1` |
| `ldr` at 0x37064 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x37070 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x37074 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x37088 | load | 4 | `[sp-120] +0x0` | `-` |
| `str` at 0x37090 | store | 4 | `sp -0x7c` | `?` |
| `ldr` at 0x37094 | load | 4 | `sp -0x7c` | `-` |
| `ldrb` at 0x37098 | load | 1 | `[sp-124] +0x2` | `-` |
| `ldr` at 0x370a4 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x370a8 | load | 4 | `[sp-120] +0x4` | `-` |
| `ldr` at 0x370ac | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x370b4 | load | 1 | `([[sp-120]+4] + [sp-36]) +0x0` | `-` |
| `ldr` at 0x370c0 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x370c4 | load | 4 | `[sp-120] +0x4` | `-` |
| `ldr` at 0x370c8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x370d0 | load | 1 | `([[sp-120]+4] + [sp-36]) +0x0` | `-` |
| `ldr` at 0x370dc | load | 4 | `sp -0x7c` | `-` |
| `ldrb` at 0x370e0 | load | 1 | `[sp-124] +0x2` | `-` |
| `ldr` at 0x370e4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x37104 | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x37108 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3710c | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x37120 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x37130 | store | 4 | `sp -0x84` | `0x1` |
| `ldr` at 0x37134 | load | 4 | `sp -0x84` | `-` |
| `str` at 0x37138 | store | 4 | `sp -0x20` | `[sp-132]` |
| `ldr` at 0x37144 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x37148 | store | 4 | `sp -0x1c` | `[sp-132]` |
| `ldr` at 0x3714c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x37154 | store | 4 | `sp -0x24` | `[sp-36]-1` |
| `ldr` at 0x3715c | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x37168 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x37168 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x37168 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x36fc0 | `(unknown callee) 0x48054` | `r1` | `r1` | `?` | `r1` |
| `bl` at 0x36fd4 | `(unknown callee) 0x1e958` | `0xf586c` | `[sp-20]` | `ret(0x48054)` | `ret(0x48054)` |
| `bl` at 0x36fdc | `(unknown callee) 0x47ba4` | `0x5` | `?` | `?` | `?` |
| `bl` at 0x36fec | `(unknown callee) 0x24f6c` | `[sp-20]` | `?` | `?` | `ret(0x47ba4)` |
| `mov` at 0x37018 | `(unknown callee) (unknown target)` | `[sp-20]` | `?` | `?` | `[[sp-120]+20]` |
| `bl` at 0x370ec | `(unknown callee) 0xca484` | `[sp-20]` | `[[sp-124]+2]:1` | `[[sp-120]+4]` | `[[sp-124]+2]:1` |
| `bl` at 0x37110 | `(unknown callee) 0x3a358` | `[sp-20]` | `[sp-124]` | `?` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x36fb8 | `r1 == 0x5` | `r1 != 0x5` |
| `beq` at 0x36ffc | `[[sp-120]+20] == 0x0` | `[[sp-120]+20] != 0x0` |
| `beq` at 0x37024 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x37030 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0x37048 | `[sp-128] == 0x0` | `[sp-128] != 0x0` |
| `blt` at 0x3706c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x370a0 | `[[sp-124]+2]:1 == 0xff` | `[[sp-124]+2]:1 != 0xff` |
| `bne` at 0x370bc | `[([[sp-120]+4] + [sp-36])]:1 != 0x0` | `[([[sp-120]+4] + [sp-36])]:1 == 0x0` |
| `bne` at 0x370d8 | `[([[sp-120]+4] + [sp-36])]:1 != 0x0` | `[([[sp-120]+4] + [sp-36])]:1 == 0x0` |
| `bne` at 0x370f8 | `ret(0xca484) != 0x0` | `ret(0xca484) == 0x0` |
| `beq` at 0x3711c | `ret(0x3a358) == 0x0` | `ret(0x3a358) != 0x0` |
| `bne` at 0x37128 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0x37140 | `[sp-132] == 0x0` | `[sp-132] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x37168 | `r0` | `[sp-28]` |

## `powerDown`

- `powerDown`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x37174 to 0x37558.
  Established. The byte through `[sp-120] +0xc` receives `0x1` on an included path (`strb` at 0x37294). The unnamed `0x6f93c` call carries `0x0`, `0x8000`, `0x0`, `sp-112` (`bl` at 0x37288).
  The indirect call at `0x37520` has unknown target and candidates `ret(0x368c4)`, unknown, unknown,
  `[0x297604]` (`mov` at 0x37520). Shutdown request effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x37184 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3718c | store | 4 | `sp -0x18` | `0x0` |
| `strb` at 0x37198 | store | 1 | `sp -0x71` | `(ret(0x488cc) & 0xff)` |
| `ldr` at 0x3719c | load | 4 | `abs 0x37538` | `-` |
| `ldr` at 0x371a0 | load | 4 | `abs 0x13219c` | `-` |
| `str` at 0x371a4 | store | 4 | `sp -0x78` | `[0x13219c]` |
| `ldrb` at 0x371b8 | load | 1 | `sp -0x71` | `-` |
| `ldr` at 0x371c8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x371cc | store | 4 | `sp -0x84` | `[sp-20]` |
| `ldr` at 0x371d0 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x371dc | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x371e8 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x371f8 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x37210 | load | 4 | `abs 0x3753c` | `-` |
| `ldr` at 0x37224 | load | 4 | `abs 0x37540` | `-` |
| `ldr` at 0x3722c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3723c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x37254 | load | 4 | `sp -0x78` | `-` |
| `ldrb` at 0x37258 | load | 1 | `[sp-120] +0xc` | `-` |
| `str` at 0x3726c | store | 4 | `sp -0x94` | `0x1` |
| `str` at 0x37274 | store | 4 | `sp -0x90` | `0x0` |
| `ldr` at 0x3728c | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x37294 | store | 1 | `[sp-120] +0xc` | `0x1` |
| `str` at 0x3729c | store | 4 | `sp -0x80` | `0x0` |
| `ldr` at 0x372a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x372b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x372bc | load | 4 | `abs 0x37544` | `-` |
| `str` at 0x372d4 | store | 4 | `sp -0x1c` | `ret(0x23458)` |
| `ldr` at 0x372dc | load | 4 | `abs 0x37548` | `-` |
| `str` at 0x372ec | store | 4 | `sp -0x1c` | `ret(0x22e24)` |
| `str` at 0x37300 | store | 4 | `sp -0x18` | `ret(0x47ba4)` |
| `str` at 0x37320 | store | 4 | `sp -0x1c` | `ret(0x3bcc4)` |
| `ldr` at 0x37324 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x37330 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3733c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x37340 | store | 4 | `sp -0x18` | `ret(0x3bcc4)` |
| `ldr` at 0x37344 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x37348 | load | 4 | `[sp-120] +0x14` | `-` |
| `str` at 0x37358 | store | 4 | `sp -0x88` | `0x0` |
| `ldr` at 0x3735c | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x37360 | load | 4 | `[sp-120] +0x14` | `-` |
| `ldr` at 0x3737c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3738c | store | 4 | `sp -0x88` | `0x1` |
| `ldr` at 0x37390 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x37394 | store | 4 | `sp -0x1c` | `[sp-136]` |
| `ldr` at 0x373a0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x373a4 | store | 4 | `sp -0x18` | `[sp-136]` |
| `ldr` at 0x373a8 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x373ac | load | 4 | `[sp-120] +0x8` | `-` |
| `str` at 0x373b4 | store | 4 | `sp -0x20` | `[[sp-120]+8]-1` |
| `ldr` at 0x373b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x373c4 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x373c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x373dc | load | 4 | `[sp-120] +0x0` | `-` |
| `str` at 0x373e4 | store | 4 | `sp -0x7c` | `?` |
| `ldr` at 0x373e8 | load | 4 | `sp -0x7c` | `-` |
| `ldrb` at 0x373ec | load | 1 | `[sp-124] +0x2` | `-` |
| `ldr` at 0x373f8 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x373fc | load | 4 | `[sp-120] +0x4` | `-` |
| `ldr` at 0x37400 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x37408 | load | 1 | `([[sp-120]+4] + [sp-32]) +0x0` | `-` |
| `ldr` at 0x37414 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x37418 | load | 4 | `[sp-120] +0x4` | `-` |
| `ldr` at 0x3741c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x37424 | load | 1 | `([[sp-120]+4] + [sp-32]) +0x0` | `-` |
| `ldr` at 0x37430 | load | 4 | `sp -0x7c` | `-` |
| `ldrb` at 0x37434 | load | 1 | `[sp-124] +0x2` | `-` |
| `str` at 0x37458 | store | 4 | `sp -0x8c` | `0x0` |
| `ldr` at 0x37460 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x37474 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x37484 | store | 4 | `sp -0x8c` | `0x1` |
| `ldr` at 0x37488 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x3748c | store | 4 | `sp -0x1c` | `[sp-140]` |
| `ldr` at 0x37498 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3749c | store | 4 | `sp -0x18` | `[sp-140]` |
| `ldr` at 0x374a0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x374a8 | store | 4 | `sp -0x20` | `[sp-32]-1` |
| `ldr` at 0x374b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x374c0 | load | 4 | `abs 0x3754c` | `-` |
| `str` at 0x374d0 | store | 4 | `sp -0x1c` | `ret(0x22e24)` |
| `ldr` at 0x374dc | load | 4 | `abs 0x37550` | `-` |
| `str` at 0x374ec | store | 4 | `sp -0x1c` | `ret(0x22e24)` |
| `ldr` at 0x374fc | load | 4 | `abs 0x37554` | `-` |
| `ldr` at 0x37500 | load | 4 | `abs 0x297604` | `-` |
| `ldr` at 0x37514 | load | 4 | `abs 0x37554` | `-` |
| `ldr` at 0x37518 | load | 4 | `abs 0x297604` | `-` |
| `ldr` at 0x37524 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x37528 | store | 4 | `sp -0x80` | `[sp-24]` |
| `ldr` at 0x3752c | load | 4 | `sp -0x80` | `-` |
| `ldm` at 0x37534 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x37534 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x37534 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x37190 | `(unknown callee) 0x488cc` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x371a8 | `(unknown callee) 0x7a750` | `ret(0x488cc)` | `?` | `?` | `[0x13219c]` |
| `bl` at 0x37214 | `(unknown callee) 0x197a8` | `sp-112` | `0xf5880` | `?` | `sp-112` |
| `bl` at 0x37228 | `(unknown callee) 0x197a8` | `sp-112` | `0xf58bc` | `?` | `sp-112` |
| `bl` at 0x37288 | `(unknown callee) 0x6f93c` | `0x0` | `0x8000` | `0x0` | `sp-112` |
| `bl` at 0x372b4 | `(unknown callee) 0x48054` | `[sp-20]` | `?` | `?` | `[sp-20]` |
| `bl` at 0x372c4 | `(unknown callee) 0x1e958` | `0xf5900` | `ret(0x48054)` | `?` | `ret(0x48054)` |
| `bl` at 0x372cc | `(unknown callee) 0x23458` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x372e4 | `(unknown callee) 0x22e24` | `0x32` | `0xf5910` | `0x1` | `ret(0x23458)` |
| `bl` at 0x372f0 | `(unknown callee) 0x76f80` | `ret(0x22e24)` | `?` | `?` | `ret(0x22e24)` |
| `bl` at 0x372f8 | `(unknown callee) 0x47ba4` | `0x5` | `?` | `?` | `?` |
| `bl` at 0x37308 | `(unknown callee) 0x24f6c` | `0x0` | `?` | `?` | `ret(0x47ba4)` |
| `bl` at 0x37318 | `(unknown callee) 0x3bcc4` | `0x0` | `0xff` | `0x2` | `?` |
| `mov` at 0x3736c | `(unknown callee) (unknown target)` | `0x0` | `?` | `?` | `[[sp-120]+20]` |
| `bl` at 0x37440 | `(unknown callee) 0xca484` | `0x0` | `[[sp-124]+2]:1` | `[[sp-120]+4]` | `[[sp-124]+2]:1` |
| `bl` at 0x37464 | `(unknown callee) 0x3a358` | `0x0` | `[sp-124]` | `?` | `0x0` |
| `bl` at 0x374c8 | `(unknown callee) 0x22e24` | `0x32` | `0xf591c` | `0x0` | `[sp-24]` |
| `bl` at 0x374e4 | `(unknown callee) 0x22e24` | `0x32` | `0xf592c` | `0x2` | `[sp-24]` |
| `bl` at 0x374f0 | `(unknown callee) 0x76f98` | `ret(0x22e24)` | `?` | `?` | `ret(0x22e24)` |
| `bl` at 0x374f8 | `(unknown callee) 0x6a754` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x37510 | `(unknown callee) 0x368c4` | `0x28` | `?` | `?` | `[0x297604]` |
| `mov` at 0x37520 | `(unknown callee) (unknown target)` | `ret(0x368c4)` | `?` | `?` | `[0x297604]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x371b4 | `ret(0x7a750) == 0x0` | `ret(0x7a750) != 0x0` |
| `beq` at 0x371c0 | `[sp-113]:1 == 0x4c` | `[sp-113]:1 != 0x4c` |
| `beq` at 0x371d8 | `[sp-20] == 0x2` | `[sp-20] != 0x2` |
| `bgt` at 0x371e4 | `[sp-20] > 0x2 (signed, 32-bit)` | `[sp-20] <= 0x2 (signed, 32-bit)` |
| `beq` at 0x371f0 | `[sp-20] == 0x1` | `[sp-20] != 0x1` |
| `beq` at 0x37200 | `[sp-20] == 0x4` | `[sp-20] != 0x4` |
| `bhi` at 0x37234 | `[sp-20] > 0x4 (unsigned, 32-bit)` | `[sp-20] <= 0x4 (unsigned, 32-bit)` |
| `bne` at 0x3724c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x37260 | `[[sp-120]+12]:1 != 0x0` | `[[sp-120]+12]:1 == 0x0` |
| `beq` at 0x372ac | `[sp-20] == 0x5` | `[sp-20] != 0x5` |
| `beq` at 0x3732c | `ret(0x3bcc4) == 0x0` | `ret(0x3bcc4) != 0x0` |
| `bne` at 0x37338 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x37350 | `[[sp-120]+20] == 0x0` | `[[sp-120]+20] != 0x0` |
| `beq` at 0x37378 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x37384 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x3739c | `[sp-136] == 0x0` | `[sp-136] != 0x0` |
| `blt` at 0x373c0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x373f4 | `[[sp-124]+2]:1 == 0xff` | `[[sp-124]+2]:1 != 0xff` |
| `bne` at 0x37410 | `[([[sp-120]+4] + [sp-32])]:1 != 0x0` | `[([[sp-120]+4] + [sp-32])]:1 == 0x0` |
| `bne` at 0x3742c | `[([[sp-120]+4] + [sp-32])]:1 != 0x0` | `[([[sp-120]+4] + [sp-32])]:1 == 0x0` |
| `bne` at 0x3744c | `ret(0xca484) != 0x0` | `ret(0xca484) == 0x0` |
| `beq` at 0x37470 | `ret(0x3a358) == 0x0` | `ret(0x3a358) != 0x0` |
| `bne` at 0x3747c | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x37494 | `[sp-140] == 0x0` | `[sp-140] != 0x0` |
| `bne` at 0x374b8 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x37508 | `[0x297604] == 0x0` | `[0x297604] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x37534 | `r0` | `?` |

## `monitorGetCfg`

- `monitorGetCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbfe74 to 0xbfecc.
  Established. The absolute word at `0x297320` is read (`ldr` at 0xbfe98). The word through entry
  `r1 +0x0` receives `[[sp-28]]` (`str` at 0xbfeb0). The return reports `[[sp-28]+8]` (`ldm` at 0xbfec4). Monitor configuration meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbfe84 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbfe88 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xbfe8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbfe94 | load | 4 | `abs 0xbfec8` | `-` |
| `ldr` at 0xbfe98 | load | 4 | `abs 0x297320` | `-` |
| `str` at 0xbfea0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xbfea4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbfea8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbfeac | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xbfeb0 | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0xbfeb4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbfeb8 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldm` at 0xbfec4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbfec4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbfec4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbfec4 | `r0` | `[[sp-28]+8]` |

## `monitorCount`

- `monitorCount`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbfecc to 0xbff10.
  Established. The word through `[sp-24] +0x8` is read (`ldr` at 0xbfefc). The return reports
  `[[sp-24]+8]` (`ldm` at 0xbff08). Monitor count interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbfedc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xbfee0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbfee8 | load | 4 | `abs 0xbff0c` | `-` |
| `ldr` at 0xbfeec | load | 4 | `abs 0x297320` | `-` |
| `str` at 0xbfef4 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xbfef8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbfefc | load | 4 | `[sp-24] +0x8` | `-` |
| `ldm` at 0xbff08 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbff08 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbff08 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbff08 | `r0` | `[[sp-24]+8]` |

## `monitorSetFlags`

- `monitorSetFlags`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbf88c to 0xbf8d8.
  Established. The byte through `[sp-28] +0xc` receives `((r1 & 0xff) & 0xff)` (`strb` at 0xbf8c8).
  The absolute word at `0x297320` is read (`ldr` at 0xbf8b4). Flag meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbf89c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xbf8a4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xbf8a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbf8b0 | load | 4 | `abs 0xbf8d4` | `-` |
| `ldr` at 0xbf8b4 | load | 4 | `abs 0x297320` | `-` |
| `str` at 0xbf8bc | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xbf8c0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbf8c4 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0xbf8c8 | store | 1 | `[sp-28] +0xc` | `((r1 & 0xff) & 0xff)` |
| `ldm` at 0xbf8d0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbf8d0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbf8d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbf8d0 | `r0` | `r0` |

## `monitorIsPresent`

- `monitorIsPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbfb1c to 0xbfb64.
  Established. The unnamed `0xc1970` call carries entry `r0`, `(r1 & 0xff)`, entry `r2`, `(r1 & 0xff)` (`bl` at 0xbfb44). The byte through `ret(0xc1970) +0x0` is read (`ldrb` at 0xbfb54). The
  return reports `[ret(0xc1970)]:1` (`ldm` at 0xbfb60). Monitor presence classification remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbfb2c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xbfb34 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xbfb38 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xbfb3c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbfb4c | store | 4 | `sp -0x1c` | `ret(0xc1970)` |
| `ldr` at 0xbfb50 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbfb54 | load | 1 | `ret(0xc1970) +0x0` | `-` |
| `ldm` at 0xbfb60 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbfb60 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbfb60 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbfb44 | `(unknown callee) 0xc1970` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbfb60 | `r0` | `[ret(0xc1970)]:1` |

## `monitorIntCount`

- `monitorIntCount`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc1278 to 0xc12c0.
  Established. The word through `ret(0xc1970) +0x4` is read (`ldr` at 0xc12b0). The return reports
  `[ret(0xc1970)+4]` (`ldm` at 0xc12bc). Interrupt count semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc1288 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xc1290 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xc1294 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc1298 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc12a8 | store | 4 | `sp -0x1c` | `ret(0xc1970)` |
| `ldr` at 0xc12ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc12b0 | load | 4 | `ret(0xc1970) +0x4` | `-` |
| `ldm` at 0xc12bc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc12bc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc12bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc12a0 | `(unknown callee) 0xc1970` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc12bc | `r0` | `[ret(0xc1970)+4]` |

## `monitorIntEnable`

- `monitorIntEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc130c to 0xc1358.
  Established. The unnamed `0xc1fa0` call carries entry `r0`, `(r1 & 0xff)`, `(r2 & 0xff)`, `0x1`
  (`bl` at 0xc1344); its result is reported by the return (`ldm` at 0xc1354). Interrupt enable
  effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc131c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xc1324 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xc132c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0xc1330 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xc1334 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xc1338 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xc1354 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc1354 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc1354 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc1344 | `(unknown callee) 0xc1fa0` | `r0` | `(r1 & 0xff)` | `(r2 & 0xff)` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc1354 | `r0` | `ret(0xc1fa0)` |

## `monitorIntDisable`

- `monitorIntDisable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc12c0 to 0xc130c.
  Established. The unnamed `0xc1fa0` call carries entry `r0`, `(r1 & 0xff)`, `(r2 & 0xff)`, `0x0`
  (`bl` at 0xc12f8). The return reports `ret(0xc1fa0)` (`ldm` at 0xc1308). Interrupt disable
  behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc12d0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xc12d8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xc12e0 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0xc12e4 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xc12e8 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xc12ec | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xc1308 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc1308 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc1308 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc12f8 | `(unknown callee) 0xc1fa0` | `r0` | `(r1 & 0xff)` | `(r2 & 0xff)` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc1308 | `r0` | `ret(0xc1fa0)` |

## `monitorName`

- `monitorName`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc1680 to 0xc16ec.
  Established. The unnamed `0xc183c` call carries entry `r0`, `(r1 & 0xff)`, entry `r2`, `(r1 & 0xff)` (`bl` at 0xc16a8). A nonzero `[ret(0xc183c)+8]` takes `0xc16d8`, while zero falls through
  (`bne` at 0xc16cc). The word at `sp -0x20` receives `0x11fc48` on an included path (`str` at 0xc16d4). Monitor label interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc1690 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xc1698 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xc169c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc16a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc16b0 | store | 4 | `sp -0x1c` | `ret(0xc183c)` |
| `ldr` at 0xc16b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc16b8 | load | 4 | `ret(0xc183c) +0x8` | `-` |
| `str` at 0xc16bc | store | 4 | `sp -0x20` | `[ret(0xc183c)+8]` |
| `ldr` at 0xc16c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc16c4 | load | 4 | `ret(0xc183c) +0x8` | `-` |
| `ldr` at 0xc16d0 | load | 4 | `abs 0xc16e8` | `-` |
| `str` at 0xc16d4 | store | 4 | `sp -0x20` | `0x11fc48` |
| `ldr` at 0xc16d8 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xc16e4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc16e4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc16e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc16a8 | `(unknown callee) 0xc183c` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc16cc | `[ret(0xc183c)+8] != 0x0` | `[ret(0xc183c)+8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc16e4 | `r0` | `?` |

## `monitorVoltageLimit`

- `monitorVoltageLimit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc11e4 to 0xc1278.
  Established. The halfword at `sp -0x18` receives `(r2 & 0xffff)` (`strh` at 0xc1200). The unsigned
  condition `[sp-24]:2 <= 0xfe` takes `0xc1250`, while larger values fall through (`bls` at 0xc1240). Included halfword assignments at `sp -0x18` write `0xfe` and `0x1` in separate paths
  (`strh` at 0xc1248, `strh` at 0xc1260). Voltage limit encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc11f4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xc11fc | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strh` at 0xc1200 | store | 2 | `sp -0x18` | `(r2 & 0xffff)` |
| `ldrb` at 0xc1204 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc1208 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc1218 | store | 4 | `sp -0x1c` | `ret(0xc183c)` |
| `ldr` at 0xc121c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1220 | load | 1 | `ret(0xc183c) +0x0` | `-` |
| `ldrh` at 0xc122c | load | 2 | `sp -0x18` | `-` |
| `strh` at 0xc1234 | store | 2 | `sp -0x18` | `?` |
| `ldrh` at 0xc1238 | load | 2 | `sp -0x18` | `-` |
| `strh` at 0xc1248 | store | 2 | `sp -0x18` | `0xfe` |
| `ldrh` at 0xc1250 | load | 2 | `sp -0x18` | `-` |
| `strh` at 0xc1260 | store | 2 | `sp -0x18` | `0x1` |
| `ldrh` at 0xc1264 | load | 2 | `sp -0x18` | `-` |
| `ldm` at 0xc1274 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc1274 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc1274 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc1210 | `(unknown callee) 0xc183c` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc1228 | `[ret(0xc183c)]:1 != 0x2` | `[ret(0xc183c)]:1 == 0x2` |
| `bls` at 0xc1240 | `[sp-24]:2 <= 0xfe (unsigned, 32-bit)` | `[sp-24]:2 > 0xfe (unsigned, 32-bit)` |
| `bne` at 0xc1258 | `[sp-24]:2 != 0x0` | `[sp-24]:2 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc1274 | `r0` | `?` |

## `monitorVoltage`

- `monitorVoltage`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc102c to 0xc11e4.
  Established. The unnamed `0xc183c` call carries entry `r0`, `(r1 & 0xff)`, entry `r2`, `(r1 & 0xff)` (`bl` at 0xc105c).
  A word at `sp -0x20` receives `0xc2c80000` on an included path (`str` at 0xc11a4). The coprocessor load at `sp -0x20` has unknown width (`ldc` at 0xc11a8). Monitor voltage
  conversion remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc103c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xc1044 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xc104c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0xc1050 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc1054 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc1064 | store | 4 | `sp -0x1c` | `ret(0xc183c)` |
| `ldr` at 0xc1068 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc106c | load | 1 | `ret(0xc183c) +0x0` | `-` |
| `str` at 0xc1070 | store | 4 | `sp -0x24` | `[ret(0xc183c)]:1` |
| `ldr` at 0xc1074 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc1080 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc1090 | load | 1 | `sp -0x16` | `-` |
| `ldrls` at 0xc1098 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc1104 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1108 | load | 1 | `ret(0xc183c) +0x3` | `-` |
| `ldrb` at 0xc1118 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xc1124 | load | 4 | `abs 0xc11c8` | `-` |
| `str` at 0xc1128 | store | 4 | `sp -0x20` | `0x3b0ffb8b` |
| `ldrb` at 0xc1130 | load | 1 | `sp -0x16` | `-` |
| `ldrls` at 0xc1138 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc11a0 | load | 4 | `abs 0xc11e0` | `-` |
| `str` at 0xc11a4 | store | 4 | `sp -0x20` | `0xc2c80000` |
| `ldc` at 0xc11a8 | load | unknown | `sp -0x20` | `-` |
| `ldm` at 0xc11b0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc11b0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc11b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc105c | `(unknown callee) 0xc183c` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc107c | `[ret(0xc183c)]:1 == 0x1` | `[ret(0xc183c)]:1 != 0x1` |
| `beq` at 0xc1088 | `[ret(0xc183c)]:1 == 0x2` | `[ret(0xc183c)]:1 != 0x2` |
| `ldrls` at 0xc1098 | `[sp-22]:1 <= 0x9 (unsigned, 32-bit)` | `[sp-22]:1 > 0x9 (unsigned, 32-bit)` |
| `beq` at 0xc1114 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xc1120 | `[sp-22]:1 == 0x2` | `[sp-22]:1 != 0x2` |
| `ldrls` at 0xc1138 | `[sp-22]:1 <= 0x8 (unsigned, 32-bit)` | `[sp-22]:1 > 0x8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc11b0 | `r0` | `ret(0xc183c)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xc1098).

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xc1138).

## `monitorReadTach`

- `monitorReadTach`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc0a10 to 0xc0b5c.
  Established. The word through entry `r3 +0x0` receives zero (`str` at 0xc0a40). A nonzero
  `[ret(0xc1970)]:1` takes `0xc0a90`, while zero falls through (`bne` at 0xc0a80). The unnamed
  `0xc4a60` call carries `[sp-20]`, `[sp-32]`, `[sp-22]:1`, `[sp-28]` (`bl` at 0xc0b30). Tachometer
  read effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc0a20 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc0a24 | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0xc0a2c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xc0a34 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0xc0a38 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc0a40 | store | 4 | `r3 +0x0` | `0x0` |
| `ldrb` at 0xc0a44 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc0a48 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc0a58 | store | 4 | `sp -0x20` | `ret(0xc183c)` |
| `ldrb` at 0xc0a5c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc0a60 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc0a70 | store | 4 | `sp -0x24` | `ret(0xc1970)` |
| `ldr` at 0xc0a74 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc0a78 | load | 1 | `ret(0xc1970) +0x0` | `-` |
| `ldr` at 0xc0a84 | load | 4 | `abs 0xc0b54` | `-` |
| `str` at 0xc0a88 | store | 4 | `sp -0x28` | `0xfffffc11` |
| `ldr` at 0xc0a90 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc0a94 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0xc0a98 | store | 4 | `sp -0x2c` | `[[sp-32]]:1` |
| `ldr` at 0xc0a9c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc0aa8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc0ab4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc0ac4 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xc0ad4 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xc0ad8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0adc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0ae4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc0af0 | store | 4 | `sp -0x28` | `ret(0xc2754)` |
| `ldrb` at 0xc0af8 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xc0afc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0b00 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0b08 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc0b14 | store | 4 | `sp -0x28` | `ret(0xc3368)` |
| `ldrb` at 0xc0b1c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xc0b20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0b24 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0b2c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc0b38 | store | 4 | `sp -0x28` | `ret(0xc4a60)` |
| `ldr` at 0xc0b40 | load | 4 | `abs 0xc0b58` | `-` |
| `str` at 0xc0b44 | store | 4 | `sp -0x28` | `0xfffff830` |
| `ldr` at 0xc0b48 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xc0b50 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc0b50 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc0b50 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc0a50 | `(unknown callee) 0xc183c` | `[sp-20]` | `[sp-21]:1` | `r3` | `[sp-21]:1` |
| `bl` at 0xc0a68 | `(unknown callee) 0xc1970` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0xc0ae8 | `(unknown callee) 0xc2754` | `[sp-20]` | `[sp-32]` | `[sp-22]:1` | `[sp-28]` |
| `bl` at 0xc0b0c | `(unknown callee) 0xc3368` | `[sp-20]` | `[sp-32]` | `[sp-22]:1` | `[sp-28]` |
| `bl` at 0xc0b30 | `(unknown callee) 0xc4a60` | `[sp-20]` | `[sp-32]` | `[sp-22]:1` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc0a80 | `[ret(0xc1970)]:1 != 0x0` | `[ret(0xc1970)]:1 == 0x0` |
| `beq` at 0xc0aa4 | `[[sp-32]]:1 == 0x2` | `[[sp-32]]:1 != 0x2` |
| `bgt` at 0xc0ab0 | `[[sp-32]]:1 > 0x2 (signed, 32-bit)` | `[[sp-32]]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0xc0abc | `[[sp-32]]:1 == 0x1` | `[[sp-32]]:1 != 0x1` |
| `beq` at 0xc0acc | `[[sp-32]]:1 == 0x6` | `[[sp-32]]:1 != 0x6` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc0b50 | `r0` | `?` |
