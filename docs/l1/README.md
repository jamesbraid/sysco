# L1 notes

Reading notes for the L1 system controller. Everything here is read from L1 1.44.0, package 1 (`Fuel/PE/O300 1MB`, composite offset `0xefdd0`, 982,412 bytes, sha256 `0c969f9e98c73826495dc37bff685b3403e248993ba92194a288333d9d42b2f4`), a ColdFire MCF5206e image loaded at `0xffe00000`, unless a note says otherwise. Addresses are in the first of the two flash-bank copies; the second sits `0x70000` higher with address constants relocated.

| Note | Subject |
|---|---|
| [package-container.md](package-container.md), [coldfire-package-sections.md](coldfire-package-sections.md), [boot-memory-base.md](boot-memory-base.md) | the seven-package container, the reset region, the loader candidates, RAMBAR and MBAR values |
| [arm-linux-packages.md](arm-linux-packages.md) | the two ARM Linux L1 packages of each set: kernel and CramFS layout, brick labels and bitstreams, user-space programs by name |
| [mbus.md](mbus.md), [mbus-environment-task.md](mbus-environment-task.md), [environment.md](environment.md), [board-records.md](board-records.md), [env-reset-debug.md](env-reset-debug.md) | the M-Bus master, EEPROM records, the environment task and its reset countdown |
| [usb-overview.md](usb-overview.md), [usb-control.md](usb-control.md), [usb-bulk.md](usb-bulk.md), [usb-descriptor-fields.md](usb-descriptor-fields.md), [usb-descriptors-routing.md](usb-descriptors-routing.md), [usb-controller-registers.md](usb-controller-registers.md), [usb-lower-controller.md](usb-lower-controller.md), [usb-timeouts.md](usb-timeouts.md) | the USB device side |
| [mcf5206e-peripherals.md](mcf5206e-peripherals.md) | the on-chip peripherals and external devices the code touches besides the M-Bus and USB |
| [rs422-router.md](rs422-router.md) | the packet router, the node UART link and the RS-422 peer links |
| [debug-console.md](debug-console.md), [debug-task.md](debug-task.md) | the text command task and the `debug` command |
| [unclassified-regions.md](unclassified-regions.md), [unguarded-indexed-anchors.md](unguarded-indexed-anchors.md) | what is known about code and data the linear listing cannot place |
| public evidence: [ip45-board-fitment.md](ip45-board-fitment.md), [ip45-mbus-public-fitment.md](ip45-mbus-public-fitment.md), [ip45-mbus-public-hardware.md](ip45-mbus-public-hardware.md), [mbus-ds1780-candidate.md](mbus-ds1780-candidate.md), [usb-controller-public-hardware.md](usb-controller-public-hardware.md), [usb-bus-public-evidence.md](usb-bus-public-evidence.md), [usb-l2-revision.md](usb-l2-revision.md), [usb-public-host-drivers.md](usb-public-host-drivers.md) | what published manuals, data sheets and posts say about the board; hints, not findings |

Curated tables, read by `scripts/sysco_tables.py` (every row carries a grade and a locator):

| Table | Rows |
|---|---|
| [functions.tsv](functions.tsv) | names and roles for routine and table addresses |
| [request-codes.tsv](request-codes.tsv) | the request catalogue by task and code |
| [io-registers.tsv](io-registers.tsv) | memory-mapped registers and external devices the code touches |
| [protocols.tsv](protocols.tsv) | descriptors, framing, routing and record formats, some with byte checks |
| [packages.tsv](packages.tsv) | one layout row per Linux L1 package: kernel, CramFS and entry counts |

## Listing

[disasm/mcf5206e-1.44.0-fuel-pe-o300/](disasm/mcf5206e-1.44.0-fuel-pe-o300/) holds the linear listing of that body (`l1.lst`: address, instruction bytes, mnemonic and operands), and a per-routine summary of calls, references and table jumps (`l1.sum`). The firmware's own text strings are not committed; read them from the image in `root/`. The decoder is a 68040-mode M68K decoder; ColdFire-only encodings and data mixed into the text decode as `.dw` words or implausible instructions. Read a listing as a map. `functions.tsv` names are descriptions in our own words, not the firmware's identifiers.

Package bodies that other sets share byte for byte are listed once in the function table; the listing for other packages and versions is not generated yet.
