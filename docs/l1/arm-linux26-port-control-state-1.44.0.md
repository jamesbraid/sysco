# ARM Linux 2.6 port control and state routines in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-port-control-state-facts.txt) retain all tool sections.

The notation `ret(ADDR)` identifies a call-result expression without distinguishing
individual call sites to that target. Each return line records machine-register
state from the analysis rather than a C return type.


Groups summarize static access records by printed shape. Instruction-address order can include different control-flow paths.

## `portConnInit`

- `portConnInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5a960 to 0x5a9a4.
  Established facts:
  Word store: target `abs 0x1a5c84`, value `r0` (`str` at 0x5a980).
  Word store: target `abs 0x1a5c88`, value `[sp-24]` (`str` at 0x5a98c).
  Recorded r0 = `[sp-24]` (`ldm` at 0x5a998).

## `portEnable`

- `portEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5a9a4 to 0x5ab28.
  Established facts:
  Branch (`bge` at 0x5a9d0): taken to 0x5ab0c when `r0 >= [0x1a5c2c] (signed, 32-bit)`, falls through to 0x5a9d4 when `r0 < [0x1a5c2c] (signed, 32-bit)`.
  Branch (`blt` at 0x5aa38): taken to 0x5aad0 when `(r1 & 0xff) < [[sp-28]+8] (signed, 32-bit)`, falls through to 0x5aa3c when `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)`.
  Call (`bl` at 0x5aa4c): (unknown callee), target 0x6e1b0, r0=`r0`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x5aa58): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x5aaa0): (unknown callee), target 0x203f4, r0=`[sp-40]`, r1=`[sp-32]`, r2=`[sp-36]`, r3=`unknown`.
  Call (`bl` at 0x5aacc): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x1621c4`, r2=`0x161`, r3=`0x0`.
  Byte store: target `(unknown effective address)`, value `0x2` (`strb` at 0x5ab08).
  Recorded r0 = `unknown` (`ldm` at 0x5ab10).

## `portDisable`

- `portDisable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5ab28 to 0x5acac.
  Established facts:
  Branch (`bge` at 0x5ab54): taken to 0x5ac90 when `r0 >= [0x1a5c2c] (signed, 32-bit)`, falls through to 0x5ab58 when `r0 < [0x1a5c2c] (signed, 32-bit)`.
  Grouped accesses: 3 reported word load records with printed base expression `[sp-28]` and nonnegative word-aligned offsets, including target `[sp-28] +0x8` (`ldr` at 0x5abb4), target `[sp-28] +0x4` (`ldr` at 0x5ac60), target `[sp-28] +0x4` (`ldr` at 0x5ac80).
  Call (`bl` at 0x5abd0): (unknown callee), target 0x6e1b0, r0=`r0`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x5abdc): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x5ac24): (unknown callee), target 0x203f4, r0=`[sp-40]`, r1=`[sp-32]`, r2=`[sp-36]`, r3=`unknown`.
  Call (`bl` at 0x5ac50): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x1621c4`, r2=`0x16f`, r3=`0x0`.
  Byte store: target `(unknown effective address)`, value `0x1` (`strb` at 0x5ac8c).
  Recorded r0 = `unknown` (`ldm` at 0x5ac94).

## `portIsEnabled`

- `portIsEnabled`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5acac to 0x5ae2c.
  Established facts:
  Word load: target `abs 0x1a5c2c` (`ldr` at 0x5acd0).
  Branch (`bge` at 0x5acd8): taken to 0x5ad08 when `r0 >= [0x1a5c2c] (signed, 32-bit)`, falls through to 0x5acdc when `r0 < [0x1a5c2c] (signed, 32-bit)`.
  Grouped accesses: 2 reported word load records with printed base expression `[sp-28]` and nonnegative word-aligned offsets, including target `[sp-28] +0x8` (`ldr` at 0x5ad40), target `[sp-28] +0x4` (`ldr` at 0x5adec).
  Call (`bl` at 0x5ad5c): (unknown callee), target 0x6e1b0, r0=`r0`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x5ad68): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x5adb0): (unknown callee), target 0x203f4, r0=`[sp-40]`, r1=`[sp-32]`, r2=`[sp-36]`, r3=`unknown`.
  Call (`bl` at 0x5addc): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x1621c4`, r2=`0x17d`, r3=`0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x5ae10).

## `portPresent`

- `portPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5ae2c to 0x5af90.
  Established facts:
  Branch (`bge` at 0x5ae58): taken to 0x5af74 when `r0 >= [0x1a5c2c] (signed, 32-bit)`, falls through to 0x5ae5c when `r0 < [0x1a5c2c] (signed, 32-bit)`.
  Branch (`blt` at 0x5aec0): taken to 0x5af58 when `(r1 & 0xff) < [[sp-28]+8] (signed, 32-bit)`, falls through to 0x5aec4 when `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)`.
  Call (`bl` at 0x5aed4): (unknown callee), target 0x6e1b0, r0=`r0`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x5aee0): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x5af28): (unknown callee), target 0x203f4, r0=`[sp-40]`, r1=`[sp-32]`, r2=`[sp-36]`, r3=`unknown`.
  Call (`bl` at 0x5af54): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x1621c4`, r2=`0x18e`, r3=`0x0`.
  Byte store: target `(unknown effective address)`, value `0x2` (`strb` at 0x5af70).
  Recorded r0 = `unknown` (`ldm` at 0x5af78).

## `portNotPresent`

- `portNotPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5af90 to 0x5b0f4.
  Established facts:
  Word load: target `abs 0x1a5c2c` (`ldr` at 0x5afb4).
  Branch (`blt` at 0x5b024): taken to 0x5b0bc when `(r1 & 0xff) < [[sp-28]+8] (signed, 32-bit)`, falls through to 0x5b028 when `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)`.
  Call (`bl` at 0x5b038): (unknown callee), target 0x6e1b0, r0=`r0`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x5b044): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x5b08c): (unknown callee), target 0x203f4, r0=`[sp-40]`, r1=`[sp-32]`, r2=`[sp-36]`, r3=`unknown`.
  Call (`bl` at 0x5b0b8): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x1621c4`, r2=`0x19b`, r3=`0x0`.
  Byte store: target `(unknown effective address)`, value `0x0` (`strb` at 0x5b0d4).
  Recorded r0 = `unknown` (`ldm` at 0x5b0dc).

## `portIsPresent`

- `portIsPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5b0f4 to 0x5b270.
  Established facts:
  Branch (`bge` at 0x5b120): taken to 0x5b150 when `r0 >= [0x1a5c2c] (signed, 32-bit)`, falls through to 0x5b124 when `r0 < [0x1a5c2c] (signed, 32-bit)`.
  Grouped accesses: 2 reported word load records with absolute target `0x1a5c28`, including target `abs 0x1a5c28` (`ldr` at 0x5b13c), target `abs 0x1a5c28` (`ldr` at 0x5b174).
  Call (`bl` at 0x5b1a4): (unknown callee), target 0x6e1b0, r0=`r0`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x5b1b0): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x5b1f8): (unknown callee), target 0x203f4, r0=`[sp-40]`, r1=`[sp-32]`, r2=`[sp-36]`, r3=`unknown`.
  Call (`bl` at 0x5b224): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x1621c4`, r2=`0x1a8`, r3=`0x0`.
  Byte load: target `(unknown effective address)` (`ldrb` at 0x5b23c).
  Recorded r0 = `unknown` (`ldm` at 0x5b258).

## `portIsUp`

- `portIsUp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5b270 to 0x5b3f0.
  Established facts:
  Word load: target `abs 0x1a5c2c` (`ldr` at 0x5b294).
  Branch (`blt` at 0x5b30c): taken to 0x5b3a4 when `(r1 & 0xff) < [[sp-28]+8] (signed, 32-bit)`, falls through to 0x5b310 when `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)`.
  Call (`bl` at 0x5b320): (unknown callee), target 0x6e1b0, r0=`r0`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x5b32c): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x5b374): (unknown callee), target 0x203f4, r0=`[sp-40]`, r1=`[sp-32]`, r2=`[sp-36]`, r3=`unknown`.
  Call (`bl` at 0x5b3a0): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x1621c4`, r2=`0x1b9`, r3=`0x0`.
  Byte load: target `(unknown effective address)` (`ldrb` at 0x5b3b8).
  Recorded r0 = `unknown` (`ldm` at 0x5b3d4).

## `portGetCfg`

- `portGetCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5b69c to 0x5b750.
  Established facts:
  Branch (`beq` at 0x5b6e4): taken to 0x5b714 when `[0x1a5c2c] == 0x0`, falls through to 0x5b6e8 when `[0x1a5c2c] != 0x0`.
  Grouped accesses: 2 reported word load records with printed base expression `[sp-28]` and nonnegative word-aligned offsets, including target `[sp-28] +0x0` (`ldr` at 0x5b728), target `[sp-28] +0x8` (`ldr` at 0x5b734).
  Word store: target `r1 +0x0`, value `[[sp-28]]` (`str` at 0x5b72c).
  Recorded r0 = `unknown` (`ldm` at 0x5b744).

