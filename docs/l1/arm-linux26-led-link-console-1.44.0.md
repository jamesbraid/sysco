# ARM Linux 2.6 LED, link and console routines in 1.44.0

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
No unresolved computed transfers were reported for these routines.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-led-link-console-facts.txt) retain all tool sections.

The notation `ret(ADDR)` identifies a call-result expression without distinguishing
individual call sites to that target. Each return line records machine-register
state from the analysis rather than a C return type.


Groups summarize static access records by printed shape. Instruction-address order can include different control-flow paths.

## `LEDsQueryAll`

- `LEDsQueryAll`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf6770 to 0xf679c.
  Established facts:
  Call (`bl` at 0xf678c): (unknown callee), target 0xf54e4, r0=`unknown`, r1=`[sp-20]`, r2=`r2`, r3=`r3`.
  Recorded r0 = `ret(0xf54e4)` (`ldm` at 0xf6798, `bl` at 0xf678c).

## `LEDsQuery`

- `LEDsQuery`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf54e4 to 0xf6748.
  Established facts:
  Decoded words in [0xf64fc, 0xf6504), [0xf650c, 0xf6514), [0xf6528, 0xf6540), [0xf6544, 0xf6558), [0xf655c, 0xf6560), [0xf6568, 0xf6570) are not reached from the entry by known edges.
  Call (`bl` at 0xf5520): (unknown callee), target 0x20550, r0=`sp-4876`, r1=`unknown`, r2=`0x12c0`, r3=`sp-4876`.
  Call (`bl` at 0xf552c): (unknown callee), target 0x6e140, r0=`ret(0x20550)`, r1=`unknown`, r2=`unknown`, r3=`0x0`.
  Call (`bl` at 0xf556c): (unknown callee), target 0x3ec6c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf5580): (unknown callee), target 0x7fbc8, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3ec6c)`.
  Call (`bl` at 0xf55b0): (unknown callee), target 0x6e190, r0=`ret(0x7fbc8)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x7fbc8)`.
  Grouped accesses: 3 reported word load records with absolute target `0x31841c`, including target `abs 0x31841c` (`ldr` at 0xf55c8), target `abs 0x31841c` (`ldr` at 0xf56c8), target `abs 0x31841c` (`ldr` at 0xf5d54).
  Call (`bl` at 0xf5674): (unknown callee), target 0x6e190, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Grouped accesses: 6 reported word load records with absolute target `0x31a520`, including target `abs 0x31a520` (`ldr` at 0xf569c), target `abs 0x31a520` (`ldr` at 0xf5744), target `abs 0x31a520` (`ldr` at 0xf5768).
  Grouped accesses: 2 reported word load records with absolute target `0x3183f4`, including target `abs 0x3183f4` (`ldr` at 0xf56d4), target `abs 0x3183f4` (`ldr` at 0xf5d60).
  Call (`bl` at 0xf5724): (unknown callee), target 0x6e190, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`[sp-48]`.
  Call (`mov` at 0xf5794): (unknown callee), target (unknown target), r0=`[sp-56]`, r1=`[sp-44]`, r2=`sp-4880`, r3=`sp-4880`.
  Call (`mov` at 0xf581c): (unknown callee), target (unknown target), r0=`[sp-44]`, r1=`sp-4880`, r2=`[0x318404]`, r3=`sp-4880`.
  Call (`bl` at 0xf58b8): (unknown callee), target 0x201fc, r0=`0x50`, r1=`sp-20`, r2=`sp-20`, r3=`unknown`.
  Call (`bl` at 0xf58ec): (unknown callee), target 0x20634, r0=`[[sp-28]]`, r1=`0x1874cc`, r2=`unknown`, r3=`[sp-48]`.
  Call (`bl` at 0xf5900): (unknown callee), target 0x205bc, r0=`[[sp-28]]`, r1=`0x1874e8`, r2=`[sp-44]`, r3=`unknown`.
  Call (`bl` at 0xf5968): (unknown callee), target 0x7fcdc, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf5990): (unknown callee), target 0x7fe94, r0=`0x0`, r1=`unknown`, r2=`sp-20`, r3=`unknown`.
  Call (`bl` at 0xf59c8): (unknown callee), target 0x125958, r0=`0x0`, r1=`unknown`, r2=`0xffffed03`, r3=`unknown`.
  Call (`bl` at 0xf5a10): (unknown callee), target 0x125b8c, r0=`0x0`, r1=`unknown`, r2=`sp-4882`, r3=`sp-4882`.
  Call (`bl` at 0xf5c78): (unknown callee), target 0xf6748, r0=`0x7`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf5ca8): (unknown callee), target 0x201fc, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf5d00): (unknown callee), target 0x6e190, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf5e38): (unknown callee), target 0x6e1b0, r0=`[sp-56]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0xf5e54): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`sp-20`, r2=`sp-20`, r3=`0xffffecf0`.
  Call (`bl` at 0xf5ec8): (unknown callee), target 0x203f4, r0=`[sp-4908]`, r1=`[sp-4900]`, r2=`[sp-4904]`, r3=`sp-20`.
  Call (`bl` at 0xf5f2c): (unknown callee), target 0x205bc, r0=`[sp-76]`, r1=`0x187504`, r2=`[sp-4932]`, r3=`[sp-4936]`.
  Call (`bl` at 0xf5f38): (unknown callee), target 0x2031c, r0=`[sp-76]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf5f60): (unknown callee), target 0x20028, r0=`[sp-76]`, r1=`0x187514`, r2=`[sp-4888]`, r3=`0xffffecfc`.
  Call (`bl` at 0xf5f68): (unknown callee), target 0x2031c, r0=`[sp-76]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf5f98): (unknown callee), target 0x7fcdc, r0=`[sp-56]`, r1=`unknown`, r2=`ret(0x2031c)`, r3=`unknown`.
  Call (`bl` at 0xf5fb0): (unknown callee), target 0x203c4, r0=`[sp-76]`, r1=`0x187530`, r2=`unknown`, r3=`ret(0x7fcdc)`.
  Call (`bl` at 0xf5fb8): (unknown callee), target 0x2031c, r0=`[sp-76]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf5fec): (unknown callee), target 0x7fe94, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf6004): (unknown callee), target 0x203c4, r0=`[sp-76]`, r1=`0x187548`, r2=`unknown`, r3=`ret(0x7fe94)`.
  Call (`bl` at 0xf600c): (unknown callee), target 0x2031c, r0=`[sp-76]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf60ec): (unknown callee), target 0x125958, r0=`0x0`, r1=`unknown`, r2=`0xffffed03`, r3=`unknown`.
  Call (`bl` at 0xf611c): (unknown callee), target 0x205bc, r0=`[sp-76]`, r1=`0x18755c`, r2=`([sp-4881]:1 + [sp-52])`, r3=`([sp-4881]:1 + [sp-52])`.
  Call (`bl` at 0xf6124): (unknown callee), target 0x2031c, r0=`[sp-76]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf61d4): (unknown callee), target 0x205bc, r0=`[sp-76]`, r1=`0x187580`, r2=`([sp-4881]:1 + [sp-52])`, r3=`[sp-32]`.
  Call (`bl` at 0xf6210): (unknown callee), target 0x6e190, r0=`[0x1875a8]`, r1=`sp-20`, r2=`0xffffece8`, r3=`sp-4920`.
  Call (`bl` at 0xf65f0): (unknown callee), target 0x205bc, r0=`[sp-76]`, r1=`0x1875d8`, r2=`[sp-4956]`, r3=`unknown`.
  Call (`bl` at 0xf66a4): (unknown callee), target 0x205bc, r0=`[sp-76]`, r1=`0x1875ec`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf66ac): (unknown callee), target 0x2031c, r0=`[sp-76]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf66ec): (unknown callee), target 0x205bc, r0=`[sp-76]`, r1=`0x18760c`, r2=`unknown`, r3=`[sp-72]`.
  Call (`bl` at 0xf66f4): (unknown callee), target 0x2031c, r0=`[sp-76]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0xf6744).

## `LinkErrQuery`

- `LinkErrQuery`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3d388 to 0x3d9dc.
  Established facts:
  Decoded words in [0x3d820, 0x3d89c) are not reached from the entry by known edges.
  Call (`bl` at 0x3d3a4): (unknown callee), target 0x201fc, r0=`0x5dc`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0x3d3cc): (unknown callee), target 0x2031c, r0=`[sp-32]`, r1=`unknown`, r2=`[[sp-24]]`, r3=`0x0`.
  Call (`bl` at 0x3d3e8): (unknown callee), target 0x28230, r0=`0x9b`, r1=`unknown`, r2=`ret(0x2031c)`, r3=`([sp-32] + ret(0x2031c))`.
  Call (`bl` at 0x3d418): (unknown callee), target 0x205bc, r0=`[sp-32]`, r1=`0x15a5c0`, r2=`[sp-72]`, r3=`unknown`.
  Call (`bl` at 0x3d420): (unknown callee), target 0x2031c, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Word load: target `abs 0x19e6d4` (`ldr` at 0x3d43c).
  Call (`bl` at 0x3d468): (unknown callee), target 0x205bc, r0=`([sp-32] + ret(0x2031c))`, r1=`0x15a5ec`, r2=`[sp-76]`, r3=`unknown`.
  Call (`bl` at 0x3d470): (unknown callee), target 0x2031c, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Grouped accesses: 2 reported word load records with absolute target `0x19e6d8`, including target `abs 0x19e6d8` (`ldr` at 0x3d48c), target `abs 0x19e6d8` (`ldr` at 0x3d588).
  Call (`bl` at 0x3d4b0): (unknown callee), target 0x205bc, r0=`[sp-32]`, r1=`0x15a60c`, r2=`[0x19e6d8]`, r3=`unknown`.
  Call (`bl` at 0x3d4b8): (unknown callee), target 0x2031c, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3d504): (unknown callee), target 0x205bc, r0=`[sp-32]`, r1=`0x15a630`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3d540): (unknown callee), target 0x205bc, r0=`[sp-32]`, r1=`0x15a640`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3d560): (unknown callee), target 0x205bc, r0=`[sp-32]`, r1=`0x15a650`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3d568): (unknown callee), target 0x2031c, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3d5b4): (unknown callee), target 0x205bc, r0=`[sp-32]`, r1=`0x15a65c`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3d5bc): (unknown callee), target 0x2031c, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3d608): (unknown callee), target 0x205bc, r0=`[sp-32]`, r1=`0x15a630`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3d644): (unknown callee), target 0x205bc, r0=`[sp-32]`, r1=`0x15a640`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3d67c): (unknown callee), target 0x205bc, r0=`[sp-32]`, r1=`0x15a630`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3d69c): (unknown callee), target 0x205bc, r0=`[sp-32]`, r1=`0x15a650`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3d6a4): (unknown callee), target 0x2031c, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3d6c4): (unknown callee), target 0x205bc, r0=`([sp-32] + ret(0x2031c))`, r1=`0x15a67c`, r2=`ret(0x2031c)`, r3=`([sp-32] + ret(0x2031c))`.
  Call (`bl` at 0x3d6cc): (unknown callee), target 0x2031c, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3d6ec): (unknown callee), target 0x205bc, r0=`([sp-32] + ret(0x2031c))`, r1=`0x15a698`, r2=`ret(0x2031c)`, r3=`([sp-32] + ret(0x2031c))`.
  Call (`bl` at 0x3d6f4): (unknown callee), target 0x2031c, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3d714): (unknown callee), target 0x205bc, r0=`([sp-32] + ret(0x2031c))`, r1=`0x15a6c4`, r2=`ret(0x2031c)`, r3=`([sp-32] + ret(0x2031c))`.
  Call (`bl` at 0x3d71c): (unknown callee), target 0x2031c, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3d73c): (unknown callee), target 0x205bc, r0=`([sp-32] + ret(0x2031c))`, r1=`0x15a6ec`, r2=`ret(0x2031c)`, r3=`([sp-32] + ret(0x2031c))`.
  Call (`bl` at 0x3d744): (unknown callee), target 0x2031c, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3d784): (unknown callee), target 0x3c980, r0=`unknown`, r1=`sp-68`, r2=`sp-68`, r3=`unknown`.
  Call (`bl` at 0x3d7ac): (unknown callee), target 0x205bc, r0=`[sp-32]`, r1=`0x15a714`, r2=`[sp-28]`, r3=`0x15a7f4`.
  Call (`bl` at 0x3d7f0): (unknown callee), target 0x205bc, r0=`[sp-32]`, r1=`0x15a72c`, r2=`[sp-28]`, r3=`[sp-67]:1`.
  Call (`bl` at 0x3d7f8): (unknown callee), target 0x2031c, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3d8a4): (unknown callee), target 0x205bc, r0=`([sp-32] + ret(0x2031c))`, r1=`0x15a74c`, r2=`ret(0x2031c)`, r3=`unknown`.
  Call (`bl` at 0x3d8ac): (unknown callee), target 0x2031c, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3d8cc): (unknown callee), target 0x205bc, r0=`([sp-32] + ret(0x2031c))`, r1=`0x15a76c`, r2=`ret(0x2031c)`, r3=`([sp-32] + ret(0x2031c))`.
  Call (`bl` at 0x3d8d4): (unknown callee), target 0x2031c, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3d8f4): (unknown callee), target 0x205bc, r0=`([sp-32] + ret(0x2031c))`, r1=`0x15a7a4`, r2=`ret(0x2031c)`, r3=`([sp-32] + ret(0x2031c))`.
  Call (`bl` at 0x3d8fc): (unknown callee), target 0x2031c, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3d93c): (unknown callee), target 0x3cbac, r0=`unknown`, r1=`sp-64`, r2=`sp-64`, r3=`unknown`.
  Call (`bl` at 0x3d96c): (unknown callee), target 0x205bc, r0=`[sp-32]`, r1=`0x15a7dc`, r2=`[sp-28]`, r3=`0x15a7f4`.
  Call (`bl` at 0x3d9a4): (unknown callee), target 0x205bc, r0=`[sp-32]`, r1=`0x15a800`, r2=`[sp-28]`, r3=`[sp-56]`.
  Call (`bl` at 0x3d9ac): (unknown callee), target 0x2031c, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `ret(0x2031c)` (`ldm` at 0x3d9d8, `bl` at 0x3d8fc, `bl` at 0x3d9ac).

## `hourmeterQuery`

- `hourmeterQuery`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4bf98 to 0x4c3ec.
  Established facts:
  Decoded words in [0x4c3c8, 0x4c3cc), [0x4c3d0, 0x4c3ec) are not reached from the entry by known edges.
  Call (`bl` at 0x4bfb4): (unknown callee), target 0x201fc, r0=`0x5dc`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0x4bfcc): (unknown callee), target 0x6e190, r0=`ret(0x201fc)`, r1=`unknown`, r2=`unknown`, r3=`[[sp-24]]`.
  Call (`bl` at 0x4bfdc): (unknown callee), target 0x81b18, r0=`ret(0x6e190)`, r1=`0x24`, r2=`unknown`, r3=`ret(0x6e190)`.
  Call (`bl` at 0x4bff4): (unknown callee), target 0x28230, r0=`0xc9`, r1=`unknown`, r2=`unknown`, r3=`ret(0x81b18)`.
  Call (`bl` at 0x4c150): (unknown callee), target 0x205bc, r0=`[sp-56]`, r1=`0x15f234`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x4c158): (unknown callee), target 0x2031c, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x4c1a0): (unknown callee), target 0x205bc, r0=`([sp-56] + ret(0x2031c))`, r1=`0x15f260`, r2=`[sp-32]`, r3=`[sp-36]`.
  Call (`bl` at 0x4c1a8): (unknown callee), target 0x2031c, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x4c1c0): (unknown callee), target 0x6e190, r0=`ret(0x2031c)`, r1=`unknown`, r2=`ret(0x2031c)`, r3=`([sp-56] + ret(0x2031c))`.
  Call (`bl` at 0x4c1d0): (unknown callee), target 0x81b18, r0=`ret(0x6e190)`, r1=`0x25`, r2=`unknown`, r3=`ret(0x6e190)`.
  Call (`bl` at 0x4c1e8): (unknown callee), target 0x28230, r0=`0xca`, r1=`unknown`, r2=`unknown`, r3=`ret(0x81b18)`.
  Call (`bl` at 0x4c344): (unknown callee), target 0x205bc, r0=`[sp-56]`, r1=`0x15f2ac`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x4c34c): (unknown callee), target 0x2031c, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x4c394): (unknown callee), target 0x205bc, r0=`([sp-56] + ret(0x2031c))`, r1=`0x15f260`, r2=`[sp-32]`, r3=`[sp-36]`.
  Call (`bl` at 0x4c39c): (unknown callee), target 0x2031c, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x4c3bc): (unknown callee), target 0x20634, r0=`([sp-56] + ret(0x2031c))`, r1=`0x15f2d8`, r2=`ret(0x2031c)`, r3=`([sp-56] + ret(0x2031c))`.
  Recorded r0 = `ret(0x20634)` (`ldm` at 0x4c3c4, `bl` at 0x4c3bc).

## `lbi_configure_pre_commands`

- `lbi_configure_pre_commands`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x14a584 to 0x14ac44.
  Established facts:
  Decoded words in [0x14ac0c, 0x14ac44) are not reached from the entry by known edges.
  Call (`bl` at 0x14a5ac): (unknown callee), target 0x12cdbc, r0=`0x1`, r1=`0x0`, r2=`r2`, r3=`r3`.
  Grouped accesses: 17 reported word store records with printed base expression `[sp-24]` and nonnegative word-aligned offsets, including target `[sp-24] +0x0`, value `0x90000000` (`str` at 0x14a5d8), target `[sp-24] +0x0`, value `0xa800ffea` (`str` at 0x14a67c), target `[sp-24] +0x4`, value `0xabcdef01` (`str` at 0x14a690).
  Call (`bl` at 0x14a614): (unknown callee), target 0x12d2b8, r0=`0x0`, r1=`0x1`, r2=`0x1`, r3=`0x0`.
  Call (`bl` at 0x14a65c): (unknown callee), target 0x205bc, r0=`[sp-28]`, r1=`0x195538`, r2=`[[sp-24]]`, r3=`[sp-24]`.
  Call (`bl` at 0x14a6d4): (unknown callee), target 0x12d2b8, r0=`0x1`, r1=`0x1`, r2=`0x1`, r3=`0x0`.
  Call (`bl` at 0x14a7c0): (unknown callee), target 0x243d4, r0=`0x195564`, r1=`[sp-56]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x14a7e8): (unknown callee), target 0x205bc, r0=`[sp-28]`, r1=`0x1953dc`, r2=`[sp-56]`, r3=`unknown`.
  Call (`bl` at 0x14a814): (unknown callee), target 0x205bc, r0=`[sp-28]`, r1=`0x195408`, r2=`[sp-52]`, r3=`0xa800ffea`.
  Call (`bl` at 0x14a870): (unknown callee), target 0x12d2b8, r0=`0x0`, r1=`0x1`, r2=`0x1`, r3=`0x0`.
  Call (`bl` at 0x14a8d8): (unknown callee), target 0x12d2b8, r0=`0x0`, r1=`0x1`, r2=`0x1`, r3=`0x0`.
  Call (`bl` at 0x14a904): (unknown callee), target 0x12cee4, r0=`0x1`, r1=`0x0`, r2=`0x2710`, r3=`[sp-28]`.
  Call (`bl` at 0x14a96c): (unknown callee), target 0x12d2b8, r0=`0x0`, r1=`0x1`, r2=`0x1`, r3=`0x0`.
  Call (`bl` at 0x14aa30): (unknown callee), target 0x12d2b8, r0=`0x1`, r1=`0x1`, r2=`0x1`, r3=`0x0`.
  Call (`bl` at 0x14aa98): (unknown callee), target 0x12d2b8, r0=`0x0`, r1=`0x1`, r2=`0x1`, r3=`0x0`.
  Call (`bl` at 0x14aac4): (unknown callee), target 0x12cee4, r0=`0x1`, r1=`0x0`, r2=`0xc`, r3=`[sp-28]`.
  Call (`bl` at 0x14aaec): (unknown callee), target 0x12cdbc, r0=`0x1`, r1=`0x0`, r2=`[sp-28]`, r3=`ret(0x12cee4)`.
  Call (`bl` at 0x14ab54): (unknown callee), target 0x12d2b8, r0=`0x0`, r1=`0x1`, r2=`0x1`, r3=`0x0`.
  Call (`bl` at 0x14abdc): (unknown callee), target 0x12d2b8, r0=`0x1`, r1=`0x1`, r2=`0x1`, r3=`0x0`.
  Recorded r0 = `[sp-60]` (`ldm` at 0x14ac08).

