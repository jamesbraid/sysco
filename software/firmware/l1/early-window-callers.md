# L1 1.8.0 to 1.16.0 pointer-relative setup callers

These entries identify the writes, branches and call candidates in each named package and ROM location.
The hardware roles of the loaded bases and helpers remain unknown. The later reference image is
described in [USB controller accesses](usb-controller-registers.md). No effect is imported from that image.

A bracketed address denotes a loaded value. `sp` is the entry stack pointer. The call tables
use assumed M68K stack argument locations with unknown arity. Bounds end at the next LINK prologue
and are inferred. The first four entries have unresolved indirect transfers: their tables cover
known edges only. Values not resolved by the analysis remain unknown.

## 6.5.14:l1#0 at 0xffe5ce90

- `0xffe5ce90`, `6.5.14/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.14:l1#0`, version `1.8.0`. Payload file offset 0x2c.
  Load 0xffe00000. Analysed span 0xffe5ce90 to 0xffe5cfc4.
  Established. Known edges only. The unsigned `bhi.w` bypasses the indirect dispatch when an unknown
  value exceeds 0x2f (`bhi.w` at 0xffe5cec0); its fall-through route reaches an unresolved `jmp`, so
  effects reached only through that transfer remain unknown (`jmp` at 0xffe5ceca). The later call to
  0xffe69a3c has candidates 1, 0x60000000, 3 and `sp-8` (`jsr` at 0xffe5cfb2).

Pointer-relative store sites reached by known edges:

| Instruction | Width in bytes | Effective target | Stored value |
|---|---:|---|---|
| `move.b` at 0xffe5cf82 | 1 | `[0x3000074c] +0x16` | `0x8c` |
| `move.w` at 0xffe5cf8c | 2 | `[0x3000074c] +0xa0` | `0x6000` |
| `clr.l` at 0xffe5cf92 | 4 | `[0x3000074c] +0xa4` | `0x0` |
| `move.w` at 0xffe5cf9c | 2 | `[0x3000074c] +0xaa` | `0x953` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `jsr` at 0xffe5ce96 | `0xffe58430` | `d7` | `?` | `?` | `a6` |
| `bsr.w` at 0xffe5cea2 | `0xffe5c9d8` | `?` | `?` | `?` | `?` |
| `jsr` at 0xffe5cfb2 | `0xffe69a3c` | `0x1` | `0x60000000` | `0x3` | `sp-8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhi.w` at 0xffe5cec0 | `? > 0x2f (unsigned, 32-bit)` | `? <= 0x2f (unsigned, 32-bit)` |
| `beq.b` at 0xffe5cf6e | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffe5cf74 | `Z == 1` | `not (Z == 1)` |

[Full fact sheet and listing](disasm/mcf5206e-1.8.0-package-0/early-window-callers-facts.txt).

## 6.5.14:l1#0 at 0xffecce90

- `0xffecce90`, `6.5.14/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.14:l1#0`, version `1.8.0`. Payload file offset 0x2c.
  Load 0xffe00000. Analysed span 0xffecce90 to 0xffeccfc4.
  Established. Known edges only. The two later `beq.b` decisions take the store region for `Z == 1`,
  with selector operands unresolved (`beq.b` at 0xffeccf6e, `beq.b` at 0xffeccf74). That region
  includes byte 0x8c through `[0x3000074c] +0x16` and four-byte zero through `[0x3000074c] +0xa4`
  (`move.b` at 0xffeccf82, `clr.l` at 0xffeccf92). The earlier indirect jump prevents a complete
  effect list (`jmp` at 0xffecceca).

Pointer-relative store sites reached by known edges:

| Instruction | Width in bytes | Effective target | Stored value |
|---|---:|---|---|
| `move.b` at 0xffeccf82 | 1 | `[0x3000074c] +0x16` | `0x8c` |
| `move.w` at 0xffeccf8c | 2 | `[0x3000074c] +0xa0` | `0x6000` |
| `clr.l` at 0xffeccf92 | 4 | `[0x3000074c] +0xa4` | `0x0` |
| `move.w` at 0xffeccf9c | 2 | `[0x3000074c] +0xaa` | `0x953` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `jsr` at 0xffecce96 | `0xffec8430` | `d7` | `?` | `?` | `a6` |
| `bsr.w` at 0xffeccea2 | `0xffecc9d8` | `?` | `?` | `?` | `?` |
| `jsr` at 0xffeccfb2 | `0xffed9a3c` | `0x1` | `0x60000000` | `0x3` | `sp-8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhi.w` at 0xffeccec0 | `? > 0x2f (unsigned, 32-bit)` | `? <= 0x2f (unsigned, 32-bit)` |
| `beq.b` at 0xffeccf6e | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffeccf74 | `Z == 1` | `not (Z == 1)` |

