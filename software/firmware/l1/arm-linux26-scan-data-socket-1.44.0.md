# ARM Linux 2.6 scan data and socket routines in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-scan-data-socket-facts.txt) retain all tool sections.

The notation `ret(ADDR)` identifies a call-result expression without distinguishing
individual call sites to that target. Each return line records machine-register
state from the analysis rather than a C return type.


Groups summarize static access records by printed shape. Instruction-address order can include different control-flow paths.

## `setup_tdrdata_tio`

- `setup_tdrdata_tio`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x145608 to 0x145c74.
  Established facts:
  Decoded words in [0x145bfc, 0x145c74) are not reached from the entry by known edges.
  Call (`bl` at 0x14562c): (unknown callee), target 0x201fc, r0=`0x44`, r1=`r1`, r2=`r2`, r3=`0x0`.
  Word load: target `r0 +0x28` (`ldr` at 0x145634).
  Branch (`bne` at 0x14563c): taken to 0x14564c when `[r0+40] != 0x0`, falls through to 0x145640 when `[r0+40] == 0x0`.
  Call (`bl` at 0x145698): (unknown callee), target 0x143db8, r0=`0x193800`, r1=`0x7c`, r2=`0x1a2644`, r3=`unknown`.
  Call (`bl` at 0x1456e8): (unknown callee), target 0x143db8, r0=`0x19380c`, r1=`0x7d`, r2=`0x1a265c`, r3=`unknown`.
  Call (`bl` at 0x145738): (unknown callee), target 0x143db8, r0=`0x193818`, r1=`0x7f`, r2=`0x1a2674`, r3=`unknown`.
  Call (`bl` at 0x145788): (unknown callee), target 0x143db8, r0=`0x193820`, r1=`0x7e`, r2=`0x1a0dc8`, r3=`unknown`.
  Call (`bl` at 0x1457d8): (unknown callee), target 0x143db8, r0=`0x193828`, r1=`0x7`, r2=`0x1a0e94`, r3=`unknown`.
  Call (`bl` at 0x145828): (unknown callee), target 0x143db8, r0=`0x193830`, r1=`0x5`, r2=`0x1a0ed0`, r3=`unknown`.
  Call (`bl` at 0x145878): (unknown callee), target 0x143db8, r0=`0x19383c`, r1=`0x6`, r2=`0x1a10b4`, r3=`unknown`.
  Call (`bl` at 0x1458c8): (unknown callee), target 0x143db8, r0=`0x193844`, r1=`0x12`, r2=`0x1a268c`, r3=`unknown`.
  Call (`bl` at 0x145918): (unknown callee), target 0x143db8, r0=`0x193854`, r1=`0x11`, r2=`0x1a26a4`, r3=`unknown`.
  Call (`bl` at 0x145968): (unknown callee), target 0x143db8, r0=`0x193864`, r1=`0xa`, r2=`0x1a1194`, r3=`unknown`.
  Call (`bl` at 0x1459b8): (unknown callee), target 0x143db8, r0=`0x193870`, r1=`0xb`, r2=`0x1a1194`, r3=`unknown`.
  Call (`bl` at 0x145a08): (unknown callee), target 0x143db8, r0=`0x19387c`, r1=`0x1a`, r2=`0x1a2790`, r3=`unknown`.
  Call (`bl` at 0x145a58): (unknown callee), target 0x143db8, r0=`0x19388c`, r1=`0x1b`, r2=`0x1a2790`, r3=`unknown`.
  Call (`bl` at 0x145aa8): (unknown callee), target 0x143db8, r0=`0x19389c`, r1=`0x60`, r2=`0x1a1234`, r3=`unknown`.
  Call (`bl` at 0x145af8): (unknown callee), target 0x143db8, r0=`0x1938ac`, r1=`0x62`, r2=`0x1a1234`, r3=`unknown`.
  Call (`bl` at 0x145b48): (unknown callee), target 0x143db8, r0=`0x1938bc`, r1=`0x61`, r2=`0x1a2808`, r3=`unknown`.
  Call (`bl` at 0x145b98): (unknown callee), target 0x143db8, r0=`0x1938cc`, r1=`0x63`, r2=`0x1a2808`, r3=`unknown`.
  Recorded r0 = `[sp-36]` (`ldm` at 0x145bf8).

## `setup_tdrdata_nl4r`