## `lbi_configure_post_commands`

- `lbi_configure_post_commands`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x14ac44 to 0x14af74.
  Established facts:
  Call (`bl` at 0x14ac6c): (unknown callee), target 0x12cdbc, r0=`0x1`, r1=`0x0`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0x14ac94): (unknown callee), target 0x12cdbc, r0=`0x1`, r1=`0x0`, r2=`[sp-28]`, r3=`ret(0x12cdbc)`.
  Grouped accesses: 5 reported word store records with printed base expression `[sp-24]` and nonnegative word-aligned offsets, including target `[sp-24] +0x0`, value `0x30000000` (`str` at 0x14acc0), target `[sp-24] +0x0`, value `0xfc000000` (`str` at 0x14ad54), target `[sp-24] +0x0`, value `0xfc000000` (`str` at 0x14adbc).
  Call (`bl` at 0x14acfc): (unknown callee), target 0x12d2b8, r0=`0x0`, r1=`0x1`, r2=`0x1`, r3=`0x0`.
  Call (`bl` at 0x14ad28): (unknown callee), target 0x12cee4, r0=`0x1`, r1=`0x0`, r2=`0xc`, r3=`[sp-28]`.
  Call (`bl` at 0x14ad90): (unknown callee), target 0x12d2b8, r0=`0x0`, r1=`0x1`, r2=`0x1`, r3=`0x0`.
  Call (`bl` at 0x14adf8): (unknown callee), target 0x12d2b8, r0=`0x0`, r1=`0x1`, r2=`0x1`, r3=`0x0`.
  Call (`bl` at 0x14ae20): (unknown callee), target 0x12cdbc, r0=`0x1`, r1=`0x0`, r2=`[sp-28]`, r3=`ret(0x12d2b8)`.
  Call (`bl` at 0x14ae48): (unknown callee), target 0x12cdbc, r0=`0x1`, r1=`0x0`, r2=`[sp-28]`, r3=`ret(0x12cdbc)`.
  Call (`bl` at 0x14aeb0): (unknown callee), target 0x12d2b8, r0=`0x0`, r1=`0x1`, r2=`0x1`, r3=`0x0`.
  Call (`bl` at 0x14aedc): (unknown callee), target 0x12cee4, r0=`0x1`, r1=`0x0`, r2=`0xc`, r3=`[sp-28]`.
  Call (`bl` at 0x14af44): (unknown callee), target 0x12d2b8, r0=`0x0`, r1=`0x1`, r2=`0x1`, r3=`0x0`.
  Recorded r0 = `[sp-60]` (`ldm` at 0x14af70).

