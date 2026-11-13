# ARM Linux 2.6 power, command and reset routines in 1.44.0

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
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-power-command-reset-facts.txt) retain all tool sections.

The notation `ret(ADDR)` identifies a call-result expression without distinguishing
individual call sites to that target. Each return line records machine-register
state from the analysis rather than a C return type.


Groups summarize static access records by printed shape. Instruction-address order can include different control-flow paths.

## `APwrInit`

- `APwrInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf8080 to 0xf82cc.
  Established facts:
  Decoded words in [0xf82a4, 0xf82cc) are not reached from the entry by known edges.
  Call (`bl` at 0xf8090): (unknown callee), target 0x6dc0c, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Branch (`beq` at 0xf80a8): taken to 0xf81ac when `[sp-28] == 0x43`, falls through to 0xf80ac when `[sp-28] != 0x43`.
  Branch (`beq` at 0xf80b4): taken to 0xf81ac when `[sp-28] == 0x33`, falls through to 0xf80b8 when `[sp-28] != 0x33`.
  Call (`bl` at 0xf81b0): (unknown callee), target 0x28230, r0=`0x32`, r1=`unknown`, r2=`unknown`, r3=`[sp-28]`.
  Call (`bl` at 0xf81d0): (unknown callee), target 0xfb5cc, r0=`0x31b9d4`, r1=`0x14`, r2=`0x0`, r3=`ret(0x28230)`.
  Call (`bl` at 0xf81d8): (unknown callee), target 0x28230, r0=`0x1`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf81e4): (unknown callee), target 0x28230, r0=`0x2`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf81f8): (unknown callee), target 0x96878, r0=`0x1b0fe0`, r1=`ret(0x28230)`, r2=`[sp-28]`, r3=`ret(0x28230)`.
  Call (`bl` at 0xf8208): (unknown callee), target 0xfb6ec, r0=`0x31b9dc`, r1=`0x1879b8`, r2=`0x1`, r3=`unknown`.
  Call (`bl` at 0xf8224): (unknown callee), target 0x29908, r0=`0x1879c4`, r1=`unknown`, r2=`unknown`, r3=`ret(0xfb6ec)`.
  Call (`bl` at 0xf823c): (unknown callee), target 0xfa264, r0=`0x31b9d4`, r1=`0xffff014`, r2=`0x2`, r3=`0x31b9dc`.
  Call (`bl` at 0xf8258): (unknown callee), target 0x29908, r0=`0x1879f0`, r1=`unknown`, r2=`unknown`, r3=`ret(0xfa264)`.
  Call (`bl` at 0xf8298): (unknown callee), target 0x22540, r0=`0x31ba80`, r1=`0x187a1c`, r2=`0xf82cc`, r3=`0x1`.
  Recorded r0 = `unknown` (`ldm` at 0xf82a0).

## `pwrStatusLEDUpdate`

- `pwrStatusLEDUpdate`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x81de0 to 0x8201c.
  Established facts:
  Decoded words in [0x82010, 0x82018) are not reached from the entry by known edges.
  Word load: target `abs 0x1a62d8` (`ldr` at 0x81e08).
  Grouped accesses: 12 reported word load records with absolute target `0x1a62d4`, including target `abs 0x1a62d4` (`ldr` at 0x81e28), target `abs 0x1a62d4` (`ldr` at 0x81e44), target `abs 0x1a62d4` (`ldr` at 0x81ec0).
  Call (`bl` at 0x81e64): (unknown callee), target 0x55e9c, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x81f0c): (unknown callee), target 0x1259bc, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x81f4c): (unknown callee), target 0x125a0c, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x81fb4): (unknown callee), target 0x125a0c, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x81ff4): (unknown callee), target 0x1259bc, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x8200c).

## `pwrStatusLEDInit`

- `pwrStatusLEDInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x81d50 to 0x81de0.
  Established facts:
  Decoded words in [0x81dd0, 0x81de0) are not reached from the entry by known edges.
  Branch (`beq` at 0x81d74): taken to 0x81dac when `[0x1a62d4] == 0x0`, falls through to 0x81d78 when `[0x1a62d4] != 0x0`.
  Call (`bl` at 0x81da8): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x1668d4`, r2=`0x2c`, r3=`0x0`.
  Word store: target `abs 0x1a62d4`, value `unknown` (`str` at 0x81db4).
  Word store: target `abs 0x1a62d8`, value `[sp-24]` (`str` at 0x81dc0).
  Call (`bl` at 0x81dc4): (unknown callee), target 0x81de0, r0=`unknown`, r1=`unknown`, r2=`0x1a62d8`, r3=`[sp-24]`.
  Recorded r0 = `ret(0x81de0)` (`ldm` at 0x81dcc, `bl` at 0x81dc4).

## `FruFmtMultiRec`

- `FruFmtMultiRec`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf92a0 to 0xf99dc.
  Established facts:
  Decoded words in [0xf9980, 0xf998c), [0xf9990, 0xf99d4), [0xf99d8, 0xf99dc) are not reached from the entry by known edges.
  Byte store: target `r3 +0x0`, value `0x0` (`strb` at 0xf92cc).
  Branch (`beq` at 0xf92e0): taken to 0xf92f4 when `[sp-28] == 0x0`, falls through to 0xf92e4 when `[sp-28] != 0x0`.
  Call (`bl` at 0xf9308): (unknown callee), target 0x20634, r0=`[sp-40]`, r1=`0x187e0c`, r2=`r3`, r3=`[sp-29]:1`.
  Call (`bl` at 0xf9318): (unknown callee), target 0x20634, r0=`[sp-40]`, r1=`0x187e20`, r2=`r3`, r3=`[sp-29]:1`.
  Call (`bl` at 0xf9328): (unknown callee), target 0xede7c, r0=`[sp-36]`, r1=`unknown`, r2=`unknown`, r3=`[sp-36]`.
  Call (`bl` at 0xf9344): (unknown callee), target 0x205bc, r0=`sp-140`, r1=`0x187e30`, r2=`(ret(0xede7c) & 0xffff)`, r3=`sp-140`.
  Call (`bl` at 0xf9354): (unknown callee), target 0x203c4, r0=`[sp-40]`, r1=`sp-140`, r2=`unknown`, r3=`sp-140`.
  Call (`bl` at 0xf9364): (unknown callee), target 0xede7c, r0=`[sp-44]+2`, r1=`unknown`, r2=`unknown`, r3=`[sp-44]+2`.
  Call (`bl` at 0xf9390): (unknown callee), target 0x205bc, r0=`sp-140`, r1=`0x187e50`, r2=`(ret(0xede7c) & 0xffff)`, r3=`sp-140`.
  Call (`bl` at 0xf93a0): (unknown callee), target 0x203c4, r0=`[sp-40]`, r1=`sp-140`, r2=`unknown`, r3=`sp-140`.
  Grouped accesses: 7 reported byte load records with printed base expression `[sp-44]` and nonnegative byte-aligned offsets, including target `[sp-44] +0x4` (`ldrb` at 0xf93a8), target `[sp-44] +0x4` (`ldrb` at 0xf93bc), target `[sp-44] +0x5` (`ldrb` at 0xf93c4).
  Call (`bl` at 0xf93d4): (unknown callee), target 0x205bc, r0=`sp-140`, r1=`0x187e64`, r2=`[[sp-44]+4]:1`, r3=`[[sp-44]+5]:1`.
  Call (`bl` at 0xf93e4): (unknown callee), target 0x203c4, r0=`[sp-40]`, r1=`sp-140`, r2=`unknown`, r3=`sp-140`.
  Call (`bl` at 0xf93f4): (unknown callee), target 0xede7c, r0=`[sp-44]+6`, r1=`unknown`, r2=`unknown`, r3=`[sp-44]+6`.
  Call (`bl` at 0xf9418): (unknown callee), target 0xede7c, r0=`[sp-44]+8`, r1=`unknown`, r2=`unknown`, r3=`[sp-44]+8`.
  Call (`bl` at 0xf9464): (unknown callee), target 0x205bc, r0=`sp-140`, r1=`0x187e94`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf9474): (unknown callee), target 0x203c4, r0=`[sp-40]`, r1=`sp-140`, r2=`unknown`, r3=`sp-140`.
  Call (`bl` at 0xf9484): (unknown callee), target 0xede7c, r0=`[sp-44]+10`, r1=`unknown`, r2=`unknown`, r3=`[sp-44]+10`.
  Call (`bl` at 0xf94a8): (unknown callee), target 0xede7c, r0=`[sp-44]+12`, r1=`unknown`, r2=`unknown`, r3=`[sp-44]+12`.
  Call (`bl` at 0xf94f4): (unknown callee), target 0x205bc, r0=`sp-140`, r1=`0x187ebc`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf9504): (unknown callee), target 0x203c4, r0=`[sp-40]`, r1=`sp-140`, r2=`unknown`, r3=`sp-140`.
  Call (`bl` at 0xf9510): (unknown callee), target 0x203c4, r0=`[sp-40]`, r1=`0x187ecc`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf9534): (unknown callee), target 0x205bc, r0=`sp-140`, r1=`0x187ed4`, r2=`[[sp-44]+14]:1`, r3=`[[sp-44]+15]:1`.
  Call (`bl` at 0xf9544): (unknown callee), target 0x203c4, r0=`[sp-40]`, r1=`sp-140`, r2=`unknown`, r3=`sp-140`.
  Call (`bl` at 0xf9560): (unknown callee), target 0x205bc, r0=`sp-140`, r1=`0x187ef8`, r2=`[[sp-44]+16]:1`, r3=`[[sp-44]+16]:1`.
  Call (`bl` at 0xf9570): (unknown callee), target 0x203c4, r0=`[sp-40]`, r1=`sp-140`, r2=`unknown`, r3=`sp-140`.
  Call (`bl` at 0xf9580): (unknown callee), target 0xede7c, r0=`[sp-44]+18`, r1=`unknown`, r2=`unknown`, r3=`[sp-44]+18`.
  Call (`bl` at 0xf95b8): (unknown callee), target 0x205bc, r0=`sp-140`, r1=`0x187f1c`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf95c8): (unknown callee), target 0x203c4, r0=`[sp-40]`, r1=`sp-140`, r2=`unknown`, r3=`sp-140`.
  Call (`bl` at 0xf95f0): (unknown callee), target 0xf99dc, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf960c): (unknown callee), target 0xf99dc, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf9620): (unknown callee), target 0xede7c, r0=`[sp-44]+21`, r1=`unknown`, r2=`unknown`, r3=`[sp-44]+21`.
  Call (`bl` at 0xf9640): (unknown callee), target 0x205bc, r0=`sp-140`, r1=`0x187f3c`, r2=`ret(0xf99dc)`, r3=`ret(0xf99dc)`.
  Call (`bl` at 0xf965c): (unknown callee), target 0x20634, r0=`[sp-40]`, r1=`0x187f60`, r2=`[sp-28]`, r3=`[sp-29]:1`.
  Call (`bl` at 0xf966c): (unknown callee), target 0x20634, r0=`[sp-40]`, r1=`0x187f70`, r2=`[sp-28]`, r3=`[sp-29]:1`.
  Call (`bl` at 0xf9684): (unknown callee), target 0xede7c, r0=`[sp-36]+1`, r1=`unknown`, r2=`unknown`, r3=`[sp-36]+1`.
  Call (`bl` at 0xf96f8): (unknown callee), target 0x205bc, r0=`sp-140`, r1=`0x187f80`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf9708): (unknown callee), target 0x203c4, r0=`[sp-40]`, r1=`sp-140`, r2=`unknown`, r3=`sp-140`.
  Call (`bl` at 0xf9724): (unknown callee), target 0x203c4, r0=`[sp-40]`, r1=`0x187fa0`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf9730): (unknown callee), target 0x203c4, r0=`[sp-40]`, r1=`0x187fac`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf9740): (unknown callee), target 0xede7c, r0=`[sp-48]+3`, r1=`unknown`, r2=`unknown`, r3=`[sp-48]+3`.
  Call (`bl` at 0xf9758): (unknown callee), target 0xede7c, r0=`[sp-48]+5`, r1=`unknown`, r2=`unknown`, r3=`[sp-48]+5`.
  Call (`bl` at 0xf9834): (unknown callee), target 0x205bc, r0=`sp-140`, r1=`0x187fb0`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf9844): (unknown callee), target 0x203c4, r0=`[sp-40]`, r1=`sp-140`, r2=`unknown`, r3=`sp-140`.
  Call (`bl` at 0xf9854): (unknown callee), target 0xede7c, r0=`[sp-48]+7`, r1=`unknown`, r2=`unknown`, r3=`[sp-48]+7`.
  Call (`bl` at 0xf9870): (unknown callee), target 0x205bc, r0=`sp-140`, r1=`0x187fec`, r2=`(ret(0xede7c) & 0xffff)`, r3=`sp-140`.
  Call (`bl` at 0xf9880): (unknown callee), target 0x203c4, r0=`[sp-40]`, r1=`sp-140`, r2=`unknown`, r3=`sp-140`.
  Call (`bl` at 0xf9890): (unknown callee), target 0xede7c, r0=`[sp-48]+9`, r1=`unknown`, r2=`unknown`, r3=`[sp-48]+9`.
  Call (`bl` at 0xf98a8): (unknown callee), target 0xede7c, r0=`[sp-48]+11`, r1=`unknown`, r2=`unknown`, r3=`[sp-48]+11`.
  Call (`bl` at 0xf9964): (unknown callee), target 0x205bc, r0=`sp-140`, r1=`0x188018`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf9974): (unknown callee), target 0x203c4, r0=`[sp-40]`, r1=`sp-140`, r2=`unknown`, r3=`sp-140`.
  Recorded r0 = `unknown` (`ldm` at 0xf997c).

## `smpcmd_ioexp`

- `smpcmd_ioexp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf767c to 0xf79f8.
  Established facts:
  Decoded words in [0xf79e0, 0xf79f8) are not reached from the entry by known edges.
  Call (`bl` at 0xf7694): (unknown callee), target 0x6e140, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0xf76ac): (unknown callee), target 0xc5488, r0=`[sp-24]`, r1=`[sp-28]`, r2=`sp-64`, r3=`sp-64`.
  Call (`bl` at 0xf770c): (unknown callee), target 0x12586c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf7750): (unknown callee), target 0x201fc, r0=`unknown`, r1=`unknown`, r2=`[sp-48]`, r3=`unknown`.
  Word load: target `[sp-28] +0x0` (`ldr` at 0xf775c).
  Call (`bl` at 0xf776c): (unknown callee), target 0x205bc, r0=`[[sp-28]]`, r1=`0x18788c`, r2=`unknown`, r3=`[[sp-28]]`.
  Call (`bl` at 0xf7774): (unknown callee), target 0x2031c, r0=`[sp-44]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf77c8): (unknown callee), target 0x12586c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf77fc): (unknown callee), target 0x6e1b0, r0=`[sp-56]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0xf7808): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf7850): (unknown callee), target 0x203f4, r0=`[sp-96]`, r1=`[sp-88]`, r2=`[sp-92]`, r3=`unknown`.
  Call (`bl` at 0xf7868): (unknown callee), target 0x1265c4, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf7884): (unknown callee), target 0x205bc, r0=`sp-84`, r1=`0x1878dc`, r2=`ret(0x203f4)`, r3=`ret(0x1265c4)`.
  Call (`bl` at 0xf78c0): (unknown callee), target 0x205bc, r0=`[sp-44]`, r1=`0x1878e4`, r2=`unknown`, r3=`sp-84`.
  Call (`bl` at 0xf78c8): (unknown callee), target 0x2031c, r0=`[sp-44]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf78f0): (unknown callee), target 0x125958, r0=`[sp-56]`, r1=`unknown`, r2=`ret(0x2031c)`, r3=`unknown`.
  Call (`bl` at 0xf7908): (unknown callee), target 0x205bc, r0=`[sp-44]`, r1=`0x1878f4`, r2=`unknown`, r3=`ret(0x125958)`.
  Call (`bl` at 0xf7924): (unknown callee), target 0x125b8c, r0=`[sp-56]`, r1=`unknown`, r2=`sp-49`, r3=`unknown`.
  Call (`bl` at 0xf7940): (unknown callee), target 0x21ce0, r0=`ret(0x125b8c)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x125b8c)`.
  Call (`bl` at 0xf7954): (unknown callee), target 0x205bc, r0=`[sp-44]`, r1=`0x187834`, r2=`ret(0x21ce0)`, r3=`ret(0x21ce0)`.
  Call (`bl` at 0xf796c): (unknown callee), target 0x125cb0, r0=`[sp-56]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf798c): (unknown callee), target 0x205bc, r0=`[sp-44]`, r1=`0x187904`, r2=`[sp-49]:1`, r3=`unknown`.
  Call (`bl` at 0xf7994): (unknown callee), target 0x2031c, r0=`[sp-44]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0xf79dc).

## `smpcmd_intr_count`

- `smpcmd_intr_count`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf79f8 to 0xf8080.
  Established facts:
  Decoded words in [0xf8060, 0xf8080) are not reached from the entry by known edges.
  Call (`bl` at 0xf7a10): (unknown callee), target 0x6e140, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0xf7a28): (unknown callee), target 0xc5488, r0=`[sp-28]`, r1=`[sp-32]`, r2=`sp-72`, r3=`sp-72`.
  Call (`bl` at 0xf7a88): (unknown callee), target 0x111e58, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf7aa0): (unknown callee), target 0x12586c, r0=`[sp-64]`, r1=`unknown`, r2=`ret(0x111e58)`, r3=`([sp-36] + ret(0x111e58))`.
  Call (`bl` at 0xf7ac4): (unknown callee), target 0x10b9b0, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`[sp-68]`.
  Call (`bl` at 0xf7acc): (unknown callee), target 0x10cc94, r0=`ret(0x10b9b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf7b14): (unknown callee), target 0x201fc, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Word load: target `[sp-32] +0x0` (`ldr` at 0xf7b20).
  Call (`bl` at 0xf7b30): (unknown callee), target 0x205bc, r0=`[[sp-32]]`, r1=`0x187918`, r2=`unknown`, r3=`[[sp-32]]`.
  Call (`bl` at 0xf7b38): (unknown callee), target 0x2031c, r0=`[sp-52]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf7b78): (unknown callee), target 0x10bd70, r0=`unknown`, r1=`0x1`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf7b8c): (unknown callee), target 0x205bc, r0=`[sp-52]`, r1=`0x187928`, r2=`ret(0x10bd70)`, r3=`ret(0x10bd70)`.
  Call (`bl` at 0xf7b94): (unknown callee), target 0x2031c, r0=`[sp-52]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf7bf8): (unknown callee), target 0x111e58, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf7c2c): (unknown callee), target 0x111978, r0=`[sp-64]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf7c4c): (unknown callee), target 0x6e1b0, r0=`[sp-64]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0xf7c58): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf7ca0): (unknown callee), target 0x203f4, r0=`[sp-84]`, r1=`[sp-76]`, r2=`[sp-80]`, r3=`unknown`.
  Call (`bl` at 0xf7cb8): (unknown callee), target 0x1144ec, r0=`[sp-64]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf7ccc): (unknown callee), target 0x205bc, r0=`[sp-52]`, r1=`0x18792c`, r2=`ret(0x203f4)`, r3=`ret(0x1144ec)`.
  Call (`bl` at 0xf7cf0): (unknown callee), target 0x114088, r0=`[sp-64]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf7d14): (unknown callee), target 0x6e1b0, r0=`[sp-64]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0xf7d20): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf7d68): (unknown callee), target 0x203f4, r0=`[sp-76]`, r1=`[sp-84]`, r2=`[sp-80]`, r3=`unknown`.
  Call (`bl` at 0xf7d80): (unknown callee), target 0x1144ec, r0=`[sp-64]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf7d98): (unknown callee), target 0x114088, r0=`[sp-64]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf7db4): (unknown callee), target 0x205bc, r0=`[sp-52]`, r1=`0x187950`, r2=`ret(0x203f4)`, r3=`ret(0x1144ec)`.
  Call (`bl` at 0xf7dbc): (unknown callee), target 0x2031c, r0=`[sp-52]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf7de8): (unknown callee), target 0x12586c, r0=`[sp-64]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf7e1c): (unknown callee), target 0x125958, r0=`[sp-64]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf7e3c): (unknown callee), target 0x6e1b0, r0=`[sp-64]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0xf7e48): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf7e90): (unknown callee), target 0x203f4, r0=`[sp-76]`, r1=`[sp-84]`, r2=`[sp-80]`, r3=`unknown`.
  Call (`bl` at 0xf7ea8): (unknown callee), target 0x1265c4, r0=`[sp-64]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf7ebc): (unknown callee), target 0x205bc, r0=`[sp-52]`, r1=`0x18796c`, r2=`ret(0x203f4)`, r3=`ret(0x1265c4)`.
  Call (`bl` at 0xf7ee0): (unknown callee), target 0x125c68, r0=`[sp-64]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf7f04): (unknown callee), target 0x6e1b0, r0=`[sp-64]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0xf7f10): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf7f58): (unknown callee), target 0x203f4, r0=`[sp-76]`, r1=`[sp-84]`, r2=`[sp-80]`, r3=`unknown`.
  Call (`bl` at 0xf7f70): (unknown callee), target 0x1265c4, r0=`[sp-64]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf7f88): (unknown callee), target 0x125c68, r0=`[sp-64]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf7fa4): (unknown callee), target 0x205bc, r0=`[sp-52]`, r1=`0x187990`, r2=`ret(0x203f4)`, r3=`ret(0x1265c4)`.
  Call (`bl` at 0xf7fac): (unknown callee), target 0x2031c, r0=`[sp-52]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf8008): (unknown callee), target 0x10ce94, r0=`unknown`, r1=`0x1879ac`, r2=`0x1`, r3=`[sp-48]`.
  Call (`bl` at 0xf801c): (unknown callee), target 0x205bc, r0=`[sp-52]`, r1=`0x1879b4`, r2=`ret(0x10ce94)`, r3=`ret(0x10ce94)`.
  Call (`bl` at 0xf8024): (unknown callee), target 0x2031c, r0=`[sp-52]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0xf805c).

