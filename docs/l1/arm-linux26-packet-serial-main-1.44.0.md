# ARM Linux 2.6 packet, serial and main routines in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-packet-serial-main-facts.txt) retain all tool sections.

The notation `ret(ADDR)` identifies a call-result expression without distinguishing
individual call sites to that target. Each return line records machine-register
state from the analysis rather than a C return type.

Groups count static records with the same width, access kind and resolved target expression. Address order need not describe one execution path.

## `PktGenMemOp`

- `PktGenMemOp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x84418 to 0x854d0.
  Established facts:
  The instructions from 0x85330 to 0x85334 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x85338 to 0x8537c are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x85380 to 0x853c4 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x853c8 to 0x853d0 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x853d4 to 0x853dc are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x853e0 to 0x853e4 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x853e8 to 0x853ec are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x853f0 to 0x85404 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x85408 to 0x85414 are not reached by any path from the entry, and their branches are not listed.
  Word load: target `abs 0x1a0888` (`ldr` at 0x8445c).
  Branch (`beq` at 0x8446c): taken to 0x8447c when `r0 == 0x1`, falls through to 0x84470 when `r0 != 0x1`.
  Word load: target `abs 0x1a6308` (`ldr` at 0x84490).
  Branch (`beq` at 0x84498): taken to 0x844c4 when `[0x1a6308] == 0x0`, falls through to 0x8449c when `[0x1a6308] != 0x0`.
  Word store: target `abs 0x31a730`, value `0x13d1a0` (`str` at 0x844a4).
  Word store: target `abs 0x31a72c`, value `[sp-40]` (`str` at 0x844b0).
  Call (`bl` at 0x84504): (unknown callee), target 0x29908, r0=`0x166de4`, r1=`r1`, r2=`[sp-192]`, r3=`unknown`.
  Call (`bl` at 0x84614): (unknown callee), target 0x13b298, r0=`unknown`, r1=`r1`, r2=`[sp-192]`, r3=`unknown`.
  Call (`bl` at 0x84628): (unknown callee), target 0x13ccc0, r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13b298)`.
  Call (`bl` at 0x8463c): (unknown callee), target 0x29908, r0=`0x166df8`, r1=`[sp-28]`, r2=`ret(0x13ccc0)`, r3=`ret(0x13ccc0)`.
  Call (`mov` at 0x8466c): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-120]]`, r2=`sp-72`, r3=`sp-76`.
  Call (`bl` at 0x84680): (unknown callee), target 0x29908, r0=`0x166e18`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x846d0): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-128]+32]`, r2=`sp-64`, r3=`sp-68`.
  Call (`mov` at 0x8472c): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-128]+4]`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x84740): (unknown callee), target 0x29908, r0=`0x166e38`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x8476c): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-128]]`, r2=`[sp-80]`, r3=`[sp-84]`.
  Call (`bl` at 0x84780): (unknown callee), target 0x29908, r0=`0x166e58`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x847b4): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-128]+28]`, r2=`[sp-80]`, r3=`[sp-84]`.
  Call (`bl` at 0x847c8): (unknown callee), target 0x29908, r0=`0x166e78`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x847fc): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-128]+24]`, r2=`[sp-80]`, r3=`[sp-84]`.
  Call (`bl` at 0x84810): (unknown callee), target 0x29908, r0=`0x166e98`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x848a8): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-128]+32]`, r2=`[sp-80]`, r3=`[sp-84]`.
  Call (`bl` at 0x848bc): (unknown callee), target 0x29908, r0=`0x166eb4`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x84910): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`unknown`, r2=`sp-80`, r3=`sp-84`.
  Call (`bl` at 0x84924): (unknown callee), target 0x29908, r0=`0x166ed4`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x8499c): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`unknown`, r2=`[sp-80]`, r3=`[sp-84]`.
  Call (`bl` at 0x849b0): (unknown callee), target 0x29908, r0=`0x166efc`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Halfword store: target `abs 0x1a6328`, value `0x145` (`strh` at 0x84a00).
  Byte store: target `abs 0x1a6312`, value `0x0` (`strb` at 0x84a80).
  Byte store: target `abs 0x1a6311`, value `0x0` (`strb` at 0x84a8c).
  Byte store: target `abs 0x1a6310`, value `0x0` (`strb` at 0x84a98).
  Word store: target `abs 0x1a6318`, value `unknown` (`str` at 0x84b10).
  Word store: target `abs 0x1a6324`, value `unknown` (`str` at 0x84b28).
  Byte load: target `abs 0x1a6317` (`ldrb` at 0x84b40).
  Byte load: target `abs 0x1a6316` (`ldrb` at 0x84b54).
  Halfword load: target `abs 0x1a6314` (`ldrh` at 0x84b60).
  Byte load: target `abs 0x1a6312` (`ldrb` at 0x84b78).
  Byte load: target `abs 0x1a6311` (`ldrb` at 0x84b8c).
  Byte load: target `abs 0x1a6310` (`ldrb` at 0x84b9c).
  Call (`mov` at 0x84bd0): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-120]+8]`, r2=`[sp-80]`, r3=`[sp-84]`.
  Call (`bl` at 0x84be4): (unknown callee), target 0x29908, r0=`0x166f24`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Halfword load: target `abs 0x1a6328` (`ldrh` at 0x84bf8).
  Byte load: target `abs 0x1a6321` (`ldrb` at 0x84c24).
  Byte load: target `abs 0x1a6320` (`ldrb` at 0x84c3c).
  Halfword load: target `abs 0x1a631e` (`ldrh` at 0x84c4c).
  Halfword load: target `abs 0x1a631c` (`ldrh` at 0x84c64).
  Call (`mov` at 0x84c9c): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-120]+12]`, r2=`[sp-80]`, r3=`[sp-84]`.
  Call (`bl` at 0x84cb0): (unknown callee), target 0x29908, r0=`0x166f44`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x84dc0): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x84dd8): (unknown callee), target 0x29908, r0=`0x166f64`, r1=`[sp-168]`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x84e38): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-120]+4]`, r2=`0x40000000`, r3=`0x8`.
  Call (`bl` at 0x84e4c): (unknown callee), target 0x29908, r0=`0x166f84`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x84eb4): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-120]+4]`, r2=`sp-80`, r3=`sp-84`.
  Call (`bl` at 0x84ec8): (unknown callee), target 0x29908, r0=`0x166fa8`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`bl` at 0x84efc): (unknown callee), target 0x29908, r0=`0x166fc0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`mov` at 0x84f64): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-128]+24]`, r2=`sp-80`, r3=`sp-84`.
  Call (`bl` at 0x84f78): (unknown callee), target 0x29908, r0=`0x166fd4`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`bl` at 0x84fc8): (unknown callee), target 0x29908, r0=`0x166fec`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`mov` at 0x85000): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-128]+4]`, r2=`sp-88`, r3=`sp-92`.
  Call (`bl` at 0x85014): (unknown callee), target 0x293f8, r0=`0x167000`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`bl` at 0x85038): (unknown callee), target 0x293f8, r0=`0x167014`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x8505c): (unknown callee), target 0x293f8, r0=`0x167024`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x8508c): (unknown callee), target 0x29908, r0=`0x167038`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`mov` at 0x850e4): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-128]+24]`, r2=`sp-180`, r3=`sp-184`.
  Call (`bl` at 0x850f8): (unknown callee), target 0x29908, r0=`0x166fd4`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x8512c): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-128]+4]`, r2=`[sp-80]`, r3=`[sp-84]`.
  Call (`bl` at 0x85140): (unknown callee), target 0x29908, r0=`0x167050`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x8516c): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-128]]`, r2=`[sp-80]`, r3=`[sp-84]`.
  Call (`bl` at 0x85180): (unknown callee), target 0x29908, r0=`0x167064`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x851b4): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-128]+40]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x851c8): (unknown callee), target 0x29908, r0=`0x16707c`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x851fc): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-128]+36]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x85210): (unknown callee), target 0x29908, r0=`0x167098`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x85248): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-128]+24]`, r2=`sp-180`, r3=`sp-184`.
  Call (`bl` at 0x8525c): (unknown callee), target 0x29908, r0=`0x166fd4`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`bl` at 0x852bc): (unknown callee), target 0x29908, r0=`0x1670b0`, r1=`unknown`, r2=`unknown`, r3=`[sp-148]`.
  Call (`mov` at 0x8530c): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-128]+32]`, r2=`[sp-64]`, r3=`[sp-68]`.
  Call (`bl` at 0x85320): (unknown callee), target 0x29908, r0=`0x1670c4`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x85488): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x8549c): (unknown callee), target 0x29908, r0=`0x1670e8`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Recorded r0 = `unknown` (`ldm` at 0x854cc).

## `PktGenOp`

- `PktGenOp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x85548 to 0x86e84.
  Established facts:
  The instructions from 0x864ac to 0x864cc are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x864d0 to 0x864f4 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x86500 to 0x86574 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x86578 to 0x8657c are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x86580 to 0x86588 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x8658c to 0x86590 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x86594 to 0x86598 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x8659c to 0x865a0 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x865a4 to 0x865cc are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x865d0 to 0x865ec are not reached by any path from the entry, and their branches are not listed.
  2 such word load accesses with resolved target `abs 0x1a6308`, including target `abs 0x1a6308` (`ldr` at 0x8559c), target `abs 0x1a6308` (`ldr` at 0x85d50).
  Branch (`beq` at 0x855a4): taken to 0x855d0 when `[0x1a6308] == 0x0`, falls through to 0x855a8 when `[0x1a6308] != 0x0`.
  Word store: target `abs 0x31a730`, value `0x13d1a0` (`str` at 0x855b0).
  Word store: target `abs 0x31a72c`, value `[sp-40]` (`str` at 0x855bc).
  Call (`bl` at 0x85610): (unknown callee), target 0x29908, r0=`0x166de4`, r1=`[sp-188]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x85718): (unknown callee), target 0x13b298, r0=`unknown`, r1=`[sp-188]`, r2=`[sp-188]`, r3=`unknown`.
  Branch (`beq` at 0x8572c): taken to 0x8576c when `ret(0x13b298) == 0x0`, falls through to 0x85730 when `ret(0x13b298) != 0x0` (source `bl` at 0x85718).
  Call (`bl` at 0x85738): (unknown callee), target 0x29908, r0=`0x167164`, r1=`[sp-28]`, r2=`unknown`, r3=`ret(0x13b298)`.
  Call (`mov` at 0x8578c): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-112]]`, r2=`sp-64`, r3=`sp-68`.
  Call (`bl` at 0x857a0): (unknown callee), target 0x29908, r0=`0x166e18`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x857f0): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-120]+32]`, r2=`sp-56`, r3=`sp-60`.
  Call (`mov` at 0x85840): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-120]+4]`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x85854): (unknown callee), target 0x29908, r0=`0x166e38`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x85888): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-120]+8]`, r2=`[sp-72]`, r3=`[sp-76]`.
  Call (`bl` at 0x8589c): (unknown callee), target 0x29908, r0=`0x167184`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x858d0): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-120]+12]`, r2=`[sp-72]`, r3=`[sp-76]`.
  Call (`bl` at 0x858e4): (unknown callee), target 0x29908, r0=`0x1671a4`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x85918): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-120]+16]`, r2=`[sp-72]`, r3=`[sp-76]`.
  Call (`bl` at 0x8592c): (unknown callee), target 0x29908, r0=`0x1671c4`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x85960): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-120]+20]`, r2=`[sp-72]`, r3=`[sp-76]`.
  Call (`bl` at 0x85974): (unknown callee), target 0x29908, r0=`0x1671e4`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x859a0): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-120]]`, r2=`[sp-72]`, r3=`[sp-76]`.
  Call (`bl` at 0x859b4): (unknown callee), target 0x29908, r0=`0x166e58`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x859e8): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-120]+28]`, r2=`[sp-72]`, r3=`[sp-76]`.
  Call (`bl` at 0x859fc): (unknown callee), target 0x29908, r0=`0x166e78`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x85a30): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-120]+24]`, r2=`[sp-72]`, r3=`[sp-76]`.
  Call (`bl` at 0x85a44): (unknown callee), target 0x29908, r0=`0x166e98`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x85acc): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-120]+32]`, r2=`unknown`, r3=`[sp-76]`.
  Call (`bl` at 0x85ae0): (unknown callee), target 0x29908, r0=`0x166eb4`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x85b34): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`unknown`, r2=`sp-72`, r3=`sp-76`.
  Call (`bl` at 0x85b48): (unknown callee), target 0x29908, r0=`0x166ed4`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x85bc0): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`unknown`, r2=`[sp-72]`, r3=`[sp-76]`.
  Call (`bl` at 0x85bd4): (unknown callee), target 0x29908, r0=`0x166efc`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`bl` at 0x85d64): (unknown callee), target 0x29908, r0=`0x167204`, r1=`[sp-48]`, r2=`unknown`, r3=`[0x1a6308]`.
  Call (`bl` at 0x85d70): (unknown callee), target 0x29908, r0=`0x167210`, r1=`[sp-52]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x85d7c): (unknown callee), target 0x29908, r0=`0x16721c`, r1=`[sp-32]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x85d88): (unknown callee), target 0x29908, r0=`0x167228`, r1=`[sp-36]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x85da0): (unknown callee), target 0x29908, r0=`0x167234`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x85db8): (unknown callee), target 0x29908, r0=`0x167240`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Halfword store: target `abs 0x1a6328`, value `0x145` (`strh` at 0x85dc4).
  Byte store: target `abs 0x1a6320`, value `([sp-52] & 0xff)` (`strb` at 0x85dd0).
  Halfword store: target `abs 0x1a631e`, value `([sp-48] & 0xffff)` (`strh` at 0x85ddc).
  Halfword store: target `abs 0x1a631c`, value `unknown` (`strh` at 0x85dec).
  Byte store: target `abs 0x1a6317`, value `([sp-36] & 0xff)` (`strb` at 0x85df8).
  Halfword store: target `abs 0x1a6314`, value `([sp-32] & 0xffff)` (`strh` at 0x85e04).
  Byte store: target `abs 0x1a6316`, value `([sp-205]:1 & 0xff)` (`strb` at 0x85e38).
  Byte store: target `abs 0x1a6312`, value `0x0` (`strb` at 0x85e44).
  Byte store: target `abs 0x1a6311`, value `0x0` (`strb` at 0x85e50).
  Byte store: target `abs 0x1a6310`, value `0x0` (`strb` at 0x85e5c).
  Byte store: target `abs 0x1a6321`, value `unknown` (`strb` at 0x85e7c).
  Word store: target `abs 0x1a6318`, value `unknown` (`str` at 0x85eb8).
  Word store: target `abs 0x1a6324`, value `unknown` (`str` at 0x85edc).
  Call (`mov` at 0x85f84): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-112]+8]`, r2=`[sp-72]`, r3=`[sp-76]`.
  Call (`bl` at 0x85f98): (unknown callee), target 0x29908, r0=`0x166f24`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x86050): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-112]+12]`, r2=`[sp-72]`, r3=`[sp-76]`.
  Call (`bl` at 0x86064): (unknown callee), target 0x29908, r0=`0x166f44`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x860d4): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-112]+16]`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x860e8): (unknown callee), target 0x29908, r0=`0x16724c`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x8613c): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-112]+20]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x86150): (unknown callee), target 0x29908, r0=`0x167270`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x861a4): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-112]+16]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x861b8): (unknown callee), target 0x29908, r0=`0x16724c`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x861ec): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-112]+20]`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x86200): (unknown callee), target 0x29908, r0=`0x167270`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x86240): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-112]+4]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x86254): (unknown callee), target 0x29908, r0=`0x166f84`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x862bc): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-112]+4]`, r2=`sp-72`, r3=`sp-76`.
  Call (`bl` at 0x862d0): (unknown callee), target 0x29908, r0=`0x166fa8`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`bl` at 0x86300): (unknown callee), target 0x86e84, r0=`0xa`, r1=`unknown`, r2=`unknown`, r3=`[sp-140]`.
  Call (`bl` at 0x86318): (unknown callee), target 0x29908, r0=`0x166fc0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`mov` at 0x86380): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-120]+24]`, r2=`sp-72`, r3=`sp-76`.
  Call (`bl` at 0x86394): (unknown callee), target 0x29908, r0=`0x166fd4`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`bl` at 0x863e0): (unknown callee), target 0x86e84, r0=`0xa`, r1=`unknown`, r2=`unknown`, r3=`[sp-212]`.
  Call (`bl` at 0x863f8): (unknown callee), target 0x29908, r0=`0x166fec`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`mov` at 0x8643c): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-120]+4]`, r2=`sp-80`, r3=`sp-84`.
  Call (`bl` at 0x86450): (unknown callee), target 0x293f8, r0=`0x167000`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x86488): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-120]+8]`, r2=`sp-88`, r3=`sp-92`.
  Call (`bl` at 0x8649c): (unknown callee), target 0x293f8, r0=`0x167000`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x86618): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-120]+12]`, r2=`sp-96`, r3=`sp-100`.
  Call (`bl` at 0x8662c): (unknown callee), target 0x293f8, r0=`0x167000`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x86664): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-120]+16]`, r2=`sp-104`, r3=`sp-108`.
  Call (`bl` at 0x86678): (unknown callee), target 0x293f8, r0=`0x167000`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`bl` at 0x8669c): (unknown callee), target 0x293f8, r0=`0x167014`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x866d0): (unknown callee), target 0x854d0, r0=`[sp-80]`, r1=`[sp-84]`, r2=`[sp-88]`, r3=`[sp-92]`.
  Call (`bl` at 0x866f4): (unknown callee), target 0x293f8, r0=`0x167024`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x86728): (unknown callee), target 0x854d0, r0=`[sp-80]`, r1=`[sp-84]`, r2=`[sp-88]`, r3=`[sp-92]`.
  Call (`bl` at 0x86758): (unknown callee), target 0x29908, r0=`0x167038`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x8678c): (unknown callee), target 0x854d0, r0=`[sp-80]`, r1=`[sp-84]`, r2=`[sp-88]`, r3=`[sp-92]`.
  Call (`bl` at 0x86840): (unknown callee), target 0x29908, r0=`0x167294`, r1=`[sp-88]`, r2=`[sp-92]`, r3=`unknown`.
  Call (`bl` at 0x86874): (unknown callee), target 0x854d0, r0=`[sp-80]`, r1=`[sp-84]`, r2=`[sp-88]`, r3=`[sp-92]`.
  Call (`mov` at 0x8694c): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-120]+24]`, r2=`sp-168`, r3=`sp-172`.
  Call (`bl` at 0x86960): (unknown callee), target 0x29908, r0=`0x166fd4`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x86994): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-120]+4]`, r2=`[sp-72]`, r3=`[sp-76]`.
  Call (`bl` at 0x869a8): (unknown callee), target 0x29908, r0=`0x167050`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x869dc): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-120]+8]`, r2=`[sp-72]`, r3=`[sp-76]`.
  Call (`bl` at 0x869f0): (unknown callee), target 0x29908, r0=`0x1672b8`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x86a24): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-120]+12]`, r2=`[sp-72]`, r3=`[sp-76]`.
  Call (`bl` at 0x86a38): (unknown callee), target 0x29908, r0=`0x1672cc`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x86a6c): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-120]+16]`, r2=`[sp-72]`, r3=`[sp-76]`.
  Call (`bl` at 0x86a80): (unknown callee), target 0x29908, r0=`0x1672e0`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x86ab4): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-120]+20]`, r2=`[sp-72]`, r3=`[sp-76]`.
  Call (`bl` at 0x86ac8): (unknown callee), target 0x29908, r0=`0x1672f4`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x86af4): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-120]]`, r2=`[sp-72]`, r3=`[sp-76]`.
  Call (`bl` at 0x86b08): (unknown callee), target 0x29908, r0=`0x167064`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x86b40): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-120]+28]`, r2=`sp-176`, r3=`sp-180`.
  Call (`bl` at 0x86b54): (unknown callee), target 0x29908, r0=`0x166fd4`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x86b88): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-120]+40]`, r2=`[sp-176]`, r3=`[sp-180]`.
  Call (`bl` at 0x86b9c): (unknown callee), target 0x29908, r0=`0x16707c`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x86bd0): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-120]+36]`, r2=`[sp-168]`, r3=`[sp-172]`.
  Call (`bl` at 0x86be4): (unknown callee), target 0x29908, r0=`0x167098`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x86c1c): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-120]+24]`, r2=`sp-168`, r3=`sp-172`.
  Call (`bl` at 0x86c30): (unknown callee), target 0x29908, r0=`0x166fd4`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`bl` at 0x86c90): (unknown callee), target 0x29908, r0=`0x1670b0`, r1=`unknown`, r2=`unknown`, r3=`[sp-140]`.
  Call (`mov` at 0x86cdc): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`0x90010000`, r2=`sp-72`, r3=`sp-76`.
  Call (`bl` at 0x86cf0): (unknown callee), target 0x29908, r0=`0x167308`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x86d24): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`0x90010000`, r2=`[sp-72]`, r3=`[sp-76]`.
  Call (`bl` at 0x86d38): (unknown callee), target 0x29908, r0=`0x167320`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x86d60): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`0x90071100`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x86d74): (unknown callee), target 0x29908, r0=`0x167320`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x86da8): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-120]+32]`, r2=`[sp-56]`, r3=`[sp-60]`.
  Call (`bl` at 0x86dbc): (unknown callee), target 0x29908, r0=`0x1670c4`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`mov` at 0x86e3c): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x86e50): (unknown callee), target 0x29908, r0=`0x1670e8`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Recorded r0 = `unknown` (`ldm` at 0x86e80).

