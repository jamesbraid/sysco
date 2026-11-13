# ARM link, PCI-X and power state in 1.34.2

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
Undecodable byte ranges remain visible as fact-sheet warnings. Values and predicates stay unknown where the analysis cannot follow an effect.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/link-pcix-power-state-facts.txt) retain all tool sections.

## `SHUB_WarStatus`

- `SHUB_WarStatus`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2a830 to 0x2a880.
  Established. A byte read uses absolute `0x119c01` (`ldrb` at 0x2a860). Entry r0 zero takes the
  earlier branch, while nonzero falls through to a call to `0x18c74` with candidates entry r0,
  `0xdb62c`, entry r2 and entry r0 (`beq` at 0x2a84c, `bl` at 0x2a858). The recorded r0 return value is
  unknown (`ldmdb` at 0x2a874). The status byte is not identified as a
  hardware register.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2a840 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2a844 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a850 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a854 | load | 4 | `abs 0x2a878` | `-` |
| `ldr` at 0x2a85c | load | 4 | `abs 0x2a87c` | `-` |
| `ldrb` at 0x2a860 | load | 1 | `abs 0x119c01` | `-` |
| `ldmdb` at 0x2a874 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2a874 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2a874 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2a858 | `(unknown callee) 0x18c74` | `r0` | `0xdb62c` | `r2` | `r0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x2a84c | `r0 == 0x0` | `r0 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2a874 | `r0` | `?` |

## `SHUB_WarAllow`

- `SHUB_WarAllow`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2a880 to 0x2a8a8.
  Established. The byte store writes `([sp-20]:1 & 0xff)` to absolute `0x119c00` (`strb` at
  0x2a89c). The exit retains entry r0 (`ldmdb` at 0x2a8a0). There are no recorded calls or
  conditional branches. The permission byte's hardware effect is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2a890 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2a894 | load | 4 | `abs 0x2a8a4` | `-` |
| `ldrb` at 0x2a898 | load | 1 | `sp -0x14` | `-` |
| `strb` at 0x2a89c | store | 1 | `abs 0x119c00` | `([sp-20]:1 & 0xff)` |
| `ldmdb` at 0x2a8a0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2a8a0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2a8a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2a8a0 | `r0` | `r0` |

## `SHUB_WarEnable`

- `SHUB_WarEnable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2a8a8 to 0x2a9a8.
  Established. Zero entry r0 takes the first branch and nonzero falls through (`beq` at 0x2a8c4). A
  subsequent byte test takes its branch for `[0x119c00]:1 == 0x0`, with nonzero falling through
  (`beq` at 0x2a8d4). The indirect call has unknown target and candidates `0xff`, entry r1, entry r2
  and `[0x119a3c]` (`mov` at 0x2a8f8). Separate paths store byte `0x1` or `0x0` at absolute
  `0x119c01` (`strb` at 0x2a938, `strb` at 0x2a970). The workaround's physical effect remains
  unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2a8b8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2a8bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a8c8 | load | 4 | `abs 0x2a980` | `-` |
| `ldrb` at 0x2a8cc | load | 1 | `abs 0x119c00` | `-` |
| `ldr` at 0x2a8d8 | load | 4 | `abs 0x2a984` | `-` |
| `ldr` at 0x2a8dc | load | 4 | `abs 0x119a3c` | `-` |
| `ldr` at 0x2a8e8 | load | 4 | `abs 0x2a984` | `-` |
| `ldr` at 0x2a8ec | load | 4 | `abs 0x119a3c` | `-` |
| `ldr` at 0x2a908 | load | 4 | `abs 0x2a988` | `-` |
| `str` at 0x2a90c | store | 4 | `sp -0x18` | `0xdb63c` |
| `ldr` at 0x2a914 | load | 4 | `abs 0x2a98c` | `-` |
| `ldr` at 0x2a928 | load | 4 | `abs 0x2a990` | `-` |
| `ldr` at 0x2a930 | load | 4 | `abs 0x2a994` | `-` |
| `strb` at 0x2a938 | store | 1 | `abs 0x119c01` | `0x1` |
| `ldr` at 0x2a93c | load | 4 | `abs 0x2a998` | `-` |
| `ldr` at 0x2a948 | load | 4 | `abs 0x2a99c` | `-` |
| `str` at 0x2a94c | store | 4 | `sp -0x18` | `0xdb668` |
| `ldr` at 0x2a954 | load | 4 | `abs 0x2a98c` | `-` |
| `ldr` at 0x2a958 | load | 4 | `abs 0x2a9a0` | `-` |
| `ldr` at 0x2a968 | load | 4 | `abs 0x2a994` | `-` |
| `strb` at 0x2a970 | store | 1 | `abs 0x119c01` | `0x0` |
| `ldr` at 0x2a974 | load | 4 | `abs 0x2a9a4` | `-` |
| `ldmdb` at 0x2a97c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2a97c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2a97c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0x2a8f8 | `(unknown callee) (unknown target)` | `0xff` | `r1` | `r2` | `[0x119a3c]` |
| `bl` at 0x2a920 | `(unknown callee) 0x1eed0` | `0x1` | `0xda5f4` | `0x860` | `0x0` |
| `bl` at 0x2a92c | `(unknown callee) 0x1d6d8` | `0xdb65c` | `?` | `?` | `?` |
| `bl` at 0x2a940 | `(unknown callee) 0x1fe64` | `0x119bac` | `?` | `0x1` | `0x119c01` |
| `bl` at 0x2a960 | `(unknown callee) 0x1eed0` | `0x3` | `0xda5f4` | `0x868` | `0x0` |
| `bl` at 0x2a978 | `(unknown callee) 0x1d6d8` | `0xdb69c` | `r1` | `0x0` | `0x119c01` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x2a8c4 | `r0 == 0x0` | `r0 != 0x0` |
| `beq` at 0x2a8d4 | `[0x119c00]:1 == 0x0` | `[0x119c00]:1 != 0x0` |
| `beq` at 0x2a8e4 | `[0x119a3c] == 0x0` | `[0x119a3c] != 0x0` |
| `beq` at 0x2a904 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2a97c | `r0` | `?` |

## `TIOInit`

- `TIOInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x45448 to 0x457c8.
  Established. The call to `0x48fac` supplies `0x45bc0`, `0x47870`, `0x4794c` and `0x46030` as
  candidates (`bl` at 0x454b8). Later stores write word `[sp-24]` at absolute `0x119e78` (`str` at
  0x455dc), byte `([sp-28]:1 & 0xff)` at absolute `0x2811d8` (`strb` at 0x455e8), and byte
  `([sp-32]:1 & 0xff)` at absolute `0x2811d0` (`strb` at 0x455f4). The first branch takes
  `[0x119e78] == 0x0` and falls through for nonzero (`beq` at 0x45470). The values do not establish
  a hardware initialization contract.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x45458 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x4545c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x45460 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x45464 | load | 4 | `abs 0x45788` | `-` |
