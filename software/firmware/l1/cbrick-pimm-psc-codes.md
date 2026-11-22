# C-brick processor module (PIMM) PSC codes: what the L1 and the IP35 PROM make of each four-bit code

Answers the open question in [cbrick-env-pimm-display-usb.md](cbrick-env-pimm-display-usb.md) section 7 (which PSC code a retail PIMM reports). Two images were read in Ghidra (decompiler output checked against capstone listings of the immediates):

- L1 1.44.0 from the IRIX 6.5.30 `l1.bin` (sha256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`), ColdFire package 0 (`Base 1MB image`, flat body sha256 `3c8a7aa93eb38502289cf0c2921ea91e6800228005bd8abd67425ce615f7b364`, loaded at `0xffe00000`). Addresses are package 0, first program copy. Package 2 carries the same C-brick tables (checked below).
- The IP35 PROM, SGI Version 6.210 (payload from the IRIX 6.5.30 image, 1,476,264 bytes, sha256 `554924380189a03a512156af2faa38344902f251b49fd5c80c253c23ecaff27b`, loaded at `0x1fc00000`; 64-bit MIPS, Ghidra `MIPS:BE:64:default`). Addresses are the PROM's physical addresses.

Grades follow [AGENTS.md](../../../AGENTS.md).

## 1. Where the code is read

- **L1 (established, package 0 `0xffe4482c`).** One read of expander 10, `PIMM PSC`: bus-switch segment 3, wire `0x4a` (write form; reads use `0x4b`, seven-bit `0x25`), all eight bits inputs. Low nibble is PIMM 0, high nibble PIMM 1. A read error is logged and treated as `0xff`. Byte `0xff` therefore means both modules absent: the six supply rows of each PIMM print `<not present>` in `env` and the PIMM 1 sensors are disabled (the PIMM 0 sensors stay enabled, see [cbrick-env-pimm-display-usb.md](cbrick-env-pimm-display-usb.md) section 2).
- **PROM (established, `0x1fc1f3a0`).** Reads one 64-bit location at node-local uncached PIO `0x9000000034000000 + (nasid << 33)` (the base constant is at `0x1fd677b8`). It takes the low nibble as PIMM 0's code; when that is `0xf` (absent) it takes the next nibble (PIMM 1's) instead. This is the same nibble layout the L1 reads, so the two are inferred to be the same strap bits seen through two paths; what the PIO location is (a Bedrock register or a mirror) is unknown. The routine also looks at bits 15:12 of that value: `0x2` adds `0x30` to the low nibble, `0x4` adds `0x20`, `0x8` adds `0x10` to the low byte. The banner routine (`0x1fc698dc`) tests the same field of a neighbouring location (`0x9000000014000000`) and prints "running in IP45 mode" for `0x8` and "running in IP34 mode" for `0x4`, so those are other board families; a C-brick code is the plain nibble 0 to 15 (inferred from the banner pairing, not from a read of a real C-brick).
- Both images treat code 15 as no module.

## 2. What the L1 accepts and does with each code (established)

- **Accepted:** 0 to 8, 10 and 15 (`0xffe446cc`). 9 and 11 to 14 log `Unknown PSC for PIMM` and leave the stored byte at its RAM start value 0 (inferred: nothing else stores it), so such a module is treated as present with code 0.
- **Pair rule (`0xffe44750`):** two present modules of different codes must both be 1 or 6; any other pair logs "PIMM types are incompatible" and PIMM 1 is dropped (set to 15).
- **Module power step (hook `0xffe44b48`, restore `0xffe44d54`):** codes 1, 3, 4, 5, 6, 7 and 10 only. At hold the PIMM's processor supply margin byte is written 0 (see section 4), 10 ticks pass, then expander 13 (PIMM 0, wire `0x42`, segment 4) or 15 (PIMM 1, segment 5) gets mask `0xc0` set; at release `0xc0` is cleared, 3 ticks pass, the margin byte is restored from the persistent setting or the per-code default below, and 10 more ticks pass. Codes 0, 2 and 8 get only the `0xc0` writes; code 15 nothing.
- **`cpu d` allowed (`0xffe449e4`):** codes 3, 4, 5, 7 and 10 only (the disable bits are bits 7 and 6 of expander 12 or 14, `PIMMn POWER`).
- **Type-change check (`0xffe44bd4`):** the two codes are compared with persistent settings `0x56` and `0x57`; a difference logs "PIMM type changed, setting the default voltage margins" and stores both. Code 8 stores 0, not 1, in settings 40 and 41 unless setting 45 is set (`0xffe447b8`).

## 3. Per-code table

L1 columns are established from the hook table the C-brick set-up routine installs (`0xffe45504`, hooks at `0xffe45034`, `0xffe451e8`, `0xffe44684`; helpers `0xffe44f84`, `0xffe44fec`, `0xffe45088`, `0xffe45174`). PROM columns are established from the table described in section 5. Voltages are the float constants decoded (for example `0x3ff33333` is 1.9 V). The DAC bytes are the values written to the PIMM monitor's output register (section 4), as unsigned bytes: low, normal, high.

| Code | VCPU nominal | VCPU DAC (low / norm / high) | SRAM nominal | SRAM ratio | SRAM DAC (low / norm / high) | PROM CPU clock | Secondary cache | Data bus |
|---:|---:|---|---:|---:|---|---:|---:|---|
| 0 | 1.9 V | 118 / 147 / 171 | 3.3 V | 1.3333 | 50 / 131 / 193 | 400 MHz | 4 MB | SDR |
| 1 | 1.5 V | 30 / 55 / 79 | 2.5 V | 1.0 | 45 / 117 / 186 | 500 MHz | 8 MB | DDR |
| 2 | 1.9 V | 118 / 147 / 171 | 3.3 V | 1.3333 | 50 / 131 / 193 | 400 MHz | 8 MB | SDR |
| 3 | 1.6 V | 77 / 110 / 142 | 2.5 V | 1.0 | 45 / 117 / 186 | 600 MHz | 8 MB | DDR |
| 4 | 1.2 V | 5 / 35 / 60 | 2.5 V | 1.0 | 45 / 117 / 186 | 700 MHz | 8 MB | DDR |
| 5 | 1.2 V | 5 / 35 / 60 | 2.5 V | 1.0 | 45 / 117 / 186 | 800 MHz | 8 MB | DDR |
| 6 | 1.6 V | 77 / 110 / 142 | 2.5 V | 1.0 | 45 / 117 / 186 | 500 MHz | 8 MB | DDR |
| 7 | 1.25 V | 39 / 62 / 86 | 2.5 V | 1.0 | 45 / 117 / 186 | 800 MHz | 8 MB | DDR |
| 8 | 1.9 V | 118 / 147 / 171 | 3.3 V | 1.3333 | 50 / 131 / 193 | 360 MHz | 4 MB | SDR |
| 10 | 1.325 V | 92 / 105 / 119 | 2.5 V | 1.0 | 45 / 117 / 186 | 800 MHz | 8 MB | DDR |

- The VCPU nominal is what the supply table prints and what the monitor limits are built around (the margin-value routine `0xffe4bc0c` calls the hook for supply types `VCPU` and `SRAM`). The SRAM ratio multiplies the SRAM monitor reading (supply class 2, hook `0xffe44684`; `0x3faaaaa8` is 1.33333). Nominal and ratio rows are established.
- Codes 1 and 6 are the two codes the pair rule lets mix, and the PROM maps both to the same row (500 MHz, 8 MB, same Config value). That fits one CPU part with two board strappings and is inferred.
- Which retail processor each row is cannot be read from either image. The clock, cache size and supply voltage fit the public product history: a 1.9 V part at 360 or 400 MHz is R12000 class, 500 and 600 MHz at 1.5 to 1.6 V is R14000 and R14000A class, 700 to 800 MHz at 1.2 to 1.3 V is R16000 class. That is a hint from outside the images (inferred, unverified here); the PROM's own table names no processor.
- A model reports the code in the row with the clock and cache it wants; the images do not say which of the duplicate rows (0 and 2 at 400 MHz, 1 and 6 at 500 MHz, 5, 7 and 10 at 800 MHz) a given retail part used. Both images accept all of 0 to 8 and 10, so any of them boots; codes 1 and 6 are the only pair that can sit in one brick next to each other with different codes.

## 4. What the margin steps write (answers the open item in [cbrick-power-up-after-scan.md](cbrick-power-up-after-scan.md))

The supply records for `PIMMn VCPU` (records 1 and 2) and `PIMMn SRAM` (10 and 15) carry a margin kind of 1 and a margin device byte at record offset 6: monitor 3, 5, 4 and 6 (`PIMM0 A`, `PIMM1 A`, `PIMM0 B`, `PIMM1 B`). The margin routine (`0xffe4d26c`, kind 1, via `0xffe68104`) writes the value to **register `0x19` of that monitor** (wire `0x5c` or `0x5e`, segment 4 or 5), an eight-bit register. The public DS1780 and ADM1024 register maps place an analogue-output register at `0x19`, which fits a trim DAC driving the regulator (hint, not verified against a data sheet here; inferred). Established: monitor and register; the value tables in section 3; the default is the "norm" value, set by `0xffe4bf60` through the hook; a persistent margin (`margin low|norm|high`, or a stored custom byte, settings `0x14 + n` and `0x82 + n`) replaces it when set.

The same register is the margin device of three non-processor supplies: `DIMM` (2.5 V, record 3) uses monitor 2 (`NODE 2`, wire `0x5c`, segment 2), the other 2.5 V rail (record 4) monitor 1 and the 1.5 V rail (record 5, margin kind 2) monitor 0. The static default bytes in the records (norm, high, low) are `0x88`, `0xc9`, `0x3e` (DIMM), `0x7f`, `0xc2`, `0x35` (2.5 V) and `0x73`, `0x49`, `0xaa` (1.5 V; the kind-2 rail runs the other way, high below low). The PIMM records' own static bytes (`0x93`, `0xab`, `0x76` for VCPU; `0x83`, `0xc1`, `0x32` for SRAM) equal the code 0, 2 and 8 row, which is the fallback if the hook were absent.

The module power step touches only the two VCPU records (supply indexes 1 and 2): at hold the VCPU register is written 0 and at release the margin byte (established, `0xffe4e4f8` and `0xffe4e450`). When the SRAM register is written was not read (unknown).

## 5. The PROM table

Established, read in full (PROM `0x1fc1f3a0`, `0x1fc1e034`, `0x1fc1dc74`):

- A 64-entry signed 32-bit index at `0x1fd51f20`, indexed by the decoded code, giving a row number or -1.
- Twenty-nine 16-byte rows from `0x1fd52020` (rows 29 and up are unrelated data). A row is: the R10000-family CP0 Config register value to print and use, the CPU clock in Hz, the hub clock in Hz (200,000,000 in every row), and four bytes (`0x10`, a variant 0, 1 or 2, `0x10`, 0). The variant byte is 0 in every row the C-brick codes reach and 1 or 2 only in rows reached by other board families' codes; its meaning is unknown.
- The index maps codes 0 to 8 to rows 2, 3, 4, 8, 10, 11, 3, 15 and 5, code 10 to row 15 and code 14 to row 1 (200 MHz, 4 MB, SDR; the L1 rejects code 14). Codes 9, 11, 12, 13 and 15 map to -1 and the PROM prints "Unknown PIMM_PSC type/no board EEPROM values".
- The Config values decode with the public R10000 Config layout (the PROM's own printer `0x1fc1dc74` names the fields: SysClkDiv bits 12:9, SCClkDiv 21:19, SCSize 18:16 as 2^(n-1) MB, DDR bit 23). The cache and data-bus columns above are that decode: for example code 1 is `0x549ca985` (SysClkDiv 4, 8 MB, DDR), code 3 `0x549cab85`, code 5 `0x569caf85`, code 0 `0x5413a785`.
- The PROM first tries a board EEPROM block, then this PSC table ("Config values loaded from board/PIMM PSC").

## 6. How the PROM and kernel name the module

- The PROM labels the two modules `PIMM 0` and `PIMM 1` (table at `0x840f8` in the payload) and the kernel prints `Pimm 0 CPU A` to `Pimm 1 CPU B` for cache errors and `PIMM0`, `PIMM1` for Sysad errors (strings in `unix.IP35`, observed). No string in either image carries a PSC code or a processor model; the kernel names the processor only from the CP0 revision (`R10000`, `R12000`, `R14000`) and no PSC table was found in it (searched by string and by the PROM routines above, not by a full read of the kernel).
- The board names are `IP35` (C-brick node board, also the identity name the L1 reads), `IP34`, `IP45`, `IP53`, `IP59` and `IP39`; the PROM and kernel print the per-node CPU-count names `IP45_4CPU`/`IP45_2CPU` (kernel) and `IP53_4CPU` to `IP59_1CPU` (PROM). The PIMM's own EEPROM name is `IP35PIMM` (hint carried from [cbrick-mbus-registration.md](cbrick-mbus-registration.md); the L1 compares none).
- The I/O-brick naming switch in the PROM (`0x1fc4b384`) maps the L1 type character to a name and a board-type number: `I` (`0x49`) Ibrick 0x71, `P` Pbrick 0x72, `X` Xbrick 0x73, `N` Nbrick 0x74, `%` PEbrick 0x75, `#` PXbrick 0x76, `=` IXbrick 0x77, `V` (`0x56`) CGbrick 0x78, anything else `WEIRDIO` 0x70. These characters are the same ones the L1 identity routine returns ([personalities.md](personalities.md)).

## Unknown

| Question | Route |
|---|---|
| The retail processor name of each row | a processor-module parts list or the R14000/R16000 data sheets matched to the Config values; neither image names one |
| What the PIO location read by the PROM is, and whether a real C-brick shows the plain nibble in bits 3:0 and zero in bits 15:12 | a Bedrock register map or a capture |
| The role of the PROM row's variant byte (0, 1, 2) and of the L1 per-code float returned by the hook at `0xffe4524c` (0.0263 to 0.0417) | read the callers of the `+0xb4` hook (margin display) and the PROM's consumers of the row fields |
| When the SRAM margin register is written | read the margin walk that visits records 10 and 15 |
