# ARM log, rack and event entry paths in 1.34.2

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
No unresolved computed transfers were reported for these routines.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/log-rack-events-facts.txt) retain all tool sections.

## `nvramLogInsert`

- `nvramLogInsert`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1d6d8 to 0x1dba4.
  Established. The initial test takes the branch when `[0x11694c] != 0` and falls through
  when zero (`bne` at 0x1d730). Byte writes through `[[0x11694c]+1936] +0x0` include `([[sp-40]]:1 &
  0xff)` (`strb` at 0x1d974), `([[sp-28]]:1 & 0xff)` (`strb` at 0x1da04) and `0xa` (`strb` at
  0x1da78). A byte at `[0x11694c] +0x7a3` receives `(ret(0x1eddc) & 0xff)` (`strb` at 0x1dae0). The
  call to `0x18ae8` has candidates `[0x11694c]+1956`, `[0x11694c]+1920`, `0x24` and
  `[0x11694c]+1920` (`bl` at 0x1db0c). The underlying storage device and persistence guarantee
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1d6e8 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x1d6f8 | store | 4 | `sp -0x24` | `ret(0x189c8)` |
| `str` at 0x1d700 | store | 4 | `sp -0x28` | `sp-36` |
| `str` at 0x1d708 | store | 4 | `sp -0x2c` | `0x5` |
| `ldr` at 0x1d70c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x1d718 | store | 4 | `sp -0x30` | `ret(0x18a1c)` |
| `strb` at 0x1d720 | store | 1 | `sp -0x31` | `0x0` |
| `ldr` at 0x1d724 | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1d728 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1d738 | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1d73c | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1d740 | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1d744 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1d748 | load | 4 | `[0x11694c] +0x788` | `-` |
| `ldr` at 0x1d74c | load | 4 | `[0x11694c] +0x784` | `-` |
| `str` at 0x1d754 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x1d75c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1d760 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x1d76c | load | 1 | `(unknown)` | `-` |
| `strb` at 0x1d77c | store | 1 | `sp -0x31` | `0x1` |
| `ldr` at 0x1d780 | load | 4 | `abs 0x1db8c` | `-` |
| `ldr` at 0x1d798 | load | 4 | `abs 0x1db90` | `-` |
| `ldr` at 0x1d79c | load | 4 | `abs 0x116e38` | `-` |
| `ldr` at 0x1d7ac | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1d7b0 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1d7b4 | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1d7b8 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1d7bc | load | 4 | `[0x11694c] +0x790` | `-` |
| `ldr` at 0x1d7c0 | load | 4 | `[0x11694c] +0x78c` | `-` |
| `ldr` at 0x1d7cc | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1d7d0 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1d7d4 | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1d7d8 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1d7dc | load | 4 | `[0x11694c] +0x78c` | `-` |
| `ldr` at 0x1d7e0 | load | 4 | `[0x11694c] +0x784` | `-` |
| `ldr` at 0x1d7e8 | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1d7ec | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1d7f0 | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1d7f4 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1d7f8 | load | 4 | `[0x11694c] +0x788` | `-` |
| `ldr` at 0x1d7fc | load | 4 | `[0x11694c] +0x790` | `-` |
| `str` at 0x1d808 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x1d810 | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1d814 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1d818 | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1d81c | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1d820 | load | 4 | `[0x11694c] +0x790` | `-` |
| `ldr` at 0x1d824 | load | 4 | `[0x11694c] +0x78c` | `-` |
| `ldr` at 0x1d830 | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1d834 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1d838 | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1d83c | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1d840 | load | 4 | `[0x11694c] +0x78c` | `-` |
| `ldr` at 0x1d844 | load | 4 | `[0x11694c] +0x790` | `-` |
| `str` at 0x1d84c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x1d854 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x1d858 | load | 4 | `sp -0x2c` | `-` |
| `ldrsb` at 0x1d860 | load | 1 | `sp -0x31` | `-` |
| `ldr` at 0x1d868 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x1d878 | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1d87c | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1d888 | load | 4 | `[0x11694c] +0x78c` | `-` |
| `ldrb` at 0x1d88c | load | 1 | `[[0x11694c]+1932] +0x0` | `-` |
| `str` at 0x1d894 | store | 4 | `[0x11694c] +0x78c` | `[[0x11694c]+1932]+1` |
| `ldr` at 0x1d8a8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x1d8b0 | store | 4 | `sp -0x38` | `[sp-56]+1` |
| `ldr` at 0x1d8b4 | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1d8b8 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1d8bc | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1d8c0 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1d8c4 | load | 4 | `[0x11694c] +0x78c` | `-` |
| `ldr` at 0x1d8c8 | load | 4 | `[0x11694c] +0x788` | `-` |
| `ldr` at 0x1d8d4 | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1d8d8 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1d8dc | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1d8e0 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1d8e4 | load | 4 | `[0x11694c] +0x784` | `-` |
| `str` at 0x1d8e8 | store | 4 | `[0x11694c] +0x78c` | `[[0x11694c]+1924]` |
| `ldr` at 0x1d8f0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x1d8f8 | store | 4 | `sp -0x38` | `[sp-56]+1` |
| `ldr` at 0x1d904 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x1d90c | store | 4 | `sp -0x2c` | `[sp-44]-1` |
| `ldr` at 0x1d91c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1d928 | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1d92c | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1d934 | load | 4 | `[0x11694c] +0x790` | `-` |
| `strb` at 0x1d940 | store | 1 | `[[0x11694c]+1936] +0x0` | `?` |
| `str` at 0x1d948 | store | 4 | `[0x11694c] +0x790` | `[[0x11694c]+1936]+1` |
| `ldr` at 0x1d950 | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1d954 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1d95c | load | 4 | `[0x11694c] +0x790` | `-` |
| `ldr` at 0x1d964 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x1d970 | load | 1 | `[sp-40] +0x0` | `-` |
| `strb` at 0x1d974 | store | 1 | `[[0x11694c]+1936] +0x0` | `([[sp-40]]:1 & 0xff)` |
| `str` at 0x1d97c | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `str` at 0x1d984 | store | 4 | `[0x11694c] +0x790` | `[[0x11694c]+1936]+1` |
| `ldr` at 0x1d988 | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1d98c | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1d990 | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1d994 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1d998 | load | 4 | `[0x11694c] +0x790` | `-` |
| `ldr` at 0x1d99c | load | 4 | `[0x11694c] +0x788` | `-` |
| `ldr` at 0x1d9a8 | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1d9ac | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1d9b0 | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1d9b4 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1d9b8 | load | 4 | `[0x11694c] +0x784` | `-` |
| `str` at 0x1d9bc | store | 4 | `[0x11694c] +0x790` | `[[0x11694c]+1924]` |
| `ldr` at 0x1d9c8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x1d9d0 | store | 4 | `sp -0x30` | `[sp-48]-1` |
| `ldr` at 0x1d9e0 | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1d9e4 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1d9ec | load | 4 | `[0x11694c] +0x790` | `-` |
| `ldr` at 0x1d9f4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x1da00 | load | 1 | `[sp-28] +0x0` | `-` |
| `strb` at 0x1da04 | store | 1 | `[[0x11694c]+1936] +0x0` | `([[sp-28]]:1 & 0xff)` |
| `str` at 0x1da0c | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x1da14 | store | 4 | `[0x11694c] +0x790` | `[[0x11694c]+1936]+1` |
| `ldr` at 0x1da18 | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1da1c | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1da20 | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1da24 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1da28 | load | 4 | `[0x11694c] +0x790` | `-` |
| `ldr` at 0x1da2c | load | 4 | `[0x11694c] +0x788` | `-` |
| `ldr` at 0x1da38 | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1da3c | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1da40 | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1da44 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1da48 | load | 4 | `[0x11694c] +0x784` | `-` |
| `str` at 0x1da4c | store | 4 | `[0x11694c] +0x790` | `[[0x11694c]+1924]` |
| `ldrsb` at 0x1da54 | load | 1 | `sp -0x31` | `-` |
| `ldr` at 0x1da60 | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1da64 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1da6c | load | 4 | `[0x11694c] +0x790` | `-` |
| `strb` at 0x1da78 | store | 1 | `[[0x11694c]+1936] +0x0` | `0xa` |
| `str` at 0x1da80 | store | 4 | `[0x11694c] +0x790` | `[[0x11694c]+1936]+1` |
| `ldr` at 0x1da84 | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1da88 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1da8c | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1da90 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1da94 | load | 4 | `[0x11694c] +0x790` | `-` |
| `ldr` at 0x1da98 | load | 4 | `[0x11694c] +0x788` | `-` |
| `ldr` at 0x1daa4 | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1daa8 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1daac | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1dab0 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1dab4 | load | 4 | `[0x11694c] +0x784` | `-` |
| `str` at 0x1dab8 | store | 4 | `[0x11694c] +0x790` | `[[0x11694c]+1924]` |
| `ldr` at 0x1dabc | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1dac0 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1dac4 | load | 4 | `abs 0x1db94` | `-` |
| `ldr` at 0x1dac8 | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1dacc | load | 4 | `abs 0x11694c` | `-` |
| `strb` at 0x1dae0 | store | 1 | `[0x11694c] +0x7a3` | `(ret(0x1eddc) & 0xff)` |
| `ldr` at 0x1dae4 | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1dae8 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1daf4 | load | 4 | `abs 0x1db88` | `-` |
| `ldr` at 0x1daf8 | load | 4 | `abs 0x11694c` | `-` |
| `str` at 0x1db18 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0x1db1c | load | 4 | `abs 0x1db8c` | `-` |
| `ldr` at 0x1db30 | load | 4 | `abs 0x1db90` | `-` |
| `ldr` at 0x1db34 | load | 4 | `abs 0x116e38` | `-` |
| `str` at 0x1db44 | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0x1db48 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x1db4c | store | 4 | `sp -0x20` | `[sp-60]` |
| `ldr` at 0x1db58 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x1db64 | load | 4 | `abs 0x1db98` | `-` |
| `str` at 0x1db68 | store | 4 | `sp -0x44` | `0xd6f5c` |
| `str` at 0x1db6c | store | 4 | `sp -0x40` | `ret(0x1e71c)` |
| `ldr` at 0x1db74 | load | 4 | `abs 0x1db9c` | `-` |
| `ldr` at 0x1db78 | load | 4 | `abs 0x1dba0` | `-` |
| `ldmdb` at 0x1db84 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x1db84 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x1db84 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x1db84 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x1db84 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1d6f0 | `(unknown callee) 0x189c8` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x1d710 | `(unknown callee) 0x18a1c` | `[sp-28]` | `?` | `?` | `0x5` |
| `bl` at 0x1d788 | `(unknown callee) 0x1fb00` | `0x11695c` | `?` | `[sp-28]` | `?` |
| `bl` at 0x1dadc | `(unknown callee) 0x1eddc` | `[0x11694c]+1920` | `0x24` | `?` | `[0x11694c]+1920` |
| `bl` at 0x1db0c | `(unknown callee) 0x18ae8` | `[0x11694c]+1956` | `[0x11694c]+1920` | `0x24` | `[0x11694c]+1920` |
| `bl` at 0x1db10 | `(unknown callee) 0x19390` | `ret(0x18ae8)` | `?` | `?` | `?` |
| `bl` at 0x1db20 | `(unknown callee) 0x1fe64` | `0x11695c` | `?` | `?` | `0x0` |
| `bl` at 0x1db5c | `(unknown callee) 0x1e71c` | `[sp-60]` | `?` | `?` | `[sp-60]` |
| `bl` at 0x1db80 | `(unknown callee) 0x1eed0` | `0x9` | `0xd6c64` | `0x743` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x1d730 | `[0x11694c] != 0x0` | `[0x11694c] == 0x0` |
| `beq` at 0x1d774 | `? == 0xa` | `? != 0xa` |
| `beq` at 0x1d794 | `ret(0x1fb00) == 0x0` | `ret(0x1fb00) != 0x0` |
| `beq` at 0x1d7a4 | `[0x116e38] == 0x1` | `[0x116e38] != 0x1` |
| `bls` at 0x1d7c8 | `[[0x11694c]+1936] <= [[0x11694c]+1932] (unsigned, 32-bit)` | `[[0x11694c]+1936] > [[0x11694c]+1932] (unsigned, 32-bit)` |
| `bhs` at 0x1d82c | `[[0x11694c]+1936] >= [[0x11694c]+1932] (unsigned, 32-bit)` | `[[0x11694c]+1936] < [[0x11694c]+1932] (unsigned, 32-bit)` |
| `bge` at 0x1d870 | `? >= [sp-56] (signed, 32-bit)` | `? < [sp-56] (signed, 32-bit)` |
| `bne` at 0x1d8a0 | `? != 0xa` | `? == 0xa` |
| `blo` at 0x1d8d0 | `[[0x11694c]+1932] < [[0x11694c]+1928] (unsigned, 32-bit)` | `[[0x11694c]+1932] >= [[0x11694c]+1928] (unsigned, 32-bit)` |
| `bne` at 0x1d914 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x1d924 | `[sp-44] != 0x4` | `[sp-44] == 0x4` |
| `blo` at 0x1d9a4 | `[[0x11694c]+1936] < [[0x11694c]+1928] (unsigned, 32-bit)` | `[[0x11694c]+1936] >= [[0x11694c]+1928] (unsigned, 32-bit)` |
| `bne` at 0x1d9d8 | `Z == 0` | `not (Z == 0)` |
| `blo` at 0x1da34 | `[[0x11694c]+1936] < [[0x11694c]+1928] (unsigned, 32-bit)` | `[[0x11694c]+1936] >= [[0x11694c]+1928] (unsigned, 32-bit)` |
| `beq` at 0x1da5c | `? == 0x0` | `? != 0x0` |
| `blo` at 0x1daa0 | `[[0x11694c]+1936] < [[0x11694c]+1928] (unsigned, 32-bit)` | `[[0x11694c]+1936] >= [[0x11694c]+1928] (unsigned, 32-bit)` |
| `beq` at 0x1db2c | `ret(0x1fe64) == 0x0` | `ret(0x1fe64) != 0x0` |
| `beq` at 0x1db3c | `[0x116e38] == 0x1` | `[0x116e38] != 0x1` |
| `beq` at 0x1db54 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x1db84 | `r0` | `?` |

