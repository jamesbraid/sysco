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
| 7 | 32-bit value that makes the twelve record words add to zero | established for the sum: all 86 records found by their header words in the ColdFire packages of 6.5.20 to 6.5.30 and both patches add to zero; what the value is derived from is unknown, see [open-questions.md](../open-questions.md) |
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

Host-side numbering of the Chimera characters. Established in the IP35 CPU PROM 6.210 (routine at `0x1fc68ba0`) and in the IRIX 6.5.30 IP35 kernel
(`set_sn12_type`, `0xc000000000041550`), both described in the chips page `docs/questions/ip35-board-class-and-flash-record.md` and the kudzu page
`docs/ip35-node-class-and-link-checks.md`: when the host asks the controller for the C-brick type, the character '7' (`Chimera Server`) becomes 1,
'8' (`Chimera Blade`) 2, '9' (`Chimera WS`) 3, '0' (`CGN Voyager`) 4 and 'Q' (`Rackmount WS`) 5; any other reply leaves the type 0. The PROM prints a different
banner line for types 1, 2, 3 and 5. The host asks only when the node board's class word (not an L1 value) is outside the Fuel, Origin 300 and PE-brick
classes, so this table is how the L1's names reach the host, and the L1 plays no part in the class itself. The L1-side reply to that host request was not
re-read for this note (established elsewhere in this repository only for the character chosen above).

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
| the controller firmware source unit 12 | Y | . | Y | . | . | node board with PIMM processor modules: PIMM power, voltage margins, `BEDROCK CNTL`, DIMM slots 4 to 7 |
| the controller firmware source unit 14 | . | Y | . | . | . | the Origin 300 node board: `IP45_`, a product-name check on the EEPROM, PSC reads |
| the controller firmware source unit 13 | . | . | . | Y | Y | node board: CPU probing, DIMM detection, VRM errors |
| the controller firmware source unit 16 | . | . | . | Y | Y | four-node (`SHUB N0` to `N3`) board |
| the controller firmware source unit 15 | . | . | Y | . | . | node-type discovery for a 53 or 59 board |
| the controller firmware source unit 11 | Y | . | Y | . | . | I/O bricks: XIO, PCI bus and slot controls, C-brick and graphics ports |
| the controller firmware source unit 10 | . | . | . | Y | Y | I/O brick with bus speed, bus present and PIC controls |
| the controller firmware source unit 38, `pci.c` | Y | Y | Y | . | . | bridge and PCI slot handling |
| `pcix.c` | . | . | Y | Y | Y | PCI-X speed and mode override |
| the controller firmware source unit 18 | Y | . | Y | . | . | the `KCARL1` board (type G) |
| the controller firmware source unit 24 | Y | . | Y | . | . | the router brick |
| the controller firmware source unit 1 | . | Y | Y | . | . | a board init with a MAC address EEPROM |
| the controller firmware source unit 37, the controller firmware source unit 4, the controller firmware source unit 5 | . | . | Y | . | . | Twister and Chimera server and workstation boards |
| the controller firmware source unit 27, the controller firmware source unit 32 | . | . | . | Y | Y | the O-type brick and the SHUB node chip |
| the controller firmware source unit 55, the controller firmware source unit 60 | . | . | Y | Y | Y | JTAG delay-tuning of the link chips (`JChipInit`, `DMIN`/`DMAX`) |
| `port.c` | Y | Y | Y | Y | Y | port configuration tables; two literals in each package |
| the controller firmware source unit 21 | Y | Y | Y | Y | Y | request tracing (`L1REQ: %08x - %04x`) and the semaphore error of the general task; referenced by the assertion calls of all five (packages 0 to 3 store it as the tail of a longer literal, package 5 as its own) |
| the controller firmware source unit 19 | Y | Y | Y | Y | Y | configuration task; package 3 stores the name as the tail of the literal the controller firmware source unit 8 |
| the controller firmware source unit 36 | . | . | Y | Y | Y | the system-communication task of the 2 MB build |

Counts of marker strings in the first bank, read from each package: `PLED_` boot-code names 192, 192, 96, 0, 0 (packages 0, 1, 2, 3, 5); `SHUB` or `SHub` 3, 3, 3, 90, 86; `EFI` 1, 1, 9, 23, 23; `SAL_` or `SAL ` 0, 0, 0, 16, 16; `JChip` 0, 0, 29, 43, 49. Packages 3 and 5 therefore carry the status words of an EFI/SAL firmware boot instead of the older processor-boot stages, and a node-chip layer (SHUB) that the first three have only as a name.

## Request catalogue of the general task

