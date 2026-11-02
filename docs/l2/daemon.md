# The L2 daemon: modes, devices, flash, NVRAM and sockets

Read from L2 1.44.0, `/stand/sysco/bin/l2` (248,344 bytes, sha256 `9923b1ee573c28154a012e5510a6f42ad420c956299ff4388f189df464835f32`) and its libraries `libmain`, `libapi`, `libsys`, `libRouter`, `libIRouter`, `libdrivers` under `/stand/sysco/lib/` (listing in [rootfs-1.44.0.tsv](rootfs-1.44.0.tsv)). Addresses are virtual addresses inside the named file: the executable at `0x10000000`, the libraries from 0. All are stripped, so routine names below are either dynamic-symbol names (the libraries export most of their routines) or descriptions of mine, written `sub_<addr>` when only an address names it. `scripts/l2_list.py` regenerates the listings I read.

Grades: established means the instructions and data were read and the statement follows; inferred means a reading from names, strings or partial code.

## Starting and options

`inittab` starts it with no arguments (see [rootfs.md](rootfs.md)). `main` is at `0x10002f68` (established). It parses these arguments (established: the compares and the stores at `0x10003030` to `0x100036f4`):

| Argument | Effect |
|---|---|
| `-net` | connection mode 1: accept L1 controllers over the network (`l1net`, routine at `0x10007cc0`) |
| `-pc` | sets two flags (`0x10043b90` and discovery `0x10043b80`, both 1) |
| `-usb` | mode 2: L1 controllers over USB through the SGI kernel driver |
| `-noconn` | mode 0: no L1 connection at all |
| `-serial <device>` | mode 3 when the next argument starts with `/` (copied to the device-name buffer at `0x10051298`, default `/dev/com2`); otherwise mode 4: the argument is a host name (buffer `0x10051e58`) and an optional port follows, read with `sscanf` into `0x10043b8c` (initial value 2345) |
| `-l1sim`, `-l1sim_manual` | mode 5: start or wait for a software L1 simulator (executable name `../utils/l1sim.linux`, looked up through `WORKAREA` and `HOME`); not used on the product |
| `--copy N` | 0 to 128; sets the port bias `IR_PORT_BIAS` to 10 times N so several daemons can coexist |
| `-discover`, `-nodiscover` | the L2-to-L2 discovery flag (initial value 1) |
| `-usbpath b<bus>;p<port>[/<port>...]` | restrict USB monitoring to those USB paths (repeatable, comma list) |
| `-debug`, `-irtr` (IRouter packet dump), `-nopwrchk`, `-smpenet`, `--mtrace` (malloc trace into `/tmp/l2.mtrace`), `-syncssn` | debug and behaviour switches |

The connection-mode word is at `0x10043b84` and its **initial value in the file is 2**, so the default with no argument is USB (established from the `.data` bytes). The mode switch is the jump table at `0x10027d00` used by `sub_10005614`: mode 0 nothing, 1 `0x10007cc0`, 2 `0x10003eec` (USB open), 3 `0x10005200`, 4 `0x100053f4`, 5 `0x10007f4c` (established by decoding the table).

Start-up order after parsing (established, with the call sites in `main`): print the banner and the current and flashed L2 version (the running version is cached in `/tmp/l2ver`; the flashed one is found by `flash_read_addr`, `flash_lock`, `FlashValidateImage`, `flash_unlock`); `nvramInit`; copy NVRAM keys 0x17 and 0x18 into two globals (`0x10043c70`, `0x10043c7c`) when they are nonzero; when the simulator options supplied a rack number, store it with `nvramSet(1, rack)` and clear key 0x16; `IRouterInit` and nine `IRouterInitID` calls for the task numbers 0, 2, 7, 4, 3, 12, 5, 13 and 9 (each fills one of the global router-ID words; the roles of the numbers are not read); `RouterSetOutputMonitor`; `L2CfgInitLocal`; the network service ports below; the signal handlers; the L1 connection for the selected mode; `sub_10008278` (the two serial ports, below); `EventStart`, the select loop, which should never return.

## Network and socket modes

