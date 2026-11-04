# ARM Linux 2.6 board probe and interrupt routines in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-board-probe-interrupt-facts.txt) retain all tool sections.

The notation `ret(ADDR)` identifies a call-result expression without distinguishing
individual call sites to that target. Each return line records machine-register
state from the analysis rather than a C return type.

Groups count static records with the same width, access kind and resolved target expression. Address order need not describe one execution path.

## `IP69_Probe`

- `IP69_Probe`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x41194 to 0x411f0.
  Established facts:
  The instructions from 0x411e8 to 0x411f0 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x411b4): (unknown callee), target 0x200ac, r0=`r1`, r1=`0x15ad88`, r2=`r2`, r3=`r3`.
  Branch (`bne` at 0x411c0): taken to 0x411d0 when `ret(0x200ac) != 0x0`, falls through to 0x411c4 when `ret(0x200ac) == 0x0` (source `bl` at 0x411b4).
  Recorded r0 = `[sp-28]` (`ldm` at 0x411e4).

## `IP63_Probe`

- `IP63_Probe`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x463f0 to 0x4644c.
  Established facts:
  The instructions from 0x46444 to 0x4644c are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x46410): (unknown callee), target 0x200ac, r0=`r1`, r1=`0x15c3d4`, r2=`r2`, r3=`r3`.
  Branch (`bne` at 0x4641c): taken to 0x4642c when `ret(0x200ac) != 0x0`, falls through to 0x46420 when `ret(0x200ac) == 0x0` (source `bl` at 0x46410).
  Recorded r0 = `[sp-28]` (`ldm` at 0x46440).

## `Daytona_Probe`

- `Daytona_Probe`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x46e10 to 0x46e6c.
  Established facts:
  The instructions from 0x46e64 to 0x46e6c are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x46e30): (unknown callee), target 0x200ac, r0=`r1`, r1=`0x15ca5c`, r2=`r2`, r3=`r3`.
  Branch (`bne` at 0x46e3c): taken to 0x46e4c when `ret(0x200ac) != 0x0`, falls through to 0x46e40 when `ret(0x200ac) == 0x0` (source `bl` at 0x46e30).
  Recorded r0 = `[sp-28]` (`ldm` at 0x46e60).

## `Abacus_Probe`

- `Abacus_Probe`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x47d8c to 0x47de8.
  Established facts:
  The instructions from 0x47de0 to 0x47de8 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x47dac): (unknown callee), target 0x200ac, r0=`r1`, r1=`0x15d1b8`, r2=`r2`, r3=`r3`.
  Branch (`bne` at 0x47db8): taken to 0x47dc8 when `ret(0x200ac) != 0x0`, falls through to 0x47dbc when `ret(0x200ac) == 0x0` (source `bl` at 0x47dac).
  Recorded r0 = `[sp-28]` (`ldm` at 0x47ddc).

## `PCIXE_Probe`

