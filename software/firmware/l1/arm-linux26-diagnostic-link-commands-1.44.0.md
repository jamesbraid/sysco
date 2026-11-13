# ARM Linux 2.6 diagnostic and link command routines in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-diagnostic-link-commands-facts.txt) retain all tool sections.

The notation `ret(ADDR)` identifies a call-result expression without distinguishing
individual call sites to that target. Each return line records machine-register
state from the analysis rather than a C return type.

Groups count static records with the same width, access kind and resolved target expression. Address order need not describe one execution path.

## `pcixcmdInit`

- `pcixcmdInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe0fc8 to 0xe1048.
  Established facts:
  The instructions from 0xe1028 to 0xe1048 are not reached by any path from the entry, and their branches are not listed.
  Word load: target `abs 0x1b0e1c` (`ldr` at 0xe0fd8).
  Branch (`bne` at 0xe0fe0): taken to 0xe1020 when `[0x1b0e1c] != 0x0`, falls through to 0xe0fe4 when `[0x1b0e1c] == 0x0`.
  Call (`bl` at 0xe0fec): (unknown callee), target 0x10184c, r0=`0x182dcc`, r1=`0x19`, r2=`r2`, r3=`[0x1b0e1c]`.
  Word store: target `abs 0x1b0cec`, value `0x7933c` (`str` at 0xe0ff8).
  Word store: target `abs 0x1b0cf0`, value `0x79670` (`str` at 0xe1004).
  Word store: target `abs 0x1b0cf4`, value `0x7995c` (`str` at 0xe1010).
  Word store: target `abs 0x1b0e1c`, value `0x1` (`str` at 0xe101c).
  Recorded r0 = `unknown` (`ldm` at 0xe1024).

## `get_bits`

- `get_bits`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf17c4 to 0xf19a0.
  Established facts:
  The instructions from 0xf199c to 0xf19a0 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0xf17ec): (unknown callee), target 0x20550, r0=`0x1b0fcc`, r1=`0x0`, r2=`0x8`, r3=`r3`.
  Branch (`blt` at 0xf1814): taken to 0xf1930 when `[sp-36] < [sp-32] (signed, 32-bit)`, falls through to 0xf1818 when `[sp-36] >= [sp-32] (signed, 32-bit)`.
  Byte load: target `(unknown effective address)` (`ldrb` at 0xf1838).
  Branch (`beq` at 0xf18b0): taken to 0xf1914 when `[sp-44] == 0x0`, falls through to 0xf18b4 when `[sp-44] != 0x0`.
  Byte load: target `(unknown effective address)` (`ldrb` at 0xf1908).
  Byte store: target `(unknown effective address)`, value `unknown` (`strb` at 0xf1910).
  Jump (`b` at 0xf192c): target 0xf1808.
  Byte load: target `abs 0x1b0fd0` (`ldrb` at 0xf1934).
  Byte load: target `abs 0x1b0fd1` (`ldrb` at 0xf194c).
  Byte load: target `abs 0x1b0fd2` (`ldrb` at 0xf1964).
  Recorded r0 = `[sp-44]` (`ldm` at 0xf1998).

## `Deadlock`

- `Deadlock`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf1b74 to 0xf2ce8.
  Established facts:
  The instructions from 0xf2cd4 to 0xf2ce8 are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0xf1b94): (unknown callee), target 0x6e140, r0=`r0`, r1=`r1`, r2=`r2`, r3=`0x5`.
  Call (`bl` at 0xf1bac): (unknown callee), target 0x201fc, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf1bcc): (unknown callee), target 0x20550, r0=`ret(0x201fc)`, r1=`0x0`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf1bd0): (unknown callee), target 0x6dc0c, r0=`ret(0x20550)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0xf1be8): taken to 0xf1ce4 when `[sp-180] == 0x40`, falls through to 0xf1bec when `[sp-180] != 0x40`.
  Branch (`beq` at 0xf1c00): taken to 0xf1e98 when `[sp-180] == 0x2e`, falls through to 0xf1c04 when `[sp-180] != 0x2e`.
  Branch (`beq` at 0xf1c44): taken to 0xf1c74 when `[sp-180] == 0x4f`, falls through to 0xf1c48 when `[sp-180] != 0x4f`.
  Call (`bl` at 0xf1c7c): (unknown callee), target 0xf1424, r0=`[sp-68]`, r1=`0x1`, r2=`unknown`, r3=`[sp-180]`.
  Call (`bl` at 0xf1c94): (unknown callee), target 0xf1480, r0=`[[sp-68]+4]`, r1=`0x0`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0xf1ca4): (unknown callee), target 0xf1424, r0=`[sp-68]`, r1=`0x2`, r2=`unknown`, r3=`[sp-180]`.
  Call (`bl` at 0xf1cbc): (unknown callee), target 0xf1480, r0=`[[sp-68]+4]`, r1=`0x0`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0xf1cdc): (unknown callee), target 0xf1480, r0=`[[sp-68]+4]+52`, r1=`0x0`, r2=`0x0`, r3=`0x1`.
  Call (`bl` at 0xf1cec): (unknown callee), target 0xf1424, r0=`[sp-68]`, r1=`0x6`, r2=`unknown`, r3=`[sp-180]`.
  Call (`bl` at 0xf1d04): (unknown callee), target 0xf1480, r0=`[[sp-68]+4]`, r1=`0x0`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0xf1d24): (unknown callee), target 0xf1480, r0=`[[sp-68]+4]+52`, r1=`0x0`, r2=`0x0`, r3=`0x1`.
  Call (`bl` at 0xf1d44): (unknown callee), target 0xf1480, r0=`[[sp-68]+4]+104`, r1=`0x0`, r2=`0x0`, r3=`0x2`.
  Call (`bl` at 0xf1d64): (unknown callee), target 0xf1480, r0=`[[sp-68]+4]+156`, r1=`0x0`, r2=`0x0`, r3=`0x3`.
  Call (`bl` at 0xf1d84): (unknown callee), target 0xf1480, r0=`[[sp-68]+4]+208`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0xf1da4): (unknown callee), target 0xf1480, r0=`[[sp-68]+4]+260`, r1=`0x1`, r2=`0x0`, r3=`0x1`.
  Call (`bl` at 0xf1db4): (unknown callee), target 0xf1424, r0=`[sp-68]`, r1=`0x1`, r2=`unknown`, r3=`[sp-180]`.
  Call (`bl` at 0xf1dcc): (unknown callee), target 0xf1480, r0=`[[sp-68]+4]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0xf1ddc): (unknown callee), target 0xf1424, r0=`[sp-68]`, r1=`0x1`, r2=`unknown`, r3=`[sp-180]`.
  Call (`bl` at 0xf1df4): (unknown callee), target 0xf1480, r0=`[[sp-68]+4]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0xf1e08): (unknown callee), target 0xf1424, r0=`[sp-68]+8`, r1=`0x1`, r2=`unknown`, r3=`[sp-68]+8`.
  Call (`bl` at 0xf1e28): (unknown callee), target 0xf1480, r0=`[[sp-68]+12]`, r1=`0x1`, r2=`0x1`, r3=`0x0`.
  Call (`bl` at 0xf1e3c): (unknown callee), target 0xf1424, r0=`[sp-68]+16`, r1=`0x1`, r2=`unknown`, r3=`[sp-68]+16`.
  Call (`bl` at 0xf1e5c): (unknown callee), target 0xf1480, r0=`[[sp-68]+20]`, r1=`0x1`, r2=`0x2`, r3=`0x0`.
  Call (`bl` at 0xf1e70): (unknown callee), target 0xf1424, r0=`[sp-68]+24`, r1=`0x1`, r2=`unknown`, r3=`[sp-68]+24`.
  Call (`bl` at 0xf1e90): (unknown callee), target 0xf1480, r0=`[[sp-68]+28]`, r1=`0x1`, r2=`0x3`, r3=`0x0`.
  Call (`bl` at 0xf1eb4): (unknown callee), target 0x3ec6c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`[sp-40]`.
  Call (`bl` at 0xf1ecc): (unknown callee), target 0x3deec, r0=`[sp-52]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3ec6c)`.
  Call (`bl` at 0xf1fa4): (unknown callee), target 0xf1424, r0=`unknown`, r1=`0x1`, r2=`unknown`, r3=`unknown`.
  Word load: target `(unknown effective address)` (`ldr` at 0xf1fb8).
  Call (`bl` at 0xf1fc8): (unknown callee), target 0xf1480, r0=`unknown`, r1=`0x3`, r2=`[sp-52]`, r3=`0x0`.
  Call (`bl` at 0xf1fe8): (unknown callee), target 0xf1424, r0=`unknown`, r1=`0x2`, r2=`unknown`, r3=`unknown`.
  Word load: target `(unknown effective address)` (`ldr` at 0xf1ffc).
  Call (`bl` at 0xf200c): (unknown callee), target 0xf1480, r0=`unknown`, r1=`0x1`, r2=`[sp-52]`, r3=`0x0`.
  Word load: target `(unknown effective address)` (`ldr` at 0xf2020).
  Call (`bl` at 0xf2038): (unknown callee), target 0xf1480, r0=`unknown`, r1=`0x1`, r2=`[sp-52]`, r3=`0x1`.
  Call (`bl` at 0xf2058): (unknown callee), target 0xf1424, r0=`unknown`, r1=`0x1`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf207c): (unknown callee), target 0xf1480, r0=`unknown`, r1=`0x2`, r2=`[sp-52]`, r3=`0x0`.
  Call (`bl` at 0xf209c): (unknown callee), target 0xf1424, r0=`unknown`, r1=`0x1`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf20c0): (unknown callee), target 0xf1480, r0=`unknown`, r1=`0x1`, r2=`[sp-52]`, r3=`0x0`.
  Call (`bl` at 0xf20e0): (unknown callee), target 0xf1424, r0=`unknown`, r1=`0x2`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf2104): (unknown callee), target 0xf1480, r0=`unknown`, r1=`0x2`, r2=`[sp-52]`, r3=`0x0`.
  Call (`bl` at 0xf2130): (unknown callee), target 0xf1480, r0=`unknown`, r1=`0x2`, r2=`[sp-52]`, r3=`0x1`.
  Call (`bl` at 0xf2144): (unknown callee), target 0x134634, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Word load: target `(unknown effective address)` (`ldr` at 0xf21bc).
  Word load: target `(unknown effective address)` (`ldr` at 0xf21f8).
  Word load: target `(unknown effective address)` (`ldr` at 0xf2200).
  Word load: target `(unknown effective address)` (`ldr` at 0xf2234).
  Word load: target `(unknown effective address)` (`ldr` at 0xf223c).
  Word load: target `(unknown effective address)` (`ldr` at 0xf2270).
  Word load: target `(unknown effective address)` (`ldr` at 0xf2278).
  Word load: target `(unknown effective address)` (`ldr` at 0xf2358).
  Word load: target `(unknown effective address)` (`ldr` at 0xf2384).
  Call (`bl` at 0xf238c): (unknown callee), target 0x13d1a0, r0=`unknown`, r1=`unknown`, r2=`sp-96`, r3=`sp-100`.
  Byte load: target `(unknown effective address)` (`ldrb` at 0xf2414).
  Byte load: target `(unknown effective address)` (`ldrb` at 0xf2428).
  Call (`bl` at 0xf2430): (unknown callee), target 0xf17c4, r0=`sp-108`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Byte load: target `(unknown effective address)` (`ldrb` at 0xf244c).
  Call (`bl` at 0xf2464): (unknown callee), target 0xf19a0, r0=`[sp-84]`, r1=`[sp-76]`, r2=`[sp-60]`, r3=`ret(0xf17c4)`.
  Call (`bl` at 0xf24ac): (unknown callee), target 0x1346fc, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`[sp-36]`.
  Call (`bl` at 0xf24b4): (unknown callee), target 0xf2ce8, r0=`0x64`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf24b8): (unknown callee), target 0x134634, r0=`ret(0xf2ce8)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf24cc): (unknown callee), target 0x1346fc, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf24f4): (unknown callee), target 0x205bc, r0=`[sp-72]`, r1=`0x186bd8`, r2=`[sp-52]`, r3=`[sp-56]`.
  Call (`bl` at 0xf24fc): (unknown callee), target 0x2031c, r0=`[sp-72]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Word load: target `(unknown effective address)` (`ldr` at 0xf2598).
  Call (`bl` at 0xf25e0): (unknown callee), target 0x201fc, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Word load: target `(unknown effective address)` (`ldr` at 0xf2684).
  Word load: target `(unknown effective address)` (`ldr` at 0xf2728).
  Word load: target `(unknown effective address)` (`ldr` at 0xf2730).
  Word load: target `(unknown effective address)` (`ldr` at 0xf2794).
  Branch (`beq` at 0xf27dc): taken to 0xf2b60 when `[sp-28] == 0x0`, falls through to 0xf27e0 when `[sp-28] != 0x0`.
  Call (`bl` at 0xf2860): (unknown callee), target 0x205bc, r0=`[sp-72]`, r1=`0x186c10`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf2868): (unknown callee), target 0x2031c, r0=`[sp-72]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf290c): (unknown callee), target 0x205bc, r0=`[sp-72]`, r1=`0x186c34`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf2914): (unknown callee), target 0x2031c, r0=`[sp-72]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`bne` at 0xf293c): taken to 0xf2b60 when `[sp-28] != 0x2`, falls through to 0xf2940 when `[sp-28] == 0x2`.
  Word load: target `(unknown effective address)` (`ldr` at 0xf29cc).
  Word load: target `(unknown effective address)` (`ldr` at 0xf29d4).
  Word load: target `(unknown effective address)` (`ldr` at 0xf2a24).
  Word load: target `(unknown effective address)` (`ldr` at 0xf2a68).
  Call (`bl` at 0xf2a7c): (unknown callee), target 0x205bc, r0=`sp-172`, r1=`0x186c5c`, r2=`unknown`, r3=`unknown`.
  Word load: target `(unknown effective address)` (`ldr` at 0xf2abc).
  Call (`bl` at 0xf2b30): (unknown callee), target 0x205bc, r0=`[sp-72]`, r1=`0x186c64`, r2=`unknown`, r3=`sp-172`.
  Call (`bl` at 0xf2b38): (unknown callee), target 0x2031c, r0=`[sp-72]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Word load: target `(unknown effective address)` (`ldr` at 0xf2bf4).
  Word load: target `(unknown effective address)` (`ldr` at 0xf2c34).
  Call (`bl` at 0xf2c38): (unknown callee), target 0x20514, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf2c7c): (unknown callee), target 0x20514, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0xf2c98): taken to 0xf2ca4 when `[sp-68] == 0x0`, falls through to 0xf2c9c when `[sp-68] != 0x0`.
  Call (`bl` at 0xf2ca0): (unknown callee), target 0x20514, r0=`[sp-68]`, r1=`unknown`, r2=`unknown`, r3=`[sp-68]`.
  Recorded r0 = `[sp-184]` (`ldm` at 0xf2cd0).

## `smpcmd_tst`

- `smpcmd_tst`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf6a90 to 0xf767c.
  Established facts:
  The instructions from 0xf6b04 to 0xf761c are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0xf7628 to 0xf7630 are not reached by any path from the entry, and their branches are not listed.
  The instructions from 0xf763c to 0xf767c are not reached by any path from the entry, and their branches are not listed.
  Call (`bl` at 0xf6ab0): (unknown callee), target 0x6e140, r0=`r0`, r1=`r1`, r2=`r2`, r3=`0x1`.
  Call (`bl` at 0xf6ac8): (unknown callee), target 0xc5488, r0=`[sp-28]`, r1=`[sp-32]`, r2=`sp-72`, r3=`sp-72`.
  Branch (`beq` at 0xf6adc): taken to 0xf6aec when `ret(0xc5488) == 0x0`, falls through to 0xf6ae0 when `ret(0xc5488) != 0x0` (source `bl` at 0xf6ac8).
  Jump (`b` at 0xf6ae8): target 0xf7630.
  Word load: target `[sp-28] +0x70` (`ldr` at 0xf6af0).
  Word load: target `[[sp-28]+112] +0x0` (`ldr` at 0xf6af4).
  Branch (`ldrls` at 0xf6afc): taken to (unknown target) when `[[[sp-28]+112]] <= 0x8 (unsigned, 32-bit)`, falls through to 0xf6b00 when `[[[sp-28]+112]] > 0x8 (unsigned, 32-bit)`.
  Omitted-path effects remain unknown at the unresolved transfer (`ldrls` at 0xf6afc).
  Jump (`b` at 0xf6b00): target 0xf761c.
  Jump (`b` at 0xf7624): target 0xf7630.
  Known-edge recorded r0 = `unknown` (`ldm` at 0xf7638).

## `verify_length_data`

- `verify_length_data`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x130f24 to 0x1310f0.
  Established facts:
  Word store: target `r3 +0x0`, value `0x0` (`str` at 0x130f4c).
  Word load: target `(unknown effective address)` (`ldr` at 0x130fa8).
  Branch (`beq` at 0x130fb8): taken to 0x13100c when `unknown == 0x0`, falls through to 0x130fbc when `unknown != 0x0`.
  Branch (`bhs` at 0x130fc8): taken to 0x13100c when `C == 1`, falls through to 0x130fcc when `not (C == 1)`.
  Branch (`bne` at 0x130fe0): taken to 0x130ffc when `[sp-48] != 0x80000000`, falls through to 0x130fe4 when `[sp-48] == 0x80000000`.
  Jump (`b` at 0x130ff8): target 0x130f98.
  Jump (`b` at 0x131008): target 0x130f98.
  Word load: target `(unknown effective address)` (`ldr` at 0x131024).
  Branch (`bne` at 0x131034): taken to 0x131088 when `unknown != 0x0`, falls through to 0x131038 when `unknown == 0x0`.
  Branch (`bhs` at 0x131044): taken to 0x131088 when `C == 1`, falls through to 0x131048 when `not (C == 1)`.
  Branch (`bne` at 0x13105c): taken to 0x131078 when `[sp-48] != 0x80000000`, falls through to 0x131060 when `[sp-48] == 0x80000000`.
  Jump (`b` at 0x131074): target 0x131014.
  Jump (`b` at 0x131084): target 0x131014.
  Branch (`bne` at 0x131094): taken to 0x1310bc when `Z == 0`, falls through to 0x131098 when `not (Z == 0)`.
  Word load: target `(unknown effective address)` (`ldr` at 0x1310a8).
  Branch (`bne` at 0x1310b8): taken to 0x1310c8 when `unknown != 0x0`, falls through to 0x1310bc when `unknown == 0x0`.
  Jump (`b` at 0x1310c4): target 0x1310e4.
  Word store: target `[sp-32] +0x0`, value `unknown` (`str` at 0x1310d8).
  Recorded r0 = `unknown` (`ldm` at 0x1310ec).

## `build_sic_tdi`

- `build_sic_tdi`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1310f0 to 0x131348.
  Established facts:
  The instructions from 0x13133c to 0x131348 are not reached by any path from the entry, and their branches are not listed.
  Branch (`bhs` at 0x13112c): taken to 0x131158 when `C == 1`, falls through to 0x131130 when `not (C == 1)`.
  Jump (`b` at 0x131154): target 0x131120.
  Word store: target `r3 +0x0`, value `0x0` (`str` at 0x131170).
  Branch (`bhs` at 0x131188): taken to 0x131320 when `C == 1`, falls through to 0x13118c when `not (C == 1)`.
  Word load: target `(unknown effective address)` (`ldr` at 0x13119c).
  Word load: target `(unknown effective address)` (`ldr` at 0x1312a0).
  Word store: target `(unknown effective address)`, value `unknown` (`str` at 0x1312a8).
  Branch (`bls` at 0x1312c4): taken to 0x131310 when `([sp-36] + [sp-24]) <= 0x1f (unsigned, 32-bit)`, falls through to 0x1312c8 when `([sp-36] + [sp-24]) > 0x1f (unsigned, 32-bit)`.
  Word store: target `(unknown effective address)`, value `unknown` (`str` at 0x13130c).
  Jump (`b` at 0x13131c): target 0x13117c.
  Recorded r0 = `unknown` (`ldm` at 0x131338).

## `add_sic_tdi`

- `add_sic_tdi`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x131348 to 0x131598.
  Established facts:
  The instructions from 0x13158c to 0x131598 are not reached by any path from the entry, and their branches are not listed.
  Branch (`bhs` at 0x131384): taken to 0x1313b0 when `C == 1`, falls through to 0x131388 when `not (C == 1)`.
  Jump (`b` at 0x1313ac): target 0x131378.
  Word load: target `r3 +0x0` (`ldr` at 0x13148c).
  Branch (`bls` at 0x1314a4): taken to 0x1314c4 when `ls flags condition (operands unknown)`, falls through to 0x1314a8 when `not (ls flags condition (operands unknown))`.
  Jump (`b` at 0x1314c0): target 0x13149c.
  Word load: target `(unknown effective address)` (`ldr` at 0x1314f0).
  Word store: target `(unknown effective address)`, value `unknown` (`str` at 0x1314f8).
  Branch (`bls` at 0x131514): taken to 0x131560 when `([sp-40] + [sp-24]) <= 0x20 (unsigned, 32-bit)`, falls through to 0x131518 when `([sp-40] + [sp-24]) > 0x20 (unsigned, 32-bit)`.
  Word store: target `(unknown effective address)`, value `unknown` (`str` at 0x13155c).
  2 such word load accesses with resolved target `[sp-32] +0x0`, including target `[sp-32] +0x0` (`ldr` at 0x131568), target `[sp-32] +0x0` (`ldr` at 0x13157c).
  Word store: target `[sp-32] +0x0`, value `([[sp-32]] + [sp-24])` (`str` at 0x131574).
  Recorded r0 = `[[sp-32]]` (`ldm` at 0x131588).

## `llpcmd_to_tdr`

- `llpcmd_to_tdr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x148c60 to 0x1490bc.
  Established facts:
  4 such word load accesses with resolved target `[sp-28] +0x4`, including target `[sp-28] +0x4` (`ldr` at 0x148c94), target `[sp-28] +0x4` (`ldr` at 0x148d64), target `[sp-28] +0x4` (`ldr` at 0x148dd8).
  Word store: target `(unknown effective address)`, value `unknown` (`str` at 0x148ca0).
  Call (`bl` at 0x148cb4): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]`, r2=`0x0`, r3=`sp-36`.
  Word store: target `(unknown effective address)`, value `0x0` (`str` at 0x148cc8).
  4 such word load accesses with resolved target `[sp-28] +0x10`, including target `[sp-28] +0x10` (`ldr` at 0x148ce4), target `[sp-28] +0x10` (`ldr` at 0x148ec4), target `[sp-28] +0x10` (`ldr` at 0x148f30).
  Word store: target `(unknown effective address)`, value `unknown` (`str` at 0x148cf0).
  Word load: target `(unknown effective address)` (`ldr` at 0x148d0c).
  Word load: target `[sp-28] +0xc` (`ldr` at 0x148d24).
  Word store: target `(unknown effective address)`, value `unknown` (`str` at 0x148d34).
  Call (`bl` at 0x148d48): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]`, r2=`0x1`, r3=`sp-36`.
  Word store: target `(unknown effective address)`, value `unknown` (`str` at 0x148d70).
  Call (`bl` at 0x148d84): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]`, r2=`0x2`, r3=`sp-36`.
  3 such word load accesses with resolved target `[sp-28] +0x8`, including target `[sp-28] +0x8` (`ldr` at 0x148da0), target `[sp-28] +0x8` (`ldr` at 0x148e4c), target `[sp-28] +0x8` (`ldr` at 0x148e88).
  Word store: target `(unknown effective address)`, value `unknown` (`str` at 0x148da8).
  Call (`bl` at 0x148dbc): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]`, r2=`0x3`, r3=`sp-36`.
  Call (`bl` at 0x148df8): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]`, r2=`0x4`, r3=`sp-36`.
  Call (`bl` at 0x148e30): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]`, r2=`0x5`, r3=`sp-36`.
  Call (`bl` at 0x148e6c): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]`, r2=`0x6`, r3=`sp-36`.
  Call (`bl` at 0x148ea8): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]`, r2=`0x7`, r3=`sp-36`.
  Call (`bl` at 0x148ee0): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]`, r2=`0x9`, r3=`sp-36`.
  Word load: target `[sp-28] +0x14` (`ldr` at 0x148efc).
  Call (`bl` at 0x148f14): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]`, r2=`0x8`, r3=`sp-36`.
  Call (`bl` at 0x148f50): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]`, r2=`0xa`, r3=`sp-36`.
  Call (`bl` at 0x148f8c): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]`, r2=`0xb`, r3=`sp-36`.
  2 such word load accesses with resolved target `[sp-28] +0x18`, including target `[sp-28] +0x18` (`ldr` at 0x148fa8), target `[sp-28] +0x18` (`ldr` at 0x148fe8).
  Call (`bl` at 0x148fcc): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]`, r2=`0xe`, r3=`sp-36`.
  Call (`bl` at 0x149008): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]`, r2=`0xf`, r3=`sp-36`.
  Word load: target `[sp-28] +0x1c` (`ldr` at 0x149024).
  Word load: target `[sp-28] +0x20` (`ldr` at 0x149044).
  Word store: target `(unknown effective address)`, value `[[sp-28]+32]` (`str` at 0x149048).
  Call (`bl` at 0x14905c): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]`, r2=`0xd`, r3=`sp-36`.
  Word load: target `[sp-28] +0x24` (`ldr` at 0x149078).
  Word load: target `[sp-28] +0x28` (`ldr` at 0x149098).
  Word store: target `(unknown effective address)`, value `[[sp-28]+40]` (`str` at 0x14909c).
  Call (`bl` at 0x1490b0): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]`, r2=`0xc`, r3=`sp-36`.
  Recorded r0 = `ret(0x13f048)` (`ldm` at 0x1490b8, `bl` at 0x1490b0).

