# Open questions

Each entry lists what has been tried and the next route. Move an answered question into the note that owns it and delete the entry.

Each entry starts with its route, one of:

- **software now**: the next step reads files already in `root/` (the L1 and L2 images, the host tools).
- **other firmware versions**: the same question in another set already in `root/`.
- **version diff**: the same question against another package in the same container.
- **IRIX host objects**: IRIX-side tools and client code that drive the controllers, read from `root/` once the file is there.
- **MAME ColdFire core**: needs MAME's MCF5206E core or a public reset-state description, neither in hand.
- **missing binary or release**: the file is not in `root/`; the entry names what to obtain.

## Triage (2026-11-10)

20 entries after five were answered and moved into their notes (the EEPROM retry question, the USB guard word, the IO8 record refresh, the brick-type setting writers and the M-Bus clock). The L2 bullet that held eight questions is split into eight entries, one per question.

| Route | Entries |
|---|---:|
| software now | 13 |
| version diff | 2 |
| other firmware versions | 1 |
| IRIX host objects | 1 |
| missing binary or release | 3 |
| MAME ColdFire core (as primary route) | 0 |

Software-now list, in the order to take them:

4. Monitor register offsets, including reads of `0x3e` and `0x3f`.
6. Readers of bank record word 7.
7. Base-relative accesses in the MCF5206E routines not yet scanned, including timers.
8. USB match rules and identity in the L2 kernel's `sgil1` driver.
9. Writer of the L1's 0x70-byte module table (pointer at `0x30000aa4`) and of its identity word `0x30000080`: which rack and bay a fresh L1 reports to an L2.
10. L2 kernel entry path and board setup.
11. L2 boot loader.
12. L2 daemon request handlers outside task 10.
13. L2 router library, beyond the packet builder, writer and free routines.
14. L2 display and touch protocol in `l2gui`.
16. Memory-window controller behind ports `0x540`, `0x548`, `0x760`.
17. Consumer of CMOS byte `0x7e` bit 7.
18. Meaning of the `FLASH` reply `STATE` fields and the serial-port flag bits.

## Other versions

- **Route: version diff.** Which request codes does each non-Origin-300 ColdFire package add or drop? Partly answered: the general task's add and drop lists for packages 3 and 5 are in [l1/personalities.md](l1/personalities.md#request-catalogue-of-the-general-task), and packages 0 and 2 have the same 39 codes as package 1. From 1.44.0 to 1.48.x the bodies of packages 0 to 3 differ only in strings, the bank record and its table, so their dispatchers do not change ([version-diffs.md](version-diffs.md)). Still open: the other dispatchers (command loop, flash, peer forwarding) and codes above `0x7fff` across the older sets and package 5, which [version-diffs.md](version-diffs.md) does not compare. Step: run the same table-shape search on those dispatchers in packages 0, 2, 3 and 5 of 1.24.8, 1.32.15 and 1.44.0.

## L1 hardware seen by the code

- **Route: software now.** Which MCF5206e on-chip peripherals does the code program beyond those in [mcf5206e-peripherals.md](l1/mcf5206e-peripherals.md)? Premise partly stale: the chip selects, interrupt controls, watchdog, quad UART and the timer 1 and timer 2 reference writes are read in [l1/board-boot-contract.md](l1/board-boot-contract.md). Still open: the base-relative accesses in the remaining routines of the first-bank listing, tabulated by offset. Step: scan the listing for accesses relative to the base stored at `0x300002e8`; add the timer routine's remaining writes and the on-chip UART 1 driver to the table.
- **Route: software now.** The fitted USB device-controller part is not named by any image. The probe is USS-820-shaped (revision at offset 0x18, scratch at 0x1b, accepted revisions 0x10, 0x11, 0x13). Premise corrected: the L2 kernel's `sgil1` driver is the host side of the link and sees the controller only through USB descriptors, so its match rules identify the L1 device (vendor and product, class triple), not the controller silicon; reading them cannot name the part ([l1/usb-public-host-drivers.md](l1/usb-public-host-drivers.md) already holds those checks). Step: compare the register offsets, accepted revisions and write sequences in [l1/usb-controller-registers.md](l1/usb-controller-registers.md) with public USS-820-family register maps and record any match as a hint. The part stays unknown unless an image names it.
- **Route: software now.** Which monitor part answers at M-Bus addresses `0x2c` to `0x2e`? The initialisation routine (`0xffe6836c`) writes offsets `0x40`, `0x43`, `0x44` and `0x47`, which agree with the DS1780 register map ([l1/ip45-monitor-registers.md](l1/ip45-monitor-registers.md), [l1/mbus-ds1780-candidate.md](l1/mbus-ds1780-candidate.md)). Premise corrected: `0xffe6904c` is not a reader. It writes one data byte (plus a zero byte) to register 0, 2 or 3 of the device at the descriptor's wire address and rejects any other register (`0xffe69064`–`0xffe69074`, `0xffe690cc`); its wrappers at `0xffe690f8` write registers 2 then 3. Which device that is, and whether it is the same part as the monitors, is unread. Step: tabulate every offset the monitor reader (`0xffe69844`) and the temperature path (`0xffe5e1bc`) touch, compare with the DS1780 and LM78 maps, and search the listing for reads of `0x3e` and `0x3f`. The `moveq` loads of those values in the listing are not yet tied to a monitor read.