- `setup_tdrdata_nl4r`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x145c74 to 0x1463a4.
  Established facts:
  Decoded words in [0x146330, 0x1463a4) are not reached from the entry by known edges.
  Call (`bl` at 0x145ca0): (unknown callee), target 0x201fc, r0=`0xa8`, r1=`r1`, r2=`r2`, r3=`0x194374`.
  Word load: target `r0 +0x28` (`ldr` at 0x145ca8).
  Branch (`bne` at 0x145cb0): taken to 0x145cc0 when `[r0+40] != 0x0`, falls through to 0x145cb4 when `[r0+40] == 0x0`.
  Call (`bl` at 0x145d0c): (unknown callee), target 0x143db8, r0=`0x194380`, r1=`0x7c`, r2=`0x1a2644`, r3=`unknown`.
  Call (`bl` at 0x145d5c): (unknown callee), target 0x143db8, r0=`0x19438c`, r1=`0x7d`, r2=`0x1a265c`, r3=`unknown`.
  Call (`bl` at 0x145dac): (unknown callee), target 0x143db8, r0=`0x194398`, r1=`0x7f`, r2=`0x1a2674`, r3=`unknown`.
  Call (`bl` at 0x145dfc): (unknown callee), target 0x143db8, r0=`0x1943a0`, r1=`0x7e`, r2=`0x1a1258`, r3=`unknown`.
  Call (`bl` at 0x145e4c): (unknown callee), target 0x143db8, r0=`0x1943a8`, r1=`0x7`, r2=`0x1a12c4`, r3=`unknown`.
  Call (`bl` at 0x145e9c): (unknown callee), target 0x143db8, r0=`0x1943b0`, r1=`0x5`, r2=`0x1a1300`, r3=`unknown`.
  Call (`bl` at 0x145eec): (unknown callee), target 0x143db8, r0=`0x1943bc`, r1=`0x6`, r2=`0x1a178c`, r3=`unknown`.
  Call (`bl` at 0x145f3c): (unknown callee), target 0x143db8, r0=`0x1943c4`, r1=`0xd`, r2=`0x1a268c`, r3=`unknown`.
  Call (`bl` at 0x145f8c): (unknown callee), target 0x143db8, r0=`0x1943d4`, r1=`0xc`, r2=`0x1a26a4`, r3=`unknown`.
  Call (`bl` at 0x145fdc): (unknown callee), target 0x143db8, r0=`0x1943e4`, r1=`0x4b`, r2=`0x1a2824`, r3=`unknown`.
  Call (`bl` at 0x14605c): (unknown callee), target 0x205bc, r0=`sp-64`, r1=`0x1943f0`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x14607c): (unknown callee), target 0x143db8, r0=`sp-64`, r1=`unknown`, r2=`0x1a1988`, r3=`unknown`.
  Call (`bl` at 0x146118): (unknown callee), target 0x205bc, r0=`sp-64`, r1=`0x1943f8`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x146138): (unknown callee), target 0x143db8, r0=`sp-64`, r1=`unknown`, r2=`0x1a2790`, r3=`unknown`.
  Call (`bl` at 0x1461d4): (unknown callee), target 0x205bc, r0=`sp-64`, r1=`0x194404`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x1461f4): (unknown callee), target 0x143db8, r0=`sp-64`, r1=`unknown`, r2=`0x1a1a64`, r3=`unknown`.
  Call (`bl` at 0x146290): (unknown callee), target 0x205bc, r0=`sp-64`, r1=`0x194414`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x1462b0): (unknown callee), target 0x143db8, r0=`sp-64`, r1=`unknown`, r2=`0x1a2808`, r3=`unknown`.
  Recorded r0 = `[sp-72]` (`ldm` at 0x14632c).

## `setup_tdrdata_shub2`

