# ARM Linux 2.6 BrickOpsInit and related entrypoints in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-board-setup-dispatch-facts.txt) retain all tool sections.

The notation `ret(ADDR)` identifies a call-result expression without distinguishing
individual call sites to that target. Each return line records machine-register
state from the analysis rather than a C return type.

## `IP71_BrickOpsInit`

- `IP71_BrickOpsInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4ebc8 to 0x4ed88.
  Established facts:
  Grouped word stores: 26 reported accesses with printed base expression `[sp-20]` and nonnegative word-aligned offsets, including offset `+0x4`, value `0x4e678` (`str` at 0x4ebf8), offset `+0x14`, value `0x0` (`str` at 0x4ec10), offset `+0x18`, value `0x160548` (`str` at 0x4ec1c).
  Byte store: target `[sp-20] +0x10`, value `0x54` (`strb` at 0x4ec04).
  Recorded r0 = `r0` (`ldm` at 0x4ed38).

## `Athena_BrickOpsInit`

- `Athena_BrickOpsInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x33c78 to 0x33e00.
  Established facts:
  Grouped word stores: 25 reported accesses with printed base expression `[sp-20]` and nonnegative word-aligned offsets, including offset `+0x4`, value `0x3366c` (`str` at 0x33ca8), offset `+0x14`, value `0x0` (`str` at 0x33cc0), offset `+0x110`, value `0xff` (`str` at 0x33ccc).
  Recorded r0 = `r0` (`ldm` at 0x33ddc).

## `OpusTio_BrickOpsInit`

- `OpusTio_BrickOpsInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3292c to 0x32adc.
  Established facts:
  Grouped word stores: 26 reported accesses with printed base expression `[sp-20]` and nonnegative word-aligned offsets, including offset `+0x4`, value `0x32620` (`str` at 0x3295c), offset `+0x14`, value `0x0` (`str` at 0x32974), offset `+0x110`, value `0x11` (`str` at 0x32980).
  Byte store: target `[sp-20] +0x10`, value `0x52` (`strb` at 0x32968).
  Recorded r0 = `r0` (`ldm` at 0x32a9c).

## `tio_test_pci_reset_int`

- `tio_test_pci_reset_int`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x32c58 to 0x32cc8.
  Established facts:
  Branch (`bne` at 0x32c90): taken to 0x32cc0 when `(r1 & 0xff) != 0xc`, falls through to 0x32c94 when `(r1 & 0xff) == 0xc`.
  Branch (`beq` at 0x32ca0): taken to 0x32cc0 when `? == 0x0`, falls through to 0x32ca4 when `? != 0x0`.
  Call (`bl` at 0x32cbc): (unknown callee), target 0x745b8, r0=`0x0`, r1=`(r1 & 0xff)`, r2=`0x2`, r3=`(r3 & 0xff)`.
  Recorded r0 = `unknown` (`ldm` at 0x32cc4).

## `TH1932_BrickOpsInit`

- `TH1932_BrickOpsInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x44e84 to 0x44f40.
  Established facts:
  Grouped word stores: 9 reported accesses with printed base expression `[sp-20]` and nonnegative word-aligned offsets, including offset `+0x4`, value `0x44a18` (`str` at 0x44eb4), offset `+0x14`, value `0x0` (`str` at 0x44ecc), offset `+0x16c`, value `0x38a40` (`str` at 0x44ed8).
  Recorded r0 = `r0` (`ldm` at 0x44f28).

## `NL4S_BrickOpsInit`

- `NL4S_BrickOpsInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x30474 to 0x304a4.
  Established facts:
  Call (`bl` at 0x30494): (unknown callee), target 0x302f0, r0=`r0`, r1=`r1`, r2=`r2`, r3=`0x19e522`.
  Recorded r0 = `ret(0x302f0)` (`ldm` at 0x3049c, `bl` at 0x30494).

## `TH191010_BrickOpsInit`

- `TH191010_BrickOpsInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x49b50 to 0x49c18.
  Established facts:
  Grouped word stores: 10 reported accesses with printed base expression `[sp-20]` and nonnegative word-aligned offsets, including offset `+0x4`, value `0x49594` (`str` at 0x49b80), offset `+0x14`, value `0x0` (`str` at 0x49b98), offset `+0x16c`, value `0x38a40` (`str` at 0x49ba4).
  Byte store: target `[sp-20] +0x10`, value `0x55` (`strb` at 0x49b8c).
  Recorded r0 = `r0` (`ldm` at 0x49c00).

## `NL4R_BrickOpsInit`

