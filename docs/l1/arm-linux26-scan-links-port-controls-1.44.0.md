# ARM Linux 2.6 scan, link and port control routines in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-scan-links-port-controls-facts.txt) retain all tool sections.

The notation `ret(ADDR)` identifies a call-result expression without distinguishing
individual call sites to that target. Each return line records machine-register
state from the analysis rather than a C return type.

## `promIceInit`

- `promIceInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x61078 to 0x61148.
  Established facts:
  The instructions from 0x61130 to 0x61148 are not reached by any path from the entry, and their branches are not listed.
  Word load: target `abs 0x31a60c` (`ldr` at 0x61094).
  Branch (`beq` at 0x6109c): taken to 0x610bc when `[0x31a60c] == 0x0`, falls through to 0x610a0 when `[0x31a60c] != 0x0`.
  Call (`bl` at 0x610b8): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x162ee0`, r2=`0x39`, r3=`0x0`.
  Call (`bl` at 0x610c8): (unknown callee), target 0x204d8, r0=`unknown`, r1=`0x1`, r2=`unknown`, r3=`unknown`.
  Word store: target `abs 0x31a610`, value `ret(0x204d8)` (`str` at 0x610d0, `bl` at 0x610c8).
  Branch (`bne` at 0x610dc): taken to 0x610fc when `[0x31a610] != 0x0`, falls through to 0x610e0 when `[0x31a610] == 0x0`.
  Call (`bl` at 0x610f8): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x162ee0`, r2=`0x40`, r3=`0x0`.
  Call (`bl` at 0x6110c): (unknown callee), target 0x20550, r0=`[0x31a610]`, r1=`0x1`, r2=`[sp-28]`, r3=`0x31a610`.
  Word store: target `abs 0x31a60c`, value `[sp-24]` (`str` at 0x61118).
  Word store: target `abs 0x31a608`, value `[sp-28]` (`str` at 0x61124).
  Recorded r0 = `ret(0x20550)` (`ldm` at 0x6112c, `bl` at 0x6110c).

## `promIceEnable`

- `promIceEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x614a0 to 0x61574.
  Established facts:
  The instructions from 0x61570 to 0x61574 are not reached by any path from the entry, and their branches are not listed.
  Branch (`bne` at 0x614c8): taken to 0x61550 when `(r0 & 0xff) != 0xff`, falls through to 0x614cc when `(r0 & 0xff) == 0xff`.
  Call (`bl` at 0x614f0): (unknown callee), target 0x61324, r0=`unknown`, r1=`unknown`, r2=`[0x31a608]`, r3=`unknown`.
  Call (`bl` at 0x61508): (unknown callee), target 0x61648, r0=`[sp-29]:1`, r1=`unknown`, r2=`unknown`, r3=`[sp-29]:1`.
  Call (`bl` at 0x61558): (unknown callee), target 0x61648, r0=`(r0 & 0xff)`, r1=`r1`, r2=`r2`, r3=`(r0 & 0xff)`.
  Recorded r0 = `unknown` (`ldm` at 0x6156c).

## `promIceDisable`