## `PktGenWrite`

- `PktGenWrite`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x86f7c to 0x87038.
  Established facts:
  Call (`bl` at 0x8701c): (unknown callee), target 0x85548, r0=`0x1`, r1=`[sp-20]`, r2=`[sp-24]`, r3=`[sp-28]`.
  Recorded r0 = `ret(0x85548)` (`ldm` at 0x87034, `bl` at 0x8701c).

## `main_task`

- `main_task`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9173c to 0x9192c.
  Established facts:
  The instructions from 0x9190c to 0x9192c are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x9176c): (unknown callee), target 0x24af0, r0=`r0`, r1=`r1`, r2=`r2`, r3=`0x1e`.
  Call (`bl` at 0x91778): (unknown callee), target 0x29908, r0=`0x168674`, r1=`0x168684`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x9177c): (unknown callee), target 0x81480, r0=`ret(0x29908)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x91780): (unknown callee), target 0xf679c, r0=`ret(0x81480)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x91788): (unknown callee), target 0x28230, r0=`0x1`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x917a0): (unknown callee), target 0x28230, r0=`0x2`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x917b4): (unknown callee), target 0xfb4f8, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x917b8): (unknown callee), target 0x6cff8, r0=`ret(0xfb4f8)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  2 such word load accesses with resolved target `abs 0x3182f8`, including target `abs 0x3182f8` (`ldr` at 0x917c0), target `abs 0x3182f8` (`ldr` at 0x917d0).
  Branch (`beq` at 0x917c8): taken to 0x917dc when `[0x3182f8] == 0x0`, falls through to 0x917cc when `[0x3182f8] != 0x0`.
  Call (`mov` at 0x917d8): (unknown callee), target (unknown target), r0=`ret(0x6cff8)`, r1=`unknown`, r2=`unknown`, r3=`[0x3182f8]`.
  Call (`bl` at 0x917e0): (unknown callee), target 0x90b74, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x917f0): (unknown callee), target 0x10ebe4, r0=`0x1`, r1=`0x2`, r2=`0x9600`, r3=`unknown`.
  Call (`bl` at 0x917f8): (unknown callee), target 0x23fe0, r0=`0x1`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x91804): (unknown callee), target 0x21ff0, r0=`0x0`, r1=`0x0`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x9180c): (unknown callee), target 0x110180, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x91810): (unknown callee), target 0x1341a0, r0=`ret(0x110180)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x91814): (unknown callee), target 0xb7358, r0=`ret(0x1341a0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Word load: target `abs 0x3182fc` (`ldr` at 0x9181c).
  Call (`mov` at 0x91824): (unknown callee), target (unknown target), r0=`ret(0xb7358)`, r1=`unknown`, r2=`unknown`, r3=`[0x3182fc]`.
  Call (`bl` at 0x91828): (unknown callee), target 0x3ef70, r0=`ret(unknown)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x9182c): (unknown callee), target 0x6fb9c, r0=`ret(0x3ef70)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x91830): (unknown callee), target 0x21120, r0=`ret(0x6fb9c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x91834): (unknown callee), target 0xbedb4, r0=`ret(0x21120)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x91838): (unknown callee), target 0xd20b0, r0=`ret(0xbedb4)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x9183c): (unknown callee), target 0xe05d4, r0=`ret(0xd20b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x91840): (unknown callee), target 0xf0e48, r0=`ret(0xe05d4)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x91844): (unknown callee), target 0x955f0, r0=`ret(0xf0e48)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x91848): (unknown callee), target 0x99350, r0=`ret(0x955f0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x91850): (unknown callee), target 0x927d8, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x91854): (unknown callee), target 0xa9e44, r0=`ret(0x927d8)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x91858): (unknown callee), target 0x13abf8, r0=`ret(0xa9e44)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x9185c): (unknown callee), target 0xe3b08, r0=`ret(0x13abf8)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x91860): (unknown callee), target 0x10b8d4, r0=`ret(0xe3b08)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  2 such word load accesses with resolved target `abs 0x1b0eac`, including target `abs 0x1b0eac` (`ldr` at 0x91868), target `abs 0x1b0eac` (`ldr` at 0x91878).
  Branch (`beq` at 0x91870): taken to 0x91884 when `[0x1b0eac] == 0x0`, falls through to 0x91874 when `[0x1b0eac] != 0x0`.
  Call (`mov` at 0x91880): (unknown callee), target (unknown target), r0=`ret(0x10b8d4)`, r1=`unknown`, r2=`unknown`, r3=`[0x1b0eac]`.
  Call (`bl` at 0x91884): (unknown callee), target 0x91954, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x91888): (unknown callee), target 0xa5fcc, r0=`ret(0x91954)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x9188c): (unknown callee), target 0xb7aa0, r0=`ret(0xa5fcc)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x91890): (unknown callee), target 0x6dc0c, r0=`ret(0xb7aa0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`bne` at 0x918a0): taken to 0x918c8 when `unknown != 0x3f`, falls through to 0x918a4 when `unknown == 0x3f`.
  Call (`bl` at 0x918b0): (unknown callee), target 0x706a0, r0=`0x31`, r1=`0x16868c`, r2=`0x2`, r3=`unknown`.
  Call (`bl` at 0x918c0): (unknown callee), target 0x706a0, r0=`0x32`, r1=`0x16869c`, r2=`0x2`, r3=`unknown`.
  Call (`bl` at 0x918c8): (unknown callee), target 0x9662c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x918d8): (unknown callee), target 0x706a0, r0=`0x32`, r1=`0x1686ac`, r2=`0x0`, r3=`unknown`.
  Call (`bl` at 0x918dc): (unknown callee), target 0x20fd8, r0=`ret(0x706a0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0x918e8): taken to 0x918fc when `ret(0x20fd8) == 0x0`, falls through to 0x918ec when `ret(0x20fd8) != 0x0` (source `bl` at 0x918dc).
  Call (`bl` at 0x918f8): (unknown callee), target 0x706a0, r0=`0x32`, r1=`0x1686b8`, r2=`0x1`, r3=`ret(0x20fd8)`.
  Call (`bl` at 0x91900): (unknown callee), target 0x9192c, r0=`0xc8`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x91904): (unknown callee), target 0x5e4c8, r0=`ret(0x9192c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  The returns section reports no exit for this span.

## `main`

- `main`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x20948 to 0x20ce0.
  Established facts:
  The instructions from 0x20c94 to 0x20ce0 are not reached by any path from the entry, and their branches are not listed.
  Word store: target `abs 0x1a28a4`, value `[r1]` (`str` at 0x20974).
  Call (`bl` at 0x20984): (unknown callee), target 0x20490, r0=`sp-164`, r1=`r1`, r2=`0x1a28a4`, r3=`sp-164`.
  Call (`bl` at 0x209a8): (unknown callee), target 0x2001c, r0=`0xb`, r1=`sp-168`, r2=`0x0`, r3=`sp-168`.
  Call (`bl` at 0x209bc): (unknown callee), target 0x2001c, r0=`0x7`, r1=`sp-168`, r2=`0x0`, r3=`sp-168`.
  Call (`bl` at 0x209d0): (unknown callee), target 0x2001c, r0=`0x6`, r1=`sp-168`, r2=`0x0`, r3=`sp-168`.
  Call (`bl` at 0x209f4): (unknown callee), target 0x2001c, r0=`0xa`, r1=`sp-168`, r2=`0x0`, r3=`sp-168`.
  Call (`bl` at 0x20a00): (unknown callee), target 0x202c8, r0=`sp-180`, r1=`unknown`, r2=`unknown`, r3=`sp-180`.
  Call (`bl` at 0x20a08): (unknown callee), target 0x20448, r0=`[sp-180]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`ble` at 0x20a14): taken to 0x20a60 when `[sp-24] <= 0x1 (signed, 32-bit)`, falls through to 0x20a18 when `[sp-24] > 0x1 (signed, 32-bit)`.
  Call (`bl` at 0x20a2c): (unknown callee), target 0x200ac, r0=`[[sp-28]+4]`, r1=`0x1502f4`, r2=`0x4`, r3=`[sp-28]+4`.
  Branch (`bne` at 0x20a38): taken to 0x20a60 when `ret(0x200ac) != 0x0`, falls through to 0x20a3c when `ret(0x200ac) == 0x0` (source `bl` at 0x20a2c).
  Call (`bl` at 0x20a3c): (unknown callee), target 0x24af0, r0=`ret(0x200ac)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x200ac)`.
  Call (`bl` at 0x20a44): (unknown callee), target 0x28230, r0=`0x46`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x20a54): (unknown callee), target 0x243d4, r0=`0x1502fc`, r1=`ret(0x28230)`, r2=`unknown`, r3=`ret(0x28230)`.
  Call (`bl` at 0x20a5c): (unknown callee), target 0x204b4, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`ble` at 0x20a68): taken to 0x20ac4 when `[sp-24] <= 0x1 (signed, 32-bit)`, falls through to 0x20a6c when `[sp-24] > 0x1 (signed, 32-bit)`.
  Call (`bl` at 0x20a80): (unknown callee), target 0x200ac, r0=`[[sp-28]+4]`, r1=`0x150304`, r2=`0x4`, r3=`[sp-28]+4`.
  Branch (`bne` at 0x20a8c): taken to 0x20ac4 when `ret(0x200ac) != 0x0`, falls through to 0x20a90 when `ret(0x200ac) == 0x0` (source `bl` at 0x20a80).
  Call (`bl` at 0x20a90): (unknown callee), target 0x24af0, r0=`ret(0x200ac)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x200ac)`.
  Call (`bl` at 0x20a98): (unknown callee), target 0x28230, r0=`0x1`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x20aa4): (unknown callee), target 0x28230, r0=`0x2`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x20ab8): (unknown callee), target 0x243d4, r0=`0x150308`, r1=`ret(0x28230)`, r2=`ret(0x28230)`, r3=`ret(0x28230)`.
  Call (`bl` at 0x20ac0): (unknown callee), target 0x204b4, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`ble` at 0x20acc): taken to 0x20afc when `[sp-24] <= 0x1 (signed, 32-bit)`, falls through to 0x20ad0 when `[sp-24] > 0x1 (signed, 32-bit)`.
  Call (`bl` at 0x20ae4): (unknown callee), target 0x200ac, r0=`[[sp-28]+4]`, r1=`0x150310`, r2=`0x4`, r3=`[sp-28]+4`.
  Branch (`bne` at 0x20af0): taken to 0x20afc when `ret(0x200ac) != 0x0`, falls through to 0x20af4 when `ret(0x200ac) == 0x0` (source `bl` at 0x20ae4).
  Branch (`ble` at 0x20b04): taken to 0x20b68 when `[sp-24] <= 0x1 (signed, 32-bit)`, falls through to 0x20b08 when `[sp-24] > 0x1 (signed, 32-bit)`.
  Call (`bl` at 0x20b1c): (unknown callee), target 0x200ac, r0=`[[sp-28]+4]`, r1=`0x150314`, r2=`0x4`, r3=`[sp-28]+4`.
  Branch (`bne` at 0x20b28): taken to 0x20b68 when `ret(0x200ac) != 0x0`, falls through to 0x20b2c when `ret(0x200ac) == 0x0` (source `bl` at 0x20b1c).
  Branch (`bgt` at 0x20b34): taken to 0x20b50 when `[sp-24] > 0x2 (signed, 32-bit)`, falls through to 0x20b38 when `[sp-24] <= 0x2 (signed, 32-bit)`.
  Word load: target `abs 0x1a288c` (`ldr` at 0x20b3c).
  Call (`bl` at 0x20b44): (unknown callee), target 0x244d4, r0=`[0x1a288c]`, r1=`0x15031c`, r2=`unknown`, r3=`0x1a288c`.
  Call (`bl` at 0x20b4c): (unknown callee), target 0x204b4, r0=`0x1`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Word store: target `abs 0x1a28a0`, value `[[sp-28]+8]` (`str` at 0x20b64).
  Call (`bl` at 0x20b70): (unknown callee), target 0x243d4, r0=`0x150348`, r1=`0x1502dc`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x20b74): (unknown callee), target 0x20f8c, r0=`ret(0x243d4)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0x20b88): taken to 0x20bb8 when `ret(0x20f8c) == 0x0`, falls through to 0x20b8c when `ret(0x20f8c) != 0x0` (source `bl` at 0x20b74).
  Call (`bl` at 0x20b90): (unknown callee), target 0x219a4, r0=`ret(0x20f8c)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x20f8c)`.
  Call (`bl` at 0x20bb4): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x1502e4`, r2=`0xdd`, r3=`0x0`.
  Call (`bl` at 0x20bc0): (unknown callee), target 0x9173c, r0=`[sp-172]`, r1=`0x0`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x20bc8): (unknown callee), target 0x1fff8, r0=`0x186a0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Byte load: target `abs 0x3162f0` (`ldrb` at 0x20bd0).
  Branch (`beq` at 0x20bd8): taken to 0x20bc4 when `[0x3162f0]:1 == 0x0`, falls through to 0x20bdc when `[0x3162f0]:1 != 0x0`.
  Branch (`beq` at 0x20bf8): taken to 0x20c60 when `Z == 1`, falls through to 0x20bfc when `not (Z == 1)`.
  2 such word load accesses with resolved target `abs 0x19e338`, including target `abs 0x19e338` (`ldr` at 0x20c04), target `abs 0x19e338` (`ldr` at 0x20c30).
  Call (`bl` at 0x20c08): (unknown callee), target 0x20310, r0=`unknown`, r1=`[0x19e338]`, r2=`unknown`, r3=`0x19e338`.
  Branch (`beq` at 0x20c1c): taken to 0x20c60 when `ret(0x20310) == 0x0`, falls through to 0x20c20 when `ret(0x20310) != 0x0` (source `bl` at 0x20c08).
  Call (`bl` at 0x20c34): (unknown callee), target 0x2031c, r0=`[0x19e338]`, r1=`unknown`, r2=`ret(0x20310)`, r3=`0x19e338`.
  Call (`bl` at 0x20c50): (unknown callee), target 0x293f8, r0=`[sp-188]`, r1=`unknown`, r2=`ret(0x2031c)`, r3=`([sp-184] + ret(0x2031c))`.
  Branch (`beq` at 0x20c68): taken to 0x20c84 when `Z == 1`, falls through to 0x20c6c when `not (Z == 1)`.
  Branch (`beq` at 0x20c78): taken to 0x20c84 when `Z == 1`, falls through to 0x20c7c when `not (Z == 1)`.
  Call (`bl` at 0x20c80): (unknown callee), target 0x293f8, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Byte store: target `abs 0x3162f0`, value `0x0` (`strb` at 0x20c8c).
  The returns section reports no exit for this span.

## `sernumGetKey`

- `sernumGetKey`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6e4ac to 0x6e628.
  Established facts:
  The instructions from 0x6e618 to 0x6e628 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x6e4c4): (unknown callee), target 0x6dc0c, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0x6e4d4): (unknown callee), target 0x6e190, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x6e4e8): (unknown callee), target 0x120340, r0=`ret(0x6e190)`, r1=`sp-28`, r2=`sp-28`, r3=`ret(0x6e190)`.
  Call (`bl` at 0x6e4ec): (unknown callee), target 0x6dc0c, r0=`ret(0x120340)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Word load: target `[sp-28] +0x1c` (`ldr` at 0x6e510).
  Branch (`beq` at 0x6e518): taken to 0x6e530 when `[[sp-28]+28] == 0x0`, falls through to 0x6e51c when `[[sp-28]+28] != 0x0`.
  Call (`bl` at 0x6e520): (unknown callee), target 0x293f8, r0=`0x164c3c`, r1=`unknown`, r2=`unknown`, r3=`[[sp-28]+28]`.
  Word load: target `[sp-28] +0x70` (`ldr` at 0x6e534).
  Word load: target `[sp-28] +0x6c` (`ldr` at 0x6e540).
  Word load: target `[sp-28] +0x74` (`ldr` at 0x6e54c).
  Branch (`beq` at 0x6e5d8): taken to 0x6e5e4 when `[sp-24] == 0x0`, falls through to 0x6e5dc when `[sp-24] != 0x0`.
  Call (`bl` at 0x6e5e0): (unknown callee), target 0x243d4, r0=`0x164c70`, r1=`[[sp-28]+112]`, r2=`unknown`, r3=`[sp-24]`.
  Call (`bl` at 0x6e5f4): (unknown callee), target 0x29908, r0=`0x164ca0`, r1=`[sp-56]`, r2=`[sp-60]`, r3=`[sp-64]`.
  Recorded r0 = `unknown` (`ldm` at 0x6e614).