[Full fact sheet and listing](disasm/mcf5206e-1.8.0-package-0/early-window-callers-facts.txt).

## 6.5.15:l1#0 at 0xffe5f9a4

- `0xffe5f9a4`, `6.5.15/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.15:l1#0`, version `1.10.0`. Payload file offset 0x2c.
  Load 0xffe00000. Analysed span 0xffe5f9a4 to 0xffe5fad8.
  Established. Known edges only. Three byte writes create 0, 5 and 0 at `sp -0x8`, `sp -0x7` and `sp
  -0x6` (`clr.b` at 0xffe5f9bc, `move.b` at 0xffe5f9c2, `clr.b` at 0xffe5f9c6). The conditional
  later block writes halfword 0x6000 through `[0x300007d8] +0xa0` before the call carrying
  0x60000000 (`move.w` at 0xffe5faa0, `jsr` at 0xffe5fac6). The indirect dispatch destination
  remains unknown (`jmp` at 0xffe5f9de).

Pointer-relative store sites reached by known edges:

| Instruction | Width in bytes | Effective target | Stored value |
|---|---:|---|---|
| `move.b` at 0xffe5fa96 | 1 | `[0x300007d8] +0x16` | `0x8c` |
| `move.w` at 0xffe5faa0 | 2 | `[0x300007d8] +0xa0` | `0x6000` |
| `clr.l` at 0xffe5faa6 | 4 | `[0x300007d8] +0xa4` | `0x0` |
| `move.w` at 0xffe5fab0 | 2 | `[0x300007d8] +0xaa` | `0x953` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `jsr` at 0xffe5f9aa | `0xffe5e960` | `d7` | `?` | `?` | `a6` |
| `bsr.w` at 0xffe5f9b6 | `0xffe5f4ec` | `?` | `?` | `?` | `?` |
| `jsr` at 0xffe5fac6 | `0xffe6ddcc` | `0x1` | `0x60000000` | `0x3` | `sp-8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhi.w` at 0xffe5f9d4 | `? > 0x2f (unsigned, 32-bit)` | `? <= 0x2f (unsigned, 32-bit)` |
| `beq.b` at 0xffe5fa82 | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffe5fa88 | `Z == 1` | `not (Z == 1)` |

[Full fact sheet and listing](disasm/mcf5206e-1.10.0-package-0/early-window-callers-facts.txt).

## 6.5.15:l1#0 at 0xffecf9a4

- `0xffecf9a4`, `6.5.15/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.15:l1#0`, version `1.10.0`. Payload file offset 0x2c.
  Load 0xffe00000. Analysed span 0xffecf9a4 to 0xffecfad8.
  Established. Known edges only; its first two call targets are 0xffece960 and 0xffecf4ec, with the
  latter's candidate values unresolved (`jsr` at 0xffecf9aa, `bsr.w` at 0xffecf9b6). Four separate loads
  read the four-byte value at 0x300007d8 (`movea.l` at 0xffecfa92, `movea.l`
  at 0xffecfa9a, `movea.l` at 0xffecfaa4, `movea.l` at 0xffecfaaa). A halfword 0x953 is stored
  through that loaded value +0xaa (`move.w` at 0xffecfab0). The unresolved jump leaves the other
  dispatch effects unestablished (`jmp` at 0xffecf9de).

Pointer-relative store sites reached by known edges:

| Instruction | Width in bytes | Effective target | Stored value |
|---|---:|---|---|
| `move.b` at 0xffecfa96 | 1 | `[0x300007d8] +0x16` | `0x8c` |
| `move.w` at 0xffecfaa0 | 2 | `[0x300007d8] +0xa0` | `0x6000` |
| `clr.l` at 0xffecfaa6 | 4 | `[0x300007d8] +0xa4` | `0x0` |
| `move.w` at 0xffecfab0 | 2 | `[0x300007d8] +0xaa` | `0x953` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `jsr` at 0xffecf9aa | `0xffece960` | `d7` | `?` | `?` | `a6` |
| `bsr.w` at 0xffecf9b6 | `0xffecf4ec` | `?` | `?` | `?` | `?` |
| `jsr` at 0xffecfac6 | `0xffedddcc` | `0x1` | `0x60000000` | `0x3` | `sp-8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhi.w` at 0xffecf9d4 | `? > 0x2f (unsigned, 32-bit)` | `? <= 0x2f (unsigned, 32-bit)` |
| `beq.b` at 0xffecfa82 | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffecfa88 | `Z == 1` | `not (Z == 1)` |

[Full fact sheet and listing](disasm/mcf5206e-1.10.0-package-0/early-window-callers-facts.txt).

## 6.5.16:l1#0 at 0xffe5e018

