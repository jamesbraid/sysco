# Open questions

Each entry lists what has been tried and the next route. Move an answered question into the note that owns it and delete the entry.

Each entry starts with its route, one of:

- **software now**: the next step reads files already in `root/` (the L1 and L2 images, the host tools).
- **other firmware versions**: the same question in another set already in `root/`.
- **version diff**: the same question against another package in the same container.
- **IRIX host objects**: IRIX-side tools and client code that drive the controllers, read from `root/` once the file is there.
- **MAME ColdFire core**: needs MAME's MCF5206E core or a public reset-state description, neither in hand.
- **missing binary or release**: the file is not in `root/`; the entry names what to obtain.

## Triage (2026-11-01)

25 entries. The L2 bullet that held eight questions is split into eight entries, one per question.

| Route | Entries |
|---|---:|
| software now | 18 |
| version diff | 2 |
| other firmware versions | 1 |
| IRIX host objects | 1 |
| missing binary or release | 3 |
| MAME ColdFire core (as primary route) | 0 |

Software-now list, in the order to take them:

1. Callers of the chunked EEPROM reader: do they retry a failed read.
2. Writers of the guard word at `0x30004d88`.
3. Routine `0xffe48370` and what re-reads the IO8 record.
4. Monitor register offsets, including reads of `0x3e` and `0x3f`.
5. Writers of settings 9 (brick type) and 0x10 (sub-type).
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
- **Route: software now.** The M-Bus clock: the divider is 0x13, but the processor clock is not stated anywhere in the image. Premise partly stale: [l1/board-boot-contract.md](l1/board-boot-contract.md) infers 48 MHz from four constants (inferred), which makes divider 0x13 a 100 kHz bus. [l1/mbus.md](l1/mbus.md) still says the clock is unknown, and the routes listed here omit the UART constant. No image states the fitted oscillator, so the value stays inferred. Step: change the clock sentence in `l1/mbus.md` to cite the board note's inferred 48 MHz, and look in public board and chip documents for a stated processor clock; a figure found there is a hint.
- **Route: software now.** Which monitor part answers at M-Bus addresses `0x2c` to `0x2e`? The initialisation routine (`0xffe6836c`) writes offsets `0x40`, `0x43`, `0x44` and `0x47`, which agree with the DS1780 register map ([l1/ip45-monitor-registers.md](l1/ip45-monitor-registers.md), [l1/mbus-ds1780-candidate.md](l1/mbus-ds1780-candidate.md)). Step: tabulate every offset the monitor readers (`0xffe69844`, `0xffe6904c`) and the temperature path (`0xffe5e1bc`) touch, compare with the DS1780 and LM78 maps, and search the listing for reads of `0x3e` and `0x3f`. The `moveq` loads of those values in the listing are not yet tied to a monitor read.
- **Route: software now.** Do callers of the chunked EEPROM reader (`0xffe6b03c`) retry a failed read? Checked: the listing has absolute calls at `0xffe1686a` and `0xffe39614`, and branch calls from the record readers at `0xffe6b180`, `0xffe6b23c`, `0xffe6b8f2`, `0xffe6b9b0` and `0xffe6ba8a`. Neither the reader nor the controller layer retries ([l1/mbus.md](l1/mbus.md)). Step: read the error branch after each of those call sites.
- **Route: software now.** What does the routine at `0xffe48370` (stored in the pointer at `0x300002f4`, which the MAC EEPROM command calls) do, and does it re-read the IO8 record? Partly answered (established, `0xffe48370` to `0xffe4838f`, read in full): the routine is one call to the EEPROM write routine `0xffe6ab50` with index 1, offset 0, the `0x70`-byte block at `0xffe49926` and a length argument, then returns; it does not read the record back. The sibling routine at `0xffe489b0` makes the same write (call at `0xffe489ec`) between two reads of record 1 (`0xffe489be`, `0xffe48a2c`). Step: read `0xffe489b0` to `0xffe48b5a` for a path that reaches the registered IO8 copy, and `0xffe6ab50` for its failure returns.
- **Route: software now.** Which routine stores the guard word at `0x30004d88` besides the peer initializer? Premise corrected: `0xffe2dc3a` is the store (`move.l d0, $18(a3)`), inside the peer initializer `peer_link_init`, whose entry is `0xffe2dba4` (`0xffe2dc0a` is the `movea.l #$30004d70, a3` in its middle, reached by a branch); `0x30004d70 + 0x18` is `0x30004d88`. Step: the other users of the record pointer `0x30004d70` in the listing (loads at `0xffe2e358`, `0xffe2e5d4`, `0xffe2e894`, `0xffe2eab8`, `0xffe2f0cc`, `0xffe2f31a`, `0xffe2f482`, `0xffe2f4ce` and `0xffe31d0c`; the only absolute reference to `0x30004d88`, at `0xffe2d23e`, is a read), and the network mode setting routine whose entry is `0xffe533f8`. This decides the USB versus RS-422 catch-all precedence ([l1/usb-descriptors-routing.md](l1/usb-descriptors-routing.md)).
- **Route: missing binary or release.** What does an external read at `0x10000040` return on the fitted board, before the loader has written `MBAR` (the reset-status test at `0xffe00ac8`)? Answered from the image: the test reads offset `0x40` of its first argument `0x10000000` and tests bit 5; bit clear goes to `0xffe00b02`, which clears the retained record's image pointer, and bit set goes to `0xffe00ad2`, which keeps a recorded image that passes the checks ([l1/board-boot-contract.md](l1/board-boot-contract.md)). The loader never writes `MBAR`, so the read reaches the on-chip register only if `MBAR` is valid at reset. Routes tried: the image (above) and MAME's MCF5206E core, which keeps the window off until `MBAR` bit 0 is set (a hint, in the board note). Not in hand: the chip's published reset-state description. Step: read the reset values of `MBAR`, `CSAR0` and `CSMR0` in the public MCF5206E user's manual; the board note's reading that this read lands in flash depends on that decode. The result stays a hint until an image shows the decode.

