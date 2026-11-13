# ARM SHUB scan and link paths in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package4/shub-scan-link-facts.txt) retain all tool sections.

## `SHUB_ScanInitLLP`

- `SHUB_ScanInitLLP`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x43088 to 0x4362c.
  Established. The included indirect call carries `[sp-20]`, unknown, unknown, `[0x132238]` (`mov` at 0x431dc).
  Equality `ret(unknown) == 0x1` takes `0x43264`, while inequality falls through (`beq` at 0x431f8).
  Included halfword stores write `0x5` and `0x0` to unknown targets (`strh` at 0x4325c, `strh` at 0x43298).
  LLP scan initialization policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x43098 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x430a0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x430a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x430ac | load | 4 | `abs 0x435d8` | `-` |
| `ldr` at 0x430b0 | load | 4 | `abs 0x132228` | `-` |
| `ldr` at 0x430b8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x430bc | store | 4 | `sp -0x74` | `?` |
| `ldr` at 0x430c0 | load | 4 | `abs 0x435dc` | `-` |
| `ldr` at 0x430c4 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x430d0 | load | 4 | `abs 0x435e0` | `-` |
| `str` at 0x430d4 | store | 4 | `sp -0x90` | `0xf6b00` |
| `ldr` at 0x430d8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x430dc | store | 4 | `sp -0x8c` | `r0` |
| `ldr` at 0x430e4 | load | 4 | `abs 0x435e4` | `-` |
| `ldr` at 0x430e8 | load | 4 | `abs 0x435e8` | `-` |
| `str` at 0x43120 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x43124 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x43130 | load | 4 | `abs 0x435ec` | `-` |
| `ldr` at 0x43134 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4313c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x43144 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x43150 | load | 4 | `abs 0x435dc` | `-` |
| `ldr` at 0x43154 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x43160 | load | 4 | `abs 0x435f0` | `-` |
| `str` at 0x43164 | store | 4 | `sp -0x90` | `0xf6b14` |
| `ldr` at 0x43168 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4316c | store | 4 | `sp -0x8c` | `[sp-20]` |
| `ldr` at 0x43170 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x43174 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x43178 | load | 4 | `abs 0x435f4` | `-` |
| `ldr` at 0x4317c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x43190 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x431a8 | load | 2 | `(unknown)` | `-` |
| `str` at 0x431ac | store | 4 | `sp -0x84` | `?` |
| `ldr` at 0x431b4 | load | 4 | `abs 0x435e4` | `-` |
| `ldr` at 0x431b8 | load | 4 | `abs 0x435f8` | `-` |
| `ldr` at 0x431c8 | load | 4 | `abs 0x435fc` | `-` |
| `ldr` at 0x431cc | load | 4 | `abs 0x132238` | `-` |
| `ldr` at 0x431d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x431d4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x431e4 | store | 4 | `sp -0x78` | `ret(unknown)` |
| `ldr` at 0x431e8 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x431ec | store | 4 | `sp -0x80` | `ret(unknown)` |
| `ldr` at 0x431f0 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x431fc | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x43208 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x43218 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x43228 | load | 4 | `abs 0x435f4` | `-` |
| `ldr` at 0x4322c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x43240 | load | 4 | `sp -0x18` | `-` |
| `strh` at 0x4325c | store | 2 | `(unknown)` | `0x5` |
| `ldr` at 0x43264 | load | 4 | `abs 0x435f4` | `-` |
| `ldr` at 0x43268 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4327c | load | 4 | `sp -0x18` | `-` |
| `strh` at 0x43298 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x432a0 | load | 4 | `abs 0x435dc` | `-` |
| `ldr` at 0x432a4 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x432b0 | load | 4 | `abs 0x43600` | `-` |
| `str` at 0x432b4 | store | 4 | `sp -0x90` | `0xf6b50` |
| `ldr` at 0x432b8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x432bc | store | 4 | `sp -0x8c` | `[sp-20]` |
| `ldr` at 0x432c4 | load | 4 | `abs 0x435e4` | `-` |
| `ldr` at 0x432d4 | load | 4 | `abs 0x435f4` | `-` |
| `ldr` at 0x432d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x432ec | load | 4 | `sp -0x18` | `-` |
| `strh` at 0x43308 | store | 2 | `(unknown)` | `0x5` |
| `ldr` at 0x4330c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x43314 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x4331c | load | 4 | `abs 0x435f4` | `-` |
| `ldr` at 0x43320 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x43340 | load | 2 | `(unknown)` | `-` |
| `str` at 0x43344 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x43348 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4334c | load | 4 | `abs 0x43604` | `-` |
| `ldr` at 0x43358 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x43360 | load | 4 | `abs 0x43604` | `-` |
| `str` at 0x4336c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x43370 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x43374 | load | 4 | `abs 0x43608` | `-` |
| `ldr` at 0x43380 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x43388 | load | 4 | `abs 0x43608` | `-` |
| `str` at 0x43394 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x43398 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4339c | load | 4 | `abs 0x4360c` | `-` |
| `ldr` at 0x433a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x433b0 | load | 4 | `abs 0x4360c` | `-` |
| `str` at 0x433bc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x433c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x433d0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x433e4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x433e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x433f8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4340c | store | 4 | `sp -0x20` | `?` |
| `str` at 0x43418 | store | 4 | `sp -0x90` | `0xd` |
| `ldr` at 0x4341c | load | 4 | `abs 0x43610` | `-` |
| `str` at 0x43420 | store | 4 | `sp -0x8c` | `0xf6b80` |
| `ldr` at 0x43424 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x43440 | load | 4 | `abs 0x43614` | `-` |
| `str` at 0x43444 | store | 4 | `sp -0x90` | `0xf6b8c` |
| `ldr` at 0x43448 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4344c | store | 4 | `sp -0x8c` | `[sp-20]` |
| `ldr` at 0x43454 | load | 4 | `abs 0x435e4` | `-` |
| `ldr` at 0x43458 | load | 4 | `abs 0x43618` | `-` |
| `ldr` at 0x43464 | load | 4 | `abs 0x4361c` | `-` |
| `str` at 0x43468 | store | 4 | `sp -0x7c` | `0xfffff888` |
| `ldr` at 0x43470 | load | 4 | `abs 0x435f4` | `-` |
| `ldr` at 0x43474 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x43494 | load | 2 | `(unknown)` | `-` |
| `str` at 0x43498 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x4349c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x434a0 | load | 4 | `abs 0x43604` | `-` |
| `ldr` at 0x434ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x434b4 | load | 4 | `abs 0x43604` | `-` |
| `str` at 0x434c0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x434c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x434c8 | load | 4 | `abs 0x43608` | `-` |
| `ldr` at 0x434d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x434dc | load | 4 | `abs 0x43608` | `-` |
| `str` at 0x434e8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x434ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x434f0 | load | 4 | `abs 0x4360c` | `-` |
| `ldr` at 0x434fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x43504 | load | 4 | `abs 0x4360c` | `-` |
| `str` at 0x43510 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x43514 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x43524 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x43538 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x4353c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4354c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x43560 | store | 4 | `sp -0x20` | `?` |
| `str` at 0x4356c | store | 4 | `sp -0x90` | `0xd` |
| `ldr` at 0x43570 | load | 4 | `abs 0x43620` | `-` |
| `str` at 0x43574 | store | 4 | `sp -0x8c` | `0xf6bb8` |
| `ldr` at 0x43578 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x43594 | load | 4 | `abs 0x43624` | `-` |
| `str` at 0x43598 | store | 4 | `sp -0x90` | `0xf6bc4` |
| `ldr` at 0x4359c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x435a0 | store | 4 | `sp -0x8c` | `[sp-20]` |
| `ldr` at 0x435a8 | load | 4 | `abs 0x435e4` | `-` |
| `ldr` at 0x435ac | load | 4 | `abs 0x43628` | `-` |
| `ldr` at 0x435b8 | load | 4 | `abs 0x4361c` | `-` |
| `str` at 0x435bc | store | 4 | `sp -0x7c` | `0xfffff888` |
| `str` at 0x435c8 | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0x435cc | load | 4 | `sp -0x7c` | `-` |
| `ldm` at 0x435d4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x435d4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x435d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x430f0 | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x30b` | `0x0` |
| `bl` at 0x43104 | `(unknown callee) 0xd0754` | `0x1` | `0x0` | `sp-112` | `sp-112` |
| `bl` at 0x43118 | `(unknown callee) 0xd093c` | `0x0` | `0x0` | `0x20` | `sp-112` |
| `bl` at 0x431c0 | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x319` | `0x0` |
| `mov` at 0x431dc | `(unknown callee) (unknown target)` | `[sp-20]` | `?` | `?` | `[0x132238]` |
| `bl` at 0x432d0 | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x328` | `0x0` |
| `bl` at 0x43430 | `(unknown callee) 0xde978` | `[sp-116]` | `0xa` | `sp-32` | `0x0` |
| `bl` at 0x43460 | `(unknown callee) 0x200a0` | `0x3` | `0xf67f0` | `0x336` | `0x0` |
| `bl` at 0x43584 | `(unknown callee) 0xde978` | `[sp-116]` | `0xb` | `sp-32` | `0x0` |
| `bl` at 0x435b4 | `(unknown callee) 0x200a0` | `0x3` | `0xf67f0` | `0x33e` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x430cc | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `bgt` at 0x4312c | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x4314c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x4315c | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `beq` at 0x431f8 | `ret(unknown) == 0x1` | `ret(unknown) != 0x1` |
| `bgt` at 0x43204 | `ret(unknown) > 0x1 (signed, 32-bit)` | `ret(unknown) <= 0x1 (signed, 32-bit)` |
| `beq` at 0x43210 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x43220 | `ret(unknown) == 0x2` | `ret(unknown) != 0x2` |
| `beq` at 0x432ac | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `beq` at 0x4343c | `ret(0xde978) == 0x0` | `ret(0xde978) != 0x0` |
| `beq` at 0x43590 | `ret(0xde978) == 0x0` | `ret(0xde978) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x435d4 | `r0` | `?` |

## `SHUB_ScanReadMCILockStatus`

