# L2 user space: the contract the shipped programs expect

This note collects, in one place, what the programs of the L2 root file system ask of the kernel and of the far end of each link: how to unpack the image, every device node and what is done with it, the ioctl requests, how the daemon reaches non-volatile storage, vital-product data and the display, what each of its link modes expects at the other end, how it sees the L1 USB link, and what it replies to the partition service requests. It is written so that someone running the shipped programs under a user-mode PowerPC emulator, or building a stand-in for a node or a link, can work from system-call traces, link traffic and console output alone. The detail behind each row is in the other notes of this directory, linked where used.

Grades as in [../../AGENTS.md](../../AGENTS.md). Unless a row says otherwise, "read" means read in L2 1.44.0 (IRIX 6.5.30). Addresses are virtual addresses inside the named file (the daemon at `0x10000000`, `l2part` and `l2gui` likewise, the libraries from 0; the kernel at offsets of the inflated `kernel.bin`, where virtual address = `0xc0000000` plus offset, established from the printk string references). Routine names are the dynamic-symbol names where the file exports them and `sub_<address>` otherwise.

## 1. The images and how to unpack one

### Which images are held

Thirteen L2 images are held, one per release of the diagnostics product that carries them and two patches. All are the same layout: the `ESTFBINR` container, a boot loader, a gzip stream with a Linux 2.4.7 kernel, then a big-endian cramfs. Sizes and hashes are in [../firmware-versions.md](../firmware-versions.md).

| Held as | L2 version | `bin/l2` bytes | Unpacks with the script | Entries in the tree |
|---|---|---|---|---|
| IRIX 6.5.20 | 1.20.1 | 199,160 | yes | 307 |
| IRIX 6.5.21 | 1.22.0 | 199,116 | yes | 307 |
| IRIX 6.5.22 | 1.24.1 | 208,096 | yes | 308 |
| IRIX 6.5.23 | 1.26.0 | 219,156 | yes | 308 |
| IRIX 6.5.24 | 1.28.1 | 219,484 | yes | 308 |
| IRIX 6.5.25 | 1.30.0 | 219,752 | yes | 308 |
| IRIX 6.5.26 | 1.32.2 | 226,612 | yes | 308 |
| IRIX 6.5.27 | 1.34.0 | 230,024 | yes | 308 |
| IRIX 6.5.28 | 1.38.8 | 244,508 | yes | 308 |
| IRIX 6.5.29 | 1.40.1 | 245,464 | yes | 308 |
| IRIX 6.5.30 | 1.44.0 | 248,344 | yes | 308 |
| patch SG0005762 | 1.32.4 | 226,836 | yes | 308 |
| patch SG0007149 | 1.48.0 | 255,476 | yes | 309 (adds `libscan`) |

Established by running the extractor on each of the thirteen files and counting the rows of the listing it writes. The release a set comes from is the "Held as" column; each file is the product's `/usr/cpu/firmware/sysco/l2.bin` (the install path in [../../IRIX_FILES.md](../../IRIX_FILES.md)).

Which findings below apply to which image:

- Everything in this note was read in full in 1.44.0.
- For the other twelve, only these are checked, by byte patterns: each low half `0x5301` to `0x5307` occurs as the immediate of an `ori r4,r4` in `bin/l2` (the request numbers of section 4), the same seven values occur in the kernel's ioctl handler, the strings `/dev/sgil1_cs`, `SGIL1_ST_READ_DEV_CFG`, `-usbpath`, `-serial`, `-noconn` and `l1_open_usb`, and the value the kernel returns for the revision query (2 in all thirteen; the scan finds the load that follows the compare and reads the constant it names). That is graded **inferred** for the twelve: the same names and constants are present, but the surrounding code was not read.
- The partition service dispatch accepts codes 1 to 8 (`cmplwi 7` after `code - 1`) in 1.24.1, 1.44.0 and 1.48.0 (established, read at `0x10001350` in each).

### Steps

Established (run on all thirteen). Needs Python 3 and nothing outside its standard library.