| `ldr` at 0x45468 | load | 4 | `abs 0x119e78` | `-` |
| `ldr` at 0x45474 | load | 4 | `abs 0x4578c` | `-` |
| `str` at 0x45478 | store | 4 | `sp -0x38` | `0xe2e20` |
| `ldr` at 0x4547c | load | 4 | `abs 0x45790` | `-` |
| `ldrb` at 0x45480 | load | 1 | `abs 0x2811d8` | `-` |
| `str` at 0x45484 | store | 4 | `sp -0x34` | `[0x2811d8]:1` |
| `ldr` at 0x4548c | load | 4 | `abs 0x45794` | `-` |
| `ldr` at 0x454a0 | load | 4 | `abs 0x45798` | `-` |
| `str` at 0x454a4 | store | 4 | `sp -0x38` | `0x46344` |
| `ldr` at 0x454a8 | load | 4 | `abs 0x4579c` | `-` |
| `ldr` at 0x454ac | load | 4 | `abs 0x457a0` | `-` |
| `ldr` at 0x454b0 | load | 4 | `abs 0x457a4` | `-` |
| `ldr` at 0x454b4 | load | 4 | `abs 0x457a8` | `-` |
| `str` at 0x454c0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x454c4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x454c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x454d8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x454e0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x454e8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x454f0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x454f4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x454fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45504 | load | 4 | `(unknown)` | `-` |
| `str` at 0x45510 | store | 4 | `sp -0x28` | `ret(0xc8e70)` |
| `ldr` at 0x45514 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x45520 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4552c | load | 4 | `abs 0x457ac` | `-` |
| `str` at 0x45530 | store | 4 | `sp -0x38` | `0xe2e50` |
| `ldr` at 0x45534 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4553c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45544 | load | 4 | `(unknown)` | `-` |
| `str` at 0x45548 | store | 4 | `sp -0x34` | `?` |
| `str` at 0x4554c | store | 4 | `sp -0x30` | `ret(0x1e71c)` |
| `ldr` at 0x45554 | load | 4 | `abs 0x45794` | `-` |
| `ldr` at 0x45564 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x4556c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x45574 | load | 4 | `abs 0x457b0` | `-` |
| `ldr` at 0x45578 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4558c | store | 4 | `abs 0x119e80` | `ret(0x18920)` |
| `ldr` at 0x45590 | load | 4 | `abs 0x119e80` | `-` |
| `ldr` at 0x4559c | load | 4 | `abs 0x457b4` | `-` |
| `str` at 0x455a0 | store | 4 | `sp -0x38` | `0xe2e6c` |
| `ldr` at 0x455a8 | load | 4 | `abs 0x45794` | `-` |
| `ldr` at 0x455b8 | load | 4 | `abs 0x457b0` | `-` |
| `ldr` at 0x455bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x455c4 | load | 4 | `abs 0x119e80` | `-` |
| `ldr` at 0x455d4 | load | 4 | `abs 0x45788` | `-` |
| `ldr` at 0x455d8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x455dc | store | 4 | `abs 0x119e78` | `[sp-24]` |
| `ldr` at 0x455e0 | load | 4 | `abs 0x45790` | `-` |
| `ldrb` at 0x455e4 | load | 1 | `sp -0x1c` | `-` |
| `strb` at 0x455e8 | store | 1 | `abs 0x2811d8` | `([sp-28]:1 & 0xff)` |
| `ldr` at 0x455ec | load | 4 | `abs 0x457b8` | `-` |
| `ldrb` at 0x455f0 | load | 1 | `sp -0x20` | `-` |
| `strb` at 0x455f4 | store | 1 | `abs 0x2811d0` | `([sp-32]:1 & 0xff)` |
| `str` at 0x4560c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x45610 | load | 4 | `abs 0x457bc` | `-` |
| `strb` at 0x45618 | store | 1 | `abs 0x119e7c` | `0x1` |
| `ldr` at 0x4561c | load | 4 | `abs 0x45790` | `-` |
| `ldrb` at 0x45620 | load | 1 | `abs 0x2811d8` | `-` |
| `ldr` at 0x45624 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x45630 | load | 4 | `abs 0x457bc` | `-` |
| `ldrb` at 0x45634 | load | 1 | `abs 0x119e7c` | `-` |
| `ldr` at 0x45644 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4564c | load | 4 | `abs 0x45788` | `-` |
| `ldr` at 0x45650 | load | 4 | `abs 0x119e78` | `-` |
| `ldrb` at 0x45658 | load | 1 | `(unknown)` | `-` |
| `str` at 0x45670 | store | 4 | `sp -0x28` | `ret(0xb7af8)` |
| `ldr` at 0x45674 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x45680 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4568c | load | 4 | `abs 0x457c0` | `-` |
| `str` at 0x45690 | store | 4 | `sp -0x38` | `0xe2e88` |
| `ldr` at 0x45694 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x45698 | store | 4 | `sp -0x34` | `[sp-44]` |
| `str` at 0x4569c | store | 4 | `sp -0x30` | `ret(0x1ea50)` |
| `ldr` at 0x456a4 | load | 4 | `abs 0x45794` | `-` |
| `ldr` at 0x456b4 | load | 4 | `abs 0x457bc` | `-` |
| `strb` at 0x456bc | store | 1 | `abs 0x119e7c` | `0x0` |
| `ldrb` at 0x456c4 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x456d4 | load | 4 | `abs 0x457bc` | `-` |
| `strb` at 0x456dc | store | 1 | `abs 0x119e7c` | `0x0` |
| `ldr` at 0x456e0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x456e8 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x456f0 | load | 4 | `abs 0x457bc` | `-` |
| `ldrb` at 0x456f4 | load | 1 | `abs 0x119e7c` | `-` |
| `str` at 0x45704 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x45708 | load | 4 | `abs 0x45790` | `-` |
| `ldrb` at 0x4570c | load | 1 | `abs 0x2811d8` | `-` |
| `ldr` at 0x45710 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x45720 | load | 1 | `sp -0x2c` | `-` |
| `str` at 0x45730 | store | 4 | `sp -0x28` | `ret(0x46634)` |
| `ldr` at 0x45734 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x45740 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4574c | load | 4 | `abs 0x457c4` | `-` |
| `str` at 0x45750 | store | 4 | `sp -0x38` | `0xe2ea8` |
| `ldr` at 0x45754 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x45758 | store | 4 | `sp -0x34` | `[sp-44]` |
| `str` at 0x4575c | store | 4 | `sp -0x30` | `ret(0x1ea50)` |
| `ldr` at 0x45764 | load | 4 | `abs 0x45794` | `-` |
| `ldr` at 0x45774 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x4577c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldmdb` at 0x45784 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x45784 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x45784 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x45784 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x45498 | `(unknown callee) 0x1eed0` | `0x9` | `0xe2e3c` | `0xc7` | `0x0` |
| `bl` at 0x4549c | `(unknown callee) 0x8ac68` | `?` | `?` | `?` | `?` |
| `bl` at 0x454b8 | `(unknown callee) 0x48fac` | `0x45bc0` | `0x47870` | `0x4794c` | `0x46030` |
| `bl` at 0x45508 | `(unknown callee) 0xc8e70` | `?` | `?` | `?` | `?` |
| `bl` at 0x45524 | `(unknown callee) 0x1e71c` | `ret(0xc8e70)` | `?` | `?` | `ret(0xc8e70)` |
| `bl` at 0x45560 | `(unknown callee) 0x1eed0` | `0x9` | `0xe2e3c` | `0xda` | `0x0` |
| `bl` at 0x45584 | `(unknown callee) 0x18920` | `?` | `?` | `?` | `?` |
| `bl` at 0x455b4 | `(unknown callee) 0x1eed0` | `0x9` | `0xe2e3c` | `0xe2` | `0x0` |
| `bl` at 0x455d0 | `(unknown callee) 0x18c20` | `[0x119e80]` | `0x0` | `?` | `?` |
| `bl` at 0x455f8 | `(unknown callee) 0x3a8c0` | `ret(0x18c20)` | `?` | `0x2811d0` | `[sp-32]:1` |
| `bl` at 0x45668 | `(unknown callee) 0xb7af8` | `?` | `sp-33` | `sp-33` | `?` |
| `bl` at 0x45684 | `(unknown callee) 0x1ea50` | `ret(0xb7af8)` | `?` | `?` | `ret(0xb7af8)` |
| `bl` at 0x456b0 | `(unknown callee) 0x1eed0` | `0x1` | `0xe2e3c` | `0xf3` | `0x0` |
| `bl` at 0x45728 | `(unknown callee) 0x46634` | `[sp-44]:1` | `?` | `[0x2811d8]:1` | `[sp-44]:1` |
| `bl` at 0x45744 | `(unknown callee) 0x1ea50` | `ret(0x46634)` | `?` | `?` | `ret(0x46634)` |
| `bl` at 0x45770 | `(unknown callee) 0x1eed0` | `0x1` | `0xe2e3c` | `0xfe` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x45470 | `[0x119e78] == 0x0` | `[0x119e78] != 0x0` |
| `blt` at 0x454d0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x4551c | `ret(0xc8e70) == 0x0` | `ret(0xc8e70) != 0x0` |
| `bne` at 0x45598 | `[0x119e80] != 0x0` | `[0x119e80] == 0x0` |
| `beq` at 0x45604 | `ret(0x3a8c0) == 0x0` | `ret(0x3a8c0) != 0x0` |
| `bge` at 0x4562c | `[sp-44] >= [0x2811d8]:1 (signed, 32-bit)` | `[sp-44] < [0x2811d8]:1 (signed, 32-bit)` |
| `bne` at 0x4563c | `[0x119e7c]:1 != 0x0` | `[0x119e7c]:1 == 0x0` |
| `beq` at 0x4567c | `ret(0xb7af8) == 0x0` | `ret(0xb7af8) != 0x0` |
| `bne` at 0x456d0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x456fc | `[0x119e7c]:1 == 0x0` | `[0x119e7c]:1 != 0x0` |
| `blt` at 0x45718 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x4573c | `ret(0x46634) == 0x0` | `ret(0x46634) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x45784 | `r0` | `?` |