- `0xffe5e018`, `6.5.16/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.16:l1#0`, version `1.12.3`. Payload file offset 0x40.
  Load 0xffe00000. Analysed span 0xffe5e018 to 0xffe5e0d4.
  Established. The four-byte value at 0x30002ad4 controls `ble.b`: signed values at most zero take
  the branch past the first store-and-call region (`tst.l` at 0xffe5e040, `ble.b` at 0xffe5e044).
  Positive values fall through to a halfword 0x4000 store through `[0x30000630] +0x88` and a call to
  0xffe6c4bc (`ble.b` at 0xffe5e044, `move.w` at 0xffe5e052, `jsr` at 0xffe5e076). The later branch
  retains only `Z == 0` as its skip condition (`bne.b` at 0xffe5e086).

Pointer-relative store sites reached by known edges:

| Instruction | Width in bytes | Effective target | Stored value |
|---|---:|---|---|
| `move.w` at 0xffe5e052 | 2 | `[0x30000630] +0x88` | `0x4000` |
| `clr.l` at 0xffe5e058 | 4 | `[0x30000630] +0x8c` | `0x0` |
| `move.w` at 0xffe5e062 | 2 | `[0x30000630] +0x92` | `0x953` |
| `move.b` at 0xffe5e092 | 1 | `[0x30000630] +0x16` | `0x8c` |
| `move.w` at 0xffe5e09c | 2 | `[0x30000630] +0xa0` | `0x6000` |
| `clr.l` at 0xffe5e0a2 | 4 | `[0x30000630] +0xa4` | `0x0` |
| `move.w` at 0xffe5e0ac | 2 | `[0x30000630] +0xaa` | `0x953` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `jsr` at 0xffe5e01e | `0xffe46d28` | `a5` | `?` | `?` | `a6` |
| `bsr.w` at 0xffe5e026 | `0xffe5db58` | `ret(0xffe46d28)` | `?` | `?` | `?` |
| `jsr` at 0xffe5e076 | `0xffe6c4bc` | `0x0` | `0x40000000` | `0x2` | `sp-8` |
| `jsr` at 0xffe5e0c2 | `0xffe6c4bc` | `0x1` | `0x60000000` | `0x3` | `sp-8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble.b` at 0xffe5e044 | `[0x30002ad4] <= 0x0 (signed, 32-bit)` | `[0x30002ad4] > 0x0 (signed, 32-bit)` |
| `bne.b` at 0xffe5e086 | `Z == 0` | `not (Z == 0)` |

[Full fact sheet and listing](disasm/mcf5206e-1.12.3-package-0/early-window-callers-facts.txt).

## 6.5.16:l1#0 at 0xffece018

- `0xffece018`, `6.5.16/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.16:l1#0`, version `1.12.3`. Payload file offset 0x40.
  Load 0xffe00000. Analysed span 0xffece018 to 0xffece0d4.
  Established. Two call sites target 0xffedc4bc, one carrying 0, 0x40000000, 2 and `sp-8`, the other
  1, 0x60000000, 3 and `sp-8` (`jsr` at 0xffece076, `jsr` at 0xffece0c2). Before the second call, a
  byte 0x8c goes through `[0x30000630] +0x16` and a halfword 0x6000 through +0xa0 (`move.b` at
  0xffece092, `move.w` at 0xffece09c). Their device meaning is unknown.

Pointer-relative store sites reached by known edges:

| Instruction | Width in bytes | Effective target | Stored value |
|---|---:|---|---|
| `move.w` at 0xffece052 | 2 | `[0x30000630] +0x88` | `0x4000` |
| `clr.l` at 0xffece058 | 4 | `[0x30000630] +0x8c` | `0x0` |
| `move.w` at 0xffece062 | 2 | `[0x30000630] +0x92` | `0x953` |
| `move.b` at 0xffece092 | 1 | `[0x30000630] +0x16` | `0x8c` |
| `move.w` at 0xffece09c | 2 | `[0x30000630] +0xa0` | `0x6000` |
| `clr.l` at 0xffece0a2 | 4 | `[0x30000630] +0xa4` | `0x0` |
| `move.w` at 0xffece0ac | 2 | `[0x30000630] +0xaa` | `0x953` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `jsr` at 0xffece01e | `0xffeb6d28` | `a5` | `?` | `?` | `a6` |
| `bsr.w` at 0xffece026 | `0xffecdb58` | `ret(0xffeb6d28)` | `?` | `?` | `?` |
| `jsr` at 0xffece076 | `0xffedc4bc` | `0x0` | `0x40000000` | `0x2` | `sp-8` |
| `jsr` at 0xffece0c2 | `0xffedc4bc` | `0x1` | `0x60000000` | `0x3` | `sp-8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble.b` at 0xffece044 | `[0x30002ad4] <= 0x0 (signed, 32-bit)` | `[0x30002ad4] > 0x0 (signed, 32-bit)` |
| `bne.b` at 0xffece086 | `Z == 0` | `not (Z == 0)` |

