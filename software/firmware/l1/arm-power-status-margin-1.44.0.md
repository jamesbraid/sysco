# ARM power status and margin paths in 1.44.0

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
Coprocessor transfer widths are unknown. Two computed transfers remain unresolved
(`ldrls` at 0x399f8, `ldrls` at 0x37ccc). Their tables cover known edges only.
Unknown data values and instruction effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/power-status-margin-facts.txt) retain all tool sections.

## `powerNotPresent`

- `powerNotPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x36088 to 0x361c0.
  Established. A negative signed entry `r1` takes `0x360e8`, while nonnegative values fall through
  (`blt` at 0x360cc). The byte at `([[sp-28]+4] + r1) +0x0` receives zero on an included path
  (`strb` at 0x3619c). The unnamed `0x48da8` call at `0x360f8` carries entry `r0`, `0x0`, `0xf56b4`,
  `0xf56bc` (`bl` at 0x360f8). Absent-power state meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x36098 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3609c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x360a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x360b4 | load | 4 | `abs 0x361a8` | `-` |
| `ldr` at 0x360b8 | load | 4 | `abs 0x13219c` | `-` |
| `str` at 0x360c0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x360c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x360d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x360d4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x360d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x360e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x360f0 | load | 4 | `abs 0x361ac` | `-` |
| `ldr` at 0x360f4 | load | 4 | `abs 0x361b0` | `-` |
| `str` at 0x36130 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x36134 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3613c | load | 4 | `abs 0x361ac` | `-` |
| `ldr` at 0x36140 | load | 4 | `abs 0x361b0` | `-` |
| `ldr` at 0x3614c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3615c | load | 4 | `abs 0x361b4` | `-` |
| `str` at 0x36160 | store | 4 | `(unknown)` | `0xf5704` |
| `str` at 0x36164 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0x36168 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3616c | store | 4 | `(unknown)` | `[sp-24]` |
| `ldr` at 0x36174 | load | 4 | `abs 0x361b8` | `-` |
| `ldr` at 0x36178 | load | 4 | `abs 0x361bc` | `-` |
| `ldr` at 0x36188 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3618c | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x36190 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x3619c | store | 1 | `([[sp-28]+4] + r1) +0x0` | `0x0` |
| `ldm` at 0x361a4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x361a4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x361a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x360f8 | `(unknown callee) 0x48da8` | `r0` | `0x0` | `0xf56b4` | `0xf56bc` |
| `bl` at 0x36104 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x36144 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0xf56b4` | `0xf56bc` |
| `bl` at 0x36154 | `(unknown callee) 0x19808` | `[sp-32]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x36180 | `(unknown callee) 0x200a0` | `0x6` | `0xf565c` | `0x1ce` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x360cc | `r1 < 0x0 (signed, 32-bit)` | `r1 >= 0x0 (signed, 32-bit)` |
| `ble` at 0x360e0 | `[[sp-28]+8] <= r1 (signed, 32-bit)` | `[[sp-28]+8] > r1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x361a4 | `r0` | `?` |

## `powerPresent`

