# ARM Linux 2.6 port, display and packet routines in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-port-display-packet-facts.txt) retain all tool sections.

The notation `ret(ADDR)` identifies a call-result expression without distinguishing
individual call sites to that target. Each return line records machine-register
state from the analysis rather than a C return type.


Groups summarize static access records by printed shape. Instruction-address order can include different control-flow paths.

## `portInit`

- `portInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x59ea4 to 0x5a2c0.
  Established facts:
  Branch (`beq` at 0x59ecc): taken to 0x59f18 when `[0x1a5c28] == 0x0`, falls through to 0x59ed0 when `[0x1a5c28] != 0x0`.
  Call (`bl` at 0x59ed0): (unknown callee), target 0x6e190, r0=`r0`, r1=`r1`, r2=`r2`, r3=`[0x1a5c28]`.
  Call (`bl` at 0x59f14): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x1621c4`, r2=`0x97`, r3=`0x0`.
  Call (`bl` at 0x59f28): (unknown callee), target 0x6e140, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`[0x1a5c2c]`.
  Call (`bl` at 0x59f60): (unknown callee), target 0x204d8, r0=`[sp-44]`, r1=`0x1`, r2=`[0x1a5c2c]`, r3=`unknown`.
  Call (`bl` at 0x59f98): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x1621c4`, r2=`0xa3`, r3=`0x0`.
  Call (`bl` at 0x59fac): (unknown callee), target 0x22b84, r0=`0x1a5c30`, r1=`0x162210`, r2=`0x1`, r3=`0x9`.
  Grouped accesses: 3 reported word store records with printed base expression `[sp-40]` and nonnegative word-aligned offsets, including target `[sp-40] +0x0`, value `unknown` (`str` at 0x59fdc), target `[sp-40] +0x8`, value `[sp-32]` (`str` at 0x59fe8), target `[sp-40] +0x4`, value `ret(0x204d8)` (`str` at 0x5a004) (source `bl` at 0x59ffc).
  Grouped accesses: 13 reported word load records with printed base expression `[sp-40]` and nonnegative word-aligned offsets, including target `[sp-40] +0x8` (`ldr` at 0x59ff4), target `[sp-40] +0x4` (`ldr` at 0x5a008), target `[sp-40] +0x8` (`ldr` at 0x5a0b8).
  Call (`bl` at 0x59ffc): (unknown callee), target 0x204d8, r0=`[[sp-40]+8]`, r1=`0x4`, r2=`[sp-40]`, r3=`[sp-40]`.
  Call (`bl` at 0x5a024): (unknown callee), target 0x6e1b0, r0=`[sp-24]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x5a030): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x5a078): (unknown callee), target 0x203f4, r0=`[sp-52]`, r1=`[sp-44]`, r2=`[sp-48]`, r3=`unknown`.
  Call (`bl` at 0x5a0a4): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x1621c4`, r2=`0xb1`, r3=`0x0`.
  Call (`bl` at 0x5a120): (unknown callee), target 0x125958, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x5a1a8): (unknown callee), target 0x125958, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x5a214): (unknown callee), target 0x125958, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Byte store: target `(unknown effective address)`, value `0x0` (`strb` at 0x5a260).
  Call (`bl` at 0x5a294): (unknown callee), target 0x5a38c, r0=`[sp-24]`, r1=`unknown`, r2=`[[sp-40]+8]`, r3=`unknown`.
  Recorded r0 = `ret(0x5a38c)` (`ldm` at 0x5a29c, `bl` at 0x5a294).

## `portRemove`

- `portRemove`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5a2c0 to 0x5a38c.
  Established facts:
  Branch (`bge` at 0x5a2e4): taken to 0x5a37c when `r0 >= [0x1a5c2c] (signed, 32-bit)`, falls through to 0x5a2e8 when `r0 < [0x1a5c2c] (signed, 32-bit)`.
  Grouped accesses: 3 reported word store records with printed base expression `[sp-24]` and nonnegative word-aligned offsets, including target `[sp-24] +0x8`, value `0x0` (`str` at 0x5a354), target `[sp-24] +0x0`, value `0x0` (`str` at 0x5a360), target `[sp-24] +0x4`, value `0x0` (`str` at 0x5a378).
  Call (`bl` at 0x5a36c): (unknown callee), target 0x20514, r0=`[[sp-24]+4]`, r1=`r1`, r2=`[sp-24]`, r3=`[sp-24]`.
  Recorded r0 = `unknown` (`ldm` at 0x5a380).

## `portUpInternal`

- `portUpInternal`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5b750 to 0x5ba44.
  Established facts:
  Branch (`bge` at 0x5b78c): taken to 0x5b7bc when `r0 >= [0x1a5c2c] (signed, 32-bit)`, falls through to 0x5b790 when `r0 < [0x1a5c2c] (signed, 32-bit)`.
  Call (`bl` at 0x5b808): (unknown callee), target 0x5f7a0, r0=`r0`, r1=`[sp-52]`, r2=`(r1 & 0xff)`, r3=`sp-44`.
  Grouped accesses: 8 reported word load records with printed base expression `[sp-52]` and nonnegative word-aligned offsets, including target `[sp-52] +0x0` (`ldr` at 0x5b85c), target `[sp-52] +0x0` (`ldr` at 0x5b89c), target `[sp-52] +0x0` (`ldr` at 0x5b8cc).
  Branch (`bne` at 0x5b880): taken to 0x5b8e4 when `[sp-22]:1 != 0x3`, falls through to 0x5b884 when `[sp-22]:1 == 0x3`.
  Call (`bl` at 0x5b998): (unknown callee), target 0x5fa88, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x5b9ec): (unknown callee), target 0x5e570, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x5ba38).

## `portDownInternal`

- `portDownInternal`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5ba44 to 0x5bcec.
  Established facts:
  Word load: target `abs 0x1a5c2c` (`ldr` at 0x5ba78).
  Call (`bl` at 0x5bafc): (unknown callee), target 0x5f7a0, r0=`r0`, r1=`[sp-52]`, r2=`(r1 & 0xff)`, r3=`sp-44`.
  Grouped accesses: 7 reported word load records with printed base expression `[sp-52]` and nonnegative word-aligned offsets, including target `[sp-52] +0x0` (`ldr` at 0x5bb50), target `[sp-52] +0x0` (`ldr` at 0x5bb90), target `[sp-52] +0x0` (`ldr` at 0x5bbc0).
  Branch (`bne` at 0x5bbe0): taken to 0x5bc44 when `[sp-22]:1 != 0x5`, falls through to 0x5bbe4 when `[sp-22]:1 == 0x5`.
  Call (`bl` at 0x5bc94): (unknown callee), target 0x600ec, r0=`[sp-20]`, r1=`unknown`, r2=`[sp-22]:1`, r3=`0x1`.
  Recorded r0 = `unknown` (`ldm` at 0x5bce0).

## `portCheckState`

- `portCheckState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5bcec to 0x5bee0.
  Established facts:
  Branch (`bge` at 0x5bd20): taken to 0x5bd50 when `r0 >= [0x1a5c2c] (signed, 32-bit)`, falls through to 0x5bd24 when `r0 < [0x1a5c2c] (signed, 32-bit)`.
  Call (`bl` at 0x5bd9c): (unknown callee), target 0x5f7a0, r0=`r0`, r1=`[sp-52]`, r2=`(r1 & 0xff)`, r3=`sp-44`.
  Grouped accesses: 4 reported word load records with printed base expression `[sp-52]` and nonnegative word-aligned offsets, including target `[sp-52] +0x0` (`ldr` at 0x5bdf0), target `[sp-52] +0x0` (`ldr` at 0x5be24), target `[sp-52] +0x4` (`ldr` at 0x5be4c).
  Byte load: target `(unknown effective address)` (`ldrb` at 0x5be54).
  Call (`bl` at 0x5be88): (unknown callee), target 0x5e744, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x5bed4).

