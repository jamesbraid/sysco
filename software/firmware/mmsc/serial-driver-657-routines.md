# MMSC serial-driver callers in the 6.5.7 image

Read from `6.5.7/usr/cpu/firmware/mmscfw.bin`.
Image SHA-256: `6c3db7ffd7fb4cc43ed02146f0c5bea0e85338def2530689c9990f877f2ddedc`.
The x86 a.out link address is `0x108000`. The [load map](load-map.md) explains segment mapping.
Analysed spans have exclusive ends inferred from the next text symbol.
Some include literals and padding decoded linearly after returns. Only sites reached
by known control-flow edges contribute effects to the tables.
Repeated `ret(F)` labels can refer to distinct call results and do not prove equal addresses.

`sp` denotes the entry stack pointer and brackets denote loaded values. Calls use
the assumed cdecl32 convention with unknown arity. Printed candidates can be local values,
saved registers or stale stack words. Unknown candidates and effective addresses stay unknown.
The access tables show explicit memory operands, including stack locations, and omit
implicit push/pop accesses. Full facts retain those accesses. Call and return instructions
remain listed without separate implicit stack access rows.
Unsupported instructions and partial-register tracking can leave results unresolved.
An analysis unknown does not establish indeterminate firmware behavior.
Field meanings, hardware roles, command semantics and callee effects remain unknown
unless explicitly established by an entry.

[Fact sheets and complete listings](disasm/serial-driver-657-facts.txt) include all tool sections.

The `_sysSerialChanGet` span ends with raw bytes `36 00` at `0x10b806` to `0x10b808`.
Linear decoding crosses tables in `_ns16550Ioctl` and `_ns16550Int`.
Their computed transfers remain unresolved and their selector-only effects are excluded.
For `_ns16550Ioctl`, the first table target is not an instruction start in that linear
listing. An independent aligned analysis from `0x179928` to `0x179d5c` follows the
entry analyses in the retained facts. It begins with fresh state and does not supply
caller arguments or effects. The remaining aligned instructions match the primary
listing after the initial boundary correction.

## `_sysSerialHwInit`

- `_sysSerialHwInit`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b618 to 0x10b758.
  Established. The loop branch is taken when `[sp-8] <= 0x5 (signed, 32-bit)` (`jle` at 0x10b73c);
  its `_ns16550DevInit` site carries candidate zero `[sp-12]` (`call` at 0x10b720). Explicit stores
  include two-byte zero, four-byte `0x1c2000`, and the four-byte literals `0x1081d0`, `0x1081b0`,
  all at unresolved targets (`mov` at 0x10b640, `mov` at 0x10b647, `mov` at 0x10b709, `mov` at
  0x10b711). The final result expression is `ret(_ns16550DevInit)` (`ret` at 0x10b74b). Object
  layout and hardware outcome are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b621 | store | 4 | `sp -0x8` | `0x0` |
| `mov` at 0x10b632 | store | 4 | `sp -0xc` | `0x1d8a10` |
| `mov` at 0x10b640 | store | 2 | `(unknown)` | `0x0` |
| `mov` at 0x10b647 | store | 4 | `(unknown)` | `0x1c2000` |
| `mov` at 0x10b64f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b653 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10b657 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10b65b | store | 1 | `(unknown)` | `?` |
| `mov` at 0x10b65f | load | 2 | `(unknown)` | `-` |
| `mov` at 0x10b664 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x10b669 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b66d | store | 4 | `(unknown)` | `?` |
| `movzx` at 0x10b671 | load | 2 | `(unknown)` | `-` |
| `add` at 0x10b676 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b67a | store | 4 | `(unknown)` | `?` |
| `movzx` at 0x10b67e | load | 2 | `(unknown)` | `-` |
| `add` at 0x10b685 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b689 | store | 4 | `(unknown)` | `?` |
| `movzx` at 0x10b68d | load | 2 | `(unknown)` | `-` |
| `add` at 0x10b694 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b698 | store | 4 | `(unknown)` | `?` |
| `movzx` at 0x10b69c | load | 2 | `(unknown)` | `-` |
| `add` at 0x10b6a4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b6a8 | store | 4 | `(unknown)` | `?` |
| `movzx` at 0x10b6ac | load | 2 | `(unknown)` | `-` |
| `add` at 0x10b6b4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b6b8 | store | 4 | `(unknown)` | `?` |
| `movzx` at 0x10b6bc | load | 2 | `(unknown)` | `-` |
| `add` at 0x10b6c4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b6c8 | store | 4 | `(unknown)` | `?` |
| `movzx` at 0x10b6cc | load | 2 | `(unknown)` | `-` |
| `add` at 0x10b6d6 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b6da | store | 4 | `(unknown)` | `?` |
| `movzx` at 0x10b6de | load | 2 | `(unknown)` | `-` |
| `add` at 0x10b6ec | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b6f0 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10b6f4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b6f8 | store | 4 | `(unknown)` | `?` |
| `movzx` at 0x10b6fc | load | 2 | `(unknown)` | `-` |
| `add` at 0x10b701 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b705 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10b709 | store | 4 | `(unknown)` | `0x1081d0` |
| `mov` at 0x10b711 | store | 4 | `(unknown)` | `0x1081b0` |
| `mov` at 0x10b719 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x10b71d | store | 4 | `sp -0x10` | `?` |
| `add` at 0x10b728 | load | 4 | `sp -0xc` | `-` |
| `add` at 0x10b728 | store | 4 | `sp -0xc` | `[sp-12]+108` |
| `inc` at 0x10b732 | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x10b732 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x10b735 | load | 4 | `sp -0x10` | `-` |
| `cmp` at 0x10b738 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b720 | `_ns16550DevInit 0x17910c` | `[sp-12]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x10b73c | `[sp-8] <= 0x5 (signed, 32-bit)` | `[sp-8] > 0x5 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b74b | `eax` | `ret(_ns16550DevInit)` |