- `powerPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x361c0 to 0x362f8.
  Established. The signed condition `[[sp-28]+8] <= r1` takes `0x36220`, while larger values fall
  through (`ble` at 0x36218). An included byte assignment at `([[sp-28]+4] + r1) +0x0` writes `0x1`
  (`strb` at 0x362d4). The unnamed `0x200a0` call carries `0x6`, `0xf565c`, `0x1de`, `0x0` (`bl` at 0x362b8). Power presence policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x361d0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x361d4 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x361d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x361ec | load | 4 | `abs 0x362e0` | `-` |
| `ldr` at 0x361f0 | load | 4 | `abs 0x13219c` | `-` |
| `str` at 0x361f8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x361fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x36208 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3620c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x36210 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x36220 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x36228 | load | 4 | `abs 0x362e4` | `-` |
| `ldr` at 0x3622c | load | 4 | `abs 0x362e8` | `-` |
| `str` at 0x36268 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x3626c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x36274 | load | 4 | `abs 0x362e4` | `-` |
| `ldr` at 0x36278 | load | 4 | `abs 0x362e8` | `-` |
| `ldr` at 0x36284 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x36294 | load | 4 | `abs 0x362ec` | `-` |
| `str` at 0x36298 | store | 4 | `(unknown)` | `0xf5730` |
| `str` at 0x3629c | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0x362a0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x362a4 | store | 4 | `(unknown)` | `[sp-24]` |
| `ldr` at 0x362ac | load | 4 | `abs 0x362f0` | `-` |
| `ldr` at 0x362b0 | load | 4 | `abs 0x362f4` | `-` |
| `ldr` at 0x362c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x362c4 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x362c8 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x362d4 | store | 1 | `([[sp-28]+4] + r1) +0x0` | `0x1` |
| `ldm` at 0x362dc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x362dc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x362dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x36230 | `(unknown callee) 0x48da8` | `r0` | `0x0` | `0xf56b4` | `0xf56bc` |
| `bl` at 0x3623c | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x3627c | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0xf56b4` | `0xf56bc` |
| `bl` at 0x3628c | `(unknown callee) 0x19808` | `[sp-32]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x362b8 | `(unknown callee) 0x200a0` | `0x6` | `0xf565c` | `0x1de` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x36204 | `r1 < 0x0 (signed, 32-bit)` | `r1 >= 0x0 (signed, 32-bit)` |
| `ble` at 0x36218 | `[[sp-28]+8] <= r1 (signed, 32-bit)` | `[[sp-28]+8] > r1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x362dc | `r0` | `?` |

## `powerIsPresent`

- `powerIsPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x36528 to 0x365bc.
  Established. The absolute word at `0x13219c` is read (`ldr` at 0x36558). A byte at `([[sp-28]+4] + r1) +0x0` is read on an included path (`ldrb` at 0x365a4). The word at `sp -0x20` receives
  `[([[sp-28]+4] + r1)]:1` (`str` at 0x365a8). Presence classification remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x36538 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3653c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x36540 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x36554 | load | 4 | `abs 0x365b8` | `-` |
| `ldr` at 0x36558 | load | 4 | `abs 0x13219c` | `-` |
| `str` at 0x36560 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x36564 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x36570 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x36574 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x36578 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3658c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x36594 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x36598 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x3659c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x365a4 | load | 1 | `([[sp-28]+4] + r1) +0x0` | `-` |
| `str` at 0x365a8 | store | 4 | `sp -0x20` | `[([[sp-28]+4] + r1)]:1` |
| `ldr` at 0x365ac | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x365b4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x365b4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x365b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x3656c | `r1 < 0x0 (signed, 32-bit)` | `r1 >= 0x0 (signed, 32-bit)` |
| `ble` at 0x36580 | `[[sp-28]+8] <= r1 (signed, 32-bit)` | `[[sp-28]+8] > r1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x365b4 | `r0` | `?` |

## `powerGetName`

- `powerGetName`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x39b0c to 0x39c00.
  Established. A word through `[sp-28] +0x0` is read (`ldr` at 0x39b60). Included word reads at
  unknown targets remain explicit (`ldr` at 0x39b68, `ldr` at 0x39b94, `ldr` at 0x39be4). The return
  reports `[sp-32]` (`ldm` at 0x39bf4). Power name selection meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x39b1c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x39b20 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x39b24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x39b38 | load | 4 | `abs 0x39bf8` | `-` |
| `ldr` at 0x39b3c | load | 4 | `abs 0x13219c` | `-` |
| `str` at 0x39b44 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x39b48 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x39b4c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39b60 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x39b68 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x39b74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x39b78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39b8c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x39b94 | load | 4 | `(unknown)` | `-` |
| `str` at 0x39b98 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x39ba0 | load | 4 | `abs 0x39bfc` | `-` |
| `ldr` at 0x39ba4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x39ba8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39bbc | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x39bc4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x39be4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x39be8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x39bec | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x39bf4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x39bf4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x39bf4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x39b70 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x39bf4 | `r0` | `[sp-32]` |

## `powerGetReading`