- `promIceDisable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x61574 to 0x61648.
  Established facts:
  The instructions from 0x61644 to 0x61648 are not reached by any path from the entry, and their branches are not listed.
  Branch (`bne` at 0x6159c): taken to 0x61624 when `(r0 & 0xff) != 0xff`, falls through to 0x615a0 when `(r0 & 0xff) == 0xff`.
  Call (`bl` at 0x615c4): (unknown callee), target 0x61324, r0=`unknown`, r1=`unknown`, r2=`[0x31a608]`, r3=`unknown`.
  Call (`bl` at 0x615dc): (unknown callee), target 0x616fc, r0=`[sp-29]:1`, r1=`unknown`, r2=`unknown`, r3=`[sp-29]:1`.
  Call (`bl` at 0x6162c): (unknown callee), target 0x616fc, r0=`(r0 & 0xff)`, r1=`r1`, r2=`r2`, r3=`(r0 & 0xff)`.
  Recorded r0 = `unknown` (`ldm` at 0x61640).

## `NL4R_SetLinkEnable`

- `NL4R_SetLinkEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2e1e4 to 0x2e624.
  Established facts:
  The instructions from 0x2e608 to 0x2e610 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x2e614 to 0x2e624 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x2e22c): (unknown callee), target 0x6e140, r0=`r0`, r1=`r1`, r2=`r2`, r3=`unknown`.
  Call (`bl` at 0x2e23c): (unknown callee), target 0x3e38c, r0=`ret(0x6e140)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x6e140)`.
  Call (`bl` at 0x2e250): (unknown callee), target 0x6e190, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2e28c): (unknown callee), target 0x6e140, r0=`unknown`, r1=`unknown`, r2=`[0x1a599c]`, r3=`unknown`.
  Call (`bl` at 0x2e29c): (unknown callee), target 0x3e38c, r0=`ret(0x6e140)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x6e140)`.
  Call (`bl` at 0x2e2b0): (unknown callee), target 0x6e190, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2e334): (unknown callee), target 0x6e140, r0=`unknown`, r1=`unknown`, r2=`[sp-64]`, r3=`[[sp-64]+8]`.
  Call (`bl` at 0x2e344): (unknown callee), target 0x3e38c, r0=`ret(0x6e140)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x6e140)`.
  Call (`bl` at 0x2e358): (unknown callee), target 0x6e190, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2e434): (unknown callee), target 0x243d4, r0=`0x155fe8`, r1=`[sp-20]`, r2=`[sp-24]`, r3=`[sp-28]`.
  Call (`bl` at 0x2e45c): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]+10`, r2=`0x1`, r3=`sp-32`.
  Call (`bl` at 0x2e484): (unknown callee), target 0x243d4, r0=`0x15601c`, r1=`[sp-24]+65`, r2=`[sp-28]`, r3=`[sp-24]+65`.
  Call (`bl` at 0x2e488): (unknown callee), target 0x134634, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2e4a4): (unknown callee), target 0x13fd34, r0=`[sp-20]`, r1=`0x3`, r2=`[sp-24]+10`, r3=`0x0`.
  Call (`bl` at 0x2e4b0): (unknown callee), target 0x1346fc, r0=`ret(0x13fd34)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13fd34)`.
  Call (`bl` at 0x2e508): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]+18`, r2=`0x0`, r3=`sp-32`.
  Call (`bl` at 0x2e530): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]+18`, r2=`0x2`, r3=`sp-32`.
  Call (`bl` at 0x2e558): (unknown callee), target 0x243d4, r0=`0x156050`, r1=`[sp-24]+65`, r2=`[sp-28]`, r3=`[sp-24]+65`.
  Call (`bl` at 0x2e55c): (unknown callee), target 0x134634, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2e578): (unknown callee), target 0x13fd34, r0=`[sp-20]`, r1=`0x3`, r2=`[sp-24]+18`, r3=`0x0`.
  Call (`bl` at 0x2e584): (unknown callee), target 0x1346fc, r0=`ret(0x13fd34)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13fd34)`.
  Call (`bl` at 0x2e5c4): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]+18`, r2=`0x0`, r3=`sp-32`.
  Call (`bl` at 0x2e5c8): (unknown callee), target 0x134634, r0=`ret(0x13f048)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2e5e4): (unknown callee), target 0x13fd34, r0=`[sp-20]`, r1=`0x3`, r2=`[sp-24]+18`, r3=`0x0`.
  Call (`bl` at 0x2e5f0): (unknown callee), target 0x1346fc, r0=`ret(0x13fd34)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13fd34)`.
  Recorded r0 = `unknown` (`ldm` at 0x2e604).

## `lbi_tapmux_init`

- `lbi_tapmux_init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x149840 to 0x14991c.
  Established facts:
  The instructions from 0x14990c to 0x14991c are not reached by any path from the entry, and their branches are not listed.
  Branch (`bne` at 0x149878): taken to 0x1498e0 when `[0x31527c] != 0x0`, falls through to 0x14987c when `[0x31527c] == 0x0`.
  Call (`bl` at 0x1498a4): (unknown callee), target 0x204d8, r0=`[sp-36]`, r1=`0x1`, r2=`0x315280`, r3=`unknown`.
  Call (`bl` at 0x1498dc): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x1953c4`, r2=`0x57`, r3=`0x0`.
  Recorded r0 = `0x0` (`ldm` at 0x149908).

## `lbi_tapmux_select`

- `lbi_tapmux_select`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x14991c to 0x149b04.
  Established facts:
  The instructions from 0x149b00 to 0x149b04 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x1499b4): (unknown callee), target 0x10ca8c, r0=`[sp-32]`, r1=`r1`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x1499cc): (unknown callee), target 0x10d17c, r0=`[sp-32]`, r1=`0x0`, r2=`unknown`, r3=`ret(0x10ca8c)`.
  Call (`bl` at 0x1499d0): (unknown callee), target 0x6e140, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x1499e0): (unknown callee), target 0x3e38c, r0=`ret(0x6e140)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x6e140)`.
  Call (`bl` at 0x1499f4): (unknown callee), target 0x6e190, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x149a2c): (unknown callee), target 0x125c20, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x149a3c): (unknown callee), target 0x6e140, r0=`ret(0x125c20)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x149a4c): (unknown callee), target 0x3e38c, r0=`ret(0x6e140)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x6e140)`.
  Call (`bl` at 0x149a60): (unknown callee), target 0x6e190, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x149aa4): (unknown callee), target 0x125a58, r0=`unknown`, r1=`unknown`, r2=`0x70`, r3=`[sp-33]:1`.
  Call (`bl` at 0x149ad0): (unknown callee), target 0x10ca8c, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x149ae8): (unknown callee), target 0x10d17c, r0=`[sp-32]`, r1=`0x1`, r2=`unknown`, r3=`ret(0x10ca8c)`.
  Recorded r0 = `unknown` (`ldm` at 0x149afc).

