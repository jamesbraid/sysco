# ARM Linux 2.6 field and data routines in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-jchip-field-data-facts.txt) retain all tool sections.

The notation `ret(ADDR)` identifies a call-result expression without distinguishing
individual call sites to that target. Each return line records machine-register
state from the analysis rather than a C return type.


Groups summarize static access records by printed shape. Instruction-address order can include different control-flow paths.

## `JChip_protect_field`

- `JChip_protect_field`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13e948 to 0x13eae8.
  Established facts:
  Decoded words in [0x13ead0, 0x13ead8), [0x13eadc, 0x13eae8) are not reached from the entry by known edges.
  Grouped accesses: 2 reported word load records with absolute target `0x315034`, including target `abs 0x315034` (`ldr` at 0x13e96c), target `abs 0x315034` (`ldr` at 0x13e9b8).
  Branch (`bls` at 0x13e974): taken to 0x13e98c when `[0x315034] <= 0x2 (unsigned, 32-bit)`, falls through to 0x13e978 when `[0x315034] > 0x2 (unsigned, 32-bit)`.
  Call (`bl` at 0x13e988): (unknown callee), target 0x243d4, r0=`0x191ff8`, r1=`r0`, r2=`r1`, r3=`r2`.
  Call (`bl` at 0x13e990): (unknown callee), target 0x13dfe0, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`bls` at 0x13e9c0): taken to 0x13ea2c when `[0x315034] <= 0x1 (unsigned, 32-bit)`, falls through to 0x13e9c4 when `[0x315034] > 0x1 (unsigned, 32-bit)`.
  Call (`bl` at 0x13ea04): (unknown callee), target 0x243d4, r0=`0x192030`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x13ea20): (unknown callee), target 0x243d4, r0=`0x192050`, r1=`[[sp-32]]`, r2=`unknown`, r3=`[sp-32]`.
  Call (`bl` at 0x13ea28): (unknown callee), target 0x243d4, r0=`0x192064`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x13ea5c): (unknown callee), target 0x144d78, r0=`unknown`, r1=`[sp-28]`, r2=`[sp-32]`, r3=`0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x13eacc).

## `JChip_get_field`

- `JChip_get_field`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13eae8 to 0x13ec18.
  Established facts:
  Decoded words in [0x13ec08, 0x13ec10), [0x13ec14, 0x13ec18) are not reached from the entry by known edges.
  Grouped accesses: 2 reported word load records with absolute target `0x315034`, including target `abs 0x315034` (`ldr` at 0x13eb0c), target `abs 0x315034` (`ldr` at 0x13eb58).
  Branch (`bls` at 0x13eb14): taken to 0x13eb2c when `[0x315034] <= 0x2 (unsigned, 32-bit)`, falls through to 0x13eb18 when `[0x315034] > 0x2 (unsigned, 32-bit)`.
  Call (`bl` at 0x13eb28): (unknown callee), target 0x243d4, r0=`0x192068`, r1=`r0`, r2=`r1`, r3=`r2`.
  Call (`bl` at 0x13eb30): (unknown callee), target 0x13dfe0, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`bls` at 0x13eb60): taken to 0x13eba8 when `[0x315034] <= 0x3 (unsigned, 32-bit)`, falls through to 0x13eb64 when `[0x315034] > 0x3 (unsigned, 32-bit)`.
  Call (`bl` at 0x13eba4): (unknown callee), target 0x243d4, r0=`0x19209c`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x13ebf0): (unknown callee), target 0x143a3c, r0=`unknown`, r1=`[sp-28]`, r2=`0x0`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x13ec04).

## `JChip_get_field_by_name`

- `JChip_get_field_by_name`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13ec18 to 0x13ed9c.
  Established facts:
  Decoded words in [0x13ed8c, 0x13ed94), [0x13ed98, 0x13ed9c) are not reached from the entry by known edges.
  Grouped accesses: 2 reported word load records with absolute target `0x315034`, including target `abs 0x315034` (`ldr` at 0x13ec3c), target `abs 0x315034` (`ldr` at 0x13ecdc).
  Branch (`bls` at 0x13ec44): taken to 0x13ec5c when `[0x315034] <= 0x2 (unsigned, 32-bit)`, falls through to 0x13ec48 when `[0x315034] > 0x2 (unsigned, 32-bit)`.
  Call (`bl` at 0x13ec58): (unknown callee), target 0x243d4, r0=`0x1920bc`, r1=`r0`, r2=`r1`, r3=`r2`.
  Call (`bl` at 0x13ec60): (unknown callee), target 0x13dfe0, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x13ecac): (unknown callee), target 0x143d1c, r0=`[sp-44]`, r1=`[sp-28]`, r2=`unknown`, r3=`unknown`.
  Halfword load: target `[sp-44] +0x2` (`ldrh` at 0x13ecbc).
  Call (`bl` at 0x13ed28): (unknown callee), target 0x243d4, r0=`0x19209c`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x13ed74): (unknown callee), target 0x143a3c, r0=`unknown`, r1=`unknown`, r2=`0x0`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x13ed88).

## `JChip_get_field_str`

- `JChip_get_field_str`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13ed9c to 0x13ef78.
  Established facts:
  Decoded words in [0x13ef64, 0x13ef6c), [0x13ef70, 0x13ef74) are not reached from the entry by known edges.
  Grouped accesses: 2 reported word load records with absolute target `0x315034`, including target `abs 0x315034` (`ldr` at 0x13edc0), target `abs 0x315034` (`ldr` at 0x13ee0c).
  Branch (`bls` at 0x13edc8): taken to 0x13ede0 when `[0x315034] <= 0x2 (unsigned, 32-bit)`, falls through to 0x13edcc when `[0x315034] > 0x2 (unsigned, 32-bit)`.
  Call (`bl` at 0x13eddc): (unknown callee), target 0x243d4, r0=`0x1920f8`, r1=`r0`, r2=`r1`, r3=`r2`.
  Call (`bl` at 0x13ede4): (unknown callee), target 0x13dfe0, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`bls` at 0x13ee14): taken to 0x13ee5c when `[0x315034] <= 0x3 (unsigned, 32-bit)`, falls through to 0x13ee18 when `[0x315034] > 0x3 (unsigned, 32-bit)`.
  Call (`bl` at 0x13ee58): (unknown callee), target 0x243d4, r0=`0x192130`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x13eec8): (unknown callee), target 0x201fc, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x13ef34): (unknown callee), target 0x143a3c, r0=`unknown`, r1=`[sp-28]`, r2=`0x0`, r3=`unknown`.
  Call (`bl` at 0x13ef44): (unknown callee), target 0x144710, r0=`[sp-40]`, r1=`[sp-44]`, r2=`[sp-32]`, r3=`unknown`.
  Call (`bl` at 0x13ef4c): (unknown callee), target 0x20514, r0=`[sp-44]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x13ef60).