## `sernumUnscrambleExtra`

- `sernumUnscrambleExtra`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6ed70 to 0x6eeb4.
  Established facts:
  Call (`bl` at 0x6eda0): (unknown callee), target 0x6f00c, r0=`r0`, r1=`0x0`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x6edb8): (unknown callee), target 0x6f00c, r0=`[sp-24]`, r1=`0x1`, r2=`0x1`, r3=`ret(0x6f00c)`.
  Call (`bl` at 0x6edc8): (unknown callee), target 0x2031c, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x6f00c)`.
  Branch (`bne` at 0x6ede0): taken to 0x6ee28 when `[sp-36] != 0x0`, falls through to 0x6ede4 when `[sp-36] == 0x0`.
  Call (`bl` at 0x6edf0): (unknown callee), target 0x6f00c, r0=`[sp-24]`, r1=`0x2`, r2=`ret(0x2031c)-1`, r3=`[sp-36]`.
  Call (`bl` at 0x6ee04): (unknown callee), target 0x6efa4, r0=`0xa`, r1=`[sp-48]`, r2=`unknown`, r3=`ret(0x6f00c)`.
  Call (`bl` at 0x6ee18): (unknown callee), target 0x150058, r0=`[sp-40]`, r1=`ret(0x6efa4)`, r2=`ret(0x6efa4)`, r3=`[sp-40]`.
  Branch (`bgt` at 0x6ee44): taken to 0x6ee90 when `gt flags condition (operands unknown)`, falls through to 0x6ee48 when `not (gt flags condition (operands unknown))`.
  Byte load: target `(unknown effective address)` (`ldrb` at 0x6ee60).
  Call (`bl` at 0x6ee68): (unknown callee), target 0x6eeb4, r0=`unknown`, r1=`unknown`, r2=`[sp-24]`, r3=`unknown`.
  Byte store: target `(unknown effective address)`, value `(ret(0x6eeb4) & 0xff)` (`strb` at 0x6ee70, `bl` at 0x6ee68).
  Byte store: target `(unknown effective address)`, value `0x0` (`strb` at 0x6eea0).
  Recorded r0 = `[sp-40]` (`ldm` at 0x6eeb0).

## `sernumCodedStringToInt`

- `sernumCodedStringToInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6f00c to 0x6f0e0.
  Established facts:
  Branch (`bgt` at 0x6f054): taken to 0x6f0d0 when `gt flags condition (operands unknown)`, falls through to 0x6f058 when `not (gt flags condition (operands unknown))`.
  Byte load: target `(unknown effective address)` (`ldrb` at 0x6f064).
  Call (`bl` at 0x6f06c): (unknown callee), target 0x6eeb4, r0=`unknown`, r1=`unknown`, r2=`[sp-28]`, r3=`unknown`.
  Call (`bl` at 0x6f07c): (unknown callee), target 0x6efa4, r0=`0xa`, r1=`[sp-40]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x6f098): (unknown callee), target 0x6efa4, r0=`0xa`, r1=`unknown`, r2=`[sp-48]`, r3=`unknown`.
  Call (`bl` at 0x6f0a8): (unknown callee), target 0x14fe64, r0=`ret(0x6efa4)`, r1=`ret(0x6efa4)`, r2=`unknown`, r3=`ret(0x6efa4)`.
  Recorded r0 = `unknown` (`ldm` at 0x6f0dc).

## `sernumBootCheck`

- `sernumBootCheck`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6f204 to 0x6f7c0.
  Established facts:
  The instructions from 0x6f744 to 0x6f754 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x6f758 to 0x6f764 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x6f768 to 0x6f788 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x6f78c to 0x6f7a8 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x6f7ac to 0x6f7bc are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x6f220): (unknown callee), target 0x28230, r0=`0xd`, r1=`r1`, r2=`r2`, r3=`0x0`.
  Call (`bl` at 0x6f22c): (unknown callee), target 0x28230, r0=`0xf`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x6f238): (unknown callee), target 0x28230, r0=`0x50`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0x6f244): taken to 0x6f254 when `ret(0x28230) == 0x0`, falls through to 0x6f248 when `ret(0x28230) != 0x0` (source `bl` at 0x6f238).
  Call (`bl` at 0x6f254): (unknown callee), target 0x6debc, r0=`ret(0x28230)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x28230)`.
  Branch (`bne` at 0x6f264): taken to 0x6f290 when `ret(0x6debc) != 0x0`, falls through to 0x6f268 when `ret(0x6debc) == 0x0` (source `bl` at 0x6f254).
  Call (`bl` at 0x6f280): (unknown callee), target 0x22070, r0=`0x1`, r1=`0x164d90`, r2=`0x238`, r3=`0x0`.
  Call (`bl` at 0x6f29c): (unknown callee), target 0x20340, r0=`[sp-32]`, r1=`ret(0x6debc)`, r2=`0x6`, r3=`ret(0x6debc)`.
  Call (`bl` at 0x6f2b0): (unknown callee), target 0x293f8, r0=`0x164da8`, r1=`unknown`, r2=`unknown`, r3=`ret(0x20340)`.
  Call (`bl` at 0x6f2c0): (unknown callee), target 0x29908, r0=`0x164dc8`, r1=`[sp-32]`, r2=`[sp-28]`, r3=`unknown`.
  Call (`bl` at 0x6f2e0): (unknown callee), target 0x200ac, r0=`[sp-24]+1`, r1=`0x164dec`, r2=`unknown`, r3=`[sp-24]+1`.
  Branch (`beq` at 0x6f2ec): taken to 0x6f310 when `ret(0x200ac) == 0x0`, falls through to 0x6f2f0 when `ret(0x200ac) != 0x0` (source `bl` at 0x6f2e0).
  Call (`bl` at 0x6f300): (unknown callee), target 0x200ac, r0=`[sp-24]+1`, r1=`0x164df4`, r2=`unknown`, r3=`[sp-24]+1`.
  Branch (`bne` at 0x6f30c): taken to 0x6f328 when `ret(0x200ac) != 0x0`, falls through to 0x6f310 when `ret(0x200ac) == 0x0` (source `bl` at 0x6f300).
  Call (`bl` at 0x6f318): (unknown callee), target 0x29908, r0=`0x164dfc`, r1=`[sp-24]`, r2=`unknown`, r3=`ret(0x200ac)`.
  3 such byte load accesses with resolved target `abs 0x318308`, including target `abs 0x318308` (`ldrb` at 0x6f32c), target `abs 0x318308` (`ldrb` at 0x6f340), target `abs 0x318308` (`ldrb` at 0x6f5c0).
  Branch (`beq` at 0x6f334): taken to 0x6f35c when `[0x318308]:1 == 0x0`, falls through to 0x6f338 when `[0x318308]:1 != 0x0`.
  Call (`bl` at 0x6f404): (unknown callee), target 0x205ec, r0=`ret(0x200ac)`, r1=`unknown`, r2=`0x1`, r3=`[[sp-24]+1]:1`.
  6 such word load accesses with resolved target `ret(0x205ec) +0x0`, including target `ret(0x205ec) +0x0` (`ldr` at 0x6f424, `bl` at 0x6f404), target `ret(0x205ec) +0x0` (`ldr` at 0x6f45c, `bl` at 0x6f43c), target `ret(0x205ec) +0x0` (`ldr` at 0x6f494, `bl` at 0x6f474).
  Call (`bl` at 0x6f43c): (unknown callee), target 0x205ec, r0=`ret(0x205ec)`, r1=`ret(0x205ec)`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x6f474): (unknown callee), target 0x205ec, r0=`ret(0x205ec)`, r1=`ret(0x205ec)`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x6f4ac): (unknown callee), target 0x205ec, r0=`ret(0x205ec)`, r1=`ret(0x205ec)`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x6f4e4): (unknown callee), target 0x205ec, r0=`ret(0x205ec)`, r1=`ret(0x205ec)`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x6f51c): (unknown callee), target 0x205ec, r0=`ret(0x205ec)`, r1=`ret(0x205ec)`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x6f56c): (unknown callee), target 0x293f8, r0=`0x164e1c`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x6f578): (unknown callee), target 0x29908, r0=`0x164e34`, r1=`[sp-24]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x6f5cc): (unknown callee), target 0x29908, r0=`0x164e40`, r1=`[0x318308]:1`, r2=`unknown`, r3=`[0x318308]:1`.
  Call (`bl` at 0x6f5dc): (unknown callee), target 0x29908, r0=`0x164e6c`, r1=`0x164eac`, r2=`unknown`, r3=`[sp-20]`.
  Call (`bl` at 0x6f5e8): (unknown callee), target 0x29908, r0=`0x164ebc`, r1=`unknown`, r2=`unknown`, r3=`[sp-20]`.
  Call (`bl` at 0x6f5f4): (unknown callee), target 0x29908, r0=`0x164efc`, r1=`unknown`, r2=`unknown`, r3=`[sp-20]`.
  Call (`bl` at 0x6f60c): (unknown callee), target 0x200ac, r0=`[sp-24]`, r1=`0x31b96c`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0x6f618): taken to 0x6f6d4 when `ret(0x200ac) == 0x0`, falls through to 0x6f61c when `ret(0x200ac) != 0x0` (source `bl` at 0x6f60c).
  Call (`bl` at 0x6f620): (unknown callee), target 0x293f8, r0=`0x164f24`, r1=`unknown`, r2=`unknown`, r3=`ret(0x200ac)`.
  Call (`bl` at 0x6f638): (unknown callee), target 0x205bc, r0=`sp-112`, r1=`0x164f44`, r2=`[sp-24]`, r3=`0x31b96c`.
  Word load: target `abs 0x1b0f54` (`ldr` at 0x6f640).
  Call (`bl` at 0x6f690): (unknown callee), target 0x203c4, r0=`sp-112`, r1=`0x164f6c`, r2=`unknown`, r3=`sp-112`.
  Call (`bl` at 0x6f6a4): (unknown callee), target 0x203c4, r0=`sp-112`, r1=`0x164f80`, r2=`unknown`, r3=`sp-112`.
  Call (`bl` at 0x6f6b8): (unknown callee), target 0x203c4, r0=`sp-112`, r1=`0x164f90`, r2=`unknown`, r3=`sp-112`.
  Call (`bl` at 0x6f6c4): (unknown callee), target 0x293f8, r0=`sp-112`, r1=`unknown`, r2=`unknown`, r3=`sp-112`.
  Call (`bl` at 0x6f6e4): (unknown callee), target 0x200ac, r0=`[sp-24]+1`, r1=`0x164fa0`, r2=`unknown`, r3=`[sp-24]+1`.
  Call (`bl` at 0x6f6fc): (unknown callee), target 0x200ac, r0=`0x31b96d`, r1=`0x164fa0`, r2=`unknown`, r3=`ret(0x200ac)`.
  Call (`bl` at 0x6f710): (unknown callee), target 0x293f8, r0=`0x164fa8`, r1=`unknown`, r2=`unknown`, r3=`ret(0x200ac)`.
  Call (`bl` at 0x6f720): (unknown callee), target 0x29908, r0=`0x164fbc`, r1=`[sp-24]`, r2=`0x31b96c`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x6f740).

