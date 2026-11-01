# L1 packages other than the Origin 300 one: what differs and how a brick picks its personality

Read from the IRIX 6.5.30 `l1.bin` (L1 1.44.0, sha256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`). The container holds five ColdFire packages. Package 1 (`Fuel/PE/O300 1MB image`) is the reference described in the other notes of this directory. The other four are compared with it here. Addresses are in the first flash bank of each package, which starts at body address `0xffe10000` (the second bank repeats it at `0xffe80000` in the 1 MB packages and `0xfff00000` in the 2 MB packages, so `0x70000` and `0xf0000` higher). Package names are the labels the images carry; no label says which product a package serves, so product names below appear only where a table in the image prints one. Grades follow [AGENTS.md](../../AGENTS.md).

## Layout of the five ColdFire packages

| Package | Label | Container offset | Length | Body base and end | Bank size | Record word 10 |
|---|---|---:|---:|---|---:|---:|
| 0 | `Base 1MB image` | `0x000000` | `0x0efdd0` | `0xffe00000`, `0xffeefd67` | `0x70000` | 1 |
| 1 | `Fuel/PE/O300 1MB image` | `0x0efdd0` | `0x0efd8c` | `0xffe00000`, `0xffeefd5f` | `0x70000` | 1 |
| 2 | `MIPS 2MB image` | `0x1dfb5c` | `0x1ac9ac` | `0xffe00000`, `0xfffac97f` | `0xf0000` | 2 |
| 3 | `Legacy 2MB image` | `0x38c508` | `0x1b0c4c` | `0xffe00000`, `0xfffb0c1f` | `0xf0000` | 2 |
| 5 | `2MB image` | `0x8e1250` | `0x1b2458` | `0xffe00000`, `0xfffb242b` | `0xf0000` | 2 |

Established (header words read in every package; sizes follow from last address minus load address plus one):

- Every ColdFire package starts with a 44-byte header (`ESTF`, `BINR`, load `0xffe00000`, last address, then fixed fields) and its flash image follows. Package 0 is the exception: its first 44 bytes are shared with the container header, so the container's package count (at `0x2c`) and its seven offset and length pairs sit where the other packages begin their image, and package 0's image starts at `0x30 + 8 * 7 = 0x68`. The earlier inventory read package 0 from byte 44, gave it a reset vector of `0x00000007 0x00000000` (the package count and the first offset) and placed every candidate routine 60 bytes off. The first image word pair at `0x68` is `0x301017fc 0xffe00400`, the same stack pointer and reset entry as the other packages. The 1.44.0 container's package 0 has body 982,376 bytes, ending at `0xffeefd67`.
- Each package holds its program twice. The loader table at body `0xffe01000` points at two bank starts, `0xffe10000` and a second one at `0xffe80000` (1 MB packages) or `0xfff00000` (2 MB packages). Each bank ends with a 48-byte record, type word `0x0004`, length word `0x0030`, twelve big-endian 32-bit words whose sum modulo 2^32 is zero (the record check described in [coldfire-package-sections.md](coldfire-package-sections.md)). The first-bank record sits at `0xffe7fd24` (package 0), `0xffe7fd1c` (1), `0xffeaa28c` (2), `0xffeaf75c` (3) and `0xffeb1798` (5); the checked sums are zero in all five, and the second-bank copies are at the second bank's matching offset.

## The bank record

All five records have the same twelve words, read here at the first-bank record of each package.

| Word | Content | Grade |
|---:|---|---|
| 0 | `0x00040030`: type 4, length 0x30 | established |
| 1 | bank start address (`0xffe10000`) | established |
| 2 | bank size (`0x70000` in packages 0 and 1, `0xf0000` in 2, 3 and 5) | established |
| 3 | bytes from the bank start to the end of the record | established: equals record address plus 0x30 minus the bank start in all five |
| 4, 5 | pointers to the build stamp and the version string | established |
| 6 | pointer to the end of the record | established |
| 7 | 32-bit value not reproduced | unknown, see [open-questions.md](../open-questions.md) |
| 8 | additive 32-bit word sum of the bank from its start up to the record | established: recomputed over every first-bank record and equal in all five |
| 9 | pointer to the supported-brick string | established |
| 10 | 1 in the 1 MB packages, 2 in the 2 MB packages | established value; meaning inferred as the flash size class in megabytes, from the two bank sizes that go with it |
| 11 | pointer to the label string, which sits directly before the supported-brick string | established |

Word 7: tried as the xor, the byte sum, the 16-bit-word sum, the little-endian word sum, the CRC-32 and Adler-32 of the pre-record bank and of the bank including the record header, and two rotating sums. None matches.

## The supported-brick string is the package's personality list

Established: the string word 9 points to, read in each package.

| Package | Supported-brick string | Characters, using the print table below |
|---|---|---|
| 0 `Base 1MB image` | `CIPXYN&?RG` | C, I, P, X, Y, N, `&`, `?`, R, G |
| 1 `Fuel/PE/O300 1MB image` | `%4A?` | `%` (PE brick), `4` (IP45), A, `?` |
| 2 `MIPS 2MB image` | `CIPXYN&%6780VQE9A?RG` | package 0's ten plus `%`, 6, 7, 8, 0, V, Q, E, 9, A |
| 3 `Legacy 2MB image` | `5O#=?` | 5 (IP41), O, `#`, `=`, `?` |
| 5 `2MB image` | `O5#=?` | the same five characters as package 3, in another order |

The host updater (`flashsc`, [package-container.md](package-container.md) describes the check) requires a requested brick byte to occur in this string, so the string decides which packages a given brick type can be offered. Established for the packages' side of that rule; the updater's side is already recorded. `?` is the character the firmware prints as `Unknown` and is present in every package.

## The brick-type character

One byte, kept in RAM, names the personality at run time. It is the argument of the boot dispatch, of the request replies that return a type, and of every brick-type test in the body. Its accessor is a 16-byte routine that returns the byte (`0xffe557b4` in package 1; `0xffe55b64` in package 0, `0xffe75fc8` in package 2, `0xffe6b378` in package 3, `0xffe79e60` in package 5). The byte is `0x3f` (`?`) until the identity routine sets it.

### Name to character, and character to name

Established. Package 1's name parser at `0xffe60b7c` compares the argument with a list of names and returns a character: `ip35` to C, `ip37` to 3, `ip45` to 4, `ip41` to 5, `twister` to 6, `vortix` to `@`, `opus` to O, `gemini` to B, `nl4r` to D, `nl4s` to S, `T1` to 1, `T2` to 2, `T` to T. Packages 0, 2, 3 and 5 contain the same routine with the same thirteen name tests for the first nine and add or drop names:

- Packages 0, 1 and 2: identical parse list (thirteen names).
- Package 3 adds `dorado` to L, `daytona` to Z and `ip63` to the vertical bar.
- Package 5 adds `dorado` to L, `ip69` to `$`, `191010` to `.`, `1932` to `,`, `1955` to `}`, `drouter` to `-` and `ip71` to `{`.

Established. The print routine (package 1 `0xffe60cd0`) turns a character into a display name through a jump table of `0x3c` entries in packages 0, 1 and 2 (`0x5a` in package 3, `0x5b` in package 5), at `0xffe6103a`, `0xffe60cf2`, `0xffe81cfe`, `0xffe7662a` and `0xffe7ec0e` (packages 0, 1, 2, 3, 5). Characters without a name print as themselves. The names, identical in packages 0, 1 and 2: C `C (IP35)`, 3 `C (IP37)`, 5 `C (IP41)`, 4 `C (IP45)`, 6 `C (Twister)`, 9 `Chimera WS`, 7 `Chimera Server`, 8 `Chimera Blade`, Q `Rackmount WS`, E `2UPX`, V `CG Voyager`, 0 `CGN Voyager`, O `Opus`, B `Gemini`, L `Dorado`, F `Opus TIO`, H `Opus IP65`, Y `X2`, `&` `P0 N`, `%` `PE`, `#` `PX`, `=` `IX`, J `PA`, K `IA`, `^` `TIO Test Card`, `@` `C (Vortix)`, D `NL4R`, S `NL4 Snooper`, `?` `Unknown`. Differences:

- Package 3 drops `Opus IP65` (H) and adds `C (IP63, 1932 Chassis)` (`|`), `C (IP67)` (M), `Daytona (1932 Chassis)` (Z) and `Athena` (`+`).
- Package 5 drops `Opus IP65` and adds `C (IP69 HACK, 1932 Chassis)` (`$`), `191010 Chassis` (`.`), `1932 Chassis` (`,`), `1955 Chassis` (`}`), `Quad Dense Router` (`-`), `IP71` (`{`), `C (IP67)` (M) and `Athena` (`+`).

### How the byte is chosen

Established for packages 0, 1 and 2 (the three identity routines are the same code: package 1 `0xffe55254`, package 0 `0xffe55604`, package 2 `0xffe75a68`), read through the string tests and stores:

1. A flash-size class byte is stored first: the routine reads a word at offset `0x68` of the record that `0x300002e8` points to (package 1) and stores 1 for `0xf0000` and 2 for `0x1f0000`. What the record is was not followed, so the meaning is inferred (flash geometry).
2. Identification record `0xff` is fetched through the record loader (package 1 `0xffe6bc04`, the board-EEPROM reader described in [mbus.md](mbus.md)). Its name string, compared by prefix, gives the character: `IP35` to C, `KCARL1` to G, `IP34` to A, `POWERON` to T, `X2PWR` to Y, `NIPWR` to `&`, `NPWR` to N, `RPWR` to R, `TWPWRIN` to 6, `WS_INT_53` to 9, `2U_INT_53` to 7.
3. If the byte is still `?`, record `0xfe` is fetched and compared: `ROUTER` to R, `PBRICK` to P, `IBRICK` to I, `XBRICK` to X, `X2BRICK` to Y, `PXBRICK` to `#`, `IP45` to 4, `PEBRICK` to `%`, `2U_INT_53` to 7.
4. If still `?`, record `0xfd` then `0xfc` are fetched; a `BIC` name in `0xfc` gives `%`.
5. If still `?`, the persistent setting with key 9 supplies a character through a table of accepted values; the stored byte is the type, except that a stored `0x34` is rejected. Key `0x10` supplies the adjacent sub-type byte.

So the type is read from the board's own identification records first, with a stored setting as the fallback. No strap or jumper read is involved in the choice that was read in full; the early flash-size class is a read of a memory word, not a pin.

Established for packages 3 and 5 (identity routines at `0xffe6b104` and `0xffe79e70`): the same order but a shorter list. Record `0xff` is tested against `IPF_2UINT` only (to O). Record `0xfe` is tested against `PXBRICK` (to `#`), `IP41` (to 5), `NL4_TEST` (to `!`) and `IPF_2UINT` (to O). Records `0xfd` and `0xfc` are not consulted. Then the key 9 fallback as above. The consequence is that these two packages can only identify the five characters of their supported-brick string by record, plus `!`; a stored setting reaches the rest. In the persistent-setting accessor (package 1 `0xffe59590`) packages 3 and 5 omit keys `0x10`, `0xb1` and `0xb2`, so the sub-type byte of step 5 does not exist there. Packages 0 and 2 have the same 132 keys as package 1.

### What the boot task does with it

Established. After registering the general request task, the boot routine at `0xffe10570` (the same address in all five packages) reads the type character and branches:

- Packages 0, 1 and 2: a jump table over characters `0x30` to `0x56`. The characters 0, 4, 7, 8, E, G, O, Q and V (nine of them) go to one path that starts the node-to-node link initializer (package 1 `0xffe2dba4`; the same routine is at `0xffe2dba4` in 0 and `0xffe3337c` in 2) and nothing else; characters 9 and A go to a path that skips both the link initializer and the second call; every other character calls the link initializer and then a second routine (package 1 `0xffe27040`, 224 bytes). The tables are identical in the three packages.
- Packages 3 and 5: one comparison with `0x4f` (O). Type O calls the link initializer only (package 3 `0xffe2cff8`, package 5 `0xffe355e4`); every other type calls it and then a second routine (package 3 `0xffe263a4`, 66 bytes; package 5 `0xffe22f74`, 208 bytes).

The boot path of the other bricks is therefore the Origin 300 path with a differently shaped branch; the link initializers are not byte-identical between packages 3, 5 and package 1.

## Which board code is linked into each package

Established. The routines compiled into a package are recognizable by the file-name literals their assertions carry. The table lists each literal found in the first bank. A dot means the literal is absent.

| Module literal | 0 | 1 | 2 | 3 | 5 | What it covers, from its neighbouring strings |
|---|---|---|---|---|---|---|
| `IP35.c` | Y | . | Y | . | . | node board with PIMM processor modules: PIMM power, voltage margins, `BEDROCK CNTL`, DIMM slots 4 to 7 |
| `IP45.c` | . | Y | . | . | . | the Origin 300 node board: `IP45_`, a product-name check on the EEPROM, PSC reads |
| `IP41.c` | . | . | . | Y | Y | node board: CPU probing, DIMM detection, VRM errors |
| `IP57.c` | . | . | . | Y | Y | four-node (`SHUB N0` to `N3`) board |
| `IP5359.c` | . | . | Y | . | . | node-type discovery for a 53 or 59 board |
| `IOlegacy.c` | Y | . | Y | . | . | I/O bricks: XIO, PCI bus and slot controls, C-brick and graphics ports |
| `IObrick.c` | . | . | . | Y | Y | I/O brick with bus speed, bus present and PIC controls |
| `XBridge.c`, `pci.c` | Y | Y | Y | . | . | bridge and PCI slot handling |
| `pcix.c` | . | . | Y | Y | Y | PCI-X speed and mode override |
| `Kcar.c` | Y | . | Y | . | . | the `KCARL1` board (type G) |
| `NL3R.c` | Y | . | Y | . | . | the router brick |
| `Asterix.c` | . | Y | Y | . | . | a board init with a MAC address EEPROM |
| `Twister.c`, `ChiServ.c`, `ChiWS.c` | . | . | Y | . | . | Twister and Chimera server and workstation boards |
| `Opus.c`, `SHUB.c` | . | . | . | Y | Y | the O-type brick and the SHUB node chip |
| `jchip.c`, `llpfun.c` | . | . | Y | Y | Y | JTAG delay-tuning of the link chips (`JChipInit`, `DMIN`/`DMAX`) |
| `port.c` | Y | . | . | Y | Y | port configuration tables |
| `L1Req.c` | . | . | . | . | Y | request tracing (`L1REQ: %08x - %04x`) |
| `L1Cfg_1.c` | Y | Y | . | . | Y | configuration task |
| `FZL1Cfg_1.c` | . | . | . | Y | . | configuration task |
| `SysCom_2m.c` | . | . | Y | Y | Y | the system-communication task of the 2 MB build |

Counts of marker strings in the first bank, read from each package: `PLED_` boot-code names 192, 192, 96, 0, 0 (packages 0, 1, 2, 3, 5); `SHUB` or `SHub` 3, 3, 3, 90, 86; `EFI` 1, 1, 9, 23, 23; `SAL_` or `SAL ` 0, 0, 0, 16, 16; `JChip` 0, 0, 29, 43, 49. Packages 3 and 5 therefore carry the status words of an EFI/SAL firmware boot instead of the older processor-boot stages, and a node-chip layer (SHUB) that the first three have only as a name.

## Request catalogue of the general task

Established by running the compare chain and jump tables of the general request task (task 6) for every code in each package.

- Packages 0, 1 and 2: 39 codes, the same set (`0x0001`-`0x0011`, `0x1001`-`0x100c`, `0x2001`-`0x2008`, `0x4000`, `0x4002`). The dispatcher code is byte-identical in the three (package 1 `0xffe11e18`, package 0 the same address, package 2 `0xffe14168`).
- Package 3: 42 codes. Added against package 1: `0x0012`, `0x0013`, `0x200d`, `0x200e`, `0x3001` and `0x4001`. Dropped: `0x1002`, `0x2007`, `0x2008`.
- Package 5: 44 codes. Added against package 1: `0x0012`, `0x0013`, `0x200d`, `0x200e`, `0x3001`, `0x3002`, `0x3003`, `0x3004`, `0x4001` and `0x4004`. Dropped: `0x000f`, `0x1002`, `0x1003`, `0x2007`, `0x2008`.

Handler addresses: package 3 `0x0012` `0xffe13092`, `0x0013` `0xffe11f04`, `0x200d` `0xffe13256`, `0x200e` `0xffe133de`, `0x3001` `0xffe13108`, `0x4001` `0xffe1321e`; package 5 `0x0012` `0xffe40f62`, `0x0013` `0xffe3fc26`, `0x200d` `0xffe411e0`, `0x200e` `0xffe41368`, `0x3001` `0xffe40fd8`, `0x3002` `0xffe41132`, `0x3003` `0xffe4115a`, `0x3004` `0xffe4117a`, `0x4001` `0xffe411a8`, `0x4004` `0xffe40c2a`.

Read in full: `0x0012` (the same in packages 3 and 5, 118 bytes) replies code 0, then one integer, which is the persistent setting with key `0x7e`, or `0xffff` when the setting with key `0x50` or the setting with key `0x80` is nonzero. Established (package 3 `0xffe13092`-`0xffe13107`, package 5 `0xffe40f62`-`0xffe40fd7`); the meaning of the three settings is unknown.

Other added handlers were skimmed, not read in full (inferred from their calls only, so the roles stay unclassified): `0x200d` formats a `date -u` command line for the controller's Linux clock and runs it (package 5's string adds `-u` to `hwclock`; package 3's does not), which suggests a set-time request; `0x4004` prints per-DIMM text built from `JEDEC,OKAY`, `JEDEC,NOT_PRESENT` and `JEDEC,ERROR`, which suggests a DIMM-information request; `0x3002`, `0x3003` and `0x3004` each reply with one stored RAM word (`0x300013cc`, `0x300013c8`, `0x300013dc`).

Of the 39 shared codes the dispatchers of packages 3 and 5 reach handlers with a different body shape for most (13 in package 3, 32 in package 5, by a masked instruction-shape comparison). Package 3 keeps 23 handlers shaped like package 1's. The remainder differ in size by a few bytes (a changed reply field) up to the inventory request `0x4000`, which is 1,422 bytes in package 1, 1,478 in package 3 and 918 in package 5.

## Routine-level comparison with the Origin 300 package

Generated by `scripts/personality_diff.py`, which pairs each routine of the first bank of a package with package 1's using the matcher of `scripts/version_diff.py`; [personality-diffs/summary.tsv](../personality-diffs/summary.tsv) holds the counts and [personality-diffs/routines.tsv](../personality-diffs/routines.tsv) one row per routine. A routine is an entry found by prologue or call target, so routines reached only through a jump table are folded into the previous one. Counts are routines, not functions.

| Package | Routines | Same bytes | Same code, moved | Changed, RAM address only | Changed | No package 1 partner |
|---|---:|---:|---:|---:|---:|---:|
| 0 `Base 1MB image` | 2,615 | 1,421 | 350 | 729 | 41 | 74 added |
| 2 `MIPS 2MB image` | 3,145 | 1,347 | 414 | 750 | 51 | 583 added |
| 3 `Legacy 2MB image` | 3,330 | 721 | 208 | 322 | 445 | 1,634 added |
| 5 `2MB image` | 3,386 | 687 | 176 | 273 | 423 | 1,827 added |

Package 1 routines with no partner: 116 against package 0, 95 against package 2, 961 against package 3 and 1,098 against package 5.

Reading: package 0 is package 1's code with the Origin 300 node board (`IP45.c`, `Asterix.c`) replaced by the node-board, I/O-brick, router and `KCARL1` modules, which are 74 routines added and 116 removed. Package 2 is package 0's modules plus the Twister, Chimera and PCI-X additions and the JTAG layer. Packages 3 and 5 are a different line: most shared routines changed, and the node-board code is `IP41.c` and `IP57.c` with SHUB. The shared base (about 2,000 routines unchanged or only moved in package 2, about 1,100 in package 3) is the M-Bus, USB, UART, router, request and debug-console code read in the other notes.

## What decides the personality, in short

1. The flash record (word 9 of the bank record, a string of type characters) decides which package the updater offers for a brick type.
2. At power-up the L1 derives the type character from the board's identification records (EEPROM), falling back to a stored setting; the character then selects display names, link-port tables and the boot branch above.
3. What the package adds beyond that is the compiled-in board modules in the module table. A package does not switch modules at run time by strap: the modules it lacks are absent from the image.

Unknown, with routes tried in [open-questions.md](../open-questions.md): the meaning of record word 7, the keys 9 and `0x10` settings' writers, and the roles of the added request codes.
