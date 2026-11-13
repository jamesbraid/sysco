# ARM Linux 2.6 link-error and LED access routines in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-link-errors-led-access-facts.txt) retain all tool sections.

The notation `ret(ADDR)` identifies a call-result expression without distinguishing
individual call sites to that target. Each return line records machine-register
state from the analysis rather than a C return type.


Groups summarize static access records by printed shape. Instruction-address order can include different control-flow paths.

## `LinkErrReadStatus`

- `LinkErrReadStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3c980 to 0x3cbac.
  Established facts:
  Decoded words in [0x3cba0, 0x3cbac) are not reached from the entry by known edges.
  Byte store: target `r1 +0x0`, value `0x1` (`strb` at 0x3c9b4).
  Call (`bl` at 0x3c9d0): (unknown callee), target 0x134634, r0=`r0`, r1=`r1`, r2=`[sp-24]`, r3=`0x0`.
  Call (`bl` at 0x3c9ec): (unknown callee), target 0x13d1a0, r0=`[sp-36]`, r1=`0x1000`, r2=`sp-48`, r3=`sp-52`.
  Branch (`beq` at 0x3ca00): taken to 0x3ca14 when `ret(0x13d1a0) == 0x0`, falls through to 0x3ca04 when `ret(0x13d1a0) != 0x0` (source `bl` at 0x3c9ec).
  Call (`bl` at 0x3ca04): (unknown callee), target 0x1346fc, r0=`ret(0x13d1a0)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13d1a0)`.
  Call (`bl` at 0x3ca14): (unknown callee), target 0x1346fc, r0=`ret(0x13d1a0)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13d1a0)`.
  Call (`bl` at 0x3ca50): (unknown callee), target 0x134634, r0=`ret(0x1346fc)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3ca6c): (unknown callee), target 0x13d1a0, r0=`[sp-36]`, r1=`[sp-40]`, r2=`sp-48`, r3=`sp-52`.
  Call (`bl` at 0x3ca84): (unknown callee), target 0x1346fc, r0=`ret(0x13d1a0)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13d1a0)`.
  Call (`bl` at 0x3caac): (unknown callee), target 0x13d1a0, r0=`[sp-36]`, r1=`[sp-44]`, r2=`sp-56`, r3=`sp-60`.
  Call (`bl` at 0x3cac4): (unknown callee), target 0x1346fc, r0=`ret(0x13d1a0)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13d1a0)`.
  Call (`bl` at 0x3cad4): (unknown callee), target 0x1346fc, r0=`ret(0x13d1a0)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13d1a0)`.
  Recorded r0 = `unknown` (`ldm` at 0x3cb9c).

## `LinkErrReadBER`

- `LinkErrReadBER`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3cbac to 0x3d034.
  Established facts:
  Decoded words in [0x3cff4, 0x3d028), [0x3d02c, 0x3d034) are not reached from the entry by known edges.
  Byte store: target `r1 +0x0`, value `0x1` (`strb` at 0x3cbd8).
  Call (`bl` at 0x3cbf4): (unknown callee), target 0x134634, r0=`r0`, r1=`r1`, r2=`r1`, r3=`unknown`.
  Call (`bl` at 0x3cc10): (unknown callee), target 0x13d1a0, r0=`[sp-36]`, r1=`0x1000`, r2=`sp-76`, r3=`sp-80`.
  Branch (`beq` at 0x3cc24): taken to 0x3cc38 when `ret(0x13d1a0) == 0x0`, falls through to 0x3cc28 when `ret(0x13d1a0) != 0x0` (source `bl` at 0x3cc10).
  Call (`bl` at 0x3cc28): (unknown callee), target 0x1346fc, r0=`ret(0x13d1a0)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13d1a0)`.
  Call (`bl` at 0x3cc38): (unknown callee), target 0x1346fc, r0=`ret(0x13d1a0)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13d1a0)`.
  Call (`bl` at 0x3cc88): (unknown callee), target 0x134634, r0=`ret(0x1346fc)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3cca4): (unknown callee), target 0x13d1a0, r0=`[sp-36]`, r1=`[sp-40]`, r2=`sp-84`, r3=`sp-88`.
  Call (`bl` at 0x3ccbc): (unknown callee), target 0x1346fc, r0=`ret(0x13d1a0)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13d1a0)`.
  Call (`bl` at 0x3cce4): (unknown callee), target 0x13d1a0, r0=`[sp-36]`, r1=`[sp-44]`, r2=`sp-92`, r3=`sp-96`.
  Call (`bl` at 0x3ccfc): (unknown callee), target 0x1346fc, r0=`ret(0x13d1a0)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13d1a0)`.
  Call (`bl` at 0x3cd24): (unknown callee), target 0x13d1a0, r0=`[sp-36]`, r1=`[sp-48]`, r2=`sp-100`, r3=`sp-104`.
  Call (`bl` at 0x3cd3c): (unknown callee), target 0x1346fc, r0=`ret(0x13d1a0)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13d1a0)`.
  Call (`bl` at 0x3cd5c): (unknown callee), target 0x13d2b8, r0=`[sp-36]`, r1=`[sp-48]`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0x3cd60): (unknown callee), target 0x1346fc, r0=`ret(0x13d2b8)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3cef4): (unknown callee), target 0x29908, r0=`0x15a4f4`, r1=`unknown`, r2=`[sp-28]`, r3=`unknown`.
  Call (`bl` at 0x3cfc4): (unknown callee), target 0x29908, r0=`0x15a50c`, r1=`[sp-124]`, r2=`[sp-120]`, r3=`[sp-60]`.
  Recorded r0 = `unknown` (`ldm` at 0x3cff0).