Established by running the compare chain and jump tables of the general request task (task 6) for every code in each package.

- Packages 0, 1 and 2: 39 codes, the same set (`0x0001`-`0x0011`, `0x1001`-`0x100c`, `0x2001`-`0x2008`, `0x4000`, `0x4002`). The dispatcher instruction stream is the same in the three (package 1 `0xffe11e18`, package 0 the same address, package 2 `0xffe14168`); in the first 0x400 bytes at that address packages 0 and 1 differ in 21 bytes, all of them the low halves of call targets and RAM addresses that moved between builds.
- Package 3: 42 codes. Added against package 1: `0x0012`, `0x0013`, `0x200d`, `0x200e`, `0x3001` and `0x4001`. Dropped: `0x1002`, `0x2007`, `0x2008`.
- Package 5: 44 codes. Added against package 1: `0x0012`, `0x0013`, `0x200d`, `0x200e`, `0x3001`, `0x3002`, `0x3003`, `0x3004`, `0x4001` and `0x4004`. Dropped: `0x000f`, `0x1002`, `0x1003`, `0x2007`, `0x2008`.

Handler addresses: package 3 `0x0012` `0xffe13092`, `0x0013` `0xffe11f04`, `0x200d` `0xffe13256`, `0x200e` `0xffe133de`, `0x3001` `0xffe13108`, `0x4001` `0xffe1321e`; package 5 `0x0012` `0xffe40f62`, `0x0013` `0xffe3fc26`, `0x200d` `0xffe411e0`, `0x200e` `0xffe41368`, `0x3001` `0xffe40fd8`, `0x3002` `0xffe41132`, `0x3003` `0xffe4115a`, `0x3004` `0xffe4117a`, `0x4001` `0xffe411a8`, `0x4004` `0xffe40c2a`.

Read in full: `0x0012` (the same in packages 3 and 5, 118 bytes) replies code 0, then one integer, which is the persistent setting with key `0x7e`, or `0xffff` when the setting with key `0x50` or the setting with key `0x80` is nonzero. Established (package 3 `0xffe13092`-`0xffe13107`, package 5 `0xffe40f62`-`0xffe40fd7`); the meaning of the three settings is unknown.

Other added handlers were skimmed, not read in full (inferred from their calls only, so the roles stay unclassified): `0x200d` formats a `date -u` command line for the controller's Linux clock and runs it (package 5's string adds `-u` to `hwclock`; package 3's does not), which suggests a set-time request; `0x4004` prints per-DIMM text built from `JEDEC,OKAY`, `JEDEC,NOT_PRESENT` and `JEDEC,ERROR`, which suggests a DIMM-information request; `0x3002`, `0x3003` and `0x3004` each reply with one stored RAM word (`0x300013cc`, `0x300013c8`, `0x300013dc`).

The handler-by-handler comparison of the shared codes, with inputs, board accesses and reply layouts, is in [brick-request-handlers.md](brick-request-handlers.md).

Of the 39 shared codes the dispatchers of packages 3 and 5 reach handlers with a different body shape for most (13 in package 3, 32 in package 5, by a masked instruction-shape comparison). Package 3 keeps 23 handlers shaped like package 1's. The remainder differ in size by a few bytes (a changed reply field) up to the inventory request `0x4000`, which is 1,422 bytes in package 1, 1,478 in package 3 and 918 in package 5.

## Routine-level comparison with the Origin 300 package

Generated by `retools/tools/sysco/personality_diff.py`, which pairs each routine of the first bank of a package with package 1's using the matcher of `retools/tools/sysco/version_diff.py`; [personality-diffs/summary.tsv](../personality-diffs/summary.tsv) holds the counts and [personality-diffs/routines.tsv](../personality-diffs/routines.tsv) one row per routine. A routine is an entry found by prologue or call target, so routines reached only through a jump table are folded into the previous one. Counts are routines, not functions.

| Package | Routines | Same bytes | Same code, moved | Changed, RAM address only | Changed | No package 1 partner |
|---|---:|---:|---:|---:|---:|---:|
| 0 `Base 1MB image` | 2,615 | 1,421 | 350 | 729 | 41 | 74 added |
| 2 `MIPS 2MB image` | 3,145 | 1,347 | 414 | 750 | 51 | 583 added |
| 3 `Legacy 2MB image` | 3,330 | 721 | 208 | 322 | 445 | 1,634 added |
| 5 `2MB image` | 3,386 | 687 | 176 | 273 | 423 | 1,827 added |

Package 1 routines with no partner: 116 against package 0, 95 against package 2, 961 against package 3 and 1,098 against package 5.