- `setup_tdrdata_shub2`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1463a4 to 0x146cd0.
  Established facts:
  Decoded words in [0x146c18, 0x146cd0) are not reached from the entry by known edges.
  Call (`bl` at 0x1463c8): (unknown callee), target 0x201fc, r0=`0x64`, r1=`r1`, r2=`r2`, r3=`0x0`.
  Word load: target `r0 +0x28` (`ldr` at 0x1463d0).
  Branch (`bne` at 0x1463d8): taken to 0x1463e8 when `[r0+40] != 0x0`, falls through to 0x1463dc when `[r0+40] == 0x0`.
  Call (`bl` at 0x146434): (unknown callee), target 0x143db8, r0=`0x194f94`, r1=`0x7c`, r2=`0x1a2644`, r3=`unknown`.
  Call (`bl` at 0x146484): (unknown callee), target 0x143db8, r0=`0x194fa0`, r1=`0x7d`, r2=`0x1a265c`, r3=`unknown`.
  Call (`bl` at 0x1464d4): (unknown callee), target 0x143db8, r0=`0x194fac`, r1=`0x7f`, r2=`0x1a2674`, r3=`unknown`.
  Call (`bl` at 0x146524): (unknown callee), target 0x143db8, r0=`0x194fb4`, r1=`0x7e`, r2=`0x1a1c08`, r3=`unknown`.
  Call (`bl` at 0x146574): (unknown callee), target 0x143db8, r0=`0x194fbc`, r1=`0x7`, r2=`0x1a1ca4`, r3=`unknown`.
  Call (`bl` at 0x1465c4): (unknown callee), target 0x143db8, r0=`0x194fc4`, r1=`0x5`, r2=`0x1a1db4`, r3=`unknown`.
  Call (`bl` at 0x146614): (unknown callee), target 0x143db8, r0=`0x194fd0`, r1=`0x6`, r2=`0x1a2000`, r3=`unknown`.
  Call (`bl` at 0x146664): (unknown callee), target 0x143db8, r0=`0x194fd8`, r1=`0x12`, r2=`0x1a268c`, r3=`unknown`.
  Call (`bl` at 0x1466b4): (unknown callee), target 0x143db8, r0=`0x194fe8`, r1=`0x11`, r2=`0x1a26a4`, r3=`unknown`.
  Call (`bl` at 0x146704): (unknown callee), target 0x143db8, r0=`0x194ff8`, r1=`0xa`, r2=`0x1a2250`, r3=`unknown`.
  Call (`bl` at 0x146754): (unknown callee), target 0x143db8, r0=`0x195004`, r1=`0xb`, r2=`0x1a2250`, r3=`unknown`.
  Call (`bl` at 0x1467a4): (unknown callee), target 0x143db8, r0=`0x195010`, r1=`0x1a`, r2=`0x1a2790`, r3=`unknown`.
  Call (`bl` at 0x1467f4): (unknown callee), target 0x143db8, r0=`0x195020`, r1=`0x1b`, r2=`0x1a2790`, r3=`unknown`.
  Call (`bl` at 0x146844): (unknown callee), target 0x143db8, r0=`0x195030`, r1=`0x60`, r2=`0x1a232c`, r3=`unknown`.
  Call (`bl` at 0x146894): (unknown callee), target 0x143db8, r0=`0x195040`, r1=`0x62`, r2=`0x1a232c`, r3=`unknown`.
  Call (`bl` at 0x1468e4): (unknown callee), target 0x143db8, r0=`0x195050`, r1=`0x61`, r2=`0x1a2808`, r3=`unknown`.
  Call (`bl` at 0x146934): (unknown callee), target 0x143db8, r0=`0x195060`, r1=`0x63`, r2=`0x1a2808`, r3=`unknown`.
  Call (`bl` at 0x146984): (unknown callee), target 0x143db8, r0=`0x195070`, r1=`0x14`, r2=`0x1a2824`, r3=`unknown`.
  Call (`bl` at 0x1469d4): (unknown callee), target 0x143db8, r0=`0x19507c`, r1=`0x9`, r2=`0x1a2040`, r3=`unknown`.
  Call (`bl` at 0x146a24): (unknown callee), target 0x143db8, r0=`0x195088`, r1=`0xc`, r2=`0x1a1cc0`, r3=`unknown`.
  Call (`bl` at 0x146a74): (unknown callee), target 0x143db8, r0=`0x195098`, r1=`0xe`, r2=`0x1a1cd8`, r3=`unknown`.
  Call (`bl` at 0x146ac4): (unknown callee), target 0x143db8, r0=`0x1950a8`, r1=`0xd`, r2=`0x1a1d28`, r3=`unknown`.
  Call (`bl` at 0x146b14): (unknown callee), target 0x143db8, r0=`0x1950b4`, r1=`0x15`, r2=`0x1a1d84`, r3=`unknown`.
  Call (`bl` at 0x146b64): (unknown callee), target 0x143db8, r0=`0x1950c0`, r1=`0x13`, r2=`0x1a283c`, r3=`unknown`.
  Call (`bl` at 0x146bb4): (unknown callee), target 0x143db8, r0=`0x1950cc`, r1=`0x3`, r2=`0x1a2878`, r3=`unknown`.
  Recorded r0 = `[sp-36]` (`ldm` at 0x146c14).

## `setup_tdrdata_shub`

