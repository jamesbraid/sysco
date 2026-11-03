# ARM Linux 2.6 power-bay control routines in 1.44.0

Read from `6.5.30/usr/cpu/firmware/sysco/l1.bin`, package `6.5.30:l1#6` (version 1.44.0),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0xa936a8`.
Carrier SHA-256: `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
Package SHA-256: `e57ab8ecd48858e39ecdd0050a23e444a491c35e87f9d8ebb48881fc25229a06`.
Inner executable SHA-256: `b33154e53b00531623db6ff365e80b1c1825bc9427961c59fdc132c24f0c819e`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1629656 bytes.
Segment SHA-256: `f6491062ef0a2afd254f8510f82b4814c563c9256691f9a2b393cf6e84ba7fd8`. Analysed routine spans use the exact
ELF function symbols and sizes. They can include trailing literal words after a return.

`sp` denotes the entry stack pointer. Register expressions such as `r0` denote entry
values, and brackets denote loaded values. Decimal numbers inside expressions retain the
exact tool notation. Call candidates use the assumed ARM AAPCS
register locations r0 through r3 with unknown arity. Custom calling conventions remain unknown.
Stack stores appear in the access tables.
Unknown values and predicates include unsupported instruction effects and merged paths.
Numeric call targets have unknown callee names in the flat-image analysis. Device roles
and pointee meanings remain unknown unless the body establishes them. Tables include the
explicit memory accesses reported on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Separate loads at the same printed address need not observe the same value.
Computed transfers with unresolved edges are identified under their routines. Effects on omitted paths remain unknown.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-pbay-control-facts.txt) retain all tool sections.

## `PBayInit`

- `PBayInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe8f50 to 0xe9054.
  Established. The call to unnamed 0x201fc has its result stored as a word at absolute 0x1b0ee0 (`bl` at 0xe8f70, `str` at 0xe8f78).
  A halfword store writes zero at absolute 0x1b0ed8 (`strh` at 0xe8fa8).
  Printed candidates for the call to unnamed 0x22540 include r1=0x1848c8, r2=0xe9c6c and r3=1 (`bl` at 0xe9004).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xe8f68 | load | 4 | `abs 0xe9044` | `-` |
| `str` at 0xe8f78 | store | 4 | `abs 0x1b0ee0` | `ret(0x201fc)` |
| `ldr` at 0xe8f7c | load | 4 | `abs 0xe9044` | `-` |
| `ldr` at 0xe8f80 | load | 4 | `abs 0x1b0ee0` | `-` |
| `ldr` at 0xe8f90 | load | 4 | `abs 0xe9048` | `-` |
| `ldr` at 0xe8fa0 | load | 4 | `abs 0xe9048` | `-` |
| `strh` at 0xe8fa8 | store | 2 | `abs 0x1b0ed8` | `0x0` |
| `ldr` at 0xe8fac | load | 4 | `abs 0xe9044` | `-` |
| `ldr` at 0xe8fb0 | load | 4 | `abs 0x1b0ee0` | `-` |
| `ldr` at 0xe8fb4 | load | 4 | `abs 0xe9048` | `-` |
| `str` at 0xe8fb8 | store | 4 | `[0x1b0ee0] +0x8` | `0x1b0ed0` |
| `ldr` at 0xe8fbc | load | 4 | `abs 0xe9044` | `-` |
| `ldr` at 0xe8fc0 | load | 4 | `abs 0x1b0ee0` | `-` |
| `ldr` at 0xe8fc8 | load | 4 | `abs 0xe9044` | `-` |
| `ldr` at 0xe8fcc | load | 4 | `abs 0x1b0ee0` | `-` |
| `str` at 0xe8fd0 | store | 4 | `sp -0x2c` | `[0x1b0ee0]` |
| `str` at 0xe8fd8 | store | 4 | `sp -0x28` | `0xa00` |
| `str` at 0xe8fe0 | store | 4 | `sp -0x24` | `0x19` |
| `str` at 0xe8fe8 | store | 4 | `sp -0x20` | `0x28` |
| `str` at 0xe8ff0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xe8ff8 | load | 4 | `abs 0xe904c` | `-` |
| `ldr` at 0xe8ffc | load | 4 | `abs 0xe9050` | `-` |
| `str` at 0xe900c | store | 4 | `sp -0x18` | `ret(0x22540)` |
| `ldr` at 0xe9010 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9030 | load | 4 | `abs 0xe9044` | `-` |
| `ldr` at 0xe9034 | load | 4 | `abs 0x1b0ee0` | `-` |
| `ldm` at 0xe9040 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xe9040 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe9040 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe9040 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe8f64 | `(unknown callee) 0xe9054` | `0x1` | `r1` | `r2` | `r3` |
| `bl` at 0xe8f70 | `(unknown callee) 0x201fc` | `0x110` | `?` | `?` | `?` |
| `bl` at 0xe8f8c | `(unknown callee) 0x20550` | `[0x1b0ee0]` | `0x0` | `0x110` | `0x1b0ee0` |
| `bl` at 0xe8f9c | `(unknown callee) 0x20550` | `0x1b0ed0` | `0x0` | `0x10` | `?` |
| `bl` at 0xe9004 | `(unknown callee) 0x22540` | `[0x1b0ee0]+4` | `0x1848c8` | `0xe9c6c` | `0x1` |
| `bl` at 0xe9024 | `(unknown callee) 0xe96f8` | `?` | `0x7` | `?` | `ret(0x22540)` |
| `bl` at 0xe902c | `(unknown callee) 0xe9054` | `0x5` | `?` | `?` | `?` |
| `bl` at 0xe9038 | `(unknown callee) 0x20514` | `[0x1b0ee0]` | `?` | `?` | `0x1b0ee0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe9018 | `ret(0x22540) == 0x0` | `ret(0x22540) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe9040 | `r0` | `?` |

## `PBaySetState`

- `PBaySetState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe9054 to 0xe9130.
  Established. A word store writes entry r0 at absolute address 0x1b0ecc (`str` at 0xe9070).
  The predicated load into pc has an unresolved target for [0x1b0ecc] <= 5 (unsigned, 32-bit) (`ldr` at 0xe9078, `ldrls` at 0xe9080).
  It falls through for [0x1b0ecc] > 5 (unsigned, 32-bit) (`ldr` at 0xe9078, `ldrls` at 0xe9080).
  A known-edge word store writes 0x18493c at absolute address 0x1a0400 (`str` at 0xe9108).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe9064 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xe9068 | load | 4 | `abs 0xe9110` | `-` |
| `ldr` at 0xe906c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe9070 | store | 4 | `abs 0x1b0ecc` | `r0` |
| `ldr` at 0xe9074 | load | 4 | `abs 0xe9110` | `-` |
| `ldr` at 0xe9078 | load | 4 | `abs 0x1b0ecc` | `-` |
| `ldrls` at 0xe9080 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe9100 | load | 4 | `abs 0xe9114` | `-` |
| `ldr` at 0xe9104 | load | 4 | `abs 0xe912c` | `-` |
| `str` at 0xe9108 | store | 4 | `abs 0x1a0400` | `0x18493c` |
| `ldm` at 0xe910c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xe910c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe910c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe910c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xe9080 | `[0x1b0ecc] <= 0x5 (unsigned, 32-bit)` | `[0x1b0ecc] > 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe910c | `r0` | `r0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xe9080).

## `PBayGetState`

