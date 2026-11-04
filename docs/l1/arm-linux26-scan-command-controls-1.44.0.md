# ARM Linux 2.6 scan-command controls in 1.44.0

Read from `6.5.30/usr/cpu/firmware/sysco/l1.bin`, package `6.5.30:l1#6` (version 1.44.0),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0xa936a8`.
Carrier SHA-256: `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
Package SHA-256: `e57ab8ecd48858e39ecdd0050a23e444a491c35e87f9d8ebb48881fc25229a06`.
Inner executable SHA-256: `b33154e53b00531623db6ff365e80b1c1825bc9427961c59fdc132c24f0c819e`.
The ELF identifies 32-bit little-endian ARM. Its executable PT_LOAD segment begins at
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
No unresolved computed transfers were reported for these routines.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-scan-command-controls-facts.txt) retain all tool sections.

## `memory_access_function`

- `memory_access_function`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13648c to 0x136758.
  Established. The result from unnamed 0x13e5b0 is stored as a word at entry sp minus 0x18 (`bl` at 0x1365a8, `str` at 0x1365b0).
  A separate call to unnamed 0x84418 has its result stored as a word at entry sp minus 0x1c (`bl` at 0x1366d8, `str` at 0x1366e0).
  A word store writes zero at the address formed by [sp-20] plus 0xc (`str` at 0x13671c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x13649c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x1364a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1364ac | load | 4 | `r0 +0xc` | `-` |
| `ldr` at 0x1364c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1364c8 | load | 4 | `r0 +0xc` | `-` |
| `str` at 0x1364d0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x1364d4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x1364dc | store | 4 | `sp -0x38` | `[sp-56]-2` |
| `ldr` at 0x1364e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1364f0 | load | 4 | `r0 +0xc` | `-` |
| `str` at 0x1364f8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x1364fc | load | 4 | `sp -0x38` | `-` |
| `str` at 0x136504 | store | 4 | `sp -0x38` | `[sp-56]-1` |
| `ldr` at 0x136508 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x13650c | store | 4 | `sp -0x20` | `[sp-56]` |
| `ldr` at 0x136510 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136514 | load | 4 | `r0 +0x0` | `-` |
| `str` at 0x136520 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x136524 | load | 4 | `abs 0x136734` | `-` |
| `ldr` at 0x136528 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x136538 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136540 | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0x136550 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136558 | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0x136564 | load | 4 | `abs 0x136738` | `-` |
| `ldr` at 0x136574 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x13657c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x13658c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136594 | load | 4 | `(unknown)` | `-` |
| `str` at 0x1365b0 | store | 4 | `sp -0x18` | `ret(0x13e5b0)` |
| `ldr` at 0x1365b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1365c0 | load | 4 | `abs 0x13673c` | `-` |
| `str` at 0x1365cc | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x1365d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1365e0 | load | 4 | `abs 0x136740` | `-` |
| `str` at 0x1365ec | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x1365f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x136600 | load | 4 | `abs 0x136744` | `-` |
| `str` at 0x13660c | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x136614 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x136624 | load | 4 | `abs 0x136748` | `-` |
| `str` at 0x136630 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x13663c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136644 | load | 4 | `[sp-20] +0x10` | `-` |
| `str` at 0x136648 | store | 4 | `sp -0x28` | `[[sp-20]+16]` |
| `ldr` at 0x136650 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136658 | load | 4 | `[sp-20] +0x14` | `-` |
| `str` at 0x13665c | store | 4 | `sp -0x2c` | `[[sp-20]+20]` |
| `ldr` at 0x136660 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x136668 | store | 4 | `sp -0x30` | `[sp-20]+24` |
| `ldr` at 0x13666c | load | 4 | `abs 0x136734` | `-` |
| `ldr` at 0x136670 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x13667c | load | 4 | `abs 0x13674c` | `-` |
| `ldr` at 0x136680 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x136684 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x136688 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x136694 | store | 4 | `sp -0x4c` | `0xff` |
| `ldr` at 0x136698 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x13669c | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x1366a0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x1366a4 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x1366a8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x1366bc | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x1366c0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x1366c4 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x1366cc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x1366e0 | store | 4 | `sp -0x1c` | `ret(0x84418)` |
| `ldr` at 0x1366e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1366f0 | load | 4 | `abs 0x136750` | `-` |
| `ldr` at 0x1366f4 | load | 4 | `abs 0x136754` | `-` |
| `ldr` at 0x1366f8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x136704 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x136710 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x13671c | store | 4 | `[sp-20] +0xc` | `0x0` |
| `str` at 0x136724 | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0x136728 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0x136730 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x136730 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x136730 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x13656c | `(unknown callee) 0x243d4` | `0x190e84` | `?` | `?` | `?` |
| `bl` at 0x1365a8 | `(unknown callee) 0x13e5b0` | `?` | `?` | `0x4` | `?` |
| `bl` at 0x1365c4 | `(unknown callee) 0x29908` | `0x190eac` | `?` | `?` | `ret(0x13e5b0)` |
| `bl` at 0x1365e4 | `(unknown callee) 0x29908` | `0x190ed8` | `?` | `?` | `[sp-36]` |
| `bl` at 0x136604 | `(unknown callee) 0x29908` | `0x190ef0` | `?` | `?` | `[sp-32]` |
| `bl` at 0x136628 | `(unknown callee) 0x29908` | `0x190f0c` | `?` | `?` | `?` |
| `bl` at 0x13668c | `(unknown callee) 0x243d4` | `0x190f2c` | `[sp-24]` | `[[sp-20]+16]` | `[[sp-20]+20]` |
| `bl` at 0x1366d8 | `(unknown callee) 0x84418` | `0x1` | `[sp-24]` | `?` | `?` |
| `bl` at 0x1366fc | `(unknown callee) 0x205bc` | `0x31e4ec` | `0x190f5c` | `ret(0x84418)` | `ret(0x84418)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x1364b8 | `? != 0x0` | `? == 0x0` |
| `bls` at 0x136530 | `[0x315034] <= 0x1 (unsigned, 32-bit)` | `[0x315034] > 0x1 (unsigned, 32-bit)` |
| `bne` at 0x1365bc | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x1365dc | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `bne` at 0x1365fc | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x136620 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x136678 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `beq` at 0x1366ec | `ret(0x84418) == 0x0` | `ret(0x84418) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x136730 | `r0` | `?` |

## `smpcmd_scan_reset`

- `smpcmd_scan_reset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x139e00 to 0x139f44.
  Established. The call to unnamed 0x201fc has its result stored as a word at the address held in entry r1 (`bl` at 0x139e20, `str` at 0x139e28).
  Printed r0-r3 candidates for one call to unnamed 0x12cc8c are 0, [sp-32], [[sp-28]] and [sp-28] (`bl` at 0x139e94).
  A separate call to that address has candidates 1, [sp-32], [[sp-28]] and [sp-28] (`bl` at 0x139ee8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x139e10 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x139e14 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x139e18 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x139e28 | store | 4 | `r1 +0x0` | `ret(0x201fc)` |
| `ldr` at 0x139e2c | load | 4 | `abs 0x139f38` | `-` |
| `ldr` at 0x139e30 | load | 4 | `abs 0x315030` | `-` |
| `ldr` at 0x139e3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x139e40 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x139e44 | load | 4 | `abs 0x139f3c` | `-` |
| `str` at 0x139e50 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x139e68 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x139e6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x139e70 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0x139e74 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0x139e84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x139e8c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x139e90 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x139ea4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x139ea8 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x139eac | load | 4 | `abs 0x139f3c` | `-` |
| `str` at 0x139eb8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x139ec0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x139ec4 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0x139ec8 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0x139ed8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x139ee0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x139ee4 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x139ef8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x139efc | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x139f00 | load | 4 | `abs 0x139f3c` | `-` |
| `str` at 0x139f0c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x139f14 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x139f18 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x139f1c | load | 4 | `abs 0x139f40` | `-` |
| `str` at 0x139f28 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x139f2c | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x139f34 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x139f34 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x139f34 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x139f34 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x139e20 | `(unknown callee) 0x201fc` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0x139e48 | `(unknown callee) 0x203c4` | `[[sp-28]]` | `0x191428` | `?` | `[sp-28]` |
| `bl` at 0x139e58 | `(unknown callee) 0x137100` | `ret(0x201fc)` | `?` | `?` | `[0x315030]` |
| `bl` at 0x139e94 | `(unknown callee) 0x12cc8c` | `0x0` | `[sp-32]` | `[[sp-28]]` | `[sp-28]` |
| `bl` at 0x139eb0 | `(unknown callee) 0x203c4` | `[[sp-28]]` | `0x191428` | `?` | `[sp-28]` |
| `bl` at 0x139ee8 | `(unknown callee) 0x12cc8c` | `0x1` | `[sp-32]` | `[[sp-28]]` | `[sp-28]` |
| `bl` at 0x139f04 | `(unknown callee) 0x203c4` | `[[sp-28]]` | `0x191428` | `?` | `[sp-28]` |
| `bl` at 0x139f20 | `(unknown callee) 0x205bc` | `[[sp-28]]` | `0x19142c` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x139e38 | `[0x315030] != 0x0` | `[0x315030] == 0x0` |
| `beq` at 0x139e80 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x139ea0 | `ret(0x12cc8c) != 0x0` | `ret(0x12cc8c) == 0x0` |
| `beq` at 0x139ed4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x139ef4 | `ret(0x12cc8c) != 0x0` | `ret(0x12cc8c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x139f34 | `r0` | `0x0` |

## `smpcmd_scan_sel`

- `smpcmd_scan_sel`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x139f44 to 0x13a12c.
  Established. A word store writes the result from unnamed 0x201fc at the address held in entry r1 (`bl` at 0x139f64, `str` at 0x139f6c).
  Other word stores place 0x58 at entry sp minus 0x20 and zero at entry sp minus 0x30 and 0x34 (`str` at 0x139fa0, `str` at 0x13a014, `str` at 0x13a01c).
  Printed r0-r3 candidates for the call to unnamed 0x12d84c are unknown, [[sp-24]+40], [[sp-24]+44] and 0 (`bl` at 0x13a0bc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x139f54 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x139f58 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x139f5c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x139f6c | store | 4 | `r1 +0x0` | `ret(0x201fc)` |
| `ldr` at 0x139f70 | load | 4 | `abs 0x13a114` | `-` |
| `ldr` at 0x139f74 | load | 4 | `abs 0x315030` | `-` |
| `ldr` at 0x139f80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x139f84 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x139f88 | load | 4 | `abs 0x13a118` | `-` |
| `str` at 0x139f94 | store | 4 | `sp -0xd8` | `0x0` |
| `str` at 0x139fa0 | store | 4 | `sp -0x20` | `0x58` |
| `ldr` at 0x139fa4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x139fa8 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0x139fac | store | 4 | `sp -0x24` | `[[sp-24]+40]` |
| `ldr` at 0x139fb0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x139fbc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x139fc0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x139fc4 | load | 4 | `abs 0x13a11c` | `-` |
| `str` at 0x139fd0 | store | 4 | `sp -0xd8` | `0x0` |
| `ldr` at 0x139fd8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x139fdc | load | 4 | `[sp-24] +0x2c` | `-` |
| `str` at 0x139fe0 | store | 4 | `sp -0x2c` | `[[sp-24]+44]` |
| `ldr` at 0x139fe4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x139fe8 | load | 4 | `abs 0x13a120` | `-` |
| `ldr` at 0x139ff4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x139ff8 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x139ffc | load | 4 | `abs 0x13a124` | `-` |
| `str` at 0x13a008 | store | 4 | `sp -0xd8` | `0x0` |
| `str` at 0x13a014 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0x13a01c | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x13a020 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x13a024 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0x13a028 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0x13a038 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x13a040 | store | 4 | `sp -0x20` | `0x5a` |
| `ldr` at 0x13a044 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x13a048 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x13a04c | store | 4 | `sp -0x30` | `[[sp-24]+48]` |
| `ldr` at 0x13a050 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x13a054 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0x13a058 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0x13a068 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x13a070 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x13a074 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x13a078 | load | 4 | `[sp-24] +0x34` | `-` |
| `str` at 0x13a07c | store | 4 | `sp -0x34` | `[[sp-24]+52]` |
| `ldr` at 0x13a080 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x13a084 | store | 4 | `sp -0xec` | `?` |
| `ldr` at 0x13a088 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x13a08c | store | 4 | `sp -0xe8` | `?` |
| `str` at 0x13a094 | store | 4 | `sp -0xe4` | `sp-56` |
| `str` at 0x13a09c | store | 4 | `sp -0xe0` | `sp-40` |
| `ldr` at 0x13a0a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x13a0a4 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x13a0a8 | store | 4 | `sp -0xdc` | `[[sp-28]]` |
| `ldr` at 0x13a0ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x13a0b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x13a0b4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x13a0cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x13a0d0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x13a0d4 | load | 4 | `abs 0x13a118` | `-` |
| `str` at 0x13a0e0 | store | 4 | `sp -0xd8` | `0x0` |
| `ldr` at 0x13a0e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x13a0ec | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x13a0f0 | load | 4 | `abs 0x13a128` | `-` |
| `ldr` at 0x13a0f4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x13a0f8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x13a104 | store | 4 | `sp -0xd8` | `0x0` |
| `ldr` at 0x13a108 | load | 4 | `sp -0xd8` | `-` |
| `ldm` at 0x13a110 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x13a110 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x13a110 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x13a110 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x139f64 | `(unknown callee) 0x201fc` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0x139f8c | `(unknown callee) 0x203c4` | `[[sp-28]]` | `0x191428` | `?` | `[sp-28]` |
| `bl` at 0x139fc8 | `(unknown callee) 0x205bc` | `[[sp-28]]` | `0x191450` | `?` | `[sp-28]` |
| `bl` at 0x13a000 | `(unknown callee) 0x205bc` | `[[sp-28]]` | `0x191484` | `[[sp-24]+44]` | `[sp-28]` |
| `bl` at 0x13a0bc | `(unknown callee) 0x12d84c` | `?` | `[[sp-24]+40]` | `[[sp-24]+44]` | `0x0` |
| `bl` at 0x13a0d8 | `(unknown callee) 0x203c4` | `[[sp-28]]` | `0x191428` | `?` | `[sp-28]` |
| `bl` at 0x13a0fc | `(unknown callee) 0x205bc` | `[[sp-28]]` | `0x1914ac` | `[sp-56]` | `[sp-40]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x139f7c | `[0x315030] != 0x0` | `[0x315030] == 0x0` |
| `bls` at 0x139fb8 | `[[sp-24]+40] <= 0xff (unsigned, 32-bit)` | `[[sp-24]+40] > 0xff (unsigned, 32-bit)` |
| `bls` at 0x139ff0 | `[[sp-24]+44] <= 0xffff (unsigned, 32-bit)` | `[[sp-24]+44] > 0xffff (unsigned, 32-bit)` |
| `beq` at 0x13a034 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x13a064 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x13a0c8 | `ret(0x12d84c) != 0x0` | `ret(0x12d84c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x13a110 | `r0` | `0x0` |

## `smpcmd_scan_config`

- `smpcmd_scan_config`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13a12c to 0x13a1c4.
  Established. The result from unnamed 0x201fc is stored as a word at the address held in entry r1 (`bl` at 0x13a14c, `str` at 0x13a154).
  The unknown-target call has printed r0-r3 candidates ret(0x201fc), unknown, unknown and [0x1a0878] (`bl` at 0x13a14c, `mov` at 0x13a190).
  Two word stores write zero at entry sp minus 0x24 (`str` at 0x13a17c, `str` at 0x13a1a8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x13a13c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x13a140 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x13a144 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x13a154 | store | 4 | `r1 +0x0` | `ret(0x201fc)` |
| `ldr` at 0x13a158 | load | 4 | `abs 0x13a1b8` | `-` |
| `ldr` at 0x13a15c | load | 4 | `abs 0x315030` | `-` |
| `ldr` at 0x13a168 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x13a16c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x13a170 | load | 4 | `abs 0x13a1bc` | `-` |
| `str` at 0x13a17c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x13a184 | load | 4 | `abs 0x13a1c0` | `-` |
| `ldr` at 0x13a188 | load | 4 | `abs 0x1a0878` | `-` |
| `ldr` at 0x13a194 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x13a198 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x13a19c | load | 4 | `abs 0x13a1bc` | `-` |
| `str` at 0x13a1a8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x13a1ac | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x13a1b4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x13a1b4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x13a1b4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x13a1b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x13a14c | `(unknown callee) 0x201fc` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0x13a174 | `(unknown callee) 0x20634` | `[[sp-28]]` | `0x191428` | `?` | `[sp-28]` |
| `mov` at 0x13a190 | `(unknown callee) (unknown target)` | `ret(0x201fc)` | `?` | `?` | `[0x1a0878]` |
| `bl` at 0x13a1a0 | `(unknown callee) 0x20634` | `[[sp-28]]` | `0x191428` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x13a164 | `[0x315030] != 0x0` | `[0x315030] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x13a1b4 | `r0` | `0x0` |

## `smpcmd_scan_chip`

- `smpcmd_scan_chip`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13a1c4 to 0x13a274.
  Established. The result from unnamed 0x13ce6c is stored as a word at entry sp minus 0x20 (`bl` at 0x13a1dc, `str` at 0x13a1e4).
  One call to unnamed 0x201fc has its result stored at the address held in [sp-28] (`bl` at 0x13a20c, `str` at 0x13a214).
  A separate call to that unnamed address has its own result stored at the address held in [sp-28] (`bl` at 0x13a240, `str` at 0x13a248).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x13a1d4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x13a1d8 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x13a1e4 | store | 4 | `sp -0x20` | `ret(0x13ce6c)` |
| `ldr` at 0x13a1e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x13a1f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x13a1f8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x13a214 | store | 4 | `[sp-28] +0x0` | `ret(0x201fc)` |
| `ldr` at 0x13a218 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x13a21c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x13a220 | load | 4 | `abs 0x13a26c` | `-` |
| `ldr` at 0x13a224 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x13a22c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x13a238 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x13a248 | store | 4 | `[sp-28] +0x0` | `ret(0x201fc)` |
| `ldr` at 0x13a24c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x13a250 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x13a254 | load | 4 | `abs 0x13a270` | `-` |
| `ldm` at 0x13a268 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x13a268 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x13a268 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x13a268 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x13a1dc | `(unknown callee) 0x13ce6c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x13a1fc | `(unknown callee) 0x2031c` | `ret(0x13ce6c)` | `?` | `?` | `ret(0x13ce6c)` |
| `bl` at 0x13a20c | `(unknown callee) 0x201fc` | `ret(0x2031c)+120` | `?` | `?` | `ret(0x2031c)+120` |
| `bl` at 0x13a228 | `(unknown callee) 0x205bc` | `[[sp-28]]` | `0x1914dc` | `[sp-32]` | `[sp-28]` |
| `bl` at 0x13a230 | `(unknown callee) 0x20514` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x13a240 | `(unknown callee) 0x201fc` | `0x50` | `?` | `?` | `ret(0x13ce6c)` |
| `bl` at 0x13a258 | `(unknown callee) 0x205bc` | `[[sp-28]]` | `0x1914f8` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x13a1f0 | `ret(0x13ce6c) == 0x0` | `ret(0x13ce6c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x13a268 | `r0` | `0x0` |

## `smpcmd_scan_port`

- `smpcmd_scan_port`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13a274 to 0x13a390.
  Established. A word store places the result from unnamed 0x201fc at the address held in entry r1 (`bl` at 0x13a294, `str` at 0x13a29c).
  The word loaded from the address formed by [sp-24] plus 0x2c is stored at entry sp minus 0x24 (`ldr` at 0x13a304, `str` at 0x13a308).
  Printed r0-r3 candidates for the unknown-target call are [sp-32], [sp-36], unknown and [0x1a0874] (`mov` at 0x13a340).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x13a284 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x13a288 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x13a28c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x13a29c | store | 4 | `r1 +0x0` | `ret(0x201fc)` |
| `ldr` at 0x13a2a0 | load | 4 | `abs 0x13a378` | `-` |
| `ldr` at 0x13a2a4 | load | 4 | `abs 0x315030` | `-` |
| `ldr` at 0x13a2b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x13a2b4 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x13a2b8 | load | 4 | `abs 0x13a37c` | `-` |
| `str` at 0x13a2c4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x13a2cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x13a2d0 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0x13a2d4 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0x13a2e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x13a2e4 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0x13a2e8 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0x13a2f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x13a2f8 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0x13a2fc | store | 4 | `sp -0x20` | `[[sp-24]+40]` |
| `ldr` at 0x13a300 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x13a304 | load | 4 | `[sp-24] +0x2c` | `-` |
| `str` at 0x13a308 | store | 4 | `sp -0x24` | `[[sp-24]+44]` |
| `ldr` at 0x13a310 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x13a314 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0x13a318 | store | 4 | `sp -0x24` | `[[sp-24]+40]` |
| `ldr` at 0x13a320 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x13a328 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x13a32c | load | 4 | `abs 0x13a380` | `-` |
| `ldr` at 0x13a330 | load | 4 | `abs 0x1a0874` | `-` |
| `ldr` at 0x13a334 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x13a338 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x13a344 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x13a348 | load | 4 | `abs 0x13a384` | `-` |
| `ldr` at 0x13a34c | load | 4 | `abs 0x13a388` | `-` |
| `ldr` at 0x13a350 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x13a354 | load | 4 | `abs 0x13a38c` | `-` |
| `ldr` at 0x13a358 | load | 4 | `abs 0x315048` | `-` |
| `ldr` at 0x13a35c | load | 4 | `abs 0x31504c` | `-` |
| `str` at 0x13a368 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x13a36c | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x13a374 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x13a374 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x13a374 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x13a374 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x13a294 | `(unknown callee) 0x201fc` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0x13a2bc | `(unknown callee) 0x20634` | `[[sp-28]]` | `0x191428` | `?` | `[sp-28]` |
| `mov` at 0x13a340 | `(unknown callee) (unknown target)` | `[sp-32]` | `[sp-36]` | `?` | `[0x1a0874]` |
| `bl` at 0x13a360 | `(unknown callee) 0x205bc` | `[[sp-28]]` | `0x191534` | `[0x315048]` | `[0x31504c]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x13a2ac | `[0x315030] != 0x0` | `[0x315030] == 0x0` |
| `beq` at 0x13a2dc | `[[[sp-24]+112]] == 0x0` | `[[[sp-24]+112]] != 0x0` |
| `bne` at 0x13a2f0 | `[[[sp-24]+112]] != 0x2` | `[[[sp-24]+112]] == 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x13a374 | `r0` | `0x0` |

## `smpcmd_scan_ids`

- `smpcmd_scan_ids`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13a390 to 0x13a88c.
  Established. Two calls to unnamed 0x201fc have their results stored as words at entry sp minus 0x40 and 0x44 respectively (`bl` at 0x13a484, `str` at 0x13a48c, `bl` at 0x13a4c4, `str` at 0x13a4cc).
  Two computed word stores write zero to unknown effective targets (`str` at 0x13a52c, `str` at 0x13a544).
  A load, add and word store write [sp-60] plus 0x20 at entry sp minus 0x3c (`ldr` at 0x13a6c4, `add` at 0x13a6c8, `str` at 0x13a6cc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x13a3a0 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x13a3a4 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x13a3a8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x13a3b8 | store | 4 | `r1 +0x0` | `ret(0x201fc)` |
| `ldr` at 0x13a3bc | load | 4 | `abs 0x13a850` | `-` |
| `ldr` at 0x13a3c0 | load | 4 | `abs 0x315030` | `-` |
| `ldr` at 0x13a3cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x13a3d0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x13a3d4 | load | 4 | `abs 0x13a854` | `-` |
| `str` at 0x13a3e0 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x13a3e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x13a3ec | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0x13a3f0 | store | 4 | `sp -0x20` | `[[sp-24]+40]` |
| `ldr` at 0x13a3f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x13a400 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x13a404 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x13a408 | load | 4 | `abs 0x13a858` | `-` |
| `str` at 0x13a418 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x13a420 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x13a42c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x13a430 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x13a434 | load | 4 | `abs 0x13a85c` | `-` |
| `str` at 0x13a440 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x13a448 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x13a450 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x13a454 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x13a468 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x13a474 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x13a478 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x13a48c | store | 4 | `sp -0x40` | `ret(0x201fc)` |
| `ldr` at 0x13a490 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x13a49c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x13a4a0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x13a4a4 | load | 4 | `abs 0x13a860` | `-` |
| `str` at 0x13a4b0 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x13a4b8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x13a4cc | store | 4 | `sp -0x44` | `ret(0x201fc)` |
| `ldr` at 0x13a4d0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x13a4dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x13a4e0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x13a4e4 | load | 4 | `abs 0x13a860` | `-` |
| `ldr` at 0x13a4ec | load | 4 | `sp -0x40` | `-` |
| `str` at 0x13a4f8 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0x13a504 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x13a508 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x13a50c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x13a518 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x13a520 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x13a52c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x13a530 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x13a538 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x13a544 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x13a548 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x13a550 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0x13a568 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x13a56c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x13a574 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x13a578 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x13a58c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x13a590 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x13a594 | load | 4 | `abs 0x13a854` | `-` |
| `ldr` at 0x13a59c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x13a5a4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x13a5b0 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x13a5b8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x13a5bc | store | 4 | `sp -0x58` | `[sp-56]` |
| `ldr` at 0x13a5c0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x13a5c4 | store | 4 | `sp -0x54` | `[sp-64]` |
| `ldr` at 0x13a5c8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x13a5cc | store | 4 | `sp -0x50` | `[sp-68]` |
| `ldr` at 0x13a5d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x13a5d4 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x13a5d8 | store | 4 | `sp -0x4c` | `[[sp-28]]` |
| `ldr` at 0x13a5e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x13a5fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x13a600 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x13a604 | load | 4 | `abs 0x13a854` | `-` |
| `ldr` at 0x13a60c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x13a614 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x13a620 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x13a628 | load | 4 | `abs 0x13a864` | `-` |
| `ldr` at 0x13a630 | load | 4 | `abs 0x13a868` | `-` |
| `ldr` at 0x13a638 | load | 4 | `abs 0x13a86c` | `-` |
| `ldr` at 0x13a640 | load | 4 | `abs 0x13a870` | `-` |
| `str` at 0x13a64c | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0x13a654 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0x13a658 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x13a65c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x13a668 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x13a66c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x13a680 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x13a688 | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldr` at 0x13a68c | load | 4 | `abs 0x13a874` | `-` |
| `ldr` at 0x13a690 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x13a6a0 | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0x13a6ac | store | 4 | `sp -0x58` | `0x20` |
| `ldr` at 0x13a6b0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x13a6b4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x13a6c4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x13a6cc | store | 4 | `sp -0x3c` | `[sp-60]+32` |
| `ldr` at 0x13a6d0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x13a6d4 | load | 4 | `abs 0x13a878` | `-` |
| `ldr` at 0x13a6e0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x13a6e8 | load | 4 | `abs 0x13a878` | `-` |
| `str` at 0x13a6f4 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x13a6f8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x13a6fc | load | 4 | `abs 0x13a87c` | `-` |
| `ldr` at 0x13a708 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x13a710 | load | 4 | `abs 0x13a87c` | `-` |
| `str` at 0x13a71c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x13a720 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x13a724 | load | 4 | `abs 0x13a880` | `-` |
| `ldr` at 0x13a730 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x13a738 | load | 4 | `abs 0x13a880` | `-` |
| `str` at 0x13a744 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x13a748 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x13a758 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x13a76c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x13a770 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x13a780 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x13a794 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x13a79c | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x13a7a8 | load | 1 | `sp -0x30` | `-` |
| `ldr` at 0x13a7b0 | load | 4 | `sp -0x30` | `-` |
| `ldrh` at 0x13a7c0 | load | 2 | `sp -0x2e` | `-` |
| `str` at 0x13a7d8 | store | 4 | `sp -0x58` | `?` |
| `ldrb` at 0x13a7dc | load | 1 | `sp -0x2d` | `-` |
| `str` at 0x13a7e8 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x13a7ec | load | 4 | `sp -0x34` | `-` |
| `str` at 0x13a7f0 | store | 4 | `sp -0x50` | `[sp-52]` |
| `ldr` at 0x13a7f4 | load | 4 | `abs 0x13a884` | `-` |
| `ldr` at 0x13a7f8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x13a804 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x13a80c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x13a814 | load | 4 | `abs 0x13a854` | `-` |
| `ldr` at 0x13a81c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x13a820 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x13a824 | load | 4 | `abs 0x13a888` | `-` |
| `ldr` at 0x13a82c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x13a834 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x13a840 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x13a844 | load | 4 | `sp -0x48` | `-` |
| `ldm` at 0x13a84c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x13a84c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x13a84c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x13a84c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x13a3b0 | `(unknown callee) 0x201fc` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0x13a3d8 | `(unknown callee) 0x20634` | `[[sp-28]]` | `0x191428` | `?` | `[sp-28]` |
| `bl` at 0x13a410 | `(unknown callee) 0x205bc` | `[[sp-28]]` | `0x191554` | `0x40` | `[sp-28]` |
| `bl` at 0x13a438 | `(unknown callee) 0x205bc` | `[[sp-28]]` | `0x19157c` | `?` | `[sp-28]` |
| `bl` at 0x13a484 | `(unknown callee) 0x201fc` | `?` | `?` | `?` | `?` |
| `bl` at 0x13a4a8 | `(unknown callee) 0x205bc` | `[[sp-28]]` | `0x1915a0` | `?` | `[sp-28]` |
| `bl` at 0x13a4c4 | `(unknown callee) 0x201fc` | `?` | `?` | `?` | `?` |
| `bl` at 0x13a4e8 | `(unknown callee) 0x205bc` | `[[sp-28]]` | `0x1915a0` | `?` | `[sp-28]` |
| `bl` at 0x13a4f0 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x13a558 | `(unknown callee) 0x137100` | `ret(0x201fc)` | `?` | `?` | `[sp-36]` |
| `bl` at 0x13a57c | `(unknown callee) 0x12cc8c` | `0x1` | `[sp-40]` | `[[sp-28]]` | `[sp-28]` |
| `bl` at 0x13a598 | `(unknown callee) 0x203c4` | `[[sp-28]]` | `0x191428` | `?` | `[sp-28]` |
| `bl` at 0x13a5a0 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x13a5a8 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x13a5ec | `(unknown callee) 0x12d2b8` | `0x1` | `0x0` | `0x1` | `[sp-40]` |
| `bl` at 0x13a608 | `(unknown callee) 0x203c4` | `[[sp-28]]` | `0x191428` | `?` | `[sp-28]` |
| `bl` at 0x13a610 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x13a618 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x13a62c | `(unknown callee) 0x243d4` | `0x1915cc` | `?` | `?` | `ret(0x12d2b8)` |
| `bl` at 0x13a634 | `(unknown callee) 0x243d4` | `0x1915d0` | `?` | `?` | `?` |
| `bl` at 0x13a63c | `(unknown callee) 0x243d4` | `0x1915e8` | `?` | `?` | `?` |
| `bl` at 0x13a644 | `(unknown callee) 0x243d4` | `0x191618` | `?` | `?` | `?` |
| `bl` at 0x13a670 | `(unknown callee) 0x131fb0` | `?` | `[sp-68]` | `?` | `[sp-32]` |
| `bl` at 0x13a694 | `(unknown callee) 0x243d4` | `0x191648` | `[sp-44]` | `?` | `[sp-60]+1` |
| `bl` at 0x13a6c0 | `(unknown callee) 0x132444` | `[sp-60]` | `[sp-68]` | `0x0` | `sp-52` |
| `bl` at 0x13a7a4 | `(unknown callee) 0x13aae0` | `[sp-52]` | `sp-48` | `?` | `sp-48` |
| `bl` at 0x13a800 | `(unknown callee) 0x243d4` | `0x191674` | `[sp-44]` | `?` | `?` |
| `bl` at 0x13a818 | `(unknown callee) 0x243d4` | `0x191428` | `?` | `?` | `[sp-32]` |
| `bl` at 0x13a828 | `(unknown callee) 0x205bc` | `[[sp-28]]` | `0x1916ac` | `?` | `[sp-28]` |
| `bl` at 0x13a830 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x13a838 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x13a3c8 | `[0x315030] != 0x0` | `[0x315030] == 0x0` |
| `bls` at 0x13a3fc | `[[sp-24]+40] <= 0x40 (unsigned, 32-bit)` | `[[sp-24]+40] > 0x40 (unsigned, 32-bit)` |
| `bne` at 0x13a428 | `[[sp-24]+40] != 0x0` | `[[sp-24]+40] == 0x0` |
| `bne` at 0x13a498 | `ret(0x201fc) != 0x0` | `ret(0x201fc) == 0x0` |
| `bne` at 0x13a4d8 | `ret(0x201fc) != 0x0` | `ret(0x201fc) == 0x0` |
| `bhs` at 0x13a514 | `C == 1` | `not (C == 1)` |
| `bne` at 0x13a588 | `ret(0x12cc8c) != 0x0` | `ret(0x12cc8c) == 0x0` |
| `bne` at 0x13a5f8 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bhs` at 0x13a664 | `C == 1` | `not (C == 1)` |
| `bne` at 0x13a67c | `ret(0x131fb0) != 0x0` | `ret(0x131fb0) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x13a84c | `r0` | `0x0` |

## `smpcmd_scan_debug`

- `smpcmd_scan_debug`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13a88c to 0x13a934.
  Established. The word loaded from the address formed by [sp-24] plus 0x28 is stored at entry sp minus 0x20 (`ldr` at 0x13a8d0, `str` at 0x13a8d4).
  A word store at absolute address 0x315034 writes [[sp-24]+40] (`str` at 0x13a8fc).
  A separate load from absolute address 0x315034 supplies the printed r2 candidate [0x315034] for a call to unnamed 0x205bc (`ldr` at 0x13a910, `bl` at 0x13a914).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x13a89c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x13a8a0 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x13a8a4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x13a8b4 | store | 4 | `r1 +0x0` | `ret(0x201fc)` |
| `ldr` at 0x13a8b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x13a8bc | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0x13a8c0 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0x13a8cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x13a8d0 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0x13a8d4 | store | 4 | `sp -0x20` | `[[sp-24]+40]` |
| `ldr` at 0x13a8d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x13a8e4 | load | 4 | `abs 0x13a928` | `-` |
| `ldr` at 0x13a8f4 | load | 4 | `abs 0x13a92c` | `-` |
| `ldr` at 0x13a8f8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x13a8fc | store | 4 | `abs 0x315034` | `[[sp-24]+40]` |
| `ldr` at 0x13a900 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x13a904 | load | 4 | `abs 0x13a92c` | `-` |
| `ldr` at 0x13a908 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x13a90c | load | 4 | `abs 0x13a930` | `-` |
| `ldr` at 0x13a910 | load | 4 | `abs 0x315034` | `-` |
| `ldm` at 0x13a924 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x13a924 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x13a924 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x13a924 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x13a8ac | `(unknown callee) 0x201fc` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0x13a8ec | `(unknown callee) 0x243d4` | `0x1916d8` | `0x4` | `?` | `[[sp-24]+40]` |
| `bl` at 0x13a914 | `(unknown callee) 0x205bc` | `[[sp-28]]` | `0x191714` | `[0x315034]` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x13a8c8 | `[[[sp-24]+112]] != 0x1` | `[[[sp-24]+112]] == 0x1` |
| `bls` at 0x13a8e0 | `[[sp-24]+40] <= 0x4 (unsigned, 32-bit)` | `[[sp-24]+40] > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x13a924 | `r0` | `0x0` |

## `smpcmd_scan_set`

- `smpcmd_scan_set`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13a934 to 0x13aa30.
  Established. The result from unnamed 0x137100 is stored as a word at entry sp minus 0x24 (`bl` at 0x13a9a0, `str` at 0x13a9a8).
  Two other word stores at that destination have unknown values (`str` at 0x13a9dc, `str` at 0x13a9fc).
  Printed r0-r3 candidates for the call to unnamed 0x137458 are [sp-36], unknown, unknown and unknown (`bl` at 0x13aa04).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x13a944 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x13a948 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x13a94c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x13a95c | store | 4 | `r1 +0x0` | `ret(0x201fc)` |
| `ldr` at 0x13a960 | load | 4 | `abs 0x13aa28` | `-` |
| `ldr` at 0x13a964 | load | 4 | `abs 0x315030` | `-` |
| `ldr` at 0x13a970 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x13a974 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x13a978 | load | 4 | `abs 0x13aa2c` | `-` |
| `str` at 0x13a984 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x13a98c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x13a990 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0x13a994 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `str` at 0x13a9a8 | store | 4 | `sp -0x24` | `ret(0x137100)` |
| `ldr` at 0x13a9ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x13a9b4 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0x13a9bc | load | 4 | `[[sp-24]+112] +0x4` | `-` |
| `ldr` at 0x13a9c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x13a9cc | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0x13a9d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x13a9d4 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `str` at 0x13a9dc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x13a9e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x13a9e8 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0x13a9ec | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0x13a9f4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x13a9fc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x13aa00 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x13aa08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x13aa0c | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x13aa18 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x13aa1c | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x13aa24 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x13aa24 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x13aa24 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x13aa24 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x13a954 | `(unknown callee) 0x201fc` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0x13a97c | `(unknown callee) 0x203c4` | `[[sp-28]]` | `0x191428` | `?` | `[sp-28]` |
| `bl` at 0x13a9a0 | `(unknown callee) 0x137100` | `ret(0x201fc)` | `?` | `?` | `[[[sp-24]+112]]` |
| `bl` at 0x13aa04 | `(unknown callee) 0x137458` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0x13aa10 | `(unknown callee) 0x137224` | `[[sp-28]]` | `?` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x13a96c | `[0x315030] != 0x0` | `[0x315030] == 0x0` |
| `beq` at 0x13a99c | `[[[sp-24]+112]] == 0x0` | `[[[sp-24]+112]] != 0x0` |
| `beq` at 0x13a9c4 | `[[[sp-24]+112]+4] == 0x0` | `[[[sp-24]+112]+4] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x13aa24 | `r0` | `0x0` |

## `smpcmd_scan_imp`

- `smpcmd_scan_imp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13aa30 to 0x13aa88.
  Established. The call to unnamed 0x201fc has printed r0-r3 candidates 0xa0, entry r1, entry r2 and entry r3 (`bl` at 0x13aa50).
  Its result is stored as a word at the address held in entry r1 (`bl` at 0x13aa50, `str` at 0x13aa58).
  Another word store writes zero at entry sp minus 0x2c (`str` at 0x13aa60).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x13aa40 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x13aa44 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x13aa48 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x13aa58 | store | 4 | `r1 +0x0` | `ret(0x201fc)` |
| `str` at 0x13aa60 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x13aa64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x13aa68 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x13aa6c | load | 4 | `abs 0x13aa84` | `-` |
| `ldm` at 0x13aa80 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x13aa80 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x13aa80 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x13aa80 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x13aa50 | `(unknown callee) 0x201fc` | `0xa0` | `r1` | `r2` | `r3` |
| `bl` at 0x13aa70 | `(unknown callee) 0x205bc` | `[[sp-28]]` | `0x19172c` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x13aa80 | `r0` | `0x0` |

## `smpcmd_scan_test`

- `smpcmd_scan_test`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13aa88 to 0x13aae0.
  Established. A word store writes the result from unnamed 0x201fc at the address held in entry r1 (`bl` at 0x13aaa8, `str` at 0x13aab0).
  A stack word at entry sp minus 0x20 receives zero (`str` at 0x13aab8).
  Printed r0-r3 candidates for the call to unnamed 0x205bc are [[sp-28]], 0x191764, unknown and [sp-28] (`bl` at 0x13aac8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x13aa98 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x13aa9c | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x13aaa0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x13aab0 | store | 4 | `r1 +0x0` | `ret(0x201fc)` |
| `str` at 0x13aab8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x13aabc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x13aac0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x13aac4 | load | 4 | `abs 0x13aadc` | `-` |
| `ldm` at 0x13aad8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x13aad8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x13aad8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x13aad8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x13aaa8 | `(unknown callee) 0x201fc` | `0xa0` | `r1` | `r2` | `r3` |
| `bl` at 0x13aac8 | `(unknown callee) 0x205bc` | `[[sp-28]]` | `0x191764` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x13aad8 | `r0` | `0x0` |

## `Daytona_JTAG_Sel_Query`

- `Daytona_JTAG_Sel_Query`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x46a5c to 0x46ba4.
  Established. A byte store writes the low eight bits of the result from unnamed 0x125c20 at entry sp minus 0x19 (`bl` at 0x46a7c, `strb` at 0x46a84).
  A separate call to that unnamed address has its low eight result bits stored as a byte at entry sp minus 0x1a (`bl` at 0x46a90, `strb` at 0x46a98).
  A call to unnamed 0x2031c, an add of one and a word store write ret(0x2031c)+1 at entry sp minus 0x28 (`bl` at 0x46b08, `add` at 0x46b10, `str` at 0x46b14).
  A separate call to unnamed 0x2031c is followed by an add and word store of ([sp-24] + ret(0x2031c)) at entry sp minus 0x18 (`bl` at 0x46b70, `add` at 0x46b80, `str` at 0x46b84).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x46a6c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x46a70 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x46a74 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x46a84 | store | 1 | `sp -0x19` | `(ret(0x125c20) & 0xff)` |
| `ldr` at 0x46a88 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x46a98 | store | 1 | `sp -0x1a` | `(ret(0x125c20) & 0xff)` |
| `ldrsb` at 0x46a9c | load | 1 | `sp -0x19` | `-` |
| `ldrsb` at 0x46aa8 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0x46ab4 | load | 4 | `abs 0x46b90` | `-` |
| `str` at 0x46ab8 | store | 4 | `sp -0x20` | `0x15ca08` |
| `ldr` at 0x46ac0 | load | 4 | `abs 0x46b94` | `-` |
| `str` at 0x46ac4 | store | 4 | `sp -0x20` | `0x15ca10` |
| `ldrsb` at 0x46acc | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0x46ad8 | load | 4 | `abs 0x46b98` | `-` |
| `str` at 0x46adc | store | 4 | `sp -0x20` | `0x15ca14` |
| `ldr` at 0x46ae4 | load | 4 | `abs 0x46b9c` | `-` |
| `str` at 0x46ae8 | store | 4 | `sp -0x20` | `0x15ca1c` |
| `ldr` at 0x46aec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x46b00 | store | 4 | `sp -0x24` | `ret(0x6e1b0)` |
| `ldr` at 0x46b04 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x46b14 | store | 4 | `sp -0x28` | `ret(0x2031c)+1` |
| `ldr` at 0x46b18 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x46b40 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x46b44 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x46b48 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x46b4c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x46b58 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x46b5c | load | 4 | `abs 0x46ba0` | `-` |
| `ldr` at 0x46b64 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x46b6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x46b7c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x46b84 | store | 4 | `sp -0x18` | `([sp-24] + ret(0x2031c))` |
| `ldm` at 0x46b8c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x46b8c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x46b8c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x46a7c | `(unknown callee) 0x125c20` | `r0` | `0x4` | `r2` | `r3` |
| `bl` at 0x46a90 | `(unknown callee) 0x125c20` | `[sp-20]` | `0x8` | `?` | `ret(0x125c20)` |
| `bl` at 0x46afc | `(unknown callee) 0x6e1b0` | `[sp-20]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x46b08 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x46b50 | `(unknown callee) 0x203f4` | `[sp-44]` | `[sp-36]` | `[sp-40]` | `?` |
| `bl` at 0x46b68 | `(unknown callee) 0x205bc` | `[sp-24]` | `0x15ca24` | `ret(0x203f4)` | `[sp-32]` |
| `bl` at 0x46b70 | `(unknown callee) 0x2031c` | `[sp-24]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x46aa4 | `? >= 0x0 (signed, 32-bit)` | `? < 0x0 (signed, 32-bit)` |
| `bge` at 0x46ab0 | `? >= 0x0 (signed, 32-bit)` | `? < 0x0 (signed, 32-bit)` |
| `bge` at 0x46ad4 | `? >= 0x0 (signed, 32-bit)` | `? < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x46b8c | `r0` | `ret(0x2031c)` |

## `Daytona_JTAG_Sel`

- `Daytona_JTAG_Sel`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x46ba4 to 0x46c9c.
  Established. Byte stores save the low eight bits of entry r1 and r2 at entry sp minus 0x15 and 0x16 (`strb` at 0x46bbc, `strb` at 0x46bc4).
  The low eight bits of the result from unnamed 0x125a58 are stored as a byte at entry sp minus 0x17 (`bl` at 0x46c28, `strb` at 0x46c30).
  A byte store at entry sp minus 0x19 has an unknown value after an unsupported mvn (`mvn` at 0x46bd4, `strb` at 0x46bd8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x46bb4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x46bbc | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x46bc4 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0x46bc8 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0x46bd8 | store | 1 | `sp -0x19` | `?` |
| `strb` at 0x46be4 | store | 1 | `sp -0x19` | `0x0` |
| `ldrb` at 0x46be8 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0x46bec | store | 1 | `sp -0x15` | `([sp-25]:1 & 0xff)` |
| `ldrb` at 0x46bf0 | load | 1 | `sp -0x16` | `-` |
| `strb` at 0x46c00 | store | 1 | `sp -0x1a` | `?` |
| `strb` at 0x46c0c | store | 1 | `sp -0x1a` | `0x0` |
| `ldrb` at 0x46c10 | load | 1 | `sp -0x1a` | `-` |
| `strb` at 0x46c14 | store | 1 | `sp -0x16` | `([sp-26]:1 & 0xff)` |
| `ldrb` at 0x46c18 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x46c1c | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x46c30 | store | 1 | `sp -0x17` | `(ret(0x125a58) & 0xff)` |
| `strb` at 0x46c38 | store | 1 | `sp -0x1b` | `0x0` |
| `ldrb` at 0x46c3c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x46c40 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x46c5c | load | 1 | `sp -0x17` | `-` |
| `strb` at 0x46c6c | store | 1 | `sp -0x1b` | `0x1` |
| `ldrb` at 0x46c70 | load | 1 | `sp -0x1b` | `-` |
| `strb` at 0x46c74 | store | 1 | `sp -0x18` | `([sp-27]:1 & 0xff)` |
| `ldrb` at 0x46c84 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0x46c88 | store | 1 | `sp -0x17` | `(([sp-27]:1 & 0xff) & 0xff)` |
| `ldrb` at 0x46c8c | load | 1 | `sp -0x17` | `-` |
| `ldm` at 0x46c98 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x46c98 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x46c98 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x46c28 | `(unknown callee) 0x125a58` | `[sp-20]` | `0x4` | `0x80` | `[sp-21]:1` |
| `bl` at 0x46c4c | `(unknown callee) 0x125a58` | `[sp-20]` | `0x8` | `0x80` | `[sp-22]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x46bd0 | `(r1 & 0xff) == 0x0` | `(r1 & 0xff) != 0x0` |
| `beq` at 0x46bf8 | `[sp-22]:1 == 0x0` | `[sp-22]:1 != 0x0` |
| `beq` at 0x46c58 | `ret(0x125a58) == 0x0` | `ret(0x125a58) != 0x0` |
| `bne` at 0x46c64 | `[sp-23]:1 != 0x0` | `[sp-23]:1 == 0x0` |
| `beq` at 0x46c80 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x46c98 | `r0` | `[sp-23]:1` |

## `Abacus_JTAG_Sel_Query`

- `Abacus_JTAG_Sel_Query`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4734c to 0x473cc.
  Established. A byte store writes the low eight bits of the result from unnamed 0x125c20 at entry sp minus 0x19 (`bl` at 0x4736c, `strb` at 0x47374).
  Printed r0-r3 candidates for the call to unnamed 0x2031c are [sp-24], unknown, unknown and unknown (`bl` at 0x473a4).
  That result contributes to the word stored at entry sp minus 0x18 as ([sp-24] + ret(0x2031c)) (`bl` at 0x473a4, `add` at 0x473b4, `str` at 0x473b8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4735c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x47360 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x47364 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x47374 | store | 1 | `sp -0x19` | `(ret(0x125c20) & 0xff)` |
| `ldrsb` at 0x47378 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x47384 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47388 | load | 4 | `abs 0x473c4` | `-` |
| `ldr` at 0x47394 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47398 | load | 4 | `abs 0x473c8` | `-` |
| `ldr` at 0x473a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x473b0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x473b8 | store | 4 | `sp -0x18` | `([sp-24] + ret(0x2031c))` |
| `ldm` at 0x473c0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x473c0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x473c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4736c | `(unknown callee) 0x125c20` | `r0` | `0x8` | `r2` | `r3` |
| `bl` at 0x4738c | `(unknown callee) 0x205bc` | `[sp-24]` | `0x15d0ac` | `?` | `?` |
| `bl` at 0x4739c | `(unknown callee) 0x205bc` | `[sp-24]` | `0x15d0c4` | `?` | `?` |
| `bl` at 0x473a4 | `(unknown callee) 0x2031c` | `[sp-24]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x47380 | `? >= 0x0 (signed, 32-bit)` | `? < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x473c0 | `r0` | `ret(0x2031c)` |

## `Abacus_JTAG_Sel_State`

- `Abacus_JTAG_Sel_State`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x473cc to 0x4740c.
  Established. The printed r0-r3 candidates for the call to unnamed 0x125c20 are entry r0, 8, entry r2 and entry r3 (`bl` at 0x473e8).
  A byte store preserves the low eight bits of that call result at entry sp minus 0x15 (`bl` at 0x473e8, `strb` at 0x473f0).
  The loaded byte participates in an AND using 0x80, while the tool's recorded return in r0 remains unknown (`ldrb` at 0x473f4, `mov` at 0x473f8, `and` at 0x473fc, `ldm` at 0x47408).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x473dc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x473e0 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x473f0 | store | 1 | `sp -0x15` | `(ret(0x125c20) & 0xff)` |
| `ldrb` at 0x473f4 | load | 1 | `sp -0x15` | `-` |
| `ldm` at 0x47408 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x47408 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x47408 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x473e8 | `(unknown callee) 0x125c20` | `r0` | `0x8` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x47408 | `r0` | `?` |
