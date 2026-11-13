# ARM L1 configuration and request entry paths in 1.34.2

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
Computed transfers in L1CfgInit and L1CfgRequest leave unresolved edges. Their tables cover known edges only.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/l1-configuration-facts.txt) retain all tool sections.

## `L1CfgInit`

- `L1CfgInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x55428 to 0x55b6c.
  Established. The global word at `0x11a0c4` receives `[0x11a0c4]+1` (`str` at 0x55484). Successive
  word writes to `0x11a0c8` contain `0x100` (`str` at 0x554f4), `0x1` (`str` at 0x55500), `0x10`
  (`str` at 0x5550c), `0x3ff` (`str` at 0x55518) and `0x0` (`str` at 0x55524). The two calls to
  `0x1f498` have third candidates `0x55ec4` (`bl` at 0x55a28) and `0x55cac` (`bl` at 0x55a68). These
  candidate addresses do not establish calls to either candidate. The computed transfer is taken
  when `[0x11a3b0]-48 <= 0x22` unsigned and falls through when it is greater (`ldrls` at 0x55808);
  its taken edges are unresolved. The initialization and task semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x55444 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x55448 | load | 4 | `abs 0x55b08` | `-` |
| `ldr` at 0x5544c | load | 4 | `abs 0x11a0c4` | `-` |
| `ldr` at 0x55458 | load | 4 | `abs 0x55b0c` | `-` |
| `str` at 0x5545c | store | 4 | `sp -0x40` | `0xe6158` |
| `ldr` at 0x55464 | load | 4 | `abs 0x55b10` | `-` |
| `ldr` at 0x55474 | load | 4 | `abs 0x55b08` | `-` |
| `ldr` at 0x55478 | load | 4 | `abs 0x55b08` | `-` |
| `ldr` at 0x5547c | load | 4 | `abs 0x11a0c4` | `-` |
| `str` at 0x55484 | store | 4 | `abs 0x11a0c4` | `[0x11a0c4]+1` |
| `ldr` at 0x55488 | load | 4 | `abs 0x55b14` | `-` |
| `str` at 0x55494 | store | 4 | `abs 0x11a0dc` | `ret(0x189c8)` |
| `str` at 0x554b0 | store | 4 | `sp -0x20` | `ret(0x18a1c)` |
| `ldr` at 0x554b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x554e0 | load | 4 | `abs 0x55b18` | `-` |
| `ldr` at 0x554ec | load | 4 | `abs 0x55b1c` | `-` |
| `str` at 0x554f4 | store | 4 | `abs 0x11a0c8` | `0x100` |
| `ldr` at 0x554f8 | load | 4 | `abs 0x55b1c` | `-` |
| `str` at 0x55500 | store | 4 | `abs 0x11a0c8` | `0x1` |
| `ldr` at 0x55504 | load | 4 | `abs 0x55b1c` | `-` |
| `str` at 0x5550c | store | 4 | `abs 0x11a0c8` | `0x10` |
| `ldr` at 0x55510 | load | 4 | `abs 0x55b1c` | `-` |
| `ldr` at 0x55514 | load | 4 | `abs 0x55b20` | `-` |
| `str` at 0x55518 | store | 4 | `abs 0x11a0c8` | `0x3ff` |
| `ldr` at 0x5551c | load | 4 | `abs 0x55b1c` | `-` |
| `str` at 0x55524 | store | 4 | `abs 0x11a0c8` | `0x0` |
| `ldr` at 0x55528 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x5552c | store | 4 | `sp -0x28` | `[sp-36]` |
| `ldr` at 0x55530 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5553c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5554c | load | 4 | `abs 0x55b24` | `-` |
| `str` at 0x55554 | store | 4 | `abs 0x1155f0` | `0xc` |
| `ldr` at 0x55558 | load | 4 | `abs 0x55b28` | `-` |
| `str` at 0x55560 | store | 4 | `abs 0x11a430` | `0x10` |
| `ldr` at 0x55564 | load | 4 | `abs 0x55b28` | `-` |
| `str` at 0x5556c | store | 4 | `abs 0x11a42c` | `0x0` |
| `ldr` at 0x55570 | load | 4 | `abs 0x55b28` | `-` |
| `ldr` at 0x55574 | load | 4 | `abs 0x55b28` | `-` |
| `ldr` at 0x55578 | load | 4 | `abs 0x11a430` | `-` |
| `str` at 0x55598 | store | 4 | `abs 0x11a434` | `ret(0x18920)` |
| `ldr` at 0x5559c | load | 4 | `abs 0x55b28` | `-` |
| `ldr` at 0x555a0 | load | 4 | `abs 0x55b28` | `-` |
| `ldr` at 0x555a4 | load | 4 | `abs 0x11a430` | `-` |
| `ldr` at 0x555b8 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x555c8 | load | 4 | `abs 0x55b28` | `-` |
| `str` at 0x555d0 | store | 4 | `abs 0x11a424` | `0xd` |
| `ldr` at 0x555d4 | load | 4 | `abs 0x55b28` | `-` |
| `ldr` at 0x555d8 | load | 4 | `abs 0x55b28` | `-` |
| `ldr` at 0x555dc | load | 4 | `abs 0x11a424` | `-` |
| `str` at 0x55604 | store | 4 | `abs 0x11a428` | `ret(0x18920)` |
| `ldr` at 0x55608 | load | 4 | `abs 0x55b28` | `-` |
| `ldr` at 0x5560c | load | 4 | `abs 0x55b28` | `-` |
| `ldr` at 0x55610 | load | 4 | `abs 0x11a424` | `-` |
| `ldr` at 0x5562c | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5563c | load | 4 | `abs 0x55b28` | `-` |
| `str` at 0x55644 | store | 4 | `abs 0x11a3a0` | `0x1` |
| `ldr` at 0x55648 | load | 4 | `abs 0x55b28` | `-` |
| `str` at 0x55650 | store | 4 | `abs 0x11a400` | `0x2` |
| `ldr` at 0x55654 | load | 4 | `abs 0x55b28` | `-` |
| `ldr` at 0x55658 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x5565c | store | 4 | `abs 0x11a3b0` | `[sp-36]` |
| `ldr` at 0x55660 | load | 4 | `abs 0x55b28` | `-` |
| `str` at 0x5566c | store | 4 | `abs 0x11a3a8` | `ret(0x1c79c)` |
| `ldr` at 0x55670 | load | 4 | `abs 0x55b28` | `-` |
| `str` at 0x5567c | store | 4 | `abs 0x11a3ac` | `ret(0x1c79c)` |
| `ldr` at 0x55680 | load | 4 | `abs 0x55b28` | `-` |
| `str` at 0x5568c | store | 4 | `abs 0x11a3c8` | `ret(0x1c79c)` |
| `ldr` at 0x55690 | load | 4 | `abs 0x55b28` | `-` |
| `ldr` at 0x55694 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x5569c | store | 4 | `abs 0x11a410` | `ret(0x5ea38)` |
| `ldr` at 0x556a0 | load | 4 | `abs 0x55b28` | `-` |
| `str` at 0x556ac | store | 4 | `abs 0x11a3dc` | `ret(0x1c79c)` |
| `ldr` at 0x556b0 | load | 4 | `abs 0x11a3dc` | `-` |
| `ldr` at 0x556bc | load | 4 | `abs 0x55b28` | `-` |
| `str` at 0x556c4 | store | 4 | `abs 0x11a3dc` | `0x0` |
| `ldr` at 0x556e0 | load | 4 | `abs 0x55b2c` | `-` |
| `ldr` at 0x556ec | load | 4 | `abs 0x55b30` | `-` |
| `ldr` at 0x556f0 | load | 4 | `abs 0x55b18` | `-` |
| `ldr` at 0x556f8 | load | 4 | `abs 0x55b28` | `-` |
| `str` at 0x55704 | store | 4 | `abs 0x11a3b4` | `ret(0x2fac0)` |
| `ldr` at 0x55708 | load | 4 | `abs 0x55b28` | `-` |
| `str` at 0x55710 | store | 4 | `abs 0x11a3b8` | `0x0` |
| `ldr` at 0x55714 | load | 4 | `abs 0x55b28` | `-` |
| `str` at 0x5571c | store | 4 | `abs 0x11a3cc` | `0x0` |
| `ldr` at 0x55720 | load | 4 | `abs 0x55b28` | `-` |
| `str` at 0x55728 | store | 4 | `abs 0x11a3d4` | `ret(0x3a8c0)` |
| `ldr` at 0x5572c | load | 4 | `abs 0x55b28` | `-` |
| `str` at 0x55734 | store | 4 | `abs 0x11a3d8` | `ret(0x3a8a0)` |
| `ldr` at 0x55738 | load | 4 | `abs 0x55b34` | `-` |
| `str` at 0x55740 | store | 4 | `abs 0x11a44c` | `0x1` |
| `ldr` at 0x55744 | load | 4 | `abs 0x55b34` | `-` |
| `str` at 0x5574c | store | 4 | `abs 0x11a478` | `0x2` |
| `ldr` at 0x55750 | load | 4 | `abs 0x55b34` | `-` |
| `str` at 0x55758 | store | 4 | `abs 0x11a494` | `0x3` |
| `ldr` at 0x5575c | load | 4 | `abs 0x55b34` | `-` |
| `str` at 0x55764 | store | 4 | `abs 0x11a498` | `0x4` |
| `ldr` at 0x55768 | load | 4 | `abs 0x55b34` | `-` |
| `str` at 0x55770 | store | 4 | `abs 0x11a49c` | `0x5` |
| `ldr` at 0x55774 | load | 4 | `abs 0x55b34` | `-` |
| `str` at 0x5577c | store | 4 | `abs 0x11a4a0` | `0x6` |
| `ldr` at 0x55780 | load | 4 | `abs 0x55b34` | `-` |
| `str` at 0x55788 | store | 4 | `abs 0x11a4a4` | `0x7` |
| `ldr` at 0x5578c | load | 4 | `abs 0x55b34` | `-` |
| `str` at 0x55794 | store | 4 | `abs 0x11a4a8` | `0x8` |
| `ldr` at 0x55798 | load | 4 | `abs 0x55b34` | `-` |
| `str` at 0x557a0 | store | 4 | `abs 0x11a448` | `0x9` |
| `ldr` at 0x557a4 | load | 4 | `abs 0x55b34` | `-` |
| `str` at 0x557ac | store | 4 | `abs 0x11a450` | `0xa` |
| `ldr` at 0x557b0 | load | 4 | `abs 0x55b34` | `-` |
| `str` at 0x557b8 | store | 4 | `abs 0x11a47c` | `0xb` |
| `ldr` at 0x557bc | load | 4 | `abs 0x55b34` | `-` |
| `str` at 0x557c4 | store | 4 | `abs 0x11a480` | `0xc` |
| `ldr` at 0x557c8 | load | 4 | `abs 0x55b34` | `-` |
| `str` at 0x557d0 | store | 4 | `abs 0x11a454` | `0x9` |
| `ldr` at 0x557d4 | load | 4 | `abs 0x55b34` | `-` |
| `str` at 0x557dc | store | 4 | `abs 0x11a458` | `0xa` |
| `ldr` at 0x557e0 | load | 4 | `abs 0x55b34` | `-` |
| `str` at 0x557e8 | store | 4 | `abs 0x11a474` | `0xb` |
| `ldr` at 0x557ec | load | 4 | `abs 0x55b34` | `-` |
| `str` at 0x557f4 | store | 4 | `abs 0x11a468` | `0xc` |
| `ldr` at 0x557f8 | load | 4 | `abs 0x55b28` | `-` |
| `ldr` at 0x557fc | load | 4 | `abs 0x11a3b0` | `-` |
| `ldrls` at 0x55808 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x55938 | load | 4 | `abs 0x55b38` | `-` |
| `ldr` at 0x5593c | load | 4 | `abs 0x55b3c` | `-` |
| `ldr` at 0x5594c | load | 4 | `abs 0x55b40` | `-` |
| `ldr` at 0x5595c | load | 4 | `abs 0x55b28` | `-` |
| `ldr` at 0x55960 | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x5596c | load | 4 | `abs 0x55b28` | `-` |
| `ldr` at 0x55970 | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x5597c | load | 4 | `abs 0x55b28` | `-` |
| `ldr` at 0x55980 | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x5598c | load | 4 | `abs 0x55b28` | `-` |
| `ldr` at 0x55990 | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x5599c | load | 4 | `abs 0x55b28` | `-` |
| `ldr` at 0x559a0 | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x559ac | load | 4 | `abs 0x55b28` | `-` |
| `ldr` at 0x559b0 | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x559bc | load | 4 | `abs 0x55b28` | `-` |
| `ldr` at 0x559c0 | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x559cc | load | 4 | `abs 0x55b28` | `-` |
| `ldr` at 0x559d0 | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x559dc | load | 4 | `abs 0x55b28` | `-` |
| `ldr` at 0x559e0 | load | 4 | `abs 0x11a3b0` | `-` |
| `str` at 0x559f4 | store | 4 | `sp -0x40` | `0x0` |
| `str` at 0x559fc | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0x55a04 | store | 4 | `sp -0x38` | `0x14` |
| `str` at 0x55a0c | store | 4 | `sp -0x34` | `0xa` |
| `str` at 0x55a14 | store | 4 | `sp -0x30` | `0x9` |
| `ldr` at 0x55a18 | load | 4 | `abs 0x55b44` | `-` |
| `ldr` at 0x55a1c | load | 4 | `abs 0x55b48` | `-` |
| `ldr` at 0x55a20 | load | 4 | `abs 0x55b4c` | `-` |
| `str` at 0x55a34 | store | 4 | `sp -0x40` | `0x0` |
| `str` at 0x55a3c | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0x55a44 | store | 4 | `sp -0x38` | `0x14` |
| `str` at 0x55a4c | store | 4 | `sp -0x34` | `0xa` |
| `str` at 0x55a54 | store | 4 | `sp -0x30` | `0x9` |
| `ldr` at 0x55a58 | load | 4 | `abs 0x55b44` | `-` |
| `ldr` at 0x55a5c | load | 4 | `abs 0x55b48` | `-` |
| `ldr` at 0x55a60 | load | 4 | `abs 0x55b50` | `-` |
| `ldr` at 0x55a6c | load | 4 | `abs 0x55b40` | `-` |
| `ldr` at 0x55a70 | load | 4 | `abs 0x55b54` | `-` |
| `ldr` at 0x55a78 | load | 4 | `abs 0x55b58` | `-` |
| `str` at 0x55a84 | store | 4 | `sp -0x1c` | `ret(0x99478)` |
| `ldr` at 0x55a88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x55a94 | load | 4 | `abs 0x55b5c` | `-` |
| `str` at 0x55a98 | store | 4 | `sp -0x40` | `0xe61a4` |
| `ldr` at 0x55a9c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x55aa0 | store | 4 | `sp -0x3c` | `ret(0x99478)` |
| `ldr` at 0x55aa8 | load | 4 | `abs 0x55b10` | `-` |
| `ldr` at 0x55aac | load | 4 | `abs 0x55b60` | `-` |
| `ldr` at 0x55ab8 | load | 4 | `abs 0x55b40` | `-` |
| `ldr` at 0x55ac4 | load | 4 | `abs 0x55b58` | `-` |
| `str` at 0x55ad0 | store | 4 | `sp -0x1c` | `ret(0x99904)` |
| `ldr` at 0x55ad4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x55ae0 | load | 4 | `abs 0x55b64` | `-` |
| `str` at 0x55ae4 | store | 4 | `sp -0x40` | `0xe61d0` |
| `ldr` at 0x55ae8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x55aec | store | 4 | `sp -0x3c` | `ret(0x99904)` |
| `ldr` at 0x55af4 | load | 4 | `abs 0x55b10` | `-` |
| `ldr` at 0x55af8 | load | 4 | `abs 0x55b68` | `-` |
| `ldmdb` at 0x55b04 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x55b04 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x55b04 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x55b04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x55438 | `(unknown callee) 0x2b31c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x55470 | `(unknown callee) 0x1eed0` | `0x9` | `0xe6174` | `0x6c` | `0x0` |
| `bl` at 0x55490 | `(unknown callee) 0x189c8` | `0x0` | `?` | `0x11a0c4` | `[0x11a0c4]+1` |
| `bl` at 0x5549c | `(unknown callee) 0x1c79c` | `0xf` | `?` | `?` | `?` |
| `bl` at 0x554a8 | `(unknown callee) 0x18a1c` | `ret(0x1c79c)` | `?` | `?` | `ret(0x1c79c)` |
| `bl` at 0x554c0 | `(unknown callee) 0x2b63c` | `ret(0x18a1c)` | `?` | `?` | `ret(0x18a1c)` |
| `bl` at 0x554d0 | `(unknown callee) 0x1abd0` | `0xf` | `ret(0x2b63c)` | `?` | `ret(0x2b63c)` |
| `bl` at 0x554d8 | `(unknown callee) 0x1c79c` | `0xd` | `?` | `?` | `?` |
| `bl` at 0x554e8 | `(unknown callee) 0x18cd4` | `0x282348` | `ret(0x1c79c)` | `?` | `ret(0x1c79c)` |
| `bl` at 0x55590 | `(unknown callee) 0x18920` | `?` | `?` | `[0x11a430]` | `?` |
| `bl` at 0x555c4 | `(unknown callee) 0x18c20` | `[0x11a434]` | `0x0` | `?` | `?` |
| `bl` at 0x555fc | `(unknown callee) 0x18920` | `?` | `?` | `[0x11a424]` | `?` |
| `bl` at 0x55638 | `(unknown callee) 0x18c20` | `[0x11a428]` | `0x0` | `?` | `?` |
| `bl` at 0x55668 | `(unknown callee) 0x1c79c` | `0x1` | `?` | `0x11a3a0` | `[sp-36]` |
| `bl` at 0x55678 | `(unknown callee) 0x1c79c` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x55688 | `(unknown callee) 0x1c79c` | `0x98` | `?` | `?` | `?` |
| `bl` at 0x55698 | `(unknown callee) 0x5ea38` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0x556a8 | `(unknown callee) 0x1c79c` | `0xe` | `?` | `?` | `?` |
| `bl` at 0x556d0 | `(unknown callee) 0x1abd0` | `0xe` | `0x0` | `0x11a3a0` | `0x0` |
| `bl` at 0x556d8 | `(unknown callee) 0x1c79c` | `0xf` | `?` | `?` | `?` |
| `bl` at 0x556e8 | `(unknown callee) 0x18cd4` | `0x11a3e0` | `ret(0x1c79c)` | `?` | `ret(0x1c79c)` |
| `bl` at 0x556f4 | `(unknown callee) 0x18cd4` | `0x11a3ec` | `0x282348` | `?` | `?` |
| `bl` at 0x55700 | `(unknown callee) 0x2fac0` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x55724 | `(unknown callee) 0x3a8c0` | `ret(0x2fac0)` | `?` | `0x11a3a0` | `0x0` |
| `bl` at 0x55730 | `(unknown callee) 0x3a8a0` | `ret(0x3a8c0)` | `?` | `?` | `?` |
| `bl` at 0x55934 | `(unknown callee) 0x55b6c` | `ret(0x3a8a0)` | `?` | `0x11a444` | `[0x11a3b0]-48` |
| `bl` at 0x55948 | `(unknown callee) 0x1f9e8` | `0x11a340` | `0xe618c` | `0x1` | `0x9` |
| `bl` at 0x55958 | `(unknown callee) 0x9a7e0` | `0x11a398` | `0xe` | `0x0` | `?` |
| `bl` at 0x55a28 | `(unknown callee) 0x1f498` | `0x11a394` | `0xe6198` | `0x55ec4` | `0x0` |
| `bl` at 0x55a68 | `(unknown callee) 0x1f498` | `0x11a394` | `0xe6198` | `0x55cac` | `0x0` |
| `bl` at 0x55a7c | `(unknown callee) 0x99478` | `0x11a398` | `0xffff00e` | `0x3` | `0x5c778` |
| `bl` at 0x55ab4 | `(unknown callee) 0x1eed0` | `0x9` | `0xe6174` | `0x119` | `0x0` |
| `bl` at 0x55ac8 | `(unknown callee) 0x99904` | `0x11a398` | `?` | `0x0` | `0x5c778` |
| `bl` at 0x55b00 | `(unknown callee) 0x1eed0` | `0x9` | `0xe6174` | `0x11e` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x55454 | `[0x11a0c4] == 0x0` | `[0x11a0c4] != 0x0` |
| `beq` at 0x554bc | `ret(0x18a1c) == 0x6` | `ret(0x18a1c) != 0x6` |
| `beq` at 0x55538 | `[sp-36] == 0x42` | `[sp-36] != 0x42` |
| `beq` at 0x55544 | `[sp-36] == 0x46` | `[sp-36] != 0x46` |
| `ble` at 0x556b8 | `[0x11a3dc] <= 0xfe (signed, 32-bit)` | `[0x11a3dc] > 0xfe (signed, 32-bit)` |
| `ldrls` at 0x55808 | `[0x11a3b0]-48 <= 0x22 (unsigned, 32-bit)` | `[0x11a3b0]-48 > 0x22 (unsigned, 32-bit)` |
| `beq` at 0x55968 | `[0x11a3b0] == 0x35` | `[0x11a3b0] != 0x35` |
| `beq` at 0x55978 | `[0x11a3b0] == 0x4f` | `[0x11a3b0] != 0x4f` |
| `beq` at 0x55988 | `[0x11a3b0] == 0x42` | `[0x11a3b0] != 0x42` |
| `beq` at 0x55998 | `[0x11a3b0] == 0x46` | `[0x11a3b0] != 0x46` |
| `beq` at 0x559a8 | `[0x11a3b0] == 0x4a` | `[0x11a3b0] != 0x4a` |
| `beq` at 0x559b8 | `[0x11a3b0] == 0x4b` | `[0x11a3b0] != 0x4b` |
| `beq` at 0x559c8 | `[0x11a3b0] == 0x21` | `[0x11a3b0] != 0x21` |
| `beq` at 0x559d8 | `[0x11a3b0] == 0x5e` | `[0x11a3b0] != 0x5e` |
| `beq` at 0x559e8 | `[0x11a3b0] == 0x4c` | `[0x11a3b0] != 0x4c` |
| `beq` at 0x55a90 | `ret(0x99478) == 0x0` | `ret(0x99478) != 0x0` |
| `beq` at 0x55adc | `ret(0x99904) == 0x0` | `ret(0x99904) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x55b04 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x55808).

## `L1CfgPortConfigure`

- `L1CfgPortConfigure`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x58e7c to 0x59774.
  Established. A word write increments `0x11a0d4` (`str` at 0x58ffc), and another increments
  `0x11a0d8` (`str` at 0x59020). Calls to `0x94a24` include second candidates `[sp-36]+16384` (`bl`
  at 0x593c4) and `[sp-36]+8192` (`bl` at 0x59418). The first result being zero takes the branch
  past its following call (`beq` at 0x593d0), while the nonzero fall-through calls `0x55c3c` (`bl`
  at 0x593d4). The later loop test retains an `lt` flags condition with unknown operands (`blt` at
  0x595c0). Port identity and the unresolved store targets remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x58e8c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x58e90 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x58e94 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x58e98 | load | 1 | `r0 +0x24` | `-` |
| `ldr` at 0x58ea8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x58eac | load | 4 | `r0 +0x28` | `-` |
| `str` at 0x58eb0 | store | 4 | `sp -0xa4` | `[r0+40]` |
| `ldr` at 0x58eb8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x58ec4 | store | 4 | `sp -0xa4` | `ret(0x9ca84)` |
| `ldr` at 0x58ec8 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x58ecc | store | 4 | `sp -0x2c` | `[sp-164]` |
| `ldr` at 0x58ed0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x58ed4 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x58ee4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x58ee8 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x58eec | store | 4 | `sp -0xa8` | `[[sp-24]+48]` |
| `ldr` at 0x58ef4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x58f00 | store | 4 | `sp -0xa8` | `ret(0x9ca84)` |
| `ldr` at 0x58f04 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0x58f08 | store | 4 | `sp -0xa0` | `[sp-168]` |
| `ldr` at 0x58f0c | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x58f18 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x58f1c | load | 1 | `[sp-24] +0x20` | `-` |
| `str` at 0x58f20 | store | 4 | `sp -0xac` | `[[sp-24]+32]:1` |
| `ldr` at 0x58f24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58f30 | load | 4 | `abs 0x59740` | `-` |
| `str` at 0x58f34 | store | 4 | `sp -0xb0` | `0xe66cc` |
| `ldr` at 0x58f3c | load | 4 | `abs 0x59744` | `-` |
| `str` at 0x58f40 | store | 4 | `sp -0xb0` | `0xe66d4` |
| `ldr` at 0x58f44 | load | 4 | `abs 0x59748` | `-` |
| `ldr` at 0x58f48 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x58f4c | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x58f60 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x58f6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x58f70 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x58f74 | store | 4 | `sp -0x24` | `[[sp-24]+12]` |
| `str` at 0x58f7c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x58f80 | load | 4 | `abs 0x5974c` | `-` |
| `ldr` at 0x58f88 | load | 4 | `abs 0x59750` | `-` |
| `ldr` at 0x58f8c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x58fa8 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x58fb0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x58fbc | load | 4 | `abs 0x59750` | `-` |
| `ldr` at 0x58fc0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x58fdc | load | 4 | `abs 0x11a428` | `-` |
| `str` at 0x58fe8 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x58fec | load | 4 | `abs 0x59754` | `-` |
| `ldr` at 0x58ff0 | load | 4 | `abs 0x59754` | `-` |
| `ldr` at 0x58ff4 | load | 4 | `abs 0x11a0d4` | `-` |
| `str` at 0x58ffc | store | 4 | `abs 0x11a0d4` | `[0x11a0d4]+1` |
| `ldr` at 0x59000 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x59010 | load | 4 | `abs 0x59758` | `-` |
| `ldr` at 0x59014 | load | 4 | `abs 0x59758` | `-` |
| `ldr` at 0x59018 | load | 4 | `abs 0x11a0d8` | `-` |
| `str` at 0x59020 | store | 4 | `abs 0x11a0d8` | `[0x11a0d8]+1` |
| `ldr` at 0x59024 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x59028 | load | 1 | `[sp-24] +0x16` | `-` |
| `ldr` at 0x59034 | load | 4 | `abs 0x59750` | `-` |
| `ldr` at 0x59038 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59054 | load | 4 | `abs 0x11a428` | `-` |
| `str` at 0x59060 | store | 4 | `(unknown)` | `0x2` |
| `ldr` at 0x59064 | load | 4 | `abs 0x5975c` | `-` |
| `ldr` at 0x59068 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5906c | load | 4 | `(unknown)` | `-` |
| `str` at 0x59070 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x59078 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x5907c | load | 2 | `[sp-24] +0x22` | `-` |
| `ldr` at 0x59088 | load | 4 | `abs 0x59750` | `-` |
| `ldr` at 0x5908c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x590a8 | load | 4 | `abs 0x11a428` | `-` |
| `str` at 0x590b4 | store | 4 | `(unknown)` | `0x2` |
| `ldr` at 0x590b8 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x590bc | load | 2 | `[sp-24] +0x22` | `-` |
| `str` at 0x590c0 | store | 4 | `sp -0x28` | `[[sp-24]+34]:2` |
| `ldr` at 0x590c4 | load | 4 | `abs 0x59760` | `-` |
| `ldr` at 0x590c8 | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x590d8 | load | 4 | `abs 0x59764` | `-` |
| `str` at 0x590dc | store | 4 | `sp -0xb4` | `0x116928` |
| `ldr` at 0x590e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x590ec | load | 4 | `abs 0x59768` | `-` |
| `str` at 0x590f0 | store | 4 | `sp -0xb8` | `0xe6710` |
| `ldr` at 0x590f8 | load | 4 | `abs 0x5976c` | `-` |
| `str` at 0x590fc | store | 4 | `sp -0xb8` | `0xe6718` |
| `ldr` at 0x59100 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x59108 | store | 4 | `sp -0xbc` | `[sp-36]+64` |
| `ldr` at 0x5910c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x59118 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x59120 | store | 4 | `sp -0xc0` | `[sp-40]+64` |
| `str` at 0x5912c | store | 4 | `sp -0xc0` | `0x3f` |
| `ldr` at 0x59130 | load | 4 | `sp -0xc0` | `-` |
| `str` at 0x59134 | store | 4 | `sp -0xe8` | `?` |
| `ldr` at 0x59138 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x5913c | store | 4 | `sp -0xe4` | `[sp-44]` |
| `ldr` at 0x59140 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x59144 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x59148 | load | 4 | `abs 0x59770` | `-` |
| `ldr` at 0x5914c | load | 4 | `sp -0xb8` | `-` |
| `ldr` at 0x59150 | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x59158 | load | 4 | `abs 0x59750` | `-` |
| `ldr` at 0x5915c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59178 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x59198 | load | 4 | `abs 0x59750` | `-` |
| `ldr` at 0x5919c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x591b8 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x591c0 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x591c4 | store | 4 | `(unknown)` | `[sp-148]` |
| `ldr` at 0x591c8 | load | 4 | `abs 0x59750` | `-` |
| `ldr` at 0x591cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x591e8 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x591f0 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x591f4 | store | 4 | `(unknown)` | `[sp-144]` |
| `ldr` at 0x591f8 | load | 4 | `abs 0x59750` | `-` |
| `ldr` at 0x591fc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59218 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x59220 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x59224 | store | 4 | `(unknown)` | `[sp-140]` |
| `ldr` at 0x59228 | load | 4 | `abs 0x59750` | `-` |
| `ldr` at 0x5922c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59248 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x59250 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x59258 | store | 4 | `(unknown)` | `ret(0x5ea38)` |
| `ldr` at 0x5925c | load | 4 | `abs 0x59750` | `-` |
| `ldr` at 0x59260 | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x5926c | load | 4 | `abs 0x59750` | `-` |
| `ldr` at 0x59270 | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x59280 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x5928c | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x5929c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x592a8 | load | 4 | `abs 0x59750` | `-` |
| `ldr` at 0x592ac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x592c8 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x592d0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x592d8 | load | 4 | `sp -0x74` | `-` |
| `str` at 0x592e4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x592e8 | load | 4 | `abs 0x59750` | `-` |
| `ldr` at 0x592ec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59308 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x59310 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x59314 | store | 4 | `(unknown)` | `[sp-40]` |
| `ldr` at 0x59318 | load | 4 | `abs 0x59750` | `-` |
| `ldr` at 0x5931c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59338 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x59340 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5934c | load | 4 | `abs 0x59750` | `-` |
| `ldr` at 0x59350 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59370 | load | 4 | `abs 0x11a428` | `-` |
| `ldrb` at 0x5937c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x59388 | load | 4 | `abs 0x59750` | `-` |
| `ldr` at 0x5938c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x593a8 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x593b4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x593dc | load | 4 | `abs 0x59750` | `-` |
| `ldr` at 0x593e0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x593fc | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x59408 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5942c | load | 4 | `abs 0x59750` | `-` |
| `ldr` at 0x59430 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5944c | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x59454 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x59460 | load | 4 | `abs 0x59750` | `-` |
| `ldr` at 0x59464 | load | 4 | `abs 0x11a410` | `-` |
| `ldr` at 0x59474 | load | 4 | `abs 0x59750` | `-` |
| `ldr` at 0x59478 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59494 | load | 4 | `abs 0x11a428` | `-` |
| `str` at 0x594a0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x594a4 | load | 4 | `abs 0x59750` | `-` |
| `ldr` at 0x594a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x594c4 | load | 4 | `abs 0x11a428` | `-` |
| `str` at 0x594d0 | store | 4 | `sp -0xc4` | `?` |
| `ldr` at 0x594d4 | load | 4 | `abs 0x59750` | `-` |
| `ldr` at 0x594d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x594f4 | load | 4 | `abs 0x11a428` | `-` |
| `str` at 0x59500 | store | 4 | `sp -0xc8` | `?` |
| `ldr` at 0x59504 | load | 4 | `abs 0x59750` | `-` |
| `ldr` at 0x59508 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59524 | load | 4 | `abs 0x11a428` | `-` |
| `str` at 0x59530 | store | 4 | `sp -0xcc` | `?` |
| `ldr` at 0x59534 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x59538 | store | 4 | `sp -0xe8` | `[sp-144]` |
| `ldr` at 0x5953c | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x59540 | store | 4 | `sp -0xe4` | `[sp-140]` |
| `ldr` at 0x59544 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x59554 | store | 4 | `sp -0xd0` | `?` |
| `str` at 0x59560 | store | 4 | `sp -0xd0` | `0x0` |
| `ldr` at 0x59564 | load | 4 | `sp -0xd0` | `-` |
| `str` at 0x59568 | store | 4 | `sp -0xe0` | `[sp-208]` |
| `str` at 0x59570 | store | 4 | `sp -0xdc` | `0x0` |
| `str` at 0x59578 | store | 4 | `sp -0xd8` | `?` |
| `ldr` at 0x5957c | load | 4 | `sp -0xc4` | `-` |
| `ldr` at 0x59580 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x59584 | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0x59588 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x59594 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x59598 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x595ac | store | 4 | `sp -0x20` | `0x2` |
| `ldr` at 0x595b0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x595b4 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x595b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x595c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x595cc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x595e0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x595f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x595f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x59608 | load | 4 | `(unknown)` | `-` |
| `str` at 0x5960c | store | 4 | `sp -0xd4` | `?` |
| `ldr` at 0x59614 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x59618 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x59620 | store | 4 | `sp -0xd4` | `ret(0x9ca84)` |
| `ldr` at 0x5962c | load | 4 | `sp -0xd4` | `-` |
| `ldr` at 0x59638 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x59648 | load | 4 | `abs 0x59750` | `-` |
| `ldr` at 0x5964c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59668 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x59674 | load | 4 | `abs 0x59750` | `-` |
| `ldr` at 0x59678 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59694 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x596a0 | load | 4 | `abs 0x59750` | `-` |
| `ldr` at 0x596a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x596c0 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x596cc | load | 4 | `sp -0x90` | `-` |
| `str` at 0x596d0 | store | 4 | `sp -0xe8` | `[sp-144]` |
| `ldr` at 0x596d4 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x596d8 | store | 4 | `sp -0xe4` | `[sp-140]` |
| `str` at 0x596e0 | store | 4 | `sp -0xe0` | `0x0` |
| `str` at 0x596e8 | store | 4 | `sp -0xdc` | `0x1` |
| `str` at 0x596f0 | store | 4 | `sp -0xd8` | `?` |
| `ldr` at 0x596f8 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x59704 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x59708 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x59718 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x59720 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0x59734 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x5973c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x5973c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5973c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5973c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x58ec0 | `(unknown callee) 0x9ca84` | `r0` | `0x0` | `r2` | `?` |
| `bl` at 0x58efc | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x58f50 | `(unknown callee) 0x20f14` | `0xe66e0` | `[[sp-24]+32]:1` | `[sp-176]` | `?` |
| `bl` at 0x58f68 | `(unknown callee) 0x5b9f4` | `sp-156` | `[sp-168]` | `0x70` | `sp-156` |
| `bl` at 0x58f84 | `(unknown callee) 0x56518` | `0xe66fc` | `?` | `?` | `0x0` |
| `bl` at 0x59154 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe6720` | `[sp-184]` | `[sp-36]+64` |
| `bl` at 0x59194 | `(unknown callee) 0x18ae8` | `?` | `sp-156` | `0x70` | `?` |
| `bl` at 0x59254 | `(unknown callee) 0x5ea38` | `[sp-140]` | `0x11a3a0` | `?` | `[0x11a428]` |
| `bl` at 0x593c4 | `(unknown callee) 0x94a24` | `?` | `[sp-36]+16384` | `?` | `[sp-36]+16384` |
| `bl` at 0x593d4 | `(unknown callee) 0x55c3c` | `ret(0x94a24)` | `?` | `?` | `ret(0x94a24)` |
| `bl` at 0x59418 | `(unknown callee) 0x94a24` | `?` | `[sp-36]+8192` | `?` | `[sp-36]+8192` |
| `bl` at 0x59428 | `(unknown callee) 0x55c3c` | `ret(0x94a24)` | `?` | `?` | `ret(0x94a24)` |
| `bl` at 0x5958c | `(unknown callee) 0x5abf4` | `[sp-196]` | `[sp-200]` | `[sp-204]` | `[sp-148]` |
| `bl` at 0x595a4 | `(unknown callee) 0x5af70` | `[sp-148]` | `[sp-144]` | `sp-156` | `0x0` |
| `bl` at 0x5961c | `(unknown callee) 0x9ca84` | `[sp-24]` | `?` | `0x24` | `?` |
| `bl` at 0x59634 | `(unknown callee) 0x5b9f4` | `sp-156` | `?` | `0x70` | `sp-156` |
| `bl` at 0x596fc | `(unknown callee) 0x5abf4` | `?` | `?` | `?` | `[sp-148]` |
| `bl` at 0x59714 | `(unknown callee) 0x5af70` | `[sp-148]` | `[sp-144]` | `sp-156` | `0x0` |
| `bl` at 0x59728 | `(unknown callee) 0x57930` | `?` | `?` | `[[sp-24]+32]:1` | `?` |
| `bl` at 0x59730 | `(unknown callee) 0x5659c` | `?` | `0x11a3a0` | `?` | `[0x11a410]` |
| `bl` at 0x59738 | `(unknown callee) 0x56b54` | `[sp-36]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x58ea4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x58ee0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x58f14 | `[sp-168] != 0x0` | `[sp-168] == 0x0` |
| `beq` at 0x58f2c | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `bne` at 0x58fb8 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x59008 | `[sp-140] == 0x44` | `[sp-140] != 0x44` |
| `bne` at 0x59030 | `[[sp-24]+22]:1 != 0x0` | `[[sp-24]+22]:1 == 0x0` |
| `beq` at 0x59084 | `[[sp-24]+34]:2 == 0x0` | `[[sp-24]+34]:2 != 0x0` |
| `beq` at 0x590d4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x590e8 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `beq` at 0x59114 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `beq` at 0x59268 | `[0x11a3b0] == 0x52` | `[0x11a3b0] != 0x52` |
| `beq` at 0x59278 | `[0x11a3b0] == 0x44` | `[0x11a3b0] != 0x44` |
| `beq` at 0x59288 | `[sp-140] == 0x52` | `[sp-140] != 0x52` |
| `beq` at 0x59294 | `[sp-140] == 0x44` | `[sp-140] != 0x44` |
| `beq` at 0x592a4 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `beq` at 0x59348 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x59384 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x593d0 | `ret(0x94a24) == 0x0` | `ret(0x94a24) != 0x0` |
| `beq` at 0x59424 | `ret(0x94a24) == 0x0` | `ret(0x94a24) != 0x0` |
| `bne` at 0x5945c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x5946c | `[0x11a410] != 0x0` | `[0x11a410] == 0x0` |
| `beq` at 0x5954c | `[sp-68] == 0x0` | `[sp-68] != 0x0` |
| `blt` at 0x595c0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x595ec | `? != 0x0` | `? == 0x0` |
| `bne` at 0x59640 | `[sp-156] != 0x0` | `[sp-156] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5973c | `r0` | `?` |

