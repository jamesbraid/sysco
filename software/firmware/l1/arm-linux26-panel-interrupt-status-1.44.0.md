# ARM Linux 2.6 panel and serial status routines in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-panel-interrupt-status-facts.txt) retain all tool sections.

The notation `ret(ADDR)` identifies a call-result expression without distinguishing
individual call sites to that target. Each return line records machine-register
state from the analysis rather than a C return type.


Groups summarize static access records by printed shape. Instruction-address order can include different control-flow paths.

## `serIntGetStatus`

- `serIntGetStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10d408 to 0x10d500.
  Established facts:
  Word load: target `abs 0x314f2c` (`ldr` at 0x10d42c).
  Branch (`blt` at 0x10d434): taken to 0x10d45c when `(r0 & 0xff) < [0x314f2c] (signed, 32-bit)`, falls through to 0x10d438 when `(r0 & 0xff) >= [0x314f2c] (signed, 32-bit)`.
  Call (`bl` at 0x10d458): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x18ce30`, r2=`0x1d3`, r3=`0x0`.
  Call (`bl` at 0x10d4b8): (unknown callee), target 0x22a68, r0=`0x1`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Halfword store: target `(unknown effective address)`, value `0x0` (`strh` at 0x10d4d4).
  Recorded r0 = `[sp-32]` (`ldm` at 0x10d4e8).

## `serIntClearStatus`

- `serIntClearStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10d500 to 0x10d5c8.
  Established facts:
  Branch (`blt` at 0x10d528): taken to 0x10d550 when `(r0 & 0xff) < [0x314f2c] (signed, 32-bit)`, falls through to 0x10d52c when `(r0 & 0xff) >= [0x314f2c] (signed, 32-bit)`.
  Call (`bl` at 0x10d54c): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x18ce30`, r2=`0x1ed`, r3=`0x0`.
  Word load: target `abs 0x314f28` (`ldr` at 0x10d568).
  Halfword store: target `(unknown effective address)`, value `unknown` (`strh` at 0x10d5a4).
  Recorded r0 = `unknown` (`ldm` at 0x10d5ac).

## `serIntClearStatusBits`

- `serIntClearStatusBits`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10d5c8 to 0x10d6f4.
  Established facts:
  Call (`bl` at 0x10d61c): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x18ce30`, r2=`0x1fc`, r3=`0x0`.
  Word load: target `abs 0x314f28` (`ldr` at 0x10d638).
  Grouped accesses: 2 reported byte load records with printed base expression `[sp-24]` and nonnegative byte-aligned offsets, including target `[sp-24] +0x2` (`ldrb` at 0x10d64c), target `[sp-24] +0x2` (`ldrb` at 0x10d6b8).
  Call (`bl` at 0x10d694): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x18ce30`, r2=`0x203`, r3=`0x0`.
  Word load: target `abs 0x314f34` (`ldr` at 0x10d6a8).
  Halfword store: target `(unknown effective address)`, value `unknown` (`strh` at 0x10d6cc).
  Recorded r0 = `unknown` (`ldm` at 0x10d6d4).

## `displayProbe`