- `powerGetReading`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x398cc to 0x39b0c.
  Established. The unnamed `0xc061c` call carries `[sp-36]`, `[ret(0x3a044)+4]:1`,
  `[ret(0x3a044)+5]:1`, `sp-54` (`bl` at 0x39948). A halfword through `[sp-44] +0x0` receives
  `([sp-54]:2 & 0xffff)` on an included path (`strh` at 0x39980). The coprocessor store through
  `[sp-48] +0x0` has unknown width and value (`stc` at 0x399c0). Reading conversion semantics remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `stc` at 0x398d4 | store | unknown | `sp -0x20` | `?` |
| `str` at 0x398e0 | store | 4 | `sp -0x24` | `r0` |
| `str` at 0x398e4 | store | 4 | `sp -0x28` | `r1` |
| `str` at 0x398e8 | store | 4 | `sp -0x2c` | `r2` |
| `str` at 0x398ec | store | 4 | `sp -0x30` | `r3` |
| `ldr` at 0x398f0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x398f4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x39900 | store | 4 | `sp -0x34` | `ret(0x3a044)` |
| `ldr` at 0x39904 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x39910 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3991c | store | 4 | `sp -0x40` | `ret(0x3a0e4)` |
| `ldr` at 0x39924 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x39928 | load | 1 | `ret(0x3a044) +0x4` | `-` |
| `ldr` at 0x3992c | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x39930 | load | 1 | `ret(0x3a044) +0x5` | `-` |
| `ldr` at 0x39938 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x39950 | store | 4 | `sp -0x3c` | `ret(0xc061c)` |
| `ldr` at 0x39954 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x39960 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x39964 | store | 4 | `sp -0x40` | `ret(0xc061c)` |
| `ldr` at 0x3996c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x39978 | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0x3997c | load | 2 | `sp -0x36` | `-` |
| `strh` at 0x39980 | store | 2 | `[sp-44] +0x0` | `([sp-54]:2 & 0xffff)` |
| `ldr` at 0x39984 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x39990 | load | 4 | `sp -0x30` | `-` |
| `ldrh` at 0x39994 | load | 2 | `sp -0x36` | `-` |
| `ldr` at 0x3999c | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x399a0 | load | 1 | `[sp-52] +0x4` | `-` |
| `ldr` at 0x399a4 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x399a8 | load | 1 | `[sp-52] +0x5` | `-` |
| `ldr` at 0x399ac | load | 4 | `sp -0x24` | `-` |
| `stc` at 0x399c0 | store | unknown | `[sp-48] +0x0` | `?` |
| `ldr` at 0x399c4 | load | 4 | `abs 0x39b08` | `-` |
| `ldr` at 0x399c8 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x399cc | load | 1 | `[sp-52] +0x0` | `-` |
| `ldrb` at 0x399ec | load | 1 | `(unknown)` | `-` |
| `ldrls` at 0x399f8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x39ad8 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x39adc | load | 4 | `sp -0x40` | `-` |
| `ldc` at 0x39ae0 | load | unknown | `sp -0x20` | `-` |
| `ldm` at 0x39ae8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x39ae8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x39ae8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x39ae8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x398f8 | `(unknown callee) 0x3a044` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x39914 | `(unknown callee) 0x3a0e4` | `[sp-36]` | `?` | `?` | `ret(0x3a044)` |
| `bl` at 0x39948 | `(unknown callee) 0xc061c` | `[sp-36]` | `[ret(0x3a044)+4]:1` | `[ret(0x3a044)+5]:1` | `sp-54` |
| `bl` at 0x399b8 | `(unknown callee) 0xc102c` | `[sp-36]` | `[[sp-52]+4]:1` | `[[sp-52]+5]:1` | `[[sp-52]+5]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3990c | `ret(0x3a044) != 0x0` | `ret(0x3a044) == 0x0` |
| `beq` at 0x3995c | `ret(0xc061c) == 0x0` | `ret(0xc061c) != 0x0` |
| `beq` at 0x39974 | `[sp-44] == 0x0` | `[sp-44] != 0x0` |
| `beq` at 0x3998c | `[sp-48] == 0x0` | `[sp-48] != 0x0` |
| `ldrls` at 0x399f8 | `? <= 0x4 (unsigned, 32-bit)` | `? > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x39ae8 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x399f8).