## `_sysSerialHwInit2`

- `_sysSerialHwInit2`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b758 to 0x10b7c8.
  Established. The `_intConnect` site has unknown candidate zero and candidate two, with `0x179e2c`
  in candidate one (`call` at 0x10b787). `_sysIntEnablePIC` has only unknown slots (`call` at
  0x10b797). The prior branch is taken when `Z == 1` (`je` at 0x10b778). The loop repeats when
  `[sp-8] <= 0x5 (signed, 32-bit)` (`jle` at 0x10b7af). The candidate address does not prove a call
  through it, and interrupt identities remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b761 | store | 4 | `sp -0x8` | `0x0` |
| `cmp` at 0x10b774 | load | 2 | `(unknown)` | `-` |
| `movzx` at 0x10b780 | load | 2 | `(unknown)` | `-` |
| `movzx` at 0x10b78f | load | 2 | `(unknown)` | `-` |
| `inc` at 0x10b7a8 | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x10b7a8 | store | 4 | `sp -0x8` | `?` |
| `cmp` at 0x10b7ab | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b787 | `_intConnect 0x17a210` | `?` | `0x179e2c` | `?` | `?` | `?` | `?` |
| `call` at 0x10b797 | `_sysIntEnablePIC 0x108428` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10b778 | `Z == 1` | `not (Z == 1)` |
| `jle` at 0x10b7af | `[sp-8] <= 0x5 (signed, 32-bit)` | `[sp-8] > 0x5 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b7ba | `eax` | `?` |

## `_sysSerialChanGet`

- `_sysSerialChanGet`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b7c8 to 0x10b808.
  Established. The signed branch takes the arithmetic arm when `[sp+4] <= 0x5`, including negative
  values (`jle` at 0x10b7d1); its returned expression is unknown (`ret` at 0x10b800). The other arm
  reports `0xffffffff` (`ret` at 0x10b7db). The sole explicit access reads four bytes at `sp +0x4`
  (`mov` at 0x10b7cb). Channel identity and an address formula are not established by the tracked
  output.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b7cb | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x10b7d1 | `[sp+4] <= 0x5 (signed, 32-bit)` | `[sp+4] > 0x5 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b7db | `eax` | `0xffffffff` |
| `ret` at 0x10b800 | `eax` | `?` |

## `_ns16550DevInit`

- `_ns16550DevInit`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x17910c to 0x17916c.
  Established. Four-byte stores write `0x1d6c44` at `[sp+4] +0x0`, `0x1790ec` at `+0x4` and `+0x8`,
  and `0xf` at `+0x68` (`mov` at 0x17911f, `mov` at 0x179128, `mov` at 0x179132, `mov` at 0x179145).
  A two-byte zero goes to `[sp+4] +0x5c` (`mov` at 0x17913c). `_ns16550InitChannel` has candidate
  zero `[sp+4]` (`call` at 0x179150). `_intUnlock` later has `[sp-8]` (`call` at 0x17915c). The
  stored pointer and option meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x179119 | store | 4 | `sp -0x8` | `ret(_intLock)` |