## `nvramLogFetch`

- `nvramLogFetch`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1dbe8 to 0x1dfd0.
  Established. The first comparison takes the branch for unsigned `[[0x11694c]+1936] <=
  [[0x11694c]+1932]` and falls through when greater (`bls` at 0x1dc1c). A call to `0x18920` has
  first candidate `0x1` (`bl` at 0x1dcbc); its nonzero result takes the branch (`bne` at 0x1dcd0) to
  a byte clear at `ret(0x18920) +0x0` (`strb` at 0x1dce8). Later byte stores have unknown effective
  targets, including an unknown value (`strb` at 0x1df50) and zero (`strb` at 0x1dfb4). The returned
  value is unknown (`ldmdb` at 0x1dfc4). The log record format is not established by these facts.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1dbfc | store | 4 | `sp -0x44` | `0x1` |
| `ldr` at 0x1dc00 | load | 4 | `abs 0x1dfc8` | `-` |
| `ldr` at 0x1dc04 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1dc08 | load | 4 | `abs 0x1dfc8` | `-` |
| `ldr` at 0x1dc0c | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1dc10 | load | 4 | `[0x11694c] +0x790` | `-` |
| `ldr` at 0x1dc14 | load | 4 | `[0x11694c] +0x78c` | `-` |
| `ldr` at 0x1dc20 | load | 4 | `abs 0x1dfc8` | `-` |
| `ldr` at 0x1dc24 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1dc28 | load | 4 | `[0x11694c] +0x78c` | `-` |
| `str` at 0x1dc2c | store | 4 | `sp -0x18` | `[[0x11694c]+1932]` |
| `ldr` at 0x1dc30 | load | 4 | `abs 0x1dfc8` | `-` |
| `ldr` at 0x1dc34 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1dc38 | load | 4 | `[0x11694c] +0x790` | `-` |
| `str` at 0x1dc3c | store | 4 | `sp -0x1c` | `[[0x11694c]+1936]` |
| `str` at 0x1dc44 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x1dc4c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x1dc54 | load | 4 | `abs 0x1dfc8` | `-` |
| `ldr` at 0x1dc58 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1dc5c | load | 4 | `abs 0x1dfc8` | `-` |
| `ldr` at 0x1dc60 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1dc64 | load | 4 | `[0x11694c] +0x790` | `-` |
| `ldr` at 0x1dc68 | load | 4 | `[0x11694c] +0x78c` | `-` |
| `ldr` at 0x1dc74 | load | 4 | `abs 0x1dfc8` | `-` |
| `ldr` at 0x1dc78 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1dc7c | load | 4 | `[0x11694c] +0x78c` | `-` |
| `str` at 0x1dc80 | store | 4 | `sp -0x18` | `[[0x11694c]+1932]` |
| `ldr` at 0x1dc84 | load | 4 | `abs 0x1dfc8` | `-` |
| `ldr` at 0x1dc88 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1dc8c | load | 4 | `[0x11694c] +0x788` | `-` |
| `str` at 0x1dc90 | store | 4 | `sp -0x1c` | `[[0x11694c]+1928]` |
| `ldr` at 0x1dc94 | load | 4 | `abs 0x1dfc8` | `-` |
| `ldr` at 0x1dc98 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1dc9c | load | 4 | `[0x11694c] +0x784` | `-` |
| `str` at 0x1dca0 | store | 4 | `sp -0x20` | `[[0x11694c]+1924]` |
| `ldr` at 0x1dca4 | load | 4 | `abs 0x1dfc8` | `-` |
| `ldr` at 0x1dca8 | load | 4 | `abs 0x11694c` | `-` |
| `ldr` at 0x1dcac | load | 4 | `[0x11694c] +0x790` | `-` |
| `str` at 0x1dcb0 | store | 4 | `sp -0x24` | `[[0x11694c]+1936]` |
| `str` at 0x1dcc4 | store | 4 | `sp -0x14` | `ret(0x18920)` |
| `ldr` at 0x1dcc8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1dcd8 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x1dce0 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x1dce8 | store | 1 | `ret(0x18920) +0x0` | `0x0` |
| `ldr` at 0x1dcec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1dcf0 | store | 4 | `sp -0x48` | `[sp-20]` |
| `str` at 0x1dcfc | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x1dd00 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x1dd04 | store | 4 | `sp -0x2c` | `[[0x11694c]+1932]` |
| `ldr` at 0x1dd08 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x1dd0c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x1dd10 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1dd20 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x1dd24 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x1dd28 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x1dd2c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x1dd3c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x1dd48 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x1dd50 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x1dd58 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x1dd5c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x1dd68 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x1dd70 | store | 4 | `sp -0x44` | `?` |
| `str` at 0x1dd78 | store | 4 | `sp -0x34` | `0x4` |
| `ldr` at 0x1dd80 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x1dd88 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x1dd8c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x1dd94 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x1dd9c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1dda0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x1ddb0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x1ddb4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x1ddb8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1ddbc | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x1ddc0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x1ddc4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x1ddc8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1ddcc | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x1ddd4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x1dde0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x1dde8 | store | 4 | `sp -0x4c` | `?` |
| `str` at 0x1ddf4 | store | 4 | `sp -0x4c` | `0x64` |
| `ldr` at 0x1ddf8 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x1ddfc | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x1de00 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x1de0c | store | 4 | `sp -0x14` | `ret(0x18920)` |
| `ldr` at 0x1de10 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1de20 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0x1de2c | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0x1de34 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x1de38 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x1de3c | store | 4 | `sp -0x2c` | `[sp-24]` |
| `ldr` at 0x1de40 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x1de44 | store | 4 | `sp -0x30` | `[sp-28]` |
| `ldr` at 0x1de48 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1de58 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x1de5c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x1de60 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x1de64 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x1de74 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x1de84 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x1de88 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x1de90 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x1de94 | store | 1 | `[sp-60] +0x0` | `?` |
| `str` at 0x1de9c | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldr` at 0x1dea0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x1dea8 | store | 4 | `sp -0x34` | `[sp-52]-1` |
| `ldr` at 0x1dec4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1dec8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x1ded8 | load | 4 | `abs 0x1dfcc` | `-` |
| `ldr` at 0x1dee4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1dee8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x1df00 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x1df08 | store | 4 | `sp -0x44` | `([sp-68] + ret(0x18a1c))` |
| `ldr` at 0x1df10 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x1df14 | load | 1 | `(unknown)` | `-` |
| `str` at 0x1df24 | store | 4 | `sp -0x3c` | `sp-56` |
| `str` at 0x1df2c | store | 4 | `sp -0x34` | `0x4` |
| `ldr` at 0x1df38 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x1df40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1df48 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x1df4c | load | 1 | `(unknown)` | `-` |
| `strb` at 0x1df50 | store | 1 | `(unknown)` | `?` |
| `str` at 0x1df58 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x1df5c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x1df64 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x1df6c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1df70 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x1df80 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x1df84 | store | 4 | `sp -0x2c` | `[sp-32]` |
| `ldr` at 0x1df88 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1df8c | store | 4 | `sp -0x30` | `[sp-36]` |
| `ldr` at 0x1df90 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x1df94 | store | 4 | `sp -0x2c` | `[sp-32]` |
| `ldr` at 0x1df98 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1df9c | store | 4 | `sp -0x30` | `[sp-36]` |
| `ldr` at 0x1dfa4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1dfa8 | load | 4 | `sp -0x44` | `-` |
| `strb` at 0x1dfb4 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x1dfb8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1dfbc | store | 4 | `sp -0x48` | `[sp-20]` |
| `ldr` at 0x1dfc0 | load | 4 | `sp -0x48` | `-` |
| `ldmdb` at 0x1dfc4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x1dfc4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x1dfc4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1dcbc | `(unknown callee) 0x18920` | `0x1` | `r1` | `[[0x11694c]+1936]` | `[[0x11694c]+1932]` |
| `bl` at 0x1de04 | `(unknown callee) 0x18920` | `?` | `r1` | `?` | `?` |
| `bl` at 0x1debc | `(unknown callee) 0x187d0` | `sp-56` | `[sp-60]` | `[sp-60]+1` | `sp-56` |
| `bl` at 0x1dee0 | `(unknown callee) 0x18938` | `([sp-20] + [sp-68])` | `0x13` | `0xd6f84` | `ret(0x187d0)` |
| `bl` at 0x1def4 | `(unknown callee) 0x18a1c` | `([sp-20] + [sp-68])` | `?` | `[sp-20]` | `([sp-20] + [sp-68])` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0x1dc1c | `[[0x11694c]+1936] <= [[0x11694c]+1932] (unsigned, 32-bit)` | `[[0x11694c]+1936] > [[0x11694c]+1932] (unsigned, 32-bit)` |
| `bhs` at 0x1dc70 | `[[0x11694c]+1936] >= [[0x11694c]+1932] (unsigned, 32-bit)` | `[[0x11694c]+1936] < [[0x11694c]+1932] (unsigned, 32-bit)` |
| `bne` at 0x1dcd0 | `ret(0x18920) != 0x0` | `ret(0x18920) == 0x0` |
| `bne` at 0x1dd18 | `Z == 0` | `not (Z == 0)` |
| `blo` at 0x1dd34 | `C == 0` | `not (C == 0)` |
| `beq` at 0x1dd44 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x1dd64 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x1dda8 | `Z == 0` | `not (Z == 0)` |
| `ble` at 0x1dddc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x1de18 | `ret(0x18920) != 0x0` | `ret(0x18920) == 0x0` |
| `bne` at 0x1de50 | `Z == 0` | `not (Z == 0)` |
| `blo` at 0x1de6c | `C == 0` | `not (C == 0)` |
| `beq` at 0x1de7c | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x1deb0 | `[sp-52]-1 != 0x0` | `[sp-52]-1 == 0x0` |
| `bne` at 0x1df1c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x1df78 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x1dfc4 | `r0` | `?` |

## `SHUB_ScanSetRackSlot`

- `SHUB_ScanSetRackSlot`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x251e8 to 0x25388.
  Established. A byte initializer writes `0x6c` at `sp -0x21` (`strb` at 0x25200). The call to
  `0x24790` has candidates `[sp-20]`, `0x101d0100`, `[sp-28]` and unknown (`bl` at 0x25318). A
  result other than one takes the branch back to the loop increment, while one falls through
  (`bne` at 0x25324) to a zero word at `sp -0x2c` (`str` at 0x2532c). The other exit path writes
  `0xfffff88e` to the same stack word (`str` at 0x25358). The rack/slot encoding and hardware
  transfer remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x251f8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x25200 | store | 1 | `sp -0x21` | `0x6c` |
| `str` at 0x25210 | store | 4 | `sp -0x28` | `ret(0x5e7a8)` |
| `ldr` at 0x25214 | load | 4 | `abs 0x25364` | `-` |
| `ldr` at 0x25218 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x25224 | load | 4 | `abs 0x25368` | `-` |
| `str` at 0x25228 | store | 4 | `sp -0x38` | `0xda654` |
| `ldr` at 0x25230 | load | 4 | `abs 0x2536c` | `-` |
| `ldr` at 0x25234 | load | 4 | `abs 0x25370` | `-` |
| `ldr` at 0x25240 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x25244 | load | 4 | `(unknown)` | `-` |
| `str` at 0x25250 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x25254 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x25258 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x25264 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x25268 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x25278 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25288 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x2528c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x2529c | load | 1 | `sp -0x21` | `-` |
| `str` at 0x252a4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x252a8 | load | 4 | `abs 0x25364` | `-` |
| `ldr` at 0x252ac | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x252b8 | load | 4 | `abs 0x25374` | `-` |
| `str` at 0x252bc | store | 4 | `sp -0x38` | `0xda66c` |
| `ldr` at 0x252c0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x252c4 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0x252c8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x252cc | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0x252d4 | load | 4 | `abs 0x2536c` | `-` |
| `ldr` at 0x252d8 | load | 4 | `abs 0x25378` | `-` |
| `str` at 0x252e8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x252ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x252fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25304 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2530c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25310 | load | 4 | `abs 0x2537c` | `-` |
| `ldr` at 0x25314 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2532c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x25334 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2533c | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x25344 | load | 4 | `abs 0x25380` | `-` |
| `ldr` at 0x25348 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2534c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x25354 | load | 4 | `abs 0x25384` | `-` |
| `str` at 0x25358 | store | 4 | `sp -0x2c` | `0xfffff88e` |
| `ldr` at 0x2535c | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x25360 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x25360 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x25360 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x25208 | `(unknown callee) 0x5e7a8` | `0x0` | `r1` | `r2` | `0x6c` |
| `bl` at 0x2523c | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x1bf` | `0x0` |
| `bl` at 0x252e0 | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x1ca` | `0x0` |
| `bl` at 0x25318 | `(unknown callee) 0x24790` | `[sp-20]` | `0x101d0100` | `[sp-28]` | `?` |
| `bl` at 0x25350 | `(unknown callee) 0x1dba4` | `0xda6a0` | `[sp-20]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x25220 | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `beq` at 0x252b4 | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `ble` at 0x252f4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x25324 | `ret(0x24790) != 0x1` | `ret(0x24790) == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x25360 | `r0` | `?` |

## `powerMargin`

- `powerMargin`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3b31c to 0x3b9cc.
  Established. The word at `0x1155e0` receives five (`str` at 0x3b48c) before a call to
  `0x5ee50` with first candidate five (`bl` at 0x3b494). Calls later include `0x3b9cc` (`bl` at
  0x3b568) and `0x3c2c8` (`bl` at 0x3b77c), with several candidates still unknown. The final branch
  is taken when `[sp-52] != 0`, skipping the last store to `0x1155e0`, and falls through when zero (`bne`
  at 0x3b958). That fall-through writes `[sp-24]` at `0x1155e0` (`str` at 0x3b964) and calls
  `0x5ee50` with that first candidate (`bl` at 0x3b96c). The electrical margin and timing effects
  are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3b32c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3b330 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x3b334 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x3b33c | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0x3b344 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x3b34c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x3b350 | load | 4 | `abs 0x3b980` | `-` |
| `ldr` at 0x3b354 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3b358 | load | 4 | `abs 0x281100` | `-` |
| `ldr` at 0x3b368 | load | 4 | `abs 0x3b984` | `-` |
| `ldr` at 0x3b36c | load | 4 | `abs 0x2810e0` | `-` |
| `ldr` at 0x3b370 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x3b378 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3b384 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3b398 | load | 4 | `abs 0x3b988` | `-` |
| `ldr` at 0x3b39c | load | 4 | `abs 0x281120` | `-` |
| `ldrb` at 0x3b3a4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3b3b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b3bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b3c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3b3d0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3b3d8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x3b3dc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3b3e4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x3b3ec | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3b3fc | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x3b404 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3b42c | store | 4 | `sp -0x30` | `ret(0x18920)` |
| `ldr` at 0x3b430 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3b43c | load | 4 | `abs 0x3b98c` | `-` |
| `str` at 0x3b440 | store | 4 | `sp -0x5c` | `0xe17cc` |
| `ldr` at 0x3b448 | load | 4 | `abs 0x3b990` | `-` |
| `ldr` at 0x3b44c | load | 4 | `abs 0x3b994` | `-` |
| `ldr` at 0x3b458 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3b474 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3b484 | load | 4 | `abs 0x3b998` | `-` |
| `str` at 0x3b48c | store | 4 | `abs 0x1155e0` | `0x5` |
| `str` at 0x3b49c | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x3b4a4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x3b4a8 | load | 4 | `abs 0x3b980` | `-` |
| `ldr` at 0x3b4ac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3b4b0 | load | 4 | `abs 0x281100` | `-` |
| `ldr` at 0x3b4c0 | load | 4 | `abs 0x3b984` | `-` |
| `ldr` at 0x3b4c4 | load | 4 | `abs 0x2810e0` | `-` |
| `ldr` at 0x3b4c8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x3b4d0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3b4dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3b4f0 | load | 4 | `abs 0x3b988` | `-` |
| `ldr` at 0x3b4f4 | load | 4 | `abs 0x281120` | `-` |
| `ldrb` at 0x3b4fc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3b508 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b514 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b518 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3b528 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3b544 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x3b54c | store | 4 | `sp -0x3c` | `?` |
| `str` at 0x3b554 | store | 4 | `sp -0x5c` | `sp-56` |
| `ldr` at 0x3b558 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3b55c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3b560 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3b564 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3b570 | store | 4 | `sp -0x20` | `ret(0x3b9cc)` |
| `ldr` at 0x3b574 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3b580 | load | 4 | `abs 0x3b99c` | `-` |
| `ldr` at 0x3b584 | load | 4 | `abs 0x119f98` | `-` |
| `ldr` at 0x3b590 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3b5a4 | load | 4 | `abs 0x3b988` | `-` |
| `ldr` at 0x3b5a8 | load | 4 | `abs 0x281120` | `-` |
| `ldr` at 0x3b5b0 | load | 4 | `abs 0x3b9a0` | `-` |
| `ldr` at 0x3b5b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3b5bc | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x3b5c4 | store | 4 | `[sp-60] +0x8` | `0x0` |
| `ldr` at 0x3b5c8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3b5d4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3b5d8 | store | 4 | `sp -0x34` | `[sp-32]` |
| `ldr` at 0x3b5e0 | load | 4 | `abs 0x3b99c` | `-` |
| `ldr` at 0x3b5e4 | load | 4 | `abs 0x119f98` | `-` |
| `ldr` at 0x3b5f0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3b5f4 | load | 4 | `[sp-60] +0x0` | `-` |
| `ldr` at 0x3b600 | load | 4 | `sp -0x3c` | `-` |
| `ldrb` at 0x3b604 | load | 1 | `[sp-60] +0x1d` | `-` |
| `ldr` at 0x3b608 | load | 4 | `sp -0x3c` | `-` |
| `ldrb` at 0x3b60c | load | 1 | `[sp-60] +0x1c` | `-` |
| `ldr` at 0x3b610 | load | 4 | `abs 0x3b9a4` | `-` |
| `ldr` at 0x3b618 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3b61c | load | 4 | `[sp-60] +0x18` | `-` |
| `ldr` at 0x3b62c | load | 4 | `sp -0x3c` | `-` |
| `ldrb` at 0x3b630 | load | 1 | `[sp-60] +0x21` | `-` |
| `ldr` at 0x3b634 | load | 4 | `sp -0x3c` | `-` |
| `ldrb` at 0x3b638 | load | 1 | `[sp-60] +0x1e` | `-` |
| `str` at 0x3b63c | store | 4 | `sp -0x5c` | `[[sp-60]+30]:1` |
| `ldr` at 0x3b640 | load | 4 | `sp -0x3c` | `-` |
| `ldrb` at 0x3b644 | load | 1 | `[sp-60] +0x20` | `-` |
| `str` at 0x3b648 | store | 4 | `sp -0x58` | `[[sp-60]+32]:1` |
| `ldr` at 0x3b64c | load | 4 | `abs 0x3b9a8` | `-` |
| `ldr` at 0x3b658 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3b65c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3b660 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3b664 | load | 4 | `sp -0x3c` | `-` |
| `ldrb` at 0x3b668 | load | 1 | `[sp-60] +0x24` | `-` |
| `str` at 0x3b66c | store | 4 | `sp -0x5c` | `[[sp-60]+36]:1` |
| `ldr` at 0x3b670 | load | 4 | `sp -0x3c` | `-` |
| `ldrb` at 0x3b674 | load | 1 | `[sp-60] +0x25` | `-` |
| `str` at 0x3b678 | store | 4 | `sp -0x58` | `[[sp-60]+37]:1` |
| `ldr` at 0x3b67c | load | 4 | `abs 0x3b9ac` | `-` |
| `ldr` at 0x3b680 | load | 4 | `[sp-60] +0x10` | `-` |
| `ldr` at 0x3b684 | load | 4 | `[sp-60] +0xc` | `-` |
| `ldr` at 0x3b688 | load | 4 | `[sp-60] +0x28` | `-` |
| `ldr` at 0x3b690 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3b69c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x3b6a4 | store | 4 | `[sp-60] +0x8` | `0x0` |
| `ldr` at 0x3b6a8 | load | 4 | `abs 0x3b99c` | `-` |
| `ldr` at 0x3b6ac | load | 4 | `abs 0x119f98` | `-` |
| `ldr` at 0x3b6b8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3b6bc | load | 4 | `[sp-60] +0x0` | `-` |
| `ldr` at 0x3b6c8 | load | 4 | `abs 0x3b9b0` | `-` |
| `ldr` at 0x3b6d4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3b6dc | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x3b6e0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3b6e8 | store | 4 | `sp -0x24` | `?` |
| `str` at 0x3b6f4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x3b6f8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3b70c | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0x3b71c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x3b720 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3b724 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3b734 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3b750 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x3b758 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x3b75c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x3b760 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3b774 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x3b784 | store | 4 | `sp -0x20` | `ret(0x3c2c8)` |
| `ldr` at 0x3b788 | load | 4 | `abs 0x3b99c` | `-` |
| `ldr` at 0x3b78c | load | 4 | `abs 0x119f98` | `-` |
| `ldr` at 0x3b798 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x3b79c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x3b7a8 | load | 4 | `abs 0x3b9b4` | `-` |
| `ldr` at 0x3b7b4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x3b7b8 | load | 4 | `[sp-64] +0x14` | `-` |
| `stmdb` at 0x3b7c4 | store | 4 | `sp -0x4c` | `ret(0xd5c38)` |
| `stmdb` at 0x3b7c4 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x3b7c8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x3b7cc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x3b7d0 | load | 4 | `[sp-64] +0x10` | `-` |
| `ldr` at 0x3b7d4 | load | 4 | `[sp-64] +0x28` | `-` |
| `ldr` at 0x3b7e4 | load | 4 | `abs 0x3b9b8` | `-` |
| `str` at 0x3b7e8 | store | 4 | `sp -0x50` | `0xe18c8` |
| `ldr` at 0x3b7f0 | load | 4 | `abs 0x3b9bc` | `-` |
| `str` at 0x3b7f4 | store | 4 | `sp -0x50` | `0xe18d0` |
| `ldr` at 0x3b7f8 | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0x3b7fc | load | 1 | `[sp-64] +0x21` | `-` |
| `str` at 0x3b800 | store | 4 | `sp -0x5c` | `[[sp-64]+33]:1` |
| `ldr` at 0x3b804 | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0x3b808 | load | 1 | `[sp-64] +0x1e` | `-` |
| `str` at 0x3b80c | store | 4 | `sp -0x58` | `[[sp-64]+30]:1` |
| `ldr` at 0x3b810 | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0x3b814 | load | 1 | `[sp-64] +0x20` | `-` |
| `str` at 0x3b818 | store | 4 | `sp -0x54` | `[[sp-64]+32]:1` |
| `ldr` at 0x3b81c | load | 4 | `abs 0x3b9c0` | `-` |
| `ldmdb` at 0x3b824 | load | 4 | `sp -0x4c` | `-` |
| `ldmdb` at 0x3b824 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3b828 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x3b830 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x3b834 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x3b838 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x3b83c | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0x3b840 | load | 1 | `[sp-64] +0x24` | `-` |
| `str` at 0x3b844 | store | 4 | `sp -0x5c` | `[[sp-64]+36]:1` |
| `ldr` at 0x3b848 | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0x3b84c | load | 1 | `[sp-64] +0x25` | `-` |
| `str` at 0x3b850 | store | 4 | `sp -0x58` | `[[sp-64]+37]:1` |
| `ldr` at 0x3b854 | load | 4 | `abs 0x3b9ac` | `-` |
| `ldr` at 0x3b858 | load | 4 | `[sp-64] +0x10` | `-` |
| `ldr` at 0x3b85c | load | 4 | `[sp-64] +0xc` | `-` |
| `ldr` at 0x3b860 | load | 4 | `[sp-64] +0x28` | `-` |
| `ldr` at 0x3b868 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3b874 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3b880 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3b884 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x3b888 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x3b890 | store | 4 | `[sp-64] +0x8` | `0x0` |
| `ldr` at 0x3b894 | load | 4 | `abs 0x3b99c` | `-` |
| `ldr` at 0x3b898 | load | 4 | `abs 0x119f98` | `-` |
| `ldr` at 0x3b8a4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x3b8a8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x3b8b4 | load | 4 | `abs 0x3b9c4` | `-` |
| `ldr` at 0x3b8c4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3b8d0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x3b8d8 | store | 4 | `[sp-64] +0x8` | `0x0` |
| `ldr` at 0x3b8dc | load | 4 | `abs 0x3b99c` | `-` |
| `ldr` at 0x3b8e0 | load | 4 | `abs 0x119f98` | `-` |
| `ldr` at 0x3b8ec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x3b8f0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x3b8fc | load | 4 | `abs 0x3b9c8` | `-` |
| `ldr` at 0x3b90c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x3b914 | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldr` at 0x3b918 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3b920 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x3b928 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3b938 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3b940 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x3b948 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3b950 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3b95c | load | 4 | `abs 0x3b998` | `-` |
| `ldr` at 0x3b960 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3b964 | store | 4 | `abs 0x1155e0` | `[sp-24]` |
| `ldr` at 0x3b970 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x3b974 | store | 4 | `sp -0x44` | `[sp-52]` |
| `ldr` at 0x3b978 | load | 4 | `sp -0x44` | `-` |
| `ldmdb` at 0x3b97c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3b97c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3b97c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3b424 | `(unknown callee) 0x18920` | `?` | `r1` | `?` | `?` |
| `bl` at 0x3b454 | `(unknown callee) 0x1eed0` | `0x9` | `0xe1584` | `0x56a` | `0x0` |
| `bl` at 0x3b480 | `(unknown callee) 0x18c20` | `?` | `0x0` | `?` | `?` |
| `bl` at 0x3b494 | `(unknown callee) 0x5ee50` | `0x5` | `?` | `0x1155e0` | `0x5` |
| `bl` at 0x3b568 | `(unknown callee) 0x3b9cc` | `?` | `?` | `[sp-24]` | `[sp-28]` |
| `bl` at 0x3b5b8 | `(unknown callee) 0x20f14` | `0xe17f0` | `?` | `?` | `?` |
| `bl` at 0x3b5f8 | `(unknown callee) 0x3d108` | `[[sp-60]]` | `?` | `?` | `[sp-60]` |
| `bl` at 0x3b614 | `(unknown callee) 0x20f14` | `0xe1804` | `ret(0x3d108)` | `[[sp-60]+29]:1` | `[[sp-60]+28]:1` |
| `bl` at 0x3b620 | `(unknown callee) 0xd5c38` | `[[sp-60]+24]` | `?` | `?` | `[sp-60]` |
| `bl` at 0x3b654 | `(unknown callee) 0x20f14` | `0xe1828` | `ret(0xd5c38)` | `?` | `[[sp-60]+33]:1` |
| `bl` at 0x3b68c | `(unknown callee) 0x20f14` | `0xe1864` | `[[sp-60]+16]` | `[[sp-60]+12]` | `[[sp-60]+40]` |
| `bl` at 0x3b6c0 | `(unknown callee) 0x3d108` | `[[sp-60]]` | `?` | `[sp-60]` | `[sp-60]` |
| `bl` at 0x3b6d0 | `(unknown callee) 0x20f14` | `0xe18ac` | `ret(0x3d108)` | `?` | `ret(0x3d108)` |
| `bl` at 0x3b714 | `(unknown callee) 0x3e294` | `0x6e` | `?` | `?` | `0x0` |
| `bl` at 0x3b77c | `(unknown callee) 0x3c2c8` | `?` | `sp-56` | `?` | `sp-56` |
| `bl` at 0x3b7a0 | `(unknown callee) 0x3d108` | `[[sp-64]]` | `?` | `?` | `[sp-64]` |
| `bl` at 0x3b7b0 | `(unknown callee) 0x20f14` | `0xe18bc` | `ret(0x3d108)` | `?` | `ret(0x3d108)` |
| `bl` at 0x3b7bc | `(unknown callee) 0xd5c38` | `[[sp-64]+20]` | `?` | `?` | `[sp-64]` |
| `bl` at 0x3b82c | `(unknown callee) 0x20f14` | `0xe18d8` | `[sp-76]` | `[sp-72]` | `[sp-80]` |
| `bl` at 0x3b864 | `(unknown callee) 0x20f14` | `0xe1864` | `[[sp-64]+16]` | `[[sp-64]+12]` | `[[sp-64]+40]` |
| `bl` at 0x3b8ac | `(unknown callee) 0x3d108` | `[[sp-64]]` | `?` | `[sp-64]` | `[sp-64]` |
| `bl` at 0x3b8bc | `(unknown callee) 0x20f14` | `0xe1918` | `ret(0x3d108)` | `?` | `ret(0x3d108)` |
| `bl` at 0x3b8f4 | `(unknown callee) 0x3d108` | `[[sp-64]]` | `?` | `[sp-64]` | `[sp-64]` |
| `bl` at 0x3b904 | `(unknown callee) 0x20f14` | `0xe192c` | `ret(0x3d108)` | `?` | `ret(0x3d108)` |
| `bl` at 0x3b94c | `(unknown callee) 0x18be4` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0x3b96c | `(unknown callee) 0x5ee50` | `[sp-24]` | `?` | `0x1155e0` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x3b360 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x3b380 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x3b3ac | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x3b3b8 | `r0 == 0x1a` | `r0 != 0x1a` |
| `beq` at 0x3b3c8 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x3b3f4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x3b438 | `ret(0x18920) != 0x0` | `ret(0x18920) == 0x0` |
| `blt` at 0x3b4b8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x3b4d8 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x3b504 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x3b510 | `[sp-20] == 0x1a` | `[sp-20] != 0x1a` |
| `beq` at 0x3b520 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x3b57c | `ret(0x3b9cc) == 0x0` | `ret(0x3b9cc) != 0x0` |
| `beq` at 0x3b58c | `[0x119f98] == 0x0` | `[0x119f98] != 0x0` |
| `bne` at 0x3b5d0 | `[sp-52] != 0x0` | `[sp-52] == 0x0` |
| `beq` at 0x3b5ec | `[0x119f98] == 0x0` | `[0x119f98] != 0x0` |
| `beq` at 0x3b698 | `[sp-56] == 0x0` | `[sp-56] != 0x0` |
| `beq` at 0x3b6b4 | `[0x119f98] == 0x0` | `[0x119f98] != 0x0` |
| `ble` at 0x3b700 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `blt` at 0x3b72c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x3b768 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x3b794 | `[0x119f98] == 0x0` | `[0x119f98] != 0x0` |
| `ble` at 0x3b7e0 | `? <= 0x0 (signed, 32-bit)` | `? > 0x0 (signed, 32-bit)` |
| `beq` at 0x3b870 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x3b87c | `[sp-52] != 0x0` | `[sp-52] == 0x0` |
| `beq` at 0x3b8a0 | `[0x119f98] == 0x0` | `[0x119f98] != 0x0` |
| `beq` at 0x3b8cc | `[sp-56] == 0x0` | `[sp-56] != 0x0` |
| `beq` at 0x3b8e8 | `[0x119f98] == 0x0` | `[0x119f98] != 0x0` |
| `bne` at 0x3b930 | `[sp-60] != 0x0` | `[sp-60] == 0x0` |
| `bne` at 0x3b958 | `[sp-52] != 0x0` | `[sp-52] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3b97c | `r0` | `?` |

## `L1CfgEventSystemConsole`

- `L1CfgEventSystemConsole`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5ed98 to 0x5edb8.
  Established. The only call has candidates `(0x1, 0x0, 0x0, entry r3)` (`bl` at 0x5edb0); its
  result reaches `r0` at the return (`ldmdb` at 0x5edb4). The memory accesses are stack saves (`push` at 0x5ed9c) and
  restores (`ldmdb` at 0x5edb4). No conditional branches are reported. The symbol alone does not establish a console operation.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x5edb4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5edb4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5edb4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5edb0 | `(unknown callee) 0x5eb18` | `0x1` | `0x0` | `0x0` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5edb4 | `r0` | `ret(0x5eb18)` |

## `L1CfgEventConsoleSubchannel`

- `L1CfgEventConsoleSubchannel`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5edb8 to 0x5edd8.
  Established. This wrapper calls `0x5eb18` with the first candidate two and the next two candidates
  zero (`bl` at 0x5edd0). Entry `r3` remains the fourth candidate at that call (`bl` at 0x5edd0).
  The exit preserves `ret(0x5eb18)` (`ldmdb` at 0x5edd4). There are no conditional branch records.
  The subchannel meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x5edd4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5edd4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5edd4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5edd0 | `(unknown callee) 0x5eb18` | `0x2` | `0x0` | `0x0` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5edd4 | `r0` | `ret(0x5eb18)` |

## `L1CfgEventConsoleEnable`

- `L1CfgEventConsoleEnable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5edd8 to 0x5edf8.
  Established. Candidates three, zero, zero and entry `r3` reach the call to `0x5eb18` (`bl` at
  0x5edf0). The recorded return contains its result (`ldmdb` at 0x5edf4). The memory accesses are stack saves (`push` at 0x5eddc)
  and restores (`ldmdb` at 0x5edf4). An enable-state transition cannot be established from the wrapper alone.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x5edf4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5edf4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5edf4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5edf0 | `(unknown callee) 0x5eb18` | `0x3` | `0x0` | `0x0` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5edf4 | `r0` | `ret(0x5eb18)` |

