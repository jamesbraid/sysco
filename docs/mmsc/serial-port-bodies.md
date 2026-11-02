# MMSC serial-driver and port-I/O bodies

These entries cover only MMSC 2.3 in `6.5.30/usr/cpu/firmware/mmscfw.bin`,
whose creation-date string is `Wed Dec 08 12:45:18 1999`. The image is mapped
at `0x108000` as described in [the load map](load-map.md). Each entry names
its analysed span. The serial-driver spans end at the next text symbol,
an inferred boundary. The `_sysA20on` and four port-helper spans cross local
nlist labels and end at the next global text symbol. These extended spans
include the local exit labels and their returns, with inferred end boundaries.

The source image SHA-256 is
`6c3db7ffd7fb4cc43ed02146f0c5bea0e85338def2530689c9990f877f2ddedc`.

In symbolic expressions, `sp` is the stack pointer at entry. Brackets
denote a loaded value. Unknown values and unresolved transfers remain
unknown. Candidate call arguments name locations under the assumed cdecl32
convention, with no claim about the callee’s argument count.

The [fact sheets](disasm/serial-port-facts.txt) include the instruction
listings and the conditions used below.

## `_ns16550CallbackInstall`

- `_ns16550CallbackInstall`, `6.5.30/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x17a19c to 0x17a210.
  Established. Selects between two pairs of four-byte stores. `je` takes the first path when `[sp+8]
  == 0x1` (`je` at 0x17a1ae): a value from `[sp+12]` is stored through `[sp+4] +0x4`, and a value
  from `[sp+16]` through `[sp-8] +0xc` (`mov` at 0x17a1c2, `mov` at 0x17a1cb). If the first
  comparison falls through, the next `je` takes the second path for `[sp+8] == 0x2` (`je` at
  0x17a1b3); its corresponding four-byte stores use `[sp+4] +0x8` and `[sp-8] +0x10` (`mov` at
  0x17a1e2, `mov` at 0x17a1eb). Each selected path loads EAX with zero (`xor` at 0x17a1ce, `xor` at
  0x17a1ee). Falling through both comparisons loads 0x47 (`mov` at 0x17a1fc). The merged return
  value is unknown, and there is no call in this body.

## `_ns16550DummyCallback`

- `_ns16550DummyCallback`, `6.5.30/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1790ec to 0x17910c.
  Established. Supplies the fixed result 0xffffffff: the constant load reaches the return without a
  call or a device access (`mov` at 0x1790ef, `ret` at 0x1790ff). The only recorded accesses save
  and reload the stack frame (`push` at 0x1790ec, `pop` at 0x1790fe).

## `_ns16550Hup`

- `_ns16550Hup`, `6.5.30/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1796ec to 0x1797cc.
  Established. Runs `_intLock` before the indirect operations and `_intUnlock` afterward, with
  `ret(_intLock)` as the latter's first candidate argument (`call` at 0x1796f1, `call` at 0x1797a3).
  The first indirect call has candidate arguments `[[sp+4]+52]` and zero (`call` at 0x179707).
  Further indirect calls include `[[sp+4]+68]` and `[[sp+4]+56]` as their first candidate arguments
  (`call` at 0x179747, `call` at 0x179774). Stores through `[sp+4]` write unknown byte values at
  +0x19 and +0x1a and an unknown four-byte value at +0x1c (`mov` at 0x17971a, `mov` at 0x17975a,
  `mov` at 0x17972b). `jne` skips the final indirect operation when the two-byte field at `[sp+4]
  +0x5c` is not equal to 2 (`mov` at 0x17977c, `jne` at 0x179784). Equality falls through to that call
  (`jne` at 0x179784, `call` at 0x17979d). The return EAX is zero (`xor` at 0x1797ab, `ret` at
  0x1797c4).

## `_ns16550InitChannel`

- `_ns16550InitChannel`, `6.5.30/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x17916c to 0x17945c.
  Established. Writes byte 3 through `[sp+4] +0x1b`, then byte 7 and later 0x89 through `[sp+4]
  +0x1a` (`mov` at 0x17917c, `mov` at 0x1791bd, `mov` at 0x1791fe); its indirect calls include first
  candidate arguments `[[sp+4]+64]`, `[[sp+4]+56]`, `[[sp+4]+84]` and `[[sp+4]+88]`, with unknown
  targets (`call` at 0x1791af, `call` at 0x1791f0, `call` at 0x1792fe, `call` at 0x179330). Two
  multiplications are unsupported, so their results are not established (`mul` at 0x179288, `mul` at
  0x1792b9). Later four-byte stores write through `[sp-8] +0x20` and `[sp-8] +0x24`, with values
  `[sp-12]` and unknown, respectively (`mov` at 0x17929a, `mov` at 0x1792ce). A byte 3 is stored
  through `[sp+4] +0x19`, then an indirect call has candidate arguments `[[sp+4]+52]` and zero
  (`mov` at 0x179372, `call` at 0x179391). `je` takes the alternate path when `[sp-8] == 0x0` (`je`
  at 0x1793b2). Both recognised arms store the four-byte value 0xb through `[sp+4] +0x1c` (`je` at
  0x1793b2, `mov` at 0x1793bd, `mov` at 0x1793d5). The merged return is an unknown callee's result
  (`ret` at 0x179458). The meanings of the stored fields remain unknown.

