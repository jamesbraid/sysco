# L1 notes

Reading notes for the L1 system controller. Everything here is read from L1 1.44.0, package 1 (`Fuel/PE/O300 1MB`, composite offset `0xefdd0`, 982,412 bytes, sha256 `0c969f9e98c73826495dc37bff685b3403e248993ba92194a288333d9d42b2f4`), a ColdFire MCF5206e image loaded at `0xffe00000`, unless a note says otherwise. Addresses are in the first of the two flash-bank copies; the second sits `0x70000` higher with address constants relocated.

| Note | Subject |
|---|---|
| [package-container.md](package-container.md), [coldfire-package-sections.md](coldfire-package-sections.md), [boot-memory-base.md](boot-memory-base.md) | the seven-package container, the reset region, the loader candidates, RAMBAR and MBAR values |
| [../l1-image-layout.md](image-layout.md) | the 1.44.0 container and its ColdFire packages 0, 1, 2, 3 and 5 side by side: headers, flat memory map, banks and records, the reset loader's bank selection, what the host updater writes and where, and what a C-brick needs to boot package 0 or 2 |
| [pre-container-layout.md](pre-container-layout.md) | the six early L1 images (6.5.9 to 6.5.15): one ColdFire package per file, the header words, where each vector table starts, and the early request dispatch |
| [arm-linux-packages.md](arm-linux-packages.md) | the two ARM Linux L1 packages of each set: kernel and CramFS layout, brick labels and bitstreams, user-space programs by name |
| [arm-programs.md](arm-programs.md) | the ARM Linux L1 helper programs read routine by routine (watchdog, clock output, memory access, FPGA load, reset button, UART tests, request client) and the `l1` program's start-up, FPGA bitstream choice and GPIO device |
| [arm-l1-serial-and-flash.md](arm-l1-serial-and-flash.md), [arm-linux-boot-scripts-and-flash-tools.md](arm-linux-boot-scripts-and-flash-tools.md) | the `l1` program's UART, console, GPIO, flash, JTAG-port, interrupt-task and router-frame layers, and the Linux image's network scripts, MTD and NAND tools and MAC-address helper |
| [cbrick-mbus-registration.md](cbrick-mbus-registration.md) | the C-brick personality: identity EEPROM addressing and type names, the expander, monitor and EEPROM tables, bus-select bytes for segments 2 to 13, boot-time traffic and error reactions, the USB window probe |
| [mbus.md](mbus.md), [mbus-environment-task.md](mbus-environment-task.md), [environment.md](environment.md), [board-records.md](board-records.md), [env-reset-debug.md](env-reset-debug.md) | the M-Bus master, EEPROM records, the environment task and its reset countdown |
| [usb-strings-and-transfers.md](usb-strings-and-transfers.md), [usb-overview.md](usb-overview.md), [usb-control.md](usb-control.md), [usb-bulk.md](usb-bulk.md), [usb-descriptor-fields.md](usb-descriptor-fields.md), [usb-descriptors-routing.md](usb-descriptors-routing.md), [usb-controller-registers.md](usb-controller-registers.md), [usb-lower-controller.md](usb-lower-controller.md), [usb-timeouts.md](usb-timeouts.md) | the USB device side |
| [pre-container-layout.md](pre-container-layout.md) | the six L1 images before the container layout (6.5.9 to 6.5.15): header words, vector table offset, the two bank records of each image |
| [mcf5206e-peripherals.md](mcf5206e-peripherals.md) | the on-chip peripherals and external devices the code touches besides the M-Bus and USB |
| [personalities.md](personalities.md) | the other four ColdFire packages of 1.44.0 against the Origin 300 one: supported-brick strings, how the brick-type character is chosen, request catalogue, linked board modules |
| [personality-board-init.md](personality-board-init.md), [personality-base-package.md](personality-base-package.md) | how each package turns the brick-type character into board code (dispatchers and descriptor blocks), and the generic 1 MB package read routine by routine against the Origin 300 one |
| [personality-2mb-packages.md](personality-2mb-packages.md) | the three 2 MB packages routine by routine against the Origin 300 one: which changed pairs are real code differences, and the board modules each adds or drops |
| [rs422-router.md](rs422-router.md) | the packet router, the node UART link and the RS-422 peer links |
| [power-reset-cbrick.md](power-reset-cbrick.md) | the C-brick's `power`, `reset`, `*`, `ctc`, `cti` and `l2` commands and automatic power-up: where a request addressed to an L2 goes with no L2 attached, whether a plain command reaches another C-brick, the local power-up and power-down sequences with their delays, the order of `* power up`, and when a first `pwr d` defers |
| [../l1-bedrock-handoffs.md](../../../platforms/bricks/l1-bedrock-handoffs.md) | the ordered L1 to Bedrock hand-offs of the C-brick and Origin 300: reset, soft-reset and NMI sequences with delays and expander bytes, the double reset during power-up, front-panel triggers, the power-state notice |
| [chunked-request-frames.md](chunked-request-frames.md) | the processor-link chunk codes `0x3001` to `0x3003` on the request side: frame layout, slot table, segment checks, error replies, the task 21 chunked reply |
| [packet-chunks-and-l2-attach.md](packet-chunks-and-l2-attach.md) | the chunked request codes `0x3001` to `0x3003` and their reassembly, the auto power-up task and what an L2 attach changes, and the answer to request `0x0ffff00e` |
| [board-boot-contract.md](board-boot-contract.md), [quad-uart-and-serial-links.md](quad-uart-and-serial-links.md), [flash-nvram-and-image-records.md](flash-nvram-and-image-records.md) | what the processor and board must provide: reset order, chip selects, watchdog, the four-channel UART and port settings, flash commands and sector map, settings storage, clock-calendar, image records |
| [serial-number-requests.md](serial-number-requests.md) | the brick-serial and system-serial requests (`0x0004`, `0x0008`), what the L1 stores and answers, the system serial format rules and where they apply, the brick serial source on the IP53 and IP59 bricks, and what the PROM and the IRIX kernel do with the strings |
| [ip45-io-expanders.md](ip45-io-expanders.md), [ip45-monitor-registers.md](ip45-monitor-registers.md), [settings-record.md](settings-record.md) | the I/O expander descriptors and what the firmware writes and tests on them, the bus switch segments, the monitor chip register reads, and the map from every setting number to its bytes in the settings window |
| [power-reset-ibrick-rbrick.md](power-reset-ibrick-rbrick.md) | what `power up`, `power down` and `reset` drive on the I-brick and R-brick, the state at mains, and which link reaches each |
| [power-up-reads-restart-and-polling.md](power-up-reads-restart-and-polling.md) | what a C-brick reads between its supply enables and the reset release (nothing it waits on), the supply enable bits and order of the I-brick family and the R-brick, whether the expander outputs are kept or re-applied after an L1-only restart, and the expander poll interval |
| [settings-window-blank-image.md](settings-window-blank-image.md) | the settings window layout and its byte-sum rules, what start-up does with a blank, erased or damaged window, the default values it writes, and how to build a valid window with a chosen rack, bay, partition and brick type |
| [request-code-names.md](request-code-names.md) | the `L1P<n>_CMD_CODE_*` header names of the request codes, read against package 1's handlers and their tasks |
| [console-command-tables.md](console-command-tables.md) | the console command tables of all five ColdFire packages: entry layout, argument kinds, registration, hidden entries, the brick-type test, and the curated table [command-table.tsv](command-table.tsv) |
| [env-fan-power-margin.md](env-fan-power-margin.md) | which bricks accept `env`, `fan`, `power`, `margin` and `pbay`, the output formats, the supply limit rule and nominal-voltage table, and the sensor tables each board personality registers (read from the 2 MB package) |
| [debug-console.md](debug-console.md), [debug-task.md](debug-task.md) | the text command task and the `debug` command |
| [console-messages.md](console-messages.md), [console-message-roles.tsv](console-message-roles.tsv) | every console and log text the ColdFire package passes to its two print routines: sinks, severity, the call sites in [inventory/console-messages.tsv](../../../evidence/inventory/console-messages.tsv), and the curated fault messages |
| [unclassified-regions.md](unclassified-regions.md), [unguarded-indexed-anchors.md](unguarded-indexed-anchors.md) | what is known about code and data the linear listing cannot place |
| public evidence: [ip45-board-fitment.md](ip45-board-fitment.md), [ip45-mbus-public-fitment.md](ip45-mbus-public-fitment.md), [ip45-mbus-public-hardware.md](ip45-mbus-public-hardware.md), [mbus-ds1780-candidate.md](mbus-ds1780-candidate.md), [usb-controller-public-hardware.md](usb-controller-public-hardware.md), [usb-bus-public-evidence.md](usb-bus-public-evidence.md), [usb-l2-revision.md](usb-l2-revision.md), [usb-public-host-drivers.md](usb-public-host-drivers.md) | what published manuals, data sheets and posts say about the board; hints, not findings, each with the software route that would settle it |