## `portGetNextPort`

- `portGetNextPort`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5c488 to 0x5c628.
  Established facts:
  Branch (`bge` at 0x5c4c0): taken to 0x5c4f0 when `r0 >= [0x1a5c2c] (signed, 32-bit)`, falls through to 0x5c4c4 when `r0 < [0x1a5c2c] (signed, 32-bit)`.
  Grouped accesses: 3 reported word load records with printed base expression `[sp-36]` and nonnegative word-aligned offsets, including target `[sp-36] +0x8` (`ldr` at 0x5c534), target `[sp-36] +0x0` (`ldr` at 0x5c584), target `[sp-36] +0x0` (`ldr` at 0x5c5a8).
  Call (`bl` at 0x5c5fc): (unknown callee), target 0x5f3c4, r0=`r0`, r1=`unknown`, r2=`r2`, r3=`0x0`.
  Word store: target `[sp-32] +0x0`, value `ret(0x5f3c4)` (`str` at 0x5c608, `bl` at 0x5c5fc).
  Recorded r0 = `unknown` (`ldm` at 0x5c61c).

## `portName`

- `portName`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5c628 to 0x5c6a4.
  Established facts:
  Call (`bl` at 0x5c650): (unknown callee), target 0x5f308, r0=`r0`, r1=`(r1 & 0xff)`, r2=`r2`, r3=`(r1 & 0xff)`.
  Branch (`beq` at 0x5c664): taken to 0x5c688 when `ret(0x5f308) == 0x0`, falls through to 0x5c668 when `ret(0x5f308) != 0x0` (source `bl` at 0x5c650).
  Grouped accesses: 2 reported word load records with printed base expression `ret(0x5f308)` and nonnegative word-aligned offsets, including target `ret(0x5f308) +0x8` (`ldr` at 0x5c66c), target `ret(0x5f308) +0x8` (`ldr` at 0x5c67c) (source `bl` at 0x5c650).
  Branch (`beq` at 0x5c674): taken to 0x5c688 when `[ret(0x5f308)+8] == 0x0`, falls through to 0x5c678 when `[ret(0x5f308)+8] != 0x0` (source `bl` at 0x5c650).
  Recorded r0 = `unknown` (`ldm` at 0x5c69c).

