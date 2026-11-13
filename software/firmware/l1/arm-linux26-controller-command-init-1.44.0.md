# ARM Linux 2.6 controller command and initialization routines in 1.44.0

Read from `6.5.30/usr/cpu/firmware/sysco/l1.bin`, package `6.5.30:l1#6` (version 1.44.0),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0xa936a8`.
Carrier SHA-256: `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
Package SHA-256: `e57ab8ecd48858e39ecdd0050a23e444a491c35e87f9d8ebb48881fc25229a06`.
Inner executable SHA-256: `b33154e53b00531623db6ff365e80b1c1825bc9427961c59fdc132c24f0c819e`.
The ELF identifies 32-bit little-endian ARM. Its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1629656 bytes.
Segment SHA-256: `f6491062ef0a2afd254f8510f82b4814c563c9256691f9a2b393cf6e84ba7fd8`. Analysed routine spans use the exact
ELF function symbols and sizes. They can include trailing literal words after a return.

`sp` denotes the entry stack pointer. Register expressions such as `r0` denote entry
values, and brackets denote loaded values. Decimal numbers inside expressions retain the
exact tool notation. Call candidates use the assumed ARM AAPCS
register locations r0 through r3 with unknown arity. Custom calling conventions remain unknown.
Call lines include every reported call instruction and every printed register slot.
Unknown values and predicates include unsupported instruction effects and merged paths.
Numeric call targets have unknown callee names in the flat-image analysis. Role: unknown. Pointee meanings remain unknown. Selected access and branch facts below cover known control-flow edges. The
full fact sheets retain all reported accesses and implicit stack operations. Separate loads at the same printed address need not observe the same value.
Computed transfers with unresolved edges are identified under their routines. Effects on omitted paths remain unknown.
In the unreached-range sentences, any path means a path represented in the reported analysis graph.
Unknown computed targets remain unresolved. Range ends are exclusive.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-controller-command-init-facts.txt) retain all tool sections.

The notation `ret(ADDR)` identifies a call-result expression without distinguishing
individual call sites to that target. Each return line records machine-register
state from the analysis rather than a C return type.

Groups summarize static records by printed access shape. Address order need not describe one execution path.

## `SMPInit`

- `SMPInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x927d8 to 0x92d38.
  Established facts:
  The instructions from 0x92ca0 to 0x92d38 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x92814): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x169eb0`, r2=`0x62`, r3=`0x0`.
  Word store: target `abs 0x31aaf8`, value `0x3` (`str` at 0x92840).
  Byte store: target `[0x31ab4c] +0x0`, value `0x0` (`strb` at 0x92864).
  Call (`bl` at 0x9286c): (unknown callee), target 0x6dc0c, r0=`unknown`, r1=`unknown`, r2=`[0x31ab4c]`, r3=`0x0`.
  Call (`bl` at 0x928a0): (unknown callee), target 0xfb6ec, r0=`0x31a854`, r1=`0x169ec4`, r2=`0x19`, r3=`0x0`.
  Call (`bl` at 0x928bc): (unknown callee), target 0x219a4, r0=`ret(0xfb6ec)`, r1=`unknown`, r2=`unknown`, r3=`ret(0xfb6ec)`.
  Call (`bl` at 0x928e0): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x169eb0`, r2=`0x80`, r3=`0x0`.
  Call (`bl` at 0x928f0): (unknown callee), target 0xfb6ec, r0=`0x31a8f8`, r1=`0x169ef0`, r2=`0x19`, r3=`unknown`.
  Call (`bl` at 0x9290c): (unknown callee), target 0x219a4, r0=`ret(0xfb6ec)`, r1=`unknown`, r2=`unknown`, r3=`ret(0xfb6ec)`.
  Call (`bl` at 0x92930): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x169eb0`, r2=`0x83`, r3=`0x0`.
  Call (`bl` at 0x92940): (unknown callee), target 0xfb6ec, r0=`0x31a99c`, r1=`0x169f20`, r2=`0x19`, r3=`unknown`.
  Call (`bl` at 0x9295c): (unknown callee), target 0x219a4, r0=`ret(0xfb6ec)`, r1=`unknown`, r2=`unknown`, r3=`ret(0xfb6ec)`.
  Call (`bl` at 0x92980): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x169eb0`, r2=`0x86`, r3=`0x0`.
  Call (`bl` at 0x92994): (unknown callee), target 0x22b84, r0=`0x31aa40`, r1=`0x169f50`, r2=`0x1`, r3=`0x9`.
  Call (`bl` at 0x929a8): (unknown callee), target 0x22b84, r0=`0x31aa94`, r1=`0x169f5c`, r2=`0x1`, r3=`0x9`.
  Call (`bl` at 0x929f4): (unknown callee), target 0x100280, r0=`0x31b1c4`, r1=`0x31b200`, r2=`0x100`, r3=`0x31b300`.
  Call (`bl` at 0x92a04): (unknown callee), target 0xfb5cc, r0=`0x31a844`, r1=`0x2`, r2=`0x0`, r3=`unknown`.
  Call (`bl` at 0x92a0c): (unknown callee), target 0xa3c7c, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x92b2c): (unknown callee), target 0x9533c, r0=`ret(0xa3c7c)`, r1=`0x31a844`, r2=`0x31a844`, r3=`unknown`.
  Call (`bl` at 0x92b74): (unknown callee), target 0x22540, r0=`0x31aae8`, r1=`0x169f68`, r2=`0x92e18`, r3=`0x0`.
  Call (`bl` at 0x92bb0): (unknown callee), target 0x22540, r0=`0x31aaec`, r1=`0x169f74`, r2=`0x93320`, r3=`0x0`.
  Call (`bl` at 0x92bec): (unknown callee), target 0x22540, r0=`0x31aaf0`, r1=`0x169f80`, r2=`0x940c8`, r3=`0x0`.
  Call (`bl` at 0x92c00): (unknown callee), target 0xfa264, r0=`0x31a844`, r1=`0xffff002`, r2=`0x1`, r3=`0x31a99c`.
  Call (`bl` at 0x92c1c): (unknown callee), target 0x219a4, r0=`ret(0xfa264)`, r1=`unknown`, r2=`unknown`, r3=`ret(0xfa264)`.
  Call (`bl` at 0x92c40): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x169eb0`, r2=`0xd1`, r3=`0x0`.
  Call (`bl` at 0x92c54): (unknown callee), target 0xfa264, r0=`0x31a844`, r1=`0xffff002`, r2=`0x2`, r3=`0x31a854`.
  Call (`bl` at 0x92c70): (unknown callee), target 0x219a4, r0=`ret(0xfa264)`, r1=`unknown`, r2=`unknown`, r3=`ret(0xfa264)`.
  Call (`bl` at 0x92c94): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x169eb0`, r2=`0xdd`, r3=`0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x92c9c).

