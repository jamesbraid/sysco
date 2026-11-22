# How the ARM L1 picks the brick type, and which family has four IP57 nodes (1.62.1)

Read from the Altix container `l1-1.62.1.bin` (L1 1.62.1, sha256 `b4b5634949a93f05ad6eb24462e6cd7f10104dd42ff5c2ae8616dd5d87697790`): package 4 `Linux L1 image` and package 2 `Legacy Linux L1 image`, and the ColdFire package 3 `2MB image`. The ARM program is `/stand/sysco/bin/l1` of each CramFS (1,999,676 bytes, sha256 `22cb62871e447a394acbd662f4f49f3146580afec93839953b37a5354545bfc5` for the Linux L1; 1,368,336 bytes, `45ff1a458f755d99954be0d8f953933675796e4580a565a5c3360bb9ae4d1eed` for the legacy one). Addresses below are of the Linux L1 program (ARM, little-endian, dynamic symbols present, so names are the firmware's own) unless a ColdFire body address is given. Read with a Capstone disassembly checked against the symbol table, and Ghidra for the ColdFire body. The question this answers: what does a Bx2 CR-brick's L1 report as its brick type, and which board name does it test for. Grades follow [AGENTS.md](../../../AGENTS.md).

## The ARM identity routine

**Established.** `brickInit` (`0x31c40`, 2,944 bytes) sets the type once at start. It reads board EEPROM records through `eepromReadData` (`0x12c538`) and compares the first characters of each record's product name with a fixed list (`strncmp`, `0x20ae4`). The first read is record `0xff`; the stored type byte is `?` until a name matches.

Record `0xff`, name prefix to type or subtype (the length compared is the string length unless given). Two matches set only the **subtype** byte (`0x1b5764`), not the type byte (`0x1add58`, still `?`), and name the record to read next:

| Name prefix | Type | Display name in the ColdFire print table |
|---|---|---|
| `VORTIX_L1` (9) | subtype `0x40` (store at `0x31ce0`); next record `0xfe`; type stays `?` | |
| `191010_L1` (9) | subtype `0x2e`; next record `0xee`; type stays `?` | |
| `DROUTER_L1` (7) | `-` | `Quad Dense Router` |
| `IP71` (4) | `{` | `IP71` |
| `WSINT2_TIOAGP` (13) | `L` (and sub-type 7 if the part number begins `030_2039_001`) | `Dorado` |
| `1932BKPL_SHUB` (13) | `,` unless bit 1 of persistent setting `0x7f` is set, then record `0xfe` beginning `IP69` gives `$` | `1932 Chassis`, `C (IP69 ...)` |
| `NL4R_PWR` (8), `TIO_IPF_2UINT` (13) | no type yet; a later pass reads the record whose number the routine stores (`0xfd` and `0xee`, from the stored negated constants) | |

(After the first pass the routine reads further records and tests: `NL4_RTR` to `D`, `VORTIX_PWR` (10, a power board name) to `@`, `TIO_AGP_RISER` to `B`, `TIO_PCIX_RISER` to `F`, `SNOOPDOG` to `S`, `IP67` to `M`, `FAN_CTRL` to `.`, `1955_FAN_CTRL` to `}`, `DROUTER` to `-`, `TIOTEST` to `^`, `IP41` to `5`, `PAIA_RISER` to `J`, `ATHENA_RASC2` to `+`, `DUAL_NL4R` to `.`, `1955_NL4R` to `}`.) The record numbers of the later passes come from a local variable; the first three reads (`0xff`, `0xfe`, `0xfd`) are literal. `brickInit` ends by calling `brickInitBrickOps` (`0x214fc`, call at `0x3272c`; a jump table), which calls one `*_BrickOpsInit` per type; type `@` reaches `Vortix_BrickOpsInit` (`0x3523c`), `?` the `Unknown_` initialiser.

So the type `@` is stored (`0x31fe8`) only when a record read in the second pass starts `VORTIX_PWR` (compare at `0x31fc8`); after a `VORTIX_L1` match at record `0xff` that record is `0xfe`. `brickInitBrickOps` dispatches on the type byte alone. The subtype byte is used elsewhere (the reviewer points at the FPGA image choice near `0x98d0c`; not read here). `brickTypeToChar` (`0x111778`) folds `@` to `c` (not the same result for every type: `B` gives `g` and `A` gives `a`) (jump table at `0x1117a0`; the `@` entry is `0x11190c`, which falls to the `c` store at `0x111a84`), the same fold it gives `5`, `O`, `F`, `L`, `M`, `$` and `{`. That function is what a display routine would call; that the `config` line uses it was not traced (inferred).

## Vortix is the only family with four IP57 nodes

**Established.** Four brick families call `IP57_Cfg_Init` (`0x882f4`) from their `*_i2cConfig`: `Gemini_i2cConfig` (call `0x36a18`), `Dorado_i2cConfig` (`0x3fcf0`), `OpusTio_i2cConfig` (`0x3ca60`) and `Vortix_i2cConfig` (`0x34f24`). The second argument, the node count, is 1 for the first three (`mov r1, #1`) and **4 for Vortix** (`0x34f18`). `IP57_Cfg_Init` builds a 16-byte entry per node, names each `SHUB N0` to `N3` and calls `SHUB_Init` (`0x6bdac`). `Vortix_i2cConfig` also calls `NL4R_Cfg_Init` (`0x36d94`) with a count of 2 (set at `0x34efc`, call at `0x34f00`), and `Vortix_BrickOpsInit` stores 8 at descriptor offset `0xfc` where Gemini stores 2 and 4 at `0x128` where Gemini stores 1 (the field meanings, processors and nodes, are inferred from those values; the neighbouring `IP57_CpuPresent`, `_CpuEnable` and `_CpuDisable` hooks are installed at `0x104` to `0x10c`).

**Inferred, strongly.** That is the shape of the CR-brick in *SGI Altix 3700 Bx2 User's Guide* `007-4377-002` (p.49, Figure 3-3 p.50): four SHub 1.2 nodes, each an "IP57 Node Card", two NL4 router ASICs, up to eight Itanium 2 processors (p.67). No other family in the container matches it, and no firmware string says "CR" or "Bx2" (no occurrence in the decompressed ARM programs or the ColdFire bodies; the `Bx2P` hit in the compressed ARM packages is inside compressed data and was not read). The family the firmware names Vortix is inferred to be the CR-brick, which is why the grade is inferred and not established.

The ColdFire side does not compete. The `2MB image` (1.62.1 package 3; flat body at `0xffe00000` after the 44-byte header) chooses its personality in `0xffe7b404` (record `0xff` `IPF_2UINT` to `O`; record `0xfe` `PXBRICK` to `#`, `IP41` to `5`, `NL4_TEST` to `!`, `IPF_2UINT` to `O`) and dispatches in `0xffea8b9c` on the five types `#`, `=`, `5`, `O`, `?` only. `@` takes the unsupported-type path (message, then store `?`). The two board modules are one node deep or two: the type-`O` initialiser `0xffe55cd8` starts a module (`0xffe55a80`) that calls the IP57 init `0xffe56ea0` with a node count of 1, and the type-`5` module `ip41_init` (`0xffe59678`) calls the same node-chip initialiser `0xffe5e660` with a count of 2. In the ColdFire `Legacy 2MB image` (package 1) the type-`O` startup function `0xffe50fdc` likewise calls its IP57 initialiser once, with count 1 (call near `0xffe510e0`). When the ColdFire type is still `?` after the record tests, it comes from persistent setting 9. Nothing in the ColdFire body creates four nodes (established by the two call sites and by `0xffe56ea0` having the one caller). The name `vortix` does parse to `@` in the console name table (`0xffe7ff50`) and `0x40` prints `C (Vortix)` (`0xffe80220`), but no initialiser follows.

## Is a Bx2-specific image held

**Established: no.** The L1 firmware exists only as shared containers. The package labels of 1.62.1 (`NL3R 1MB`, `Legacy 2MB`, `Legacy Linux L1`, `2MB`, `Linux L1`) and of the IRIX sets name no product, and no string in them names a CR-brick or Bx2. The brick is served by the two ARM packages, whose supported-brick string `?@BDSJKF^+LM...` contains `@` (the ColdFire strings `O5#=?` and `?R` do not). `VORTIX_L1`, `Vortix_i2cConfig` and `IP57_Cfg_Init` are in the `l1` program of every ARM package read: 6.5.27 (1.34.2, the Linux 2.4 package), 6.5.30 and patch images (1.44.0 both), and 1.62.1 (both). IRIX 6.5.26 and earlier have no ARM package, so a Bx2 L1 older than 6.5.27 (December 2004) is unknown: the ColdFire packages of those sets were not read for a four-node module and the 1.62.1 body has none.

## Unknown

- Whether a CR-brick's record `0xfe` (after a `VORTIX_L1` record `0xff`) or another second-pass record actually names `VORTIX_PWR`, which is what sets type `@`. No record from a brick is held. Nothing but the node-count match ties the family to the CR-brick.
- Which ARM L1 package (Linux 2.4 or 2.6) shipped on a Bx2; both carry the same `@` handling (the Linux L1 addresses above were not compared with the legacy program).
- Whether the Prism Extreme's CR-brick, which the Prism guide shares, reports the same letter. Same board family, same inference.