## `JChip_can_override_field`

- `JChip_can_override_field`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13ef78 to 0x13f048.
  Established facts:
  Decoded words in [0x13f040, 0x13f048) are not reached from the entry by known edges.
  Word load: target `abs 0x315034` (`ldr` at 0x13ef98).
  Branch (`bls` at 0x13efa0): taken to 0x13efb8 when `[0x315034] <= 0x2 (unsigned, 32-bit)`, falls through to 0x13efa4 when `[0x315034] > 0x2 (unsigned, 32-bit)`.
  Call (`bl` at 0x13efb4): (unknown callee), target 0x243d4, r0=`0x192154`, r1=`r0`, r2=`r1`, r3=`r2`.
  Call (`bl` at 0x13efbc): (unknown callee), target 0x13dfe0, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Halfword load: target `(unknown effective address)` (`ldrh` at 0x13f008).
  Recorded r0 = `unknown` (`ldm` at 0x13f03c).

## `JChip_override_field`

- `JChip_override_field`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13f048 to 0x13f1ac.
  Established facts:
  Decoded words in [0x13f19c, 0x13f1a4), [0x13f1a8, 0x13f1ac) are not reached from the entry by known edges.
  Grouped accesses: 2 reported word load records with absolute target `0x315034`, including target `abs 0x315034` (`ldr` at 0x13f06c), target `abs 0x315034` (`ldr` at 0x13f108).
  Branch (`bls` at 0x13f074): taken to 0x13f098 when `[0x315034] <= 0x2 (unsigned, 32-bit)`, falls through to 0x13f078 when `[0x315034] > 0x2 (unsigned, 32-bit)`.
  Call (`bl` at 0x13f094): (unknown callee), target 0x243d4, r0=`0x192190`, r1=`r0`, r2=`r1`, r3=`r2`.
  Call (`bl` at 0x13f09c): (unknown callee), target 0x13dfe0, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`bls` at 0x13f110): taken to 0x13f160 when `[0x315034] <= 0x3 (unsigned, 32-bit)`, falls through to 0x13f114 when `[0x315034] > 0x3 (unsigned, 32-bit)`.
  Call (`bl` at 0x13f15c): (unknown callee), target 0x243d4, r0=`0x1921d8`, r1=`unknown`, r2=`[[sp-32]]`, r3=`unknown`.
  Call (`bl` at 0x13f184): (unknown callee), target 0x144d78, r0=`unknown`, r1=`[sp-28]`, r2=`[sp-32]`, r3=`0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x13f198).

## `JChip_override_field_flags`

- `JChip_override_field_flags`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13f1ac to 0x13f32c.
  Established facts:
  Decoded words in [0x13f31c, 0x13f324), [0x13f328, 0x13f32c) are not reached from the entry by known edges.
  Grouped accesses: 2 reported word load records with absolute target `0x315034`, including target `abs 0x315034` (`ldr` at 0x13f1d8), target `abs 0x315034` (`ldr` at 0x13f27c).
  Branch (`bls` at 0x13f1e0): taken to 0x13f20c when `[0x315034] <= 0x2 (unsigned, 32-bit)`, falls through to 0x13f1e4 when `[0x315034] > 0x2 (unsigned, 32-bit)`.
  Call (`bl` at 0x13f208): (unknown callee), target 0x243d4, r0=`0x19220c`, r1=`r0`, r2=`r1`, r3=`r2`.
  Call (`bl` at 0x13f210): (unknown callee), target 0x13dfe0, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`bls` at 0x13f284): taken to 0x13f2dc when `[0x315034] <= 0x3 (unsigned, 32-bit)`, falls through to 0x13f288 when `[0x315034] > 0x3 (unsigned, 32-bit)`.
  Call (`bl` at 0x13f2d8): (unknown callee), target 0x243d4, r0=`0x192268`, r1=`unknown`, r2=`[[sp-32]]`, r3=`[sp-34]:2`.
  Call (`bl` at 0x13f304): (unknown callee), target 0x144d78, r0=`unknown`, r1=`[sp-28]`, r2=`[sp-32]`, r3=`[sp-34]:2`.
  Recorded r0 = `unknown` (`ldm` at 0x13f318).

## `JChip_override_fields`

- `JChip_override_fields`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13f32c to 0x13f45c.
  Established facts:
  Decoded words in [0x13f454, 0x13f45c) are not reached from the entry by known edges.
  Word load: target `abs 0x315034` (`ldr` at 0x13f350).
  Branch (`bls` at 0x13f358): taken to 0x13f370 when `[0x315034] <= 0x3 (unsigned, 32-bit)`, falls through to 0x13f35c when `[0x315034] > 0x3 (unsigned, 32-bit)`.
  Call (`bl` at 0x13f36c): (unknown callee), target 0x243d4, r0=`0x1922b0`, r1=`r0`, r2=`r2`, r3=`(r3 & 0xffff)`.
  Word load: target `[sp-44] +0x4` (`ldr` at 0x13f398).
  Call (`bl` at 0x13f400): (unknown callee), target 0x13f1ac, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x13f450).

## `JChip_override_field_by_name`

- `JChip_override_field_by_name`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13f45c to 0x13f614.
  Established facts:
  Decoded words in [0x13f604, 0x13f60c), [0x13f610, 0x13f614) are not reached from the entry by known edges.
  Grouped accesses: 2 reported word load records with absolute target `0x315034`, including target `abs 0x315034` (`ldr` at 0x13f480), target `abs 0x315034` (`ldr` at 0x13f570).
  Branch (`bls` at 0x13f488): taken to 0x13f4ac when `[0x315034] <= 0x2 (unsigned, 32-bit)`, falls through to 0x13f48c when `[0x315034] > 0x2 (unsigned, 32-bit)`.
  Call (`bl` at 0x13f4a8): (unknown callee), target 0x243d4, r0=`0x1922f0`, r1=`r0`, r2=`r1`, r3=`r2`.
  Call (`bl` at 0x13f4b0): (unknown callee), target 0x13dfe0, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x13f4fc): (unknown callee), target 0x143d1c, r0=`[sp-44]`, r1=`[sp-28]`, r2=`unknown`, r3=`unknown`.
  Branch (`bls` at 0x13f578): taken to 0x13f5c8 when `[0x315034] <= 0x3 (unsigned, 32-bit)`, falls through to 0x13f57c when `[0x315034] > 0x3 (unsigned, 32-bit)`.
  Call (`bl` at 0x13f5c4): (unknown callee), target 0x243d4, r0=`0x192344`, r1=`unknown`, r2=`[[sp-32]]`, r3=`unknown`.
  Call (`bl` at 0x13f5ec): (unknown callee), target 0x144d78, r0=`unknown`, r1=`unknown`, r2=`[sp-32]`, r3=`0x8`.
  Recorded r0 = `unknown` (`ldm` at 0x13f600).

## `JChip_find_tdr`

- `JChip_find_tdr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13f6bc to 0x13f7a4.
  Established facts:
  Decoded words in [0x13f798, 0x13f7a0) are not reached from the entry by known edges.
  Word load: target `abs 0x315034` (`ldr` at 0x13f6d8).
  Branch (`bls` at 0x13f6e0): taken to 0x13f6f4 when `[0x315034] <= 0x2 (unsigned, 32-bit)`, falls through to 0x13f6e4 when `[0x315034] > 0x2 (unsigned, 32-bit)`.
  Call (`bl` at 0x13f6f0): (unknown callee), target 0x243d4, r0=`0x1923b8`, r1=`r0`, r2=`r1`, r3=`[0x315034]`.
  Call (`bl` at 0x13f6f8): (unknown callee), target 0x13dfe0, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Word load: target `(unknown effective address)` (`ldr` at 0x13f72c).
  Call (`bl` at 0x13f758): (unknown callee), target 0x200ac, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x13f794).