## `smpcmd_mgn`

- `smpcmd_mgn`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcd6b0 to 0xce064.
  Established facts:
  Decoded words in [0xce038, 0xce064) are not reached from the entry by known edges.
  Call (`bl` at 0xcd6e0): (unknown callee), target 0x6dc0c, r0=`r0`, r1=`r1`, r2=`r2`, r3=`0x0`.
  Call (`bl` at 0xcd6fc): (unknown callee), target 0xc4d10, r0=`[[[sp-24]+112]]`, r1=`[sp-28]`, r2=`unknown`, r3=`[[sp-24]+112]`.
  Call (`bl` at 0xcd7b8): (unknown callee), target 0x6e140, r0=`ret(0xc4d10)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xcd7d0): (unknown callee), target 0xc5488, r0=`[sp-24]`, r1=`[sp-28]`, r2=`sp-52`, r3=`sp-52`.
  Word load: target `abs 0x31a604` (`ldr` at 0xcd840).
  Call (`mov` at 0xcd84c): (unknown callee), target (unknown target), r0=`[sp-53]:1`, r1=`unknown`, r2=`[sp-53]:1`, r3=`[0x31a604]`.
  Call (`bl` at 0xcd86c): (unknown callee), target 0x201fc, r0=`0x50`, r1=`unknown`, r2=`unknown`, r3=`ret(unknown)`.
  Call (`bl` at 0xcd87c): (unknown callee), target 0x219a4, r0=`[sp-36]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x201fc)`.
  Call (`bl` at 0xcd890): (unknown callee), target 0x205bc, r0=`[[sp-28]]`, r1=`0x176c64`, r2=`ret(0x219a4)`, r3=`[sp-28]`.
  Branch (`bls` at 0xcd8b4): taken to 0xcdb38 when `[sp-48] <= 0x1 (unsigned, 32-bit)`, falls through to 0xcd8b8 when `[sp-48] > 0x1 (unsigned, 32-bit)`.
  Call (`bl` at 0xcd90c): (unknown callee), target 0x201fc, r0=`unknown`, r1=`unknown`, r2=`[sp-48]`, r3=`unknown`.
  Byte store: target `[[sp-28]] +0x0`, value `0x0` (`strb` at 0xcd928).
  Call (`bl` at 0xcd970): (unknown callee), target 0x530f4, r0=`unknown`, r1=`sp-60`, r2=`unknown`, r3=`sp-60`.
  Call (`bl` at 0xcd994): (unknown callee), target 0x6e1b0, r0=`[sp-44]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0xcd9a0): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xcd9e8): (unknown callee), target 0x203f4, r0=`[sp-88]`, r1=`[sp-80]`, r2=`[sp-84]`, r3=`unknown`.
  Call (`bl` at 0xcd9fc): (unknown callee), target 0x205bc, r0=`[sp-76]`, r1=`0x178468`, r2=`ret(0x203f4)`, r3=`ret(0x203f4)`.
  Call (`bl` at 0xcda04): (unknown callee), target 0x2031c, r0=`[sp-76]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xcda34): (unknown callee), target 0x5656c, r0=`[sp-44]`, r1=`0x28`, r2=`[sp-53]:1`, r3=`0x0`.
  Call (`bl` at 0xcda5c): (unknown callee), target 0x6e1b0, r0=`[sp-44]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0xcda68): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xcdab0): (unknown callee), target 0x203f4, r0=`[sp-80]`, r1=`[sp-88]`, r2=`[sp-84]`, r3=`unknown`.
  Call (`bl` at 0xcdabc): (unknown callee), target 0x219a4, r0=`[sp-36]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xcdad0): (unknown callee), target 0x205bc, r0=`[sp-76]`, r1=`0x176fb8`, r2=`ret(0x203f4)`, r3=`ret(0x219a4)`.
  Call (`bl` at 0xcdad8): (unknown callee), target 0x2031c, r0=`[sp-76]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xcdb48): (unknown callee), target 0xc50d8, r0=`[sp-24]`, r1=`[sp-28]`, r2=`sp-44`, r3=`sp-44`.
  Call (`bl` at 0xcdbb4): (unknown callee), target 0x530f4, r0=`[sp-44]`, r1=`sp-60`, r2=`unknown`, r3=`sp-60`.
  Call (`bl` at 0xcdbd0): (unknown callee), target 0x201fc, r0=`0x50`, r1=`unknown`, r2=`unknown`, r3=`ret(0x530f4)`.
  Call (`bl` at 0xcdbe8): (unknown callee), target 0x20634, r0=`[[sp-28]]`, r1=`0x17822c`, r2=`unknown`, r3=`[sp-28]`.
  Call (`bl` at 0xcdc98): (unknown callee), target 0x201fc, r0=`0x50`, r1=`unknown`, r2=`[sp-88]`, r3=`[sp-64]`.
  Call (`bl` at 0xcdcb4): (unknown callee), target 0x6e1b0, r0=`[sp-44]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0xcdcc0): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xcdd08): (unknown callee), target 0x203f4, r0=`[sp-96]`, r1=`[sp-80]`, r2=`[sp-92]`, r3=`unknown`.
  Call (`bl` at 0xcdd24): (unknown callee), target 0x205bc, r0=`[[sp-28]]`, r1=`0x178490`, r2=`ret(0x203f4)`, r3=`[[sp-24]+40]`.
  Call (`bl` at 0xcdd64): (unknown callee), target 0x5656c, r0=`[sp-44]`, r1=`[sp-40]`, r2=`[sp-53]:1`, r3=`0x0`.
  Call (`bl` at 0xcdd8c): (unknown callee), target 0x560d4, r0=`[sp-44]`, r1=`[sp-40]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xcddc0): (unknown callee), target 0x20334, r0=`[[sp-24]+76]`, r1=`unknown`, r2=`0x4`, r3=`[sp-24]`.
  Call (`bl` at 0xcde08): (unknown callee), target 0x201fc, r0=`0xa0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xcde24): (unknown callee), target 0x6e1b0, r0=`[sp-44]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0xcde30): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xcde78): (unknown callee), target 0x203f4, r0=`[sp-88]`, r1=`[sp-96]`, r2=`[sp-92]`, r3=`unknown`.
  Call (`bl` at 0xcde94): (unknown callee), target 0x205bc, r0=`[[sp-28]]`, r1=`0x1784dc`, r2=`ret(0x203f4)`, r3=`[[sp-24]+76]`.
  Call (`bl` at 0xcdeb4): (unknown callee), target 0x5656c, r0=`[sp-44]`, r1=`0x1a`, r2=`0x6`, r3=`[sp-72]`.
  Call (`bl` at 0xcdecc): (unknown callee), target 0x201fc, r0=`0x50`, r1=`unknown`, r2=`0x4`, r3=`[[[sp-24]+112]+4]`.
  Call (`bl` at 0xcdee8): (unknown callee), target 0x6e1b0, r0=`[sp-44]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0xcdef4): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xcdf3c): (unknown callee), target 0x203f4, r0=`[sp-88]`, r1=`[sp-96]`, r2=`[sp-92]`, r3=`unknown`.
  Call (`bl` at 0xcdf58): (unknown callee), target 0x205bc, r0=`[[sp-28]]`, r1=`0x178508`, r2=`ret(0x203f4)`, r3=`[[sp-24]+40]`.
  Call (`bl` at 0xcdf7c): (unknown callee), target 0x201fc, r0=`0x50`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xcdf98): (unknown callee), target 0x6e1b0, r0=`[sp-44]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0xcdfa4): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xcdfec): (unknown callee), target 0x203f4, r0=`[sp-88]`, r1=`[sp-96]`, r2=`[sp-92]`, r3=`unknown`.
  Call (`bl` at 0xcdff8): (unknown callee), target 0x219a4, r0=`[sp-36]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xce014): (unknown callee), target 0x205bc, r0=`[[sp-28]]`, r1=`0x176fb8`, r2=`ret(0x203f4)`, r3=`ret(0x219a4)`.
  Recorded r0 = `unknown` (`ldm` at 0xce034).

## `PPPWrite`

- `PPPWrite`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1003b8 to 0x1008c8.
  Established facts:
  Word load: target `r0 +0x18` (`ldr` at 0x1003e8).
  Word store: target `r0 +0x20`, value `[r0+24]` (`str` at 0x1003ec).
  Branch (`beq` at 0x10042c): taken to 0x100660 when `[sp] == 0x0`, falls through to 0x100430 when `[sp] != 0x0`.
  Call (`bl` at 0x100584): (unknown callee), target 0x1001a4, r0=`[[sp-32]]:1`, r1=`[sp-38]:2`, r2=`[sp-38]:2`, r3=`[[sp-32]]:1`.
  Call (`mov` at 0x1005e8): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-20]+24]`, r2=`[[sp-20]+36]`, r3=`[sp-20]`.
  Call (`bl` at 0x100630): (unknown callee), target 0x1001fc, r0=`[sp-20]`, r1=`ret(unknown)`, r2=`[[sp-20]+24]`, r3=`[sp-52]`.
  Call (`mov` at 0x10085c): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[[sp-20]+24]`, r2=`[[sp-20]+36]`, r3=`[sp-20]`.
  Call (`bl` at 0x1008a4): (unknown callee), target 0x1001fc, r0=`[sp-20]`, r1=`ret(unknown)`, r2=`[[sp-20]+24]`, r3=`[sp-52]`.
  Recorded r0 = `unknown` (`ldm` at 0x1008c4).

## `PPPReset`

- `PPPReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x100354 to 0x1003b8.
  Established facts:
  Branch (`beq` at 0x100378): taken to 0x100394 when `? == 0x0`, falls through to 0x10037c when `? != 0x0`.
  Word store: target `r0 +0x14`, value `0x0` (`str` at 0x100384).
  Word store: target `[sp-20] +0x38`, value `0x0` (`str` at 0x100390).
  Branch (`beq` at 0x1003a0): taken to 0x1003b0 when `? == 0x0`, falls through to 0x1003a4 when `? != 0x0`.
  Word store: target `(unknown effective address)`, value `0x0` (`str` at 0x1003ac).
  Recorded r0 = `r0` (`ldm` at 0x1003b4).

