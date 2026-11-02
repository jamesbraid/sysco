# MMSC 2.2 serial-driver and port routines

Read from `6.5.6/usr/cpu/firmware/mmscfw.bin` (IRIX 6.5.6, MMSC 2.2).
Image SHA-256: `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol, or the next global text symbol across local
labels, as an inferred end and can include literals
after returns. Statements below concern sites reached by known control-flow edges.
Unresolved computed jumps make the effects incomplete for the affected routines.
I/O prefixes in the access table identify ports rather than memory addresses.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include I/O ports and explicit memory operands, including stack records.
Implicit stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/serial-2.2-facts.txt) include all tool sections.

## `_ns16550CallbackInstall`

- `_ns16550CallbackInstall`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x17a18c to 0x17a200.
  Established. The first `je` selects the stores of `[sp+12]` through `[sp+4] +0x4` and `[sp+16]`
  through `[sp-8] +0xc` for `[sp+8] == 0x1` (`je` at 0x17a19e, `mov` at 0x17a1b2, `mov` at
  0x17a1bb). The second `je` selects the stores of `[sp+12]` through `[sp+4] +0x8` and `[sp+16]`
  through `[sp-8] +0x10` for `[sp+8] == 0x2` (`je` at 0x17a1a3, `mov` at 0x17a1d2, `mov` at
  0x17a1db).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x17a192 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a195 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x17a198 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x17a1ac | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x17a1af | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x17a1b2 | store | 4 | `[sp+4] +0x4` | `[sp+12]` |
| `mov` at 0x17a1b5 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x17a1b8 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x17a1bb | store | 4 | `[sp-8] +0xc` | `[sp+16]` |
| `mov` at 0x17a1cc | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x17a1cf | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x17a1d2 | store | 4 | `[sp+4] +0x8` | `[sp+12]` |
| `mov` at 0x17a1d5 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x17a1d8 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x17a1db | store | 4 | `[sp-8] +0x10` | `[sp+16]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x17a19e | `[sp+8] == 0x1` | `[sp+8] != 0x1` |
| `je` at 0x17a1a3 | `[sp+8] == 0x2` | `[sp+8] != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x17a1ff | `eax` | `?` |

## `_ns16550DummyCallback`

- `_ns16550DummyCallback`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1790dc to 0x1790fc.
  Established. The recorded EAX return value is 0xffffffff (`ret` at 0x1790ef). The all-ones value
  is loaded before the jump to that return tail (`mov` at 0x1790df, `jmp` at 0x1790e4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1790ef | `eax` | `0xffffffff` |

## `_ns16550Hup`

- `_ns16550Hup`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1796dc to 0x1797bc.
  Established. The first indirect call has candidates beginning `[[sp+4]+52]` and zero (`call` at
  0x1796f7). Byte store sites through `[sp+4] +0x19` and `[sp+4] +0x1a`, and a four-byte site
  through `[sp+4] +0x1c`, have values left unknown (`mov` at 0x17970a, `mov` at 0x17974a, `mov` at
  0x17971b).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1796e8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1796ed | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1796f0 | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x1796f4 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x1796fc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1796ff | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179702 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x17970a | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x17970d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179710 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179713 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x17971b | store | 4 | `[sp+4] +0x1c` | `?` |
| `mov` at 0x17971e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179721 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179724 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x17972d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179730 | load | 4 | `[sp+4] +0x44` | `-` |
| `mov` at 0x179734 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x17973c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17973f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179742 | load | 1 | `[sp+4] +0x1a` | `-` |
| `mov` at 0x17974a | store | 1 | `[sp+4] +0x1a` | `?` |
| `mov` at 0x17974d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179750 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179753 | load | 1 | `[sp+4] +0x1a` | `-` |
| `mov` at 0x17975a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17975d | load | 4 | `[sp+4] +0x38` | `-` |
| `mov` at 0x179761 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x179769 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17976c | load | 2 | `[sp+4] +0x5c` | `-` |
| `mov` at 0x179776 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179779 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17977c | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x179783 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179786 | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x17978a | load | 4 | `[sp+4] +0x2c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1796e1 | `_intLock 0x175138` | `ebx` | `esi` | `ebp` | `?` | `?` | `?` |
| `call` at 0x1796f7 | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x179737 | `(unknown callee) (unknown target)` | `[[sp+4]+68]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x179764 | `(unknown callee) (unknown target)` | `[[sp+4]+56]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x17978d | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x179793 | `_intUnlock 0x175148` | `ret(_intLock)` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x179774 | `([[sp+4]+92]:2 & 0xffff) != 0x2` | `([[sp+4]+92]:2 & 0xffff) == 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1797b4 | `eax` | `0x0` |

## `_ns16550InitChannel`

- `_ns16550InitChannel`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x17915c to 0x17944c.
  Established. Explicit byte stores place 3 through `[sp+4] +0x1b`, then 7 and 0x89 through `[sp+4]
  +0x1a` at distinct sites (`mov` at 0x17916c, `mov` at 0x1791ad, `mov` at 0x1791ee). The later
  indirect call has first candidates `[[sp+4]+52]` and zero (`call` at 0x179381).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x179163 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179166 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x179169 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x17916c | store | 1 | `[sp+4] +0x1b` | `0x3` |
| `mov` at 0x179170 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179173 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x179176 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179179 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x17917c | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x17917f | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x179185 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x179188 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x17918c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17918f | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x179192 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x179195 | load | 4 | `[sp+4] +0x40` | `-` |
| `mov` at 0x179199 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x17919c | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x1791a4 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1791a7 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x1791aa | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1791ad | store | 1 | `[sp+4] +0x1a` | `0x7` |
| `mov` at 0x1791b1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1791b4 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x1791b7 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1791ba | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x1791bd | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1791c0 | load | 1 | `[sp+4] +0x1a` | `-` |
| `mov` at 0x1791c6 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x1791c9 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1791cd | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1791d0 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x1791d3 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1791d6 | load | 4 | `[sp+4] +0x38` | `-` |
| `mov` at 0x1791da | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1791dd | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x1791e5 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1791e8 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x1791eb | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1791ee | store | 1 | `[sp+4] +0x1a` | `0x89` |
| `mov` at 0x1791f2 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1791f5 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x1791f8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1791fb | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x1791fe | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x179201 | load | 1 | `[sp+4] +0x1a` | `-` |
| `mov` at 0x179207 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x17920a | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x17920e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179211 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x179214 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x179217 | load | 4 | `[sp+4] +0x38` | `-` |
| `mov` at 0x17921b | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x17921e | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x179226 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179229 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x17922c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17922f | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x179232 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x179235 | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x17923c | store | 1 | `sp -0xc` | `?` |
| `movsx` at 0x17923f | load | 1 | `sp -0xc` | `-` |
| `mov` at 0x179244 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179247 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x17924a | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x17924d | load | 4 | `[sp+4] +0x40` | `-` |
| `mov` at 0x179251 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x179254 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x17925c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17925f | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x179262 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179265 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x179268 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x17926b | load | 4 | `[sp+4] +0x64` | `-` |
| `mov` at 0x17926e | store | 4 | `sp -0xc` | `0x1b4e81b5` |
| `mov` at 0x179275 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x179281 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x179284 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x179287 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x17928a | store | 4 | `[sp-8] +0x20` | `[sp-12]` |
| `mov` at 0x17928d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179290 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x179293 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179296 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x179299 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x17929c | load | 4 | `[sp+4] +0x64` | `-` |
| `mov` at 0x17929f | store | 4 | `sp -0xc` | `0x1b4e81b5` |
| `mov` at 0x1792a6 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1792b2 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x1792b5 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1792bb | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1792be | store | 4 | `[sp-8] +0x24` | `?` |
| `mov` at 0x1792c1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1792c4 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x1792c7 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1792ca | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x1792cd | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1792d0 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x1792d3 | store | 1 | `sp -0xc` | `?` |
| `movsx` at 0x1792d6 | load | 1 | `sp -0xc` | `-` |
| `mov` at 0x1792db | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1792de | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x1792e1 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1792e4 | load | 4 | `[sp+4] +0x54` | `-` |
| `mov` at 0x1792e8 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1792eb | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x1792f3 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1792f6 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x1792f9 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1792fc | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x1792ff | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x179302 | load | 4 | `[sp+4] +0x24` | `-` |
| `mov` at 0x179305 | store | 1 | `sp -0xc` | `?` |
| `movsx` at 0x179308 | load | 1 | `sp -0xc` | `-` |
| `mov` at 0x17930d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179310 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x179313 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x179316 | load | 4 | `[sp+4] +0x58` | `-` |
| `mov` at 0x17931a | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x17931d | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x179325 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179328 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x17932b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17932e | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x179331 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x179334 | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x17933a | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x17933d | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x179341 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179344 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x179347 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x17934a | load | 4 | `[sp+4] +0x40` | `-` |
| `mov` at 0x17934e | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x179351 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x179359 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17935c | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x17935f | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x179362 | store | 1 | `[sp+4] +0x19` | `0x3` |
| `mov` at 0x179366 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179369 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x17936e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179371 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x179374 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x179377 | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x17937b | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x17937e | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x179386 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179389 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x17938c | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x17938f | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x179392 | store | 4 | `sp -0xc` | `[[sp+4]+104]` |
| `mov` at 0x179395 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x17939b | store | 4 | `sp -0x8` | `?` |
| `cmp` at 0x17939e | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1793a4 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1793a7 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x1793aa | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1793ad | store | 4 | `[sp+4] +0x1c` | `0xb` |
| `mov` at 0x1793bc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1793bf | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x1793c2 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1793c5 | store | 4 | `[sp+4] +0x1c` | `0xb` |
| `mov` at 0x1793cc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1793cf | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x1793d2 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1793d5 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x1793d8 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1793db | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x1793e2 | store | 1 | `sp -0xc` | `?` |
| `mov` at 0x1793e5 | load | 1 | `sp -0xc` | `-` |
| `mov` at 0x1793e8 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1793eb | store | 1 | `[sp+4] +0x19` | `[sp-12]:1` |
| `mov` at 0x1793ee | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1793f1 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x1793f4 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1793f7 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x1793fa | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1793fd | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x179404 | store | 1 | `sp -0xc` | `?` |
| `mov` at 0x179407 | load | 1 | `sp -0xc` | `-` |
| `mov` at 0x17940a | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x17940d | store | 1 | `[sp+4] +0x19` | `[sp-12]:1` |
| `mov` at 0x179410 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179413 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x179416 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179419 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x17941c | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x17941f | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x179422 | store | 1 | `sp -0xc` | `?` |
| `movsx` at 0x179425 | load | 1 | `sp -0xc` | `-` |
| `mov` at 0x17942a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17942d | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x179430 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x179433 | load | 4 | `[sp+4] +0x44` | `-` |
| `mov` at 0x179437 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x17943a | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x179442 | load | 4 | `sp -0x10` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x17919f | `(unknown callee) (unknown target)` | `[[sp+4]+64]` | `[sp-12]` | `?` | `[sp+4]` | `[sp+4]` | `?` |
| `call` at 0x1791e0 | `(unknown callee) (unknown target)` | `[[sp+4]+56]` | `[sp-12]` | `?` | `[sp+4]` | `[sp+4]` | `?` |
| `call` at 0x179221 | `(unknown callee) (unknown target)` | `[[sp+4]+56]` | `[sp-12]` | `?` | `[sp+4]` | `[sp+4]` | `?` |
| `call` at 0x179257 | `(unknown callee) (unknown target)` | `[[sp+4]+64]` | `?` | `?` | `[sp+4]` | `[sp+4]` | `?` |
| `call` at 0x1792ee | `(unknown callee) (unknown target)` | `[[sp+4]+84]` | `?` | `?` | `[sp+4]` | `[sp+4]` | `?` |
| `call` at 0x179320 | `(unknown callee) (unknown target)` | `[[sp+4]+88]` | `?` | `?` | `[sp+4]` | `[sp+4]` | `?` |
| `call` at 0x179354 | `(unknown callee) (unknown target)` | `[[sp+4]+64]` | `[sp-12]` | `?` | `[sp+4]` | `[sp+4]` | `?` |
| `call` at 0x179381 | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `0x0` | `?` | `[sp+4]` | `[sp+4]` | `?` |
| `call` at 0x17943d | `(unknown callee) (unknown target)` | `[[sp+4]+68]` | `?` | `?` | `[sp+4]` | `[sp+4]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1793a2 | `[sp-8] == 0x0` | `[sp-8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x179448 | `eax` | `ret(unknown)` |

## `_ns16550Int`

- `_ns16550Int`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x179e1c to 0x179ffc.
  Established. The known paths include indirect calls with first candidates `[[sp+4]+60]` and
  `[[sp+4]+52]` (`call` at 0x179e30, `call` at 0x179e4e, `call` at 0x179fe3). The computed jump
  target remains unresolved, so the effects table does not describe all dispatch cases (`jmp` at
  0x179e69).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x179e23 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179e26 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179e29 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x179e2d | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x179e3c | store | 1 | `sp -0x5` | `?` |
| `mov` at 0x179e3f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179e44 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179e47 | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x179e4b | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x179e53 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x179e62 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x179fcc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179fcf | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179fd2 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x179fd9 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179fdc | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x179fe0 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x179fe8 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x179e30 | `(unknown callee) (unknown target)` | `[[sp+4]+60]` | `ebx` | `?` | `ebp` | `?` | `?` |
| `call` at 0x179e4e | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x179fe3 | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ja` at 0x179e5c | `? > 0xc (unsigned, 32-bit)` | `? <= 0xc (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x179fee | `eax` | `ret(unknown)` |

Known-path coverage stops at the unresolved transfer (`jmp` at 0x179e69).

## `_ns16550IntEx`

- `_ns16550IntEx`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x179e0c to 0x179e1c.
  Established. The recorded return preserves the entry EAX expression `eax` (`ret` at 0x179e12). The
  body restores the stack pointer and pops EBP before that return (`mov` at 0x179e0f, `pop` at
  0x179e11).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x179e12 | `eax` | `eax` |

## `_ns16550Ioctl`

- `_ns16550Ioctl`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1798bc to 0x179d4c.
  Established. The explicit local store places `[sp+8]-4099` at `sp -0x8` (`mov` at 0x1798cf). `ja`
  takes the known out-of-range path for `[sp+8]-4099 > 0xa (unsigned, 32-bit)` (`ja` at 0x1798d6).
  The other route reaches an unresolved computed jump, so its cases remain outside these known-path
  effects (`jmp` at 0x1798e9).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1798c7 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1798cf | store | 4 | `sp -0x8` | `[sp+8]-4099` |
| `cmp` at 0x1798d2 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1798dc | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1798df | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1798e6 | store | 4 | `sp -0x8` | `?` |
| `jmp` at 0x1798e9 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ja` at 0x1798d6 | `[sp+8]-4099 > 0xa (unsigned, 32-bit)` | `[sp+8]-4099 <= 0xa (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x179d45 | `eax` | `0x47` |

Known-path coverage stops at the unresolved transfer (`jmp` at 0x1798e9).

## `_ns16550Open`

- `_ns16550Open`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1797bc to 0x1798bc.
  Established. The initial indirect call has first candidate `[[sp+4]+76]` (`call` at 0x1797d1). The
  later `jne` bypasses another indirect call for `([[sp+4]+92]:2 & 0xffff) != 0x2`, while equality
  falls through to it (`jne` at 0x17987e, `call` at 0x179897).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1797c4 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1797c7 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1797ca | load | 4 | `[sp+4] +0x4c` | `-` |
| `mov` at 0x1797ce | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x1797dd | store | 1 | `sp -0x5` | `?` |
| `mov` at 0x1797e0 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x1797f2 | store | 4 | `sp -0xc` | `ret(_intLock)` |
| `mov` at 0x1797f5 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1797f8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1797fb | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x179803 | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x179806 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179809 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17980c | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x179814 | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x179817 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17981a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17981d | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x179825 | store | 4 | `[sp+4] +0x1c` | `?` |
| `mov` at 0x179828 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17982b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17982e | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x179837 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17983a | load | 4 | `[sp+4] +0x44` | `-` |
| `mov` at 0x17983e | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x179846 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179849 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17984c | load | 1 | `[sp+4] +0x1a` | `-` |
| `mov` at 0x179854 | store | 1 | `[sp+4] +0x1a` | `?` |
| `mov` at 0x179857 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17985a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17985d | load | 1 | `[sp+4] +0x1a` | `-` |
| `mov` at 0x179864 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179867 | load | 4 | `[sp+4] +0x38` | `-` |
| `mov` at 0x17986b | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x179873 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179876 | load | 2 | `[sp+4] +0x5c` | `-` |
| `mov` at 0x179880 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179883 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179886 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x17988d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179890 | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x179894 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x17989c | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1797d1 | `(unknown callee) (unknown target)` | `[[sp+4]+76]` | `ebx` | `esi` | `?` | `?` | `ebp` |
| `call` at 0x1797ed | `_intLock 0x175138` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x179841 | `(unknown callee) (unknown target)` | `[[sp+4]+68]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x17986e | `(unknown callee) (unknown target)` | `[[sp+4]+56]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x179897 | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1798a0 | `_intUnlock 0x175148` | `[sp-12]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1797e7 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `jne` at 0x17987e | `([[sp+4]+92]:2 & 0xffff) != 0x2` | `([[sp+4]+92]:2 & 0xffff) == 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1798b4 | `eax` | `0x0` |

## `_ns16550OptsSet`

- `_ns16550OptsSet`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x17944c to 0x1796dc.
  Established. Four byte-store sites through `[sp+4] +0x1b` carry values zero, 1, 2 and 3 on
  separate paths (`mov` at 0x1794af, `mov` at 0x1794bf, `mov` at 0x1794cf, `mov` at 0x1794df). A
  later byte store writes 0x87 through `[sp+4] +0x1a`, and the final four-byte store copies `[sp+8]`
  through `[sp+4] +0x68` (`mov` at 0x179651, `mov` at 0x1796c4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x179451 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179457 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x17947c | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1794ac | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1794af | store | 1 | `[sp+4] +0x1b` | `0x0` |
| `mov` at 0x1794bc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1794bf | store | 1 | `[sp+4] +0x1b` | `0x1` |
| `mov` at 0x1794cc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1794cf | store | 1 | `[sp+4] +0x1b` | `0x2` |
| `mov` at 0x1794dc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1794df | store | 1 | `[sp+4] +0x1b` | `0x3` |
| `mov` at 0x1794ec | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1794f6 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1794f9 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1794fc | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x179504 | store | 1 | `[sp+4] +0x1b` | `?` |
| `mov` at 0x17950c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17950f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179512 | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x179515 | store | 1 | `[sp+4] +0x1b` | `[[sp+4]+27]:1` |
| `mov` at 0x179518 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x17954c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17954f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179552 | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x17955a | store | 1 | `[sp+4] +0x1b` | `?` |
| `mov` at 0x17956c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17956f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179572 | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x17957a | store | 1 | `[sp+4] +0x1b` | `?` |
| `mov` at 0x17958c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17958f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179592 | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x179595 | store | 1 | `[sp+4] +0x1b` | `[[sp+4]+27]:1` |
| `mov` at 0x17959c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1795a1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1795a4 | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x1795a8 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x1795b0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1795b3 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1795b6 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x1795be | store | 4 | `[sp+4] +0x1c` | `?` |
| `mov` at 0x1795c1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1795c4 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1795c7 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x1795cf | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x1795d2 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1795dc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1795df | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1795e2 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x1795ea | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x1795fc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1795ff | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179602 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x17960a | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x179614 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179617 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17961a | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x179621 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179624 | load | 4 | `[sp+4] +0x40` | `-` |
| `mov` at 0x179628 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x179630 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179633 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179636 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x17963f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179642 | load | 4 | `[sp+4] +0x44` | `-` |
| `mov` at 0x179646 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x17964e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179651 | store | 1 | `[sp+4] +0x1a` | `0x87` |
| `mov` at 0x179655 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179658 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17965b | load | 1 | `[sp+4] +0x1a` | `-` |
| `mov` at 0x179662 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179665 | load | 4 | `[sp+4] +0x38` | `-` |
| `mov` at 0x179669 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x179671 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x17967b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17967e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179681 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x179689 | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x17968c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17968f | load | 2 | `[sp+4] +0x5c` | `-` |
| `mov` at 0x179699 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17969c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17969f | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x1796a6 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1796a9 | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x1796ad | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x1796be | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1796c1 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1796c4 | store | 4 | `[sp+4] +0x68` | `[sp+8]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1795ab | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x17960d | `_intLock 0x175138` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x17962b | `(unknown callee) (unknown target)` | `[[sp+4]+64]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x179649 | `(unknown callee) (unknown target)` | `[[sp+4]+68]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x17966c | `(unknown callee) (unknown target)` | `[[sp+4]+56]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1796b0 | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1796b6 | `_intUnlock 0x175148` | `ret(_intLock)` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x179455 | `[sp+4] == 0x0` | `[sp+4] != 0x0` |
| `jne` at 0x179461 | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `je` at 0x179485 | `? == 0x4` | `? != 0x4` |
| `ja` at 0x17948a | `? > 0x4 (unsigned, 32-bit)` | `? <= 0x4 (unsigned, 32-bit)` |
| `je` at 0x17948e | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x17949f | `? == 0x8` | `? != 0x8` |
| `je` at 0x1794a4 | `? == 0xc` | `? != 0xc` |
| `je` at 0x1794f4 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x179523 | `? == 0x40` | `? != 0x40` |
| `ja` at 0x179528 | `? > 0x40 (unsigned, 32-bit)` | `? <= 0x40 (unsigned, 32-bit)` |
| `je` at 0x17952c | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x179541 | `? == 0xc0` | `? != 0xc0` |
| `jne` at 0x1795da | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `je` at 0x179679 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `jne` at 0x179697 | `([[sp+4]+92]:2 & 0xffff) != 0x2` | `([[sp+4]+92]:2 & 0xffff) == 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1796d4 | `eax` | `?` |

