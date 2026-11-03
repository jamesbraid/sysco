# ARM Linux 2.6 configuration discovery and ports in 1.44.0

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
register locations r0 through r3 with unknown arity. Stack stores appear in the access tables.
Unknown values and predicates include unsupported instruction effects and merged paths.
Numeric call targets have unknown callee names in the flat-image analysis. Device roles
and pointee meanings remain unknown unless the body establishes them. Tables include the
explicit memory accesses reported on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Separate loads at the same printed address need not observe the same value.
No unresolved computed transfers were reported for these routines.
No undecodable byte ranges were reported in these spans.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-configuration-discovery-ports-facts.txt) retain all tool sections.

## `L1CfgSetBits`

- `L1CfgSetBits`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x992a4 to 0x99350.
  Established. A word load reads absolute 0x1a6924 and saves the value at sp-0x1c (`ldr` at 0x992c0, `str` at 0x992c4).
  A later word store writes an unknown value to that same absolute address (`str` at 0x9930c).
  The call to unnamed 0x9a0d4 has candidate values 0x1a6900 twice, the word at 0x1a6924 and unknown, without establishing hardware bit meanings (`bl` at 0x9933c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x992b4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x992b8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x992bc | load | 4 | `abs 0x9934c` | `-` |
| `ldr` at 0x992c0 | load | 4 | `abs 0x1a6924` | `-` |
| `str` at 0x992c4 | store | 4 | `sp -0x1c` | `[0x1a6924]` |
| `ldr` at 0x992c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x992d4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x992d8 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x992dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x992e0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x992e8 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0x992ec | load | 4 | `abs 0x9934c` | `-` |
| `ldr` at 0x992f0 | load | 4 | `abs 0x9934c` | `-` |
| `ldr` at 0x992f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x992fc | load | 4 | `abs 0x1a6924` | `-` |
| `ldr` at 0x99304 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9930c | store | 4 | `abs 0x1a6924` | `?` |
| `ldr` at 0x99310 | load | 4 | `abs 0x9934c` | `-` |
| `ldr` at 0x99314 | load | 4 | `abs 0x1a6924` | `-` |
| `ldr` at 0x99318 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x99324 | load | 4 | `abs 0x9934c` | `-` |
| `ldr` at 0x99328 | load | 4 | `abs 0x1a6924` | `-` |
| `ldr` at 0x9932c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x99348 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x99348 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x99348 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9933c | `(unknown callee) 0x9a0d4` | `0x1a6900` | `0x1a6900` | `[0x1a6924]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x992d0 | `r1 != 0x0` | `r1 == 0x0` |
| `beq` at 0x99320 | `[0x1a6924] == [sp-28]` | `[0x1a6924] != [sp-28]` |
| `beq` at 0x99338 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x99348 | `r0` | `?` |

## `L1CfgInit`

- `L1CfgInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x99350 to 0x99e9c.
  Established. Five word stores to absolute 0x1a6628 carry 0x100, one, 0x10, 0x1c0 and zero in instruction order (`str` at 0x99420, `str` at 0x9942c, `str` at 0x99438, `str` at 0x99444, `str` at 0x99450).
  Another word store puts 0x25 at absolute 0x1a6984 (`str` at 0x994fc).
  Two calls to unnamed 0x22540 have candidate values 0x1a68f4, 0x16a700, 0x9a350 or 0x9a138, and zero (`bl` at 0x99d50, `bl` at 0x99d90).
  The address values and software initialization stores do not establish a machine topology.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9936c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x99370 | load | 4 | `abs 0x99e34` | `-` |
| `ldr` at 0x99374 | load | 4 | `abs 0x1a6624` | `-` |
| `ldr` at 0x99380 | load | 4 | `abs 0x99e38` | `-` |
| `str` at 0x99384 | store | 4 | `sp -0x5c` | `0x16a6c0` |
| `ldr` at 0x9938c | load | 4 | `abs 0x99e3c` | `-` |
| `ldr` at 0x9939c | load | 4 | `abs 0x99e34` | `-` |
| `ldr` at 0x993a0 | load | 4 | `abs 0x99e34` | `-` |
| `ldr` at 0x993a4 | load | 4 | `abs 0x1a6624` | `-` |
| `str` at 0x993ac | store | 4 | `abs 0x1a6624` | `[0x1a6624]+1` |
| `ldr` at 0x993b0 | load | 4 | `abs 0x99e40` | `-` |
| `str` at 0x993bc | store | 4 | `abs 0x1a663c` | `ret(0x202c8)` |
| `str` at 0x993d8 | store | 4 | `sp -0x24` | `ret(0x2031c)` |
| `ldr` at 0x993dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x99408 | load | 4 | `abs 0x99e44` | `-` |
| `ldr` at 0x99418 | load | 4 | `abs 0x99e48` | `-` |
| `str` at 0x99420 | store | 4 | `abs 0x1a6628` | `0x100` |
| `ldr` at 0x99424 | load | 4 | `abs 0x99e48` | `-` |
| `str` at 0x9942c | store | 4 | `abs 0x1a6628` | `0x1` |
| `ldr` at 0x99430 | load | 4 | `abs 0x99e48` | `-` |
| `str` at 0x99438 | store | 4 | `abs 0x1a6628` | `0x10` |
| `ldr` at 0x9943c | load | 4 | `abs 0x99e48` | `-` |
| `str` at 0x99444 | store | 4 | `abs 0x1a6628` | `0x1c0` |
| `ldr` at 0x99448 | load | 4 | `abs 0x99e48` | `-` |
| `str` at 0x99450 | store | 4 | `abs 0x1a6628` | `0x0` |
| `ldr` at 0x99454 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x99458 | store | 4 | `sp -0x3c` | `[sp-40]` |
| `ldr` at 0x9945c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x99468 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x99478 | load | 4 | `abs 0x99e4c` | `-` |
| `str` at 0x99480 | store | 4 | `abs 0x1a0350` | `0xc` |
| `ldr` at 0x99484 | load | 4 | `abs 0x99e50` | `-` |
| `str` at 0x9948c | store | 4 | `abs 0x1a6990` | `0x10` |
| `ldr` at 0x99490 | load | 4 | `abs 0x99e50` | `-` |
| `str` at 0x99498 | store | 4 | `abs 0x1a698c` | `0x0` |
| `ldr` at 0x9949c | load | 4 | `abs 0x99e50` | `-` |
| `ldr` at 0x994a0 | load | 4 | `abs 0x99e50` | `-` |
| `ldr` at 0x994a4 | load | 4 | `abs 0x1a6990` | `-` |
| `str` at 0x994c4 | store | 4 | `abs 0x1a6994` | `ret(0x201fc)` |
| `ldr` at 0x994c8 | load | 4 | `abs 0x99e50` | `-` |
| `ldr` at 0x994cc | load | 4 | `abs 0x99e50` | `-` |
| `ldr` at 0x994d0 | load | 4 | `abs 0x1a6990` | `-` |
| `ldr` at 0x994e4 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x994f4 | load | 4 | `abs 0x99e50` | `-` |
| `str` at 0x994fc | store | 4 | `abs 0x1a6984` | `0x25` |
| `ldr` at 0x99500 | load | 4 | `abs 0x99e50` | `-` |
| `ldr` at 0x99504 | load | 4 | `abs 0x99e50` | `-` |
| `ldr` at 0x99508 | load | 4 | `abs 0x1a6984` | `-` |
| `str` at 0x99530 | store | 4 | `abs 0x1a6988` | `ret(0x201fc)` |
| `ldr` at 0x99534 | load | 4 | `abs 0x99e50` | `-` |
| `ldr` at 0x99538 | load | 4 | `abs 0x99e50` | `-` |
| `ldr` at 0x9953c | load | 4 | `abs 0x1a6984` | `-` |
| `ldr` at 0x99558 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x99568 | load | 4 | `abs 0x99e50` | `-` |
| `str` at 0x99570 | store | 4 | `abs 0x1a6900` | `0x1` |
| `ldr` at 0x99574 | load | 4 | `abs 0x99e50` | `-` |
| `str` at 0x9957c | store | 4 | `abs 0x1a6960` | `0x2` |
| `ldr` at 0x99580 | load | 4 | `abs 0x99e50` | `-` |
| `ldr` at 0x99584 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x99588 | store | 4 | `abs 0x1a6910` | `[sp-40]` |
| `ldr` at 0x9958c | load | 4 | `abs 0x99e50` | `-` |
| `str` at 0x99598 | store | 4 | `abs 0x1a6908` | `ret(0x28230)` |
| `ldr` at 0x9959c | load | 4 | `abs 0x99e50` | `-` |
| `str` at 0x995a8 | store | 4 | `abs 0x1a690c` | `ret(0x28230)` |
| `ldr` at 0x995ac | load | 4 | `abs 0x99e50` | `-` |
| `str` at 0x995b8 | store | 4 | `abs 0x1a6928` | `ret(0x28230)` |
| `ldr` at 0x995bc | load | 4 | `abs 0x99e50` | `-` |
| `strb` at 0x995cc | store | 1 | `abs 0x1a691c` | `(ret(0x28230) & 0xff)` |
| `ldr` at 0x995d0 | load | 4 | `abs 0x99e50` | `-` |
| `strb` at 0x995e0 | store | 1 | `abs 0x1a691d` | `(ret(0x28230) & 0xff)` |
| `ldr` at 0x995e4 | load | 4 | `abs 0x99e50` | `-` |
| `ldr` at 0x995e8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x995f0 | store | 4 | `abs 0x1a6970` | `ret(0xa3f14)` |
| `ldr` at 0x995f4 | load | 4 | `abs 0x99e50` | `-` |
| `str` at 0x99600 | store | 4 | `abs 0x1a693c` | `ret(0x28230)` |
| `ldr` at 0x99604 | load | 4 | `abs 0x1a693c` | `-` |
| `ldr` at 0x99610 | load | 4 | `abs 0x99e50` | `-` |
| `str` at 0x99618 | store | 4 | `abs 0x1a693c` | `0x0` |
| `ldr` at 0x99634 | load | 4 | `abs 0x99e54` | `-` |
| `ldr` at 0x99640 | load | 4 | `abs 0x99e58` | `-` |
| `ldr` at 0x99644 | load | 4 | `abs 0x99e44` | `-` |
| `str` at 0x99668 | store | 4 | `sp -0x30` | `?` |
| `str` at 0x99688 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x9969c | load | 4 | `sp -0x34` | `-` |
| `str` at 0x996a4 | store | 4 | `sp -0x40` | `ret(0x3e458)` |
| `ldr` at 0x996ac | load | 4 | `sp -0x30` | `-` |
| `str` at 0x996b0 | store | 4 | `sp -0x40` | `[sp-48]` |
| `ldr` at 0x996c4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x996cc | store | 4 | `sp -0x44` | `ret(0x3e498)` |
| `ldr` at 0x996d0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x996d8 | store | 4 | `sp -0x44` | `ret(0x3e498)+2` |
| `ldr` at 0x996e0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x996e8 | store | 4 | `sp -0x44` | `[sp-52]+2` |
| `ldr` at 0x996ec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x996f0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x996fc | store | 4 | `sp -0x38` | `ret(0x81b18)` |
| `ldr` at 0x99700 | load | 4 | `abs 0x99e50` | `-` |
| `ldr` at 0x99704 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x9970c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x99710 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x99714 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x99724 | store | 4 | `abs 0x1a6914` | `?` |
| `ldr` at 0x99728 | load | 4 | `abs 0x99e50` | `-` |
| `str` at 0x99730 | store | 4 | `abs 0x1a6918` | `0x0` |
| `ldr` at 0x99734 | load | 4 | `abs 0x99e50` | `-` |
| `str` at 0x9973c | store | 4 | `abs 0x1a692c` | `0x0` |
| `ldr` at 0x99740 | load | 4 | `abs 0x99e50` | `-` |
| `str` at 0x99748 | store | 4 | `abs 0x1a6934` | `ret(0x5572c)` |
| `ldr` at 0x9974c | load | 4 | `abs 0x99e50` | `-` |
| `str` at 0x99754 | store | 4 | `abs 0x1a6938` | `ret(0x556e0)` |
| `ldr` at 0x99758 | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x99760 | store | 4 | `abs 0x1a69ac` | `0x1` |
| `ldr` at 0x99764 | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x9976c | store | 4 | `abs 0x1a69d8` | `0x2` |
| `ldr` at 0x99770 | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x99778 | store | 4 | `abs 0x1a69f4` | `0x3` |
| `ldr` at 0x9977c | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x99784 | store | 4 | `abs 0x1a69f8` | `0x4` |
| `ldr` at 0x99788 | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x99790 | store | 4 | `abs 0x1a69fc` | `0x5` |
| `ldr` at 0x99794 | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x9979c | store | 4 | `abs 0x1a6a00` | `0x6` |
| `ldr` at 0x997a0 | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x997a8 | store | 4 | `abs 0x1a6a04` | `0x7` |
| `ldr` at 0x997ac | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x997b4 | store | 4 | `abs 0x1a6a08` | `0x8` |
| `ldr` at 0x997b8 | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x997c0 | store | 4 | `abs 0x1a6a0c` | `0x9` |
| `ldr` at 0x997c4 | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x997cc | store | 4 | `abs 0x1a6a10` | `0xa` |
| `ldr` at 0x997d0 | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x997d8 | store | 4 | `abs 0x1a6a14` | `0xb` |
| `ldr` at 0x997dc | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x997e4 | store | 4 | `abs 0x1a6a18` | `0xc` |
| `ldr` at 0x997e8 | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x997f0 | store | 4 | `abs 0x1a6a1c` | `0xd` |
| `ldr` at 0x997f4 | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x997fc | store | 4 | `abs 0x1a6a20` | `0xe` |
| `ldr` at 0x99800 | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x99808 | store | 4 | `abs 0x1a6a24` | `0xf` |
| `ldr` at 0x9980c | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x99814 | store | 4 | `abs 0x1a6a28` | `0x10` |
| `ldr` at 0x99818 | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x99820 | store | 4 | `abs 0x1a6a2c` | `0x11` |
| `ldr` at 0x99824 | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x9982c | store | 4 | `abs 0x1a6a30` | `0x12` |
| `ldr` at 0x99830 | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x99838 | store | 4 | `abs 0x1a6a34` | `0x13` |
| `ldr` at 0x9983c | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x99844 | store | 4 | `abs 0x1a6a38` | `0x14` |
| `ldr` at 0x99848 | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x99850 | store | 4 | `abs 0x1a6a3c` | `0x15` |
| `ldr` at 0x99854 | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x9985c | store | 4 | `abs 0x1a6a40` | `0x16` |
| `ldr` at 0x99860 | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x99868 | store | 4 | `abs 0x1a6a44` | `0x17` |
| `ldr` at 0x9986c | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x99874 | store | 4 | `abs 0x1a6a48` | `0x18` |
| `ldr` at 0x99878 | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x99880 | store | 4 | `abs 0x1a6a4c` | `0x19` |
| `ldr` at 0x99884 | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x9988c | store | 4 | `abs 0x1a6a50` | `0x1a` |
| `ldr` at 0x99890 | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x99898 | store | 4 | `abs 0x1a6a54` | `0x1b` |
| `ldr` at 0x9989c | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x998a4 | store | 4 | `abs 0x1a6a58` | `0x1c` |
| `ldr` at 0x998a8 | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x998b0 | store | 4 | `abs 0x1a6a5c` | `0x1d` |
| `ldr` at 0x998b4 | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x998bc | store | 4 | `abs 0x1a6a60` | `0x1e` |
| `ldr` at 0x998c0 | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x998c8 | store | 4 | `abs 0x1a6a64` | `0x1f` |
| `ldr` at 0x998cc | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x998d4 | store | 4 | `abs 0x1a6a68` | `0x20` |
| `ldr` at 0x998d8 | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x998e0 | store | 4 | `abs 0x1a69a8` | `0x21` |
| `ldr` at 0x998e4 | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x998ec | store | 4 | `abs 0x1a69b0` | `0x22` |
| `ldr` at 0x998f0 | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x998f8 | store | 4 | `abs 0x1a69dc` | `0x23` |
| `ldr` at 0x998fc | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x99904 | store | 4 | `abs 0x1a69e0` | `0x24` |
| `ldr` at 0x99908 | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x99910 | store | 4 | `abs 0x1a69b4` | `0x21` |
| `ldr` at 0x99914 | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x9991c | store | 4 | `abs 0x1a69b8` | `0x22` |
| `ldr` at 0x99920 | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x99928 | store | 4 | `abs 0x1a69d4` | `0x23` |
| `ldr` at 0x9992c | load | 4 | `abs 0x99e5c` | `-` |
| `str` at 0x99934 | store | 4 | `abs 0x1a69c8` | `0x24` |
| `ldr` at 0x99938 | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x99940 | store | 4 | `abs 0x1a6ba8` | `0x2` |
| `ldr` at 0x99944 | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x9994c | store | 4 | `abs 0x1a6bac` | `0xd` |
| `ldr` at 0x99950 | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x99958 | store | 4 | `abs 0x1a6bb0` | `0x14` |
| `ldr` at 0x9995c | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x99964 | store | 4 | `abs 0x1a6bb4` | `0x15` |
| `ldr` at 0x99968 | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x99970 | store | 4 | `abs 0x1a6bb8` | `0x16` |
| `ldr` at 0x99974 | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x9997c | store | 4 | `abs 0x1a6bbc` | `0x17` |
| `ldr` at 0x99980 | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x99988 | store | 4 | `abs 0x1a6bc0` | `0x18` |
| `ldr` at 0x9998c | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x99994 | store | 4 | `abs 0x1a6bc4` | `0x19` |
| `ldr` at 0x99998 | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x999a0 | store | 4 | `abs 0x1a6bc8` | `0x1a` |
| `ldr` at 0x999a4 | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x999ac | store | 4 | `abs 0x1a6bcc` | `0x1b` |
| `ldr` at 0x999b0 | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x999b8 | store | 4 | `abs 0x1a6bd0` | `0x1c` |
| `ldr` at 0x999bc | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x999c4 | store | 4 | `abs 0x1a6bd4` | `0x1d` |
| `ldr` at 0x999c8 | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x999d0 | store | 4 | `abs 0x1a6bd8` | `0x1e` |
| `ldr` at 0x999d4 | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x999dc | store | 4 | `abs 0x1a6bdc` | `0x1f` |
| `ldr` at 0x999e0 | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x999e8 | store | 4 | `abs 0x1a6be0` | `0x20` |
| `ldr` at 0x999ec | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x999f4 | store | 4 | `abs 0x1a6be4` | `0x21` |
| `ldr` at 0x999f8 | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x99a00 | store | 4 | `abs 0x1a6be8` | `0x22` |
| `ldr` at 0x99a04 | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x99a0c | store | 4 | `abs 0x1a6bec` | `0x23` |
| `ldr` at 0x99a10 | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x99a18 | store | 4 | `abs 0x1a6bf0` | `0x24` |
| `ldr` at 0x99a1c | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x99a24 | store | 4 | `abs 0x1a6bf4` | `0x25` |
| `ldr` at 0x99a28 | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x99a30 | store | 4 | `abs 0x1a6bf8` | `0x26` |
| `ldr` at 0x99a34 | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x99a3c | store | 4 | `abs 0x1a6bfc` | `0x27` |
| `ldr` at 0x99a40 | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x99a48 | store | 4 | `abs 0x1a6c00` | `0x28` |
| `ldr` at 0x99a4c | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x99a54 | store | 4 | `abs 0x1a6c04` | `0x29` |
| `ldr` at 0x99a58 | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x99a60 | store | 4 | `abs 0x1a6c08` | `0x2a` |
| `ldr` at 0x99a64 | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x99a6c | store | 4 | `abs 0x1a6c0c` | `0x2b` |
| `ldr` at 0x99a70 | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x99a78 | store | 4 | `abs 0x1a6c10` | `0x2c` |
| `ldr` at 0x99a7c | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x99a84 | store | 4 | `abs 0x1a6c14` | `0x2d` |
| `ldr` at 0x99a88 | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x99a90 | store | 4 | `abs 0x1a6c18` | `0x2e` |
| `ldr` at 0x99a94 | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x99a9c | store | 4 | `abs 0x1a6c1c` | `0x2f` |
| `ldr` at 0x99aa0 | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x99aa8 | store | 4 | `abs 0x1a6c20` | `0x30` |
| `ldr` at 0x99aac | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x99ab4 | store | 4 | `abs 0x1a6c24` | `0x31` |
| `ldr` at 0x99ab8 | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x99ac0 | store | 4 | `abs 0x1a6c28` | `0x1` |
| `ldr` at 0x99ac4 | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x99acc | store | 4 | `abs 0x1a6c2c` | `0x3` |
| `ldr` at 0x99ad0 | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x99ad8 | store | 4 | `abs 0x1a6c30` | `0xe` |
| `ldr` at 0x99adc | load | 4 | `abs 0x99e60` | `-` |
| `str` at 0x99ae4 | store | 4 | `abs 0x1a6c34` | `0xf` |
| `ldr` at 0x99ae8 | load | 4 | `abs 0x99e50` | `-` |
| `ldr` at 0x99aec | load | 4 | `abs 0x1a6910` | `-` |
| `str` at 0x99af0 | store | 4 | `sp -0x48` | `[0x1a6910]` |
| `ldr` at 0x99af4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x99b00 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x99b0c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x99b18 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x99b24 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x99b34 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x99b40 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x99b4c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x99b5c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x99b6c | load | 4 | `abs 0x99e50` | `-` |
| `ldr` at 0x99b70 | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0x99b7c | load | 4 | `abs 0x99e50` | `-` |
| `ldr` at 0x99b80 | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0x99b8c | load | 4 | `abs 0x99e50` | `-` |
| `ldr` at 0x99b90 | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0x99ba0 | load | 4 | `abs 0x99e50` | `-` |
| `ldr` at 0x99ba4 | load | 4 | `abs 0x1a693c` | `-` |
| `ldr` at 0x99bb0 | load | 4 | `abs 0x99e50` | `-` |
| `str` at 0x99bb8 | store | 4 | `abs 0x1a693c` | `0x0` |
| `ldr` at 0x99bc8 | load | 4 | `abs 0x99e50` | `-` |
| `str` at 0x99bd4 | store | 4 | `abs 0x1a692c` | `ret(0x28230)` |
| `ldr` at 0x99bd8 | load | 4 | `abs 0x1a692c` | `-` |
| `str` at 0x99bdc | store | 4 | `sp -0x2c` | `[0x1a692c]` |
| `ldr` at 0x99be0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x99bf0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x99bf8 | load | 4 | `abs 0x99e64` | `-` |
| `str` at 0x99c0c | store | 4 | `sp -0x2c` | `0xf` |
| `ldr` at 0x99c10 | load | 4 | `abs 0x99e50` | `-` |
| `ldr` at 0x99c14 | load | 4 | `abs 0x1a692c` | `-` |
| `ldr` at 0x99c18 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x99c24 | load | 4 | `abs 0x99e50` | `-` |
| `ldr` at 0x99c28 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x99c2c | store | 4 | `abs 0x1a692c` | `?` |
| `ldr` at 0x99c40 | load | 4 | `abs 0x99e68` | `-` |
| `ldr` at 0x99c44 | load | 4 | `abs 0x99e6c` | `-` |
| `ldr` at 0x99c54 | load | 4 | `abs 0x99e70` | `-` |
| `ldr` at 0x99c64 | load | 4 | `abs 0x99e50` | `-` |
| `ldr` at 0x99c68 | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0x99c74 | load | 4 | `abs 0x99e50` | `-` |
| `ldr` at 0x99c78 | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0x99c84 | load | 4 | `abs 0x99e50` | `-` |
| `ldr` at 0x99c88 | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0x99c94 | load | 4 | `abs 0x99e50` | `-` |
| `ldr` at 0x99c98 | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0x99ca4 | load | 4 | `abs 0x99e50` | `-` |
| `ldr` at 0x99ca8 | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0x99cb4 | load | 4 | `abs 0x99e50` | `-` |
| `ldr` at 0x99cb8 | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0x99cc4 | load | 4 | `abs 0x99e50` | `-` |
| `ldr` at 0x99cc8 | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0x99cd4 | load | 4 | `abs 0x99e50` | `-` |
| `ldr` at 0x99cd8 | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0x99ce4 | load | 4 | `abs 0x99e50` | `-` |
| `ldr` at 0x99ce8 | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0x99cf4 | load | 4 | `abs 0x99e50` | `-` |
| `ldr` at 0x99cf8 | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0x99d04 | load | 4 | `abs 0x99e50` | `-` |
| `ldr` at 0x99d08 | load | 4 | `abs 0x1a6910` | `-` |
| `str` at 0x99d1c | store | 4 | `sp -0x5c` | `0x0` |
| `str` at 0x99d24 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0x99d2c | store | 4 | `sp -0x54` | `0x14` |
| `str` at 0x99d34 | store | 4 | `sp -0x50` | `0xa` |
| `str` at 0x99d3c | store | 4 | `sp -0x4c` | `0x9` |
| `ldr` at 0x99d40 | load | 4 | `abs 0x99e74` | `-` |
| `ldr` at 0x99d44 | load | 4 | `abs 0x99e78` | `-` |
| `ldr` at 0x99d48 | load | 4 | `abs 0x99e7c` | `-` |
| `str` at 0x99d5c | store | 4 | `sp -0x5c` | `0x0` |
| `str` at 0x99d64 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0x99d6c | store | 4 | `sp -0x54` | `0x14` |
| `str` at 0x99d74 | store | 4 | `sp -0x50` | `0xa` |
| `str` at 0x99d7c | store | 4 | `sp -0x4c` | `0x9` |
| `ldr` at 0x99d80 | load | 4 | `abs 0x99e74` | `-` |
| `ldr` at 0x99d84 | load | 4 | `abs 0x99e78` | `-` |
| `ldr` at 0x99d88 | load | 4 | `abs 0x99e80` | `-` |
| `ldr` at 0x99d94 | load | 4 | `abs 0x99e70` | `-` |
| `ldr` at 0x99d98 | load | 4 | `abs 0x99e84` | `-` |
| `ldr` at 0x99da0 | load | 4 | `abs 0x99e88` | `-` |
| `str` at 0x99dac | store | 4 | `sp -0x20` | `ret(0xfa1b8)` |
| `ldr` at 0x99db0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x99dbc | load | 4 | `abs 0x99e8c` | `-` |
| `str` at 0x99dc0 | store | 4 | `sp -0x5c` | `0x16a70c` |
| `ldr` at 0x99dc4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x99dc8 | store | 4 | `sp -0x58` | `ret(0xfa1b8)` |
| `ldr` at 0x99dd0 | load | 4 | `abs 0x99e3c` | `-` |
| `ldr` at 0x99dd4 | load | 4 | `abs 0x99e90` | `-` |
| `ldr` at 0x99de0 | load | 4 | `abs 0x99e70` | `-` |
| `ldr` at 0x99dec | load | 4 | `abs 0x99e88` | `-` |
| `str` at 0x99df8 | store | 4 | `sp -0x20` | `ret(0xfa64c)` |
| `ldr` at 0x99dfc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x99e08 | load | 4 | `abs 0x99e94` | `-` |
| `str` at 0x99e0c | store | 4 | `sp -0x5c` | `0x16a738` |
| `ldr` at 0x99e10 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x99e14 | store | 4 | `sp -0x58` | `ret(0xfa64c)` |
| `ldr` at 0x99e1c | load | 4 | `abs 0x99e3c` | `-` |
| `ldr` at 0x99e20 | load | 4 | `abs 0x99e98` | `-` |
| `ldm` at 0x99e30 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x99e30 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x99e30 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x99e30 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x99e30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x99360 | `(unknown callee) 0x6dc0c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x99398 | `(unknown callee) 0x22070` | `0x9` | `0x16a6dc` | `0x88` | `0x0` |
| `bl` at 0x993b8 | `(unknown callee) 0x202c8` | `0x0` | `?` | `0x1a6624` | `[0x1a6624]+1` |
| `bl` at 0x993c4 | `(unknown callee) 0x28230` | `0xf` | `?` | `?` | `?` |
| `bl` at 0x993d0 | `(unknown callee) 0x2031c` | `ret(0x28230)` | `?` | `?` | `ret(0x28230)` |
| `bl` at 0x993e8 | `(unknown callee) 0x6debc` | `ret(0x2031c)` | `?` | `?` | `ret(0x2031c)` |
| `bl` at 0x993f8 | `(unknown callee) 0x2624c` | `0xf` | `ret(0x6debc)` | `?` | `ret(0x6debc)` |
| `bl` at 0x99400 | `(unknown callee) 0x28230` | `0xd` | `?` | `?` | `?` |
| `bl` at 0x99410 | `(unknown callee) 0x20634` | `0x31b96c` | `ret(0x28230)` | `?` | `ret(0x28230)` |
| `bl` at 0x99414 | `(unknown callee) 0xa451c` | `ret(0x20634)` | `?` | `?` | `?` |
| `bl` at 0x994bc | `(unknown callee) 0x201fc` | `?` | `?` | `[0x1a6990]` | `?` |
| `bl` at 0x994f0 | `(unknown callee) 0x20550` | `[0x1a6994]` | `0x0` | `?` | `?` |
| `bl` at 0x99528 | `(unknown callee) 0x201fc` | `?` | `?` | `[0x1a6984]` | `?` |
| `bl` at 0x99564 | `(unknown callee) 0x20550` | `[0x1a6988]` | `0x0` | `?` | `?` |
| `bl` at 0x99594 | `(unknown callee) 0x28230` | `0x1` | `?` | `0x1a6900` | `[sp-40]` |
| `bl` at 0x995a4 | `(unknown callee) 0x28230` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x995b4 | `(unknown callee) 0x28230` | `0x98` | `?` | `?` | `?` |
| `bl` at 0x995c4 | `(unknown callee) 0x28230` | `0xcb` | `?` | `?` | `?` |
| `bl` at 0x995d8 | `(unknown callee) 0x28230` | `0xcc` | `?` | `?` | `ret(0x28230)` |
| `bl` at 0x995ec | `(unknown callee) 0xa3f14` | `[sp-40]` | `?` | `?` | `ret(0x28230)` |
| `bl` at 0x995fc | `(unknown callee) 0x28230` | `0xe` | `?` | `?` | `?` |
| `bl` at 0x99624 | `(unknown callee) 0x2624c` | `0xe` | `0x0` | `0x1a6900` | `0x0` |
| `bl` at 0x9962c | `(unknown callee) 0x28230` | `0xf` | `?` | `?` | `?` |
| `bl` at 0x9963c | `(unknown callee) 0x20634` | `0x1a6940` | `ret(0x28230)` | `?` | `ret(0x28230)` |
| `bl` at 0x99648 | `(unknown callee) 0x20634` | `0x1a694c` | `0x31b96c` | `?` | `?` |
| `bl` at 0x9964c | `(unknown callee) 0x6e190` | `ret(0x20634)` | `?` | `?` | `?` |
| `bl` at 0x9965c | `(unknown callee) 0x81b18` | `ret(0x6e190)` | `0x1` | `?` | `ret(0x6e190)` |
| `bl` at 0x9966c | `(unknown callee) 0x6e190` | `ret(0x81b18)` | `?` | `?` | `?` |
| `bl` at 0x9967c | `(unknown callee) 0x81b18` | `ret(0x6e190)` | `0x1` | `?` | `ret(0x6e190)` |
| `bl` at 0x9968c | `(unknown callee) 0x3e38c` | `ret(0x81b18)` | `?` | `?` | `?` |
| `bl` at 0x996a0 | `(unknown callee) 0x3e458` | `[sp-52]` | `?` | `?` | `ret(0x3e38c)` |
| `bl` at 0x996b4 | `(unknown callee) 0x3e38c` | `?` | `?` | `?` | `?` |
| `bl` at 0x996c8 | `(unknown callee) 0x3e498` | `[sp-52]` | `?` | `?` | `ret(0x3e38c)` |
| `bl` at 0x996f4 | `(unknown callee) 0x81b18` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x99718 | `(unknown callee) 0x99e9c` | `[sp-40]` | `[sp-52]` | `ret(0x81b18)` | `[sp-48]` |
| `bl` at 0x99744 | `(unknown callee) 0x5572c` | `ret(0x99e9c)` | `?` | `0x1a6900` | `0x0` |
| `bl` at 0x99750 | `(unknown callee) 0x556e0` | `ret(0x5572c)` | `?` | `?` | `?` |
| `bl` at 0x99bc4 | `(unknown callee) 0x2624c` | `0xe` | `0x0` | `0x1a6900` | `0x0` |
| `bl` at 0x99bd0 | `(unknown callee) 0x28230` | `0x3c` | `?` | `?` | `?` |
| `bl` at 0x99c38 | `(unknown callee) 0x2624c` | `0x3c` | `?` | `?` | `0x1a6900` |
| `bl` at 0x99c3c | `(unknown callee) 0x9a000` | `?` | `?` | `?` | `?` |
| `bl` at 0x99c50 | `(unknown callee) 0x22b84` | `0x1a68a0` | `0x16a6f4` | `0x1` | `0x9` |
| `bl` at 0x99c60 | `(unknown callee) 0xfb5cc` | `0x1a68f8` | `0xe` | `0x0` | `?` |
| `bl` at 0x99d50 | `(unknown callee) 0x22540` | `0x1a68f4` | `0x16a700` | `0x9a350` | `0x0` |
| `bl` at 0x99d90 | `(unknown callee) 0x22540` | `0x1a68f4` | `0x16a700` | `0x9a138` | `0x0` |
| `bl` at 0x99da4 | `(unknown callee) 0xfa1b8` | `0x1a68f8` | `0xffff00e` | `0x3` | `0xa1cec` |
| `bl` at 0x99ddc | `(unknown callee) 0x22070` | `0x9` | `0x16a6dc` | `0x19d` | `0x0` |
| `bl` at 0x99df0 | `(unknown callee) 0xfa64c` | `0x1a68f8` | `?` | `0x0` | `0xa1cec` |
| `bl` at 0x99e28 | `(unknown callee) 0x22070` | `0x9` | `0x16a6dc` | `0x1a2` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x9937c | `[0x1a6624] == 0x0` | `[0x1a6624] != 0x0` |
| `beq` at 0x993e4 | `ret(0x2031c) == 0x6` | `ret(0x2031c) != 0x6` |
| `beq` at 0x99464 | `[sp-40] == 0x42` | `[sp-40] != 0x42` |
| `beq` at 0x99470 | `[sp-40] == 0x46` | `[sp-40] != 0x46` |
| `ble` at 0x9960c | `[0x1a693c] <= 0xfe (signed, 32-bit)` | `[0x1a693c] > 0xfe (signed, 32-bit)` |
| `beq` at 0x99698 | `ret(0x3e38c) == 0x0` | `ret(0x3e38c) != 0x0` |
| `beq` at 0x996c0 | `ret(0x3e38c) == 0x0` | `ret(0x3e38c) != 0x0` |
| `beq` at 0x99afc | `[0x1a6910] == 0x40` | `[0x1a6910] != 0x40` |
| `bgt` at 0x99b08 | `[0x1a6910] > 0x40 (signed, 32-bit)` | `[0x1a6910] <= 0x40 (signed, 32-bit)` |
| `blt` at 0x99b14 | `[0x1a6910] < 0x2d (signed, 32-bit)` | `[0x1a6910] >= 0x2d (signed, 32-bit)` |
| `ble` at 0x99b20 | `[0x1a6910] <= 0x2e (signed, 32-bit)` | `[0x1a6910] > 0x2e (signed, 32-bit)` |
| `beq` at 0x99b2c | `[0x1a6910] == 0x36` | `[0x1a6910] != 0x36` |
| `beq` at 0x99b3c | `[0x1a6910] == 0x52` | `[0x1a6910] != 0x52` |
| `bgt` at 0x99b48 | `[0x1a6910] > 0x52 (signed, 32-bit)` | `[0x1a6910] <= 0x52 (signed, 32-bit)` |
| `beq` at 0x99b54 | `[0x1a6910] == 0x44` | `[0x1a6910] != 0x44` |
| `beq` at 0x99b64 | `[0x1a6910] == 0x7d` | `[0x1a6910] != 0x7d` |
| `beq` at 0x99b78 | `[0x1a6910] == 0x52` | `[0x1a6910] != 0x52` |
| `beq` at 0x99b88 | `[0x1a6910] == 0x44` | `[0x1a6910] != 0x44` |
| `beq` at 0x99b98 | `[0x1a6910] == 0x2d` | `[0x1a6910] != 0x2d` |
| `beq` at 0x99bac | `[0x1a693c] == 0x0` | `[0x1a693c] != 0x0` |
| `bhi` at 0x99be8 | `[0x1a692c] > 0xf (unsigned, 32-bit)` | `[0x1a692c] <= 0xf (unsigned, 32-bit)` |
| `bne` at 0x99c04 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x99c20 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x99c70 | `[0x1a6910] == 0x35` | `[0x1a6910] != 0x35` |
| `beq` at 0x99c80 | `[0x1a6910] == 0x4f` | `[0x1a6910] != 0x4f` |
| `beq` at 0x99c90 | `[0x1a6910] == 0x42` | `[0x1a6910] != 0x42` |
| `beq` at 0x99ca0 | `[0x1a6910] == 0x46` | `[0x1a6910] != 0x46` |
| `beq` at 0x99cb0 | `[0x1a6910] == 0x4a` | `[0x1a6910] != 0x4a` |
| `beq` at 0x99cc0 | `[0x1a6910] == 0x4b` | `[0x1a6910] != 0x4b` |
| `beq` at 0x99cd0 | `[0x1a6910] == 0x21` | `[0x1a6910] != 0x21` |
| `beq` at 0x99ce0 | `[0x1a6910] == 0x5e` | `[0x1a6910] != 0x5e` |
| `beq` at 0x99cf0 | `[0x1a6910] == 0x2b` | `[0x1a6910] != 0x2b` |
| `beq` at 0x99d00 | `[0x1a6910] == 0x4c` | `[0x1a6910] != 0x4c` |
| `beq` at 0x99d10 | `[0x1a6910] == 0x4d` | `[0x1a6910] != 0x4d` |
| `beq` at 0x99db8 | `ret(0xfa1b8) == 0x0` | `ret(0xfa1b8) != 0x0` |
| `beq` at 0x99e04 | `ret(0xfa64c) == 0x0` | `ret(0xfa64c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x99e30 | `r0` | `?` |

## `L1CfgReset`

- `L1CfgReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9a000 to 0x9a0d4.
  Established. A word store clears absolute 0x1a698c (`str` at 0x9a018).
  Words loaded from absolute 0x1a690c and 0x1a6910 are copied into memory at sp-0x30 and sp-0x2c (`ldr` at 0x9a054, `str` at 0x9a058, `ldr` at 0x9a060, `str` at 0x9a064).
  The call to unnamed 0xa00bc has candidate values 0x1a6994, 0x1a698c, 0x1a6990 and the word at offset eight of the pointer from sp-0x18 (`bl` at 0x9a0b8).
  These updates do not establish a physical reset sequence.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x9a010 | load | 4 | `abs 0x9a0c4` | `-` |
| `str` at 0x9a018 | store | 4 | `abs 0x1a698c` | `0x0` |
| `ldr` at 0x9a01c | load | 4 | `abs 0x9a0c4` | `-` |
| `ldr` at 0x9a020 | load | 4 | `abs 0x9a0c4` | `-` |
| `ldr` at 0x9a024 | load | 4 | `abs 0x1a6990` | `-` |
| `ldr` at 0x9a038 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9a048 | load | 4 | `abs 0x9a0c4` | `-` |
| `str` at 0x9a04c | store | 4 | `sp -0x18` | `0x1a6900` |
| `ldr` at 0x9a050 | load | 4 | `abs 0x9a0c4` | `-` |
| `ldr` at 0x9a054 | load | 4 | `abs 0x1a690c` | `-` |
| `str` at 0x9a058 | store | 4 | `sp -0x30` | `[0x1a690c]` |
| `ldr` at 0x9a05c | load | 4 | `abs 0x9a0c4` | `-` |
| `ldr` at 0x9a060 | load | 4 | `abs 0x1a6910` | `-` |
| `str` at 0x9a064 | store | 4 | `sp -0x2c` | `[0x1a6910]` |
| `ldr` at 0x9a068 | load | 4 | `abs 0x9a0c4` | `-` |
| `ldr` at 0x9a06c | load | 4 | `abs 0x1a6958` | `-` |
| `str` at 0x9a07c | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x9a088 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x9a08c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x9a090 | store | 4 | `sp -0x28` | `[sp-28]` |
| `str` at 0x9a098 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x9a0a0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x9a0a4 | load | 4 | `abs 0x9a0c8` | `-` |
| `ldr` at 0x9a0a8 | load | 4 | `abs 0x9a0cc` | `-` |
| `ldr` at 0x9a0ac | load | 4 | `abs 0x9a0d0` | `-` |
| `ldr` at 0x9a0b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9a0b4 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldm` at 0x9a0c0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9a0c0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9a0c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9a044 | `(unknown callee) 0x20550` | `[0x1a6994]` | `0x0` | `?` | `?` |
| `bl` at 0x9a0b8 | `(unknown callee) 0xa00bc` | `0x1a6994` | `0x1a698c` | `0x1a6990` | `[[sp-24]+8]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x9a074 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9a0c0 | `r0` | `ret(0xa00bc)` |

## `L1CfgRefresh`

- `L1CfgRefresh`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9a0d4 to 0x9a138.
  Established. The word at sp-0x14 starts at zero and is incremented by a later store (`str` at 0x9a0e8, `str` at 0x9a128).
  The nonzero-result branch after unnamed 0xa92d8 targets 0x9a110, while the zero-result path falls through to a jump to 0x9a120 (`bl` at 0x9a0fc, `bne` at 0x9a108, `b` at 0x9a10c).
  The computed word load has unknown target, and the following call to unnamed 0xa1144 has candidate values unknown, unknown, 0x1a6ba4 and the word at sp-0x14 (`ldr` at 0x9a118, `bl` at 0x9a11c).
  The refresh operation's hardware effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9a0e8 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x9a0ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9a0f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9a110 | load | 4 | `abs 0x9a134` | `-` |
| `ldr` at 0x9a114 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9a118 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a120 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9a128 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldm` at 0x9a130 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x9a130 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9a130 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9a130 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9a0fc | `(unknown callee) 0xa92d8` | `?` | `?` | `?` | `?` |
| `bl` at 0x9a11c | `(unknown callee) 0xa1144` | `?` | `?` | `0x1a6ba4` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0x9a0f4 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0x9a108 | `ret(0xa92d8) != 0x0` | `ret(0xa92d8) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9a130 | `r0` | `?` |

## `L1CfgMeshTask`

- `L1CfgMeshTask`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9a138 to 0x9a350.
  Established. Word stores clear absolute 0x1a6654 and put five or 0xf at absolute 0x1a0358 (`str` at 0x9a1b0, `str` at 0x9a1e4, `str` at 0x9a304).
  A word loaded from absolute 0x1a6644 is copied into memory at absolute 0x1a6640 (`ldr` at 0x9a290, `str` at 0x9a294).
  The call to unnamed 0x28230 has candidate values 0x9a and three unknowns (`bl` at 0x9a29c).
  The recorded control flow has no return, and the numeric fields do not establish mesh-discovery timing or protocol semantics.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9a148 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9a14c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x9a154 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x9a15c | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x9a164 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x9a170 | load | 4 | `abs 0x9a328` | `-` |
| `ldr` at 0x9a174 | load | 4 | `abs 0x1a6654` | `-` |
| `ldr` at 0x9a18c | load | 4 | `abs 0x9a328` | `-` |
| `ldr` at 0x9a190 | load | 4 | `abs 0x1a6654` | `-` |
| `ldr` at 0x9a1a0 | load | 4 | `abs 0x9a32c` | `-` |
| `ldr` at 0x9a1a8 | load | 4 | `abs 0x9a328` | `-` |
| `str` at 0x9a1b0 | store | 4 | `abs 0x1a6654` | `0x0` |
| `ldr` at 0x9a1b4 | load | 4 | `abs 0x9a330` | `-` |
| `ldr` at 0x9a1b8 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9a1c8 | load | 4 | `abs 0x9a334` | `-` |
| `ldr` at 0x9a1cc | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9a1d0 | load | 4 | `abs 0x9a338` | `-` |
| `ldr` at 0x9a1dc | load | 4 | `abs 0x9a33c` | `-` |
| `str` at 0x9a1e4 | store | 4 | `abs 0x1a0358` | `0x5` |
| `ldr` at 0x9a1ec | load | 4 | `abs 0x9a340` | `-` |
| `ldr` at 0x9a1f0 | load | 4 | `abs 0x1a6644` | `-` |
| `ldr` at 0x9a208 | load | 4 | `abs 0x9a340` | `-` |
| `ldr` at 0x9a20c | load | 4 | `abs 0x1a6644` | `-` |
| `str` at 0x9a214 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x9a218 | load | 4 | `abs 0x9a344` | `-` |
| `ldr` at 0x9a21c | load | 4 | `abs 0x1a6648` | `-` |
| `ldr` at 0x9a234 | load | 4 | `abs 0x9a344` | `-` |
| `ldr` at 0x9a238 | load | 4 | `abs 0x1a6648` | `-` |
| `str` at 0x9a240 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x9a244 | load | 4 | `abs 0x9a348` | `-` |
| `ldr` at 0x9a248 | load | 4 | `abs 0x1a698c` | `-` |
| `ldr` at 0x9a254 | load | 4 | `abs 0x9a33c` | `-` |
| `ldr` at 0x9a258 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9a25c | load | 4 | `abs 0x1a0358` | `-` |
| `str` at 0x9a270 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x9a278 | load | 4 | `abs 0x9a34c` | `-` |
| `ldr` at 0x9a27c | load | 4 | `abs 0x1a6640` | `-` |
| `ldr` at 0x9a288 | load | 4 | `abs 0x9a34c` | `-` |
| `ldr` at 0x9a28c | load | 4 | `abs 0x9a340` | `-` |
| `ldr` at 0x9a290 | load | 4 | `abs 0x1a6644` | `-` |
| `str` at 0x9a294 | store | 4 | `abs 0x1a6640` | `[0x1a6644]` |
| `str` at 0x9a2a0 | store | 4 | `sp -0x28` | `ret(0x28230)` |
| `ldr` at 0x9a2a4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9a2a8 | store | 4 | `sp -0x2c` | `ret(0x28230)` |
| `ldr` at 0x9a2ac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9a2b8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9a2c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9a2d4 | load | 4 | `abs 0x9a33c` | `-` |
| `ldr` at 0x9a2d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9a2dc | load | 4 | `abs 0x1a0358` | `-` |
| `ldr` at 0x9a2f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9a2fc | load | 4 | `abs 0x9a33c` | `-` |
| `str` at 0x9a304 | store | 4 | `abs 0x1a0358` | `0xf` |
| `ldr` at 0x9a30c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9a320 | store | 4 | `sp -0x24` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9a16c | `(unknown callee) 0x22944` | `0x64` | `?` | `?` | `?` |
| `bl` at 0x9a184 | `(unknown callee) 0x202c8` | `0x0` | `?` | `?` | `[0x1a6654]` |
| `bl` at 0x9a1a4 | `(unknown callee) 0x9adfc` | `0x16a758` | `?` | `[0x1a6654]` | `ret(0x202c8)` |
| `bl` at 0x9a1d4 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16a768` | `0x1a6654` | `0x1a288c` |
| `bl` at 0x9a1d8 | `(unknown callee) 0x9c3e0` | `?` | `?` | `?` | `?` |
| `bl` at 0x9a200 | `(unknown callee) 0x202c8` | `0x0` | `?` | `?` | `[0x1a6644]` |
| `bl` at 0x9a22c | `(unknown callee) 0x202c8` | `0x0` | `?` | `?` | `[0x1a6648]` |
| `bl` at 0x9a29c | `(unknown callee) 0x28230` | `0x9a` | `?` | `?` | `?` |
| `bl` at 0x9a2ec | `(unknown callee) 0x9be44` | `ret(0x28230)` | `?` | `?` | `?` |
| `bl` at 0x9a318 | `(unknown callee) 0x9be44` | `ret(0x28230)` | `?` | `?` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x9a17c | `[0x1a6654] == 0x0` | `[0x1a6654] != 0x0` |
| `bge` at 0x9a198 | `ret(0x202c8) >= [0x1a6654] (signed, 32-bit)` | `ret(0x202c8) < [0x1a6654] (signed, 32-bit)` |
| `beq` at 0x9a1c4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9a1f8 | `[0x1a6644] == 0x0` | `[0x1a6644] != 0x0` |
| `beq` at 0x9a224 | `[0x1a6648] == 0x0` | `[0x1a6648] != 0x0` |
| `beq` at 0x9a250 | `[0x1a698c] == 0x1` | `[0x1a698c] != 0x1` |
| `blt` at 0x9a264 | `[sp-28] < [0x1a0358] (signed, 32-bit)` | `[sp-28] >= [0x1a0358] (signed, 32-bit)` |
| `bne` at 0x9a284 | `[0x1a6640] != 0x0` | `[0x1a6640] == 0x0` |
| `beq` at 0x9a2b4 | `ret(0x28230) == 0x0` | `ret(0x28230) != 0x0` |
| `beq` at 0x9a2c0 | `ret(0x28230) == 0x1` | `ret(0x28230) != 0x1` |
| `beq` at 0x9a2d0 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `bge` at 0x9a2e4 | `[sp-32] >= [0x1a0358] (signed, 32-bit)` | `[sp-32] < [0x1a0358] (signed, 32-bit)` |
| `ble` at 0x9a2f8 | `[sp-28] <= 0x3c (signed, 32-bit)` | `[sp-28] > 0x3c (signed, 32-bit)` |
| `beq` at 0x9a314 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|

## `L1CfgRingTask`

- `L1CfgRingTask`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9a350 to 0x9add4.
  Established. Two stores at sp-0x44 and sp-0x48 receive separately loaded words from absolute 0x1a6988 plus 156 and 312 respectively (`ldr` at 0x9a3e0, `str` at 0x9a3e8, `ldr` at 0x9a3f0, `str` at 0x9a3f8).
  A call to unnamed 0xfd4a8 has candidate values 0x1a68f8, zero, zero and 0x64 (`bl` at 0x9a904);
  its result is stored at sp-0x1c, and a following word store at offset 0x18 from that result receives the word loaded from absolute 0x1a6fa8 (`bl` at 0x9a904, `str` at 0x9a90c, `str` at 0x9a92c).
  The object fields and loop do not identify physical ring ports.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9a360 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9a364 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x9a370 | load | 4 | `abs 0x9ad9c` | `-` |
| `ldr` at 0x9a374 | load | 4 | `abs 0x1a6654` | `-` |
| `ldr` at 0x9a38c | load | 4 | `abs 0x9ad9c` | `-` |
| `ldr` at 0x9a390 | load | 4 | `abs 0x1a6654` | `-` |
| `ldr` at 0x9a3a0 | load | 4 | `abs 0x9ada0` | `-` |
| `ldr` at 0x9a3a8 | load | 4 | `abs 0x9ad9c` | `-` |
| `str` at 0x9a3b0 | store | 4 | `abs 0x1a6654` | `0x0` |
| `ldr` at 0x9a3b4 | load | 4 | `abs 0x9ada4` | `-` |
| `ldr` at 0x9a3b8 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9a3c8 | load | 4 | `abs 0x9ada8` | `-` |
| `ldr` at 0x9a3cc | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9a3d0 | load | 4 | `abs 0x9adac` | `-` |
| `ldr` at 0x9a3dc | load | 4 | `abs 0x9adb0` | `-` |
| `ldr` at 0x9a3e0 | load | 4 | `abs 0x1a6988` | `-` |
| `str` at 0x9a3e8 | store | 4 | `sp -0x44` | `[0x1a6988]+156` |
| `ldr` at 0x9a3ec | load | 4 | `abs 0x9adb0` | `-` |
| `ldr` at 0x9a3f0 | load | 4 | `abs 0x1a6988` | `-` |
| `str` at 0x9a3f8 | store | 4 | `sp -0x48` | `[0x1a6988]+312` |
| `ldr` at 0x9a424 | load | 4 | `abs 0x9adb4` | `-` |
| `str` at 0x9a42c | store | 4 | `abs 0x1a662c` | `?` |
| `ldr` at 0x9a448 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9a44c | load | 4 | `[sp-68] +0x4` | `-` |
| `ldr` at 0x9a458 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9a45c | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x9a468 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9a46c | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x9a478 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9a47c | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x9a488 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9a48c | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x9a498 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9a49c | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x9a4a8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9a4ac | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x9a4b8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9a4bc | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x9a4c8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9a4cc | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x9a4d8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9a4dc | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x9a4e8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9a4ec | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x9a4f8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9a4fc | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x9a51c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9a520 | load | 4 | `[sp-72] +0x4` | `-` |
| `ldr` at 0x9a52c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9a530 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x9a53c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9a540 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x9a54c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9a550 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x9a55c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9a560 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x9a56c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9a570 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x9a57c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9a580 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x9a58c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9a590 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x9a59c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9a5a0 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x9a5ac | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9a5b0 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x9a5bc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9a5c0 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x9a5cc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9a5d0 | load | 4 | `[sp-72] +0x10` | `-` |
| `str` at 0x9a60c | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x9a610 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9a614 | load | 4 | `[sp-68] +0x4` | `-` |
| `ldr` at 0x9a620 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9a624 | load | 4 | `[sp-68] +0x78` | `-` |
| `str` at 0x9a634 | store | 4 | `sp -0x4c` | `0x2` |
| `ldr` at 0x9a63c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9a640 | load | 4 | `[sp-72] +0x4` | `-` |
| `ldr` at 0x9a64c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9a650 | load | 4 | `[sp-72] +0x78` | `-` |
| `str` at 0x9a660 | store | 4 | `sp -0x4c` | `0x1` |
| `ldr` at 0x9a664 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x9a670 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9a674 | load | 4 | `[sp-68] +0x4` | `-` |
| `ldr` at 0x9a680 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9a684 | load | 4 | `[sp-68] +0x78` | `-` |
| `str` at 0x9a694 | store | 4 | `sp -0x4c` | `0x2` |
| `ldr` at 0x9a69c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9a6a0 | load | 4 | `[sp-72] +0x4` | `-` |
| `ldr` at 0x9a6ac | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9a6b0 | load | 4 | `[sp-72] +0x78` | `-` |
| `str` at 0x9a6c0 | store | 4 | `sp -0x4c` | `0x1` |
| `ldr` at 0x9a6c4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x9a6d0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9a6d4 | load | 4 | `[sp-68] +0x4` | `-` |
| `ldr` at 0x9a6e0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9a6e4 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x9a6f0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9a6f4 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x9a700 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9a704 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x9a710 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9a714 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x9a720 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9a724 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x9a730 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9a734 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x9a740 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9a744 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x9a750 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9a754 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x9a760 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9a764 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x9a770 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9a774 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x9a780 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9a784 | load | 4 | `[sp-68] +0x10` | `-` |
| `str` at 0x9a794 | store | 4 | `sp -0x4c` | `0x2` |
| `ldr` at 0x9a79c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9a7a0 | load | 4 | `[sp-72] +0x4` | `-` |
| `ldr` at 0x9a7ac | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9a7b0 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x9a7bc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9a7c0 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x9a7cc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9a7d0 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x9a7dc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9a7e0 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x9a7ec | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9a7f0 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x9a7fc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9a800 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x9a80c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9a810 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x9a81c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9a820 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x9a82c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9a830 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x9a83c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9a840 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x9a84c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9a850 | load | 4 | `[sp-72] +0x10` | `-` |
| `str` at 0x9a860 | store | 4 | `sp -0x4c` | `0x1` |
| `ldr` at 0x9a864 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x9a8c8 | load | 4 | `abs 0x9adb0` | `-` |
| `str` at 0x9a8d8 | store | 4 | `sp -0x5c` | `0x2` |
| `str` at 0x9a8e0 | store | 4 | `sp -0x58` | `0x2` |
| `ldr` at 0x9a8e4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x9a8e8 | store | 4 | `sp -0x54` | `[sp-64]` |
| `str` at 0x9a8f0 | store | 4 | `sp -0x50` | `sp-60` |
| `ldr` at 0x9a8f4 | load | 4 | `abs 0x9adb8` | `-` |
| `str` at 0x9a90c | store | 4 | `sp -0x1c` | `ret(0xfd4a8)` |
| `ldr` at 0x9a910 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9a920 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9a924 | load | 4 | `abs 0x9adbc` | `-` |
| `ldr` at 0x9a928 | load | 4 | `abs 0x1a6fa8` | `-` |
| `str` at 0x9a92c | store | 4 | `ret(0xfd4a8) +0x18` | `[0x1a6fa8]` |
| `ldr` at 0x9a930 | load | 4 | `abs 0x9adc0` | `-` |
| `ldr` at 0x9a934 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x9a948 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a954 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9a95c | load | 4 | `abs 0x9adb4` | `-` |
| `str` at 0x9a964 | store | 4 | `abs 0x1a662c` | `?` |
| `ldr` at 0x9a96c | load | 4 | `abs 0x9adb4` | `-` |
| `ldr` at 0x9a970 | load | 4 | `abs 0x1a662c` | `-` |
| `ldr` at 0x9a9a4 | load | 4 | `abs 0x9adb4` | `-` |
| `str` at 0x9a9ac | store | 4 | `abs 0x1a662c` | `0x0` |
| `ldr` at 0x9a9b0 | load | 4 | `abs 0x9adc4` | `-` |
| `ldr` at 0x9a9c0 | load | 4 | `abs 0x9adc4` | `-` |
| `ldr` at 0x9a9c4 | load | 4 | `abs 0x9adb0` | `-` |
| `ldr` at 0x9a9e4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9a9e8 | load | 4 | `[sp-68] +0x4` | `-` |
| `ldr` at 0x9a9f4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9a9f8 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x9aa04 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9aa08 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x9aa14 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9aa18 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x9aa24 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9aa28 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x9aa34 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9aa38 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x9aa44 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9aa48 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x9aa54 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9aa58 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x9aa64 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9aa68 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x9aa74 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9aa78 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x9aa84 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9aa88 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x9aa94 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9aa98 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x9ab00 | load | 4 | `abs 0x9adc4` | `-` |
| `str` at 0x9ab10 | store | 4 | `sp -0x5c` | `0x2` |
| `str` at 0x9ab18 | store | 4 | `sp -0x58` | `0x2` |
| `ldr` at 0x9ab1c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x9ab20 | store | 4 | `sp -0x54` | `[sp-64]` |
| `str` at 0x9ab28 | store | 4 | `sp -0x50` | `sp-60` |
| `ldr` at 0x9ab2c | load | 4 | `abs 0x9adb8` | `-` |
| `str` at 0x9ab44 | store | 4 | `sp -0x1c` | `ret(0xfd4a8)` |
| `ldr` at 0x9ab48 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9ab54 | load | 4 | `abs 0x9adc8` | `-` |
| `ldr` at 0x9ab60 | load | 4 | `abs 0x9ada4` | `-` |
| `ldr` at 0x9ab64 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9ab74 | load | 4 | `abs 0x9ada8` | `-` |
| `ldr` at 0x9ab78 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9ab7c | load | 4 | `abs 0x9adcc` | `-` |
| `ldr` at 0x9ab84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9ab88 | load | 4 | `abs 0x9adbc` | `-` |
| `ldr` at 0x9ab8c | load | 4 | `abs 0x1a6fa8` | `-` |
| `str` at 0x9ab90 | store | 4 | `(unknown)` | `[0x1a6fa8]` |
| `ldr` at 0x9ab94 | load | 4 | `abs 0x9adc0` | `-` |
| `ldr` at 0x9ab98 | load | 4 | `abs 0x31b510` | `-` |
| `ldr` at 0x9aba4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9abc0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9abc4 | load | 4 | `[sp-72] +0x4` | `-` |
| `ldr` at 0x9abd0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9abd4 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x9abe0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9abe4 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x9abf0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9abf4 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x9ac00 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9ac04 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x9ac10 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9ac14 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x9ac20 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9ac24 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x9ac30 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9ac34 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x9ac40 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9ac44 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x9ac50 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9ac54 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x9ac60 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9ac64 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x9ac70 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9ac74 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x9acdc | load | 4 | `abs 0x9adc4` | `-` |
| `str` at 0x9acec | store | 4 | `sp -0x5c` | `0x2` |
| `str` at 0x9acf4 | store | 4 | `sp -0x58` | `0x2` |
| `ldr` at 0x9acf8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x9acfc | store | 4 | `sp -0x54` | `[sp-64]` |
| `str` at 0x9ad04 | store | 4 | `sp -0x50` | `sp-60` |
| `ldr` at 0x9ad08 | load | 4 | `abs 0x9adb8` | `-` |
| `str` at 0x9ad20 | store | 4 | `sp -0x1c` | `ret(0xfd4a8)` |
| `ldr` at 0x9ad24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9ad30 | load | 4 | `abs 0x9adc8` | `-` |
| `ldr` at 0x9ad3c | load | 4 | `abs 0x9ada4` | `-` |
| `ldr` at 0x9ad40 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9ad50 | load | 4 | `abs 0x9ada8` | `-` |
| `ldr` at 0x9ad54 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9ad58 | load | 4 | `abs 0x9add0` | `-` |
| `ldr` at 0x9ad60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9ad64 | load | 4 | `abs 0x9adbc` | `-` |
| `ldr` at 0x9ad68 | load | 4 | `abs 0x1a6fa8` | `-` |
| `str` at 0x9ad6c | store | 4 | `(unknown)` | `[0x1a6fa8]` |
| `ldr` at 0x9ad70 | load | 4 | `abs 0x9adc0` | `-` |
| `ldr` at 0x9ad74 | load | 4 | `abs 0x31b514` | `-` |
| `ldr` at 0x9ad80 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x9ad98 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9ad98 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9ad98 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9a36c | `(unknown callee) 0x9add4` | `0x1f4` | `r1` | `r2` | `r3` |
| `bl` at 0x9a384 | `(unknown callee) 0x202c8` | `0x0` | `?` | `?` | `[0x1a6654]` |
| `bl` at 0x9a3a4 | `(unknown callee) 0x9adfc` | `0x16a758` | `?` | `[0x1a6654]` | `ret(0x202c8)` |
| `bl` at 0x9a3d4 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16a768` | `0x1a6654` | `0x1a288c` |
| `bl` at 0x9a3d8 | `(unknown callee) 0x9c3e0` | `?` | `?` | `?` | `?` |
| `bl` at 0x9a400 | `(unknown callee) 0xa92d8` | `0x1` | `?` | `?` | `[0x1a6988]+312` |
| `bl` at 0x9a414 | `(unknown callee) 0xa92d8` | `0x2` | `?` | `?` | `ret(0xa92d8)` |
| `bl` at 0x9a438 | `(unknown callee) 0xa92d8` | `0x1` | `?` | `?` | `ret(0xa92d8)` |
| `bl` at 0x9a50c | `(unknown callee) 0xa92d8` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x9a5e4 | `(unknown callee) 0xa92d8` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x9a5f8 | `(unknown callee) 0xa92d8` | `0x2` | `?` | `?` | `ret(0xa92d8)` |
| `bl` at 0x9a884 | `(unknown callee) 0x958d4` | `sp-64` | `sp-60` | `0x3` | `sp-64` |
| `bl` at 0x9a89c | `(unknown callee) 0x95934` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x9a8b4 | `(unknown callee) 0x95934` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x9a8d0 | `(unknown callee) 0xa0cd4` | `sp-64` | `sp-60` | `0x1a6900` | `0x1` |
| `bl` at 0x9a904 | `(unknown callee) 0xfd4a8` | `0x1a68f8` | `0x0` | `0x0` | `0x64` |
| `bl` at 0x9a958 | `(unknown callee) 0xfcb5c` | `?` | `[sp-28]` | `0x31b45c` | `?` |
| `bl` at 0x9a980 | `(unknown callee) 0xa92d8` | `0x1` | `?` | `?` | `[0x1a662c]` |
| `bl` at 0x9a994 | `(unknown callee) 0xa92d8` | `0x2` | `?` | `?` | `ret(0xa92d8)` |
| `bl` at 0x9a9bc | `(unknown callee) 0x20550` | `0x1a6658` | `0x0` | `0x230` | `?` |
| `bl` at 0x9a9cc | `(unknown callee) 0x203f4` | `0x1a6658` | `0x1a6900` | `0x70` | `?` |
| `bl` at 0x9a9d4 | `(unknown callee) 0xa92d8` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x9aabc | `(unknown callee) 0x958d4` | `sp-64` | `sp-60` | `0x2` | `sp-64` |
| `bl` at 0x9aad4 | `(unknown callee) 0x95934` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x9aaec | `(unknown callee) 0x95934` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x9ab08 | `(unknown callee) 0xa0cd4` | `sp-64` | `sp-60` | `0x1a6658` | `0x5` |
| `bl` at 0x9ab3c | `(unknown callee) 0xfd4a8` | `0x1a68f8` | `0x0` | `0x0` | `0x64` |
| `bl` at 0x9ab58 | `(unknown callee) 0x293f8` | `0x16a77c` | `?` | `?` | `ret(0xfd4a8)` |
| `bl` at 0x9ab80 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16a7a8` | `?` | `0x1a288c` |
| `bl` at 0x9aba8 | `(unknown callee) 0xfcb5c` | `[0x31b510]+20` | `[sp-28]` | `?` | `[0x31b510]+20` |
| `bl` at 0x9abb0 | `(unknown callee) 0xa92d8` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x9ac98 | `(unknown callee) 0x958d4` | `sp-64` | `sp-60` | `0x2` | `sp-64` |
| `bl` at 0x9acb0 | `(unknown callee) 0x95934` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x9acc8 | `(unknown callee) 0x95934` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x9ace4 | `(unknown callee) 0xa0cd4` | `sp-64` | `sp-60` | `0x1a6658` | `0x5` |
| `bl` at 0x9ad18 | `(unknown callee) 0xfd4a8` | `0x1a68f8` | `0x0` | `0x0` | `0x64` |
| `bl` at 0x9ad34 | `(unknown callee) 0x293f8` | `0x16a77c` | `?` | `?` | `ret(0xfd4a8)` |
| `bl` at 0x9ad5c | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16a7c0` | `?` | `0x1a288c` |
| `bl` at 0x9ad84 | `(unknown callee) 0xfcb5c` | `[0x31b514]+20` | `[sp-28]` | `?` | `[0x31b514]+20` |
| `bl` at 0x9ad8c | `(unknown callee) 0x9add4` | `0x1f4` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x9a37c | `[0x1a6654] == 0x0` | `[0x1a6654] != 0x0` |
| `bge` at 0x9a398 | `ret(0x202c8) >= [0x1a6654] (signed, 32-bit)` | `ret(0x202c8) < [0x1a6654] (signed, 32-bit)` |
| `beq` at 0x9a3c4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x9a40c | `ret(0xa92d8) != 0x0` | `ret(0xa92d8) == 0x0` |
| `bne` at 0x9a420 | `ret(0xa92d8) != 0x0` | `ret(0xa92d8) == 0x0` |
| `beq` at 0x9a444 | `ret(0xa92d8) == 0x0` | `ret(0xa92d8) != 0x0` |
| `beq` at 0x9a454 | `[[sp-68]+4] == 0x0` | `[[sp-68]+4] != 0x0` |
| `beq` at 0x9a464 | `[[sp-68]+16] == 0x35` | `[[sp-68]+16] != 0x35` |
| `beq` at 0x9a474 | `[[sp-68]+16] == 0x4f` | `[[sp-68]+16] != 0x4f` |
| `beq` at 0x9a484 | `[[sp-68]+16] == 0x42` | `[[sp-68]+16] != 0x42` |
| `beq` at 0x9a494 | `[[sp-68]+16] == 0x46` | `[[sp-68]+16] != 0x46` |
| `beq` at 0x9a4a4 | `[[sp-68]+16] == 0x4a` | `[[sp-68]+16] != 0x4a` |
| `beq` at 0x9a4b4 | `[[sp-68]+16] == 0x4b` | `[[sp-68]+16] != 0x4b` |
| `beq` at 0x9a4c4 | `[[sp-68]+16] == 0x21` | `[[sp-68]+16] != 0x21` |
| `beq` at 0x9a4d4 | `[[sp-68]+16] == 0x5e` | `[[sp-68]+16] != 0x5e` |
| `beq` at 0x9a4e4 | `[[sp-68]+16] == 0x2b` | `[[sp-68]+16] != 0x2b` |
| `beq` at 0x9a4f4 | `[[sp-68]+16] == 0x4c` | `[[sp-68]+16] != 0x4c` |
| `bne` at 0x9a504 | `[[sp-68]+16] != 0x4d` | `[[sp-68]+16] == 0x4d` |
| `beq` at 0x9a518 | `ret(0xa92d8) == 0x0` | `ret(0xa92d8) != 0x0` |
| `beq` at 0x9a528 | `[[sp-72]+4] == 0x0` | `[[sp-72]+4] != 0x0` |
| `beq` at 0x9a538 | `[[sp-72]+16] == 0x35` | `[[sp-72]+16] != 0x35` |
| `beq` at 0x9a548 | `[[sp-72]+16] == 0x4f` | `[[sp-72]+16] != 0x4f` |
| `beq` at 0x9a558 | `[[sp-72]+16] == 0x42` | `[[sp-72]+16] != 0x42` |
| `beq` at 0x9a568 | `[[sp-72]+16] == 0x46` | `[[sp-72]+16] != 0x46` |
| `beq` at 0x9a578 | `[[sp-72]+16] == 0x4a` | `[[sp-72]+16] != 0x4a` |
| `beq` at 0x9a588 | `[[sp-72]+16] == 0x4b` | `[[sp-72]+16] != 0x4b` |
| `beq` at 0x9a598 | `[[sp-72]+16] == 0x21` | `[[sp-72]+16] != 0x21` |
| `beq` at 0x9a5a8 | `[[sp-72]+16] == 0x5e` | `[[sp-72]+16] != 0x5e` |
| `beq` at 0x9a5b8 | `[[sp-72]+16] == 0x2b` | `[[sp-72]+16] != 0x2b` |
| `beq` at 0x9a5c8 | `[[sp-72]+16] == 0x4c` | `[[sp-72]+16] != 0x4c` |
| `bne` at 0x9a5d8 | `[[sp-72]+16] != 0x4d` | `[[sp-72]+16] == 0x4d` |
| `beq` at 0x9a5f0 | `ret(0xa92d8) == 0x0` | `ret(0xa92d8) != 0x0` |
| `beq` at 0x9a604 | `ret(0xa92d8) == 0x0` | `ret(0xa92d8) != 0x0` |
| `beq` at 0x9a61c | `[[sp-68]+4] == 0x0` | `[[sp-68]+4] != 0x0` |
| `bne` at 0x9a62c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x9a648 | `[[sp-72]+4] == 0x0` | `[[sp-72]+4] != 0x0` |
| `bne` at 0x9a658 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x9a66c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x9a67c | `[[sp-68]+4] == 0x0` | `[[sp-68]+4] != 0x0` |
| `beq` at 0x9a68c | `[[sp-68]+120] == 0x0` | `[[sp-68]+120] != 0x0` |
| `beq` at 0x9a6a8 | `[[sp-72]+4] == 0x0` | `[[sp-72]+4] != 0x0` |
| `beq` at 0x9a6b8 | `[[sp-72]+120] == 0x0` | `[[sp-72]+120] != 0x0` |
| `bne` at 0x9a6cc | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x9a6dc | `[[sp-68]+4] == 0x0` | `[[sp-68]+4] != 0x0` |
| `beq` at 0x9a6ec | `[[sp-68]+16] == 0x35` | `[[sp-68]+16] != 0x35` |
| `beq` at 0x9a6fc | `[[sp-68]+16] == 0x4f` | `[[sp-68]+16] != 0x4f` |
| `beq` at 0x9a70c | `[[sp-68]+16] == 0x42` | `[[sp-68]+16] != 0x42` |
| `beq` at 0x9a71c | `[[sp-68]+16] == 0x46` | `[[sp-68]+16] != 0x46` |
| `beq` at 0x9a72c | `[[sp-68]+16] == 0x4a` | `[[sp-68]+16] != 0x4a` |
| `beq` at 0x9a73c | `[[sp-68]+16] == 0x4b` | `[[sp-68]+16] != 0x4b` |
| `beq` at 0x9a74c | `[[sp-68]+16] == 0x21` | `[[sp-68]+16] != 0x21` |
| `beq` at 0x9a75c | `[[sp-68]+16] == 0x5e` | `[[sp-68]+16] != 0x5e` |
| `beq` at 0x9a76c | `[[sp-68]+16] == 0x2b` | `[[sp-68]+16] != 0x2b` |
| `beq` at 0x9a77c | `[[sp-68]+16] == 0x4c` | `[[sp-68]+16] != 0x4c` |
| `beq` at 0x9a78c | `[[sp-68]+16] == 0x4d` | `[[sp-68]+16] != 0x4d` |
| `beq` at 0x9a7a8 | `[[sp-72]+4] == 0x0` | `[[sp-72]+4] != 0x0` |
| `beq` at 0x9a7b8 | `[[sp-72]+16] == 0x35` | `[[sp-72]+16] != 0x35` |
| `beq` at 0x9a7c8 | `[[sp-72]+16] == 0x4f` | `[[sp-72]+16] != 0x4f` |
| `beq` at 0x9a7d8 | `[[sp-72]+16] == 0x42` | `[[sp-72]+16] != 0x42` |
| `beq` at 0x9a7e8 | `[[sp-72]+16] == 0x46` | `[[sp-72]+16] != 0x46` |
| `beq` at 0x9a7f8 | `[[sp-72]+16] == 0x4a` | `[[sp-72]+16] != 0x4a` |
| `beq` at 0x9a808 | `[[sp-72]+16] == 0x4b` | `[[sp-72]+16] != 0x4b` |
| `beq` at 0x9a818 | `[[sp-72]+16] == 0x21` | `[[sp-72]+16] != 0x21` |
| `beq` at 0x9a828 | `[[sp-72]+16] == 0x5e` | `[[sp-72]+16] != 0x5e` |
| `beq` at 0x9a838 | `[[sp-72]+16] == 0x2b` | `[[sp-72]+16] != 0x2b` |
| `beq` at 0x9a848 | `[[sp-72]+16] == 0x4c` | `[[sp-72]+16] != 0x4c` |
| `beq` at 0x9a858 | `[[sp-72]+16] == 0x4d` | `[[sp-72]+16] != 0x4d` |
| `beq` at 0x9a86c | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x9a918 | `ret(0xfd4a8) != 0x0` | `ret(0xfd4a8) == 0x0` |
| `blt` at 0x9a978 | `[0x1a662c] < 0x0 (signed, 32-bit)` | `[0x1a662c] >= 0x0 (signed, 32-bit)` |
| `beq` at 0x9a98c | `ret(0xa92d8) == 0x0` | `ret(0xa92d8) != 0x0` |
| `bne` at 0x9a9a0 | `ret(0xa92d8) != 0x0` | `ret(0xa92d8) == 0x0` |
| `beq` at 0x9a9e0 | `ret(0xa92d8) == 0x0` | `ret(0xa92d8) != 0x0` |
| `beq` at 0x9a9f0 | `[[sp-68]+4] == 0x0` | `[[sp-68]+4] != 0x0` |
| `beq` at 0x9aa00 | `[[sp-68]+16] == 0x35` | `[[sp-68]+16] != 0x35` |
| `beq` at 0x9aa10 | `[[sp-68]+16] == 0x4f` | `[[sp-68]+16] != 0x4f` |
| `beq` at 0x9aa20 | `[[sp-68]+16] == 0x42` | `[[sp-68]+16] != 0x42` |
| `beq` at 0x9aa30 | `[[sp-68]+16] == 0x46` | `[[sp-68]+16] != 0x46` |
| `beq` at 0x9aa40 | `[[sp-68]+16] == 0x4a` | `[[sp-68]+16] != 0x4a` |
| `beq` at 0x9aa50 | `[[sp-68]+16] == 0x4b` | `[[sp-68]+16] != 0x4b` |
| `beq` at 0x9aa60 | `[[sp-68]+16] == 0x21` | `[[sp-68]+16] != 0x21` |
| `beq` at 0x9aa70 | `[[sp-68]+16] == 0x5e` | `[[sp-68]+16] != 0x5e` |
| `beq` at 0x9aa80 | `[[sp-68]+16] == 0x2b` | `[[sp-68]+16] != 0x2b` |
| `beq` at 0x9aa90 | `[[sp-68]+16] == 0x4c` | `[[sp-68]+16] != 0x4c` |
| `beq` at 0x9aaa0 | `[[sp-68]+16] == 0x4d` | `[[sp-68]+16] != 0x4d` |
| `bne` at 0x9ab50 | `ret(0xfd4a8) != 0x0` | `ret(0xfd4a8) == 0x0` |
| `beq` at 0x9ab70 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9abbc | `ret(0xa92d8) == 0x0` | `ret(0xa92d8) != 0x0` |
| `beq` at 0x9abcc | `[[sp-72]+4] == 0x0` | `[[sp-72]+4] != 0x0` |
| `beq` at 0x9abdc | `[[sp-72]+16] == 0x35` | `[[sp-72]+16] != 0x35` |
| `beq` at 0x9abec | `[[sp-72]+16] == 0x4f` | `[[sp-72]+16] != 0x4f` |
| `beq` at 0x9abfc | `[[sp-72]+16] == 0x42` | `[[sp-72]+16] != 0x42` |
| `beq` at 0x9ac0c | `[[sp-72]+16] == 0x46` | `[[sp-72]+16] != 0x46` |
| `beq` at 0x9ac1c | `[[sp-72]+16] == 0x4a` | `[[sp-72]+16] != 0x4a` |
| `beq` at 0x9ac2c | `[[sp-72]+16] == 0x4b` | `[[sp-72]+16] != 0x4b` |
| `beq` at 0x9ac3c | `[[sp-72]+16] == 0x21` | `[[sp-72]+16] != 0x21` |
| `beq` at 0x9ac4c | `[[sp-72]+16] == 0x5e` | `[[sp-72]+16] != 0x5e` |
| `beq` at 0x9ac5c | `[[sp-72]+16] == 0x2b` | `[[sp-72]+16] != 0x2b` |
| `beq` at 0x9ac6c | `[[sp-72]+16] == 0x4c` | `[[sp-72]+16] != 0x4c` |
| `beq` at 0x9ac7c | `[[sp-72]+16] == 0x4d` | `[[sp-72]+16] != 0x4d` |
| `bne` at 0x9ad2c | `ret(0xfd4a8) != 0x0` | `ret(0xfd4a8) == 0x0` |
| `beq` at 0x9ad4c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9ad98 | `r0` | `ret(0xfd4a8)` |

## `L1CfgLock`

- `L1CfgLock`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9adfc to 0x9afe8.
  Established. The call to unnamed 0x22ca0 has candidate values 0x1a68a0, 0x1f4 and two unknowns, with its result stored at sp-0x18 (`bl` at 0x9ae28, `str` at 0x9ae30);
  its zero-result branch targets 0x9ae9c, while the nonzero path falls through into the subsequent calls (`beq` at 0x9ae3c).
  Later word stores have unknown targets and values, so their locations cannot support a lock-state layout (`str` at 0x9aedc, `str` at 0x9af10).
  The final call to unnamed 0x20634 has candidate values 0x31b40c, the saved entry r0 and two unknowns (`bl` at 0x9afc0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9ae0c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x9ae20 | load | 4 | `abs 0x9afcc` | `-` |
| `str` at 0x9ae30 | store | 4 | `sp -0x18` | `ret(0x22ca0)` |
| `ldr` at 0x9ae34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9ae48 | load | 4 | `abs 0x9afd0` | `-` |
| `ldr` at 0x9ae58 | load | 4 | `abs 0x9afd4` | `-` |
| `ldr` at 0x9ae60 | load | 4 | `abs 0x9afd8` | `-` |
| `str` at 0x9ae64 | store | 4 | `sp -0xac` | `0x16a7d8` |
| `ldr` at 0x9ae68 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9ae6c | store | 4 | `sp -0xa8` | `[sp-24]` |
| `ldr` at 0x9ae70 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9ae74 | store | 4 | `sp -0xa4` | `[sp-20]` |
| `str` at 0x9ae7c | store | 4 | `sp -0xa0` | `sp-100` |
| `str` at 0x9ae84 | store | 4 | `sp -0x9c` | `sp-152` |
| `ldr` at 0x9ae8c | load | 4 | `abs 0x9afdc` | `-` |
| `ldr` at 0x9ae90 | load | 4 | `abs 0x9afe0` | `-` |
| `ldr` at 0x9aed0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9aed4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9aedc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x9af04 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9af08 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9af10 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x9af20 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9af44 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9af4c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x9af68 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9af74 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x9af84 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9afa8 | load | 4 | `abs 0x9afe4` | `-` |
| `ldr` at 0x9afac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9afb0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9afb8 | load | 4 | `abs 0x9afd0` | `-` |
| `ldr` at 0x9afbc | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x9afc8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9afc8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9afc8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9ae1c | `(unknown callee) 0x2043c` | `sp-32` | `0x0` | `r2` | `sp-32` |
| `bl` at 0x9ae28 | `(unknown callee) 0x22ca0` | `0x1a68a0` | `0x1f4` | `?` | `?` |
| `bl` at 0x9ae4c | `(unknown callee) 0x20634` | `sp-100` | `0x31b40c` | `?` | `sp-100` |
| `bl` at 0x9ae5c | `(unknown callee) 0x20634` | `sp-152` | `0x31b434` | `?` | `sp-152` |
| `bl` at 0x9ae98 | `(unknown callee) 0x22070` | `0x9` | `0x16a6dc` | `0x2f2` | `0x0` |
| `bl` at 0x9aea8 | `(unknown callee) 0x2043c` | `sp-40` | `0x0` | `?` | `sp-40` |
| `bl` at 0x9afb4 | `(unknown callee) 0x29908` | `0x16a810` | `?` | `?` | `?` |
| `bl` at 0x9afc0 | `(unknown callee) 0x20634` | `0x31b40c` | `[sp-20]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x9ae3c | `ret(0x22ca0) == 0x0` | `ret(0x22ca0) != 0x0` |
| `bge` at 0x9af28 | `? >= 0x0 (signed, 32-bit)` | `? < 0x0 (signed, 32-bit)` |
| `ble` at 0x9af8c | `? <= 0x1 (signed, 32-bit)` | `? > 0x1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9afc8 | `r0` | `ret(0x20634)` |

## `L1CfgUnlock`

- `L1CfgUnlock`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9afe8 to 0x9b070.
  Established. The first two calls to unnamed 0x20634 have candidate pairs 0x31b434 with 0x31b40c, then 0x31b40c with 0x16a834 (`bl` at 0x9b000, `bl` at 0x9b00c).
  The call to unnamed 0x2302c has candidate values 0x1a68a0 and three unknowns, and its result is stored at sp-0x14 (`bl` at 0x9b014, `str` at 0x9b01c).
  A zero result branches to 0x9b050, while a nonzero result falls through to the call to unnamed 0x22070 (`beq` at 0x9b028).
  The numeric callees do not establish a lock-release mechanism or runtime output.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x9aff8 | load | 4 | `abs 0x9b058` | `-` |
| `ldr` at 0x9affc | load | 4 | `abs 0x9b05c` | `-` |
| `ldr` at 0x9b004 | load | 4 | `abs 0x9b05c` | `-` |
| `ldr` at 0x9b008 | load | 4 | `abs 0x9b060` | `-` |
| `ldr` at 0x9b010 | load | 4 | `abs 0x9b064` | `-` |
| `str` at 0x9b01c | store | 4 | `sp -0x14` | `ret(0x2302c)` |
| `ldr` at 0x9b020 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9b02c | load | 4 | `abs 0x9b068` | `-` |
| `str` at 0x9b030 | store | 4 | `sp -0x1c` | `0x16a840` |
| `ldr` at 0x9b034 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9b038 | store | 4 | `sp -0x18` | `ret(0x2302c)` |
| `ldr` at 0x9b040 | load | 4 | `abs 0x9b06c` | `-` |
| `ldm` at 0x9b054 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9b054 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9b054 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9b000 | `(unknown callee) 0x20634` | `0x31b434` | `0x31b40c` | `r2` | `r3` |
| `bl` at 0x9b00c | `(unknown callee) 0x20634` | `0x31b40c` | `0x16a834` | `?` | `?` |
| `bl` at 0x9b014 | `(unknown callee) 0x2302c` | `0x1a68a0` | `?` | `?` | `?` |
| `bl` at 0x9b04c | `(unknown callee) 0x22070` | `0x9` | `0x16a6dc` | `0x308` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x9b028 | `ret(0x2302c) == 0x0` | `ret(0x2302c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9b054 | `r0` | `?` |

## `L1CfgDiscoveryResponse`

- `L1CfgDiscoveryResponse`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9b070 to 0x9b5b4.
  Established. A word at entry r0 plus 0xc is saved in memory at sp-0x1c (`ldr` at 0x9b088, `str` at 0x9b08c).
  The call to unnamed 0xfde7c has candidate values the word at sp-0x18, four and two unknowns, with its result stored at sp-0x28 (`bl` at 0x9b0f0, `str` at 0x9b0f4).
  A later word store copies the value from sp-0x28 into memory at offset 0x90 of the pointer from sp-0x24 (`str` at 0x9b100).
  The stores alone do not establish discovery-response contents or transport behavior.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9b080 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x9b084 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9b088 | load | 4 | `r0 +0xc` | `-` |
| `str` at 0x9b08c | store | 4 | `sp -0x1c` | `[r0+12]` |
| `ldr` at 0x9b090 | load | 4 | `abs 0x9b590` | `-` |
| `ldr` at 0x9b098 | load | 4 | `abs 0x9b594` | `-` |
| `ldr` at 0x9b09c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9b0b8 | load | 4 | `abs 0x1a6988` | `-` |
| `str` at 0x9b0c0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x9b0c4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x9b0c8 | load | 1 | `[sp-24] +0x44` | `-` |
| `ldr` at 0x9b0d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9b0dc | load | 4 | `[sp-24] +0x48` | `-` |
| `str` at 0x9b0e0 | store | 4 | `sp -0x28` | `[[sp-24]+72]` |
| `ldr` at 0x9b0e8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9b0f4 | store | 4 | `sp -0x28` | `ret(0xfde7c)` |
| `ldr` at 0x9b0f8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9b0fc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9b100 | store | 4 | `[sp-36] +0x90` | `[sp-40]` |
| `ldr` at 0x9b104 | load | 4 | `abs 0x9b598` | `-` |
| `ldr` at 0x9b108 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9b120 | load | 4 | `abs 0x9b59c` | `-` |
| `ldr` at 0x9b124 | load | 4 | `abs 0x9b594` | `-` |
| `ldr` at 0x9b128 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9b144 | load | 4 | `abs 0x1a6988` | `-` |
| `str` at 0x9b14c | store | 4 | `sp -0x34` | `ret(0x202c8)` |
| `ldr` at 0x9b150 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9b154 | load | 4 | `abs 0x9b5a0` | `-` |
| `ldr` at 0x9b158 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9b15c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9b164 | load | 4 | `abs 0x9b594` | `-` |
| `ldr` at 0x9b168 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9b184 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9b18c | load | 4 | `abs 0x9b594` | `-` |
| `ldr` at 0x9b190 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9b1ac | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9b1b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9b1b8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9b1c4 | load | 4 | `abs 0x9b594` | `-` |
| `ldr` at 0x9b1c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9b1e4 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9b1ec | load | 4 | `abs 0x9b594` | `-` |
| `ldr` at 0x9b1f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9b20c | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9b214 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9b230 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x9b234 | load | 4 | `abs 0x9b594` | `-` |
| `ldr` at 0x9b238 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9b254 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9b25c | load | 4 | `abs 0x9b594` | `-` |
| `ldr` at 0x9b260 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9b27c | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9b284 | load | 4 | `abs 0x9b594` | `-` |
| `ldr` at 0x9b288 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9b2a4 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9b2ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9b2c0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9b2d0 | store | 4 | `(unknown)` | `ret(0x203b8)` |
| `ldr` at 0x9b2d4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x9b2d8 | load | 1 | `[sp-24] +0x4c` | `-` |
| `ldr` at 0x9b2e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9b2ec | load | 4 | `[sp-24] +0x50` | `-` |
| `str` at 0x9b2f0 | store | 4 | `sp -0x2c` | `[[sp-24]+80]` |
| `ldr` at 0x9b2f8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9b304 | store | 4 | `sp -0x2c` | `ret(0xfde7c)` |
| `ldr` at 0x9b308 | load | 4 | `abs 0x9b594` | `-` |
| `ldr` at 0x9b30c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9b328 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9b330 | load | 4 | `abs 0x9b594` | `-` |
| `ldr` at 0x9b334 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9b350 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9b358 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9b36c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9b370 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9b37c | load | 4 | `abs 0x9b598` | `-` |
| `ldr` at 0x9b380 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9b390 | load | 4 | `abs 0x9b5a4` | `-` |
| `ldr` at 0x9b394 | load | 4 | `abs 0x1a0354` | `-` |
| `ldr` at 0x9b398 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9b3ac | load | 4 | `abs 0x9b59c` | `-` |
| `ldr` at 0x9b3b0 | load | 4 | `abs 0x9b594` | `-` |
| `ldr` at 0x9b3b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9b3d0 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9b3d8 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9b3dc | load | 4 | `abs 0x9b5a8` | `-` |
| `ldr` at 0x9b3e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9b3e4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9b3f0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x9b3f4 | load | 4 | `abs 0x9b594` | `-` |
| `ldr` at 0x9b3f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9b414 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9b41c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9b420 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9b42c | load | 4 | `abs 0x9b59c` | `-` |
| `ldr` at 0x9b430 | load | 4 | `abs 0x9b594` | `-` |
| `ldr` at 0x9b434 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9b450 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9b458 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9b46c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9b474 | load | 4 | `abs 0x9b594` | `-` |
| `ldr` at 0x9b478 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9b494 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9b49c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9b4b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9b4b8 | load | 4 | `abs 0x9b594` | `-` |
| `ldr` at 0x9b4bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9b4d8 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9b4e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9b4f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9b4fc | load | 4 | `(unknown)` | `-` |
| `str` at 0x9b500 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x9b504 | load | 4 | `abs 0x9b594` | `-` |
| `ldr` at 0x9b508 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9b524 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9b52c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9b540 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9b548 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9b54c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x9b550 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9b554 | load | 4 | `abs 0x9b5ac` | `-` |
| `ldr` at 0x9b558 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9b55c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9b564 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9b56c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0x9b574 | load | 4 | `abs 0x9b59c` | `-` |
| `ldr` at 0x9b578 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9b57c | load | 4 | `abs 0x9b5b0` | `-` |
| `ldm` at 0x9b58c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x9b58c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9b58c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9b58c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9b094 | `(unknown callee) 0x9adfc` | `0x16a864` | `r1` | `r2` | `[r0+12]` |
| `bl` at 0x9b0f0 | `(unknown callee) 0xfde7c` | `[sp-24]` | `0x4` | `?` | `?` |
| `bl` at 0x9b11c | `(unknown callee) 0x202c8` | `0x0` | `?` | `[sp-36]` | `?` |
| `bl` at 0x9b160 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16a87c` | `[sp-28]` | `?` |
| `bl` at 0x9b2c8 | `(unknown callee) 0x203b8` | `?` | `?` | `?` | `?` |
| `bl` at 0x9b300 | `(unknown callee) 0xfde7c` | `[sp-24]` | `0x5` | `?` | `?` |
| `bl` at 0x9b378 | `(unknown callee) 0x203f4` | `?` | `[sp-44]` | `?` | `?` |
| `bl` at 0x9b3e8 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16a8b4` | `[sp-28]` | `?` |
| `bl` at 0x9b560 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16a8c4` | `?` | `?` |
| `bl` at 0x9b580 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16a8d8` | `?` | `0x1a288c` |
| `bl` at 0x9b584 | `(unknown callee) 0x9c3e0` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x9b0d4 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x9b114 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x9b1c0 | `? <= ? (signed, 32-bit)` | `? > ? (signed, 32-bit)` |
| `bne` at 0x9b2e4 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x9b38c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9b3a8 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x9b428 | `? <= [sp-32] (signed, 32-bit)` | `? > [sp-32] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9b58c | `r0` | `ret(0x9c3e0)` |

## `L1CfgDiscoveryRequest`

- `L1CfgDiscoveryRequest`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9b5b4 to 0x9be44.
  Established. Two byte stores and one halfword store write zero through unknown effective addresses, while a word store through another unknown address carries 0x3e8 (`strb` at 0x9b654, `strb` at 0x9b674, `strh` at 0x9b6b4, `str` at 0x9b694).
  The call to unnamed 0x201fc has candidate values 0x18, sp-0x10, unknown and the word at sp-0x78 plus one (`bl` at 0x9b9dc);
  its result is saved in memory at sp-0x18 (`bl` at 0x9b9dc, `str` at 0x9b9e4).
  The unresolved access targets prevent these bytes and words from defining a discovery-request format.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9b5c4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9b5cc | store | 4 | `sp -0x78` | `0x0` |
| `ldr` at 0x9b5d0 | load | 4 | `abs 0x9be0c` | `-` |
| `ldr` at 0x9b5d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9b5f0 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9b5f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9b608 | load | 4 | `abs 0x9be10` | `-` |
| `ldr` at 0x9b60c | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9b61c | load | 4 | `abs 0x9be14` | `-` |
| `ldr` at 0x9b620 | load | 4 | `abs 0x9be0c` | `-` |
| `ldr` at 0x9b624 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9b628 | load | 4 | `abs 0x9be18` | `-` |
| `ldr` at 0x9b62c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9b630 | load | 4 | `abs 0x1a698c` | `-` |
| `ldr` at 0x9b638 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x9b654 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x9b658 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x9b674 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x9b678 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x9b694 | store | 4 | `(unknown)` | `0x3e8` |
| `ldr` at 0x9b698 | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x9b6b4 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x9b6b8 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x9b6c0 | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x9b6c4 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x9b6e0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x9b6e4 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x9b700 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x9b704 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x9b71c | load | 4 | `abs 0x9be0c` | `-` |
| `ldr` at 0x9b720 | load | 4 | `abs 0x1a6908` | `-` |
| `str` at 0x9b724 | store | 4 | `(unknown)` | `[0x1a6908]` |
| `ldr` at 0x9b728 | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x9b744 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x9b748 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x9b750 | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x9b754 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x9b770 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x9b774 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x9b790 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x9b794 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x9b7ac | load | 4 | `abs 0x9be0c` | `-` |
| `ldr` at 0x9b7b0 | load | 4 | `abs 0x1a690c` | `-` |
| `str` at 0x9b7b4 | store | 4 | `(unknown)` | `[0x1a690c]` |
| `ldr` at 0x9b7b8 | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x9b7d4 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x9b7d8 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x9b7e0 | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x9b7e4 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x9b800 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x9b804 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x9b820 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x9b824 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x9b83c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9b840 | store | 4 | `(unknown)` | `[sp-20]` |
| `ldr` at 0x9b844 | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x9b860 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x9b864 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x9b86c | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x9b870 | load | 4 | `abs 0x9be1c` | `-` |
| `ldr` at 0x9b878 | load | 4 | `abs 0x9be0c` | `-` |
| `ldr` at 0x9b87c | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0x9b888 | load | 4 | `abs 0x9be0c` | `-` |
| `ldr` at 0x9b88c | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0x9b898 | load | 4 | `abs 0x9be0c` | `-` |
| `ldr` at 0x9b89c | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0x9b8ac | load | 4 | `abs 0x9be0c` | `-` |
| `ldr` at 0x9b8b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9b8cc | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9b8d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9b8e0 | load | 4 | `abs 0x9be0c` | `-` |
| `ldr` at 0x9b8e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9b900 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9b908 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9b914 | load | 4 | `abs 0x9be0c` | `-` |
| `ldr` at 0x9b918 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9b934 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9b93c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9b94c | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x9b968 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x9b96c | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x9b988 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x9b98c | load | 4 | `sp -0x78` | `-` |
| `str` at 0x9b9a8 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x9b9ac | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x9b9c8 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x9b9cc | load | 4 | `sp -0x78` | `-` |
| `str` at 0x9b9d4 | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `str` at 0x9b9e4 | store | 4 | `sp -0x18` | `ret(0x201fc)` |
| `ldr` at 0x9b9e8 | load | 4 | `abs 0x9be0c` | `-` |
| `ldr` at 0x9b9ec | load | 4 | `abs 0x9be0c` | `-` |
| `ldr` at 0x9b9f0 | load | 4 | `abs 0x1a6908` | `-` |
| `ldr` at 0x9b9f4 | load | 4 | `abs 0x1a690c` | `-` |
| `str` at 0x9ba00 | store | 4 | `sp -0x7c` | `ret(0xa3c34)` |
| `ldr` at 0x9ba04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9ba08 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x9ba14 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x9ba30 | store | 1 | `(unknown)` | `0x9` |
| `ldr` at 0x9ba34 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x9ba50 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x9ba54 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x9ba6c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9ba70 | store | 4 | `(unknown)` | `[sp-24]` |
| `ldr` at 0x9ba74 | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x9ba90 | store | 2 | `(unknown)` | `0x18` |
| `ldr` at 0x9ba94 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x9ba9c | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x9baa4 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x9bac0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x9bac4 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x9bae0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x9bae4 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x9bafc | load | 4 | `abs 0x9be0c` | `-` |
| `ldr` at 0x9bb00 | load | 4 | `abs 0x1a698c` | `-` |
| `str` at 0x9bb04 | store | 4 | `(unknown)` | `[0x1a698c]` |
| `ldr` at 0x9bb08 | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x9bb24 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x9bb28 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x9bb30 | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x9bb34 | load | 4 | `abs 0x9be0c` | `-` |
| `ldr` at 0x9bb38 | load | 4 | `abs 0x1a698c` | `-` |
| `str` at 0x9bb58 | store | 4 | `sp -0x18` | `ret(0x201fc)` |
| `ldr` at 0x9bb5c | load | 4 | `abs 0x9be0c` | `-` |
| `ldr` at 0x9bb60 | load | 4 | `abs 0x9be0c` | `-` |
| `ldr` at 0x9bb64 | load | 4 | `abs 0x1a698c` | `-` |
| `ldr` at 0x9bb78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9bb7c | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9bb88 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x9bba4 | store | 1 | `(unknown)` | `0x9` |
| `ldr` at 0x9bba8 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x9bbc4 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x9bbc8 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x9bbe0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9bbe4 | store | 4 | `(unknown)` | `[sp-24]` |
| `ldr` at 0x9bbe8 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x9bc00 | load | 4 | `abs 0x9be0c` | `-` |
| `ldr` at 0x9bc04 | load | 4 | `abs 0x1a698c` | `-` |
| `strh` at 0x9bc18 | store | 2 | `(unknown)` | `?` |
| `ldr` at 0x9bc1c | load | 4 | `sp -0x78` | `-` |
| `str` at 0x9bc24 | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x9bc2c | load | 4 | `abs 0x9be20` | `-` |
| `str` at 0x9bc30 | store | 4 | `sp -0x1c` | `0xffff00e` |
| `ldr` at 0x9bc34 | load | 4 | `abs 0x9be10` | `-` |
| `ldr` at 0x9bc38 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9bc48 | load | 4 | `abs 0x9be24` | `-` |
| `ldr` at 0x9bc4c | load | 4 | `abs 0x1a0354` | `-` |
| `ldr` at 0x9bc50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9bc64 | load | 4 | `abs 0x9be14` | `-` |
| `ldr` at 0x9bc68 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9bc6c | load | 4 | `abs 0x9be28` | `-` |
| `ldr` at 0x9bc70 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9bc7c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x9bc80 | load | 4 | `abs 0x9be0c` | `-` |
| `ldr` at 0x9bc84 | load | 4 | `abs 0x1a698c` | `-` |
| `ldr` at 0x9bc88 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9bc94 | load | 4 | `abs 0x9be14` | `-` |
| `ldr` at 0x9bc98 | load | 4 | `abs 0x9be0c` | `-` |
| `ldr` at 0x9bc9c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9bcb0 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9bcb8 | load | 4 | `abs 0x9be0c` | `-` |
| `ldr` at 0x9bcbc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9bcd0 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9bcd8 | load | 4 | `abs 0x9be0c` | `-` |
| `ldr` at 0x9bcdc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9bcf0 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9bcf8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9bcfc | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x9bd00 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9bd04 | load | 4 | `abs 0x9be2c` | `-` |
| `ldr` at 0x9bd08 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9bd0c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9bd14 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9bd1c | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x9bd24 | load | 4 | `abs 0x9be14` | `-` |
| `ldr` at 0x9bd28 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9bd2c | load | 4 | `abs 0x9be30` | `-` |
| `str` at 0x9bd3c | store | 4 | `sp -0x8c` | `0x0` |
| `str` at 0x9bd44 | store | 4 | `sp -0x88` | `0x2` |
| `ldr` at 0x9bd48 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x9bd4c | store | 4 | `sp -0x84` | `[sp-120]` |
| `str` at 0x9bd54 | store | 4 | `sp -0x80` | `sp-116` |
| `ldr` at 0x9bd58 | load | 4 | `abs 0x9be34` | `-` |
| `str` at 0x9bd70 | store | 4 | `sp -0x20` | `ret(0xfd4a8)` |
| `ldr` at 0x9bd74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9bd80 | load | 4 | `abs 0x9be14` | `-` |
| `ldr` at 0x9bd84 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9bd88 | load | 4 | `abs 0x9be38` | `-` |
| `ldr` at 0x9bd94 | load | 4 | `abs 0x9be3c` | `-` |
| `ldr` at 0x9bd98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9bdac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9bdb8 | load | 4 | `abs 0x9be3c` | `-` |
| `ldr` at 0x9bdbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9bdd0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9bddc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9bdf0 | load | 4 | `abs 0x9be40` | `-` |
| `ldr` at 0x9bdf4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9bdfc | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x9be08 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9be08 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9be08 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9b634 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16a8dc` | `[sp-20]` | `[0x1a698c]` |
| `bl` at 0x9b874 | `(unknown callee) 0x9adfc` | `0x16a90c` | `sp-16` | `?` | `[sp-120]+1` |
| `bl` at 0x9b9dc | `(unknown callee) 0x201fc` | `0x18` | `sp-16` | `?` | `[sp-120]+1` |
| `bl` at 0x9b9f8 | `(unknown callee) 0xa3c34` | `[0x1a6908]` | `[0x1a690c]` | `0x1a6900` | `0x1a6900` |
| `bl` at 0x9ba10 | `(unknown callee) 0x203f4` | `[sp-24]` | `ret(0xa3c34)` | `0x18` | `ret(0xa3c34)` |
| `bl` at 0x9bb50 | `(unknown callee) 0x201fc` | `?` | `sp-16` | `[0x1a698c]` | `?` |
| `bl` at 0x9bb84 | `(unknown callee) 0x203f4` | `ret(0x201fc)` | `[0x1a6994]` | `?` | `?` |
| `bl` at 0x9bc28 | `(unknown callee) 0x9afe8` | `ret(0x203f4)` | `?` | `?` | `[sp-120]+1` |
| `bl` at 0x9bc74 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16a924` | `[sp-20]` | `0x1a288c` |
| `bl` at 0x9bd10 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16a930` | `?` | `?` |
| `bl` at 0x9bd30 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16a8d8` | `[0x1a698c]` | `0x1a288c` |
| `bl` at 0x9bd68 | `(unknown callee) 0xfd4a8` | `0x1a68f8` | `sp-28` | `0x0` | `0x0` |
| `bl` at 0x9bd8c | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16a940` | `?` | `0x1a288c` |
| `bl` at 0x9bde0 | `(unknown callee) 0xfcb5c` | `?` | `ret(0xfd4a8)` | `0x31b45c` | `?` |
| `bl` at 0x9bdf8 | `(unknown callee) 0x29908` | `0x16a974` | `[sp-20]` | `?` | `ret(0xfcb5c)` |
| `bl` at 0x9be00 | `(unknown callee) 0xfb794` | `[sp-32]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x9b600 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9b618 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9b884 | `[0x1a6910] == 0x52` | `[0x1a6910] != 0x52` |
| `beq` at 0x9b894 | `[0x1a6910] == 0x44` | `[0x1a6910] != 0x44` |
| `beq` at 0x9b8a4 | `[0x1a6910] == 0x2d` | `[0x1a6910] != 0x2d` |
| `beq` at 0x9b8dc | `? == 0x52` | `? != 0x52` |
| `beq` at 0x9b910 | `? == 0x44` | `? != 0x44` |
| `beq` at 0x9b944 | `? == 0x2d` | `? != 0x2d` |
| `beq` at 0x9bc44 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9bc60 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x9bc90 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x9bd7c | `ret(0xfd4a8) != 0x0` | `ret(0xfd4a8) == 0x0` |
| `beq` at 0x9bdb4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9bdec | `ret(0xfcb5c) == 0x0` | `ret(0xfcb5c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9be08 | `r0` | `?` |

## `L1CfgUpdate`

- `L1CfgUpdate`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9be44 to 0x9c10c.
  Established. Byte stores clear sp-0x94 and sp-0x93, while a halfword store clears sp-0x92 and a word store puts one at sp-0x90 (`strb` at 0x9be68, `strb` at 0x9be70, `strh` at 0x9be80, `str` at 0x9be78).
  The call to unnamed 0xa0cd4 has candidate values sp-0x98, sp-0x94, 0x1a6900 and one (`bl` at 0x9bea8).
  A call to unnamed 0x202c8 returns a value stored at absolute 0x1a6648 (`bl` at 0x9c058, `str` at 0x9c05c).
  These software updates do not establish a configuration exchange on physical hardware.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9be58 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0x9be5c | load | 4 | `abs 0x9c0ec` | `-` |
| `strb` at 0x9be68 | store | 1 | `sp -0x94` | `0x0` |
| `strb` at 0x9be70 | store | 1 | `sp -0x93` | `0x0` |
| `str` at 0x9be78 | store | 4 | `sp -0x90` | `0x1` |
| `strh` at 0x9be80 | store | 2 | `sp -0x92` | `0x0` |
| `ldr` at 0x9be84 | load | 4 | `sp -0x98` | `-` |
| `str` at 0x9be8c | store | 4 | `sp -0x98` | `[sp-152]+1` |
| `ldr` at 0x9bea0 | load | 4 | `abs 0x9c0f0` | `-` |
| `str` at 0x9beb0 | store | 4 | `sp -0x9c` | `0x0` |
| `ldr` at 0x9beb4 | load | 4 | `abs 0x9c0f0` | `-` |
| `ldr` at 0x9beb8 | load | 4 | `abs 0x1a6984` | `-` |
| `ldr` at 0x9bebc | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9bec8 | load | 4 | `abs 0x9c0f0` | `-` |
| `ldr` at 0x9becc | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9bee8 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9bef0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9befc | load | 4 | `abs 0x9c0f0` | `-` |
| `ldr` at 0x9bf00 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9bf1c | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9bf24 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9bf34 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x9bf38 | load | 4 | `abs 0x9c0f0` | `-` |
| `ldr` at 0x9bf3c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9bf58 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9bf60 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9bf64 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x9bf70 | load | 4 | `abs 0x9c0f0` | `-` |
| `ldr` at 0x9bf74 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9bf90 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9bf98 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x9bfac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9bfb4 | load | 4 | `abs 0x9c0f0` | `-` |
| `ldr` at 0x9bfb8 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9bfd4 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9bfdc | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x9bff0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9bff8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9bffc | load | 4 | `(unknown)` | `-` |
| `str` at 0x9c004 | store | 4 | `sp -0xa4` | `ret(0xa3a70)` |
| `ldr` at 0x9c008 | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x9c024 | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x9c030 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x9c038 | store | 4 | `sp -0xa0` | `[sp-160]+1` |
| `ldr` at 0x9c040 | load | 4 | `sp -0x9c` | `-` |
| `str` at 0x9c048 | store | 4 | `sp -0x9c` | `[sp-156]+1` |
| `ldr` at 0x9c050 | load | 4 | `abs 0x9c0f4` | `-` |
| `str` at 0x9c05c | store | 4 | `abs 0x1a6648` | `ret(0x202c8)` |
| `strb` at 0x9c068 | store | 1 | `sp -0x94` | `0x0` |
| `strb` at 0x9c070 | store | 1 | `sp -0x93` | `0x0` |
| `ldr` at 0x9c074 | load | 4 | `sp -0x98` | `-` |
| `str` at 0x9c07c | store | 4 | `sp -0x90` | `[sp-152]-1` |
| `strh` at 0x9c084 | store | 2 | `sp -0x92` | `0x0` |
| `ldr` at 0x9c088 | load | 4 | `abs 0x9c0f8` | `-` |
| `ldr` at 0x9c08c | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9c0a8 | load | 4 | `abs 0x9c0fc` | `-` |
| `ldr` at 0x9c0ac | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9c0b0 | load | 4 | `abs 0x9c100` | `-` |
| `ldr` at 0x9c0b4 | load | 4 | `sp -0x98` | `-` |
| `str` at 0x9c0c4 | store | 4 | `sp -0xac` | `sp-148` |
| `str` at 0x9c0cc | store | 4 | `sp -0xa8` | `0x0` |
| `ldr` at 0x9c0d0 | load | 4 | `abs 0x9c104` | `-` |
| `ldr` at 0x9c0d8 | load | 4 | `abs 0x9c108` | `-` |
| `ldr` at 0x9c0dc | load | 4 | `sp -0x98` | `-` |
| `ldm` at 0x9c0e8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x9c0e8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9c0e8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9c0e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9be60 | `(unknown callee) 0x9adfc` | `0x16a9a4` | `r1` | `r2` | `0x0` |
| `bl` at 0x9bea8 | `(unknown callee) 0xa0cd4` | `sp-152` | `sp-148` | `0x1a6900` | `0x1` |
| `bl` at 0x9c000 | `(unknown callee) 0xa3a70` | `?` | `?` | `?` | `?` |
| `bl` at 0x9c02c | `(unknown callee) 0xa0cd4` | `sp-152` | `sp-148` | `ret(0xa3a70)` | `0x1` |
| `bl` at 0x9c058 | `(unknown callee) 0x202c8` | `0x0` | `?` | `[0x1a6984]` | `?` |
| `bl` at 0x9c060 | `(unknown callee) 0x9afe8` | `ret(0x202c8)` | `?` | `?` | `?` |
| `bl` at 0x9c0a0 | `(unknown callee) 0x202c8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x9c0bc | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16a9b0` | `[sp-152]` | `ret(0x202c8)` |
| `bl` at 0x9c0e0 | `(unknown callee) 0xfd284` | `0x1a68f8` | `0x0` | `0x3e9` | `[sp-152]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x9bec4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x9bef8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9bf2c | `? == 0x0` | `? != 0x0` |
| `ble` at 0x9bf6c | `? <= [sp-160] (signed, 32-bit)` | `? > [sp-160] (signed, 32-bit)` |
| `beq` at 0x9c010 | `ret(0xa3a70) == 0x0` | `ret(0xa3a70) != 0x0` |
| `beq` at 0x9c098 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9c0e8 | `r0` | `ret(0xfd284)` |

## `L1CfgOptimize`

- `L1CfgOptimize`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9c3e0 to 0x9d698.
  Established. A word store puts the value loaded from absolute 0x1a6650 plus five at absolute 0x1a6654 (`ldr` at 0x9c498, `str` at 0x9c4a0).
  The call to unnamed 0x9e6d0 has candidate values the words at absolute 0x1a6994 and 0x1a698c, then two words from the pointer at sp-0x98 (`bl` at 0x9c74c);
  its result is saved at sp-0x9c, and the following store at offset 0x10 from that result receives the word at sp-0x80 (`bl` at 0x9c74c, `str` at 0x9c754, `str` at 0x9c788).
  The later decrement of the word at absolute 0x1a6998 does not establish a hardware topology optimization rule (`str` at 0x9d57c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x9c3f0 | load | 4 | `abs 0x9d3b0` | `-` |
| `ldr` at 0x9c3f4 | load | 4 | `abs 0x1a6654` | `-` |
| `ldr` at 0x9c400 | load | 4 | `abs 0x9d3b4` | `-` |
| `ldr` at 0x9c404 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9c414 | load | 4 | `abs 0x9d3b8` | `-` |
| `ldr` at 0x9c418 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9c41c | load | 4 | `abs 0x9d3bc` | `-` |
| `ldr` at 0x9c420 | load | 4 | `abs 0x9d3c0` | `-` |
| `ldr` at 0x9c430 | load | 4 | `abs 0x9d3c4` | `-` |
| `ldr` at 0x9c434 | load | 4 | `abs 0x1a6650` | `-` |
| `ldr` at 0x9c44c | load | 4 | `abs 0x9d3c4` | `-` |
| `ldr` at 0x9c450 | load | 4 | `abs 0x1a6650` | `-` |
| `str` at 0x9c458 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x9c45c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x9c468 | load | 4 | `abs 0x9d3b4` | `-` |
| `ldr` at 0x9c46c | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9c47c | load | 4 | `abs 0x9d3b8` | `-` |
| `ldr` at 0x9c480 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9c484 | load | 4 | `abs 0x9d3c8` | `-` |
| `ldr` at 0x9c488 | load | 4 | `abs 0x9d3c0` | `-` |
| `ldr` at 0x9c490 | load | 4 | `abs 0x9d3b0` | `-` |
| `ldr` at 0x9c494 | load | 4 | `abs 0x9d3c4` | `-` |
| `ldr` at 0x9c498 | load | 4 | `abs 0x1a6650` | `-` |
| `str` at 0x9c4a0 | store | 4 | `abs 0x1a6654` | `[0x1a6650]+5` |
| `ldr` at 0x9c4b8 | load | 4 | `abs 0x9d3c4` | `-` |
| `str` at 0x9c4bc | store | 4 | `abs 0x1a6650` | `ret(0x202c8)` |
| `ldr` at 0x9c4c0 | load | 4 | `abs 0x9d3cc` | `-` |
| `ldr` at 0x9c4c4 | load | 4 | `abs 0x1a6890` | `-` |
| `ldr` at 0x9c4d0 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9c4d4 | load | 4 | `abs 0x9d3cc` | `-` |
| `ldr` at 0x9c4d8 | load | 4 | `abs 0x1a698c` | `-` |
| `ldr` at 0x9c4dc | load | 4 | `abs 0x1a6890` | `-` |
| `ldr` at 0x9c4ec | load | 4 | `abs 0x9d3d0` | `-` |
| `ldr` at 0x9c4f0 | load | 4 | `abs 0x1a688c` | `-` |
| `ldr` at 0x9c4fc | load | 4 | `abs 0x9d3d0` | `-` |
| `ldr` at 0x9c500 | load | 4 | `abs 0x1a688c` | `-` |
| `ldr` at 0x9c508 | load | 4 | `abs 0x9d3cc` | `-` |
| `ldr` at 0x9c50c | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9c510 | load | 4 | `abs 0x1a698c` | `-` |
| `str` at 0x9c52c | store | 4 | `abs 0x1a6890` | `?` |
| `ldr` at 0x9c530 | load | 4 | `abs 0x9d3d0` | `-` |
| `ldr` at 0x9c534 | load | 4 | `abs 0x9d3cc` | `-` |
| `ldr` at 0x9c538 | load | 4 | `abs 0x1a6890` | `-` |
| `str` at 0x9c558 | store | 4 | `abs 0x1a688c` | `ret(0x201fc)` |
| `ldr` at 0x9c55c | load | 4 | `abs 0x9d3d0` | `-` |
| `ldr` at 0x9c560 | load | 4 | `abs 0x1a688c` | `-` |
| `ldr` at 0x9c570 | load | 4 | `abs 0x9d3d0` | `-` |
| `ldr` at 0x9c574 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9c578 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9c57c | load | 4 | `abs 0x1a698c` | `-` |
| `ldr` at 0x9c590 | load | 4 | `abs 0x1a688c` | `-` |
| `ldr` at 0x9c594 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9c5a0 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9c5a4 | load | 4 | `abs 0x1a698c` | `-` |
| `str` at 0x9c5a8 | store | 4 | `sp -0x7c` | `[0x1a698c]` |
| `ldr` at 0x9c5ac | load | 4 | `abs 0x9d3b4` | `-` |
| `ldr` at 0x9c5b0 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9c5c0 | load | 4 | `abs 0x9d3b8` | `-` |
| `ldr` at 0x9c5c4 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9c5c8 | load | 4 | `abs 0x9d3d4` | `-` |
| `str` at 0x9c5d8 | store | 4 | `sp -0x80` | `0x0` |
| `ldr` at 0x9c5dc | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9c5e0 | load | 4 | `abs 0x1a6984` | `-` |
| `ldr` at 0x9c5e4 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x9c5f0 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9c5f4 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x9c610 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9c618 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9c624 | load | 4 | `abs 0x9d3b4` | `-` |
| `ldr` at 0x9c628 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9c638 | load | 4 | `abs 0x9d3b8` | `-` |
| `ldr` at 0x9c63c | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9c640 | load | 4 | `abs 0x9d3d8` | `-` |
| `ldr` at 0x9c644 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x9c650 | load | 4 | `abs 0x9d3b4` | `-` |
| `ldr` at 0x9c654 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9c664 | load | 4 | `abs 0x9d3b8` | `-` |
| `ldr` at 0x9c668 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9c66c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x9c688 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9c690 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9c694 | load | 4 | `abs 0x9d3dc` | `-` |
| `ldr` at 0x9c698 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x9c69c | load | 4 | `(unknown)` | `-` |
| `str` at 0x9c6a8 | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x9c6ac | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9c6b0 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x9c6cc | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9c6d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9c6d8 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x9c6e4 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9c6e8 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x9c704 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9c70c | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x9c720 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9c728 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x9c72c | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9c730 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9c734 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x9c738 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x9c73c | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9c740 | load | 4 | `abs 0x1a698c` | `-` |
| `ldr` at 0x9c744 | load | 4 | `[sp-152] +0x0` | `-` |
| `ldr` at 0x9c748 | load | 4 | `[sp-152] +0x4` | `-` |
| `str` at 0x9c754 | store | 4 | `sp -0x9c` | `ret(0x9e6d0)` |
| `ldr` at 0x9c758 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9c764 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x9c768 | load | 4 | `[sp-152] +0x14` | `-` |
| `ldr` at 0x9c770 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9c774 | load | 4 | `ret(0x9e6d0) +0x14` | `-` |
| `ldr` at 0x9c780 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9c784 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x9c788 | store | 4 | `ret(0x9e6d0) +0x10` | `[sp-128]` |
| `ldr` at 0x9c78c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9c790 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x9c794 | load | 4 | `[sp-152] +0x14` | `-` |
| `str` at 0x9c79c | store | 4 | `[sp-156] +0x14` | `[[sp-152]+20]+1` |
| `ldr` at 0x9c7a0 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9c7a4 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x9c7a8 | load | 4 | `[sp-152] +0xc` | `-` |
| `str` at 0x9c7ac | store | 4 | `[sp-156] +0xc` | `[[sp-152]+12]` |
| `ldr` at 0x9c7b0 | load | 4 | `abs 0x9d3b4` | `-` |
| `ldr` at 0x9c7b4 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9c7c4 | load | 4 | `abs 0x9d3b8` | `-` |
| `ldr` at 0x9c7c8 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9c7cc | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9c7d0 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9c7d4 | load | 4 | `[sp-156] +0x10` | `-` |
| `str` at 0x9c7dc | store | 4 | `sp -0xc0` | `[[sp-156]+16]+64` |
| `ldr` at 0x9c7e0 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9c7e4 | load | 4 | `[sp-156] +0x10` | `-` |
| `str` at 0x9c7e8 | store | 4 | `sp -0xbc` | `[[sp-156]+16]` |
| `ldr` at 0x9c7ec | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9c7f0 | load | 4 | `[sp-156] +0x14` | `-` |
| `str` at 0x9c7f4 | store | 4 | `sp -0xb8` | `[[sp-156]+20]` |
| `ldr` at 0x9c7f8 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9c7fc | load | 4 | `abs 0x9d3e0` | `-` |
| `ldr` at 0x9c800 | load | 4 | `[sp-156] +0x0` | `-` |
| `ldr` at 0x9c804 | load | 4 | `[sp-156] +0x4` | `-` |
| `ldr` at 0x9c810 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x9c814 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9c818 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x9c834 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9c83c | load | 4 | `[sp-152] +0x10` | `-` |
| `ldr` at 0x9c840 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9c84c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x9c850 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x9c854 | load | 4 | `[sp-152] +0x4` | `-` |
| `str` at 0x9c858 | store | 4 | `sp -0xc0` | `[[sp-152]+4]` |
| `ldr` at 0x9c85c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x9c860 | load | 4 | `[sp-152] +0x8` | `-` |
| `str` at 0x9c864 | store | 4 | `sp -0xbc` | `[[sp-152]+8]` |
| `ldr` at 0x9c868 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x9c86c | load | 4 | `[sp-152] +0xc` | `-` |
| `str` at 0x9c870 | store | 4 | `sp -0xb8` | `[[sp-152]+12]` |
| `ldr` at 0x9c874 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x9c878 | load | 4 | `[sp-152] +0x14` | `-` |
| `str` at 0x9c880 | store | 4 | `sp -0xb4` | `[[sp-152]+20]+1` |
| `ldr` at 0x9c884 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x9c888 | store | 4 | `sp -0xb0` | `[sp-128]` |
| `ldr` at 0x9c88c | load | 4 | `abs 0x9d3e4` | `-` |
| `ldr` at 0x9c890 | load | 4 | `abs 0x9d3e8` | `-` |
| `ldr` at 0x9c894 | load | 4 | `abs 0x9d3ec` | `-` |
| `ldr` at 0x9c898 | load | 4 | `[sp-152] +0x0` | `-` |
| `ldr` at 0x9c8a4 | load | 4 | `abs 0x9d3b4` | `-` |
| `ldr` at 0x9c8a8 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9c8b8 | load | 4 | `abs 0x9d3b8` | `-` |
| `ldr` at 0x9c8bc | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x9c8c0 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x9c8c4 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x9c8cc | store | 4 | `sp -0xc0` | `[sp-128]+64` |
| `ldr` at 0x9c8d0 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9c8d4 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x9c8f0 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9c8f8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9c900 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x9c904 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x9c908 | load | 4 | `[sp-152] +0x10` | `-` |
| `str` at 0x9c910 | store | 4 | `sp -0xb8` | `[[sp-152]+16]+64` |
| `ldr` at 0x9c914 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9c918 | load | 4 | `abs 0x9d3f0` | `-` |
| `ldr` at 0x9c91c | load | 4 | `[sp-152] +0x0` | `-` |
| `ldr` at 0x9c920 | load | 4 | `[sp-152] +0x4` | `-` |
| `ldr` at 0x9c928 | load | 4 | `sp -0x84` | `-` |
| `str` at 0x9c930 | store | 4 | `sp -0x84` | `[sp-132]+1` |
| `ldr` at 0x9c938 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x9c940 | store | 4 | `sp -0x80` | `[sp-128]+1` |
| `str` at 0x9c94c | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x9c950 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9c954 | load | 4 | `abs 0x1a698c` | `-` |
| `ldr` at 0x9c958 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x9c964 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9c968 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x9c97c | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9c984 | load | 4 | `abs 0x9d3f4` | `-` |
| `ldr` at 0x9c988 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9c98c | load | 4 | `abs 0x1a0350` | `-` |
| `ldr` at 0x9c998 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9c99c | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9c9a0 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9c9a4 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x9c9b8 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9c9c0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9c9c4 | store | 4 | `sp -0xc0` | `?` |
| `ldr` at 0x9c9c8 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9c9cc | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x9c9e0 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9c9e8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9c9ec | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x9c9f0 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9c9f4 | load | 4 | `abs 0x9d3e8` | `-` |
| `ldr` at 0x9c9f8 | load | 4 | `abs 0x1a69a0` | `-` |
| `ldr` at 0x9c9fc | load | 4 | `abs 0x9d3f8` | `-` |
| `ldr` at 0x9ca04 | load | 4 | `sp -0x84` | `-` |
| `str` at 0x9ca0c | store | 4 | `sp -0x84` | `?` |
| `ldr` at 0x9ca14 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9ca18 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9ca1c | load | 4 | `abs 0x1a6908` | `-` |
| `ldr` at 0x9ca20 | load | 4 | `abs 0x1a690c` | `-` |
| `str` at 0x9ca2c | store | 4 | `sp -0x90` | `ret(0xa3c34)` |
| `ldr` at 0x9ca30 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9ca34 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x9ca3c | store | 4 | `ret(0xa3c34) +0xc` | `0x0` |
| `str` at 0x9ca44 | store | 4 | `abs 0x1a6974` | `0x0` |
| `ldr` at 0x9ca48 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9ca4c | load | 4 | `abs 0x1a6958` | `-` |
| `ldr` at 0x9ca58 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9ca5c | load | 4 | `sp -0x90` | `-` |
| `str` at 0x9ca64 | store | 4 | `[sp-144] +0xc` | `?` |
| `str` at 0x9ca6c | store | 4 | `abs 0x1a6974` | `?` |
| `ldr` at 0x9ca70 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9ca74 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9ca78 | load | 4 | `abs 0x1a6908` | `-` |
| `str` at 0x9ca7c | store | 4 | `abs 0x1a6978` | `[0x1a6908]` |
| `ldr` at 0x9ca80 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9ca84 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9ca88 | load | 4 | `abs 0x1a690c` | `-` |
| `str` at 0x9ca8c | store | 4 | `abs 0x1a697c` | `[0x1a690c]` |
| `ldr` at 0x9ca94 | load | 4 | `abs 0x9d438` | `-` |
| `str` at 0x9ca9c | store | 4 | `abs 0x1a6958` | `0x0` |
| `str` at 0x9caa8 | store | 4 | `sp -0x94` | `ret(0x9e944)` |
| `ldr` at 0x9caac | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x9cab8 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x9cabc | load | 4 | `ret(0x9e944) +0xc` | `-` |
| `ldr` at 0x9cac8 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x9cacc | load | 4 | `[sp-148] +0xc` | `-` |
| `ldr` at 0x9cad8 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x9cadc | load | 4 | `[sp-148] +0x8` | `-` |
| `ldr` at 0x9cae8 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x9caec | load | 4 | `[sp-148] +0x8` | `-` |
| `ldr` at 0x9caf8 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x9cafc | load | 4 | `[sp-148] +0x8` | `-` |
| `ldr` at 0x9cb0c | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9cb10 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x9cb14 | load | 4 | `[sp-148] +0x10` | `-` |
| `str` at 0x9cb18 | store | 4 | `abs 0x1a6958` | `[[sp-148]+16]` |
| `ldr` at 0x9cb1c | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x9cb20 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x9cb24 | load | 4 | `[sp-148] +0x10` | `-` |
| `str` at 0x9cb28 | store | 4 | `[sp-144] +0xc` | `[[sp-148]+16]` |
| `ldr` at 0x9cb2c | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9cb30 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x9cb34 | load | 4 | `[sp-148] +0x10` | `-` |
| `str` at 0x9cb38 | store | 4 | `abs 0x1a6974` | `[[sp-148]+16]` |
| `ldr` at 0x9cb3c | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9cb40 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x9cb44 | load | 4 | `[sp-148] +0x0` | `-` |
| `str` at 0x9cb48 | store | 4 | `abs 0x1a6978` | `[[sp-148]]` |
| `ldr` at 0x9cb4c | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9cb50 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x9cb54 | load | 4 | `[sp-148] +0x4` | `-` |
| `str` at 0x9cb58 | store | 4 | `abs 0x1a697c` | `[[sp-148]+4]` |
| `ldr` at 0x9cb5c | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9cb60 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x9cb64 | load | 4 | `[sp-148] +0x14` | `-` |
| `str` at 0x9cb68 | store | 4 | `abs 0x1a6980` | `[[sp-148]+20]` |
| `ldr` at 0x9cb6c | load | 4 | `abs 0x9d3b4` | `-` |
| `ldr` at 0x9cb70 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9cb80 | load | 4 | `abs 0x9d3b8` | `-` |
| `ldr` at 0x9cb84 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9cb88 | load | 4 | `abs 0x9d3fc` | `-` |
| `str` at 0x9cb94 | store | 4 | `sp -0xa4` | `0x0` |
| `ldr` at 0x9cb98 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9cb9c | load | 4 | `abs 0x1a698c` | `-` |
| `ldr` at 0x9cba0 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x9cbac | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9cbb0 | load | 4 | `abs 0x9d3d0` | `-` |
| `ldr` at 0x9cbb4 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9cbb8 | load | 4 | `abs 0x1a698c` | `-` |
| `ldr` at 0x9cbcc | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9cbd0 | load | 4 | `abs 0x1a688c` | `-` |
| `str` at 0x9cbf0 | store | 4 | `sp -0xa4` | `0x1` |
| `ldr` at 0x9cbf4 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x9cbf8 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x9cbfc | load | 4 | `abs 0x9d3b4` | `-` |
| `ldr` at 0x9cc00 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9cc10 | load | 4 | `abs 0x9d3b8` | `-` |
| `ldr` at 0x9cc14 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9cc18 | load | 4 | `abs 0x9d400` | `-` |
| `ldr` at 0x9cc1c | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x9cc24 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x9cc30 | load | 4 | `abs 0x9d3b4` | `-` |
| `ldr` at 0x9cc34 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9cc44 | load | 4 | `abs 0x9d3b8` | `-` |
| `ldr` at 0x9cc48 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9cc4c | load | 4 | `abs 0x9d404` | `-` |
| `str` at 0x9cc58 | store | 4 | `sp -0x8c` | `0x0` |
| `ldr` at 0x9cc5c | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9cc60 | load | 4 | `abs 0x1a698c` | `-` |
| `ldr` at 0x9cc64 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x9cc70 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x9cc78 | store | 4 | `sp -0x8c` | `0x1` |
| `ldr` at 0x9cc80 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9cc88 | load | 4 | `abs 0x9d408` | `-` |
| `ldr` at 0x9cc8c | load | 4 | `abs 0x1a698c` | `-` |
| `ldr` at 0x9cc90 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x9cca4 | load | 4 | `abs 0x9d3b4` | `-` |
| `ldr` at 0x9cca8 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9ccb8 | load | 4 | `abs 0x9d3b8` | `-` |
| `ldr` at 0x9ccc0 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9ccc4 | load | 4 | `abs 0x9d40c` | `-` |
| `str` at 0x9ccd0 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x9ccd4 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9ccd8 | load | 4 | `abs 0x1a698c` | `-` |
| `ldr` at 0x9ccdc | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x9cce8 | load | 4 | `abs 0x9d3d0` | `-` |
| `ldr` at 0x9ccec | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9ccf0 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x9cd04 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9cd0c | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9cd10 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x9cd24 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9cd2c | load | 4 | `abs 0x1a688c` | `-` |
| `ldr` at 0x9cd30 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x9cd34 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9cd38 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9cd44 | store | 4 | `sp -0x9c` | `ret(0x9e6d0)` |
| `ldr` at 0x9cd48 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9cd54 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x9cd5c | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x9cd64 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9cd68 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x9cd7c | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9cd84 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9cd88 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x9cd9c | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9cda4 | load | 4 | `abs 0x9d410` | `-` |
| `ldr` at 0x9cda8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9cdac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9cdc0 | load | 4 | `abs 0x9d3b4` | `-` |
| `ldr` at 0x9cdc4 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9cdd4 | load | 4 | `abs 0x9d3b8` | `-` |
| `ldr` at 0x9cddc | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9cde0 | load | 4 | `abs 0x9d40c` | `-` |
| `ldr` at 0x9cde8 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x9cdf0 | store | 4 | `sp -0xa0` | `[sp-160]+1` |
| `str` at 0x9cdfc | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x9ce00 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x9ce04 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x9ce10 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9ce14 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9ce18 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x9ce2c | load | 4 | `abs 0x9d3d0` | `-` |
| `ldr` at 0x9ce30 | load | 4 | `abs 0x1a688c` | `-` |
| `ldr` at 0x9ce38 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x9ce4c | load | 4 | `abs 0x9d3d0` | `-` |
| `ldr` at 0x9ce50 | load | 4 | `abs 0x1a688c` | `-` |
| `ldr` at 0x9ce58 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9ce5c | load | 4 | `abs 0x1a698c` | `-` |
| `ldr` at 0x9ce60 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9ce64 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9ce70 | store | 4 | `sp -0x9c` | `ret(0x9e6d0)` |
| `ldr` at 0x9ce74 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9ce80 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x9ce88 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x9ce90 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x9cea4 | load | 4 | `abs 0x9d3d0` | `-` |
| `ldr` at 0x9cea8 | load | 4 | `abs 0x1a688c` | `-` |
| `ldr` at 0x9ceb0 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x9cec4 | load | 4 | `abs 0x9d3d0` | `-` |
| `ldr` at 0x9cec8 | load | 4 | `abs 0x1a688c` | `-` |
| `ldr` at 0x9ced4 | load | 4 | `abs 0x9d414` | `-` |
| `ldr` at 0x9ced8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9cedc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9cef0 | load | 4 | `abs 0x9d3b4` | `-` |
| `ldr` at 0x9cef4 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9cf04 | load | 4 | `abs 0x9d3b8` | `-` |
| `ldr` at 0x9cf0c | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9cf10 | load | 4 | `abs 0x9d40c` | `-` |
| `ldr` at 0x9cf18 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x9cf20 | store | 4 | `sp -0xa0` | `[sp-160]+1` |
| `str` at 0x9cf2c | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x9cf30 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9cf34 | load | 4 | `abs 0x1a698c` | `-` |
| `ldr` at 0x9cf38 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x9cf44 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9cf48 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x9cf5c | load | 4 | `abs 0x1a6994` | `-` |
| `str` at 0x9cf64 | store | 4 | `sp -0x9c` | `?` |
| `ldr` at 0x9cf68 | load | 4 | `abs 0x9d3d0` | `-` |
| `ldr` at 0x9cf6c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9cf70 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9cf74 | load | 4 | `abs 0x1a688c` | `-` |
| `ldr` at 0x9cf78 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x9cf7c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9cf80 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9cf8c | store | 4 | `sp -0x98` | `ret(0x9e6d0)` |
| `ldr` at 0x9cf90 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x9cf9c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9cfa0 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x9cfa4 | load | 4 | `[sp-156] +0x14` | `-` |
| `ldr` at 0x9cfa8 | load | 4 | `ret(0x9e6d0) +0x14` | `-` |
| `ldr` at 0x9cfb4 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x9cfbc | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x9cfc4 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9cfc8 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9cfcc | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x9cfd0 | load | 4 | `ret(0x9e6d0) +0x14` | `-` |
| `str` at 0x9cfd4 | store | 4 | `sp -0xc0` | `[ret(0x9e6d0)+20]` |
| `ldr` at 0x9cfd8 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9cfdc | load | 4 | `[sp-156] +0x14` | `-` |
| `str` at 0x9cfe0 | store | 4 | `sp -0xbc` | `[[sp-156]+20]` |
| `ldr` at 0x9cfe8 | load | 4 | `abs 0x9d418` | `-` |
| `ldr` at 0x9cfec | load | 4 | `[sp-156] +0x0` | `-` |
| `ldr` at 0x9cff0 | load | 4 | `[sp-156] +0x4` | `-` |
| `ldr` at 0x9d004 | load | 4 | `abs 0x9d3b4` | `-` |
| `ldr` at 0x9d008 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9d018 | load | 4 | `abs 0x9d3b8` | `-` |
| `ldr` at 0x9d020 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9d024 | load | 4 | `abs 0x9d40c` | `-` |
| `ldr` at 0x9d02c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x9d038 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9d03c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x9d040 | load | 4 | `[sp-156] +0x10` | `-` |
| `ldr` at 0x9d044 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9d050 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x9d058 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x9d060 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9d064 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9d068 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x9d06c | load | 4 | `(unknown)` | `-` |
| `str` at 0x9d070 | store | 4 | `sp -0xc0` | `?` |
| `ldr` at 0x9d074 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9d078 | load | 4 | `[sp-156] +0x10` | `-` |
| `str` at 0x9d07c | store | 4 | `sp -0xbc` | `[[sp-156]+16]` |
| `ldr` at 0x9d084 | load | 4 | `abs 0x9d41c` | `-` |
| `ldr` at 0x9d088 | load | 4 | `[sp-156] +0x0` | `-` |
| `ldr` at 0x9d08c | load | 4 | `[sp-156] +0x4` | `-` |
| `ldr` at 0x9d0a0 | load | 4 | `abs 0x9d3b4` | `-` |
| `ldr` at 0x9d0a4 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9d0b4 | load | 4 | `abs 0x9d3b8` | `-` |
| `ldr` at 0x9d0bc | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9d0c0 | load | 4 | `abs 0x9d40c` | `-` |
| `ldr` at 0x9d0c8 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x9d0d4 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9d0d8 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x9d0dc | load | 4 | `[sp-156] +0xc` | `-` |
| `ldr` at 0x9d0e0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9d0ec | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x9d0f4 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x9d0fc | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9d100 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9d104 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x9d108 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9d10c | store | 4 | `sp -0xc0` | `?` |
| `ldr` at 0x9d110 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9d114 | load | 4 | `[sp-156] +0xc` | `-` |
| `str` at 0x9d118 | store | 4 | `sp -0xbc` | `[[sp-156]+12]` |
| `ldr` at 0x9d120 | load | 4 | `abs 0x9d420` | `-` |
| `ldr` at 0x9d124 | load | 4 | `[sp-156] +0x0` | `-` |
| `ldr` at 0x9d128 | load | 4 | `[sp-156] +0x4` | `-` |
| `ldr` at 0x9d13c | load | 4 | `abs 0x9d3b4` | `-` |
| `ldr` at 0x9d140 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9d150 | load | 4 | `abs 0x9d3b8` | `-` |
| `ldr` at 0x9d158 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9d15c | load | 4 | `abs 0x9d40c` | `-` |
| `ldr` at 0x9d164 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x9d16c | store | 4 | `sp -0xa0` | `[sp-160]+1` |
| `ldr` at 0x9d17c | load | 4 | `abs 0x9d424` | `-` |
| `ldr` at 0x9d180 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x9d194 | load | 4 | `abs 0x9d3b4` | `-` |
| `ldr` at 0x9d198 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9d1a8 | load | 4 | `abs 0x9d3b8` | `-` |
| `ldr` at 0x9d1b0 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9d1b4 | load | 4 | `abs 0x9d40c` | `-` |
| `ldr` at 0x9d1bc | load | 4 | `abs 0x9d3b8` | `-` |
| `ldr` at 0x9d1c0 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9d1c4 | load | 4 | `abs 0x9d404` | `-` |
| `str` at 0x9d1d0 | store | 4 | `sp -0x88` | `0x0` |
| `ldr` at 0x9d1d4 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9d1d8 | load | 4 | `abs 0x1a6998` | `-` |
| `ldr` at 0x9d1dc | load | 4 | `sp -0x88` | `-` |
| `str` at 0x9d1ec | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x9d1f0 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9d1f4 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9d1f8 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9d1fc | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x9d210 | load | 4 | `abs 0x1a69a0` | `-` |
| `ldr` at 0x9d218 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9d21c | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x9d230 | load | 4 | `abs 0x1a69a0` | `-` |
| `ldr` at 0x9d238 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9d23c | load | 4 | `abs 0x1a698c` | `-` |
| `ldr` at 0x9d240 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9d244 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9d250 | store | 4 | `sp -0x9c` | `ret(0x9e6d0)` |
| `ldr` at 0x9d254 | load | 4 | `abs 0x9d3b4` | `-` |
| `ldr` at 0x9d258 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9d268 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9d278 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x9d280 | store | 4 | `sp -0xa0` | `?` |
| `ldr` at 0x9d284 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x9d290 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x9d294 | store | 4 | `sp -0xa8` | `[sp-160]` |
| `ldr` at 0x9d298 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9d29c | load | 4 | `abs 0x1a6958` | `-` |
| `ldr` at 0x9d2a8 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9d2ac | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9d2b0 | load | 4 | `[sp-156] +0x10` | `-` |
| `ldr` at 0x9d2cc | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9d2d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9d2e0 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0x9d2e8 | store | 4 | `sp -0xa8` | `?` |
| `ldr` at 0x9d2ec | load | 4 | `sp -0xa8` | `-` |
| `str` at 0x9d2f0 | store | 4 | `sp -0xa0` | `[sp-168]` |
| `ldr` at 0x9d2f4 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x9d2f8 | store | 4 | `sp -0xac` | `[sp-168]` |
| `ldr` at 0x9d2fc | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x9d30c | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9d310 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9d314 | load | 4 | `[sp-156] +0x10` | `-` |
| `ldr` at 0x9d330 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9d338 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9d344 | load | 4 | `sp -0xac` | `-` |
| `str` at 0x9d34c | store | 4 | `sp -0xac` | `?` |
| `ldr` at 0x9d350 | load | 4 | `sp -0xac` | `-` |
| `str` at 0x9d354 | store | 4 | `sp -0xa0` | `?` |
| `ldr` at 0x9d358 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9d364 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9d368 | load | 4 | `abs 0x1a6958` | `-` |
| `ldr` at 0x9d374 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9d378 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9d37c | load | 4 | `[sp-156] +0x10` | `-` |
| `ldr` at 0x9d398 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9d3a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9d440 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x9d450 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9d454 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9d458 | load | 4 | `[sp-156] +0x10` | `-` |
| `ldr` at 0x9d474 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9d47c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9d488 | load | 4 | `abs 0x9d42c` | `-` |
| `ldr` at 0x9d48c | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9d49c | load | 4 | `abs 0x9d430` | `-` |
| `ldr` at 0x9d4a0 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9d4a4 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x9d4b8 | load | 4 | `abs 0x1a69a0` | `-` |
| `ldr` at 0x9d4c0 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9d4c4 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x9d4d8 | load | 4 | `abs 0x1a69a0` | `-` |
| `ldr` at 0x9d4e0 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x9d4e4 | store | 4 | `sp -0xc0` | `[sp-160]` |
| `ldr` at 0x9d4e8 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9d4ec | load | 4 | `abs 0x9d428` | `-` |
| `ldr` at 0x9d4f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9d4f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9d4fc | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9d500 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x9d514 | load | 4 | `abs 0x1a69a0` | `-` |
| `ldr` at 0x9d51c | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9d520 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x9d534 | load | 4 | `abs 0x1a69a0` | `-` |
| `ldr` at 0x9d540 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9d544 | load | 4 | `abs 0x1a6998` | `-` |
| `ldr` at 0x9d548 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x9d56c | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9d570 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9d574 | load | 4 | `abs 0x1a6998` | `-` |
| `str` at 0x9d57c | store | 4 | `abs 0x1a6998` | `[0x1a6998]-1` |
| `ldr` at 0x9d580 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x9d588 | store | 4 | `sp -0x88` | `[sp-136]-1` |
| `ldr` at 0x9d58c | load | 4 | `sp -0x88` | `-` |
| `str` at 0x9d594 | store | 4 | `sp -0x88` | `[sp-136]+1` |
| `ldr` at 0x9d5a0 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x9d5ac | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x9d5b8 | load | 4 | `abs 0x9d42c` | `-` |
| `ldr` at 0x9d5bc | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9d5cc | load | 4 | `abs 0x9d430` | `-` |
| `ldr` at 0x9d5d0 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9d5d4 | load | 4 | `abs 0x9d434` | `-` |
| `str` at 0x9d5e0 | store | 4 | `sp -0x80` | `0x0` |
| `ldr` at 0x9d5e4 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9d5e8 | load | 4 | `abs 0x1a6984` | `-` |
| `ldr` at 0x9d5ec | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x9d5f8 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9d5fc | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x9d618 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9d620 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9d630 | load | 4 | `abs 0x9d438` | `-` |
| `ldr` at 0x9d634 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x9d650 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9d658 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9d664 | load | 4 | `abs 0x9d43c` | `-` |
| `ldr` at 0x9d668 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x9d66c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9d678 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x9d680 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x9d688 | store | 4 | `sp -0x80` | `[sp-128]+1` |
| `ldm` at 0x9d694 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x9d694 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9d694 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9d694 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9c424 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16a9e4` | `0x31b40c` | `0x1a288c` |
| `bl` at 0x9c428 | `(unknown callee) 0x9afe8` | `?` | `?` | `?` | `?` |
| `bl` at 0x9c444 | `(unknown callee) 0x202c8` | `0x0` | `r1` | `r2` | `[0x1a6650]` |
| `bl` at 0x9c48c | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16aa04` | `0x31b40c` | `0x1a288c` |
| `bl` at 0x9c4a4 | `(unknown callee) 0x9afe8` | `?` | `?` | `0x1a6654` | `[0x1a6650]+5` |
| `bl` at 0x9c4b0 | `(unknown callee) 0x202c8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x9c504 | `(unknown callee) 0x20514` | `[0x1a688c]` | `?` | `?` | `0x1a688c` |
| `bl` at 0x9c550 | `(unknown callee) 0x201fc` | `?` | `0x1a6890` | `[0x1a6890]` | `?` |
| `bl` at 0x9c56c | `(unknown callee) 0x20550` | `[0x1a688c]` | `0x0` | `0x4` | `0x1a688c` |
| `bl` at 0x9c59c | `(unknown callee) 0x203f4` | `[0x1a688c]` | `[0x1a6994]` | `?` | `?` |
| `bl` at 0x9c5cc | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16aa20` | `?` | `0x1a288c` |
| `bl` at 0x9c5d0 | `(unknown callee) 0x9a000` | `?` | `?` | `?` | `?` |
| `bl` at 0x9c648 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16aa34` | `[sp-128]` | `0x1a288c` |
| `bl` at 0x9c6a0 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16aa54` | `[sp-128]` | `?` |
| `bl` at 0x9c74c | `(unknown callee) 0x9e6d0` | `[0x1a6994]` | `[0x1a698c]` | `[[sp-152]]` | `[[sp-152]+4]` |
| `bl` at 0x9c808 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16aa74` | `[[sp-156]]` | `[[sp-156]+4]` |
| `bl` at 0x9c89c | `(unknown callee) 0xa00bc` | `0x1a6994` | `0x1a698c` | `0x1a6990` | `[[sp-152]]` |
| `bl` at 0x9c924 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16aaa4` | `[[sp-152]]` | `[[sp-152]+4]` |
| `bl` at 0x9ca00 | `(unknown callee) 0xa07b0` | `[0x1a6994]` | `0x1a698c` | `[0x1a69a0]` | `0x1a6998` |
| `bl` at 0x9ca24 | `(unknown callee) 0xa3c34` | `[0x1a6908]` | `[0x1a690c]` | `0x1a6900` | `0x1a6900` |
| `bl` at 0x9caa0 | `(unknown callee) 0x9e944` | `ret(0xa3c34)` | `0x1a6900` | `0x1a6900` | `0x0` |
| `bl` at 0x9cb8c | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16aacc` | `?` | `0x1a288c` |
| `bl` at 0x9cbd8 | `(unknown callee) 0x20364` | `[0x1a6994]` | `[0x1a688c]` | `?` | `?` |
| `bl` at 0x9cc20 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16aae0` | `?` | `0x1a288c` |
| `bl` at 0x9cc50 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16aaf8` | `?` | `0x1a288c` |
| `bl` at 0x9cc94 | `(unknown callee) 0x205bc` | `sp-120` | `0x16ab1c` | `[0x1a698c]` | `[sp-124]` |
| `bl` at 0x9cca0 | `(unknown callee) 0x9c10c` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x9ccc8 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16ab40` | `sp-120` | `0x1a288c` |
| `bl` at 0x9cd3c | `(unknown callee) 0x9e6d0` | `[0x1a688c]` | `[sp-124]` | `?` | `?` |
| `bl` at 0x9cdb0 | `(unknown callee) 0x205bc` | `sp-120` | `0x16ab48` | `?` | `?` |
| `bl` at 0x9cdbc | `(unknown callee) 0x9c10c` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x9cde4 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16ab40` | `sp-120` | `0x1a288c` |
| `bl` at 0x9ce68 | `(unknown callee) 0x9e6d0` | `[0x1a6994]` | `[0x1a698c]` | `?` | `?` |
| `bl` at 0x9cee0 | `(unknown callee) 0x205bc` | `sp-120` | `0x16ab58` | `?` | `?` |
| `bl` at 0x9ceec | `(unknown callee) 0x9c10c` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x9cf14 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16ab40` | `sp-120` | `0x1a288c` |
| `bl` at 0x9cf84 | `(unknown callee) 0x9e6d0` | `[0x1a688c]` | `[sp-124]` | `?` | `?` |
| `bl` at 0x9cff4 | `(unknown callee) 0x205bc` | `sp-120` | `0x16ab6c` | `[[sp-156]]` | `[[sp-156]+4]` |
| `bl` at 0x9d000 | `(unknown callee) 0x9c10c` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x9d028 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16ab40` | `sp-120` | `0x1a288c` |
| `bl` at 0x9d090 | `(unknown callee) 0x205bc` | `sp-120` | `0x16ab84` | `[[sp-156]]` | `[[sp-156]+4]` |
| `bl` at 0x9d09c | `(unknown callee) 0x9c10c` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x9d0c4 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16ab40` | `sp-120` | `0x1a288c` |
| `bl` at 0x9d12c | `(unknown callee) 0x205bc` | `sp-120` | `0x16aba0` | `[[sp-156]]` | `[[sp-156]+4]` |
| `bl` at 0x9d138 | `(unknown callee) 0x9c10c` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x9d160 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16ab40` | `sp-120` | `0x1a288c` |
| `bl` at 0x9d184 | `(unknown callee) 0x205bc` | `sp-120` | `0x16abbc` | `?` | `sp-120` |
| `bl` at 0x9d190 | `(unknown callee) 0x9c10c` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x9d1b8 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16ab40` | `sp-120` | `0x1a288c` |
| `bl` at 0x9d1c8 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16aaf8` | `?` | `0x1a288c` |
| `bl` at 0x9d248 | `(unknown callee) 0x9e6d0` | `[0x1a6994]` | `[0x1a698c]` | `?` | `?` |
| `bl` at 0x9d4f8 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16abcc` | `?` | `?` |
| `bl` at 0x9d568 | `(unknown callee) 0x203f4` | `?` | `?` | `?` | `?` |
| `bl` at 0x9d59c | `(unknown callee) 0x9afe8` | `?` | `?` | `[0x1a6998]` | `?` |
| `bl` at 0x9d5d8 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16abec` | `?` | `0x1a288c` |
| `bl` at 0x9d670 | `(unknown callee) 0xa1144` | `?` | `0x1a6900` | `0x1a6ba4` | `[sp-128]` |
| `bl` at 0x9d67c | `(unknown callee) 0x9b5b4` | `[sp-128]` | `0x1a6900` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x9c3fc | `[0x1a6654] == 0x0` | `[0x1a6654] != 0x0` |
| `beq` at 0x9c410 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9c43c | `[0x1a6650] == 0x0` | `[0x1a6650] != 0x0` |
| `bgt` at 0x9c464 | `[sp-152] > 0x4 (signed, 32-bit)` | `[sp-152] <= 0x4 (signed, 32-bit)` |
| `beq` at 0x9c478 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9c4cc | `[0x1a6890] == 0x0` | `[0x1a6890] != 0x0` |
| `bgt` at 0x9c4e4 | `[0x1a698c] > [0x1a6890] (signed, 32-bit)` | `[0x1a698c] <= [0x1a6890] (signed, 32-bit)` |
| `beq` at 0x9c4f8 | `[0x1a688c] == 0x0` | `[0x1a688c] != 0x0` |
| `beq` at 0x9c5bc | `? == 0x0` | `? != 0x0` |
| `ble` at 0x9c5ec | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x9c620 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x9c634 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9c660 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x9c6e0 | `? <= [sp-132] (signed, 32-bit)` | `? > [sp-132] (signed, 32-bit)` |
| `beq` at 0x9c760 | `ret(0x9e6d0) == 0x0` | `ret(0x9e6d0) != 0x0` |
| `bhs` at 0x9c77c | `[[sp-152]+20]+1 >= [ret(0x9e6d0)+20] (unsigned, 32-bit)` | `[[sp-152]+20]+1 < [ret(0x9e6d0)+20] (unsigned, 32-bit)` |
| `beq` at 0x9c7c0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9c848 | `[[sp-152]+16] == ?` | `[[sp-152]+16] != ?` |
| `beq` at 0x9c8b4 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x9c960 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bls` at 0x9c994 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x9ca54 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x9cab4 | `ret(0x9e944) == 0x0` | `ret(0x9e944) != 0x0` |
| `beq` at 0x9cac4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x9cad4 | `[[sp-148]+12] == 0x0` | `[[sp-148]+12] != 0x0` |
| `beq` at 0x9cae4 | `[[sp-148]+8] == 0x52` | `[[sp-148]+8] != 0x52` |
| `beq` at 0x9caf4 | `[[sp-148]+8] == 0x44` | `[[sp-148]+8] != 0x44` |
| `beq` at 0x9cb04 | `[[sp-148]+8] == 0x2d` | `[[sp-148]+8] != 0x2d` |
| `beq` at 0x9cb7c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x9cba8 | `[0x1a698c] != [sp-124]` | `[0x1a698c] == [sp-124]` |
| `bne` at 0x9cbe4 | `ret(0x20364) != 0x0` | `ret(0x20364) == 0x0` |
| `beq` at 0x9cc0c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9cc2c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x9cc40 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9cc6c | `[0x1a698c] == [sp-124]` | `[0x1a698c] != [sp-124]` |
| `beq` at 0x9ccb4 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x9cce4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x9cd50 | `ret(0x9e6d0) != 0x0` | `ret(0x9e6d0) == 0x0` |
| `beq` at 0x9cdd0 | `? == 0x0` | `? != 0x0` |
| `bge` at 0x9ce0c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x9ce7c | `ret(0x9e6d0) != 0x0` | `ret(0x9e6d0) == 0x0` |
| `beq` at 0x9cf00 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x9cf40 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x9cf98 | `ret(0x9e6d0) == 0x0` | `ret(0x9e6d0) != 0x0` |
| `beq` at 0x9cfb0 | `[[sp-156]+20] == [ret(0x9e6d0)+20]` | `[[sp-156]+20] != [ret(0x9e6d0)+20]` |
| `beq` at 0x9d014 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9d034 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x9d04c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x9d0b0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9d0d0 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x9d0e8 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x9d14c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9d1a4 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x9d1e4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x9d264 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x9d28c | `[sp-160] != 0x0` | `[sp-160] == 0x0` |
| `beq` at 0x9d2a4 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x9d2dc | `? != 0x0` | `? == 0x0` |
| `beq` at 0x9d308 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x9d340 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x9d360 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x9d370 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x9d3a8 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x9d44c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x9d484 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x9d498 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9d5a8 | `[sp-140] == 0x0` | `[sp-140] != 0x0` |
| `beq` at 0x9d5b4 | `[sp-140] == 0x2` | `[sp-140] != 0x2` |
| `beq` at 0x9d5c8 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x9d5f4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x9d628 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x9d660 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9d694 | `r0` | `?` |

## `L1CfgRingEvent`

- `L1CfgRingEvent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9d698 to 0x9d89c.
  Established. A word store copies the value from sp-0x18 into memory at offset 0x38 of the pointer from sp-0x14 (`str` at 0x9d768).
  A later store increments the word at offset 0x30 of that pointer (`str` at 0x9d7c4).
  Calls to unnamed 0xfde7c have candidate values headed by the word at sp-0x14 and one, and the later result is stored at sp-0x2c (`bl` at 0x9d750, `bl` at 0x9d824, `str` at 0x9d828).
  The field changes do not establish a physical ring event or its routing.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9d6a8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9d6ac | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x9d6b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9d6b4 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x9d6c4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x9d6c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9d6cc | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x9d6d8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x9d6dc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x9d6e0 | load | 1 | `r0 +0x24` | `-` |
| `ldr` at 0x9d6f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9d6f4 | load | 4 | `r0 +0x28` | `-` |
| `str` at 0x9d6f8 | store | 4 | `sp -0x30` | `[r0+40]` |
| `ldr` at 0x9d700 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9d70c | store | 4 | `sp -0x30` | `ret(0xfde7c)` |
| `ldr` at 0x9d710 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x9d720 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x9d724 | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0x9d734 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9d738 | load | 4 | `[sp-20] +0x30` | `-` |
| `ldr` at 0x9d748 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9d760 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9d764 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9d768 | store | 4 | `[sp-20] +0x38` | `[sp-24]` |
| `ldr` at 0x9d76c | load | 4 | `abs 0x9d884` | `-` |
| `ldr` at 0x9d770 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9d780 | load | 4 | `abs 0x9d888` | `-` |
| `ldr` at 0x9d784 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9d788 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x9d798 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9d79c | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x9d7a8 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9d7ac | load | 4 | `abs 0x9d88c` | `-` |
| `ldr` at 0x9d7b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9d7b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9d7bc | load | 4 | `[sp-20] +0x30` | `-` |
| `str` at 0x9d7c4 | store | 4 | `[sp-20] +0x30` | `[[sp-20]+48]+1` |
| `ldr` at 0x9d7c8 | load | 4 | `abs 0x9d890` | `-` |
| `ldr` at 0x9d7cc | load | 4 | `abs 0x1a6908` | `-` |
| `ldr` at 0x9d7d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9d7dc | load | 4 | `abs 0x9d890` | `-` |
| `ldr` at 0x9d7e0 | load | 4 | `abs 0x1a690c` | `-` |
| `ldr` at 0x9d7e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9d7f0 | load | 4 | `abs 0x9d894` | `-` |
| `str` at 0x9d7f4 | store | 4 | `sp -0x28` | `0x1a662c` |
| `ldr` at 0x9d7f8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x9d7fc | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0x9d80c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9d810 | load | 4 | `[sp-20] +0x30` | `-` |
| `str` at 0x9d814 | store | 4 | `sp -0x2c` | `[[sp-20]+48]` |
| `ldr` at 0x9d81c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9d828 | store | 4 | `sp -0x2c` | `ret(0xfde7c)` |
| `ldr` at 0x9d82c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9d830 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9d834 | store | 4 | `[sp-40] +0x0` | `[sp-44]` |
| `ldr` at 0x9d838 | load | 4 | `abs 0x9d884` | `-` |
| `ldr` at 0x9d83c | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9d84c | load | 4 | `abs 0x9d888` | `-` |
| `ldr` at 0x9d850 | load | 4 | `abs 0x9d894` | `-` |
| `ldr` at 0x9d854 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9d858 | load | 4 | `abs 0x9d898` | `-` |
| `ldr` at 0x9d85c | load | 4 | `abs 0x1a662c` | `-` |
| `str` at 0x9d868 | store | 4 | `sp -0x24` | `?` |
| `str` at 0x9d874 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x9d878 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x9d880 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9d880 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9d880 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9d708 | `(unknown callee) 0xfde7c` | `r0` | `0x0` | `r2` | `?` |
| `bl` at 0x9d750 | `(unknown callee) 0xfde7c` | `[sp-20]` | `0x1` | `[sp-48]` | `?` |
| `bl` at 0x9d7b0 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16ac00` | `?` | `?` |
| `bl` at 0x9d824 | `(unknown callee) 0xfde7c` | `[sp-20]` | `0x1` | `[0x1a690c]` | `?` |
| `bl` at 0x9d860 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16ac28` | `[0x1a662c]` | `0x1a288c` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x9d6ec | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x9d71c | `[sp-48]-1 > 0x2 (unsigned, 32-bit)` | `[sp-48]-1 <= 0x2 (unsigned, 32-bit)` |
| `bne` at 0x9d730 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x9d740 | `[[sp-20]+48] != 0x0` | `[[sp-20]+48] == 0x0` |
| `bne` at 0x9d75c | `ret(0xfde7c) != 0x0` | `ret(0xfde7c) == 0x0` |
| `beq` at 0x9d77c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x9d7d8 | `[0x1a6908] != [sp-28]` | `[0x1a6908] == [sp-28]` |
| `bne` at 0x9d7ec | `[0x1a690c] != [sp-32]` | `[0x1a690c] == [sp-32]` |
| `bne` at 0x9d808 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x9d848 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9d880 | `r0` | `?` |

## `L1CfgPortEnable`

- `L1CfgPortEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9d89c to 0x9d974.
  Established. Two word stores with unknown effective addresses carry one and zero (`str` at 0x9d90c, `str` at 0x9d93c).
  A separate word store increments the value loaded from absolute 0x1a6630 at that same address (`ldr` at 0x9d948, `str` at 0x9d950).
  The final call to unnamed 0x9afe8 has candidate values unknown, 0x1a6900, 0x1a6630 and the incremented value (`bl` at 0x9d954).
  The unknown store targets leave the physical port-enable operation unestablished.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9d8ac | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x9d8b0 | load | 4 | `abs 0x9d95c` | `-` |
| `ldr` at 0x9d8b8 | load | 4 | `abs 0x9d960` | `-` |
| `ldr` at 0x9d8bc | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9d8cc | load | 4 | `abs 0x9d964` | `-` |
| `ldr` at 0x9d8d0 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9d8d4 | load | 4 | `abs 0x9d968` | `-` |
| `ldr` at 0x9d8d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9d8e0 | load | 4 | `abs 0x9d96c` | `-` |
| `ldr` at 0x9d8e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9d900 | load | 4 | `abs 0x1a6988` | `-` |
| `str` at 0x9d90c | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x9d910 | load | 4 | `abs 0x9d96c` | `-` |
| `ldr` at 0x9d914 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9d930 | load | 4 | `abs 0x1a6988` | `-` |
| `str` at 0x9d93c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x9d940 | load | 4 | `abs 0x9d970` | `-` |
| `ldr` at 0x9d944 | load | 4 | `abs 0x9d970` | `-` |
| `ldr` at 0x9d948 | load | 4 | `abs 0x1a6630` | `-` |
| `str` at 0x9d950 | store | 4 | `abs 0x1a6630` | `[0x1a6630]+1` |
| `ldm` at 0x9d958 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x9d958 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9d958 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9d958 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9d8b4 | `(unknown callee) 0x9adfc` | `0x16ac44` | `r1` | `r2` | `r3` |
| `bl` at 0x9d8dc | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16ac54` | `[sp-20]` | `0x1a288c` |
| `bl` at 0x9d954 | `(unknown callee) 0x9afe8` | `?` | `0x1a6900` | `0x1a6630` | `[0x1a6630]+1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x9d8c8 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9d958 | `r0` | `ret(0x9afe8)` |

## `L1CfgPortConfigure`

- `L1CfgPortConfigure`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9d974 to 0x9e2d4.
  Established. Two stores increment the words loaded from absolute 0x1a6634 and 0x1a6638 at those same addresses (`str` at 0x9daf4, `str` at 0x9db18).
  The call to unnamed 0xa3f14 has candidate values the word at sp-0x8c, 0x1a6900, unknown and the word at absolute 0x1a6988 (`bl` at 0x9dd4c).
  A following word store has unknown target and carries that call result (`bl` at 0x9dd4c, `str` at 0x9dd50).
  Later calls to unnamed 0xa0438 have candidate values the words at sp-0x94 and sp-0x90, sp-0x9c and zero, without establishing a physical port-configuration protocol (`bl` at 0x9e104, `bl` at 0x9e270).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9d984 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x9d988 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x9d98c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x9d990 | load | 1 | `r0 +0x24` | `-` |
| `ldr` at 0x9d9a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9d9a4 | load | 4 | `r0 +0x28` | `-` |
| `str` at 0x9d9a8 | store | 4 | `sp -0xa8` | `[r0+40]` |
| `ldr` at 0x9d9b0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9d9bc | store | 4 | `sp -0xa8` | `ret(0xfde7c)` |
| `ldr` at 0x9d9c0 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0x9d9c4 | store | 4 | `sp -0x2c` | `[sp-168]` |
| `ldr` at 0x9d9c8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x9d9cc | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x9d9dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9d9e0 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x9d9e4 | store | 4 | `sp -0xac` | `[[sp-24]+48]` |
| `ldr` at 0x9d9ec | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9d9f8 | store | 4 | `sp -0xac` | `ret(0xfde7c)` |
| `ldr` at 0x9d9fc | load | 4 | `sp -0xac` | `-` |
| `str` at 0x9da00 | store | 4 | `sp -0xa0` | `[sp-172]` |
| `ldr` at 0x9da04 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x9da10 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9da14 | store | 4 | `sp -0xb0` | `[sp-24]` |
| `ldr` at 0x9da18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9da24 | load | 4 | `abs 0x9e2a0` | `-` |
| `str` at 0x9da28 | store | 4 | `sp -0xb4` | `0x16ac6c` |
| `ldr` at 0x9da30 | load | 4 | `abs 0x9e2a4` | `-` |
| `str` at 0x9da34 | store | 4 | `sp -0xb4` | `0x16ac74` |
| `ldr` at 0x9da38 | load | 4 | `abs 0x9e2a8` | `-` |
| `ldr` at 0x9da3c | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x9da40 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x9da44 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x9da58 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x9da64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9da68 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x9da6c | store | 4 | `sp -0x24` | `[[sp-24]+12]` |
| `str` at 0x9da74 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x9da78 | load | 4 | `abs 0x9e2ac` | `-` |
| `ldr` at 0x9da80 | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9da84 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9daa0 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9daa8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9dab4 | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9dab8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9dad4 | load | 4 | `abs 0x1a6988` | `-` |
| `str` at 0x9dae0 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x9dae4 | load | 4 | `abs 0x9e2b4` | `-` |
| `ldr` at 0x9dae8 | load | 4 | `abs 0x9e2b4` | `-` |
| `ldr` at 0x9daec | load | 4 | `abs 0x1a6634` | `-` |
| `str` at 0x9daf4 | store | 4 | `abs 0x1a6634` | `[0x1a6634]+1` |
| `ldr` at 0x9daf8 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x9db08 | load | 4 | `abs 0x9e2b8` | `-` |
| `ldr` at 0x9db0c | load | 4 | `abs 0x9e2b8` | `-` |
| `ldr` at 0x9db10 | load | 4 | `abs 0x1a6638` | `-` |
| `str` at 0x9db18 | store | 4 | `abs 0x1a6638` | `[0x1a6638]+1` |
| `ldr` at 0x9db1c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x9db20 | load | 1 | `[sp-24] +0x16` | `-` |
| `ldr` at 0x9db2c | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9db30 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9db4c | load | 4 | `abs 0x1a6988` | `-` |
| `str` at 0x9db58 | store | 4 | `(unknown)` | `0x2` |
| `ldr` at 0x9db5c | load | 4 | `abs 0x9e2bc` | `-` |
| `ldr` at 0x9db60 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9db64 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9db68 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x9db70 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x9db74 | load | 2 | `[sp-24] +0x22` | `-` |
| `ldr` at 0x9db80 | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9db84 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9dba0 | load | 4 | `abs 0x1a6988` | `-` |
| `str` at 0x9dbac | store | 4 | `(unknown)` | `0x2` |
| `ldr` at 0x9dbb0 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x9dbb4 | load | 2 | `[sp-24] +0x22` | `-` |
| `str` at 0x9dbb8 | store | 4 | `sp -0x28` | `[[sp-24]+34]:2` |
| `ldr` at 0x9dbbc | load | 4 | `abs 0x9e2c0` | `-` |
| `ldr` at 0x9dbc0 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9dbd0 | load | 4 | `abs 0x9e2c4` | `-` |
| `str` at 0x9dbd4 | store | 4 | `sp -0xb8` | `0x1a288c` |
| `ldr` at 0x9dbd8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9dbe4 | load | 4 | `abs 0x9e2c8` | `-` |
| `str` at 0x9dbe8 | store | 4 | `sp -0xbc` | `0x16acb0` |
| `ldr` at 0x9dbf0 | load | 4 | `abs 0x9e2cc` | `-` |
| `str` at 0x9dbf4 | store | 4 | `sp -0xbc` | `0x16acb8` |
| `ldr` at 0x9dbf8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9dc00 | store | 4 | `sp -0xc0` | `[sp-36]+64` |
| `ldr` at 0x9dc04 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9dc10 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9dc18 | store | 4 | `sp -0xc4` | `[sp-40]+64` |
| `str` at 0x9dc24 | store | 4 | `sp -0xc4` | `0x3f` |
| `ldr` at 0x9dc28 | load | 4 | `sp -0xc4` | `-` |
| `str` at 0x9dc2c | store | 4 | `sp -0xdc` | `?` |
| `ldr` at 0x9dc30 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x9dc34 | store | 4 | `sp -0xd8` | `[sp-44]` |
| `ldr` at 0x9dc38 | load | 4 | `sp -0xb8` | `-` |
| `ldr` at 0x9dc3c | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9dc40 | load | 4 | `abs 0x9e2d0` | `-` |
| `ldr` at 0x9dc44 | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x9dc48 | load | 4 | `sp -0xc0` | `-` |
| `ldr` at 0x9dc50 | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9dc54 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9dc70 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9dc90 | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9dc94 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9dcb0 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9dcb8 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x9dcbc | store | 4 | `(unknown)` | `[sp-148]` |
| `ldr` at 0x9dcc0 | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9dcc4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9dce0 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9dce8 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x9dcec | store | 4 | `(unknown)` | `[sp-144]` |
| `ldr` at 0x9dcf0 | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9dcf4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9dd10 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9dd18 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x9dd1c | store | 4 | `(unknown)` | `[sp-140]` |
| `ldr` at 0x9dd20 | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9dd24 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9dd40 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9dd48 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x9dd50 | store | 4 | `(unknown)` | `ret(0xa3f14)` |
| `ldr` at 0x9dd54 | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9dd58 | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0x9dd64 | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9dd68 | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0x9dd74 | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9dd78 | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0x9dd84 | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9dd88 | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0x9dd94 | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9dd98 | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0x9dda4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x9ddb0 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x9ddbc | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x9ddc8 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x9ddd4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x9dde4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9ddf4 | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9ddf8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9de14 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9de1c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9de24 | load | 4 | `sp -0x74` | `-` |
| `str` at 0x9de30 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x9de34 | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9de38 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9de54 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9de5c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9de60 | store | 4 | `(unknown)` | `[sp-40]` |
| `ldr` at 0x9de64 | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9de68 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9de84 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9de8c | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9de90 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9de94 | load | 4 | `abs 0x1a6924` | `-` |
| `ldr` at 0x9de9c | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9dea0 | load | 4 | `abs 0x1a6924` | `-` |
| `ldr` at 0x9deac | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9deb0 | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9deb4 | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9deb8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9ded4 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9dedc | load | 4 | `abs 0x1a6924` | `-` |
| `ldr` at 0x9dee0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9dee8 | store | 4 | `abs 0x1a6924` | `?` |
| `ldr` at 0x9def0 | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9def4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9df14 | load | 4 | `abs 0x1a6988` | `-` |
| `ldrb` at 0x9df20 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x9df2c | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9df30 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9df4c | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9df58 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9df70 | store | 4 | `sp -0xa4` | `ret(0xee1bc)` |
| `ldr` at 0x9df78 | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9df7c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9df98 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9dfa4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9dfbc | store | 4 | `sp -0xa4` | `ret(0xee1bc)` |
| `ldr` at 0x9dfc0 | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9dfc4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9dfe0 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9dfe8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9dff4 | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9dff8 | load | 4 | `abs 0x1a6970` | `-` |
| `ldr` at 0x9e008 | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9e00c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9e028 | load | 4 | `abs 0x1a6988` | `-` |
| `str` at 0x9e034 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x9e038 | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9e03c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9e058 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9e064 | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9e068 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9e084 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9e090 | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9e094 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9e0b0 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9e0bc | load | 4 | `sp -0x90` | `-` |
| `str` at 0x9e0c0 | store | 4 | `sp -0xdc` | `[sp-144]` |
| `ldr` at 0x9e0c4 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x9e0c8 | store | 4 | `sp -0xd8` | `[sp-140]` |
| `ldr` at 0x9e0cc | load | 4 | `sp -0x44` | `-` |
| `str` at 0x9e0d0 | store | 4 | `sp -0xd4` | `[sp-68]` |
| `str` at 0x9e0d8 | store | 4 | `sp -0xd0` | `0x0` |
| `str` at 0x9e0e0 | store | 4 | `sp -0xcc` | `?` |
| `ldr` at 0x9e0e8 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x9e0f4 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x9e0f8 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x9e10c | store | 4 | `sp -0x20` | `0x2` |
| `ldr` at 0x9e110 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x9e114 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x9e118 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9e124 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9e128 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x9e13c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x9e14c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9e150 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9e164 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9e168 | store | 4 | `sp -0xc8` | `?` |
| `ldr` at 0x9e170 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9e174 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9e17c | store | 4 | `sp -0xc8` | `ret(0xfde7c)` |
| `ldr` at 0x9e188 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x9e194 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x9e1a4 | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9e1a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9e1c4 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9e1d0 | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9e1d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9e1f0 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9e1fc | load | 4 | `abs 0x9e2b0` | `-` |
| `ldr` at 0x9e200 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9e21c | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9e228 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x9e22c | store | 4 | `sp -0xdc` | `[sp-144]` |
| `ldr` at 0x9e230 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x9e234 | store | 4 | `sp -0xd8` | `[sp-140]` |
| `str` at 0x9e23c | store | 4 | `sp -0xd4` | `0x0` |
| `str` at 0x9e244 | store | 4 | `sp -0xd0` | `0x1` |
| `str` at 0x9e24c | store | 4 | `sp -0xcc` | `?` |
| `ldr` at 0x9e254 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x9e260 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x9e264 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x9e274 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9e27c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0x9e290 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x9e29c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x9e29c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9e29c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9e29c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9d9b8 | `(unknown callee) 0xfde7c` | `r0` | `0x0` | `r2` | `?` |
| `bl` at 0x9d9f4 | `(unknown callee) 0xfde7c` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x9da48 | `(unknown callee) 0x243d4` | `0x16ac80` | `[[sp-24]+12]` | `[sp-180]` | `[sp-24]` |
| `bl` at 0x9da60 | `(unknown callee) 0xa0f3c` | `sp-156` | `[sp-172]` | `0x70` | `sp-156` |
| `bl` at 0x9da7c | `(unknown callee) 0x9adfc` | `0x16ac9c` | `?` | `?` | `0x0` |
| `bl` at 0x9dc4c | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16acc0` | `[sp-188]` | `[sp-36]+64` |
| `bl` at 0x9dc8c | `(unknown callee) 0x203f4` | `?` | `sp-156` | `0x70` | `?` |
| `bl` at 0x9dd4c | `(unknown callee) 0xa3f14` | `[sp-140]` | `0x1a6900` | `?` | `[0x1a6988]` |
| `bl` at 0x9deec | `(unknown callee) 0x9a0d4` | `0x1a6900` | `0x1a6900` | `[0x1a6924]` | `?` |
| `bl` at 0x9df68 | `(unknown callee) 0xee1bc` | `?` | `[sp-36]+16384` | `?` | `[sp-36]+16384` |
| `bl` at 0x9dfb4 | `(unknown callee) 0xee1bc` | `?` | `[sp-36]+8192` | `?` | `[sp-36]+8192` |
| `bl` at 0x9e0ec | `(unknown callee) 0xa00bc` | `?` | `?` | `?` | `[sp-148]` |
| `bl` at 0x9e104 | `(unknown callee) 0xa0438` | `[sp-148]` | `[sp-144]` | `sp-156` | `0x0` |
| `bl` at 0x9e178 | `(unknown callee) 0xfde7c` | `[sp-24]` | `?` | `[sp-24]` | `?` |
| `bl` at 0x9e190 | `(unknown callee) 0xa0f3c` | `sp-156` | `?` | `0x70` | `sp-156` |
| `bl` at 0x9e258 | `(unknown callee) 0xa00bc` | `?` | `?` | `?` | `[sp-148]` |
| `bl` at 0x9e270 | `(unknown callee) 0xa0438` | `[sp-148]` | `[sp-144]` | `sp-156` | `0x0` |
| `bl` at 0x9e284 | `(unknown callee) 0x9c3e0` | `?` | `?` | `[[sp-24]+32]:1` | `?` |
| `bl` at 0x9e28c | `(unknown callee) 0x9afe8` | `ret(0xee1bc)` | `0x1a6900` | `?` | `[0x1a6970]` |
| `bl` at 0x9e294 | `(unknown callee) 0x9b5b4` | `[sp-36]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x9d99c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x9d9d8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x9da0c | `[sp-172] != 0x0` | `[sp-172] == 0x0` |
| `beq` at 0x9da20 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `bne` at 0x9dab0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x9db00 | `[sp-140] == 0x44` | `[sp-140] != 0x44` |
| `bne` at 0x9db28 | `[[sp-24]+22]:1 != 0x0` | `[[sp-24]+22]:1 == 0x0` |
| `beq` at 0x9db7c | `[[sp-24]+34]:2 == 0x0` | `[[sp-24]+34]:2 != 0x0` |
| `beq` at 0x9dbcc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9dbe0 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `beq` at 0x9dc0c | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `beq` at 0x9dd60 | `[0x1a6910] == 0x44` | `[0x1a6910] != 0x44` |
| `beq` at 0x9dd70 | `[0x1a6910] == 0x40` | `[0x1a6910] != 0x40` |
| `beq` at 0x9dd80 | `[0x1a6910] == 0x7d` | `[0x1a6910] != 0x7d` |
| `beq` at 0x9dd90 | `[0x1a6910] == 0x2e` | `[0x1a6910] != 0x2e` |
| `beq` at 0x9dda0 | `[0x1a6910] == 0x2d` | `[0x1a6910] != 0x2d` |
| `beq` at 0x9ddac | `[sp-140] == 0x44` | `[sp-140] != 0x44` |
| `beq` at 0x9ddb8 | `[sp-140] == 0x40` | `[sp-140] != 0x40` |
| `beq` at 0x9ddc4 | `[sp-140] == 0x7d` | `[sp-140] != 0x7d` |
| `beq` at 0x9ddd0 | `[sp-140] == 0x2e` | `[sp-140] != 0x2e` |
| `beq` at 0x9dddc | `[sp-140] == 0x2d` | `[sp-140] != 0x2d` |
| `bne` at 0x9ddec | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `beq` at 0x9dea8 | `? == [0x1a6924]` | `? != [0x1a6924]` |
| `beq` at 0x9df28 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x9dff0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x9e000 | `[0x1a6970] != 0x0` | `[0x1a6970] == 0x0` |
| `ble` at 0x9e120 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x9e148 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x9e19c | `[sp-156] != 0x0` | `[sp-156] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9e29c | `r0` | `?` |