## `PPPDecodeReset`

- `PPPDecodeReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1012c0 to 0x1012ec.
  Established facts:
  Word store: target `r0 +0x30`, value `0x1` (`str` at 0x1012dc).
  Recorded r0 = `0x0` (`ldm` at 0x1012e8).

## `exceptionIsSoftReset`

- `exceptionIsSoftReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x20fd8 to 0x21058.
  Established facts:
  Decoded words in [0x2104c, 0x21058) are not reached from the entry by known edges.
  Word load: target `abs 0x1a28a8` (`ldr` at 0x20fec).
  Branch (`bne` at 0x20ff4): taken to 0x2102c when `[0x1a28a8] != 0x0`, falls through to 0x20ff8 when `[0x1a28a8] == 0x0`.
  Call (`bl` at 0x20ffc): (unknown callee), target 0x200d0, r0=`0x1503dc`, r1=`r1`, r2=`r2`, r3=`[0x1a28a8]`.
  Call (`bl` at 0x2101c): (unknown callee), target 0x200ac, r0=`ret(0x200d0)`, r1=`0x1503ec`, r2=`unknown`, r3=`ret(0x200d0)`.
  Recorded r0 = `unknown` (`ldm` at 0x21048).

## `exceptionReset`

- `exceptionReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x21058 to 0x21108.
  Established facts:
  Word load: target `abs 0x1a28a8` (`ldr` at 0x21074).
  Branch (`beq` at 0x2107c): taken to 0x21098 when `[0x1a28a8] == 0x0`, falls through to 0x21080 when `[0x1a28a8] != 0x0`.
  Grouped accesses: 2 reported word load records with absolute target `0x1a288c`, including target `abs 0x1a288c` (`ldr` at 0x21084), target `abs 0x1a288c` (`ldr` at 0x210d8).
  Call (`bl` at 0x2108c): (unknown callee), target 0x244d4, r0=`[0x1a288c]`, r1=`0x1503f4`, r2=`r2`, r3=`0x1a288c`.
  Call (`bl` at 0x21094): (unknown callee), target 0x204b4, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x210ac): (unknown callee), target 0x204b4, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`0x3e7`.
  Call (`bl` at 0x210c0): (unknown callee), target 0x110c10, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x210c8): (unknown callee), target 0x201a8, r0=`0x150430`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x210d0): (unknown callee), target 0x22944, r0=`0x1388`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x210e0): (unknown callee), target 0x244d4, r0=`[0x1a288c]`, r1=`0x150440`, r2=`unknown`, r3=`0x1a288c`.
  Call (`bl` at 0x210e8): (unknown callee), target 0x204b4, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  The analysis reports no return in this span.

## `SMPCmdCheckBrickType`

- `SMPCmdCheckBrickType`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc4d10 to 0xc500c.
  Established facts:
  Decoded words in [0xc4da0, 0xc4f30) are not reached from the entry by known edges.
  Call (`bl` at 0xc4d28): (unknown callee), target 0x6dc0c, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Byte load: target `[sp-20] +0x0` (`ldrb` at 0xc4d6c).
  Branch (`bne` at 0xc4d74): taken to 0xc4d8c when `[[sp-20]]:1 != 0x7e`, falls through to 0xc4d78 when `[[sp-20]]:1 == 0x7e`.
  Branch (`ldrls` at 0xc4d98): taken to (unknown target) when `(ret(0x6dc0c) & 0xff)-35 <= 0x5a (unsigned, 32-bit)`, falls through to 0xc4d9c when `(ret(0x6dc0c) & 0xff)-35 > 0x5a (unsigned, 32-bit)` (source `bl` at 0xc4d28).
  Omitted-path effects remain unknown at the unresolved transfer (`ldrls` at 0xc4d98).
  Call (`bl` at 0xc4f44): (unknown callee), target 0x20040, r0=`[sp-20]`, r1=`(ret(0x6dc0c) & 0xff)`, r2=`unknown`, r3=`(ret(0x6dc0c) & 0xff)`.
  Call (`bl` at 0xc4f7c): (unknown callee), target 0x20040, r0=`[sp-20]`, r1=`[sp-26]:1`, r2=`unknown`, r3=`[sp-26]:1`.
  Call (`bl` at 0xc4fd4): (unknown callee), target 0x20514, r0=`[[sp-24]]`, r1=`unknown`, r2=`unknown`, r3=`[sp-24]`.
  Call (`bl` at 0xc4fe8): (unknown callee), target 0xc4cb8, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Known-edge recorded r0 = `unknown` (`ldm` at 0xc5008).

## `SMPCmdValidateSlot`

- `SMPCmdValidateSlot`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc50d8 to 0xc5488.
  Established facts:
  Decoded words in [0xc5478, 0xc5488) are not reached from the entry by known edges.
  Call (`bl` at 0xc50f8): (unknown callee), target 0x6e190, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0xc5100): (unknown callee), target 0x6e140, r0=`ret(0x6e190)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Word load: target `[sp-24] +0x74` (`ldr` at 0xc5120).
  Grouped accesses: 16 reported word store records with printed base expression `[sp-32]` and nonnegative word-aligned offsets, including target `[sp-32] +0x0`, value `0x0` (`str` at 0xc5264), target `[sp-32] +0x0`, value `0x1` (`str` at 0xc5274), target `[sp-32] +0x0`, value `0x2` (`str` at 0xc5284).
  Call (`bl` at 0xc5364): (unknown callee), target 0x201fc, r0=`0x50`, r1=`unknown`, r2=`unknown`, r3=`[[sp-24]+116]`.
  Call (`bl` at 0xc537c): (unknown callee), target 0x205bc, r0=`[[sp-28]]`, r1=`0x176654`, r2=`unknown`, r3=`[sp-28]`.
  Call (`bl` at 0xc5390): (unknown callee), target 0x3e274, r0=`ret(0x6e140)`, r1=`unknown`, r2=`[sp-32]`, r3=`unknown`.
  Call (`bl` at 0xc53ac): (unknown callee), target 0x201fc, r0=`0x50`, r1=`unknown`, r2=`ret(0x3e274)`, r3=`[[sp-32]]`.
  Call (`bl` at 0xc53c4): (unknown callee), target 0x205bc, r0=`[[sp-28]]`, r1=`0x176690`, r2=`unknown`, r3=`[sp-28]`.
  Call (`bl` at 0xc53dc): (unknown callee), target 0x3e58c, r0=`[[sp-32]]`, r1=`unknown`, r2=`ret(0x3e274)`, r3=`[sp-32]`.
  Call (`bl` at 0xc53f4): (unknown callee), target 0x201fc, r0=`0x50`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e58c)`.
  Call (`bl` at 0xc540c): (unknown callee), target 0x205bc, r0=`[[sp-28]]`, r1=`0x1766c0`, r2=`unknown`, r3=`[sp-28]`.
  Call (`bl` at 0xc5424): (unknown callee), target 0x3ec6c, r0=`[[sp-32]]`, r1=`unknown`, r2=`unknown`, r3=`[sp-32]`.
  Call (`bl` at 0xc543c): (unknown callee), target 0x201fc, r0=`0x50`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3ec6c)`.
  Call (`bl` at 0xc5454): (unknown callee), target 0x205bc, r0=`[[sp-28]]`, r1=`0x1766f4`, r2=`unknown`, r3=`[sp-28]`.
  Recorded r0 = `unknown` (`ldm` at 0xc5474).