## `sernumInit`

- `sernumInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6fb9c to 0x70214.
  Established facts:
  The instructions from 0x701f4 to 0x70214 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x6fbb4): (unknown callee), target 0x6dc0c, r0=`r0`, r1=`r1`, r2=`r2`, r3=`0x0`.
  Call (`bl` at 0x6fbc8): (unknown callee), target 0x28230, r0=`0x3d`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x6fbd4): (unknown callee), target 0x28230, r0=`0xd`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x6fbe0): (unknown callee), target 0x28230, r0=`0xf`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  2 such word load accesses with resolved target `abs 0x318474`, including target `abs 0x318474` (`ldr` at 0x6fbec), target `abs 0x318474` (`ldr` at 0x6fbfc).
  Branch (`beq` at 0x6fbf4): taken to 0x6fc0c when `[0x318474] == 0x0`, falls through to 0x6fbf8 when `[0x318474] != 0x0`.
  Call (`mov` at 0x6fc04): (unknown callee), target (unknown target), r0=`ret(0x28230)`, r1=`unknown`, r2=`unknown`, r3=`[0x318474]`.
  Call (`bl` at 0x6fc14): (unknown callee), target 0x2624c, r0=`0x54`, r1=`0x1`, r2=`unknown`, r3=`[0x318474]`.
  Call (`bl` at 0x6fc18): (unknown callee), target 0x6debc, r0=`ret(0x2624c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`bne` at 0x6fc28): taken to 0x6fc50 when `ret(0x6debc) != 0x0`, falls through to 0x6fc2c when `ret(0x6debc) == 0x0` (source `bl` at 0x6fc18).
  Call (`bl` at 0x6fc44): (unknown callee), target 0x22070, r0=`0x1`, r1=`0x164d90`, r2=`0x2ff`, r3=`0x0`.
  Call (`bl` at 0x6fd00): (unknown callee), target 0x2624c, r0=`0x50`, r1=`0x0`, r2=`unknown`, r3=`[sp-20]`.
  Call (`bl` at 0x6fe5c): (unknown callee), target 0x28230, r0=`0x50`, r1=`unknown`, r2=`unknown`, r3=`[sp-20]`.
  Branch (`bne` at 0x6fe68): taken to 0x6ff40 when `ret(0x28230) != 0x0`, falls through to 0x6fe6c when `ret(0x28230) == 0x0` (source `bl` at 0x6fe5c).
  Call (`bl` at 0x6ff2c): (unknown callee), target 0x28230, r0=`0x3e`, r1=`unknown`, r2=`unknown`, r3=`[sp-20]`.
  Branch (`bne` at 0x6ff38): taken to 0x6ff40 when `ret(0x28230) != 0x0`, falls through to 0x6ff3c when `ret(0x28230) == 0x0` (source `bl` at 0x6ff2c).
  3 such byte load accesses with resolved target `abs 0x318308`, including target `abs 0x318308` (`ldrb` at 0x6ff44), target `abs 0x318308` (`ldrb` at 0x6ff5c), target `abs 0x318308` (`ldrb` at 0x6ff80).
  Call (`bl` at 0x6ff94): (unknown callee), target 0x2624c, r0=`0xd`, r1=`sp-172`, r2=`[0x165018]`, r3=`sp-172`.
  Call (`bl` at 0x6ff9c): (unknown callee), target 0x293f8, r0=`0x16501c`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x70040): (unknown callee), target 0x205ec, r0=`unknown`, r1=`unknown`, r2=`0x1`, r3=`[[sp-32]+1]:1`.
  6 such word load accesses with resolved target `ret(0x205ec) +0x0`, including target `ret(0x205ec) +0x0` (`ldr` at 0x70060, `bl` at 0x70040), target `ret(0x205ec) +0x0` (`ldr` at 0x70098, `bl` at 0x70078), target `ret(0x205ec) +0x0` (`ldr` at 0x700d0, `bl` at 0x700b0).
  Call (`bl` at 0x70078): (unknown callee), target 0x205ec, r0=`ret(0x205ec)`, r1=`ret(0x205ec)`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x700b0): (unknown callee), target 0x205ec, r0=`ret(0x205ec)`, r1=`ret(0x205ec)`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x700e8): (unknown callee), target 0x205ec, r0=`ret(0x205ec)`, r1=`ret(0x205ec)`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x70120): (unknown callee), target 0x205ec, r0=`ret(0x205ec)`, r1=`ret(0x205ec)`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x70158): (unknown callee), target 0x205ec, r0=`ret(0x205ec)`, r1=`ret(0x205ec)`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x70198): (unknown callee), target 0x2624c, r0=`0xd`, r1=`0x165038`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x701a0): (unknown callee), target 0x293f8, r0=`0x16501c`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x701bc): (unknown callee), target 0x20340, r0=`[sp-36]`, r1=`[sp-40]`, r2=`0x6`, r3=`[sp-40]`.
  Branch (`beq` at 0x701c8): taken to 0x701ec when `ret(0x20340) == 0x0`, falls through to 0x701cc when `ret(0x20340) != 0x0` (source `bl` at 0x701bc).
  Call (`bl` at 0x701e0): (unknown callee), target 0x2624c, r0=`0xf`, r1=`[sp-40]`, r2=`unknown`, r3=`[sp-28]`.
  Call (`bl` at 0x701e8): (unknown callee), target 0x293f8, r0=`0x165044`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x701f0).