- `SHUB_ScanReadMCILockStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x468c0 to 0x46d3c.
  Established. The unnamed `0xde978` call at `0x46968` carries `[sp-72]`, `0x7`, `sp-48`, `sp-64` (`bl` at 0x46968);
  its zero result takes `0x469a8`, while nonzero falls through (`beq` at 0x46974).
  An included word store assigns `0xfffff890` to `sp -0x4c` (`str` at 0x469a0).
  Lock-status bit interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x468d4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x468dc | store | 4 | `sp -0x1c` | `0x0` |
| `ldrb` at 0x468e0 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x468e8 | load | 4 | `abs 0x46cec` | `-` |
| `ldr` at 0x468ec | load | 4 | `abs 0x132228` | `-` |
| `ldr` at 0x468f4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x468f8 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x468fc | load | 4 | `abs 0x46cf0` | `-` |
| `ldr` at 0x46900 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x4690c | load | 4 | `abs 0x46cf4` | `-` |
| `str` at 0x46910 | store | 4 | `sp -0x5c` | `0xf7498` |
| `ldr` at 0x46918 | load | 4 | `abs 0x46cf8` | `-` |
| `ldr` at 0x4691c | load | 4 | `abs 0x46cfc` | `-` |
| `ldrb` at 0x46930 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x46950 | store | 4 | `sp -0x5c` | `0x4c` |
| `ldr` at 0x46954 | load | 4 | `abs 0x46d00` | `-` |
| `str` at 0x46958 | store | 4 | `sp -0x58` | `0xf74b8` |
| `ldr` at 0x4695c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x46978 | load | 4 | `abs 0x46d04` | `-` |
| `str` at 0x4697c | store | 4 | `sp -0x5c` | `0xf74c0` |
| `ldrb` at 0x46980 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x46984 | store | 4 | `sp -0x58` | `[sp-17]:1` |
| `ldr` at 0x4698c | load | 4 | `abs 0x46cf8` | `-` |
| `ldr` at 0x46990 | load | 4 | `abs 0x46d08` | `-` |
| `ldr` at 0x4699c | load | 4 | `abs 0x46d0c` | `-` |
| `str` at 0x469a0 | store | 4 | `sp -0x4c` | `0xfffff890` |
| `ldrb` at 0x469a8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x469d0 | store | 4 | `sp -0x5c` | `0x4c` |
| `ldr` at 0x469d4 | load | 4 | `abs 0x46d00` | `-` |
| `str` at 0x469d8 | store | 4 | `sp -0x58` | `0xf74b8` |
| `ldr` at 0x469dc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x469f8 | load | 4 | `abs 0x46d04` | `-` |
| `str` at 0x469fc | store | 4 | `sp -0x5c` | `0xf74c0` |
| `ldrb` at 0x46a00 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x46a04 | store | 4 | `sp -0x58` | `[sp-17]:1` |
| `ldr` at 0x46a0c | load | 4 | `abs 0x46cf8` | `-` |
| `ldr` at 0x46a10 | load | 4 | `abs 0x46d10` | `-` |
| `ldr` at 0x46a1c | load | 4 | `abs 0x46d0c` | `-` |
| `str` at 0x46a20 | store | 4 | `sp -0x4c` | `0xfffff890` |
| `ldrb` at 0x46a28 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x46a3c | load | 4 | `abs 0x46d14` | `-` |
| `ldrb` at 0x46a40 | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0x46a58 | load | 2 | `(unknown)` | `-` |
| `strh` at 0x46a5c | store | 2 | `sp -0x42` | `?` |
| `ldr` at 0x46a60 | load | 4 | `abs 0x46d18` | `-` |
| `ldrb` at 0x46a64 | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0x46a7c | load | 2 | `(unknown)` | `-` |
| `strh` at 0x46a80 | store | 2 | `sp -0x44` | `?` |
| `ldrh` at 0x46a84 | load | 2 | `sp -0x42` | `-` |
| `ldrh` at 0x46a88 | load | 2 | `sp -0x44` | `-` |
| `strh` at 0x46a90 | store | 2 | `sp -0x42` | `?` |
| `ldrh` at 0x46a94 | load | 2 | `sp -0x42` | `-` |
| `strh` at 0x46aa0 | store | 2 | `sp -0x42` | `?` |
| `ldrh` at 0x46aa4 | load | 2 | `sp -0x42` | `-` |
| `ldr` at 0x46ab0 | load | 4 | `abs 0x46d1c` | `-` |
| `ldrb` at 0x46abc | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0x46ac0 | load | 2 | `sp -0x42` | `-` |
| `ldr` at 0x46ac4 | load | 4 | `abs 0x46d20` | `-` |
| `str` at 0x46ad4 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x46ad8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x46ae4 | load | 4 | `abs 0x46cf0` | `-` |
| `ldr` at 0x46ae8 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x46af4 | load | 4 | `abs 0x46d24` | `-` |
| `str` at 0x46af8 | store | 4 | `sp -0x5c` | `0xf7528` |
| `ldrb` at 0x46afc | load | 1 | `sp -0x11` | `-` |
| `str` at 0x46b00 | store | 4 | `sp -0x58` | `[sp-17]:1` |
| `ldr` at 0x46b04 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x46b08 | store | 4 | `sp -0x54` | `?` |
| `ldrh` at 0x46b0c | load | 2 | `sp -0x42` | `-` |
| `str` at 0x46b14 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x46b1c | load | 4 | `abs 0x46cf8` | `-` |
| `ldr` at 0x46b20 | load | 4 | `abs 0x46d28` | `-` |
| `ldrh` at 0x46b2c | load | 2 | `sp -0x42` | `-` |
| `strh` at 0x46b34 | store | 2 | `sp -0x42` | `?` |
| `ldr` at 0x46b38 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x46b40 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x46b48 | load | 4 | `abs 0x46d14` | `-` |
| `ldrb` at 0x46b4c | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0x46b64 | load | 2 | `(unknown)` | `-` |
| `strh` at 0x46b68 | store | 2 | `sp -0x42` | `?` |
| `ldr` at 0x46b6c | load | 4 | `abs 0x46d18` | `-` |
| `ldrb` at 0x46b70 | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0x46b88 | load | 2 | `(unknown)` | `-` |
| `strh` at 0x46b8c | store | 2 | `sp -0x44` | `?` |
| `ldrh` at 0x46b90 | load | 2 | `sp -0x42` | `-` |
| `ldrh` at 0x46b94 | load | 2 | `sp -0x44` | `-` |
| `strh` at 0x46b9c | store | 2 | `sp -0x42` | `?` |
| `ldrh` at 0x46ba0 | load | 2 | `sp -0x42` | `-` |
| `ldr` at 0x46bac | load | 4 | `abs 0x46d1c` | `-` |
| `ldrb` at 0x46bb8 | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0x46bbc | load | 2 | `sp -0x42` | `-` |
| `ldr` at 0x46bc0 | load | 4 | `abs 0x46d2c` | `-` |
| `str` at 0x46bd0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x46bd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x46be0 | load | 4 | `abs 0x46cf0` | `-` |
| `ldr` at 0x46be4 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x46bf0 | load | 4 | `abs 0x46d30` | `-` |
| `str` at 0x46bf4 | store | 4 | `sp -0x5c` | `0xf7580` |
| `ldrb` at 0x46bf8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x46bfc | store | 4 | `sp -0x58` | `[sp-17]:1` |
| `ldr` at 0x46c00 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x46c04 | store | 4 | `sp -0x54` | `?` |
| `ldrh` at 0x46c08 | load | 2 | `sp -0x42` | `-` |
| `str` at 0x46c10 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x46c18 | load | 4 | `abs 0x46cf8` | `-` |
| `ldr` at 0x46c1c | load | 4 | `abs 0x46d34` | `-` |
| `ldrh` at 0x46c28 | load | 2 | `sp -0x42` | `-` |
| `strh` at 0x46c30 | store | 2 | `sp -0x42` | `?` |
| `ldr` at 0x46c34 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x46c3c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x46c44 | load | 4 | `abs 0x46d14` | `-` |
| `ldrb` at 0x46c48 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x46c68 | load | 1 | `(unknown)` | `-` |
| `strh` at 0x46c6c | store | 2 | `sp -0x42` | `?` |
| `ldr` at 0x46c70 | load | 4 | `abs 0x46d18` | `-` |
| `ldrb` at 0x46c74 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x46c94 | load | 1 | `(unknown)` | `-` |
| `strh` at 0x46c98 | store | 2 | `sp -0x44` | `?` |
| `ldrh` at 0x46c9c | load | 2 | `sp -0x42` | `-` |
| `ldrh` at 0x46ca0 | load | 2 | `sp -0x44` | `-` |
| `strh` at 0x46ca8 | store | 2 | `sp -0x42` | `?` |
| `ldrh` at 0x46cac | load | 2 | `sp -0x42` | `-` |
| `strh` at 0x46cb8 | store | 2 | `sp -0x42` | `?` |
| `ldrh` at 0x46cbc | load | 2 | `sp -0x42` | `-` |
| `ldrb` at 0x46cc8 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x46ccc | load | 4 | `abs 0x46d38` | `-` |
| `str` at 0x46cdc | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0x46ce0 | load | 4 | `sp -0x4c` | `-` |
| `ldm` at 0x46ce8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x46ce8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x46ce8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x46924 | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x6be` | `0x0` |
| `bl` at 0x4692c | `(unknown callee) 0x428b0` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x46940 | `(unknown callee) 0x46010` | `[sp-17]:1` | `sp-48` | `sp-48` | `[sp-17]:1` |
| `bl` at 0x46968 | `(unknown callee) 0xde978` | `[sp-72]` | `0x7` | `sp-48` | `sp-64` |
| `bl` at 0x46998 | `(unknown callee) 0x200a0` | `0x3` | `0xf67f0` | `0x6cb` | `0x0` |
| `bl` at 0x469b8 | `(unknown callee) 0x46418` | `[sp-17]:1` | `0x1` | `sp-64` | `[sp-17]:1` |
| `bl` at 0x469c0 | `(unknown callee) 0x428b0` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x469e8 | `(unknown callee) 0xde978` | `[sp-72]` | `?` | `sp-48` | `sp-64` |
| `bl` at 0x46a18 | `(unknown callee) 0x200a0` | `0x3` | `0xf67f0` | `0x6de` | `0x0` |
| `bl` at 0x46a38 | `(unknown callee) 0x46418` | `[sp-17]:1` | `0x2` | `sp-64` | `[sp-17]:1` |
| `bl` at 0x46acc | `(unknown callee) 0x1e958` | `0xf74ec` | `[sp-17]:1` | `[sp-66]:2` | `[sp-17]:1` |
| `bl` at 0x46b28 | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x6f2` | `0x0` |
| `bl` at 0x46bc8 | `(unknown callee) 0x1e958` | `0xf7544` | `[sp-17]:1` | `[sp-66]:2` | `[sp-17]:1` |
| `bl` at 0x46c24 | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x6fc` | `0x0` |
| `bl` at 0x46cd4 | `(unknown callee) 0x1e958` | `0xf759c` | `[sp-17]:1` | `[sp-66]:2` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x46908 | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `beq` at 0x46974 | `ret(0xde978) == 0x0` | `ret(0xde978) != 0x0` |
| `beq` at 0x469f4 | `ret(0xde978) == 0x0` | `ret(0xde978) != 0x0` |
| `beq` at 0x46ab8 | `? == 0x1ff` | `? != 0x1ff` |
| `bgt` at 0x46ae0 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x46af0 | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `beq` at 0x46bb4 | `? == 0x1ff` | `? != 0x1ff` |
| `bgt` at 0x46bdc | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x46bec | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `beq` at 0x46cc4 | `[sp-66]:2 == 0x1` | `[sp-66]:2 != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x46ce8 | `r0` | `?` |

## `SHUB_ScanInitConfig`

- `SHUB_ScanInitConfig`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x44c98 to 0x45714.
  Established. The unnamed `0xde978` call at `0x44d90` carries `[sp-168]`, `0x7c`, `0x0`, `0x0` (`bl` at 0x44d90);
  its zero result takes `0x44dd0`, while nonzero falls through (`beq` at 0x44d9c).
  An included absolute halfword store writes `0x320` to `0x2997b0` (`strh` at 0x44e24).
  Scan configuration sequencing remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x44ca8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x44cac | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x44cb8 | store | 4 | `sp -0xa4` | `0x0` |
| `str` at 0x44cc4 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0x44cd0 | store | 4 | `sp -0x9c` | `0x0` |
| `str` at 0x44cdc | store | 4 | `sp -0x98` | `0x0` |
| `str` at 0x44ce8 | store | 4 | `sp -0x94` | `0x0` |
| `str` at 0x44cf4 | store | 4 | `sp -0x90` | `0x0` |
| `str` at 0x44d00 | store | 4 | `sp -0x8c` | `0x0` |
| `str` at 0x44d0c | store | 4 | `sp -0x88` | `0x0` |
| `ldr` at 0x44d10 | load | 4 | `abs 0x45658` | `-` |
| `str` at 0x44d14 | store | 4 | `sp -0xa4` | `0x7ffffd28` |
| `ldr` at 0x44d18 | load | 4 | `abs 0x4565c` | `-` |
| `str` at 0x44d1c | store | 4 | `sp -0xa0` | `0x602800` |
| `ldr` at 0x44d20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x44d28 | load | 4 | `abs 0x45660` | `-` |
| `ldr` at 0x44d2c | load | 4 | `abs 0x132228` | `-` |
| `ldr` at 0x44d34 | load | 4 | `(unknown)` | `-` |
| `str` at 0x44d38 | store | 4 | `sp -0xa8` | `?` |
| `ldr` at 0x44d3c | load | 4 | `abs 0x45664` | `-` |
| `ldr` at 0x44d40 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x44d4c | load | 4 | `abs 0x45668` | `-` |
| `str` at 0x44d50 | store | 4 | `sp -0xc0` | `0xf6f6c` |
| `ldr` at 0x44d54 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x44d58 | store | 4 | `sp -0xbc` | `r0` |
| `ldr` at 0x44d60 | load | 4 | `abs 0x4566c` | `-` |
| `ldr` at 0x44d64 | load | 4 | `abs 0x45670` | `-` |
| `str` at 0x44d74 | store | 4 | `sp -0xc0` | `0x0` |
| `ldr` at 0x44d78 | load | 4 | `abs 0x45674` | `-` |
| `str` at 0x44d7c | store | 4 | `sp -0xbc` | `0xf6f88` |
| `ldr` at 0x44d80 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x44da0 | load | 4 | `abs 0x45678` | `-` |
| `str` at 0x44da4 | store | 4 | `sp -0xc0` | `0xf6f94` |
| `ldr` at 0x44da8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x44dac | store | 4 | `sp -0xbc` | `[sp-20]` |
| `ldr` at 0x44db4 | load | 4 | `abs 0x4566c` | `-` |
| `ldr` at 0x44db8 | load | 4 | `abs 0x4567c` | `-` |
| `ldr` at 0x44dc4 | load | 4 | `abs 0x45680` | `-` |
| `str` at 0x44dc8 | store | 4 | `sp -0xb4` | `0xfffff887` |
| `ldr` at 0x44dd0 | load | 4 | `abs 0x45684` | `-` |
| `ldrh` at 0x44dd4 | load | 2 | `abs 0x2997b0` | `-` |
| `ldr` at 0x44de0 | load | 4 | `abs 0x45664` | `-` |
| `ldr` at 0x44de4 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x44df0 | load | 4 | `abs 0x45688` | `-` |
| `str` at 0x44df4 | store | 4 | `sp -0xc0` | `0xf6fc4` |
| `ldr` at 0x44df8 | load | 4 | `abs 0x45684` | `-` |
| `ldrh` at 0x44dfc | load | 2 | `abs 0x2997b0` | `-` |
| `str` at 0x44e00 | store | 4 | `sp -0xbc` | `[0x2997b0]:2` |
| `ldr` at 0x44e08 | load | 4 | `abs 0x4566c` | `-` |
| `ldr` at 0x44e0c | load | 4 | `abs 0x4568c` | `-` |
| `ldr` at 0x44e1c | load | 4 | `abs 0x45684` | `-` |
| `strh` at 0x44e24 | store | 2 | `abs 0x2997b0` | `0x320` |
| `ldr` at 0x44e28 | load | 4 | `abs 0x45690` | `-` |
| `ldrb` at 0x44e2c | load | 1 | `abs 0x13222c` | `-` |
| `str` at 0x44e3c | store | 4 | `sp -0xb0` | `0x0` |
| `ldr` at 0x44e40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x44e48 | load | 4 | `abs 0x45660` | `-` |
| `ldr` at 0x44e4c | load | 4 | `abs 0x132228` | `-` |
| `ldrb` at 0x44e54 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x44e58 | load | 4 | `abs 0x45690` | `-` |
| `ldrb` at 0x44e5c | load | 1 | `abs 0x13222c` | `-` |
| `ldr` at 0x44e64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x44e78 | load | 4 | `abs 0x45694` | `-` |
| `ldr` at 0x44e7c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x44e9c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x44ea8 | load | 4 | `abs 0x45694` | `-` |
| `ldr` at 0x44eac | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x44ecc | load | 1 | `(unknown)` | `-` |
| `str` at 0x44ed0 | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0x44ed4 | load | 4 | `abs 0x45664` | `-` |
| `ldr` at 0x44ed8 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x44ee4 | load | 4 | `abs 0x45698` | `-` |
| `str` at 0x44ee8 | store | 4 | `sp -0xc0` | `0xf6fdc` |
| `ldr` at 0x44eec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x44ef0 | store | 4 | `sp -0xbc` | `[sp-20]` |
| `ldr` at 0x44ef4 | load | 4 | `sp -0xb0` | `-` |
| `str` at 0x44ef8 | store | 4 | `sp -0xb8` | `[sp-176]` |
| `ldr` at 0x44f00 | load | 4 | `abs 0x4566c` | `-` |
| `ldr` at 0x44f04 | load | 4 | `abs 0x4569c` | `-` |
| `ldr` at 0x44f14 | load | 4 | `sp -0xac` | `-` |
| `ldrb` at 0x44f18 | load | 1 | `[sp-172] +0x6` | `-` |
| `str` at 0x44f1c | store | 4 | `sp -0xb0` | `[[sp-172]+6]:1` |
| `ldr` at 0x44f20 | load | 4 | `abs 0x45664` | `-` |
| `ldr` at 0x44f24 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x44f30 | load | 4 | `abs 0x456a0` | `-` |
| `str` at 0x44f34 | store | 4 | `sp -0xc0` | `0xf7000` |
| `ldr` at 0x44f38 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x44f3c | store | 4 | `sp -0xbc` | `[sp-20]` |
| `ldr` at 0x44f40 | load | 4 | `sp -0xb0` | `-` |
| `str` at 0x44f44 | store | 4 | `sp -0xb8` | `[[sp-172]+6]:1` |
| `ldr` at 0x44f4c | load | 4 | `abs 0x4566c` | `-` |
| `ldr` at 0x44f50 | load | 4 | `abs 0x456a4` | `-` |
| `ldr` at 0x44f5c | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x44f6c | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x44f84 | load | 4 | `abs 0x45694` | `-` |
| `ldr` at 0x44f88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x44fa8 | load | 4 | `sp -0xb0` | `-` |
| `strb` at 0x44fac | store | 1 | `(unknown)` | `([sp-176] & 0xff)` |
| `ldr` at 0x44fb4 | load | 4 | `abs 0x456a8` | `-` |
| `str` at 0x44fb8 | store | 4 | `sp -0xc0` | `0xf7018` |
| `ldr` at 0x44fbc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x44fc0 | store | 4 | `sp -0xbc` | `[sp-20]` |
| `ldr` at 0x44fc8 | load | 4 | `abs 0x4566c` | `-` |
| `ldr` at 0x44fcc | load | 4 | `abs 0x456ac` | `-` |
| `ldr` at 0x44fd8 | load | 4 | `abs 0x45680` | `-` |
| `str` at 0x44fdc | store | 4 | `sp -0xb4` | `0xfffff887` |
| `ldr` at 0x44fe4 | load | 4 | `abs 0x456b0` | `-` |
| `ldr` at 0x44fe8 | load | 4 | `abs 0x132244` | `-` |
| `ldr` at 0x44ff4 | load | 4 | `abs 0x45664` | `-` |
| `ldr` at 0x44ff8 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x45004 | load | 4 | `abs 0x456b4` | `-` |
| `str` at 0x45008 | store | 4 | `sp -0xc0` | `0xf706c` |
| `ldr` at 0x4500c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x45010 | store | 4 | `sp -0xbc` | `[sp-20]` |
| `ldr` at 0x45014 | load | 4 | `abs 0x45694` | `-` |
| `ldr` at 0x45018 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x45038 | load | 1 | `(unknown)` | `-` |
| `str` at 0x4503c | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0x45044 | load | 4 | `abs 0x4566c` | `-` |
| `ldr` at 0x45048 | load | 4 | `abs 0x456b8` | `-` |
| `ldr` at 0x45058 | load | 4 | `abs 0x45694` | `-` |
| `ldr` at 0x4505c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4507c | load | 4 | `sp -0xac` | `-` |
| `ldrb` at 0x45080 | load | 1 | `[sp-172] +0xc` | `-` |
| `strb` at 0x45084 | store | 1 | `(unknown)` | `([[sp-172]+12]:1 & 0xff)` |
| `ldr` at 0x45088 | load | 4 | `abs 0x45664` | `-` |
| `ldr` at 0x4508c | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x45098 | load | 4 | `abs 0x456bc` | `-` |
| `str` at 0x4509c | store | 4 | `sp -0xc0` | `0xf7090` |
| `ldr` at 0x450a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x450a4 | store | 4 | `sp -0xbc` | `[sp-20]` |
| `ldr` at 0x450a8 | load | 4 | `abs 0x45694` | `-` |
| `ldr` at 0x450ac | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x450cc | load | 1 | `(unknown)` | `-` |
| `str` at 0x450d0 | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0x450d8 | load | 4 | `abs 0x4566c` | `-` |
| `ldr` at 0x450dc | load | 4 | `abs 0x456c0` | `-` |
| `ldr` at 0x450e8 | load | 4 | `abs 0x45694` | `-` |
| `ldr` at 0x450ec | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x4510c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x45118 | load | 4 | `abs 0x45664` | `-` |
| `ldr` at 0x4511c | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x45128 | load | 4 | `abs 0x456c4` | `-` |
| `str` at 0x4512c | store | 4 | `sp -0xc0` | `0xf70b4` |
| `ldr` at 0x45130 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x45134 | store | 4 | `sp -0xbc` | `[sp-20]` |
| `ldr` at 0x45138 | load | 4 | `abs 0x45694` | `-` |
| `ldr` at 0x4513c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x4515c | load | 2 | `(unknown)` | `-` |
| `str` at 0x45160 | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0x45168 | load | 4 | `abs 0x4566c` | `-` |
| `ldr` at 0x4516c | load | 4 | `abs 0x456c8` | `-` |
| `ldr` at 0x4517c | load | 4 | `abs 0x45694` | `-` |
| `ldr` at 0x45180 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x451a0 | load | 4 | `sp -0xac` | `-` |
| `ldrh` at 0x451a4 | load | 2 | `[sp-172] +0x8` | `-` |
| `strh` at 0x451a8 | store | 2 | `(unknown)` | `([[sp-172]+8]:2 & 0xffff)` |
| `ldr` at 0x451ac | load | 4 | `abs 0x45664` | `-` |
| `ldr` at 0x451b0 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x451bc | load | 4 | `abs 0x456cc` | `-` |
| `str` at 0x451c0 | store | 4 | `sp -0xc0` | `0xf70e0` |
| `ldr` at 0x451c4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x451c8 | store | 4 | `sp -0xbc` | `[sp-20]` |
| `ldr` at 0x451cc | load | 4 | `abs 0x45694` | `-` |
| `ldr` at 0x451d0 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x451f0 | load | 2 | `(unknown)` | `-` |
| `str` at 0x451f4 | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0x451fc | load | 4 | `abs 0x4566c` | `-` |
| `ldr` at 0x45200 | load | 4 | `abs 0x456d0` | `-` |
| `ldr` at 0x4520c | load | 4 | `abs 0x45694` | `-` |
| `ldr` at 0x45210 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x45230 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x4523c | load | 4 | `abs 0x45664` | `-` |
| `ldr` at 0x45240 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x4524c | load | 4 | `abs 0x456d4` | `-` |
| `str` at 0x45250 | store | 4 | `sp -0xc0` | `0xf7100` |
| `ldr` at 0x45254 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x45258 | store | 4 | `sp -0xbc` | `[sp-20]` |
| `ldr` at 0x4525c | load | 4 | `abs 0x45694` | `-` |
| `ldr` at 0x45260 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x45280 | load | 2 | `(unknown)` | `-` |
| `str` at 0x45284 | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0x4528c | load | 4 | `abs 0x4566c` | `-` |
| `ldr` at 0x452a0 | load | 4 | `abs 0x45694` | `-` |
| `ldr` at 0x452a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x452c4 | load | 4 | `sp -0xac` | `-` |
| `ldrh` at 0x452c8 | load | 2 | `[sp-172] +0xa` | `-` |
| `strh` at 0x452cc | store | 2 | `(unknown)` | `([[sp-172]+10]:2 & 0xffff)` |
| `ldr` at 0x452d0 | load | 4 | `abs 0x45664` | `-` |
| `ldr` at 0x452d4 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x452e0 | load | 4 | `abs 0x456d8` | `-` |
| `str` at 0x452e4 | store | 4 | `sp -0xc0` | `0xf712c` |
| `ldr` at 0x452e8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x452ec | store | 4 | `sp -0xbc` | `[sp-20]` |
| `ldr` at 0x452f0 | load | 4 | `abs 0x45694` | `-` |
| `ldr` at 0x452f4 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x45314 | load | 2 | `(unknown)` | `-` |
| `str` at 0x45318 | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0x45320 | load | 4 | `abs 0x4566c` | `-` |
| `ldr` at 0x45324 | load | 4 | `abs 0x456dc` | `-` |
| `ldr` at 0x45334 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x45348 | store | 4 | `sp -0xc0` | `0x38` |
| `ldr` at 0x4534c | load | 4 | `abs 0x456e0` | `-` |
| `str` at 0x45350 | store | 4 | `sp -0xbc` | `0xf714c` |
| `ldr` at 0x45354 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x45370 | load | 4 | `abs 0x456e4` | `-` |
| `str` at 0x45374 | store | 4 | `sp -0xc0` | `0xf7154` |
| `ldr` at 0x45378 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4537c | store | 4 | `sp -0xbc` | `[sp-20]` |
| `ldr` at 0x45384 | load | 4 | `abs 0x4566c` | `-` |
| `ldr` at 0x45388 | load | 4 | `abs 0x456e8` | `-` |
| `ldr` at 0x45394 | load | 4 | `abs 0x45680` | `-` |
| `str` at 0x45398 | store | 4 | `sp -0xb4` | `0xfffff887` |
| `ldr` at 0x453a0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x453d8 | store | 4 | `sp -0xb4` | `0x0` |
| `ldr` at 0x453e0 | load | 4 | `abs 0x45694` | `-` |
| `ldr` at 0x453e4 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x45404 | load | 2 | `(unknown)` | `-` |
| `str` at 0x45460 | store | 4 | `sp -0xc0` | `0xf2` |
| `ldr` at 0x45464 | load | 4 | `abs 0x456ec` | `-` |
| `str` at 0x45468 | store | 4 | `sp -0xbc` | `0xf717c` |
| `ldr` at 0x4546c | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x45488 | load | 4 | `abs 0x456f0` | `-` |
| `str` at 0x4548c | store | 4 | `sp -0xc0` | `0xf718c` |
| `ldr` at 0x45490 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x45494 | store | 4 | `sp -0xbc` | `[sp-20]` |
| `ldr` at 0x4549c | load | 4 | `abs 0x4566c` | `-` |
| `ldr` at 0x454ac | load | 4 | `abs 0x45680` | `-` |
| `str` at 0x454b0 | store | 4 | `sp -0xb4` | `0xfffff887` |
| `ldr` at 0x454b8 | load | 4 | `abs 0x45694` | `-` |
| `ldr` at 0x454bc | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x454dc | load | 2 | `(unknown)` | `-` |
| `str` at 0x45538 | store | 4 | `sp -0xc0` | `0xf2` |
| `ldr` at 0x4553c | load | 4 | `abs 0x456f4` | `-` |
| `str` at 0x45540 | store | 4 | `sp -0xbc` | `0xf71bc` |
| `ldr` at 0x45544 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x45560 | load | 4 | `abs 0x456f8` | `-` |
| `str` at 0x45564 | store | 4 | `sp -0xc0` | `0xf71cc` |
| `ldr` at 0x45568 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4556c | store | 4 | `sp -0xbc` | `[sp-20]` |
| `ldr` at 0x45574 | load | 4 | `abs 0x4566c` | `-` |
| `ldr` at 0x45578 | load | 4 | `abs 0x456fc` | `-` |
| `ldr` at 0x45584 | load | 4 | `abs 0x45680` | `-` |
| `str` at 0x45588 | store | 4 | `sp -0xb4` | `0xfffff887` |
| `str` at 0x45594 | store | 4 | `sp -0xc0` | `0x0` |
| `ldr` at 0x45598 | load | 4 | `abs 0x45700` | `-` |
| `str` at 0x4559c | store | 4 | `sp -0xbc` | `0xf71fc` |
| `ldr` at 0x455a0 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x455c0 | load | 4 | `abs 0x45704` | `-` |
| `str` at 0x455c4 | store | 4 | `sp -0xc0` | `0xf7208` |
| `ldr` at 0x455c8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x455cc | store | 4 | `sp -0xbc` | `[sp-20]` |
| `ldr` at 0x455d4 | load | 4 | `abs 0x4566c` | `-` |
| `ldr` at 0x455d8 | load | 4 | `abs 0x45708` | `-` |
| `ldr` at 0x455e4 | load | 4 | `abs 0x45680` | `-` |
| `str` at 0x455e8 | store | 4 | `sp -0xb4` | `0xfffff887` |
| `ldr` at 0x455f0 | load | 4 | `abs 0x45664` | `-` |
| `ldr` at 0x455f4 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x45600 | load | 4 | `abs 0x4570c` | `-` |
| `str` at 0x45604 | store | 4 | `sp -0xc0` | `0xf7234` |
| `ldr` at 0x4560c | load | 4 | `abs 0x4566c` | `-` |
| `ldr` at 0x45610 | load | 4 | `abs 0x45710` | `-` |
| `str` at 0x45648 | store | 4 | `sp -0xb4` | `0x0` |
| `ldr` at 0x4564c | load | 4 | `sp -0xb4` | `-` |
| `ldm` at 0x45654 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x45654 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x45654 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x44d6c | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x4c2` | `0x0` |
| `bl` at 0x44d90 | `(unknown callee) 0xde978` | `[sp-168]` | `0x7c` | `0x0` | `0x0` |
| `bl` at 0x44dc0 | `(unknown callee) 0x200a0` | `0x3` | `0xf67f0` | `0x4ca` | `0x0` |
| `bl` at 0x44e14 | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x4d1` | `0x0` |
| `bl` at 0x44e74 | `(unknown callee) 0x4362c` | `[sp-20]` | `?` | `[0x13222c]:1` | `sp-172` |
| `bl` at 0x44f0c | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x4ee` | `0x0` |
| `bl` at 0x44f58 | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x4f1` | `0x0` |
| `bl` at 0x44fd4 | `(unknown callee) 0x200a0` | `0x3` | `0xf67f0` | `0x4fd` | `0x0` |
| `bl` at 0x45050 | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x507` | `0x0` |
| `bl` at 0x450e4 | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x50a` | `0x0` |
| `bl` at 0x45174 | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x513` | `0x0` |
| `bl` at 0x45208 | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x517` | `0x0` |
| `bl` at 0x45298 | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x520` | `0x0` |
| `bl` at 0x4532c | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x524` | `0x0` |
| `bl` at 0x4533c | `(unknown callee) 0x44604` | `[sp-20]` | `sp-132` | `?` | `sp-132` |
| `bl` at 0x45360 | `(unknown callee) 0xde978` | `[sp-168]` | `0x7e` | `sp-132` | `0x0` |
| `bl` at 0x45390 | `(unknown callee) 0x200a0` | `0x3` | `0xf67f0` | `0x531` | `0x0` |
| `bl` at 0x453bc | `(unknown callee) 0xd0754` | `0x1` | `0x0` | `sp-124` | `sp-124` |
| `bl` at 0x453d0 | `(unknown callee) 0xd093c` | `0x0` | `0x0` | `0x20` | `sp-124` |
| `bl` at 0x45420 | `(unknown callee) 0xd5914` | `0x29` | `sp-164` | `[sp-20]` | `sp-164` |
| `bl` at 0x45430 | `(unknown callee) 0xd5914` | `0x2a` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x45444 | `(unknown callee) 0xd598c` | `0x29` | `sp-164` | `[sp-20]` | `sp-164` |
| `bl` at 0x45454 | `(unknown callee) 0xd598c` | `0x2a` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x45478 | `(unknown callee) 0xde978` | `[sp-168]` | `0x1a` | `sp-164` | `0x0` |
| `bl` at 0x454a8 | `(unknown callee) 0x200a0` | `0x3` | `0xf67f0` | `0x550` | `0x0` |
| `bl` at 0x454f8 | `(unknown callee) 0xd5914` | `0x29` | `sp-164` | `[sp-20]` | `sp-164` |
| `bl` at 0x45508 | `(unknown callee) 0xd5914` | `0x2a` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x4551c | `(unknown callee) 0xd598c` | `0x29` | `sp-164` | `[sp-20]` | `sp-164` |
| `bl` at 0x4552c | `(unknown callee) 0xd598c` | `0x2a` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x45550 | `(unknown callee) 0xde978` | `[sp-168]` | `0x1b` | `sp-164` | `0x0` |
| `bl` at 0x45580 | `(unknown callee) 0x200a0` | `0x3` | `0xf67f0` | `0x55d` | `0x0` |
| `bl` at 0x455b0 | `(unknown callee) 0xde978` | `[sp-168]` | `0x7d` | `0x0` | `0x0` |
| `bl` at 0x455e0 | `(unknown callee) 0x200a0` | `0x3` | `0xf67f0` | `0x567` | `0x0` |
| `bl` at 0x45618 | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x56a` | `0x0` |
| `bl` at 0x4562c | `(unknown callee) 0xd0754` | `0x1` | `0x0` | `sp-124` | `sp-124` |
| `bl` at 0x45640 | `(unknown callee) 0xd093c` | `0x0` | `0x0` | `0x20` | `sp-124` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x44d48 | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `beq` at 0x44d9c | `ret(0xde978) == 0x0` | `ret(0xde978) != 0x0` |
| `beq` at 0x44ddc | `[0x2997b0]:2 == 0x0` | `[0x2997b0]:2 != 0x0` |
| `beq` at 0x44dec | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `beq` at 0x44e34 | `[0x13222c]:1 == 0x0` | `[0x13222c]:1 != 0x0` |
| `beq` at 0x44ea4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x44ee0 | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `beq` at 0x44f2c | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `bhi` at 0x44f64 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0x44f7c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x44ff0 | `[0x132244] == 0x0` | `[0x132244] != 0x0` |
| `beq` at 0x45000 | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `beq` at 0x45094 | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `beq` at 0x45114 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x45124 | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `beq` at 0x451b8 | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `beq` at 0x45238 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x45248 | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `beq` at 0x452dc | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `beq` at 0x4536c | `ret(0xde978) == 0x0` | `ret(0xde978) != 0x0` |
| `bne` at 0x453a8 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x45410 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x45484 | `ret(0xde978) == 0x0` | `ret(0xde978) != 0x0` |
| `beq` at 0x454e8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x4555c | `ret(0xde978) == 0x0` | `ret(0xde978) != 0x0` |
| `beq` at 0x455bc | `ret(0xde978) == 0x0` | `ret(0xde978) != 0x0` |
| `beq` at 0x455fc | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x45654 | `r0` | `?` |