## `LinkErrMonitor`

- `LinkErrMonitor`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3d034 to 0x3d360.
  Established facts:
  Decoded words in [0x3d324, 0x3d360) are not reached from the entry by known edges.
  Call (`bl` at 0x3d054): (unknown callee), target 0x3dacc, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3d064): (unknown callee), target 0x5572c, r0=`ret(0x3dacc)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3dacc)`.
  Call (`bl` at 0x3d084): (unknown callee), target 0x293f8, r0=`0x15a538`, r1=`unknown`, r2=`unknown`, r3=`[sp-36]`.
  Call (`bl` at 0x3d0fc): (unknown callee), target 0x3d360, r0=`0x1770`, r1=`unknown`, r2=`0x1a5a44`, r3=`0x0`.
  Call (`bl` at 0x3d128): (unknown callee), target 0x3c980, r0=`unknown`, r1=`sp-68`, r2=`sp-68`, r3=`unknown`.
  Call (`bl` at 0x3d1ac): (unknown callee), target 0x205bc, r0=`sp-268`, r1=`0x15a55c`, r2=`[sp-28]+65`, r3=`[sp-280]`.
  Call (`bl` at 0x3d1dc): (unknown callee), target 0xa9740, r0=`0x1`, r1=`0x8100`, r2=`0x0`, r3=`sp-268`.
  Call (`bl` at 0x3d20c): (unknown callee), target 0xa9740, r0=`0x1`, r1=`0x8101`, r2=`0x0`, r3=`sp-268`.
  Call (`bl` at 0x3d218): (unknown callee), target 0x293f8, r0=`sp-268`, r1=`unknown`, r2=`unknown`, r3=`sp-268`.
  Call (`bl` at 0x3d280): (unknown callee), target 0x3cbac, r0=`unknown`, r1=`sp-64`, r2=`sp-64`, r3=`unknown`.
  Call (`bl` at 0x3d2b4): (unknown callee), target 0x205bc, r0=`sp-268`, r1=`0x15a590`, r2=`[sp-28]+65`, r3=`[sp-48]`.
  Call (`bl` at 0x3d2dc): (unknown callee), target 0xa9740, r0=`0x1`, r1=`0x8102`, r2=`0x0`, r3=`sp-268`.
  Call (`bl` at 0x3d2e8): (unknown callee), target 0x293f8, r0=`sp-268`, r1=`unknown`, r2=`unknown`, r3=`sp-268`.
  Call (`bl` at 0x3d2fc): (unknown callee), target 0x5572c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x3d31c): (unknown callee), target 0x3d360, r0=`[0x19e6d8]`, r1=`unknown`, r2=`unknown`, r3=`0x19e6d8`.
  Jump (`b` at 0x3d320): target 0x3d054.

## `LinkErrTaskInit`