- `setup_tdrdata_shub`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x14504c to 0x145608.
  Established facts:
  Decoded words in [0x1455a0, 0x145608) are not reached from the entry by known edges.
  Call (`bl` at 0x145070): (unknown callee), target 0x201fc, r0=`0x3c`, r1=`r1`, r2=`r2`, r3=`0x0`.
  Word load: target `r0 +0x28` (`ldr` at 0x145078).
  Branch (`bne` at 0x145080): taken to 0x145090 when `[r0+40] != 0x0`, falls through to 0x145084 when `[r0+40] == 0x0`.
  Call (`bl` at 0x1450dc): (unknown callee), target 0x143db8, r0=`0x193154`, r1=`0x7c`, r2=`0x1a2644`, r3=`unknown`.
  Call (`bl` at 0x14512c): (unknown callee), target 0x143db8, r0=`0x193160`, r1=`0x7d`, r2=`0x1a265c`, r3=`unknown`.
  Call (`bl` at 0x14517c): (unknown callee), target 0x143db8, r0=`0x19316c`, r1=`0x7f`, r2=`0x1a2674`, r3=`unknown`.
  Call (`bl` at 0x1451cc): (unknown callee), target 0x143db8, r0=`0x193174`, r1=`0x7e`, r2=`0x1a095c`, r3=`unknown`.
  Call (`bl` at 0x14521c): (unknown callee), target 0x143db8, r0=`0x19317c`, r1=`0x7`, r2=`0x1a09ec`, r3=`unknown`.
  Call (`bl` at 0x14526c): (unknown callee), target 0x143db8, r0=`0x193184`, r1=`0x5`, r2=`0x1a0a28`, r3=`unknown`.
  Call (`bl` at 0x1452bc): (unknown callee), target 0x143db8, r0=`0x193190`, r1=`0x6`, r2=`0x1a0be4`, r3=`unknown`.
  Call (`bl` at 0x14530c): (unknown callee), target 0x143db8, r0=`0x193198`, r1=`0xa`, r2=`0x1a0c8c`, r3=`unknown`.
  Call (`bl` at 0x14535c): (unknown callee), target 0x143db8, r0=`0x1931a4`, r1=`0xb`, r2=`0x1a0c8c`, r3=`unknown`.
  Call (`bl` at 0x1453ac): (unknown callee), target 0x143db8, r0=`0x1931b0`, r1=`0x1a`, r2=`0x1a2790`, r3=`unknown`.
  Call (`bl` at 0x1453fc): (unknown callee), target 0x143db8, r0=`0x1931c0`, r1=`0x1b`, r2=`0x1a2790`, r3=`unknown`.
  Call (`bl` at 0x14544c): (unknown callee), target 0x143db8, r0=`0x1931d0`, r1=`0x60`, r2=`0x1a0d04`, r3=`unknown`.
  Call (`bl` at 0x14549c): (unknown callee), target 0x143db8, r0=`0x1931e0`, r1=`0x62`, r2=`0x1a0d04`, r3=`unknown`.
  Call (`bl` at 0x1454ec): (unknown callee), target 0x143db8, r0=`0x1931f0`, r1=`0x61`, r2=`0x1a2808`, r3=`unknown`.
  Call (`bl` at 0x14553c): (unknown callee), target 0x143db8, r0=`0x193200`, r1=`0x63`, r2=`0x1a2808`, r3=`unknown`.
  Recorded r0 = `[sp-36]` (`ldm` at 0x14559c).

## `setup_tdrdata_psic`

