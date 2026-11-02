# MMSC serial-driver and port routines in IRIX 6.5.2

Read from `6.5.2/usr/cpu/firmware/mmscfw.bin` (IRIX 6.5.2 carrier).
Image SHA-256: `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol, or the next global text symbol across local
labels, as an inferred end and can include literals
after returns. Statements below concern sites reached by known control-flow edges.
Unresolved computed jumps make the effects incomplete for the affected routines.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands and I/O ports. Implicit
stack operations appear in the full fact sheets. I/O prefixes identify ports. Candidate slots
can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/serial-6.5.2-facts.txt) include all tool sections.

## `_ns16550CallbackInstall`

- `_ns16550CallbackInstall`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x176424 to 0x176498.
  Established. The second `je` takes the write region for `[sp+8] == 0x2`, where four-byte stores
  place `[sp+12]` through `[sp+4] +0x8` and `[sp+16]` through `[sp-8] +0x10` (`je` at 0x17643b,
  `mov` at 0x17646a, `mov` at 0x176473). The earlier `je` selects stores through `[sp+4] +0x4` and
  `[sp-8] +0xc` for `[sp+8] == 0x1`, carrying `[sp+12]` and `[sp+16]` respectively (`je` at
  0x176436, `mov` at 0x17644a, `mov` at 0x176453).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x17642a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17642d | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x176430 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x176444 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x176447 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x17644a | store | 4 | `[sp+4] +0x4` | `[sp+12]` |
| `mov` at 0x17644d | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x176450 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x176453 | store | 4 | `[sp-8] +0xc` | `[sp+16]` |
| `mov` at 0x176464 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x176467 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x17646a | store | 4 | `[sp+4] +0x8` | `[sp+12]` |
| `mov` at 0x17646d | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x176470 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x176473 | store | 4 | `[sp-8] +0x10` | `[sp+16]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x176436 | `[sp+8] == 0x1` | `[sp+8] != 0x1` |
| `je` at 0x17643b | `[sp+8] == 0x2` | `[sp+8] != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x176497 | `eax` | `?` |

## `_ns16550DummyCallback`

- `_ns16550DummyCallback`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x175374 to 0x175394.
  Established. The all-ones EAX load precedes a jump to the return tail (`mov` at 0x175377, `jmp` at
  0x17537c). The recorded exit value is 0xffffffff (`ret` at 0x175387).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x175387 | `eax` | `0xffffffff` |

## `_ns16550Hup`

- `_ns16550Hup`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x175974 to 0x175a54.
  Established. The `_intUnlock` first candidate is `ret(_intLock)` (`call` at 0x175a2b). Earlier
  indirect calls have first candidates `[[sp+4]+52]`, `[[sp+4]+68]`, `[[sp+4]+56]` and `[[sp+4]+52]`
  at distinct sites, with second candidate zero at the first site and unknown at the other three
  (`call` at 0x17598f, `call` at 0x1759cf, `call` at 0x1759fc, `call` at 0x175a25).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x175980 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175985 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175988 | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x17598c | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x175994 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175997 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17599a | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x1759a2 | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x1759a5 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1759a8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1759ab | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x1759b3 | store | 4 | `[sp+4] +0x1c` | `?` |
| `mov` at 0x1759b6 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1759b9 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1759bc | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x1759c5 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1759c8 | load | 4 | `[sp+4] +0x44` | `-` |
| `mov` at 0x1759cc | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x1759d4 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1759d7 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1759da | load | 1 | `[sp+4] +0x1a` | `-` |
| `mov` at 0x1759e2 | store | 1 | `[sp+4] +0x1a` | `?` |
| `mov` at 0x1759e5 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1759e8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1759eb | load | 1 | `[sp+4] +0x1a` | `-` |
| `mov` at 0x1759f2 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1759f5 | load | 4 | `[sp+4] +0x38` | `-` |
| `mov` at 0x1759f9 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x175a01 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175a04 | load | 2 | `[sp+4] +0x5c` | `-` |
| `mov` at 0x175a0e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175a11 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175a14 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x175a1b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175a1e | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x175a22 | load | 4 | `[sp+4] +0x2c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x175979 | `_intLock 0x1713d0` | `ebx` | `esi` | `ebp` | `?` | `?` | `?` |
| `call` at 0x17598f | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x1759cf | `(unknown callee) (unknown target)` | `[[sp+4]+68]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1759fc | `(unknown callee) (unknown target)` | `[[sp+4]+56]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x175a25 | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x175a2b | `_intUnlock 0x1713e0` | `ret(_intLock)` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x175a0c | `([[sp+4]+92]:2 & 0xffff) != 0x2` | `([[sp+4]+92]:2 & 0xffff) == 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x175a4c | `eax` | `0x0` |

## `_ns16550InitChannel`

- `_ns16550InitChannel`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1753f4 to 0x1756e4.
  Established. Byte-store sites carry 3 through `[sp+4] +0x1b`, 7 and 0x89 through `[sp+4] +0x1a`,
  and 3 through `[sp+4] +0x19` (`mov` at 0x175404, `mov` at 0x175445, `mov` at 0x175486, `mov` at
  0x1755fa). The indirect call after the latter store has candidates beginning `[[sp+4]+52]` and
  zero (`call` at 0x175619).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1753fb | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1753fe | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x175401 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x175404 | store | 1 | `[sp+4] +0x1b` | `0x3` |
| `mov` at 0x175408 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17540b | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x17540e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175411 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x175414 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x175417 | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x17541d | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x175420 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x175424 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175427 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x17542a | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x17542d | load | 4 | `[sp+4] +0x40` | `-` |
| `mov` at 0x175431 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x175434 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x17543c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17543f | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x175442 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x175445 | store | 1 | `[sp+4] +0x1a` | `0x7` |
| `mov` at 0x175449 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17544c | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x17544f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175452 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x175455 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x175458 | load | 1 | `[sp+4] +0x1a` | `-` |
| `mov` at 0x17545e | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x175461 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x175465 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175468 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x17546b | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x17546e | load | 4 | `[sp+4] +0x38` | `-` |
| `mov` at 0x175472 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x175475 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x17547d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175480 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x175483 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x175486 | store | 1 | `[sp+4] +0x1a` | `0x89` |
| `mov` at 0x17548a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17548d | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x175490 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175493 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x175496 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x175499 | load | 1 | `[sp+4] +0x1a` | `-` |
| `mov` at 0x17549f | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x1754a2 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1754a6 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1754a9 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x1754ac | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1754af | load | 4 | `[sp+4] +0x38` | `-` |
| `mov` at 0x1754b3 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1754b6 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x1754be | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1754c1 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x1754c4 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1754c7 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x1754ca | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1754cd | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x1754d4 | store | 1 | `sp -0xc` | `?` |
| `movsx` at 0x1754d7 | load | 1 | `sp -0xc` | `-` |
| `mov` at 0x1754dc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1754df | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x1754e2 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1754e5 | load | 4 | `[sp+4] +0x40` | `-` |
| `mov` at 0x1754e9 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1754ec | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x1754f4 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1754f7 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x1754fa | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1754fd | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x175500 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x175503 | load | 4 | `[sp+4] +0x64` | `-` |
| `mov` at 0x175506 | store | 4 | `sp -0xc` | `0x1b4e81b5` |
| `mov` at 0x17550d | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x175519 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x17551c | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x17551f | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x175522 | store | 4 | `[sp-8] +0x20` | `[sp-12]` |
| `mov` at 0x175525 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175528 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x17552b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17552e | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x175531 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x175534 | load | 4 | `[sp+4] +0x64` | `-` |
| `mov` at 0x175537 | store | 4 | `sp -0xc` | `0x1b4e81b5` |
| `mov` at 0x17553e | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x17554a | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x17554d | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x175553 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x175556 | store | 4 | `[sp-8] +0x24` | `?` |
| `mov` at 0x175559 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17555c | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x17555f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175562 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x175565 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x175568 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x17556b | store | 1 | `sp -0xc` | `?` |
| `movsx` at 0x17556e | load | 1 | `sp -0xc` | `-` |
| `mov` at 0x175573 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175576 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x175579 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x17557c | load | 4 | `[sp+4] +0x54` | `-` |
| `mov` at 0x175580 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x175583 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x17558b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17558e | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x175591 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175594 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x175597 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x17559a | load | 4 | `[sp+4] +0x24` | `-` |
| `mov` at 0x17559d | store | 1 | `sp -0xc` | `?` |
| `movsx` at 0x1755a0 | load | 1 | `sp -0xc` | `-` |
| `mov` at 0x1755a5 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1755a8 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x1755ab | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1755ae | load | 4 | `[sp+4] +0x58` | `-` |
| `mov` at 0x1755b2 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1755b5 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x1755bd | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1755c0 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x1755c3 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1755c6 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x1755c9 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1755cc | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x1755d2 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x1755d5 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1755d9 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1755dc | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x1755df | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1755e2 | load | 4 | `[sp+4] +0x40` | `-` |
| `mov` at 0x1755e6 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1755e9 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x1755f1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1755f4 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x1755f7 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1755fa | store | 1 | `[sp+4] +0x19` | `0x3` |
| `mov` at 0x1755fe | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175601 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x175606 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175609 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x17560c | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x17560f | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x175613 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x175616 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x17561e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175621 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x175624 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x175627 | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x17562a | store | 4 | `sp -0xc` | `[[sp+4]+104]` |
| `mov` at 0x17562d | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x175633 | store | 4 | `sp -0x8` | `?` |
| `cmp` at 0x175636 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x17563c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17563f | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x175642 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x175645 | store | 4 | `[sp+4] +0x1c` | `0xb` |
| `mov` at 0x175654 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175657 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x17565a | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x17565d | store | 4 | `[sp+4] +0x1c` | `0xb` |
| `mov` at 0x175664 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175667 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x17566a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17566d | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x175670 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x175673 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x17567a | store | 1 | `sp -0xc` | `?` |
| `mov` at 0x17567d | load | 1 | `sp -0xc` | `-` |
| `mov` at 0x175680 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x175683 | store | 1 | `[sp+4] +0x19` | `[sp-12]:1` |
| `mov` at 0x175686 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175689 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x17568c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17568f | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x175692 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x175695 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x17569c | store | 1 | `sp -0xc` | `?` |
| `mov` at 0x17569f | load | 1 | `sp -0xc` | `-` |
| `mov` at 0x1756a2 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1756a5 | store | 1 | `[sp+4] +0x19` | `[sp-12]:1` |
| `mov` at 0x1756a8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1756ab | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x1756ae | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1756b1 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x1756b4 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1756b7 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x1756ba | store | 1 | `sp -0xc` | `?` |
| `movsx` at 0x1756bd | load | 1 | `sp -0xc` | `-` |
| `mov` at 0x1756c2 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1756c5 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x1756c8 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1756cb | load | 4 | `[sp+4] +0x44` | `-` |
| `mov` at 0x1756cf | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1756d2 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x1756da | load | 4 | `sp -0x10` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x175437 | `(unknown callee) (unknown target)` | `[[sp+4]+64]` | `[sp-12]` | `?` | `[sp+4]` | `[sp+4]` | `?` |
| `call` at 0x175478 | `(unknown callee) (unknown target)` | `[[sp+4]+56]` | `[sp-12]` | `?` | `[sp+4]` | `[sp+4]` | `?` |
| `call` at 0x1754b9 | `(unknown callee) (unknown target)` | `[[sp+4]+56]` | `[sp-12]` | `?` | `[sp+4]` | `[sp+4]` | `?` |
| `call` at 0x1754ef | `(unknown callee) (unknown target)` | `[[sp+4]+64]` | `?` | `?` | `[sp+4]` | `[sp+4]` | `?` |
| `call` at 0x175586 | `(unknown callee) (unknown target)` | `[[sp+4]+84]` | `?` | `?` | `[sp+4]` | `[sp+4]` | `?` |
| `call` at 0x1755b8 | `(unknown callee) (unknown target)` | `[[sp+4]+88]` | `?` | `?` | `[sp+4]` | `[sp+4]` | `?` |
| `call` at 0x1755ec | `(unknown callee) (unknown target)` | `[[sp+4]+64]` | `[sp-12]` | `?` | `[sp+4]` | `[sp+4]` | `?` |
| `call` at 0x175619 | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `0x0` | `?` | `[sp+4]` | `[sp+4]` | `?` |
| `call` at 0x1756d5 | `(unknown callee) (unknown target)` | `[[sp+4]+68]` | `?` | `?` | `[sp+4]` | `[sp+4]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x17563a | `[sp-8] == 0x0` | `[sp-8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1756e0 | `eax` | `ret(unknown)` |

## `_ns16550Int`

- `_ns16550Int`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1760b4 to 0x176294.
  Established. The known direct-branch routes include indirect calls whose first candidates are
  `[[sp+4]+60]` and `[[sp+4]+52]` (`call` at 0x1760c8, `call` at 0x1760e6, `call` at 0x17627b). The
  unresolved computed jump leaves its dispatch cases unanalysed here (`jmp` at 0x176101).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1760bb | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1760be | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1760c1 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x1760c5 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x1760d4 | store | 1 | `sp -0x5` | `?` |
| `mov` at 0x1760d7 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1760dc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1760df | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x1760e3 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x1760eb | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x1760fa | load | 4 | `(unknown)` | `-` |
| `mov` at 0x176264 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x176267 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17626a | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x176271 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x176274 | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x176278 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x176280 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1760c8 | `(unknown callee) (unknown target)` | `[[sp+4]+60]` | `ebx` | `?` | `ebp` | `?` | `?` |
| `call` at 0x1760e6 | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x17627b | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ja` at 0x1760f4 | `? > 0xc (unsigned, 32-bit)` | `? <= 0xc (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x176286 | `eax` | `ret(unknown)` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x176101).

## `_ns16550IntEx`

- `_ns16550IntEx`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1760a4 to 0x1760b4.
  Established. The return records the EAX expression `eax` (`ret` at 0x1760aa). Stack restoration
  and the EBP pop immediately precede it (`mov` at 0x1760a7, `pop` at 0x1760a9).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1760aa | `eax` | `eax` |

## `_ns16550Ioctl`

- `_ns16550Ioctl`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x175b54 to 0x175fe4.
  Established. The explicit local four-byte store carries `[sp+8]-4099` at `sp -0x8` (`mov` at
  0x175b67). The taken `ja` path for `[sp+8]-4099 > 0xa (unsigned, 32-bit)` reaches the recorded EAX
  return value 0x47 (`ja` at 0x175b6e, `ret` at 0x175fdd); its fall-through reaches an unresolved
  computed jump (`ja` at 0x175b6e, `jmp` at 0x175b81).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x175b5f | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x175b67 | store | 4 | `sp -0x8` | `[sp+8]-4099` |
| `cmp` at 0x175b6a | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x175b74 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x175b77 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x175b7e | store | 4 | `sp -0x8` | `?` |
| `jmp` at 0x175b81 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ja` at 0x175b6e | `[sp+8]-4099 > 0xa (unsigned, 32-bit)` | `[sp+8]-4099 <= 0xa (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x175fdd | `eax` | `0x47` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x175b81).

## `_ns16550Open`

- `_ns16550Open`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x175a54 to 0x175b54.
  Established. The first indirect call receives `[[sp+4]+76]` as its first candidate (`call` at
  0x175a69). Two byte stores through `[sp+4] +0x19` and a four-byte store through `[sp+4] +0x1c`
  carry unresolved values (`mov` at 0x175a9b, `mov` at 0x175aac, `mov` at 0x175abd).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x175a5c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175a5f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175a62 | load | 4 | `[sp+4] +0x4c` | `-` |
| `mov` at 0x175a66 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x175a75 | store | 1 | `sp -0x5` | `?` |
| `mov` at 0x175a78 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x175a8a | store | 4 | `sp -0xc` | `ret(_intLock)` |
| `mov` at 0x175a8d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175a90 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175a93 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x175a9b | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x175a9e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175aa1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175aa4 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x175aac | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x175aaf | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175ab2 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175ab5 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x175abd | store | 4 | `[sp+4] +0x1c` | `?` |
| `mov` at 0x175ac0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175ac3 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175ac6 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x175acf | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175ad2 | load | 4 | `[sp+4] +0x44` | `-` |
| `mov` at 0x175ad6 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x175ade | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175ae1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175ae4 | load | 1 | `[sp+4] +0x1a` | `-` |
| `mov` at 0x175aec | store | 1 | `[sp+4] +0x1a` | `?` |
| `mov` at 0x175aef | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175af2 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175af5 | load | 1 | `[sp+4] +0x1a` | `-` |
| `mov` at 0x175afc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175aff | load | 4 | `[sp+4] +0x38` | `-` |
| `mov` at 0x175b03 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x175b0b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175b0e | load | 2 | `[sp+4] +0x5c` | `-` |
| `mov` at 0x175b18 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175b1b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175b1e | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x175b25 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175b28 | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x175b2c | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x175b34 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x175a69 | `(unknown callee) (unknown target)` | `[[sp+4]+76]` | `ebx` | `esi` | `?` | `?` | `ebp` |
| `call` at 0x175a85 | `_intLock 0x1713d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x175ad9 | `(unknown callee) (unknown target)` | `[[sp+4]+68]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x175b06 | `(unknown callee) (unknown target)` | `[[sp+4]+56]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x175b2f | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x175b38 | `_intUnlock 0x1713e0` | `[sp-12]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x175a7f | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `jne` at 0x175b16 | `([[sp+4]+92]:2 & 0xffff) != 0x2` | `([[sp+4]+92]:2 & 0xffff) == 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x175b4c | `eax` | `0x0` |