## `_ns16550Int`

- `_ns16550Int`, `6.5.30/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x179e2c to 0x17a00c.
  Established for known control-flow edges. The entry path calls an unresolved function with
  `[[sp+4]+60]` as its first candidate argument, then another with `[[sp+4]+52]` and zero (`call` at
  0x179e40, `call` at 0x179e5e). `ja` takes the common exit route when an unknown unsigned 32-bit
  value is greater than 0xc (`ja` at 0x179e6c). The smaller-or-equal path falls through to dispatch
  (`ja` at 0x179e6c). The indirect jump target is unresolved (`jmp` at 0x179e79). Accesses and calls
  reached only through that dispatch remain unknown. The recognised common exit includes an indirect
  call whose first candidate argument is `[[sp+4]+52]` and whose next argument is unknown (`call` at
  0x179ff3). No complete list of interrupt-handler effects is established.

## `_ns16550IntEx`

- `_ns16550IntEx`, `6.5.30/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x179e1c to 0x179e2c.
  Established. Leaves EAX unchanged while establishing and dismantling a stack frame (`push` at
  0x179e1c, `mov` at 0x179e1d, `mov` at 0x179e1f, `pop` at 0x179e21, `ret` at 0x179e22); its facts
  contain no calls and no accesses outside that frame. A hardware service performed by this entry is
  not established.

## `_ns16550Ioctl`

- `_ns16550Ioctl`, `6.5.30/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1798cc to 0x179d5c.
  Established for known control-flow edges; its entry guard compares `[sp+8]-4099` with 0xa as an
  unsigned 32-bit value (`cmp` at 0x1798e2). `ja` takes the default path when that value is greater,
  and falls through otherwise (`ja` at 0x1798e6). The default path loads 0x47 and reaches the return
  with that value in EAX (`mov` at 0x179d3c, `mov` at 0x179d41, `ret` at 0x179d55). The other route
  reads an indirect jump pointer from entry stack -0x8 and transfers to an unknown target (`jmp` at
  0x1798f9). The dispatch targets and case-handler effects are unknown in this analysis, so the
  default result does not describe every request.

## `_ns16550Open`

- `_ns16550Open`, `6.5.30/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1797cc to 0x1798cc.
  Established. Starts with an indirect call whose first candidate argument is `[[sp+4]+76]` (`call`
  at 0x1797e1). `je` takes the short path for `(? & ?) == 0x0`, and falls through for a nonzero
  result (`je` at 0x1797f7). The operands remain unknown. The longer path calls `_intLock`, writes
  unknown bytes through `[sp+4] +0x19` twice and +0x1a once, and writes an unknown four-byte value
  at +0x1c (`call` at 0x1797fd, `mov` at 0x179813, `mov` at 0x179824, `mov` at 0x179864, `mov` at
  0x179835). `jne` skips the last indirect call when the two-byte field at `[sp+4] +0x5c` is not 2
  (`mov` at 0x179886, `jne` at 0x17988e, `call` at 0x1798a7). `_intUnlock` receives `[sp-12]` as its
  first candidate argument (`call` at 0x1798b0). Both recognised paths reach EAX zero (`xor` at
  0x1798b8, `ret` at 0x1798c4).

## `_ns16550OptsSet`