- `setup_tdrdata_psic`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x14af74 to 0x14b54c.
  Established facts:
  Decoded words in [0x14b4f0, 0x14b54c) are not reached from the entry by known edges.
  Call (`bl` at 0x14af98): (unknown callee), target 0x201fc, r0=`0x16c`, r1=`r1`, r2=`r2`, r3=`0x0`.
  Word load: target `r0 +0x28` (`ldr` at 0x14afa4).
  Branch (`bne` at 0x14afac): taken to 0x14afbc when `[r0+40] != 0x0`, falls through to 0x14afb0 when `[r0+40] == 0x0`.
  Call (`bl` at 0x14b008): (unknown callee), target 0x143db8, r0=`0x1956d8`, r1=`0x8`, r2=`0x1a2344`, r3=`unknown`.
  Call (`bl` at 0x14b0d4): (unknown callee), target 0x205bc, r0=`sp-68`, r1=`unknown`, r2=`[sp-32]`, r3=`unknown`.
  Call (`bl` at 0x14b114): (unknown callee), target 0x143db8, r0=`sp-68`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x14b184): (unknown callee), target 0x143db8, r0=`0x19575c`, r1=`0x10`, r2=`0x1a235c`, r3=`unknown`.
  Call (`bl` at 0x14b1d4): (unknown callee), target 0x143db8, r0=`0x195768`, r1=`0x11`, r2=`0x1a2374`, r3=`unknown`.
  Call (`bl` at 0x14b224): (unknown callee), target 0x143db8, r0=`0x195774`, r1=`0x12`, r2=`0x1a2374`, r3=`unknown`.
  Call (`bl` at 0x14b274): (unknown callee), target 0x143db8, r0=`0x195780`, r1=`0x13`, r2=`0x1a238c`, r3=`unknown`.
  Call (`bl` at 0x14b2c4): (unknown callee), target 0x143db8, r0=`0x19578c`, r1=`0x14`, r2=`0x1a238c`, r3=`unknown`.
  Call (`bl` at 0x14b314): (unknown callee), target 0x143db8, r0=`0x195798`, r1=`0x15`, r2=`0x1a23f8`, r3=`unknown`.
  Call (`bl` at 0x14b364): (unknown callee), target 0x143db8, r0=`0x1957a4`, r1=`0x16`, r2=`0x1a23f8`, r3=`unknown`.
  Call (`bl` at 0x14b3b4): (unknown callee), target 0x143db8, r0=`0x1957b0`, r1=`0x17`, r2=`0x1a2410`, r3=`unknown`.
  Call (`bl` at 0x14b404): (unknown callee), target 0x143db8, r0=`0x1957bc`, r1=`0x18`, r2=`0x1a2410`, r3=`unknown`.
  Call (`bl` at 0x14b454): (unknown callee), target 0x143db8, r0=`0x1957c8`, r1=`0x19`, r2=`0x1a253c`, r3=`unknown`.
  Call (`bl` at 0x14b4cc): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x195818`, r2=`0x153`, r3=`0x0`.
  Recorded r0 = `[sp-112]` (`ldm` at 0x14b4ec).

## `setup_tdrdata_lbi`

- `setup_tdrdata_lbi`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x14b54c to 0x14b574.
  Established facts:
  Call (`bl` at 0x14b564): (unknown callee), target 0x14af74, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Recorded r0 = `ret(0x14af74)` (`ldm` at 0x14b570, `bl` at 0x14b564).

## `tdrdata_dump`

- `tdrdata_dump`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x144090 to 0x1443d0.
  Established facts:
  Decoded words in [0x1443a0, 0x1443d0) are not reached from the entry by known edges.
  Branch (`bne` at 0x1440b8): taken to 0x1440c0 when `r2 != 0x0`, falls through to 0x1440bc when `r2 == 0x0`.
  Word load: target `abs 0x1a2888` (`ldr` at 0x1440d0).
  Call (`bl` at 0x1440ec): (unknown callee), target 0x244d4, r0=`unknown`, r1=`0x192aec`, r2=`r2`, r3=`r1`.
  Call (`bl` at 0x144100): (unknown callee), target 0x244d4, r0=`unknown`, r1=`0x192b0c`, r2=`r1`, r3=`r1`.
  Call (`bl` at 0x144134): (unknown callee), target 0x244d4, r0=`[sp-20]`, r1=`0x192b18`, r2=`unknown`, r3=`unknown`.
  Grouped accesses: 4 reported halfword load records with printed base expression `[sp-28]` and nonnegative halfword-aligned offsets, including target `[sp-28] +0x4` (`ldrh` at 0x14413c), target `[sp-28] +0x4` (`ldrh` at 0x144144), target `[sp-28] +0x6` (`ldrh` at 0x144158).
  Call (`bl` at 0x144150): (unknown callee), target 0x244d4, r0=`[sp-20]`, r1=`0x192b24`, r2=`[[sp-28]+4]:2`, r3=`[[sp-28]+4]:2`.
  Call (`bl` at 0x14416c): (unknown callee), target 0x244d4, r0=`[sp-20]`, r1=`0x192b3c`, r2=`[[sp-28]+6]:2`, r3=`[[sp-28]+6]:2`.
  Call (`bl` at 0x1441a0): (unknown callee), target 0x14364c, r0=`[sp-20]`, r1=`0x192b54`, r2=`[[sp-28]+8]`, r3=`[sp-28]`.
  Call (`bl` at 0x1441bc): (unknown callee), target 0x244d4, r0=`[sp-20]`, r1=`0x192b70`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x144254): (unknown callee), target 0x244d4, r0=`[sp-20]`, r1=`0x192b8c`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x144270): (unknown callee), target 0x244d4, r0=`[sp-20]`, r1=`0x192b94`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x1442d8): (unknown callee), target 0x244d4, r0=`[sp-20]`, r1=`0x192b8c`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x1442f4): (unknown callee), target 0x244d4, r0=`[sp-20]`, r1=`0x192b94`, r2=`unknown`, r3=`[sp-48]`.
  Call (`bl` at 0x144374): (unknown callee), target 0x14bab4, r0=`[sp-20]`, r1=`0x192b98`, r2=`[sp-56]`, r3=`[[sp-28]+16]`.
  Call (`bl` at 0x144394): (unknown callee), target 0x1443d0, r0=`[sp-20]`, r1=`[sp-28]`, r2=`0x1`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x14439c).

## `tdrdata_dump_fields`

- `tdrdata_dump_fields`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1443d0 to 0x144710.
  Established facts:
  Decoded words in [0x144700, 0x144710) are not reached from the entry by known edges.
  Branch (`bne` at 0x144410): taken to 0x144418 when `[r1+8] != 0x0`, falls through to 0x144414 when `[r1+8] == 0x0`.
  Call (`bl` at 0x144494): (unknown callee), target 0x201fc, r0=`[sp-120]`, r1=`r1`, r2=`r2`, r3=`unknown`.
  Call (`bl` at 0x1444bc): (unknown callee), target 0x244d4, r0=`[sp-20]`, r1=`0x192bac`, r2=`[[sp-24]]`, r3=`[sp-24]`.
  Grouped accesses: 2 reported word store records with absolute target `0x3151cc`, including target `abs 0x3151cc`, value `0x0` (`str` at 0x144528), target `abs 0x3151cc`, value `[0x3151cc]+1` (`str` at 0x144628).
  Grouped accesses: 4 reported word load records with absolute target `0x3151cc`, including target `abs 0x3151cc` (`ldr` at 0x1445a0), target `abs 0x3151cc` (`ldr` at 0x1445ec), target `abs 0x3151cc` (`ldr` at 0x144600).
  Call (`bl` at 0x144650): (unknown callee), target 0x143a3c, r0=`[[sp-24]+8]`, r1=`[sp-32]`, r2=`0x0`, r3=`[[sp-24]+16]`.
  Call (`bl` at 0x144688): (unknown callee), target 0x144710, r0=`unknown`, r1=`[sp-36]`, r2=`sp-116`, r3=`unknown`.
  Call (`bl` at 0x1446dc): (unknown callee), target 0x243d4, r0=`0x192bcc`, r1=`unknown`, r2=`unknown`, r3=`sp-116`.
  Call (`bl` at 0x1446f4): (unknown callee), target 0x20514, r0=`[sp-36]`, r1=`unknown`, r2=`[[[sp-24]+8]+2]:2`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x1446fc).

## `tdrdata_create_field_str`

- `tdrdata_create_field_str`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x144710 to 0x144848.
  Established facts:
  Decoded words in [0x144844, 0x144848) are not reached from the entry by known edges.
  Byte store: target `(unknown effective address)`, value `unknown` (`strb` at 0x1447e0).
  Branch (`bne` at 0x1447f8): taken to 0x144814 when `[sp-44] != 0x0`, falls through to 0x1447fc when `[sp-44] == 0x0`.
  Byte store: target `(unknown effective address)`, value `0x0` (`strb` at 0x144838).
  Recorded r0 = `r0` (`ldm` at 0x144840).

## `tdrdata_build_mask`

- `tdrdata_build_mask`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x144f4c to 0x14504c.
  Established facts:
  Word load: target `r0 +0x8` (`ldr` at 0x144f70).
  Halfword load: target `[r0+8] +0x0` (`ldrh` at 0x144f74).
  Call (`bl` at 0x144f84): (unknown callee), target 0x132208, r0=`r2`, r1=`[[r0+8]]:2`, r2=`r3`, r3=`[[r0+8]]:2`.
  Call (`bl` at 0x145030): (unknown callee), target 0x13232c, r0=`unknown`, r1=`unknown`, r2=`[sp-32]`, r3=`unknown`.
  Recorded r0 = `unknown` (`ldm` at 0x145048).

## `nl4r_init_ports`

- `nl4r_init_ports`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2f150 to 0x2f650.
  Established facts:
  Decoded words in [0x2f624, 0x2f650) are not reached from the entry by known edges.
  Byte load: target `abs 0x1562a4` (`ldrb` at 0x2f168).
  Call (`bl` at 0x2f180): (unknown callee), target 0x20550, r0=`sp-55`, r1=`0x0`, r2=`0x13`, r3=`sp-55`.
  Grouped accesses: 6 reported word load records with absolute target `0x315034`, including target `abs 0x315034` (`ldr` at 0x2f190), target `abs 0x315034` (`ldr` at 0x2f3b0), target `abs 0x315034` (`ldr` at 0x2f46c).
  Branch (`beq` at 0x2f198): taken to 0x2f1a8 when `[0x315034] == 0x0`, falls through to 0x2f19c when `[0x315034] != 0x0`.
  Call (`bl` at 0x2f1a4): (unknown callee), target 0x243d4, r0=`0x1562b8`, r1=`[sp-20]`, r2=`unknown`, r3=`[0x315034]`.
  Call (`bl` at 0x2f1cc): (unknown callee), target 0x2fabc, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2f238): (unknown callee), target 0x13ccc0, r0=`[sp-20]`, r1=`unknown`, r2=`unknown`, r3=`[sp-56]:1`.
  Call (`bl` at 0x2f24c): (unknown callee), target 0x29908, r0=`0x1562dc`, r1=`ret(0x13ccc0)`, r2=`sp-56`, r3=`ret(0x13ccc0)`.
  Call (`bl` at 0x2f2bc): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-24]+10`, r2=`0x1`, r3=`sp-32`.
  Call (`bl` at 0x2f2c8): (unknown callee), target 0x2fabc, r0=`[sp-20]`, r1=`[sp-24]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2f304): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-76]`, r2=`0x2`, r3=`sp-64`.
  Call (`bl` at 0x2f318): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-80]`, r2=`0x1`, r3=`sp-72`.
  Call (`bl` at 0x2f324): (unknown callee), target 0x2fc94, r0=`[sp-20]`, r1=`[sp-24]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2f36c): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-76]`, r2=`0x1f`, r3=`sp-32`.
  Call (`bl` at 0x2f380): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-76]`, r2=`0x20`, r3=`sp-32`.
  Call (`bl` at 0x2f394): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-76]`, r2=`0x21`, r3=`sp-32`.
  Call (`bl` at 0x2f3a8): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`[sp-76]`, r2=`0x22`, r3=`sp-32`.
  Call (`bl` at 0x2f3d4): (unknown callee), target 0x243d4, r0=`0x156304`, r1=`[sp-24]+65`, r2=`[sp-64]`, r3=`[sp-32]`.
  Call (`bl` at 0x2f3e8): (unknown callee), target 0x13fd34, r0=`[sp-20]`, r1=`0x2`, r2=`[sp-76]`, r3=`0x0`.
  Call (`bl` at 0x2f41c): (unknown callee), target 0x13eae8, r0=`[sp-20]`, r1=`[sp-76]`, r2=`0x0`, r3=`sp-32`.
  Call (`bl` at 0x2f488): (unknown callee), target 0x243d4, r0=`0x15633c`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2f4a4): (unknown callee), target 0x13fd34, r0=`[sp-20]`, r1=`0x2`, r2=`unknown`, r3=`0x0`.
  Call (`bl` at 0x2f50c): (unknown callee), target 0x243d4, r0=`0x156360`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x2f528): (unknown callee), target 0x13fd34, r0=`[sp-20]`, r1=`0x2`, r2=`unknown`, r3=`0x0`.
  Call (`bl` at 0x2f574): (unknown callee), target 0x13f048, r0=`[sp-20]`, r1=`0x5`, r2=`0x0`, r3=`sp-32`.
  Call (`bl` at 0x2f58c): (unknown callee), target 0x243d4, r0=`0x156384`, r1=`unknown`, r2=`unknown`, r3=`[0x315034]`.
  Call (`bl` at 0x2f5a0): (unknown callee), target 0x13fd34, r0=`[sp-20]`, r1=`0x2`, r2=`0x5`, r3=`0x0`.
  Call (`bl` at 0x2f5d8): (unknown callee), target 0x243d4, r0=`0x1563a4`, r1=`unknown`, r2=`unknown`, r3=`[0x315034]`.
  Call (`bl` at 0x2f5ec): (unknown callee), target 0x13fd34, r0=`[sp-20]`, r1=`0x2`, r2=`0x9`, r3=`0x0`.
  Recorded r0 = `unknown` (`ldm` at 0x2f620).

## `t1_displayInit`

- `t1_displayInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12a9d4 to 0x12aa78.
  Established facts:
  Decoded words in [0x12aa64, 0x12aa78) are not reached from the entry by known edges.
  Byte store: target `r0 +0x2`, value `0xc` (`strb` at 0x12a9f0).
  Grouped accesses: 5 reported word store records with printed base expression `[sp-20]` and nonnegative word-aligned offsets, including target `[sp-20] +0x8`, value `0x31e494` (`str` at 0x12aa20), target `[sp-20] +0xc`, value `0x12aa78` (`str` at 0x12aa2c), target `[sp-20] +0x10`, value `0x12ac24` (`str` at 0x12aa38).
  Word store: target `abs 0x314ff8`, value `[sp-20]` (`str` at 0x12aa5c).
  Recorded r0 = `r0` (`ldm` at 0x12aa60).