`libRouter` is a select-based event loop and socket router (established from its imports: `select`, `socket`, `bind`, `listen`, `accept`, `connect`, `setsockopt` with `SO_REUSEADDR`, send and receive buffers and `TCP_NODELAY`, `IP_TOS`, `getpeername`, and Unix sockets whose path format is `/usr/tmp/UNIX_%04x`). `EnetServerStart` is called five times with the port bias plus these constants: **9001, 9002, 9004, 8000 and 8002** (established: the `addi` constants `0x2329`, `0x232a`, `0x232c`, `0x1f40`, `0x1f42` at `0x10003acc` to `0x10003b4c`). `libapi`, which the other programs link, connects to bias+8000 (or 8002 when its mode flag at `0x244a0` is set) in `sn1SCInit` and to bias+9002 (or 9004) in `sn1SCInitL2App` (established, `0xaa9c` to `0xab98`). So command-line tools use 8000 and application daemons (`l2gui`, `l2part`, `l2flash`) use 9002; the second ports of each pair are the same service with the flag set (inferred); 9001 is not used by `libapi`, and is inferred to be the L2-to-L2 link. Port 8001 (bias+`0x1f41`) is where `sub_10008278` attaches the console and modem sessions (established that the constant is used there; purpose inferred from the session table strings).

`sn1SCInitL2App(host, ?, task)` registers a client under a task number. `l2part` passes 10, `l2flash` 8, `l2gui` 6 (established from the `li r5` before each call).

Direct telnet to the L2 is separate: `inetd` runs `telnetd -L /bin/l2term`, which runs `l2term --l2 localhost`; the shell window is service `sgil2`, port 5000 (see [rootfs.md](rootfs.md)).

## Serial modes

Two meanings share the word "serial".

- **Operator serial ports.** `sub_10008278` opens one port, `/dev/ttyS0` or, with the `-pc` option, `/dev/ttyS1`, with flags 0x82 (read-write plus `O_EXCL`; the earlier reading as no controlling terminal was wrong, see [userspace.md](userspace.md)) and attaches it as the modem session; the console session is the daemon's standard input (the "console port 38400" and "modem port" rows of the session table strings). The modem-port speed is one of 38400, 19200 or 9600 (the `baud` command), stored in NVRAM. `INFO: can't open modem port` is the failure text (established).
- **L1 over a serial line or a terminal server** (modes 3 and 4). After opening, the daemon initialises its PPP framing context (`PPPInit`, a 0x100-byte output buffer and an 8-byte input buffer), then writes **one byte, 0x0d** (the word at `0x10043bac` holds `0d 00 00 00`), by `write(2)` on the device in mode 3 and `SocketWrite` in mode 4 (`0x10005800` to `0x10005848`); a write result other than 1 is `FATAL: can't initiate L1 protocol`. It then sleeps 2 seconds, writes a second byte `0x18` (word at `0x10043ba8`), reads one byte at a time until it receives `0x18` (`can't confirm L1 protocol` on a read of 0 or an error; other bytes are discarded) and sets a delay mode (`can't set delay mode on L1 connection`). Corrected and extended in [userspace.md](userspace.md). The bytes and calls are established; that the 0x0d byte makes an L1 leave its text console for PPP packets is inferred from the L1 notes in [../l1/debug-console.md](../l1/debug-console.md). Packets are then PPP-framed (`PPPRead`, `PPPWrite` in `libsys`).

## USB mode and the SGI L1 kernel driver

Established from `l1_open_usb` (`0x10003eec`), the device open (`0x100046c4` onward), the reset routine (`0x10004210`) and the service routine (`0x10004e8c`); the kernel side is the `sgil1.c` code in the inflated kernel.