[Full fact sheet and listing](disasm/mcf5206e-1.12.3-package-0/early-window-callers-facts.txt).

## 6.5.16:l1#1 at 0xffe6614c

- `0xffe6614c`, `6.5.16/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.16:l1#1`, version `1.12.3`. Payload file offset 0xedf50.
  Load 0xffe00000. Analysed span 0xffe6614c to 0xffe66208.
  Established. The earlier call to 0xffe4aaf8 supplies the first candidate of the subsequent
  0xffe65c8c call as `ret(0xffe4aaf8)` (`jsr` at 0xffe66152, `bsr.w` at 0xffe6615a). The later
  four-byte read at 0x30002b58 feeds a signed `ble.b` that skips the first setup region for values
  at most zero (`tst.l` at 0xffe66174, `ble.b` at 0xffe66178). Separate four-byte clear sites use
  `[0x300006a0] +0x8c` and +0xa4 (`clr.l` at 0xffe6618c, `clr.l` at 0xffe661d6).

Pointer-relative store sites reached by known edges:

| Instruction | Width in bytes | Effective target | Stored value |
|---|---:|---|---|
| `move.w` at 0xffe66186 | 2 | `[0x300006a0] +0x88` | `0x4000` |
| `clr.l` at 0xffe6618c | 4 | `[0x300006a0] +0x8c` | `0x0` |
| `move.w` at 0xffe66196 | 2 | `[0x300006a0] +0x92` | `0x953` |
| `move.b` at 0xffe661c6 | 1 | `[0x300006a0] +0x16` | `0x8c` |
| `move.w` at 0xffe661d0 | 2 | `[0x300006a0] +0xa0` | `0x6000` |
| `clr.l` at 0xffe661d6 | 4 | `[0x300006a0] +0xa4` | `0x0` |
| `move.w` at 0xffe661e0 | 2 | `[0x300006a0] +0xaa` | `0x953` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `jsr` at 0xffe66152 | `0xffe4aaf8` | `a5` | `?` | `?` | `a6` |
| `bsr.w` at 0xffe6615a | `0xffe65c8c` | `ret(0xffe4aaf8)` | `?` | `?` | `?` |
| `jsr` at 0xffe661aa | `0xffe75844` | `0x0` | `0x40000000` | `0x2` | `sp-8` |
| `jsr` at 0xffe661f6 | `0xffe75844` | `0x1` | `0x60000000` | `0x3` | `sp-8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble.b` at 0xffe66178 | `[0x30002b58] <= 0x0 (signed, 32-bit)` | `[0x30002b58] > 0x0 (signed, 32-bit)` |
| `bne.b` at 0xffe661ba | `Z == 0` | `not (Z == 0)` |

[Full fact sheet and listing](disasm/mcf5206e-1.12.3-package-1/early-window-callers-facts.txt).

## 6.5.16:l1#1 at 0xfff5614c

- `0xfff5614c`, `6.5.16/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.16:l1#1`, version `1.12.3`. Payload file offset 0xedf50.
  Load 0xffe00000. Analysed span 0xfff5614c to 0xfff56208.
  Established. Halfword 0x953 is written at two sites through `[0x300006a0] +0x92` and +0xaa
  (`move.w` at 0xfff56196, `move.w` at 0xfff561e0). The first region's helper call has first
  candidate zero; the later one has first candidate 1, both targeting 0xfff65844 (`jsr` at
  0xfff561aa, `jsr` at 0xfff561f6). The final load restores A5 from entry stack -0x10 before frame
  removal and return (`movea.l` at 0xfff56200, `unlk` at 0xfff56204, `rts` at 0xfff56206).

Pointer-relative store sites reached by known edges:

| Instruction | Width in bytes | Effective target | Stored value |
|---|---:|---|---|
| `move.w` at 0xfff56186 | 2 | `[0x300006a0] +0x88` | `0x4000` |
| `clr.l` at 0xfff5618c | 4 | `[0x300006a0] +0x8c` | `0x0` |
| `move.w` at 0xfff56196 | 2 | `[0x300006a0] +0x92` | `0x953` |
| `move.b` at 0xfff561c6 | 1 | `[0x300006a0] +0x16` | `0x8c` |
| `move.w` at 0xfff561d0 | 2 | `[0x300006a0] +0xa0` | `0x6000` |
| `clr.l` at 0xfff561d6 | 4 | `[0x300006a0] +0xa4` | `0x0` |
| `move.w` at 0xfff561e0 | 2 | `[0x300006a0] +0xaa` | `0x953` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `jsr` at 0xfff56152 | `0xfff3aaf8` | `a5` | `?` | `?` | `a6` |
| `bsr.w` at 0xfff5615a | `0xfff55c8c` | `ret(0xfff3aaf8)` | `?` | `?` | `?` |
| `jsr` at 0xfff561aa | `0xfff65844` | `0x0` | `0x40000000` | `0x2` | `sp-8` |
| `jsr` at 0xfff561f6 | `0xfff65844` | `0x1` | `0x60000000` | `0x3` | `sp-8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble.b` at 0xfff56178 | `[0x30002b58] <= 0x0 (signed, 32-bit)` | `[0x30002b58] > 0x0 (signed, 32-bit)` |
| `bne.b` at 0xfff561ba | `Z == 0` | `not (Z == 0)` |

