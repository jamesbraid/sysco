# MMSC serial and interrupt paths in the 6.4-origin image

Read from `6.4-origin/usr/cpu/firmware/mmscfw.bin`.
Image SHA-256: `b766e559f86078526916d8bd98bbb8079294ad774cce9177928873153c6fb09b`.
The x86 a.out link address is `0x108000`. The [load map](load-map.md) explains segment mapping.
Analysed spans have exclusive ends inferred from the next text symbol.
Some include literals and padding decoded linearly after returns.
Only sites reached by known control-flow edges contribute effects to the tables.
The computed transfer in `_i8250Ioctl` remains unresolved. Selector-only effects are excluded.
An aligned supplementary listing covers its internal target with an independent entry state.
That entry state does not resolve selector dataflow or establish additional caller effects.
The four residual bytes from `0x10b7a4` to exclusive `0x10b7a8` were read directly:
`36 00 90 90`. They are outside the decoded instruction list.

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

[Fact sheets and complete listings](disasm/serial-pic-64-origin-facts.txt) include all tool sections.

## `_sysSerialHwInit`

- `_sysSerialHwInit`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b618 to 0x10b718.
  Established. Four-byte stores at unknown effective addresses include values `0x1081d0` (`mov` at
  0x10b6ca) and `0x1081b0` (`mov` at 0x10b6d2). The `_i8250HrdInit` call retains unknown candidates
  (`call` at 0x10b6e1). The repeat branch is taken when `[sp-8] <= 0x5 (signed, 32-bit)` (`jle` at
  0x10b6ff). No object count or callback execution is inferred.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b621 | store | 4 | `sp -0x8` | `0x0` |
| `mov` at 0x10b63c | load | 2 | `(unknown)` | `-` |
| `mov` at 0x10b643 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x10b648 | store | 2 | `(unknown)` | `0x0` |
| `movzx` at 0x10b64f | load | 2 | `(unknown)` | `-` |
| `add` at 0x10b657 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b65b | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10b65f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b663 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10b667 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b66b | store | 4 | `(unknown)` | `?` |
| `movzx` at 0x10b66f | load | 2 | `(unknown)` | `-` |
| `add` at 0x10b674 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b678 | store | 4 | `(unknown)` | `?` |
| `movzx` at 0x10b67c | load | 2 | `(unknown)` | `-` |
| `add` at 0x10b681 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b685 | store | 4 | `(unknown)` | `?` |
| `movzx` at 0x10b689 | load | 2 | `(unknown)` | `-` |
| `add` at 0x10b690 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b694 | store | 4 | `(unknown)` | `?` |
| `movzx` at 0x10b698 | load | 2 | `(unknown)` | `-` |
| `add` at 0x10b6a0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b6a4 | store | 4 | `(unknown)` | `?` |
| `movzx` at 0x10b6a8 | load | 2 | `(unknown)` | `-` |
| `add` at 0x10b6b0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b6b4 | store | 4 | `(unknown)` | `?` |
| `movzx` at 0x10b6b8 | load | 2 | `(unknown)` | `-` |
| `add` at 0x10b6c2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b6c6 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10b6ca | store | 4 | `(unknown)` | `0x1081d0` |
| `mov` at 0x10b6d2 | store | 4 | `(unknown)` | `0x1081b0` |
| `mov` at 0x10b6db | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x10b6de | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x10b6e9 | load | 4 | `sp -0x10` | `-` |
| `inc` at 0x10b6f5 | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x10b6f5 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x10b6f8 | load | 4 | `sp -0xc` | `-` |
| `cmp` at 0x10b6fb | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b6e1 | `_i8250HrdInit 0x173b6c` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x10b6ff | `[sp-8] <= 0x5 (signed, 32-bit)` | `[sp-8] > 0x5 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b70e | `eax` | `ret(_i8250HrdInit)` |

## `_sysSerialHwInit2`

- `_sysSerialHwInit2`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b718 to 0x10b788.
  Established. The `_intConnect` call presents unknown candidate zero, `0x173e5c` as candidate one,
  and unknown candidate two (`call` at 0x10b74f). The next `_sysIntEnablePIC` call retains unknown
  candidates (`call` at 0x10b75f). The first branch bypasses both sites when its taken condition is
  `Z == 1` (`je` at 0x10b73c). A four-byte store earlier writes `0x0` at `sp -0x8` (`mov` at
  0x10b721).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b721 | store | 4 | `sp -0x8` | `0x0` |
| `cmp` at 0x10b734 | load | 2 | `(unknown)` | `-` |
| `movzx` at 0x10b744 | load | 2 | `(unknown)` | `-` |
| `movzx` at 0x10b757 | load | 2 | `(unknown)` | `-` |
| `inc` at 0x10b770 | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x10b770 | store | 4 | `sp -0x8` | `?` |
| `cmp` at 0x10b773 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b74f | `_intConnect 0x173fe8` | `?` | `0x173e5c` | `?` | `?` | `?` | `?` |
| `call` at 0x10b75f | `_sysIntEnablePIC 0x108428` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10b73c | `Z == 1` | `not (Z == 1)` |
| `jle` at 0x10b777 | `[sp-8] <= 0x5 (signed, 32-bit)` | `[sp-8] > 0x5 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b782 | `eax` | `?` |