## `L1CfgPortDisable`

- `L1CfgPortDisable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x59774 to 0x59b44.
  Established. The global word updates include `[0x11a0d4]-1` at `0x11a0d4` (`str` at 0x5982c),
  `[0x11a0d8]-1` at `0x11a0d8` (`str` at 0x59878) and `[0x11a0d0]-1` at `0x11a0d0` (`str` at
  0x5988c). Three additional word stores contain zero at unknown effective targets (`str` at
  0x598bc, `str` at 0x598ec, `str` at 0x5991c). The zero result of `0x9c10c` falls through to the
  exit path, while a nonzero result takes the branch (`bne` at 0x59a8c). On that taken path,
  `[0x11a748]` is written at `ret(0x9c10c) +0x18` (`str` at 0x59aa0). The physical port effects are
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x59784 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x59788 | load | 4 | `abs 0x59b18` | `-` |
| `ldr` at 0x59790 | load | 4 | `abs 0x59b1c` | `-` |
| `ldr` at 0x59794 | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x597a4 | load | 4 | `abs 0x59b20` | `-` |
| `ldr` at 0x597a8 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x597ac | load | 4 | `abs 0x59b24` | `-` |
| `ldr` at 0x597b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x597b8 | load | 4 | `abs 0x59b28` | `-` |
| `ldr` at 0x597bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x597d8 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x597e0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x597e4 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x597e8 | load | 4 | `abs 0x59b28` | `-` |
| `ldr` at 0x597ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59808 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x59810 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5981c | load | 4 | `abs 0x59b2c` | `-` |
| `ldr` at 0x59820 | load | 4 | `abs 0x59b2c` | `-` |
| `ldr` at 0x59824 | load | 4 | `abs 0x11a0d4` | `-` |
| `str` at 0x5982c | store | 4 | `abs 0x11a0d4` | `[0x11a0d4]-1` |
| `ldr` at 0x59830 | load | 4 | `abs 0x59b28` | `-` |
| `ldr` at 0x59834 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59850 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x59858 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x59868 | load | 4 | `abs 0x59b30` | `-` |
| `ldr` at 0x5986c | load | 4 | `abs 0x59b30` | `-` |
| `ldr` at 0x59870 | load | 4 | `abs 0x11a0d8` | `-` |
| `str` at 0x59878 | store | 4 | `abs 0x11a0d8` | `[0x11a0d8]-1` |
| `ldr` at 0x5987c | load | 4 | `abs 0x59b34` | `-` |
| `ldr` at 0x59880 | load | 4 | `abs 0x59b34` | `-` |
| `ldr` at 0x59884 | load | 4 | `abs 0x11a0d0` | `-` |
| `str` at 0x5988c | store | 4 | `abs 0x11a0d0` | `[0x11a0d0]-1` |
| `ldr` at 0x59890 | load | 4 | `abs 0x59b28` | `-` |
| `ldr` at 0x59894 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x598b0 | load | 4 | `abs 0x11a428` | `-` |
| `str` at 0x598bc | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x598c0 | load | 4 | `abs 0x59b28` | `-` |
| `ldr` at 0x598c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x598e0 | load | 4 | `abs 0x11a428` | `-` |
| `str` at 0x598ec | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x598f0 | load | 4 | `abs 0x59b28` | `-` |
| `ldr` at 0x598f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59910 | load | 4 | `abs 0x11a428` | `-` |
| `str` at 0x5991c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x59920 | load | 4 | `abs 0x59b28` | `-` |
| `ldr` at 0x59924 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59940 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5995c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x59974 | load | 4 | `abs 0x59b28` | `-` |
| `ldr` at 0x59978 | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x59984 | load | 4 | `abs 0x59b28` | `-` |
| `ldr` at 0x59988 | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x59994 | load | 4 | `abs 0x59b28` | `-` |
| `ldr` at 0x59998 | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x599a4 | load | 4 | `abs 0x59b28` | `-` |
| `ldr` at 0x599a8 | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x599b4 | load | 4 | `abs 0x59b28` | `-` |
| `ldr` at 0x599b8 | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x599c4 | load | 4 | `abs 0x59b28` | `-` |
| `ldr` at 0x599c8 | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x599d4 | load | 4 | `abs 0x59b28` | `-` |
| `ldr` at 0x599d8 | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x599e4 | load | 4 | `abs 0x59b28` | `-` |
| `ldr` at 0x599e8 | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x599f4 | load | 4 | `abs 0x59b28` | `-` |
| `ldr` at 0x599f8 | load | 4 | `abs 0x11a3b0` | `-` |
| `str` at 0x59a54 | store | 4 | `sp -0x4c` | `0x2` |
| `ldr` at 0x59a58 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x59a5c | store | 4 | `sp -0x48` | `[sp-64]` |
| `str` at 0x59a64 | store | 4 | `sp -0x44` | `sp-60` |
| `ldr` at 0x59a68 | load | 4 | `abs 0x59b38` | `-` |
| `str` at 0x59a80 | store | 4 | `sp -0x1c` | `ret(0x9c10c)` |
| `ldr` at 0x59a84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59a94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59a98 | load | 4 | `abs 0x59b3c` | `-` |
| `ldr` at 0x59a9c | load | 4 | `abs 0x11a748` | `-` |
| `str` at 0x59aa0 | store | 4 | `ret(0x9c10c) +0x18` | `[0x11a748]` |
| `ldr` at 0x59aa4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59ac4 | load | 4 | `abs 0x59b40` | `-` |
| `ldr` at 0x59ac8 | load | 4 | `abs 0x281f78` | `-` |
| `ldr` at 0x59ad4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59adc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59afc | load | 4 | `abs 0x59b40` | `-` |
| `ldr` at 0x59b00 | load | 4 | `abs 0x281f74` | `-` |
| `ldr` at 0x59b0c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x59b14 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x59b14 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x59b14 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5978c | `(unknown callee) 0x56518` | `0xe674c` | `r1` | `r2` | `r3` |
| `bl` at 0x597b4 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe6760` | `[sp-20]` | `0x116928` |
| `bl` at 0x59958 | `(unknown callee) 0x18c20` | `?` | `0x0` | `0x70` | `?` |
| `bl` at 0x59968 | `(unknown callee) 0x57930` | `ret(0x18c20)` | `?` | `?` | `[sp-24]` |
| `bl` at 0x59970 | `(unknown callee) 0x5659c` | `ret(0x18c20)` | `?` | `?` | `[sp-24]` |
| `bl` at 0x59a1c | `(unknown callee) 0x541d4` | `sp-64` | `sp-60` | `0x1` | `sp-64` |
| `bl` at 0x59a34 | `(unknown callee) 0x54230` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x59a4c | `(unknown callee) 0x54230` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x59a78 | `(unknown callee) 0x9c10c` | `0x11a398` | `0x0` | `0x64` | `0x2` |
| `bl` at 0x59ab4 | `(unknown callee) 0x63278` | `0x2` | `?` | `ret(0x9c10c)` | `[sp-20]` |
| `bl` at 0x59ad8 | `(unknown callee) 0x9ba98` | `[0x281f78]+20` | `[sp-28]` | `?` | `[0x281f78]+20` |
| `bl` at 0x59aec | `(unknown callee) 0x63278` | `0x1` | `?` | `?` | `[sp-20]` |
| `bl` at 0x59b10 | `(unknown callee) 0x9ba98` | `[0x281f74]+20` | `[sp-28]` | `?` | `[0x281f74]+20` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x597a0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x59818 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x59860 | `? == 0x44` | `? != 0x44` |
| `beq` at 0x59964 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0x59980 | `[0x11a3b0] == 0x35` | `[0x11a3b0] != 0x35` |
| `beq` at 0x59990 | `[0x11a3b0] == 0x4f` | `[0x11a3b0] != 0x4f` |
| `beq` at 0x599a0 | `[0x11a3b0] == 0x42` | `[0x11a3b0] != 0x42` |
| `beq` at 0x599b0 | `[0x11a3b0] == 0x46` | `[0x11a3b0] != 0x46` |
| `beq` at 0x599c0 | `[0x11a3b0] == 0x4a` | `[0x11a3b0] != 0x4a` |
| `beq` at 0x599d0 | `[0x11a3b0] == 0x4b` | `[0x11a3b0] != 0x4b` |
| `beq` at 0x599e0 | `[0x11a3b0] == 0x21` | `[0x11a3b0] != 0x21` |
| `beq` at 0x599f0 | `[0x11a3b0] == 0x5e` | `[0x11a3b0] != 0x5e` |
| `beq` at 0x59a00 | `[0x11a3b0] == 0x4c` | `[0x11a3b0] != 0x4c` |
| `bne` at 0x59a8c | `ret(0x9c10c) != 0x0` | `ret(0x9c10c) == 0x0` |
| `bne` at 0x59aac | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `beq` at 0x59ac0 | `ret(0x63278) == 0x0` | `ret(0x63278) != 0x0` |
| `bne` at 0x59ae4 | `[sp-20] != 0x2` | `[sp-20] == 0x2` |
| `beq` at 0x59af8 | `ret(0x63278) == 0x0` | `ret(0x63278) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x59b14 | `r0` | `?` |

## `L1CfgFindL2`

- `L1CfgFindL2`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x59db8 to 0x5abf4.
  Established. Two scans initialize the same stack word at `sp -0x14` to zero (`str` at 0x59dd4,
  `str` at 0x5a4e8). Their loop tests expose `lt` flags conditions with unknown operands (`blt` at
  0x59de8, `blt` at 0x5a4fc). One first-scan comparison takes its branch when an unknown value is
  unsigned greater than or equal to `[[sp-24]+20]`, and falls through when less (`bhs` at 0x59e80).
  There are no call records. The return value remains unknown (`ldmdb` at 0x5abec). The
  routine name does not establish election priority or the identity of the returned value.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x59dcc | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x59dd4 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x59dd8 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x59ddc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59de0 | load | 4 | `abs 0x11a42c` | `-` |
| `ldr` at 0x59df0 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x59df4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59e08 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x59e10 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x59e1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x59e28 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x59e2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59e40 | load | 4 | `abs 0x11a434` | `-` |
| `str` at 0x59e48 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x59e50 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x59e54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59e68 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x59e70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x59e74 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x59e78 | load | 4 | `[sp-24] +0x14` | `-` |
| `ldr` at 0x59e84 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x59e88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59e9c | load | 4 | `abs 0x11a434` | `-` |
| `str` at 0x59ea4 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x59eac | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x59eb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59ec4 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x59ecc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x59ed0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x59ed4 | load | 4 | `[sp-24] +0x14` | `-` |
| `ldr` at 0x59ee0 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x59ee4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x59ee8 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x59f04 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x59f0c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x59f18 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x59f1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x59f20 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x59f3c | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x59f44 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x59f54 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x59f58 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x59f5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59f70 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x59f78 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x59f94 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x59f9c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x59fa8 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x59fac | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x59fb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59fc4 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x59fcc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x59fe8 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x59ff0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x59ffc | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a000 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a004 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a018 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5a020 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a03c | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5a044 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a050 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a054 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a058 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a06c | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5a074 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a090 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5a098 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a0a4 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a0a8 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a0ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a0c0 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5a0c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a0e4 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5a0ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a0f8 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a0fc | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a100 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a114 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5a11c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a138 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5a140 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a14c | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a150 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a154 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a168 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5a170 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a18c | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5a194 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a1a0 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a1a4 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a1a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a1bc | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5a1c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a1e0 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5a1e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a1f4 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a1f8 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a1fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a210 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5a218 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a234 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5a23c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a248 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a24c | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a250 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a264 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5a26c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a288 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5a290 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a29c | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a2a0 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a2a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a2b8 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5a2c0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a2dc | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5a2e4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a2f0 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a2f4 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a2f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a30c | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5a314 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a330 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5a338 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a344 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a348 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a34c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a360 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5a368 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a384 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5a38c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a398 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a39c | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a3a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a3b4 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5a3bc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a3d8 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5a3e0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a3ec | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a3f0 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a3f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a408 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5a410 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a42c | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5a434 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a440 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a444 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a448 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a45c | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5a464 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a480 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5a488 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a498 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a49c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a4b0 | load | 4 | `abs 0x11a434` | `-` |
| `str` at 0x5a4b8 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x5a4bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5a4c4 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0x5a4cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5a4d8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5a4dc | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x5a4e8 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x5a4ec | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a4f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a4f4 | load | 4 | `abs 0x11a42c` | `-` |
| `ldr` at 0x5a504 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a508 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a51c | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5a524 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a530 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5a53c | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a540 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a554 | load | 4 | `abs 0x11a434` | `-` |
| `str` at 0x5a55c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x5a564 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a568 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a57c | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5a584 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5a588 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a58c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a598 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a59c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a5b0 | load | 4 | `abs 0x11a434` | `-` |
| `str` at 0x5a5b8 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x5a5c0 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a5c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a5d8 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5a5e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5a5e4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a5e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a5f4 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a5f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5a5fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a618 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5a620 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a62c | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a630 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5a634 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x5a650 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5a658 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a668 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a66c | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a670 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a684 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5a68c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a6a8 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5a6b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a6bc | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a6c0 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a6c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a6d8 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5a6e0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a6fc | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5a704 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a710 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a714 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a718 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a72c | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5a734 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a750 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5a758 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a764 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a768 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a76c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a780 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5a788 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a7a4 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5a7ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a7b8 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a7bc | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a7c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a7d4 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5a7dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a7f8 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5a800 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a80c | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a810 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a814 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a828 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5a830 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a84c | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5a854 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a860 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a864 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a868 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a87c | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5a884 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a8a0 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5a8a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a8b4 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a8b8 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a8bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a8d0 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5a8d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a8f4 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5a8fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a908 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a90c | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a910 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a924 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5a92c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a948 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5a950 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a95c | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a960 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a964 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a978 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5a980 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a99c | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5a9a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a9b0 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a9b4 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5a9b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a9cc | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5a9d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5a9f0 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5a9f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5aa04 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5aa08 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5aa0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5aa20 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5aa28 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5aa44 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5aa4c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5aa58 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5aa5c | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5aa60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5aa74 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5aa7c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5aa98 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5aaa0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5aaac | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5aab0 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5aab4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5aac8 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5aad0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5aaec | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5aaf4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5ab00 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5ab04 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5ab08 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5ab1c | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5ab24 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5ab40 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5ab48 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5ab54 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5ab58 | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5ab5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5ab70 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5ab78 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5ab94 | load | 4 | `abs 0x11a428` | `-` |
| `ldr` at 0x5ab9c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5abac | load | 4 | `abs 0x5abf0` | `-` |
| `ldr` at 0x5abb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5abc4 | load | 4 | `abs 0x11a434` | `-` |
| `str` at 0x5abcc | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x5abd0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5abd8 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0x5abe0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5abe4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x5abe8 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x5abec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5abec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5abec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x59de8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x59e18 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x59e24 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `bhs` at 0x59e80 | `? >= [[sp-24]+20] (unsigned, 32-bit)` | `? < [[sp-24]+20] (unsigned, 32-bit)` |
| `bne` at 0x59edc | `? != [[sp-24]+20]` | `? == [[sp-24]+20]` |
| `beq` at 0x59f14 | `? == 0x52` | `? != 0x52` |
| `beq` at 0x59f4c | `? == 0x44` | `? != 0x44` |
| `beq` at 0x59fa4 | `? == 0x43` | `? != 0x43` |
| `beq` at 0x59ff8 | `? == 0x33` | `? != 0x33` |
| `beq` at 0x5a04c | `? == 0x35` | `? != 0x35` |
| `beq` at 0x5a0a0 | `? == 0x34` | `? != 0x34` |
| `beq` at 0x5a0f4 | `? == 0x41` | `? != 0x41` |
| `beq` at 0x5a148 | `? == 0x36` | `? != 0x36` |
| `beq` at 0x5a19c | `? == 0x39` | `? != 0x39` |
| `beq` at 0x5a1f0 | `? == 0x37` | `? != 0x37` |
| `beq` at 0x5a244 | `? == 0x38` | `? != 0x38` |
| `beq` at 0x5a298 | `? == 0x30` | `? != 0x30` |
| `beq` at 0x5a2ec | `? == 0x51` | `? != 0x51` |
| `beq` at 0x5a340 | `? == 0x40` | `? != 0x40` |
| `beq` at 0x5a394 | `? == 0x4f` | `? != 0x4f` |
| `beq` at 0x5a3e8 | `? == 0x42` | `? != 0x42` |
| `beq` at 0x5a43c | `? == 0x46` | `? != 0x46` |
| `beq` at 0x5a490 | `? == 0x4c` | `? != 0x4c` |
| `beq` at 0x5a4d4 | `Z == 1` | `not (Z == 1)` |
| `blt` at 0x5a4fc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x5a52c | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x5a538 | `Z == 0` | `not (Z == 0)` |
| `bhs` at 0x5a594 | `C == 1` | `not (C == 1)` |
| `bne` at 0x5a5f0 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x5a628 | `? == 0x52` | `? != 0x52` |
| `beq` at 0x5a660 | `? == 0x44` | `? != 0x44` |
| `beq` at 0x5a6b8 | `? == 0x43` | `? != 0x43` |
| `beq` at 0x5a70c | `? == 0x33` | `? != 0x33` |
| `beq` at 0x5a760 | `? == 0x35` | `? != 0x35` |
| `beq` at 0x5a7b4 | `? == 0x34` | `? != 0x34` |
| `beq` at 0x5a808 | `? == 0x41` | `? != 0x41` |
| `beq` at 0x5a85c | `? == 0x36` | `? != 0x36` |
| `beq` at 0x5a8b0 | `? == 0x39` | `? != 0x39` |
| `beq` at 0x5a904 | `? == 0x37` | `? != 0x37` |
| `beq` at 0x5a958 | `? == 0x38` | `? != 0x38` |
| `beq` at 0x5a9ac | `? == 0x30` | `? != 0x30` |
| `beq` at 0x5aa00 | `? == 0x51` | `? != 0x51` |
| `beq` at 0x5aa54 | `? == 0x40` | `? != 0x40` |
| `beq` at 0x5aaa8 | `? == 0x4f` | `? != 0x4f` |
| `beq` at 0x5aafc | `? == 0x42` | `? != 0x42` |
| `beq` at 0x5ab50 | `? == 0x46` | `? != 0x46` |
| `beq` at 0x5aba4 | `? == 0x4c` | `? != 0x4c` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5abec | `r0` | `?` |

## `L1CfgAddBrick`

- `L1CfgAddBrick`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5abf4 to 0x5af70.
  Established. The call to `0x59b44` has candidates `[r0]`, `[r1]`, `r3` and `[sp]` (`bl` at
  0x5ac2c). A zero result takes the branch into the longer path (`beq` at 0x5ac40). The nonzero
  fall-through writes `[sp+8]` at `ret(0x59b44) +0xc` (`str` at 0x5ac4c). Later word stores include
  `[sp-36]` (`str` at 0x5ae80), `[sp]` (`str` at 0x5aea4) and `[sp+4]` (`str` at 0x5aec8), all with
  unknown effective targets. The global word at `0x11a0c0` receives `[[sp-28]]` (`str` at 0x5af58).
  Record ownership and field roles are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5ac04 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x5ac08 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x5ac0c | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x5ac10 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0x5ac14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5ac18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5ac1c | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0x5ac20 | load | 4 | `r1 +0x0` | `-` |
| `ldr` at 0x5ac24 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5ac28 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x5ac34 | store | 4 | `sp -0x30` | `ret(0x59b44)` |
| `ldr` at 0x5ac38 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5ac44 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5ac48 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x5ac4c | store | 4 | `ret(0x59b44) +0xc` | `[sp+8]` |
| `ldr` at 0x5ac54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5ac58 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x5ac60 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5ac64 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x5ac70 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5ac74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5ac78 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0x5ac80 | store | 4 | `[sp-32] +0x0` | `[[sp-32]]+16` |
| `ldr` at 0x5ac84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5ac88 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5ac8c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5ac90 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x5aca4 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x5acb4 | store | 4 | `[sp-24] +0x0` | `ret(0x18aac)` |
| `ldr` at 0x5acb8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5acbc | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x5acc0 | store | 4 | `sp -0x2c` | `[[sp-24]]` |
| `str` at 0x5acc8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x5accc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5acd0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5acd4 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x5ace4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5acec | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x5acf4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5ad08 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5ad10 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5ad18 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5ad2c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5ad34 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5ad44 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x5ad4c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x5ad54 | load | 4 | `abs 0x5af60` | `-` |
| `ldr` at 0x5ad58 | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x5ad68 | load | 4 | `abs 0x5af64` | `-` |
| `ldr` at 0x5ad6c | load | 4 | `sp +0x0` | `-` |
| `str` at 0x5ad70 | store | 4 | `sp -0x44` | `[sp]` |
| `ldr` at 0x5ad74 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x5ad78 | store | 4 | `sp -0x40` | `[sp+16]` |
| `ldr` at 0x5ad7c | load | 4 | `sp +0xc` | `-` |
| `str` at 0x5ad80 | store | 4 | `sp -0x3c` | `[sp+12]` |
| `ldr` at 0x5ad84 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x5ad88 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x5ad8c | load | 4 | `sp +0x8` | `-` |
| `str` at 0x5ad90 | store | 4 | `sp -0x34` | `[sp+8]` |
| `ldr` at 0x5ad94 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x5ad98 | load | 4 | `abs 0x5af68` | `-` |
| `ldr` at 0x5ad9c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5ada0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5ada8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5adb4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5adbc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5adc0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x5add8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5ade8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5adec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5adf0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x5adfc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5ae10 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5ae1c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5ae30 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5ae38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5ae3c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x5ae40 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5ae60 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5ae74 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5ae7c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x5ae80 | store | 4 | `(unknown)` | `[sp-36]` |
| `ldr` at 0x5ae84 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5ae98 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5aea0 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x5aea4 | store | 4 | `(unknown)` | `[sp]` |
| `ldr` at 0x5aea8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5aebc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5aec4 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x5aec8 | store | 4 | `(unknown)` | `[sp+4]` |
| `ldr` at 0x5aecc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5aee0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5aee8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x5aeec | store | 4 | `(unknown)` | `[sp+8]` |
| `ldr` at 0x5aef0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5af04 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5af0c | load | 4 | `sp +0xc` | `-` |
| `str` at 0x5af10 | store | 4 | `(unknown)` | `[sp+12]` |
| `ldr` at 0x5af14 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5af28 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5af30 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x5af34 | store | 4 | `(unknown)` | `[sp+16]` |
| `ldr` at 0x5af38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5af3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5af40 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x5af48 | store | 4 | `[sp-28] +0x0` | `[[sp-28]]+1` |
| `ldr` at 0x5af4c | load | 4 | `abs 0x5af6c` | `-` |
| `ldr` at 0x5af50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5af54 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x5af58 | store | 4 | `abs 0x11a0c0` | `[[sp-28]]` |
| `ldmdb` at 0x5af5c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x5af5c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5af5c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5af5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5ac2c | `(unknown callee) 0x59b44` | `[r0]` | `[r1]` | `r3` | `[sp]` |
| `bl` at 0x5acac | `(unknown callee) 0x18aac` | `[[sp-24]]` | `?` | `[[sp-32]]` | `?` |
| `bl` at 0x5ada4 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe6778` | `[[sp-24]]` | `[sp-36]` |
| `bl` at 0x5ade0 | `(unknown callee) 0x18968` | `?` | `?` | `?` | `?` |
| `bl` at 0x5ae5c | `(unknown callee) 0x18968` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5ac40 | `ret(0x59b44) == 0x0` | `ret(0x59b44) != 0x0` |
| `ble` at 0x5ac6c | `[[sp-28]]+1 <= [[sp-32]] (signed, 32-bit)` | `[[sp-28]]+1 > [[sp-32]] (signed, 32-bit)` |
| `blt` at 0x5acdc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blo` at 0x5ad40 | `? < ? (unsigned, 32-bit)` | `? >= ? (unsigned, 32-bit)` |
| `beq` at 0x5ad64 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x5adb0 | `Z == 0` | `not (Z == 0)` |
| `bge` at 0x5adf8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5af5c | `r0` | `?` |

## `L1CfgAddState`

- `L1CfgAddState`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5af70 to 0x5b2bc.
  Established. Entry `r3 == 0` takes the branch past the call at 0x5afbc, and `r3 != 0` falls through
  (`beq` at 0x5afa0). A subsequent zero result from `0x59c60` takes the branch into the longer path
  (`beq` at 0x5b028). That path includes a word increment by 16 at `0x11a43c` (`str` at 0x5b098) and
  a word store of `ret(0x18aac)` at `0x11a440` (`str` at 0x5b0cc). Another word write puts
  `[0x11a438]+1` at `0x11a438` (`str` at 0x5b29c). The stored objects and their hardware
  significance remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5af80 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x5af84 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x5af88 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x5af8c | store | 4 | `sp -0x24` | `r3` |
| `str` at 0x5af94 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x5af98 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5afa4 | load | 4 | `abs 0x5b2a4` | `-` |
| `ldr` at 0x5afa8 | load | 4 | `abs 0x5b2a4` | `-` |
| `ldr` at 0x5afac | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5afb0 | load | 4 | `abs 0x11a42c` | `-` |
| `ldr` at 0x5afb4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5afb8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5afcc | load | 4 | `abs 0x5b2a8` | `-` |
| `ldr` at 0x5afd0 | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x5afe0 | load | 4 | `abs 0x5b2ac` | `-` |
| `ldr` at 0x5afe4 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x5afe8 | load | 4 | `abs 0x5b2b0` | `-` |
| `ldr` at 0x5afec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5aff0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5affc | load | 4 | `abs 0x5b2a4` | `-` |
| `ldr` at 0x5b000 | load | 4 | `abs 0x5b2a4` | `-` |
| `ldr` at 0x5b004 | load | 4 | `abs 0x11a440` | `-` |
| `ldr` at 0x5b008 | load | 4 | `abs 0x11a438` | `-` |
| `ldr` at 0x5b00c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5b010 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5b01c | store | 4 | `sp -0x28` | `ret(0x59c60)` |
| `ldr` at 0x5b020 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5b02c | load | 4 | `abs 0x5b2a8` | `-` |
| `ldr` at 0x5b030 | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x5b040 | load | 4 | `abs 0x5b2ac` | `-` |
| `ldr` at 0x5b044 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x5b048 | load | 4 | `abs 0x5b2b4` | `-` |
| `ldr` at 0x5b04c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5b050 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5b058 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5b05c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5b06c | load | 4 | `abs 0x5b2a4` | `-` |
| `ldr` at 0x5b070 | load | 4 | `abs 0x11a438` | `-` |
| `ldr` at 0x5b078 | load | 4 | `abs 0x5b2a4` | `-` |
| `ldr` at 0x5b07c | load | 4 | `abs 0x11a43c` | `-` |
| `ldr` at 0x5b088 | load | 4 | `abs 0x5b2a4` | `-` |
| `ldr` at 0x5b08c | load | 4 | `abs 0x5b2a4` | `-` |
| `ldr` at 0x5b090 | load | 4 | `abs 0x11a43c` | `-` |
| `str` at 0x5b098 | store | 4 | `abs 0x11a43c` | `[0x11a43c]+16` |
| `ldr` at 0x5b09c | load | 4 | `abs 0x5b2a4` | `-` |
| `ldr` at 0x5b0a0 | load | 4 | `abs 0x5b2a4` | `-` |
| `ldr` at 0x5b0a4 | load | 4 | `abs 0x5b2a4` | `-` |
| `ldr` at 0x5b0a8 | load | 4 | `abs 0x11a43c` | `-` |
| `ldr` at 0x5b0bc | load | 4 | `abs 0x11a440` | `-` |
| `str` at 0x5b0cc | store | 4 | `abs 0x11a440` | `ret(0x18aac)` |
| `str` at 0x5b0d4 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x5b0d8 | load | 4 | `abs 0x5b2a4` | `-` |
| `ldr` at 0x5b0dc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5b0e0 | load | 4 | `abs 0x11a438` | `-` |
| `ldr` at 0x5b0f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5b0f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5b100 | load | 4 | `abs 0x5b2a4` | `-` |
| `ldr` at 0x5b104 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5b118 | load | 4 | `abs 0x11a440` | `-` |
| `ldr` at 0x5b120 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5b128 | load | 4 | `abs 0x5b2a4` | `-` |
| `ldr` at 0x5b12c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5b140 | load | 4 | `abs 0x11a440` | `-` |
| `ldr` at 0x5b148 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5b158 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x5b160 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x5b168 | load | 4 | `abs 0x5b2a8` | `-` |
| `ldr` at 0x5b16c | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x5b17c | load | 4 | `abs 0x5b2ac` | `-` |
| `ldr` at 0x5b180 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x5b184 | load | 4 | `abs 0x5b2b8` | `-` |
| `ldr` at 0x5b188 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5b18c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5b194 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5b1a0 | load | 4 | `abs 0x5b2a4` | `-` |
| `ldr` at 0x5b1a4 | load | 4 | `abs 0x11a440` | `-` |
| `ldr` at 0x5b1ac | load | 4 | `abs 0x5b2a4` | `-` |
| `ldr` at 0x5b1b0 | load | 4 | `abs 0x5b2a4` | `-` |
| `ldr` at 0x5b1b4 | load | 4 | `abs 0x11a438` | `-` |
| `ldr` at 0x5b1cc | load | 4 | `abs 0x11a440` | `-` |
| `ldr` at 0x5b1dc | load | 4 | `abs 0x5b2a4` | `-` |
| `ldr` at 0x5b1e0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5b1e4 | load | 4 | `abs 0x11a438` | `-` |
| `ldr` at 0x5b1f0 | load | 4 | `abs 0x5b2a4` | `-` |
| `ldr` at 0x5b1f4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5b208 | load | 4 | `abs 0x11a440` | `-` |
| `ldr` at 0x5b214 | load | 4 | `abs 0x5b2a4` | `-` |
| `ldr` at 0x5b218 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5b22c | load | 4 | `abs 0x11a440` | `-` |
| `ldr` at 0x5b234 | load | 4 | `abs 0x5b2a4` | `-` |
| `ldr` at 0x5b238 | load | 4 | `abs 0x11a438` | `-` |
| `ldr` at 0x5b23c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5b25c | load | 4 | `abs 0x5b2a4` | `-` |
| `ldr` at 0x5b260 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5b274 | load | 4 | `abs 0x11a440` | `-` |
| `ldr` at 0x5b280 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5b28c | load | 4 | `abs 0x5b2a4` | `-` |
| `ldr` at 0x5b290 | load | 4 | `abs 0x5b2a4` | `-` |
| `ldr` at 0x5b294 | load | 4 | `abs 0x11a438` | `-` |
| `str` at 0x5b29c | store | 4 | `abs 0x11a438` | `[0x11a438]+1` |
| `ldmdb` at 0x5b2a0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x5b2a0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5b2a0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5b2a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5afbc | `(unknown callee) 0x59b44` | `[0x11a434]` | `[0x11a42c]` | `r0` | `r1` |
| `bl` at 0x5aff4 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe67b0` | `[sp-24]` | `[sp-28]` |
| `bl` at 0x5b014 | `(unknown callee) 0x59c60` | `[0x11a440]` | `[0x11a438]` | `?` | `?` |
| `bl` at 0x5b054 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe67d0` | `[sp-24]` | `[sp-28]` |
| `bl` at 0x5b064 | `(unknown callee) 0x18ae8` | `?` | `[sp-32]` | `0x70` | `?` |
| `bl` at 0x5b0c4 | `(unknown callee) 0x18aac` | `[0x11a440]` | `?` | `[0x11a43c]` | `?` |
| `bl` at 0x5b190 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe67e8` | `[sp-24]` | `[sp-28]` |
| `bl` at 0x5b1d4 | `(unknown callee) 0x18968` | `[0x11a440]+112` | `[0x11a440]` | `?` | `?` |
| `bl` at 0x5b258 | `(unknown callee) 0x18968` | `?` | `?` | `?` | `?` |
| `bl` at 0x5b288 | `(unknown callee) 0x18ae8` | `?` | `[sp-32]` | `0x70` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5afa0 | `r3 == 0x0` | `r3 != 0x0` |
| `bne` at 0x5afc8 | `ret(0x59b44) != 0x0` | `ret(0x59b44) == 0x0` |
| `beq` at 0x5afdc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x5b028 | `ret(0x59c60) == 0x0` | `ret(0x59c60) != 0x0` |
| `beq` at 0x5b03c | `? == 0x0` | `? != 0x0` |
| `ble` at 0x5b084 | `[0x11a438]+1 <= [0x11a43c] (signed, 32-bit)` | `[0x11a438]+1 > [0x11a43c] (signed, 32-bit)` |
| `blt` at 0x5b0e8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x5b154 | `? < ? (signed, 32-bit)` | `? >= ? (signed, 32-bit)` |
| `beq` at 0x5b178 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x5b19c | `Z == 0` | `not (Z == 0)` |
| `bge` at 0x5b1ec | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5b2a0 | `r0` | `?` |