1. Take `l2.bin` from the product's install path above. Do not strip or alter it.
2. `python3 scripts/l2_extract.py l2.bin OUT --list OUT.tsv`.
3. `OUT/kernel.bin` is the inflated kernel: the first `1f 8b 08` in the file starts one gzip stream (at `0x4a218` in 1.44.0); the script inflates it with a gzip-window decompressor and notes where the stream ends.
4. The cramfs follows the gzip stream: the script searches forward from the stream's end for the cramfs magic (`28 cd 3d 45`, big-endian). Every offset inside this cramfs counts from 512 bytes **before** the superblock, so the script slices the file from `superblock - 512` and reads the root directory entry at `512 + 64`. A reader that counts from the superblock lands 512 bytes early. Stock cramfs tools were not tried; the byte order is big-endian, so a little-endian host's native cramfs reader is not expected to take it.
5. File data is one zlib stream per 4 KiB block, preceded by a table of big-endian end offsets; the script inflates each and trims to the inode's size.
6. Regular files are written with their modes. Symbolic links are created. A device node, FIFO or socket is **not** created; the script writes `<name>.node` beside it containing the mode and the device numbers (cramfs keeps a node's major and minor in the size field: major is `size >> 8`, minor `size & 0xff`). A user running the tree needs those nodes (or stand-ins, see section 3) made from the listing: `OUT.tsv` gives path, kind (`chr`, `blk`, `fifo`), mode and `major,minor`.
7. Check: `OUT/rootfs/etc/l2firmware_version` holds the version in the table above, and `OUT/rootfs/stand/sysco/bin/l2` has the size in the table.

The tree layout, boot sequence and the file groups are in [rootfs.md](rootfs.md).

### What the user-mode runs need besides the tree

All from [user-mode.md](user-mode.md), observed with a Debian-packaged user-mode PowerPC emulator and the 1.44.0 tree: a regular file standing in for `/dev/mem` (sparse, with the flash window at its end filled with `0xff`); an `etc/hosts` mapping the host name to `127.0.0.1`; the emulator's guest root prefix and library-path variable. Without the `/dev/mem` stand-in the daemon dies with a segmentation fault after printing its banner (observed, and explained by reading the start-up version lookup). The flash base the library maps (`0xfc000000`, then `0xfe000000`) fails under that emulator with `EOVERFLOW`; the observed workaround, patching those two words, is in the same note.

## 2. Programs and what starts them

Established from `/etc/inittab` and `rc.sysinit` ([rootfs.md](rootfs.md)): `inittab` starts `l2` with no arguments on runlevels 3 to 5 (respawned, with the console as its standard streams), `l2flash --l2 localhost` on 4 and 5, `l2gui --l2 localhost` and `l2part --l2 localhost` on 5 only; the default level is 5. With no argument the daemon's connection mode is USB (initial value 2 in `.data`). `l2part` is therefore the task-10 service and exists only at level 5.

## 3. Device nodes, and what each program does on each

Numbers are the major and minor in the cramfs. "Operations" are the calls the program makes, from the call sites cited.

| Node | Number | Opened by | Flags | Operations and what they mean | Grade |
|---|---|---|---|---|---|
| `/dev/sgil1_cs` | 180,104 | `l2` USB mode, `0x10003f88` | read-only (0) | One `ioctl 0x40045306` at open (section 4). Then `read(fd, buf, 0x28)` whenever the descriptor is readable, `sub_10004e8c`. The kernel returns `min(count, 40)` bytes, one per device slot in slot order, each **0 or 1**: 1 when that slot's USB device is connected. The node polls readable (`POLLIN\|POLLRDNORM`, 0x41) only after a connect or disconnect since the last read; a read clears that. No `write`. | established (kernel read handler `0x9d21c` to `0x9d32c`, poll `0x9d330` to `0x9d3a4`, change flag set by the connect and disconnect path `0x9d5d4`) |
| `/dev/sgil1_N`, N 0 to 39 | 180,64+N | `l2`, `0x100048d0`, for slots the status read reports present | read-write (2) | `read`, `write`, `poll`, three reset ioctls, two configuration ioctls (section 4). Semantics below. | established |
| `/dev/ttyS0`, `/dev/ttyS1` | 4,64 / 4,65 | `l2`, `0x10008348`, from the table of the two operator ports | `0x82` | Each becomes a command session. `0x82` is `O_RDWR` plus `0x80`, which is `O_EXCL` in the PowerPC Linux flag numbering (`O_NOCTTY` is `0x100`), so the earlier reading "no controlling terminal" in [daemon.md](daemon.md) was wrong; `O_EXCL` without `O_CREAT` has no effect on a character device (inferred from the public flag definitions, not run). The session code reads the port's current settings with `tcgetattr`, saves them, then sets its own with `cfsetospeed`, `cfsetispeed` and `tcsetattr(…, 0, …)` (`0x1000c940` to `0x1000cb88`); the new flags are not transcribed bit by bit (they clear input conversion and set the speed, inferred raw mode). The speed is the session's: console 38400, modem 9600, 19200 or 38400. | established for the calls; inferred for the mode |
| a serial path given with `-serial /dev/…` | any | `l2`, `sub_10005ed0` | read-write (2) | `tcgetattr`, then `tcsetattr(TCSANOW)` with: input flags `5` (`IGNBRK\|IGNPAR` in PowerPC numbering), output flags 0, control flags `0x80008b00` (`CRTSCTS`, `CLOCAL`, `CREAD`, 8 data bits, no parity; the baud bits of the control word are 0), local flags 0, `VMIN` 1, `VTIME` 0, and the two separate speed fields of the C library's record both `0xf` (38400). Raw 8N1 with RTS/CTS flow control. A Linux 2.4 kernel takes the speed from the control word's baud bits, which are 0 here, so the speed the line really runs at is not established by this code (the product does not use this mode: it is a debug path). | established (stores at `0x10005f98` to `0x10005fe0`; the flag names are the public PowerPC definitions) |
| `/dev/fb0` | 29,0 | `l2gui`, `0x1003f8c4` (path from `FRAMEBUFFER` when set) | read-write (2) | `ioctl 0x4602`, then `ioctl 0x4600`, into stack buffers (public frame-buffer fixed and variable screen information; the offsets read match the public layouts: type at +24 and visual at +32 of the fixed record, width at +0, height at +4, bits per pixel at +24, green length at +48 and transparency length at +72 of the variable record). When the visual is 2 (true colour) or 4 (direct colour) the bits per pixel must be 8, 16, 24 or 32, otherwise it prints `Unsupported %d color (%d bpp) truecolor framebuffer` and gives up; the drawing variant is then chosen by the bits per pixel, and for 16 by whether the green length is 5, for 32 by whether the transparency length is 8. Any other visual skips the check and takes a palette-style variant (`0x1003f9dc` to `0x1003fa98`). Then `mmap(0, len, PROT_READ\|PROT_WRITE, MAP_SHARED, fd, 0)` with `len` the value held in the screen record rounded up to a page (`0x1003fb58`; the record's length word is set by the chosen variant's set-up and was not followed, so the value is not stated). The colour-map ioctls `0x4604` (get) and `0x4605` (put) are used only on the palette path (`0x1003fd34` to `0x1003ff24`). | established for the calls and the checks; inferred that the structures are the public Linux ones |
| `/dev/ttyS3` | 4,67 | `l2gui`, `0x10040630` | `0x800` (`O_NONBLOCK`) | `tcsetattr` with control flags `0x8b0d` (`CLOCAL`, `CREAD`, 8 data bits, **9600 baud**), other flags zero, `VMIN` 1 (`0x10040660` to `0x10040674`). Then `read(fd, buf, 4)` per poll (`0x1004078c`). The poll routine returns 2 after a read of 4 bytes, 0 when the read fails with `EAGAIN` (11), and -1 for a read of 0 to 3 bytes or any other error (what the caller does with -1 was not followed). In a 4-byte report only bytes 1 and 2 are used: the first output is `(byte2 & 0xfe) * 16 + 16` and the second `(byte1 >> 1) * 34 + 17` (the mask is bits 1 to 7 of byte 2, from `rlwinm` with shift 4 and mask `0xfe0`), then a pointer event with the value 4 is queued with them (`0x100407bc` to `0x100407fc`). That the outputs are the x and y of a touch panel is inferred (no string says so); the arithmetic is established. Bytes 0 and 3 are ignored. | established for the settings and the read; inferred for the role |
| `/dev/mem` | 1,1 | `libdrivers`, `flash_init` `0x1854` | `0x1002` (`O_RDWR\|O_SYNC`) | `mmap` 4 MiB, `PROT_READ\|PROT_WRITE`, `MAP_SHARED`, at physical `0xfc000000`, else `0xfe000000`. This is the only path to flash, NVRAM and the partition database (section 5). | established ([libdrivers.md](libdrivers.md)) |
| `/dev/rtc` | 10,135 | `hwclock --hctosys` in `rc.sysinit` | n/a | boot-time clock read | established (script text) |
| `/dev/mtd0` to `3`, `/dev/i2c-0`, `/dev/watchdog`, `/dev/ppp` | | no L2 program opens them | | not touched by the daemon, libraries or `l2part` (string search of every file in the tree, repeated here for `i2c` and `vpd` in `l2` and every library) | established |

Besides nodes: a System V semaphore, key `0x0124f0e5`, created `semget(key, 1, 0x3b6)` and used with `semop` to serialise flash access between the daemon and `l2part` ([libdrivers.md](libdrivers.md)); TCP and Unix sockets (section 7); `/tmp/l2ver` (cached version string).

### Semantics of `/dev/sgil1_N` as the program sees them

Read from the kernel driver (`sgil1.c` in the inflated kernel) and from `l1_read_usb` (`0x10006128`) and `l1_write_usb` (`0x10006dec`) in the daemon. Established unless marked.

- **Open.** Fails with `ENODEV` (19) when that slot is not connected. Otherwise allocates one 4 KiB receive buffer and one 4 KiB transmit buffer; out of memory is `ENOMEM` (12). No other failure was seen in the routine (the routine was read to its exits, but the slot's open count is kept in a byte whose use was not followed).
- **Close.** Resets the USB device (`SGI L1 device reset failed on close` is logged if the reset fails).
- **Write.** `write(fd, buf, n)` queues one USB bulk-out transfer and **returns `n` as soon as it is submitted**; it does not wait for the transfer. `n` must be at least 1 and at most 4096 (the transfer limit in the slot record), else `EINVAL`. The kernel copies the bytes, then **overwrites the first two bytes of the transfer with `n` as a big-endian 16-bit value** (`sth` at `0x9de98`) and submits it. Completion is seen by the next call: if the previous transfer is still pending, the next `write` sleeps for up to 300 ticks (`0x12c`; the tick rate is not read, so the time is not stated) for it, returns `-ERESTARTSYS` (512) if a signal arrives, and on a timeout logs `write timeout`, abandons it and fails with `ETIMEDOUT` (110). If the previous transfer finished with an error, the next `write` returns that error without sending and does not clear it (no store seen). A transfer whose completed length differs from the requested one logs `short write` and counts as error `EIO` (5) for the next call. Because a segment's first two bytes already hold its length ([router-packet.md](router-packet.md)), the overwrite changes nothing for packets the daemon builds. Established (kernel write routine `0x9dc48` to `0x9df4c`, completion routines `0x9d7cc` and `0x9d834`).
- **Read.** The descriptor is read-ready (`poll`/`select` bit `POLLIN`) when a receive transfer has completed; `poll` itself starts the next receive transfer if none is pending. `read(fd, buf, n)` returns the **whole completed transfer** (one USB bulk-in transfer, at most 4096 bytes) or fails with `EINVAL` when `n` is smaller than the transfer, leaving the transfer in place for a retry with a larger buffer. With no completed transfer it returns 0 and logs `read not done`; after a failed transfer it returns the error once and goes idle; a zero-length transfer returns 0; after disconnect it returns `ENODEV`; a bad user buffer gives `EFAULT` (14). The daemon always reads into the packet buffer plus `0x10` with the segment size it expects (1024 for an ordinary segment; the reader is [router-packet.md](router-packet.md)).
- **Errors the daemon acts on** (`sub_100060ac`): `ENODEV` or `EINVAL` from a read or write is "device gone": the slot is closed (`[no device]`) and nothing is retried. Any other error increments a per-slot counter (kept at slot-table offsets `+0x40` read, `+0x44` write; the table has 0x48 bytes a slot at `0x100512b8`); when it passes 2 the daemon logs `closing … after multiple read errors` or `write errors`, resets the device (`sub_10004210(fd, 1)`) and closes the slot (`[rd error]`). A successful read zeroes the read counter.
- **Packets larger than one write.** The router may build segments of up to `0x3000` bytes when both address words are of router form (the "local" case in [router-packet.md](router-packet.md)); a write above 4096 bytes would fail with `EINVAL` and the daemon would then close the slot. Whether the daemon ever builds such a segment for a USB device was not followed: open ([../open-questions.md](../open-questions.md)).

## 4. ioctls

All numbers are in PowerPC Linux encoding: the top three bits are the direction (1 none, 2 read, 4 write), the next 13 the argument size, then the type byte (`'S'` = `0x53`) and the request number. A user-mode emulator whose host is not PowerPC must therefore translate or handle them itself; the unknown type is not in a generic table (inferred; not tried). Established for every row from the call sites and the kernel handler (`0x9e280` to `0x9e460`, `0x9d3d0` to `0x9d560`).

| Request | Node | Direction, size | Argument and meaning | Issued by |
|---|---|---|---|---|
| `0x40045306` | `sgil1_cs` | read, 4 | writes a 32-bit big-endian integer, the driver revision. The kernel returns **2** in all thirteen images (section 1). The daemon stores it at `0x10043bc0`; a failed call sets 0. A revision of 0 or 1 makes `-usbpath` fail with a message. | `0x10004018`, `0x10004110` |
| `0xc0095307` | `sgil1_N` | read-write, 9 | the 9-byte device record (below) for slot `index`, where `index` is byte 2 of the argument on input. Slots above 39 or not connected give `EINVAL` (22). Issued only when a path filter is in force and revision is above 1. | `0x1000470c` |
| `0x40095304` | `sgil1_N` | read, 9 | the 9-byte device record of the device behind this node. Issued when the word at `0x10043bbc` (initial value 1) is nonzero; the revision is not tested. | `0x1000494c` |
| `0x20005301` | `sgil1_N` | none | reset the read side (kernel: `SGIL1_RESET_READ`) | `0x10004320`, `0x100062d4` |
| `0x20005302` | `sgil1_N` | none | reset the write side (`SGIL1_RESET_WRITE`) | `0x10004270` |
| `0x20005303` | `sgil1_N` | none | reset the USB device (`SGIL1_RESET_DEVICE`); when the USB reset fails the slot's state word is set to 3 and the error is returned | `0x100042c0` and others |
| `0x20005305` | `sgil1_N` | none | in the kernel it takes the same code path as `0x20005302` (the write reset) and then, because the compare falls through, the read reset as well as the shared tail; the daemon uses it instead of the `5302`/`5301` pair when the revision is above 0. | `0x1000424c` |

An unknown request on either node returns the kernel-internal "no such ioctl" value `-515` (`-0x203`, `0x9d544` and `0x9e42c`), which the C library reports as `ENOTTY`.

The wrapper at `0x100041a8` skips the ioctl and returns 0 when the word at `0x10043bbc` is 0. Its initial value is 1.

**The 9-byte device record** (established from the formatter `sub_1000444c`, which prints it, and from the kernel, which fills it):

| Byte | Meaning |
|---|---|
| 0 | USB bus number |
| 1 | the number `P` of hub ports in the path, at most 6 (the kernel stops the walk after 6) |
| 2 | USB device address on that bus |
| 3 to 3+P-1 | port numbers from the root hub down, one byte each |

The daemon prints it as `b<bus>;p<port>/<port>…;d<dev>` (for example `b1;p2/3;d4`), the name it matches `-usbpath` against and shows in `INFO: opened USB device at <path> (<node>)`.

**The reset decision** (`sub_10004210(fd, also_device)`), established by reading the whole routine: with revision above 0 it issues `0x5305`, then `0x5303` if `also_device` is nonzero; a failure of either returns -1. With revision 0 or 1: issue `0x5302`; if that fails log and issue `0x5303` (return 0 if it works, -1 if not); if it works issue `0x5301`; if that fails log and issue `0x5303` (same returns); if it works issue `0x5303` only when `also_device` is nonzero (-1 on failure). Open calls it with `also_device` 0 (`0x100049dc`); the error paths call it with 1. This corrects [daemon.md](daemon.md), which said revision above 0 issues `0x5305` alone.

## 5. NVRAM, VPD and the front display

**NVRAM.** The daemon holds a 16 KiB image in RAM and reads and writes it **through `/dev/mem`**, as flash: the block is at physical `0xfe3f0000` (old location `0xfe2e0000`), erased and reprogrammed by `libdrivers` on save. There is no NVRAM device node and no I2C access. The format (IPMI-style 8-byte common header, a 0x82c-byte internal-use area, then an event log), the key table and the load and save rules are in [daemon.md](daemon.md#nvram); a stand-in only needs the 4 MiB window with erased flash (`0xff`) as in [user-mode.md](user-mode.md), and with all-ones it creates default contents and saves them. Established.

**Vital-product data.** No program or library of the L2 root file system opens an I2C node, and none contains an EEPROM or VPD string beyond the record codec names (string search of `l2`, `l2part`, `l2gui` and each library, repeated here). The record codec (`FRUUnpackData` and friends) is called by the daemon only to detach and dump records it received (`FRUDetach` at `0x1000b424`, `FRUDumpAll` at `0x1000b44c`, no other call site in `l2`); a brick's VPD reaches the L2 as packet payload from the brick's L1 over the router. The boot loader (not read here) fetches the Ethernet address from an I2C EEPROM, per the kernel strings; user space does not. Established for the user-space statements; inferred that the L1 supplies the records (no request that fetches them was followed on the L2 side; the L1 request catalogue is in [../l1/](../l1/)).

**Front display.** The daemon does not touch the display. `l2gui` is a separate process that registers with the daemon as task 6 and draws into `/dev/fb0` with `mmap` (section 3); touch input arrives on `/dev/ttyS3`. The two `l2dsp` commands in the daemon (lock, debug) set daemon-side state; how it reaches `l2gui` is not followed (it would be a router event between task 6 and the daemon; unknown, in [../open-questions.md](../open-questions.md)). Established for the split.

## 6. What the link modes expect at the far end

The mode is chosen by argument ([daemon.md](daemon.md#starting-and-options)); the word at `0x10043b84` holds it (default 2). The dispatch table is at `0x10027d00`.

| Mode | Argument | What the daemon does | What the far end must do |
|---|---|---|---|
| 0 | `-noconn` | nothing | nothing |
| 1 | `-net` | listens on TCP port `bias + 0x232b` (9003 with bias 0) and on the Unix socket `/usr/tmp/UNIX_%04x` for that number (`sub_10007cc0`, `SocketExportInetConnection`, `SocketExportUnixConnection`), accepts each connection as one L1 port ("network") | connect to that port and speak the segment stream (below) |
| 2 | `-usb` (default) | section 8 | an OHCI-visible USB device as in section 8 |
| 3 | `-serial /dev/…` | open and set raw 38400 8N1 RTS/CTS (section 3), then the handshake below | a serial peer that speaks the handshake and then PPP-framed segments |
| 4 | `-serial HOST [PORT]` | `connect` TCP to HOST and PORT (default 2345), then the same handshake over the socket (`SocketWrite`, `SocketRead`) | a TCP listener that behaves as the serial peer |
| 5 | `-l1sim`, `-l1sim_manual` | starts or awaits a simulator; reuses mode 1's listener | not used on a product |

**Handshake for modes 3 and 4** (read from `sub_10005614` and the code after `0x10005798`; this corrects the shorter account in [daemon.md](daemon.md)): after `PPPInit`, the daemon writes one byte `0x0d`, sleeps 2 seconds, writes one byte `0x18` (the two values are the initialised bytes at `0x10043bac` and `0x10043ba8`: `0d` and `18`), then **reads one byte at a time** until it gets `0x18`; any other byte is read and ignored, a read of 0 or an error is `FATAL: can't confirm L1 protocol`. On `0x18` it sets the socket to no-delay (mode 4 only meaningful; a failure is `can't set delay mode`), registers the port and from then on treats the stream as PPP-framed segments. A write of anything other than 1 byte is `FATAL: can't initiate L1 protocol`. The byte values are established; that `0x0d` then `0x18` is the L1's text-console escape into packet mode is inferred (the L1 side of this sequence is not read here).

**Framing.** Which reader a mode uses is the table at `0x100280a0`, indexed by mode minus 1 (entries decoded): USB uses `l1_read_usb` (`0x10006128`), modes 3 and 4 use `l1_read_ppp` (`0x100066b8`, `PPPRead`), modes 1 and 5 use the socket reader (`0x100067b0`). Established.

- Serial modes (3 and 4, the latter over TCP): HDLC-style async frames as in the public PPP framing standard. The writer (`PPPWrite`, `libsys` `0x9e60`) starts a frame with flag `0x7e`; it escapes a data byte by sending `0x7d` followed by the byte xor `0x20`, for the bytes `0x7e`, `0x7d`, `0x13`, `0x11` and `0x0d` (the compares at `0x9ef4` to `0x9f20`); it keeps a 16-bit frame check that starts at `0xffff`, is updated per byte as `(fcs >> 8) xor table[(fcs xor byte) & 0xff]` with the 16-bit table at `0x142fc` of `libsys` (first entries `0000 1189 2312 329b`, the PPP frame-check table), and is complemented at the end (`0xa110`; the writer's final byte stores are at `0xa120` and `0xa134`, order not confirmed here). The reader (`0xa384`) has the same compares. Established for the flag, escape rule and escape set, the table and the initial value and the complement; the order of the two check bytes and whether address and control bytes precede the data (no compare against them was seen) were not read.
- Socket modes (1 and 5): the reader reads 2 bytes, converts them with `ntohs`, and reads that many bytes minus 2 into the buffer after the 2 length bytes (`0x100067fc` to `0x1000685c`): the first two bytes are a big-endian length that **includes themselves**, and the body follows. Established.
- USB: none; each transfer is one segment (section 8).

**Segments.** What travels is the segment of [router-packet.md](router-packet.md): wire bytes start with a 16-bit byte count (big-endian) and the first segment has bit 7 of byte 5 set, with the segment count in the low seven bits, counting down by one on each continuation to 1 on the last.

## 7. Sockets the daemon serves

For clients such as `l2part`, `l2gui`, `l2flash` and the command-line tools: TCP ports (bias 0) 8000 and 8002 (command clients), 9001 (inferred L2 to L2), 9002 and 9004 (application clients, task registration), 8001 and 8003 (text sessions, "L2>" prompt); plus 9003 in `-net` mode. Established as constants; observed listening in the user-mode run ([user-mode.md](user-mode.md)). On connect the ports 8000, 8002, 9002 and 9004 send an unsolicited 28-byte record and 9001 an 8-byte record (bytes in [user-mode.md](user-mode.md)).

## 8. The L1 USB link as the daemon sees it

Established from `l1_open_usb` (`0x10003eec`), `sub_10004e8c`, `sub_10004680` and the kernel driver.

1. **Control node.** Open `/dev/sgil1_cs` read-only, issue `0x40045306`, register `sub_10004e8c` as the input handler, and call it once at once (`0x10004134`).
2. **Status read.** `sub_10004e8c` reads 0x28 bytes. A read of 0 logs `no devices`; negative logs the errno. Otherwise, for each slot `i` below the byte count, with `s` the byte and `state[i]` the daemon's slot state (table at `0x100512b8`, 0x48 bytes a slot): first pass, for every slot where `s` is 0 and the state is nonzero, set the state to 0 and run the close routine (`sub_10004bf8`, label `[driver]`); second pass, for every slot where `s` differs from the state and `s` is nonzero, set the state to `s` and run `sub_10004680(i)`. Because the kernel only returns 0 or 1, a slot is opened when its byte becomes 1 and closed when it becomes 0.
3. **Open a device** (`sub_10004680(i)`): when `-usbpath` is in force and the revision is above 1, issue `0xc0095307` with byte 2 = `i` to get the record for the slot and match its printed path against the filters (a string prefix match that must end at the end of the string or at `/`); then `open("/dev/sgil1_%d", O_RDWR)` (error: `ERROR: can't open %s, %s`); then, when the flag at `0x10043bbc` is nonzero, `0x40095304` for the record, copy the printed path and the `d`-less path into the slot record, then `sub_10004210(fd, 0)` (a reset with no device reset on revision above 0). It prints `INFO: opened USB device at <path> (<node>)`, records the slot, adds the port to the configuration with type 2 (`L2CfgMAddPort`; a return of -1 is `ERROR: device is already in use.`), schedules a timeout of `0xb71b0` (750,000, unit not read) calling `0x1000d9a8`, and registers `0x100069b0` as the descriptor's input handler.
4. **Receive.** Whenever the descriptor is readable `0x100069b0` runs: it calls the router packet reader with the USB read routine, so each `read` returns one USB bulk-in transfer holding one segment with the layout above; the reader checks the first-segment flag and the down-count rule ([router-packet.md](router-packet.md)). Established for the call path; the byte checks are in the packet reader.
5. **Send.** The packet writer calls `l1_write_usb` once per segment with the segment's wire bytes (`IRouterWritePacket`, [router-packet.md](router-packet.md)); the kernel adds nothing but the two-byte stamp (section 3).
6. **What the daemon sends first.** Nothing was found in the open path: no write follows the open before the first inbound packet (inferred from the open routine's calls; whether a later timer sends a probe is not read). The first requests the daemon makes of an L1 after it appears are not followed (open).
7. **Loss.** A status byte falling to 0, or `ENODEV` or `EINVAL` on an operation, closes the slot; other errors close it after the third in a row (section 3).

## 9. The reply to the partition-record requests

Tasks and codes here are read from `l2part` and its libraries in 1.44.0 (the three versions listed in section 1 share the code table). See [partition-task.md](partition-task.md) for the codes and the database record.

**Where `0x3001` to `0x3003` fit.** The stock boot firmware's partition fetch reaches the L2 as a request to controller type 1, task `0xa`. The codes `0x3001` to `0x3003` are the L1's multi-frame transport on the serial link between that firmware and the L1; the L1 consumes them and forwards an ordinary request. Seen from the L2:

- No L2 program contains a handler for `0x3001`, `0x3002` or `0x3003`. A search of the `ori`, `addi`, `li` and `cmpwi` operands of `l2`, `libmain`, `libRouter`, `libapi`, `libsys`, `l2part` and `partmgr` finds none in 1.24.1, 1.44.0 or 1.48.0; the only occurrences are in `libIRouter`, in the chunk reader and writer for **descriptor streams** (`IRouterConnectOS`, used by the client library when it talks to a device rather than the daemon). Established for the absence in those three versions.
- On those descriptor streams the L2 writes a 9-byte frame header (four zero bytes, the big-endian code `0x3001`, `0x3002` or `0x3003`, one byte, then the big-endian message length) and at most 118 payload bytes per frame ([partition-task.md](partition-task.md)). The L1 serial transport is the same layout with 113 bytes per reply frame (L1 datasheet figure; not re-read here). The two are different code, so the 118 and 113 are not a contradiction. Not on the USB link, where a segment is sent as-is (section 6).
- So a task-`0xa` request that arrives over USB is **not** framed: it is a normal router request whose code field (offset `0x22` of the segment record, `0x12` of the wire bytes) is the request code. What code the boot firmware puts there is **unknown** here (no boot firmware is part of these images); the handler answers codes 1 to 8 and any other code with result -3.

**The request.** Wire offsets are those of a segment's wire bytes (record offset minus `0x10`). The request's first argument descriptor is at record offset `0x24` (wire `0x14`): type in the low three bits of byte `0x24` (0 = integer, value in the word at `0x28`; otherwise the argument is fetched with `IRArg` from the payload), length halfword at `0x26`. The class byte at record `0x16` (wire `0x06`) is 0 for a request; the reply is sent only when it is 0.

**The reply** is built by the library's response path, established from `sn1SCSendRsp` and `IRouterResponseL3` (`libapi` `0xbc80`, `libIRouter` `0x3f6c`) and the common exit of `l2part` (`0x10001aa0`):

| Reply field | Value |
|---|---|
| destination | the request's source word (request record `+0x1c`, wire `+0x0c`) |
| class | 1 (response) |
| code | the request's code, copied |
| sequence | the request's 16-bit sequence number copied (record `+0x14` and `+0x21`) |
| byte at record `+0x13` | `(-a) mod 100` with `a` the status argument; `l2part` always passes 0, so 0 |
| descriptors | the reply record that `l2part` builds on its stack has the same 8-byte shape as a descriptor at its start (a type byte, a byte, a length halfword, a value word at `+4` that holds the result), so the first descriptor is always an **integer result** (type 0, value = result, 0 on success), followed by the argument descriptors below. The count handed to the sender is the number of descriptors including the result. |

| Request code | Descriptors after the result | Result value |
|---|---|---|
| 1 brick to partition | one integer: the partition number, 0 if unassigned | 0 |
| 2 environment of a partition | one integer: the number of variables (the return value of the environment copy, not a byte length); one binary: the block, byte length in the descriptor's halfword (low 16 bits of the length), `NAME`, NUL, `VALUE`, NUL per variable, then two more NULs | 0 |
| 3 whole database | one binary: the database image of up to 65,536 bytes, with the descriptor length as a 16-bit value (an image of exactly 65,536 would wrap to 0; inferred from the halfword) | 0 |
| 4 database timestamp | one integer: the timestamp word | 0 |
| 5 set database | none (count 1) | the validation result negated, or 7 for an older timestamp, or 0; when the request came from a local client (class byte 2) the handler sends an event instead | see [partition-task.md](partition-task.md) |
| 6 set mode | none (count 1) | 0 or -1 to -6 as listed there | |
| 7, 8 | none; code 7 sends no reply at all (the request is parked), code 8 replies to the parked request with a string | | |
| any other | none | -3 | |

This corrects [partition-task.md](partition-task.md) in two places: the descriptor count is 1 (result only) for codes 5 and 6, not 2, and the first integer of the code-2 reply is the variable count, with the byte length in the binary descriptor. Established from the stores at `0x100014a0` to `0x1000151c` (code 2), `0x10001558` to `0x1000158c` (code 4), `0x100015b4` to `0x10001604` (code 3) and the count word at stack offset `0x58`, which is set to 0 at `0x10001308`, then 1 at `0x10001344`, and stored again only by the handlers for codes 1 to 4 (2, 3, 2 and 2 at `0x1000142c`, `0x1000151c`, `0x1000158c` and `0x10001604`; the handlers for 5 and 6 do not store it).

**Layout of a code-4 reply's descriptors** (8 bytes each: type byte, segment-number byte, length halfword, value word): result descriptor `00 s 00 00 00 00 00 00`, timestamp descriptor `00 s 00 00 <timestamp, big-endian>`, where `s` is 0 as `l2part` leaves it and the packet builder writes the segment number (1 in the first segment) over it; that the builder writes it for integer descriptors too was not checked (the builder note speaks of descriptors with a payload), so `s` is 0 or 1. Inferred for `s`; established for the rest.

## 10. Open items

- Which request code and arguments the stock boot firmware puts in a task-`0xa` request. Routes tried: both L2 sides of the framing, every L2 binary for the three codes, the L1 notes. Next: the boot firmware itself, when an image of it is available.
- The order of the two frame-check bytes in the serial framing and whether address and control bytes are sent.
- The daemon's first requests to an L1 after open, and the timeout callback at `0x1000d9a8`.
- Whether a segment over 4096 bytes is ever written to a USB node.
- The kernel's USB match routine and the 9-byte record fill were read to the extent needed here (the match checks a descriptor word equal to `0x065e1234`, one configuration, one interface, a bulk-out then a bulk-in endpoint; the field mapping of that compare is as stated in the L1 notes and is inferred on this side).