## `llpcmd_from_tdr`

- `llpcmd_from_tdr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1490bc to 0x149640.
  Established facts:
  Call (`bl` at 0x1490e8): (unknown callee), target 0x13eae8, r0=`r0`, r1=`r1`, r2=`0x0`, r3=`sp-36`.
  Word load: target `(unknown effective address)` (`ldr` at 0x149104).
  4 such word load accesses with resolved target `[sp-28] +0x4`, including target `[sp-28] +0x4` (`ldr` at 0x14911c), target `[sp-28] +0x4` (`ldr` at 0x149204), target `[sp-28] +0x4` (`ldr` at 0x1492a8).
  4 such word store accesses with resolved target `[sp-28] +0x4`, including target `[sp-28] +0x4`, value `unknown` (`str` at 0x149128), target `[sp-28] +0x4`, value `unknown` (`str` at 0x149210), target `[sp-28] +0x4`, value `unknown` (`str` at 0x1492b4).
  Call (`bl` at 0x14913c): (unknown callee), target 0x13eae8, r0=`[sp-20]`, r1=`[sp-24]`, r2=`0x1`, r3=`sp-36`.
  Word load: target `(unknown effective address)` (`ldr` at 0x14914c).
  4 such word load accesses with resolved target `[sp-28] +0x10`, including target `[sp-28] +0x10` (`ldr` at 0x149180), target `[sp-28] +0x10` (`ldr` at 0x1493f0), target `[sp-28] +0x10` (`ldr` at 0x149478).
  4 such word store accesses with resolved target `[sp-28] +0x10`, including target `[sp-28] +0x10`, value `unknown` (`str` at 0x14918c), target `[sp-28] +0x10`, value `unknown` (`str` at 0x1493fc), target `[sp-28] +0x10`, value `unknown` (`str` at 0x149484).
  Word load: target `(unknown effective address)` (`ldr` at 0x14919c).
  Word store: target `[sp-28] +0xc`, value `[sp-40]` (`str` at 0x1491bc).
  Call (`bl` at 0x1491d0): (unknown callee), target 0x13eae8, r0=`[sp-20]`, r1=`[sp-24]`, r2=`0x2`, r3=`sp-36`.
  Word load: target `(unknown effective address)` (`ldr` at 0x1491ec).
  Call (`bl` at 0x149224): (unknown callee), target 0x13eae8, r0=`[sp-20]`, r1=`[sp-24]`, r2=`0x3`, r3=`sp-36`.
  Word load: target `(unknown effective address)` (`ldr` at 0x149240).
  3 such word load accesses with resolved target `[sp-28] +0x8`, including target `[sp-28] +0x8` (`ldr` at 0x149254), target `[sp-28] +0x8` (`ldr` at 0x14934c), target `[sp-28] +0x8` (`ldr` at 0x1493a0).
  3 such word store accesses with resolved target `[sp-28] +0x8`, including target `[sp-28] +0x8`, value `unknown` (`str` at 0x149260), target `[sp-28] +0x8`, value `unknown` (`str` at 0x149358), target `[sp-28] +0x8`, value `unknown` (`str` at 0x1493ac).
  Call (`bl` at 0x149274): (unknown callee), target 0x13eae8, r0=`[sp-20]`, r1=`[sp-24]`, r2=`0x4`, r3=`sp-36`.
  Call (`bl` at 0x1492c8): (unknown callee), target 0x13eae8, r0=`[sp-20]`, r1=`[sp-24]`, r2=`0x5`, r3=`sp-36`.
  Call (`bl` at 0x149318): (unknown callee), target 0x13eae8, r0=`[sp-20]`, r1=`[sp-24]`, r2=`0x6`, r3=`sp-36`.
  Call (`bl` at 0x14936c): (unknown callee), target 0x13eae8, r0=`[sp-20]`, r1=`[sp-24]`, r2=`0x7`, r3=`sp-36`.
  Call (`bl` at 0x1493c0): (unknown callee), target 0x13eae8, r0=`[sp-20]`, r1=`[sp-24]`, r2=`0x9`, r3=`sp-36`.
  Call (`bl` at 0x149410): (unknown callee), target 0x13eae8, r0=`[sp-20]`, r1=`[sp-24]`, r2=`0x8`, r3=`sp-36`.
  Word store: target `[sp-28] +0x14`, value `unknown` (`str` at 0x149430).
  Call (`bl` at 0x149444): (unknown callee), target 0x13eae8, r0=`[sp-20]`, r1=`[sp-24]`, r2=`0xa`, r3=`sp-36`.
  Call (`bl` at 0x149498): (unknown callee), target 0x13eae8, r0=`[sp-20]`, r1=`[sp-24]`, r2=`0xb`, r3=`sp-36`.
  Call (`bl` at 0x1494ec): (unknown callee), target 0x13eae8, r0=`[sp-20]`, r1=`[sp-24]`, r2=`0xe`, r3=`sp-36`.
  2 such word load accesses with resolved target `[sp-28] +0x18`, including target `[sp-28] +0x18` (`ldr` at 0x149508), target `[sp-28] +0x18` (`ldr` at 0x149564).
  2 such word store accesses with resolved target `[sp-28] +0x18`, including target `[sp-28] +0x18`, value `unknown` (`str` at 0x149534), target `[sp-28] +0x18`, value `unknown` (`str` at 0x14958c).
  Call (`bl` at 0x149548): (unknown callee), target 0x13eae8, r0=`[sp-20]`, r1=`[sp-24]`, r2=`0xf`, r3=`sp-36`.
  Call (`bl` at 0x1495a0): (unknown callee), target 0x13eae8, r0=`[sp-20]`, r1=`[sp-24]`, r2=`0xd`, r3=`sp-36`.
  Word store: target `[sp-28] +0x1c`, value `unknown` (`str` at 0x1495c0).
  Word load: target `(unknown effective address)` (`ldr` at 0x1495dc).
  Word store: target `[sp-28] +0x20`, value `unknown` (`str` at 0x1495e0).
  Call (`bl` at 0x1495f4): (unknown callee), target 0x13eae8, r0=`[sp-20]`, r1=`[sp-24]`, r2=`0xc`, r3=`sp-36`.
  Word store: target `[sp-28] +0x24`, value `unknown` (`str` at 0x149614).
  Word store: target `[sp-28] +0x28`, value `unknown` (`str` at 0x149634).
  Recorded r0 = `sp-16` (`ldm` at 0x14963c).

## `llpcmd_create`

- `llpcmd_create`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x14f640 to 0x14f690.
  Established facts:
  Call (`bl` at 0x14f654): (unknown callee), target 0x201fc, r0=`0x2c`, r1=`r1`, r2=`r2`, r3=`r3`.
  Branch (`bne` at 0x14f664): taken to 0x14f674 when `ret(0x201fc) != 0x0`, falls through to 0x14f668 when `ret(0x201fc) == 0x0` (source `bl` at 0x14f654).
  Call (`bl` at 0x14f678): (unknown callee), target 0x14f690, r0=`ret(0x201fc)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x201fc)`.
  Recorded r0 = `unknown` (`ldm` at 0x14f68c).

## `llpcmd_init`

- `llpcmd_init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x14f690 to 0x14f6c4.
  Established facts:
  Branch (`beq` at 0x14f6ac): taken to 0x14f6c0 when `r0 == 0x0`, falls through to 0x14f6b0 when `r0 != 0x0`.
  Call (`bl` at 0x14f6bc): (unknown callee), target 0x20550, r0=`r0`, r1=`0x0`, r2=`0x2c`, r3=`r0`.
  Recorded r0 = `unknown` (`ldm` at 0x14f6c0).