Reading: package 0 is package 1's code with the Origin 300 node board (the controller firmware source unit 14, the controller firmware source unit 1) replaced by the node-board, I/O-brick, router and `KCARL1` modules, which are 74 routines added and 116 removed. Package 2 is package 0's modules plus the Twister, Chimera and PCI-X additions and the JTAG layer. Packages 3 and 5 are a different line: most shared routines changed, and the node-board code is the controller firmware source unit 13 and the controller firmware source unit 16 with SHUB. The shared base (about 2,000 routines unchanged or only moved in package 2, about 1,100 in package 3) is the M-Bus, USB, UART, router, request and debug-console code read in the other notes.

## What decides the personality, in short

1. The flash record (word 9 of the bank record, a string of type characters) decides which package the updater offers for a brick type.
2. At power-up the L1 derives the type character from the board's identification records (EEPROM), falling back to a stored setting; the character then selects display names, link-port tables and the boot branch above.
3. What the package adds beyond that is the compiled-in board modules in the module table. A package does not switch modules at run time by strap: the modules it lacks are absent from the image.

Unknown, with routes tried in [open-questions.md](../open-questions.md): the meaning of record word 7, the keys 9 and `0x10` settings' writers, and the roles of the added request codes.

## Source files each package references

Established. A file name counts when a word in the first bank points at it (the assertion and trace calls pass the name); each count below is the number of such pointer words in the first bank of that package. Many literals carry two extra leading bytes (the controller firmware source unit 26, the controller firmware source unit 25) or a longer prefix (the controller firmware source unit 8); the pointer reaches the short name, which is the one listed, and what the leading bytes are is unknown. A dot means no pointer word.

| Source file | 0 | 1 | 2 | 3 | 5 |
|---|---:|---:|---:|---:|---:|
| the controller firmware source unit 1 | . | 16 | 16 | . | . |
| the controller firmware source unit 3 | . | 6 | 6 | . | . |
| the controller firmware source unit 30 | . | 1 | 1 | . | . |
| the controller firmware source unit 46 | . | 1 | 1 | . | . |
| the controller firmware source unit 14 | . | 6 | . | . | . |
| the controller firmware source unit 12 | 7 | . | 7 | . | . |
| the controller firmware source unit 11, the controller firmware source unit 18, the controller firmware source unit 24, the controller firmware source unit 43 | 2, 6, 1, 2 | . | 2, 6, 1, 2 | . | . |
| the controller firmware source unit 15, the controller firmware source unit 4, the controller firmware source unit 5, the controller firmware source unit 37, the controller firmware source unit 39, the controller firmware source unit 45 | . | . | 11, 1, 1, 1, 1, 4 | . | . |
| the controller firmware source unit 38 | 9 | 9 | 9 | . | . |
| `pci.c` | 6 | 6 | 6 | . | . |
| the controller firmware source unit 74 | . | 23 | 23 | 23 | 23 |
| the controller firmware source unit 84, the controller firmware source unit 87, the controller firmware source unit 65, the controller firmware source unit 56, the controller firmware source unit 50, the controller firmware source unit 64, the controller firmware source unit 31, the controller firmware source unit 72, the controller firmware source unit 75, the controller firmware source unit 76, the controller firmware source unit 36 | . | . | 21, 7, 9, 15, 1, 1, 3, 10, 3, 3, 2 | 21, 7, 9, 15, 1, 1, 3, 10, 3, 3, 2 | 21, 7, 9, 17, 1, 1, 3, 10, 3, 3, 2 |
| `pcix.c`, the controller firmware source unit 60, the controller firmware source unit 55 | . | . | 16, 1, 12 | 17, 1, 14 | 17, 1, 18 |
| the controller firmware source unit 69 | . | . | 10 | 3 | 4 |
| the controller firmware source unit 13, the controller firmware source unit 16, the controller firmware source unit 10, the controller firmware source unit 27, the controller firmware source unit 32, the controller firmware source unit 63, the controller firmware source unit 20 | . | . | . | 22, 10, 4, 11, 88, 4, 1 | 22, 8, 4, 11, 88, 4, 1 |