## `LBIConfigMemErr`

- `LBIConfigMemErr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8b820 to 0x8b878.
  Established facts:
  Call (`bl` at 0x8b840): (unknown callee), target 0x134634, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0x8b858): (unknown callee), target 0x8b390, r0=`[sp-20]`, r1=`[sp-24]`, r2=`[sp-26]:2`, r3=`[sp-32]`.
  Call (`bl` at 0x8b864): (unknown callee), target 0x1346fc, r0=`ret(0x8b390)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x8b390)`.
  Recorded r0 = `[sp-36]` (`ldm` at 0x8b874).

## `SMPSystemConsole`

- `SMPSystemConsole`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x944f8 to 0x94708.
  Established facts:
  Decoded words in [0x946fc, 0x94708) are not reached from the entry by known edges.
  Call (`bl` at 0x94574): (unknown callee), target 0x6e140, r0=`r0`, r1=`r1`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x945a0): (unknown callee), target 0xb80fc, r0=`[sp-52]`, r1=`[sp-32]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x945e8): (unknown callee), target 0x6e140, r0=`ret(0xb80fc)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x94634): (unknown callee), target 0x6e140, r0=`ret(0xb80fc)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Grouped accesses: 3 reported word load records with absolute target `0x31b1b8`, including target `abs 0x31b1b8` (`ldr` at 0x94678), target `abs 0x31b1b8` (`ldr` at 0x946cc), target `abs 0x31b1b8` (`ldr` at 0x946e8).
  Word store: target `abs 0x31b1b8`, value `[sp-56]` (`str` at 0x94690).
  Branch (`beq` at 0x946a0): taken to 0x946e4 when `[0x1a646c] == 0x0`, falls through to 0x946a4 when `[0x1a646c] != 0x0`.
  Call (`bl` at 0x946d0): (unknown callee), target 0x95f98, r0=`[0x31b1b8]`, r1=`unknown`, r2=`0x31a844`, r3=`0x31a844`.
  Call (`bl` at 0x946e0): (unknown callee), target 0x243d4, r0=`0x16a2c0`, r1=`ret(0x95f98)`, r2=`unknown`, r3=`ret(0x95f98)`.
  Recorded r0 = `unknown` (`ldm` at 0x946f8).