## `SHUB_Scan_IDCODE`

- `SHUB_Scan_IDCODE`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x45714 to 0x45af4.
  Established. The unnamed `0xd0814` call carries `0x0`, `0x0`, `sp-104`, `sp-104` (`bl` at 0x4579c);
  its nonzero result takes `0x457d4`, while zero falls through (`bne` at 0x457a8).
  An included word assignment writes `0xfffff886` to `sp -0x6c` (`str` at 0x457cc).
  Scan IDCODE validation contract remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x45724 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4572c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x45730 | load | 4 | `abs 0x45a9c` | `-` |
| `ldr` at 0x45734 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4573c | load | 4 | `abs 0x45aa0` | `-` |
| `ldr` at 0x45740 | load | 4 | `abs 0x132228` | `-` |
| `ldr` at 0x45748 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4575c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x45764 | load | 4 | `abs 0x45aa0` | `-` |
| `ldr` at 0x45768 | load | 4 | `abs 0x132228` | `-` |
| `ldr` at 0x45770 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4577c | load | 4 | `abs 0x12da68` | `-` |
| `ldr` at 0x457ac | load | 4 | `abs 0x45aa4` | `-` |
| `str` at 0x457b0 | store | 4 | `sp -0x78` | `0xf7260` |
| `ldr` at 0x457b8 | load | 4 | `abs 0x45aa8` | `-` |
| `ldr` at 0x457bc | load | 4 | `abs 0x45aac` | `-` |
| `ldr` at 0x457c8 | load | 4 | `abs 0x45ab0` | `-` |
| `str` at 0x457cc | store | 4 | `sp -0x6c` | `0xfffff886` |
| `ldr` at 0x457d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x457dc | load | 4 | `abs 0x45aa0` | `-` |
| `ldr` at 0x457e0 | load | 4 | `abs 0x132228` | `-` |
| `ldr` at 0x457e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x457f0 | load | 4 | `abs 0x45ab4` | `-` |
| `str` at 0x457fc | store | 4 | `sp -0x78` | `0x20` |
| `ldr` at 0x45800 | load | 4 | `abs 0x45ab8` | `-` |
| `str` at 0x45804 | store | 4 | `sp -0x74` | `0xf7278` |
| `ldr` at 0x45808 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x45828 | load | 4 | `abs 0x45abc` | `-` |
| `str` at 0x4582c | store | 4 | `sp -0x78` | `0xf7288` |
| `ldr` at 0x45830 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x45834 | store | 4 | `sp -0x74` | `[sp-20]` |
| `ldr` at 0x4583c | load | 4 | `abs 0x45aa8` | `-` |
| `ldr` at 0x45840 | load | 4 | `abs 0x45ac0` | `-` |
| `ldr` at 0x4584c | load | 4 | `abs 0x45ab0` | `-` |
| `str` at 0x45850 | store | 4 | `sp -0x6c` | `0xfffff886` |
| `ldr` at 0x45858 | load | 4 | `abs 0x45ab4` | `-` |
| `ldr` at 0x4585c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x45860 | load | 4 | `abs 0x45ab4` | `-` |
| `ldr` at 0x45864 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x45868 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4586c | load | 4 | `abs 0x45ac4` | `-` |
| `ldr` at 0x45878 | load | 4 | `abs 0x45ab4` | `-` |
| `ldr` at 0x4587c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x45880 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x45888 | load | 4 | `abs 0x45ac4` | `-` |
| `str` at 0x45894 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x45898 | load | 4 | `abs 0x45ab4` | `-` |
| `ldr` at 0x4589c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x458a0 | load | 4 | `abs 0x45ab4` | `-` |
| `ldr` at 0x458a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x458a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x458ac | load | 4 | `abs 0x45ac8` | `-` |
| `ldr` at 0x458b8 | load | 4 | `abs 0x45ab4` | `-` |
| `ldr` at 0x458bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x458c0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x458c8 | load | 4 | `abs 0x45ac8` | `-` |
| `str` at 0x458d4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x458d8 | load | 4 | `abs 0x45ab4` | `-` |
| `ldr` at 0x458dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x458e0 | load | 4 | `abs 0x45ab4` | `-` |
| `ldr` at 0x458e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x458e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x458ec | load | 4 | `abs 0x45acc` | `-` |
| `ldr` at 0x458f8 | load | 4 | `abs 0x45ab4` | `-` |
| `ldr` at 0x458fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x45900 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x45908 | load | 4 | `abs 0x45acc` | `-` |
| `str` at 0x45914 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x45918 | load | 4 | `abs 0x45ab4` | `-` |
| `ldr` at 0x4591c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x45920 | load | 4 | `abs 0x45ab4` | `-` |
| `ldr` at 0x45924 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x45928 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x45938 | load | 4 | `abs 0x45ab4` | `-` |
| `ldr` at 0x4593c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x45940 | load | 4 | `(unknown)` | `-` |
| `str` at 0x45954 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x45958 | load | 4 | `abs 0x45ab4` | `-` |
| `ldr` at 0x4595c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x45960 | load | 4 | `abs 0x45ab4` | `-` |
| `ldr` at 0x45964 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x45968 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x45978 | load | 4 | `abs 0x45ab4` | `-` |
| `ldr` at 0x4597c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x45980 | load | 4 | `(unknown)` | `-` |
| `str` at 0x45994 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x45998 | load | 4 | `abs 0x45ab4` | `-` |
| `ldr` at 0x4599c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x459a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x459a8 | load | 4 | `abs 0x45ad0` | `-` |
| `ldr` at 0x459b4 | load | 4 | `abs 0x45ad4` | `-` |
| `str` at 0x459b8 | store | 4 | `sp -0x78` | `0xf72b8` |
| `ldr` at 0x459c0 | load | 4 | `abs 0x45aa8` | `-` |
| `ldr` at 0x459c4 | load | 4 | `abs 0x45ad8` | `-` |
| `ldr` at 0x459d0 | load | 4 | `abs 0x45adc` | `-` |
| `str` at 0x459d4 | store | 4 | `sp -0x78` | `0xf72f4` |
| `ldr` at 0x459d8 | load | 4 | `abs 0x45ab4` | `-` |
| `ldr` at 0x459dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x459e0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x459e8 | store | 4 | `sp -0x74` | `?` |
| `ldr` at 0x459f0 | load | 4 | `abs 0x45aa8` | `-` |
| `ldr` at 0x459f4 | load | 4 | `abs 0x45ae0` | `-` |
| `ldr` at 0x45a00 | load | 4 | `abs 0x45ab0` | `-` |
| `str` at 0x45a04 | store | 4 | `sp -0x18` | `0xfffff886` |
| `ldr` at 0x45a0c | load | 4 | `abs 0x45ae4` | `-` |
| `ldr` at 0x45a10 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x45a1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x45a28 | load | 4 | `abs 0x45ae8` | `-` |
| `str` at 0x45a2c | store | 4 | `sp -0x78` | `0xf7324` |
| `ldr` at 0x45a30 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x45a34 | store | 4 | `sp -0x74` | `[sp-20]` |
| `str` at 0x45a38 | store | 4 | `sp -0x70` | `ret(0x45af4)` |
| `ldr` at 0x45a40 | load | 4 | `abs 0x45aa8` | `-` |
| `ldr` at 0x45a44 | load | 4 | `abs 0x45aec` | `-` |
| `ldr` at 0x45a50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x45a64 | load | 4 | `abs 0x45af0` | `-` |
| `str` at 0x45a68 | store | 4 | `sp -0x78` | `0xf7334` |
| `ldr` at 0x45a70 | load | 4 | `abs 0x45aa8` | `-` |
| `ldr` at 0x45a80 | load | 4 | `abs 0x45ab0` | `-` |
| `str` at 0x45a84 | store | 4 | `sp -0x18` | `0xfffff886` |
| `ldr` at 0x45a88 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x45a8c | store | 4 | `sp -0x6c` | `[sp-24]` |
| `ldr` at 0x45a90 | load | 4 | `sp -0x6c` | `-` |
| `ldm` at 0x45a98 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x45a98 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x45a98 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0x45788 | `(unknown callee) (unknown target)` | `?` | `?` | `[0x12da68]` | `?` |
| `bl` at 0x4579c | `(unknown callee) 0xd0814` | `0x0` | `0x0` | `sp-104` | `sp-104` |
| `bl` at 0x457c4 | `(unknown callee) 0x200a0` | `0x3` | `0xf67f0` | `0x585` | `0x0` |
| `bl` at 0x45818 | `(unknown callee) 0xde978` | `?` | `0x3` | `0x0` | `?` |
| `bl` at 0x45848 | `(unknown callee) 0x200a0` | `0x3` | `0xf67f0` | `0x58b` | `0x0` |
| `bl` at 0x459cc | `(unknown callee) 0x200a0` | `0x3` | `0xf67f0` | `0x595` | `0x0` |
| `bl` at 0x459fc | `(unknown callee) 0x200a0` | `0x3` | `0xf67f0` | `0x597` | `0x0` |
| `bl` at 0x45a20 | `(unknown callee) 0x45af4` | `[sp-20]` | `?` | `?` | `[0x29737c]` |
| `bl` at 0x45a4c | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x59b` | `0x0` |
| `bl` at 0x45a54 | `(unknown callee) 0x45af4` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x45a7c | `(unknown callee) 0x200a0` | `0x3` | `0xf67f0` | `0x5a0` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x457a8 | `ret(0xd0814) != 0x0` | `ret(0xd0814) == 0x0` |
| `beq` at 0x45824 | `ret(0xde978) == 0x0` | `ret(0xde978) != 0x0` |
| `beq` at 0x459b0 | `? == 0x4363049` | `? != 0x4363049` |
| `beq` at 0x45a18 | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `bgt` at 0x45a60 | `ret(0x45af4) > 0x1 (signed, 32-bit)` | `ret(0x45af4) <= 0x1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x45a98 | `r0` | `?` |