## `SMPLock`

- `SMPLock`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x92d38 to 0x92dcc.
  Established facts:
  The instructions from 0x92dc0 to 0x92dcc are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x92d50): (unknown callee), target 0x24368, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Branch (`bne` at 0x92d80): taken to 0x92d90 when `[0x31a850] != 0x0`, falls through to 0x92d84 when `[0x31a850] == 0x0`.
  Word store: target `abs 0x1a6488`, value `ret(0x24368)` (`str` at 0x92d98, `bl` at 0x92d50).
  Call (`bl` at 0x92da8): (unknown callee), target 0x10f724, r0=`[0x1a6488]`, r1=`0x1a64bc`, r2=`0x1a6488`, r3=`0x1a6488`.
  Recorded r0 = `unknown` (`ldm` at 0x92dbc).

## `SMPUnlock`

- `SMPUnlock`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x92dcc to 0x92e18.
  Established facts:
  The instructions from 0x92e10 to 0x92e18 are not reached by any path from the entry, and their branches are not listed.
  Branch (`bne` at 0x92de4): taken to 0x92dec when `[0x1a6488] != 0x0`, falls through to 0x92de8 when `[0x1a6488] == 0x0`.
  Call (`bl` at 0x92df8): (unknown callee), target 0x10f764, r0=`[0x1a6488]`, r1=`0x1a64bc`, r2=`r2`, r3=`0x1a6488`.
  Word store: target `abs 0x1a6488`, value `0x0` (`str` at 0x92e04).
  Call (`bl` at 0x92e08): (unknown callee), target 0x243b8, r0=`ret(0x10f764)`, r1=`unknown`, r2=`0x1a6488`, r3=`0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x92e0c).

## `SMPLibrary`

- `SMPLibrary`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x937d8 to 0x940c8.
  Established facts:
  The instructions from 0x9405c to 0x940c8 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x93808): (unknown callee), target 0x22ca0, r0=`0x31aa94`, r1=`unknown`, r2=`r2`, r3=`0x0`.
  Call (`bl` at 0x93824): (unknown callee), target 0x219a4, r0=`ret(0x22ca0)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x22ca0)`.
  Call (`bl` at 0x93848): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x169eb0`, r2=`0x28d`, r3=`0x0`.
  Grouped accesses: 2 reported word store records with absolute target `0x31b408`, including target `abs 0x31b408`, value `0x51` (`str` at 0x93868), target `abs 0x31b408`, value `0x0` (`str` at 0x938c0).
  Call (`bl` at 0x93870): (unknown callee), target 0x201fc, r0=`0x144`, r1=`unknown`, r2=`0x31a844`, r3=`0x51`.
  Call (`bl` at 0x938f0): (unknown callee), target 0x94f04, r0=`0x31a844`, r1=`0x16a134`, r2=`0x1`, r3=`[0x31aaf4]`.
  Call (`bl` at 0x93904): (unknown callee), target 0x2302c, r0=`0x31aa94`, r1=`unknown`, r2=`unknown`, r3=`ret(0x94f04)`.
  Call (`bl` at 0x93920): (unknown callee), target 0x219a4, r0=`ret(0x2302c)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x2302c)`.
  Call (`bl` at 0x93944): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x169eb0`, r2=`0x2a5`, r3=`0x0`.
  Call (`bl` at 0x93960): (unknown callee), target 0x10f724, r0=`[0x31a84c]`, r1=`0x1a6528`, r2=`unknown`, r3=`0x31a844`.
  Byte store: target `abs 0x1a653d`, value `0x1` (`strb` at 0x9396c).
  Call (`bl` at 0x9397c): (unknown callee), target 0x10f764, r0=`[0x31a84c]`, r1=`0x1a6528`, r2=`0x1a6528`, r3=`0x31a844`.
  Word store: target `abs 0x31aaf4`, value `0x1` (`str` at 0x93988).
  Call (`bl` at 0x93990): (unknown callee), target 0x2302c, r0=`0x31aa94`, r1=`unknown`, r2=`0x31a844`, r3=`0x1`.
  Call (`bl` at 0x939ac): (unknown callee), target 0x219a4, r0=`ret(0x2302c)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x2302c)`.
  Call (`bl` at 0x939d0): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x169eb0`, r2=`0x2ba`, r3=`0x0`.
  Call (`bl` at 0x93a34): (unknown callee), target 0x95138, r0=`0x16a15c`, r1=`unknown`, r2=`0x31a844`, r3=`0x2`.
  Call (`bl` at 0x93a3c): (unknown callee), target 0x95138, r0=`0x31ab50`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x93a54): (unknown callee), target 0x94ae4, r0=`[sp-28]`, r1=`unknown`, r2=`[0x31aaf8]`, r3=`[[sp-28]]:1`.
  Call (`bl` at 0x93bd8): (unknown callee), target 0x95138, r0=`0x16a180`, r1=`unknown`, r2=`[0x31aaf8]`, r3=`[0x1a64b8]`.
  Call (`bl` at 0x93be0): (unknown callee), target 0x5572c, r0=`unknown`, r1=`unknown`, r2=`[0x31aaf8]`, r3=`[0x1a64b8]`.
  Call (`bl` at 0x93bf4): (unknown callee), target 0x95138, r0=`0x16a1a4`, r1=`unknown`, r2=`unknown`, r3=`ret(0x5572c)`.
  Call (`bl` at 0x93c10): (unknown callee), target 0x95f98, r0=`[0x31b1b8]`, r1=`unknown`, r2=`unknown`, r3=`0x31a844`.
  Call (`bl` at 0x93c24): (unknown callee), target 0x205bc, r0=`sp-120`, r1=`0x16a1d8`, r2=`ret(0x95f98)`, r3=`sp-120`.
  Call (`bl` at 0x93c30): (unknown callee), target 0x95138, r0=`sp-120`, r1=`unknown`, r2=`unknown`, r3=`sp-120`.
  Call (`bl` at 0x93c6c): (unknown callee), target 0x2031c, r0=`0x31aafc`, r1=`unknown`, r2=`[0x31aaf8]`, r3=`[[sp-28]]:1`.
  Call (`bl` at 0x93ca4): (unknown callee), target 0x20634, r0=`unknown`, r1=`0x31aafc`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x93d34): (unknown callee), target 0x95138, r0=`0x16a210`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x93d38): (unknown callee), target 0x94c9c, r0=`ret(0x95138)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x93d70): (unknown callee), target 0x5572c, r0=`ret(0x94c9c)`, r1=`unknown`, r2=`[0x31ab4c]`, r3=`[0x31aaf8]`.
  Call (`bl` at 0x93d84): (unknown callee), target 0x95138, r0=`0x16a1a4`, r1=`unknown`, r2=`unknown`, r3=`ret(0x5572c)`.
  Call (`bl` at 0x93d90): (unknown callee), target 0x95f98, r0=`[0x31b1b8]`, r1=`unknown`, r2=`unknown`, r3=`0x31a844`.
  Call (`bl` at 0x93da4): (unknown callee), target 0x205bc, r0=`sp-120`, r1=`0x16a214`, r2=`ret(0x95f98)`, r3=`sp-120`.
  Call (`bl` at 0x93dbc): (unknown callee), target 0x95138, r0=`sp-120`, r1=`unknown`, r2=`0x31a844`, r3=`sp-120`.
  Call (`bl` at 0x93dc8): (unknown callee), target 0x95138, r0=`0x31ab50`, r1=`unknown`, r2=`[0x31ab4c]`, r3=`[0x31aaf8]`.
  Call (`bl` at 0x93e18): (unknown callee), target 0x95138, r0=`0x16a250`, r1=`0x31a844`, r2=`[0x31ab4c]-1`, r3=`0x0`.
  Call (`bl` at 0x93ebc): (unknown callee), target 0x95138, r0=`0x16a250`, r1=`0x31a844`, r2=`[0x31ab4c]-1`, r3=`0x31aafb`.
  Call (`bl` at 0x93f48): (unknown callee), target 0x20634, r0=`0x31aafc`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x93f54): (unknown callee), target 0x2031c, r0=`0x31aafc`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x93f70): (unknown callee), target 0x95138, r0=`0x31aafc`, r1=`unknown`, r2=`ret(0x2031c)`, r3=`ret(0x2031c)+0x31aafc`.
  Call (`bl` at 0x93ff4): (unknown callee), target 0x95138, r0=`0x16a254`, r1=`unknown`, r2=`[0x31ab4c]`, r3=`0x0`.
  Call (`bl` at 0x93ffc): (unknown callee), target 0x95138, r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x94004): (unknown callee), target 0x2302c, r0=`0x31aa94`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x94020): (unknown callee), target 0x219a4, r0=`ret(0x2302c)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x2302c)`.
  Call (`bl` at 0x94044): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x169eb0`, r2=`0x3b0`, r3=`0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x94058).

## `SMPCmdInit`

- `SMPCmdInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd20b0 to 0xd234c.
  Established facts:
  The instructions from 0xd22e4 to 0xd2334 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0xd2338 to 0xd234c are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0xd20e8): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x17943c`, r2=`0x1a41`, r3=`0x0`.
  Call (`bl` at 0xd20f8): (unknown callee), target 0xfb5cc, r0=`0x31b684`, r1=`0x3`, r2=`0x0`, r3=`unknown`.
  Call (`bl` at 0xd2108): (unknown callee), target 0xfb6ec, r0=`0x31b68c`, r1=`0x179454`, r2=`0xa`, r3=`unknown`.
  Call (`bl` at 0xd2124): (unknown callee), target 0x219a4, r0=`ret(0xfb6ec)`, r1=`unknown`, r2=`unknown`, r3=`ret(0xfb6ec)`.
  Call (`bl` at 0xd2148): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x17943c`, r2=`0x1a4e`, r3=`0x0`.
  Call (`bl` at 0xd215c): (unknown callee), target 0xfa264, r0=`0x31b684`, r1=`0xffff003`, r2=`0x1`, r3=`0x31b68c`.
  Call (`bl` at 0xd2178): (unknown callee), target 0x219a4, r0=`ret(0xfa264)`, r1=`unknown`, r2=`unknown`, r3=`ret(0xfa264)`.
  Call (`bl` at 0xd219c): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x17943c`, r2=`0x1a56`, r3=`0x0`.
  Call (`bl` at 0xd21ac): (unknown callee), target 0xfb6ec, r0=`0x31b730`, r1=`0x1794a8`, r2=`0xa`, r3=`unknown`.
  Call (`bl` at 0xd21c8): (unknown callee), target 0x219a4, r0=`ret(0xfb6ec)`, r1=`unknown`, r2=`unknown`, r3=`ret(0xfb6ec)`.
  Call (`bl` at 0xd21ec): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x17943c`, r2=`0x1a5d`, r3=`0x0`.
  Call (`bl` at 0xd2200): (unknown callee), target 0xfa264, r0=`0x31b684`, r1=`0xffff003`, r2=`0x2`, r3=`0x31b730`.
  Call (`bl` at 0xd221c): (unknown callee), target 0x219a4, r0=`ret(0xfa264)`, r1=`unknown`, r2=`unknown`, r3=`ret(0xfa264)`.
  Call (`bl` at 0xd2240): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x17943c`, r2=`0x1a65`, r3=`0x0`.
  Call (`bl` at 0xd2254): (unknown callee), target 0x22b84, r0=`0x31b7d8`, r1=`0x179500`, r2=`0x1`, r3=`0x9`.
  Call (`bl` at 0xd2258): (unknown callee), target 0xd234c, r0=`ret(0x22b84)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`bls` at 0xd2268): taken to 0xd2278 when `ret(0xd234c) <= 0x7ffff (unsigned, 32-bit)`, falls through to 0xd226c when `ret(0xd234c) > 0x7ffff (unsigned, 32-bit)` (source `bl` at 0xd2258).
  Call (`bl` at 0xd22b8): (unknown callee), target 0x22540, r0=`0x31b7d4`, r1=`0x17950c`, r2=`0xd3678`, r3=`0x0`.
  Call (`bl` at 0xd22c4): (unknown callee), target 0x10184c, r0=`0x171278`, r1=`0x13b`, r2=`unknown`, r3=`unknown`.
  Word store: target `abs 0x1b0cf8`, value `[0x1b0cf8]+1` (`str` at 0xd22d8).
  Recorded r0 = `ret(0x10184c)` (`ldm` at 0xd22e0, `bl` at 0xd22c4).