## `SMPConsoleReset`

- `SMPConsoleReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x94708 to 0x94844.
  Established facts:
  Call (`bl` at 0x94724): (unknown callee), target 0x3e38c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x94734): (unknown callee), target 0x3e4d8, r0=`ret(0x3e38c)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e38c)`.
  Call (`bl` at 0x9475c): (unknown callee), target 0x3e38c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`[sp-24]`.
  Call (`bl` at 0x94770): (unknown callee), target 0x3e458, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e38c)`.
  Call (`bl` at 0x94784): (unknown callee), target 0x3e38c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x94798): (unknown callee), target 0x3e498, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e38c)`.
  Call (`bl` at 0x947c8): (unknown callee), target 0x818a0, r0=`[sp-32]`, r1=`unknown`, r2=`0x0`, r3=`unknown`.
  Call (`bl` at 0x947dc): (unknown callee), target 0x6e190, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`[sp-24]`.
  Call (`bl` at 0x947fc): (unknown callee), target 0x942d4, r0=`0x0`, r1=`0x0`, r2=`0x0`, r3=`[sp-20]`.
  Call (`bl` at 0x94808): (unknown callee), target 0xa3c7c, r0=`0x0`, r1=`unknown`, r2=`ret(0x6e190)`, r3=`[sp-20]`.
  Call (`bl` at 0x94838): (unknown callee), target 0x942d4, r0=`[sp-20]`, r1=`0x0`, r2=`0x0`, r3=`[sp-20]`.
  Recorded r0 = `unknown` (`ldm` at 0x94840).

## `SMPConsoleSubchannel`

- `SMPConsoleSubchannel`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x942d4 to 0x944f8.
  Established facts:
  Decoded words in [0x944ec, 0x944f8) are not reached from the entry by known edges.
  Call (`bl` at 0x94300): (unknown callee), target 0xb80fc, r0=`r1`, r1=`r2`, r2=`r2`, r3=`r0`.
  Call (`bl` at 0x94314): (unknown callee), target 0xa4564, r0=`[sp-44]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x94318): (unknown callee), target 0x3e38c, r0=`ret(0xa4564)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x9432c): (unknown callee), target 0x3e458, r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e38c)`.
  Call (`bl` at 0x94340): (unknown callee), target 0x3e38c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x94354): (unknown callee), target 0x3e498, r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e38c)`.
  Call (`bl` at 0x94384): (unknown callee), target 0x818a0, r0=`[sp-48]`, r1=`unknown`, r2=`[sp-32]`, r3=`unknown`.
  Call (`bl` at 0x94388): (unknown callee), target 0x6e190, r0=`ret(0x818a0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x943b0): (unknown callee), target 0x818a0, r0=`ret(0x6e190)`, r1=`0x1`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x943b8): (unknown callee), target 0xa3c7c, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Grouped accesses: 2 reported word load records with absolute target `0x31b1b8`, including target `abs 0x31b1b8` (`ldr` at 0x94470), target `abs 0x31b1b8` (`ldr` at 0x944c4).
  Word store: target `abs 0x31b1b8`, value `[sp-56]` (`str` at 0x94488).
  Branch (`beq` at 0x94498): taken to 0x944dc when `[0x1a646c] == 0x0`, falls through to 0x9449c when `[0x1a646c] != 0x0`.
  Call (`bl` at 0x944c8): (unknown callee), target 0x95f98, r0=`[0x31b1b8]`, r1=`unknown`, r2=`0x31a844`, r3=`0x31a844`.
  Call (`bl` at 0x944d8): (unknown callee), target 0x243d4, r0=`0x16a298`, r1=`ret(0x95f98)`, r2=`unknown`, r3=`ret(0x95f98)`.
  Recorded r0 = `[sp-44]` (`ldm` at 0x944e8).