| `mov` at 0x17911c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17911f | store | 4 | `[sp+4] +0x0` | `0x1d6c44` |
| `mov` at 0x179125 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179128 | store | 4 | `[sp+4] +0x4` | `0x1790ec` |
| `mov` at 0x17912f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179132 | store | 4 | `[sp+4] +0x8` | `0x1790ec` |
| `mov` at 0x179139 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17913c | store | 2 | `[sp+4] +0x5c` | `0x0` |
| `mov` at 0x179142 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179145 | store | 4 | `[sp+4] +0x68` | `0xf` |
| `mov` at 0x17914c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179158 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x179112 | `_intLock 0x175148` | `?` | `ebp` | `?` | `?` | `?` | `?` |
| `call` at 0x179150 | `_ns16550InitChannel 0x17916c` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x17915c | `_intUnlock 0x175158` | `[sp-8]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x179167 | `eax` | `ret(_intUnlock)` |

## `_ns16550InitChannel`

- `_ns16550InitChannel`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x17916c to 0x17945c.
  Established. Byte assignments include `0x3` at `[sp+4] +0x1b`, `0x7` and `0x89` at `[sp+4] +0x1a`,
  and `0x3` at `[sp+4] +0x19` (`mov` at 0x17917c, `mov` at 0x1791bd, `mov` at 0x1791fe, `mov` at
  0x179372). The branch is taken when `[sp-8] == 0x0` and falls through when `[sp-8] != 0x0`
  (`je` at 0x1793b2). Each arm writes four-byte `0xb` at `[sp+4] +0x1c` (`mov` at 0x1793bd,
  `mov` at 0x1793d5). The indirect call carrying `[[sp+4]+52]`, `0x0` has an unknown callee (`call`
  at 0x179391). Register identities and serial configuration semantics are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x179173 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179176 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x179179 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x17917c | store | 1 | `[sp+4] +0x1b` | `0x3` |
| `mov` at 0x179180 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179183 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x179186 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179189 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x17918c | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x17918f | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x179195 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x179198 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x17919c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17919f | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x1791a2 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1791a5 | load | 4 | `[sp+4] +0x40` | `-` |
| `mov` at 0x1791a9 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1791ac | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x1791b4 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1791b7 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x1791ba | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1791bd | store | 1 | `[sp+4] +0x1a` | `0x7` |
| `mov` at 0x1791c1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1791c4 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x1791c7 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1791ca | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x1791cd | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1791d0 | load | 1 | `[sp+4] +0x1a` | `-` |
| `mov` at 0x1791d6 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x1791d9 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1791dd | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1791e0 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x1791e3 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1791e6 | load | 4 | `[sp+4] +0x38` | `-` |
| `mov` at 0x1791ea | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1791ed | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x1791f5 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1791f8 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x1791fb | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1791fe | store | 1 | `[sp+4] +0x1a` | `0x89` |
| `mov` at 0x179202 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179205 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x179208 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17920b | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x17920e | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x179211 | load | 1 | `[sp+4] +0x1a` | `-` |
| `mov` at 0x179217 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x17921a | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x17921e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179221 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x179224 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x179227 | load | 4 | `[sp+4] +0x38` | `-` |
| `mov` at 0x17922b | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x17922e | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x179236 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179239 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x17923c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17923f | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x179242 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x179245 | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x17924c | store | 1 | `sp -0xc` | `?` |
| `movsx` at 0x17924f | load | 1 | `sp -0xc` | `-` |
| `mov` at 0x179254 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179257 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x17925a | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x17925d | load | 4 | `[sp+4] +0x40` | `-` |
| `mov` at 0x179261 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x179264 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x17926c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17926f | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x179272 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179275 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x179278 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x17927b | load | 4 | `[sp+4] +0x64` | `-` |
| `mov` at 0x17927e | store | 4 | `sp -0xc` | `0x1b4e81b5` |
| `mov` at 0x179285 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x179291 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x179294 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x179297 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x17929a | store | 4 | `[sp-8] +0x20` | `[sp-12]` |
| `mov` at 0x17929d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1792a0 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x1792a3 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1792a6 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x1792a9 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1792ac | load | 4 | `[sp+4] +0x64` | `-` |
| `mov` at 0x1792af | store | 4 | `sp -0xc` | `0x1b4e81b5` |
| `mov` at 0x1792b6 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1792c2 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x1792c5 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1792cb | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1792ce | store | 4 | `[sp-8] +0x24` | `?` |
| `mov` at 0x1792d1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1792d4 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x1792d7 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1792da | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x1792dd | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1792e0 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x1792e3 | store | 1 | `sp -0xc` | `?` |
| `movsx` at 0x1792e6 | load | 1 | `sp -0xc` | `-` |
| `mov` at 0x1792eb | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1792ee | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x1792f1 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1792f4 | load | 4 | `[sp+4] +0x54` | `-` |
| `mov` at 0x1792f8 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1792fb | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x179303 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179306 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x179309 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17930c | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x17930f | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x179312 | load | 4 | `[sp+4] +0x24` | `-` |
| `mov` at 0x179315 | store | 1 | `sp -0xc` | `?` |
| `movsx` at 0x179318 | load | 1 | `sp -0xc` | `-` |
| `mov` at 0x17931d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179320 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x179323 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x179326 | load | 4 | `[sp+4] +0x58` | `-` |
| `mov` at 0x17932a | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x17932d | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x179335 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179338 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x17933b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17933e | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x179341 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x179344 | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x17934a | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x17934d | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x179351 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179354 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x179357 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x17935a | load | 4 | `[sp+4] +0x40` | `-` |
| `mov` at 0x17935e | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x179361 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x179369 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17936c | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x17936f | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x179372 | store | 1 | `[sp+4] +0x19` | `0x3` |
| `mov` at 0x179376 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179379 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x17937e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179381 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x179384 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x179387 | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x17938b | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x17938e | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x179396 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179399 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x17939c | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x17939f | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x1793a2 | store | 4 | `sp -0xc` | `[[sp+4]+104]` |
| `mov` at 0x1793a5 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1793ab | store | 4 | `sp -0x8` | `?` |
| `cmp` at 0x1793ae | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1793b4 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1793b7 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x1793ba | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1793bd | store | 4 | `[sp+4] +0x1c` | `0xb` |
| `mov` at 0x1793cc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1793cf | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x1793d2 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1793d5 | store | 4 | `[sp+4] +0x1c` | `0xb` |
| `mov` at 0x1793dc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1793df | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x1793e2 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1793e5 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x1793e8 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x1793eb | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x1793f2 | store | 1 | `sp -0xc` | `?` |
| `mov` at 0x1793f5 | load | 1 | `sp -0xc` | `-` |
| `mov` at 0x1793f8 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1793fb | store | 1 | `[sp+4] +0x19` | `[sp-12]:1` |
| `mov` at 0x1793fe | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179401 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x179404 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179407 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x17940a | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x17940d | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x179414 | store | 1 | `sp -0xc` | `?` |
| `mov` at 0x179417 | load | 1 | `sp -0xc` | `-` |
| `mov` at 0x17941a | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x17941d | store | 1 | `[sp+4] +0x19` | `[sp-12]:1` |
| `mov` at 0x179420 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179423 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x179426 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179429 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x17942c | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x17942f | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x179432 | store | 1 | `sp -0xc` | `?` |
| `movsx` at 0x179435 | load | 1 | `sp -0xc` | `-` |
| `mov` at 0x17943a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17943d | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x179440 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x179443 | load | 4 | `[sp+4] +0x44` | `-` |
| `mov` at 0x179447 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x17944a | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x179452 | load | 4 | `sp -0x10` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1791af | `(unknown callee) (unknown target)` | `[[sp+4]+64]` | `[sp-12]` | `?` | `[sp+4]` | `[sp+4]` | `?` |
| `call` at 0x1791f0 | `(unknown callee) (unknown target)` | `[[sp+4]+56]` | `[sp-12]` | `?` | `[sp+4]` | `[sp+4]` | `?` |
| `call` at 0x179231 | `(unknown callee) (unknown target)` | `[[sp+4]+56]` | `[sp-12]` | `?` | `[sp+4]` | `[sp+4]` | `?` |
| `call` at 0x179267 | `(unknown callee) (unknown target)` | `[[sp+4]+64]` | `?` | `?` | `[sp+4]` | `[sp+4]` | `?` |
| `call` at 0x1792fe | `(unknown callee) (unknown target)` | `[[sp+4]+84]` | `?` | `?` | `[sp+4]` | `[sp+4]` | `?` |
| `call` at 0x179330 | `(unknown callee) (unknown target)` | `[[sp+4]+88]` | `?` | `?` | `[sp+4]` | `[sp+4]` | `?` |
| `call` at 0x179364 | `(unknown callee) (unknown target)` | `[[sp+4]+64]` | `[sp-12]` | `?` | `[sp+4]` | `[sp+4]` | `?` |
| `call` at 0x179391 | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `0x0` | `?` | `[sp+4]` | `[sp+4]` | `?` |
| `call` at 0x17944d | `(unknown callee) (unknown target)` | `[[sp+4]+68]` | `?` | `?` | `[sp+4]` | `[sp+4]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1793b2 | `[sp-8] == 0x0` | `[sp-8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x179458 | `eax` | `ret(unknown)` |

## `_ns16550OptsSet`

- `_ns16550OptsSet`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x17945c to 0x1796ec.
  Established. The initial zero argument takes the branch to the literal-return arm (`je` at
  0x179465). Byte stores at `[sp+4] +0x1b` include `0x0`, `0x1`, `0x2`, and `0x3` at separate sites
  (`mov` at 0x1794bf, `mov` at 0x1794cf, `mov` at 0x1794df, `mov` at 0x1794ef). A later byte
  assignment writes `0x87` at `[sp+4] +0x1a` (`mov` at 0x179661). The final four-byte store copies
  `[sp+8]` into `[sp+4] +0x68` (`mov` at 0x1796d4). The indirect callee at the site with candidates
  `[[sp+4]+52]`, `0x0` remains unknown (`call` at 0x1795bb). Option encoding is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x179461 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179467 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x17948c | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1794bc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1794bf | store | 1 | `[sp+4] +0x1b` | `0x0` |
| `mov` at 0x1794cc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1794cf | store | 1 | `[sp+4] +0x1b` | `0x1` |
| `mov` at 0x1794dc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1794df | store | 1 | `[sp+4] +0x1b` | `0x2` |
| `mov` at 0x1794ec | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1794ef | store | 1 | `[sp+4] +0x1b` | `0x3` |
| `mov` at 0x1794fc | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x179506 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179509 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17950c | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x179514 | store | 1 | `[sp+4] +0x1b` | `?` |
| `mov` at 0x17951c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17951f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179522 | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x179525 | store | 1 | `[sp+4] +0x1b` | `[[sp+4]+27]:1` |
| `mov` at 0x179528 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x17955c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17955f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179562 | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x17956a | store | 1 | `[sp+4] +0x1b` | `?` |
| `mov` at 0x17957c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17957f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179582 | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x17958a | store | 1 | `[sp+4] +0x1b` | `?` |
| `mov` at 0x17959c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17959f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1795a2 | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x1795a5 | store | 1 | `[sp+4] +0x1b` | `[[sp+4]+27]:1` |
| `mov` at 0x1795ac | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1795b1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1795b4 | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x1795b8 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x1795c0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1795c3 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1795c6 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x1795ce | store | 4 | `[sp+4] +0x1c` | `?` |
| `mov` at 0x1795d1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1795d4 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1795d7 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x1795df | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x1795e2 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1795ec | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1795ef | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1795f2 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x1795fa | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x17960c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17960f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179612 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x17961a | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x179624 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179627 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17962a | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x179631 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179634 | load | 4 | `[sp+4] +0x40` | `-` |
| `mov` at 0x179638 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x179640 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179643 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179646 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x17964f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179652 | load | 4 | `[sp+4] +0x44` | `-` |
| `mov` at 0x179656 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x17965e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179661 | store | 1 | `[sp+4] +0x1a` | `0x87` |
| `mov` at 0x179665 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179668 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17966b | load | 1 | `[sp+4] +0x1a` | `-` |
| `mov` at 0x179672 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179675 | load | 4 | `[sp+4] +0x38` | `-` |
| `mov` at 0x179679 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x179681 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x17968b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17968e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179691 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x179699 | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x17969c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17969f | load | 2 | `[sp+4] +0x5c` | `-` |
| `mov` at 0x1796a9 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1796ac | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1796af | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x1796b6 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1796b9 | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x1796bd | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x1796ce | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1796d1 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1796d4 | store | 4 | `[sp+4] +0x68` | `[sp+8]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1795bb | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x17961d | `_intLock 0x175148` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x17963b | `(unknown callee) (unknown target)` | `[[sp+4]+64]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x179659 | `(unknown callee) (unknown target)` | `[[sp+4]+68]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x17967c | `(unknown callee) (unknown target)` | `[[sp+4]+56]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1796c0 | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1796c6 | `_intUnlock 0x175158` | `ret(_intLock)` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x179465 | `[sp+4] == 0x0` | `[sp+4] != 0x0` |
| `jne` at 0x179471 | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `je` at 0x179495 | `? == 0x4` | `? != 0x4` |
| `ja` at 0x17949a | `? > 0x4 (unsigned, 32-bit)` | `? <= 0x4 (unsigned, 32-bit)` |
| `je` at 0x17949e | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x1794af | `? == 0x8` | `? != 0x8` |
| `je` at 0x1794b4 | `? == 0xc` | `? != 0xc` |
| `je` at 0x179504 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x179533 | `? == 0x40` | `? != 0x40` |
| `ja` at 0x179538 | `? > 0x40 (unsigned, 32-bit)` | `? <= 0x40 (unsigned, 32-bit)` |
| `je` at 0x17953c | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x179551 | `? == 0xc0` | `? != 0xc0` |
| `jne` at 0x1795ea | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `je` at 0x179689 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `jne` at 0x1796a7 | `([[sp+4]+92]:2 & 0xffff) != 0x2` | `([[sp+4]+92]:2 & 0xffff) == 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1796e4 | `eax` | `?` |

