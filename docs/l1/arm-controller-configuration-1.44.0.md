# ARM controller configuration paths in 1.44.0

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
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/controller-configuration-facts.txt) retain all tool sections.

## `L1CfgReset`

- `L1CfgReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x60b1c to 0x60bf0.
  Established.
  The absolute word at `0x132adc` receives zero (`str` at 0x60b34).
  The unnamed `0x19748` call carries `[0x132ae4]`, `0x0`, unknown, unknown (`bl` at 0x60b60).
  The unnamed `0x66694` call carries `0x132ae4`, `0x132adc`, `0x132ae0`, `[[sp-24]+8]` (`bl` at 0x60bd4).
  Configuration reset effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x60b2c | load | 4 | `abs 0x60be0` | `-` |
| `str` at 0x60b34 | store | 4 | `abs 0x132adc` | `0x0` |
| `ldr` at 0x60b38 | load | 4 | `abs 0x60be0` | `-` |
| `ldr` at 0x60b3c | load | 4 | `abs 0x60be0` | `-` |
| `ldr` at 0x60b40 | load | 4 | `abs 0x132ae0` | `-` |
| `ldr` at 0x60b54 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x60b64 | load | 4 | `abs 0x60be0` | `-` |
| `str` at 0x60b68 | store | 4 | `sp -0x18` | `0x132a50` |
| `ldr` at 0x60b6c | load | 4 | `abs 0x60be0` | `-` |
| `ldr` at 0x60b70 | load | 4 | `abs 0x132a5c` | `-` |
| `str` at 0x60b74 | store | 4 | `sp -0x30` | `[0x132a5c]` |
| `ldr` at 0x60b78 | load | 4 | `abs 0x60be0` | `-` |
| `ldr` at 0x60b7c | load | 4 | `abs 0x132a60` | `-` |
| `str` at 0x60b80 | store | 4 | `sp -0x2c` | `[0x132a60]` |
| `ldr` at 0x60b84 | load | 4 | `abs 0x60be0` | `-` |
| `ldr` at 0x60b88 | load | 4 | `abs 0x132aa8` | `-` |
| `str` at 0x60b98 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x60ba4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x60ba8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x60bac | store | 4 | `sp -0x28` | `[sp-28]` |
| `str` at 0x60bb4 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x60bbc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x60bc0 | load | 4 | `abs 0x60be4` | `-` |
| `ldr` at 0x60bc4 | load | 4 | `abs 0x60be8` | `-` |
| `ldr` at 0x60bc8 | load | 4 | `abs 0x60bec` | `-` |
| `ldr` at 0x60bcc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x60bd0 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldm` at 0x60bdc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x60bdc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x60bdc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x60b60 | `(unknown callee) 0x19748` | `[0x132ae4]` | `0x0` | `?` | `?` |
| `bl` at 0x60bd4 | `(unknown callee) 0x66694` | `0x132ae4` | `0x132adc` | `0x132ae0` | `[[sp-24]+8]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x60b90 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x60bdc | `r0` | `ret(0x66694)` |

## `L1CfgRefresh`

- `L1CfgRefresh`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x60bf0 to 0x60c60.
  Established.
  Included calls of the unnamed `0x676c0` pass `0x2`, `0xd`, `0x14`, `0xf` as their first candidates at separate sites (`bl` at 0x60c00, `bl` at 0x60c08, `bl` at 0x60c10, `bl` at 0x60c58).
  The first call retains three entry register candidates, while the other cited calls have unknown remaining candidates (`bl` at 0x60c00, `bl` at 0x60c08, `bl` at 0x60c10, `bl` at 0x60c58).
  The included return record carries `ret(0x676c0)` in `r0` (`ldm` at 0x60c5c).
  Refresh request meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldm` at 0x60c5c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x60c5c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x60c5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x60c00 | `(unknown callee) 0x676c0` | `0x2` | `r1` | `r2` | `r3` |