## `TIOActive`

- `TIOActive`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x45bc0 to 0x45be0.
  Established. The body reads a byte at absolute `0x119e7c` (`ldrb` at 0x45bd0); its recorded exit
  carries `[0x119e7c]:1` in r0 (`ldmdb` at 0x45bd8). No calls or conditional branches intervene. The
  byte's relation to a physical link is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x45bcc | load | 4 | `abs 0x45bdc` | `-` |
| `ldrb` at 0x45bd0 | load | 1 | `abs 0x119e7c` | `-` |
| `ldmdb` at 0x45bd8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x45bd8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x45bd8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x45bd8 | `r0` | `[0x119e7c]:1` |

## `TIOSetLinkEnable`

- `TIOSetLinkEnable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x45be0 to 0x45c60.
  Established. A byte at absolute `0x119e7c` is tested (`ldrb` at 0x45c1c). Nonzero takes the branch
  and zero falls through (`bne` at 0x45c24). The call to `0x45c60` has candidates entry r0, entry
  r1, entry r2 and `[0x119e7c]:1` (`bl` at 0x45c40). The earlier branch compares an unknown value
  with `0x4`, following equality and falling through on inequality (`beq` at 0x45c08). Link-number
  interpretation remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x45bf0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x45bf4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x45bf8 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x45bfc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x45c10 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x45c18 | load | 4 | `abs 0x45c5c` | `-` |
| `ldrb` at 0x45c1c | load | 1 | `abs 0x119e7c` | `-` |
| `str` at 0x45c2c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x45c34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x45c38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45c3c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x45c48 | store | 4 | `sp -0x20` | `ret(0x45c60)` |
| `ldr` at 0x45c4c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x45c50 | store | 4 | `sp -0x24` | `ret(0x45c60)` |
| `ldr` at 0x45c54 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x45c58 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x45c58 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x45c58 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x45c40 | `(unknown callee) 0x45c60` | `r0` | `r1` | `r2` | `[0x119e7c]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x45c08 | `? == 0x4` | `? != 0x4` |
| `bne` at 0x45c24 | `[0x119e7c]:1 != 0x0` | `[0x119e7c]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x45c58 | `r0` | `?` |

## `tio_set_link_enable`

- `tio_set_link_enable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x45c60 to 0x45e44.
  Established. The first word test takes `[0x27eb98] == 0x0` and falls through for nonzero (`beq` at
  0x45c88). Calls include `0xcb020` with candidates `[sp-20]`, `[sp-24]+11`, `0x2` and `sp-32` (`bl`
  at 0x45d48), then `0xcb8f4` with `[sp-20]`, `0x3`, `[sp-24]+11` and `0x0` (`bl` at 0x45d88). The
  access tables retain stack writes used around those calls. The callee names and physical link
  transaction remain unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x45c70 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x45c74 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x45c78 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x45c7c | load | 4 | `abs 0x45e2c` | `-` |