## `KeyLogInsert`

- `KeyLogInsert`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6f7c0 to 0x6f9c4.
  Established facts:
  The instructions from 0x6f9c0 to 0x6f9c4 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x6f7dc): (unknown callee), target 0x28230, r0=`0x51`, r1=`r1`, r2=`r2`, r3=`sp-28`.
  Call (`bl` at 0x6f7ec): (unknown callee), target 0x28230, r0=`0x52`, r1=`unknown`, r2=`unknown`, r3=`ret(0x28230)`.
  Call (`bl` at 0x6f7f8): (unknown callee), target 0x28230, r0=`0x53`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Byte store: target `[sp-36] +0x0`, value `unknown` (`strb` at 0x6f858).
  Byte store: target `[sp-36] +0x1`, value `unknown` (`strb` at 0x6f880).
  Byte store: target `[sp-36] +0x2`, value `unknown` (`strb` at 0x6f8ac).
  Byte store: target `[sp-36] +0x3`, value `unknown` (`strb` at 0x6f8d4).
  Byte store: target `[sp-36] +0x4`, value `unknown` (`strb` at 0x6f900).
  Byte store: target `[sp-36] +0x5`, value `unknown` (`strb` at 0x6f92c).
  Byte store: target `[sp-36] +0x6`, value `unknown` (`strb` at 0x6f954).
  Call (`bl` at 0x6f9a0): (unknown callee), target 0x2624c, r0=`0x51`, r1=`[sp-29]:1`, r2=`unknown`, r3=`[sp-29]:1`.
  Call (`bl` at 0x6f9ac): (unknown callee), target 0x2624c, r0=`0x52`, r1=`[sp-40]`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `0x0` (`ldm` at 0x6f9bc).

