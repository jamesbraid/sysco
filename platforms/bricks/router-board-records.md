# Router board records in the L1 EEPROM

A router's board record sits in an EEPROM on the router board. The L1 of the
enclosure that holds the board serves it to the host, through the router's own
UART, when the host sends request `0x0006` (EEPROM read) for component 1. This
note covers the records for the three router boards that Origin 3000-family
machines carry:

- the R-brick router (Origin 3400 and 3800, Onyx R-bricks), personality `R`;
- the NUMAlink module of the Origin 300 and Origin 350, inferred to run
  personality `R`;
- the router inside the Origin 3900 Cx-brick, personality `6`.

It also covers the component numbering the L1 uses for each requesting port
on a Cx-brick, the other answers its handler gives that personality, the power
inlet record and the DIMM SPD records of the Cx-brick.

**Grades.** The sysco scheme applies: **established** is read in full from the
L1 image, **inferred** is a reasoned step from partial evidence, **unknown**
is not settled by any source found. Host software behaviour (the boot firmware
and the IRIX kernel), public driver source, and inventories posted by owners of
real machines are software expectations or observations, so they are inferred
here, not established.

**Status.** The personality `6` table, the port rule and the other `6` answers
were taken from a separate reading of L1 1.44.0 package 2 (the `MIPS 2MB image`
body, sha256 `ac81f501f01c6789e756ab1f5ec69fadb1e4c04225511d541340831686e04e70`).
That reading is not repeated in this repository. Those entries are Unverified.
The `0x0006` and `0x0007` handlers of package 2 are the ones at `0xffe14876` and
`0xffe146ba` in [l1/ip53-eeprom-record-tables-1.44.0.md](../../software/firmware/l1/ip53-eeprom-record-tables-1.44.0.md).

## Who asks for the record

Two host programs read the router record (inferred, from the readings of host
software that are not repeated here):

- the boot firmware, once per router, to build the router's manufacturing string;
- the IRIX kernel, at router set-up, to label the router's hardware graph vertex.

Both send `0x0006` to the L1 for component 1: area 1 (chassis), then area 2
(board). Public Linux SN1 code does the same for the router motherboard, at the
L1's logic component (1). Its router power board is component 0, board area only
(inferred, public Linux `v2.5.40` `arch/ia64/sn/io/eeprom.c`).

Neither host program was seen to read component 0 on the router path, so the
power board record matters only if something else asks for it (see unknowns).

## Records per product

| Router | Personality | Component 0 | Component 1 | Board name, part, revision | Grade |
|---|---|---|---|---|---|
| Origin 3400/3800 R-brick | `R` | power board, wire `0xa4` | logic board, wire `0xa8` | `ROUTER`, `030-1634-002`, `-B` | established for the component table; inferred for the name, part and revision (see below) |
| Origin 300 and Origin 350 NUMAlink module | `R` (inferred) | power board (inferred) | router board (inferred) | `ROUTER`, `030-1634-003`, `-A` | inferred for everything here |
| Origin 3900 Cx-brick router | `6` | power inlet, wire `0xa4` | router board, wire `0xa8` | unknown | established for the table and for these numbers as the router UART asks them; unknown for the record contents |

## Personality `R`

Established, L1 1.44.0 package 2. The firmware picks `R` when the board record at
wire `0xa4` is named `RPWR`, or, if no record at `0xa4` has a matching name, when
the record at `0xa8` is named `ROUTER`. The `R` installer registers two records
in this order:

| Component | Wire address | Kind | Registry name |
|---|---|---|---|
| 0 | `0xa4` | board record | `POWER` |
| 1 | `0xa8` | board record | `LOGIC` |

Component 1 is the logic board. The same two records appear in the R-brick
table of [l1-board-contract.md](../../devices/l1_board/README.md).

The NUMAlink module is inferred to run `R`. Its board is named `ROUTER` in the
inventories below, and only the two Origin 3000 images carry an `R` personality.
The Origin 300 base module's own image has none. A `brick` command on a NUMAlink
module L1, reading its `type` and `source` fields, would settle it.

## Personality `6` (Cx-brick): the record table

Established (L1 1.44.0 package 2; table at `0xffe5596c`, 38 entries, registered by `0xffe50d18`; re-read). The identity routine maps one
product name, read at wire `0xa4`, to letter `6`. The installer for `6` registers
38 EEPROM records. Their names are plain words (`PWR INLET`, `ROUTER`,
`NODE 0` to `NODE 3`, `N0 DIMM0` and so on). This is the only table in the image
with four node boards, so it is the Cx-brick personality. The identification from
the names and count is inferred; the table itself is established.