## `SMPCmdValidateSBMASK`

- `SMPCmdValidateSBMASK`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc5488 to 0xc55dc.
  Established facts:
  Decoded words in [0xc55d4, 0xc55dc) are not reached from the entry by known edges.
  Word store: target `r2 +0x0`, value `unknown` (`str` at 0xc54ac).
  Call (`bl` at 0xc54b0): (unknown callee), target 0x6e140, r0=`r0`, r1=`r1`, r2=`r2`, r3=`unknown`.
  Grouped accesses: 3 reported word load records with printed base expression `[sp-24]` and nonnegative word-aligned offsets, including target `[sp-24] +0x74` (`ldr` at 0xc54cc), target `[sp-24] +0x74` (`ldr` at 0xc5518), target `[sp-24] +0x74` (`ldr` at 0xc552c).
  Call (`bl` at 0xc54e0): (unknown callee), target 0x201fc, r0=`0x50`, r1=`unknown`, r2=`unknown`, r3=`[[sp-24]+116]`.
  Call (`bl` at 0xc54f8): (unknown callee), target 0x20634, r0=`[[sp-28]]`, r1=`0x17672c`, r2=`unknown`, r3=`[sp-28]`.
  Word store: target `[sp-32] +0x0`, value `[[sp-24]+116]` (`str` at 0xc5530).
  Call (`bl` at 0xc5558): (unknown callee), target 0x3ec6c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xc5598): (unknown callee), target 0x201fc, r0=`0x50`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xc55b0): (unknown callee), target 0x205bc, r0=`[[sp-28]]`, r1=`0x176760`, r2=`unknown`, r3=`[sp-28]`.
  Recorded r0 = `unknown` (`ldm` at 0xc55d0).
