# Which L1 packages serve the SN2 (Altix) bricks

The IRIX 6.5.30 `l1.bin` (L1 1.44.0) is a container of seven packages, five ColdFire and two ARM Linux
([package-container.md](package-container.md)); patch SG0007149 carries the same layout at 1.48.1, and the Linux
Altix support disc ships 1.62.1. IRIX never runs on an Altix, but SGI shipped one controller firmware set for the
Origin 3000, Origin 350 and Altix families. This note says, brick type by brick type, which package can serve an Altix or
Prism brick. The chips repository's `platforms/sn2/` pages describe the machines.

Grades follow [AGENTS.md](../../../AGENTS.md). Sources: the package readings in this repository (cited by note),
SGI's *L1 and L2 Controller Software User's Guide* `007-3938-005` (April 2006, printed pages; its revision log adds the
Altix 350 at 003, the Altix 3700 Bx2 and the Prism at 004 and the Altix 4700 at 005), SGI's Altix 350 guide `007-4660-001`
and Altix 4700 guide `007-4823-001`, and the brick type codes in Linux v5.3
(`arch/ia64/include/asm/sn/l1.h`).

## What identifies an Altix brick in the firmware

**Established.** Two sets of names in the shipped packages are not Origin names:

- The ColdFire packages 3 (`Legacy 2MB image`) and 5 (`2MB image`) name `SHUB`, `EFI` and `SAL` in their status words
  (90 and 86 `SHUB` strings against 3 in packages 0 to 2, 23 `EFI` against 1 to 9, and 16 `SAL` against none), carry a four-node board
  module (`SHUB N0` to `N3`) and the SHUB node chip layer, and replace the processor-boot stage names of the older
  packages ([personalities.md](personalities.md), "Which board code is linked into each package").
- The ARM Linux packages contain named routines for the Altix node boards: `IP57_Init`, `IP57_Probe`, `IP57_Set_MemCfg`
  and seventeen more (twenty `IP57_*` routines in all); `IP67_Cfg_Init`; `SHUB2_Init`, `SHUB2_Cfg_Init`, `SHUB2_ResetHold` and
  `SHUB2_ResetRelease`; `TIOInit` and the `TIO*` link and reset routines; `OpusTio_BrickOpsInit`; `Athena_BrickOpsInit`; and
  `Daytona_JTAG_Sel`. They appear in every ARM reading from 1.34.2 (6.5.27) to 1.44.0 (for example
  [arm-ip57-configuration-control-1.40.5.md](arm-ip57-configuration-control-1.40.5.md),
  [arm-linux26-shub2-tio-state-1.44.0.md](arm-linux26-shub2-tio-state-1.44.0.md),
  [arm-tio-link-reset-1.40.5.md](arm-tio-link-reset-1.40.5.md)).

**Established, from SGI's manuals.** `IP57` is the node board of the Altix 350 (007-4660-001 pp.100, 121 and 132: "IP57 Node
Board" in the base compute, CPU expansion and CMPX modules). The L1 guide's valid blade types for the Altix 4700 call
the compute blades C1 and C2 "SHub 2.x based" (007-3938-005 p.50), which is the generation the `SHUB2_*` routines serve.

## The brick type letters

**Established.** The package name table maps product names to the type character
(`ip35` to C, `opus` to O, `nl4r` to D and the rest of [personalities.md](personalities.md)), and the display table maps
characters to names. The Altix-relevant names are: `O` Opus, `F` Opus TIO, `J` PA, `K` IA, `^` TIO Test Card, `+` Athena,
`Z` Daytona (1932 Chassis), `|` C (IP63, 1932 Chassis), `$` C (IP69 HACK, 1932 Chassis), `.` 191010 Chassis, `,` 1932
Chassis, `}` 1955 Chassis, `-` Quad Dense Router, `{` IP71, `M` C (IP67), `5` C (IP41), `#` PX, `=` IX.