## `_sysSerialChanGet`

- `_sysSerialChanGet`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b788 to 0x10b7a8.
  Established. Two four-byte loads use `sp +0x4` (`mov` at 0x10b78b, `add` at 0x10b791). The exit
  expression in `eax` is unknown (`ret` at 0x10b79e). A channel address or structure size is not
  established from the routine name or the unresolved arithmetic.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b78b | load | 4 | `sp +0x4` | `-` |
| `add` at 0x10b791 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b79e | `eax` | `?` |

## `_i8250HrdInit`

- `_i8250HrdInit`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x173b6c to 0x173bcc.
  Established. The initial indirect call has leading candidates `[[sp+4]+48]` and `0x0` (`call` at
  0x173b7c). The later branch falls through when `[0x1d882c] == 0x0` (`jne` at 0x173b85), reaching
  four-byte stores that include `0x173c2c` at absolute `0x1d882c` (`mov` at 0x173b87) and `0x173f9c`
  at absolute `0x1d883c` (`mov` at 0x173baf). A common four-byte store writes `0x1d882c` through
  `[sp+4] +0x0` (`mov` at 0x173bb9). Those stored values do not establish subsequent execution.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x173b70 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x173b75 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x173b79 | load | 4 | `[sp+4] +0x1c` | `-` |