## `powerGetIndexOfConfig`

- `powerGetIndexOfConfig`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x39f8c to 0x3a044.
  Established. The absolute word at `0x13219c` is read (`ldr` at 0x39fbc). The word at `sp -0x1c`
  receives `[sp-28]+1` on an included path (`str` at 0x3a024). The return reports `[sp-36]` (`ldm` at 0x3a03c). Configuration index matching remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x39f9c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x39fa0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x39fa4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x39fb8 | load | 4 | `abs 0x3a040` | `-` |
| `ldr` at 0x39fbc | load | 4 | `abs 0x13219c` | `-` |
| `str` at 0x39fc4 | store | 4 | `sp -0x20` | `?` |
| `str` at 0x39fcc | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x39fd0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x39fd4 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0x39fd8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x39fe4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x39fe8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x39ffc | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x3a004 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3a010 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3a014 | store | 4 | `sp -0x24` | `[sp-28]` |
| `ldr` at 0x3a01c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3a024 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x3a030 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x3a034 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x3a03c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3a03c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3a03c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x39fe0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x3a00c | `? != [sp-24]` | `? == [sp-24]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3a03c | `r0` | `[sp-36]` |

## `powerGetMarginValue`

- `powerGetMarginValue`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3a8d8 to 0x3a914.
  Established. The byte through entry `r1 +0x10` is read (`ldrb` at 0x3a8f4). The unnamed `0x1d444`
  call carries `[r1+16]:1+130`, entry `r1`, entry `r2`, `[r1+16]:1+130` (`bl` at 0x3a900). The
  return reports `ret(0x1d444)` (`ldm` at 0x3a910). Margin value lookup semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3a8e8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3a8ec | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x3a8f0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3a8f4 | load | 1 | `r1 +0x10` | `-` |
| `ldm` at 0x3a910 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3a910 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3a910 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3a900 | `(unknown callee) 0x1d444` | `[r1+16]:1+130` | `r1` | `r2` | `[r1+16]:1+130` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3a910 | `r0` | `ret(0x1d444)` |

## `powerSetMarginValue`

- `powerSetMarginValue`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3a914 to 0x3a950.
  Established. The byte through entry `r1 +0x10` is read (`ldrb` at 0x3a934). The unnamed `0x1b5f0`
  call carries `[r1+16]:1+130`, entry `r2`, entry `r2`, `[r1+16]:1+130` (`bl` at 0x3a944); its
  result is reported by the return (`ldm` at 0x3a94c). Margin value assignment effects remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3a924 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3a928 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x3a92c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x3a930 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3a934 | load | 1 | `r1 +0x10` | `-` |
| `ldr` at 0x3a940 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x3a94c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3a94c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3a94c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3a944 | `(unknown callee) 0x1b5f0` | `[r1+16]:1+130` | `r2` | `r2` | `[r1+16]:1+130` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3a94c | `r0` | `ret(0x1b5f0)` |

## `powerGetMarginState`

- `powerGetMarginState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3a950 to 0x3a98c.
  Established. The unnamed `0x1d444` call carries `[r1+16]:1+20`, entry `r1`, entry `r2`,
  `[r1+16]:1+20` (`bl` at 0x3a978). The byte used at entry `r1 +0x10` is read (`ldrb` at 0x3a96c).
  The return reports `ret(0x1d444)` (`ldm` at 0x3a988). Margin state identifiers remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3a960 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3a964 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x3a968 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3a96c | load | 1 | `r1 +0x10` | `-` |
| `ldm` at 0x3a988 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3a988 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3a988 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3a978 | `(unknown callee) 0x1d444` | `[r1+16]:1+20` | `r1` | `r2` | `[r1+16]:1+20` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3a988 | `r0` | `ret(0x1d444)` |

## `powerSetMarginState`

