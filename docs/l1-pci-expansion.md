# L1 in the Origin 300 PCI expansion chassis

The Origin 300 PCI expansion chassis has an L1 of its own. It runs the same
selected L1 1.44.0 image as the compute chassis, and takes a different
personality from its type character and its board inventory. It has no
processor. The chassis holds a power board, a PCI logic board and one or two
BIC boards; the documented product has one BIC.

Grades follow [l1-board-contract.md](l1-board-contract.md) section 1.

**Source and status.** These facts come from a firmware reading of the selected
L1 1.44.0 image, reviewed independently. No image address is recorded for them
yet, so they count as located, not established, until each one is tied to an
address in the image. Unverified.

## Identity

- General request `0x0003` returns type `0x25` on the expansion chassis
  (established; `PCI_EXPANSION_TYPE` in `devices/l1_board/registers.toml`). The compute
  chassis returns `0x34` for the same request, the character `4` that
  [l1-board-contract.md](l1-board-contract.md) uses for the IP45 brick.
- `0x25` is the character `%`. The board-name table in the same contract
  assigns `%` to the BIC record, so the two readings agree.
- Request `0x0003` takes the stored brick-type character, so the type comes from
  stored state, not from a probe of the chassis.

## Board-record positions

The expansion controller has four kind-1 board-record positions. Each position's
configured M-Bus selector matches the default wire table for the four reserved
record numbers (`0xff` to `0xfc`) that
[l1-board-contract.md](l1-board-contract.md) section 8 gives:

| Ordinal | Owner | Configured selector | Default record number |
|---|---|---|---|
| 0 | Power board | `0xa4` | `0xff` |
| 1 | PCI logic board | `0xa8` | `0xfe` |
| 2 | BIC position 0 | `0xa0` | `0xfd` |
| 3 | BIC position 1 | `0xac` | `0xfc` |

- The selected image sets a capacity of `0x0200` for each position (established).
  This is a table value. It is not a probed EEPROM size and does not show that
  the device holds that many valid bytes.
- Request `0x0006` takes the same ordinal as its component argument. It uses
  the kind-1 area, chunk and error rules of the compute chassis.
- On a chassis with one BIC, the present BIC record is returned. A read for the
  unfitted BIC position returns an absence error. The error code is not
  carried here.
- The expansion personality adds no IO-8 record and no memory-module record.

## UART selectors and slots

- Type `0x25` associates UART API selectors 3 and 4 with software slots 1 and 2.
- Type `0x34` associates selectors 3 and 4 with slots 1 and 9. This matches the
  peer-slot mapping in [rs422-router.md](rs422-router.md).
- These are software tables. They do not establish which connector a selector
  reaches, and they do not allow forwarding between slots with the same number.
- The expansion path skips the compute chassis's port naming, output queue and
  writer task. Its common endpoint initialiser writes a RAM descriptor and does
  not touch UART registers.
- The selected image probes four external UART blocks and configures peer
  selectors 3 and 4 and management-console selector 5. A compute chassis of type
  `0x34` also configures selector 6 and attaches its processor packet service to
  it. The expansion chassis does neither. These are call paths read from the
  image, not records of successful peripheral initialisation.

## Unknowns

- Whether the BIC's Bedrock UART register bank is present and ready. The
  expansion's UART probe is read from the call paths only.
- Which cable each expansion selector reaches, and whether the BIC connects
  electrically to the L1 selectors at all.
- Whether main-power control and BIC startup work as the image expects.
- What request `0x0003` returns on a real expansion chassis. A controller
  capture of that request, with the L1 version recorded, would test the reading
  above. No such capture is on record.