## `_ns16550Hup`

- `_ns16550Hup`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1796ec to 0x1797cc.
  Established. The first unknown callee has candidates `[[sp+4]+52]`, `0x0` (`call` at 0x179707).
  Later unknown callees receive `[[sp+4]+68]` and `[[sp+4]+56]` in candidate zero (`call` at
  0x179747, `call` at 0x179774). The branch falls through when `([[sp+4]+92]:2 & 0xffff) == 0x2`
  (`jne` at 0x179784), reaching another unknown call with candidate zero `[[sp+4]+52]` (`call` at
  0x17979d). Three explicit stores at `[sp+4] +0x19`, `+0x1c`, and `+0x1a` have unknown values
  (`mov` at 0x17971a, `mov` at 0x17972b, `mov` at 0x17975a). Hardware effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1796f8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1796fd | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179700 | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x179704 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x17970c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17970f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179712 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x17971a | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x17971d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179720 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179723 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x17972b | store | 4 | `[sp+4] +0x1c` | `?` |
| `mov` at 0x17972e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179731 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179734 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x17973d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179740 | load | 4 | `[sp+4] +0x44` | `-` |
| `mov` at 0x179744 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x17974c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17974f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179752 | load | 1 | `[sp+4] +0x1a` | `-` |
| `mov` at 0x17975a | store | 1 | `[sp+4] +0x1a` | `?` |
| `mov` at 0x17975d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179760 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179763 | load | 1 | `[sp+4] +0x1a` | `-` |
| `mov` at 0x17976a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17976d | load | 4 | `[sp+4] +0x38` | `-` |
| `mov` at 0x179771 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x179779 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17977c | load | 2 | `[sp+4] +0x5c` | `-` |
| `mov` at 0x179786 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179789 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17978c | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x179793 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179796 | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x17979a | load | 4 | `[sp+4] +0x2c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1796f1 | `_intLock 0x175148` | `ebx` | `esi` | `ebp` | `?` | `?` | `?` |
| `call` at 0x179707 | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x179747 | `(unknown callee) (unknown target)` | `[[sp+4]+68]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x179774 | `(unknown callee) (unknown target)` | `[[sp+4]+56]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x17979d | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1797a3 | `_intUnlock 0x175158` | `ret(_intLock)` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x179784 | `([[sp+4]+92]:2 & 0xffff) != 0x2` | `([[sp+4]+92]:2 & 0xffff) == 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1797c4 | `eax` | `0x0` |