## `SMPCmd_2m_Init`

- `SMPCmd_2m_Init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe05b4 to 0xe05d4.
  Established facts:
  The instructions from 0xe05d0 to 0xe05d4 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0xe05c8): (unknown callee), target 0x10184c, r0=`0x17c298`, r1=`0xf2`, r2=`r2`, r3=`r3`.
  Recorded r0 = `ret(0x10184c)` (`ldm` at 0xe05cc, `bl` at 0xe05c8).

## `bladecmdInit`

- `bladecmdInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe3638 to 0xe3680.
  Established facts:
  The instructions from 0xe3678 to 0xe3680 are not reached by any path from the entry, and their branches are not listed.
  Branch (`bne` at 0xe3650): taken to 0xe366c when `[0x1b0e20] != 0x0`, falls through to 0xe3654 when `[0x1b0e20] == 0x0`.
  Call (`bl` at 0xe365c): (unknown callee), target 0x10184c, r0=`0x183e24`, r1=`0x3`, r2=`r2`, r3=`[0x1b0e20]`.
  Word store: target `abs 0x1b0e20`, value `0x1` (`str` at 0xe3668).
  Recorded r0 = `0x0` (`ldm` at 0xe3674).

## `SMPCmdLibrary`

- `SMPCmdLibrary`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd2364 to 0xd3678.
  Established facts:
  The instructions from 0xd32ec to 0xd32f4 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0xd32f8 to 0xd32fc are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0xd3300 to 0xd3304 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0xd3308 to 0xd3334 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0xd3338 to 0xd333c are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0xd3340 to 0xd3350 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0xd23a8): (unknown callee), target 0xa3c7c, r0=`0x0`, r1=`r1`, r2=`r2`, r3=`0x0`.
  Call (`bl` at 0xd23f4): (unknown callee), target 0x201fc, r0=`0x8`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Word store: target `[sp-2680] +0x4`, value `0x0` (`str` at 0xd2424).
  Call (`bl` at 0xd243c): (unknown callee), target 0x20550, r0=`sp-2664`, r1=`0x0`, r2=`0x618`, r3=`sp-2664`.
  Call (`bl` at 0xd246c): (unknown callee), target 0x1018f0, r0=`0x1`, r1=`[sp-24]`, r2=`[sp-2680]`, r3=`0x0`.
  Call (`bl` at 0xd24a4): (unknown callee), target 0x22ca0, r0=`0x31b7d8`, r1=`0x0`, r2=`unknown`, r3=`[sp-2656]`.
  Call (`bl` at 0xd24c8): (unknown callee), target 0x20634, r0=`[[sp-2680]]`, r1=`0x179518`, r2=`unknown`, r3=`[sp-2680]`.
  Call (`bl` at 0xd24dc): (unknown callee), target 0x20514, r0=`[sp-2680]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x22ca0)`.
  Call (`bl` at 0xd24f4): (unknown callee), target 0x201fc, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Halfword load: target `abs 0x31b68a` (`ldrh` at 0xd259c).
  Call (`bl` at 0xd25d8): (unknown callee), target 0x958d4, r0=`sp-76`, r1=`sp-72`, r2=`0x0`, r3=`sp-76`.
  Call (`bl` at 0xd25e0): (unknown callee), target 0x2031c, r0=`[sp-2660]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd2608): (unknown callee), target 0x95b5c, r0=`sp-76`, r1=`sp-72`, r2=`[sp-2660]`, r3=`ret(0x2031c)`.
  Call (`bl` at 0xd2644): (unknown callee), target 0xfd4a8, r0=`0x31b684`, r1=`sp-2704`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0xd2664): (unknown callee), target 0xa92d8, r0=`[sp-2708]`, r1=`unknown`, r2=`unknown`, r3=`ret(0xfd4a8)`.
  Call (`bl` at 0xd26a4): (unknown callee), target 0x23f34, r0=`unknown`, r1=`sp-48`, r2=`0x64`, r3=`unknown`.
  Call (`bl` at 0xd26b8): (unknown callee), target 0xfb794, r0=`[sp-48]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x23f34)`.
  Call (`bl` at 0xd26d0): (unknown callee), target 0x201fc, r0=`0x50`, r1=`unknown`, r2=`unknown`, r3=`[sp-2680]`.
  Grouped accesses: 4 reported word load records with absolute target `0x31b684`, including target `abs 0x31b684` (`ldr` at 0xd26e0), target `abs 0x31b684` (`ldr` at 0xd2818), target `abs 0x31b684` (`ldr` at 0xd2b80).
  Call (`bl` at 0xd26e4): (unknown callee), target 0x95dd0, r0=`[0x31b684]`, r1=`unknown`, r2=`unknown`, r3=`0x31b684`.
  Call (`bl` at 0xd2700): (unknown callee), target 0x20634, r0=`unknown`, r1=`ret(0x95dd0)`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd271c): (unknown callee), target 0x203c4, r0=`unknown`, r1=`0x179540`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd2744): (unknown callee), target 0x2031c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd2758): (unknown callee), target 0x6dc0c, r0=`ret(0x2031c)`, r1=`unknown`, r2=`ret(0x2031c)`, r3=`unknown`.
  Call (`bl` at 0xd276c): (unknown callee), target 0x96990, r0=`unknown`, r1=`[sp-2708]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd277c): (unknown callee), target 0x20634, r0=`unknown`, r1=`ret(0x96990)`, r2=`unknown`, r3=`ret(0x96990)`.
  Call (`bl` at 0xd2798): (unknown callee), target 0x203c4, r0=`unknown`, r1=`0x176d28`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd285c): (unknown callee), target 0x1018f0, r0=`0x1`, r1=`[sp-2660]`, r2=`sp-2708`, r3=`0x0`.
  Call (`bl` at 0xd28a0): (unknown callee), target 0x201fc, r0=`0x50`, r1=`unknown`, r2=`unknown`, r3=`[sp-2680]`.
  Call (`bl` at 0xd28d4): (unknown callee), target 0x95dd0, r0=`unknown`, r1=`sp-20`, r2=`0xfffff5bc`, r3=`unknown`.
  Call (`bl` at 0xd28f0): (unknown callee), target 0x20634, r0=`unknown`, r1=`ret(0x95dd0)`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd290c): (unknown callee), target 0x203c4, r0=`unknown`, r1=`0x179568`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd2928): (unknown callee), target 0x203c4, r0=`unknown`, r1=`[sp-2708]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd296c): (unknown callee), target 0x95dd0, r0=`unknown`, r1=`sp-20`, r2=`0xfffff5bc`, r3=`unknown`.
  Call (`bl` at 0xd2988): (unknown callee), target 0x20634, r0=`unknown`, r1=`ret(0x95dd0)`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd29b8): (unknown callee), target 0x203c4, r0=`unknown`, r1=`0x17956c`, r2=`[sp-2684]+1`, r3=`unknown`.
  Call (`bl` at 0xd29c0): (unknown callee), target 0x20058, r0=`[sp-2708]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd2a04): (unknown callee), target 0x20514, r0=`[sp-2708]`, r1=`unknown`, r2=`unknown`, r3=`0x0`.
  Call (`bl` at 0xd2a6c): (unknown callee), target 0x958d4, r0=`sp-76`, r1=`sp-72`, r2=`0x0`, r3=`sp-76`.
  Call (`bl` at 0xd2a74): (unknown callee), target 0x2031c, r0=`[sp-2660]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd2a9c): (unknown callee), target 0x95b5c, r0=`sp-76`, r1=`sp-72`, r2=`[sp-2660]`, r3=`ret(0x2031c)`.
  Call (`bl` at 0xd2b04): (unknown callee), target 0xfcf98, r0=`0x31b684`, r1=`unknown`, r2=`sp-2700`, r3=`0x0`.
  Call (`bl` at 0xd2b70): (unknown callee), target 0x201fc, r0=`0x50`, r1=`unknown`, r2=`unknown`, r3=`[sp-2680]`.
  Call (`bl` at 0xd2b84): (unknown callee), target 0x95dd0, r0=`[0x31b684]`, r1=`unknown`, r2=`unknown`, r3=`0x31b684`.
  Call (`bl` at 0xd2ba0): (unknown callee), target 0x20634, r0=`unknown`, r1=`ret(0x95dd0)`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd2bbc): (unknown callee), target 0x203c4, r0=`unknown`, r1=`0x179570`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd2be4): (unknown callee), target 0x2031c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd2c14): (unknown callee), target 0x95dd0, r0=`unknown`, r1=`sp-20`, r2=`0xfffff5bc`, r3=`unknown`.
  Call (`bl` at 0xd2c24): (unknown callee), target 0x20634, r0=`unknown`, r1=`ret(0x95dd0)`, r2=`unknown`, r3=`ret(0x95dd0)`.
  Call (`bl` at 0xd2c40): (unknown callee), target 0x203c4, r0=`unknown`, r1=`0x176d28`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd2ca0): (unknown callee), target 0x219a4, r0=`[sp-36]`, r1=`sp-20`, r2=`unknown`, r3=`[sp-2676]+1`.
  Call (`bl` at 0xd2cb0): (unknown callee), target 0x29908, r0=`0x179598`, r1=`ret(0x219a4)`, r2=`unknown`, r3=`ret(0x219a4)`.
  Call (`bl` at 0xd2cc8): (unknown callee), target 0x20514, r0=`[sp-2660]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd2cfc): (unknown callee), target 0xfb734, r0=`sp-48`, r1=`0x31b730`, r2=`0x44c`, r3=`sp-48`.
  Call (`bl` at 0xd2d98): (unknown callee), target 0x201fc, r0=`0x50`, r1=`sp-20`, r2=`unknown`, r3=`[sp-2680]`.
  Call (`bl` at 0xd2dac): (unknown callee), target 0x95dd0, r0=`[0x31b684]`, r1=`unknown`, r2=`unknown`, r3=`0x31b684`.
  Call (`bl` at 0xd2dc8): (unknown callee), target 0x20634, r0=`unknown`, r1=`ret(0x95dd0)`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd2de4): (unknown callee), target 0x203c4, r0=`unknown`, r1=`0x1795bc`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd2e0c): (unknown callee), target 0x2031c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd2e3c): (unknown callee), target 0x95dd0, r0=`unknown`, r1=`sp-20`, r2=`0xfffff5bc`, r3=`unknown`.
  Call (`bl` at 0xd2e4c): (unknown callee), target 0x20634, r0=`unknown`, r1=`ret(0x95dd0)`, r2=`unknown`, r3=`ret(0x95dd0)`.
  Call (`bl` at 0xd2e68): (unknown callee), target 0x203c4, r0=`unknown`, r1=`0x176d28`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd2ec0): (unknown callee), target 0xfde7c, r0=`[sp-48]`, r1=`0x0`, r2=`ret(0xfb734)`, r3=`[[sp-48]+19]:1`.
  Call (`bl` at 0xd2ee4): (unknown callee), target 0xfb794, r0=`[sp-48]`, r1=`unknown`, r2=`ret(0xfde7c)`, r3=`unknown`.
  Call (`bl` at 0xd300c): (unknown callee), target 0xfb794, r0=`[sp-48]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd3020): (unknown callee), target 0x219a4, r0=`[sp-36]`, r1=`ret(0xfb734)`, r2=`ret(0xfb734)`, r3=`ret(0xfb734)`.
  Call (`bl` at 0xd3034): (unknown callee), target 0x29908, r0=`0x1795d8`, r1=`ret(0x219a4)`, r2=`[sp-36]`, r3=`ret(0x219a4)`.
  Call (`bl` at 0xd30bc): (unknown callee), target 0xfde7c, r0=`[sp-48]`, r1=`0x0`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd3104): (unknown callee), target 0x201fc, r0=`0x50`, r1=`unknown`, r2=`unknown`, r3=`[sp-2680]`.
  Call (`bl` at 0xd3124): (unknown callee), target 0x95dd0, r0=`[[sp-48]+28]`, r1=`unknown`, r2=`unknown`, r3=`[sp-48]`.
  Call (`bl` at 0xd3140): (unknown callee), target 0x20634, r0=`unknown`, r1=`ret(0x95dd0)`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd315c): (unknown callee), target 0x203c4, r0=`unknown`, r1=`0x179568`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd318c): (unknown callee), target 0xfde7c, r0=`[sp-48]`, r1=`0x1`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd31ac): (unknown callee), target 0x203c4, r0=`unknown`, r1=`[sp-2728]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd31ec): (unknown callee), target 0x95dd0, r0=`[[sp-48]+28]`, r1=`unknown`, r2=`unknown`, r3=`[sp-48]`.
  Call (`bl` at 0xd3208): (unknown callee), target 0x20634, r0=`unknown`, r1=`ret(0x95dd0)`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd3238): (unknown callee), target 0x203c4, r0=`unknown`, r1=`0x17956c`, r2=`[sp-2684]+1`, r3=`unknown`.
  Call (`bl` at 0xd3268): (unknown callee), target 0xfde7c, r0=`[sp-48]`, r1=`0x1`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd3274): (unknown callee), target 0x20058, r0=`[sp-2732]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd3358): (unknown callee), target 0xfde7c, r0=`[sp-48]`, r1=`0x1`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd3364): (unknown callee), target 0x20058, r0=`[sp-2736]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd33c4): (unknown callee), target 0xfde7c, r0=`[sp-48]`, r1=`0x2`, r2=`[sp-2684]+1`, r3=`unknown`.
  Call (`bl` at 0xd33d0): (unknown callee), target 0x20058, r0=`[sp-2740]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd3434): (unknown callee), target 0x201fc, r0=`0x50`, r1=`unknown`, r2=`unknown`, r3=`[sp-2680]`.
  Call (`bl` at 0xd3448): (unknown callee), target 0x95dd0, r0=`[[sp-48]+28]`, r1=`unknown`, r2=`unknown`, r3=`[sp-48]`.
  Call (`bl` at 0xd3464): (unknown callee), target 0x20634, r0=`unknown`, r1=`ret(0x95dd0)`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd3480): (unknown callee), target 0x203c4, r0=`unknown`, r1=`0x179600`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd34a8): (unknown callee), target 0x2031c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd34e0): (unknown callee), target 0x95dd0, r0=`unknown`, r1=`sp-20`, r2=`0xfffff5bc`, r3=`unknown`.
  Call (`bl` at 0xd34f0): (unknown callee), target 0x20634, r0=`unknown`, r1=`ret(0x95dd0)`, r2=`unknown`, r3=`ret(0x95dd0)`.
  Call (`bl` at 0xd350c): (unknown callee), target 0x203c4, r0=`unknown`, r1=`0x176d28`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd3540): (unknown callee), target 0xfb794, r0=`[sp-48]`, r1=`unknown`, r2=`unknown`, r3=`[sp-1624]`.
  Call (`mov` at 0xd3590): (unknown callee), target (unknown target), r0=`sp-2664`, r1=`unknown`, r2=`sp-2664`, r3=`unknown`.
  Call (`bl` at 0xd35e8): (unknown callee), target 0xfb794, r0=`unknown`, r1=`sp-20`, r2=`0xfffff9c4`, r3=`unknown`.
  Call (`bl` at 0xd3600): (unknown callee), target 0x2302c, r0=`0x31b7d8`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd361c): (unknown callee), target 0x219a4, r0=`ret(0x2302c)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x2302c)`.
  Call (`bl` at 0xd3640): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x17943c`, r2=`0x1c33`, r3=`0x0`.
  Recorded r0 = `unknown` (`ldm` at 0xd3674).