## `build_ir_access_chip`

- `build_ir_access_chip`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13e4f0 to 0x13e5b0.
  Established facts:
  The instructions from 0x13e5ac to 0x13e5b0 are not reached by any path from the entry, and their branches are not listed.
  Branch (`bhs` at 0x13e550): taken to 0x13e588 when `[0x31509c] >= [sp-36] (unsigned, 32-bit)`, falls through to 0x13e554 when `[0x31509c] < [sp-36] (unsigned, 32-bit)`.
  Call (`bl` at 0x13e5a0): (unknown callee), target 0x132444, r0=`0x0`, r1=`[sp-32]`, r2=`[sp-24]`, r3=`[sp]`.
  Recorded r0 = `ret(0x132444)` (`ldm` at 0x13e5a8, `bl` at 0x13e5a0).

## `build_dr_access_chip`

- `build_dr_access_chip`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13e778 to 0x13e868.
  Established facts:
  The instructions from 0x13e864 to 0x13e868 are not reached by any path from the entry, and their branches are not listed.
  Word store: target `[sp] +0x0`, value `(r0 + r2)-1` (`str` at 0x13e7ac).
  Branch (`beq` at 0x13e7bc): taken to 0x13e85c when `[sp-32] == [sp+4]`, falls through to 0x13e7c0 when `[sp-32] != [sp+4]`.
  Call (`bl` at 0x13e858): (unknown callee), target 0x132444, r0=`0x0`, r1=`[sp-32]`, r2=`[sp-24]`, r3=`[sp+4]`.
  Recorded r0 = `unknown` (`ldm` at 0x13e860).

## `slit_test`