## `KeyInLog`

- `KeyInLog`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6f9c4 to 0x6fb9c.
  Established facts:
  Call (`bl` at 0x6f9e0): (unknown callee), target 0x28230, r0=`0x52`, r1=`r1`, r2=`r2`, r3=`sp-28`.
  Call (`bl` at 0x6f9ec): (unknown callee), target 0x28230, r0=`0x53`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`bgt` at 0x6fa04): taken to 0x6fb88 when `gt flags condition (operands unknown)`, falls through to 0x6fa08 when `not (gt flags condition (operands unknown))`.
  Byte load: target `([sp-44] + [sp-36]) +0x0` (`ldrb` at 0x6fa28).
  Byte load: target `([sp-44] + [sp-36]) +0x1` (`ldrb` at 0x6fa4c).
  Branch (`bne` at 0x6fa68): taken to 0x6fb78 when `[sp-28] != [sp-48]`, falls through to 0x6fa6c when `[sp-28] == [sp-48]`.
  Byte load: target `([sp-44] + [sp-36]) +0x2` (`ldrb` at 0x6fa80).
  Byte load: target `([sp-44] + [sp-36]) +0x3` (`ldrb` at 0x6faa4).
  Branch (`bne` at 0x6fac0): taken to 0x6fb78 when `[sp-24] != [sp-52]`, falls through to 0x6fac4 when `[sp-24] == [sp-52]`.
  Byte load: target `([sp-44] + [sp-36]) +0x4` (`ldrb` at 0x6fad8).
  Byte load: target `([sp-44] + [sp-36]) +0x5` (`ldrb` at 0x6fafc).
  Byte load: target `([sp-44] + [sp-36]) +0x6` (`ldrb` at 0x6fb28).
  Branch (`bne` at 0x6fb44): taken to 0x6fb78 when `[sp-20] != [sp-56]`, falls through to 0x6fb48 when `[sp-20] == [sp-56]`.
  Branch (`beq` at 0x6fb5c): taken to 0x6fb6c when `unknown == 0x0`, falls through to 0x6fb60 when `unknown != 0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x6fb98).

## `SerNumLearn`

- `SerNumLearn`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xee1bc to 0xee750.
  Established facts:
  The instructions from 0xee334 to 0xee598 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0xee714 to 0xee750 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0xee1d8): (unknown callee), target 0x28230, r0=`0xd`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0xee1e0): (unknown callee), target 0x6dc0c, r0=`ret(0x28230)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xee1f4): (unknown callee), target 0x28230, r0=`0x1`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xee200): (unknown callee), target 0x28230, r0=`0x2`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0xee210): taken to 0xee260 when `[sp-140] == 0x52`, falls through to 0xee214 when `[sp-140] != 0x52`.
  2 such word load accesses with resolved target `abs 0x1a6110`, including target `abs 0x1a6110` (`ldr` at 0xee264), target `abs 0x1a6110` (`ldr` at 0xee280).
  Branch (`bne` at 0xee26c): taken to 0xee27c when `[0x1a6110] != 0x2`, falls through to 0xee270 when `[0x1a6110] == 0x2`.
  Branch (`bne` at 0xee288): taken to 0xee2a8 when `[0x1a6110] != 0x1`, falls through to 0xee28c when `[0x1a6110] == 0x1`.
  Branch (`bgt` at 0xee298): taken to 0xee2a8 when `[sp-24] > 0x3fff (signed, 32-bit)`, falls through to 0xee29c when `[sp-24] <= 0x3fff (signed, 32-bit)`.
  Word load: target `abs 0x1b0f54` (`ldr` at 0xee2b4).
  Branch (`blt` at 0xee2c0): taken to 0xee318 when `unknown < unknown (signed, 32-bit)`, falls through to 0xee2c4 when `unknown >= unknown (signed, 32-bit)`.
  Call (`bl` at 0xee2d4): (unknown callee), target 0x200ac, r0=`[sp-20]+1`, r1=`0x184d2c`, r2=`unknown`, r3=`[sp-20]+1`.
  Branch (`bne` at 0xee2e0): taken to 0xee2fc when `ret(0x200ac) != 0x0`, falls through to 0xee2e4 when `ret(0x200ac) == 0x0` (source `bl` at 0xee2d4).
  2 such word store accesses with resolved target `abs 0x1b0f54`, including target `abs 0x1b0f54`, value `0x0` (`str` at 0xee2ec), target `abs 0x1b0f54`, value `[sp-24]` (`str` at 0xee310).
  Call (`bl` at 0xee304): (unknown callee), target 0x20634, r0=`0x31b96c`, r1=`[sp-20]`, r2=`unknown`, r3=`ret(0x200ac)`.
  Branch (`ldrls` at 0xee32c): taken to (unknown target) when `[sp-140]-36 <= 0x59 (unsigned, 32-bit)`, falls through to 0xee330 when `[sp-140]-36 > 0x59 (unsigned, 32-bit)`.
  Omitted-path effects remain unknown at the unresolved transfer (`ldrls` at 0xee32c).
  Call (`bl` at 0xee5a0): (unknown callee), target 0x200ac, r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`[sp-140]-36`.
  Branch (`beq` at 0xee5ac): taken to 0xee700 when `ret(0x200ac) == 0x0`, falls through to 0xee5b0 when `ret(0x200ac) != 0x0` (source `bl` at 0xee5a0).
  Call (`bl` at 0xee5b8): (unknown callee), target 0x200ac, r0=`[sp-28]`, r1=`0x184de4`, r2=`unknown`, r3=`ret(0x200ac)`.
  Branch (`beq` at 0xee5c4): taken to 0xee700 when `ret(0x200ac) == 0x0`, falls through to 0xee5c8 when `ret(0x200ac) != 0x0` (source `bl` at 0xee5b8).
  Byte load: target `[sp-28] +0x0` (`ldrb` at 0xee5cc).
  Branch (`beq` at 0xee5d4): taken to 0xee700 when `[[sp-28]]:1 == 0x0`, falls through to 0xee5d8 when `[[sp-28]]:1 != 0x0`.
  Call (`bl` at 0xee5dc): (unknown callee), target 0xee750, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`[[sp-28]]:1`.
  Branch (`beq` at 0xee5e8): taken to 0xee6f0 when `ret(0xee750) == 0x0`, falls through to 0xee5ec when `ret(0xee750) != 0x0` (source `bl` at 0xee5dc).
  Call (`bl` at 0xee5f4): (unknown callee), target 0x2624c, r0=`0xd`, r1=`[sp-20]`, r2=`unknown`, r3=`ret(0xee750)`.
  Call (`bl` at 0xee60c): (unknown callee), target 0x205bc, r0=`sp-128`, r1=`0x184df0`, r2=`[sp-24]`, r3=`[sp-20]`.
  Branch (`beq` at 0xee624): taken to 0xee688 when `[sp-156] == 0x2000`, falls through to 0xee628 when `[sp-156] != 0x2000`.
  Branch (`bgt` at 0xee630): taken to 0xee644 when `[sp-156] > 0x2000 (signed, 32-bit)`, falls through to 0xee634 when `[sp-156] <= 0x2000 (signed, 32-bit)`.
  Branch (`beq` at 0xee63c): taken to 0xee69c when `[sp-156] == 0x1000`, falls through to 0xee640 when `[sp-156] != 0x1000`.
  Branch (`beq` at 0xee64c): taken to 0xee674 when `[sp-156] == 0x4000`, falls through to 0xee650 when `[sp-156] != 0x4000`.
  Branch (`beq` at 0xee658): taken to 0xee660 when `[sp-156] == 0x8000`, falls through to 0xee65c when `[sp-156] != 0x8000`.
  Call (`bl` at 0xee66c): (unknown callee), target 0x203c4, r0=`sp-128`, r1=`0x184e30`, r2=`unknown`, r3=`sp-128`.
  Call (`bl` at 0xee680): (unknown callee), target 0x203c4, r0=`sp-128`, r1=`0x184e38`, r2=`unknown`, r3=`sp-128`.
  Call (`bl` at 0xee694): (unknown callee), target 0x203c4, r0=`sp-128`, r1=`0x184e44`, r2=`unknown`, r3=`sp-128`.
  Call (`bl` at 0xee6a8): (unknown callee), target 0x203c4, r0=`sp-128`, r1=`0x184e50`, r2=`unknown`, r3=`sp-128`.
  Call (`bl` at 0xee6b4): (unknown callee), target 0x293f8, r0=`sp-128`, r1=`unknown`, r2=`unknown`, r3=`sp-128`.
  Call (`bl` at 0xee6c4): (unknown callee), target 0x243d4, r0=`0x184e5c`, r1=`sp-128`, r2=`unknown`, r3=`sp-128`.
  Call (`bl` at 0xee6e0): (unknown callee), target 0xa485c, r0=`unknown`, r1=`unknown`, r2=`[sp-24]`, r3=`unknown`.
  Call (`bl` at 0xee6f4): (unknown callee), target 0x293f8, r0=`0x184e60`, r1=`unknown`, r2=`unknown`, r3=`ret(0xee750)`.
  Call (`bl` at 0xee6fc): (unknown callee), target 0x293f8, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Known-edge recorded r0 = `unknown` (`ldm` at 0xee710).