**Established.** Linux v5.3 defines brick type response codes with the same letters in lower case: PX `#` (0x23), IX `=`
(0x3d), IP41 `5` (0x35), OPUS `o`, OPUS_TIO `f`, PA `j`, IA `k`, SA `^`, ATHENA `+`, DAYTONA `z`, IP45 `4`, 1932 `,` and
191010 `.`. The L1's own `config` output prints the type letter in lower case too (`003c01`, 007-3938-005 p.49). The
case is the only difference for every letter that appears in both lists: the thirteen name pairs `C c`, `I i`, `N n`, `P p`,
`R r`, `X x`, `Y y` (X2), `A a` (IP34), `O o`, `F f`, `J j`, `K k` and `Z z` match by product name. Inferred that the firmware keeps the
upper-case form internally and folds it on the wire; the fold itself was not read in a routine. One name differs: Linux calls
code `^` "SA" and the firmware displays it as "TIO Test Card".

Linux v5.3 also defines the board codes the L1 reports for Altix 4700 blades (`l1.h`): `CA` IP69, `CB` IP63, `IB` base
I/O, `IC` two-slot PCIe, `ID` three-slot PCI-X, `IE` four-slot PCI-X and PCIe, `AB` ABACUS, `AD` Daytona. They match
the blade types of 007-3938-005 p.51 by description: base I/O is the IA blade, two-slot PCIe is I2E, three-slot PCI-X is
I3X, four-slot is I4E (the guide's table prints "14E"). Established for those four. Inferred that `CA` and `CB` are the
C1 and C2 compute blades and `AB` the RC FPGA blade, from the order and the names.

## Package assignment

The host updater offers an image to a brick only if the brick's type letter occurs in the image's supported-brick
string ([package-container.md](package-container.md), "Updater selection evidence"), so the strings below are the
authoritative filter for the ColdFire packages. The ARM packages carry no such string in the readings so far; their
reach is by the routines and labels named in [arm-linux-packages.md](arm-linux-packages.md).

| Brick or blade | Product | Packages that can serve it | Grade |
|---|---|---|---|
| Altix 350 compute, CPU expansion and CMPX modules (IP57) | Altix 350 | ARM, both: `IP57_*` routines. ColdFire 3 and 5 carry type `O` and the record name `IPF_2UINT` | The IP57 routines and the board name are established. Inferred that `O` (Opus) is this module: `IPF_2UINT` reads as an Itanium 2U unit, and the 350 modules are 2U (007-4660-001 pp.69, 100). **Unknown** which CPU the 350's L1 has, so whether the ColdFire or ARM package ships on it. Route: the 350's `l1cmd cfg` version output or an L1 flash dump |
| Altix 330 module | Altix 330 | not tied | **Unknown.** No label, routine or string names the 330; its guide (007-4790-001) names no board. Route: read the 330 guide's internal-components pages for a board code |
| C-brick (Altix 3000) | Altix 3300, 3700 | ColdFire `Legacy 2MB` and `2MB` only (`5` is in their string; no ARM package lists `5` or `C`) | Established for the L1 and PROM logic (an IP41 product name in record `0xfe` gives `0x35`); inferred that a real Altix 3000 C-brick's record holds `IP41` (support: the PROM's built-in default string, the banner, and `5` being the only candidate in the supported-brick string). The L1 chooses the type from the product name in board record `0xfe`, and replies `0x35` to the node's rack, bay and type request; read in the 1.62.1 `2MB image` ([sn2-l1-1.62.1-packages.md](sn2-l1-1.62.1-packages.md)). The captured `config` or `hinv` line was not seen |
| R-brick | Altix 3700 | ColdFire 0 and 2 carry `C` and `R`; ColdFire 3 and 5 carry neither `R` nor `C`; in 1.62.1 `NL3R 1MB image` carries `R` | Inferred that it is the Origin 3000 R-brick (same letter and enclosure) |
| IX-brick, PX-brick | Origin 3000 and Altix 3000, 3700 Bx2 | ColdFire 3 and 5 (string `5O#=?`, `O5#=?`) | Established for the strings. These bricks are shared with Origin 3900, so the strings do not separate the families. The Origin 3000 `PXBRICK` name in record `0xfe` is tested by packages 0 to 2 too, but their strings lack `#` and `=` |
| CR-brick | Altix 3700 Bx2 | probably ColdFire 3 and 5: a board module for four SHUB nodes is linked into both | Inferred: the guide gives the CR-brick up to four SHub nodes (007-4377-002 p.49), the firmware module names `SHUB N0` to `N3`. Unproven which type letter it reports |
| Compute, memory, I/O and RASC blades, IRU, dense router | Altix 4700, Altix 450 | ARM Linux 2.6 package (label `Linux L1 image`), both versions | Inferred. Only the 2.6 package labels the IP69 chassis, IP71, the `191010` and `1955` chassis and the Quad Dense Router; the 4700's optional dense router module is named alike (007-4823-001 contents, Chapter 5); the 2.6 images are built 9 June 2006, after the 4700 guide of March 2006 |
| Prism (deskside and rack) | Silicon Graphics Prism | not tied | **Unknown.** The L1 guide adds the Prism at revision 004 (007-3938-005 record of revision) but no label in any package names it |
| `TIO Test Card`, `Opus TIO` | TIO board support | the ColdFire display tables name `^` and `F`; the ARM packages name `TIO*` and `OpusTio_*` routines | Established as names. Their product is unknown |