## `dmcout_data_display`

- `dmcout_data_display`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x133400 to 0x1335c0.
  Established facts:
  Decoded words in [0x1335a4, 0x1335c0) are not reached from the entry by known edges.
  Branch (`beq` at 0x133428): taken to 0x133440 when `r1 == 0x0`, falls through to 0x13342c when `r1 != 0x0`.
  Word load: target `r1 +0x0` (`ldr` at 0x133438).
  Call (`bl` at 0x13343c): (unknown callee), target 0x243d4, r0=`0x190a88`, r1=`r0`, r2=`[r1]`, r3=`r1`.
  Call (`bl` at 0x133460): (unknown callee), target 0x243d4, r0=`0x190aa0`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x13346c): (unknown callee), target 0x243d4, r0=`0x190ab8`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x1334a4): (unknown callee), target 0x243d4, r0=`0x190ad0`, r1=`unknown`, r2=`[sp-32]`, r3=`unknown`.
  Call (`bl` at 0x1334c8): (unknown callee), target 0x243d4, r0=`0x190adc`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x1334d4): (unknown callee), target 0x243d4, r0=`0x190ae0`, r1=`[sp-20]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x13350c): (unknown callee), target 0x243d4, r0=`0x190ad0`, r1=`unknown`, r2=`[sp-32]`, r3=`unknown`.
  Call (`bl` at 0x133530): (unknown callee), target 0x243d4, r0=`0x190adc`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x13353c): (unknown callee), target 0x243d4, r0=`0x190af8`, r1=`[sp-20]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x133574): (unknown callee), target 0x243d4, r0=`0x190ad0`, r1=`unknown`, r2=`[sp-32]`, r3=`unknown`.
  Call (`bl` at 0x133598): (unknown callee), target 0x243d4, r0=`0x190adc`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `ret(0x243d4)` (`ldm` at 0x1335a0, `bl` at 0x133598).