## `L1CfgEventPowerState`

- `L1CfgEventPowerState`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5edf8 to 0x5ee50.
  Established. Equality between `[sp-20]` and `[ret(0x5e7a8)+52]` takes the branch past the update,
  while inequality falls through (`beq` at 0x5ee2c). The fall-through writes a word containing
  `[sp-20]` at `ret(0x5e7a8) +0x34` (`str` at 0x5ee38), then calls `0x5eb18` with candidates `(0x4,
  0x0, 0x0, [sp-20])` (`bl` at 0x5ee48). The power-state encoding and physical transition remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5ee08 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x5ee18 | store | 4 | `sp -0x18` | `ret(0x5e7a8)` |
| `ldr` at 0x5ee1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5ee20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5ee24 | load | 4 | `ret(0x5e7a8) +0x34` | `-` |
| `ldr` at 0x5ee30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5ee34 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5ee38 | store | 4 | `ret(0x5e7a8) +0x34` | `[sp-20]` |
| `ldmdb` at 0x5ee4c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5ee4c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5ee4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5ee10 | `(unknown callee) 0x5e7a8` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x5ee48 | `(unknown callee) 0x5eb18` | `0x4` | `0x0` | `0x0` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5ee2c | `[sp-20] == [ret(0x5e7a8)+52]` | `[sp-20] != [ret(0x5e7a8)+52]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5ee4c | `r0` | `?` |

## `L1CfgEventMarginState`

- `L1CfgEventMarginState`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5ee50 to 0x5eea8.
  Established. The branch skips the update when `[sp-20] == [ret(0x5e7a8)+56]`, with inequality
  falling through (`beq` at 0x5ee84). On the fall-through path, the word at `ret(0x5e7a8) +0x38`
  receives `[sp-20]` (`str` at 0x5ee90). The subsequent call carries candidates `(0x5, 0x0, 0x0,
  [sp-20])` (`bl` at 0x5eea0). The represented margin setting is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5ee60 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x5ee70 | store | 4 | `sp -0x18` | `ret(0x5e7a8)` |
| `ldr` at 0x5ee74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5ee78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5ee7c | load | 4 | `ret(0x5e7a8) +0x38` | `-` |
| `ldr` at 0x5ee88 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5ee8c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5ee90 | store | 4 | `ret(0x5e7a8) +0x38` | `[sp-20]` |
| `ldmdb` at 0x5eea4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5eea4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5eea4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5ee68 | `(unknown callee) 0x5e7a8` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x5eea0 | `(unknown callee) 0x5eb18` | `0x5` | `0x0` | `0x0` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5ee84 | `[sp-20] == [ret(0x5e7a8)+56]` | `[sp-20] != [ret(0x5e7a8)+56]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5eea4 | `r0` | `?` |