## `_ns16550OptsSet`

- `_ns16550OptsSet`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1756e4 to 0x175974.
  Established. Separate byte-store sites through `[sp+4] +0x1b` carry zero, 1, 2 and 3 (`mov` at
  0x175747, `mov` at 0x175757, `mov` at 0x175767, `mov` at 0x175777). A later byte store writes 0x87
  through `[sp+4] +0x1a`, followed by a four-byte store of `[sp+8]` through `[sp+4] +0x68` (`mov` at
  0x1758e9, `mov` at 0x17595c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x1756e9 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1756ef | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x175714 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x175744 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175747 | store | 1 | `[sp+4] +0x1b` | `0x0` |
| `mov` at 0x175754 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175757 | store | 1 | `[sp+4] +0x1b` | `0x1` |
| `mov` at 0x175764 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175767 | store | 1 | `[sp+4] +0x1b` | `0x2` |
| `mov` at 0x175774 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175777 | store | 1 | `[sp+4] +0x1b` | `0x3` |
| `mov` at 0x175784 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x17578e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175791 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175794 | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x17579c | store | 1 | `[sp+4] +0x1b` | `?` |
| `mov` at 0x1757a4 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1757a7 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1757aa | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x1757ad | store | 1 | `[sp+4] +0x1b` | `[[sp+4]+27]:1` |
| `mov` at 0x1757b0 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1757e4 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1757e7 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1757ea | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x1757f2 | store | 1 | `[sp+4] +0x1b` | `?` |
| `mov` at 0x175804 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175807 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17580a | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x175812 | store | 1 | `[sp+4] +0x1b` | `?` |
| `mov` at 0x175824 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175827 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17582a | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x17582d | store | 1 | `[sp+4] +0x1b` | `[[sp+4]+27]:1` |
| `mov` at 0x175834 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175839 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17583c | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x175840 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x175848 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17584b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17584e | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x175856 | store | 4 | `[sp+4] +0x1c` | `?` |
| `mov` at 0x175859 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17585c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17585f | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x175867 | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x17586a | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x175874 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175877 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17587a | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x175882 | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x175894 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175897 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17589a | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x1758a2 | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x1758ac | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1758af | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1758b2 | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x1758b9 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1758bc | load | 4 | `[sp+4] +0x40` | `-` |
| `mov` at 0x1758c0 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x1758c8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1758cb | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1758ce | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x1758d7 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1758da | load | 4 | `[sp+4] +0x44` | `-` |
| `mov` at 0x1758de | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x1758e6 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1758e9 | store | 1 | `[sp+4] +0x1a` | `0x87` |
| `mov` at 0x1758ed | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1758f0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1758f3 | load | 1 | `[sp+4] +0x1a` | `-` |
| `mov` at 0x1758fa | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1758fd | load | 4 | `[sp+4] +0x38` | `-` |
| `mov` at 0x175901 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x175909 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x175913 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175916 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175919 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x175921 | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x175924 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175927 | load | 2 | `[sp+4] +0x5c` | `-` |
| `mov` at 0x175931 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175934 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175937 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x17593e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175941 | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x175945 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x175956 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x175959 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x17595c | store | 4 | `[sp+4] +0x68` | `[sp+8]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x175843 | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x1758a5 | `_intLock 0x1713d0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1758c3 | `(unknown callee) (unknown target)` | `[[sp+4]+64]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1758e1 | `(unknown callee) (unknown target)` | `[[sp+4]+68]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x175904 | `(unknown callee) (unknown target)` | `[[sp+4]+56]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x175948 | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x17594e | `_intUnlock 0x1713e0` | `ret(_intLock)` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1756ed | `[sp+4] == 0x0` | `[sp+4] != 0x0` |
| `jne` at 0x1756f9 | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `je` at 0x17571d | `? == 0x4` | `? != 0x4` |
| `ja` at 0x175722 | `? > 0x4 (unsigned, 32-bit)` | `? <= 0x4 (unsigned, 32-bit)` |
| `je` at 0x175726 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x175737 | `? == 0x8` | `? != 0x8` |
| `je` at 0x17573c | `? == 0xc` | `? != 0xc` |
| `je` at 0x17578c | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x1757bb | `? == 0x40` | `? != 0x40` |
| `ja` at 0x1757c0 | `? > 0x40 (unsigned, 32-bit)` | `? <= 0x40 (unsigned, 32-bit)` |
| `je` at 0x1757c4 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x1757d9 | `? == 0xc0` | `? != 0xc0` |
| `jne` at 0x175872 | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `je` at 0x175911 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `jne` at 0x17592f | `([[sp+4]+92]:2 & 0xffff) != 0x2` | `([[sp+4]+92]:2 & 0xffff) == 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x17596c | `eax` | `?` |

## `_ns16550TxStartup`

- `_ns16550TxStartup`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x176294 to 0x176354.
  Established. The `jne` selects the return-tail route when `([[sp+4]+92]:2 & 0xffff) != 0x2` (`jne`
  at 0x1762a6). On its fall-through region, indirect call candidates begin `[[sp+4]+76]` and later
  `[[sp+4]+52]` at separate sites (`jne` at 0x1762a6, `call` at 0x1762e1, `call` at 0x17633c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x17629b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17629e | load | 2 | `[sp+4] +0x5c` | `-` |
| `mov` at 0x1762ac | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1762af | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x1762bb | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1762be | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1762c1 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x1762c9 | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x1762d4 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1762d7 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1762da | load | 4 | `[sp+4] +0x4c` | `-` |
| `mov` at 0x1762de | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x1762e6 | store | 1 | `sp -0x5` | `?` |
| `mov` at 0x1762e9 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x1762f2 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1762f5 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1762f8 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x176300 | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x176314 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x176317 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17631a | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x176322 | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x176325 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x176328 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17632b | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x176332 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x176335 | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x176339 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x176341 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1762e1 | `(unknown callee) (unknown target)` | `[[sp+4]+76]` | `ebx` | `?` | `ebp` | `?` | `?` |
| `call` at 0x17633c | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x1762a6 | `([[sp+4]+92]:2 & 0xffff) != 0x2` | `([[sp+4]+92]:2 & 0xffff) == 0x2` |
| `je` at 0x1762b9 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x1762f0 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x176347 | `eax` | `?` |

## `_sysA20on`

- `_sysA20on`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108100 to 0x1081a0.
  Established. Byte port writes carry 0xd1 at port 0x64, 0xdf at port 0x60, 0xff at port 0x64 and 2
  at port 0x92 at distinct sites (`out` at 0x10810a, `out` at 0x108116, `out` at 0x108122, `out` at
  0x108152). The two `jne` sites each take the return route when `[0x100000] != 0x1234567`, with
  equality falling through (`jne` at 0x10814b, `jne` at 0x10817e).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `out` at 0x10810a | store | 1 | `I/O abs 0x64` | `0xd1` |
| `out` at 0x108116 | store | 1 | `I/O abs 0x60` | `0xdf` |
| `out` at 0x108122 | store | 1 | `I/O abs 0x64` | `0xff` |
| `mov` at 0x108133 | store | 4 | `abs 0x0` | `0x0` |
| `mov` at 0x108139 | store | 4 | `abs 0x100000` | `0x0` |
| `mov` at 0x10813f | store | 4 | `abs 0x0` | `0x1234567` |
| `cmp` at 0x108145 | load | 4 | `abs 0x100000` | `-` |
| `out` at 0x108152 | store | 1 | `I/O abs 0x92` | `0x2` |
| `in` at 0x108156 | load | 1 | `I/O abs 0x92` | `-` |
| `mov` at 0x108166 | store | 4 | `abs 0x0` | `0x0` |
| `mov` at 0x10816c | store | 4 | `abs 0x100000` | `0x0` |
| `mov` at 0x108172 | store | 4 | `abs 0x0` | `0x1234567` |
| `cmp` at 0x108178 | load | 4 | `abs 0x100000` | `-` |
| `mov` at 0x108180 | store | 4 | `abs 0x108190` | `0xffffffff` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108100 | `_sysWait 0x1081a0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10810c | `_sysWait 0x1081a0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108118 | `_sysWait 0x1081a0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108124 | `_sysWait 0x1081a0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10814b | `[0x100000] != 0x1234567` | `[0x100000] == 0x1234567` |
| `loope` at 0x10815a | `oope flags condition (operands unknown)` | `not (oope flags condition (operands unknown))` |
| `jne` at 0x10817e | `[0x100000] != 0x1234567` | `[0x100000] == 0x1234567` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10818a | `eax` | `0x0` |

## `_sysInByte`

- `_sysInByte`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081b0 to 0x1081c0.
  Established. The input site reads one byte from an unresolved port, leaving its value unknown
  (`in` at 0x1081b9); its prior four-byte memory read renders as `sp +0x4` (`mov` at 0x1081b0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1081b0 | load | 4 | `sp +0x4` | `-` |
| `in` at 0x1081b9 | load | 1 | `I/O (unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1081bc | `eax` | `?` |

## `_sysInWord`

- `_sysInWord`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081c0 to 0x1081d0.
  Established. The two-byte port input has an unknown port and value (`in` at 0x1081c9). EAX at the
  recorded exit remains unresolved (`ret` at 0x1081cd).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1081c0 | load | 4 | `sp +0x4` | `-` |
| `in` at 0x1081c9 | load | 2 | `I/O (unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1081cd | `eax` | `?` |

## `_sysOutByte`

- `_sysOutByte`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081d0 to 0x1081e0.
  Established. The output site writes one byte to an unresolved port and has an unknown stored value
  (`out` at 0x1081d8). The return retains `[sp+8]` in EAX (`ret` at 0x1081db).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1081d0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1081d4 | load | 4 | `sp +0x8` | `-` |
| `out` at 0x1081d8 | store | 1 | `I/O (unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1081db | `eax` | `[sp+8]` |

## `_sysOutWord`

- `_sysOutWord`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081e0 to 0x1081f0.
  Established. Two four-byte memory reads render as `sp +0x4` and `sp +0x8` (`mov` at 0x1081e0,
  `mov` at 0x1081e4). The following two-byte port output has unresolved port and stored value (`out`
  at 0x1081e8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1081e0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1081e4 | load | 4 | `sp +0x8` | `-` |
| `out` at 0x1081e8 | store | 2 | `I/O (unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1081ec | `eax` | `[sp+8]` |