| `ldr` at 0x45c80 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x45c8c | load | 4 | `abs 0x45e30` | `-` |
| `ldr` at 0x45c90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x45c94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45c98 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45ca0 | load | 4 | `abs 0x45e34` | `-` |
| `ldr` at 0x45ca4 | load | 4 | `abs 0x119f84` | `-` |
| `ldr` at 0x45cb0 | load | 4 | `abs 0x45e30` | `-` |
| `str` at 0x45cb4 | store | 4 | `sp -0x44` | `0xe2ecc` |
| `ldr` at 0x45cb8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x45cbc | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x45cc0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x45cc4 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x45cc8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x45ccc | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x45cd4 | load | 4 | `abs 0x45e38` | `-` |
| `ldr` at 0x45cd8 | load | 4 | `abs 0x45e3c` | `-` |
| `ldr` at 0x45ce4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x45cf4 | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0x45d00 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x45d04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45d10 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x45d28 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x45d2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45d38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x45d4c | load | 4 | `abs 0x45e2c` | `-` |
| `ldr` at 0x45d50 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x45d5c | load | 4 | `abs 0x45e40` | `-` |
| `ldr` at 0x45d60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45d64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45d70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45d78 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x45d90 | store | 4 | `sp -0x2c` | `ret(0xcb8f4)` |
| `ldr` at 0x45d98 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x45da4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x45da8 | store | 4 | `sp -0x34` | `[sp-44]` |
| `str` at 0x45db4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x45db8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45dc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x45ddc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45de4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x45dfc | store | 4 | `sp -0x2c` | `ret(0xcb8f4)` |
| `ldr` at 0x45e04 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x45e10 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x45e14 | store | 4 | `sp -0x34` | `[sp-44]` |
| `str` at 0x45e20 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x45e24 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x45e28 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x45e28 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x45e28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x45c9c | `(unknown callee) 0x20f14` | `0xe2ecc` | `r0` | `r1` | `r2` |
| `bl` at 0x45ce0 | `(unknown callee) 0x1eed0` | `0x7` | `0xe2e3c` | `0x1eb` | `0x0` |
| `bl` at 0x45d20 | `(unknown callee) 0xcb020` | `?` | `?` | `0x0` | `sp-32` |
| `bl` at 0x45d48 | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]+11` | `0x2` | `sp-32` |
| `bl` at 0x45d68 | `(unknown callee) 0x20f14` | `0xe2f04` | `[sp-24]` | `[sp-28]` | `[0x27eb98]` |
| `bl` at 0x45d6c | `(unknown callee) 0xc2c9c` | `?` | `?` | `?` | `?` |
| `bl` at 0x45d88 | `(unknown callee) 0xcb8f4` | `[sp-20]` | `0x3` | `[sp-24]+11` | `0x0` |
| `bl` at 0x45d94 | `(unknown callee) 0xc2d08` | `ret(0xcb8f4)` | `?` | `?` | `ret(0xcb8f4)` |
| `bl` at 0x45dd4 | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]+11` | `0x0` | `sp-32` |
| `bl` at 0x45dd8 | `(unknown callee) 0xc2c9c` | `ret(0xcb020)` | `?` | `?` | `?` |
| `bl` at 0x45df4 | `(unknown callee) 0xcb8f4` | `[sp-20]` | `0x3` | `[sp-24]+11` | `0x0` |
| `bl` at 0x45e00 | `(unknown callee) 0xc2d08` | `ret(0xcb8f4)` | `?` | `?` | `ret(0xcb8f4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x45c88 | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `beq` at 0x45cac | `[0x119f84] == 0x0` | `[0x119f84] != 0x0` |
| `bne` at 0x45cec | `Z == 0` | `not (Z == 0)` |
| `bls` at 0x45d58 | `[0x27eb98] <= 0x1 (unsigned, 32-bit)` | `[0x27eb98] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x45da0 | `[sp-44] >= 0x0 (signed, 32-bit)` | `[sp-44] < 0x0 (signed, 32-bit)` |
| `bge` at 0x45e0c | `[sp-44] >= 0x0 (signed, 32-bit)` | `[sp-44] < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x45e28 | `r0` | `?` |

## `pcixASICInit`

- `pcixASICInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x48fac to 0x49020.
  Established. Five word stores populate absolute `0x119e84`, `0x119e88`, `0x119e8c`, `0x119e90` and
  `0x119e94` with entry r0, `[sp-24]`, `[sp-28]`, `[sp-32]` and `[sp]`, respectively (`str` at
  0x48fd4, `str` at 0x48fe0, `str` at 0x48fec, `str` at 0x48ff8, `str` at 0x49004). This body has no
  recorded calls or conditional branches. Entry r0 survives at the exit (`ldmdb` at 0x49008).
  Whether the stored words are callbacks is not established by these stores alone.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x48fbc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x48fc0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x48fc4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x48fc8 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x48fcc | load | 4 | `abs 0x4900c` | `-` |
| `ldr` at 0x48fd0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x48fd4 | store | 4 | `abs 0x119e84` | `r0` |
| `ldr` at 0x48fd8 | load | 4 | `abs 0x49010` | `-` |
| `ldr` at 0x48fdc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x48fe0 | store | 4 | `abs 0x119e88` | `[sp-24]` |
| `ldr` at 0x48fe4 | load | 4 | `abs 0x49014` | `-` |
| `ldr` at 0x48fe8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x48fec | store | 4 | `abs 0x119e8c` | `[sp-28]` |
| `ldr` at 0x48ff0 | load | 4 | `abs 0x49018` | `-` |
| `ldr` at 0x48ff4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x48ff8 | store | 4 | `abs 0x119e90` | `[sp-32]` |
| `ldr` at 0x48ffc | load | 4 | `abs 0x4901c` | `-` |
| `ldr` at 0x49000 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x49004 | store | 4 | `abs 0x119e94` | `[sp]` |
| `ldmdb` at 0x49008 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x49008 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x49008 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x49008 | `r0` | `r0` |

## `pcixGetBusCfg`

- `pcixGetBusCfg`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x49068 to 0x490b0.
  Established. The zero-r0 branch bypasses a word store, while nonzero falls through (`beq` at
  0x49084). That store writes `[0x119e98]` at `r0 +0x0` (`str` at 0x49094). A later word read uses
  absolute `0x119e9c`, whose loaded value reaches the r0 exit (`ldr` at 0x4909c, `ldmdb` at
  0x490a4). Bus-configuration record meaning is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x49078 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x4907c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x49088 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4908c | load | 4 | `abs 0x490a8` | `-` |
| `ldr` at 0x49090 | load | 4 | `abs 0x119e98` | `-` |
| `str` at 0x49094 | store | 4 | `r0 +0x0` | `[0x119e98]` |
| `ldr` at 0x49098 | load | 4 | `abs 0x490ac` | `-` |
| `ldr` at 0x4909c | load | 4 | `abs 0x119e9c` | `-` |
| `ldmdb` at 0x490a4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x490a4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x490a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x49084 | `r0 == 0x0` | `r0 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x490a4 | `r0` | `[0x119e9c]` |

## `pcixSetBusSpeed`

- `pcixSetBusSpeed`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4a384 to 0x4a430.
  Established. Two word reads use absolute `0x119e9c` and absolute `0x119e98` (`ldr` at 0x4a3ac,
  `ldr` at 0x4a3d4). The call to `0x4bea0` has unknown candidate r0, `0x119e98` in candidate r1, and
  unknown r2 and r3 (`bl` at 0x4a3e0). The result comparison takes the branch for `ret(0x4bea0) ==
  0x0` and falls through for nonzero (`beq` at 0x4a3f4). A speed unit or register-setting effect is
  not proved by this body.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4a398 | store | 4 | `sp -0x14` | `0x0` |
| `str` at 0x4a3a0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x4a3a4 | load | 4 | `abs 0x4a428` | `-` |
| `ldr` at 0x4a3a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4a3ac | load | 4 | `abs 0x119e9c` | `-` |
| `ldr` at 0x4a3bc | load | 4 | `abs 0x4a42c` | `-` |
| `ldr` at 0x4a3c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4a3d4 | load | 4 | `abs 0x119e98` | `-` |
| `str` at 0x4a3e8 | store | 4 | `sp -0x18` | `ret(0x4bea0)` |
| `ldr` at 0x4a3ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4a3f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4a404 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4a408 | store | 4 | `sp -0x14` | `ret(0x4bea0)` |
| `ldr` at 0x4a40c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4a414 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x4a41c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x4a424 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4a424 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4a424 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4a3e0 | `(unknown callee) 0x4bea0` | `?` | `0x119e98` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x4a3b4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x4a3f4 | `ret(0x4bea0) == 0x0` | `ret(0x4bea0) != 0x0` |
| `bne` at 0x4a400 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4a424 | `r0` | `?` |

## `pcixBusPopulation`

- `pcixBusPopulation`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4a584 to 0x4a694.
  Established. The opening call targets `0x4b7e4` with candidates `(r0 & 0xff)`, entry r1, entry r2
  and `(r0 & 0xff)` (`bl` at 0x4a5ac); its result takes the branch for nonzero and falls through for
  zero (`bne` at 0x4a5c0). A later byte read uses `ret(0x4b7e4) +0x1` (`ldrb` at 0x4a5d4). Another
  call targets `0x4bccc` with unknown, `sp-32`, `sp-32` and unknown as candidates (`bl` at 0x4a620).
  Physical slot population is not independently established here.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4a598 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x4a5a0 | store | 4 | `sp -0x24` | `0x0` |
| `ldrb` at 0x4a5a4 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4a5b4 | store | 4 | `sp -0x18` | `ret(0x4b7e4)` |
| `ldr` at 0x4a5b8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4a5c8 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x4a5d0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4a5d4 | load | 1 | `ret(0x4b7e4) +0x1` | `-` |
| `str` at 0x4a5d8 | store | 4 | `sp -0x2c` | `[ret(0x4b7e4)+1]:1` |
| `ldr` at 0x4a5dc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4a5e0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x4a5e4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4a5e8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x4a5f0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4a600 | load | 4 | `abs 0x4a690` | `-` |
| `ldr` at 0x4a604 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4a60c | load | 4 | `abs 0x119ea4` | `-` |
| `str` at 0x4a628 | store | 4 | `sp -0x28` | `ret(0x4bccc)` |
| `ldr` at 0x4a62c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4a638 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x4a63c | store | 4 | `sp -0x30` | `[sp-36]` |
| `ldrb` at 0x4a650 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x4a664 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x4a66c | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x4a670 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x4a678 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x4a680 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x4a684 | store | 4 | `sp -0x30` | `[sp-36]` |
| `ldr` at 0x4a688 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0x4a68c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4a68c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4a68c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4a5ac | `(unknown callee) 0x4b7e4` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x4a620 | `(unknown callee) 0x4bccc` | `?` | `sp-32` | `sp-32` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x4a5c0 | `ret(0x4b7e4) != 0x0` | `ret(0x4b7e4) == 0x0` |
| `blt` at 0x4a5f8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x4a634 | `ret(0x4bccc) == 0x0` | `ret(0x4bccc) != 0x0` |
| `beq` at 0x4a660 | `? == 0x3` | `? != 0x3` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4a68c | `r0` | `?` |

## `SMPInit`

- `SMPInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x514a4 to 0x519c4.
  Established. Word stores include `0x0` at absolute `0x2815e0`, `0x3` at absolute `0x2815e4`, and
  `0x2815e8` at absolute `0x281638` (`str` at 0x5150c, `str` at 0x51518, `str` at 0x5152c). A byte
  store then clears `[0x281638] +0x0` (`strb` at 0x5153c). The first branch takes `[0x119fa0] ==
  0x0` and falls through for nonzero (`beq` at 0x514d0). The opening call to `0x2b31c` has entry r0
  through r3 as candidates (`bl` at 0x514b8). The SMP state fields are not assigned device meanings.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x514b4 | store | 4 | `sp -0x18` | `r0` |