## `L1CfgDelete`

- `L1CfgDelete`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5b2bc to 0x5b5f8.
  Established. The first zero comparison takes its branch when entry `r0` is zero and falls through
  otherwise (`beq` at 0x5b2ec). Two calls to `0x18c20` use candidates `(unknown, 0x0, 0x18,
  unknown)` (`bl` at 0x5b434) and `(unknown, 0x0, 0x70, unknown)` (`bl` at 0x5b5b8). Word updates
  write `[[sp-24]]-1` at `[sp-24] +0x0` (`str` at 0x5b448) and `[[sp-32]]-1` at `[sp-32] +0x0`
  (`str` at 0x5b5cc). The meanings of the updated words and the external effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5b2cc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x5b2d0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x5b2d4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x5b2d8 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0x5b2e0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x5b2e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5b2f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5b2f4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5b2f8 | load | 4 | `r1 +0x0` | `-` |
| `ldr` at 0x5b308 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5b31c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5b324 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5b328 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x5b334 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5b348 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5b350 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5b354 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x5b360 | load | 4 | `abs 0x5b5e8` | `-` |
| `ldr` at 0x5b364 | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x5b374 | load | 4 | `abs 0x5b5ec` | `-` |
| `ldr` at 0x5b378 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x5b37c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x5b380 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x5b384 | load | 4 | `abs 0x5b5f0` | `-` |
| `ldr` at 0x5b388 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x5b38c | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x5b394 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5b3a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5b3b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5b3c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5b3d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5b3d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5b3d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5b404 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5b408 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x5b41c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5b438 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5b43c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5b440 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x5b448 | store | 4 | `[sp-24] +0x0` | `[[sp-24]]-1` |
| `ldr` at 0x5b450 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x5b458 | store | 4 | `sp -0x24` | `?` |
| `str` at 0x5b464 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x5b468 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5b474 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5b478 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5b47c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5b48c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5b4a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5b4a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5b4ac | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x5b4b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5b4cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5b4d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5b4d8 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x5b4e4 | load | 4 | `abs 0x5b5e8` | `-` |
| `ldr` at 0x5b4e8 | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x5b4f8 | load | 4 | `abs 0x5b5ec` | `-` |
| `ldr` at 0x5b4fc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x5b500 | store | 4 | `sp -0x28` | `[sp-36]` |
| `ldr` at 0x5b504 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x5b508 | load | 4 | `abs 0x5b5f4` | `-` |
| `ldr` at 0x5b50c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x5b510 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x5b518 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5b52c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5b534 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5b548 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5b554 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5b558 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x5b55c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5b588 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5b58c | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x5b5a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5b5bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5b5c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5b5c4 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0x5b5cc | store | 4 | `[sp-32] +0x0` | `[[sp-32]]-1` |
| `ldr` at 0x5b5d4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x5b5dc | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldmdb` at 0x5b5e4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5b5e4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5b5e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5b390 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe6800` | `[sp]` | `[sp+4]` |
| `bl` at 0x5b400 | `(unknown callee) 0x18968` | `?` | `?` | `?` | `?` |
| `bl` at 0x5b434 | `(unknown callee) 0x18c20` | `?` | `0x0` | `0x18` | `?` |
| `bl` at 0x5b514 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe6820` | `[sp]` | `[sp+4]` |
| `bl` at 0x5b584 | `(unknown callee) 0x18968` | `?` | `?` | `?` | `?` |
| `bl` at 0x5b5b8 | `(unknown callee) 0x18c20` | `?` | `0x0` | `0x70` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5b2ec | `r0 == 0x0` | `r0 != 0x0` |
| `blt` at 0x5b300 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x5b330 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x5b35c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x5b370 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x5b470 | `Z == 1` | `not (Z == 1)` |
| `blt` at 0x5b484 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x5b4b4 | `? != [sp]` | `? == [sp]` |
| `bne` at 0x5b4e0 | `? != [sp+4]` | `? == [sp+4]` |
| `beq` at 0x5b4f4 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5b5e4 | `r0` | `?` |

## `L1CfgUpdateState`

- `L1CfgUpdateState`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5b5f8 to 0x5b7e4.
  Established. The call result from `0x189c8` is written as a word at `0x11a0ec` (`str` at 0x5b6b8).
  The earlier tests take their branches for `[sp-292] != [0x11a3a8]` (`bne` at 0x5b690) and
  `[sp-296] != [0x11a3ac]` (`bne` at 0x5b6a4), with equality falling through in each case. A later
  call to `0x5af70` has candidates `[sp-404]`, `[sp-400]`, `sp-412` and `0x1` (`bl` at 0x5b7b0).
  State-field meanings and the physical effect are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5b608 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x5b60c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5b610 | load | 1 | `r0 +0x24` | `-` |
| `ldr` at 0x5b620 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5b624 | load | 4 | `r0 +0x28` | `-` |
| `str` at 0x5b628 | store | 4 | `sp -0x1a0` | `[r0+40]` |
| `ldr` at 0x5b630 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5b63c | store | 4 | `sp -0x1a0` | `ret(0x9ca84)` |
| `ldr` at 0x5b640 | load | 4 | `sp -0x1a0` | `-` |
| `str` at 0x5b644 | store | 4 | `sp -0x120` | `[sp-416]` |
| `ldr` at 0x5b648 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5b64c | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0x5b65c | store | 4 | `sp -0x124` | `?` |
| `ldr` at 0x5b660 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5b664 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0x5b670 | store | 4 | `sp -0x128` | `?` |
| `ldr` at 0x5b674 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5b678 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x5b67c | store | 4 | `sp -0x12c` | `[[sp-24]+12]` |
| `ldr` at 0x5b680 | load | 4 | `abs 0x5b7cc` | `-` |
| `ldr` at 0x5b684 | load | 4 | `sp -0x124` | `-` |
| `ldr` at 0x5b688 | load | 4 | `abs 0x11a3a8` | `-` |
| `ldr` at 0x5b694 | load | 4 | `abs 0x5b7cc` | `-` |
| `ldr` at 0x5b698 | load | 4 | `sp -0x128` | `-` |
| `ldr` at 0x5b69c | load | 4 | `abs 0x11a3ac` | `-` |
| `ldr` at 0x5b6ac | load | 4 | `abs 0x5b7d0` | `-` |
| `str` at 0x5b6b8 | store | 4 | `abs 0x11a0ec` | `ret(0x189c8)` |
| `ldr` at 0x5b6bc | load | 4 | `abs 0x5b7d4` | `-` |
| `ldr` at 0x5b6c4 | load | 4 | `abs 0x5b7d8` | `-` |
| `ldr` at 0x5b6c8 | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x5b6e4 | load | 4 | `abs 0x5b7dc` | `-` |
| `ldr` at 0x5b6e8 | load | 4 | `sp -0x12c` | `-` |
| `str` at 0x5b6ec | store | 4 | `sp -0x1ac` | `[sp-300]` |
| `str` at 0x5b6f0 | store | 4 | `sp -0x1a8` | `ret(0x189c8)` |
| `ldr` at 0x5b6f4 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x5b6f8 | load | 4 | `abs 0x5b7e0` | `-` |
| `ldr` at 0x5b6fc | load | 4 | `sp -0x124` | `-` |
| `ldr` at 0x5b700 | load | 4 | `sp -0x128` | `-` |
| `str` at 0x5b70c | store | 4 | `sp -0x11c` | `0x0` |
| `ldr` at 0x5b710 | load | 4 | `sp -0x11c` | `-` |
| `ldr` at 0x5b714 | load | 4 | `sp -0x120` | `-` |
| `ldr` at 0x5b724 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5b728 | load | 4 | `sp -0x11c` | `-` |
| `ldrb` at 0x5b73c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5b74c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5b750 | load | 4 | `sp -0x11c` | `-` |
| `ldr` at 0x5b764 | load | 4 | `(unknown)` | `-` |
| `str` at 0x5b768 | store | 4 | `sp -0x1a4` | `?` |
| `ldr` at 0x5b770 | load | 4 | `sp -0x11c` | `-` |
| `ldr` at 0x5b778 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5b784 | store | 4 | `sp -0x1a4` | `ret(0x9ca84)` |
| `ldr` at 0x5b790 | load | 4 | `sp -0x1a4` | `-` |
| `ldr` at 0x5b7a0 | load | 4 | `sp -0x194` | `-` |
| `ldr` at 0x5b7a4 | load | 4 | `sp -0x190` | `-` |
| `ldr` at 0x5b7b4 | load | 4 | `sp -0x11c` | `-` |
| `str` at 0x5b7bc | store | 4 | `sp -0x11c` | `[sp-284]+1` |
| `ldmdb` at 0x5b7c8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x5b7c8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5b7c8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5b7c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5b638 | `(unknown callee) 0x9ca84` | `r0` | `0x0` | `r2` | `?` |
| `bl` at 0x5b6b4 | `(unknown callee) 0x189c8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x5b6c0 | `(unknown callee) 0x56518` | `0xe6840` | `?` | `?` | `?` |
| `bl` at 0x5b6dc | `(unknown callee) 0x189c8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x5b704 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe6854` | `[sp-292]` | `[sp-296]` |
| `bl` at 0x5b780 | `(unknown callee) 0x9ca84` | `[sp-24]` | `?` | `0x2c` | `?` |
| `bl` at 0x5b798 | `(unknown callee) 0x5b9f4` | `sp-412` | `?` | `0x70` | `sp-412` |
| `bl` at 0x5b7b0 | `(unknown callee) 0x5af70` | `[sp-404]` | `[sp-400]` | `sp-412` | `0x1` |
| `bl` at 0x5b7c4 | `(unknown callee) 0x5659c` | `?` | `?` | `?` | `[sp-288]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5b61c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x5b690 | `[sp-292] != [0x11a3a8]` | `[sp-292] == [0x11a3a8]` |
| `bne` at 0x5b6a4 | `[sp-296] != [0x11a3ac]` | `[sp-296] == [0x11a3ac]` |
| `beq` at 0x5b6d4 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x5b71c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x5b748 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5b7c8 | `r0` | `?` |

## `L1CfgRequest`

- `L1CfgRequest`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5bba0 to 0x5bde4.
  Established. The initial call to `0x63278` uses `[sp-84]` as its first candidate (`bl` at
  0x5bbd0); its nonzero result takes the branch to 0x5bbe4, while zero falls
  through to the return (`bne` at 0x5bbdc). The computed transfer is taken when `[sp-20]-1 <=
  0x18` unsigned and falls through when greater (`ldrls` at 0x5bc08); its target edges remain
  unresolved, so the later decoded call sites are not included in the known-edge call table. The
  request format and delivery effect are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5bbb0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x5bbb8 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0x5bbbc | load | 4 | `abs 0x5bdbc` | `-` |
| `ldr` at 0x5bbc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5bbc4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x5bbc8 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x5bbcc | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x5bbf4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5bbfc | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0x5bc08 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x5bdb8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5bdb8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5bdb8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5bbd0 | `(unknown callee) 0x63278` | `[sp-84]` | `r1` | `0x11a444` | `?` |
| `bl` at 0x5bbf8 | `(unknown callee) 0x541d4` | `sp-76` | `sp-72` | `[sp-20]` | `sp-76` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5bbdc | `ret(0x63278) != 0x0` | `ret(0x63278) == 0x0` |
| `ldrls` at 0x5bc08 | `[sp-20]-1 <= 0x18 (unsigned, 32-bit)` | `[sp-20]-1 > 0x18 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5bdb8 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x5bc08).