## `_ns16550Open`

- `_ns16550Open`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1797cc to 0x1798cc.
  Established. The first indirect call has candidate zero `[[sp+4]+76]` and an unknown callee
  (`call` at 0x1797e1). The branch takes the bypass arm when `(? & ?) == 0x0` and falls through
  when `(? & ?) != 0x0` (`je` at 0x1797f7). Explicit assignments at `[sp+4] +0x19`, `+0x1c`, `+0x1a` retain
  unknown values (`mov` at 0x179813, `mov` at 0x179824, `mov` at 0x179835, `mov` at 0x179864). A
  later branch falls through when `([[sp+4]+92]:2 & 0xffff) == 0x2` (`jne` at 0x17988e), reaching an
  unknown callee with candidate zero `[[sp+4]+52]` (`call` at 0x1798a7). The exit is zero (`ret` at
  0x1798c4). Open-line semantics are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1797d4 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1797d7 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1797da | load | 4 | `[sp+4] +0x4c` | `-` |
| `mov` at 0x1797de | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x1797ed | store | 1 | `sp -0x5` | `?` |
| `mov` at 0x1797f0 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x179802 | store | 4 | `sp -0xc` | `ret(_intLock)` |
| `mov` at 0x179805 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179808 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17980b | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x179813 | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x179816 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179819 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17981c | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x179824 | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x179827 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17982a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17982d | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x179835 | store | 4 | `[sp+4] +0x1c` | `?` |
| `mov` at 0x179838 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17983b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17983e | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x179847 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17984a | load | 4 | `[sp+4] +0x44` | `-` |
| `mov` at 0x17984e | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x179856 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179859 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17985c | load | 1 | `[sp+4] +0x1a` | `-` |
| `mov` at 0x179864 | store | 1 | `[sp+4] +0x1a` | `?` |
| `mov` at 0x179867 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17986a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17986d | load | 1 | `[sp+4] +0x1a` | `-` |
| `mov` at 0x179874 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179877 | load | 4 | `[sp+4] +0x38` | `-` |
| `mov` at 0x17987b | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x179883 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179886 | load | 2 | `[sp+4] +0x5c` | `-` |
| `mov` at 0x179890 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179893 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179896 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x17989d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1798a0 | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x1798a4 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x1798ac | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1797e1 | `(unknown callee) (unknown target)` | `[[sp+4]+76]` | `ebx` | `esi` | `?` | `?` | `ebp` |
| `call` at 0x1797fd | `_intLock 0x175148` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x179851 | `(unknown callee) (unknown target)` | `[[sp+4]+68]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x17987e | `(unknown callee) (unknown target)` | `[[sp+4]+56]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1798a7 | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1798b0 | `_intUnlock 0x175158` | `[sp-12]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1797f7 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `jne` at 0x17988e | `([[sp+4]+92]:2 & 0xffff) != 0x2` | `([[sp+4]+92]:2 & 0xffff) == 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1798c4 | `eax` | `0x0` |