## Which letter the SN2 PROM accepts from the L1

The SHub 1 PROM (snprom 10.54) asks the L1 for the brick type twice: request `0x0003` for the KLCONFIG builder and request `0x0013` for the PCI
bridge attach. Both replies carry the single stored type character; the PROM lower-cases `A` to `Z` and does no other folding on the path to the bridge
attach (the module word it builds from request `0x0003` is a separate path: it folds `5`, `o`, `m` and `@` to `c`, see [sn2-l1-1.62.1-packages.md](sn2-l1-1.62.1-packages.md)). The bridge attach (`pic_attach2`) accepts only the lower-cased letters `#`, `=`, `m`, `o` and `v`; any other letter, `f` and `c` included, prints "Unknown
ioboard" and abandons the bridge (established; trace in the chips repository's PIC page, SAL runtime section). For the L1 side this means:

- An Altix 350 module whose L1 answers `F` (the Opus TIO letter) would not get a PIC attached by this PROM. The ARM L1's `F` handler is the one that installs
  the `IP57_*` hooks and answers the IO10 as device id 99 ([sn2-prom-link-1.62.1.md](sn2-prom-link-1.62.1.md)), so the firmware family that looks like the 350 reports `F`.
  That is an unresolved conflict, not a settled letter: either the 350's L1 reports `O` (the ColdFire packages 3 and 5 carry `O` with the record name
  `IPF_2UINT`, above) or the ARM `F` family is not what ships on the 350 base compute module, or the PROM never attaches the 350's PIC through this path.
  Inferred: `O` (Linux `L1_BRICKTYPE_OPUS`, `0x6f`), because it is the accepted letter closest to an Altix 350 module (`#` and `=` are the IX and PX bricks, `M` is the C-brick IP67 in the table above, `V` is unnamed).
- What settles it: a captured `0x0013` or `0x0003` reply from a 350's L1, or a `config` line from its L1 console (the L1 prints the letter in lower case, 007-3938-005 p.49).

## Packages that do not serve an SN2 brick

**Established** by their strings and linked modules:

- ColdFire package 1 (`Fuel/PE/O300 1MB image`), string `%4A?`: Origin 300, Fuel and the PE-brick only.
- ColdFire packages 0 (`Base 1MB image`) and 2 (`MIPS 2MB image`) serve the Origin 3000 brick set plus the Chimera and
  Voyager boards; the readings link no `SHUB` or `SAL` code. They share the `C` and `R` letters with the Altix 3000
  bricks, so they may serve those two Altix bricks; that is inferred only, from the shared enclosures
  (007-4579-001 pp.99 and the chips repository's R-brick page).
- The container version, 1.44.0, is also the ColdFire version; the Altix-specific work in this set therefore came from
  the same firmware release as Origin 350 L1 support.

## Open

1. The Altix 330 and the Prism: no tie. Needs the product name in some package's file list (the Linux packages' user
   space is only named by program, not read), or a board name from the machines' guides.
2. ~~The Altix 3000 C-brick's type letter.~~ Read: `5` (`IP41`), from the product name of board record `0xfe`
   ([sn2-l1-1.62.1-packages.md](sn2-l1-1.62.1-packages.md)). Still open: the contents of a real brick's record `0xfe`
   and a captured `config` line.