| Component | Bus selector, wire address | Kind | Registry name |
|---|---|---|---|
| 0 | 0, `0xa4` | board record | `PWR INLET` |
| 1 | 0, `0xa8` | board record | `ROUTER` |
| 2 | 2, `0xac` | board record | `NODE 0` |
| 3 to 10 | selector bytes `0x12` to `0x82`, `0xa0` | SPD | node 0 DIMMs, in the order the table lists them |
| 11 | 3, `0xac` | board record | `NODE 1` |
| 12 to 19 | selector bytes `0x13` to `0x83`, `0xa0` | SPD | node 1 DIMMs, same order |
| 20 | 4, `0xac` | board record | `NODE 2` |
| 21 to 28 | selector bytes `0x14` to `0x84`, `0xa0` | SPD | node 2 DIMMs |
| 29 | 5, `0xac` | board record | `NODE 3` |
| 30 to 37 | selector bytes `0x15` to `0x85`, `0xa0` | SPD | node 3 DIMMs |

The node board records use selectors 2 to 5 for nodes 0 to 3 (established). The
selector byte of an SPD record is inferred to pick a multiplexer channel: its low
nibble follows the node and its high nibble the DIMM. The source gives only the
range of those bytes, so the byte for each DIMM is not listed here. The DIMM order
per node is 0, 2, 4, 6, 1, 3, 5, 7 in table order.

The table numbers are not the numbers a host sends. Requests are renumbered by
the port they arrive on, as described next.

## Component numbers by requesting port (Cx-brick)

Established, L1 1.44.0 package 2. Unverified.

Every `0x0006` and `0x0007` request passes its component number through a
translation step before the presence and range checks. Package 2 installs the
step for every personality. For most letters it changes nothing. Letters `0`,
`7`, `8`, `9`, `E`, `Q` and `V` swap components 0 and 4. Letter `5` and letter
`O` have rules of their own. Letter `6` uses the rule below.

**Ports.** Personality `6` opens five serial ports on the system-controller side.
Each port has its own receive task. A request that arrives on port *p* is stamped
with a source field of *p* × 24 plus the request's subchannel number modulo 24.
The translation recovers *p* by dividing that field by 24. Ports 0 to 3 are the
four node boards' Bedrock chips, in board order. Port 4 is the router ASIC's UART
(inferred from the brick-serial lookup below, which maps positions 0 to 3 to the
`NODE` records and position 4 to `ROUTER`). So the four Bedrocks and the router
reach the L1 on five separate ports (inferred for the physical wiring).

**Rule** for a request for component *c* on port *p* (established):

| Port | *c* = 0 | *c* = 1 | *c* ≥ 2 |
|---|---|---|---|
| node board *n* (*p* = *n*, 0 to 3) | 2 + 9*n*, `NODE` *n* | 3 + 9*n*, node *n*'s DIMM 0 SPD | *c* + 9*n* |
| router UART (*p* = 4) | 0, power inlet | 1, `ROUTER` | *c* + 36, past the table |

After translation the handler checks the number. If the record is absent, or the
number is 38 or more, it answers minus 104. A board-area request (areas 0 to 3)
to an SPD record, or an SPD request (area 4) to a board record, also answers
minus 104 (established, the same checks as the handler for the Origin 300 table in
[l1/mbus.md](../../software/firmware/l1/mbus.md)).

What this means for the requests a host sends (established for the rule; the
requests themselves are inferred from the host readings above):

- A node board's component 0 request reaches its own `NODE` record: node 0 gets
  record 2, node 1 gets 11, node 2 gets 20 and node 3 gets 29.
- A node board's components 3 to 10 reach its own eight DIMM SPD records, entries
  3 + 9*n* to 10 + 9*n*.
- A node board's component 2 also reaches its own `NODE` record (2 + 9*n*), the
  same record as its component 0.
- A node board's component 1 reaches its own DIMM 0 SPD record, not the router. A
  board-area request there answers minus 104. The host asks components 1 and 2 for
  a board area only through its local I/O-board path, which a Cx-brick node may
  not take. Whether it does was not traced (unknown).