## `L1CfgProcessRequest`

- `L1CfgProcessRequest`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5bde4 to 0x5c49c.
  Established. The selector comparison takes its branch for `[sp-116] == 0x3e8` (`beq` at 0x5becc).
  That path writes the word `ret(0x189c8)` at `0x11a0e4` (`str` at 0x5bee0) before calling `0x56610`
  (`bl` at 0x5bee8). Other paths store words `[0x11a42c]` at `0x11a334` (`str` at 0x5c05c) and
  `ret(0x18920)` at `0x11a338` (`str` at 0x5c074) and `0x11a33c` (`str` at 0x5c0a0). The byte store
  at `([0x11a338] + [sp-136]) +0x0` contains `([sp-128]:1 & 0xff)` (`strb` at 0x5c16c). The selector
  and output-field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5bdf4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x5bdfc | store | 4 | `sp -0x64` | `0x0` |
| `ldr` at 0x5be00 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5be04 | load | 1 | `r0 +0x24` | `-` |
| `ldr` at 0x5be14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5be18 | load | 4 | `r0 +0x28` | `-` |
| `str` at 0x5be1c | store | 4 | `sp -0x8c` | `[r0+40]` |
| `ldr` at 0x5be24 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5be30 | store | 4 | `sp -0x8c` | `ret(0x9ca84)` |
| `ldr` at 0x5be34 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x5be38 | store | 4 | `sp -0x74` | `[sp-140]` |
| `ldr` at 0x5be4c | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x5be54 | load | 4 | `sp -0x74` | `-` |
| `str` at 0x5be58 | store | 4 | `sp -0x98` | `[sp-116]` |
| `ldr` at 0x5be5c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x5be68 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x5be74 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x5be80 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x5be8c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x5be9c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x5beac | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x5beb8 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x5bec4 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x5bed4 | load | 4 | `abs 0x5c45c` | `-` |
| `str` at 0x5bee0 | store | 4 | `abs 0x11a0e4` | `ret(0x189c8)` |
| `ldr` at 0x5bee4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5bef0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5bef4 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x5bf00 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5bf04 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x5bf14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5bf18 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x5bf1c | store | 4 | `sp -0x90` | `[[sp-24]+48]` |
| `ldr` at 0x5bf24 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5bf30 | store | 4 | `sp -0x90` | `ret(0x9ca84)` |
| `ldr` at 0x5bf34 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x5bf38 | store | 4 | `sp -0x78` | `[sp-144]` |
| `ldr` at 0x5bf3c | load | 4 | `sp -0x78` | `-` |
| `str` at 0x5bf4c | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0x5bf50 | load | 4 | `abs 0x5c460` | `-` |
| `ldr` at 0x5bf54 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x5bf68 | load | 4 | `abs 0x5c460` | `-` |
| `ldr` at 0x5bf6c | load | 4 | `sp -0x78` | `-` |
| `str` at 0x5bf78 | store | 4 | `sp -0x7c` | `0x1` |
| `ldr` at 0x5bf7c | load | 4 | `sp -0x78` | `-` |
| `str` at 0x5bf98 | store | 4 | `sp -0x7c` | `0x1` |
| `ldr` at 0x5bf9c | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x5bfac | load | 4 | `abs 0x5c464` | `-` |
| `ldr` at 0x5bfb4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5bfb8 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x5bfc4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5bfc8 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x5bfd8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5bfdc | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0x5bfe0 | store | 4 | `sp -0x94` | `[[sp-24]+56]` |
| `ldr` at 0x5bfe8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5bff4 | store | 4 | `sp -0x94` | `ret(0x9ca84)` |
| `ldr` at 0x5bff8 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x5bffc | store | 4 | `sp -0x7c` | `[sp-148]` |
| `ldr` at 0x5c000 | load | 4 | `abs 0x5c468` | `-` |
| `ldr` at 0x5c004 | load | 4 | `abs 0x5c46c` | `-` |
| `ldr` at 0x5c008 | load | 4 | `abs 0x11a334` | `-` |
| `ldr` at 0x5c00c | load | 4 | `abs 0x11a42c` | `-` |
| `ldr` at 0x5c018 | load | 4 | `abs 0x5c470` | `-` |
| `ldr` at 0x5c01c | load | 4 | `abs 0x11a338` | `-` |
| `ldr` at 0x5c028 | load | 4 | `abs 0x5c470` | `-` |
| `ldr` at 0x5c02c | load | 4 | `abs 0x11a338` | `-` |
| `ldr` at 0x5c034 | load | 4 | `abs 0x5c474` | `-` |
| `ldr` at 0x5c038 | load | 4 | `abs 0x11a33c` | `-` |
| `ldr` at 0x5c044 | load | 4 | `abs 0x5c474` | `-` |
| `ldr` at 0x5c048 | load | 4 | `abs 0x11a33c` | `-` |
| `ldr` at 0x5c050 | load | 4 | `abs 0x5c468` | `-` |
| `ldr` at 0x5c054 | load | 4 | `abs 0x5c46c` | `-` |
| `ldr` at 0x5c058 | load | 4 | `abs 0x11a42c` | `-` |
| `str` at 0x5c05c | store | 4 | `abs 0x11a334` | `[0x11a42c]` |
| `ldr` at 0x5c060 | load | 4 | `abs 0x5c470` | `-` |
| `ldr` at 0x5c064 | load | 4 | `abs 0x5c468` | `-` |
| `ldr` at 0x5c068 | load | 4 | `abs 0x11a334` | `-` |
| `str` at 0x5c074 | store | 4 | `abs 0x11a338` | `ret(0x18920)` |
| `ldr` at 0x5c078 | load | 4 | `abs 0x5c474` | `-` |
| `ldr` at 0x5c07c | load | 4 | `abs 0x5c468` | `-` |
| `ldr` at 0x5c080 | load | 4 | `abs 0x11a334` | `-` |
| `str` at 0x5c0a0 | store | 4 | `abs 0x11a33c` | `ret(0x18920)` |
| `ldr` at 0x5c0bc | load | 4 | `sp -0x7c` | `-` |
| `str` at 0x5c0cc | store | 4 | `sp -0x84` | `0x0` |
| `str` at 0x5c0d4 | store | 4 | `sp -0x88` | `0x0` |
| `str` at 0x5c0dc | store | 4 | `sp -0x80` | `0x0` |
| `ldr` at 0x5c0e0 | load | 4 | `abs 0x5c46c` | `-` |
| `ldr` at 0x5c0e4 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x5c0e8 | load | 4 | `abs 0x11a42c` | `-` |
| `ldr` at 0x5c0f8 | load | 4 | `abs 0x5c46c` | `-` |
| `ldr` at 0x5c0fc | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x5c110 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5c118 | load | 4 | `abs 0x5c46c` | `-` |
| `ldr` at 0x5c11c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x5c130 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5c138 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5c13c | load | 4 | `(unknown)` | `-` |
| `str` at 0x5c148 | store | 4 | `sp -0x78` | `ret(0x5e5a8)` |
| `ldr` at 0x5c14c | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x5c158 | load | 4 | `abs 0x5c470` | `-` |
| `ldr` at 0x5c15c | load | 4 | `abs 0x11a338` | `-` |
| `ldr` at 0x5c160 | load | 4 | `sp -0x88` | `-` |
| `ldrb` at 0x5c168 | load | 1 | `sp -0x80` | `-` |
| `strb` at 0x5c16c | store | 1 | `([0x11a338] + [sp-136]) +0x0` | `([sp-128]:1 & 0xff)` |
| `ldr` at 0x5c170 | load | 4 | `abs 0x5c474` | `-` |
| `ldr` at 0x5c174 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x5c188 | load | 4 | `abs 0x11a33c` | `-` |
| `ldr` at 0x5c194 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x5c1a0 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x5c1a8 | store | 4 | `sp -0x88` | `[sp-136]+1` |
| `ldr` at 0x5c1ac | load | 4 | `sp -0x80` | `-` |
| `str` at 0x5c1b4 | store | 4 | `sp -0x80` | `[sp-128]+1` |
| `ldr` at 0x5c1cc | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x5c1e4 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x5c20c | load | 4 | `abs 0x5c470` | `-` |
| `ldr` at 0x5c218 | load | 4 | `abs 0x11a338` | `-` |
| `ldr` at 0x5c21c | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x5c22c | load | 4 | `abs 0x5c474` | `-` |
| `ldr` at 0x5c238 | load | 4 | `abs 0x11a33c` | `-` |
| `ldr` at 0x5c23c | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x5c258 | load | 4 | `abs 0x5c46c` | `-` |
| `ldr` at 0x5c274 | load | 4 | `abs 0x5c478` | `-` |
| `ldr` at 0x5c290 | load | 4 | `abs 0x5c478` | `-` |
| `ldr` at 0x5c2ac | load | 4 | `abs 0x5c478` | `-` |
| `str` at 0x5c2c0 | store | 4 | `sp -0xa4` | `sp-96` |
| `ldr` at 0x5c2c4 | load | 4 | `abs 0x5c47c` | `-` |
| `ldr` at 0x5c2cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5c2d0 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x5c2dc | store | 4 | `sp -0x68` | `ret(0x9bd94)` |
| `ldr` at 0x5c2f4 | load | 4 | `abs 0x5c46c` | `-` |
| `ldr` at 0x5c310 | load | 4 | `abs 0x5c478` | `-` |
| `ldr` at 0x5c31c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5c320 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0x5c324 | store | 4 | `sp -0x20` | `[[sp-24]+28]` |
| `ldr` at 0x5c328 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5c32c | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x5c330 | store | 4 | `sp -0x70` | `[[sp-24]+12]` |
| `ldr` at 0x5c334 | load | 4 | `abs 0x5c480` | `-` |
| `ldr` at 0x5c338 | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x5c348 | load | 4 | `abs 0x5c484` | `-` |
| `ldr` at 0x5c34c | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x5c350 | load | 4 | `abs 0x5c488` | `-` |
| `ldr` at 0x5c354 | load | 4 | `sp -0x70` | `-` |
| `str` at 0x5c364 | store | 4 | `sp -0xa4` | `0x2` |
| `ldr` at 0x5c368 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x5c36c | store | 4 | `sp -0xa0` | `[sp-100]` |
| `str` at 0x5c374 | store | 4 | `sp -0x9c` | `sp-96` |
| `ldr` at 0x5c378 | load | 4 | `abs 0x5c47c` | `-` |
| `ldr` at 0x5c380 | load | 4 | `sp -0x70` | `-` |
| `str` at 0x5c390 | store | 4 | `sp -0x1c` | `ret(0x9c10c)` |
| `ldr` at 0x5c394 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5c3a0 | load | 4 | `abs 0x5c484` | `-` |
| `ldr` at 0x5c3a4 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x5c3a8 | load | 4 | `abs 0x5c48c` | `-` |
| `ldr` at 0x5c3b4 | load | 4 | `abs 0x5c480` | `-` |
| `ldr` at 0x5c3b8 | load | 4 | `abs 0x11a0c8` | `-` |
| `ldr` at 0x5c3c8 | load | 4 | `abs 0x5c484` | `-` |
| `ldr` at 0x5c3cc | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x5c3d0 | load | 4 | `abs 0x5c490` | `-` |
| `ldr` at 0x5c3d4 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x5c3dc | load | 4 | `abs 0x5c494` | `-` |
| `ldr` at 0x5c3e0 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x5c3f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5c400 | load | 4 | `abs 0x5c494` | `-` |
| `ldr` at 0x5c404 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x5c418 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5c424 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5c438 | load | 4 | `abs 0x5c498` | `-` |
| `ldr` at 0x5c43c | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x5c444 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5c44c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5c458 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x5c458 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5c458 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5c458 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5be2c | `(unknown callee) 0x9ca84` | `r0` | `0x0` | `r2` | `?` |
| `bl` at 0x5be50 | `(unknown callee) 0x541d4` | `sp-100` | `sp-96` | `[sp-140]` | `sp-100` |
| `bl` at 0x5bedc | `(unknown callee) 0x189c8` | `0x0` | `?` | `?` | `[sp-116]` |
| `bl` at 0x5bee8 | `(unknown callee) 0x56610` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x5bf2c | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x5bf58 | `(unknown callee) 0x187e8` | `0x11a404` | `[sp-144]` | `?` | `0x0` |
| `bl` at 0x5bf70 | `(unknown callee) 0x18cd4` | `0x11a404` | `[sp-120]` | `?` | `ret(0x187e8)` |
| `bl` at 0x5bf84 | `(unknown callee) 0x94a24` | `[sp-120]` | `0x8000` | `?` | `?` |
| `bl` at 0x5bfa8 | `(unknown callee) 0x5f028` | `ret(0x94a24)` | `?` | `?` | `[sp-124]` |
| `bl` at 0x5bfb0 | `(unknown callee) 0x56518` | `0xe68e4` | `?` | `?` | `?` |
| `bl` at 0x5bff0 | `(unknown callee) 0x9ca84` | `[sp-24]` | `0x2` | `?` | `?` |
| `bl` at 0x5c030 | `(unknown callee) 0x18be4` | `[0x11a338]` | `[0x11a334]` | `0x11a3a0` | `0x11a338` |
| `bl` at 0x5c04c | `(unknown callee) 0x18be4` | `[0x11a33c]` | `?` | `?` | `0x11a33c` |
| `bl` at 0x5c06c | `(unknown callee) 0x18920` | `[0x11a334]` | `?` | `0x11a334` | `0x11a334` |
| `bl` at 0x5c098 | `(unknown callee) 0x18920` | `?` | `?` | `[0x11a334]` | `?` |
| `bl` at 0x5c0b8 | `(unknown callee) 0x541d4` | `sp-100` | `sp-96` | `0x7` | `sp-100` |
| `bl` at 0x5c140 | `(unknown callee) 0x5e5a8` | `?` | `?` | `?` | `?` |
| `bl` at 0x5c19c | `(unknown callee) 0x18ae8` | `?` | `[sp-120]` | `0x70` | `?` |
| `bl` at 0x5c1d0 | `(unknown callee) 0x54230` | `sp-100` | `sp-96` | `[sp-124]` | `sp-100` |
| `bl` at 0x5c1e8 | `(unknown callee) 0x54230` | `sp-100` | `sp-96` | `[sp-136]` | `sp-100` |
| `bl` at 0x5c200 | `(unknown callee) 0x54230` | `sp-100` | `sp-96` | `0x70` | `sp-100` |
| `bl` at 0x5c220 | `(unknown callee) 0x543a8` | `sp-100` | `sp-96` | `[0x11a338]` | `[sp-136]` |
| `bl` at 0x5c240 | `(unknown callee) 0x5b7e4` | `sp-100` | `sp-96` | `[0x11a33c]` | `[sp-136]` |
| `bl` at 0x5c260 | `(unknown callee) 0x5b7e4` | `sp-100` | `sp-96` | `0x11a3a0` | `0x1` |
| `bl` at 0x5c27c | `(unknown callee) 0x5b7e4` | `sp-100` | `sp-96` | `0x11a544` | `0x1` |
| `bl` at 0x5c298 | `(unknown callee) 0x5b7e4` | `sp-100` | `sp-96` | `0x11a544` | `0x1` |
| `bl` at 0x5c2b4 | `(unknown callee) 0x5b7e4` | `sp-100` | `sp-96` | `0x11a544` | `0x1` |
| `bl` at 0x5c2b8 | `(unknown callee) 0x5659c` | `?` | `?` | `?` | `?` |
| `bl` at 0x5c2d4 | `(unknown callee) 0x9bd94` | `0x11a398` | `0x0` | `[sp-24]` | `[sp-100]` |
| `bl` at 0x5c2fc | `(unknown callee) 0x5b7e4` | `sp-100` | `sp-96` | `0x11a3a0` | `0x1` |
| `bl` at 0x5c318 | `(unknown callee) 0x5b7e4` | `sp-100` | `sp-96` | `0x11a544` | `0x1` |
| `bl` at 0x5c358 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe68fc` | `[[sp-24]+12]` | `0x116928` |
| `bl` at 0x5c388 | `(unknown callee) 0x9c10c` | `0x11a398` | `sp-32` | `?` | `0x1` |
| `bl` at 0x5c3ac | `(unknown callee) 0x21010` | `[0x116928]` | `0xe691c` | `?` | `0x116928` |
| `bl` at 0x5c3d8 | `(unknown callee) 0x21010` | `[0x116928]` | `0xe6954` | `[sp-112]` | `0x116928` |
| `bl` at 0x5c428 | `(unknown callee) 0x9ba98` | `?` | `?` | `0x50` | `?` |
| `bl` at 0x5c440 | `(unknown callee) 0x1dba4` | `0xe6978` | `[sp-112]` | `?` | `ret(0x9ba98)` |
| `bl` at 0x5c448 | `(unknown callee) 0x9a998` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x5c454 | `(unknown callee) 0x58e7c` | `[sp-24]` | `0x1` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5be10 | `? != 0x0` | `? == 0x0` |
| `bgt` at 0x5be64 | `[sp-116] > 0xf (signed, 32-bit)` | `[sp-116] <= 0xf (signed, 32-bit)` |
| `bge` at 0x5be70 | `[sp-116] >= 0xc (signed, 32-bit)` | `[sp-116] < 0xc (signed, 32-bit)` |
| `beq` at 0x5be7c | `[sp-116] == 0x6` | `[sp-116] != 0x6` |
| `bgt` at 0x5be88 | `[sp-116] > 0x6 (signed, 32-bit)` | `[sp-116] <= 0x6 (signed, 32-bit)` |
| `blt` at 0x5be94 | `[sp-116] < 0x1 (signed, 32-bit)` | `[sp-116] >= 0x1 (signed, 32-bit)` |
| `beq` at 0x5bea4 | `[sp-116] == 0x9` | `[sp-116] != 0x9` |
| `blt` at 0x5beb4 | `[sp-116] < 0x14 (signed, 32-bit)` | `[sp-116] >= 0x14 (signed, 32-bit)` |
| `ble` at 0x5bec0 | `[sp-116] <= 0x19 (signed, 32-bit)` | `[sp-116] > 0x19 (signed, 32-bit)` |
| `beq` at 0x5becc | `[sp-116] == 0x3e8` | `[sp-116] != 0x3e8` |
| `bls` at 0x5befc | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` |
| `bne` at 0x5bf10 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x5bf44 | `[sp-144] == 0x0` | `[sp-144] != 0x0` |
| `beq` at 0x5bf64 | `ret(0x187e8) == 0x0` | `ret(0x187e8) != 0x0` |
| `beq` at 0x5bf90 | `ret(0x94a24) == 0x0` | `ret(0x94a24) != 0x0` |
| `beq` at 0x5bfa4 | `[sp-124] == 0x0` | `[sp-124] != 0x0` |
| `bls` at 0x5bfc0 | `[[sp-24]+32]:1 <= 0x2 (unsigned, 32-bit)` | `[[sp-24]+32]:1 > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x5bfd4 | `? != 0x0` | `? == 0x0` |
| `bge` at 0x5c014 | `[0x11a334] >= [0x11a42c] (signed, 32-bit)` | `[0x11a334] < [0x11a42c] (signed, 32-bit)` |
| `beq` at 0x5c024 | `[0x11a338] == 0x0` | `[0x11a338] != 0x0` |
| `beq` at 0x5c040 | `[0x11a33c] == 0x0` | `[0x11a33c] != 0x0` |
| `bne` at 0x5c0c4 | `[sp-124] != 0x0` | `[sp-124] == 0x0` |
| `blt` at 0x5c0f0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x5c154 | `ret(0x5e5a8) == 0x0` | `ret(0x5e5a8) != 0x0` |
| `beq` at 0x5c344 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x5c39c | `ret(0x9c10c) != 0x0` | `ret(0x9c10c) == 0x0` |
| `beq` at 0x5c3c4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x5c3fc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x5c434 | `ret(0x9ba98) == 0x0` | `ret(0x9ba98) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5c458 | `r0` | `?` |