## `portStatus`

- `portStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5bee0 to 0x5c488.
  Established facts:
  Call (`bl` at 0x5bf14): (unknown callee), target 0x5f308, r0=`r0`, r1=`(r1 & 0xff)`, r2=`r2`, r3=`(r1 & 0xff)`.
  Call (`bl` at 0x5bf30): (unknown callee), target 0x5f8b0, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x5f308)`.
  Call (`bl` at 0x5bf6c): (unknown callee), target 0x5b0f4, r0=`[sp-20]`, r1=`[sp-21]:1`, r2=`unknown`, r3=`[sp-21]:1`.
  Call (`bl` at 0x5bfb0): (unknown callee), target 0x5f3c4, r0=`[sp-20]`, r1=`[sp-32]`, r2=`sp-52`, r3=`0x0`.
  Call (`bl` at 0x5c030): (unknown callee), target 0x5acac, r0=`[sp-52]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x5c064): (unknown callee), target 0x605e8, r0=`[sp-20]`, r1=`[sp-32]`, r2=`sp-48`, r3=`sp-36`.
  Call (`bl` at 0x5c07c): (unknown callee), target 0x125b8c, r0=`[sp-48]`, r1=`[[sp-36]]:1`, r2=`[sp-28]`, r3=`[[sp-36]]:1`.
  Grouped accesses: 4 reported halfword load records with printed base expression `[sp-36]` and nonnegative halfword-aligned offsets, including target `[sp-36] +0x4` (`ldrh` at 0x5c0a4), target `[sp-36] +0x4` (`ldrh` at 0x5c13c), target `[sp-36] +0x4` (`ldrh` at 0x5c1cc).
  Branch (`beq` at 0x5c10c): taken to 0x5c124 when `[[sp-36]+1]:1 == 0x0`, falls through to 0x5c110 when `[[sp-36]+1]:1 != 0x0`.
  Grouped accesses: 4 reported word store records with printed base expression `[sp-28]` and nonnegative word-aligned offsets, including target `[sp-28] +0x0`, value `[sp-76]` (`str` at 0x5c218), target `[sp-28] +0x0`, value `[sp-80]` (`str` at 0x5c26c), target `[sp-28] +0x0`, value `[sp-84]` (`str` at 0x5c308).
  Grouped accesses: 2 reported byte load records with printed base expression `[sp-32]` and nonnegative byte-aligned offsets, including target `[sp-32] +0x2` (`ldrb` at 0x5c274), target `[sp-32] +0x3` (`ldrb` at 0x5c2b0).
  Call (`bl` at 0x5c284): (unknown callee), target 0x125b8c, r0=`[sp-20]`, r1=`[[sp-32]+2]:1`, r2=`sp-41`, r3=`[[sp-32]+2]:1`.
  Call (`bl` at 0x5c3a8): (unknown callee), target 0x6dc0c, r0=`[[sp-28]]`, r1=`[sp-28]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x5c40c): (unknown callee), target 0x293f8, r0=`0x16247c`, r1=`unknown`, r2=`unknown`, r3=`[sp-92]`.
  Call (`bl` at 0x5c410): (unknown callee), target 0x6e190, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x5c42c): (unknown callee), target 0x5801c, r0=`ret(0x6e190)`, r1=`[sp-56]`, r2=`0x0`, r3=`sp-60`.
  Recorded r0 = `unknown` (`ldm` at 0x5c468).