## `llpcmd_set_drvr`

- `llpcmd_set_drvr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x14f6c4 to 0x14f784.
  Established facts:
  Branch (`beq` at 0x14f6fc): taken to 0x14f708 when `(r0 & 0xff) == 0x0`, falls through to 0x14f700 when `(r0 & 0xff) != 0x0`.
  Call (`bl` at 0x14f704): (unknown callee), target 0x14f690, r0=`[sp]`, r1=`r3`, r2=`r2`, r3=`(r0 & 0xff)`.
  Word store: target `[sp] +0xc`, value `unknown` (`str` at 0x14f718).
  Word load: target `[sp] +0x10` (`ldr` at 0x14f73c).
  Word store: target `[sp] +0x10`, value `unknown` (`str` at 0x14f748).
  Word load: target `[sp] +0x4` (`ldr` at 0x14f76c).
  Word store: target `[sp] +0x4`, value `unknown` (`str` at 0x14f778).
  Recorded r0 = `unknown` (`ldm` at 0x14f780).

## `llpcmd_set_rcvr`

- `llpcmd_set_rcvr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x14f784 to 0x14f860.
  Established facts:
  Branch (`beq` at 0x14f7c0): taken to 0x14f7cc when `(r0 & 0xff) == 0x0`, falls through to 0x14f7c4 when `(r0 & 0xff) != 0x0`.
  Call (`bl` at 0x14f7c8): (unknown callee), target 0x14f690, r0=`[sp]`, r1=`r1`, r2=`r2`, r3=`(r0 & 0xff)`.
  3 such word load accesses with resolved target `[sp] +0x8`, including target `[sp] +0x8` (`ldr` at 0x14f7ec), target `[sp] +0x8` (`ldr` at 0x14f81c), target `[sp] +0x8` (`ldr` at 0x14f84c).
  3 such word store accesses with resolved target `[sp] +0x8`, including target `[sp] +0x8`, value `unknown` (`str` at 0x14f7f8), target `[sp] +0x8`, value `unknown` (`str` at 0x14f828), target `[sp] +0x8`, value `unknown` (`str` at 0x14f858).
  Recorded r0 = `unknown` (`ldm` at 0x14f85c).

