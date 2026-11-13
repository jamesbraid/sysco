# L1 in the Origin 300 PCI expansion chassis

The Origin 300 PCI expansion chassis has an L1 of its own. It runs the same
selected L1 1.44.0 image as the compute chassis, and takes a different
personality from its type character and its board inventory. It has no
processor. The firmware registers four board-record positions: a power board,
a PCI logic board and two BIC positions. How many BIC boards the product
fits is not settled here.

Grades follow [l1-board-contract.md](../../devices/l1_board/README.md) section 1.

**Source and status.** Each item below was re-read in the 1.44.0 package 1
listing (the Fuel/PE/O300 1 MB image, load address `0xffe00000`) and carries
its address. Items marked inferred are call-path readings that were not traced
to the end.

## Identity

- General request `0x0003` returns the stored brick-type character (handler
  `0xffe11ffc`, [request-code-names.md](../../software/firmware/l1/request-code-names.md)),
  which is `0x25` on the expansion chassis (established;
  `PCI_EXPANSION_TYPE` in `devices/l1_board/registers.toml`). The identity
  routine sets `%` from the record names `PEBRICK` (record `0xfe`) or `BIC`
  (record `0xfc`) and tests for `0x25` at `0xffe555be`. The compute
  chassis returns `0x34` for the same request, the character `4` that
  [l1-board-contract.md](../../devices/l1_board/README.md) uses for the IP45 brick.
- `0x25` is the character `%`. The board-name table in the same contract
  assigns `%` to the BIC record, so the two readings agree.
- Because the reply is the stored character, the type comes from stored state,
  not from a probe of the chassis.

## Board-record positions

The expansion controller has four kind-1 board-record positions, registered
from the table at `0xffe44a14` (four 12-byte entries: selector, capacity, kind
and a name pointer to `POWER`, `LOGIC`, `BIC 0` and `BIC 1`). Each position's
configured M-Bus selector matches the default wire table for the four reserved
record numbers (`0xff` to `0xfc`) that
[l1-board-contract.md](../../devices/l1_board/README.md) section 8 gives:

| Ordinal | Owner | Configured selector | Default record number |
|---|---|---|---|
| 0 | Power board | `0xa4` | `0xff` |
| 1 | PCI logic board | `0xa8` | `0xfe` |
| 2 | BIC position 0 | `0xa0` | `0xfd` |
| 3 | BIC position 1 | `0xac` | `0xfc` |

- The table at `0xffe44a14` sets a capacity of `0x0200` for each position (established).
  This is a table value. It is not a probed EEPROM size and does not show that
  the device holds that many valid bytes.
- Inferred, not traced: request `0x0006` takes the same ordinal as its
  component argument and uses the kind-1 area, chunk and error rules of the
  compute chassis.
- Inferred, not traced: a read of a BIC position with no board fitted returns
  an absence error, and the present BIC record is returned. The error code is
  not carried here.
- The expansion personality's table at `0xffe44a14` and its registration
  routine `0xffe44544` list no IO-8 record and no memory-module record
  (established for the registered tables; other paths were not searched).

## UART selectors and slots

- Type `0x25` associates UART API selectors 3 and 4 with software slots 1 and 2
  (established: the peer-link initialiser `0xffe2dba4` jumps through a table at
  `0xffe2dcd6` indexed by type minus `0x21`; the `%` arm at `0xffe2dd96` stores
  3 and 4 in the slot 1 and slot 2 entries).
- Type `0x34` associates selectors 3 and 4 with slots 1 and 9 (arm
  `0xffe2dda2`). This matches the peer-slot mapping in
  [rs422-router.md](../../software/firmware/l1/rs422-router.md).
- These are software tables. They do not establish which connector a selector
  reaches, and they do not allow forwarding between slots with the same number.
- Inferred, not traced: the expansion path skips the compute chassis's port
  naming, output queue and writer task, and its common endpoint initialiser
  writes a RAM descriptor without touching UART registers.
- The selected image probes four external UART blocks and configures peer
  selectors 3 and 4 and management-console selector 5 for type `%`; the node
  link on selector 6 is configured for `4` and not for `%`
  ([l1-board-contract.md](../../devices/l1_board/README.md), table membership
  of the quad UART setup). These are call paths read from the image, not
  records of successful peripheral initialisation.

## Unknowns

- Whether the BIC's Bedrock UART register bank is present and ready. The
  expansion's UART probe is read from the call paths only.
- Which cable each expansion selector reaches, and whether the BIC connects
  electrically to the L1 selectors at all.
- Whether main-power control and BIC startup work as the image expects.
- What request `0x0003` returns on a real expansion chassis. A controller
  capture of that request, with the L1 version recorded, would test the reading
  above. No such capture is on record.
