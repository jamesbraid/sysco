# L2 notes

Reading notes for the L2 system controller, the PowerPC MPC8xx board that runs an embedded Linux 2.4.7 kernel with a read-only cramfs. Everything is read from firmware 1.44.0 (IRIX 6.5.30 `l2.bin`) unless a note says another set. The other three sets (1.24.1, 1.32.4, 1.48.0) were carved with the same script; only file sizes and hashes were compared across them.

| Note | Subject |
|---|---|
| [image-layout.md](image-layout.md) | the container, the gzip kernel, the cramfs and its 512-byte offset bias, versions, the missing `libscan` |
| [rootfs.md](rootfs.md) | the file tree, the boot sequence, the device nodes, differences between sets; the full listing is [rootfs-1.44.0.tsv](rootfs-1.44.0.tsv) |
| [daemon.md](daemon.md) | the `l2` daemon: options and connection modes, service ports, serial and USB links to L1 controllers, the SGI L1 driver's ioctls, flash access, NVRAM, VPD records, the display |
| [libdrivers.md](libdrivers.md) | the flash library, routine by routine: the 4 MiB window, locking, command sequences, erase and program, return codes |
| [libpart.md](libpart.md) | the partition library: header word, checksum, body layout, lookups, brick and rack identifiers |
| [userspace.md](userspace.md) | the contract of the shipped programs: how to unpack each held image, every device node and ioctl with argument layouts, NVRAM, VPD and display access, what each link mode expects at the far end, the USB link as the daemon sees it, and the reply to the partition service requests |
| [config-request-reply.md](config-request-reply.md) | the daemon's recurring configuration request to an L1, the L1's reply, how the reply becomes a route (and why `NO RECEIVER` without it), the request period, and the flashed-version check |
| [../l2-board-contract.md](../../../platforms/l2/board-contract.md) | the board the image expects: the core, boot stages, memory map, chip selects, DRAM program and every device the boot code and kernel touch, with the unknowns and their tests |
| [../l2-l1-conversation.md](l1-conversation.md) | the L2's side of the L1 conversation in order: attach, the recurring configuration request, route building and packet routing, the routed `p <id> pwr d` shutdown, and the task-10 partition-record reply |
| [user-mode.md](user-mode.md) | running `l2` and `l2part` under a user-mode PowerPC emulator |
| [../l2-kernel-interface.md](kernel-interface.md) | the kernel side: boot-loader hand-off, mapped windows, kernel facilities present and absent, the PCI and USB chain, display, reset, and what must hold to register an L1 |
| [partition-task.md](partition-task.md) | task 10 (`l2part`): codes 1 to 8, the database record, flash location, and the chunked frame with codes `0x3001` to `0x3003` |
| [partition-record.md](partition-record.md) | the partition record end to end: the code-1 reply byte by byte, how it meets the boot firmware's reply checks, the database producer in `partmgr`, test vectors |
| [power-queue.md](power-queue.md) | the `power` command: the send queue and its runner, the sender's class order, and the order and spacing of `fast` and `slow` power-up and of power-down |
| [router-packet.md](router-packet.md) | the router packet in memory: segment record, the builder, writer and free routines of `libIRouter` |

Curated tables (rows carry a grade and a locator; they are not yet read by `retools/tools/sysco/sysco_tables.py`):

| Table | Rows |
|---|---|
| [devices.tsv](devices.tsv) | device nodes the L2 programs open |
| [ioctls.tsv](ioctls.tsv) | ioctl requests with direction, argument size and the code that issues them |
| [exports.tsv](exports.tsv) | one row per routine of `libdrivers` and `libpart`, with address, size, role and grade |
| [request-codes.tsv](request-codes.tsv) | the task-10 request catalogue and the chunk frame codes |

## Tools

- `retools/tools/sysco/l2_extract.py l2.bin OUTDIR [--list FILE]` carves the kernel and the cramfs with the standard library only.
- `retools/tools/sysco/l2_list.py ELF [--out FILE]` makes the annotated PowerPC listing the notes cite (needs capstone and pyelftools, in the toolchain image). It resolves lazy-PLT and text-relocation targets and constants built from `lis` plus `addi`. A branch to a stripped local routine appears as `sub_<address>`.

## Running the user space without hardware

[user-mode.md](user-mode.md) is the tested recipe for running the daemon and `l2part` under a PowerPC user-mode emulator, with what they printed and what could not be exercised.