- `displayProbe`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x70214 to 0x703e8.
  Established facts:
  Grouped accesses: 2 reported byte load records with absolute target `0x1a0270`, including target `abs 0x1a0270` (`ldrb` at 0x70230), target `abs 0x1a0270` (`ldrb` at 0x702f8).
  Branch (`beq` at 0x70238): taken to 0x70258 when `[0x1a0270]:1 == 0xff`, falls through to 0x7023c when `[0x1a0270]:1 != 0xff` (source `ldrb` at 0x70230).
  Call (`bl` at 0x70254): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x16507c`, r2=`0x80`, r3=`0x0`.
  Call (`bl` at 0x70258): (unknown callee), target 0x6e190, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x70274): (unknown callee), target 0x125b8c, r0=`ret(0x6e190)`, r1=`[sp-17]:1`, r2=`sp-29`, r3=`ret(0x6e190)`.
  Call (`bl` at 0x70290): (unknown callee), target 0x21ce0, r0=`ret(0x125b8c)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x125b8c)`.
  Call (`bl` at 0x702b4): (unknown callee), target 0x22070, r0=`0x1`, r1=`0x16507c`, r2=`0x86`, r3=`0x0`.
  Call (`bl` at 0x702b8): (unknown callee), target 0x6e190, r0=`ret(0x22070)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x702cc): (unknown callee), target 0x124cb8, r0=`ret(0x6e190)`, r1=`[sp-17]:1`, r2=`[sp-17]:1`, r3=`ret(0x6e190)`.
  Byte store: target `abs 0x1a6115`, value `0x1` (`strb` at 0x702e8).
  Call (`bl` at 0x702ec): (unknown callee), target 0x6e190, r0=`ret(0x125b8c)`, r1=`unknown`, r2=`0x1a6115`, r3=`0x1`.
  Call (`bl` at 0x70304): (unknown callee), target 0x126630, r0=`ret(0x6e190)`, r1=`[0x1a0270]:1`, r2=`ret(0x6e190)`, r3=`[0x1a0270]:1`.
  Grouped accesses: 2 reported byte store records with absolute target `0x1a6169`, including target `abs 0x1a6169`, value `0x76` (`strb` at 0x70338), target `abs 0x1a6169`, value `unknown` (`strb` at 0x70370).
  Call (`mov` at 0x70360): (unknown callee), target (unknown target), r0=`0x1a6168`, r1=`unknown`, r2=`0x1a6168`, r3=`[0x1a04c4]`.
  Call (`bl` at 0x70378): (unknown callee), target 0x12a9d4, r0=`0x1a6168`, r1=`unknown`, r2=`unknown`, r3=`0x1a6168`.
  Branch (`bls` at 0x70388): taken to 0x703c0 when `[0x1a616a]:1 <= 0x10 (unsigned, 32-bit)`, falls through to 0x7038c when `[0x1a616a]:1 > 0x10 (unsigned, 32-bit)`.
  Call (`bl` at 0x703b0): (unknown callee), target 0x22070, r0=`0x1`, r1=`0x16507c`, r2=`0xa3`, r3=`0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x703c4).

## `displayInit`

- `displayInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x70560 to 0x70680.
  Established facts:
  Branch (`bne` at 0x7057c): taken to 0x70584 when `[0x1a0270]:1 != 0xff`, falls through to 0x70580 when `[0x1a0270]:1 == 0xff`.
  Branch (`beq` at 0x70590): taken to 0x705b0 when `[0x1a6114]:1 == 0x0`, falls through to 0x70594 when `[0x1a6114]:1 != 0x0`.
  Call (`bl` at 0x705ac): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x16507c`, r2=`0xea`, r3=`0x0`.
  Byte store: target `abs 0x1a6114`, value `0x1` (`strb` at 0x705b8).
  Call (`bl` at 0x705c0): (unknown callee), target 0x70f1c, r0=`0x1`, r1=`unknown`, r2=`0x1a6114`, r3=`0x1`.
  Call (`bl` at 0x705dc): (unknown callee), target 0x21ce0, r0=`ret(0x70f1c)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x70f1c)`.
  Call (`bl` at 0x70600): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x16507c`, r2=`0x105`, r3=`0x0`.
  Call (`bl` at 0x70610): (unknown callee), target 0x706a0, r0=`0x32`, r1=`0x16510c`, r2=`0x0`, r3=`unknown`.
  Call (`bl` at 0x7062c): (unknown callee), target 0x21ce0, r0=`ret(0x706a0)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x706a0)`.
  Call (`bl` at 0x70650): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x16507c`, r2=`0x115`, r3=`0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x70658).

## `displayPresent`