## `L1CfgProcessEvent`

- `L1CfgProcessEvent`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5c56c to 0x5c778.
  Established. A nonzero byte `[r0+19]:1` takes the first branch directly to the return, while
  zero falls through (`bne` at 0x5c58c). The halfword at entry `r0 +0x22` is loaded for subsequent
  comparisons (`ldrh` at 0x5c594). Those comparisons include the equality branch for `0x6` (`beq` at
  0x5c5a4) and the equality branch for `0x3e9` (`beq` at 0x5c650). Their selected paths call
  `0x5f94c` (`bl` at 0x5c698) and `0x5b5f8` (`bl` at 0x5c768), respectively. The event meanings and
  device actions are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5c57c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x5c580 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x5c584 | load | 1 | `r0 +0x13` | `-` |
| `ldr` at 0x5c590 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x5c594 | load | 2 | `r0 +0x22` | `-` |
| `str` at 0x5c598 | store | 4 | `sp -0x8c` | `[r0+34]:2` |
| `ldr` at 0x5c59c | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x5c5a8 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x5c5b4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x5c5c0 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x5c5cc | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x5c5d8 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x5c5e8 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x5c5f4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x5c604 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x5c610 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x5c61c | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x5c628 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x5c638 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x5c644 | load | 4 | `abs 0x5c770` | `-` |
| `ldr` at 0x5c648 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x5c658 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5c664 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5c670 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5c67c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5c688 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5c694 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5c6a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5c6ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5c6b8 | load | 4 | `abs 0x5c774` | `-` |
| `ldr` at 0x5c6bc | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x5c6c8 | load | 4 | `abs 0x5c774` | `-` |
| `ldr` at 0x5c6cc | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x5c6d8 | load | 4 | `abs 0x5c774` | `-` |
| `ldr` at 0x5c6dc | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x5c6e8 | load | 4 | `abs 0x5c774` | `-` |
| `ldr` at 0x5c6ec | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x5c6f8 | load | 4 | `abs 0x5c774` | `-` |
| `ldr` at 0x5c6fc | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x5c708 | load | 4 | `abs 0x5c774` | `-` |
| `ldr` at 0x5c70c | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x5c718 | load | 4 | `abs 0x5c774` | `-` |
| `ldr` at 0x5c71c | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x5c728 | load | 4 | `abs 0x5c774` | `-` |
| `ldr` at 0x5c72c | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x5c738 | load | 4 | `abs 0x5c774` | `-` |
| `ldr` at 0x5c73c | load | 4 | `abs 0x11a3b0` | `-` |
| `ldr` at 0x5c74c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5c758 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5c764 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x5c76c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5c76c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5c76c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5c65c | `(unknown callee) 0x5f3c8` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x5c668 | `(unknown callee) 0x5f128` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x5c674 | `(unknown callee) 0x5f5cc` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x5c680 | `(unknown callee) 0x5f73c` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x5c68c | `(unknown callee) 0x5f844` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x5c698 | `(unknown callee) 0x5f94c` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x5c6a4 | `(unknown callee) 0x5fa54` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x5c6b0 | `(unknown callee) 0x5fb64` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x5c750 | `(unknown callee) 0x5fc6c` | `r0` | `r1` | `[r0+34]:2` | `[0x11a3b0]` |
| `bl` at 0x5c75c | `(unknown callee) 0x602e8` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x5c768 | `(unknown callee) 0x5b5f8` | `r0` | `r1` | `[r0+34]:2` | `0x3e9` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5c58c | `[r0+19]:1 != 0x0` | `[r0+19]:1 == 0x0` |
| `beq` at 0x5c5a4 | `[r0+34]:2 == 0x6` | `[r0+34]:2 != 0x6` |
| `bgt` at 0x5c5b0 | `[r0+34]:2 > 0x6 (signed, 32-bit)` | `[r0+34]:2 <= 0x6 (signed, 32-bit)` |
| `beq` at 0x5c5bc | `[r0+34]:2 == 0x3` | `[r0+34]:2 != 0x3` |
| `bgt` at 0x5c5c8 | `[r0+34]:2 > 0x3 (signed, 32-bit)` | `[r0+34]:2 <= 0x3 (signed, 32-bit)` |
| `beq` at 0x5c5d4 | `[r0+34]:2 == 0x1` | `[r0+34]:2 != 0x1` |
| `beq` at 0x5c5e0 | `[r0+34]:2 == 0x2` | `[r0+34]:2 != 0x2` |
| `beq` at 0x5c5f0 | `[r0+34]:2 == 0x4` | `[r0+34]:2 != 0x4` |
| `beq` at 0x5c5fc | `[r0+34]:2 == 0x5` | `[r0+34]:2 != 0x5` |
| `beq` at 0x5c60c | `[r0+34]:2 == 0x64` | `[r0+34]:2 != 0x64` |
| `bgt` at 0x5c618 | `[r0+34]:2 > 0x64 (signed, 32-bit)` | `[r0+34]:2 <= 0x64 (signed, 32-bit)` |
| `beq` at 0x5c624 | `[r0+34]:2 == 0x7` | `[r0+34]:2 != 0x7` |
| `beq` at 0x5c630 | `[r0+34]:2 == 0x8` | `[r0+34]:2 != 0x8` |
| `beq` at 0x5c640 | `[r0+34]:2 == 0x66` | `[r0+34]:2 != 0x66` |
| `beq` at 0x5c650 | `[r0+34]:2 == 0x3e9` | `[r0+34]:2 != 0x3e9` |
| `beq` at 0x5c6c4 | `[0x11a3b0] == 0x35` | `[0x11a3b0] != 0x35` |
| `beq` at 0x5c6d4 | `[0x11a3b0] == 0x4f` | `[0x11a3b0] != 0x4f` |
| `beq` at 0x5c6e4 | `[0x11a3b0] == 0x42` | `[0x11a3b0] != 0x42` |
| `beq` at 0x5c6f4 | `[0x11a3b0] == 0x46` | `[0x11a3b0] != 0x46` |
| `beq` at 0x5c704 | `[0x11a3b0] == 0x4a` | `[0x11a3b0] != 0x4a` |
| `beq` at 0x5c714 | `[0x11a3b0] == 0x4b` | `[0x11a3b0] != 0x4b` |
| `beq` at 0x5c724 | `[0x11a3b0] == 0x21` | `[0x11a3b0] != 0x21` |
| `beq` at 0x5c734 | `[0x11a3b0] == 0x5e` | `[0x11a3b0] != 0x5e` |
| `beq` at 0x5c744 | `[0x11a3b0] == 0x4c` | `[0x11a3b0] != 0x4c` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5c76c | `r0` | `?` |