- A node board's component 11 or more reaches another board's entries (node 0 asking
  11 gets `NODE 1`), or from 38 on answers minus 104. No host request of this kind
  is known.
- The router UART reaches only the power inlet (0) and the router (1). Its component
  1 request is the host's router read, so that read reaches the `ROUTER` record.
  Components 2 and above answer minus 104.

The Origin 300 port rule in [l1/ip53-eeprom-record-tables-1.44.0.md](../../software/firmware/l1/ip53-eeprom-record-tables-1.44.0.md)
differs in three places for a Cx-brick node link: component 1 is that node's DIMM 0
SPD, not the router; components 11 and above are *c* + 9*n*, not the table number
(link 0 gives the same result either way); and on the router link components 2
and above answer minus 104, not the table record.

## Other answers from the same handler (personality `6`)

Established, L1 1.44.0 package 2. Unverified.

- **`0x0003`** returns rack and bay, then the personality letter. On a Cx-brick that
  letter is `6` (byte `0x36`).
- **`0x0004`** (brick serial number) answers per position: the board serial of
  `NODE 0` to `NODE 3` for positions 0 to 3, of `ROUTER` for position 4, and of the
  power inlet for any higher position. It answers minus 104 when that record is
  absent or has no serial. The brick serial the L1 reports for itself is the
  `ROUTER` record's serial on letter `6`. Other letters use the record at index 0.
- **`0x000e`** returns the position, the port number *p* above. Each node's request
  gets its own position.

## Power inlet record (Cx-brick component 0)