- `displayPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x70680 to 0x706a0.
  Established facts:
  Byte load: target `abs 0x1a6115` (`ldrb` at 0x70690).
  Recorded r0 = `[0x1a6115]:1` (`ldm` at 0x70698).

## `displayString`

- `displayString`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x706a0 to 0x7091c.
  Established facts:
  Branch (`bne` at 0x706c8): taken to 0x706d8 when `[0x1a6114]:1 != 0x0`, falls through to 0x706cc when `[0x1a6114]:1 == 0x0`.
  Call (`bl` at 0x70720): (unknown callee), target 0x20634, r0=`0x1a0274`, r1=`r1`, r2=`r2`, r3=`r2`.
  Call (`bl` at 0x7073c): (unknown callee), target 0x70b70, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r2`.
  Call (`bl` at 0x7074c): (unknown callee), target 0x2031c, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`0x1a6199`.
  Call (`bl` at 0x70788): (unknown callee), target 0x20634, r0=`0x1a0274`, r1=`[sp-24]`, r2=`ret(0x2031c)`, r3=`unknown`.
  Call (`bl` at 0x7080c): (unknown callee), target 0x205ec, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Byte store: target `(unknown effective address)`, value `0x0` (`strb` at 0x7088c).
  Word load: target `abs 0x1a6180` (`ldr` at 0x70898).
  Word load: target `abs 0x1a6170` (`ldr` at 0x7089c).
  Call (`mov` at 0x708ac): (unknown callee), target (unknown target), r0=`[0x1a6170]`, r1=`[sp-32]`, r2=`[sp-36]`, r3=`[0x1a6180]`.
  Branch (`bne` at 0x708bc): taken to 0x708e0 when `[0x1a61e2]:1 != 0x0`, falls through to 0x708c0 when `[0x1a61e2]:1 == 0x0`.
  Call (`mov` at 0x708d4): (unknown callee), target (unknown target), r0=`[sp-20]`, r1=`[sp-36]`, r2=`unknown`, r3=`[0x1a6178]`.
  Recorded r0 = `unknown` (`ldm` at 0x708f8).

## `displayChar`

- `displayChar`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7091c to 0x70a8c.
  Established facts:
  Grouped accesses: 2 reported word load records with absolute target `0x1a617c`, including target `abs 0x1a617c` (`ldr` at 0x70960), target `abs 0x1a617c` (`ldr` at 0x70a38).
  Branch (`bne` at 0x70968): taken to 0x70978 when `[0x1a617c] != 0x0`, falls through to 0x7096c when `[0x1a617c] == 0x0` (source `ldr` at 0x70960).
  Branch (`bhi` at 0x70988): taken to 0x70998 when `[0x1a616a]:1 > (r1 & 0xff) (unsigned, 32-bit)`, falls through to 0x7098c when `[0x1a616a]:1 <= (r1 & 0xff) (unsigned, 32-bit)`.
  Call (`bl` at 0x70998): (unknown callee), target 0x205ec, r0=`r0`, r1=`r1`, r2=`[0x1a616a]:1`, r3=`(r1 & 0xff)`.
  Grouped accesses: 2 reported byte store records with printed base expression `[sp-21]:1` and nonnegative byte-aligned offsets, including target `[sp-21]:1 +0x1a6188`, value `([sp-22]:1 & 0xff)` (`strb` at 0x709e4), target `[sp-21]:1 +0x1a6199`, value `([sp-22]:1 & 0xff)` (`strb` at 0x709f8).
  Call (`mov` at 0x70a18): (unknown callee), target (unknown target), r0=`sp-23`, r1=`sp-22`, r2=`0x1`, r3=`[0x1a6180]`.
  Call (`mov` at 0x70a4c): (unknown callee), target (unknown target), r0=`[sp-20]`, r1=`[sp-21]:1`, r2=`[sp-23]:1`, r3=`[0x1a617c]`.
  Recorded r0 = `unknown` (`ldm` at 0x70a70).

## `displayHeatbeat`

- `displayHeatbeat`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x70a8c to 0x70b70.
  Established facts:
  Branch (`beq` at 0x70aa8): taken to 0x70ac0 when `[0x1a617c] == 0x0`, falls through to 0x70aac when `[0x1a617c] != 0x0`.
  Branch (`bne` at 0x70ab8): taken to 0x70ac0 when `[0x1a61e2]:1 != 0x0`, falls through to 0x70abc when `[0x1a61e2]:1 == 0x0`.
  Byte store: target `abs 0x1a6160`, value `([0x1a6160]:1+1 & 0xff)` (`strb` at 0x70b20).
  Call (`mov` at 0x70b44): (unknown callee), target (unknown target), r0=`0x31`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x70b54).

## `displayLock`

- `displayLock`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x70b70 to 0x70b90.
  Established facts:
  Byte store: target `abs 0x1a6116`, value `0x1` (`strb` at 0x70b84).
  Recorded r0 = `r0` (`ldm` at 0x70b88).