## `llpcmd_build_old_fmt`

- `llpcmd_build_old_fmt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x14f860 to 0x14facc.
  Established facts:
  Word store: target `r1 +0x0`, value `0x0` (`str` at 0x14f880).
  4 such word store accesses with resolved target `[sp-24] +0x4`, including target `[sp-24] +0x4`, value `0x0` (`str` at 0x14f894), target `[sp-24] +0x4`, value `unknown` (`str` at 0x14fa34), target `[sp-24] +0x4`, value `unknown` (`str` at 0x14fa74).
  Word load: target `[sp-20] +0x4` (`ldr` at 0x14f8a4).
  Branch (`beq` at 0x14f8b4): taken to 0x14f8cc when `unknown == 0x0`, falls through to 0x14f8b8 when `unknown != 0x0`.
  7 such word load accesses with resolved target `[sp-24] +0x0`, including target `[sp-24] +0x0` (`ldr` at 0x14f8c0), target `[sp-24] +0x0` (`ldr` at 0x14f8f4), target `[sp-24] +0x0` (`ldr` at 0x14f928).
  7 such word store accesses with resolved target `[sp-24] +0x0`, including target `[sp-24] +0x0`, value `unknown` (`str` at 0x14f8c8), target `[sp-24] +0x0`, value `unknown` (`str` at 0x14f8fc), target `[sp-24] +0x0`, value `unknown` (`str` at 0x14f930).
  3 such word load accesses with resolved target `[sp-20] +0x8`, including target `[sp-20] +0x8` (`ldr` at 0x14f8d8), target `[sp-20] +0x8` (`ldr` at 0x14f90c), target `[sp-20] +0x8` (`ldr` at 0x14f948).
  Branch (`beq` at 0x14f8e8): taken to 0x14f900 when `unknown == 0x0`, falls through to 0x14f8ec when `unknown != 0x0`.
  Branch (`beq` at 0x14f91c): taken to 0x14f934 when `unknown == 0x0`, falls through to 0x14f920 when `unknown != 0x0`.
  3 such word load accesses with resolved target `[sp-20] +0x10`, including target `[sp-20] +0x10` (`ldr` at 0x14f970), target `[sp-20] +0x10` (`ldr` at 0x14f9a4), target `[sp-20] +0x10` (`ldr` at 0x14fa5c).
  Branch (`beq` at 0x14f980): taken to 0x14f998 when `unknown == 0x0`, falls through to 0x14f984 when `unknown != 0x0`.
  Branch (`beq` at 0x14f9b4): taken to 0x14f9cc when `unknown == 0x0`, falls through to 0x14f9b8 when `unknown != 0x0`.
  2 such word load accesses with resolved target `[sp-20] +0x1c`, including target `[sp-20] +0x1c` (`ldr` at 0x14f9e0), target `[sp-20] +0x1c` (`ldr` at 0x14fa20).
  3 such word load accesses with resolved target `[sp-24] +0x4`, including target `[sp-24] +0x4` (`ldr` at 0x14fa2c), target `[sp-24] +0x4` (`ldr` at 0x14fa6c), target `[sp-24] +0x4` (`ldr` at 0x14faa8).
  Word load: target `[sp-20] +0xc` (`ldr` at 0x14fa9c).
  Call (`bl` at 0x14fac0): (unknown callee), target 0x13262c, r0=`0x0`, r1=`0x40`, r2=`[sp-24]`, r3=`unknown`.
  Recorded r0 = `ret(0x13262c)` (`ldm` at 0x14fac8, `bl` at 0x14fac0).

## `llpcmd_extract_old_fmt`

- `llpcmd_extract_old_fmt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x14facc to 0x14fd6c.
  Established facts:
  Call (`bl` at 0x14faf0): (unknown callee), target 0x13262c, r0=`0x0`, r1=`0x40`, r2=`r0`, r3=`r3`.
  Branch (`bhi` at 0x14fb04): taken to 0x14fb30 when `hi flags condition (operands unknown)`, falls through to 0x14fb08 when `not (hi flags condition (operands unknown))`.
  Word store: target `(unknown effective address)`, value `0x0` (`str` at 0x14fb1c).
  7 such word load accesses with resolved target `[sp-20] +0x0`, including target `[sp-20] +0x0` (`ldr` at 0x14fb34), target `[sp-20] +0x0` (`ldr` at 0x14fb70), target `[sp-20] +0x0` (`ldr` at 0x14fbac).
  Branch (`beq` at 0x14fb40): taken to 0x14fb6c when `unknown == 0x0`, falls through to 0x14fb44 when `unknown != 0x0`.
  Word load: target `[sp-24] +0x4` (`ldr` at 0x14fb5c).
  Word store: target `[sp-24] +0x4`, value `unknown` (`str` at 0x14fb68).
  Branch (`beq` at 0x14fb7c): taken to 0x14fba8 when `unknown == 0x0`, falls through to 0x14fb80 when `unknown != 0x0`.
  3 such word load accesses with resolved target `[sp-24] +0x8`, including target `[sp-24] +0x8` (`ldr` at 0x14fb98), target `[sp-24] +0x8` (`ldr` at 0x14fbd4), target `[sp-24] +0x8` (`ldr` at 0x14fc1c).
  3 such word store accesses with resolved target `[sp-24] +0x8`, including target `[sp-24] +0x8`, value `unknown` (`str` at 0x14fba4), target `[sp-24] +0x8`, value `unknown` (`str` at 0x14fbe0), target `[sp-24] +0x8`, value `unknown` (`str` at 0x14fc28).
  Branch (`beq` at 0x14fbb8): taken to 0x14fbe4 when `unknown == 0x0`, falls through to 0x14fbbc when `unknown != 0x0`.
  Branch (`beq` at 0x14fc3c): taken to 0x14fc68 when `unknown == 0x0`, falls through to 0x14fc40 when `unknown != 0x0`.
  3 such word load accesses with resolved target `[sp-24] +0x10`, including target `[sp-24] +0x10` (`ldr` at 0x14fc58), target `[sp-24] +0x10` (`ldr` at 0x14fc94), target `[sp-24] +0x10` (`ldr` at 0x14fd2c).
  3 such word store accesses with resolved target `[sp-24] +0x10`, including target `[sp-24] +0x10`, value `unknown` (`str` at 0x14fc64), target `[sp-24] +0x10`, value `unknown` (`str` at 0x14fca0), target `[sp-24] +0x10`, value `unknown` (`str` at 0x14fd38).
  Branch (`beq` at 0x14fc78): taken to 0x14fca4 when `unknown == 0x0`, falls through to 0x14fc7c when `unknown != 0x0`.
  3 such word load accesses with resolved target `[sp-20] +0x4`, including target `[sp-20] +0x4` (`ldr` at 0x14fcc4), target `[sp-20] +0x4` (`ldr` at 0x14fcf8), target `[sp-20] +0x4` (`ldr` at 0x14fd54).
  Word store: target `[sp-24] +0x1c`, value `[sp-28]` (`str` at 0x14fce8).
  Word store: target `[sp-24] +0xc`, value `unknown` (`str` at 0x14fd60).
  Recorded r0 = `unknown` (`ldm` at 0x14fd68).