- `PBayGetState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe9130 to 0xe9150.
  Established. The word load at 0xe9140 reads absolute 0x1b0ecc (`ldr` at 0xe9140).
  A mov copies the loaded word to r0 (`ldr` at 0xe9140, `mov` at 0xe9144).
  The return record retains [0x1b0ecc] (`ldm` at 0xe9148).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xe913c | load | 4 | `abs 0xe914c` | `-` |
| `ldr` at 0xe9140 | load | 4 | `abs 0x1b0ecc` | `-` |
| `ldm` at 0xe9148 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe9148 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe9148 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe9148 | `r0` | `[0x1b0ecc]` |

## `PBayCmd`

- `PBayCmd`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe9150 to 0xe96f8.
  Established. One call to unnamed 0x201fc has its result stored at the address held in [sp-32] (`bl` at 0xe91dc, `str` at 0xe91e4).
  Another call to that unnamed address records its own result at the address held in [sp-32] (`bl` at 0xe921c, `str` at 0xe9224).
  The transfer at 0xe92dc remains unresolved for the unsigned predicate [[[sp-28]+112]+4]-1 <= 0xf, with fall-through for values greater than 0xf (`ldr` at 0xe92d0, `sub` at 0xe92d4, `ldrls` at 0xe92dc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe9160 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0xe9164 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0xe916c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe9170 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe9178 | store | 4 | `r1 +0x0` | `0x0` |
| `ldr` at 0xe917c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe9180 | load | 4 | `[sp-28] +0x70` | `-` |
| `ldr` at 0xe9184 | load | 4 | `[[sp-28]+112] +0x0` | `-` |
| `ldr` at 0xe9188 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe91a0 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xe91a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe91b0 | load | 4 | `[sp-28] +0x70` | `-` |
| `ldr` at 0xe91b8 | load | 4 | `[[sp-28]+112] +0x4` | `-` |
| `ldr` at 0xe91d4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe91e4 | store | 4 | `[sp-32] +0x0` | `ret(0x201fc)` |
| `ldr` at 0xe91e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe91ec | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xe91f0 | load | 4 | `abs 0xe96d0` | `-` |
| `ldr` at 0xe91f8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe91fc | store | 4 | `sp -0x34` | `[sp-36]` |
| `ldr` at 0xe9204 | load | 4 | `abs 0xe96d4` | `-` |
| `ldr` at 0xe9208 | load | 4 | `abs 0x1a0400` | `-` |
| `ldr` at 0xe9214 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe9224 | store | 4 | `[sp-32] +0x0` | `ret(0x201fc)` |
| `ldr` at 0xe9228 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe922c | load | 4 | `abs 0xe96d4` | `-` |
| `ldr` at 0xe9230 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xe9234 | load | 4 | `abs 0x1a0400` | `-` |
| `str` at 0xe9240 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe9248 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe9250 | load | 4 | `[sp-28] +0x70` | `-` |
| `ldr` at 0xe9258 | load | 4 | `[[sp-28]+112] +0x4` | `-` |
| `ldr` at 0xe9264 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe9270 | store | 4 | `sp -0x24` | `ret(0xe9bf8)` |
| `ldr` at 0xe9274 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe9278 | store | 4 | `sp -0x34` | `ret(0xe9bf8)` |
| `ldr` at 0xe9280 | load | 4 | `abs 0xe96d8` | `-` |
| `ldr` at 0xe9284 | load | 4 | `abs 0x1b0ee0` | `-` |
| `str` at 0xe929c | store | 4 | `sp -0x2c` | `ret(0x22ca0)` |
| `ldr` at 0xe92a0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe92ac | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe92b0 | store | 4 | `sp -0x34` | `[sp-36]` |
| `ldr` at 0xe92c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe92c8 | load | 4 | `[sp-28] +0x70` | `-` |
| `ldr` at 0xe92d0 | load | 4 | `[[sp-28]+112] +0x4` | `-` |
| `ldrls` at 0xe92dc | load | 4 | `(unknown)` | `-` |
| `str` at 0xe9680 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe9684 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe968c | load | 4 | `[sp-28] +0x70` | `-` |
| `ldr` at 0xe9694 | load | 4 | `[[sp-28]+112] +0x4` | `-` |
| `ldr` at 0xe96a8 | load | 4 | `abs 0xe96d8` | `-` |
| `ldr` at 0xe96ac | load | 4 | `abs 0x1b0ee0` | `-` |
| `ldr` at 0xe96bc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe96c0 | store | 4 | `sp -0x34` | `[sp-36]` |
| `ldr` at 0xe96c4 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0xe96cc | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xe96cc | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xe96cc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe96cc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe96cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe918c | `(unknown callee) 0xc4d10` | `[[[sp-28]+112]]` | `[sp-32]` | `r1` | `[[sp-28]+112]` |
| `bl` at 0xe91c4 | `(unknown callee) 0xe9130` | `ret(0xc4d10)` | `?` | `0x4` | `[[[sp-28]+112]+4]` |
| `bl` at 0xe91dc | `(unknown callee) 0x201fc` | `0x50` | `?` | `?` | `ret(0xe9130)` |
| `bl` at 0xe91f4 | `(unknown callee) 0x20634` | `[[sp-32]]` | `0x184950` | `?` | `[sp-32]` |
| `bl` at 0xe921c | `(unknown callee) 0x201fc` | `0x50` | `?` | `0x4` | `[0x1a0400]` |
| `bl` at 0xe9238 | `(unknown callee) 0x20634` | `[[sp-32]]` | `[0x1a0400]` | `0x1a0400` | `[sp-32]` |
| `bl` at 0xe9268 | `(unknown callee) 0xe9bf8` | `[sp-32]` | `?` | `0x4` | `[[[sp-28]+112]+4]` |
| `bl` at 0xe9294 | `(unknown callee) 0x22ca0` | `[0x1b0ee0]+176` | `0x0` | `0x4` | `[0x1b0ee0]+176` |
| `bl` at 0xe92bc | `(unknown callee) 0xe9054` | `0x3` | `?` | `?` | `ret(0x22ca0)` |
| `bl` at 0xe96a4 | `(unknown callee) 0xe9054` | `0x2` | `?` | `0x4` | `[[[sp-28]+112]+4]` |
| `bl` at 0xe96b8 | `(unknown callee) 0x2302c` | `[0x1b0ee0]+176` | `?` | `?` | `[0x1b0ee0]+176` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe9198 | `ret(0xc4d10) == 0x0` | `ret(0xc4d10) != 0x0` |
| `bne` at 0xe91c0 | `[[[sp-28]+112]+4] != 0xf` | `[[[sp-28]+112]+4] == 0xf` |
| `beq` at 0xe91d0 | `ret(0xe9130) == 0x5` | `ret(0xe9130) != 0x5` |
| `beq` at 0xe9210 | `[0x1a0400] == 0x0` | `[0x1a0400] != 0x0` |
| `bne` at 0xe9260 | `[[[sp-28]+112]+4] != 0x10` | `[[[sp-28]+112]+4] == 0x10` |
| `beq` at 0xe92a8 | `ret(0x22ca0) == 0x0` | `ret(0x22ca0) != 0x0` |
| `ldrls` at 0xe92dc | `[[[sp-28]+112]+4]-1 <= 0xf (unsigned, 32-bit)` | `[[[sp-28]+112]+4]-1 > 0xf (unsigned, 32-bit)` |
| `beq` at 0xe969c | `[[[sp-28]+112]+4] == 0xf` | `[[[sp-28]+112]+4] != 0xf` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe96cc | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xe92dc).

## `PBayError`

- `PBayError`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe96f8 to 0xe9738.
  Established. Word stores save entry r0 and r1 at sp-0x14 and sp-0x18 (`str` at 0xe9708, `str` at 0xe970c).
  A word load reads absolute 0x1a288c (`ldr` at 0xe9714).
  The tool reports r0=[0x1a288c], r1=0x1849c8, r2=r0 and r3=r1 as candidates for the call to unnamed 0x244d4 (`bl` at 0xe9724).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe9708 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe970c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xe9710 | load | 4 | `abs 0xe9730` | `-` |
| `ldr` at 0xe9714 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xe9718 | load | 4 | `abs 0xe9734` | `-` |
| `ldr` at 0xe971c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe9720 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xe972c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe972c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe972c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe9724 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x1849c8` | `r0` | `r1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe972c | `r0` | `ret(0x244d4)` |

## `PBayInitFunc`

- `PBayInitFunc`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe9c6c to 0xe9d30.
  Established. The branch takes 0xe9ccc when the result of the call to unnamed 0x22b84 is zero, and falls through when it is nonzero (`bl` at 0xe9ca4, `beq` at 0xe9cb0).
  A halfword store at [[sp-36]+8]+4 has an unknown value (`strh` at 0xe9d0c).
  The call to unnamed 0xe9054 at 0xe9d20 has r0=5 as a printed candidate (`bl` at 0xe9d20).
  The merged return record is ret(0xe9054) (`bl` at 0xe9cc4, `bl` at 0xe9d20, `ldm` at 0xe9d28).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe9c7c | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0xe9c80 | store | 4 | `sp -0x20` | `r1` |
| `ldr` at 0xe9c84 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe9c88 | store | 4 | `sp -0x24` | `r1` |
| `ldr` at 0xe9c8c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe9c98 | load | 4 | `abs 0xe9d2c` | `-` |
| `ldr` at 0xe9cd4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe9cd8 | load | 4 | `[sp-36] +0x8` | `-` |
| `strh` at 0xe9d0c | store | 2 | `[[sp-36]+8] +0x4` | `?` |
| `ldm` at 0xe9d28 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xe9d28 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xe9d28 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe9d28 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe9d28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe9ca4 | `(unknown callee) 0x22b84` | `r1+176` | `0x1849fc` | `0x0` | `?` |
| `bl` at 0xe9cbc | `(unknown callee) 0xe96f8` | `?` | `0x11` | `?` | `ret(0x22b84)` |
| `bl` at 0xe9cc4 | `(unknown callee) 0xe9054` | `0x5` | `?` | `?` | `?` |
| `bl` at 0xe9cd0 | `(unknown callee) 0xe9054` | `0x1` | `?` | `?` | `ret(0x22b84)` |
| `bl` at 0xe9ce0 | `(unknown callee) 0x28230` | `0x1` | `?` | `?` | `[sp-36]` |
| `bl` at 0xe9cf4 | `(unknown callee) 0x28230` | `0x2` | `?` | `?` | `?` |
| `bl` at 0xe9d18 | `(unknown callee) 0xe96f8` | `?` | `0x2` | `?` | `?` |
| `bl` at 0xe9d20 | `(unknown callee) 0xe9054` | `0x5` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe9cb0 | `ret(0x22b84) == 0x0` | `ret(0x22b84) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe9d28 | `r0` | `ret(0xe9054)` |

## `PBayInitIcmb`

- `PBayInitIcmb`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe9d30 to 0xea010.
  Established. Halfword stores write zero and 0x4400 at [[sp-24]+8]+6 (`strh` at 0xe9d58, `strh` at 0xe9e70).
  A halfword store writes the low 16 bits of the result from unnamed 0xede7c at [[sp-24]+8]+6 (`bl` at 0xe9dfc, `strh` at 0xe9e00).
  A byte loaded from the result of unnamed 0xedbdc plus one is stored as a byte at [[sp-24]+8]+1 (`bl` at 0xe9f60, `ldrb` at 0xe9f74, `strb` at 0xe9f78).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe9d40 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xe9d4c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9d50 | load | 4 | `[sp-24] +0x8` | `-` |