- `powerSetMarginState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3a98c to 0x3a9c8.
  Established. The word at `sp -0x1c` receives entry `r2` (`str` at 0x3a9a4). The unnamed `0x1b5f0`
  call carries `[r1+16]:1+20`, entry `r2`, entry `r2`, `[r1+16]:1+20` (`bl` at 0x3a9bc). The return
  reports `ret(0x1b5f0)` (`ldm` at 0x3a9c4). Margin state update policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3a99c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3a9a0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x3a9a4 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x3a9a8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3a9ac | load | 1 | `r1 +0x10` | `-` |
| `ldr` at 0x3a9b8 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x3a9c4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3a9c4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3a9c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3a9bc | `(unknown callee) 0x1b5f0` | `[r1+16]:1+20` | `r2` | `r2` | `[r1+16]:1+20` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3a9c4 | `r0` | `ret(0x1b5f0)` |

## `powerGetMargin`

- `powerGetMargin`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x37c5c to 0x37fb4.
  Established. The unnamed `0x3a044` call carries entry `r0`, `r1`, `r2`, `r3` (`bl` at 0x37c80). A
  byte through `ret(0x3a044) +0x6` is read (`ldrb` at 0x37cb0); its equality to `0xff` takes
  `0x37f7c`, while inequality falls through (`beq` at 0x37cb8). Margin read interpretation remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x37c6c | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x37c70 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x37c74 | store | 4 | `sp -0x24` | `r2` |
| `ldr` at 0x37c78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x37c7c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x37c88 | store | 4 | `sp -0x28` | `ret(0x3a044)` |
| `ldr` at 0x37c8c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x37c98 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x37ca4 | store | 4 | `sp -0x34` | `ret(0x3a0e4)` |
| `ldr` at 0x37cac | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x37cb0 | load | 1 | `ret(0x3a044) +0x6` | `-` |
| `ldr` at 0x37cbc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x37cc0 | load | 4 | `ret(0x3a044) +0x8` | `-` |
| `ldrls` at 0x37ccc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x37ed8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x37ee0 | load | 4 | `abs 0x37f9c` | `-` |
| `ldr` at 0x37ee4 | load | 4 | `abs 0x37fa0` | `-` |
| `str` at 0x37f20 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x37f24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x37f2c | load | 4 | `abs 0x37f9c` | `-` |
| `ldr` at 0x37f30 | load | 4 | `abs 0x37fa0` | `-` |
| `ldr` at 0x37f3c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x37f4c | load | 4 | `abs 0x37fa4` | `-` |
| `str` at 0x37f50 | store | 4 | `(unknown)` | `0xf5940` |
| `str` at 0x37f54 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0x37f58 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x37f5c | load | 4 | `[sp-40] +0x8` | `-` |
| `str` at 0x37f60 | store | 4 | `(unknown)` | `[[sp-40]+8]` |
| `ldr` at 0x37f68 | load | 4 | `abs 0x37fa8` | `-` |
| `ldr` at 0x37f6c | load | 4 | `abs 0x37fac` | `-` |
| `ldr` at 0x37f7c | load | 4 | `abs 0x37fb0` | `-` |
| `str` at 0x37f80 | store | 4 | `sp -0x34` | `0xfffff9be` |
| `str` at 0x37f8c | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x37f90 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0x37f98 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x37f98 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x37f98 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x37f98 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x37f98 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x37c80 | `(unknown callee) 0x3a044` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x37c9c | `(unknown callee) 0x3a0e4` | `[sp-28]` | `?` | `?` | `ret(0x3a044)` |
| `bl` at 0x37ee8 | `(unknown callee) 0x48da8` | `[sp-28]` | `0x0` | `0xf56b4` | `0xf56bc` |
| `bl` at 0x37ef4 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x37f34 | `(unknown callee) 0x48da8` | `[sp-28]` | `0x0` | `0xf56b4` | `0xf56bc` |
| `bl` at 0x37f44 | `(unknown callee) 0x19808` | `[sp-48]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x37f74 | `(unknown callee) 0x200a0` | `0x9` | `0xf565c` | `0x5cf` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x37c94 | `ret(0x3a044) != 0x0` | `ret(0x3a044) == 0x0` |
| `beq` at 0x37cb8 | `[ret(0x3a044)+6]:1 == 0xff` | `[ret(0x3a044)+6]:1 != 0xff` |
| `ldrls` at 0x37ccc | `[ret(0x3a044)+8]-1 <= 0x4 (unsigned, 32-bit)` | `[ret(0x3a044)+8]-1 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x37f98 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x37ccc).

## `powerSetMargin`

- `powerSetMargin`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x37b4c to 0x37c5c.
  Established. A zero `ret(0x3a430)` takes `0x37c0c`, while nonzero falls through (`beq` at 0x37be8). Included absolute word assignments at `0x12d6f8` write `0x5` and `0x4` in separate paths
  (`str` at 0x37bf4, `str` at 0x37c14). The unnamed `0x3a914` call carries `[sp-20]`, `[sp-32]`,
  `[sp-25]:1`, `[sp-25]:1` (`bl` at 0x37c40). Margin application effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x37b5c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x37b60 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x37b68 | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `ldr` at 0x37b6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x37b70 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x37b88 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x37b90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x37b94 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x37ba0 | store | 4 | `sp -0x20` | `ret(0x3a044)` |
| `ldr` at 0x37ba4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37bb0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x37bbc | store | 4 | `sp -0x28` | `ret(0x3a0e4)` |
| `ldrb` at 0x37bc4 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x37bc8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x37bcc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x37bdc | store | 4 | `sp -0x24` | `ret(0x3a430)` |
| `ldr` at 0x37be0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x37bec | load | 4 | `abs 0x37c58` | `-` |
| `str` at 0x37bf4 | store | 4 | `abs 0x12d6f8` | `0x5` |
| `ldr` at 0x37c00 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x37c04 | store | 4 | `sp -0x28` | `[sp-36]` |
| `ldr` at 0x37c0c | load | 4 | `abs 0x37c58` | `-` |
| `str` at 0x37c14 | store | 4 | `abs 0x12d6f8` | `0x4` |
| `ldr` at 0x37c20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x37c24 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x37c30 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x37c34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x37c38 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x37c48 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x37c4c | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x37c54 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x37c54 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x37c54 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x37b74 | `(unknown callee) 0x36528` | `r0` | `r1` | `r2` | `r2` |
| `bl` at 0x37b98 | `(unknown callee) 0x3a044` | `[sp-20]` | `[sp-24]` | `?` | `ret(0x36528)` |
| `bl` at 0x37bb4 | `(unknown callee) 0x3a0e4` | `[sp-20]` | `?` | `?` | `ret(0x3a044)` |
| `bl` at 0x37bd4 | `(unknown callee) 0x3a430` | `[sp-20]` | `ret(0x3a044)` | `[sp-25]:1` | `[sp-25]:1` |
| `bl` at 0x37bfc | `(unknown callee) 0x6a7b0` | `0x5` | `?` | `0x5` | `0x12d6f8` |
| `bl` at 0x37c1c | `(unknown callee) 0x6a7b0` | `0x4` | `?` | `0x4` | `0x12d6f8` |
| `bl` at 0x37c2c | `(unknown callee) 0x3a98c` | `[sp-20]` | `[sp-32]` | `0x4` | `?` |
| `bl` at 0x37c40 | `(unknown callee) 0x3a914` | `[sp-20]` | `[sp-32]` | `[sp-25]:1` | `[sp-25]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x37b80 | `ret(0x36528) != 0x0` | `ret(0x36528) == 0x0` |
| `bne` at 0x37bac | `ret(0x3a044) != 0x0` | `ret(0x3a044) == 0x0` |
| `beq` at 0x37be8 | `ret(0x3a430) == 0x0` | `ret(0x3a430) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x37c54 | `r0` | `?` |

## `powerMarginStatusAll`

- `powerMarginStatusAll`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x375c4 to 0x375e4.
  Established. The absolute word at `0x12d6f8` is read (`ldr` at 0x375d4). The return reports
  `[0x12d6f8]` (`ldm` at 0x375dc). Aggregate margin status meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x375d0 | load | 4 | `abs 0x375e0` | `-` |
| `ldr` at 0x375d4 | load | 4 | `abs 0x12d6f8` | `-` |
| `ldm` at 0x375dc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x375dc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x375dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x375dc | `r0` | `[0x12d6f8]` |

## `powerStatusAll`

- `powerStatusAll`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x375e4 to 0x37674.
  Established. The unnamed `0x37674` call at `0x37600` carries `0x0`, entry `r1`, entry `r2`, `0x0`
  (`bl` at 0x37600). The absolute word at `0x1321a0` is read (`ldr` at 0x3761c). Equality of
  `ret(0x37674)` and `[sp-24]` takes `0x3764c`, while inequality falls through (`beq` at 0x3763c).
  Combined power status policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x375f8 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x37608 | store | 4 | `sp -0x18` | `ret(0x37674)` |
| `str` at 0x37610 | store | 4 | `sp -0x14` | `0x1` |
| `ldr` at 0x37614 | load | 4 | `abs 0x37670` | `-` |
| `ldr` at 0x37618 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3761c | load | 4 | `abs 0x1321a0` | `-` |
| `ldr` at 0x37628 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x37634 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x37644 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x3764c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x37654 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0x3765c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x37660 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x37664 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x3766c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3766c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3766c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x37600 | `(unknown callee) 0x37674` | `0x0` | `r1` | `r2` | `0x0` |
| `bl` at 0x3762c | `(unknown callee) 0x37674` | `?` | `?` | `?` | `[0x1321a0]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x37624 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x3763c | `ret(0x37674) == [sp-24]` | `ret(0x37674) != [sp-24]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3766c | `r0` | `?` |

## `powerStatus`

- `powerStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3783c to 0x37960.
  Established. A nonzero `ret(0x36528)` takes `0x37878`, while zero falls through (`bne` at 0x37868). The word at `sp -0x24` receives `0xfffff9b9` on an included path (`str` at 0x37870). The
  unnamed `0xca7a0` call carries `[sp-20]`, `[[sp-28]+2]:1`, unknown, `[[sp-28]+2]:1` (`bl` at 0x378e4). Status polarity remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3784c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x37850 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x37854 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x37858 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3786c | load | 4 | `abs 0x37954` | `-` |