## `L1CfgEventPartition`

- `L1CfgEventPartition`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5eea8 to 0x5f028.
  Established. The initial equality branch skips the remaining work when `[sp-20] ==
  [ret(0x5e7a8)+60]` (`beq` at 0x5eedc); its inequality fall-through writes `[sp-20]` at
  `ret(0x5e7a8) +0x3c` (`str` at 0x5eee8). Later calls to `0x5bba0` have first candidates one (`bl`
  at 0x5f008), three (`bl` at 0x5f010), `0xe` (`bl` at 0x5f018) and `0xf` (`bl` at 0x5f020). The
  branch table preserves each selector comparison separately. Partition identity and propagation
  semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5eeb8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x5eec8 | store | 4 | `sp -0x18` | `ret(0x5e7a8)` |
| `ldr` at 0x5eecc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5eed0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5eed4 | load | 4 | `ret(0x5e7a8) +0x3c` | `-` |
| `ldr` at 0x5eee0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5eee4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5eee8 | store | 4 | `ret(0x5e7a8) +0x3c` | `[sp-20]` |
| `ldr` at 0x5ef00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5ef04 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x5ef10 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5ef14 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x5ef20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5ef24 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x5ef30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5ef34 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x5ef40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5ef44 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x5ef50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5ef54 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x5ef60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5ef64 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x5ef70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5ef74 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x5ef80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5ef84 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x5ef90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5ef94 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x5efa0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5efa4 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x5efb0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5efb4 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x5efc0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5efc4 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x5efd0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5efd4 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x5efe0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5efe4 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x5eff0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5eff4 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldmdb` at 0x5f024 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5f024 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5f024 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5eec0 | `(unknown callee) 0x5e7a8` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x5eeec | `(unknown callee) 0x54c20` | `ret(0x5e7a8)` | `?` | `ret(0x5e7a8)` | `[sp-20]` |
| `bl` at 0x5eefc | `(unknown callee) 0x5eb18` | `0x6` | `0x0` | `0x0` | `?` |
| `bl` at 0x5f008 | `(unknown callee) 0x5bba0` | `0x1` | `?` | `?` | `[[sp-24]+16]` |
| `bl` at 0x5f010 | `(unknown callee) 0x5bba0` | `0x3` | `?` | `?` | `?` |
| `bl` at 0x5f018 | `(unknown callee) 0x5bba0` | `0xe` | `?` | `?` | `?` |
| `bl` at 0x5f020 | `(unknown callee) 0x5bba0` | `0xf` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5eedc | `[sp-20] == [ret(0x5e7a8)+60]` | `[sp-20] != [ret(0x5e7a8)+60]` |
| `beq` at 0x5ef0c | `[[sp-24]+16] == 0x43` | `[[sp-24]+16] != 0x43` |
| `beq` at 0x5ef1c | `[[sp-24]+16] == 0x33` | `[[sp-24]+16] != 0x33` |
| `beq` at 0x5ef2c | `[[sp-24]+16] == 0x35` | `[[sp-24]+16] != 0x35` |
| `beq` at 0x5ef3c | `[[sp-24]+16] == 0x34` | `[[sp-24]+16] != 0x34` |
| `beq` at 0x5ef4c | `[[sp-24]+16] == 0x41` | `[[sp-24]+16] != 0x41` |
| `beq` at 0x5ef5c | `[[sp-24]+16] == 0x36` | `[[sp-24]+16] != 0x36` |
| `beq` at 0x5ef6c | `[[sp-24]+16] == 0x39` | `[[sp-24]+16] != 0x39` |
| `beq` at 0x5ef7c | `[[sp-24]+16] == 0x37` | `[[sp-24]+16] != 0x37` |
| `beq` at 0x5ef8c | `[[sp-24]+16] == 0x38` | `[[sp-24]+16] != 0x38` |
| `beq` at 0x5ef9c | `[[sp-24]+16] == 0x30` | `[[sp-24]+16] != 0x30` |
| `beq` at 0x5efac | `[[sp-24]+16] == 0x51` | `[[sp-24]+16] != 0x51` |
| `beq` at 0x5efbc | `[[sp-24]+16] == 0x40` | `[[sp-24]+16] != 0x40` |
| `beq` at 0x5efcc | `[[sp-24]+16] == 0x4f` | `[[sp-24]+16] != 0x4f` |
| `beq` at 0x5efdc | `[[sp-24]+16] == 0x42` | `[[sp-24]+16] != 0x42` |
| `beq` at 0x5efec | `[[sp-24]+16] == 0x46` | `[[sp-24]+16] != 0x46` |
| `beq` at 0x5effc | `[[sp-24]+16] == 0x4c` | `[[sp-24]+16] != 0x4c` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5f024 | `r0` | `?` |

## `L1CfgEventSystemSN`

- `L1CfgEventSystemSN`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5f028 to 0x5f0a4.
  Established. Two calls to `0x1c79c` each carry first candidate `0xd` (`bl` at 0x5f054, `bl` at
  0x5f074). Calls to `0x18cd4` use first candidates `[sp-20]+76` (`bl` at 0x5f06c) and `0x282348`
  (`bl` at 0x5f084), with the respective `ret(0x1c79c)` as second candidate. The call to `0x5eb18`
  has first candidate seven (`bl` at 0x5f094), followed by a call to `0x55c3c` (`bl` at 0x5f098).
  The return records the latter result (`ldmdb` at 0x5f09c). Serial-number representation and
  copying semantics are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5f044 | store | 4 | `sp -0x14` | `ret(0x5e7a8)` |
| `str` at 0x5f04c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x5f05c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5f07c | load | 4 | `abs 0x5f0a0` | `-` |
| `ldmdb` at 0x5f09c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5f09c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5f09c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5f03c | `(unknown callee) 0x5e7a8` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x5f054 | `(unknown callee) 0x1c79c` | `0xd` | `?` | `?` | `0x0` |
| `bl` at 0x5f06c | `(unknown callee) 0x18cd4` | `[sp-20]+76` | `ret(0x1c79c)` | `ret(0x1c79c)` | `[sp-20]+76` |
| `bl` at 0x5f074 | `(unknown callee) 0x1c79c` | `0xd` | `?` | `?` | `?` |
| `bl` at 0x5f084 | `(unknown callee) 0x18cd4` | `0x282348` | `ret(0x1c79c)` | `?` | `ret(0x1c79c)` |
| `bl` at 0x5f094 | `(unknown callee) 0x5eb18` | `0x7` | `0x0` | `0x0` | `?` |
| `bl` at 0x5f098 | `(unknown callee) 0x55c3c` | `ret(0x5eb18)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5f09c | `r0` | `ret(0x55c3c)` |