| `strh` at 0xe9d58 | store | 2 | `[[sp-24]+8] +0x6` | `0x0` |
| `ldr` at 0xe9d5c | load | 4 | `abs 0xea00c` | `-` |
| `strh` at 0xe9d68 | store | 2 | `sp -0x1b4` | `0x0` |
| `str` at 0xe9d70 | store | 4 | `sp -0x1b0` | `0x10` |
| `str` at 0xe9d78 | store | 4 | `sp -0x11c` | `0x0` |
| `str` at 0xe9d80 | store | 4 | `sp -0x118` | `0xff` |
| `str` at 0xe9d88 | store | 4 | `sp -0x28` | `0x3` |
| `str` at 0xe9d90 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe9d94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9d9c | load | 4 | `[sp-24] +0x8` | `-` |
| `str` at 0xe9dac | store | 4 | `sp -0x1b0` | `0x2` |
| `str` at 0xe9db4 | store | 4 | `sp -0x28` | `0x3` |
| `str` at 0xe9dbc | store | 4 | `sp -0x24` | `0x3` |
| `str` at 0xe9dc4 | store | 4 | `sp -0x11c` | `0x0` |
| `ldr` at 0xe9dc8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9dd0 | load | 4 | `[sp-24] +0x8` | `-` |
| `str` at 0xe9de0 | store | 4 | `sp -0x1c` | `ret(0xea010)` |
| `ldr` at 0xe9de4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe9df0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9df4 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xe9df8 | load | 4 | `sp -0x2c` | `-` |
| `strh` at 0xe9e00 | store | 2 | `[[sp-24]+8] +0x6` | `(ret(0xede7c) & 0xffff)` |
| `str` at 0xe9e08 | store | 4 | `sp -0x1b0` | `0x12` |
| `str` at 0xe9e10 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0xe9e14 | load | 4 | `abs 0xea00c` | `-` |
| `ldr` at 0xe9e18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9e1c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrh` at 0xe9e20 | load | 2 | `[[sp-24]+8] +0x6` | `-` |
| `strh` at 0xe9e28 | store | 2 | `sp -0x1b4` | `([[[sp-24]+8]+6]:2 & 0xffff)` |
| `str` at 0xe9e30 | store | 4 | `sp -0x11c` | `0x0` |
| `ldr` at 0xe9e34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9e3c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xe9e48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9e4c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrh` at 0xe9e50 | load | 2 | `[[sp-24]+8] +0x6` | `-` |
| `ldr` at 0xe9e64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9e68 | load | 4 | `[sp-24] +0x8` | `-` |
| `strh` at 0xe9e70 | store | 2 | `[[sp-24]+8] +0x6` | `0x4400` |
| `ldr` at 0xe9e78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9e7c | load | 4 | `[sp-24] +0x8` | `-` |
| `strb` at 0xe9e84 | store | 1 | `[[sp-24]+8] +0x0` | `0x0` |
| `str` at 0xe9e8c | store | 4 | `sp -0x1b0` | `0x13` |
| `str` at 0xe9e94 | store | 4 | `sp -0x24` | `0x3` |
| `str` at 0xe9e9c | store | 4 | `sp -0x11c` | `0x0` |
| `ldr` at 0xe9ea0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9ea8 | load | 4 | `[sp-24] +0x8` | `-` |
| `str` at 0xe9eb8 | store | 4 | `sp -0x1c` | `ret(0xea010)` |
| `ldr` at 0xe9ebc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe9ed4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9ed8 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xe9edc | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xe9ee0 | load | 1 | `[sp-44] +0x0` | `-` |
| `strb` at 0xe9ee4 | store | 1 | `[[sp-24]+8] +0x0` | `([[sp-44]]:1 & 0xff)` |
| `str` at 0xe9eec | store | 4 | `sp -0x1b0` | `0x20` |
| `str` at 0xe9ef4 | store | 4 | `sp -0x11c` | `0x0` |
| `str` at 0xe9efc | store | 4 | `sp -0x118` | `0x0` |
| `ldr` at 0xe9f04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9f08 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrb` at 0xe9f0c | load | 1 | `[[sp-24]+8] +0x0` | `-` |
| `strb` at 0xe9f14 | store | 1 | `(unknown)` | `([[[sp-24]+8]]:1 & 0xff)` |
| `str` at 0xe9f1c | store | 4 | `sp -0x110` | `0x0` |
| `strb` at 0xe9f28 | store | 1 | `sp -0x10c` | `0x0` |
| `str` at 0xe9f30 | store | 4 | `sp -0xc8` | `0x0` |
| `ldr` at 0xe9f34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9f3c | load | 4 | `[sp-24] +0x8` | `-` |
| `str` at 0xe9f4c | store | 4 | `sp -0x1c` | `ret(0xea010)` |
| `ldr` at 0xe9f50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe9f5c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe9f64 | store | 4 | `sp -0x1b8` | `ret(0xedbdc)` |
| `ldr` at 0xe9f68 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9f6c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xe9f70 | load | 4 | `sp -0x1b8` | `-` |
| `ldrb` at 0xe9f74 | load | 1 | `ret(0xedbdc) +0x1` | `-` |
| `strb` at 0xe9f78 | store | 1 | `[[sp-24]+8] +0x1` | `([ret(0xedbdc)+1]:1 & 0xff)` |
| `ldr` at 0xe9f7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9f80 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xe9f84 | load | 4 | `sp -0x1b8` | `-` |
| `ldrb` at 0xe9f88 | load | 1 | `[sp-440] +0x2` | `-` |
| `strb` at 0xe9f8c | store | 1 | `[[sp-24]+8] +0x2` | `([[sp-440]+2]:1 & 0xff)` |
| `ldr` at 0xe9f90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9f94 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xe9f98 | load | 4 | `sp -0x1b8` | `-` |
| `ldrb` at 0xe9f9c | load | 1 | `[sp-440] +0x3` | `-` |
| `strb` at 0xe9fa0 | store | 1 | `[[sp-24]+8] +0x3` | `([[sp-440]+3]:1 & 0xff)` |
| `ldr` at 0xe9fa4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9fa8 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrb` at 0xe9fac | load | 1 | `[[sp-24]+8] +0x1` | `-` |
| `ldr` at 0xe9fb8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9fbc | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrb` at 0xe9fc0 | load | 1 | `[[sp-24]+8] +0x1` | `-` |
| `ldr` at 0xe9fcc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9fd0 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrb` at 0xe9fd4 | load | 1 | `[[sp-24]+8] +0x2` | `-` |
| `ldr` at 0xe9fe0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9fe4 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xe9fec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9ff0 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldm` at 0xea008 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xea008 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xea008 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xea008 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe9d48 | `(unknown callee) 0xe9054` | `0x1` | `r1` | `r2` | `r3` |
| `bl` at 0xe9da4 | `(unknown callee) 0xea010` | `[[sp-24]+8]` | `sp-436` | `sp-436` | `[sp-24]` |
| `bl` at 0xe9dd8 | `(unknown callee) 0xea010` | `[[sp-24]+8]` | `sp-436` | `sp-436` | `[sp-24]` |
| `bl` at 0xe9dfc | `(unknown callee) 0xede7c` | `[sp-44]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xe9e44 | `(unknown callee) 0xea010` | `[[sp-24]+8]` | `sp-436` | `sp-436` | `[sp-24]` |
| `bl` at 0xe9e60 | `(unknown callee) 0xe9054` | `0x5` | `?` | `?` | `[[[sp-24]+8]+6]:2` |
| `bl` at 0xe9eb0 | `(unknown callee) 0xea010` | `[[sp-24]+8]` | `sp-436` | `sp-436` | `[sp-24]` |
| `bl` at 0xe9ecc | `(unknown callee) 0xe9054` | `0x5` | `?` | `?` | `ret(0xea010)` |
| `bl` at 0xe9f44 | `(unknown callee) 0xea010` | `[[sp-24]+8]` | `sp-436` | `sp-436` | `[sp-24]` |
| `bl` at 0xe9f60 | `(unknown callee) 0xedbdc` | `[sp-44]` | `?` | `?` | `ret(0xea010)` |
| `bl` at 0xe9fc8 | `(unknown callee) 0xed590` | `[[[sp-24]+8]+1]:1` | `?` | `[[sp-24]+8]` | `[[[sp-24]+8]+1]:1` |
| `bl` at 0xe9fe8 | `(unknown callee) 0xea4c4` | `[[sp-24]+8]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xe9ff8 | `(unknown callee) 0xec2cc` | `[[sp-24]+8]` | `0x0` | `?` | `[sp-24]` |
| `bl` at 0xea000 | `(unknown callee) 0xe9054` | `0x2` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe9dec | `ret(0xea010) == 0x0` | `ret(0xea010) != 0x0` |
| `bne` at 0xe9e58 | `[[[sp-24]+8]+6]:2 != 0x0` | `[[[sp-24]+8]+6]:2 == 0x0` |
| `bne` at 0xe9ec4 | `ret(0xea010) != 0x0` | `ret(0xea010) == 0x0` |
| `beq` at 0xe9f58 | `ret(0xea010) == 0x0` | `ret(0xea010) != 0x0` |
| `beq` at 0xe9fb4 | `[[[sp-24]+8]+1]:1 == 0x0` | `[[[sp-24]+8]+1]:1 != 0x0` |
| `beq` at 0xe9fdc | `[[[sp-24]+8]+2]:1 == 0x0` | `[[[sp-24]+8]+2]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xea008 | `r0` | `ret(0xe9054)` |

## `IcmbMsgRqRs`