## `SocketExportUnixConnection`

- `SocketExportUnixConnection`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfeb30 to 0xfec88.
  Established facts:
  Decoded words in [0xfec74, 0xfec88) are not reached from the entry by known edges.
  Call (`bl` at 0xfeb50): (unknown callee), target 0x205d4, r0=`0x1`, r1=`0x1`, r2=`0x0`, r3=`r3`.
  Call (`bl` at 0xfeb68): (unknown callee), target 0x20184, r0=`ret(0x205d4)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x205d4)`.
  Call (`bl` at 0xfeb80): (unknown callee), target 0x215fc, r0=`unknown`, r1=`0x1884f4`, r2=`[ret(0x20184)]`, r3=`0x1884fc`.
  Call (`bl` at 0xfeb9c): (unknown callee), target 0xffc08, r0=`sp-132`, r1=`[sp-20]`, r2=`unknown`, r3=`sp-132`.
  Call (`bl` at 0xfebb4): (unknown callee), target 0x20100, r0=`sp-130`, r1=`unknown`, r2=`unknown`, r3=`sp-130`.
  Call (`bl` at 0xfebc8): (unknown callee), target 0x203d0, r0=`[sp-136]`, r1=`sp-132`, r2=`[sp-140]`, r3=`sp-132`.
  Call (`bl` at 0xfebd8): (unknown callee), target 0x20184, r0=`ret(0x203d0)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x203d0)`.
  Call (`bl` at 0xfec00): (unknown callee), target 0x215fc, r0=`unknown`, r1=`0x18851c`, r2=`[ret(0x20184)]`, r3=`0x18852c`.
  Call (`bl` at 0xfec08): (unknown callee), target 0x200b8, r0=`[sp-136]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xfec20): (unknown callee), target 0x201f0, r0=`[sp-136]`, r1=`0x5`, r2=`unknown`, r3=`ret(0x203d0)`.
  Call (`bl` at 0xfec30): (unknown callee), target 0x20184, r0=`ret(0x201f0)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x201f0)`.
  Call (`bl` at 0xfec48): (unknown callee), target 0x215fc, r0=`unknown`, r1=`0x188524`, r2=`[ret(0x20184)]`, r3=`0x1884fc`.
  Call (`bl` at 0xfec50): (unknown callee), target 0x200b8, r0=`[sp-136]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `[sp-144]` (`ldm` at 0xfec70).