| `strb` at 0x514c0 | store | 1 | `sp -0x25` | `(ret(0x2b31c) & 0xff)` |
| `ldr` at 0x514c4 | load | 4 | `abs 0x5192c` | `-` |
| `ldr` at 0x514c8 | load | 4 | `abs 0x119fa0` | `-` |
| `ldr` at 0x514d4 | load | 4 | `abs 0x51930` | `-` |
| `str` at 0x514d8 | store | 4 | `sp -0x44` | `0xe5a80` |
| `ldr` at 0x514e0 | load | 4 | `abs 0x51934` | `-` |
| `ldr` at 0x514f0 | load | 4 | `abs 0x5192c` | `-` |
| `ldr` at 0x514f4 | load | 4 | `abs 0x5192c` | `-` |
| `ldr` at 0x514f8 | load | 4 | `abs 0x119fa0` | `-` |
| `str` at 0x51500 | store | 4 | `abs 0x119fa0` | `[0x119fa0]+1` |
| `ldr` at 0x51504 | load | 4 | `abs 0x51938` | `-` |
| `str` at 0x5150c | store | 4 | `abs 0x2815e0` | `0x0` |
| `ldr` at 0x51510 | load | 4 | `abs 0x51938` | `-` |
| `str` at 0x51518 | store | 4 | `abs 0x2815e4` | `0x3` |
| `str` at 0x51520 | store | 4 | `sp -0x24` | `0x14` |
| `ldr` at 0x51524 | load | 4 | `abs 0x51938` | `-` |
| `ldr` at 0x51528 | load | 4 | `abs 0x5193c` | `-` |
| `str` at 0x5152c | store | 4 | `abs 0x281638` | `0x2815e8` |
| `ldr` at 0x51530 | load | 4 | `abs 0x51938` | `-` |
| `ldr` at 0x51534 | load | 4 | `abs 0x281638` | `-` |
| `strb` at 0x5153c | store | 1 | `[0x281638] +0x0` | `0x0` |
| `ldr` at 0x51540 | load | 4 | `abs 0x51940` | `-` |
| `ldrb` at 0x51544 | load | 1 | `sp -0x25` | `-` |
| `str` at 0x51548 | store | 4 | `abs 0x119fd8` | `[sp-37]:1` |
| `ldr` at 0x5154c | load | 4 | `abs 0x119fd8` | `-` |
| `ldr` at 0x5155c | load | 4 | `abs 0x51940` | `-` |
| `str` at 0x5156c | store | 4 | `abs 0x119fd8` | `?` |
| `ldr` at 0x51570 | load | 4 | `abs 0x51938` | `-` |
| `str` at 0x51578 | store | 4 | `abs 0x281c9c` | `0x0` |
| `ldr` at 0x5157c | load | 4 | `abs 0x51938` | `-` |
| `str` at 0x51584 | store | 4 | `abs 0x281ca0` | `0x0` |
| `ldr` at 0x51588 | load | 4 | `abs 0x51944` | `-` |
| `ldr` at 0x5158c | load | 4 | `abs 0x51948` | `-` |
| `str` at 0x5159c | store | 4 | `sp -0x1c` | `ret(0x9a8f8)` |
| `ldr` at 0x515a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x515ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x515b8 | load | 4 | `abs 0x5194c` | `-` |
| `str` at 0x515bc | store | 4 | `sp -0x44` | `0xe5ac0` |
| `str` at 0x515c0 | store | 4 | `sp -0x40` | `ret(0x1e71c)` |
| `ldr` at 0x515c8 | load | 4 | `abs 0x51934` | `-` |
| `ldr` at 0x515d8 | load | 4 | `abs 0x51950` | `-` |
| `ldr` at 0x515dc | load | 4 | `abs 0x51954` | `-` |
| `str` at 0x515ec | store | 4 | `sp -0x1c` | `ret(0x9a8f8)` |
| `ldr` at 0x515f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x515fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x51608 | load | 4 | `abs 0x51958` | `-` |
| `str` at 0x5160c | store | 4 | `sp -0x44` | `0xe5aec` |
| `str` at 0x51610 | store | 4 | `sp -0x40` | `ret(0x1e71c)` |
| `ldr` at 0x51618 | load | 4 | `abs 0x51934` | `-` |
| `ldr` at 0x51628 | load | 4 | `abs 0x5195c` | `-` |
| `ldr` at 0x5162c | load | 4 | `abs 0x51960` | `-` |
| `str` at 0x5163c | store | 4 | `sp -0x1c` | `ret(0x9a8f8)` |
| `ldr` at 0x51640 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5164c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x51658 | load | 4 | `abs 0x51964` | `-` |
| `str` at 0x5165c | store | 4 | `sp -0x44` | `0xe5b1c` |
| `str` at 0x51660 | store | 4 | `sp -0x40` | `ret(0x1e71c)` |
| `ldr` at 0x51668 | load | 4 | `abs 0x51934` | `-` |
| `ldr` at 0x51678 | load | 4 | `abs 0x51968` | `-` |
| `ldr` at 0x5167c | load | 4 | `abs 0x5196c` | `-` |
| `ldr` at 0x5168c | load | 4 | `abs 0x51970` | `-` |
| `ldr` at 0x51690 | load | 4 | `abs 0x51974` | `-` |
| `str` at 0x516a4 | store | 4 | `sp -0x44` | `0x100` |
| `str` at 0x516ac | store | 4 | `sp -0x40` | `0x0` |
| `str` at 0x516b4 | store | 4 | `sp -0x3c` | `0x1` |
| `str` at 0x516bc | store | 4 | `sp -0x38` | `0x10` |
| `str` at 0x516c4 | store | 4 | `sp -0x34` | `0x410` |
| `ldr` at 0x516c8 | load | 4 | `abs 0x51978` | `-` |
| `str` at 0x516cc | store | 4 | `sp -0x30` | `0x53954` |
| `str` at 0x516d4 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x516d8 | load | 4 | `abs 0x5197c` | `-` |
| `ldr` at 0x516dc | load | 4 | `abs 0x51980` | `-` |
| `ldr` at 0x516e4 | load | 4 | `abs 0x51984` | `-` |
| `ldr` at 0x516ec | load | 4 | `abs 0x51938` | `-` |
| `str` at 0x51708 | store | 4 | `sp -0x20` | `ret(0x5e7a8)` |
| `ldr` at 0x5170c | load | 4 | `abs 0x51988` | `-` |
| `ldr` at 0x51710 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x51714 | load | 4 | `ret(0x5e7a8) +0x8` | `-` |
| `str` at 0x51718 | store | 4 | `abs 0x11a064` | `[ret(0x5e7a8)+8]` |
| `ldr` at 0x5171c | load | 4 | `abs 0x5198c` | `-` |
| `ldr` at 0x51720 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x51724 | load | 4 | `[sp-32] +0xc` | `-` |
| `str` at 0x51728 | store | 4 | `abs 0x11a068` | `[[sp-32]+12]` |
| `ldr` at 0x5172c | load | 4 | `abs 0x51938` | `-` |
| `ldr` at 0x51730 | load | 4 | `abs 0x51938` | `-` |
| `ldr` at 0x51734 | load | 4 | `abs 0x281360` | `-` |
| `str` at 0x5174c | store | 4 | `abs 0x281ca4` | `?` |
| `ldr` at 0x51750 | load | 4 | `abs 0x51938` | `-` |
| `ldr` at 0x51754 | load | 4 | `abs 0x51938` | `-` |
| `ldr` at 0x51758 | load | 4 | `abs 0x281ca4` | `-` |
| `ldr` at 0x51760 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x51764 | load | 4 | `[sp-32] +0x14` | `-` |
| `str` at 0x51774 | store | 4 | `abs 0x281ca4` | `?` |
| `ldr` at 0x51778 | load | 4 | `abs 0x51938` | `-` |
| `ldr` at 0x5177c | load | 4 | `abs 0x51938` | `-` |
| `ldr` at 0x51780 | load | 4 | `abs 0x281360` | `-` |
| `str` at 0x51798 | store | 4 | `abs 0x281cac` | `?` |
| `ldr` at 0x517a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x517ac | load | 4 | `abs 0x51940` | `-` |
| `ldr` at 0x517b0 | load | 4 | `abs 0x119fd8` | `-` |
| `ldr` at 0x517bc | load | 4 | `abs 0x51940` | `-` |
| `ldr` at 0x517c0 | load | 4 | `abs 0x119fd8` | `-` |
| `str` at 0x517d0 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x517d4 | load | 4 | `abs 0x51990` | `-` |
| `str` at 0x517d8 | store | 4 | `sp -0x40` | `0xc18` |
| `str` at 0x517e0 | store | 4 | `sp -0x3c` | `0x14` |
| `str` at 0x517e8 | store | 4 | `sp -0x38` | `0xa` |
| `str` at 0x517f0 | store | 4 | `sp -0x34` | `0x9` |
| `ldr` at 0x517f4 | load | 4 | `abs 0x51994` | `-` |
| `ldr` at 0x517f8 | load | 4 | `abs 0x51998` | `-` |
| `ldr` at 0x517fc | load | 4 | `abs 0x5199c` | `-` |
| `str` at 0x5180c | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x51810 | load | 4 | `abs 0x51990` | `-` |
| `str` at 0x51814 | store | 4 | `sp -0x40` | `0xc18` |
| `str` at 0x5181c | store | 4 | `sp -0x3c` | `0x14` |
| `str` at 0x51824 | store | 4 | `sp -0x38` | `0x28` |
| `str` at 0x5182c | store | 4 | `sp -0x34` | `0x9` |
| `ldr` at 0x51830 | load | 4 | `abs 0x519a0` | `-` |
| `ldr` at 0x51834 | load | 4 | `abs 0x519a4` | `-` |
| `ldr` at 0x51838 | load | 4 | `abs 0x519a8` | `-` |
| `str` at 0x51848 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x5184c | load | 4 | `abs 0x51990` | `-` |
| `str` at 0x51850 | store | 4 | `sp -0x40` | `0xc18` |
| `str` at 0x51858 | store | 4 | `sp -0x3c` | `0x14` |
| `str` at 0x51860 | store | 4 | `sp -0x38` | `0xa` |
| `str` at 0x51868 | store | 4 | `sp -0x34` | `0x9` |
| `ldr` at 0x5186c | load | 4 | `abs 0x519ac` | `-` |
| `ldr` at 0x51870 | load | 4 | `abs 0x519b0` | `-` |
| `ldr` at 0x51874 | load | 4 | `abs 0x519b4` | `-` |
| `ldr` at 0x51880 | load | 4 | `abs 0x51938` | `-` |
| `ldr` at 0x51884 | load | 4 | `abs 0x519b8` | `-` |
| `ldr` at 0x5188c | load | 4 | `abs 0x5195c` | `-` |
| `str` at 0x51898 | store | 4 | `sp -0x1c` | `ret(0x99520)` |
| `ldr` at 0x5189c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x518a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x518b4 | load | 4 | `abs 0x519bc` | `-` |
| `str` at 0x518b8 | store | 4 | `sp -0x44` | `0xe5b7c` |
| `str` at 0x518bc | store | 4 | `sp -0x40` | `ret(0x1e71c)` |
| `ldr` at 0x518c4 | load | 4 | `abs 0x51934` | `-` |
| `ldr` at 0x518d4 | load | 4 | `abs 0x51938` | `-` |
| `ldr` at 0x518d8 | load | 4 | `abs 0x519b8` | `-` |
| `ldr` at 0x518e0 | load | 4 | `abs 0x51944` | `-` |
| `str` at 0x518ec | store | 4 | `sp -0x1c` | `ret(0x99520)` |
| `ldr` at 0x518f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x518fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x51908 | load | 4 | `abs 0x519c0` | `-` |
| `str` at 0x5190c | store | 4 | `sp -0x44` | `0xe5ba0` |
| `str` at 0x51910 | store | 4 | `sp -0x40` | `ret(0x1e71c)` |
| `ldr` at 0x51918 | load | 4 | `abs 0x51934` | `-` |
| `ldmdb` at 0x51928 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x51928 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x51928 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x51928 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x514b8 | `(unknown callee) 0x2b31c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x514ec | `(unknown callee) 0x1eed0` | `0x9` | `0xe5aa0` | `0x63` | `0x0` |
| `bl` at 0x51560 | `(unknown callee) 0x2b368` | `?` | `?` | `0x119fd8` | `[0x119fd8]-49` |
| `bl` at 0x51594 | `(unknown callee) 0x9a8f8` | `0x281370` | `0xe5ab4` | `0xa` | `0x0` |
| `bl` at 0x515b0 | `(unknown callee) 0x1e71c` | `ret(0x9a8f8)` | `?` | `?` | `ret(0x9a8f8)` |
| `bl` at 0x515d4 | `(unknown callee) 0x1eed0` | `0x9` | `0xe5aa0` | `0x85` | `0x0` |
| `bl` at 0x515e4 | `(unknown callee) 0x9a8f8` | `0x281404` | `0xe5ae0` | `0xa` | `?` |
| `bl` at 0x51600 | `(unknown callee) 0x1e71c` | `ret(0x9a8f8)` | `?` | `?` | `ret(0x9a8f8)` |
| `bl` at 0x51624 | `(unknown callee) 0x1eed0` | `0x9` | `0xe5aa0` | `0x88` | `0x0` |
| `bl` at 0x51634 | `(unknown callee) 0x9a8f8` | `0x281498` | `0xe5b10` | `0xa` | `?` |
| `bl` at 0x51650 | `(unknown callee) 0x1e71c` | `ret(0x9a8f8)` | `?` | `?` | `ret(0x9a8f8)` |
| `bl` at 0x51674 | `(unknown callee) 0x1eed0` | `0x9` | `0xe5aa0` | `0x8b` | `0x0` |
| `bl` at 0x51688 | `(unknown callee) 0x1f9e8` | `0x28152c` | `0xe5b40` | `0x1` | `0x9` |
| `bl` at 0x5169c | `(unknown callee) 0x1f9e8` | `0x281580` | `0xe5b4c` | `0x1` | `0x9` |
| `bl` at 0x516e8 | `(unknown callee) 0x9ed54` | `0x281cb0` | `0x281cec` | `0x100` | `0x281dec` |
| `bl` at 0x516f8 | `(unknown callee) 0x9a7e0` | `0x281360` | `0x2` | `0x0` | `?` |
| `bl` at 0x51700 | `(unknown callee) 0x5e7a8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x5179c | `(unknown callee) 0x53bdc` | `ret(0x5e7a8)` | `0x281360` | `0x281360` | `?` |
| `bl` at 0x51804 | `(unknown callee) 0x1f498` | `0x2815d4` | `0xe5b58` | `0x51aa0` | `0x0` |
| `bl` at 0x51840 | `(unknown callee) 0x1f498` | `0x2815d8` | `0xe5b64` | `0x51f88` | `0x0` |
| `bl` at 0x5187c | `(unknown callee) 0x1f498` | `0x2815dc` | `0xe5b70` | `0x52c78` | `0x0` |
| `bl` at 0x51890 | `(unknown callee) 0x99520` | `0x281360` | `0xffff002` | `0x1` | `0x281498` |
| `bl` at 0x518ac | `(unknown callee) 0x1e71c` | `ret(0x99520)` | `?` | `?` | `ret(0x99520)` |
| `bl` at 0x518d0 | `(unknown callee) 0x1eed0` | `0x9` | `0xe5aa0` | `0xd5` | `0x0` |
| `bl` at 0x518e4 | `(unknown callee) 0x99520` | `0x281360` | `0xffff002` | `0x2` | `0x281370` |
| `bl` at 0x51900 | `(unknown callee) 0x1e71c` | `ret(0x99520)` | `?` | `?` | `ret(0x99520)` |
| `bl` at 0x51924 | `(unknown callee) 0x1eed0` | `0x9` | `0xe5aa0` | `0xe1` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x514d0 | `[0x119fa0] == 0x0` | `[0x119fa0] != 0x0` |
| `bhi` at 0x51558 | `[0x119fd8]-49 > 0x1 (unsigned, 32-bit)` | `[0x119fd8]-49 <= 0x1 (unsigned, 32-bit)` |
| `beq` at 0x515a8 | `ret(0x9a8f8) == 0x0` | `ret(0x9a8f8) != 0x0` |
| `beq` at 0x515f8 | `ret(0x9a8f8) == 0x0` | `ret(0x9a8f8) != 0x0` |
| `beq` at 0x51648 | `ret(0x9a8f8) == 0x0` | `ret(0x9a8f8) != 0x0` |
| `bne` at 0x517a8 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x517b8 | `[0x119fd8] == 0x4e` | `[0x119fd8] != 0x4e` |
| `beq` at 0x517c8 | `[0x119fd8] == 0x26` | `[0x119fd8] != 0x26` |
| `beq` at 0x518a4 | `ret(0x99520) == 0x0` | `ret(0x99520) != 0x0` |
| `beq` at 0x518f8 | `ret(0x99520) == 0x0` | `ret(0x99520) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x51928 | `r0` | `?` |

## `SMPLock`

- `SMPLock`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x519c4 to 0x51a54.
  Established. The call to `0x20eac` carries entry r0 through r3 as candidates (`bl` at 0x519dc). A
  nonzero result takes the first branch and zero falls through (`bne` at 0x519f0). A later word
  store places `ret(0x20eac)` at absolute `0x119fa4` (`str` at 0x51a24). The call to `0xadf60` has
  candidates `[0x119fa4]`, `0x119fdc`, `0x119fa4` and `0x119fa4` (`bl` at 0x51a34). Lock ownership
  and failure semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x519d4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x519d8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x519e4 | store | 4 | `sp -0x18` | `ret(0x20eac)` |
| `ldr` at 0x519e8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x519f8 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x51a00 | load | 4 | `abs 0x51a48` | `-` |
| `ldr` at 0x51a04 | load | 4 | `abs 0x28136c` | `-` |
| `ldr` at 0x51a10 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x51a14 | store | 4 | `sp -0x1c` | `ret(0x20eac)` |
| `ldr` at 0x51a1c | load | 4 | `abs 0x51a4c` | `-` |
| `ldr` at 0x51a20 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x51a24 | store | 4 | `abs 0x119fa4` | `ret(0x20eac)` |
| `ldr` at 0x51a28 | load | 4 | `abs 0x51a4c` | `-` |
| `ldr` at 0x51a2c | load | 4 | `abs 0x119fa4` | `-` |
| `ldr` at 0x51a30 | load | 4 | `abs 0x51a50` | `-` |
| `ldr` at 0x51a38 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x51a3c | store | 4 | `sp -0x1c` | `[sp-24]` |
| `ldr` at 0x51a40 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x51a44 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x51a44 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x51a44 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x519dc | `(unknown callee) 0x20eac` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x51a34 | `(unknown callee) 0xadf60` | `[0x119fa4]` | `0x119fdc` | `0x119fa4` | `0x119fa4` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x519f0 | `ret(0x20eac) != 0x0` | `ret(0x20eac) == 0x0` |
| `bne` at 0x51a0c | `[0x28136c] != 0x0` | `[0x28136c] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x51a44 | `r0` | `?` |