## `SHUB_ScanSetBusratio`

- `SHUB_ScanSetBusratio`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x45b28 to 0x45ef0.
  Established. The unnamed `0xc5738` call carries `0x0`, `sp-32`, entry `r2`, `sp-32` (`bl` at 0x45b6c).
  Unsigned `[sp-34]:2 <= 0x8fc` takes `0x45e0c`, while larger values fall through (`bls` at 0x45de4).
  An included byte store writes `([sp-35]:1 & 0xff)` to an unknown target (`strb` at 0x45e94).
  Bus-ratio conversion and field encoding remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x45b38 | store | 4 | `sp -0x14` | `r0` |
| `strh` at 0x45b40 | store | 2 | `sp -0x22` | `0x0` |
| `ldr` at 0x45b44 | load | 4 | `abs 0x45eac` | `-` |
| `ldr` at 0x45b48 | load | 4 | `abs 0x13224c` | `-` |
| `str` at 0x45b58 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x45b74 | store | 4 | `sp -0x1c` | `ret(0xc5738)` |
| `ldr` at 0x45b78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45b84 | load | 4 | `abs 0x45eb0` | `-` |
| `str` at 0x45b88 | store | 4 | `sp -0x38` | `0xf7374` |
| `ldr` at 0x45b90 | load | 4 | `abs 0x45eb4` | `-` |
| `ldr` at 0x45b94 | load | 4 | `abs 0x45eb8` | `-` |
| `str` at 0x45ba4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x45bac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x45bb4 | load | 4 | `abs 0x45ebc` | `-` |
| `ldr` at 0x45bb8 | load | 4 | `abs 0x132228` | `-` |
| `ldrb` at 0x45bc0 | load | 1 | `(unknown)` | `-` |
| `str` at 0x45bd0 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x45bdc | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x45be0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x45be8 | load | 4 | `abs 0x45ebc` | `-` |
| `ldr` at 0x45bec | load | 4 | `abs 0x132228` | `-` |
| `ldrb` at 0x45bf4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x45c08 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x45c10 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x45c14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45c20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x45c28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45c50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x45c58 | load | 4 | `abs 0x45ebc` | `-` |
| `ldr` at 0x45c5c | load | 4 | `abs 0x132228` | `-` |
| `ldr` at 0x45c64 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x45c68 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x45c94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45ca8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x45cb0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x45cbc | load | 4 | `abs 0x45ec0` | `-` |
| `ldr` at 0x45cc0 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x45ccc | load | 4 | `abs 0x45ec4` | `-` |
| `str` at 0x45cd0 | store | 4 | `sp -0x38` | `0xf7390` |
| `ldr` at 0x45cd4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x45cd8 | store | 4 | `sp -0x34` | `[sp-20]` |
| `ldr` at 0x45cdc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x45ce0 | store | 4 | `sp -0x30` | `[sp-24]` |
| `ldr` at 0x45ce4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45cf8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x45d00 | load | 4 | `(unknown)` | `-` |
| `str` at 0x45d04 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x45d0c | load | 4 | `abs 0x45eb4` | `-` |
| `ldr` at 0x45d10 | load | 4 | `abs 0x45ec8` | `-` |
| `ldrh` at 0x45d1c | load | 2 | `sp -0x22` | `-` |
| `ldrh` at 0x45d28 | load | 2 | `sp -0x22` | `-` |
| `ldr` at 0x45d2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45d40 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x45d48 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x45d58 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45d6c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x45d74 | load | 4 | `(unknown)` | `-` |
| `strh` at 0x45d78 | store | 2 | `sp -0x22` | `?` |
| `ldr` at 0x45d7c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x45d84 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldrh` at 0x45d8c | load | 2 | `sp -0x22` | `-` |
| `ldr` at 0x45d98 | load | 4 | `abs 0x45ec0` | `-` |
| `ldr` at 0x45d9c | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x45da8 | load | 4 | `abs 0x45ecc` | `-` |
| `str` at 0x45dac | store | 4 | `sp -0x38` | `0xf73ac` |
| `ldr` at 0x45db0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x45db4 | store | 4 | `sp -0x34` | `[sp-20]` |
| `ldr` at 0x45dbc | load | 4 | `abs 0x45eb4` | `-` |
| `ldr` at 0x45dc0 | load | 4 | `abs 0x45ed0` | `-` |
| `str` at 0x45dd0 | store | 4 | `sp -0x28` | `0x0` |
| `ldrh` at 0x45dd8 | load | 2 | `sp -0x22` | `-` |
| `ldr` at 0x45ddc | load | 4 | `abs 0x45ed4` | `-` |
| `ldr` at 0x45de8 | load | 4 | `abs 0x45ed8` | `-` |
| `str` at 0x45dec | store | 4 | `sp -0x38` | `0xf73e0` |
| `ldr` at 0x45df0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x45df4 | store | 4 | `sp -0x34` | `[sp-20]` |
| `ldr` at 0x45dfc | load | 4 | `abs 0x45eb4` | `-` |
| `ldr` at 0x45e00 | load | 4 | `abs 0x45edc` | `-` |
| `ldrh` at 0x45e0c | load | 2 | `sp -0x22` | `-` |
| `ldr` at 0x45e14 | load | 4 | `abs 0x45ee0` | `-` |
| `strb` at 0x45e2c | store | 1 | `sp -0x23` | `?` |
| `ldr` at 0x45e30 | load | 4 | `abs 0x45ec0` | `-` |
| `ldr` at 0x45e34 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x45e40 | load | 4 | `abs 0x45ee4` | `-` |
| `str` at 0x45e44 | store | 4 | `sp -0x38` | `0xf7420` |
| `ldr` at 0x45e48 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x45e4c | store | 4 | `sp -0x34` | `[sp-20]` |
| `ldrb` at 0x45e50 | load | 1 | `sp -0x23` | `-` |
| `str` at 0x45e54 | store | 4 | `sp -0x30` | `[sp-35]:1` |
| `ldr` at 0x45e5c | load | 4 | `abs 0x45eb4` | `-` |
| `ldr` at 0x45e60 | load | 4 | `abs 0x45ee8` | `-` |
| `ldr` at 0x45e6c | load | 4 | `abs 0x45eec` | `-` |
| `ldr` at 0x45e70 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x45e90 | load | 1 | `sp -0x23` | `-` |
| `strb` at 0x45e94 | store | 1 | `(unknown)` | `([sp-35]:1 & 0xff)` |
| `str` at 0x45e9c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x45ea0 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x45ea8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x45ea8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x45ea8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x45b6c | `(unknown callee) 0xc5738` | `0x0` | `sp-32` | `r2` | `sp-32` |
| `bl` at 0x45b9c | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x5c7` | `0x0` |
| `bl` at 0x45c40 | `(unknown callee) 0x542cc` | `?` | `?` | `[sp-24]` | `?` |
| `bl` at 0x45c80 | `(unknown callee) 0xc5790` | `0x0` | `?` | `[sp-24]` | `?` |
| `bl` at 0x45d18 | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x5d7` | `0x0` |
| `bl` at 0x45dc8 | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x5e1` | `0x0` |
| `bl` at 0x45e08 | `(unknown callee) 0x200a0` | `0x4` | `0xf67f0` | `0x5e7` | `0x0` |
| `bl` at 0x45e68 | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x5ea` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x45b50 | `[0x13224c] == 0x0` | `[0x13224c] != 0x0` |
| `bne` at 0x45b80 | `ret(0xc5738) != 0x0` | `ret(0xc5738) == 0x0` |
| `bne` at 0x45bc8 | `? != 0xff` | `? == 0xff` |
| `bgt` at 0x45c1c | `[sp-24] > 0x1 (signed, 32-bit)` | `[sp-24] <= 0x1 (signed, 32-bit)` |
| `beq` at 0x45c4c | `ret(0x542cc) == 0x0` | `ret(0x542cc) != 0x0` |
| `bne` at 0x45c8c | `ret(0xc5790) != 0x0` | `ret(0xc5790) == 0x0` |
| `bne` at 0x45cb8 | `? != 0x3` | `? == 0x3` |
| `beq` at 0x45cc8 | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `beq` at 0x45d24 | `[sp-34]:2 == 0x0` | `[sp-34]:2 != 0x0` |
| `bge` at 0x45d50 | `[sp-34]:2 >= ? (signed, 32-bit)` | `[sp-34]:2 < ? (signed, 32-bit)` |
| `bne` at 0x45d94 | `[sp-34]:2 != 0x0` | `[sp-34]:2 == 0x0` |
| `beq` at 0x45da4 | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `bls` at 0x45de4 | `[sp-34]:2 <= 0x8fc (unsigned, 32-bit)` | `[sp-34]:2 > 0x8fc (unsigned, 32-bit)` |
| `beq` at 0x45e3c | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x45ea8 | `r0` | `0x0` |

## `SHUB_ScanInit_LLP_TEST_CTL`

- `SHUB_ScanInit_LLP_TEST_CTL`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x42ed0 to 0x43088.
  Established. The unnamed `0x4182c` call at `0x42f84` carries `[sp-20]`, `0x50000028`, `[sp-24]`, `0xa5fffff` (`bl` at 0x42f84);
  its nonzero result takes `0x42fac`, while zero falls through (`bne` at 0x42f90).
  Included word stores assign `0xa00300` and `0xa00000` to `sp -0x18` (`str` at 0x42f64, `str` at 0x42f70).
  Link test-control programming remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x42ee0 | load | 4 | `abs 0x43054` | `-` |
| `ldr` at 0x42ee4 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x42ef0 | load | 4 | `abs 0x43058` | `-` |
| `str` at 0x42ef4 | store | 4 | `sp -0x20` | `0xf6aa4` |
| `ldr` at 0x42efc | load | 4 | `abs 0x4305c` | `-` |
| `str` at 0x42f10 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x42f14 | load | 4 | `abs 0x43060` | `-` |
| `ldrb` at 0x42f18 | load | 1 | `abs 0x132224` | `-` |
| `ldr` at 0x42f1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x42f28 | load | 4 | `abs 0x43064` | `-` |
| `ldr` at 0x42f2c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x42f4c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x42f60 | load | 4 | `abs 0x43068` | `-` |
| `str` at 0x42f64 | store | 4 | `sp -0x18` | `0xa00300` |
| `str` at 0x42f70 | store | 4 | `sp -0x18` | `0xa00000` |
| `ldr` at 0x42f74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x42f78 | load | 4 | `abs 0x4306c` | `-` |
| `ldr` at 0x42f7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x42f80 | load | 4 | `abs 0x43070` | `-` |
| `ldr` at 0x42f94 | load | 4 | `abs 0x43074` | `-` |
| `ldr` at 0x42f98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x42fa0 | load | 4 | `abs 0x43078` | `-` |
| `str` at 0x42fa4 | store | 4 | `sp -0x1c` | `0xfffff88a` |
| `ldr` at 0x42fac | load | 4 | `abs 0x43064` | `-` |
| `ldr` at 0x42fb0 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x42fd0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x42fe4 | load | 4 | `abs 0x43068` | `-` |
| `str` at 0x42fe8 | store | 4 | `sp -0x18` | `0xa00300` |
| `str` at 0x42ff4 | store | 4 | `sp -0x18` | `0xa00000` |
| `ldr` at 0x42ff8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x42ffc | load | 4 | `abs 0x4307c` | `-` |
| `ldr` at 0x43000 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x43004 | load | 4 | `abs 0x43070` | `-` |
| `ldr` at 0x43018 | load | 4 | `abs 0x43080` | `-` |
| `ldr` at 0x4301c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x43024 | load | 4 | `abs 0x43084` | `-` |
| `str` at 0x43028 | store | 4 | `sp -0x1c` | `0xfffff889` |
| `ldr` at 0x43030 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x43038 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `str` at 0x43044 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x43048 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x43050 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x43050 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x43050 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x42f08 | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x2d8` | `0x0` |
| `bl` at 0x42f84 | `(unknown callee) 0x4182c` | `[sp-20]` | `0x50000028` | `[sp-24]` | `0xa5fffff` |
| `bl` at 0x42f9c | `(unknown callee) 0x1e958` | `0xf6ac0` | `[sp-20]` | `?` | `ret(0x4182c)` |
| `bl` at 0x43008 | `(unknown callee) 0x4182c` | `[sp-20]` | `0x50002028` | `[sp-24]` | `0xa5fffff` |
| `bl` at 0x43020 | `(unknown callee) 0x1e958` | `0xf6ae0` | `[sp-20]` | `?` | `ret(0x4182c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x42eec | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `ble` at 0x42f24 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x42f5c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x42f90 | `ret(0x4182c) != 0x0` | `ret(0x4182c) == 0x0` |
| `beq` at 0x42fe0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x43014 | `ret(0x4182c) != 0x0` | `ret(0x4182c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x43050 | `r0` | `?` |

## `SHUB_Rev`

- `SHUB_Rev`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x45af4 to 0x45b28.
  Established. Entry `r0` is retained in the word at `sp -0x14` (`str` at 0x45b04).
  A later word load has an unknown effective target (`ldr` at 0x45b10).
  Revision decoding is Unknown because the integer return value is unresolved (`ldm` at 0x45b20).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x45b04 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x45b08 | load | 4 | `abs 0x45b24` | `-` |
| `ldr` at 0x45b0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x45b10 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x45b20 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x45b20 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x45b20 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x45b20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x45b20 | `r0` | `?` |

## `SHUB_WarEnable`

- `SHUB_WarEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x47658 to 0x47760.
  Established. Included absolute byte stores write `0x1` and `0x0` to `0x132405` (`strb` at 0x476e8, `strb` at 0x47720).
  A zero entry `r0` takes `0x47718`, while nonzero falls through (`beq` at 0x47674).
  The indirect call carries `0xff`, entry `r1`, entry `r2`, `[0x13223c]` (`mov` at 0x476a8).
  Enable policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x47668 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x4766c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x47678 | load | 4 | `abs 0x47734` | `-` |
| `ldrb` at 0x4767c | load | 1 | `abs 0x132404` | `-` |
| `ldr` at 0x47688 | load | 4 | `abs 0x47738` | `-` |
| `ldr` at 0x4768c | load | 4 | `abs 0x13223c` | `-` |
| `ldr` at 0x47698 | load | 4 | `abs 0x47738` | `-` |
| `ldr` at 0x4769c | load | 4 | `abs 0x13223c` | `-` |
| `ldr` at 0x476b8 | load | 4 | `abs 0x4773c` | `-` |
| `str` at 0x476bc | store | 4 | `sp -0x18` | `0xf7804` |
| `ldr` at 0x476c4 | load | 4 | `abs 0x47740` | `-` |
| `ldr` at 0x476c8 | load | 4 | `abs 0x47744` | `-` |
| `ldr` at 0x476d8 | load | 4 | `abs 0x47748` | `-` |
| `ldr` at 0x476e0 | load | 4 | `abs 0x4774c` | `-` |
| `strb` at 0x476e8 | store | 1 | `abs 0x132405` | `0x1` |
| `ldr` at 0x476ec | load | 4 | `abs 0x47750` | `-` |
| `ldr` at 0x476f8 | load | 4 | `abs 0x47754` | `-` |
| `str` at 0x476fc | store | 4 | `sp -0x18` | `0xf7830` |
| `ldr` at 0x47704 | load | 4 | `abs 0x47740` | `-` |
| `ldr` at 0x47708 | load | 4 | `abs 0x47758` | `-` |
| `ldr` at 0x47718 | load | 4 | `abs 0x4774c` | `-` |
| `strb` at 0x47720 | store | 1 | `abs 0x132405` | `0x0` |
| `ldr` at 0x47724 | load | 4 | `abs 0x4775c` | `-` |
| `ldm` at 0x47730 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x47730 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x47730 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0x476a8 | `(unknown callee) (unknown target)` | `0xff` | `r1` | `r2` | `[0x13223c]` |
| `bl` at 0x476d0 | `(unknown callee) 0x200a0` | `0x1` | `0xf67f0` | `0x844` | `0x0` |
| `bl` at 0x476dc | `(unknown callee) 0x1e4ac` | `0xf7824` | `?` | `?` | `?` |
| `bl` at 0x476f0 | `(unknown callee) 0x2105c` | `0x1323b0` | `?` | `0x1` | `0x132405` |
| `bl` at 0x47710 | `(unknown callee) 0x200a0` | `0x3` | `0xf67f0` | `0x84c` | `0x0` |
| `bl` at 0x47728 | `(unknown callee) 0x1e4ac` | `0xf7864` | `r1` | `0x0` | `0x132405` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x47674 | `r0 == 0x0` | `r0 != 0x0` |
| `beq` at 0x47684 | `[0x132404]:1 == 0x0` | `[0x132404]:1 != 0x0` |
| `beq` at 0x47694 | `[0x13223c] == 0x0` | `[0x13223c] != 0x0` |
| `beq` at 0x476b4 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x47730 | `r0` | `?` |

## `SHUB_WarStatus`

- `SHUB_WarStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x475e0 to 0x47630.
  Established. The byte at absolute `0x132405` is read (`ldrb` at 0x47610).
  A zero entry `r0` takes `0x4760c`, while nonzero falls through (`beq` at 0x475fc).
  The unnamed `0x197a8` call carries entry `r0`, `0xf77f4`, entry `r2`, entry `r0` (`bl` at 0x47608).
  Status flag interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x475f0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x475f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x47600 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x47604 | load | 4 | `abs 0x47628` | `-` |
| `ldr` at 0x4760c | load | 4 | `abs 0x4762c` | `-` |
| `ldrb` at 0x47610 | load | 1 | `abs 0x132405` | `-` |
| `ldm` at 0x47624 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x47624 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x47624 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x47624 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x47608 | `(unknown callee) 0x197a8` | `r0` | `0xf77f4` | `r2` | `r0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x475fc | `r0 == 0x0` | `r0 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x47624 | `r0` | `?` |

## `SHUB_WarAllow`

- `SHUB_WarAllow`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x47630 to 0x47658.
  Established. The absolute byte at `0x132404` receives `(r0 & 0xff)` (`strb` at 0x4764c).
  The integer return record retains entry `r0` (`ldm` at 0x47650).
  Permission flag encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x47640 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x47644 | load | 4 | `abs 0x47654` | `-` |
| `ldr` at 0x47648 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x4764c | store | 1 | `abs 0x132404` | `(r0 & 0xff)` |
| `ldm` at 0x47650 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x47650 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x47650 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x47650 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x47650 | `r0` | `r0` |

## `SHUB_WarCheck`

- `SHUB_WarCheck`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4753c to 0x475e0.
  Established. Included byte loads read absolute `0x132404` and `0x132405` (`ldrb` at 0x47558, `ldrb` at 0x47570).
  The unnamed `0x200a0` call carries `0x1`, `0xf67f0`, `0x81d`, `0x0` (`bl` at 0x475c0).
  Status consistency policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4754c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x47550 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x47554 | load | 4 | `abs 0x475cc` | `-` |
| `ldrb` at 0x47558 | load | 1 | `abs 0x132404` | `-` |
| `ldr` at 0x4755c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4756c | load | 4 | `abs 0x475d0` | `-` |
| `ldrb` at 0x47570 | load | 1 | `abs 0x132405` | `-` |
| `ldr` at 0x47574 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47590 | load | 4 | `abs 0x475d4` | `-` |
| `str` at 0x47594 | store | 4 | `sp -0x24` | `0xf77ac` |
| `ldr` at 0x47598 | load | 4 | `abs 0x475cc` | `-` |
| `ldrb` at 0x4759c | load | 1 | `abs 0x132404` | `-` |
| `str` at 0x475a0 | store | 4 | `sp -0x20` | `[0x132404]:1` |
| `ldr` at 0x475a4 | load | 4 | `abs 0x475d0` | `-` |
| `ldrb` at 0x475a8 | load | 1 | `abs 0x132405` | `-` |
| `str` at 0x475ac | store | 4 | `sp -0x1c` | `[0x132405]:1` |
| `ldr` at 0x475b4 | load | 4 | `abs 0x475d8` | `-` |
| `ldr` at 0x475b8 | load | 4 | `abs 0x475dc` | `-` |
| `ldm` at 0x475c8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x475c8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x475c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x475c0 | `(unknown callee) 0x200a0` | `0x1` | `0xf67f0` | `0x81d` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4758c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x475c8 | `r0` | `?` |

## `TIOJTAGInit`

- `TIOJTAGInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4b358 to 0x4b524.
  Established. Included absolute byte stores assign `0x1` and `0x0` to `0x132494` (`strb` at 0x4b388, `strb` at 0x4b434).
  A zero `ret(0x375e4)` takes `0x4b4fc`, while nonzero falls through (`beq` at 0x4b374).
  The unnamed `0xca70c` call carries `0x0`, unknown, `sp-17`, unknown (`bl` at 0x4b3e0).
  Board scan availability detection remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4b37c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x4b380 | load | 4 | `abs 0x4b504` | `-` |
| `strb` at 0x4b388 | store | 1 | `abs 0x132494` | `0x1` |
| `ldr` at 0x4b38c | load | 4 | `abs 0x4b508` | `-` |
| `ldrb` at 0x4b390 | load | 1 | `abs 0x299842` | `-` |
| `ldr` at 0x4b394 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4b3a0 | load | 4 | `abs 0x4b504` | `-` |
| `ldrb` at 0x4b3a4 | load | 1 | `abs 0x132494` | `-` |
| `ldr` at 0x4b3b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4b3c4 | load | 4 | `abs 0x4b50c` | `-` |
| `ldr` at 0x4b3c8 | load | 4 | `abs 0x132490` | `-` |
| `ldrb` at 0x4b3d0 | load | 1 | `(unknown)` | `-` |
| `str` at 0x4b3e8 | store | 4 | `sp -0x1c` | `ret(0xca70c)` |
| `ldr` at 0x4b3ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4b3f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4b404 | load | 4 | `abs 0x4b510` | `-` |
| `str` at 0x4b408 | store | 4 | `sp -0x28` | `0xf8354` |
| `ldr` at 0x4b40c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4b410 | store | 4 | `sp -0x24` | `[sp-24]` |
| `str` at 0x4b414 | store | 4 | `sp -0x20` | `ret(0x1fc1c)` |
| `ldr` at 0x4b41c | load | 4 | `abs 0x4b514` | `-` |
| `ldr` at 0x4b420 | load | 4 | `abs 0x4b518` | `-` |
| `ldr` at 0x4b42c | load | 4 | `abs 0x4b504` | `-` |
| `strb` at 0x4b434 | store | 1 | `abs 0x132494` | `0x0` |
| `ldrb` at 0x4b43c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x4b44c | load | 4 | `abs 0x4b504` | `-` |
| `strb` at 0x4b454 | store | 1 | `abs 0x132494` | `0x0` |
| `ldr` at 0x4b458 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4b460 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x4b468 | load | 4 | `abs 0x4b504` | `-` |
| `ldrb` at 0x4b46c | load | 1 | `abs 0x132494` | `-` |
| `str` at 0x4b47c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x4b480 | load | 4 | `abs 0x4b508` | `-` |
| `ldrb` at 0x4b484 | load | 1 | `abs 0x299842` | `-` |
| `ldr` at 0x4b488 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4b494 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4b4a8 | store | 4 | `sp -0x1c` | `ret(0x4c4e8)` |
| `ldr` at 0x4b4ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4b4b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4b4c4 | load | 4 | `abs 0x4b51c` | `-` |
| `str` at 0x4b4c8 | store | 4 | `sp -0x28` | `0xf8374` |
| `ldr` at 0x4b4cc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4b4d0 | store | 4 | `sp -0x24` | `[sp-24]` |
| `str` at 0x4b4d4 | store | 4 | `sp -0x20` | `ret(0x1fc1c)` |
| `ldr` at 0x4b4dc | load | 4 | `abs 0x4b514` | `-` |
| `ldr` at 0x4b4e0 | load | 4 | `abs 0x4b520` | `-` |
| `ldr` at 0x4b4ec | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4b4f4 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldm` at 0x4b500 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4b500 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4b500 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4b368 | `(unknown callee) 0x375e4` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x4b3e0 | `(unknown callee) 0xca70c` | `0x0` | `?` | `sp-17` | `?` |
| `bl` at 0x4b3fc | `(unknown callee) 0x1fc1c` | `ret(0xca70c)` | `?` | `?` | `ret(0xca70c)` |
| `bl` at 0x4b428 | `(unknown callee) 0x200a0` | `0x1` | `0xf8308` | `0x107` | `0x0` |
| `bl` at 0x4b4a0 | `(unknown callee) 0x4c4e8` | `?` | `?` | `[0x299842]:1` | `?` |
| `bl` at 0x4b4bc | `(unknown callee) 0x1fc1c` | `ret(0x4c4e8)` | `?` | `?` | `ret(0x4c4e8)` |
| `bl` at 0x4b4e8 | `(unknown callee) 0x200a0` | `0x1` | `0xf8308` | `0x113` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4b374 | `ret(0x375e4) == 0x0` | `ret(0x375e4) != 0x0` |
| `ble` at 0x4b39c | `[0x299842]:1 <= [sp-24] (signed, 32-bit)` | `[0x299842]:1 > [sp-24] (signed, 32-bit)` |
| `beq` at 0x4b3ac | `[0x132494]:1 == 0x0` | `[0x132494]:1 != 0x0` |
| `beq` at 0x4b3f4 | `ret(0xca70c) == 0x0` | `ret(0xca70c) != 0x0` |
| `bne` at 0x4b448 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x4b474 | `[0x132494]:1 == 0x0` | `[0x132494]:1 != 0x0` |
| `ble` at 0x4b490 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x4b4b4 | `ret(0x4c4e8) == 0x0` | `ret(0x4c4e8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4b500 | `r0` | `?` |

## `JTAG_Cfg_L3SetCfg_f`

- `JTAG_Cfg_L3SetCfg_f`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdb090 to 0xdb4e8.
  Established. The unnamed `0xda000` call carries `[0x29738c]`, unknown, unknown, `0x29738c` (`bl` at 0xdb238);
  its nonzero result takes `0xdb268`, while zero falls through (`bne` at 0xdb244).
  An included byte store writes `0x3` to an unknown target (`strb` at 0xdb424).
  Configuration-message field interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdb0a0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xdb0a4 | load | 4 | `abs 0xdb4c4` | `-` |
| `ldr` at 0xdb0a8 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xdb0b4 | load | 4 | `abs 0xdb4c8` | `-` |
| `str` at 0xdb0b8 | store | 4 | `sp -0x3c` | `0x121988` |
| `ldr` at 0xdb0c0 | load | 4 | `abs 0xdb4cc` | `-` |
| `ldr` at 0xdb0d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdb0dc | load | 4 | `(unknown)` | `-` |
| `str` at 0xdb0e0 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xdb0e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdb0f0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xdb0f4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xdb0f8 | load | 4 | `abs 0xdb4c4` | `-` |
| `ldr` at 0xdb0fc | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xdb108 | load | 4 | `abs 0xdb4d0` | `-` |
| `str` at 0xdb10c | store | 4 | `sp -0x3c` | `0x12199c` |
| `ldr` at 0xdb110 | load | 4 | `abs 0xdb4d4` | `-` |
| `ldr` at 0xdb114 | load | 4 | `abs 0x29738c` | `-` |
| `str` at 0xdb118 | store | 4 | `sp -0x38` | `[0x29738c]` |
| `ldr` at 0xdb11c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdb120 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xdb124 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdb128 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xdb130 | load | 4 | `abs 0xdb4cc` | `-` |
| `ldr` at 0xdb134 | load | 4 | `abs 0xdb4d8` | `-` |
| `ldr` at 0xdb140 | load | 4 | `abs 0xdb4c4` | `-` |
| `ldr` at 0xdb144 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xdb150 | load | 4 | `abs 0xdb4dc` | `-` |
| `ldr` at 0xdb154 | load | 4 | `abs 0xdb4d4` | `-` |
| `ldr` at 0xdb158 | load | 4 | `abs 0x29738c` | `-` |
| `ldr` at 0xdb170 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdb174 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdb180 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdb18c | load | 4 | `abs 0xdb4dc` | `-` |
| `ldr` at 0xdb190 | load | 4 | `abs 0xdb4d4` | `-` |
| `ldr` at 0xdb194 | load | 4 | `abs 0x29738c` | `-` |
| `ldr` at 0xdb1b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdb1b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdb1c4 | load | 4 | `abs 0xdb4e0` | `-` |
| `str` at 0xdb1c8 | store | 4 | `sp -0x3c` | `0x1219cc` |
| `ldr` at 0xdb1cc | load | 4 | `abs 0xdb4d4` | `-` |
| `ldr` at 0xdb1d0 | load | 4 | `abs 0x29738c` | `-` |
| `str` at 0xdb1d4 | store | 4 | `sp -0x38` | `[0x29738c]` |
| `ldr` at 0xdb1d8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdb1dc | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xdb1e0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdb1e4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xdb1e8 | load | 4 | `abs 0xdb4dc` | `-` |
| `ldr` at 0xdb1ec | load | 4 | `abs 0xdb4d4` | `-` |
| `ldr` at 0xdb1f0 | load | 4 | `abs 0x29738c` | `-` |
| `ldr` at 0xdb210 | load | 4 | `(unknown)` | `-` |
| `str` at 0xdb214 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xdb21c | load | 4 | `abs 0xdb4cc` | `-` |
| `ldr` at 0xdb220 | load | 4 | `abs 0xdb4e4` | `-` |
| `ldr` at 0xdb22c | load | 4 | `abs 0xdb4d4` | `-` |
| `ldr` at 0xdb230 | load | 4 | `abs 0x29738c` | `-` |
| `ldr` at 0xdb234 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdb248 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdb24c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdb250 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xdb258 | store | 4 | `[sp-20] +0x0` | `?` |
| `str` at 0xdb260 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xdb268 | load | 4 | `abs 0xdb4dc` | `-` |
| `ldr` at 0xdb26c | load | 4 | `abs 0xdb4d4` | `-` |
| `ldr` at 0xdb270 | load | 4 | `abs 0x29738c` | `-` |
| `ldr` at 0xdb288 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdb28c | store | 4 | `(unknown)` | `[sp-24]` |
| `strb` at 0xdb294 | store | 1 | `sp -0x22` | `0x0` |
| `ldrb` at 0xdb298 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xdb29c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xdb2a8 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xdb2b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdb2c0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xdb2c4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xdb2c8 | load | 4 | `abs 0xdb4dc` | `-` |
| `ldr` at 0xdb2cc | load | 4 | `abs 0xdb4d4` | `-` |
| `ldr` at 0xdb2d0 | load | 4 | `abs 0x29738c` | `-` |
| `ldrb` at 0xdb2f0 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xdb2f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdb300 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xdb308 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xdb30c | load | 4 | `abs 0xdb4dc` | `-` |
| `ldr` at 0xdb310 | load | 4 | `abs 0xdb4d4` | `-` |
| `ldr` at 0xdb314 | load | 4 | `abs 0x29738c` | `-` |
| `ldrb` at 0xdb334 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xdb33c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdb344 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xdb348 | store | 1 | `(unknown)` | `([sp-32] & 0xff)` |
| `ldr` at 0xdb34c | load | 4 | `abs 0xdb4dc` | `-` |
| `ldr` at 0xdb350 | load | 4 | `abs 0xdb4d4` | `-` |
| `ldr` at 0xdb354 | load | 4 | `abs 0x29738c` | `-` |
| `ldrb` at 0xdb374 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xdb37c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xdb384 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xdb390 | load | 4 | `abs 0xdb4dc` | `-` |
| `ldr` at 0xdb394 | load | 4 | `abs 0xdb4d4` | `-` |
| `ldr` at 0xdb398 | load | 4 | `abs 0x29738c` | `-` |
| `ldrb` at 0xdb3b8 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xdb3c0 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xdb3c8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xdb3e8 | load | 4 | `abs 0xdb4dc` | `-` |
| `ldr` at 0xdb3ec | load | 4 | `abs 0xdb4d4` | `-` |
| `ldr` at 0xdb3f0 | load | 4 | `abs 0x29738c` | `-` |
| `ldrb` at 0xdb410 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xdb418 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xdb424 | store | 1 | `(unknown)` | `0x3` |
| `ldrb` at 0xdb428 | load | 1 | `sp -0x22` | `-` |
| `strb` at 0xdb430 | store | 1 | `sp -0x22` | `([sp-34]:1+1 & 0xff)` |
| `ldr` at 0xdb438 | load | 4 | `abs 0xdb4d4` | `-` |
| `ldr` at 0xdb43c | load | 4 | `abs 0x29738c` | `-` |
| `ldr` at 0xdb444 | load | 4 | `abs 0xdb4dc` | `-` |
| `ldr` at 0xdb448 | load | 4 | `abs 0xdb4d4` | `-` |
| `ldr` at 0xdb44c | load | 4 | `abs 0x29738c` | `-` |
| `strb` at 0xdb470 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xdb474 | load | 4 | `abs 0xdb4dc` | `-` |
| `ldr` at 0xdb478 | load | 4 | `abs 0xdb4d4` | `-` |
| `ldr` at 0xdb47c | load | 4 | `abs 0x29738c` | `-` |
| `str` at 0xdb4a0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xdb4a4 | load | 4 | `abs 0xdb4d4` | `-` |
| `ldr` at 0xdb4a8 | load | 4 | `abs 0x29738c` | `-` |
| `str` at 0xdb4b4 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0xdb4b8 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xdb4c0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdb4c0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdb4c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdb0cc | `(unknown callee) 0x200a0` | `0x6` | `0x121700` | `0x1d4` | `0x0` |
| `bl` at 0xdb13c | `(unknown callee) 0x200a0` | `0x6` | `0x121700` | `0x1dd` | `0x0` |
| `bl` at 0xdb228 | `(unknown callee) 0x200a0` | `0x6` | `0x121700` | `0x1e2` | `0x0` |
| `bl` at 0xdb238 | `(unknown callee) 0xda000` | `[0x29738c]` | `?` | `?` | `0x29738c` |
| `bl` at 0xdb3d8 | `(unknown callee) 0xdef00` | `0x3` | `?` | `?` | `?` |
| `bl` at 0xdb440 | `(unknown callee) 0xda778` | `[0x29738c]` | `?` | `?` | `0x29738c` |
| `bl` at 0xdb4ac | `(unknown callee) 0xdd46c` | `[0x29738c]` | `0x29c540` | `?` | `0x29738c` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xdb0b0 | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `beq` at 0xdb104 | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `beq` at 0xdb14c | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `beq` at 0xdb17c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xdb188 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xdb1c0 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xdb244 | `ret(0xda000) != 0x0` | `ret(0xda000) == 0x0` |
| `bhs` at 0xdb2a4 | `C == 1` | `not (C == 1)` |
| `bne` at 0xdb38c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xdb3d0 | `? != 0x8` | `? == 0x8` |
| `beq` at 0xdb3e4 | `ret(0xdef00) == 0x0` | `ret(0xdef00) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdb4c0 | `r0` | `?` |

## `JTAG_Cfg_display_f`

- `JTAG_Cfg_display_f`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdadd8 to 0xdb090.
  Established. Unsigned `[0x29737c] <= 0x2` takes `0xdae14`, while larger values fall through (`bls` at 0xdadf4).
  The unnamed `0x221f0` call at `0xdae28` carries `0x1218f4`, `[0x297388]`, unknown, `0x297388` (`bl` at 0xdae28).
  Included halfword and byte loads have unknown targets (`ldrh` at 0xdafa0, `ldrb` at 0xdafd8).
  Configuration display field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xdade8 | load | 4 | `abs 0xdb05c` | `-` |
| `ldr` at 0xdadec | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xdadf8 | load | 4 | `abs 0xdb060` | `-` |
| `str` at 0xdadfc | store | 4 | `sp -0x20` | `0x1218c8` |
| `ldr` at 0xdae04 | load | 4 | `abs 0xdb064` | `-` |
| `ldr` at 0xdae08 | load | 4 | `abs 0xdb068` | `-` |
| `ldr` at 0xdae14 | load | 4 | `abs 0xdb06c` | `-` |
| `ldr` at 0xdae1c | load | 4 | `abs 0xdb070` | `-` |
| `ldr` at 0xdae20 | load | 4 | `abs 0xdb074` | `-` |
| `ldr` at 0xdae24 | load | 4 | `abs 0x297388` | `-` |
| `ldr` at 0xdae2c | load | 4 | `abs 0xdb078` | `-` |
| `ldr` at 0xdae30 | load | 4 | `abs 0xdb07c` | `-` |
| `ldr` at 0xdae34 | load | 4 | `abs 0x29738c` | `-` |
| `str` at 0xdae40 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0xdae44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdae50 | load | 4 | `abs 0xdb080` | `-` |
| `ldr` at 0xdae54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdae74 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdae84 | load | 4 | `abs 0xdb080` | `-` |
| `ldr` at 0xdae88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdaea0 | load | 4 | `abs 0xdb080` | `-` |
| `ldr` at 0xdaea4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdaec4 | load | 4 | `abs 0xdb080` | `-` |
| `ldr` at 0xdaec8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdaee8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xdaeec | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xdaef0 | load | 4 | `abs 0xdb080` | `-` |
| `ldr` at 0xdaef4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdaf14 | load | 4 | `(unknown)` | `-` |
| `str` at 0xdaf18 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xdaf1c | load | 4 | `abs 0xdb084` | `-` |
| `ldr` at 0xdaf20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdaf24 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdaf28 | load | 4 | `(unknown)` | `-` |
| `str` at 0xdaf34 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xdaf38 | load | 4 | `abs 0xdb080` | `-` |
| `ldr` at 0xdaf3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdaf5c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdaf60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdaf6c | load | 4 | `abs 0xdb080` | `-` |
| `ldr` at 0xdaf70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdaf90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdaf98 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xdafa0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xdafa4 | load | 4 | `abs 0xdb080` | `-` |
| `ldr` at 0xdafa8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdafc8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdafd0 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xdafd8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xdafdc | load | 4 | `abs 0xdb080` | `-` |
| `ldr` at 0xdafe0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdb000 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdb008 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xdb010 | load | 1 | `(unknown)` | `-` |
| `str` at 0xdb014 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xdb018 | load | 4 | `abs 0xdb088` | `-` |
| `ldr` at 0xdb01c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdb02c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdb034 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0xdb03c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdb044 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xdb04c | load | 4 | `abs 0xdb08c` | `-` |
| `ldm` at 0xdb058 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdb058 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdb058 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdae10 | `(unknown callee) 0x200a0` | `0x6` | `0x121700` | `0x1b1` | `0x0` |
| `bl` at 0xdae18 | `(unknown callee) 0x221f0` | `0x1218dc` | `?` | `?` | `?` |
| `bl` at 0xdae28 | `(unknown callee) 0x221f0` | `0x1218f4` | `[0x297388]` | `?` | `0x297388` |
| `bl` at 0xdae38 | `(unknown callee) 0x221f0` | `0x121904` | `[0x29738c]` | `?` | `0x29738c` |
| `bl` at 0xdaf2c | `(unknown callee) 0x221f0` | `0x121914` | `?` | `?` | `?` |
| `bl` at 0xdb028 | `(unknown callee) 0x221f0` | `0x121950` | `?` | `?` | `?` |
| `bl` at 0xdb050 | `(unknown callee) 0x221f0` | `0x121984` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xdadf4 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bhi` at 0xdae4c | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0xdae7c | `Z == 0` | `not (Z == 0)` |
| `bls` at 0xdaf68 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdb058 | `r0` | `ret(0x221f0)` |

## `JTAG_LLP_SC_f`

- `JTAG_LLP_SC_f`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe7540 to 0xe7f58.
  Established. The unnamed `0xdf1b0` call carries unknown, unknown, `0x4`, unknown (`bl` at 0xe758c).
  A nonzero `ret(0xdefac)` takes `0xe7744`, while zero falls through (`bne` at 0xe7708).
  An included word store writes an unknown value through `[sp-44] +0x0` (`str` at 0xe7a98).
  Link scan command and reply semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe7550 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xe7558 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7560 | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0xe7570 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7578 | load | 4 | `r0 +0x4` | `-` |
| `str` at 0xe7594 | store | 4 | `sp -0x18` | `ret(0xdf1b0)` |
| `ldr` at 0xe7598 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe75a4 | load | 4 | `abs 0xe7f34` | `-` |
| `str` at 0xe75a8 | store | 4 | `sp -0x48` | `0xfffff7c6` |
| `ldr` at 0xe75b0 | load | 4 | `abs 0xe7f38` | `-` |
| `ldr` at 0xe75b4 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xe75c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe75cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe75d0 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xe75dc | load | 4 | `abs 0xe7f3c` | `-` |
| `ldr` at 0xe75e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe75ec | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe75f4 | store | 4 | `sp -0x2c` | `[sp-20]+16` |
| `ldr` at 0xe75f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe75fc | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xe7608 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe760c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe7610 | load | 4 | `[sp-20] +0x10` | `-` |
| `str` at 0xe7620 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xe7624 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe7630 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xe7634 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe7640 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe764c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe765c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe766c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe7678 | load | 4 | `abs 0xe7f40` | `-` |
| `str` at 0xe767c | store | 4 | `sp -0x48` | `0xfffff7c0` |
| `ldr` at 0xe7684 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe768c | store | 4 | `sp -0x20` | `[sp-32]+9` |
| `ldr` at 0xe7694 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe76a0 | load | 4 | `abs 0xe7f40` | `-` |
| `str` at 0xe76a4 | store | 4 | `sp -0x48` | `0xfffff7c0` |
| `ldr` at 0xe76ac | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe76b4 | store | 4 | `sp -0x20` | `[sp-32]+11` |
| `ldr` at 0xe76bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe76c8 | load | 4 | `abs 0xe7f40` | `-` |
| `str` at 0xe76cc | store | 4 | `sp -0x48` | `0xfffff7c0` |
| `ldr` at 0xe76d4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe76dc | store | 4 | `sp -0x20` | `[sp-32]+18` |
| `ldr` at 0xe76e4 | load | 4 | `abs 0xe7f40` | `-` |
| `str` at 0xe76e8 | store | 4 | `sp -0x48` | `0xfffff7c0` |
| `ldr` at 0xe76f0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe76fc | store | 4 | `sp -0x34` | `ret(0xdefac)` |
| `ldr` at 0xe7700 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe770c | load | 4 | `abs 0xe7f44` | `-` |
| `ldr` at 0xe7710 | load | 4 | `abs 0xe7f48` | `-` |
| `ldr` at 0xe7714 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe771c | load | 4 | `abs 0xe7f44` | `-` |
| `str` at 0xe7720 | store | 4 | `sp -0x60` | `0x29c6f8` |
| `ldr` at 0xe7728 | load | 4 | `abs 0xe7f4c` | `-` |
| `ldr` at 0xe772c | load | 4 | `abs 0xe7f50` | `-` |
| `ldr` at 0xe7738 | load | 4 | `abs 0xe7f34` | `-` |
| `str` at 0xe773c | store | 4 | `sp -0x48` | `0xfffff7c6` |
| `ldr` at 0xe7744 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe7748 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe7750 | load | 4 | `ret(0xdefac) +0x28` | `-` |
| `ldr` at 0xe7758 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe775c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe7760 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe7764 | load | 4 | `[sp-56] +0x8` | `-` |
| `ldrh` at 0xe7768 | load | 2 | `[[sp-56]+8] +0x2` | `-` |
| `str` at 0xe776c | store | 4 | `sp -0x28` | `[[[sp-56]+8]+2]:2` |
| `ldr` at 0xe7770 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe777c | store | 4 | `sp -0x44` | `ret(0x1943c)` |
| `ldr` at 0xe7780 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe778c | load | 4 | `abs 0xe7f54` | `-` |
| `str` at 0xe7790 | store | 4 | `sp -0x48` | `0xfffff7cc` |
| `str` at 0xe779c | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xe77a0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe77a4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe77b0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe77b4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe77bc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe77c0 | load | 4 | `[sp-56] +0xc` | `-` |
| `ldr` at 0xe77c4 | load | 4 | `sp -0x3c` | `-` |
| `ldrb` at 0xe77cc | load | 1 | `(unknown)` | `-` |
| `strb` at 0xe77d0 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe77d4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe77dc | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldr` at 0xe77e4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe77e8 | load | 4 | `[sp-56] +0x8` | `-` |
| `ldrh` at 0xe77ec | load | 2 | `[[sp-56]+8] +0x0` | `-` |
| `ldr` at 0xe7804 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe7808 | load | 4 | `[sp-56] +0x8` | `-` |
| `ldrh` at 0xe780c | load | 2 | `[[sp-56]+8] +0x0` | `-` |
| `str` at 0xe781c | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xe7824 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe7828 | load | 4 | `[sp-56] +0x8` | `-` |
| `ldrh` at 0xe782c | load | 2 | `[[sp-56]+8] +0x0` | `-` |
| `str` at 0xe783c | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xe7840 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xe7848 | store | 4 | `sp -0x50` | `[sp-80]+1` |
| `ldr` at 0xe784c | load | 4 | `sp -0x50` | `-` |
| `str` at 0xe7850 | store | 4 | `sp -0x28` | `[sp-80]` |
| `ldr` at 0xe7854 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe7868 | store | 4 | `sp -0x40` | `ret(0x1943c)` |
| `ldr` at 0xe786c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe7878 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe7880 | load | 4 | `abs 0xe7f54` | `-` |
| `str` at 0xe7884 | store | 4 | `sp -0x48` | `0xfffff7cc` |
| `str` at 0xe7890 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xe7894 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe7898 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe78a4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe78ac | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe78b4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe78b8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe78c0 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0xe78c8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe78cc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe78d0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe78d8 | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldr` at 0xe78e0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe78e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe78f0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe78fc | store | 4 | `sp -0x1c` | `ret(0xe0104)` |
| `ldr` at 0xe7900 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7910 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe791c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7920 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe7924 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe792c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7934 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe7944 | store | 4 | `sp -0x1c` | `ret(0xe0540)` |
| `ldr` at 0xe7948 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7958 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe7968 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe7970 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7978 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe7988 | store | 4 | `sp -0x1c` | `ret(0xe0540)` |
| `ldr` at 0xe798c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe79a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe79a4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe79b4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe79b8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe79c4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe79d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe79d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe79d8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe79e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe79e8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe79f8 | store | 4 | `sp -0x1c` | `ret(0xe0540)` |
| `ldr` at 0xe79fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7a0c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe7a14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7a1c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe7a2c | store | 4 | `sp -0x1c` | `ret(0xe0540)` |
| `ldr` at 0xe7a30 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7a44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7a48 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe7a54 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe7a5c | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xe7a64 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe7a70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7a74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe7a78 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe7a80 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe7a84 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe7a88 | load | 4 | `[sp-44] +0x0` | `-` |
| `str` at 0xe7a98 | store | 4 | `[sp-44] +0x0` | `?` |
| `str` at 0xe7aa0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xe7aa8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7aac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe7ab0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe7ab8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe7ac8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe7adc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe7af0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7af4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe7b00 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe7b10 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe7b18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7b1c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe7b30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7b34 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe7b40 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe7b50 | store | 4 | `sp -0x54` | `0x0` |
| `str` at 0xe7b5c | store | 4 | `sp -0x54` | `0x3` |
| `ldr` at 0xe7b60 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xe7b64 | store | 4 | `sp -0x30` | `[sp-84]` |
| `ldr` at 0xe7b6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7b70 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe7b7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7b84 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe7b94 | store | 4 | `sp -0x1c` | `ret(0xe0540)` |
| `ldr` at 0xe7b98 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7ba8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe7bb8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe7bc8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe7bd0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7bd4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe7be4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe7bf4 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0xe7c00 | store | 4 | `sp -0x58` | `0x3` |
| `ldr` at 0xe7c04 | load | 4 | `sp -0x58` | `-` |
| `str` at 0xe7c08 | store | 4 | `sp -0x30` | `[sp-88]` |
| `ldr` at 0xe7c10 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7c14 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe7c20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7c28 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe7c38 | store | 4 | `sp -0x1c` | `ret(0xe0540)` |
| `ldr` at 0xe7c3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7c4c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe7c5c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe7c68 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xe7c70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7c74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe7c84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7c88 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe7c94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7c9c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe7cac | store | 4 | `sp -0x1c` | `ret(0xe0540)` |
| `ldr` at 0xe7cb0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7cc4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7cc8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe7cd8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe7cdc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe7ce8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7cf4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe7d00 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xe7d08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7d0c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe7d1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7d20 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe7d2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7d34 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe7d44 | store | 4 | `sp -0x1c` | `ret(0xe0540)` |
| `ldr` at 0xe7d48 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7d5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7d60 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe7d6c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe7d74 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xe7d7c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe7d88 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7d8c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe7d90 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe7d98 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe7d9c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe7da0 | load | 4 | `[sp-44] +0x0` | `-` |
| `str` at 0xe7db0 | store | 4 | `[sp-44] +0x0` | `?` |
| `str` at 0xe7db8 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xe7dc0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7dc4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe7dc8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe7dd0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe7dd4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe7dd8 | load | 4 | `[sp-44] +0x0` | `-` |
| `str` at 0xe7de4 | store | 4 | `[sp-44] +0x0` | `?` |
| `str` at 0xe7dec | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xe7df0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe7df4 | load | 4 | `[sp-56] +0x8` | `-` |
| `ldrh` at 0xe7df8 | load | 2 | `[[sp-56]+8] +0x2` | `-` |
| `str` at 0xe7dfc | store | 4 | `sp -0x28` | `[[[sp-56]+8]+2]:2` |
| `str` at 0xe7e04 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xe7e08 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe7e0c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7e18 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe7e1c | load | 4 | `[sp-56] +0xc` | `-` |
| `ldr` at 0xe7e20 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe7e28 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe7e2c | load | 4 | `sp -0x3c` | `-` |
| `ldrb` at 0xe7e34 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xe7e38 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe7e3c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe7e44 | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldr` at 0xe7e4c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe7e50 | load | 4 | `[sp-56] +0x8` | `-` |
| `ldrh` at 0xe7e54 | load | 2 | `[[sp-56]+8] +0x0` | `-` |
| `ldr` at 0xe7e6c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe7e70 | load | 4 | `[sp-56] +0x8` | `-` |
| `ldrh` at 0xe7e74 | load | 2 | `[[sp-56]+8] +0x0` | `-` |
| `str` at 0xe7e84 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0xe7e8c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe7e90 | load | 4 | `[sp-56] +0x8` | `-` |
| `ldrh` at 0xe7e94 | load | 2 | `[[sp-56]+8] +0x0` | `-` |
| `str` at 0xe7ea4 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0xe7ea8 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xe7eb0 | store | 4 | `sp -0x5c` | `[sp-92]+1` |
| `ldr` at 0xe7eb4 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xe7eb8 | store | 4 | `sp -0x28` | `[sp-92]` |
| `str` at 0xe7ec0 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xe7ec4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe7ec8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7ed4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe7ed8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe7ee0 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0xe7ee8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe7ef0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe7ef8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe7efc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe7f00 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe7f08 | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldr` at 0xe7f10 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe7f18 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe7f20 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe7f24 | store | 4 | `sp -0x48` | `[sp-28]` |
| `ldr` at 0xe7f28 | load | 4 | `sp -0x48` | `-` |
| `ldm` at 0xe7f30 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe7f30 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe7f30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe758c | `(unknown callee) 0xdf1b0` | `?` | `?` | `0x4` | `?` |
| `bl` at 0xe75c4 | `(unknown callee) 0xdde20` | `[sp-24]` | `?` | `?` | `[0x29737c]` |
| `bl` at 0xe75e8 | `(unknown callee) 0x221f0` | `0x124904` | `ret(0xdde20)` | `[sp-24]` | `?` |
| `bl` at 0xe76f4 | `(unknown callee) 0xdefac` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0xe7718 | `(unknown callee) 0x197a8` | `0x29c6f8` | `0x124930` | `[sp-24]` | `ret(0xdefac)` |
| `bl` at 0xe7734 | `(unknown callee) 0x200a0` | `0x3` | `0x124960` | `0x19d` | `0x0` |
| `bl` at 0xe7774 | `(unknown callee) 0x1943c` | `[[[sp-56]+8]+2]:2` | `ret(0xdefac)` | `?` | `[[[sp-56]+8]+2]:2` |
| `bl` at 0xe7860 | `(unknown callee) 0x1943c` | `?` | `?` | `?` | `?` |
| `bl` at 0xe787c | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `ret(0x1943c)` |
| `bl` at 0xe78e4 | `(unknown callee) 0xe5150` | `[sp-56]` | `?` | `?` | `[sp-40]` |
| `bl` at 0xe78f4 | `(unknown callee) 0xe0104` | `[sp-24]` | `0x0` | `[sp-32]` | `?` |
| `bl` at 0xe7928 | `(unknown callee) 0xe7f58` | `[sp-24]` | `[sp-32]` | `[sp-44]` | `[sp-36]` |
| `bl` at 0xe793c | `(unknown callee) 0xe0540` | `[sp-24]` | `0x0` | `[sp-32]` | `0x0` |
| `bl` at 0xe796c | `(unknown callee) 0xe5150` | `[sp-56]` | `?` | `?` | `?` |
| `bl` at 0xe7980 | `(unknown callee) 0xe0540` | `[sp-24]` | `0xc` | `[sp-32]` | `0x0` |
| `bl` at 0xe79ac | `(unknown callee) 0xdf6d8` | `[sp-24]` | `[sp-32]` | `0xa` | `sp-48` |
| `bl` at 0xe79dc | `(unknown callee) 0xe7f58` | `[sp-24]` | `[sp-32]` | `[sp-44]` | `?` |
| `bl` at 0xe79f0 | `(unknown callee) 0xe0540` | `[sp-24]` | `0x0` | `[sp-32]` | `0x0` |
| `bl` at 0xe7a10 | `(unknown callee) 0xe5150` | `[sp-56]` | `?` | `?` | `ret(0xe0540)` |
| `bl` at 0xe7a24 | `(unknown callee) 0xe0540` | `[sp-24]` | `0xc` | `[sp-32]` | `0x0` |
| `bl` at 0xe7a50 | `(unknown callee) 0xdf6d8` | `[sp-24]` | `[sp-32]` | `0xa` | `sp-48` |
| `bl` at 0xe7a7c | `(unknown callee) 0xe83b4` | `[sp-24]` | `[sp-32]` | `[sp-44]` | `[sp-48]` |
| `bl` at 0xe7ab4 | `(unknown callee) 0xe7f58` | `[sp-24]` | `[sp-32]` | `[sp-44]` | `[sp-48]` |
| `bl` at 0xe7afc | `(unknown callee) 0xdf6d8` | `[sp-24]` | `[sp-32]` | `0x0` | `sp-48` |
| `bl` at 0xe7b24 | `(unknown callee) 0xdfc34` | `[sp-24]` | `[sp-32]` | `0x0` | `sp-48` |
| `bl` at 0xe7b3c | `(unknown callee) 0xdf6d8` | `[sp-24]` | `[sp-32]` | `0x2` | `sp-48` |
| `bl` at 0xe7b78 | `(unknown callee) 0xdfc34` | `[sp-24]` | `[sp-32]` | `0x2` | `sp-48` |
| `bl` at 0xe7b8c | `(unknown callee) 0xe0540` | `[sp-24]` | `0x0` | `[sp-32]` | `0x0` |
| `bl` at 0xe7bdc | `(unknown callee) 0xdfc34` | `[sp-24]` | `[sp-32]` | `0x0` | `sp-48` |
| `bl` at 0xe7c1c | `(unknown callee) 0xdfc34` | `[sp-24]` | `[sp-32]` | `0x2` | `sp-48` |
| `bl` at 0xe7c30 | `(unknown callee) 0xe0540` | `[sp-24]` | `0x0` | `[sp-32]` | `0x0` |
| `bl` at 0xe7c60 | `(unknown callee) 0xe5150` | `[sp-56]` | `?` | `?` | `?` |
| `bl` at 0xe7c7c | `(unknown callee) 0xdfc34` | `[sp-24]` | `[sp-32]` | `0x6` | `sp-48` |
| `bl` at 0xe7c90 | `(unknown callee) 0xdfc34` | `[sp-24]` | `[sp-32]` | `0x7` | `sp-48` |
| `bl` at 0xe7ca4 | `(unknown callee) 0xe0540` | `[sp-24]` | `0xc` | `[sp-32]` | `0x0` |
| `bl` at 0xe7cd0 | `(unknown callee) 0xdf6d8` | `[sp-24]` | `[sp-32]` | `0xa` | `sp-48` |
| `bl` at 0xe7cf8 | `(unknown callee) 0xe5150` | `[sp-56]` | `?` | `?` | `?` |
| `bl` at 0xe7d14 | `(unknown callee) 0xdfc34` | `[sp-24]` | `[sp-32]` | `0x6` | `sp-48` |
| `bl` at 0xe7d28 | `(unknown callee) 0xdfc34` | `[sp-24]` | `[sp-32]` | `0x7` | `sp-48` |
| `bl` at 0xe7d3c | `(unknown callee) 0xe0540` | `[sp-24]` | `0xc` | `[sp-32]` | `0x0` |
| `bl` at 0xe7d68 | `(unknown callee) 0xdf6d8` | `[sp-24]` | `[sp-32]` | `0xa` | `sp-48` |
| `bl` at 0xe7d94 | `(unknown callee) 0xe83b4` | `[sp-24]` | `[sp-32]` | `[sp-44]` | `[sp-48]` |
| `bl` at 0xe7dcc | `(unknown callee) 0xe83b4` | `[sp-24]` | `[sp-32]` | `[sp-44]` | `?` |
| `bl` at 0xe7f14 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe7f1c | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe75a0 | `Z == 0` | `not (Z == 0)` |
| `bls` at 0xe75bc | `[0x29737c] <= 0x1 (unsigned, 32-bit)` | `[0x29737c] > 0x1 (unsigned, 32-bit)` |
| `beq` at 0xe763c | `[sp-76] == 0x4` | `[sp-76] != 0x4` |
| `bgt` at 0xe7648 | `[sp-76] > 0x4 (signed, 32-bit)` | `[sp-76] <= 0x4 (signed, 32-bit)` |
| `beq` at 0xe7654 | `[sp-76] == 0x2` | `[sp-76] != 0x2` |
| `beq` at 0xe7664 | `[sp-76] == 0x6` | `[sp-76] != 0x6` |
| `bls` at 0xe7674 | `[sp-32] <= 0x1 (unsigned, 32-bit)` | `[sp-32] > 0x1 (unsigned, 32-bit)` |
| `bls` at 0xe769c | `[sp-32] <= 0x1 (unsigned, 32-bit)` | `[sp-32] > 0x1 (unsigned, 32-bit)` |
| `bls` at 0xe76c4 | `[sp-32] <= 0x7 (unsigned, 32-bit)` | `[sp-32] > 0x7 (unsigned, 32-bit)` |
| `bne` at 0xe7708 | `ret(0xdefac) != 0x0` | `ret(0xdefac) == 0x0` |
| `bne` at 0xe7788 | `ret(0x1943c) != 0x0` | `ret(0x1943c) == 0x0` |
| `bhs` at 0xe77ac | `C == 1` | `not (C == 1)` |
| `bne` at 0xe7800 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe7874 | `ret(0x1943c) != 0x0` | `ret(0x1943c) == 0x0` |
| `bhs` at 0xe78a0 | `C == 1` | `not (C == 1)` |
| `bge` at 0xe7908 | `ret(0xe0104) >= 0x0 (signed, 32-bit)` | `ret(0xe0104) < 0x0 (signed, 32-bit)` |
| `beq` at 0xe7918 | `[sp-36] == 0x2` | `[sp-36] != 0x2` |
| `bge` at 0xe7950 | `ret(0xe0540) >= 0x0 (signed, 32-bit)` | `ret(0xe0540) < 0x0 (signed, 32-bit)` |
| `beq` at 0xe7964 | `? == 0x0` | `? != 0x0` |
| `bge` at 0xe7994 | `ret(0xe0540) >= 0x0 (signed, 32-bit)` | `ret(0xe0540) < 0x0 (signed, 32-bit)` |
| `beq` at 0xe79c0 | `[sp-48] == 0x0` | `[sp-48] != 0x0` |
| `bhi` at 0xe79cc | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bge` at 0xe7a04 | `ret(0xe0540) >= 0x0 (signed, 32-bit)` | `ret(0xe0540) < 0x0 (signed, 32-bit)` |
| `bge` at 0xe7a38 | `ret(0xe0540) >= 0x0 (signed, 32-bit)` | `ret(0xe0540) < 0x0 (signed, 32-bit)` |
| `beq` at 0xe7a6c | `[sp-48] == 0x0` | `[sp-48] != 0x0` |
| `bne` at 0xe7ac4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe7ad4 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xe7ae8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xe7b48 | `[sp-48] == 0x0` | `[sp-48] != 0x0` |
| `bge` at 0xe7ba0 | `ret(0xe0540) >= 0x0 (signed, 32-bit)` | `ret(0xe0540) < 0x0 (signed, 32-bit)` |
| `beq` at 0xe7bb4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xe7bec | `[sp-48] == 0x0` | `[sp-48] != 0x0` |
| `bge` at 0xe7c44 | `ret(0xe0540) >= 0x0 (signed, 32-bit)` | `ret(0xe0540) < 0x0 (signed, 32-bit)` |
| `beq` at 0xe7c58 | `? == 0x0` | `? != 0x0` |
| `bge` at 0xe7cb8 | `ret(0xe0540) >= 0x0 (signed, 32-bit)` | `ret(0xe0540) < 0x0 (signed, 32-bit)` |
| `bne` at 0xe7ce4 | `[sp-48] != 0x0` | `[sp-48] == 0x0` |
| `bhi` at 0xe7cf0 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bge` at 0xe7d50 | `ret(0xe0540) >= 0x0 (signed, 32-bit)` | `ret(0xe0540) < 0x0 (signed, 32-bit)` |
| `bne` at 0xe7d84 | `[sp-48] != 0x0` | `[sp-48] == 0x0` |
| `bhs` at 0xe7e14 | `C == 1` | `not (C == 1)` |
| `bne` at 0xe7e68 | `? != 0x0` | `? == 0x0` |
| `bhs` at 0xe7ed0 | `C == 1` | `not (C == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe7f30 | `r0` | `?` |
