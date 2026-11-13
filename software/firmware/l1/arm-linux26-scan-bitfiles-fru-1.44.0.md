# ARM Linux 2.6 scan bitfiles and FRU helpers in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-scan-bitfiles-fru-facts.txt) retain all tool sections.

The notation `ret(ADDR)` identifies a call-result expression without distinguishing
individual call sites to that target. Each return line records machine-register
state from the analysis rather than a C return type.

## `Abacus_JTAG_Sel`

- `Abacus_JTAG_Sel`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4740c to 0x4747c.
  Established facts:
  Branch (`beq` at 0x47430): taken to 0x47440 when `(r1 & 0xff) == 0x0`, falls through to 0x47434 when `(r1 & 0xff) != 0x0`.
  Call (`bl` at 0x47460): (unknown callee), target 0x125a58, r0=`[sp-20]`, r1=`0x8`, r2=`0x80`, r3=`([sp-23]:1 & 0xff)`.
  Recorded r0 = `(ret(0x125a58) & 0xff)` (`ldm` at 0x47478).

## `lbi_jtag_verify`

- `lbi_jtag_verify`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x149b04 to 0x149e50.
  Established facts:
  Word store: target `r0 +0x0`, value `0x0` (`str` at 0x149b2c).
  Call (`bl` at 0x149b3c): (unknown callee), target 0x12cdbc, r0=`0x1`, r1=`[sp-48]`, r2=`[sp-24]`, r3=`0x0`.
  Word store: target `(unknown effective address)`, value `0xa800ffea` (`str` at 0x149b70).
  Word store: target `(unknown effective address)`, value `0xabcdef01` (`str` at 0x149b84).
  Call (`bl` at 0x149be0): (unknown callee), target 0x12d2b8, r0=`0x1`, r1=`0x1`, r2=`0x1`, r3=`[sp-48]`.
  Call (`bl` at 0x149dc8): (unknown callee), target 0x205bc, r0=`[sp-24]`, r1=`0x1953dc`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x149e0c): (unknown callee), target 0x205bc, r0=`[sp-24]`, r1=`0x195408`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `[sp-52]` (`ldm` at 0x149e2c).

## `lbi_jtag_configure`