## `SMPCmdSource`

- `SMPCmdSource`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc55dc to 0xc5730.
  Established facts:
  The instructions from 0xc56a0 to 0xc56f4 are not reached by any path from the entry, and their branches are not listed.
  Word load: target `r0 +0x1c` (`ldr` at 0xc55f4).
  Branch (`bge` at 0xc5604): taken to 0xc5630 when `[r0+28] >= 0x0 (signed, 32-bit)`, falls through to 0xc5608 when `[r0+28] < 0x0 (signed, 32-bit)`.
  Branch (`ldrls` at 0xc5698): taken to (unknown target) when `[sp-40]-1 <= 0xe (unsigned, 32-bit)`, falls through to 0xc569c when `[sp-40]-1 > 0xe (unsigned, 32-bit)`.
  Omitted-path effects remain unknown at the unresolved transfer (`ldrls` at 0xc5698).
  Known-edge recorded r0 = `unknown` (`ldm` at 0xc572c).

## `SMPCmdUnsupported`

- `SMPCmdUnsupported`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc4cb8 to 0xc4d10.
  Established facts:
  The instructions from 0xc4d0c to 0xc4d10 are not reached by any path from the entry, and their branches are not listed.
  Word load: target `r0 +0x0` (`ldr` at 0xc4cd0).
  Branch (`bne` at 0xc4cd8): taken to 0xc4cf0 when `[r0] != 0x0`, falls through to 0xc4cdc when `[r0] == 0x0`.
  Call (`bl` at 0xc4ce4): (unknown callee), target 0x201fc, r0=`0x50`, r1=`r1`, r2=`r2`, r3=`[r0]`.
  Word store: target `r0 +0x0`, value `ret(0x201fc)` (`str` at 0xc4cec, `bl` at 0xc4ce4).
  Call (`bl` at 0xc4cfc): (unknown callee), target 0x20634, r0=`unknown`, r1=`0x176624`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0xc4d08).