## `SMPUnlock`

- `SMPUnlock`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x51a54 to 0x51aa0.
  Established. The word test branches for `[0x119fa4] != 0x0` and falls through for zero (`bne` at
  0x51a6c). On the nonzero arm, the call to `0xadf9c` carries `[0x119fa4]`, `0x119fdc`, entry r2 and
  `0x119fa4` as candidates (`bl` at 0x51a80). A word store clears absolute `0x119fa4` (`str` at
  0x51a8c), followed by the call to `0x20ef8` with candidates `ret(0xadf9c)`, unknown, `0x119fa4` and `0x0`
  (`bl` at 0x51a90). The synchronization contract is unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x51a60 | load | 4 | `abs 0x51a98` | `-` |
| `ldr` at 0x51a64 | load | 4 | `abs 0x119fa4` | `-` |
| `ldr` at 0x51a74 | load | 4 | `abs 0x51a98` | `-` |
| `ldr` at 0x51a78 | load | 4 | `abs 0x119fa4` | `-` |
| `ldr` at 0x51a7c | load | 4 | `abs 0x51a9c` | `-` |
| `ldr` at 0x51a84 | load | 4 | `abs 0x51a98` | `-` |
| `str` at 0x51a8c | store | 4 | `abs 0x119fa4` | `0x0` |
| `ldmdb` at 0x51a94 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x51a94 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x51a94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x51a80 | `(unknown callee) 0xadf9c` | `[0x119fa4]` | `0x119fdc` | `r2` | `0x119fa4` |
| `bl` at 0x51a90 | `(unknown callee) 0x20ef8` | `ret(0xadf9c)` | `?` | `0x119fa4` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x51a6c | `[0x119fa4] != 0x0` | `[0x119fa4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x51a94 | `r0` | `?` |

## `APwrInit`

- `APwrInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x974f4 to 0x97730.
  Established. The selector comparisons take branches for `[sp-28] == 0x31` and `[sp-28] == 0x32`,
  with inequality falling through at each decision (`beq` at 0x9751c, `beq` at 0x97528). Later calls
  include `0x1c79c` with candidates `0x32`, unknown, unknown and `[sp-28]` (`bl` at 0x97618). The
  final call targets `0x1f498` with candidates `0x28245c`, `0xfc810`, `0x97730` and `0x1` (`bl` at 0x97700).
  Power sequencing and selector product identities remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x97510 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x97514 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x97520 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x97530 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x9754c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x97550 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9755c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x97568 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x97574 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x97580 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9758c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x97598 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x975a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x975b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x975bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x975c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x975d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x975e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x975ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x975f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x97604 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9762c | load | 4 | `abs 0x97708` | `-` |