Table component 0 is a board record (header, chassis, board and internal-use
areas) at bus selector 0, wire `0xa4`, registered as `PWR INLET` (established for
the registration). It is inferred to be the EEPROM of the Cx-brick's power entry
module, from the registry name and the manual's name for that module, the inlet
board. That module carries the four 48 V to 12 V converter sets, the power switch,
the console and L1 USB ports and the display connector (established in the
Origin 3900 user's guide, `007-4653-001`, printed page 57). The record's field
contents are unknown.

What can read it:

- The L1 itself. At start the identity routine reads the board record at `0xa4`
  and maps one product name there to personality `6`. This is inferred to be the
  same EEPROM, since it has the same selector and address.
- A host request on port 4 for component 0. No request on ports 0 to 3 can reach
  it. The boot firmware never asks component 0 on the router path. The kernel's
  router read has a power-board branch that asks component 0's board area, which
  on a Cx-brick would land here. Its callers were not traced (unknown).

A reimplementation of the L1 may leave this record absent and answer minus 104, as
long as it states that limitation. A record that is served must be a checksummed
board record. Its contents would be an approximation.

## DIMM SPD records (Cx-brick)

**Record length.** The registration table at `0xffe5596c` gives each SPD entry a capacity of
`0x0100` (256 bytes; established, entries 3 to 10 and the like). An earlier version of this note said
package 2 sizes each SPD record at 160 bytes and the host reads all 160. No instruction that serves
160 bytes of SPD was found (the `0xa0` immediates near the request handlers are array element
sizes), so the 160-byte figure is unconfirmed and the 256-byte table capacity is the established one.

**Which component the host asks for.** During memory set-up the boot firmware reads
the eight SPDs of each node, one area 4 read per DIMM in 64-byte chunks, until the
remaining count is 0. It picks the component for DIMM *k* (0 to 7) from a formula
chosen by the node's board class and the L1's type letter (inferred, host
software):

| Node | Component for DIMM *k* |
|---|---|
| board class `0x8` (matches the Origin 300 table) | 2 + (*k* >> 1) + 2(*k* & 1), *k* ≤ 3 |
| board class `0x4` (matches the components a Fuel's OpenBSD kernel asks for) | 4 + (*k* >> 1) + 2(*k* & 1), *k* ≤ 3 |
| L1 type letter `7`, `8`, `9`, `0` or `Q` | 5 + (*k* >> 1) + 4(*k* & 1) |
| any other, Cx-brick letter `6` included | 3 + (*k* >> 1) + 4(*k* & 1) |

On a Cx-brick node, DIMMs 0, 2, 4, 6, 1, 3, 5, 7 are components 3 to 10 (inferred),
and the port rule then reaches that node's own DIMMs.

**What the L1 must serve.** Inferred, from the host's checks on the bytes:

- An SPD only where a DIMM is fitted. An empty socket answers minus 104.
- Bytes 0 to 63, with byte 63 the checksum of bytes 0 to 62 (modulo 256). The host
  counts a DIMM as present only when the checksum matches. The L1 is reported to
  answer minus 107 when its own check of those bytes fails. Package 2's check is
  inferred, not established.
- Byte 64 not `0xff`. A real JEDEC manufacturer code is the natural choice.
- Byte 13 zero, or the real device width, the same in both DIMMs of a pair.
- Byte 31 consistent with the memory the bank really has.
- Bytes 2 and the timing bytes are not read by the host. Zero passes, as long as
  byte 63 matches. A real DDR DIMM carries real values here (unknown in detail).

The L1's `serial all` printer shows bytes 95 to 98 as the DIMM serial. Its
DIMM-validation step reads bytes 95 to 98 and 128 to 151, but only for some type
letters. On letter `6` the column prints `N/A` (established for the printer). Nothing
traced reads byte 99.

**Real Cx-brick SPDs.** Cx-brick DIMMs are DDR SDRAM: 512 MB DIMMs built from
128-Mbit devices and 1 GB DIMMs from 256-Mbit devices, sold as premium-directory
kits (established in the Cx-brick addendum, `007-4572-001` table 2, page 4). No
Cx-brick SPD dump is known. The nearest real data is the `serial all` DIMM table of
an Origin 350 node board, which prints bytes 64 to 71 and 95 to 98, the part number
from bytes 73 to 90, the revision from bytes 91 and 92, and a speed from byte 9.
That capture shows Samsung parts (`M3 46L2820DT2-CA0`, revision `2D`, speed `10.0`),
so byte 9 is `0xa0` (observed in the posted output).

The fitted DIMM population per node is unknown. The Cx-brick addendum's rules for
DIMM pairs (first pair in slots 0 and 1, both DIMMs of a pair alike) are the only
published source for it.

## Board record format

The router records use the board-record format the L1 serves for every board
(inferred, from public Linux `v2.5.40` `include/asm-ia64/sn/eeprom.h` and
`arch/ia64/sn/io/eeprom.c`). A field's type/length byte has the type in bits 7:6
(`11` is 8-bit ASCII) and the length in bits 5:0. An area's byte 1 is its length in
8-byte units. The area's bytes, `8 x` that length, sum to zero modulo 256, the last
byte being the checksum.

- **Chassis area (area 1):** format byte, length, chassis type, part-number field
  (up to 20 bytes), serial-number field (up to 10 bytes), padding, checksum. Public
  Linux says the chassis type is always `0x17`, rack mount (inferred for the driver;
  unknown for a router).
- **Board area (area 2):** format byte, length, language (0), three-byte
  manufacturing date in minutes from 1996-01-01, then type/length-prefixed fields:
  manufacturer (up to 10 bytes), product name (up to 14), serial number (up to 10),
  part number (up to 20), one FRU file byte, board revision, an EEPROM-size field and
  one size byte, a temperature-waiver field, then the end marker `0xc1` or, on main
  boards, further key fields. The area pads to its length and ends with the checksum.

Field text: the router's EEPROM stores the part as `030_1634_002` with revision `_B`
(R-brick) or `030_1634_003` with `_A` (NUMAlink module). Software prints them with
hyphens. The product name is not converted (inferred, from the one printed C-brick
EEPROM dump in `007-3938-001` page 47, part `030_1604_001`, revision `_F`, and the
underscore-to-hyphen conversion in public Linux).

Keep each area at or under 512 bytes. The host's read buffers are 512 bytes and its
chunk loop has no bound of its own (inferred).

## Part numbers and names by product

**R-brick (Origin 3400/3800, Onyx R-bricks).** `ROUTER`, `030-1634-002`, revision
`-B`. Observed on both R-bricks of an Origin 3400 `hinv -vm` posted 2009-04-06
(inferred, a posted inventory). The 3800 and Onyx R-brick rows are inferred, with no
inventory retained.

**NUMAlink module (Origin 300 and Origin 350, Onyx 350).** `ROUTER`, `030-1634-003`,
revision `-A`. Observed in retained inventories from three machines: an Origin 300
`hinv -vm` posted 2013-03-02, an Origin 350 `hinv -vm` posted 2009-12-12, and Onyx
350 inventories posted 2021-07-22 to 2021-07-29 (inferred, posted outputs). The two
part numbers differ in the last group and the revision letter, so they are different
boards for this note's purpose. Whether the router silicon differs is unknown.

A 2004 reseller stock list in `comp.sys.sgi.marketplace` names `030-1631-002` as
"ROUTER POWER BOARD, R-BRICK" and `030-1634-002` as the "SN1, ROUTER BOARD, O3000"
(Usenet message `a47377f9.0401261309.38ca70ab@posting.google.com`). That is inferred
as the R-brick's power board part, but no boot path reads component 0, so this note
does not depend on it.

**Cx-brick router.** The name `ROUTER` is inferred. Every retained Origin 3000-family
router label uses it, and the Cx table names the entry `ROUTER`. The part and revision
are unknown. The Cx router is a different board from the R-brick's. It carries the
same router ASIC, the brick's L1 controller, four NUMAlink 3 connectors, two internal
XIO connectors, four sets of node-board connectors and its own 48 V to 2.5 V converter
(established in `007-4653-001` printed page 57 and `007-4572-001` page 4). So its part
is neither `030-1634-002` nor `030-1634-003` (inferred). The brick serial the L1 shows
is the `ROUTER` record's serial, so on a real Cx-brick it is the router board's barcode
(inferred).

No retained Origin 3900 or Onyx 3000-with-Cx-brick inventory exists. The inventories
of IP53 and IP59 systems are all Origin 350, Onyx 350 and Tezro machines, which have no
router.

## Unknowns

| Unknown | What settles it |
|---|---|
| Power-board (component 0) record contents, and which software reads it. On a Cx-brick, the power inlet record's contents | an L1 `eeprom` or `serial all` listing on an R-brick, NUMAlink module or Cx-brick; tracing the kernel's power-board read callers |
| Router board chassis-area contents (type, part, serial) | the same listing |
| Board-area manufacturer, date and extension fields of a router record | the same listing |
| Cx-brick router name, part and revision | an IRIX `hinv -m` on an Origin 3900 or Onyx 3000 with Cx-bricks, or an L1 `serial all` from a Cx-brick |
| Real Cx-brick SPD bytes 2, 5, 13, 31 and the timing bytes, and the fitted DIMM population per node | an L1 `serial all` or SPD dump from a Cx-brick, or the JEDEC data sheet for the DIMM part |
| Whether the host asks components 1 and 2 for a board area on a Cx-brick node link (component 1 answers minus 104) | tracing the local I/O-board path's callers in the boot firmware and the kernel |
| Which kernel paths ask for DIMM SPDs | tracing the callers of the kernel's node-board EEPROM reader |
| Whether a NUMAlink module L1 runs personality `R` | `brick` output from a NUMAlink module L1 |
| Why an Origin 300 under IRIX 6.5.22 lists no router although its boot firmware printed no error | that machine under IRIX 6.5.30, or the 6.5.22 kernel's router path |

## Sources

- L1 1.44.0 package 2 (`MIPS 2MB image`), container range sha256
  `22e340c4657850647ce06e36173f6e671359ed8d26a6468c3c0ef6bde644540c`, body sha256
  `ac81f501f01c6789e756ab1f5ec69fadb1e4c04225511d541340831686e04e70`. Source of the
  personality `6` table, the port rule and the other `6` answers (Unverified here).
- Public Linux `v2.5.40`: `arch/ia64/sn/io/eeprom.c`, `include/asm-ia64/sn/eeprom.h`,
  `include/asm-ia64/sn/ksys/l1.h`.
- `007-4653-001`, *SGI Origin 3900 Server User's Guide*, printed page 57 (router board
  and power entry module contents).
- `007-4572-001`, Cx-brick addendum, page 4 (router board contents; table 2, DDR SDRAM
  DIMMs and device densities).
- `007-3938-001`, printed page 47 (C-brick EEPROM dump, field text form).
- Usenet message `a47377f9.0401261309.38ca70ab@posting.google.com` (reseller stock list,
  posted 2004-01-26).
- Posted inventories and consoles: Origin 3400 `hinv -vm` (2009-04-06), Origin 350
  `hinv -vm` (2009-12-12), Origin 300 `hinv -vm` (2013-03-02), Onyx 350 `hinv` and
  `hinv -mv`/`-mvv` (2021-07-22 to 2021-07-29), Origin 350 `hinv -mv` with `serial all`
  DIMM tables (2010-10-22), Origin 300 console and `hinv -vm` (2010-11-11).