## `_ns16550Ioctl`

- `_ns16550Ioctl`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1798cc to 0x179d5c.
  Established. The entry stores `[sp+8]-4099` at `sp -0x8` (`mov` at 0x1798df). The unsigned branch
  takes its literal-return arm when `[sp+8]-4099 > 0xa (unsigned, 32-bit)` (`ja` at 0x1798e6). The
  other arm reaches a computed transfer whose edges remain unresolved (`jmp` at 0x1798f9). The
  indexed four-byte load target is unknown (`mov` at 0x1798ef). The known return has value `0x47`
  (`ret` at 0x179d55), while selector-only effects are excluded. Command meanings and the full
  result catalogue are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1798d7 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1798df | store | 4 | `sp -0x8` | `[sp+8]-4099` |
| `cmp` at 0x1798e2 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1798ec | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1798ef | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1798f6 | store | 4 | `sp -0x8` | `?` |
| `jmp` at 0x1798f9 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ja` at 0x1798e6 | `[sp+8]-4099 > 0xa (unsigned, 32-bit)` | `[sp+8]-4099 <= 0xa (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x179d55 | `eax` | `0x47` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x1798f9).

## `_ns16550IntWr`

- `_ns16550IntWr`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x179d5c to 0x179ddc.
  Established. The first unknown callee receives candidate zero `[[sp+4]+12]` and candidate one
  `sp-5` (`call` at 0x179d74); its result equal to `0xffffffff` takes the branch to the alternate
  arm (`je` at 0x179d7e). That arm stores one byte of unknown value at `[sp+4] +0x19` (`mov` at
  0x179daa) and reaches another unknown callee with candidate zero `[[sp+4]+52]` (`call` at
  0x179dc4). The fall-through call has candidate zero `[[sp+4]+48]` (`call` at 0x179d92). Byte
  transport and callback identities are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x179d63 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179d6a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179d6d | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x179d71 | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x179d80 | load | 4 | `sp +0x4` | `-` |