## `portQuery`

- `portQuery`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5cf30 to 0x5d0b4.
  Established facts:
  Call (`bl` at 0x5cf64): (unknown callee), target 0x6e140, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x5cf9c): (unknown callee), target 0x5b69c, r0=`[sp-40]`, r1=`sp-52`, r2=`[sp-24]`, r3=`sp-52`.
  Branch (`bne` at 0x5cfc8): taken to 0x5cff4 when `[sp-36] != 0x0`, falls through to 0x5cfcc when `[sp-36] == 0x0`.
  Call (`bl` at 0x5cfd4): (unknown callee), target 0x201fc, r0=`0x50`, r1=`unknown`, r2=`ret(0x6e140)`, r3=`[sp-36]`.
  Grouped accesses: 2 reported word load records with printed base expression `[sp-28]` and nonnegative word-aligned offsets, including target `[sp-28] +0x0` (`ldr` at 0x5cfe4), target `[sp-28] +0x0` (`ldr` at 0x5d01c).
  Call (`bl` at 0x5cfec): (unknown callee), target 0x20634, r0=`[[sp-28]]`, r1=`0x1626d8`, r2=`unknown`, r3=`[sp-28]`.
  Call (`bl` at 0x5d010): (unknown callee), target 0x201fc, r0=`unknown`, r1=`unknown`, r2=`[sp-36]`, r3=`unknown`.
  Byte store: target `[[sp-28]] +0x0`, value `0x0` (`strb` at 0x5d02c).
  Call (`bl` at 0x5d038): (unknown callee), target 0x6e140, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x5d078): (unknown callee), target 0x5c7b4, r0=`[sp-40]`, r1=`[sp-44]`, r2=`sp-48`, r3=`[sp-32]`.
  Call (`bl` at 0x5d080): (unknown callee), target 0x2031c, r0=`[sp-44]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x5d0ac).

## `portLinkQuery`

- `portLinkQuery`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5de7c to 0x5dfd0.
  Established facts:
  Call (`bl` at 0x5deac): (unknown callee), target 0x6e140, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x5dee4): (unknown callee), target 0x5b69c, r0=`[sp-44]`, r1=`sp-52`, r2=`[sp-24]`, r3=`sp-52`.
  Branch (`bne` at 0x5df10): taken to 0x5df18 when `[sp-40] != 0x0`, falls through to 0x5df14 when `[sp-40] == 0x0`.
  Call (`bl` at 0x5df34): (unknown callee), target 0x201fc, r0=`unknown`, r1=`unknown`, r2=`[sp-40]`, r3=`unknown`.
  Word load: target `[sp-28] +0x0` (`ldr` at 0x5df40).
  Byte store: target `[[sp-28]] +0x0`, value `0x0` (`strb` at 0x5df50).
  Call (`bl` at 0x5df5c): (unknown callee), target 0x6e140, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x5df98): (unknown callee), target 0x5d0b4, r0=`[sp-44]`, r1=`[sp-48]`, r2=`[sp-32]`, r3=`[sp-36]`.
  Call (`bl` at 0x5dfa0): (unknown callee), target 0x2031c, r0=`[sp-48]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `ret(0x6e140)` (`ldm` at 0x5dfcc, `bl` at 0x5deac, `bl` at 0x5df5c).

