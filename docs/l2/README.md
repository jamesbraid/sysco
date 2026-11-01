# L2 notes

Reading notes for the L2 system controller, the PowerPC MPC8xx board that runs an embedded Linux 2.4.7 kernel with a read-only cramfs. Everything is read from firmware 1.44.0 (IRIX 6.5.30 `l2.bin`) unless a note says another set. The other three sets (1.24.1, 1.32.4, 1.48.0) were carved with the same script; only file sizes and hashes were compared across them.

| Note | Subject |
|---|---|
| [image-layout.md](image-layout.md) | the container, the gzip kernel, the cramfs and its 512-byte offset bias, versions, the missing `libscan` |
| [rootfs.md](rootfs.md) | the file tree, the boot sequence, the device nodes, differences between sets; the full listing is [rootfs-1.44.0.tsv](rootfs-1.44.0.tsv) |
| [daemon.md](daemon.md) | the `l2` daemon: options and connection modes, service ports, serial and USB links to L1 controllers, the SGI L1 driver's ioctls, flash access, NVRAM, VPD records, the display |
| [partition-task.md](partition-task.md) | task 10 (`l2part`): codes 1 to 8, the database record, flash location, and the chunked frame with codes `0x3001` to `0x3003` |

Curated tables (rows carry a grade and a locator; they are not yet read by `scripts/sysco_tables.py`):

| Table | Rows |
|---|---|
| [devices.tsv](devices.tsv) | device nodes the L2 programs open |
| [ioctls.tsv](ioctls.tsv) | ioctl requests with direction, argument size and the code that issues them |
| [request-codes.tsv](request-codes.tsv) | the task-10 request catalogue and the chunk frame codes |

## Tools

- `scripts/l2_extract.py l2.bin OUTDIR [--list FILE]` carves the kernel and the cramfs with the standard library only.
- `scripts/l2_list.py ELF [--out FILE]` makes the annotated PowerPC listing the notes cite (needs capstone and pyelftools, in the toolchain image). It resolves lazy-PLT and text-relocation targets and constants built from `lis` plus `addi`. A branch to a stripped local routine appears as `sub_<address>`.

## Running the user space without hardware (a hint, not run)

All L2 programs are 32-bit big-endian PowerPC Linux executables (`ET_EXEC` at `0x10000000`, interpreter `/lib/ld.so.1`, glibc 2.1.3, library path `/stand/sysco/lib` in the daemon's `DT_RPATH`). A user-mode PowerPC process emulator, the kind that runs one foreign-architecture Linux binary and forwards its system calls to the host, can run them with the carved `rootfs/` directory as the library prefix. Facts from the images that bear on that, all established above:

- Kernel dependence: the daemon needs `/dev/sgil1_*` (USB mode) or a TCP or serial peer for the L1 link, and `libdrivers` maps flash through `/dev/mem`. With `-noconn` (connection mode 0) it makes no L1 connection, but `nvramInit` and the flash-version check still call `flash_read` through `libdrivers`, whose `flash_init` fails without `/dev/mem` at `0xfe000000`. What the daemon does after that failure, in particular whether `nvramInit` falls back to defaults, is not read; so whether `-noconn` reaches its event loop under user-mode emulation is unknown.
- The command clients (`l2term`, `partmgr`, `l2part`, `l2flash`) are TCP clients of the daemon on ports 8000 and 9002 (see [daemon.md](daemon.md)) and could be pointed at a stand-in server. `partmgr` and `l2part` also need the flash through `libdrivers` and the partition database at `0xfe3e0000`.
- `libscan.ppclinux.so` is missing from 1.44.0 (see [image-layout.md](image-layout.md)); the small tools need it in that set, so use the 1.48.0 tree for them or treat them as unrunnable.

No user-mode PowerPC emulator is installed in the toolchain image or on the host that carved these files, so none of this has been run. It is recorded so the route is not rediscovered.