Shared by all five with the same pointer count (so unchanged in role): the controller firmware source unit 82, the controller firmware source unit 91 and the controller firmware source unit 90 of the common system code, the controller firmware source unit 2, the controller firmware source unit 22, the controller firmware source unit 23, the controller firmware source unit 29, the controller firmware source unit 33, the controller firmware source unit 34, the controller firmware source unit 40, the controller firmware source unit 49, `i2c.c`, the controller firmware source unit 51, `main.c`, the controller firmware source unit 61, `network.c`, `nvram.c`, the controller firmware source unit 66, `test.c`, `uart.c`, the controller firmware source unit 71, the controller firmware source unit 73, the controller firmware source unit 77. Shared with different counts (the module gained or lost call sites): `display.c` (25, 25, 25, 31, 31), `eeprom.c` (14, 14, 14, 20, 20), `fan.c` (17, 17, 17, 17, 26), `port.c` (24, 24, 24, 25, 37), `power.c` (16, 16, 16, 11, 11), the controller firmware source unit 35 (6, 6, 6, 6, 7), `monitor.c` (25, 25, 25, 26, 26), the controller firmware source unit 53 (19, 19, 19, 21, 21), `temp.c` (2, 2, 2, 3, 4), the controller firmware source unit 78 and the controller firmware source unit 79, the controller firmware source unit 52, the controller firmware source unit 9 (15, 15, 15, 15, 12).

Two reads follow. The 1 MB Origin 300 package differs from the generic 1 MB package by swapping the controller firmware source unit 12, the controller firmware source unit 11, the controller firmware source unit 18, the controller firmware source unit 24 and the controller firmware source unit 43 for the controller firmware source unit 14, the controller firmware source unit 1, the controller firmware source unit 30, the controller firmware source unit 3 and the controller firmware source unit 46; the MIPS 2 MB package has all of those except the controller firmware source unit 14 (it carries the controller firmware source unit 1, the controller firmware source unit 30, the controller firmware source unit 3 and the controller firmware source unit 46 too). Packages 3 and 5 share their set of files exactly; their differences are in call counts (the controller firmware source unit 56 15 against 17, the controller firmware source unit 55 14 against 18, the controller firmware source unit 16 10 against 8, `fan.c`, `port.c`).

## Devices shared by every package

Established by byte patterns in each package's first bank. The chip-select 1 window at `0x20000000` holds a clock device read through the offsets `0x7f8` to `0x7ff`. Every package has the same pointer words in the same numbers: `0x200007f8` five times, `0x200007ff` three, `0x200007f9` twice and `0x200007fa` twice (package 1 `0xffe5a742`, `0xffe5a832`, `0xffe5e33e`, `0xffe5e5da`, `0xffe5f082`; package 5 `0xffe74d62`, `0xffe74e52`, `0xffe7a816`, `0xffe7aab2`, `0xffe7b55a`). The routine that reads the time is `0xffe5e378` (440 bytes) in package 1, `0xffe73c44` (88 bytes, preceded by a split-off helper) in package 3 and `0xffe7a804` (516 bytes) in package 5, all three classed changed against each other by the matcher, so the read has been rewritten rather than copied.

Read in package 5 (`0xffe7a804`, to `0xffe7a8c0`): the routine writes bit 6 of the byte at `0x200007f8` (a read latch), then assembles a four-digit year from that byte (bits 5 and 4 times 1000 plus bits 3 to 0 times 100) and the two-digit year from `0x200007ff` (tens in bits 7 to 4, units in bits 3 to 0), the month from `0x200007fe` (bit 4 tens), and the rest from `0x200007fd`, `0x200007fb` and the neighbouring bytes with the same tens and units split. The fields are packed decimal digits. Which register is the day, hour, minute and second beyond the three read here follows the usual order of such a device and is inferred.

Not compared here: the UART blocks at `0x50000000` and `0x60000000` and the USB windows at `0x40000000`. The 2 MB packages additionally reference the controller firmware source unit 72, the controller firmware source unit 75 and the controller firmware source unit 76; the registers those modules use are not read in this note.

## Request codes added by packages 3 and 5, read in full

All handlers below are task 6 handlers. The reply helpers are the status writer (package 3 `0xffe45bec`, package 5 `0xffe46f64`) and the integer writer (`0xffe45c10`, `0xffe46f88`). A status of `0x9c` (minus 100) is "too few arguments", `0x9b` (minus 101) "unknown or unavailable", and `0x96` (minus 106) is a hook failure.