## `L1CfgIRouter`

- `L1CfgIRouter`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5c778 to 0x5c834.
  Established. The first branch falls through when `[r0+22]:1 == 0x2` and is taken otherwise (`bne`
  at 0x5c7a0). A later branch splits on the signed word `[r0+28] >= 0`, following the nonnegative arm
  (`bge` at 0x5c7b0). Other paths test the entry byte against zero (`bne` at 0x5c800) and one (`bne`
  at 0x5c81c). The final call uses `[sp-20]` as its first candidate (`bl` at 0x5c82c), and its
  result reaches the return (`ldmdb` at 0x5c830). The byte values do not establish protocol message
  types.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5c788 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x5c790 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x5c794 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x5c798 | load | 1 | `r0 +0x16` | `-` |
| `ldr` at 0x5c7a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5c7a8 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x5c7b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5c7b8 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x5c7d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5c7d4 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x5c7e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5c7f4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x5c7f8 | load | 1 | `r0 +0x16` | `-` |
| `ldr` at 0x5c804 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5c810 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x5c814 | load | 1 | `r0 +0x16` | `-` |
| `ldr` at 0x5c820 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5c828 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x5c830 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5c830 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5c830 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5c7ec | `(unknown callee) 0x5c56c` | `r0` | `r1` | `r2` | `?` |
| `bl` at 0x5c808 | `(unknown callee) 0x5bde4` | `r0` | `r1` | `r2` | `[r0+22]:1` |
| `bl` at 0x5c824 | `(unknown callee) 0x5c49c` | `r0` | `r1` | `r2` | `[r0+22]:1` |
| `bl` at 0x5c82c | `(unknown callee) 0x9a998` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5c7a0 | `[r0+22]:1 != 0x2` | `[r0+22]:1 == 0x2` |
| `bge` at 0x5c7b0 | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `beq` at 0x5c7c8 | `? == 0xe` | `? != 0xe` |
| `beq` at 0x5c7e0 | `? == 0xe` | `? != 0xe` |
| `bne` at 0x5c800 | `[r0+22]:1 != 0x0` | `[r0+22]:1 == 0x0` |
| `bne` at 0x5c81c | `[r0+22]:1 != 0x1` | `[r0+22]:1 == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5c830 | `r0` | `ret(0x9a998)` |

## `L1CfgGetBrickType`

- `L1CfgGetBrickType`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5e520 to 0x5e5a8.
  Established. The call to `0x59b44` has candidates `[0x11a434]`, `[0x11a42c]`, `[sp-20]` and `[sp-24]`
  (`bl` at 0x5e558); its zero result takes the branch (`beq` at 0x5e56c) to a stack word store of
  `0x3f` at `sp -0x24` (`str` at 0x5e584). The nonzero fall-through loads a word at `ret(0x59b44)
  +0x8` (`ldr` at 0x5e574). The merged return is `[sp-32]` (`ldmdb` at 0x5e59c). The type encoding
  represented by these values is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5e530 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x5e534 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x5e538 | load | 4 | `abs 0x5e5a0` | `-` |
| `ldr` at 0x5e540 | load | 4 | `abs 0x5e5a4` | `-` |
| `ldr` at 0x5e544 | load | 4 | `abs 0x5e5a4` | `-` |
| `ldr` at 0x5e548 | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5e54c | load | 4 | `abs 0x11a42c` | `-` |
| `ldr` at 0x5e550 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5e554 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5e560 | store | 4 | `sp -0x1c` | `ret(0x59b44)` |
| `ldr` at 0x5e564 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5e570 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5e574 | load | 4 | `ret(0x59b44) +0x8` | `-` |
| `str` at 0x5e578 | store | 4 | `sp -0x24` | `[ret(0x59b44)+8]` |
| `str` at 0x5e584 | store | 4 | `sp -0x24` | `0x3f` |
| `ldr` at 0x5e588 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x5e58c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x5e594 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x5e59c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5e59c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5e59c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5e53c | `(unknown callee) 0x56518` | `0xe6cf8` | `r1` | `r2` | `r3` |
| `bl` at 0x5e558 | `(unknown callee) 0x59b44` | `[0x11a434]` | `[0x11a42c]` | `[sp-20]` | `[sp-24]` |
| `bl` at 0x5e590 | `(unknown callee) 0x5659c` | `ret(0x59b44)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5e56c | `ret(0x59b44) == 0x0` | `ret(0x59b44) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5e59c | `r0` | `[sp-32]` |

## `L1CfgGetBrickCfg`

- `L1CfgGetBrickCfg`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5e5a8 to 0x5e5f8.
  Established. A single call uses candidates `[0x11a440]`, `[0x11a438]`, entry `r0` and entry `r1`
  (`bl` at 0x5e5e0); its result replaces the earlier zero in the stack word at `sp -0x1c` (`str` at
  0x5e5e4). The exit reports `ret(0x59c60)` in `r0` (`ldmdb` at 0x5e5f0), and no conditional branch
  records appear. The referenced configuration layout remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5e5b8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x5e5bc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x5e5c4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x5e5c8 | load | 4 | `abs 0x5e5f4` | `-` |
| `ldr` at 0x5e5cc | load | 4 | `abs 0x5e5f4` | `-` |
| `ldr` at 0x5e5d0 | load | 4 | `abs 0x11a440` | `-` |
| `ldr` at 0x5e5d4 | load | 4 | `abs 0x11a438` | `-` |
| `ldr` at 0x5e5d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5e5dc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5e5e4 | store | 4 | `sp -0x1c` | `ret(0x59c60)` |
| `ldr` at 0x5e5e8 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x5e5f0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5e5f0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5e5f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5e5e0 | `(unknown callee) 0x59c60` | `[0x11a440]` | `[0x11a438]` | `r0` | `r1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5e5f0 | `r0` | `ret(0x59c60)` |