## `SocketSetBufferSize`

- `SocketSetBufferSize`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xff2b8 to 0xff684.
  Established facts:
  Decoded words in [0xff664, 0xff684) are not reached from the entry by known edges.
  Call (`bl` at 0xff304): (unknown callee), target 0x20268, r0=`r0`, r1=`0x1`, r2=`0x8`, r3=`sp-44`.
  Call (`bl` at 0xff32c): (unknown callee), target 0x202a4, r0=`[sp-20]`, r1=`0x1`, r2=`0x7`, r3=`sp-24`.
  Call (`bl` at 0xff344): (unknown callee), target 0x20184, r0=`ret(0x202a4)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x202a4)`.
  Call (`bl` at 0xff35c): (unknown callee), target 0x215fc, r0=`unknown`, r1=`0x188554`, r2=`[ret(0x20184)]`, r3=`0x1884fc`.
  Call (`bl` at 0xff390): (unknown callee), target 0x20268, r0=`[sp-20]`, r1=`0x1`, r2=`0x7`, r3=`sp-40`.
  Grouped accesses: 2 reported word load records with absolute target `0x1a288c`, including target `abs 0x1a288c` (`ldr` at 0xff3c8), target `abs 0x1a288c` (`ldr` at 0xff4f8).
  Call (`bl` at 0xff3d8): (unknown callee), target 0x244d4, r0=`[0x1a288c]`, r1=`0x188564`, r2=`[sp-44]`, r3=`[sp-24]`.
  Call (`bl` at 0xff3dc): (unknown callee), target 0x20184, r0=`ret(0x244d4)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xff3f4): (unknown callee), target 0x215fc, r0=`unknown`, r1=`0x188554`, r2=`[ret(0x20184)]`, r3=`0x1884fc`.
  Call (`bl` at 0xff434): (unknown callee), target 0x20268, r0=`unknown`, r1=`0x1`, r2=`0x8`, r3=`sp-48`.
  Call (`bl` at 0xff45c): (unknown callee), target 0x202a4, r0=`[sp-20]`, r1=`0x1`, r2=`0x8`, r3=`sp-28`.
  Call (`bl` at 0xff474): (unknown callee), target 0x20184, r0=`ret(0x202a4)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x202a4)`.
  Call (`bl` at 0xff48c): (unknown callee), target 0x215fc, r0=`unknown`, r1=`0x1885a0`, r2=`[ret(0x20184)]`, r3=`0x1884fc`.
  Call (`bl` at 0xff4c0): (unknown callee), target 0x20268, r0=`[sp-20]`, r1=`0x1`, r2=`0x8`, r3=`sp-40`.
  Call (`bl` at 0xff508): (unknown callee), target 0x244d4, r0=`[0x1a288c]`, r1=`0x1885b0`, r2=`[sp-48]`, r3=`[sp-28]`.
  Call (`bl` at 0xff50c): (unknown callee), target 0x20184, r0=`ret(0x244d4)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xff524): (unknown callee), target 0x215fc, r0=`unknown`, r1=`0x1885a0`, r2=`[ret(0x20184)]`, r3=`0x1884fc`.
  Call (`bl` at 0xff57c): (unknown callee), target 0x202a4, r0=`unknown`, r1=`0x6`, r2=`0x1`, r3=`sp-52`.
  Call (`bl` at 0xff594): (unknown callee), target 0x20184, r0=`ret(0x202a4)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x202a4)`.
  Call (`bl` at 0xff5a8): (unknown callee), target 0x20184, r0=`ret(0x20184)`, r1=`unknown`, r2=`unknown`, r3=`[ret(0x20184)]`.
  Call (`bl` at 0xff5c0): (unknown callee), target 0x215fc, r0=`unknown`, r1=`0x1885ec`, r2=`[ret(0x20184)]`, r3=`0x1884fc`.
  Call (`bl` at 0xff5fc): (unknown callee), target 0x202a4, r0=`[sp-20]`, r1=`0x0`, r2=`0x1`, r3=`sp-52`.
  Call (`bl` at 0xff614): (unknown callee), target 0x20184, r0=`ret(0x202a4)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x202a4)`.
  Call (`bl` at 0xff628): (unknown callee), target 0x20184, r0=`ret(0x20184)`, r1=`unknown`, r2=`unknown`, r3=`[ret(0x20184)]`.
  Call (`bl` at 0xff640): (unknown callee), target 0x215fc, r0=`unknown`, r1=`0x188604`, r2=`[ret(0x20184)]`, r3=`0x1884fc`.
  Recorded r0 = `unknown` (`ldm` at 0xff660).