## `displayUnlock`

- `displayUnlock`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x70b90 to 0x70bc4.
  Established facts:
  Byte store: target `abs 0x1a6116`, value `0x0` (`strb` at 0x70ba4).
  Call (`bl` at 0x70bb4): (unknown callee), target 0x706a0, r0=`0x32`, r1=`0x1a0274`, r2=`0x0`, r3=`0x1a6116`.
  Recorded r0 = `ret(0x706a0)` (`ldm` at 0x70bb8, `bl` at 0x70bb4).

## `displayLedOn`

- `displayLedOn`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x70bc4 to 0x70cf0.
  Established facts:
  Branch (`bne` at 0x70be4): taken to 0x70bf4 when `[0x1a6114]:1 != 0x0`, falls through to 0x70be8 when `[0x1a6114]:1 == 0x0`.
  Branch (`beq` at 0x70c04): taken to 0x70c50 when `r0 == 0x1`, falls through to 0x70c08 when `r0 != 0x1`.
  Call (`bl` at 0x70c24): (unknown callee), target 0x6e190, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r0`.
  Grouped accesses: 3 reported byte load records with absolute target `0x1a0270`, including target `abs 0x1a0270` (`ldrb` at 0x70c30), target `abs 0x1a0270` (`ldrb` at 0x70c5c), target `abs 0x1a0270` (`ldrb` at 0x70c88).
  Call (`bl` at 0x70c40): (unknown callee), target 0x125a0c, r0=`ret(0x6e190)`, r1=`[0x1a0270]:1`, r2=`0x40`, r3=`[0x1a0270]:1`.
  Call (`bl` at 0x70c50): (unknown callee), target 0x6e190, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r0`.
  Call (`bl` at 0x70c6c): (unknown callee), target 0x125a0c, r0=`ret(0x6e190)`, r1=`[0x1a0270]:1`, r2=`0x10`, r3=`[0x1a0270]:1`.
  Call (`bl` at 0x70c7c): (unknown callee), target 0x6e190, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r0`.
  Call (`bl` at 0x70c98): (unknown callee), target 0x125a0c, r0=`ret(0x6e190)`, r1=`[0x1a0270]:1`, r2=`0x20`, r3=`[0x1a0270]:1`.
  Call (`bl` at 0x70cc8): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x16507c`, r2=`0x217`, r3=`0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x70cd4).

## `displayLedOff`

- `displayLedOff`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x70cf0 to 0x70e18.
  Established facts:
  Branch (`bne` at 0x70d10): taken to 0x70d20 when `[0x1a6114]:1 != 0x0`, falls through to 0x70d14 when `[0x1a6114]:1 == 0x0`.
  Branch (`blo` at 0x70d3c): taken to 0x70d50 when `r0 < 0x1 (unsigned, 32-bit)`, falls through to 0x70d40 when `r0 >= 0x1 (unsigned, 32-bit)`.
  Call (`bl` at 0x70d50): (unknown callee), target 0x6e190, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r0`.
  Grouped accesses: 3 reported byte load records with absolute target `0x1a0270`, including target `abs 0x1a0270` (`ldrb` at 0x70d5c), target `abs 0x1a0270` (`ldrb` at 0x70d88), target `abs 0x1a0270` (`ldrb` at 0x70db4).
  Call (`bl` at 0x70d6c): (unknown callee), target 0x1259bc, r0=`ret(0x6e190)`, r1=`[0x1a0270]:1`, r2=`0x40`, r3=`[0x1a0270]:1`.
  Call (`bl` at 0x70d7c): (unknown callee), target 0x6e190, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r0`.
  Call (`bl` at 0x70d98): (unknown callee), target 0x1259bc, r0=`ret(0x6e190)`, r1=`[0x1a0270]:1`, r2=`0x10`, r3=`[0x1a0270]:1`.
  Call (`bl` at 0x70da8): (unknown callee), target 0x6e190, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r0`.
  Call (`bl` at 0x70dc4): (unknown callee), target 0x1259bc, r0=`ret(0x6e190)`, r1=`[0x1a0270]:1`, r2=`0x20`, r3=`[0x1a0270]:1`.
  Call (`bl` at 0x70df4): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x16507c`, r2=`0x234`, r3=`0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x70e00).