[Full fact sheet and listing](disasm/mcf5206e-1.12.3-package-1/early-window-callers-facts.txt).

## 6.5.17:l1#0 at 0xffe58d18

- `0xffe58d18`, `6.5.17/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.17:l1#0`, version `1.14.1`. Payload file offset 0x40.
  Load 0xffe00000. Analysed span 0xffe58d18 to 0xffe58de4.
  Established. A byte 5 is stored at entry stack -0x7 between two byte clears at -0x8 and -0x6
  (`move.b` at 0xffe58d32, `clr.b` at 0xffe58d2c, `clr.b` at 0xffe58d36). The later setup calls both
  target 0xffe6de74, with `sp-8` in the fourth candidate location (`jsr` at 0xffe58d76, `jsr` at
  0xffe58dc2). The second region contains a four-byte zero store through `[0x300002d4] +0xa4`
  (`clr.l` at 0xffe58da2).

Pointer-relative store sites reached by known edges:

| Instruction | Width in bytes | Effective target | Stored value |
|---|---:|---|---|
| `move.w` at 0xffe58d52 | 2 | `[0x300002d4] +0x88` | `0x4000` |
| `clr.l` at 0xffe58d58 | 4 | `[0x300002d4] +0x8c` | `0x0` |
| `move.w` at 0xffe58d62 | 2 | `[0x300002d4] +0x92` | `0x953` |
| `move.b` at 0xffe58d92 | 1 | `[0x300002d4] +0x16` | `0x8c` |
| `move.w` at 0xffe58d9c | 2 | `[0x300002d4] +0xa0` | `0x6000` |
| `clr.l` at 0xffe58da2 | 4 | `[0x300002d4] +0xa4` | `0x0` |
| `move.w` at 0xffe58dac | 2 | `[0x300002d4] +0xaa` | `0x953` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `jsr` at 0xffe58d1e | `0xffe4eb08` | `a5` | `?` | `?` | `a6` |
| `bsr.w` at 0xffe58d26 | `0xffe58840` | `ret(0xffe4eb08)` | `?` | `?` | `?` |
| `jsr` at 0xffe58d76 | `0xffe6de74` | `0x0` | `0x40000000` | `0x2` | `sp-8` |
| `jsr` at 0xffe58dc2 | `0xffe6de74` | `0x1` | `0x60000000` | `0x3` | `sp-8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble.b` at 0xffe58d44 | `[0x30002db8] <= 0x0 (signed, 32-bit)` | `[0x30002db8] > 0x0 (signed, 32-bit)` |
| `bne.b` at 0xffe58d86 | `Z == 0` | `not (Z == 0)` |

[Full fact sheet and listing](disasm/mcf5206e-1.14.1-package-0/early-window-callers-facts.txt).

## 6.5.17:l1#0 at 0xffec8d18

- `0xffec8d18`, `6.5.17/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.17:l1#0`, version `1.14.1`. Payload file offset 0x40.
  Load 0xffe00000. Analysed span 0xffec8d18 to 0xffec8de4.
  Established. The first write region includes halfword 0x4000 at `[0x300002d4] +0x88`, four-byte
  zero at +0x8c and halfword 0x953 at +0x92 (`move.w` at 0xffec8d52, `clr.l` at 0xffec8d58, `move.w`
  at 0xffec8d62); its signed guard takes `ble.b` past that region for `[0x30002db8] <= 0` (`ble.b`
  at 0xffec8d44). The later `bne.b` skips the second region for `Z == 0`, with compared operands
  unknown (`bne.b` at 0xffec8d86).

Pointer-relative store sites reached by known edges:

| Instruction | Width in bytes | Effective target | Stored value |
|---|---:|---|---|
| `move.w` at 0xffec8d52 | 2 | `[0x300002d4] +0x88` | `0x4000` |
| `clr.l` at 0xffec8d58 | 4 | `[0x300002d4] +0x8c` | `0x0` |
| `move.w` at 0xffec8d62 | 2 | `[0x300002d4] +0x92` | `0x953` |
| `move.b` at 0xffec8d92 | 1 | `[0x300002d4] +0x16` | `0x8c` |
| `move.w` at 0xffec8d9c | 2 | `[0x300002d4] +0xa0` | `0x6000` |
| `clr.l` at 0xffec8da2 | 4 | `[0x300002d4] +0xa4` | `0x0` |
| `move.w` at 0xffec8dac | 2 | `[0x300002d4] +0xaa` | `0x953` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `jsr` at 0xffec8d1e | `0xffebeb08` | `a5` | `?` | `?` | `a6` |
| `bsr.w` at 0xffec8d26 | `0xffec8840` | `ret(0xffebeb08)` | `?` | `?` | `?` |
| `jsr` at 0xffec8d76 | `0xffedde74` | `0x0` | `0x40000000` | `0x2` | `sp-8` |
| `jsr` at 0xffec8dc2 | `0xffedde74` | `0x1` | `0x60000000` | `0x3` | `sp-8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble.b` at 0xffec8d44 | `[0x30002db8] <= 0x0 (signed, 32-bit)` | `[0x30002db8] > 0x0 (signed, 32-bit)` |
| `bne.b` at 0xffec8d86 | `Z == 0` | `not (Z == 0)` |

[Full fact sheet and listing](disasm/mcf5206e-1.14.1-package-0/early-window-callers-facts.txt).

## 6.5.17:l1#1 at 0xffe658b4

- `0xffe658b4`, `6.5.17/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.17:l1#1`, version `1.14.1`. Payload file offset 0xee8b4.
  Load 0xffe00000. Analysed span 0xffe658b4 to 0xffe65980.
  Established. Two separate loads read 0x30002ecc as four bytes before the two branch sites (`tst.l`
  at 0xffe658dc, `move.l` at 0xffe6591c). The first decision skips the initial store block when that
  signed value is at most zero (`ble.b` at 0xffe658e0). The second `bne.b` takes its skip for `Z == 0`, with compared operands
  unresolved (`bne.b` at 0xffe65922). The two setup calls have second candidates 0x40000000 and
  0x60000000 (`jsr` at 0xffe65912, `jsr` at 0xffe6595e).

Pointer-relative store sites reached by known edges:

| Instruction | Width in bytes | Effective target | Stored value |
|---|---:|---|---|
| `move.w` at 0xffe658ee | 2 | `[0x30000370] +0x88` | `0x4000` |
| `clr.l` at 0xffe658f4 | 4 | `[0x30000370] +0x8c` | `0x0` |
| `move.w` at 0xffe658fe | 2 | `[0x30000370] +0x92` | `0x953` |
| `move.b` at 0xffe6592e | 1 | `[0x30000370] +0x16` | `0x8c` |
| `move.w` at 0xffe65938 | 2 | `[0x30000370] +0xa0` | `0x6000` |
| `clr.l` at 0xffe6593e | 4 | `[0x30000370] +0xa4` | `0x0` |
| `move.w` at 0xffe65948 | 2 | `[0x30000370] +0xaa` | `0x953` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `jsr` at 0xffe658ba | `0xffe55cec` | `a5` | `?` | `?` | `a6` |
| `bsr.w` at 0xffe658c2 | `0xffe653dc` | `ret(0xffe55cec)` | `?` | `?` | `?` |
| `jsr` at 0xffe65912 | `0xffe7f858` | `0x0` | `0x40000000` | `0x2` | `sp-8` |
| `jsr` at 0xffe6595e | `0xffe7f858` | `0x1` | `0x60000000` | `0x3` | `sp-8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble.b` at 0xffe658e0 | `[0x30002ecc] <= 0x0 (signed, 32-bit)` | `[0x30002ecc] > 0x0 (signed, 32-bit)` |
| `bne.b` at 0xffe65922 | `Z == 0` | `not (Z == 0)` |

[Full fact sheet and listing](disasm/mcf5206e-1.14.1-package-1/early-window-callers-facts.txt).

## 6.5.17:l1#1 at 0xfff558b4

- `0xfff558b4`, `6.5.17/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.17:l1#1`, version `1.14.1`. Payload file offset 0xee8b4.
  Load 0xffe00000. Analysed span 0xfff558b4 to 0xfff55980.
  Established. In the later write region, byte 0x8c and halfword 0x6000 go through `[0x30000370]
  +0x16` and +0xa0 (`move.b` at 0xfff5592e, `move.w` at 0xfff55938). That path calls 0xfff6f858 with
  candidates 1, 0x60000000, 3 and `sp-8` (`jsr` at 0xfff5595e). The prior `bne.b` selects the
  return region when `Z == 0`; otherwise execution falls through into these writes (`bne.b` at
  0xfff55922).

Pointer-relative store sites reached by known edges:

| Instruction | Width in bytes | Effective target | Stored value |
|---|---:|---|---|
| `move.w` at 0xfff558ee | 2 | `[0x30000370] +0x88` | `0x4000` |
| `clr.l` at 0xfff558f4 | 4 | `[0x30000370] +0x8c` | `0x0` |
| `move.w` at 0xfff558fe | 2 | `[0x30000370] +0x92` | `0x953` |
| `move.b` at 0xfff5592e | 1 | `[0x30000370] +0x16` | `0x8c` |
| `move.w` at 0xfff55938 | 2 | `[0x30000370] +0xa0` | `0x6000` |
| `clr.l` at 0xfff5593e | 4 | `[0x30000370] +0xa4` | `0x0` |
| `move.w` at 0xfff55948 | 2 | `[0x30000370] +0xaa` | `0x953` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `jsr` at 0xfff558ba | `0xfff45cec` | `a5` | `?` | `?` | `a6` |
| `bsr.w` at 0xfff558c2 | `0xfff553dc` | `ret(0xfff45cec)` | `?` | `?` | `?` |
| `jsr` at 0xfff55912 | `0xfff6f858` | `0x0` | `0x40000000` | `0x2` | `sp-8` |
| `jsr` at 0xfff5595e | `0xfff6f858` | `0x1` | `0x60000000` | `0x3` | `sp-8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble.b` at 0xfff558e0 | `[0x30002ecc] <= 0x0 (signed, 32-bit)` | `[0x30002ecc] > 0x0 (signed, 32-bit)` |
| `bne.b` at 0xfff55922 | `Z == 0` | `not (Z == 0)` |

[Full fact sheet and listing](disasm/mcf5206e-1.14.1-package-1/early-window-callers-facts.txt).

## 6.5.18:l1#0 at 0xffe539bc

- `0xffe539bc`, `6.5.18/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.18:l1#0`, version `1.16.0`. Payload file offset 0x40.
  Load 0xffe00000. Analysed span 0xffe539bc to 0xffe53a88.
  Established. The signed test at 0x30002e14 can bypass the region using 0x40000000: `ble.b` takes
  that skip for values at most zero (`tst.l` at 0xffe539e4, `ble.b` at 0xffe539e8, `jsr` at
  0xffe53a1a). The later call to 0xffe6cd58 instead carries 1, 0x60000000, 3 and `sp-8` (`jsr` at
  0xffe53a66); its write region includes halfword 0x953 through `[0x3000029c] +0xaa` (`move.w` at
  0xffe53a50).

Pointer-relative store sites reached by known edges:

| Instruction | Width in bytes | Effective target | Stored value |
|---|---:|---|---|
| `move.w` at 0xffe539f6 | 2 | `[0x3000029c] +0x88` | `0x4000` |
| `clr.l` at 0xffe539fc | 4 | `[0x3000029c] +0x8c` | `0x0` |
| `move.w` at 0xffe53a06 | 2 | `[0x3000029c] +0x92` | `0x953` |
| `move.b` at 0xffe53a36 | 1 | `[0x3000029c] +0x16` | `0x8c` |
| `move.w` at 0xffe53a40 | 2 | `[0x3000029c] +0xa0` | `0x6000` |
| `clr.l` at 0xffe53a46 | 4 | `[0x3000029c] +0xa4` | `0x0` |
| `move.w` at 0xffe53a50 | 2 | `[0x3000029c] +0xaa` | `0x953` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `jsr` at 0xffe539c2 | `0xffe4c1d8` | `a5` | `?` | `?` | `a6` |
| `bsr.w` at 0xffe539ca | `0xffe533c8` | `ret(0xffe4c1d8)` | `?` | `?` | `?` |
| `jsr` at 0xffe53a1a | `0xffe6cd58` | `0x0` | `0x40000000` | `0x2` | `sp-8` |
| `jsr` at 0xffe53a66 | `0xffe6cd58` | `0x1` | `0x60000000` | `0x3` | `sp-8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble.b` at 0xffe539e8 | `[0x30002e14] <= 0x0 (signed, 32-bit)` | `[0x30002e14] > 0x0 (signed, 32-bit)` |
| `bne.b` at 0xffe53a2a | `Z == 0` | `not (Z == 0)` |

[Full fact sheet and listing](disasm/mcf5206e-1.16.0-package-0/early-window-callers-facts.txt).

## 6.5.18:l1#0 at 0xffec39bc

- `0xffec39bc`, `6.5.18/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.18:l1#0`, version `1.16.0`. Payload file offset 0x40.
  Load 0xffe00000. Analysed span 0xffec39bc to 0xffec3a88.
  Established. The two address-bearing call sites use target 0xffedcd58 with distinct first three
  candidate values, but both use `sp-8` as the fourth (`jsr` at 0xffec3a1a, `jsr` at 0xffec3a66).
  Their respective regions include four-byte clears through `[0x3000029c] +0x8c` and +0xa4 (`clr.l`
  at 0xffec39fc, `clr.l` at 0xffec3a46). The merged D0 return value remains unknown (`rts` at
  0xffec3a76).