- `NL4R_BrickOpsInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x30434 to 0x30474.
  Established facts:
  Word store: target `r0 +0x0`, value `0x30138` (`str` at 0x30458).
  Call (`bl` at 0x30460): (unknown callee), target 0x302f0, r0=`[sp-20]`, r1=`r1`, r2=`r0`, r3=`0x30138`.
  Recorded r0 = `ret(0x302f0)` (`ldm` at 0x30468, `bl` at 0x30460).

## `PA_BrickOpsInit`

- `PA_BrickOpsInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x30f54 to 0x310c8.
  Established facts:
  Grouped word stores: 23 reported accesses with printed base expression `[sp-20]` and nonnegative word-aligned offsets, including offset `+0x4`, value `0x30a98` (`str` at 0x30f84), offset `+0x14`, value `0x0` (`str` at 0x30f9c), offset `+0x110`, value `0xff` (`str` at 0x30fa8).
  Recorded r0 = `r0` (`ldm` at 0x310a0).

## `IP69_BrickOpsInit`

- `IP69_BrickOpsInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4446c to 0x44654.
  Established facts:
  Grouped word stores: 27 reported accesses with printed base expression `[sp-20]` and nonnegative word-aligned offsets, including offset `+0x4`, value `0x438d0` (`str` at 0x4449c), offset `+0x14`, value `0x0` (`str` at 0x444b4), offset `+0x110`, value `0xb` (`str` at 0x444c0).
  Call (`bl` at 0x445ec): (unknown callee), target 0x2624c, r0=`0x2a`, r1=`0x0`, r2=`[sp-20]`, r3=`0x44094`.
  Call (`bl` at 0x445f8): (unknown callee), target 0x2624c, r0=`0x2b`, r1=`0x0`, r2=`unknown`, r3=`unknown`.
  Word store: target `abs 0x1a5c20`, value `0x1` (`str` at 0x44604).
  Recorded r0 = `ret(0x2624c)` (`ldm` at 0x4460c, `bl` at 0x445f8).

## `Vortix_BrickOpsInit`

- `Vortix_BrickOpsInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2b6bc to 0x2b8c8.
  Established facts:
  Call (`bl` at 0x2b6e8): (unknown callee), target 0x203f4, r0=`sp-96`, r1=`0x15522c`, r2=`0x4c`, r3=`0x15522c`.
  Grouped word stores: 29 reported accesses with printed base expression `[sp-20]` and nonnegative word-aligned offsets, including offset `+0x0`, value `0x2b5b0` (`str` at 0x2b6fc), offset `+0x4`, value `0x2b298` (`str` at 0x2b708), offset `+0x14`, value `0x0` (`str` at 0x2b720).
  Call (`bl` at 0x2b7b4): (unknown callee), target 0x203f4, r0=`[sp-20]+28`, r1=`sp-96`, r2=`0x4c`, r3=`[sp-20]+28`.
  Recorded r0 = `ret(0x203f4)` (`ldm` at 0x2b87c, `bl` at 0x2b7b4).

## `Dorado_BrickOpsInit`

- `Dorado_BrickOpsInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x35b88 to 0x35d98.
  Established facts:
  Grouped word stores: 32 reported accesses with printed base expression `[sp-20]` and nonnegative word-aligned offsets, including offset `+0x4`, value `0x3589c` (`str` at 0x35bb8), offset `+0x14`, value `0x0` (`str` at 0x35bd0), offset `+0x110`, value `0x13` (`str` at 0x35bdc).
  Recorded r0 = `r0` (`ldm` at 0x35d40).

## `TIOTest_BrickOpsInit`

- `TIOTest_BrickOpsInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x33b2c to 0x33c78.
  Established facts:
  Grouped word stores: 20 reported accesses with printed base expression `[sp-20]` and nonnegative word-aligned offsets, including offset `+0x4`, value `0x3344c` (`str` at 0x33b5c), offset `+0x14`, value `0x0` (`str` at 0x33b74), offset `+0x110`, value `0xff` (`str` at 0x33b80).
  Recorded r0 = `r0` (`ldm` at 0x33c54).

## `TH1955_BrickOpsInit`

- `TH1955_BrickOpsInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x50d48 to 0x50e10.
  Established facts:
  Grouped word stores: 10 reported accesses with printed base expression `[sp-20]` and nonnegative word-aligned offsets, including offset `+0x4`, value `0x508d0` (`str` at 0x50d78), offset `+0x14`, value `0x0` (`str` at 0x50d90), offset `+0x16c`, value `0x38a40` (`str` at 0x50d9c).
  Byte store: target `[sp-20] +0x10`, value `0x57` (`strb` at 0x50d84).
  Recorded r0 = `r0` (`ldm` at 0x50df8).

## `Gemini_BrickOpsInit`

- `Gemini_BrickOpsInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2d064 to 0x2d214.
  Established facts:
  Grouped word stores: 26 reported accesses with printed base expression `[sp-20]` and nonnegative word-aligned offsets, including offset `+0x4`, value `0x2cda0` (`str` at 0x2d094), offset `+0x14`, value `0x0` (`str` at 0x2d0ac), offset `+0x110`, value `0xf` (`str` at 0x2d0b8).
  Recorded r0 = `r0` (`ldm` at 0x2d1d4).