| `movsx` at 0x179d83 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x179d88 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179d8b | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x179d8f | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x179d9c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179d9f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179da2 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x179daa | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x179dad | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179db0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179db3 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x179dba | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179dbd | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x179dc1 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x179dc9 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x179d74 | `(unknown callee) (unknown target)` | `[[sp+4]+12]` | `sp-5` | `ebx` | `?` | `ebp` | `?` |
| `call` at 0x179d92 | `(unknown callee) (unknown target)` | `[[sp+4]+48]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x179dc4 | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x179d7e | `ret(unknown) == 0xffffffff` | `ret(unknown) != 0xffffffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x179dcf | `eax` | `ret(unknown)` |

## `_ns16550IntRd`

- `_ns16550IntRd`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x179ddc to 0x179e1c.
  Established. Two indirect sites have unknown callees and candidate zero values `[[sp+4]+48]` and
  `[[sp+4]+16]` (`call` at 0x179df0, `call` at 0x179e0a). Between them a byte of unknown value is
  stored at `sp -0x5` (`mov` at 0x179df5). Candidate one at the second site is unknown (`call` at
  0x179e0a). The exit carries `ret(unknown)` (`ret` at 0x179e15). The byte-receive role is not
  established by these call sites.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x179de3 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179de6 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179de9 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x179ded | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x179df5 | store | 1 | `sp -0x5` | `?` |