| `cmp` at 0x173b7e | load | 4 | `abs 0x1d882c` | `-` |
| `mov` at 0x173b87 | store | 4 | `abs 0x1d882c` | `0x173c2c` |
| `mov` at 0x173b91 | store | 4 | `abs 0x1d8830` | `0x173f3c` |
| `mov` at 0x173b9b | store | 4 | `abs 0x1d8834` | `0x173b1c` |
| `mov` at 0x173ba5 | store | 4 | `abs 0x1d8838` | `0x173f5c` |
| `mov` at 0x173baf | store | 4 | `abs 0x1d883c` | `0x173f9c` |
| `mov` at 0x173bb9 | store | 4 | `[sp+4] +0x0` | `0x1d882c` |
| `mov` at 0x173bbf | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x173b7c | `(unknown callee) (unknown target)` | `[[sp+4]+48]` | `0x0` | `ebx` | `ebp` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x173b85 | `[0x1d882c] != 0x0` | `[0x1d882c] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x173bc5 | `eax` | `ret(unknown)` |

## `_i8250InitChannel`

- `_i8250InitChannel`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x173bcc to 0x173c2c.
  Established. The indirect call sites include leading pairs `[[sp+4]+32]`, `0x3` (`call` at
  0x173be4), `[[sp+4]+40]`, `0xb` (`call` at 0x173bf2), and `[[sp+4]+48]`, `0x1` (`call` at
  0x173c0c). Another indirect site presents candidate zero `[[sp+4]+64]` and unknown remaining
  values (`call` at 0x173bfe). The `_intUnlock` site has candidate zero `ret(_intLock)` (`call` at
  0x173c12). Hardware meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x173bd1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x173bdd | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x173be1 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x173beb | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x173bef | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x173bf7 | load | 4 | `[sp+4] +0x40` | `-` |
| `mov` at 0x173bfb | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x173c05 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x173c09 | load | 4 | `[sp+4] +0x1c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x173bd4 | `_intLock 0x16fb78` | `ebx` | `esi` | `ebp` | `?` | `?` | `?` |
| `call` at 0x173be4 | `(unknown callee) (unknown target)` | `[[sp+4]+32]` | `0x3` | `?` | `?` | `?` | `?` |
| `call` at 0x173bf2 | `(unknown callee) (unknown target)` | `[[sp+4]+40]` | `0xb` | `?` | `?` | `?` | `?` |
| `call` at 0x173bfe | `(unknown callee) (unknown target)` | `[[sp+4]+64]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x173c0c | `(unknown callee) (unknown target)` | `[[sp+4]+48]` | `0x1` | `?` | `?` | `?` | `?` |
| `call` at 0x173c12 | `_intUnlock 0x16fb88` | `ret(_intLock)` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x173c1f | `eax` | `ret(_intUnlock)` |

## `_i8250Ioctl`

- `_i8250Ioctl`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x173c2c to 0x173e5c.
  Established. The range branch is taken when `[sp+8]-4099 > 0x6 (unsigned, 32-bit)` (`ja` at
  0x173c43), reaching the known-path exit value `0x47` (`ret` at 0x173e4c); its fall-through reaches
  a computed jump whose four-byte operand has an unknown effective address (`jmp` at 0x173c49).
  Selector-only calls, stores and return values remain excluded.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x173c35 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x173c38 | load | 4 | `sp +0x8` | `-` |
| `jmp` at 0x173c49 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ja` at 0x173c43 | `[sp+8]-4099 > 0x6 (unsigned, 32-bit)` | `[sp+8]-4099 <= 0x6 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x173e4c | `eax` | `0x47` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x173c49).

## `_i8250Int`

- `_i8250Int`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x173e5c to 0x173f3c.
  Established. The first indirect call presents candidate zero `[[sp+4]+52]` (`call` at 0x173e70). A
  later indirect site has leading candidates `[[sp+4]+12]` and `sp-5` (`call` at 0x173ee2); its
  result branch falls through when `(ret(unknown) & ret(unknown)) == 0x0` (`jne` at 0x173ee9),
  reaching the byte load at `sp -0x5` (`movsx` at 0x173eeb). The later indirect call retains unknown
  candidate values (`call` at 0x173f05). No byte-transfer effect is inferred.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x173e64 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x173e69 | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x173e6d | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x173e80 | load | 4 | `[sp+4] +0x24` | `-` |
| `cmp` at 0x173e91 | load | 4 | `[sp+4] +0x8` | `-` |
| `mov` at 0x173e97 | load | 4 | `[sp+4] +0x40` | `-` |
| `mov` at 0x173e9b | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x173ea9 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x173ead | load | 4 | `[sp+4] +0x8` | `-` |
| `mov` at 0x173ebc | load | 4 | `[sp+4] +0x40` | `-` |
| `mov` at 0x173ec0 | load | 4 | `[sp+4] +0x18` | `-` |
| `cmp` at 0x173ed1 | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x173edb | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x173edf | load | 4 | `[sp+4] +0x4` | `-` |
| `movsx` at 0x173eeb | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x173ef0 | load | 4 | `[sp+4] +0x40` | `-` |
| `mov` at 0x173efe | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x173f02 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x173f0a | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x173f0e | load | 4 | `[sp+4] +0x18` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x173e70 | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `ebx` | `esi` | `?` | `ebp` | `?` |
| `call` at 0x173e9e | `(unknown callee) (unknown target)` | `[[sp+4]+64]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x173eb0 | `(unknown callee) (unknown target)` | `[[sp+4]+16]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x173ec3 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x173ee2 | `(unknown callee) (unknown target)` | `[[sp+4]+12]` | `sp-5` | `?` | `?` | `?` | `?` |
| `call` at 0x173f05 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x173f11 | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x173e7e | `? != 0x6` | `? == 0x6` |
| `jne` at 0x173e8f | `? != 0x4` | `? == 0x4` |
| `je` at 0x173e95 | `[[sp+4]+8] == 0x0` | `[[sp+4]+8] != 0x0` |
| `jne` at 0x173ecf | `? != 0x2` | `? == 0x2` |
| `je` at 0x173ed5 | `[[sp+4]+4] == 0x0` | `[[sp+4]+4] != 0x0` |
| `jne` at 0x173ee9 | `(ret(unknown) & ret(unknown)) != 0x0` | `(ret(unknown) & ret(unknown)) == 0x0` |
| `jne` at 0x173f1b | `(? & 0x1) != 0x0` | `(? & 0x1) == 0x0` |
| `jle` at 0x173f23 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x173f31 | `eax` | `?` |

## `_i8250Startup`

- `_i8250Startup`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x173f3c to 0x173f5c.
  Established. The two-byte comparison branch is taken when `([[sp+4]+22]:2 & 0xffff) == 0x1` (`je`
  at 0x173f47), bypassing the indirect call. Falling through reaches that call with leading
  candidates `[[sp+4]+48]` and `0x3` (`call` at 0x173f52). The common exit carries `0x0` in `eax`
  (`ret` at 0x173f59), without an established startup effect.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x173f3f | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x173f42 | load | 2 | `[sp+4] +0x16` | `-` |
| `mov` at 0x173f4b | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x173f4f | load | 4 | `[sp+4] +0x1c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x173f52 | `(unknown callee) (unknown target)` | `[[sp+4]+48]` | `0x3` | `ebp` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x173f47 | `([[sp+4]+22]:2 & 0xffff) == 0x1` | `([[sp+4]+22]:2 & 0xffff) != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x173f59 | `eax` | `0x0` |

## `_i8250PRxChar`

- `_i8250PRxChar`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x173f5c to 0x173f9c.
  Established. The first indirect call has candidate zero `[[sp+4]+36]` (`call` at 0x173f6e); its
  following branch falls through when `(? & 0x1) != 0x0` (`je` at 0x173f75), reaching another
  indirect call with candidate zero `[[sp+4]+64]` (`call` at 0x173f7e). The byte store through
  `[sp+8] +0x0` retains unknown value (`mov` at 0x173f80). The tested value and callee effects stay
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x173f61 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x173f64 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x173f67 | load | 4 | `[sp+4] +0x24` | `-` |
| `mov` at 0x173f6b | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x173f77 | load | 4 | `[sp+4] +0x40` | `-` |
| `mov` at 0x173f7b | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x173f80 | store | 1 | `[sp+8] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x173f6e | `(unknown callee) (unknown target)` | `[[sp+4]+36]` | `ebx` | `esi` | `ebp` | `?` | `?` |
| `call` at 0x173f7e | `(unknown callee) (unknown target)` | `[[sp+4]+64]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x173f75 | `(? & 0x1) == 0x0` | `(? & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x173f99 | `eax` | `?` |

## `_i8250PTxChar`

- `_i8250PTxChar`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x173f9c to 0x173fe8.
  Established. A byte store copies `[sp+8]:1` to `sp -0x8` (`mov` at 0x173fa9). The first indirect
  call has candidate zero `[[sp+4]+36]` (`call` at 0x173fb3). The result branch falls through when
  `(? & 0x40) != 0x0` (`je` at 0x173fba), reaching the second indirect call with candidate zero
  `[[sp+4]+64]` and unknown candidate one (`call` at 0x173fc8). No transmitted value is established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x173fa3 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x173fa6 | load | 1 | `sp +0x8` | `-` |
| `mov` at 0x173fa9 | store | 1 | `sp -0x8` | `[sp+8]:1` |
| `mov` at 0x173fac | load | 4 | `[sp+4] +0x24` | `-` |
| `mov` at 0x173fb0 | load | 4 | `[sp+4] +0x18` | `-` |
| `movsx` at 0x173fbc | load | 1 | `sp -0x8` | `-` |
| `mov` at 0x173fc1 | load | 4 | `[sp+4] +0x40` | `-` |
| `mov` at 0x173fc5 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x173fe1 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x173fb3 | `(unknown callee) (unknown target)` | `[[sp+4]+36]` | `ebx` | `?` | `ebp` | `?` | `?` |
| `call` at 0x173fc8 | `(unknown callee) (unknown target)` | `[[sp+4]+64]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x173fba | `(? & 0x40) == 0x0` | `(? & 0x40) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x173fe7 | `eax` | `?` |

## `_i8250CallbackInstall`

- `_i8250CallbackInstall`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x173b1c to 0x173b6c.
  Established. Taking the first branch when `[sp+8] == 0x1` (`je` at 0x173b2f) reaches four-byte
  stores of `[sp+12]` through `[sp+4] +0x4` (`mov` at 0x173b3c) and `[sp+16]` through `[sp+4] +0xc`
  (`mov` at 0x173b3f). The separate branch taken when `[sp+8] == 0x2` (`je` at 0x173b34) reaches
  stores of `[sp+12]` through `[sp+4] +0x8` (`mov` at 0x173b4c) and `[sp+16]` through `[sp+4] +0x10`
  (`mov` at 0x173b4f). No stored-value execution is claimed.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x173b20 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x173b23 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x173b26 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x173b29 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x173b3c | store | 4 | `[sp+4] +0x4` | `[sp+12]` |
| `mov` at 0x173b3f | store | 4 | `[sp+4] +0xc` | `[sp+16]` |
| `mov` at 0x173b4c | store | 4 | `[sp+4] +0x8` | `[sp+12]` |
| `mov` at 0x173b4f | store | 4 | `[sp+4] +0x10` | `[sp+16]` |
| `mov` at 0x173b61 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x173b2f | `[sp+8] == 0x1` | `[sp+8] != 0x1` |
| `je` at 0x173b34 | `[sp+8] == 0x2` | `[sp+8] != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x173b67 | `eax` | `?` |

## `_sysIntInitPIC`

- `_sysIntInitPIC`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1082f8 to 0x108398.
  Established. The `_sysOutByte` sites include leading pairs `0x20`, `0x11` (`call` at 0x1082ff),
  `0xa0`, `0x11` (`call` at 0x108338), `0x21`, `0xfffffffb` (`call` at 0x10837b), and `0xa1`,
  `0xffffffff` (`call` at 0x10838a). Other call candidates remain listed separately. Two byte loads
  read absolute `0x1a9bf8` (`movsx` at 0x108307, `mov` at 0x108340). No interrupt-controller
  register meanings are assigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movsx` at 0x108307 | load | 1 | `abs 0x1a9bf8` | `-` |
| `mov` at 0x108340 | load | 1 | `abs 0x1a9bf8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1082ff | `_sysOutByte 0x1081d0` | `0x20` | `0x11` | `ebp` | `?` | `?` | `?` |
| `call` at 0x108311 | `_sysOutByte 0x1081d0` | `0x21` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10831d | `_sysOutByte 0x1081d0` | `0x21` | `0x4` | `?` | `?` | `?` | `?` |
| `call` at 0x108329 | `_sysOutByte 0x1081d0` | `0x21` | `0x1` | `?` | `?` | `?` | `?` |
| `call` at 0x108338 | `_sysOutByte 0x1081d0` | `0xa0` | `0x11` | `?` | `?` | `?` | `?` |
| `call` at 0x108351 | `_sysOutByte 0x1081d0` | `0xa1` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108360 | `_sysOutByte 0x1081d0` | `0xa1` | `0x2` | `?` | `?` | `?` | `?` |
| `call` at 0x10836f | `_sysOutByte 0x1081d0` | `0xa1` | `0x1` | `?` | `?` | `?` | `?` |
| `call` at 0x10837b | `_sysOutByte 0x1081d0` | `0x21` | `0xfffffffb` | `?` | `?` | `?` | `?` |
| `call` at 0x10838a | `_sysOutByte 0x1081d0` | `0xa1` | `0xffffffff` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108392 | `eax` | `ret(_sysOutByte)` |

## `_sysIntEOI`

- `_sysIntEOI`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108398 to 0x1083b8.
  Established. Two `_sysOutByte` calls present leading pairs `0x20`, `0x20` (`call` at 0x10839f) and
  `0xa0`, `0x20` (`call` at 0x1083ae). The exit reports `ret(_sysOutByte)` in `eax` (`ret` at
  0x1083b6). An interrupt acknowledgement effect is not established by the routine name alone.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10839f | `_sysOutByte 0x1081d0` | `0x20` | `0x20` | `ebp` | `?` | `?` | `?` |
| `call` at 0x1083ae | `_sysOutByte 0x1081d0` | `0xa0` | `0x20` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1083b6 | `eax` | `ret(_sysOutByte)` |

## `_sysIntDisablePIC`

- `_sysIntDisablePIC`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1083b8 to 0x108428.
  Established. Taking the signed branch when `[sp+4] > 0x7 (signed, 32-bit)` (`jg` at 0x1083c2)
  reaches `_sysInByte` with candidate zero `0xa1` (`call` at 0x1083ed); its fall-through instead
  reaches `_sysInByte` with candidate zero `0x21` (`call` at 0x1083c6). The common `_sysOutByte`
  site has unknown candidate values (`call` at 0x10840c). The exit value is `0x0` (`ret` at
  0x108419).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1083bc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x108413 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1083c6 | `_sysInByte 0x1081b0` | `0x21` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x1083ed | `_sysInByte 0x1081b0` | `0xa1` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x10840c | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x1083c2 | `[sp+4] > 0x7 (signed, 32-bit)` | `[sp+4] <= 0x7 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108419 | `eax` | `0x0` |

## `_sysIntEnablePIC`

- `_sysIntEnablePIC`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108428 to 0x108498.
  Established. The initial branch falls through when `[sp+4] <= 0x7 (signed, 32-bit)` (`jg` at
  0x108432), reaching `_sysInByte` with candidate zero `0x21` (`call` at 0x108436). Taking that
  branch instead reaches the separate `_sysInByte` call with candidate zero `0xa1` (`call` at 0x10845d).
  The common `_sysOutByte` call retains unknown candidates (`call` at 0x10847c); its output value
  and hardware effect remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10842c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x108483 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108436 | `_sysInByte 0x1081b0` | `0x21` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x10845d | `_sysInByte 0x1081b0` | `0xa1` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x10847c | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x108432 | `[sp+4] > 0x7 (signed, 32-bit)` | `[sp+4] <= 0x7 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108489 | `eax` | `0x0` |
