# L1 1.44.0: the release scan per brick type (I-brick, R-brick, Cx-brick)

What the release scan expects behind the parallel-port TAP for each brick-type letter, and why a chain that satisfies the C-brick fails on the I-brick. [scan-interface.md](scan-interface.md) reads the C-brick case; this note reads the per-letter table that feeds the same scan routine. Read from L1 1.44.0 of the IRIX 6.5.30 `l1.bin` (sha256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`): package 0 (`Base 1MB`, type letters `CIPXYN&?RG`) and package 2 (`MIPS 2MB`, which adds `6`, the Cx-brick), first program copy, through a linear listing cross-checked against a Ghidra decompile (`68000:BE:32:Coldfire`, flat at `0xffe00000`). Grades are established, inferred, unknown ([../../../AGENTS.md](../../../AGENTS.md)).

## 1. The per-letter table (established)

The reset-brick routine (package 0 `0xffe72214`, package 2 `0xffe95270`) subtracts `0x25` from the type letter, rejects a result above `0x34`, and jumps through a table of 16-bit offsets (package 0 `0xffe722a8`, package 2 `0xffe95304`). Each listed letter loads two small constants and calls the scan, `0xffe74474` (package 2 `0xffe9e2b0`), with flags `0x18`, the first constant as the **SIC address** argument, the second as the **setting** argument and the rest zero. Every other letter takes the skip arm: the scan is not run and the release succeeds after the second pulse. The two scan routines are the same code (normalised listings are line-for-line identical, the same constants at the same offsets).

| Letter | Brick | SIC address (`a`) | Setting nibble (`s`) | Setting byte | Expected IR capture, 12 bits |
|---|---|---:|---:|---:|---|
| `C`, `A`, `G` | C-brick, `A`, `G` | `0x00` | `7` | `0x77` | `0x001` |
| `I` | I-brick | `0x0a` | `0xb` | `0xbb` | `0x0a1` |
| `%` | PE brick | `0x0a` | `9` | `0x99` | `0x0a1` |
| `P` | P-brick | `0x0a` | `8` | `0x88` | `0x0a1` |
| `R` | router | `0x00` | `1` | `0x11` | `0x001` |
| `X`, `Y` | X-brick, X2-brick | `0x00` | `1` | `0x11` | `0x001` |
| `N` | `N` | `0x00` | `0xf` | `0xff` | `0x001` |
| every other letter, including `&`, `4`, `6`, `7` | | no scan | | | |

Package 0 table entries: `%` `0xffe72324`, `A` `0xffe72318`, `C` `0xffe72312`, `G` `0xffe7233c`, `I` `0xffe7231e`, `N` `0xffe72342`, `P` `0xffe7232a`, `R` `0xffe72336`, `X`/`Y` `0xffe72330`; default (skip) `0xffe72348`. Package 2's table (`0xffe95304`) gives the same set with the same constants and `6` falls to its default (`0xffe953a4`). **Cx-brick (`6`): no scan, established** for both packages; its release cannot raise the scan error through this routine. The routine uses the hook at word `+0xc0` of the table at `0x30001e30` (package 2 `0x3000281c`) instead when it is nonzero; no store to that word was found in either package (inferred, the search was by store patterns).

The release wrapper gates the whole thing: package 0 `0xffe4f144` (package 2's `0xffe6f5c0` range) calls the routine only when bit 7 of the byte at `MBAR+0x41` (`SYPCR`) is set, and passes the brick-type byte from `0xffe55b64`. A zero return becomes error `-2301` (class SCAN, "unable to reset scan hardware"); the routine returns `-2300` if the word at `0x30000044` is zero.

## 2. What the scan shifts (established for the sequence, inferred for the meaning)

`a` is the SIC address and `s` the setting nibble from the table; `E(a) = 0x001 | (a << 4)`. Values are given LSB first (time order, TDI and TDO bit 0 first); the TMS strings and per-clock pin order are in [scan-interface.md](scan-interface.md) section 6. The scan `0xffe74474(0x18, a, s, ...)` does, in order:

| # | Step | Shifted | TDO checked | Grade |
|---:|---|---|---|---|
| 1 | TAP reset, 32 clocks | | no | established |
| 2 | IR, 12 bits, from Test-Logic-Reset | `0x004` (decompile: `*buf = 4`, bit-reversed, `0xffe745c6`) | masked, not compared | established |
| 3 | DR, 8 bits | `a` (`0xffe74672`) | masked, not compared | established |
| 4 | IR, 12 bits | `0xfff` (`0xffe748a0`) | **first 12 samples must equal `E(a)`** (`0xffe7494c`) | established |
| 5 | DR, 32 bits | `0xa800ffea`, MSB first as written | **32 samples must be `0x54007ff5`**, i.e. the word delayed one bit, first sample 0 (`0xffe74988`) | established |
| 6 | IR, 12 bits | `0x008` (`0xffe74af8`) | **first 12 samples must equal `E(a)` again** (`0xffe74b9e`) | established |
| 7 | DR, 8 bits | `(s << 4) \| s`, the setting byte of the table (`0xffe74bca`..`0xffe74bd4`) | masked, not compared | established |
| 8 | TAP reset, then second TAP reset with TRST asserted for the first, then end | | no | established |

Flags `0x18` leave the optional blocks off: no extra leading TAP reset (bit 0), no IR `0xe` / DR block (bit 1, argument at `0x18(a6)`), no IR `9` / DR block (bit 2, argument at `0x1c(a6)`), no capture compare on the setting scan (bit 9). Any failed compare writes a diagnostic into the error buffer (IR: "Unexpected data shifted out of IR: 0x%08lX.", at `0xffe74ba2`; bypass: "Unexpected data shifted out of BYPASS register.") and the scan returns 0.

Instruction meaning, from the controller guide's `scan sel <addr> <CER> [<MR> [<IOR>]]` ("Select SIC", 007-3938-004, printed pp. 86 to 88), is inferred: IR `4` with an 8-bit DR is "select SIC `a`", IR `8` with an 8-bit DR writes the chip-enable register `s s`, IR `9` and `0xe` the optional MR and IOR. The firmware never reads any of these back.

## 3. The chain this implies (inferred from the compares, with the compares established)

- **One TAP.** Step 5's pattern is the shifted word delayed by exactly one bit, so with the IR at all ones the chain holds one 1-bit bypass register. Two devices would delay it by two.
- **12-bit IR.** Step 4 and 6 sample 12 bits and then Update. An IR shorter than 12 would let TDI's ones appear in the sample positions the check expects to be zero (sample 11 of `E(a)` is 0 for the table's addresses); an IR longer than 12 would be loaded with 12 ones plus leftover capture bits and not be BYPASS, which step 5 needs. So the IR is 12 bits.
- **The IR capture carries the selected SIC address.** `E(a)` is `0001` in bits 3:0 and `a` in bits 11:4 (the decompile builds it as the bit-reverse of `a` shifted into bits 27:20 with bit 31 set; the listing is the same at `0xffe744fc`..`0xffe74584`). For `a = 0` it is the C-brick's "1 then eleven 0s". For `a = 0x0a` the 12 samples are `1,0,0,0, 0,1,0,1, 0,0,0,0`.
- **TDO per scan for the I-brick** (`a = 0x0a`, `s = 0xb`): step 4 and step 6 each return `1,0,0,0,0,1,0,1,0,0,0,0`; step 5 returns `0` then the 31 leading bits of `0xa800ffea` MSB first; steps 2, 3 and 7 may return anything.

Whether the real part loads the capture from the value shifted in step 3, or from a strap read at reset, is unknown: the firmware selects the same address it later expects, so both behave alike for this scan. A chain that satisfies the firmware without more knowledge: on Update-DR after IR `0x004` store the low 8 DR bits as the selected address; on Capture-IR load `0x001 | (sel << 4)`; every other IR value other than `0xfff` and the four above is unconstrained; IR `0xfff` selects a 1-bit bypass that starts at 0. Reset value of `sel` is not constrained by this scan (step 4 follows the selection); unknown.

## 4. Reading a model trace of the symptom

A model whose TAP captures only `0x001` (the C-brick chain) on an I-brick shifts a 12-bit IR (time-order word `0x200`, which is value `0x004`), an 8-bit DR (time-order byte `0x50`, which is value `0x0a`, `a` of the table), then a 12-bit IR of ones and captures `0x800` (1 then zeros) where `0x850` is expected. That is the step 4 compare at `0xffe7494c`: it fails, the scan returns 0 and the release reports the SCAN error before the 32-bit scan runs. The two raw forms are the left-aligned MSB-first words the driver passes, decoded once above against the table. The trace is model behaviour, not hardware evidence.

## 5. Per personality

- **I-brick (`I`), P-brick (`P`), PE brick (`%`):** the chain of section 3 with `a = 0x0a`; only the setting byte differs (`0xbb`, `0x88`, `0x99`, never compared).
- **R-brick (`R`), X-brick (`X`), X2-brick (`Y`):** `a = 0`; the C-brick chain passes unchanged. Setting byte `0x11`, uncompared. `N` likewise with `0xff`; `G` and `A` as the C-brick.
- **Cx-brick (`6`):** no scan; nothing to model for the scan error.

Unknown: which real device is the TAP (IDCODE, vendor), whether the capture follows the select or a strap, and what the select-register reset value is; none is read by the release path.

## Review

Independent review, 2026-11-21, by a reviewer who did not write it (sysco `f663579`). Method: package 0 of the 6.5.30 `l1.bin` (body sha256 `3c8a7aa9...`) in Ghidra, flat at `0xffe00000`. Confirmed: the letter table of `0xffe72214` (the address and setting pairs for `%`, `A`, `C`, `G`, `I`, `N`, `P`, `R`, `X`, `Y`, the skip arm for every other letter including `6`), the -2300 and -2301 results, the hook word `0x30001ef0`; the scan `0xffe74474` step by step (IR 4, DR `a`, IR all ones compared with the address-dependent capture, the 32-bit pattern `0xa800ffea` against `0x54007ff5`, IR 8 compared again, the setting byte, the two TAP resets) and the effect of flags `0x18` (no leading reset, no IR `0xe` or IR 9 block, no capture compare on the setting scan); the capture word is the bit-reverse of the address in bits 27:20 with bit 31 set, which is `E(a)`; the section 4 time-order words (`0x200`, `0x50`, `0x800` against `0x850`) decode correctly. Not checked: package 2's table and scan (the page's "line-for-line identical" was taken from the page), the release wrapper's `SYPCR` gate, and the inference that the IR is exactly 12 bits (a longer IR passes the step 4 compare, as [scan-interface.md](scan-interface.md) says; the page's argument rests on step 5 needing a bypass register).