| `bl` at 0x60c08 | `(unknown callee) 0x676c0` | `0xd` | `?` | `?` | `?` |
| `bl` at 0x60c10 | `(unknown callee) 0x676c0` | `0x14` | `?` | `?` | `?` |
| `bl` at 0x60c18 | `(unknown callee) 0x676c0` | `0x15` | `?` | `?` | `?` |
| `bl` at 0x60c20 | `(unknown callee) 0x676c0` | `0x16` | `?` | `?` | `?` |
| `bl` at 0x60c28 | `(unknown callee) 0x676c0` | `0x17` | `?` | `?` | `?` |
| `bl` at 0x60c30 | `(unknown callee) 0x676c0` | `0x18` | `?` | `?` | `?` |
| `bl` at 0x60c38 | `(unknown callee) 0x676c0` | `0x19` | `?` | `?` | `?` |
| `bl` at 0x60c40 | `(unknown callee) 0x676c0` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x60c48 | `(unknown callee) 0x676c0` | `0x3` | `?` | `?` | `?` |
| `bl` at 0x60c50 | `(unknown callee) 0x676c0` | `0xe` | `?` | `?` | `?` |
| `bl` at 0x60c58 | `(unknown callee) 0x676c0` | `0xf` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x60c5c | `r0` | `ret(0x676c0)` |

## `L1CfgLock`

- `L1CfgLock`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x619dc to 0x61a78.
  Established.
  The unnamed `0x20cd0` call carries `0x1329f0`, `0xc8`, entry `r2`, entry `r3` (`bl` at 0x619f8);
  its zero result takes `0x61a4c`, while nonzero falls through (`beq` at 0x61a0c).
  The unnamed `0x19808` call carries `0x29a500`, `[sp-20]`, unknown, unknown (`bl` at 0x61a54).
  Configuration lock semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x619ec | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x619f0 | load | 4 | `abs 0x61a60` | `-` |