## `SocketExportInetConnection`

- `SocketExportInetConnection`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfe954 to 0xfeb30.
  Established facts:
  Decoded words in [0xfeb18, 0xfeb30) are not reached from the entry by known edges.
  Call (`bl` at 0xfe97c): (unknown callee), target 0x205d4, r0=`0x2`, r1=`0x1`, r2=`0x0`, r3=`0x1`.
  Call (`bl` at 0xfe994): (unknown callee), target 0x20184, r0=`ret(0x205d4)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x205d4)`.
  Call (`bl` at 0xfe9ac): (unknown callee), target 0x215fc, r0=`0xfffffe70`, r1=`0x1884f4`, r2=`[ret(0x20184)]`, r3=`0x1884fc`.
  Call (`bl` at 0xfe9d4): (unknown callee), target 0x202a4, r0=`ret(0x205d4)`, r1=`0x1`, r2=`0x2`, r3=`sp-44`.
  Call (`bl` at 0xfe9e4): (unknown callee), target 0x20184, r0=`ret(0x202a4)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x202a4)`.
  Call (`bl` at 0xfe9fc): (unknown callee), target 0x215fc, r0=`0xfffffe70`, r1=`0x188500`, r2=`[ret(0x20184)]`, r3=`0x18850c`.
  Call (`bl` at 0xfea04): (unknown callee), target 0x200b8, r0=`[sp-40]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xfea24): (unknown callee), target 0x20550, r0=`sp-36`, r1=`0x0`, r2=`0x10`, r3=`sp-36`.
  Call (`bl` at 0xfea40): (unknown callee), target 0x20538, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xfea64): (unknown callee), target 0x203d0, r0=`[sp-40]`, r1=`sp-36`, r2=`0x10`, r3=`sp-36`.
  Call (`bl` at 0xfea74): (unknown callee), target 0x20184, r0=`ret(0x203d0)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x203d0)`.
  Call (`bl` at 0xfeaa0): (unknown callee), target 0x21664, r0=`0xfffffe70`, r1=`0x18851c`, r2=`[ret(0x20184)]`, r3=`0x1884fc`.
  Call (`bl` at 0xfeaa8): (unknown callee), target 0x200b8, r0=`[sp-40]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xfeac0): (unknown callee), target 0x201f0, r0=`[sp-40]`, r1=`0x5`, r2=`unknown`, r3=`ret(0x203d0)`.
  Call (`bl` at 0xfead0): (unknown callee), target 0x20184, r0=`ret(0x201f0)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x201f0)`.
  Call (`bl` at 0xfeae8): (unknown callee), target 0x215fc, r0=`0xfffffe70`, r1=`0x188524`, r2=`[ret(0x20184)]`, r3=`0x1884fc`.
  Call (`bl` at 0xfeaf0): (unknown callee), target 0x200b8, r0=`[sp-40]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `[sp-48]` (`ldm` at 0xfeb10).