- `slit_test`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x149640 to 0x149840.
  Established facts:
  The instructions from 0x149824 to 0x149840 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x149670): (unknown callee), target 0x134634, r0=`r0`, r1=`r1`, r2=`r2`, r3=`0x0`.
  Call (`bl` at 0x149694): (unknown callee), target 0x14dce0, r0=`[sp-20]`, r1=`[sp-28]`, r2=`[sp-32]`, r3=`[sp+4]`.
  Call (`bl` at 0x1496a0): (unknown callee), target 0x1346fc, r0=`ret(0x14dce0)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x14dce0)`.
  Call (`bl` at 0x1496d0): (unknown callee), target 0x205bc, r0=`[sp+12]`, r1=`[sp-52]`, r2=`0x195364`, r3=`[sp-24]`.
  Branch (`bne` at 0x1496e0): taken to 0x14970c when `[sp-40] != 0x0`, falls through to 0x1496e4 when `[sp-40] == 0x0`.
  Call (`bl` at 0x149704): (unknown callee), target 0x205bc, r0=`[sp+12]`, r1=`[sp-52]`, r2=`0x19536c`, r3=`[sp-24]`.
  Call (`bl` at 0x14973c): (unknown callee), target 0x205bc, r0=`[sp+12]`, r1=`[sp-52]`, r2=`0x195374`, r3=`[sp-24]`.
  Call (`bl` at 0x14974c): (unknown callee), target 0x134634, r0=`ret(0x1346fc)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x149770): (unknown callee), target 0x14dce0, r0=`[sp-20]`, r1=`[sp+4]`, r2=`[sp+8]`, r3=`[sp-28]`.
  Call (`bl` at 0x14977c): (unknown callee), target 0x1346fc, r0=`ret(0x14dce0)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x14dce0)`.
  Call (`bl` at 0x1497ac): (unknown callee), target 0x205bc, r0=`[sp+12]`, r1=`[sp-52]`, r2=`0x195364`, r3=`[sp-24]`.
  Call (`bl` at 0x1497e0): (unknown callee), target 0x205bc, r0=`[sp+12]`, r1=`[sp-52]`, r2=`0x19536c`, r3=`[sp-24]`.
  Call (`bl` at 0x149808): (unknown callee), target 0x205bc, r0=`[sp+12]`, r1=`[sp-52]`, r2=`0x195374`, r3=`[sp-24]`.
  Recorded r0 = `[sp-44]` (`ldm` at 0x149820).

## `slit_test_link`

- `slit_test_link`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x14dce0 to 0x14ddd0.
  Established facts:
  Call (`bl` at 0x14dd6c): (unknown callee), target 0x14ddd0, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Word load: target `[sp+4] +0x0` (`ldr` at 0x14dd94).
  Branch (`bne` at 0x14dd9c): taken to 0x14ddac when `[[sp+4]] != 0x0`, falls through to 0x14dda0 when `[[sp+4]] == 0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x14ddcc).

## `slit_apply_pattern`