- `LinkErrTaskInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3d9dc to 0x3da58.
  Established facts:
  Decoded words in [0x3da44, 0x3da58) are not reached from the entry by known edges.
  Call (`bl` at 0x3d9fc): (unknown callee), target 0x22b84, r0=`0x1a5a48`, r1=`0x15a828`, r2=`0x0`, r3=`0x9`.
  Call (`bl` at 0x3da38): (unknown callee), target 0x22540, r0=`0x1a5a9c`, r1=`0x15a834`, r2=`0x3d034`, r3=`0x1`.
  Recorded r0 = `ret(0x22540)` (`ldm` at 0x3da40, `bl` at 0x3da38).

## `LinkErrChangeFreq`

- `LinkErrChangeFreq`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3da58 to 0x3dacc.
  Established facts:
  Decoded words in [0x3dabc, 0x3dacc) are not reached from the entry by known edges.
  Branch (`ble` at 0x3da88): taken to 0x3da9c when `[0x19e6d8] <= 0x176f (signed, 32-bit)`, falls through to 0x3da8c when `[0x19e6d8] > 0x176f (signed, 32-bit)`.
  Call (`bl` at 0x3daac): (unknown callee), target 0x14fe64, r0=`0x1770`, r1=`[0x19e6d8]`, r2=`[0x19e6d8]`, r3=`0x19e6d8`.
  Recorded r0 = `unknown` (`ldm` at 0x3dab8).

## `LinkErrActive`

- `LinkErrActive`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3dacc to 0x3db34.
  Established facts:
  Decoded words in [0x3db30, 0x3db34) are not reached from the entry by known edges.
  Call (`bl` at 0x3dae0): (unknown callee), target 0x28230, r0=`0x9b`, r1=`r1`, r2=`r2`, r3=`r3`.
  Word load: target `abs 0x19e6d4` (`ldr` at 0x3daf0).
  Branch (`beq` at 0x3db00): taken to 0x3db1c when `(ret(0x28230) & 0xff) == 0x0`, falls through to 0x3db04 when `(ret(0x28230) & 0xff) != 0x0` (source `bl` at 0x3dae0).
  Recorded r0 = `unknown` (`ldm` at 0x3db2c).

## `frontpanelLEDBlinkTask`

- `frontpanelLEDBlinkTask`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x71dd4 to 0x71f28.
  Established facts:
  Decoded words in [0x71f08, 0x71f28) are not reached from the entry by known edges.
  Branch (`bne` at 0x71dec): taken to 0x71df4 when `[0x1a61e8] != 0x0`, falls through to 0x71df0 when `[0x1a61e8] == 0x0`.
  Call (`bl` at 0x71ec8): (unknown callee), target 0x72038, r0=`[0x1a61fd]:1`, r1=`[0x1a61fe]:1`, r2=`0x1a61fd`, r3=`[0x1a61fd]:1`.
  Call (`bl` at 0x71edc): (unknown callee), target 0x71f28, r0=`[0x1a61fd]:1`, r1=`[0x1a61fe]:1`, r2=`0x1a61fd`, r3=`[0x1a61fd]:1`.
  Recorded r0 = `unknown` (`ldm` at 0x71f04).

## `fp_led_on_raw`

- `fp_led_on_raw`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x71f28 to 0x72038.
  Established facts:
  Decoded words in [0x7202c, 0x72038) are not reached from the entry by known edges.
  Call (`bl` at 0x71fd0): (unknown callee), target 0x125a58, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Branch (`beq` at 0x71fe4): taken to 0x71ff0 when `ret(0x125a58) == 0x0`, falls through to 0x71fe8 when `ret(0x125a58) != 0x0` (source `bl` at 0x71fd0).
  Recorded r0 = `[sp-24]` (`ldm` at 0x72028).

## `fp_led_off_raw`

- `fp_led_off_raw`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x72038 to 0x72138.
  Established facts:
  Decoded words in [0x7212c, 0x72138) are not reached from the entry by known edges.
  Call (`bl` at 0x720cc): (unknown callee), target 0x125a58, r0=`0x0`, r1=`unknown`, r2=`unknown`, r3=`0x0`.
  Branch (`beq` at 0x720e0): taken to 0x720ec when `ret(0x125a58) == 0x0`, falls through to 0x720e4 when `ret(0x125a58) != 0x0` (source `bl` at 0x720cc).
  Byte store: target `abs 0x1a61fe`, value `unknown` (`strb` at 0x72108).
  Recorded r0 = `[sp-24]` (`ldm` at 0x72128).

## `LBIReadLEDS`

- `LBIReadLEDS`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8a520 to 0x8a94c.
  Established facts:
  Decoded words in [0x8a924, 0x8a94c) are not reached from the entry by known edges.
  Word load: target `abs 0x1a63cc` (`ldr` at 0x8a560).
  Branch (`bne` at 0x8a568): taken to 0x8a624 when `[0x1a63cc] != 0x1`, falls through to 0x8a56c when `[0x1a63cc] == 0x1`.
  Call (`bl` at 0x8a624): (unknown callee), target 0x134634, r0=`r0`, r1=`r1`, r2=`r2`, r3=`[0x1a63cc]`.
  Call (`bl` at 0x8a650): (unknown callee), target 0x13d9cc, r0=`[sp-20]`, r1=`[0x1a6410]`, r2=`0x0`, r3=`sp-32`.
  Call (`bl` at 0x8a660): (unknown callee), target 0x1346fc, r0=`ret(0x13d9cc)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13d9cc)`.
  Call (`bl` at 0x8a668): (unknown callee), target 0x87564, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x8a68c): (unknown callee), target 0x22070, r0=`0x3`, r1=`0x1673e8`, r2=`0x474`, r3=`0x0`.
  Call (`bl` at 0x8a69c): (unknown callee), target 0x1346fc, r0=`ret(0x13d9cc)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x13d9cc)`.
  Call (`bl` at 0x8a6c0): (unknown callee), target 0x132444, r0=`0x0`, r1=`sp-32`, r2=`0x0`, r3=`sp-40`.
  Call (`bl` at 0x8a7ac): (unknown callee), target 0x132444, r0=`0x20`, r1=`sp-32`, r2=`0x0`, r3=`sp-36`.
  Recorded r0 = `unknown` (`ldm` at 0x8a920).

## `LBIReadMemErr`

- `LBIReadMemErr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8b34c to 0x8b390.
  Established facts:
  Call (`bl` at 0x8b364): (unknown callee), target 0x134634, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0x8b370): (unknown callee), target 0x8a94c, r0=`[sp-20]`, r1=`[sp-24]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x8b37c): (unknown callee), target 0x1346fc, r0=`ret(0x8a94c)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x8a94c)`.
  Recorded r0 = `[sp-28]` (`ldm` at 0x8b38c).

## `LBIReadJunkbusStatus`

- `LBIReadJunkbusStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8bbac to 0x8bbf0.
  Established facts:
  Call (`bl` at 0x8bbc4): (unknown callee), target 0x134634, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0x8bbd0): (unknown callee), target 0x8b878, r0=`[sp-20]`, r1=`[sp-24]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x8bbdc): (unknown callee), target 0x1346fc, r0=`ret(0x8b878)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x8b878)`.
  Recorded r0 = `[sp-28]` (`ldm` at 0x8bbec).

## `promIceEnabled`

- `promIceEnabled`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x61388 to 0x614a0.
  Established facts:
  Decoded words in [0x61498, 0x614a0) are not reached from the entry by known edges.
  Call (`bl` at 0x613b0): (unknown callee), target 0x617b0, r0=`(r0 & 0xff)`, r1=`sp-24`, r2=`sp-24`, r3=`(r0 & 0xff)`.
  Call (`bl` at 0x613d0): (unknown callee), target 0x61324, r0=`[sp-17]:1`, r1=`unknown`, r2=`unknown`, r3=`[sp-17]:1`.
  Call (`bl` at 0x613fc): (unknown callee), target 0x125b08, r0=`0x0`, r1=`[[sp-24]]:1`, r2=`[[sp-24]+1]:1`, r3=`[[sp-24]+1]:1`.
  Branch (`beq` at 0x61418): taken to 0x6142c when `ret(0x125b08) == 0x0`, falls through to 0x6141c when `ret(0x125b08) != 0x0` (source `bl` at 0x613fc).
  Branch (`beq` at 0x61424): taken to 0x61438 when `ret(0x125b08) == 0x1`, falls through to 0x61428 when `ret(0x125b08) != 0x1` (source `bl` at 0x613fc).
  Call (`bl` at 0x61454): (unknown callee), target 0x21ce0, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x61480): (unknown callee), target 0x22070, r0=`0x1`, r1=`0x162ee0`, r2=`0xa9`, r3=`0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x61494).