## `JChip_find_tdr_field`

- `JChip_find_tdr_field`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13f7a4 to 0x13f880.
  Established facts:
  Decoded words in [0x13f874, 0x13f87c) are not reached from the entry by known edges.
  Word load: target `abs 0x315034` (`ldr` at 0x13f7c4).
  Branch (`bls` at 0x13f7cc): taken to 0x13f7e4 when `[0x315034] <= 0x2 (unsigned, 32-bit)`, falls through to 0x13f7d0 when `[0x315034] > 0x2 (unsigned, 32-bit)`.
  Call (`bl` at 0x13f7e0): (unknown callee), target 0x243d4, r0=`0x1923e4`, r1=`r0`, r2=`r1`, r3=`r2`.
  Call (`bl` at 0x13f7e8): (unknown callee), target 0x13dfe0, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x13f834): (unknown callee), target 0x143d1c, r0=`[sp-40]`, r1=`[sp-28]`, r2=`unknown`, r3=`unknown`.
  Halfword load: target `[sp-40] +0x2` (`ldrh` at 0x13f844).
  Recorded r0 = `unknown` (`ldm` at 0x13f870).

## `tdrdata_set_data`

- `tdrdata_set_data`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x144848 to 0x144920.
  Established facts:
  Halfword load: target `(unknown effective address)` (`ldrh` at 0x144870).
  Halfword load: target `(unknown effective address)` (`ldrh` at 0x144890).
  Halfword load: target `(unknown effective address)` (`ldrh` at 0x1448b8).
  Word store: target `(unknown effective address)`, value `unknown` (`str` at 0x144904).
  Recorded r0 = `r0` (`ldm` at 0x14491c).