## Images not yet read

- **Route: version diff.** L1 packages 0, 2, 3 and 5: [l1/personalities.md](l1/personalities.md) compares the 1.44.0 ones with the Origin 300 package (layout, personality selection, request codes, module list). Not read: the board devices each module drives (I/O addresses, expected replies), the other sets' versions of these packages, and the added request handlers other than 0x0012. Step: run the register scan used for package 1 over each module's device accesses.
- **Route: software now.** What is word 7 of the bank record? Premise corrected: the record starts at `0xffe7fd1c` in package 1, so word 7 is at `0xffe7fd38`, not `0xffe7fd24` (that address is word 2). Word 7 is the balancing word of the 12-word zero sum the loader checks ([l1/flash-nvram-and-image-records.md](l1/flash-nvram-and-image-records.md)). Checksum searches over the bank cannot find it, so they are dropped. Open: whether any code reads word 7 as data. Step: search the listing for reads at offset `0x1c` from each package's record.
- **Route: software now.** Whether a fresh L1's settings record can hold a non-zero rack or bay. The table writer (`0xffe3132c`) and the identity word's writer (`0xffe78314`) are read and take rack and bay from settings 1 and 2 ([l2/config-request-reply.md](l2/config-request-reply.md)); a zeroed fresh record gives rack 0, bay 0. Open: the settings version-upgrade steps (jump table `0xffe5a858`) and the board initializer `0xffe55254`, for stores into the first record's rack and bay bytes.
- **Route: missing binary or release.** The L2 PROM's partition fetch. The PROM that sends task 10 requests is not in any L2 image read ([l2/partition-task.md](l2/partition-task.md)), and the IP35 PROM image that would reach this tree is not in `root/`. Narrowed: the request shape and the published reply checks fit task-10 code 1 (brick in, partition out) and nothing else, and the reply is one 36-byte segment ([l2/partition-record.md](l2/partition-record.md)); the code halfword the PROM actually sends is still unknown. The L1 side can be read now: the requests that forward to task 10 (task 5, in [l1/rs422-router.md](l1/rs422-router.md)).
- **Route: software now.** The router library beyond the chunk frame and the packet builder, writer and free routines ([l2/router-packet.md](l2/router-packet.md)). Open: the connection, event and request-queue code. Step: read those routines in `libRouter` and `libIRouter` from the L2 tree.
- **Route: software now.** The L2 kernel's entry path and board setup. The inflated kernel is a flat PowerPC binary whose first word is `mr r31,r3` ([l2/image-layout.md](l2/image-layout.md)). Step: read the entry path and the board setup, and find what the loader passes in `r3`.
- **Route: software now.** The L2 daemon's request handlers outside the partition task ([l2/daemon.md](l2/daemon.md); task 10 is in [l2/partition-task.md](l2/partition-task.md)). Step: list the handlers from the daemon ELF in the L2 tree and read them.
- **Route: software now.** The display and touch protocol in `l2gui`. `l2gui` maps `/dev/fb0` and opens `/dev/ttyS3`, whose touch role is inferred ([l2/daemon.md](l2/daemon.md), [l2/rootfs.md](l2/rootfs.md)). Step: read the reads and writes on `ttyS3` and the frame-buffer ioctls in `l2gui`, and name the protocol from them.
- **Route: IRIX host objects.** The text-command routine that `l1cmd` and `l2cmd` share, and the update requests `flashsc` sends, are read in [host-tool-options.md](host-tool-options.md) (request and reply of the text command in full; the status, erase, program and validate requests of `flashsc`'s three update routines by call shape). Still open: the controller side of requests `0x73` and `0x74` (handlers `0xffe3c4c2`, `0xffe3c41e`), the argument layout of the `0x72` reply as `flashsc` consumes arguments 6 to 12, the type 3 (inventory record) reply format behind `FRUDumpAll`, the unit of the wait argument, and the code behind each other `flashsc` option. Step: read the two handlers in the Origin 300 package, then `FlashUseNewestImage` and the option parser in `main` (`0x10007eb0`).
- **Route: software now.** What is the memory-window controller behind ports `0x540`, `0x548` and `0x760` in the MMSC image? Tried: both the flash module and the battery-backed store drive these three ports (flash: `0x548 <- 0xff`, page byte on `0x760`, `0x540 <- 0xff`; SRAM: `0x548 <- 0`, `0x760 <- 0x81`, `0x540 <- 0`), the page tables and values are read in full ([mmsc/flash-nvram-download.md](mmsc/flash-nvram-download.md)), and no string or symbol in the three images names the part. Step: look for the board-support routines that write the same ports during start-up (the `0x108000` entry path) and the I/O range the PC BSP reserves.
- **Route: missing binary or release.** What reads CMOS byte `0x7e` bit 7 (the downloader flag the `DOWNLOADER` command and the flash console state set and clear)? Searched in every distinct MMSC image (eight): only the two read-modify-write routines select that index, and the entry path reads no CMOS port, so the consumer is not in any shipped image ([mmsc/flash-nvram-download.md](mmsc/flash-nvram-download.md)). Step: obtain the MMSC boot ROM or a published description of its boot sequence; nothing else can settle it.
- **Route: missing binary or release.** What did NVRAM layout versions 1, 3 and 4 store (object count and total)? Versions 2 (6.4 builds), 5 (6.5, 6.5.2) and 6 are read and share one thirteen-object table ([mmsc/flash-nvram-download.md](mmsc/flash-nvram-download.md)). No held image writes version 1, 3 or 4. Step: an MMSC image older than 6.4 or built between 6.4 and 6.5.
- **Route: software now.** What do the console `STATE BS FS FP M` fields of the `FLASH` reply and the flag bits of the serial-port records (object 4) mean? Tried: both are read as values ([mmsc/flash-nvram-download.md](mmsc/flash-nvram-download.md)); their use by `flashmmsc` and by the console task is not traced. Step: follow the reply fields through the console routine and the host tool's parsing of the reply.

## MSC and ELSC host side

- **Route: missing binary or release.** What does the controller answer to each request in [elsc-host-requests.md](elsc-host-requests.md)? The words of `pwr q`, `rsw` and `nic`, the two words of `pwr r` and what `fl` and `ok` stand for there, the reply `err pwr_state`, and whether an idle `get` returns an empty reply are fixed only on the host side. Route tried: the MMSC image's MSC client, which speaks the serial dialect of the same controller and sends only `fsc`, `ech`, `mod`, `pas` and `pwr <word>` ([mmsc/msc-client.md](mmsc/msc-client.md)); it does not carry these replies. Step: obtain the MSC (ELSC) firmware, which no scanned medium ships.
- **Route: missing binary or release.** The kernel-side callers of the driver routines. `elsc_rpwr_query` reads a byte at the absolute address -15069 that no relocation names, `elsc_command` takes a second argument that only `elsc_process` sets to 1, and the byte at slave 0x20 (`elsc_ust_read`, `elsc_ust_write`) has no stated meaning. The IP27 kernel is not in `root/`, so the callers are not readable; obtain the IP27 kernel image of a 6.5 release.

## Media

- **Route: missing binary or release.** Other IRIX releases may carry L1 and L2 versions between 1.24 and 1.44. Premise stale: the overlay discs of 6.5.20 to 6.5.29 are already scanned, and their controllers carry 1.20 to 1.40 ([firmware-versions.md](firmware-versions.md), [IRIX_FILES.md](../IRIX_FILES.md)). Still open: the overlay discs of 6.5.19 and earlier, the customer diagnostics discs, and any patch newer than SG0007149. Step: obtain those media and list `field_diags_sysco` on each, per [IRIX_FILES.md](../IRIX_FILES.md).

## Everest host side

- **Controller CPU and firmware (Challenge and Onyx).** Route: missing binary or release. No tool on the media scanned flashes or carries the controller firmware; the driver and daemon give only the wire protocol ([everest/](everest/README.md)). Routes tried: strings and sections of `sysctlrd`, the three `sysctlr.o` and `scmsg`; the man pages. The IP19 standalone diagnostic of the 5.2 and 5.3 diagnostics discs carries only a client of the controller (serial number get and set), no controller code ([everest/prom-client.md](everest/prom-client.md)). Next: the IP21 diagnostic of the 6.0.1 disc and the power-board diagnostics, if any exist.
- **Meaning of the length-mode answers and of the `N` and `06Y` replies.** Route: software now. The framing is read; that `Y` means length mode on is inferred ([everest/daemon-wire.md](everest/daemon-wire.md)). The IP19 standalone diagnostic was read and never sends the `C` item ([everest/prom-client.md](everest/prom-client.md)); it settles only that plain frames work from power-up. Next: the IP25 and IP21 CPU PROM, if one is obtained.
- **Producer of the `syssgi` 102 record.** Route: missing binary or release. The consumer, `check_inventory`, and the slot record are read ([everest/inventory-record.md](everest/inventory-record.md)); the kernel handler that fills the buffer is not in `root/`. Step: obtain an IRIX kernel image for IP19, IP21 or IP25 that carries selector 102.
- **Effect of the `scmsg` short write.** Route: unknown. The tool writes four bytes fewer than it frames ([everest/scmsg.md](everest/scmsg.md)); the controller side is not available. The short write began in IRIX 5.1.1, with the length digits; the 5.0 tool wrote a complete frame ([everest/early-builds.md](everest/early-builds.md)).
## MMSC rack election

- **Route: software now.** What are the fan warning speeds, the temperature advisory, critical and fault limits and the supply type index in the L1 board tables? Which supply type each board record carries, the fan warning speeds and the temperature advisory, critical and fault limits are fields of the registration tables ([l1/env-fan-power-margin.md](l1/env-fan-power-margin.md)); the field layouts were not decoded. Next: read the registration routines `0xffe6be58`, `0xffe6ae18`, `0xffe6aec4` and `0xffe7f080` and the printers `0xffe3064c`, `0xffe307d8` in package 2.
- **Route: other firmware versions.** What are the sensor tables of the Twister, Chimera, PE and `A` personalities and of packages 0, 1, 3 and 5? Counts for those blocks were not read, and routine addresses differ per package.
- **Route: software now.** What fills the module records above 0 when no L2 is attached, and what the lower USB read returns with no host? What a C-brick does with the power-state notice from a peer is answered (it stores the word in the peer's module record; the automatic power-up countdown is the one consumer found): [l1/power-reset-cbrick.md](l1/power-reset-cbrick.md) sections 6 and 7, which also read the C-brick reset, soft-reset and NMI bits. Step: read the callers of the record-apply routine `0xffe31f18` and the task-14 response table `0xffe33dcc` in package 0, and the USB state byte writers. Which Bedrock or XBridge input each reset bit reaches is not in any image (the expanders are labelled `BEDROCK CNTL 1`, `BEDROCK CNTL 2` and `XBRIDGE CONTROL` only); a published chip pin description is the route.


## ARM library NVRAM dispatch

- **Route: software now.** Which taken destinations and effects follow the unresolved
  transfers in `nvramSet` and `nvramGet`?
  The [bounded unknown findings](l1/arm-library-nvram-unresolved-dispatch-1.44.0.md) retain the full selected spans and known-edge observations.
  Tried: native analysis of both full ELF symbol spans, raw instructions and contexts,
  and ELF symbol and relocation metadata. The indexed PC-load destinations remain unresolved. Next: resolve
  the guarded destinations against the image relocation records, then read any newly
  reached paths.

## ARM library console dispatch

- **Route: software now.** Which taken destinations and effects follow the unresolved
  transfer in `L2CfgMSetConsole`?
  The [bounded library observations](l1/arm-library-protocol-config-paths-1.44.0.md) retain the selected full symbol span and known-edge facts.
  Tried: native analysis of that selected span, raw instruction words and bounded
  contexts, and ELF symbol, segment and relocation metadata. The taken destination
  remains unknown. Next: establish the guarded destination interpretation from the
  image relocation records, then read any newly exposed paths.

## ARM router indirect call

- **Route: software now.** Which routine does the indirect call at `0x9f88` in `RouterInput` reach, and what is done with the candidate arguments in `r0` to `r2`?
  The [bounded library observations](l1/arm-library-router-paths-1.44.0.md) retain the selected full symbol span and the call with its target left open.
  The target register is loaded at `0x9f7c` from offset `0xf8` of the object at `[sp-24]`, and no store to that offset appears in the span.
  Tried: native analysis of that span, raw instruction words and bounded contexts, and ELF symbol, segment and relocation metadata. The native list of unresolved transfers is empty, but that does not name the callee. Next: find where the other router routines write that field, then read the routine they store.
