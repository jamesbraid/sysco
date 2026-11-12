# Origin 2000 module management

How a four-node Origin 2000 module wires its management resources: the module system
controller (MSC, also called ELSC), the shared I2C bus, the node-side controller interfaces,
the shared management NVRAM and the module number. The [ELSC host request set](elsc-host-requests.md)
covers what the IRIX host driver sends to the MSC. The [multi-node console note](msc-multinode-console.md)
covers console ownership across modules. This page covers the hardware they sit on.

No shipped image was re-read for this page, so nothing here is graded established. Each
item says what it rests on, following [the published sources](published-sources.md): a
hint is a claim from a manual or a public post that no image has confirmed yet. Facts that
rest only on non-public source were left out.

## Module population

- **Hint.** One module controller (the MSC) serves the four nodes of a module. It sits on one
  shared management I2C bus, with a branch to each of Nodes 1 to 4. The module's NVRAM and
  its voltage-margin and UST control block sit on that same bus. Source:
  SGI 108-0164-002, *Origin2000 and Onyx2 Diagnostic Roadmap*, printed B-1 (PDF page 55),
  Figure B-1.
- **Hint.** The figure shows no four independent MSCs behind the four nodes and no private
  NVRAM copy for each node. Each node has its own interface to the bus, one per Hub.
- **Hint, unverified.** The MSC has one RS-232 alternate-console port. Four node interfaces
  do not mean four console ports or four module controllers. A deskside module can attach a
  terminal to that port; a rack module can route it to the multimodule system controller.
  The page reference for this was not checked in this pass.
- **Conflict.** [The multi-node console note](msc-multinode-console.md) says each node has its
  own module and its own MSC. For a four-node module this page says the four nodes share one
  MSC. The two statements cannot both hold for the same module. The console note needs
  reconciling; this page does not change it.

## Management NVRAM

- **Hint.** The four nodes share one logical NVRAM space on the management bus. Nothing in the
  figure makes it per-node.
- **Unknown.** How many physical EEPROM packages hold that space. Nothing read so far settles
  it, and a bill of materials or a board photograph would be the route.
- The controller-reserved words in that space are already in the [ELSC host request set](elsc-host-requests.md#non-volatile-store),
  with the magic byte at 0x700 and the module number at 0x708. This page adds no new addresses.

## Module number

- **Hint.** The module number is an integer from 1 to 255. Zero means the module is unassigned.
  Assigned numbers are unique within a system. A module number is not a node NASID, a physical
  slot number, a graph identifier or an array index. Source: SGI 108-0170-001, *Origin200/2000
  IP27 PROM Technical Reference Manual*, printed pages 1-4 and 1-5 (PDF pages 18 and 19).
- **Hint.** The MSC keeps the number in its shared NVRAM. A documented maintenance change to the
  system's identity takes effect at the next reboot. The same manual pages describe automatic
  assignment for a module that has no number. The manual is the only source for these three
  points, and the automatic-assignment procedure itself is not written up here.
- **Hint, unverified.** Node-card PROM logs keep backup copies of the module number, for use
  when the MSC's NVRAM cannot be read. The page reference for this was not checked in this pass.
- The module number lives in the MSC's NVRAM, which the host request set reaches at byte 0x708
  (decimal 1800). The validity marker is byte 0x700 (decimal 1792), read as 0x37. Those are the
  existing sysco readings, not from this page.

## Module serial and midplane records

Each module's serial comes from two one-wire parts: a `MODULEID` record (family 0x09 in
[defs/msc_multinode.toml](../defs/msc_multinode.toml)) and a midplane record whose name ends
in `_MPLN` (family 0x0b).

- **Inferred.** In the public captures that show locations, both parts appear under the `n1`
  node only. The one capture with locations is a `hinv -mvv` from a two-module Origin 2400 rack
  posted on 2007-11-08, with midplane part `030-0762-006` rev H. The `n2` to `n4` nodes show only
  their own node and PIMM records. A 2001 Usenet `hinv -m` for a one-module, four-node Origin 2000
  (message `9uhh6o$j75@dispatch.concentric.net`, part `013-1547-003` rev E) shows one of each but
  gives no locations. The reading that the parts are wired to the `n1` Hub's one-wire net only
  is therefore inferred from captures, not observed on hardware.
- **Unknown.** Which node's one-wire net the module controller's midplane pin reaches. A wiring
  description of the midplane would settle it; the captures do not.
- **Inferred.** The `MODULEID` record in those captures has empty part and revision fields and
  `ff` in its group, capability and variety bytes.

## Midplane part for a four-node module

- **Inferred.** A four-node Origin 2000 module, deskside or rack, carries an `8P12_MPLN`
  midplane. Part numbers seen with that name in captures are `030-0762-006` (revs H and K),
  `013-1547-003` (revs D and E) and `030-1535-001` (rev B). Every held capture of these uses
  IP31 nodes. None shows the IP27 180 or 195 MHz nodes, so no capture ties a midplane part to
  that era.
- **Inferred.** The Onyx2 deskside two-node midplane is `4P1G5_MPLN` (`013-1839-001`). It is
  not the four-node module's midplane.
- **Unknown.** Whether the `013-` and `030-` part number prefixes are related. Whether a
  two-node Origin 2000 deskside uses `8P12_MPLN` or another midplane. A `hinv -mv` from one of
  those machines would show it.

## Sources

- SGI 108-0164-002, *Origin2000 and Onyx2 Diagnostic Roadmap*, printed B-1 (PDF page 55), Figure B-1.
- SGI 108-0170-001, *Origin200/2000 IP27 PROM Technical Reference Manual*, printed pages 1-4 and 1-5.
- Public inventory output: a 2001 Usenet `hinv -m` (message `9uhh6o$j75@dispatch.concentric.net`)
  and a 2007-11-08 posted `hinv -mvv` from a two-module Origin 2400 rack.
- [Published sources](published-sources.md), for the other documents this repository cites.