- `PCIXE_Probe`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x483c4 to 0x484cc.
  Established facts:
  The instructions from 0x484b4 to 0x484cc are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x483e4): (unknown callee), target 0x200ac, r0=`r1`, r1=`0x15d5c4`, r2=`r2`, r3=`r3`.
  Branch (`beq` at 0x483f0): taken to 0x48400 when `ret(0x200ac) == 0x0`, falls through to 0x483f4 when `ret(0x200ac) != 0x0` (source `bl` at 0x483e4).
  Call (`bl` at 0x48410): (unknown callee), target 0x11ff70, r0=`[sp-20]`, r1=`0x1`, r2=`sp-144`, r3=`sp-144`.
  Branch (`bne` at 0x4841c): taken to 0x48484 when `ret(0x11ff70) != 0x0`, falls through to 0x48420 when `ret(0x11ff70) == 0x0` (source `bl` at 0x48410).
  Call (`bl` at 0x48428): (unknown callee), target 0x200ac, r0=`[sp-68]`, r1=`0x15d5d4`, r2=`unknown`, r3=`ret(0x11ff70)`.
  Branch (`bne` at 0x48434): taken to 0x48444 when `ret(0x200ac) != 0x0`, falls through to 0x48438 when `ret(0x200ac) == 0x0` (source `bl` at 0x48428).
  Call (`bl` at 0x48454): (unknown callee), target 0x120154, r0=`sp-144`, r1=`unknown`, r2=`unknown`, r3=`sp-144`.
  Branch (`beq` at 0x48460): taken to 0x48470 when `[sp-148] == 0x0`, falls through to 0x48464 when `[sp-148] != 0x0`.
  Call (`bl` at 0x4849c): (unknown callee), target 0x22070, r0=`0x1`, r1=`0x15d618`, r2=`0x393`, r3=`0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x484b0).

## `BaseIO_Probe`

- `BaseIO_Probe`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x48b30 to 0x48b8c.
  Established facts:
  The instructions from 0x48b84 to 0x48b8c are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x48b50): (unknown callee), target 0x200ac, r0=`r1`, r1=`0x15d910`, r2=`r2`, r3=`r3`.
  Branch (`bne` at 0x48b5c): taken to 0x48b6c when `ret(0x200ac) != 0x0`, falls through to 0x48b60 when `ret(0x200ac) == 0x0` (source `bl` at 0x48b50).
  Recorded r0 = `[sp-28]` (`ldm` at 0x48b80).

## `RTR1010_Probe`

- `RTR1010_Probe`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x49034 to 0x49090.
  Established facts:
  The instructions from 0x49088 to 0x49090 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x49054): (unknown callee), target 0x200ac, r0=`r1`, r1=`0x15ddc8`, r2=`r2`, r3=`r3`.
  Branch (`bne` at 0x49060): taken to 0x49070 when `ret(0x200ac) != 0x0`, falls through to 0x49064 when `ret(0x200ac) == 0x0` (source `bl` at 0x49054).
  Recorded r0 = `[sp-28]` (`ldm` at 0x49084).

## `IP73_Probe`

- `IP73_Probe`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4bb94 to 0x4bcc4.
  Established facts:
  The instructions from 0x4bcac to 0x4bcc4 are not reached by any path from the entry, and their branches are not listed.
  Word load: target `abs 0x19fc94` (`ldr` at 0x4bbb0).
  Branch (`beq` at 0x4bbb8): taken to 0x4bbcc when `[0x19fc94] == 0x0`, falls through to 0x4bbbc when `[0x19fc94] != 0x0`.
  Word store: target `abs 0x19fc94`, value `0x0` (`str` at 0x4bbc4).
  Call (`bl` at 0x4bbc8): (unknown callee), target 0x4b85c, r0=`r0`, r1=`r1`, r2=`0x19fc94`, r3=`0x0`.
  Branch (`bgt` at 0x4bbdc): taken to 0x4bc50 when `gt flags condition (operands unknown)`, falls through to 0x4bbe0 when `not (gt flags condition (operands unknown))`.
  Word load: target `(unknown effective address)` (`ldr` at 0x4bc08).
  Call (`bl` at 0x4bc0c): (unknown callee), target 0x200ac, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`bne` at 0x4bc18): taken to 0x4bc40 when `ret(0x200ac) != 0x0`, falls through to 0x4bc1c when `ret(0x200ac) == 0x0` (source `bl` at 0x4bc0c).
  Call (`bl` at 0x4bc58): (unknown callee), target 0x200ac, r0=`unknown`, r1=`0x15f20c`, r2=`unknown`, r3=`unknown`.
  Branch (`bne` at 0x4bc64): taken to 0x4bc74 when `ret(0x200ac) != 0x0`, falls through to 0x4bc68 when `ret(0x200ac) == 0x0` (source `bl` at 0x4bc58).
  Call (`bl` at 0x4bc7c): (unknown callee), target 0x200ac, r0=`[sp-24]`, r1=`0x15f214`, r2=`unknown`, r3=`ret(0x200ac)`.
  Branch (`bne` at 0x4bc88): taken to 0x4bc98 when `ret(0x200ac) != 0x0`, falls through to 0x4bc8c when `ret(0x200ac) == 0x0` (source `bl` at 0x4bc7c).
  Recorded r0 = `unknown` (`ldm` at 0x4bca8).

## `PCIXHS_Probe`

- `PCIXHS_Probe`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4cb44 to 0x4cc4c.
  Established facts:
  The instructions from 0x4cc34 to 0x4cc4c are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x4cb64): (unknown callee), target 0x200ac, r0=`r1`, r1=`0x15f6d4`, r2=`r2`, r3=`r3`.
  Branch (`beq` at 0x4cb70): taken to 0x4cb80 when `ret(0x200ac) == 0x0`, falls through to 0x4cb74 when `ret(0x200ac) != 0x0` (source `bl` at 0x4cb64).
  Call (`bl` at 0x4cb90): (unknown callee), target 0x11ff70, r0=`[sp-20]`, r1=`0x1`, r2=`sp-144`, r3=`sp-144`.
  Branch (`bne` at 0x4cb9c): taken to 0x4cc04 when `ret(0x11ff70) != 0x0`, falls through to 0x4cba0 when `ret(0x11ff70) == 0x0` (source `bl` at 0x4cb90).
  Call (`bl` at 0x4cba8): (unknown callee), target 0x200ac, r0=`[sp-68]`, r1=`0x15f6e4`, r2=`unknown`, r3=`ret(0x11ff70)`.
  Branch (`bne` at 0x4cbb4): taken to 0x4cbc4 when `ret(0x200ac) != 0x0`, falls through to 0x4cbb8 when `ret(0x200ac) == 0x0` (source `bl` at 0x4cba8).
  Call (`bl` at 0x4cbd4): (unknown callee), target 0x120154, r0=`sp-144`, r1=`unknown`, r2=`unknown`, r3=`sp-144`.
  Branch (`beq` at 0x4cbe0): taken to 0x4cbf0 when `[sp-148] == 0x0`, falls through to 0x4cbe4 when `[sp-148] != 0x0`.
  Call (`bl` at 0x4cc1c): (unknown callee), target 0x22070, r0=`0x1`, r1=`0x15f724`, r2=`0x37f`, r3=`0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x4cc30).