| Code | Package 3 | Package 5 | What it does | Grade |
|---|---|---|---|---|
| `0x0013` | `0xffe11f04` | `0xffe3fc26` | Reply 0, then integer 0, then the brick-type character as an integer. Package 3 reads the type from the accessor `0xffe6b378` in the dispatcher's prologue; package 5 from `0xffe79e60`. | established |
| `0x200d` | `0xffe13256` | `0xffe411e0` | Needs seven words including the code, otherwise reply `-100`. Builds `date -u MMDDhhmmYYYY.ss; hwclock --systohc` (package 5 adds `-u`: `hwclock -u --systohc`) from arguments 1 to 6 and runs it. The remainder instruction (extension word 1 puts the remainder in the divisor register) reduces them: argument 2 modulo 13 is the month, argument 3 masked with `0x1f` the day, argument 4 modulo 24 the hour, argument 5 modulo 60 the minute, argument 1 modulo 10000 the year, argument 6 modulo 60 the second. A nonzero result from the runner prints an invalid-date message and still replies 0. The request therefore sets the time of the system the shell commands run on. | established for the argument order and formats; inferred that the commands reach a clock (only the tool names say so) |
| `0x200e` | `0xffe133de` | `0xffe41368` | Reply 0, read the time and break it into fields, then reply the year (record field at +0x14 plus 1900), the month (+0x10 plus 1), and the fields at +0xc, +8, +4 and 0 as integers. The field order is the usual broken-down time record: day, hour, minute, second. | established for the order of stores; inferred for the callee roles (`0xffe73c44` then `0xffe7586c` in package 3; `0xffe7a804` then `0xffe7e44c` in package 5, the first being the clock read above) |
| `0x3001` | `0xffe13108` | `0xffe40fd8` | Needs three words including the code (`-100` otherwise). Applies only to listed brick types: in package 3 `5`, `@`, `O`, `B`, `F`, `L`, `M` and `|`; in package 5 the same list minus `|` plus `$`, `,`, `}`, `.` and `{`. For any other type the reply is `-101`, and in package 3 so is an empty hook word at `0x30001114`. Otherwise package 3 calls the hook through that word with (0, argument 1, argument 2, 0), replies 0 when the hook result is positive and `-106` otherwise, then the result as an integer. Package 5 has no hook word and no empty-hook test: it calls the routine `0xffe7f3b8` directly with the same four arguments and replies 0 when the result is not negative. | established for both packages (package 3 through the hook word, package 5 by a direct call; see [brick-request-handlers-detail.md](brick-request-handlers-detail.md)) |
| `0x4001` | `0xffe1321e` | `0xffe411a8` | Reply 0, then 1 if the flag byte of the SHUB workaround state is set and 0 if not: that byte is at `0x30000845` in package 3 (routine `0xffe5cb40`) and at `0x3000082d` in package 5 (routine `0xffe5cfd0`). The routine prints either `WAR disabled` or the maximum and last durations of the workaround to its text argument, which this handler passes as zero. | established |
| `0x3002`, `0x3003`, `0x3004` | . | `0xffe41132`, `0xffe4115a`, `0xffe4117a` | Each calls through a hook word that the FRU module (the controller firmware source unit 20) installs when its task starts: `0x300013cc` (with the sender word at frame +0x1c) returning one integer, `0x300013c8` with no argument (reply 0, then no further field), and `0x300013dc` with the reply buffer (the hook's result replaces the reply). | established for the call shapes |
| `0x4004` | . | `0xffe40c2a` | Loops over a slot count (from the routine at `0xffe7a2fc`) and prints one comma-separated line per slot: `<n>,<name>,` then either `JEDEC,NOT_PRESENT`, or `JEDEC,ERROR,<text>`, or `JEDEC,OKAY,` followed by hex byte fields (`%02X`) and `%3d.%-2d,` values. | established for the text layout and flow ([brick-request-handlers-detail.md](brick-request-handlers-detail.md)); the naming of the fields is a hint from a public layout |

The FRU task installs seven hooks into RAM words at `0x300013c8`, `0x300013cc`, `0x300013d0`, `0x300013d4`, `0x300013d8`, `0x300013dc` and `0x300013e0`, pointing in package 5 at `0xffe2d2c8`, `0xffe2d460`, `0xffe2d710`, `0xffe2d248`, `0xffe2d2b0`, `0xffe2d730` and `0xffe2d764` (initialiser `0xffe2cfdc`, which appears in no Origin 300 counterpart: the matcher reports it and `0xffe2d460` as added). Established from the store sequence at `0xffe2d018`-`0xffe2d06c`.

Package 3's equivalent of the `0x3001` hook is `0xffe3b690` (1,616 bytes, added against package 1), installed at `0x30001114` together with a second hook at `0x30001124` (`0xffe3b368`) by the routine at `0xffe3cb98`, called from `0xffea8b10`. Established: the installation. Its body was read to the brick-type dispatch (selector 0 to 4) and to a writer of a block at record offset `0x80` that stores the bytes 2, 3 and 4 at offsets `0x80`, `0x81`, `0x82`, the value 8 at `0x87`, 1 at `0x90`, `0xc1` at `0x92` and clears `0x93` to `0x96`; the roles of those bytes are not known.