Pointer-relative store sites reached by known edges:

| Instruction | Width in bytes | Effective target | Stored value |
|---|---:|---|---|
| `move.w` at 0xffec39f6 | 2 | `[0x3000029c] +0x88` | `0x4000` |
| `clr.l` at 0xffec39fc | 4 | `[0x3000029c] +0x8c` | `0x0` |
| `move.w` at 0xffec3a06 | 2 | `[0x3000029c] +0x92` | `0x953` |
| `move.b` at 0xffec3a36 | 1 | `[0x3000029c] +0x16` | `0x8c` |
| `move.w` at 0xffec3a40 | 2 | `[0x3000029c] +0xa0` | `0x6000` |
| `clr.l` at 0xffec3a46 | 4 | `[0x3000029c] +0xa4` | `0x0` |
| `move.w` at 0xffec3a50 | 2 | `[0x3000029c] +0xaa` | `0x953` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `jsr` at 0xffec39c2 | `0xffebc1d8` | `a5` | `?` | `?` | `a6` |
| `bsr.w` at 0xffec39ca | `0xffec33c8` | `ret(0xffebc1d8)` | `?` | `?` | `?` |
| `jsr` at 0xffec3a1a | `0xffedcd58` | `0x0` | `0x40000000` | `0x2` | `sp-8` |
| `jsr` at 0xffec3a66 | `0xffedcd58` | `0x1` | `0x60000000` | `0x3` | `sp-8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble.b` at 0xffec39e8 | `[0x30002e14] <= 0x0 (signed, 32-bit)` | `[0x30002e14] > 0x0 (signed, 32-bit)` |
| `bne.b` at 0xffec3a2a | `Z == 0` | `not (Z == 0)` |

[Full fact sheet and listing](disasm/mcf5206e-1.16.0-package-0/early-window-callers-facts.txt).

## 6.5.18:l1#1 at 0xffe62790

- `0xffe62790`, `6.5.18/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.18:l1#1`, version `1.16.0`. Payload file offset 0xecca0.
  Load 0xffe00000. Analysed span 0xffe62790 to 0xffe6285c.
  Established. The call to 0xffe6219c follows the call to 0xffe53b90 and receives
  `ret(0xffe53b90)` as its first candidate (`jsr` at 0xffe62796, `bsr.w` at 0xffe6279e). A signed
  four-byte test at 0x30002f34 skips the first store-and-call block for values at most zero (`tst.l`
  at 0xffe627b8, `ble.b` at 0xffe627bc). Both later setup calls target 0xffe811fc, with 0x40000000
  and 0x60000000 as their second candidates (`jsr` at 0xffe627ee, `jsr` at 0xffe6283a).

Pointer-relative store sites reached by known edges:

| Instruction | Width in bytes | Effective target | Stored value |
|---|---:|---|---|
| `move.w` at 0xffe627ca | 2 | `[0x30000340] +0x88` | `0x4000` |
| `clr.l` at 0xffe627d0 | 4 | `[0x30000340] +0x8c` | `0x0` |
| `move.w` at 0xffe627da | 2 | `[0x30000340] +0x92` | `0x953` |
| `move.b` at 0xffe6280a | 1 | `[0x30000340] +0x16` | `0x8c` |
| `move.w` at 0xffe62814 | 2 | `[0x30000340] +0xa0` | `0x6000` |
| `clr.l` at 0xffe6281a | 4 | `[0x30000340] +0xa4` | `0x0` |
| `move.w` at 0xffe62824 | 2 | `[0x30000340] +0xaa` | `0x953` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `jsr` at 0xffe62796 | `0xffe53b90` | `a5` | `?` | `?` | `a6` |
| `bsr.w` at 0xffe6279e | `0xffe6219c` | `ret(0xffe53b90)` | `?` | `?` | `?` |
| `jsr` at 0xffe627ee | `0xffe811fc` | `0x0` | `0x40000000` | `0x2` | `sp-8` |
| `jsr` at 0xffe6283a | `0xffe811fc` | `0x1` | `0x60000000` | `0x3` | `sp-8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble.b` at 0xffe627bc | `[0x30002f34] <= 0x0 (signed, 32-bit)` | `[0x30002f34] > 0x0 (signed, 32-bit)` |
| `bne.b` at 0xffe627fe | `Z == 0` | `not (Z == 0)` |

[Full fact sheet and listing](disasm/mcf5206e-1.16.0-package-1/early-window-callers-facts.txt).