3. ~~Whether the 1.62.1 images on the Altix support disc add Altix 450 and 4700 bricks beyond the 1.44.0 labels.~~ Read:
   the 1.62.1 ARM packages carry the same supported-brick strings as 1.44.0, and the Linux L1 program adds SHub 2, TIO,
   PCI Express and Altix 4700 blade support (IP73, IP75, IP81) while its IP57 routines are unchanged
   ([sn2-l1-1.62.1-packages.md](sn2-l1-1.62.1-packages.md)).

## Review

Independent review, 2026-11-14, by a reviewer who did not write this note. Reviewed sysco revision 89bff53.

Checks:
- IP57 is the Altix 350 node board: 007-4660-001 prints "IP57 Node Board" in the base compute module (p.100), the CPU expansion module (p.121) and the CMPX module (p.132), and the second guide repeats the board in its interface-board text (-002 p.114). Established as written; the page range was corrected from "pp.100 to 105" to the three pages that name the board in each module (mechanical).
- "The 350 modules are 2U" (007-4660-001 p.69): confirmed. The inference that type `O` with the record name `IPF_2UINT` is the 350 module is reasoning from a name and the enclosure height; it is correctly graded inferred, and the note correctly says the 350's own L1 CPU is unknown.
- 007-3938-005 p.50 "SHub 2.x based" for C1 and C2 and p.51 blade list (including the table's "14E" misprint): confirmed. p.49 `config` output with `003c01` and the "IRouter:read failed" text: confirmed.
- Linux v5.3 `arch/ia64/include/asm/sn/l1.h`: every brick type code and board code the note lists is present with the stated value and case (PX 0x23, IX 0x3d, IP41 0x35, IP45 0x34, OPUS `o`, OPUS_TIO `f`, PA `j`, IA `k`, SA `^`, ATHENA `+`, DAYTONA `z`, 1932 `,`, 191010 `.`; board codes CA, CB, IB, IC, ID, IE, AB, AD). The count of thirteen case-folded name pairs checks out. The package-to-brick reasoning (type letter in the supported-brick string decides whether the updater offers an image; shared enclosures do not separate Origin 3000 from Altix 3000) is sound as stated and every unknown names a route.
- Not re-verified here: the package readings the note cites (string counts, `IP57_*` routine counts, supported-brick strings, the June 2006 build date of the 2.6 images), the Altix 4700 guide's chapter 5 dense-router naming, and the revision-log dates of 007-3938-005 (its record of revision has no extractable text).
- No manual text beyond short quoted phrases; names such as Opus, Athena and Daytona are strings from the shipped firmware; no private paths.

Findings: none that change a claim. Note the `l1.h` table in the note is graded established, which matches the evidence order (SGI's Linux platform headers rank first).

Cleared scope: the IP57 claim, the type-letter mapping against Linux, and the package assignment table as reasoning. Not cleared: the firmware-side counts listed as not re-verified.

### Review (type letter accepted by the SN2 PROM, commit badc55e)

Independent re-read in Ghidra of the SHub 1 PROM (snprom 10.54) and of the 1.62.1 ARM `l1` program (sha256 `22cb6287...`, extracted from `l1-1.62.1.bin` package 4). At stops in the sof=2 leaf helpers, r34 and above are out of frame. Confirmed: the PROM sends `0x0013` (`0xffa7cbe0`), lower-cases `A` to `Z` and no more, falls back to `0x0003` only when the reply status is -101 and the caller wants a type, and `pic_attach2` accepts `0x23`, `0x3d`, `0x6d`, `0x6f`, `0x76` and aborts after the PIC, configuration and BAR writes. The ARM L1's `0x0013` case: for a type other than `,` `}` `.` `$` with `brickNumSlots` of 1 or less it replies status 0, integer 0, the stored letter unchanged (so `F` stays upper case on the wire); multi-slot bricks get the slot and a halfword, `-106` on a failed state check; an unknown request gets -101. Corrected: [sn2-prom-link-1.62.1.md](sn2-prom-link-1.62.1.md) described `0x0013` as slot and state only; and "O is the only accepted letter that names an Altix product" was too strong, since `M` is a C-brick letter and `#`, `=` are used by Altix 3000. Unchanged and open: whether an `F` brick has one slot, and which letter a 350 reports. A single-slot `F` answer would be refused by this PROM. Real hardware can pass the list with `O`, `#`, `=` or `M` (and an unnamed `V`).