- `lbi_jtag_configure`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x149e50 to 0x14a584.
  Established facts:
  Call (`bl` at 0x149e78): (unknown callee), target 0x201fc, r0=`unknown`, r1=`r1`, r2=`r2`, r3=`unknown`.
  Call (`bl` at 0x149e98): (unknown callee), target 0x205bc, r0=`[sp-28]`, r1=`0x195434`, r2=`unknown`, r3=`ret(0x201fc)`.
  Call (`bl` at 0x149eb8): (unknown callee), target 0x243d4, r0=`0x195464`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x149ec4): (unknown callee), target 0x14ea70, r0=`[sp-20]`, r1=`[sp-28]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x149ef0): (unknown callee), target 0x14f4fc, r0=`0x0`, r1=`ret(0x14ea70)`, r2=`unknown`, r3=`ret(0x14ea70)`.
  Call (`bl` at 0x149efc): (unknown callee), target 0x200dc, r0=`[0x1a2888]`, r1=`unknown`, r2=`unknown`, r3=`0x1a2888`.
  Call (`bl` at 0x149f0c): (unknown callee), target 0x14a584, r0=`[sp-44]`, r1=`[sp-48]`, r2=`[sp-28]`, r3=`unknown`.
  Call (`bl` at 0x149f24): (unknown callee), target 0x20400, r0=`[[sp-44]+28]`, r1=`unknown`, r2=`unknown`, r3=`[sp-44]`.
  Call (`bl` at 0x149f30): (unknown callee), target 0x14f418, r0=`sp-44`, r1=`unknown`, r2=`unknown`, r3=`sp-44`.
  Word store: target `[sp-48] +0x0`, value `0xa0000000` (`str` at 0x149f50).
  Call (`bl` at 0x149f98): (unknown callee), target 0x12d2b8, r0=`0x0`, r1=`0x1`, r2=`0x1`, r3=`0x0`.
  Call (`bl` at 0x14a168): (unknown callee), target 0x243d4, r0=`0x19548c`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x14a170): (unknown callee), target 0x243d4, r0=`0x1954b0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x14a1a0): (unknown callee), target 0x14f1a0, r0=`[sp-44]`, r1=`[sp-36]`, r2=`[sp-48]`, r3=`[[sp-44]+24]`.
  Call (`bl` at 0x14a1bc): (unknown callee), target 0x20400, r0=`[[sp-44]+28]`, r1=`unknown`, r2=`unknown`, r3=`[sp-44]`.
  Call (`bl` at 0x14a1c8): (unknown callee), target 0x14f418, r0=`sp-44`, r1=`unknown`, r2=`unknown`, r3=`sp-44`.
  Call (`bl` at 0x14a214): (unknown callee), target 0x12d2b8, r0=`0x1`, r1=`[sp-60]`, r2=`0x6`, r3=`0x0`.
  Call (`bl` at 0x14a24c): (unknown callee), target 0x12cee4, r0=`0x6`, r1=`0x0`, r2=`0x0`, r3=`[sp-28]`.
  Call (`bl` at 0x14a264): (unknown callee), target 0x20400, r0=`[[sp-44]+28]`, r1=`unknown`, r2=`unknown`, r3=`[sp-44]`.
  Call (`bl` at 0x14a270): (unknown callee), target 0x14f418, r0=`sp-44`, r1=`unknown`, r2=`unknown`, r3=`sp-44`.
  Call (`bl` at 0x14a28c): (unknown callee), target 0x14ac44, r0=`[sp-44]`, r1=`[sp-48]`, r2=`[sp-28]`, r3=`ret(0x12cee4)`.
  Call (`bl` at 0x14a2a4): (unknown callee), target 0x20400, r0=`[[sp-44]+28]`, r1=`unknown`, r2=`unknown`, r3=`[sp-44]`.
  Call (`bl` at 0x14a2b0): (unknown callee), target 0x14f418, r0=`sp-44`, r1=`unknown`, r2=`unknown`, r3=`sp-44`.
  Call (`bl` at 0x14a2c8): (unknown callee), target 0x20400, r0=`[[sp-44]+28]`, r1=`unknown`, r2=`unknown`, r3=`[sp-44]`.
  Call (`bl` at 0x14a2d4): (unknown callee), target 0x14f418, r0=`sp-44`, r1=`unknown`, r2=`unknown`, r3=`sp-44`.
  Word store: target `[sp-48] +0x0`, value `0xfc000000` (`str` at 0x14a2e8).
  Call (`bl` at 0x14a330): (unknown callee), target 0x12d2b8, r0=`0x0`, r1=`0x1`, r2=`0x1`, r3=`0x0`.
  Call (`bl` at 0x14a500): (unknown callee), target 0x243d4, r0=`0x1954e8`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x14a538): (unknown callee), target 0x205bc, r0=`[sp-28]`, r1=`0x19550c`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `[sp-64]` (`ldm` at 0x14a558).

## `bitfile_open`