## `displayLedStatus`

- `displayLedStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x70e18 to 0x70f1c.
  Established facts:
  Grouped accesses: 2 reported byte load records with absolute target `0x1a0270`, including target `abs 0x1a0270` (`ldrb` at 0x70e30), target `abs 0x1a0270` (`ldrb` at 0x70ecc).
  Branch (`bne` at 0x70e38): taken to 0x70e48 when `[0x1a0270]:1 != 0xff`, falls through to 0x70e3c when `[0x1a0270]:1 == 0xff`.
  Branch (`beq` at 0x70e70): taken to 0x70e90 when `r0 == 0x2`, falls through to 0x70e74 when `r0 != 0x2`.
  Call (`bl` at 0x70ebc): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x16507c`, r2=`0x257`, r3=`0x0`.
  Call (`bl` at 0x70ec0): (unknown callee), target 0x6e190, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x70ed8): (unknown callee), target 0x125c20, r0=`ret(0x6e190)`, r1=`[0x1a0270]:1`, r2=`ret(0x6e190)`, r3=`[0x1a0270]:1`.
  Recorded r0 = `unknown` (`ldm` at 0x70f04).

## `displayReset`

- `displayReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x70f1c to 0x7114c.
  Established facts:
  Branch (`bne` at 0x70f3c): taken to 0x70f4c when `[0x1a6114]:1 != 0x0`, falls through to 0x70f40 when `[0x1a6114]:1 == 0x0`.
  Call (`bl` at 0x70f4c): (unknown callee), target 0x6e190, r0=`r0`, r1=`r1`, r2=`r2`, r3=`[0x1a6114]:1`.
  Call (`bl` at 0x70f64): (unknown callee), target 0x125c20, r0=`ret(0x6e190)`, r1=`[0x1a0270]:1`, r2=`ret(0x6e190)`, r3=`[0x1a0270]:1`.
  Call (`bl` at 0x70f74): (unknown callee), target 0x6e190, r0=`ret(0x125c20)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x70f94): (unknown callee), target 0x125a58, r0=`ret(0x6e190)`, r1=`[0x1a0270]:1`, r2=`0x70`, r3=`0x0`.
  Grouped accesses: 2 reported byte store records with absolute target `0x1a61e2`, including target `abs 0x1a61e2`, value `0x1` (`strb` at 0x70fc0), target `abs 0x1a61e2`, value `0x0` (`strb` at 0x71068).
  Call (`bl` at 0x70fc4): (unknown callee), target 0x6e190, r0=`ret(0x125a58)`, r1=`unknown`, r2=`0x1a61e2`, r3=`0x1`.
  Call (`mov` at 0x71000): (unknown callee), target (unknown target), r0=`ret(0x6e190)`, r1=`[0x1a0270]:1`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x7101c): (unknown callee), target 0x21ce0, r0=`ret(unknown)`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`bl` at 0x71040): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x16507c`, r2=`0x27e`, r3=`0x0`.
  Call (`mov` at 0x7105c): (unknown callee), target (unknown target), r0=`0x1a61e3`, r1=`0x16505c`, r2=`0x4`, r3=`[0x1a6180]`.
  Call (`bl` at 0x7106c): (unknown callee), target 0x70b90, r0=`ret(unknown)`, r1=`unknown`, r2=`0x1a61e2`, r3=`0x0`.
  Call (`bl` at 0x71070): (unknown callee), target 0x6e190, r0=`ret(0x70b90)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x71094): (unknown callee), target 0x125a58, r0=`ret(0x6e190)`, r1=`[0x1a0270]:1`, r2=`0x70`, r3=`[sp-21]:1`.
  Branch (`beq` at 0x710c4): taken to 0x710f8 when `[0x1a6188]:1 == 0x0`, falls through to 0x710c8 when `[0x1a6188]:1 != 0x0`.
  Call (`bl` at 0x710d4): (unknown callee), target 0x706a0, r0=`0x31`, r1=`0x1a6188`, r2=`0x0`, r3=`[0x1a6188]:1`.
  Call (`bl` at 0x71104): (unknown callee), target 0x706a0, r0=`0x32`, r1=`0x165150`, r2=`0x1`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x71118).