## `_parse_cpu`

- `_parse_cpu`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc683c to 0xc6adc.
  Established facts:
  The instructions from 0xc6ac8 to 0xc6adc are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0xc6860): (unknown callee), target 0x6dc0c, r0=`r0`, r1=`r1`, r2=`r2`, r3=`0x0`.
  Call (`bl` at 0xc6874): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xc6888): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xc689c): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xc68b0): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xc68c4): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xc68d8): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xc6910): (unknown callee), target 0x6e140, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xc6920): (unknown callee), target 0x6e140, r0=`ret(0x6e140)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x6e140)`.
  Word load: target `abs 0x31841c` (`ldr` at 0xc6934).
  Call (`bl` at 0xc6940): (unknown callee), target 0x2031c, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xc69d0): (unknown callee), target 0x201b4, r0=`[[sp-24]]:1`, r1=`unknown`, r2=`unknown`, r3=`[[sp-24]]:1`.
  Call (`bl` at 0xc69f8): (unknown callee), target 0x6e140, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xc6a08): (unknown callee), target 0x6e140, r0=`ret(0x6e140)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x6e140)`.
  Call (`bl` at 0xc6a1c): (unknown callee), target 0x150058, r0=`[sp-36]`, r1=`ret(0x6e140)`, r2=`ret(0x6e140)`, r3=`[sp-36]`.
  Word load: target `abs 0x31a520` (`ldr` at 0xc6a2c).
  Word load: target `abs 0x3183f4` (`ldr` at 0xc6a48).
  Call (`bl` at 0xc6a84): (unknown callee), target 0x243d4, r0=`0x176ca4`, r1=`[sp-36]`, r2=`[sp-40]`, r3=`unknown`.
  Call (`bl` at 0xc6a88): (unknown callee), target 0x6e140, r0=`ret(0x243d4)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xc6aa0): (unknown callee), target 0x205bc, r0=`[sp-28]`, r1=`0x176cb0`, r2=`unknown`, r3=`ret(0x6e140)`.
  Call (`bl` at 0xc6ab0): (unknown callee), target 0x205bc, r0=`[sp-28]`, r1=`0x176cf0`, r2=`unknown`, r3=`ret(0x6e140)`.
  Recorded r0 = `[sp-52]` (`ldm` at 0xc6ac4).

## `UtilDspBrick`

- `UtilDspBrick`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9662c to 0x96878.
  Established facts:
  The instructions from 0x96688 to 0x96854 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x96870 to 0x96878 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x96640): (unknown callee), target 0xa3c7c, r0=`0x0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Grouped accesses: 3 reported word load records with printed base expression `ret(0xa3c7c)` and nonnegative word-aligned offsets, including target `ret(0xa3c7c) +0x8` (`ldr` at 0x96660), target `ret(0xa3c7c) +0x10` (`ldr` at 0x96664), target `ret(0xa3c7c) +0xc` (`ldr` at 0x96668) (source `bl` at 0x96640).
  Call (`bl` at 0x9666c): (unknown callee), target 0x96878, r0=`sp-36`, r1=`[ret(0xa3c7c)+8]`, r2=`[ret(0xa3c7c)+16]`, r3=`[ret(0xa3c7c)+12]`.
  Branch (`ldrls` at 0x96680): taken to (unknown target) when `[[sp-40]+16]-35 <= 0x5a (unsigned, 32-bit)`, falls through to 0x96684 when `[[sp-40]+16]-35 > 0x5a (unsigned, 32-bit)`.
  Omitted-path effects remain unknown at the unresolved transfer (`ldrls` at 0x96680).
  Call (`bl` at 0x96864): (unknown callee), target 0x706a0, r0=`0x31`, r1=`sp-36`, r2=`0x0`, r3=`sp-36`.
  Known-edge recorded r0 = `ret(0x706a0)` (`ldm` at 0x9686c, `bl` at 0x96864).