Curated tables, read by `retools/tools/sysco/sysco_tables.py` (every row carries a grade and a locator):

| Table | Rows |
|---|---|
| [functions.tsv](functions.tsv) | names and roles for routine and table addresses |
| [request-codes.tsv](request-codes.tsv) | the request catalogue by task and code |
| [io-registers.tsv](io-registers.tsv) | memory-mapped registers and external devices the code touches |
| [protocols.tsv](protocols.tsv) | descriptors, framing, routing and record formats, some with byte checks |
| [packages.tsv](packages.tsv) | one layout row per Linux L1 package: kernel, CramFS and entry counts |

## Listing

[disasm/mcf5206e-1.44.0-fuel-pe-o300/](disasm/mcf5206e-1.44.0-fuel-pe-o300/) holds the linear listing of that body (`l1.lst`: address, instruction bytes, mnemonic and operands), and a per-routine summary of calls, references and table jumps (`l1.sum`). The listing carries no text; `l1.sum` annotates each reference with the short literal it points at. Read longer text from the image in `root/`. The decoder is a 68040-mode M68K decoder; ColdFire-only encodings and data mixed into the text decode as `.dw` words or implausible instructions. Read a listing as a map. `functions.tsv` names are descriptions in our own words, not the firmware's identifiers.

Package bodies that other sets share byte for byte are listed once in the function table; the listing for other packages and versions is not generated yet.