## `config`

- `config`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb6348 to 0xb6bd4.
  Established facts:
  Decoded words in [0xb6bb0, 0xb6bd4) are not reached from the entry by known edges.
  Grouped accesses: 25 reported word load records with absolute target `0x31b5b8`, including target `abs 0x31b5b8` (`ldr` at 0xb6370), target `abs 0x31b5b8` (`ldr` at 0xb638c), target `abs 0x31b5b8` (`ldr` at 0xb63b4).
  Call (`bl` at 0xb639c): (unknown callee), target 0x204d8, r0=`[sp-28]`, r1=`0x734`, r2=`unknown`, r3=`[0x31b5b8]`.
  Call (`bl` at 0xb6960): (unknown callee), target 0x28230, r0=`0x8`, r1=`[sp-40]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xb6ae8): (unknown callee), target 0x100280, r0=`unknown`, r1=`unknown`, r2=`0x100`, r3=`unknown`.
  Call (`bl` at 0xb6b94): (unknown callee), target 0x101248, r0=`unknown`, r1=`unknown`, r2=`0x100`, r3=`0xbccfc`.
  Recorded r0 = `unknown` (`ldm` at 0xb6bac).

## `SMPCmdResponseDelay`

- `SMPCmdResponseDelay`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc500c to 0xc50d8.
  Established facts:
  Decoded words in [0xc50d0, 0xc50d4) are not reached from the entry by known edges.
  Call (`bl` at 0xc503c): (unknown callee), target 0x958d4, r0=`sp-36`, r1=`sp-52`, r2=`r0`, r3=`sp-36`.
  Branch (`beq` at 0xc5048): taken to 0xc507c when `[sp-24] == 0x0`, falls through to 0xc504c when `[sp-24] != 0x0`.
  Call (`bl` at 0xc5050): (unknown callee), target 0x2031c, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`[sp-24]`.
  Call (`bl` at 0xc5078): (unknown callee), target 0x95b5c, r0=`sp-36`, r1=`sp-52`, r2=`[sp-24]`, r3=`ret(0x2031c)`.
  Word load: target `[sp-28] +0x0` (`ldr` at 0xc5090).
  Call (`bl` at 0xc5098): (unknown callee), target 0xfd0dc, r0=`0x31b684`, r1=`0xfffffd95`, r2=`[[sp-28]]`, r3=`[sp-36]`.
  Recorded r0 = `[sp-56]` (`ldm` at 0xc50cc).

## `SMPCmdNoResponse`

- `SMPCmdNoResponse`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc5cac to 0xc5cfc.
  Established facts:
  Decoded words in [0xc5cf8, 0xc5cfc) are not reached from the entry by known edges.
  Call (`bl` at 0xc5cc8): (unknown callee), target 0x200ac, r0=`r0`, r1=`0x170974`, r2=`r2`, r3=`r3`.
  Branch (`bne` at 0xc5cd4): taken to 0xc5ce4 when `ret(0x200ac) != 0x0`, falls through to 0xc5cd8 when `ret(0x200ac) == 0x0` (source `bl` at 0xc5cc8).
  Recorded r0 = `[sp-24]` (`ldm` at 0xc5cf4).

## `slit_set_command_function`

- `slit_set_command_function`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x14dcb8 to 0x14dce0.
  Established facts:
  Decoded words in [0x14dcdc, 0x14dce0) are not reached from the entry by known edges.
  Word store: target `abs 0x3162bc`, value `r0` (`str` at 0x14dcd4).
  Recorded r0 = `r0` (`ldm` at 0x14dcd8).