- `bitfile_open`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x14ea70 to 0x14ee74.
  Established facts:
  Call (`bl` at 0x14ea8c): (unknown callee), target 0x14f378, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0x14eaac): (unknown callee), target 0x205bc, r0=`[sp-28]`, r1=`0x195c14`, r2=`unknown`, r3=`ret(0x14f378)`.
  Call (`bl` at 0x14eac8): (unknown callee), target 0x20580, r0=`[sp-24]`, r1=`0x195c44`, r2=`unknown`, r3=`ret(0x14f378)`.
  Word store: target `ret(0x14f378) +0x1c`, value `ret(0x20580)` (`str` at 0x14eacc, `bl` at 0x14eac8, `bl` at 0x14ea8c).
  Call (`bl` at 0x14eae4): (unknown callee), target 0x205bc, r0=`[sp-28]`, r1=`0x195c48`, r2=`unknown`, r3=`[ret(0x14f378)+28]`.
  Call (`bl` at 0x14eaf0): (unknown callee), target 0x14f418, r0=`sp-296`, r1=`unknown`, r2=`unknown`, r3=`sp-296`.
  Call (`bl` at 0x14eb08): (unknown callee), target 0x14ee74, r0=`[[sp-296]+28]`, r1=`unknown`, r2=`unknown`, r3=`[sp-296]`.
  Call (`bl` at 0x14eb20): (unknown callee), target 0x20400, r0=`[[sp-296]+28]`, r1=`unknown`, r2=`unknown`, r3=`[sp-296]`.
  Call (`bl` at 0x14eb2c): (unknown callee), target 0x14f418, r0=`sp-296`, r1=`unknown`, r2=`unknown`, r3=`sp-296`.
  Call (`bl` at 0x14eb38): (unknown callee), target 0x205bc, r0=`[sp-28]`, r1=`0x195c70`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x14eb60): (unknown callee), target 0x14ef18, r0=`[[sp-296]+28]`, r1=`sp-29`, r2=`sp-288`, r3=`[sp-296]`.
  Call (`bl` at 0x14eb78): (unknown callee), target 0x20400, r0=`[[sp-296]+28]`, r1=`unknown`, r2=`unknown`, r3=`[sp-296]`.
  Call (`bl` at 0x14eb84): (unknown callee), target 0x14f418, r0=`sp-296`, r1=`unknown`, r2=`unknown`, r3=`sp-296`.
  Call (`bl` at 0x14eb90): (unknown callee), target 0x205bc, r0=`[sp-28]`, r1=`0x195c9c`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x14ebac): (unknown callee), target 0x20058, r0=`sp-288`, r1=`unknown`, r2=`unknown`, r3=`sp-288`.
  Call (`bl` at 0x14ebcc): (unknown callee), target 0x20400, r0=`[[sp-296]+28]`, r1=`unknown`, r2=`unknown`, r3=`[sp-296]`.
  Call (`bl` at 0x14ebd8): (unknown callee), target 0x14f418, r0=`sp-296`, r1=`unknown`, r2=`unknown`, r3=`sp-296`.
  Call (`bl` at 0x14ebe4): (unknown callee), target 0x205bc, r0=`[sp-28]`, r1=`0x195ccc`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x14ec0c): (unknown callee), target 0x14ef18, r0=`[[sp-296]+28]`, r1=`sp-29`, r2=`sp-288`, r3=`[sp-296]`.
  Call (`bl` at 0x14ec24): (unknown callee), target 0x20400, r0=`[[sp-296]+28]`, r1=`unknown`, r2=`unknown`, r3=`[sp-296]`.
  Call (`bl` at 0x14ec30): (unknown callee), target 0x14f418, r0=`sp-296`, r1=`unknown`, r2=`unknown`, r3=`sp-296`.
  Call (`bl` at 0x14ec3c): (unknown callee), target 0x205bc, r0=`[sp-28]`, r1=`0x195c9c`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x14ec58): (unknown callee), target 0x20058, r0=`sp-288`, r1=`unknown`, r2=`unknown`, r3=`sp-288`.
  Call (`bl` at 0x14ec78): (unknown callee), target 0x20400, r0=`[[sp-296]+28]`, r1=`unknown`, r2=`unknown`, r3=`[sp-296]`.
  Call (`bl` at 0x14ec84): (unknown callee), target 0x14f418, r0=`sp-296`, r1=`unknown`, r2=`unknown`, r3=`sp-296`.
  Call (`bl` at 0x14ec90): (unknown callee), target 0x205bc, r0=`[sp-28]`, r1=`0x195ccc`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x14ecb8): (unknown callee), target 0x14ef18, r0=`[[sp-296]+28]`, r1=`sp-29`, r2=`sp-288`, r3=`[sp-296]`.
  Call (`bl` at 0x14ecd0): (unknown callee), target 0x20400, r0=`[[sp-296]+28]`, r1=`unknown`, r2=`unknown`, r3=`[sp-296]`.
  Call (`bl` at 0x14ecdc): (unknown callee), target 0x14f418, r0=`sp-296`, r1=`unknown`, r2=`unknown`, r3=`sp-296`.
  Call (`bl` at 0x14ece8): (unknown callee), target 0x205bc, r0=`[sp-28]`, r1=`0x195c9c`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x14ed04): (unknown callee), target 0x20058, r0=`sp-288`, r1=`unknown`, r2=`unknown`, r3=`sp-288`.
  Call (`bl` at 0x14ed24): (unknown callee), target 0x20400, r0=`[[sp-296]+28]`, r1=`unknown`, r2=`unknown`, r3=`[sp-296]`.
  Call (`bl` at 0x14ed30): (unknown callee), target 0x14f418, r0=`sp-296`, r1=`unknown`, r2=`unknown`, r3=`sp-296`.
  Call (`bl` at 0x14ed3c): (unknown callee), target 0x205bc, r0=`[sp-28]`, r1=`0x195ccc`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x14ed64): (unknown callee), target 0x14ef18, r0=`[[sp-296]+28]`, r1=`sp-29`, r2=`sp-288`, r3=`[sp-296]`.
  Call (`bl` at 0x14ed7c): (unknown callee), target 0x20400, r0=`[[sp-296]+28]`, r1=`unknown`, r2=`unknown`, r3=`[sp-296]`.
  Call (`bl` at 0x14ed88): (unknown callee), target 0x14f418, r0=`sp-296`, r1=`unknown`, r2=`unknown`, r3=`sp-296`.
  Call (`bl` at 0x14ed94): (unknown callee), target 0x205bc, r0=`[sp-28]`, r1=`0x195c9c`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x14edb0): (unknown callee), target 0x20058, r0=`sp-288`, r1=`unknown`, r2=`unknown`, r3=`sp-288`.
  Call (`bl` at 0x14edd0): (unknown callee), target 0x20400, r0=`[[sp-296]+28]`, r1=`unknown`, r2=`unknown`, r3=`[sp-296]`.
  Call (`bl` at 0x14eddc): (unknown callee), target 0x14f418, r0=`sp-296`, r1=`unknown`, r2=`unknown`, r3=`sp-296`.
  Call (`bl` at 0x14ede8): (unknown callee), target 0x205bc, r0=`[sp-28]`, r1=`0x195ccc`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x14ee04): (unknown callee), target 0x14f064, r0=`[[sp-296]+28]`, r1=`unknown`, r2=`unknown`, r3=`[sp-296]`.
  Call (`bl` at 0x14ee20): (unknown callee), target 0x20400, r0=`[[sp-296]+28]`, r1=`unknown`, r2=`unknown`, r3=`[sp-296]`.
  Call (`bl` at 0x14ee2c): (unknown callee), target 0x14f418, r0=`sp-296`, r1=`unknown`, r2=`unknown`, r3=`sp-296`.
  Call (`bl` at 0x14ee38): (unknown callee), target 0x205bc, r0=`[sp-28]`, r1=`0x195c9c`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x14ee58).

