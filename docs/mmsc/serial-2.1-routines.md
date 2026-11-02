# MMSC 2.1 serial and port-I/O routines

The source for these entries is `6.5.5/usr/cpu/firmware/mmscfw.bin`, MMSC 2.1.
SHA-256: `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
The [load map](load-map.md) places the x86 a.out image at `0x108000`.
Serial-driver spans end at the next text symbol. The five `_sys` spans extend
across local nlist labels to the next global text symbol so their returns are included.
These boundaries are inferred and can include padding or data.

`sp` is the entry stack pointer. Brackets denote loaded values. Call candidates
assume cdecl32 and do not establish arity. Device roles of context fields and
unresolved callees remain unknown. Indirect dispatch limits are stated per entry.

[Facts and decoded listings](disasm/serial-2.1-facts.txt).

## `_ns16550CallbackInstall`

- `_ns16550CallbackInstall`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x17a17c to 0x17a1f0.
  Established. Tests `[sp+8]` against 1 and then 2, with each equality following its `je` to a
  different pair of stores (`je` at 0x17a18e, `je` at 0x17a193). The first pair writes four-byte
  `[sp+12]` through `[sp+4] +0x4` and four-byte `[sp+16]` through `[sp-8] +0xc` (`mov` at 0x17a1a2,
  `mov` at 0x17a1ab). The second writes those values through `[sp+4] +0x8` and `[sp-8] +0x10` (`mov`
  at 0x17a1c2, `mov` at 0x17a1cb). Each selected arm loads EAX zero (`xor` at 0x17a1ae, `xor` at
  0x17a1ce). Falling through both comparisons instead reaches the 0x47 EAX load (`je` at 0x17a18e,
  `je` at 0x17a193, `mov` at 0x17a1dc). The merged return is unknown (`ret` at 0x17a1ef).

## `_ns16550DummyCallback`

- `_ns16550DummyCallback`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1790cc to 0x1790ec.
  Established. Loads EAX with 0xffffffff and returns that value (`mov` at 0x1790cf, `ret` at
  0x1790df). The only accesses collected for this body are the four-byte stack-frame save and reload
  at `sp -0x4` (`push` at 0x1790cc, `pop` at 0x1790de). No call or I/O access appears in the
  routine's facts.

## `_ns16550Hup`

- `_ns16550Hup`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1796cc to 0x1797ac.
  Established. Brackets the indirect-call region with `_intLock` and `_intUnlock`, passing
  `ret(_intLock)` as the unlock call's first candidate (`call` at 0x1796d1, `call` at 0x179783). The
  initial unresolved call's candidates begin `[[sp+4]+52]` and zero (`call` at 0x1796e7). Byte
  stores through `[sp+4] +0x19` and `[sp+4] +0x1a`, and a four-byte store through `[sp+4] +0x1c`,
  have unknown values (`mov` at 0x1796fa, `mov` at 0x17973a, `mov` at 0x17970b). Two intervening
  unresolved callees have first candidates `[[sp+4]+68]` and `[[sp+4]+56]` (`call` at 0x179727,
  `call` at 0x179754). The two-byte read through `[sp+4] +0x5c` feeds `jne`, taken when the value
  is not equal to 2 to skip the final indirect call (`mov` at 0x17975c, `jne` at 0x179764, `call` at
  0x17977d). The recorded EAX result is zero (`ret` at 0x1797a4).

## `_ns16550InitChannel`

- `_ns16550InitChannel`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x17914c to 0x17943c.
  Established. Stores byte 3 through `[sp+4] +0x1b`, and bytes 7 then 0x89 through `[sp+4] +0x1a`
  (`mov` at 0x17915c, `mov` at 0x17919d, `mov` at 0x1791de). Calls include unresolved targets with
  first candidates `[[sp+4]+64]`, `[[sp+4]+56]`, `[[sp+4]+84]` and `[[sp+4]+88]` (`call` at
  0x17918f, `call` at 0x1791d0, `call` at 0x1792de, `call` at 0x179310). Two unsupported
  multiplications leave their computed values unestablished (`mul` at 0x179268, `mul` at 0x179299).
  Four-byte stores through `[sp-8] +0x20` and `[sp-8] +0x24` have values `[sp-12]` and unknown
  (`mov` at 0x17927a, `mov` at 0x1792ae). Later byte 3 goes through `[sp+4] +0x19`, before an
  indirect call with candidates `[[sp+4]+52]` and zero (`mov` at 0x179352, `call` at 0x179371). The
  only conditional branch takes `je` for zero `[sp-8]`; both recognised arms contain a four-byte
  store of 0xb through `[sp+4] +0x1c` (`je` at 0x179392, `mov` at 0x17939d, `mov` at 0x1793b5). The
  last unresolved call's first candidate is `[[sp+4]+68]` (`call` at 0x17942d).

## `_ns16550Int`

- `_ns16550Int`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x179e0c to 0x179fec.
  Established. Known edges only. Two initial calls have unknown targets, with first candidate
  `[[sp+4]+60]` at the first and candidates `[[sp+4]+52]`, zero at the second (`call` at 0x179e20,
  `call` at 0x179e3e). `ja` selects the common exit route for an unknown unsigned value greater than
  0xc (`ja` at 0x179e4c). The other condition falls through to an unresolved indirect `jmp` (`ja` at
  0x179e4c, `jmp` at 0x179e59). The recognised common exit includes a call with unknown target and
  candidates `[[sp+4]+52]`, unknown (`call` at 0x179fd3). Case-handler accesses and calls cannot be
  listed completely from this unresolved dispatch.

## `_ns16550IntEx`

- `_ns16550IntEx`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x179dfc to 0x179e0c.
  Established. Establishes a stack frame and immediately dismantles it without changing the recorded
  EAX value (`push` at 0x179dfc, `mov` at 0x179dfd, `mov` at 0x179dff, `pop` at 0x179e01, `ret` at
  0x179e02); its accesses are confined to the four-byte save and reload at `sp -0x4` (`push` at
  0x179dfc, `pop` at 0x179e01). The tool finds no call in this body; any hardware service remains
  unknown.

## `_ns16550Ioctl`

- `_ns16550Ioctl`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1798ac to 0x179d3c.
  Established. Known edges only. The `ja` guard takes the default route when `[sp+8]-4099 > 0xa`
  unsigned, falling through otherwise (`ja` at 0x1798c6). On that default route EAX receives 0x47
  and reaches the recorded return (`mov` at 0x179d1c, `mov` at 0x179d21, `ret` at 0x179d35). The
  fall-through route reads a four-byte indirect-jump target from `sp -0x8`, then transfers to an
  unknown destination (`jmp` at 0x1798d9). The unresolved dispatch prevents a complete description
  of request-specific effects or results.

## `_ns16550Open`

- `_ns16550Open`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1797ac to 0x1798ac.
  Established. The first indirect call has unknown target and first candidate `[[sp+4]+76]` (`call`
  at 0x1797c1). A following `je` skips the longer region for `(? & ?) == 0`, with operands unknown
  (`je` at 0x1797d7). The fall-through region includes `_intLock`, two unknown byte stores through
  `[sp+4] +0x19`, one through `[sp+4] +0x1a`, and an unknown four-byte store through `[sp+4] +0x1c`
  (`call` at 0x1797dd, `mov` at 0x1797f3, `mov` at 0x179804, `mov` at 0x179844, `mov` at 0x179815);
  its unresolved calls include first candidates `[[sp+4]+68]` and `[[sp+4]+56]` (`call` at 0x179831,
  `call` at 0x17985e). A two-byte load through `[sp+4] +0x5c` controls `jne`, taken for a value
  unequal to 2 to bypass the final indirect call (`mov` at 0x179866, `jne` at 0x17986e, `call` at
  0x179887). `_intUnlock` receives `[sp-12]` as its first candidate, and the common exit has EAX
  zero (`call` at 0x179890, `ret` at 0x1798a4).

## `_ns16550OptsSet`

- `_ns16550OptsSet`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x17943c to 0x1796cc.
  Established. Takes `je` to the 0xffffffff EAX load for zero `[sp+4]` (`je` at 0x179445, `mov` at
  0x17945c). The other rejection test retains unknown operands (`jne` at 0x179451). Store sites
  through `[sp+4] +0x1b` include byte values 0, 1, 2 and 3, with later writes to that field also
  present (`mov` at 0x17949f, `mov` at 0x1794af, `mov` at 0x1794bf, `mov` at 0x1794cf,
  `mov` at 0x1794f4, `mov` at 0x179505, `mov` at 0x17954a, `mov` at 0x17956a, `mov` at 0x179585). An indirect
  call receives candidates `[[sp+4]+52]` and zero (`call` at 0x17959b). The later region includes
  `_intLock`, three unresolved calls with first candidates `[[sp+4]+64]`, `[[sp+4]+68]`,
  `[[sp+4]+56]`, and byte 0x87 stored through `[sp+4] +0x1a` (`call` at 0x1795fd, `call` at
  0x17961b, `call` at 0x179639, `call` at 0x17965c, `mov` at 0x179641). `jne` skips the last
  indirect call when the two-byte field through `[sp+4] +0x5c` is not equal to 2 (`mov` at 0x17967f,
  `jne` at 0x179687, `call` at 0x1796a0). After `_intUnlock` with first candidate `ret(_intLock)`, a
  four-byte store writes `[sp+8]` through `[sp+4] +0x68` (`call` at 0x1796a6, `mov` at 0x1796b4).

## `_ns16550TxStartup`

- `_ns16550TxStartup`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x179fec to 0x17a0ac.
  Established. Reads two bytes through `[sp+4] +0x5c`; `jne` goes directly to the exit region when
  this value is not equal to 2 (`mov` at 0x179ff6, `jne` at 0x179ffe). The remaining two `je` decisions
  resolve only to `(? & ?) == 0`, with unknown operands (`je` at 0x17a011, `je` at 0x17a048). Byte
  stores through `[sp+4] +0x19` have unknown values at three sites (`mov` at 0x17a021, `mov` at
  0x17a058, `mov` at 0x17a07a). The two indirect calls retain unknown targets and first candidates
  `[[sp+4]+76]` and `[[sp+4]+52]` respectively (`call` at 0x17a039, `call` at 0x17a094). The
  latter's second candidate and the merged exit value are unknown (`call` at 0x17a094, `ret` at
  0x17a09f).

## `_sysA20on`

- `_sysA20on`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108100 to 0x1081a0.
  Established. Calls `_sysWait` before each of three byte outputs: 0xd1 at port 0x64, 0xdf at port
  0x60 and 0xff at port 0x64 (`call` at 0x108100, `out` at 0x10810a, `call` at 0x10810c, `out` at
  0x108116, `call` at 0x108118, `out` at 0x108122). A fourth `_sysWait` precedes four-byte zero
  stores at absolute addresses zero and 0x100000, then a four-byte store of 0x1234567 at zero
  (`call` at 0x108124, `mov` at 0x108133, `mov` at 0x108139, `mov` at 0x10813f). `jne` exits if
  `[0x100000] != 0x1234567` (`jne` at 0x10814b). Equality falls through to byte output 2 at port
  0x92 and a byte input from that port (`jne` at 0x10814b, `out` at 0x108152, `in` at 0x108156). The
  backward `loope` condition is unresolved (`loope` at 0x10815a). A second set of four-byte writes
  clears zero and 0x100000 and writes 0x1234567 at zero (`mov` at 0x108166, `mov` at 0x10816c, `mov`
  at 0x108172). The second `jne` also exits on signature inequality; equality falls through to
  four-byte 0xffffffff at 0x108190 (`jne` at 0x10817e, `mov` at 0x108180). The known return has EAX
  zero (`ret` at 0x10818a).

## `_sysInByte`

- `_sysInByte`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081b0 to 0x1081c0.
  Established. Loads four bytes from entry stack +0x4 before one-byte input from an unresolved I/O
  port (`mov` at 0x1081b0, `in` at 0x1081b9). The local-label jump reaches the return, where EAX is
  unknown (`jmp` at 0x1081ba, `ret` at 0x1081bc). No callee appears between entry and return.

## `_sysInWord`

- `_sysInWord`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081c0 to 0x1081d0.
  Established. The port input is two bytes wide, while the prior read at entry stack +0x4 is
  four bytes wide (`in` at 0x1081c9, `mov` at 0x1081c0). The port address is unknown (`in` at
  0x1081c9). A jump to the local exit label reaches a return with unresolved EAX (`jmp` at
  0x1081cb, `ret` at 0x1081cd).

## `_sysOutByte`

- `_sysOutByte`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081d0 to 0x1081e0.
  Established. Reads four-byte values at entry stack +0x4 and +0x8 (`mov` at 0x1081d0, `mov` at
  0x1081d4). The subsequent port output writes one byte with unknown effective port and unknown
  value (`out` at 0x1081d8). The recorded return preserves EAX as `[sp+8]` (`ret` at 0x1081db).

## `_sysOutWord`

- `_sysOutWord`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081e0 to 0x1081f0.
  Established. Combines two four-byte stack reads, at entry +0x4 and +0x8, with a two-byte I/O
  output (`mov` at 0x1081e0, `mov` at 0x1081e4, `out` at 0x1081e8). That output's port and value
  remain unknown (`out` at 0x1081e8). The local-label transfer reaches a return whose EAX is
  `[sp+8]` (`jmp` at 0x1081ea, `ret` at 0x1081ec).