- `IcmbMsgRqRs`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xea010 to 0xea49c.
  Established. The result from unnamed 0xed99c is stored as a word at entry r1 plus 0x98 (`bl` at 0xea08c, `str` at 0xea090).
  The result from unnamed 0xee0e0 is stored at the address formed by [sp-28] plus 0x188 (`bl` at 0xea3a0, `str` at 0xea3a4).
  A load, add and store sequence writes [sp-368]+1 at sp-0x170 (`ldr` at 0xea1f0, `add` at 0xea1f4, `str` at 0xea1f8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xea020 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xea024 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xea02c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xea030 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea034 | load | 4 | `r1 +0x9c` | `-` |
| `ldr` at 0xea040 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea044 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xea048 | load | 1 | `r1 +0xa0` | `-` |
| `ldr` at 0xea04c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xea050 | load | 1 | `r1 +0xa8` | `-` |
| `ldr` at 0xea054 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea058 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea05c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xea064 | store | 4 | `sp -0x1b4` | `r1+169` |
| `ldr` at 0xea068 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea06c | load | 4 | `r1 +0xec` | `-` |
| `str` at 0xea070 | store | 4 | `sp -0x1b0` | `[r1+236]` |
| `ldr` at 0xea074 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xea07c | store | 4 | `sp -0x1ac` | `r1+8` |
| `ldr` at 0xea084 | load | 4 | `r1 +0xa4` | `-` |
| `ldr` at 0xea088 | load | 4 | `r1 +0x9c` | `-` |
| `str` at 0xea090 | store | 4 | `r1 +0x98` | `ret(0xed99c)` |
| `strb` at 0xea0a8 | store | 1 | `(unknown)` | `(ret(0xea904) & 0xff)` |
| `ldrb` at 0xea0b4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xea0b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea0bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea0c4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xea0cc | store | 4 | `sp -0x1b4` | `sp-184` |
| `ldr` at 0xea0d4 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xea0dc | load | 4 | `[sp-28] +0x98` | `-` |
| `str` at 0xea0e8 | store | 4 | `sp -0xbc` | `ret(0xee13c)` |
| `ldr` at 0xea0ec | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xea0f0 | load | 2 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xea0f4 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xea0f8 | load | 2 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xea100 | load | 4 | `sp -0xbc` | `-` |
| `str` at 0xea10c | store | 4 | `sp -0x1b4` | `sp-340` |
| `str` at 0xea124 | store | 4 | `sp -0x158` | `ret(0xedcd0)` |
| `str` at 0xea12c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xea130 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea134 | load | 4 | `[sp-28] +0x18c` | `-` |
| `ldr` at 0xea138 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xea144 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xea150 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xea164 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea168 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xea174 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xea17c | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xea18c | store | 4 | `sp -0x19c` | `0x3` |
| `ldr` at 0xea190 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xea198 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xea1a8 | store | 4 | `sp -0x170` | `0x0` |
| `ldr` at 0xea1ac | load | 4 | `sp -0x170` | `-` |
| `ldr` at 0xea1b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xea1c0 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xea1c8 | load | 4 | `sp -0x158` | `-` |
| `str` at 0xea1d4 | store | 4 | `sp -0x16c` | `ret(0x10f690)` |
| `ldr` at 0xea1d8 | load | 4 | `sp -0x16c` | `-` |
| `ldr` at 0xea1dc | load | 4 | `abs 0xea494` | `-` |
| `ldr` at 0xea1f0 | load | 4 | `sp -0x170` | `-` |
| `str` at 0xea1f8 | store | 4 | `sp -0x170` | `[sp-368]+1` |
| `ldr` at 0xea200 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea204 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xea214 | store | 4 | `sp -0x19c` | `0x0` |
| `ldr` at 0xea218 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xea220 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xea22c | load | 4 | `sp -0x170` | `-` |
| `str` at 0xea240 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xea244 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea248 | load | 4 | `[sp-28] +0x190` | `-` |
| `ldr` at 0xea24c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xea258 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xea264 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xea268 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea270 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xea284 | store | 4 | `sp -0x168` | `ret(0x10f4b8)` |
| `ldr` at 0xea288 | load | 4 | `sp -0x168` | `-` |
| `ldr` at 0xea28c | load | 4 | `abs 0xea498` | `-` |
| `ldr` at 0xea29c | load | 4 | `sp -0x168` | `-` |
| `ldr` at 0xea2ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea2b4 | load | 4 | `sp -0x168` | `-` |
| `str` at 0xea2e0 | store | 4 | `sp -0x164` | `ret(0xeddac)` |
| `ldr` at 0xea2e4 | load | 4 | `sp -0x164` | `-` |
| `ldr` at 0xea2f4 | load | 4 | `sp -0x160` | `-` |
| `str` at 0xea300 | store | 4 | `sp -0x178` | `ret(0xedf20)` |
| `ldr` at 0xea304 | load | 4 | `sp -0x178` | `-` |
| `ldr` at 0xea318 | load | 4 | `sp -0x160` | `-` |
| `strb` at 0xea328 | store | 1 | `(unknown)` | `(ret(0xedfb8) & 0xff)` |
| `ldr` at 0xea32c | load | 4 | `sp -0x160` | `-` |
| `str` at 0xea338 | store | 4 | `sp -0x180` | `ret(0xee03c)` |
| `ldr` at 0xea33c | load | 4 | `sp -0x160` | `-` |
| `str` at 0xea348 | store | 4 | `sp -0x184` | `ret(0xee0a8)` |
| `ldr` at 0xea34c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea350 | load | 4 | `sp -0x184` | `-` |
| `str` at 0xea354 | store | 4 | `[sp-28] +0x194` | `ret(0xee0a8)` |
| `ldr` at 0xea358 | load | 4 | `sp -0x184` | `-` |
| `ldr` at 0xea364 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea368 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xea36c | load | 4 | `sp -0x180` | `-` |
| `ldrb` at 0xea384 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xea38c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xea398 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea39c | load | 4 | `sp -0x160` | `-` |
| `str` at 0xea3a4 | store | 4 | `[sp-28] +0x188` | `ret(0xee0e0)` |
| `ldr` at 0xea3a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea3ac | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xea3b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea3bc | load | 4 | `[sp-28] +0x188` | `-` |
| `str` at 0xea3c8 | store | 4 | `sp -0x184` | `ret(0xedba4)` |
| `ldr` at 0xea3cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea3d0 | load | 4 | `[sp-28] +0x188` | `-` |
| `str` at 0xea3dc | store | 4 | `sp -0x180` | `ret(0xedb6c)` |
| `ldr` at 0xea3e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea3e4 | load | 4 | `[sp-28] +0x188` | `-` |
| `str` at 0xea3f0 | store | 4 | `sp -0x188` | `ret(0xedb28)` |
| `ldr` at 0xea3f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea3f8 | load | 4 | `[sp-28] +0x188` | `-` |
| `str` at 0xea404 | store | 4 | `sp -0x18c` | `ret(0xedae4)` |
| `ldr` at 0xea408 | load | 4 | `sp -0x184` | `-` |
| `ldr` at 0xea414 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea418 | load | 4 | `[sp-28] +0x9c` | `-` |
| `ldr` at 0xea41c | load | 4 | `sp -0x180` | `-` |
| `ldr` at 0xea428 | load | 4 | `sp -0x188` | `-` |
| `ldr` at 0xea434 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea438 | load | 4 | `[sp-28] +0xa4` | `-` |
| `ldr` at 0xea43c | load | 4 | `sp -0x18c` | `-` |
| `str` at 0xea44c | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xea450 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xea458 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xea460 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xea468 | store | 4 | `sp -0x24` | `?` |
| `ldrb` at 0xea478 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xea484 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xea490 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xea490 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xea490 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xea490 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xea08c | `(unknown callee) 0xed99c` | `[r1+160]:1` | `[r1+168]:1` | `[r1+164]` | `[r1+156]` |
| `bl` at 0xea098 | `(unknown callee) 0xea904` | `0x20` | `?` | `?` | `?` |
| `bl` at 0xea0e0 | `(unknown callee) 0xee13c` | `?` | `[[sp-28]+4]` | `[sp-28]+8` | `[[sp-28]+152]` |
| `bl` at 0xea11c | `(unknown callee) 0xedcd0` | `[[sp-24]+4]:2` | `[[sp-28]]:2` | `sp-184` | `?` |
| `bl` at 0xea160 | `(unknown callee) 0xea49c` | `0x14` | `?` | `[[sp-28]+396]` | `?` |
| `bl` at 0xea184 | `(unknown callee) 0x10f724` | `[[sp-24]+12]` | `sp-424` | `sp-424` | `[sp-24]` |
| `bl` at 0xea1a0 | `(unknown callee) 0x10f764` | `[[sp-24]+12]` | `sp-424` | `sp-424` | `[sp-24]` |
| `bl` at 0xea1cc | `(unknown callee) 0x10f690` | `[[sp-24]+12]` | `sp-340` | `?` | `[sp-24]` |
| `bl` at 0xea1ec | `(unknown callee) 0xea49c` | `0x1` | `?` | `ret(0x10f690)` | `0xfffff698` |
| `bl` at 0xea228 | `(unknown callee) 0x10f764` | `[[sp-24]+12]` | `sp-424` | `sp-424` | `[sp-24]` |
| `bl` at 0xea27c | `(unknown callee) 0x10f4b8` | `[[sp-24]+12]` | `[sp-28]+240` | `0x98` | `[sp-28]+240` |
| `bl` at 0xea2d8 | `(unknown callee) 0xeddac` | `[sp-28]+240` | `?` | `sp-370` | `sp-352` |
| `bl` at 0xea2f8 | `(unknown callee) 0xedf20` | `[sp-352]` | `?` | `?` | `ret(0xeddac)` |
| `bl` at 0xea31c | `(unknown callee) 0xedfb8` | `[sp-352]` | `?` | `?` | `ret(0xedf20)` |
| `bl` at 0xea330 | `(unknown callee) 0xee03c` | `[sp-352]` | `?` | `sp-20` | `ret(0xedfb8)` |
| `bl` at 0xea340 | `(unknown callee) 0xee0a8` | `[sp-352]` | `?` | `?` | `ret(0xee03c)` |
| `bl` at 0xea3a0 | `(unknown callee) 0xee0e0` | `[sp-352]` | `?` | `?` | `?` |
| `bl` at 0xea3c0 | `(unknown callee) 0xedba4` | `[[sp-28]+392]` | `?` | `?` | `[sp-28]` |
| `bl` at 0xea3d4 | `(unknown callee) 0xedb6c` | `[[sp-28]+392]` | `?` | `?` | `[sp-28]` |
| `bl` at 0xea3e8 | `(unknown callee) 0xedb28` | `[[sp-28]+392]` | `?` | `?` | `[sp-28]` |
| `bl` at 0xea3fc | `(unknown callee) 0xedae4` | `[[sp-28]+392]` | `?` | `?` | `[sp-28]` |
| `bl` at 0xea480 | `(unknown callee) 0xea904` | `?` | `sp-20` | `[[sp-28]+396]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xea03c | `[r1+156] == 0xff` | `[r1+156] != 0xff` |
| `ble` at 0xea140 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xea14c | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xea158 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xea170 | `[[sp-28]+4] != 0xb` | `[[sp-28]+4] == 0xb` |
| `bgt` at 0xea1b4 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0xea1e4 | `ret(0x10f690) != 0xfffff698` | `ret(0x10f690) == 0xfffff698` |
| `bne` at 0xea20c | `[[sp-28]+4] != 0xb` | `[[sp-28]+4] == 0xb` |
| `bne` at 0xea234 | `Z == 0` | `not (Z == 0)` |
| `ble` at 0xea254 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xea260 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0xea294 | `ret(0x10f4b8) != 0xfffff69e` | `ret(0x10f4b8) == 0xfffff69e` |
| `bgt` at 0xea2a4 | `ret(0x10f4b8) > 0x0 (signed, 32-bit)` | `ret(0x10f4b8) <= 0x0 (signed, 32-bit)` |
| `bne` at 0xea2ec | `ret(0xeddac) != 0x0` | `ret(0xeddac) == 0x0` |
| `beq` at 0xea30c | `ret(0xedf20) == 0x1` | `ret(0xedf20) != 0x1` |
| `bne` at 0xea360 | `[sp-388] != 0x0` | `[sp-388] == 0x0` |
| `bne` at 0xea374 | `[[sp-28]+4] != [sp-384]` | `[[sp-28]+4] == [sp-384]` |
| `bne` at 0xea394 | `? != ?` | `? == ?` |
| `bne` at 0xea3b4 | `[[sp-28]+4] != 0x20` | `[[sp-28]+4] == 0x20` |
| `bne` at 0xea410 | `[sp-388] != 0x0` | `[sp-388] == 0x0` |
| `bne` at 0xea424 | `[[sp-28]+156] != [sp-384]` | `[[sp-28]+156] == [sp-384]` |
| `bne` at 0xea430 | `[sp-392] != 0x1` | `[sp-392] == 0x1` |
| `bne` at 0xea444 | `[[sp-28]+164] != ret(0xedae4)` | `[[sp-28]+164] == ret(0xedae4)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xea490 | `r0` | `[sp-32]` |

## `PBayPollHwStatus`

- `PBayPollHwStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeaa10 to 0xeb0a0.
  Established. Word stores initialize sp-0x4c to one and sp-0x50 to zero (`str` at 0xeaa28, `str` at 0xeaa30).
  The call to unnamed 0x28230 has its result stored at sp-0x18 (`bl` at 0xeaa38, `str` at 0xeaa3c).
  The increment at sp-0x2c loads its word, adds one and stores [sp-44]+1 (`ldr` at 0xeac04, `add` at 0xeac08, `str` at 0xeac0c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xeaa20 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xeaa28 | store | 4 | `sp -0x4c` | `0x1` |
| `str` at 0xeaa30 | store | 4 | `sp -0x50` | `0x0` |
| `str` at 0xeaa3c | store | 4 | `sp -0x18` | `ret(0x28230)` |
| `ldr` at 0xeaa40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeaa64 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xeaa94 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xeaa98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeaaa8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeaab0 | load | 4 | `[sp-20] +0x8` | `-` |
| `str` at 0xeaac0 | store | 4 | `sp -0x24` | `ret(0xed870)` |
| `ldr` at 0xeaac4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xeaae0 | store | 4 | `sp -0x20` | `?` |
| `str` at 0xeaae8 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xeaaf0 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xeaaf4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xeab04 | store | 4 | `sp -0xac` | `0x0` |
| `ldr` at 0xeab0c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xeab14 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xeab1c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xeab2c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xeab40 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeab44 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xeab4c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xeab50 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeab54 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeab64 | load | 4 | `abs 0xeb078` | `-` |
| `str` at 0xeab68 | store | 4 | `sp -0xac` | `0x184a50` |
| `ldr` at 0xeab6c | load | 4 | `abs 0xeb07c` | `-` |
| `str` at 0xeab70 | store | 4 | `sp -0x44` | `0xbbf` |
| `str` at 0xeab78 | store | 4 | `sp -0x48` | `0x7500` |
| `ldr` at 0xeab80 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeab84 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeab94 | load | 4 | `abs 0xeb080` | `-` |
| `str` at 0xeab98 | store | 4 | `sp -0xac` | `0x184a5c` |
| `str` at 0xeaba0 | store | 4 | `sp -0x44` | `0xbc0` |
| `str` at 0xeaba8 | store | 4 | `sp -0x48` | `0x7400` |
| `ldr` at 0xeabac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeabb8 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0xeabcc | load | 4 | `abs 0xeb084` | `-` |
| `ldr` at 0xeabd0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xeabd4 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0xeabe0 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xeabe4 | store | 4 | `sp -0xb4` | `[sp-76]` |
| `ldr` at 0xeabe8 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xeabec | store | 4 | `sp -0xb0` | `[sp-80]` |
| `ldr` at 0xeabf4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xeabf8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xeac04 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xeac0c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xeac14 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeac18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeac24 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xeac3c | store | 4 | `sp -0x58` | `0x3` |
| `str` at 0xeac44 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xeac48 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xeac54 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeac60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeac68 | load | 4 | `[sp-20] +0x8` | `-` |
| `str` at 0xeac78 | store | 4 | `sp -0x24` | `ret(0xed870)` |
| `ldr` at 0xeac7c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeac90 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xeaca0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xeacb0 | load | 4 | `abs 0xeb088` | `-` |
| `ldr` at 0xeacb4 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0xeacc0 | store | 1 | `[sp-44] +0x1b0ee8` | `0x0` |
| `str` at 0xeacc8 | store | 4 | `sp -0x58` | `0x3` |
| `ldr` at 0xeaccc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xeacd4 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xeace0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xeacf0 | load | 4 | `abs 0xeb088` | `-` |
| `ldr` at 0xeacf4 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xeacfc | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xead00 | load | 1 | `[sp-44] +0x1b0ee8` | `-` |
| `strb` at 0xead08 | store | 1 | `sp -0x39` | `?` |
| `ldrb` at 0xead0c | load | 1 | `sp -0x39` | `-` |
| `str` at 0xead1c | store | 4 | `sp -0x58` | `0x3` |
| `ldr` at 0xead20 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xead28 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xead34 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xead44 | load | 1 | `sp -0x39` | `-` |
| `ldrb` at 0xead48 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xead50 | store | 1 | `sp -0x39` | `?` |
| `ldrb` at 0xead54 | load | 1 | `sp -0x39` | `-` |
| `ldr` at 0xead64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xead78 | load | 4 | `sp -0x58` | `-` |
| `str` at 0xead84 | store | 4 | `sp -0x58` | `[sp-88]-1` |
| `str` at 0xead94 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xead9c | load | 4 | `abs 0xeb08c` | `-` |
| `str` at 0xeada0 | store | 4 | `sp -0xac` | `0x184a78` |
| `ldr` at 0xeada4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xeadb0 | store | 4 | `sp -0x44` | `[sp-44]+427` |
| `ldr` at 0xeadb4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xeadbc | store | 4 | `sp -0x48` | `[sp-44]+28672` |
| `ldrb` at 0xeadc4 | load | 1 | `sp -0x39` | `-` |
| `ldr` at 0xeadd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeade4 | load | 4 | `abs 0xeb090` | `-` |
| `str` at 0xeade8 | store | 4 | `sp -0xac` | `0x184a94` |
| `ldr` at 0xeadec | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xeadf8 | store | 4 | `sp -0x44` | `[sp-44]+433` |
| `ldr` at 0xeadfc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xeae04 | store | 4 | `sp -0x48` | `[sp-44]+28928` |
| `ldrb` at 0xeae0c | load | 1 | `sp -0x39` | `-` |
| `ldr` at 0xeae1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeae2c | load | 4 | `abs 0xeb094` | `-` |
| `str` at 0xeae30 | store | 4 | `sp -0xac` | `0x184aac` |
| `ldr` at 0xeae34 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xeae40 | store | 4 | `sp -0x44` | `[sp-44]+439` |
| `ldr` at 0xeae44 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xeae4c | store | 4 | `sp -0x48` | `[sp-44]+29184` |
| `str` at 0xeae58 | store | 4 | `sp -0xac` | `0x0` |
| `ldr` at 0xeae5c | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0xeae70 | load | 4 | `abs 0xeb084` | `-` |
| `ldr` at 0xeae74 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xeae78 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0xeae84 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xeae88 | store | 4 | `sp -0xb4` | `[sp-76]` |
| `ldr` at 0xeae8c | load | 4 | `sp -0x50` | `-` |
| `str` at 0xeae90 | store | 4 | `sp -0xb0` | `[sp-80]` |
| `ldr` at 0xeae98 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xeae9c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xeaea8 | load | 4 | `abs 0xeb088` | `-` |
| `ldr` at 0xeaeac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xeaeb8 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xeaec8 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xeaecc | store | 1 | `[sp-44] +0x1b0ee8` | `?` |
| `str` at 0xeaed4 | store | 4 | `sp -0x58` | `0x3` |
| `ldr` at 0xeaed8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xeaee0 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xeaee8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xeaefc | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xeaf00 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xeaf10 | store | 4 | `sp -0x58` | `0x2` |
| `str` at 0xeaf18 | store | 4 | `sp -0x54` | `0x1` |
| `ldr` at 0xeaf1c | load | 4 | `sp -0x54` | `-` |
| `str` at 0xeaf2c | store | 4 | `sp -0x54` | `0x0` |
| `ldr` at 0xeaf30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeaf34 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xeaf38 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xeaf44 | store | 4 | `sp -0x40` | `ret(0xeb39c)` |
| `ldr` at 0xeaf48 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xeaf54 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xeaf64 | load | 4 | `abs 0xeb098` | `-` |
| `ldr` at 0xeaf68 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xeaf70 | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0xeaf74 | load | 1 | `[sp-44] +0x1b0eef` | `-` |
| `strb` at 0xeaf84 | store | 1 | `sp -0x39` | `?` |
| `ldrb` at 0xeaf88 | load | 1 | `sp -0x39` | `-` |
| `ldr` at 0xeaf98 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xeafac | load | 4 | `sp -0x58` | `-` |
| `str` at 0xeafb8 | store | 4 | `sp -0x58` | `[sp-88]-1` |
| `str` at 0xeafc8 | store | 4 | `sp -0x54` | `0x1` |
| `ldr` at 0xeafe0 | load | 4 | `abs 0xeb09c` | `-` |
| `ldr` at 0xeafe4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xeafec | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xeaff8 | store | 4 | `sp -0x44` | `[sp-44]+445` |
| `ldr` at 0xeaffc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xeb004 | store | 4 | `sp -0x48` | `[sp-44]+29440` |
| `ldr` at 0xeb00c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xeb010 | store | 4 | `sp -0xb4` | `[sp-76]` |
| `ldr` at 0xeb014 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xeb018 | store | 4 | `sp -0xb0` | `[sp-80]` |
| `ldr` at 0xeb020 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xeb024 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xeb030 | load | 4 | `abs 0xeb098` | `-` |
| `ldr` at 0xeb034 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xeb03c | load | 4 | `sp -0x40` | `-` |
| `strb` at 0xeb040 | store | 1 | `[sp-44] +0x1b0eef` | `?` |
| `ldr` at 0xeb048 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xeb050 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xeb060 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xeb074 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xeb074 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xeb074 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeaa38 | `(unknown callee) 0x28230` | `0x55` | `r1` | `r2` | `0x0` |
| `bl` at 0xeaa50 | `(unknown callee) 0xe9130` | `ret(0x28230)` | `?` | `?` | `ret(0x28230)` |
| `bl` at 0xeaa74 | `(unknown callee) 0x22ca0` | `[sp-20]+176` | `0x0` | `?` | `[sp-20]+176` |
| `bl` at 0xeaa8c | `(unknown callee) 0xe9054` | `0x3` | `?` | `?` | `ret(0x22ca0)` |
| `bl` at 0xeaab8 | `(unknown callee) 0xed870` | `[[sp-20]+8]` | `sp-56` | `sp-56` | `[sp-20]` |
| `bl` at 0xeaad4 | `(unknown callee) 0x28230` | `0x69` | `?` | `?` | `ret(0xed870)` |
| `bl` at 0xeabd8 | `(unknown callee) 0x205bc` | `sp-168` | `0x184a64` | `[sp-44]` | `[sp-172]` |
| `bl` at 0xeac00 | `(unknown callee) 0xa9740` | `0x0` | `[sp-72]` | `[sp-68]` | `sp-168` |
| `bl` at 0xeac34 | `(unknown callee) 0x2624c` | `0x69` | `?` | `[sp-32]` | `?` |
| `bl` at 0xeac70 | `(unknown callee) 0xed870` | `[[sp-20]+8]` | `sp-56` | `sp-56` | `[sp-20]` |
| `bl` at 0xeae7c | `(unknown callee) 0x205bc` | `sp-168` | `0x184a64` | `[sp-44]` | `?` |
| `bl` at 0xeaea4 | `(unknown callee) 0xa9740` | `0x0` | `[sp-72]` | `[sp-68]` | `sp-168` |
| `bl` at 0xeaf3c | `(unknown callee) 0xeb39c` | `[[sp-20]+8]` | `?` | `?` | `[sp-20]` |
| `bl` at 0xeafd0 | `(unknown callee) 0xea49c` | `0x64` | `[sp-88]` | `[sp-88]-1` | `0x1` |
| `bl` at 0xeafe8 | `(unknown callee) 0x205bc` | `sp-168` | `0x184abc` | `[sp-44]` | `sp-168` |
| `bl` at 0xeb02c | `(unknown callee) 0xa9740` | `0x0` | `[sp-44]+29440` | `[sp-44]+445` | `sp-168` |
| `bl` at 0xeb05c | `(unknown callee) 0xe9054` | `0x2` | `?` | `?` | `?` |
| `bl` at 0xeb06c | `(unknown callee) 0x2302c` | `[sp-20]+176` | `?` | `?` | `[sp-20]+176` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xeaa48 | `ret(0x28230) != 0x0` | `ret(0x28230) == 0x0` |
| `beq` at 0xeaa5c | `ret(0xe9130) == 0x2` | `ret(0xe9130) != 0x2` |
| `beq` at 0xeaa80 | `ret(0x22ca0) == 0x0` | `ret(0x22ca0) != 0x0` |
| `beq` at 0xeaaa4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xeaacc | `ret(0xed870) == 0x0` | `ret(0xed870) != 0x0` |
| `bgt` at 0xeaafc | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0xeab3c | `? == 0x0` | `? != 0x0` |
| `bne` at 0xeab60 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xeab90 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xeabb4 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `beq` at 0xeabc0 | `[sp-172] == 0x0` | `[sp-172] != 0x0` |
| `beq` at 0xeac20 | `Z == 1` | `not (Z == 1)` |
| `bgt` at 0xeac50 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0xeac5c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xeac84 | `ret(0xed870) != 0x0` | `ret(0xed870) == 0x0` |
| `beq` at 0xeacac | `? == 0x0` | `? != 0x0` |
| `bne` at 0xead14 | `[sp-57]:1 != 0x0` | `[sp-57]:1 == 0x0` |
| `beq` at 0xead60 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xead70 | `? == 0x0` | `? != 0x0` |
| `ble` at 0xead8c | `[sp-88] <= 0x0 (signed, 32-bit)` | `[sp-88] > 0x0 (signed, 32-bit)` |
| `beq` at 0xeadd0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xeade0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xeae18 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xeae28 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xeae64 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xeaef4 | `? == 0x0` | `? != 0x0` |
| `bgt` at 0xeaf08 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0xeaf24 | `Z == 1` | `not (Z == 1)` |
| `blt` at 0xeaf50 | `ret(0xeb39c) < 0x0 (signed, 32-bit)` | `ret(0xeb39c) >= 0x0 (signed, 32-bit)` |
| `bne` at 0xeaf60 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xeaf90 | `[sp-57]:1 != 0x0` | `[sp-57]:1 == 0x0` |
| `beq` at 0xeafa4 | `? == 0x0` | `? != 0x0` |
| `ble` at 0xeafc0 | `[sp-88] <= 0x0 (signed, 32-bit)` | `[sp-88] > 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xeb074 | `r0` | `?` |

## `PBaySdrGetDpsStateHdr`

- `PBaySdrGetDpsStateHdr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeb0a0 to 0xeb0ec.
  Established. Printed candidates for the call to unnamed 0x201fc include r0=0x50 (`bl` at 0xeb0b4);
  its result is stored as a word at sp-0x14 (`bl` at 0xeb0b4, `str` at 0xeb0bc).
  The return record is [sp-20], loaded before the return (`ldr` at 0xeb0d8, `ldm` at 0xeb0e0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xeb0bc | store | 4 | `sp -0x14` | `ret(0x201fc)` |
| `ldr` at 0xeb0c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb0c4 | load | 4 | `abs 0xeb0e4` | `-` |
| `ldr` at 0xeb0cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb0d0 | load | 4 | `abs 0xeb0e8` | `-` |
| `ldr` at 0xeb0d8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xeb0e0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xeb0e0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xeb0e0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xeb0e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeb0b4 | `(unknown callee) 0x201fc` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0xeb0c8 | `(unknown callee) 0x20634` | `ret(0x201fc)` | `0x184aec` | `?` | `ret(0x201fc)` |
| `bl` at 0xeb0d4 | `(unknown callee) 0x203c4` | `[sp-20]` | `0x184b0c` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xeb0e0 | `r0` | `[sp-20]` |

## `PBaySdrGetDpsState`

- `PBaySdrGetDpsState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeb0ec to 0xeb350.
  Established. The result from unnamed 0x201fc is stored at sp-0x28 and at the address held in [sp-28] (`bl` at 0xeb170, `str` at 0xeb178, `str` at 0xeb184).
  A byte loaded from the result of unnamed 0xedbdc plus two is stored as a byte at sp-0x2d (`bl` at 0xeb208, `ldrb` at 0xeb214, `strb` at 0xeb218).
  A word store writes zero at sp-0x1d0 (`str` at 0xeb30c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xeb0fc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xeb100 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xeb104 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xeb108 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xeb110 | store | 4 | `r2 +0x0` | `0x0` |
| `ldr` at 0xeb114 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb120 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xeb134 | store | 4 | `sp -0x1d0` | `0x0` |
| `ldr` at 0xeb13c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb144 | load | 4 | `abs 0xeb31c` | `-` |
| `str` at 0xeb14c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xeb150 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xeb154 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0xeb164 | store | 4 | `sp -0x1d0` | `0x0` |
| `str` at 0xeb178 | store | 4 | `sp -0x28` | `ret(0x201fc)` |
| `ldr` at 0xeb17c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeb180 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xeb184 | store | 4 | `[sp-28] +0x0` | `ret(0x201fc)` |
| `ldr` at 0xeb188 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeb18c | load | 4 | `abs 0xeb320` | `-` |
| `ldr` at 0xeb198 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeb1a0 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xeb1a8 | store | 1 | `[sp-40] +0x1` | `([sp-24]+48 & 0xff)` |
| `ldr` at 0xeb1b0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xeb1b4 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xeb1b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb1cc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xeb1dc | store | 4 | `sp -0x1cc` | `ret(0xea010)` |
| `ldr` at 0xeb1e0 | load | 4 | `sp -0x1cc` | `-` |
| `ldr` at 0xeb1ec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeb1f0 | load | 4 | `abs 0xeb324` | `-` |
| `str` at 0xeb1fc | store | 4 | `sp -0x1d0` | `?` |
| `ldr` at 0xeb204 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xeb20c | store | 4 | `sp -0x24` | `ret(0xedbdc)` |
| `ldr` at 0xeb210 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xeb214 | load | 1 | `ret(0xedbdc) +0x2` | `-` |
| `strb` at 0xeb218 | store | 1 | `sp -0x2d` | `([ret(0xedbdc)+2]:1 & 0xff)` |
| `ldrb` at 0xeb21c | load | 1 | `sp -0x2d` | `-` |
| `ldrb` at 0xeb22c | load | 1 | `sp -0x2d` | `-` |
| `ldr` at 0xeb23c | load | 4 | `abs 0xeb328` | `-` |
| `str` at 0xeb240 | store | 4 | `sp -0x2c` | `0x184b48` |
| `ldr` at 0xeb248 | load | 4 | `abs 0xeb32c` | `-` |
| `str` at 0xeb24c | store | 4 | `sp -0x2c` | `0x184b50` |
| `ldr` at 0xeb250 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeb254 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xeb25c | load | 1 | `sp -0x2d` | `-` |
| `ldr` at 0xeb26c | load | 4 | `abs 0xeb330` | `-` |
| `str` at 0xeb270 | store | 4 | `sp -0x2c` | `0x184b58` |
| `ldr` at 0xeb278 | load | 4 | `abs 0xeb334` | `-` |
| `str` at 0xeb27c | store | 4 | `sp -0x2c` | `0x184b64` |
| `ldr` at 0xeb280 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeb284 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xeb28c | load | 1 | `sp -0x2d` | `-` |
| `ldr` at 0xeb29c | load | 4 | `abs 0xeb338` | `-` |
| `str` at 0xeb2a0 | store | 4 | `sp -0x2c` | `0x184b70` |
| `ldr` at 0xeb2a8 | load | 4 | `abs 0xeb33c` | `-` |
| `str` at 0xeb2ac | store | 4 | `sp -0x2c` | `0x184b7c` |
| `ldr` at 0xeb2b0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeb2b4 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xeb2bc | load | 1 | `sp -0x2d` | `-` |
| `ldr` at 0xeb2cc | load | 4 | `abs 0xeb340` | `-` |
| `str` at 0xeb2d0 | store | 4 | `sp -0x2c` | `0x184b88` |
| `ldr` at 0xeb2d8 | load | 4 | `abs 0xeb344` | `-` |
| `str` at 0xeb2dc | store | 4 | `sp -0x2c` | `0x184b90` |
| `ldr` at 0xeb2e0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeb2e4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xeb2f0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeb2f4 | load | 4 | `abs 0xeb348` | `-` |
| `ldr` at 0xeb2fc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeb300 | load | 4 | `abs 0xeb34c` | `-` |
| `str` at 0xeb30c | store | 4 | `sp -0x1d0` | `0x0` |
| `ldr` at 0xeb310 | load | 4 | `sp -0x1d0` | `-` |
| `ldm` at 0xeb318 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xeb318 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xeb318 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeb170 | `(unknown callee) 0x201fc` | `0x50` | `r1` | `?` | `[[sp-32]]:1` |
| `bl` at 0xeb190 | `(unknown callee) 0x20634` | `[sp-40]` | `0x184b30` | `[sp-28]` | `ret(0x201fc)` |
| `bl` at 0xeb1c4 | `(unknown callee) 0xecd34` | `[sp-20]` | `sp-456` | `[[sp-32]]:1` | `[[sp-32]]:1` |
| `bl` at 0xeb1d4 | `(unknown callee) 0xea010` | `[sp-20]` | `sp-456` | `?` | `sp-456` |
| `bl` at 0xeb1f4 | `(unknown callee) 0x203c4` | `[sp-40]` | `0x184b38` | `?` | `ret(0xea010)` |
| `bl` at 0xeb208 | `(unknown callee) 0xedbdc` | `[sp-64]` | `?` | `?` | `ret(0xea010)` |
| `bl` at 0xeb258 | `(unknown callee) 0x203c4` | `[sp-40]` | `[sp-44]` | `?` | `?` |
| `bl` at 0xeb288 | `(unknown callee) 0x203c4` | `[sp-40]` | `[sp-44]` | `?` | `?` |
| `bl` at 0xeb2b8 | `(unknown callee) 0x203c4` | `[sp-40]` | `[sp-44]` | `?` | `?` |
| `bl` at 0xeb2e8 | `(unknown callee) 0x203c4` | `[sp-40]` | `[sp-44]` | `?` | `?` |
| `bl` at 0xeb2f8 | `(unknown callee) 0x203c4` | `[sp-40]` | `0x184b98` | `?` | `?` |
| `bl` at 0xeb304 | `(unknown callee) 0x203c4` | `[sp-40]` | `0x184ba4` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xeb11c | `[sp-24] <= 0x0 (signed, 32-bit)` | `[sp-24] > 0x0 (signed, 32-bit)` |
| `bgt` at 0xeb128 | `[sp-24] > 0x6 (signed, 32-bit)` | `[sp-24] <= 0x6 (signed, 32-bit)` |
| `bne` at 0xeb15c | `[[sp-32]]:1 != 0xff` | `[[sp-32]]:1 == 0xff` |
| `bne` at 0xeb1e8 | `ret(0xea010) != 0x0` | `ret(0xea010) == 0x0` |
| `bne` at 0xeb228 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xeb238 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xeb268 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xeb298 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xeb2c8 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xeb318 | `r0` | `?` |

## `PBaySdrGetDcPortStateHdr`

- `PBaySdrGetDcPortStateHdr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeb350 to 0xeb39c.
  Established. The result of the call to unnamed 0x201fc is stored at sp-0x14 (`bl` at 0xeb364, `str` at 0xeb36c).
  Printed candidates for the call to unnamed 0x20634 include r0=ret(0x201fc) and r1=0x184ba8 (`bl` at 0xeb364, `bl` at 0xeb378).
  The recorded return r0 is [sp-20] (`ldm` at 0xeb390).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xeb36c | store | 4 | `sp -0x14` | `ret(0x201fc)` |
| `ldr` at 0xeb370 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb374 | load | 4 | `abs 0xeb394` | `-` |
| `ldr` at 0xeb37c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb380 | load | 4 | `abs 0xeb398` | `-` |
| `ldr` at 0xeb388 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xeb390 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xeb390 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xeb390 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xeb390 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeb364 | `(unknown callee) 0x201fc` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0xeb378 | `(unknown callee) 0x20634` | `ret(0x201fc)` | `0x184ba8` | `?` | `ret(0x201fc)` |
| `bl` at 0xeb384 | `(unknown callee) 0x203c4` | `[sp-20]` | `0x184bbc` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xeb390 | `r0` | `[sp-20]` |

## `PBaySdrGetDcPort`

- `PBaySdrGetDcPort`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeb39c to 0xeb480.
  Established. A call to unnamed 0xea010 has its result stored as a word at sp-0x1bc (`bl` at 0xeb434, `str` at 0xeb43c).
  Unnamed 0xedbdc is called and its result recorded at sp-0x20 (`bl` at 0xeb45c, `str` at 0xeb460).
  A byte load from that result plus two is stored as a word at sp-0x1c0 (`bl` at 0xeb45c, `ldrb` at 0xeb468, `str` at 0xeb46c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xeb3ac | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xeb3b0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xeb3b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb3c0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xeb3d4 | store | 4 | `sp -0x1c0` | `?` |
| `ldr` at 0xeb3dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb3e4 | load | 4 | `abs 0xeb47c` | `-` |
| `str` at 0xeb3ec | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xeb3f0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xeb3f4 | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0xeb404 | store | 4 | `sp -0x1c0` | `?` |
| `ldr` at 0xeb410 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xeb414 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xeb418 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb42c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xeb43c | store | 4 | `sp -0x1bc` | `ret(0xea010)` |
| `ldr` at 0xeb440 | load | 4 | `sp -0x1bc` | `-` |
| `str` at 0xeb450 | store | 4 | `sp -0x1c0` | `?` |
| `ldr` at 0xeb458 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xeb460 | store | 4 | `sp -0x20` | `ret(0xedbdc)` |
| `ldr` at 0xeb464 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xeb468 | load | 1 | `ret(0xedbdc) +0x2` | `-` |
| `str` at 0xeb46c | store | 4 | `sp -0x1c0` | `[ret(0xedbdc)+2]:1` |
| `ldr` at 0xeb470 | load | 4 | `sp -0x1c0` | `-` |
| `ldm` at 0xeb478 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xeb478 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xeb478 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeb424 | `(unknown callee) 0xecd34` | `r0` | `sp-440` | `[[sp-28]]:1` | `[[sp-28]]:1` |
| `bl` at 0xeb434 | `(unknown callee) 0xea010` | `[sp-20]` | `sp-440` | `?` | `sp-440` |
| `bl` at 0xeb45c | `(unknown callee) 0xedbdc` | `[sp-48]` | `?` | `?` | `ret(0xea010)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xeb3bc | `r1 <= 0x0 (signed, 32-bit)` | `r1 > 0x0 (signed, 32-bit)` |
| `bgt` at 0xeb3c8 | `r1 > 0x8 (signed, 32-bit)` | `r1 <= 0x8 (signed, 32-bit)` |
| `bne` at 0xeb3fc | `[[sp-28]]:1 != 0xff` | `[[sp-28]]:1 == 0xff` |
| `bne` at 0xeb448 | `ret(0xea010) != 0x0` | `ret(0xea010) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xeb478 | `r0` | `[sp-448]` |

## `PBaySdrGetDcPortState`

- `PBaySdrGetDcPortState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeb480 to 0xeb5d0.
  Established. A word store writes zero to the address held in entry r2 (`str` at 0xeb4a4).
  The result from unnamed 0x201fc is stored at sp-0x28 and at the address held in [sp-28] (`bl` at 0xeb4d8, `str` at 0xeb4e0, `str` at 0xeb4ec).
  A byte store writes ([sp-24]+48 & 0xff) at the address formed by [sp-40] plus 3 (`strb` at 0xeb510).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xeb490 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xeb494 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xeb498 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xeb49c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xeb4a4 | store | 4 | `r2 +0x0` | `0x0` |
| `ldr` at 0xeb4a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb4ac | load | 4 | `sp -0x18` | `-` |
| `str` at 0xeb4b8 | store | 4 | `sp -0x24` | `ret(0xeb39c)` |
| `ldr` at 0xeb4bc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xeb4cc | store | 4 | `sp -0x1cc` | `0x0` |
| `str` at 0xeb4e0 | store | 4 | `sp -0x28` | `ret(0x201fc)` |
| `ldr` at 0xeb4e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeb4e8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xeb4ec | store | 4 | `[sp-28] +0x0` | `ret(0x201fc)` |
| `ldr` at 0xeb4f0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeb4f4 | load | 4 | `abs 0xeb5b4` | `-` |
| `ldr` at 0xeb500 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeb508 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xeb510 | store | 1 | `[sp-40] +0x3` | `([sp-24]+48 & 0xff)` |
| `ldr` at 0xeb514 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeb520 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeb524 | load | 4 | `abs 0xeb5b8` | `-` |
| `str` at 0xeb530 | store | 4 | `sp -0x1cc` | `?` |
| `ldr` at 0xeb538 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeb548 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeb54c | load | 4 | `abs 0xeb5bc` | `-` |
| `ldr` at 0xeb558 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeb55c | load | 4 | `abs 0xeb5c0` | `-` |
| `ldr` at 0xeb564 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeb574 | load | 4 | `abs 0xeb5c4` | `-` |
| `str` at 0xeb578 | store | 4 | `sp -0x1d0` | `0x184b88` |
| `ldr` at 0xeb580 | load | 4 | `abs 0xeb5c8` | `-` |
| `str` at 0xeb584 | store | 4 | `sp -0x1d0` | `0x184bf4` |
| `ldr` at 0xeb588 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeb58c | load | 4 | `sp -0x1d0` | `-` |
| `ldr` at 0xeb594 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeb598 | load | 4 | `abs 0xeb5cc` | `-` |
| `str` at 0xeb5a4 | store | 4 | `sp -0x1cc` | `0x0` |
| `ldr` at 0xeb5a8 | load | 4 | `sp -0x1cc` | `-` |
| `ldm` at 0xeb5b0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xeb5b0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xeb5b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeb4b0 | `(unknown callee) 0xeb39c` | `[sp-20]` | `[sp-24]` | `r2` | `0x0` |
| `bl` at 0xeb4d8 | `(unknown callee) 0x201fc` | `0x50` | `?` | `?` | `ret(0xeb39c)` |
| `bl` at 0xeb4f8 | `(unknown callee) 0x20634` | `[sp-40]` | `0x184bd8` | `[sp-28]` | `ret(0x201fc)` |
| `bl` at 0xeb528 | `(unknown callee) 0x203c4` | `[sp-40]` | `0x184b38` | `[sp-40]+3` | `[sp-36]` |
| `bl` at 0xeb550 | `(unknown callee) 0x203c4` | `[sp-40]` | `0x184be0` | `[sp-40]+3` | `?` |
| `bl` at 0xeb560 | `(unknown callee) 0x203c4` | `[sp-40]` | `0x184be8` | `[sp-40]+3` | `?` |
| `bl` at 0xeb590 | `(unknown callee) 0x203c4` | `[sp-40]` | `[sp-464]` | `?` | `?` |
| `bl` at 0xeb59c | `(unknown callee) 0x203c4` | `[sp-40]` | `0x184ba4` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xeb4c4 | `Z == 0` | `not (Z == 0)` |
| `bge` at 0xeb51c | `[sp-36] >= 0x0 (signed, 32-bit)` | `[sp-36] < 0x0 (signed, 32-bit)` |
| `beq` at 0xeb544 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xeb570 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xeb5b0 | `r0` | `?` |

## `PBaySdrGetMain`

- `PBaySdrGetMain`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeb5d0 to 0xeb7f4.
  Established. The result from unnamed 0x201fc is stored at sp-0x20 and at the address held in [sp-24] (`bl` at 0xeb644, `str` at 0xeb64c, `str` at 0xeb658).
  A byte store at the result of unnamed 0xedbdc plus three writes ([sp-500] & 0xff) (`bl` at 0xeb758, `strb` at 0xeb768).
  A load, add and store sequence writes [sp-496]+1 at sp-0x1f0 (`ldr` at 0xeb7b4, `add` at 0xeb7b8, `str` at 0xeb7bc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xeb5e0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xeb5e4 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xeb5e8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xeb5f0 | store | 4 | `r1 +0x0` | `0x0` |
| `ldr` at 0xeb5f4 | load | 4 | `abs 0xeb7d8` | `-` |
| `ldr` at 0xeb5f8 | load | 4 | `abs 0x1b0ef8` | `-` |
| `str` at 0xeb608 | store | 4 | `sp -0x200` | `0x0` |
| `ldr` at 0xeb614 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xeb624 | store | 4 | `sp -0x1f4` | `ret(0xed870)` |
| `ldr` at 0xeb628 | load | 4 | `sp -0x1f4` | `-` |
| `str` at 0xeb638 | store | 4 | `sp -0x200` | `0x0` |
| `str` at 0xeb64c | store | 4 | `sp -0x20` | `ret(0x201fc)` |
| `ldr` at 0xeb650 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb654 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xeb658 | store | 4 | `[sp-24] +0x0` | `ret(0x201fc)` |
| `ldr` at 0xeb65c | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xeb664 | store | 1 | `[sp-32] +0x0` | `0x0` |
| `str` at 0xeb66c | store | 4 | `sp -0x1f0` | `0x0` |
| `ldr` at 0xeb670 | load | 4 | `abs 0xeb7d8` | `-` |
| `ldr` at 0xeb674 | load | 4 | `sp -0x1f0` | `-` |
| `ldr` at 0xeb678 | load | 4 | `abs 0x1b0ef8` | `-` |
| `ldr` at 0xeb688 | load | 4 | `abs 0xeb7dc` | `-` |
| `ldr` at 0xeb68c | load | 4 | `sp -0x1f0` | `-` |
| `ldrb` at 0xeb6a4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xeb6a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb6b8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xeb6c8 | store | 4 | `sp -0x1ec` | `ret(0xea010)` |
| `ldr` at 0xeb6cc | load | 4 | `sp -0x1ec` | `-` |
| `ldr` at 0xeb6d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeb6dc | load | 4 | `abs 0xeb7e0` | `-` |
| `str` at 0xeb6e8 | store | 4 | `sp -0x200` | `?` |
| `ldr` at 0xeb6f0 | load | 4 | `sp -0x1f0` | `-` |
| `ldr` at 0xeb704 | load | 4 | `abs 0xeb7e4` | `-` |
| `str` at 0xeb718 | store | 4 | `sp -0x24` | `ret(0xed170)` |
| `ldr` at 0xeb71c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xeb720 | load | 1 | `ret(0xed170) +0x0` | `-` |
| `ldr` at 0xeb72c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeb730 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeb73c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeb740 | load | 4 | `abs 0xeb7e8` | `-` |
| `ldr` at 0xeb748 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeb74c | load | 4 | `abs 0xeb7ec` | `-` |
| `ldr` at 0xeb754 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xeb75c | store | 4 | `sp -0x1c` | `ret(0xedbdc)` |
| `ldr` at 0xeb760 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeb764 | load | 4 | `sp -0x1f4` | `-` |
| `strb` at 0xeb768 | store | 1 | `ret(0xedbdc) +0x3` | `([sp-500] & 0xff)` |
| `ldr` at 0xeb76c | load | 4 | `sp -0x1f0` | `-` |
| `ldr` at 0xeb780 | load | 4 | `abs 0xeb7e4` | `-` |
| `ldr` at 0xeb78c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeb79c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeb7a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeb7ac | load | 4 | `abs 0xeb7f0` | `-` |
| `ldr` at 0xeb7b4 | load | 4 | `sp -0x1f0` | `-` |
| `str` at 0xeb7bc | store | 4 | `sp -0x1f0` | `[sp-496]+1` |
| `str` at 0xeb7c8 | store | 4 | `sp -0x200` | `0x0` |
| `ldr` at 0xeb7cc | load | 4 | `sp -0x200` | `-` |
| `ldm` at 0xeb7d4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xeb7d4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xeb7d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeb61c | `(unknown callee) 0xed870` | `[sp-20]` | `sp-508` | `r1` | `sp-508` |
| `bl` at 0xeb644 | `(unknown callee) 0x201fc` | `0x100` | `?` | `?` | `ret(0xed870)` |
| `bl` at 0xeb6b0 | `(unknown callee) 0xecd34` | `[sp-20]` | `sp-488` | `?` | `?` |
| `bl` at 0xeb6c0 | `(unknown callee) 0xea010` | `[sp-20]` | `sp-488` | `?` | `sp-488` |
| `bl` at 0xeb6e0 | `(unknown callee) 0x203c4` | `[sp-32]` | `0x184b38` | `?` | `ret(0xea010)` |
| `bl` at 0xeb710 | `(unknown callee) 0xed170` | `?` | `?` | `?` | `?` |
| `bl` at 0xeb734 | `(unknown callee) 0x203c4` | `[sp-32]` | `ret(0xed170)` | `?` | `[ret(0xed170)]:1` |
| `bl` at 0xeb744 | `(unknown callee) 0x203c4` | `[sp-32]` | `0x184bf8` | `?` | `[ret(0xed170)]:1` |
| `bl` at 0xeb750 | `(unknown callee) 0x203c4` | `[sp-32]` | `0x184c08` | `?` | `?` |
| `bl` at 0xeb758 | `(unknown callee) 0xedbdc` | `[sp-96]` | `?` | `?` | `?` |
| `bl` at 0xeb794 | `(unknown callee) 0xed0c8` | `[sp-28]` | `?` | `sp-76` | `?` |
| `bl` at 0xeb7a4 | `(unknown callee) 0x203c4` | `[sp-32]` | `sp-76` | `?` | `sp-76` |
| `bl` at 0xeb7b0 | `(unknown callee) 0x203c4` | `[sp-32]` | `0x184ba4` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xeb600 | `[0x1b0ef8] != 0x0` | `[0x1b0ef8] == 0x0` |
| `bne` at 0xeb630 | `ret(0xed870) != 0x0` | `ret(0xed870) == 0x0` |
| `bge` at 0xeb680 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0xeb6d4 | `ret(0xea010) != 0x0` | `ret(0xea010) == 0x0` |
| `beq` at 0xeb728 | `[ret(0xed170)]:1 == 0x0` | `[ret(0xed170)]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xeb7d4 | `r0` | `?` |
