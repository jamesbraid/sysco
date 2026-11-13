# Tezro L1 brick identity

Which brick-type character the L1 firmware gives a Tezro, and what the published
software says about the Tezro product lines. The character table and the
firmware's own names are in [l1/personalities.md](../../software/firmware/l1/personalities.md); the
2U server and workstation device tables are in
[l1/ip53-eeprom-record-tables-1.44.0.md](../../software/firmware/l1/ip53-eeprom-record-tables-1.44.0.md).

Grades follow [l1-board-contract.md](../../devices/l1_board/README.md) section 1.

## Product mapping

- Established, from the firmware. The character `9` is the workstation profile:
  its identity record is `WS_INT_53` and its display name is `Chimera WS`. The
  type-9 initialiser stores the required first letter `P` of the system serial.
  See [l1/ip53-eeprom-record-tables-1.44.0.md](../../software/firmware/l1/ip53-eeprom-record-tables-1.44.0.md).
- Established, from the firmware. The character `Q` has the display name
  `Rackmount WS`. See [l1/personalities.md](../../software/firmware/l1/personalities.md).
- Inferred, from a software expectation. The public Linux IP35
  machine-identification patch 35 names `9` as a Tezro deskside system and `Q`
  as a Tezro rackmount system. The patch's own wording is the source for both
  names. The firmware does not use the word Tezro. Unverified: the patch text has
  not been read in this repository, and the public copy could not be fetched.
- Inferred, from the same patch. It names `7` as Origin 350 or Onyx 350, and says
  which one depends on the graphics inventory. The firmware calls `7` the 2U
  server profile, which
  [l1/ip53-eeprom-record-tables-1.44.0.md](../../software/firmware/l1/ip53-eeprom-record-tables-1.44.0.md)
  reads as Origin 350 and Onyx 350 without a graphics test. The graphics rule is
  the patch's, not the firmware's.

The character decides the board code. The firmware chooses it from the board's
identification records and a stored setting, as
[l1/personalities.md](../../software/firmware/l1/personalities.md) describes, and that reading does not
include a graphics inventory. The graphics-dependent name for `7` therefore has
no firmware reading behind it here.

## Rack Tezro boards

The one retained rack Tezro L1 record does not show an IP53 node board. See
[serial-number-requests.md](../../software/firmware/l1/serial-number-requests.md). The `Q` mapping above
says nothing about which node boards a rack Tezro carries.

## Firmware revision

- Established, from the image: patch SG0007149 carries an L1 whose version is
  1.48.1 ([firmware-versions.md](../../evidence/firmware-versions.md)). Its release note covers
  IRIX 6.5.24 to 6.5.30 and does not assign that image to a product, Tezro
  included. Unverified: the note's text was not reread in this repository.
- Inferred, from the Tezro captures this repository cites: none of them, in
  [serial-number-requests.md](../../software/firmware/l1/serial-number-requests.md), records an L1
  revision. No public Tezro record here supports 1.48.1 or any other value.
- Unknown: which L1 image a Tezro was shipped or fitted with. The 1.48.1 image
  is the newest L1 in the patch, not a measured Tezro value. The public forum
  thread that names L1 1.48.1 for Tezro flashing does so from a post, not an
  image ([controllers.md](../../evidence/controllers.md)). Routes tried: the patch release
  note, that thread, and the serial and part captures. What would settle it: an
  L1 version reply (general request `0x0005`, the firmware-revision request)
  from a Tezro with its board and firmware identity recorded, or a flash image
  from a release aimed at Tezro.

## Unknown

- The reply a Tezro L1 gives to general request `0x0003`, the brick-type request.
  No reply from a Tezro L1 with a recorded board revision is in the sources this
  repository cites. What would settle it: a retained reply from a Tezro whose
  board revision is recorded, showing the character returned.
- Whether the Linux names match a Tezro's firmware character. Settling this needs
  the same retained reply.