- `slit_apply_pattern`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x14ddd0 to 0x14e1ec.
  Established facts:
  The instructions from 0x14e1e0 to 0x14e1ec are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x14de5c): (unknown callee), target 0x14f690, r0=`sp-76`, r1=`0x1a2590`, r2=`[sp+4]`, r3=`sp-76`.
  Call (`bl` at 0x14de7c): (unknown callee), target 0x14f6c4, r0=`0x1`, r1=`[sp-80]`, r2=`unknown`, r3=`0x0`.
  Call (`bl` at 0x14deb8): (unknown callee), target 0x14f784, r0=`0x0`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`mov` at 0x14def8): (unknown callee), target (unknown target), r0=`[sp-24]`, r1=`0x0`, r2=`sp-76`, r3=`[0x3162bc]`.
  Call (`bl` at 0x14df24): (unknown callee), target 0x14f690, r0=`sp-76`, r1=`unknown`, r2=`unknown`, r3=`sp-76`.
  Call (`bl` at 0x14df40): (unknown callee), target 0x14f784, r0=`0x1`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x14df80): (unknown callee), target 0x14f6c4, r0=`0x0`, r1=`[sp-80]`, r2=`unknown`, r3=`0x0`.
  Call (`mov` at 0x14dfc0): (unknown callee), target (unknown target), r0=`[sp-32]`, r1=`0x1`, r2=`sp-76`, r3=`[0x3162bc]`.
  Call (`bl` at 0x14e00c): (unknown callee), target 0x14f690, r0=`sp-76`, r1=`unknown`, r2=`unknown`, r3=`sp-76`.
  Call (`bl` at 0x14e02c): (unknown callee), target 0x14f6c4, r0=`0x1`, r1=`[sp-80]`, r2=`unknown`, r3=`0x0`.
  Call (`mov` at 0x14e07c): (unknown callee), target (unknown target), r0=`[sp-24]`, r1=`0x0`, r2=`sp-76`, r3=`[0x3162bc]`.
  Call (`bl` at 0x14e0a8): (unknown callee), target 0x14f690, r0=`sp-76`, r1=`unknown`, r2=`unknown`, r3=`sp-76`.
  Call (`mov` at 0x14e0e8): (unknown callee), target (unknown target), r0=`[sp-32]`, r1=`0x2`, r2=`sp-76`, r3=`[0x3162bc]`.
  Call (`bl` at 0x14e154): (unknown callee), target 0x201fc, r0=`0xc8`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x14e178): (unknown callee), target 0x14e1ec, r0=`[sp+4]`, r1=`sp-76`, r2=`unknown`, r3=`sp-76`.
  Call (`bl` at 0x14e1c0): (unknown callee), target 0x243d4, r0=`0x1959f4`, r1=`[sp-24]`, r2=`[sp-28]`, r3=`[sp-32]`.
  Call (`bl` at 0x14e1c8): (unknown callee), target 0x20514, r0=`[sp-96]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x14e1dc).

## `slit_check_data`

- `slit_check_data`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x14e1ec to 0x14e910.
  Established facts:
  The instructions from 0x14e900 to 0x14e910 are not reached by any path from the entry, and their branches are not listed.
  Branch (`bne` at 0x14e680): taken to 0x14e6c4 when `[sp-76] != 0x0`, falls through to 0x14e684 when `[sp-76] == 0x0`.
  Branch (`bne` at 0x14e68c): taken to 0x14e6c4 when `[sp-72] != 0x0`, falls through to 0x14e690 when `[sp-72] == 0x0`.
  Branch (`bne` at 0x14e698): taken to 0x14e6c4 when `[sp-68] != 0x0`, falls through to 0x14e69c when `[sp-68] == 0x0`.
  Call (`bl` at 0x14e708): (unknown callee), target 0x205bc, r0=`unknown`, r1=`0x195a24`, r2=`0x10`, r3=`unknown`.
  Call (`bl` at 0x14e710): (unknown callee), target 0x2031c, r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x14e76c): (unknown callee), target 0x205bc, r0=`([sp-28] + ret(0x2031c))`, r1=`0x195a60`, r2=`unknown`, r3=`[[sp-24]+40]`.
  Call (`bl` at 0x14e774): (unknown callee), target 0x2031c, r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x14e7a0): (unknown callee), target 0x205bc, r0=`([sp-28] + ret(0x2031c))`, r1=`0x195a60`, r2=`[sp-88]`, r3=`[sp-40]`.
  Call (`bl` at 0x14e7a8): (unknown callee), target 0x2031c, r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x14e7d4): (unknown callee), target 0x205bc, r0=`([sp-28] + ret(0x2031c))`, r1=`0x195a74`, r2=`[sp-60]`, r3=`[sp-64]`.
  Call (`bl` at 0x14e7dc): (unknown callee), target 0x2031c, r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x14e83c): (unknown callee), target 0x205bc, r0=`([sp-28] + ret(0x2031c))`, r1=`0x195a60`, r2=`unknown`, r3=`[[sp-24]+32]`.
  Call (`bl` at 0x14e844): (unknown callee), target 0x2031c, r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x14e894): (unknown callee), target 0x205bc, r0=`([sp-28] + ret(0x2031c))`, r1=`0x195a60`, r2=`unknown`, r3=`[sp-48]`.
  Call (`bl` at 0x14e89c): (unknown callee), target 0x2031c, r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x14e8c8): (unknown callee), target 0x205bc, r0=`([sp-28] + ret(0x2031c))`, r1=`0x195a74`, r2=`[sp-56]`, r3=`[sp-72]`.
  Call (`bl` at 0x14e8d0): (unknown callee), target 0x2031c, r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x14e8fc).

## `slit_enable_nulls`

- `slit_enable_nulls`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x14e910 to 0x14ea70.
  Established facts:
  The instructions from 0x14ea6c to 0x14ea70 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x14e938): (unknown callee), target 0x14f690, r0=`sp-76`, r1=`r1`, r2=`r2`, r3=`sp-76`.
  Call (`bl` at 0x14e958): (unknown callee), target 0x14f6c4, r0=`0x1`, r1=`[sp-28]`, r2=`unknown`, r3=`0x0`.
  Call (`mov` at 0x14e998): (unknown callee), target (unknown target), r0=`[sp-20]`, r1=`0x0`, r2=`sp-76`, r3=`[0x3162bc]`.
  Call (`bl` at 0x14e9c4): (unknown callee), target 0x14f690, r0=`sp-76`, r1=`unknown`, r2=`unknown`, r3=`sp-76`.
  Call (`bl` at 0x14e9e4): (unknown callee), target 0x14f6c4, r0=`0x1`, r1=`[sp-28]`, r2=`unknown`, r3=`0x0`.
  Call (`mov` at 0x14ea34): (unknown callee), target (unknown target), r0=`[sp-20]`, r1=`0x0`, r2=`sp-76`, r3=`[0x3162bc]`.
  Recorded r0 = `unknown` (`ldm` at 0x14ea68).

## `UtilPortMask`

- `UtilPortMask`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x97d18 to 0x98be4.
  Established facts:
  The instructions from 0x97da0 to 0x988f4 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x98ad8 to 0x98be4 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x97d5c): (unknown callee), target 0x203ac, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Byte store: target `(unknown effective address)`, value `(ret(0x203ac) & 0xff)` (`strb` at 0x97d64, `bl` at 0x97d5c).
  Call (`bl` at 0x97d84): (unknown callee), target 0x6dc0c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`0x0`.
  Branch (`ldrls` at 0x97d98): taken to (unknown target) when `? <= 0x14 (unsigned, 32-bit)`, falls through to 0x97d9c when `? > 0x14 (unsigned, 32-bit)`.
  Omitted-path effects remain unknown at the unresolved transfer (`ldrls` at 0x97d98).
  Call (`bl` at 0x988fc): (unknown callee), target 0x20598, r0=`[sp-24]`, r1=`0x16a5fc`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x98924): (unknown callee), target 0x200ac, r0=`ret(0x20598)`, r1=`0x16a5c8`, r2=`unknown`, r3=`0x0`.
  Call (`bl` at 0x98954): (unknown callee), target 0x200ac, r0=`[sp-36]`, r1=`0x16a5cc`, r2=`unknown`, r3=`ret(0x200ac)`.
  Call (`bl` at 0x98984): (unknown callee), target 0x200ac, r0=`[sp-36]`, r1=`0x16a5d0`, r2=`unknown`, r3=`ret(0x200ac)`.
  Call (`bl` at 0x989b4): (unknown callee), target 0x200ac, r0=`[sp-36]`, r1=`0x16a5d4`, r2=`unknown`, r3=`ret(0x200ac)`.
  Call (`bl` at 0x989e4): (unknown callee), target 0x200ac, r0=`[sp-36]`, r1=`0x16a5d8`, r2=`unknown`, r3=`ret(0x200ac)`.
  Call (`bl` at 0x98a14): (unknown callee), target 0x200ac, r0=`[sp-36]`, r1=`0x16a5dc`, r2=`unknown`, r3=`ret(0x200ac)`.
  Call (`bl` at 0x98a44): (unknown callee), target 0x200ac, r0=`[sp-36]`, r1=`0x16a5e0`, r2=`unknown`, r3=`ret(0x200ac)`.
  Call (`bl` at 0x98a74): (unknown callee), target 0x200ac, r0=`[sp-36]`, r1=`0x16a5e4`, r2=`unknown`, r3=`ret(0x200ac)`.
  Call (`bl` at 0x98aa8): (unknown callee), target 0x205bc, r0=`0x1a659e`, r1=`0x16a660`, r2=`[sp-36]`, r3=`ret(0x200ac)`.
  Known-edge recorded r0 = `unknown` (`ldm` at 0x98ad4).

## `UtilPortStr`

- `UtilPortStr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x96990 to 0x97130.
  Established facts:
  The instructions from 0x96a14 to 0x96c14 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x96c34 to 0x96c74 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x96c94 to 0x96d14 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x96d34 to 0x96d74 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x96d94 to 0x96eb4 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x96ed4 to 0x96ff4 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0x97018 to 0x97130 are not reached by any path from the entry, and their branches are not listed.
  Branch (`beq` at 0x969b8): taken to 0x96c80 when `r0 == 0x2e`, falls through to 0x969bc when `r0 != 0x2e`.
  Branch (`ldrls` at 0x96a0c): taken to (unknown target) when `r1-1 <= 0x1f (unsigned, 32-bit)`, falls through to 0x96a10 when `r1-1 > 0x1f (unsigned, 32-bit)`.
  Omitted-path effects remain unknown at the unresolved transfer (`ldrls` at 0x96a0c).
  Branch (`ldrls` at 0x96c2c): taken to (unknown target) when `r1-1 <= 0x3 (unsigned, 32-bit)`, falls through to 0x96c30 when `r1-1 > 0x3 (unsigned, 32-bit)`.
  Omitted-path effects remain unknown at the unresolved transfer (`ldrls` at 0x96c2c).
  Branch (`ldrls` at 0x96c8c): taken to (unknown target) when `r1-1 <= 0x7 (unsigned, 32-bit)`, falls through to 0x96c90 when `r1-1 > 0x7 (unsigned, 32-bit)`.
  Omitted-path effects remain unknown at the unresolved transfer (`ldrls` at 0x96c8c).
  Branch (`ldrls` at 0x96d2c): taken to (unknown target) when `r1-1 <= 0x3 (unsigned, 32-bit)`, falls through to 0x96d30 when `r1-1 > 0x3 (unsigned, 32-bit)`.
  Omitted-path effects remain unknown at the unresolved transfer (`ldrls` at 0x96d2c).
  Branch (`ldrls` at 0x96d8c): taken to (unknown target) when `r1-1 <= 0x23 (unsigned, 32-bit)`, falls through to 0x96d90 when `r1-1 > 0x23 (unsigned, 32-bit)`.
  Omitted-path effects remain unknown at the unresolved transfer (`ldrls` at 0x96d8c).
  Branch (`ldrls` at 0x96ecc): taken to (unknown target) when `r1-1 <= 0x23 (unsigned, 32-bit)`, falls through to 0x96ed0 when `r1-1 > 0x23 (unsigned, 32-bit)`.
  Omitted-path effects remain unknown at the unresolved transfer (`ldrls` at 0x96ecc).
  Call (`bl` at 0x97000): (unknown callee), target 0x29908, r0=`0x16a5e8`, r1=`r0`, r2=`r1`, r3=`r1-1`.
  Known-edge recorded r0 = `0x16a554` (`ldm` at 0x97014).