## `portState`

- `portState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5c6a4 to 0x5c7b4.
  Established facts:
  Branch (`bne` at 0x5c6e8): taken to 0x5c6f8 when `[0x1a5c2c] != 0x0`, falls through to 0x5c6ec when `[0x1a5c2c] == 0x0`.
  Grouped accesses: 3 reported word load records with printed base expression `[sp-56]` and nonnegative word-aligned offsets, including target `[sp-56] +0x8` (`ldr` at 0x5c724), target `[sp-56] +0x4` (`ldr` at 0x5c73c), target `[sp-56] +0x4` (`ldr` at 0x5c76c).
  Branch (`bge` at 0x5c72c): taken to 0x5c754 when `(r1 & 0xff) >= [[sp-56]+8] (signed, 32-bit)`, falls through to 0x5c730 when `(r1 & 0xff) < [[sp-56]+8] (signed, 32-bit)`.
  Recorded r0 = `unknown` (`ldm` at 0x5c79c).

## `portQuerySlot`

- `portQuerySlot`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5ce64 to 0x5cf30.
  Established facts:
  Call (`bl` at 0x5ce94): (unknown callee), target 0x5b69c, r0=`r0`, r1=`sp-40`, r2=`r2`, r3=`sp-40`.
  Branch (`bne` at 0x5cea8): taken to 0x5ced4 when `ret(0x5b69c) != 0x0`, falls through to 0x5ceac when `ret(0x5b69c) == 0x0` (source `bl` at 0x5ce94).
  Call (`bl` at 0x5ceb4): (unknown callee), target 0x201fc, r0=`0x50`, r1=`unknown`, r2=`unknown`, r3=`ret(0x5b69c)`.
  Grouped accesses: 2 reported word load records with printed base expression `[sp-28]` and nonnegative word-aligned offsets, including target `[sp-28] +0x0` (`ldr` at 0x5cec4), target `[sp-28] +0x0` (`ldr` at 0x5cefc).
  Call (`bl` at 0x5cecc): (unknown callee), target 0x20634, r0=`[[sp-28]]`, r1=`0x1626d8`, r2=`unknown`, r3=`[sp-28]`.
  Call (`bl` at 0x5cef0): (unknown callee), target 0x201fc, r0=`unknown`, r1=`unknown`, r2=`ret(0x5b69c)`, r3=`unknown`.
  Byte store: target `[[sp-28]] +0x0`, value `0x0` (`strb` at 0x5cf0c).
  Call (`bl` at 0x5cf20): (unknown callee), target 0x5c7b4, r0=`[sp-24]`, r1=`[sp-56]`, r2=`0x0`, r3=`[sp-32]`.
  Recorded r0 = `unknown` (`ldm` at 0x5cf28).

## `portLinkQuerySlot`

- `portLinkQuerySlot`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5ddd4 to 0x5de7c.
  Established facts:
  Call (`bl` at 0x5de08): (unknown callee), target 0x5b69c, r0=`r0`, r1=`sp-44`, r2=`r2`, r3=`sp-44`.
  Branch (`bne` at 0x5de1c): taken to 0x5de24 when `ret(0x5b69c) != 0x0`, falls through to 0x5de20 when `ret(0x5b69c) == 0x0` (source `bl` at 0x5de08).
  Call (`bl` at 0x5de40): (unknown callee), target 0x201fc, r0=`unknown`, r1=`unknown`, r2=`ret(0x5b69c)`, r3=`unknown`.
  Word load: target `[sp-28] +0x0` (`ldr` at 0x5de4c).
  Byte store: target `[[sp-28]] +0x0`, value `0x0` (`strb` at 0x5de5c).
  Call (`bl` at 0x5de70): (unknown callee), target 0x5d0b4, r0=`[sp-24]`, r1=`[sp-60]`, r2=`[sp-32]`, r3=`[sp-36]`.
  Recorded r0 = `unknown` (`ldm` at 0x5de78).

## `portPollState`

- `portPollState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5e4c8 to 0x5e570.
  Established facts:
  Branch (`beq` at 0x5e4e4): taken to 0x5e504 when `[0x1a0250] == 0x0`, falls through to 0x5e4e8 when `[0x1a0250] != 0x0`.
  Word store: target `abs 0x1a0250`, value `0x0` (`str` at 0x5e4f0).
  Grouped accesses: 2 reported word store records with absolute target `0x1a5c24`, including target `abs 0x1a5c24`, value `0x1` (`str` at 0x5e4fc), target `abs 0x1a5c24`, value `0x0` (`str` at 0x5e51c).
  Call (`bl` at 0x5e524): (unknown callee), target 0x293f8, r0=`0x162ac4`, r1=`r1`, r2=`0x0`, r3=`0x1a5c24`.
  Call (`bl` at 0x5e530): (unknown callee), target 0x6e140, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x5e54c): (unknown callee), target 0x5bcec, r0=`[sp-20]`, r1=`0xff`, r2=`ret(0x6e140)`, r3=`[sp-20]`.
  Recorded r0 = `unknown` (`ldm` at 0x5e560).