## `portInt`

- `portInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5dfd0 to 0x5e4c8.
  Established facts:
  Call (`bl` at 0x5e008): (unknown callee), target 0x22ca0, r0=`0x1a5c30`, r1=`unknown`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0x5e040): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x1621c4`, r2=`0x583`, r3=`0x0`.
  Call (`bl` at 0x5e064): (unknown callee), target 0x5f924, r0=`sp-52`, r1=`[sp-25]:1`, r2=`0x0`, r3=`sp-52`.
  Call (`bl` at 0x5e084): (unknown callee), target 0x5f924, r0=`sp-48`, r1=`[sp-25]:1`, r2=`0x1`, r3=`sp-48`.
  Call (`bl` at 0x5e0b8): (unknown callee), target 0x6e1b0, r0=`[sp-24]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x5e0c4): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x5e10c): (unknown callee), target 0x203f4, r0=`[sp-64]`, r1=`[sp-56]`, r2=`[sp-60]`, r3=`unknown`.
  Call (`bl` at 0x5e120): (unknown callee), target 0x1265c4, r0=`[sp-24]`, r1=`[sp-25]:1`, r2=`unknown`, r3=`[sp-25]:1`.
  Call (`bl` at 0x5e150): (unknown callee), target 0x22070, r0=`0x1`, r1=`0x1621c4`, r2=`0x590`, r3=`0x0`.
  Call (`bl` at 0x5e164): (unknown callee), target 0x125cf8, r0=`[sp-24]`, r1=`[sp-25]:1`, r2=`[sp-26]:1`, r3=`[sp-25]:1`.
  Call (`bl` at 0x5e170): (unknown callee), target 0x6103c, r0=`[sp-48]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x5e188): (unknown callee), target 0x6103c, r0=`[sp-52]`, r1=`[sp-32]`, r2=`unknown`, r3=`ret(0x6103c)`.
  Call (`bl` at 0x5e1ac): (unknown callee), target 0x6e1b0, r0=`[sp-24]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x5e1b8): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x5e200): (unknown callee), target 0x203f4, r0=`[sp-56]`, r1=`[sp-64]`, r2=`[sp-60]`, r3=`unknown`.
  Word load: target `abs 0x1a58ec` (`ldr` at 0x5e220).
  Call (`bl` at 0x5e22c): (unknown callee), target 0x51c54, r0=`[0x1a58ec]`, r1=`0x162a54`, r2=`ret(0x203f4)`, r3=`[sp-25]:1`.
  Word store: target `abs 0x1a0250`, value `0x1` (`str` at 0x5e238).
  Call (`bl` at 0x5e280): (unknown callee), target 0x5eb20, r0=`[sp-52]`, r1=`[sp-32]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x5e2a8): (unknown callee), target 0x6e1b0, r0=`[sp-52]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x5e2b4): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x5e2fc): (unknown callee), target 0x203f4, r0=`[sp-56]`, r1=`[sp-64]`, r2=`[sp-60]`, r3=`unknown`.
  Call (`bl` at 0x5e308): (unknown callee), target 0x21ce0, r0=`[sp-40]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x5e33c): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x1621c4`, r2=`0x5a4`, r3=`0x0`.
  Halfword load: target `[sp-36] +0x4` (`ldrh` at 0x5e350).
  Call (`bl` at 0x5e38c): (unknown callee), target 0x5eb20, r0=`[sp-48]`, r1=`[sp-36]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x5e3b4): (unknown callee), target 0x6e1b0, r0=`[sp-48]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x5e3c0): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x5e408): (unknown callee), target 0x203f4, r0=`[sp-56]`, r1=`[sp-64]`, r2=`[sp-60]`, r3=`unknown`.
  Call (`bl` at 0x5e414): (unknown callee), target 0x21ce0, r0=`[sp-40]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x5e448): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x1621c4`, r2=`0x5aa`, r3=`0x0`.
  Call (`bl` at 0x5e450): (unknown callee), target 0x2302c, r0=`0x1a5c30`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x5e488): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x1621c4`, r2=`0x5b2`, r3=`0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x5e490).

## `displayStatus`

- `displayStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7114c to 0x71250.
  Established facts:
  Branch (`bne` at 0x7116c): taken to 0x7117c when `[0x1a6114]:1 != 0x0`, falls through to 0x71170 when `[0x1a6114]:1 == 0x0`.
  Byte store: target `abs 0x1a6162`, value `0x2e` (`strb` at 0x711a0).
  Grouped accesses: 3 reported word store records with printed base expression `[sp-20]` and nonnegative word-aligned offsets, including target `[sp-20] +0x0`, value `0x1a6188` (`str` at 0x711d0), target `[sp-20] +0x4`, value `0x1a6199` (`str` at 0x711dc), target `[sp-20] +0x8`, value `0x1a6161` (`str` at 0x711e8).
  Call (`bl` at 0x711fc): (unknown callee), target 0x6e190, r0=`r0`, r1=`r1`, r2=`[sp-20]`, r3=`[0x1a616b]:1`.
  Call (`bl` at 0x7121c): (unknown callee), target 0x125b8c, r0=`ret(0x6e190)`, r1=`[0x1a0270]:1`, r2=`[sp-20]+16`, r3=`[sp-20]+16`.
  Recorded r0 = `unknown` (`ldm` at 0x71230).