## Images not yet read

- **Route: version diff.** L1 packages 0, 2, 3 and 5: [l1/personalities.md](l1/personalities.md) compares the 1.44.0 ones with the Origin 300 package (layout, personality selection, request codes, module list). Not read: the board devices each module drives (I/O addresses, expected replies), the other sets' versions of these packages, and the added request handlers other than 0x0012. Step: run the register scan used for package 1 over each module's device accesses.
- **Route: software now.** What is word 7 of the bank record? Premise corrected: the record starts at `0xffe7fd1c` in package 1, so word 7 is at `0xffe7fd38`, not `0xffe7fd24` (that address is word 2). Word 7 is the balancing word of the 12-word zero sum the loader checks ([l1/flash-nvram-and-image-records.md](l1/flash-nvram-and-image-records.md)). Checksum searches over the bank cannot find it, so they are dropped. Open: whether any code reads word 7 as data. Step: search the listing for reads at offset `0x1c` from each package's record.
- **Route: software now.** Who writes persistent setting keys 9 (brick type) and 0x10 (sub-type)? Located in package 1: the setter at `0xffe58758` is called with key 9 at `0xffe5578c`, the end of the brick identity routine (`0xffe55254`), which stores the chosen type character on every path, and at `0xffe146b6` in a command handler that prints a brick-type error string; with key `0x10` only at `0xffe146ec`, in the same handler. The debug handler (`0xffe15214`) calls the setter with key 3 and with a key taken from its parsed arguments (`0xffe154c0`), so it can reach 9 and `0x10` only by that generic path. Step: read the command handler around `0xffe146b6` in full, name the command and its value checks, and write the result in [l1/settings-record.md](l1/settings-record.md), which still describes key 9 only as a fallback.
- **Route: software now.** The L1's module table and identity word. The reply to the L2's configuration request is read ([l2/config-request-reply.md](l2/config-request-reply.md)); the routine that fills the table behind `0x30000aa4` (the field at `+0x60`, the valid word at `+0x00`, the rack and bay words at `+0x08` and `+0x0c`, the strings at `+0x40` and `+0x4c`) and stores `0x30000080` was not found: no instruction names `0x30000080` outside `endpoint_init`. Step: read the callers of the per-port record routine `0xffe31f00` and the board-registration routine `0xffe55254`.
- **Route: missing binary or release.** The L2 PROM's partition fetch. The PROM that sends task 10 requests is not in any L2 image read ([l2/partition-task.md](l2/partition-task.md)), and the IP35 PROM image that would reach this tree is not in `root/`. Narrowed: the request shape and the published reply checks fit task-10 code 1 (brick in, partition out) and nothing else, and the reply is one 36-byte segment ([l2/partition-record.md](l2/partition-record.md)); the code halfword the PROM actually sends is still unknown. The L1 side can be read now: the requests that forward to task 10 (task 5, in [l1/rs422-router.md](l1/rs422-router.md)).
- **Route: software now.** The router library beyond the chunk frame and the packet builder, writer and free routines ([l2/router-packet.md](l2/router-packet.md)). Open: the connection, event and request-queue code. Step: read those routines in `libRouter` and `libIRouter` from the L2 tree.
- **Route: software now.** The L2 boot loader: bytes `0x34` to `0x4a218` of `l2.bin`, with its banner at `0x207c` ([l2/image-layout.md](l2/image-layout.md)). Step: disassemble as PowerPC, find the entry, and find the handoff to the kernel's gzip stream at `0x4a218`.
- **Route: software now.** The L2 kernel's entry path and board setup. The inflated kernel is a flat PowerPC binary whose first word is `mr r31,r3` ([l2/image-layout.md](l2/image-layout.md)). Step: read the entry path and the board setup, and find what the loader passes in `r3`.
- **Route: software now.** The L2 daemon's request handlers outside the partition task ([l2/daemon.md](l2/daemon.md); task 10 is in [l2/partition-task.md](l2/partition-task.md)). Step: list the handlers from the daemon ELF in the L2 tree and read them.
- **Route: software now.** The display and touch protocol in `l2gui`. `l2gui` maps `/dev/fb0` and opens `/dev/ttyS3`, whose touch role is inferred ([l2/daemon.md](l2/daemon.md), [l2/rootfs.md](l2/rootfs.md)). Step: read the reads and writes on `ttyS3` and the frame-buffer ioctls in `l2gui`, and name the protocol from them.
- **Route: IRIX host objects.** The text-command routine that `l1cmd` and `l2cmd` share, and the update requests `flashsc` sends, are read in [host-tool-options.md](host-tool-options.md) (request and reply of the text command in full; the status, erase, program and validate requests of `flashsc`'s three update routines by call shape). Still open: the controller side of requests `0x73` and `0x74` (handlers `0xffe3c4c2`, `0xffe3c41e`), the argument layout of the `0x72` reply as `flashsc` consumes arguments 6 to 12, the type 3 (inventory record) reply format behind `FRUDumpAll`, the unit of the wait argument, and the code behind each other `flashsc` option. Step: read the two handlers in the Origin 300 package, then `FlashUseNewestImage` and the option parser in `main` (`0x10007eb0`).
- **Route: software now.** What is the memory-window controller behind ports `0x540`, `0x548` and `0x760` in the MMSC image? Tried: both the flash module and the battery-backed store drive these three ports (flash: `0x548 <- 0xff`, page byte on `0x760`, `0x540 <- 0xff`; SRAM: `0x548 <- 0`, `0x760 <- 0x81`, `0x540 <- 0`), the page tables and values are read in full ([mmsc/flash-nvram-download.md](mmsc/flash-nvram-download.md)), and no string or symbol in the three images names the part. Step: look for the board-support routines that write the same ports during start-up (the `0x108000` entry path) and the I/O range the PC BSP reserves.
- **Route: software now.** What reads CMOS byte `0x7e` bit 7 (the downloader flag the `DOWNLOADER` command and the flash console state set and clear)? Tried: every image routine that touches ports `0x70` and `0x71` was found; only the two flag routines use that byte. The consumer is boot code that no image read contains, so this is the boot ROM and is listed as absent in [controllers.md](controllers.md). Step: search the `0x108000` entry path of each build for a read of `0x7e`; otherwise the question stays open until a boot image is available.
- **Route: other firmware versions.** What were the object counts and sizes of NVRAM layout versions 1 to 5? Tried: the 2.1, 2.2 and 2.3 images carry only the version-6 table and the five upgrade steps, which work on the version-6 object ids ([mmsc/flash-nvram-download.md](mmsc/flash-nvram-download.md)). Step: read `_nvramObjectInfo` in the 6.4 to 6.5.2 builds already in `root/` (they predate the version-6 table if their upgrade chain is shorter).
- **Route: software now.** What do the console `STATE BS FS FP M` fields of the `FLASH` reply and the flag bits of the serial-port records (object 4) mean? Tried: both are read as values ([mmsc/flash-nvram-download.md](mmsc/flash-nvram-download.md)); their use by `flashmmsc` and by the console task is not traced. Step: follow the reply fields through the console routine and the host tool's parsing of the reply.

## MSC and ELSC host side

- **Route: software now.** What does the controller answer to each request in [elsc-host-requests.md](elsc-host-requests.md)? The words of `pwr q`, `rsw` and `nic`, the two words of `pwr r` and what `fl` and `ok` stand for there, the reply `err pwr_state`, and whether an idle `get` returns an empty reply are fixed only on the host side. Next: read the MSC client in the MMSC image (`mmscfw.bin`, module `elsc`, lane 2 in [controllers.md](controllers.md)), which speaks the same set from the controller's side.
- **Route: missing binary or release.** The kernel-side callers of the driver routines. `elsc_rpwr_query` reads a byte at the absolute address -15069 that no relocation names, `elsc_command` takes a second argument that only `elsc_process` sets to 1, and the byte at slave 0x20 (`elsc_ust_read`, `elsc_ust_write`) has no stated meaning. The IP27 kernel is not in `root/`, so the callers are not readable; obtain the IP27 kernel image of a 6.5 release.
- **Route: version diff.** Whether the instruction streams of `elsc.o` in 6.5.5, 6.5.6, 6.5.9 and 6.5.10 behave like the 6.5.7 copy beyond the identical routine, string and callee lists ([elsc-host-requests.md](elsc-host-requests.md#versions)). Next: compare the routines whose sizes differ in 6.5.5 (`_elsc_hbt`, `_elscuart_flush`, `_elscuart_puts`).

## Media

- **Route: missing binary or release.** Other IRIX releases may carry L1 and L2 versions between 1.24 and 1.44. Premise stale: the overlay discs of 6.5.20 to 6.5.29 are already scanned, and their controllers carry 1.20 to 1.40 ([firmware-versions.md](firmware-versions.md), [IRIX_FILES.md](../IRIX_FILES.md)). Still open: the overlay discs of 6.5.19 and earlier, the customer diagnostics discs, and any patch newer than SG0007149. Step: obtain those media and list `field_diags_sysco` on each, per [IRIX_FILES.md](../IRIX_FILES.md).

## Everest host side

- **Controller CPU and firmware (Challenge and Onyx).** Route: missing binary or release. No tool on the media scanned flashes or carries the controller firmware; the driver and daemon give only the wire protocol ([everest/](everest/README.md)). Routes tried: strings and sections of `sysctlrd`, the three `sysctlr.o` and `scmsg`; the man pages. The IP19 standalone diagnostic of the 5.2 and 5.3 diagnostics discs carries only a client of the controller (serial number get and set), no controller code ([everest/prom-client.md](everest/prom-client.md)). Next: the IP21 diagnostic of the 6.0.1 disc and the power-board diagnostics, if any exist.
- **Meaning of the length-mode answers and of the `N` and `06Y` replies.** Route: software now. The framing is read; that `Y` means length mode on is inferred ([everest/daemon-wire.md](everest/daemon-wire.md)). The IP19 standalone diagnostic was read and never sends the `C` item ([everest/prom-client.md](everest/prom-client.md)); it settles only that plain frames work from power-up. Next: the IP25 and IP21 CPU PROM, if one is obtained.
- **`check_inventory` and the `syssgi` 102 buffer.** Route: software now. Only the strings are located; the routine (`0x1000703c`) and the 1,768-byte record it reads are not.
- **Effect of the `scmsg` short write.** Route: unknown. The tool writes four bytes fewer than it frames ([everest/scmsg.md](everest/scmsg.md)); the controller side is not available. The short write began in IRIX 5.1.1, with the length digits; the 5.0 tool wrote a complete frame ([everest/early-builds.md](everest/early-builds.md)).
## MMSC rack election

- **Route: software now.** What is the capability flag bit 0 that makes two MMSCs exchange a 112-byte user record, and what do the first two copied dwords of that record carry? Read so far: the flag is 1 locally and gates the record in [mmsc/remote-link.md](mmsc/remote-link.md). Step: read `userExtractUserInfo`'s source fields (user record +0x18, +0x1c) through their uses in the console code.
- **Route: software now.** Do the 2.1 and 2.2 builds (IRIX 6.5.5, 6.5.6) and the 6.4 builds elect the same way? Only 2.3 was read ([mmsc/rack-election.md](mmsc/rack-election.md)). Step: locate the same routines by symbol name in those images and compare frame version, type table and the 2-second timer.
- **Route: software now.** How long is a clock tick on the MMSC (`sysClkRateGet`), which fixes the 3-tick reply delay and the per-rack reboot stagger? Step: read the clock set-up in the board support code of the image.
- **Route: software now.** In the L2 power queue, what is the order of the brick table inside a class, what does `ver check` do, and can the reply-timer value change? The queue, sender and both modes are read ([l2/power-queue.md](l2/power-queue.md)); the builder of the brick table (`0x1001e554` in `bin/l2`), the `ver check` request (`0x10010aac`) and any store to the 15-second reply-timer word (`0x10043c70`) were not read in full. Next: read those three, then compare the entry `0x10013ba8` in L2 1.24.1, 1.32.4 and 1.48.0.
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

## MMSC unresolved indirect jump

- **Route: software now.** Which destinations and effects follow the unresolved
  transfer in `_csSysIn` in `6.5.6/usr/cpu/firmware/mmscfw.bin`?
  The [bounded unknown finding](mmsc/unresolved-indirect-jump-6.5.6-observations.md) preserves the full selected symbol window and known-edge observations.
  Tried: selected full symbol span, native known-edge analysis, a.out symbol metadata
  and bounded contexts. The indirect destination remains unresolved. Next: resolve
  the guarded destination set from the exact image, then read any newly reached paths.

## ARM library console dispatch

- **Route: software now.** Which taken destinations and effects follow the unresolved
  transfer in `L2CfgMSetConsole`?
  The [bounded library observations](l1/arm-library-protocol-config-paths-1.44.0.md) retain the selected full symbol span and known-edge facts.
  Tried: native analysis of that selected span, raw instruction words and bounded
  contexts, and ELF symbol, segment and relocation metadata. The taken destination
  remains unknown. Next: establish the guarded destination interpretation from the
  image relocation records, then read any newly exposed paths.

## MMSC interrupt helper continuations

- **Route: software now.** What return state is known beyond the selected
  native `_intUnlock` symbol windows in the five MMSC builds in the linked findings?
  The [bounded window findings](mmsc/port-interrupt-leaf-windows.md) retain each known
  branch destination outside its own view and its boundary fallthrough.
  Tried: selected complete native symbol windows, a.out nlist metadata, raw bounded
  neighbor contexts and qualified caller records. Those own-window models contain
  no exit. Next: justify an extended view using independent entry boundaries, then
  rerun exact native analysis before assigning returns or effects outside the stated
  views.