## `bitfile_read_hdr_prefix`

- `bitfile_read_hdr_prefix`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x14ee74 to 0x14ef18.
  Established facts:
  Call (`bl` at 0x14eea0): (unknown callee), target 0x203f4, r0=`sp-60`, r1=`0x195cf9`, r2=`0xd`, r3=`0x195cf9`.
  Call (`bl` at 0x14eec0): (unknown callee), target 0x20244, r0=`sp-44`, r1=`0x1`, r2=`0xd`, r3=`[sp-20]`.
  Branch (`beq` at 0x14eed8): taken to 0x14eee8 when `ret(0x20244) == [sp-24]`, falls through to 0x14eedc when `ret(0x20244) != [sp-24]` (source `bl` at 0x14eec0).
  Call (`bl` at 0x14eefc): (unknown callee), target 0x20364, r0=`sp-44`, r1=`sp-60`, r2=`[sp-24]`, r3=`sp-44`.
  Recorded r0 = `[sp-64]` (`ldm` at 0x14ef10).

## `bitfile_read_hdr_section`

- `bitfile_read_hdr_section`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x14ef18 to 0x14f064.
  Established facts:
  Call (`bl` at 0x14ef50): (unknown callee), target 0x20244, r0=`sp-40`, r1=`0x1`, r2=`0x1`, r3=`r0`.
  Byte store: target `(unknown effective address)`, value `0x0` (`strb` at 0x14ef90).
  Call (`bl` at 0x14efc8): (unknown callee), target 0x20244, r0=`sp-40`, r1=`0x1`, r2=`0x2`, r3=`[sp-20]`.
  Call (`bl` at 0x14f014): (unknown callee), target 0x20244, r0=`[sp-28]`, r1=`0x1`, r2=`[sp-32]`, r3=`[sp-20]`.
  Recorded r0 = `[sp-44]` (`ldm` at 0x14f060).

## `bitfile_read_hdr_length`

- `bitfile_read_hdr_length`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x14f064 to 0x14f1a0.
  Established facts:
  Call (`bl` at 0x14f094): (unknown callee), target 0x20244, r0=`sp-32`, r1=`0x1`, r2=`0x1`, r3=`r0`.
  Branch (`beq` at 0x14f0ac): taken to 0x14f0bc when `ret(0x20244) == [sp-24]`, falls through to 0x14f0b0 when `ret(0x20244) != [sp-24]`.
  Call (`bl` at 0x14f0f0): (unknown callee), target 0x20244, r0=`sp-32`, r1=`0x1`, r2=`0x4`, r3=`[sp-20]`.
  Recorded r0 = `[sp-44]` (`ldm` at 0x14f19c).

## `bitfile_read_data`

- `bitfile_read_data`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x14f1a0 to 0x14f378.
  Established facts:
  Call (`bl` at 0x14f200): (unknown callee), target 0x20244, r0=`[sp-28]`, r1=`0x1`, r2=`[sp-32]`, r3=`[[sp-20]+28]`.
  Branch (`beq` at 0x14f218): taken to 0x14f228 when `ret(0x20244) == [sp-32]`, falls through to 0x14f21c when `ret(0x20244) != [sp-32]` (source `bl` at 0x14f200).
  Byte store: target `([sp-32] + [sp-52]) +0x0`, value `0x0` (`strb` at 0x14f27c).
  Word store: target `[sp-20] +0x20`, value `unknown` (`str` at 0x14f360).
  Recorded r0 = `[sp-56]` (`ldm` at 0x14f374).