## `L1CfgGetBrickIndex`

- `L1CfgGetBrickIndex`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5e5f8 to 0x5e6f0.
  Established. The two stack initializers clear words at `sp -0x20` (`str` at 0x5e614) and `sp
  -0x1c` (`str` at 0x5e61c). A later word store sets `sp -0x1c` to one (`str` at 0x5e6a8). The loop
  test has an `lt` flags condition with unknown operands (`blt` at 0x5e63c), and the final split has
  only the predicate `Z == 1` (`beq` at 0x5e6c4). The return is `[sp-36]` (`ldmdb` at 0x5e6e8). No
  call is recorded, and the relation of that result to a physical brick remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5e608 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x5e60c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x5e614 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x5e61c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x5e620 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5e62c | load | 4 | `abs 0x5e6ec` | `-` |
| `ldr` at 0x5e630 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5e634 | load | 4 | `abs 0x11a42c` | `-` |
| `ldr` at 0x5e644 | load | 4 | `abs 0x5e6ec` | `-` |
| `ldr` at 0x5e648 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5e65c | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5e664 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5e668 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5e674 | load | 4 | `abs 0x5e6ec` | `-` |
| `ldr` at 0x5e678 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5e68c | load | 4 | `abs 0x11a434` | `-` |
| `ldr` at 0x5e694 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5e698 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5e6a8 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x5e6ac | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5e6b4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x5e6bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5e6c8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5e6d0 | store | 4 | `sp -0x24` | `?` |
| `str` at 0x5e6dc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x5e6e0 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x5e6e8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5e6e8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5e6e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5e628 | `Z == 0` | `not (Z == 0)` |
| `blt` at 0x5e63c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x5e670 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x5e6a0 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x5e6c4 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5e6e8 | `r0` | `[sp-36]` |