## `displayInt`

- `displayInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x71250 to 0x718f8.
  Established facts:
  Branch (`beq` at 0x71290): taken to 0x712b8 when `(r1 & 0xff) == [0x1a0270]:1`, falls through to 0x71294 when `(r1 & 0xff) != [0x1a0270]:1`.
  Call (`bl` at 0x712b4): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x16507c`, r2=`0x2dd`, r3=`0x0`.
  Call (`bl` at 0x712f8): (unknown callee), target 0x5572c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x7130c): (unknown callee), target 0x28230, r0=`0x77`, r1=`unknown`, r2=`unknown`, r3=`ret(0x5572c)`.
  Call (`bl` at 0x71334): (unknown callee), target 0x22070, r0=`0x6`, r1=`0x16507c`, r2=`0x2fd`, r3=`0x0`.
  Call (`bl` at 0x71344): (unknown callee), target 0x706a0, r0=`0x32`, r1=`0x1651c4`, r2=`0x1`, r3=`unknown`.
  Word load: target `abs 0x31a5fc` (`ldr` at 0x71350).
  Call (`mov` at 0x7135c): (unknown callee), target (unknown target), r0=`0x5`, r1=`unknown`, r2=`unknown`, r3=`[0x31a5fc]`.
  Call (`bl` at 0x71378): (unknown callee), target 0x21ce0, r0=`ret(unknown)`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`bl` at 0x7139c): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x16507c`, r2=`0x302`, r3=`0x0`.
  Call (`mov` at 0x713b8): (unknown callee), target (unknown target), r0=`0x1`, r1=`0x5`, r2=`unknown`, r3=`[0x31a600]`.
  Call (`bl` at 0x713d4): (unknown callee), target 0x21ce0, r0=`ret(unknown)`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`bl` at 0x713f8): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x16507c`, r2=`0x306`, r3=`0x0`.
  Call (`bl` at 0x7141c): (unknown callee), target 0x6dc0c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x71430): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x71444): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x71458): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x7146c): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x71480): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x71494): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x714a8): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x714bc): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x714d0): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x714e4): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x714f8): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x7150c): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x71520): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x71534): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x71548): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x7155c): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x71570): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x71584): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x71598): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x715ac): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x715c0): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x715dc): (unknown callee), target 0x28230, r0=`0x78`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x71604): (unknown callee), target 0x22070, r0=`0x6`, r1=`0x16507c`, r2=`0x312`, r3=`0x0`.
  Call (`bl` at 0x71614): (unknown callee), target 0x706a0, r0=`0x32`, r1=`0x165238`, r2=`0x1`, r3=`unknown`.
  Call (`bl` at 0x71628): (unknown callee), target 0x706a0, r0=`0x32`, r1=`0x16524c`, r2=`0x1`, r3=`ret(0x28230)`.
  Call (`bl` at 0x71644): (unknown callee), target 0x21ce0, r0=`ret(0x706a0)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x706a0)`.
  Call (`bl` at 0x71668): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x16507c`, r2=`0x316`, r3=`0x0`.
  Word load: target `abs 0x1a0264` (`ldr` at 0x71670).
  Call (`mov` at 0x71680): (unknown callee), target (unknown target), r0=`0x5`, r1=`0xff`, r2=`unknown`, r3=`[0x1a0264]`.
  Call (`bl` at 0x7169c): (unknown callee), target 0x21ce0, r0=`ret(unknown)`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`bl` at 0x716c0): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x16507c`, r2=`0x31a`, r3=`0x0`.
  Call (`bl` at 0x716d0): (unknown callee), target 0x706a0, r0=`0x32`, r1=`0x165264`, r2=`0x1`, r3=`unknown`.
  Call (`bl` at 0x716ec): (unknown callee), target 0x21ce0, r0=`ret(0x706a0)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x706a0)`.
  Call (`bl` at 0x71710): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x16507c`, r2=`0x31d`, r3=`0x0`.
  Call (`bl` at 0x71738): (unknown callee), target 0x28230, r0=`0x79`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x71760): (unknown callee), target 0x22070, r0=`0x6`, r1=`0x16507c`, r2=`0x328`, r3=`0x0`.
  Call (`bl` at 0x71770): (unknown callee), target 0x706a0, r0=`0x32`, r1=`0x1652a8`, r2=`0x1`, r3=`unknown`.
  Call (`bl` at 0x71784): (unknown callee), target 0x706a0, r0=`0x32`, r1=`0x1652bc`, r2=`0x1`, r3=`ret(0x28230)`.
  Call (`bl` at 0x717a0): (unknown callee), target 0x21ce0, r0=`ret(0x706a0)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x706a0)`.
  Call (`bl` at 0x717c4): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x16507c`, r2=`0x32d`, r3=`0x0`.
  Call (`mov` at 0x717d8): (unknown callee), target (unknown target), r0=`0x5`, r1=`unknown`, r2=`unknown`, r3=`[0x1a025c]`.
  Call (`bl` at 0x717f4): (unknown callee), target 0x21ce0, r0=`ret(unknown)`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`bl` at 0x71818): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x16507c`, r2=`0x331`, r3=`0x0`.
  Call (`bl` at 0x71828): (unknown callee), target 0x706a0, r0=`0x32`, r1=`0x1652d8`, r2=`0x1`, r3=`unknown`.
  Call (`bl` at 0x71844): (unknown callee), target 0x21ce0, r0=`ret(0x706a0)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x706a0)`.
  Call (`bl` at 0x71868): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x16507c`, r2=`0x334`, r3=`0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x71870).

## `syscom_process_inbound_data`

- `syscom_process_inbound_data`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8e64 to 0xb9050.
  Established facts:
  Call (`bl` at 0xb8e98): (unknown callee), target 0x958d4, r0=`sp-84`, r1=`sp-80`, r2=`0x0`, r3=`sp-84`.
  Call (`bl` at 0xb8eb8): (unknown callee), target 0x95b5c, r0=`sp-84`, r1=`sp-80`, r2=`[sp-36]`, r3=`[sp-40]`.
  Call (`bl` at 0xb8ec8): (unknown callee), target 0xb5c90, r0=`[sp-28]`, r1=`[sp-32]`, r2=`0x1`, r3=`unknown`.
  Call (`bl` at 0xb8ef0): (unknown callee), target 0xb5db0, r0=`[sp-28]`, r1=`[sp-32]`, r2=`0x16d53c`, r3=`[sp-36]`.
  Word load: target `abs 0x31b5a0` (`ldr` at 0xb8f04).
  Call (`bl` at 0xb8f5c): (unknown callee), target 0xb80fc, r0=`[sp-32]`, r1=`0x4`, r2=`sp-24`, r3=`unknown`.
  Word load: target `abs 0x31b5bc` (`ldr` at 0xb8f84).
  Call (`bl` at 0xb8f9c): (unknown callee), target 0xfd284, r0=`sp-48`, r1=`[sp-28]`, r2=`0x0`, r3=`[sp-84]`.
  Branch (`beq` at 0xb8fb0): taken to 0xb903c when `ret(0xfd284) == 0x0`, falls through to 0xb8fb4 when `ret(0xfd284) != 0x0` (source `bl` at 0xb8f9c).
  Call (`bl` at 0xb8fc4): (unknown callee), target 0x6e1b0, r0=`[sp-28]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0xb8fd0): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xb9018): (unknown callee), target 0x203f4, r0=`[sp-100]`, r1=`[sp-92]`, r2=`[sp-96]`, r3=`unknown`.
  Call (`bl` at 0xb9024): (unknown callee), target 0x219a4, r0=`[sp-88]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xb9038): (unknown callee), target 0x29908, r0=`0x16d54c`, r1=`ret(0x203f4)`, r2=`[sp-32]`, r3=`ret(0x219a4)`.
  Recorded r0 = `unknown` (`ldm` at 0xb9040).

## `syscom_process_inbound_packet`

- `syscom_process_inbound_packet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb9a24 to 0xb9b10.
  Established facts:
  Byte load: target `r2 +0x0` (`ldrb` at 0xb9a48).
  Word store: target `abs 0x1b0c9c`, value `[0x1b0c9c]+1` (`str` at 0xb9a84).
  Call (`bl` at 0xb9ab0): (unknown callee), target 0xb9050, r0=`[sp-20]`, r1=`[sp-24]`, r2=`[sp-28]`, r3=`[sp-32]`.
  Word store: target `abs 0x1b0ca4`, value `[0x1b0ca4]+1` (`str` at 0xb9ac8).
  Call (`bl` at 0xb9af4): (unknown callee), target 0xb94e8, r0=`[sp-20]`, r1=`[sp-24]`, r2=`[sp-28]`, r3=`[sp-32]`.
  Recorded r0 = `unknown` (`ldm` at 0xb9afc).