## `bitfile_create`

- `bitfile_create`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x14f378 to 0x14f418.
  Established facts:
  Call (`bl` at 0x14f390): (unknown callee), target 0x201fc, r0=`0x24`, r1=`r1`, r2=`r2`, r3=`r3`.
  Branch (`bne` at 0x14f3a4): taken to 0x14f3b4 when `ret(0x201fc) != 0x0`, falls through to 0x14f3a8 when `ret(0x201fc) == 0x0` (source `bl` at 0x14f390).
  Call (`bl` at 0x14f3c0): (unknown callee), target 0x20550, r0=`ret(0x201fc)`, r1=`0x0`, r2=`0x24`, r3=`ret(0x201fc)`.
  Call (`bl` at 0x14f3d8): (unknown callee), target 0x20058, r0=`[sp-24]`, r1=`unknown`, r2=`unknown`, r3=`[sp-24]`.
  Word store: target `[sp-28] +0x0`, value `ret(0x20058)` (`str` at 0x14f3e0, `bl` at 0x14f3d8).
  Call (`bl` at 0x14f3f4): (unknown callee), target 0x20514, r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`[[sp-28]]`.
  Recorded r0 = `unknown` (`ldm` at 0x14f414).

## `bitfile_destroy`

- `bitfile_destroy`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x14f418 to 0x14f4fc.
  Established facts:
  Branch (`beq` at 0x14f43c): taken to 0x14f450 when `[[r0]] == 0x0`, falls through to 0x14f440 when `[[r0]] != 0x0`.
  Call (`bl` at 0x14f44c): (unknown callee), target 0x20514, r0=`[[r0]]`, r1=`r1`, r2=`r2`, r3=`[r0]`.
  Call (`bl` at 0x14f470): (unknown callee), target 0x20514, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x14f494): (unknown callee), target 0x20514, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x14f4b8): (unknown callee), target 0x20514, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x14f4dc): (unknown callee), target 0x20514, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x14f4e8): (unknown callee), target 0x20514, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Word store: target `[sp-20] +0x0`, value `0x0` (`str` at 0x14f4f4).
  Recorded r0 = `ret(0x20514)` (`ldm` at 0x14f4f8).

## `bitfile_display`

- `bitfile_display`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x14f4fc to 0x14f640.
  Established facts:
  Branch (`bne` at 0x14f524): taken to 0x14f534 when `r0 != 0x0`, falls through to 0x14f528 when `r0 == 0x0`.
  Word load: target `abs 0x1a2888` (`ldr` at 0x14f52c).
  Call (`bl` at 0x14f54c): (unknown callee), target 0x244d4, r0=`unknown`, r1=`0x195d08`, r2=`[r1]`, r3=`r1`.
  Call (`bl` at 0x14f560): (unknown callee), target 0x244d4, r0=`[sp-20]`, r1=`0x195d1c`, r2=`[[sp-24]+4]`, r3=`[sp-24]`.
  Call (`bl` at 0x14f574): (unknown callee), target 0x244d4, r0=`[sp-20]`, r1=`0x195d30`, r2=`[[sp-24]+8]`, r3=`[sp-24]`.
  Call (`bl` at 0x14f588): (unknown callee), target 0x244d4, r0=`[sp-20]`, r1=`0x195d44`, r2=`[[sp-24]+12]`, r3=`[sp-24]`.
  Call (`bl` at 0x14f59c): (unknown callee), target 0x244d4, r0=`[sp-20]`, r1=`0x195d58`, r2=`[[sp-24]+16]`, r3=`[sp-24]`.
  Call (`bl` at 0x14f5b0): (unknown callee), target 0x244d4, r0=`[sp-20]`, r1=`0x195d6c`, r2=`[[sp-24]+20]`, r3=`[sp-24]`.
  Call (`bl` at 0x14f5c4): (unknown callee), target 0x244d4, r0=`[sp-20]`, r1=`0x195d80`, r2=`[[sp-24]+28]`, r3=`[sp-24]`.
  Call (`bl` at 0x14f5e0): (unknown callee), target 0x244d4, r0=`[sp-20]`, r1=`0x195d98`, r2=`[[sp-24]+24]`, r3=`[[sp-24]+24]`.
  Call (`bl` at 0x14f5fc): (unknown callee), target 0x244d4, r0=`[sp-20]`, r1=`0x195db4`, r2=`[[sp-24]+32]`, r3=`[[sp-24]+32]`.
  Call (`bl` at 0x14f608): (unknown callee), target 0x244d4, r0=`[sp-20]`, r1=`0x195dd0`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `ret(0x244d4)` (`ldm` at 0x14f610).