## `L1CfgEventEnvironmental`

- `L1CfgEventEnvironmental`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5f0a4 to 0x5f0fc.
  Established. The comparison takes its branch when `[sp-20] == [ret(0x5e7a8)+92]`, and falls
  through when unequal (`beq` at 0x5f0d8). The unequal path stores `[sp-20]` as a word at
  `ret(0x5e7a8) +0x5c` (`str` at 0x5f0e4), then calls `0x5eb18` with candidates `(0x8, 0x0, 0x0,
  [sp-20])` (`bl` at 0x5f0f4). The environmental encoding is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5f0b4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x5f0c4 | store | 4 | `sp -0x18` | `ret(0x5e7a8)` |
| `ldr` at 0x5f0c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5f0cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5f0d0 | load | 4 | `ret(0x5e7a8) +0x5c` | `-` |
| `ldr` at 0x5f0dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5f0e0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5f0e4 | store | 4 | `ret(0x5e7a8) +0x5c` | `[sp-20]` |
| `ldmdb` at 0x5f0f8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5f0f8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5f0f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5f0bc | `(unknown callee) 0x5e7a8` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x5f0f4 | `(unknown callee) 0x5eb18` | `0x8` | `0x0` | `0x0` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5f0d8 | `[sp-20] == [ret(0x5e7a8)+92]` | `[sp-20] != [ret(0x5e7a8)+92]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5f0f8 | `r0` | `?` |

## `L1CfgEventNVRAM`

- `L1CfgEventNVRAM`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5f0fc to 0x5f128.
  Established. The two entry values are saved as words at `sp -0x14` (`str` at 0x5f10c) and `sp
  -0x18` (`str` at 0x5f110). The call to `0x5eb18` uses candidates `(0x66, entry r0, entry r1, entry
  r3)` (`bl` at 0x5f120), and its result reaches the exit (`ldmdb` at 0x5f124). The NVRAM operation
  associated with that candidate remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5f10c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x5f110 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x5f118 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5f11c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5f124 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5f124 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5f124 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5f120 | `(unknown callee) 0x5eb18` | `0x66` | `r0` | `r1` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5f124 | `r0` | `ret(0x5eb18)` |

## `EnvPowerInt`

- `EnvPowerInt`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x66ed8 to 0x6712c.
  Established. Entry values are truncated into stack bytes as `(r0 & 0xff)` (`strb` at 0x66ef0) and
  `(r1 & 0xff)` (`strb` at 0x66ef8). The byte store of one has an unknown effective target (`strb`
  at 0x670cc). The subsequent call to `0x1fe64` has first candidate `0x11a7a0` (`bl` at 0x670d4). A
  zero result takes the branch past the byte clear, while nonzero falls through (`beq` at 0x670e0)
  to a zero byte at `[sp-32] +0x1` (`strb` at 0x670ec). Interrupt ownership and device
  acknowledgement are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x66ef0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x66ef8 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `str` at 0x66f00 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x66f04 | load | 4 | `abs 0x67100` | `-` |
| `ldr` at 0x66f08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x66f0c | load | 4 | `abs 0x11aa68` | `-` |
| `ldr` at 0x66f1c | load | 4 | `abs 0x67100` | `-` |
| `ldr` at 0x66f20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x66f34 | load | 4 | `abs 0x11aa74` | `-` |
| `str` at 0x66f3c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x66f40 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x66f44 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x66f48 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x66f54 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x66f58 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x66f5c | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0x66f6c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x66f74 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x66f7c | load | 4 | `abs 0x67100` | `-` |
| `ldr` at 0x66f80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x66f84 | load | 4 | `abs 0x11aa68` | `-` |
| `ldr` at 0x66f90 | load | 4 | `abs 0x67104` | `-` |
| `str` at 0x66f94 | store | 4 | `sp -0x34` | `0xe7988` |
| `ldrb` at 0x66f98 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x66f9c | store | 4 | `sp -0x30` | `(r0 & 0xff)` |
| `ldrb` at 0x66fa0 | load | 1 | `sp -0x12` | `-` |
| `str` at 0x66fa4 | store | 4 | `sp -0x2c` | `(r1 & 0xff)` |
| `ldr` at 0x66fac | load | 4 | `abs 0x67108` | `-` |
| `ldr` at 0x66fb0 | load | 4 | `abs 0x6710c` | `-` |
| `str` at 0x66fc0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x66fc8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x66fd0 | load | 4 | `abs 0x67110` | `-` |
| `str` at 0x66fd8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x66fdc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x66fe0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x66fec | load | 4 | `abs 0x67100` | `-` |
| `ldr` at 0x66ff0 | load | 4 | `abs 0x11aa5c` | `-` |
| `ldr` at 0x66ffc | load | 4 | `abs 0x67100` | `-` |
| `ldr` at 0x67000 | load | 4 | `abs 0x11aa60` | `-` |
| `ldr` at 0x6700c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x67024 | load | 4 | `abs 0x67114` | `-` |
| `str` at 0x67028 | store | 4 | `sp -0x34` | `0xe79c0` |
| `str` at 0x6702c | store | 4 | `sp -0x30` | `ret(0x3d108)` |
| `ldrb` at 0x67030 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x67034 | store | 4 | `sp -0x2c` | `[sp-17]:1` |
| `ldrb` at 0x67038 | load | 1 | `sp -0x12` | `-` |
| `str` at 0x6703c | store | 4 | `sp -0x28` | `[sp-18]:1` |
| `ldr` at 0x67044 | load | 4 | `abs 0x67108` | `-` |
| `ldr` at 0x67048 | load | 4 | `abs 0x67118` | `-` |
| `str` at 0x67058 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x67060 | load | 4 | `abs 0x6711c` | `-` |
| `ldr` at 0x67064 | load | 4 | `abs 0x11a764` | `-` |
| `ldr` at 0x67074 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6708c | load | 4 | `abs 0x67120` | `-` |
| `ldr` at 0x67098 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x6709c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x670a8 | load | 4 | `abs 0x6711c` | `-` |
| `ldr` at 0x670ac | load | 4 | `abs 0x11a764` | `-` |
| `ldr` at 0x670bc | load | 4 | `abs 0x67124` | `-` |
| `ldr` at 0x670c4 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x670cc | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x670d0 | load | 4 | `abs 0x67128` | `-` |
| `ldr` at 0x670e4 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x670ec | store | 1 | `[sp-32] +0x1` | `0x0` |
| `str` at 0x670f4 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x670f8 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x670fc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x670fc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x670fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x66fb8 | `(unknown callee) 0x1eed0` | `0x1` | `0xe7664` | `0x59c` | `0x0` |
| `bl` at 0x67010 | `(unknown callee) 0x3da54` | `[sp-28]` | `?` | `?` | `[0x11aa60]` |
| `bl` at 0x6701c | `(unknown callee) 0x3d108` | `ret(0x3da54)` | `?` | `?` | `ret(0x3da54)` |
| `bl` at 0x67050 | `(unknown callee) 0x1eed0` | `0x1` | `0xe7664` | `0x5a9` | `0x0` |
| `bl` at 0x67078 | `(unknown callee) 0x3da54` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x67084 | `(unknown callee) 0x3d108` | `ret(0x3da54)` | `?` | `?` | `ret(0x3da54)` |
| `bl` at 0x67094 | `(unknown callee) 0x20f14` | `0xe79f4` | `ret(0x3d108)` | `?` | `ret(0x3d108)` |
| `bl` at 0x670c0 | `(unknown callee) 0x20f14` | `0xe7a08` | `?` | `?` | `?` |
| `bl` at 0x670d4 | `(unknown callee) 0x1fe64` | `0x11a7a0` | `?` | `?` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0x66f14 | `C == 0` | `not (C == 0)` |
| `bne` at 0x66f50 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x66f64 | `Z == 0` | `not (Z == 0)` |
| `blo` at 0x66f8c | `C == 0` | `not (C == 0)` |
| `bne` at 0x66fe8 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x66ff8 | `[0x11aa5c] != 0x0` | `[0x11aa5c] == 0x0` |
| `bne` at 0x67008 | `[0x11aa60] != 0x0` | `[0x11aa60] == 0x0` |
| `beq` at 0x67070 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x670a4 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x670b8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x670e0 | `ret(0x1fe64) == 0x0` | `ret(0x1fe64) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x670fc | `r0` | `?` |