| `str` at 0x37870 | store | 4 | `sp -0x24` | `0xfffff9b9` |
| `ldr` at 0x37878 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3788c | load | 4 | `abs 0x37958` | `-` |
| `ldr` at 0x37890 | load | 4 | `abs 0x13219c` | `-` |
| `ldr` at 0x37898 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x378ac | load | 4 | `(unknown)` | `-` |
| `str` at 0x378b4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x378b8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x378bc | load | 1 | `[sp-28] +0x2` | `-` |
| `ldr` at 0x378c8 | load | 4 | `abs 0x3795c` | `-` |
| `str` at 0x378cc | store | 4 | `sp -0x24` | `0xfffff9b8` |
| `ldr` at 0x378d4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x378d8 | load | 1 | `[sp-28] +0x2` | `-` |
| `ldr` at 0x378dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x378ec | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x378f0 | load | 1 | `[sp-28] +0x3` | `-` |
| `str` at 0x378fc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x37900 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x37914 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x37924 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x3792c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3793c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x37940 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x37944 | store | 4 | `sp -0x24` | `[sp-40]` |
| `ldr` at 0x37948 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x37950 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x37950 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x37950 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3785c | `(unknown callee) 0x36528` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x378e4 | `(unknown callee) 0xca7a0` | `[sp-20]` | `[[sp-28]+2]:1` | `?` | `[[sp-28]+2]:1` |
| `bl` at 0x37904 | `(unknown callee) 0x3a150` | `[sp-28]` | `?` | `ret(0xca7a0)` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x37868 | `ret(0x36528) != 0x0` | `ret(0x36528) == 0x0` |
| `bne` at 0x378c4 | `[[sp-28]+2]:1 != 0xff` | `[[sp-28]+2]:1 == 0xff` |
| `beq` at 0x37910 | `ret(0x3a150) == 0x0` | `ret(0x3a150) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x37950 | `r0` | `?` |