- `_ns16550OptsSet`, `6.5.30/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x17945c to 0x1796ec.
  Established. Rejects a zero `[sp+4]` by following `je` to the constant 0xffffffff path (`je` at
  0x179465, `mov` at 0x17947c). Another rejection branch depends on unknown operands (`jne` at
  0x179471). Recognised paths write byte values 0, 1, 2 or 3 through `[sp+4] +0x1b`, with later
  writes to that byte also present (`mov` at 0x1794bf, `mov` at 0x1794cf, `mov` at 0x1794df, `mov`
  at 0x1794ef, `mov` at 0x179514, `mov` at 0x179525, `mov` at 0x17956a, `mov` at 0x17958a, `mov` at
  0x1795a5). An indirect call has `[[sp+4]+52]` and zero as candidate arguments (`call` at
  0x1795bb). Subsequent operations include `_intLock`, indirect calls with first candidate arguments
  `[[sp+4]+64]`, `[[sp+4]+68]` and `[[sp+4]+56]`, and a byte 0x87 stored through `[sp+4] +0x1a`
  (`call` at 0x17961d, `call` at 0x17963b, `call` at 0x179659, `call` at 0x17967c, `mov` at
  0x179661). `jne` skips the final indirect operation for a two-byte field at `[sp+4] +0x5c` unequal
  to 2 (`mov` at 0x17969f, `jne` at 0x1796a7, `call` at 0x1796c0). `_intUnlock` receives
  `ret(_intLock)` as its first candidate argument, after which `[sp+8]` is stored as a four-byte
  value through `[sp+4] +0x68` (`call` at 0x1796c6, `mov` at 0x1796d4). The merged EAX result is
  unknown.

## `_ns16550TxStartup`

- `_ns16550TxStartup`, `6.5.30/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x17a00c to 0x17a0cc.
  Established. Continues past `jne` only when the two-byte field at `[sp+4] +0x5c` equals 2 (`mov`
  at 0x17a016, `jne` at 0x17a01e). A different value takes the return path (`mov` at 0x17a016, `jne`
  at 0x17a01e). Subsequent `je` conditions retain unknown operands, so their hardware meaning is
  unknown (`je` at 0x17a031, `je` at 0x17a068). The body includes byte stores through `[sp+4] +0x19`
  whose values are unresolved (`mov` at 0x17a041, `mov` at 0x17a078, `mov` at 0x17a09a). The
  indirect call at 0x17a059 has `[[sp+4]+76]` in its first candidate argument location, and the
  common later call has `[[sp+4]+52]` followed by an unknown value (`call` at 0x17a059, `call` at
  0x17a0b4). The merged EAX result is unknown.

## `_sysA20on`

- `_sysA20on`, `6.5.30/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108100 to 0x1081a0.
  Established. Combines port writes with two memory comparisons. It calls `_sysWait` before writing
  byte 0xd1 to port 0x64, calls it again before byte 0xdf to port 0x60, and again before byte 0xff
  to port 0x64 (`call` at 0x108100, `out` at 0x10810a, `call` at 0x10810c, `out` at 0x108116, `call`
  at 0x108118, `out` at 0x108122). Another `_sysWait` call precedes clearing four-byte values at
  absolute addresses zero and 0x100000 and writing 0x1234567 at zero (`call` at 0x108124, `mov` at
  0x108133, `mov` at 0x108139, `mov` at 0x10813f). `jne` exits when the four-byte value at 0x100000
  is not equal to 0x1234567 (`cmp` at 0x108145, `jne` at 0x10814b). Equality falls through to byte 2
  written at port 0x92 and a byte read from that port (`out` at 0x108152, `in` at 0x108156). The
  `loope` condition is unresolved (`loope` at 0x10815a). A second pair of four-byte zero stores and
  signature store precedes another comparison at 0x100000 (`mov` at 0x108166, `mov` at 0x10816c,
  `mov` at 0x108172, `cmp` at 0x108178); its `jne` exits on a differing four-byte value (`cmp` at
  0x108178, `jne` at 0x10817e). Equality falls through to storing 0xffffffff at 0x108190 (`jne` at
  0x10817e, `mov` at 0x108180). The recognised return has EAX zero (`ret` at 0x10818a).

## `_sysInByte`

- `_sysInByte`, `6.5.30/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081b0 to 0x1081c0.
  Established. Reads one byte from an I/O port (`in` at 0x1081b9). The port address remains unknown
  in the facts. A prior four-byte load reads entry stack +0x4 (`mov` at 0x1081b0). The recorded
  range follows the local exit label through the return (`ret` at 0x1081bc). The final EAX value is
  unknown.

## `_sysInWord`

- `_sysInWord`, `6.5.30/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081c0 to 0x1081d0.
  Established. Performs a two-byte I/O input (`in` at 0x1081c9), after reading a four-byte value at
  entry stack +0x4 (`mov` at 0x1081c0). The effective port and returned EAX value are unknown; its
  return instruction is at the local nlist exit label (`ret` at 0x1081cd).

## `_sysOutByte`

- `_sysOutByte`, `6.5.30/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081d0 to 0x1081e0.
  Established. Issues a one-byte port write, whose address and value are unknown in the facts (`out`
  at 0x1081d8). Earlier loads read four-byte values at entry stack +0x4 and +0x8 (`mov` at 0x1081d0,
  `mov` at 0x1081d4). Control reaches the return through the local exit label (`ret` at 0x1081db).
  No call is present.

## `_sysOutWord`

- `_sysOutWord`, `6.5.30/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081e0 to 0x1081f0.
  Established. Writes two bytes to an unresolved I/O port (`out` at 0x1081e8). The facts list
  separate four-byte reads at entry stack +0x4 and +0x8 (`mov` at 0x1081e0, `mov` at 0x1081e4), but
  leave the port and stored value unknown. The local exit label leads to the return (`ret` at
  0x1081ec).