- Control node: `/dev/sgil1_cs` (character 180,104), opened read-only. After that the daemon issues the ioctl **`0x40045306`** (read-direction, 4-byte argument, group `'S'` number 6), which returns an integer **driver revision** into `0x10043bc0`; a failed call sets the revision to 0. If `-usbpath` was given and the revision is 0 or 1 the daemon prints that the driver does not support the option and fails. Otherwise it registers `sub_10004e8c` as an input handler on the control descriptor (`EventAddInput`).
- Service: when the control descriptor is readable, `sub_10004e8c` reads up to 0x28 bytes of connection status (`l1_service_usb_device`); messages `WARNING: can't read USB connection status` and `no devices` show the failure cases. The record is one byte per device slot, 0 or 1 (decoded in [userspace.md](userspace.md)).
- Per-device nodes: `/dev/sgil1_%d` (180,64 to 180,103, so up to 40 devices), opened read-write. The daemon calls **`0xc0095307`** on the control descriptor, not on a device node (read-write, 9-byte argument; `SGIL1_ST_READ_DEV_CFG` in the error text), before it opens the device, with byte 2 of the argument set to the low byte of the device index, only when a USB path filter is in force and the revision is above 1; and **`0x40095304`** (read, 9 bytes; `SGIL1_READ_CFG`, issued only when the flag at `0x10043bbc` is nonzero) to learn the device's bus, path and device numbers, which it formats as `b<bus>;p<path>;d<dev>` for display and matching. Failure texts are `SGIL1_ST_READ_DEV_CFG error` and `SGIL1_READ_CFG error`. `INFO: opened USB device at <path> (<name>)` follows.
- Reset: ioctls **`0x20005301`** (`SGIL1_RESET_READ`), **`0x20005302`** (`SGIL1_RESET_WRITE`), **`0x20005303`** (`SGIL1_RESET_DEVICE`), no argument, all issued through one wrapper (`0x100041a8`) that skips the ioctl and returns 0 when the word at `0x10043bbc` is zero (its initial value in the file is 1, so the ioctls are issued; what clears it is not followed). The reset routine `sub_10004210` issues **`0x20005305`** and then, when its second argument is nonzero, `0x20005303` when the driver revision is above 0, and otherwise `0x5302`, `0x5301` and `0x5303` in a sequence that depends on which of them fail (transcribed in [userspace.md](userspace.md)); each failure prints the matching text (`SGIL1_RESET_WRITE error`, `SGIL1_RESET_READ error`, `SGIL1_RESET_DEVICE error`) and a failed device reset returns -1. The decision tree is not transcribed. The three names and numbers match the kernel driver's own strings and the compares in its ioctl handler (kernel offsets `0x9e2c0` to `0x9e3a0`): `0x20005301`, `0x20005302` and `0x20005303` print `SGIL1_RESET_READ`, `_WRITE` and `_DEVICE`; `0x20005305` takes the write-reset case. The driver also resets the device on close (`SGI L1 device reset failed on close`). The kernel code for `0x40045306` and `0xc0095307` copies 4 and 9 bytes to and from user space; the 9-byte record is bus, path depth, device address and the port numbers (decoded in [userspace.md](userspace.md)).
- Data: reads and writes of packets use `read(2)`/`write(2)` on the device (routines named `l1_read_usb` and `l1_write_usb` in the daemon's own strings); after 'multiple read errors' or 'multiple write errors' the daemon closes the device. The kernel prints `short write`, `write timeout`, `read not done`. The packet format on the wire is the L1 stream: the L1's USB side delivers router packets (see [../l1/usb-descriptors-routing.md](../l1/usb-descriptors-routing.md)).

## Flash, and where NVRAM lives

`libdrivers` (14,036 bytes) is the only flash access. Established from `flash_init` (`0x17a0`) and the data it reads:

- It opens `/dev/mem` with flags 0x1002 (read-write, synchronous) and `mmap`s the flash window with protection 3 (read-write), shared. It tries the bases `0xfc000000` then `0xfe000000`, mapping 4 MiB at each and running `flash_autoselect_test`; a base that passes is kept, and when the kept base is not `0xfe000000` it logs `flash memory detected at non-standard location` and records an offset so callers' `0xfe...` addresses still land in the window. If both fail the second is kept anyway. Flash is normally mapped from physical `0xfe000000`, the address the image header gives as its load address. The full routine-by-routine reading is in [libdrivers.md](libdrivers.md).
- Access is serialised by a System V semaphore, key `0x0124f0e5`, created with `semget(key, 1, 0x3b6)`; `flash_lock` and `flash_unlock` take and release it. The daemon and `l2part` both use it.
- Status polling in `flash_seq_done` tests bits `0x00400040` and `0x00200020`: two 16-bit flash devices side by side form a 32-bit bus (toggle and error bits in each half). Entries `flash_erase`, `flash_program`, `flash_read`, `flash_get_id`, `flash_erase_all`, and the RAM-resident copies `flash_erase_ram` and `flash_program_ram` are exported.
- The exported `sector_map` (`0x28bc`, 156 bytes) is 39 end addresses: 31 sectors of 64 KiB ending at `0x0000ffff` through `0x001effff`, then eight of 8 KiB ending `0x001f1fff` through `0x001fffff` (read from the table's bytes). That is one 2 MiB device with eight small boot sectors at its top. With two such devices side by side on the 32-bit bus (suggested by the paired status bits above) the span is 4 MiB with eight 16 KiB sectors in its top 128 KiB, `0xfe3e0000` to `0xfe3fffff`, which is exactly where the partition database (64 KiB) and the NVRAM (the next 64 KiB) sit below; the image itself ends at `0xfe2ca03f`. the code doubles the `sector_map` entries (read in `flash_erase`) and maps a 4 MiB window, so the two-device reading is established, not inferred (see [libdrivers.md](libdrivers.md)).

Uses of the flash by absolute address (established from the `lis` immediates):

| Consumer | Address | Role |
|---|---|---|
| `nvram.c` in `libmain` | `0xfe3f0000` (new location), `0xfe2e0000` (old) | NVRAM block. If the first 16 bytes at `0xfe3f0000` are all `0xff` the library reads from the old location instead and logs `NVRAM data detected at old flash location--moving to new location`; after a load it rewrites the new location. |
| `db.c` in `l2part` | `0xfe3e0000`, 64 KiB | the partition mode database (see [partition-task.md](partition-task.md)) |

## NVRAM

`libmain` keeps a 16 KiB image of the NVRAM in RAM (`nvram`, 0x4000 bytes) and writes it back to flash by erasing and reprogramming (`nvramSave`; the error texts are `nvramSave: error erasing NVRAM` and `error programming NVRAM`). Established from `nvramInit` (`0xc6fc`):

- The block is IPMI-style: an 8-byte common header, checked with `ipmiCheckCSum`, whose second byte is the offset of the **internal use area** in 8-byte units; the area is 0x82c bytes long and has its own checksum. A bad header checksum logs `checksum error (CH) - nvram reset.` and a bad area checksum `checksum error (IUA) - nvram reset.`; both reset the contents. After the area comes an event log running to the end of the 16 KiB, bounded by `nvramLogStart` and `nvramLogEnd`; a log whose recorded bounds do not match the computed ones is reset with the text `log size changed - log reset.` The log is read and written through `nvramLogInsert`, `nvramLogFetch`, `nvramLogFinsert` and `nvramLogReset`.
- Loading and saving, read in full (`sub_dcf4` at `0xdcf4` loads, `sub_de14` at `0xde14` saves, `sub_dc0c` at `0xdc0c` converts the log pointers). The load reads 16 bytes at `0xfe3f0000` with `flash_read`; if they are all `0xff` it reads the 16 KiB from the old location `0xfe2e0000` instead and remembers to save, otherwise from `0xfe3f0000`. It ignores the return values of `flash_read`, so a failed flash read leaves the RAM image as it was (zero) and the load still reports success. The four log words at internal-use-area offsets `0x818`, `0x81c`, `0x820` and `0x824` are stored in flash as offsets from the start of the image and are converted to RAM addresses on load by adding the image address (the converter adds the difference between the new and old base to each and recomputes the area checksum byte at `0x82b`); the save subtracts it again before writing. Save is `flash_erase(0xfe3f0000, 16 KiB)` then `flash_program` of the converted image; failures log `error erasing NVRAM` or `error programming NVRAM` with the flash error code, and the image is converted back to RAM addresses either way. A save is skipped while the word at `0x31f20` is positive; the load sets it to 1 before it reads and clears it before the final save, so it means "load in progress".
- `nvramInit(mode)` (the argument is 3 from `main` and from the `-syncssn` option; with argument 1 the call returns 0 at once when an earlier call already set the loaded flag at `0x31f24`, and otherwise loads without the already-initialised warning): a header whose checksum fails, whose format-version nibble (low nibble of byte 0) is zero or whose internal-use-area offset (byte 1) is zero is replaced by a new header (`sub_cb9c`: version 1, area offset 1, checksum at byte 7) and a fresh area (`sub_caf8`, not read), with `checksum error (CH) - nvram reset.` added to the event log; a failed area checksum replaces the area only (`checksum error (IUA)`); log bounds that are not the computed ones, or log pointers outside them, reset the log. With argument 3 the image is then always saved to flash (`sub_de14` at `0xca28`), whether or not anything was reset. It returns -1 if any of the three resets happened, else 0; `main` ignores the result in both places. `-syncssn <string>` (the `sub_10003e1c` path) calls `nvramInit(3)`, compares NVRAM key 3 with the string and, unless a server already listens on port 9004 plus the bias (`EnetServerCheck`), stores the new string with `nvramSet(3, ...)` and exits; its purpose (keeping the serial number in step with another L2) is inferred.
- After loading, `nvramInit` sets the environment variable `TZ` from key 17 (the string is `TZ`, established).
- Settings are read with `nvramGet(key)` and written with `nvramSet(key, value)`. `nvramGet` accepts keys 1 to 26 through the jump table at `0x1ede0`; any other key returns -1. The table below is what each key returns, read from the table's targets (offsets are in the internal use area; a "bit" is a bit of the 32-bit big-endian word at +4 counted from the least significant end; a string key returns a pointer):

| Key | Returns | Seen used as |
|---|---|---|
| 1 | byte at +4 (high byte of the flag word) | inferred: the rack ID (the simulator rack argument is stored with `nvramSet(1, ...)`; the `rackid` command text) |
| 2 | bit 23 | |
| 3 | pointer to a 16-byte string at +8 | inferred: the system serial number (set by a routine that clears and copies 15 characters) |
| 4 | pointer to a 32-byte string at +0x18 | inferred: the system name (31 characters) |
| 5 | bit 22 | |
| 6, 7, 8 | words at +0x38, +0x3c, +0x40 | static IP address, netmask, broadcast (`l2staticip` reads exactly these keys, established) |
| 9 | bit 21 | |
| 10 | word at +0x44 | USB debug setting (`l1_open_usb` reads key 10, established) |
| 11 to 15 | words at +0x48, +0x4c, +0x50, +0x54, +0x58 | not identified |
| 16 | bit 20 | DHCP server disabled (`l2dhcps` reads key 16; the `dhcpserver` command text, inferred) |
| 17 | pointer to an 80-byte string at +0x5c | the time zone string |
| 18 | byte at +0xac | |
| 19 | pointer to a 16-byte string at +0xb0 | the L2 password (`l2passwd` reads key 19, established) |
| 20 | word at +0xc0 | |
| 21 | bit 19 | |
| 22 | byte at +3 | |
| 23, 24 | bytes at +0xc4, +0xc5 | copied by `main` into two globals at start-up (keys 0x17 and 0x18); roles not followed |
| 25 | word at +0xc8 | static gateway (`l2staticip` reads key 25) |
| 26 | bits 16 to 18 of the flag word | |

The remaining keys are not named because no reader of them was followed. `nvramReset` (the `nvram` command, "return L2 NVRAM settings to factory default") is at `0xcaa0`; its effect beyond clearing the area was not read.

## VPD and system records

`libsys` has a FRU record codec (`FRUInit`, `FRUPackData` 4,832 bytes, `FRUUnpackData` 4,048 bytes, `FRUAttach`, `FRUDetach`, `FRUDump`, `FRUDumpAll`, `FRUStatus`, `FRUSource`, `FRUFree`) and IPMI-style checksum helpers (`ipmiInit`, `ipmiCheckCSum`, `ipmiCalcCSum`), the same record family as the NVRAM header above and the L1's board records ([../l1/board-records.md](../l1/board-records.md)). The L2 itself fetches the brick VPD from each L1 over the router rather than from a local I2C device: no L2 binary opens `/dev/i2c-0` (the string appears in no shipped L2 program or library; established by a string search of every file in the tree). Four exported tables, `FRU_SHUB_MMR_Addr`, `FRU_SHUB2_MMR_Addr`, `FRU_NL4R_MMR_Addr` and `FRU_TIO_MMR_Addr`, list register addresses of node-board chips (their names are the only evidence for which chips; inferred). `brickConvertString`, `brickConvertType`, `brickTypeToChar` and `brickIsCompute` map brick type codes and names. The record layouts themselves are not decoded here.

## Display

The daemon does not draw. `l2gui` (a separate process, task 6) opens `/dev/fb0` (or the file named by the environment variable `FRAMEBUFFER`), calls the frame-buffer ioctls `0x4602`, `0x4600`, `0x4604` and `0x4605`, maps it with `mmap`, and opens `/dev/ttyS3` non-blocking with a terminal attribute change (the touch-panel link, inferred: no string names it). The daemon holds only the display's settings, through commands `l2dsp lock|unlock` and `l2dsp debug` ("lock the L2 display (disable touch-screen input)"); how those settings reach `l2gui` is not followed. The kernel names the controller (an Epson SED137x, see [image-layout.md](image-layout.md)).

## Session and password handling

The daemon serves command sessions (L2 prompt on a serial port or telnet) with a password check: three wrong attempts disconnect (`disconnecting after 3 failed password attempts`), a secondary L2 limits access, and `password clear` is refused except from the console or modem serial port (established as strings; the check code is not read). The password itself is NVRAM key 19, stored as the string `l2passwd` later applies to the account files (`/etc/passwd` rewritten with `root`, `.rhosts` copied from `/rhosts.allow`; established from the strings and calls).