## `_FRUDumpTXT`

- `_FRUDumpTXT`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x106010 to 0x106f04.
  Established facts:
  Word load: target `r3 +0xc` (`ldr` at 0x106054).
  Word load: target `r3 +0x10` (`ldr` at 0x106058).
  Call (`bl` at 0x10605c): (unknown callee), target 0x205bc, r0=`sp-132`, r1=`0x18bfd4`, r2=`[r3+12]`, r3=`[r3+16]`.
  Call (`bl` at 0x106070): (unknown callee), target 0x105fb0, r0=`[sp-32]`, r1=`[sp-36]`, r2=`sp-132`, r3=`sp-132`.
  Call (`bl` at 0x106088): (unknown callee), target 0x205bc, r0=`sp-132`, r1=`0x18bff4`, r2=`[[sp-40]]`, r3=`sp-132`.
  Call (`bl` at 0x10609c): (unknown callee), target 0x105fb0, r0=`[sp-32]`, r1=`[sp-36]`, r2=`sp-132`, r3=`sp-132`.
  Call (`bl` at 0x1060a8): (unknown callee), target 0x1079c4, r0=`[[sp-40]+20]`, r1=`unknown`, r2=`unknown`, r3=`[sp-40]`.
  Call (`bl` at 0x1060d0): (unknown callee), target 0x205bc, r0=`sp-132`, r1=`0x18c008`, r2=`[[sp-40]+12]`, r3=`[[sp-40]+16]`.
  Call (`bl` at 0x1060e4): (unknown callee), target 0x105fb0, r0=`[sp-32]`, r1=`[sp-36]`, r2=`sp-132`, r3=`sp-132`.
  Branch (`beq` at 0x1060f4): taken to 0x106130 when `[[sp-40]+4] == 0x0`, falls through to 0x1060f8 when `[[sp-40]+4] != 0x0`.
  Call (`bl` at 0x106104): (unknown callee), target 0x20424, r0=`[sp-40]+4`, r1=`unknown`, r2=`unknown`, r3=`[sp-40]+4`.
  Call (`bl` at 0x106118): (unknown callee), target 0x205bc, r0=`sp-132`, r1=`0x18c020`, r2=`ret(0x20424)`, r3=`sp-132`.
  Call (`bl` at 0x10612c): (unknown callee), target 0x105fb0, r0=`[sp-32]`, r1=`[sp-36]`, r2=`sp-132`, r3=`sp-132`.
  Call (`bl` at 0x106138): (unknown callee), target 0x1073c8, r0=`[[sp-40]+24]`, r1=`unknown`, r2=`unknown`, r3=`[sp-40]`.
  Call (`bl` at 0x10614c): (unknown callee), target 0x205bc, r0=`sp-132`, r1=`0x18c030`, r2=`ret(0x1073c8)`, r3=`sp-132`.
  Call (`bl` at 0x106160): (unknown callee), target 0x105fb0, r0=`[sp-32]`, r1=`[sp-36]`, r2=`sp-132`, r3=`sp-132`.
  Call (`bl` at 0x106180): (unknown callee), target 0x205bc, r0=`sp-132`, r1=`0x18c040`, r2=`unknown`, r3=`sp-132`.
  Call (`bl` at 0x106194): (unknown callee), target 0x105fb0, r0=`[sp-32]`, r1=`[sp-36]`, r2=`sp-132`, r3=`sp-132`.
  Call (`bl` at 0x106288): (unknown callee), target 0x1073c8, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x106298): (unknown callee), target 0x1079c4, r0=`[[sp-40]+20]`, r1=`unknown`, r2=`unknown`, r3=`[sp-40]`.
  Call (`bl` at 0x10635c): (unknown callee), target 0x205bc, r0=`sp-132`, r1=`0x18c058`, r2=`[[sp-40]+12]`, r3=`[[sp-40]+16]`.
  Call (`bl` at 0x106370): (unknown callee), target 0x105fb0, r0=`[sp-32]`, r1=`[sp-36]`, r2=`sp-132`, r3=`sp-132`.
  Call (`bl` at 0x1063a0): (unknown callee), target 0x205bc, r0=`sp-132`, r1=`0x18c084`, r2=`[[sp-40]+28]`, r3=`sp-132`.
  Call (`bl` at 0x1063b4): (unknown callee), target 0x105fb0, r0=`[sp-32]`, r1=`[sp-36]`, r2=`sp-132`, r3=`sp-132`.
  Call (`bl` at 0x1063c4): (unknown callee), target 0x205bc, r0=`sp-132`, r1=`0x18c09c`, r2=`unknown`, r3=`sp-132`.
  Call (`bl` at 0x1063d8): (unknown callee), target 0x105fb0, r0=`[sp-32]`, r1=`[sp-36]`, r2=`sp-132`, r3=`sp-132`.
  Call (`bl` at 0x10674c): (unknown callee), target 0x205bc, r0=`sp-132`, r1=`0x18c0cc`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x106760): (unknown callee), target 0x105fb0, r0=`[sp-32]`, r1=`[sp-36]`, r2=`sp-132`, r3=`sp-132`.
  Call (`bl` at 0x106820): (unknown callee), target 0x205bc, r0=`sp-132`, r1=`0x18c0f4`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x106830): (unknown callee), target 0x2031c, r0=`sp-132`, r1=`unknown`, r2=`unknown`, r3=`sp-132`.
  Call (`bl` at 0x106b20): (unknown callee), target 0x205bc, r0=`(sp-132 + ret(0x2031c))`, r1=`0x18c104`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x106b34): (unknown callee), target 0x105fb0, r0=`[sp-32]`, r1=`[sp-36]`, r2=`sp-132`, r3=`sp-132`.
  Call (`bl` at 0x106b58): (unknown callee), target 0x205bc, r0=`sp-132`, r1=`0x18c128`, r2=`[sp-136]`, r3=`sp-132`.
  Call (`bl` at 0x106b6c): (unknown callee), target 0x105fb0, r0=`[sp-32]`, r1=`[sp-36]`, r2=`sp-132`, r3=`sp-132`.
  Call (`bl` at 0x106bcc): (unknown callee), target 0x205bc, r0=`sp-132`, r1=`0x18c138`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x106be0): (unknown callee), target 0x105fb0, r0=`[sp-32]`, r1=`[sp-36]`, r2=`sp-132`, r3=`sp-132`.
  Call (`bl` at 0x106bf4): (unknown callee), target 0x105fb0, r0=`[sp-32]`, r1=`[sp-36]`, r2=`[[sp-40]+36]`, r3=`[sp-40]`.
  Call (`bl` at 0x106c04): (unknown callee), target 0x205bc, r0=`sp-132`, r1=`0x18c15c`, r2=`unknown`, r3=`sp-132`.
  Call (`bl` at 0x106c18): (unknown callee), target 0x105fb0, r0=`[sp-32]`, r1=`[sp-36]`, r2=`sp-132`, r3=`sp-132`.
  Call (`bl` at 0x106c48): (unknown callee), target 0x205bc, r0=`sp-132`, r1=`0x18c170`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x106c5c): (unknown callee), target 0x105fb0, r0=`[sp-32]`, r1=`[sp-36]`, r2=`sp-132`, r3=`sp-132`.
  Call (`bl` at 0x106c70): (unknown callee), target 0x105fb0, r0=`[sp-32]`, r1=`[sp-36]`, r2=`[[sp-40]+40]`, r3=`[sp-40]`.
  Call (`bl` at 0x106c80): (unknown callee), target 0x205bc, r0=`sp-132`, r1=`0x18c190`, r2=`unknown`, r3=`sp-132`.
  Call (`bl` at 0x106c94): (unknown callee), target 0x105fb0, r0=`[sp-32]`, r1=`[sp-36]`, r2=`sp-132`, r3=`sp-132`.
  Call (`bl` at 0x106cc4): (unknown callee), target 0x205bc, r0=`sp-132`, r1=`0x18c1a4`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x106cd8): (unknown callee), target 0x105fb0, r0=`[sp-32]`, r1=`[sp-36]`, r2=`sp-132`, r3=`sp-132`.
  Call (`bl` at 0x106cec): (unknown callee), target 0x105fb0, r0=`[sp-32]`, r1=`[sp-36]`, r2=`[[sp-40]+44]`, r3=`[sp-40]`.
  Call (`bl` at 0x106cfc): (unknown callee), target 0x205bc, r0=`sp-132`, r1=`0x18c1c4`, r2=`unknown`, r3=`sp-132`.
  Call (`bl` at 0x106d10): (unknown callee), target 0x105fb0, r0=`[sp-32]`, r1=`[sp-36]`, r2=`sp-132`, r3=`sp-132`.
  Call (`bl` at 0x106d40): (unknown callee), target 0x205bc, r0=`sp-132`, r1=`0x18c1d4`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x106d54): (unknown callee), target 0x105fb0, r0=`[sp-32]`, r1=`[sp-36]`, r2=`sp-132`, r3=`sp-132`.
  Call (`bl` at 0x106d68): (unknown callee), target 0x105fb0, r0=`[sp-32]`, r1=`[sp-36]`, r2=`[[sp-40]+48]`, r3=`[sp-40]`.
  Call (`bl` at 0x106d78): (unknown callee), target 0x205bc, r0=`sp-132`, r1=`0x18c1f4`, r2=`unknown`, r3=`sp-132`.
  Call (`bl` at 0x106d8c): (unknown callee), target 0x105fb0, r0=`[sp-32]`, r1=`[sp-36]`, r2=`sp-132`, r3=`sp-132`.
  Call (`bl` at 0x106dbc): (unknown callee), target 0x205bc, r0=`sp-132`, r1=`0x18c208`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x106dd0): (unknown callee), target 0x105fb0, r0=`[sp-32]`, r1=`[sp-36]`, r2=`sp-132`, r3=`sp-132`.
  Call (`bl` at 0x106de4): (unknown callee), target 0x105fb0, r0=`[sp-32]`, r1=`[sp-36]`, r2=`[[sp-40]+52]`, r3=`[sp-40]`.
  Call (`bl` at 0x106df4): (unknown callee), target 0x205bc, r0=`sp-132`, r1=`0x18c228`, r2=`unknown`, r3=`sp-132`.
  Call (`bl` at 0x106e08): (unknown callee), target 0x105fb0, r0=`[sp-32]`, r1=`[sp-36]`, r2=`sp-132`, r3=`sp-132`.
  Call (`bl` at 0x106e38): (unknown callee), target 0x205bc, r0=`sp-132`, r1=`0x18c23c`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x106e4c): (unknown callee), target 0x105fb0, r0=`[sp-32]`, r1=`[sp-36]`, r2=`sp-132`, r3=`sp-132`.
  Call (`bl` at 0x106e60): (unknown callee), target 0x105fb0, r0=`[sp-32]`, r1=`[sp-36]`, r2=`[[sp-40]+56]`, r3=`[sp-40]`.
  Call (`bl` at 0x106e70): (unknown callee), target 0x205bc, r0=`sp-132`, r1=`0x18c25c`, r2=`unknown`, r3=`sp-132`.
  Call (`bl` at 0x106e84): (unknown callee), target 0x105fb0, r0=`[sp-32]`, r1=`[sp-36]`, r2=`sp-132`, r3=`sp-132`.
  Recorded r0 = `unknown` (`ldm` at 0x106e8c).