## `SerNumValid`

- `SerNumValid`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xee750 to 0xee9d4.
  Established facts:
  The instructions from 0xee9c8 to 0xee9d4 are not reached by any path from the entry, and their branches are not listed.
  2 such byte load accesses with resolved target `abs 0x318308`, including target `abs 0x318308` (`ldrb` at 0xee768), target `abs 0x318308` (`ldrb` at 0xee7a4).
  Branch (`bne` at 0xee770): taken to 0xee780 when `[0x318308]:1 != 0x0`, falls through to 0xee774 when `[0x318308]:1 == 0x0`.
  Call (`bl` at 0xee784): (unknown callee), target 0x2031c, r0=`r0`, r1=`r1`, r2=`r2`, r3=`[0x318308]:1`.
  Branch (`beq` at 0xee790): taken to 0xee7a0 when `ret(0x2031c) == 0x8`, falls through to 0xee794 when `ret(0x2031c) != 0x8` (source `bl` at 0xee784).
  Byte load: target `[sp-20] +0x0` (`ldrb` at 0xee7b4).
  Call (`bl` at 0xee7c0): (unknown callee), target 0x20040, r0=`sp-28`, r1=`[[sp-20]]:1`, r2=`sp-28`, r3=`[[sp-20]]:1`.
  Branch (`bne` at 0xee7cc): taken to 0xee7dc when `ret(0x20040) != 0x0`, falls through to 0xee7d0 when `ret(0x20040) == 0x0` (source `bl` at 0xee7c0).
  Call (`bl` at 0xee7ec): (unknown callee), target 0x200ac, r0=`[sp-20]+1`, r1=`0x184e88`, r2=`unknown`, r3=`[sp-20]+1`.
  Branch (`beq` at 0xee7f8): taken to 0xee81c when `ret(0x200ac) == 0x0`, falls through to 0xee7fc when `ret(0x200ac) != 0x0` (source `bl` at 0xee7ec).
  Call (`bl` at 0xee80c): (unknown callee), target 0x200ac, r0=`[sp-20]+1`, r1=`0x184d34`, r2=`unknown`, r3=`[sp-20]+1`.
  Branch (`bne` at 0xee818): taken to 0xee828 when `ret(0x200ac) != 0x0`, falls through to 0xee81c when `ret(0x200ac) == 0x0` (source `bl` at 0xee80c).
  2 such byte load accesses with resolved target `[sp-20] +0x1`, including target `[sp-20] +0x1` (`ldrb` at 0xee834), target `[sp-20] +0x1` (`ldrb` at 0xee84c).
  Branch (`bls` at 0xee83c): taken to 0xee9a8 when `[[sp-20]+1]:1 <= 0x2f (unsigned, 32-bit)`, falls through to 0xee840 when `[[sp-20]+1]:1 > 0x2f (unsigned, 32-bit)`.
  Branch (`bhi` at 0xee854): taken to 0xee9a8 when `[[sp-20]+1]:1 > 0x33 (unsigned, 32-bit)`, falls through to 0xee858 when `[[sp-20]+1]:1 <= 0x33 (unsigned, 32-bit)`.
  Call (`bl` at 0xee858): (unknown callee), target 0x205ec, r0=`ret(0x200ac)`, r1=`unknown`, r2=`0x1`, r3=`[[sp-20]+1]:1`.
  Byte load: target `[sp-20] +0x2` (`ldrb` at 0xee870).
  6 such word load accesses with resolved target `ret(0x205ec) +0x0`, including target `ret(0x205ec) +0x0` (`ldr` at 0xee878, `bl` at 0xee858), target `ret(0x205ec) +0x0` (`ldr` at 0xee8b0, `bl` at 0xee890), target `ret(0x205ec) +0x0` (`ldr` at 0xee8e8, `bl` at 0xee8c8).
  Halfword load: target `(unknown effective address)` (`ldrh` at 0xee880).
  Branch (`beq` at 0xee88c): taken to 0xee9a8 when `unknown == 0x0`, falls through to 0xee890 when `unknown != 0x0`.
  Call (`bl` at 0xee890): (unknown callee), target 0x205ec, r0=`ret(0x205ec)`, r1=`ret(0x205ec)`, r2=`unknown`, r3=`unknown`.
  Byte load: target `[sp-20] +0x3` (`ldrb` at 0xee8a8).
  Halfword load: target `(unknown effective address)` (`ldrh` at 0xee8b8).
  Branch (`beq` at 0xee8c4): taken to 0xee9a8 when `unknown == 0x0`, falls through to 0xee8c8 when `unknown != 0x0`.
  Call (`bl` at 0xee8c8): (unknown callee), target 0x205ec, r0=`ret(0x205ec)`, r1=`ret(0x205ec)`, r2=`unknown`, r3=`unknown`.
  Byte load: target `[sp-20] +0x4` (`ldrb` at 0xee8e0).
  Halfword load: target `(unknown effective address)` (`ldrh` at 0xee8f0).
  Branch (`beq` at 0xee8fc): taken to 0xee9a8 when `unknown == 0x0`, falls through to 0xee900 when `unknown != 0x0`.
  Call (`bl` at 0xee900): (unknown callee), target 0x205ec, r0=`ret(0x205ec)`, r1=`ret(0x205ec)`, r2=`unknown`, r3=`unknown`.
  Byte load: target `[sp-20] +0x5` (`ldrb` at 0xee918).
  Halfword load: target `(unknown effective address)` (`ldrh` at 0xee928).
  Branch (`beq` at 0xee934): taken to 0xee9a8 when `unknown == 0x0`, falls through to 0xee938 when `unknown != 0x0`.
  Call (`bl` at 0xee938): (unknown callee), target 0x205ec, r0=`ret(0x205ec)`, r1=`ret(0x205ec)`, r2=`unknown`, r3=`unknown`.
  Byte load: target `[sp-20] +0x6` (`ldrb` at 0xee950).
  Halfword load: target `(unknown effective address)` (`ldrh` at 0xee960).
  Branch (`beq` at 0xee96c): taken to 0xee9a8 when `unknown == 0x0`, falls through to 0xee970 when `unknown != 0x0`.
  Call (`bl` at 0xee970): (unknown callee), target 0x205ec, r0=`ret(0x205ec)`, r1=`ret(0x205ec)`, r2=`unknown`, r3=`unknown`.
  Byte load: target `[sp-20] +0x7` (`ldrb` at 0xee988).
  Halfword load: target `(unknown effective address)` (`ldrh` at 0xee998).
  Branch (`bne` at 0xee9a4): taken to 0xee9b4 when `unknown != 0x0`, falls through to 0xee9a8 when `unknown == 0x0`.
  Recorded r0 = `unknown` (`ldm` at 0xee9c4).

## `L1LogInit`

- `L1LogInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf679c to 0xf684c.
  Established facts:
  The instructions from 0xf6830 to 0xf684c are not reached by any path from the entry, and their branches are not listed.
  2 such word load accesses with resolved target `abs 0x1b0fd8`, including target `abs 0x1b0fd8` (`ldr` at 0xf67b0), target `abs 0x1b0fd8` (`ldr` at 0xf67e0).
  Branch (`beq` at 0xf67b8): taken to 0xf67d8 when `[0x1b0fd8] == 0x0`, falls through to 0xf67bc when `[0x1b0fd8] != 0x0`.
  Call (`bl` at 0xf67d4): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x18762c`, r2=`0x32`, r3=`0x0`.
  Word store: target `abs 0x1b0fd8`, value `[0x1b0fd8]+1` (`str` at 0xf67e8).
  Call (`bl` at 0xf6824): (unknown callee), target 0x22540, r0=`0x1b0fd4`, r1=`0x187634`, r2=`0xf684c`, r3=`0x0`.
  Recorded r0 = `ret(0x22540)` (`ldm` at 0xf682c, `bl` at 0xf6824).
