# V-brick L1 controller: published sources

The Onyx 3000 V-brick carries an L1 controller of its own. No firmware image
sysco reads covers it, and no reply it gives has been captured. This page
collects what the published SGI manuals and one public OpenBSD header say about
it, and what they leave open.

Grades follow [l1-board-contract.md](l1-board-contract.md) section 1, with one
addition: a fact taken from a manual or a driver that no firmware image confirms
is a **hint**, not established. Nothing here gives a register, a reply value or
an address in an image.

## Topology

- Hint. The *SGI Onyx 3000 Series Graphics System Hardware Owner's Guide*,
  007-4264-002, printed pages 52 to 55, places an L1 controller on the V-brick
  power board. That controller is separate from the XBridge midplane board and
  the V12 boards. The guide does not give the controller's type, firmware or
  serial.
- Hint. The same guide, printed page 131, describes an Onyx 3000 management
  connection that runs from the C-brick and R-brick to an L2. It is written for
  Onyx 3000. It does not show how a V-brick L1 attaches to anything, and it does
  not apply to the Origin 300.

## Type character

- Hint. The *SGI L1 and L2 Controller Software User's Guide*, 007-3938-002,
  printed page 39, lists `?` as an unknown brick type in its configuration
  table. `?` is byte `0x3f`. The table is a label, not a reply. No source held
  shows that a V-brick L1 returns `0x3f` for the brick-type request.

## No-data reply status

- Inferred, from a software expectation. The OpenBSD SGI L1 header,
  `sys/arch/sgi/sgi/l1.h` at commit `2df91d80c104636deeef8124252ddd5951b55fbf`,
  names general task 6 (EEPROM read) and a reply status for "no data" called
  `L1_RESP_NXDATA`, with the value -104.
- The L1 request tables for packages 1, 3 and 5 return -104 as "no data" for
  brick serial and system serial when the value is absent, and package 1 does
  the same for inventory (see
  `formats/l1_pkg1_requests/format.toml`, `formats/l1_pkg3_requests/format.toml` and
  `formats/l1_pkg5_requests/format.toml`; established there). The driver name and the
  image value agree on the number.
- The same header defines no V-brick reply and no reply for an unknown
  identity. A reply of -104 from a V-brick L1 would be a choice, not something
  the header or the manuals establish.

## Unknown

- Whether a V-brick L1 answers the brick-type, serial, EEPROM or sensor
  requests at all, and with what.
- The V-brick type character, and whether any type character is assigned.
- Whether the stock IRIX host command enumerates a V-brick L1. The manuals and
  the OpenBSD header do not say.

Routes tried: the two Onyx 3000 and L1/L2 manuals, checked by title and printed
page, and the OpenBSD header at the commit above. A reading of an image that
carries a V-brick controller would settle the first two unknowns. Nothing held
here includes one.