## `FRUFree`

- `FRUFree`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1070e8 to 0x1073c8.
  Established facts:
  Word load: target `(unknown effective address)` (`ldr` at 0x107108).
  Branch (`ble` at 0x107114): taken to 0x1072fc when `le flags condition (operands unknown)`, falls through to 0x107118 when `not (le flags condition (operands unknown))`.
  Call (`bl` at 0x1071e8): (unknown callee), target 0x20514, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x107284): (unknown callee), target 0x20514, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x1072e8): (unknown callee), target 0x20514, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x107314): (unknown callee), target 0x20514, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x107330): (unknown callee), target 0x20514, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x10734c): (unknown callee), target 0x20514, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x107368): (unknown callee), target 0x20514, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x107384): (unknown callee), target 0x20514, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x1073a0): (unknown callee), target 0x20514, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x1073bc): (unknown callee), target 0x20514, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x1073c4).

## `FRUStatus`

- `FRUStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1073c8 to 0x1074b4.
  Established facts:
  Branch (`ldrls` at 0x1073e4): taken to (unknown target) when `r0 <= 0x8 (unsigned, 32-bit)`, falls through to 0x1073e8 when `r0 > 0x8 (unsigned, 32-bit)`.
  Omitted-path effects remain unknown at the unresolved transfer (`ldrls` at 0x1073e4).
  Known-edge recorded r0 = `0x18c350` (`ldm` at 0x10748c).

## `FRUSource`

- `FRUSource`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1074b4 to 0x107550.
  Established facts:
  Branch (`beq` at 0x1074d8): taken to 0x107514 when `r0 == 0x2`, falls through to 0x1074dc when `r0 != 0x2`.
  Branch (`bgt` at 0x1074e4): taken to 0x1074f8 when `r0 > 0x2 (signed, 32-bit)`, falls through to 0x1074e8 when `r0 <= 0x2 (signed, 32-bit)`.
  Branch (`beq` at 0x1074f0): taken to 0x107508 when `r0 == 0x1`, falls through to 0x1074f4 when `r0 != 0x1`.
  Branch (`beq` at 0x107500): taken to 0x107520 when `r0 == 0x3`, falls through to 0x107504 when `r0 != 0x3`.
  Recorded r0 = `unknown` (`ldm` at 0x10753c).