## `ip69_set_link_enable`

- `ip69_set_link_enable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x44094 to 0x4414c.
  Established facts:
  Decoded words in [0x44148, 0x4414c) are not reached from the entry by known edges.
  Byte load: target `abs 0x1a5b38` (`ldrb` at 0x440d4).
  Branch (`bne` at 0x440dc): taken to 0x440ec when `[0x1a5b38]:1 != 0x0`, falls through to 0x440e0 when `[0x1a5b38]:1 == 0x0`.
  Call (`bl` at 0x4410c): (unknown callee), target 0x2e1e4, r0=`r0`, r1=`r1`, r2=`r2`, r3=`unknown`.
  Call (`bl` at 0x44128): (unknown callee), target 0x6a918, r0=`r0`, r1=`r1`, r2=`r2`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x44144).

## `vortix_set_link_enable`

- `vortix_set_link_enable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2af68 to 0x2b058.
  Established facts:
  Decoded words in [0x2b054, 0x2b058) are not reached from the entry by known edges.
  Byte load: target `abs 0x1a5978` (`ldrb` at 0x2afe0).
  Branch (`bne` at 0x2afe8): taken to 0x2aff8 when `[0x1a5978]:1 != 0x0`, falls through to 0x2afec when `[0x1a5978]:1 == 0x0`.
  Call (`bl` at 0x2b018): (unknown callee), target 0x2e1e4, r0=`r0`, r1=`r1`, r2=`r2`, r3=`unknown`.
  Call (`bl` at 0x2b034): (unknown callee), target 0x6a918, r0=`r0`, r1=`r1`, r2=`r2`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x2b050).