## `_ns16550TxStartup`

- `_ns16550TxStartup`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x179ffc to 0x17a0bc.
  Established. The deciding `jne` bypasses the body region when `([[sp+4]+92]:2 & 0xffff) != 0x2`
  (`jne` at 0x17a00e). The indirect call later in that region has first candidate `[[sp+4]+52]` and
  unknown second candidate (`call` at 0x17a0a4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x17a003 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a006 | load | 2 | `[sp+4] +0x5c` | `-` |
| `mov` at 0x17a014 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a017 | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x17a023 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a026 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a029 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x17a031 | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x17a03c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a03f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a042 | load | 4 | `[sp+4] +0x4c` | `-` |
| `mov` at 0x17a046 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x17a04e | store | 1 | `sp -0x5` | `?` |
| `mov` at 0x17a051 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x17a05a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a05d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a060 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x17a068 | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x17a07c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a07f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a082 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x17a08a | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x17a08d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a090 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a093 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x17a09a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a09d | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x17a0a1 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x17a0a9 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x17a049 | `(unknown callee) (unknown target)` | `[[sp+4]+76]` | `ebx` | `?` | `ebp` | `?` | `?` |
| `call` at 0x17a0a4 | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x17a00e | `([[sp+4]+92]:2 & 0xffff) != 0x2` | `([[sp+4]+92]:2 & 0xffff) == 0x2` |
| `je` at 0x17a021 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x17a058 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x17a0af | `eax` | `?` |

## `_sysA20on`

- `_sysA20on`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108100 to 0x1081a0.
  Established. The byte port writes carry 0xd1 to 0x64, 0xdf to 0x60, 0xff to 0x64 and 2 to 0x92 at
  separate sites (`out` at 0x10810a, `out` at 0x108116, `out` at 0x108122, `out` at 0x108152). Both
  `jne` sites select the return route for `[0x100000] != 0x1234567`, with equality falling through
  (`jne` at 0x10814b, `jne` at 0x10817e).

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

- `_sysInByte`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081b0 to 0x1081c0.
  Established. The port input site reads one byte, with port and input value unresolved (`in` at
  0x1081b9). A prior four-byte memory read renders as `sp +0x4` (`mov` at 0x1081b0).

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

- `_sysInWord`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081c0 to 0x1081d0.
  Established. A two-byte input uses an unresolved port, and the tool leaves its value unknown (`in`
  at 0x1081c9). The recorded EAX exit is also unknown (`ret` at 0x1081cd).

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

- `_sysOutByte`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081d0 to 0x1081e0.
  Established. The byte port-output site has unknown port and stored value (`out` at 0x1081d8). The
  return carries `[sp+8]` in EAX (`ret` at 0x1081db).

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

- `_sysOutWord`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081e0 to 0x1081f0.
  Established. The output site writes two bytes to an unresolved port, with the stored value unknown
  (`out` at 0x1081e8). The prior four-byte reads render as `sp +0x4` and `sp +0x8` (`mov` at
  0x1081e0, `mov` at 0x1081e4).

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