## `syscom_process_outbound_packet`

- `syscom_process_outbound_packet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbb4f8 to 0xbb578.
  Established facts:
  Byte load: target `r0 +0x16` (`ldrb` at 0xbb510).
  Branch (`beq` at 0xbb520): taken to 0xbb568 when `[r0+22]:1 == 0x1`, falls through to 0xbb524 when `[r0+22]:1 != 0x1`.
  Branch (`beq` at 0xbb548): taken to 0xbb55c when `[r0+22]:1 == 0x2`, falls through to 0xbb54c when `[r0+22]:1 != 0x2`.
  Call (`bl` at 0xbb554): (unknown callee), target 0xb9fa8, r0=`r0`, r1=`r1`, r2=`r2`, r3=`[r0+22]:1`.
  Call (`bl` at 0xbb560): (unknown callee), target 0xba514, r0=`r0`, r1=`r1`, r2=`r2`, r3=`[r0+22]:1`.
  Call (`bl` at 0xbb56c): (unknown callee), target 0xbacf8, r0=`r0`, r1=`r1`, r2=`r2`, r3=`[r0+22]:1`.
  Recorded r0 = `unknown` (`ldm` at 0xbb574).

## `SysComCfg`

- `SysComCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf0b40 to 0xf0e48.
  Established facts:
  Grouped accesses: 4 reported word load records with absolute target `0x31b5b8`, including target `abs 0x31b5b8` (`ldr` at 0xf0b84), target `abs 0x31b5b8` (`ldr` at 0xf0c0c), target `abs 0x31b5b8` (`ldr` at 0xf0ccc).
  Call (`bl` at 0xf0bc8): (unknown callee), target 0x201fc, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Word load: target `r0 +0x0` (`ldr` at 0xf0bd4).
  Call (`bl` at 0xf0c34): (unknown callee), target 0x6e1b0, r0=`unknown`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0xf0c40): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf0c88): (unknown callee), target 0x203f4, r0=`[sp-52]`, r1=`[sp-44]`, r2=`[sp-48]`, r3=`unknown`.
  Call (`bl` at 0xf0ca0): (unknown callee), target 0x205bc, r0=`[sp-28]`, r1=`0x185084`, r2=`ret(0x203f4)`, r3=`[sp-36]`.
  Call (`bl` at 0xf0ca8): (unknown callee), target 0x2031c, r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0xf0d10): taken to 0xf0d24 when `[sp-56] == 0x0`, falls through to 0xf0d14 when `[sp-56] != 0x0`.
  Call (`bl` at 0xf0d2c): (unknown callee), target 0x20634, r0=`[sp-28]`, r1=`0x185094`, r2=`unknown`, r3=`[sp-56]`.
  Call (`bl` at 0xf0d3c): (unknown callee), target 0x20634, r0=`[sp-28]`, r1=`0x1850a4`, r2=`unknown`, r3=`[sp-56]`.
  Call (`bl` at 0xf0d44): (unknown callee), target 0x2031c, r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf0dc8): (unknown callee), target 0x20634, r0=`[sp-28]`, r1=`0x1850b4`, r2=`unknown`, r3=`[sp-60]`.
  Call (`bl` at 0xf0dd8): (unknown callee), target 0x20634, r0=`[sp-28]`, r1=`0x1850c4`, r2=`unknown`, r3=`[sp-60]`.
  Call (`bl` at 0xf0de4): (unknown callee), target 0x203c4, r0=`[sp-28]`, r1=`0x18506c`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf0dec): (unknown callee), target 0x2031c, r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0xf0e28).