## `powerQuery`

- `powerQuery`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7cb00 to 0x7cf00.
  Established. The first call uses `sp-36` as its first candidate (`bl` at 0x7cb28); its nonzero
  result takes the branch, while zero falls through (`bne` at 0x7cb38) to a call with first
  candidate `0x50` (`bl` at 0x7cb44) and a word store of `ret(0x18920)` at `[sp-24] +0x0` (`str` at
  0x7cb4c). The longer path includes calls to `0x3cd7c` with candidates `([sp-44], 0x0, sp-72,
  sp-72)` (`bl` at 0x7cd38) and `0x3ae7c` with candidates `([sp-44], sp-57, unknown, sp-57)` (`bl`
  at 0x7ce08). Query fields and measurement units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7cb10 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x7cb14 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x7cb1c | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x7cb2c | store | 4 | `sp -0x28` | `ret(0x39994)` |
| `ldr` at 0x7cb30 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7cb3c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7cb4c | store | 4 | `[sp-24] +0x0` | `ret(0x18920)` |
| `ldr` at 0x7cb50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7cb54 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x7cb58 | load | 4 | `abs 0x7cebc` | `-` |
| `ldr` at 0x7cb64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7cb68 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x7cb8c | store | 4 | `[sp-24] +0x0` | `ret(0x18920)` |
| `ldr` at 0x7cb90 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x7cb94 | store | 4 | `sp -0x4c` | `[[sp-24]]` |
| `ldr` at 0x7cb98 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7cba4 | load | 4 | `abs 0x7cec0` | `-` |
| `str` at 0x7cba8 | store | 4 | `sp -0x50` | `0xf3218` |
| `ldr` at 0x7cbb0 | load | 4 | `abs 0x7cec4` | `-` |
| `str` at 0x7cbb4 | store | 4 | `sp -0x50` | `0xec324` |
| `ldr` at 0x7cbb8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7cbc4 | load | 4 | `abs 0x7cec8` | `-` |
| `str` at 0x7cbc8 | store | 4 | `sp -0x54` | `0xf3220` |
| `ldr` at 0x7cbd0 | load | 4 | `abs 0x7cec4` | `-` |
| `str` at 0x7cbd4 | store | 4 | `sp -0x54` | `0xec324` |
| `ldr` at 0x7cbd8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x7cbdc | load | 4 | `abs 0x7cecc` | `-` |
| `ldr` at 0x7cbe0 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x7cbe4 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x7cbec | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x7cbfc | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x7cc04 | store | 4 | `sp -0x4c` | `([sp-76] + ret(0x18a1c))` |
| `str` at 0x7cc0c | store | 4 | `sp -0x2c` | `0x0` |
| `strb` at 0x7cc14 | store | 1 | `sp -0x2d` | `?` |
| `ldr` at 0x7cc18 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x7cc1c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7cc2c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x7cc38 | store | 4 | `sp -0x34` | `ret(0x3d108)` |
| `ldr` at 0x7cc3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7cc48 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x7cc5c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x7cc64 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x7cc74 | load | 1 | `sp -0x2d` | `-` |
| `strb` at 0x7cc7c | store | 1 | `sp -0x2d` | `([sp-45]:1+1 & 0xff)` |
| `ldr` at 0x7cc80 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x7cc94 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x7cca0 | store | 4 | `sp -0x40` | `ret(0x3aa60)` |
| `ldr` at 0x7cca4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x7cca8 | load | 4 | `abs 0x7ced0` | `-` |
| `ldr` at 0x7ccb4 | load | 4 | `abs 0x7ced4` | `-` |
| `str` at 0x7ccb8 | store | 4 | `sp -0x44` | `0xf328c` |
| `ldr` at 0x7ccc0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x7cccc | load | 4 | `abs 0x7ced8` | `-` |
| `str` at 0x7ccd0 | store | 4 | `sp -0x58` | `0xe9288` |
| `ldr` at 0x7ccd8 | load | 4 | `abs 0x7cedc` | `-` |
| `str` at 0x7ccdc | store | 4 | `sp -0x58` | `0xe92a4` |
| `ldr` at 0x7cce0 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x7cce4 | store | 4 | `sp -0x44` | `[sp-88]` |
| `ldr` at 0x7cce8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x7ccfc | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x7cd04 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7cd10 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x7cd14 | load | 4 | `abs 0x7cee0` | `-` |
| `ldr` at 0x7cd18 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x7cd1c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x7cd2c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x7cd40 | store | 4 | `sp -0x20` | `ret(0x3cd7c)` |
| `ldr` at 0x7cd44 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7cd50 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7cd54 | store | 4 | `sp -0x64` | `ret(0x3cd7c)` |
| `ldr` at 0x7cd58 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x7cd5c | load | 4 | `abs 0x7cee4` | `-` |
| `ldr` at 0x7cd60 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x7cd64 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x7cd70 | load | 4 | `sp -0x48` | `-` |
| `stm` at 0x7cd80 | store | 4 | `sp -0x64` | `ret(0xd5c38)` |
| `stm` at 0x7cd80 | store | 4 | `sp -0x60` | `?` |
| `ldr` at 0x7cd84 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x7cd88 | load | 4 | `abs 0x7cee8` | `-` |
| `ldr` at 0x7cd8c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x7cd90 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x7cd98 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x7cda8 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x7cdb0 | store | 4 | `sp -0x4c` | `([sp-76] + ret(0x18a1c))` |
| `ldr` at 0x7cdb4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x7cdc8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x7cdd0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7cddc | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x7cde0 | load | 4 | `abs 0x7ceec` | `-` |
| `ldr` at 0x7cdec | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x7cdf8 | store | 4 | `sp -0x38` | `ret(0x3d1d0)` |
| `ldr` at 0x7ce00 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x7ce10 | store | 4 | `sp -0x20` | `ret(0x3ae7c)` |
| `ldr` at 0x7ce14 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7ce20 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x7ce24 | load | 4 | `abs 0x7cef0` | `-` |
| `ldr` at 0x7ce28 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x7ce2c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7ce38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7ce44 | load | 4 | `abs 0x7cef4` | `-` |
| `str` at 0x7ce48 | store | 4 | `sp -0x5c` | `0xf3300` |
| `ldr` at 0x7ce50 | load | 4 | `abs 0x7cef8` | `-` |
| `str` at 0x7ce54 | store | 4 | `sp -0x5c` | `0xf3310` |
| `ldrb` at 0x7ce58 | load | 1 | `sp -0x39` | `-` |
| `ldrb` at 0x7ce5c | load | 1 | `sp -0x2d` | `-` |
| `str` at 0x7ce60 | store | 4 | `sp -0x64` | `[sp-45]:1` |
| `ldr` at 0x7ce64 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x7ce68 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x7ce6c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x7ce7c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x7ce80 | load | 4 | `abs 0x7cefc` | `-` |
| `ldr` at 0x7ce84 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x7ce8c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x7ce9c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x7cea4 | store | 4 | `sp -0x4c` | `([sp-76] + ret(0x18a1c))` |
| `ldr` at 0x7cea8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x7ceb0 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldmdb` at 0x7ceb8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x7ceb8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x7ceb8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x7ceb8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7cb28 | `(unknown callee) 0x39994` | `sp-36` | `r1` | `r2` | `sp-36` |
| `bl` at 0x7cb44 | `(unknown callee) 0x18920` | `0x50` | `?` | `?` | `ret(0x39994)` |
| `bl` at 0x7cb5c | `(unknown callee) 0x18cd4` | `[[sp-24]]` | `0xf31f0` | `?` | `[sp-24]` |
| `bl` at 0x7cb84 | `(unknown callee) 0x18920` | `?` | `?` | `ret(0x39994)` | `?` |
| `bl` at 0x7cbe8 | `(unknown callee) 0x18c74` | `[[sp-24]]` | `0xf3228` | `[sp-80]` | `[sp-84]` |
| `bl` at 0x7cbf0 | `(unknown callee) 0x18a1c` | `[sp-76]` | `?` | `?` | `?` |
| `bl` at 0x7cc30 | `(unknown callee) 0x3d108` | `[sp-44]` | `?` | `[sp-44]` | `[sp-40]` |
| `bl` at 0x7cc84 | `(unknown callee) 0x399d0` | `[sp-44]` | `?` | `?` | `[sp-45]:1+1` |
| `bl` at 0x7cc98 | `(unknown callee) 0x3aa60` | `[sp-44]` | `?` | `?` | `ret(0x399d0)` |
| `bl` at 0x7cd20 | `(unknown callee) 0x18c74` | `[sp-76]` | `0xf3290` | `[sp-52]` | `?` |
| `bl` at 0x7cd38 | `(unknown callee) 0x3cd7c` | `[sp-44]` | `0x0` | `sp-72` | `sp-72` |
| `bl` at 0x7cd68 | `(unknown callee) 0x18c74` | `[sp-76]` | `0xf32ac` | `[sp-52]` | `[sp-68]` |
| `bl` at 0x7cd74 | `(unknown callee) 0xd5c38` | `[sp-72]` | `?` | `?` | `ret(0x3cd7c)` |
| `bl` at 0x7cd94 | `(unknown callee) 0x18c74` | `[sp-76]` | `0xf32c8` | `[sp-52]` | `[sp-68]` |
| `bl` at 0x7cd9c | `(unknown callee) 0x18a1c` | `[sp-76]` | `?` | `?` | `?` |
| `bl` at 0x7cde4 | `(unknown callee) 0x18c74` | `([sp-76] + ret(0x18a1c))` | `0xf32e0` | `?` | `?` |
| `bl` at 0x7cdf0 | `(unknown callee) 0x3d1d0` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0x7ce08 | `(unknown callee) 0x3ae7c` | `[sp-44]` | `sp-57` | `?` | `sp-57` |
| `bl` at 0x7ce30 | `(unknown callee) 0x18c74` | `[sp-76]` | `0xf32ec` | `[sp-56]` | `ret(0x3ae7c)` |
| `bl` at 0x7ce74 | `(unknown callee) 0x18c74` | `[sp-76]` | `[sp-92]` | `[sp-56]` | `[sp-57]:1` |
| `bl` at 0x7ce88 | `(unknown callee) 0x18c74` | `[sp-76]` | `0xf331c` | `[sp-52]` | `ret(0x399d0)` |
| `bl` at 0x7ce90 | `(unknown callee) 0x18a1c` | `[sp-76]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x7cb38 | `ret(0x39994) != 0x0` | `ret(0x39994) == 0x0` |
| `beq` at 0x7cba0 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `beq` at 0x7cbc0 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `blo` at 0x7cc24 | `[sp-44] < [sp-40] (unsigned, 32-bit)` | `[sp-44] >= [sp-40] (unsigned, 32-bit)` |
| `beq` at 0x7cc44 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `bne` at 0x7cc6c | `? != 0xff` | `? == 0xff` |
| `beq` at 0x7cc90 | `ret(0x399d0) == 0x0` | `ret(0x399d0) != 0x0` |
| `bne` at 0x7ccb0 | `ret(0x3aa60) != 0xfffff9b8` | `ret(0x3aa60) == 0xfffff9b8` |
| `bne` at 0x7ccc8 | `ret(0x3aa60) != 0x1` | `ret(0x3aa60) == 0x1` |
| `bne` at 0x7cd0c | `? != 0xff` | `? == 0xff` |
| `beq` at 0x7cd4c | `ret(0x3cd7c) == 0x0` | `ret(0x3cd7c) != 0x0` |
| `bne` at 0x7cdd8 | `? != 0xff` | `? == 0xff` |
| `beq` at 0x7ce1c | `ret(0x3ae7c) == 0x0` | `ret(0x3ae7c) != 0x0` |
| `beq` at 0x7ce40 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x7ceb8 | `r0` | `?` |