## `QDRTR_Probe`

- `QDRTR_Probe`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4d8ec to 0x4d948.
  Established facts:
  The instructions from 0x4d940 to 0x4d948 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x4d90c): (unknown callee), target 0x200ac, r0=`r1`, r1=`0x15fc98`, r2=`r2`, r3=`r3`.
  Branch (`bne` at 0x4d918): taken to 0x4d928 when `ret(0x200ac) != 0x0`, falls through to 0x4d91c when `ret(0x200ac) == 0x0` (source `bl` at 0x4d90c).
  Recorded r0 = `[sp-28]` (`ldm` at 0x4d93c).

## `RTR55_Probe`

- `RTR55_Probe`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x50518 to 0x50574.
  Established facts:
  The instructions from 0x5056c to 0x50574 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x50538): (unknown callee), target 0x200ac, r0=`r1`, r1=`0x1608ec`, r2=`r2`, r3=`r3`.
  Branch (`bne` at 0x50544): taken to 0x50554 when `ret(0x200ac) != 0x0`, falls through to 0x50548 when `ret(0x200ac) == 0x0` (source `bl` at 0x50538).
  Recorded r0 = `[sp-28]` (`ldm` at 0x50568).

## `IX2S_Probe`

- `IX2S_Probe`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x51380 to 0x51488.
  Established facts:
  The instructions from 0x51470 to 0x51488 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x513a0): (unknown callee), target 0x200ac, r0=`r1`, r1=`0x16100c`, r2=`r2`, r3=`r3`.
  Branch (`beq` at 0x513ac): taken to 0x513bc when `ret(0x200ac) == 0x0`, falls through to 0x513b0 when `ret(0x200ac) != 0x0` (source `bl` at 0x513a0).
  Call (`bl` at 0x513cc): (unknown callee), target 0x11ff70, r0=`[sp-20]`, r1=`0x1`, r2=`sp-144`, r3=`sp-144`.
  Branch (`bne` at 0x513d8): taken to 0x51440 when `ret(0x11ff70) != 0x0`, falls through to 0x513dc when `ret(0x11ff70) == 0x0` (source `bl` at 0x513cc).
  Call (`bl` at 0x513e4): (unknown callee), target 0x200ac, r0=`[sp-68]`, r1=`0x161018`, r2=`unknown`, r3=`ret(0x11ff70)`.
  Branch (`bne` at 0x513f0): taken to 0x51400 when `ret(0x200ac) != 0x0`, falls through to 0x513f4 when `ret(0x200ac) == 0x0` (source `bl` at 0x513e4).
  Call (`bl` at 0x51410): (unknown callee), target 0x120154, r0=`sp-144`, r1=`unknown`, r2=`unknown`, r3=`sp-144`.
  Branch (`beq` at 0x5141c): taken to 0x5142c when `[sp-148] == 0x0`, falls through to 0x51420 when `[sp-148] != 0x0`.
  Call (`bl` at 0x51458): (unknown callee), target 0x22070, r0=`0x1`, r1=`0x161058`, r2=`0x2f7`, r3=`0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x5146c).

## `IE2S_Probe`

- `IE2S_Probe`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x51a10 to 0x51b28.
  Established facts:
  The instructions from 0x51b0c to 0x51b28 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x51a30): (unknown callee), target 0x200ac, r0=`r1`, r1=`0x161450`, r2=`r2`, r3=`r3`.
  Branch (`beq` at 0x51a3c): taken to 0x51a4c when `ret(0x200ac) == 0x0`, falls through to 0x51a40 when `ret(0x200ac) != 0x0` (source `bl` at 0x51a30).
  Call (`bl` at 0x51a5c): (unknown callee), target 0x11ff70, r0=`[sp-20]`, r1=`0x1`, r2=`sp-144`, r3=`sp-144`.
  Branch (`bne` at 0x51a68): taken to 0x51adc when `ret(0x11ff70) != 0x0`, falls through to 0x51a6c when `ret(0x11ff70) == 0x0` (source `bl` at 0x51a5c).
  Call (`bl` at 0x51a74): (unknown callee), target 0x200ac, r0=`[sp-68]`, r1=`0x16145c`, r2=`unknown`, r3=`ret(0x11ff70)`.
  Branch (`bne` at 0x51a80): taken to 0x51a90 when `ret(0x200ac) != 0x0`, falls through to 0x51a84 when `ret(0x200ac) == 0x0` (source `bl` at 0x51a74).
  Call (`bl` at 0x51a98): (unknown callee), target 0x243d4, r0=`0x16146c`, r1=`[sp-68]`, r2=`unknown`, r3=`ret(0x200ac)`.
  Call (`bl` at 0x51aac): (unknown callee), target 0x120154, r0=`sp-144`, r1=`unknown`, r2=`unknown`, r3=`sp-144`.
  Branch (`beq` at 0x51ab8): taken to 0x51ac8 when `[sp-148] == 0x0`, falls through to 0x51abc when `[sp-148] != 0x0`.
  Call (`bl` at 0x51af4): (unknown callee), target 0x22070, r0=`0x1`, r1=`0x1614c4`, r2=`0x36b`, r3=`0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x51b08).

## `ip69Int`

- `ip69Int`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x44390 to 0x443dc.
  Established facts:
  The instructions from 0x443d8 to 0x443dc are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0x443d0): (unknown callee), target 0x29908, r0=`0x15b8b0`, r1=`(r0 & 0xff)`, r2=`(r1 & 0xff)`, r3=`(r2 & 0xff)`.
  Recorded r0 = `ret(0x29908)` (`ldm` at 0x443d4, `bl` at 0x443d0).

## `daytona_vsc055_int`

- `daytona_vsc055_int`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x46764 to 0x467c4.
  Established facts:
  Call (`bl` at 0x467b0): (unknown callee), target 0x1289e8, r0=`(r1 & 0xff)-3`, r1=`(r2 & 0xff)`, r2=`(r3 & 0xff)`, r3=`(r1 & 0xff)-3`.
  Recorded r0 = `ret(0x1289e8)` (`ldm` at 0x467c0, `bl` at 0x467b0).