| `str` at 0x61a00 | store | 4 | `sp -0x18` | `ret(0x20cd0)` |
| `ldr` at 0x61a04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x61a10 | load | 4 | `abs 0x61a64` | `-` |
| `str` at 0x61a14 | store | 4 | `sp -0x2c` | `0xfbdac` |
| `ldr` at 0x61a18 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x61a1c | store | 4 | `sp -0x28` | `ret(0x20cd0)` |
| `ldr` at 0x61a20 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x61a24 | store | 4 | `sp -0x24` | `[sp-20]` |
| `ldr` at 0x61a28 | load | 4 | `abs 0x61a68` | `-` |
| `str` at 0x61a2c | store | 4 | `sp -0x20` | `0x29a500` |
| `ldr` at 0x61a30 | load | 4 | `abs 0x61a6c` | `-` |
| `str` at 0x61a34 | store | 4 | `sp -0x1c` | `0x29a528` |
| `ldr` at 0x61a3c | load | 4 | `abs 0x61a70` | `-` |
| `ldr` at 0x61a40 | load | 4 | `abs 0x61a74` | `-` |
| `ldr` at 0x61a4c | load | 4 | `abs 0x61a68` | `-` |
| `ldr` at 0x61a50 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x61a5c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x61a5c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x61a5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x619f8 | `(unknown callee) 0x20cd0` | `0x1329f0` | `0xc8` | `r2` | `r3` |
| `bl` at 0x61a48 | `(unknown callee) 0x200a0` | `0x9` | `0xfbcb0` | `0x27a` | `0x0` |
| `bl` at 0x61a54 | `(unknown callee) 0x19808` | `0x29a500` | `[sp-20]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x61a0c | `ret(0x20cd0) == 0x0` | `ret(0x20cd0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x61a5c | `r0` | `ret(0x19808)` |

## `L1CfgUnlock`

- `L1CfgUnlock`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x61a78 to 0x61b04.
  Established.
  The unnamed `0x19808` call at `0x61a90` carries `0x29a528`, `0x29a500`, entry `r2`, entry `r3` (`bl` at 0x61a90).
  The unnamed `0x2105c` call carries `0x1329f0` and three unknown candidates (`bl` at 0x61aa4);
  its zero result takes `0x61ae0`, while nonzero falls through (`beq` at 0x61ab8).
  Configuration release behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x61a88 | load | 4 | `abs 0x61ae8` | `-` |
| `ldr` at 0x61a8c | load | 4 | `abs 0x61aec` | `-` |
| `ldr` at 0x61a94 | load | 4 | `abs 0x61aec` | `-` |
| `ldr` at 0x61a98 | load | 4 | `abs 0x61af0` | `-` |
| `ldr` at 0x61aa0 | load | 4 | `abs 0x61af4` | `-` |
| `str` at 0x61aac | store | 4 | `sp -0x14` | `ret(0x2105c)` |
| `ldr` at 0x61ab0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61abc | load | 4 | `abs 0x61af8` | `-` |
| `str` at 0x61ac0 | store | 4 | `sp -0x1c` | `0xfbdf0` |
| `ldr` at 0x61ac4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x61ac8 | store | 4 | `sp -0x18` | `ret(0x2105c)` |
| `ldr` at 0x61ad0 | load | 4 | `abs 0x61afc` | `-` |
| `ldr` at 0x61ad4 | load | 4 | `abs 0x61b00` | `-` |
| `ldm` at 0x61ae4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x61ae4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x61ae4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x61a90 | `(unknown callee) 0x19808` | `0x29a528` | `0x29a500` | `r2` | `r3` |
| `bl` at 0x61a9c | `(unknown callee) 0x19808` | `0x29a500` | `0xfbde4` | `?` | `?` |
| `bl` at 0x61aa4 | `(unknown callee) 0x2105c` | `0x1329f0` | `?` | `?` | `?` |
| `bl` at 0x61adc | `(unknown callee) 0x200a0` | `0x9` | `0xfbcb0` | `0x28a` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x61ab8 | `ret(0x2105c) == 0x0` | `ret(0x2105c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x61ae4 | `r0` | `?` |

## `L1CfgRequest`

- `L1CfgRequest`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x676c0 to 0x678e0.
  Established.
  The unsigned condition `[sp-20] > 0x19` takes `0x678a8`, while less or equal values fall through (`bhi` at 0x67724).
  An included word assignment at `sp -0xc8` writes `0xffff00e` (`str` at 0x677c0).
  The unnamed `0xaa424` call carries `0x132a48`, `sp-200`, `0x0`, `0x0` (`bl` at 0x6781c).
  Configuration request protocol remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x676d0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x676d8 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0x676dc | load | 4 | `abs 0x678b0` | `-` |
| `ldr` at 0x676e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x676e4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x676e8 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x676ec | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x67714 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6771c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6772c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x67734 | store | 4 | `sp -0xcc` | `?` |
| `ldr` at 0x67738 | load | 4 | `abs 0x678b4` | `-` |
| `ldr` at 0x6773c | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0x6774c | load | 4 | `abs 0x678b8` | `-` |
| `ldr` at 0x67750 | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0x67774 | load | 4 | `abs 0x678bc` | `-` |
| `ldr` at 0x67790 | load | 4 | `abs 0x678c0` | `-` |
| `ldr` at 0x677b0 | load | 4 | `abs 0x678bc` | `-` |
| `ldr` at 0x677bc | load | 4 | `abs 0x678c4` | `-` |
| `str` at 0x677c0 | store | 4 | `sp -0xc8` | `0xffff00e` |
| `ldr` at 0x677c4 | load | 4 | `abs 0x678c8` | `-` |
| `ldr` at 0x677c8 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x677d8 | load | 4 | `abs 0x678cc` | `-` |
| `ldr` at 0x677dc | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x677e0 | load | 4 | `abs 0x678d0` | `-` |
| `ldr` at 0x677e4 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x677e8 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x677f8 | store | 4 | `sp -0xd8` | `0x2` |
| `ldr` at 0x677fc | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x67800 | store | 4 | `sp -0xd4` | `[sp-76]` |
| `str` at 0x67808 | store | 4 | `sp -0xd0` | `sp-72` |
| `ldr` at 0x6780c | load | 4 | `abs 0x678d4` | `-` |
| `str` at 0x67824 | store | 4 | `sp -0x18` | `ret(0xaa424)` |
| `ldr` at 0x67828 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x67838 | load | 4 | `abs 0x678d8` | `-` |
| `ldr` at 0x6783c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x67850 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6785c | load | 4 | `abs 0x678d8` | `-` |
| `ldr` at 0x67860 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x67874 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x67880 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x67894 | load | 4 | `abs 0x678dc` | `-` |
| `ldr` at 0x67898 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x678a0 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x678ac | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x678ac | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x678ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x676f0 | `(unknown callee) 0x6f4d4` | `[sp-84]` | `r1` | `0x132af4` | `?` |
| `bl` at 0x67718 | `(unknown callee) 0x5eac8` | `sp-76` | `sp-72` | `[sp-20]` | `sp-76` |
| `bl` at 0x6777c | `(unknown callee) 0x67280` | `sp-76` | `sp-72` | `0x132a50` | `0x1` |
| `bl` at 0x67798 | `(unknown callee) 0x67280` | `sp-76` | `sp-72` | `0x132cf4` | `0x1` |
| `bl` at 0x677b8 | `(unknown callee) 0x67280` | `sp-76` | `sp-72` | `0x132a50` | `0x1` |
| `bl` at 0x677ec | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfc3e0` | `[sp-84]` | `[sp-76]` |
| `bl` at 0x6781c | `(unknown callee) 0xaa424` | `0x132a48` | `sp-200` | `0x0` | `0x0` |
| `bl` at 0x67884 | `(unknown callee) 0xa9d18` | `?` | `ret(0xaa424)` | `0x29a550` | `?` |
| `bl` at 0x6789c | `(unknown callee) 0x1e958` | `0xfc40c` | `[sp-84]` | `?` | `ret(0xa9d18)` |
| `bl` at 0x678a4 | `(unknown callee) 0xa8b98` | `[sp-24]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x676fc | `ret(0x6f4d4) != 0x0` | `ret(0x6f4d4) == 0x0` |
| `bhi` at 0x67724 | `[sp-20] > 0x19 (unsigned, 32-bit)` | `[sp-20] <= 0x19 (unsigned, 32-bit)` |
| `bne` at 0x67748 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x6775c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x677d4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x67830 | `ret(0xaa424) != 0x0` | `ret(0xaa424) == 0x0` |
| `beq` at 0x67858 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x67890 | `ret(0xa9d18) == 0x0` | `ret(0xa9d18) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x678ac | `r0` | `?` |

## `L1CfgProcessEvent`

- `L1CfgProcessEvent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x68070 to 0x681e8.
  Established.
  A nonzero byte through entry `r0 +0x13` takes `0x681dc`, while zero falls through (`bne` at 0x68090).
  The halfword through entry `r0 +0x22` is read (`ldrh` at 0x68098).
  The unnamed `0x6b324` call carries entry `r0`, entry `r1`, `[r0+34]:2`, `[r0+34]:2` (`bl` at 0x6819c).
  Event dispatch interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x68080 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x68084 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x68088 | load | 1 | `r0 +0x13` | `-` |
| `ldr` at 0x68094 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x68098 | load | 2 | `r0 +0x22` | `-` |
| `str` at 0x6809c | store | 4 | `sp -0x8c` | `[r0+34]:2` |
| `ldr` at 0x680a0 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x680ac | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x680b8 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x680c4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x680d0 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x680dc | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x680ec | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x680f8 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x68108 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x68114 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x68120 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x6812c | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x6813c | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x68148 | load | 4 | `abs 0x681e4` | `-` |
| `ldr` at 0x6814c | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x6815c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68168 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68174 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68180 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6818c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68198 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x681a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x681b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x681bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x681c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x681d4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x681e0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x681e0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x681e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x68160 | `(unknown callee) 0x6ad90` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x6816c | `(unknown callee) 0x6aaec` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x68178 | `(unknown callee) 0x6af98` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x68184 | `(unknown callee) 0x6b10c` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x68190 | `(unknown callee) 0x6b218` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x6819c | `(unknown callee) 0x6b324` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x681a8 | `(unknown callee) 0x6b430` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x681b4 | `(unknown callee) 0x6b544` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x681c0 | `(unknown callee) 0x6bd40` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x681cc | `(unknown callee) 0x6c418` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x681d8 | `(unknown callee) 0x67094` | `r0` | `r1` | `[r0+34]:2` | `0x3e9` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x68090 | `[r0+19]:1 != 0x0` | `[r0+19]:1 == 0x0` |
| `beq` at 0x680a8 | `[r0+34]:2 == 0x6` | `[r0+34]:2 != 0x6` |
| `bgt` at 0x680b4 | `[r0+34]:2 > 0x6 (signed, 32-bit)` | `[r0+34]:2 <= 0x6 (signed, 32-bit)` |
| `beq` at 0x680c0 | `[r0+34]:2 == 0x3` | `[r0+34]:2 != 0x3` |
| `bgt` at 0x680cc | `[r0+34]:2 > 0x3 (signed, 32-bit)` | `[r0+34]:2 <= 0x3 (signed, 32-bit)` |
| `beq` at 0x680d8 | `[r0+34]:2 == 0x1` | `[r0+34]:2 != 0x1` |
| `beq` at 0x680e4 | `[r0+34]:2 == 0x2` | `[r0+34]:2 != 0x2` |
| `beq` at 0x680f4 | `[r0+34]:2 == 0x4` | `[r0+34]:2 != 0x4` |
| `beq` at 0x68100 | `[r0+34]:2 == 0x5` | `[r0+34]:2 != 0x5` |
| `beq` at 0x68110 | `[r0+34]:2 == 0x64` | `[r0+34]:2 != 0x64` |
| `bgt` at 0x6811c | `[r0+34]:2 > 0x64 (signed, 32-bit)` | `[r0+34]:2 <= 0x64 (signed, 32-bit)` |
| `beq` at 0x68128 | `[r0+34]:2 == 0x7` | `[r0+34]:2 != 0x7` |
| `beq` at 0x68134 | `[r0+34]:2 == 0x8` | `[r0+34]:2 != 0x8` |
| `beq` at 0x68144 | `[r0+34]:2 == 0x66` | `[r0+34]:2 != 0x66` |
| `beq` at 0x68154 | `[r0+34]:2 == 0x3e9` | `[r0+34]:2 != 0x3e9` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x681e0 | `r0` | `?` |

## `L1CfgProcessResponse`

- `L1CfgProcessResponse`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x67f9c to 0x68070.
  Established.
  A nonzero byte through entry `r0 +0x13` takes `0x68058`, while zero falls through (`bne` at 0x67fbc).
  The condition `[sp-36] == 0x3e8` takes `0x6800c`, while inequality falls through (`beq` at 0x68004).
  The absolute word at `0x132794` receives `ret(0x194f0)` on an included path (`str` at 0x68044).
  Response processing policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x67fac | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x67fb0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x67fb4 | load | 1 | `r0 +0x13` | `-` |
| `ldr` at 0x67fc0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x67fc4 | load | 1 | `r0 +0x24` | `-` |
| `ldr` at 0x67fd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x67fd8 | load | 4 | `r0 +0x28` | `-` |
| `str` at 0x67fdc | store | 4 | `sp -0x24` | `[r0+40]` |
| `ldr` at 0x67fe4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x67ff0 | store | 4 | `sp -0x24` | `ret(0xaadf4)` |
| `ldr` at 0x67ff4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x67ff8 | store | 4 | `sp -0x20` | `[sp-36]` |
| `ldr` at 0x67ffc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6800c | load | 4 | `abs 0x68060` | `-` |
| `ldr` at 0x68010 | load | 4 | `abs 0x132778` | `-` |
| `ldr` at 0x68020 | load | 4 | `abs 0x68064` | `-` |
| `ldr` at 0x68024 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x68028 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x6802c | load | 4 | `abs 0x68068` | `-` |
| `ldr` at 0x68030 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x68038 | load | 4 | `abs 0x6806c` | `-` |
| `str` at 0x68044 | store | 4 | `abs 0x132794` | `ret(0x194f0)` |
| `ldr` at 0x6804c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x6805c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x6805c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6805c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6805c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x67fec | `(unknown callee) 0xaadf4` | `r0` | `0x0` | `r2` | `?` |
| `bl` at 0x68034 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xfc4f4` | `[[sp-24]+12]` | `0x12eeb8` |
| `bl` at 0x68040 | `(unknown callee) 0x194f0` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x68054 | `(unknown callee) 0x643b8` | `[sp-24]` | `0x0` | `?` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x67fbc | `[r0+19]:1 != 0x0` | `[r0+19]:1 == 0x0` |
| `bne` at 0x67fd0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x68004 | `[sp-36] == 0x3e8` | `[sp-36] != 0x3e8` |
| `beq` at 0x6801c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6805c | `r0` | `?` |

## `L1CfgIRouter`

- `L1CfgIRouter`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x681e8 to 0x682a8.
  Established.
  The byte through entry `r0 +0x16` is read (`ldrb` at 0x68208);
  its inequality with `0x2` takes `0x68264`, while equality falls through (`bne` at 0x68210).
  The unnamed `0x67f9c` call carries entry `r0`, entry `r1`, entry `r2`, `[r0+22]:1` (`bl` at 0x68294).
  Configuration packet routing meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x681f8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x68200 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x68204 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x68208 | load | 1 | `r0 +0x16` | `-` |
| `ldr` at 0x68214 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68218 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x68224 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68228 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x68240 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68244 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x68258 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68264 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x68268 | load | 1 | `r0 +0x16` | `-` |
| `ldr` at 0x68274 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68280 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x68284 | load | 1 | `r0 +0x16` | `-` |
| `ldr` at 0x68290 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68298 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x682a4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x682a4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x682a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6825c | `(unknown callee) 0x68070` | `r0` | `r1` | `r2` | `?` |
| `bl` at 0x68278 | `(unknown callee) 0x678e0` | `r0` | `r1` | `r2` | `[r0+22]:1` |
| `bl` at 0x68294 | `(unknown callee) 0x67f9c` | `r0` | `r1` | `r2` | `[r0+22]:1` |
| `bl` at 0x6829c | `(unknown callee) 0xa8b98` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x68210 | `[r0+22]:1 != 0x2` | `[r0+22]:1 == 0x2` |
| `bge` at 0x68220 | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `beq` at 0x68238 | `? == 0xe` | `? != 0xe` |
| `beq` at 0x68250 | `? == 0xe` | `? != 0xe` |
| `bne` at 0x68270 | `[r0+22]:1 != 0x0` | `[r0+22]:1 == 0x0` |
| `bne` at 0x6828c | `[r0+22]:1 != 0x1` | `[r0+22]:1 == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x682a4 | `r0` | `ret(0xa8b98)` |

## `L1CfgPortNonRoutered`

- `L1CfgPortNonRoutered`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6a400 to 0x6a474.
  Established.
  The signed condition `[0x132ad4] <= r0` takes `0x6a45c`, while greater values fall through (`ble` at 0x6a424).
  An included word assignment at `sp -0x18` writes zero (`str` at 0x6a460).
  The absolute word at `0x132ad8` is read (`ldr` at 0x6a448).
  Port routing classification remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6a410 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x6a414 | load | 4 | `abs 0x6a470` | `-` |
| `ldr` at 0x6a418 | load | 4 | `abs 0x132ad4` | `-` |
| `ldr` at 0x6a41c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6a428 | load | 4 | `abs 0x6a470` | `-` |
| `ldr` at 0x6a42c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6a448 | load | 4 | `abs 0x132ad8` | `-` |
| `ldr` at 0x6a450 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6a454 | store | 4 | `sp -0x18` | `?` |
| `str` at 0x6a460 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x6a464 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x6a46c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6a46c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6a46c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x6a424 | `[0x132ad4] <= r0 (signed, 32-bit)` | `[0x132ad4] > r0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6a46c | `r0` | `?` |

## `L1CfgTypeNonRoutered`

- `L1CfgTypeNonRoutered`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6a390 to 0x6a400.
  Established.
  The unsigned condition `r0-64 > 0x13` takes `0x6a3e8`, while less or equal values fall through (`bhi` at 0x6a3b8).
  Separate included assignments at `sp -0x18` write zero and `0x1` (`str` at 0x6a3e0, `str` at 0x6a3ec).
  The included return record gives an unknown integer value in `r0` (`ldm` at 0x6a3f8).
  Type classification policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6a3a0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x6a3a4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6a3ac | store | 4 | `sp -0x1c` | `r0-64` |
| `ldr` at 0x6a3b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6a3c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6a3c8 | load | 4 | `abs 0x6a3fc` | `-` |
| `str` at 0x6a3e0 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x6a3ec | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0x6a3f0 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x6a3f8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6a3f8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6a3f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhi` at 0x6a3b8 | `r0-64 > 0x13 (unsigned, 32-bit)` | `r0-64 <= 0x13 (unsigned, 32-bit)` |
| `bne` at 0x6a3d4 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6a3f8 | `r0` | `?` |

## `L1CfgGetBrick`

- `L1CfgGetBrick`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6a0b0 to 0x6a0f8.
  Established.
  The absolute words at `0x132ae4` and `0x132adc` are read (`ldr` at 0x6a0d0, `ldr` at 0x6a0d4).
  The unnamed `0x65068` call carries `[0x132ae4]`, `[0x132adc]`, entry `r0`, entry `r1` (`bl` at 0x6a0e0).
  The included return record gives `ret(0x65068)` in `r0` (`ldm` at 0x6a0f0).
  Brick lookup semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6a0c0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6a0c4 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x6a0c8 | load | 4 | `abs 0x6a0f4` | `-` |
| `ldr` at 0x6a0cc | load | 4 | `abs 0x6a0f4` | `-` |
| `ldr` at 0x6a0d0 | load | 4 | `abs 0x132ae4` | `-` |
| `ldr` at 0x6a0d4 | load | 4 | `abs 0x132adc` | `-` |
| `ldr` at 0x6a0d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6a0dc | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x6a0f0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6a0f0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6a0f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6a0e0 | `(unknown callee) 0x65068` | `[0x132ae4]` | `[0x132adc]` | `r0` | `r1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6a0f0 | `r0` | `ret(0x65068)` |

## `L1CfgGetIndexBrick`

- `L1CfgGetIndexBrick`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6a038 to 0x6a0b0.
  Established.
  A negative signed entry `r0` takes `0x6a070`, while nonnegative values fall through (`blt` at 0x6a054).
  The signed condition `[0x132adc] <= r0` also takes `0x6a070`, while greater values fall through (`ble` at 0x6a068).
  An included word assignment at `sp -0x18` writes zero (`str` at 0x6a074).
  Brick index bounds meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6a048 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x6a04c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6a058 | load | 4 | `abs 0x6a0ac` | `-` |
| `ldr` at 0x6a05c | load | 4 | `abs 0x132adc` | `-` |
| `ldr` at 0x6a060 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6a074 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x6a07c | load | 4 | `abs 0x6a0ac` | `-` |
| `ldr` at 0x6a080 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6a094 | load | 4 | `abs 0x132ae4` | `-` |
| `str` at 0x6a09c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x6a0a0 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x6a0a8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6a0a8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6a0a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x6a054 | `r0 < 0x0 (signed, 32-bit)` | `r0 >= 0x0 (signed, 32-bit)` |
| `ble` at 0x6a068 | `[0x132adc] <= r0 (signed, 32-bit)` | `[0x132adc] > r0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6a0a8 | `r0` | `?` |

## `L1CfgBrickCount`

- `L1CfgBrickCount`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6a370 to 0x6a390.
  Established.
  The absolute word at `0x132adc` is read (`ldr` at 0x6a380).
  The included return record carries `[0x132adc]` in `r0` (`ldm` at 0x6a388).
  Brick count interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x6a37c | load | 4 | `abs 0x6a38c` | `-` |
| `ldr` at 0x6a380 | load | 4 | `abs 0x132adc` | `-` |
| `ldm` at 0x6a388 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6a388 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6a388 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6a388 | `r0` | `[0x132adc]` |

## `L1CfgFindBrick`

- `L1CfgFindBrick`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x65068 to 0x65184.
  Established.
  A zero entry `r0` takes `0x65130`, while nonzero falls through (`beq` at 0x650ac).
  The word at `sp -0x28` receives `0x1` on an included path (`str` at 0x6511c).
  The word at `sp -0x2c` receives zero in the included fallback arm (`str` at 0x65170).
  Brick search matching remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x65078 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6507c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x65080 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x65084 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0x6508c | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x65094 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x65098 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x650a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x650b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x650b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x650c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x650d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x650dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x650e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x650ec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x65100 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65108 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6510c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x6511c | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x65120 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x65128 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x65130 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6513c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x65150 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x65158 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x6515c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x65164 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0x65170 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x65174 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x65180 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x65180 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x65180 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x650a0 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x650ac | `r0 == 0x0` | `r0 != 0x0` |
| `bge` at 0x650bc | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x650e8 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x65114 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x65138 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x65180 | `r0` | `?` |

## `L1CfgFindState`

- `L1CfgFindState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x65184 to 0x652dc.
  Established.
  The inequality `[0x132a58] != r2` takes `0x651d8`, while equality falls through (`bne` at 0x651b4).
  An included word assignment at `sp -0x2c` writes `0x132a50` (`str` at 0x651d0).
  The word at `sp -0x30` receives `[sp-48]-112` on another included path (`str` at 0x652b4).
  State search result semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x65194 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x65198 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x6519c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x651a0 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x651a4 | load | 4 | `abs 0x652d8` | `-` |
| `ldr` at 0x651a8 | load | 4 | `abs 0x132a58` | `-` |
| `ldr` at 0x651ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x651b8 | load | 4 | `abs 0x652d8` | `-` |
| `ldr` at 0x651bc | load | 4 | `abs 0x132a5c` | `-` |
| `ldr` at 0x651c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x651cc | load | 4 | `abs 0x652d8` | `-` |
| `str` at 0x651d0 | store | 4 | `sp -0x2c` | `0x132a50` |
| `str` at 0x651dc | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x651e4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x651e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x651f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65200 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x65204 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x65210 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x65224 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6522c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65230 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6523c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x65250 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65258 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6525c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x6526c | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x65270 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x65278 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x65280 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6528c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x652a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x652a8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x652ac | load | 4 | `sp -0x30` | `-` |
| `str` at 0x652b4 | store | 4 | `sp -0x30` | `[sp-48]-112` |
| `str` at 0x652c0 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x652c4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x652c8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x652cc | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x652d4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x652d4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x652d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x651b4 | `[0x132a58] != r2` | `[0x132a58] == r2` |
| `bne` at 0x651c8 | `[0x132a5c] != r3` | `[0x132a5c] == r3` |
| `bne` at 0x651f0 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x651fc | `Z == 1` | `not (Z == 1)` |
| `bge` at 0x6520c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x65238 | `? != [sp-28]` | `? == [sp-28]` |
| `bne` at 0x65264 | `? != [sp-32]` | `? == [sp-32]` |
| `beq` at 0x65288 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x652d4 | `r0` | `?` |