| `mov` at 0x179df8 | load | 4 | `sp +0x4` | `-` |
| `movsx` at 0x179dfb | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x179e00 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179e03 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x179e07 | load | 4 | `[sp+4] +0x8` | `-` |
| `mov` at 0x179e0f | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x179df0 | `(unknown callee) (unknown target)` | `[[sp+4]+48]` | `ebx` | `?` | `ebp` | `?` | `?` |
| `call` at 0x179e0a | `(unknown callee) (unknown target)` | `[[sp+4]+16]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x179e15 | `eax` | `ret(unknown)` |

## `_ns16550IntEx`

- `_ns16550IntEx`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x179e1c to 0x179e2c.
  Established. The body copies `ebp` into `esp` (`mov` at 0x179e1f) and reaches a return retaining
  the entry `eax` expression (`ret` at 0x179e22); its explicit-access, call and
  conditional-branch tables are empty. No interrupt action is established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x179e22 | `eax` | `eax` |

## `_ns16550Int`

- `_ns16550Int`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x179e2c to 0x17a00c.
  Established. The first unknown call has candidate zero `[[sp+4]+60]` (`call` at 0x179e40). Another
  has leading candidates `[[sp+4]+52]`, `0x0` (`call` at 0x179e5e). The unsigned branch takes the
  shared-exit route when `? > 0xc (unsigned, 32-bit)` (`ja` at 0x179e6c); its other arm reaches an
  unresolved computed jump (`jmp` at 0x179e79). Selector-only sites remain excluded. The final
  known-edge call also has an unknown callee and candidate zero `[[sp+4]+52]` (`call` at 0x179ff3).
  Interrupt causes and servicing behavior are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x179e33 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179e36 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179e39 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x179e3d | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x179e4c | store | 1 | `sp -0x5` | `?` |
| `mov` at 0x179e4f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179e54 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179e57 | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x179e5b | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x179e63 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x179e72 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x179fdc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179fdf | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179fe2 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x179fe9 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x179fec | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x179ff0 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x179ff8 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x179e40 | `(unknown callee) (unknown target)` | `[[sp+4]+60]` | `ebx` | `?` | `ebp` | `?` | `?` |
| `call` at 0x179e5e | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x179ff3 | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ja` at 0x179e6c | `? > 0xc (unsigned, 32-bit)` | `? <= 0xc (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x179ffe | `eax` | `ret(unknown)` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x179e79).

## `_ns16550TxStartup`

- `_ns16550TxStartup`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x17a00c to 0x17a0cc.
  Established. A two-byte load reads `[sp+4] +0x5c` (`mov` at 0x17a016). The deciding branch takes
  its exit arm when `([[sp+4]+92]:2 & 0xffff) != 0x2` (`jne` at 0x17a01e). The remaining path
  includes two unknown callees with candidate zero `[[sp+4]+76]` and `[[sp+4]+52]` (`call` at
  0x17a059, `call` at 0x17a0b4). Stores at `[sp+4] +0x19` have unknown values (`mov` at 0x17a041,
  `mov` at 0x17a078, `mov` at 0x17a09a). Transmitter behavior is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x17a013 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a016 | load | 2 | `[sp+4] +0x5c` | `-` |
| `mov` at 0x17a024 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a027 | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x17a033 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a036 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a039 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x17a041 | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x17a04c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a04f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a052 | load | 4 | `[sp+4] +0x4c` | `-` |
| `mov` at 0x17a056 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x17a05e | store | 1 | `sp -0x5` | `?` |
| `mov` at 0x17a061 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x17a06a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a06d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a070 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x17a078 | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x17a08c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a08f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a092 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x17a09a | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x17a09d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a0a0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a0a3 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x17a0aa | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a0ad | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x17a0b1 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x17a0b9 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x17a059 | `(unknown callee) (unknown target)` | `[[sp+4]+76]` | `ebx` | `?` | `ebp` | `?` | `?` |
| `call` at 0x17a0b4 | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x17a01e | `([[sp+4]+92]:2 & 0xffff) != 0x2` | `([[sp+4]+92]:2 & 0xffff) == 0x2` |
| `je` at 0x17a031 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x17a068 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x17a0bf | `eax` | `?` |

## `_ns16550CallbackInstall`

- `_ns16550CallbackInstall`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x17a19c to 0x17a210.
  Established. The first branch takes its store arm when `[sp+8] == 0x1` (`je` at 0x17a1ae). That
  arm writes four-byte `[sp+12]` at `[sp+4] +0x4` and `[sp+16]` at `[sp-8] +0xc` (`mov` at 0x17a1c2,
  `mov` at 0x17a1cb). The second branch takes its arm when `[sp+8] == 0x2` (`je` at 0x17a1b3), with
  stores to `[sp+4] +0x8` and `[sp-8] +0x10` respectively (`mov` at 0x17a1e2, `mov` at 0x17a1eb). No
  callee is invoked. Callback ownership and registration behavior remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x17a1a2 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a1a5 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x17a1a8 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x17a1bc | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x17a1bf | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x17a1c2 | store | 4 | `[sp+4] +0x4` | `[sp+12]` |
| `mov` at 0x17a1c5 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x17a1c8 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x17a1cb | store | 4 | `[sp-8] +0xc` | `[sp+16]` |
| `mov` at 0x17a1dc | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x17a1df | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x17a1e2 | store | 4 | `[sp+4] +0x8` | `[sp+12]` |
| `mov` at 0x17a1e5 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x17a1e8 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x17a1eb | store | 4 | `[sp-8] +0x10` | `[sp+16]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x17a1ae | `[sp+8] == 0x1` | `[sp+8] != 0x1` |
| `je` at 0x17a1b3 | `[sp+8] == 0x2` | `[sp+8] != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x17a20f | `eax` | `?` |