## `UtilFmtBrick`

- `UtilFmtBrick`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x96878 to 0x96910.
  Established facts:
  The instructions from 0x9690c to 0x96910 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x9689c): (unknown callee), target 0x107e4c, r0=`r2`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0x968c0): (unknown callee), target 0x205bc, r0=`[sp-20]`, r1=`0x16a4a4`, r2=`[sp-24]`, r3=`unknown`.
  Branch (`bne` at 0x968cc): taken to 0x968e0 when `[sp-24] != 0x0`, falls through to 0x968d0 when `[sp-24] == 0x0`.
  Call (`bl` at 0x968dc): (unknown callee), target 0x20550, r0=`[sp-20]`, r1=`0x2d`, r2=`0x3`, r3=`[sp-24]`.
  Call (`bl` at 0x96900): (unknown callee), target 0x20550, r0=`[sp-20]+4`, r1=`0x2d`, r2=`0x2`, r3=`[sp-20]+4`.
  Recorded r0 = `unknown` (`ldm` at 0x96908).

## `brickIsCompute`

- `brickIsCompute`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1081c0 to 0x108300.
  Established facts:
  Branch (`beq` at 0x1081e4): taken to 0x1082e8 when `r0 == 0x43`, falls through to 0x1081e8 when `r0 != 0x43`.
  Branch (`beq` at 0x1081f0): taken to 0x1082e8 when `r0 == 0x33`, falls through to 0x1081f4 when `r0 != 0x33`.
  Branch (`beq` at 0x1082e0): taken to 0x1082e8 when `r0 == 0x7b`, falls through to 0x1082e4 when `r0 != 0x7b`.
  Recorded r0 = `unknown` (`ldm` at 0x1082fc).

## `brickTypeToChar`

- `brickTypeToChar`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x107e4c to 0x1081c0.
  Established facts:
  The instructions from 0x107e74 to 0x1081a8 are not reached by any path from the entry, and their branches are not listed.
  Branch (`ldrls` at 0x107e6c): taken to (unknown target) when `r0-35 <= 0x5a (unsigned, 32-bit)`, falls through to 0x107e70 when `r0-35 > 0x5a (unsigned, 32-bit)`.
  Omitted-path effects remain unknown at the unresolved transfer (`ldrls` at 0x107e6c).
  Call (`bl` at 0x1081ac): (unknown callee), target 0x201b4, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r0-35`.
  Known-edge recorded r0 = `unknown` (`ldm` at 0x1081bc).