| `ldr` at 0x97654 | load | 4 | `abs 0x9770c` | `-` |
| `ldr` at 0x9765c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x97664 | load | 4 | `abs 0x97710` | `-` |
| `ldr` at 0x97668 | load | 4 | `abs 0x97714` | `-` |
| `str` at 0x97678 | store | 4 | `sp -0x18` | `ret(0x9a8f8)` |
| `ldr` at 0x9767c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x97688 | load | 4 | `abs 0x97718` | `-` |
| `ldr` at 0x97694 | load | 4 | `abs 0x97708` | `-` |
| `ldr` at 0x97698 | load | 4 | `abs 0x9771c` | `-` |
| `ldr` at 0x976a0 | load | 4 | `abs 0x97710` | `-` |
| `str` at 0x976ac | store | 4 | `sp -0x18` | `ret(0x99520)` |
| `ldr` at 0x976b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x976bc | load | 4 | `abs 0x97720` | `-` |
| `ldr` at 0x976c8 | load | 4 | `abs 0x97708` | `-` |
| `str` at 0x976cc | store | 4 | `sp -0x30` | `0x2823c0` |
| `str` at 0x976d4 | store | 4 | `sp -0x2c` | `0x4b0` |
| `str` at 0x976dc | store | 4 | `sp -0x28` | `0x19` |
| `str` at 0x976e4 | store | 4 | `sp -0x24` | `0xa` |
| `str` at 0x976ec | store | 4 | `sp -0x20` | `0x9` |
| `ldr` at 0x976f0 | load | 4 | `abs 0x97724` | `-` |
| `ldr` at 0x976f4 | load | 4 | `abs 0x97728` | `-` |
| `ldr` at 0x976f8 | load | 4 | `abs 0x9772c` | `-` |
| `ldmdb` at 0x97704 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x97704 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x97704 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x97704 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x97504 | `(unknown callee) 0x2b31c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x97540 | `(unknown callee) 0x2b368` | `ret(0x2b31c)` | `?` | `?` | `[sp-28]` |
| `bl` at 0x97618 | `(unknown callee) 0x1c79c` | `0x32` | `?` | `?` | `[sp-28]` |
| `bl` at 0x97638 | `(unknown callee) 0x9a7e0` | `0x2823c0` | `0x14` | `0x0` | `ret(0x1c79c)` |
| `bl` at 0x97640 | `(unknown callee) 0x1c79c` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x9764c | `(unknown callee) 0x1c79c` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x97660 | `(unknown callee) 0x54dd8` | `0x11b414` | `ret(0x1c79c)` | `[sp-28]` | `ret(0x1c79c)` |
| `bl` at 0x97670 | `(unknown callee) 0x9a8f8` | `0x2823c8` | `0xfc7ac` | `0x1` | `?` |
| `bl` at 0x9768c | `(unknown callee) 0x1dba4` | `0xfc7b8` | `?` | `?` | `ret(0x9a8f8)` |
| `bl` at 0x976a4 | `(unknown callee) 0x99520` | `0x2823c0` | `0xffff014` | `0x2` | `0x2823c8` |
| `bl` at 0x976c0 | `(unknown callee) 0x1dba4` | `0xfc7e4` | `?` | `?` | `ret(0x99520)` |
| `bl` at 0x97700 | `(unknown callee) 0x1f498` | `0x28245c` | `0xfc810` | `0x97730` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x9751c | `[sp-28] == 0x31` | `[sp-28] != 0x31` |
| `beq` at 0x97528 | `[sp-28] == 0x32` | `[sp-28] != 0x32` |
| `bne` at 0x97538 | `[sp-28] != 0x32` | `[sp-28] == 0x32` |
| `beq` at 0x97558 | `[sp-28] == 0x43` | `[sp-28] != 0x43` |
| `beq` at 0x97564 | `[sp-28] == 0x33` | `[sp-28] != 0x33` |
| `beq` at 0x97570 | `[sp-28] == 0x35` | `[sp-28] != 0x35` |
| `beq` at 0x9757c | `[sp-28] == 0x34` | `[sp-28] != 0x34` |
| `beq` at 0x97588 | `[sp-28] == 0x41` | `[sp-28] != 0x41` |
| `beq` at 0x97594 | `[sp-28] == 0x36` | `[sp-28] != 0x36` |
| `beq` at 0x975a0 | `[sp-28] == 0x39` | `[sp-28] != 0x39` |
| `beq` at 0x975ac | `[sp-28] == 0x37` | `[sp-28] != 0x37` |
| `beq` at 0x975b8 | `[sp-28] == 0x38` | `[sp-28] != 0x38` |
| `beq` at 0x975c4 | `[sp-28] == 0x30` | `[sp-28] != 0x30` |
| `beq` at 0x975d0 | `[sp-28] == 0x51` | `[sp-28] != 0x51` |
| `beq` at 0x975dc | `[sp-28] == 0x40` | `[sp-28] != 0x40` |
| `beq` at 0x975e8 | `[sp-28] == 0x4f` | `[sp-28] != 0x4f` |
| `beq` at 0x975f4 | `[sp-28] == 0x42` | `[sp-28] != 0x42` |
| `beq` at 0x97600 | `[sp-28] == 0x46` | `[sp-28] != 0x46` |
| `beq` at 0x9760c | `[sp-28] == 0x4c` | `[sp-28] != 0x4c` |
| `bne` at 0x97624 | `ret(0x1c79c) != 0x0` | `ret(0x1c79c) == 0x0` |
| `beq` at 0x97684 | `ret(0x9a8f8) == 0x0` | `ret(0x9a8f8) != 0x0` |
| `beq` at 0x976b8 | `ret(0x99520) == 0x0` | `ret(0x99520) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x97704 | `r0` | `?` |