## `tdrdata_get_data`

- `tdrdata_get_data`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x144920 to 0x144ac4.
  Established facts:
  Branch (`beq` at 0x144944): taken to 0x144a04 when `r1 == 0x0`, falls through to 0x144948 when `r1 != 0x0`.
  Halfword load: target `(unknown effective address)` (`ldrh` at 0x144958).
  Halfword load: target `(unknown effective address)` (`ldrh` at 0x144978).
  Halfword load: target `(unknown effective address)` (`ldrh` at 0x1449a0).
  Word store: target `(unknown effective address)`, value `unknown` (`str` at 0x1449f0).
  Word store: target `(unknown effective address)`, value `unknown` (`str` at 0x144aa8).
  Recorded r0 = `unknown` (`ldm` at 0x144ac0).

## `tdrdata_override_field`

- `tdrdata_override_field`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x144d78 to 0x144e3c.
  Established facts:
  Grouped accesses: 3 reported word load records with printed base expression `r0` and nonnegative word-aligned offsets, including target `r0 +0xc` (`ldr` at 0x144db4), target `r0 +0xc` (`ldr` at 0x144ddc), target `r0 +0xc` (`ldr` at 0x144df0).
  Halfword store: target `(unknown effective address)`, value `unknown` (`strh` at 0x144e0c).
  Call (`bl` at 0x144e30): (unknown callee), target 0x1437f4, r0=`[[sp-20]+8]`, r1=`[sp-24]`, r2=`[sp-28]`, r3=`0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x144e38).

## `tdrdata_add_overrides`

- `tdrdata_add_overrides`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x144e3c to 0x144f4c.
  Established facts:
  Halfword load: target `(unknown effective address)` (`ldrh` at 0x144ec0).
  Halfword load: target `(unknown effective address)` (`ldrh` at 0x144eec).
  Halfword load: target `(unknown effective address)` (`ldrh` at 0x144f1c).
  Call (`bl` at 0x144f30): (unknown callee), target 0x132444, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x144f48).
