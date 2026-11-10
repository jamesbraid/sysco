# L2: what the user-space programs need from the kernel and the board

This note is the kernel side of the L2 contract. [l2/userspace.md](l2/userspace.md) already lists every device node the programs open, every ioctl, the `/dev/mem` flash window, the NVRAM and partition-database addresses, the L1 USB link as the daemon sees it, and the failure paths of the opens. This note does not repeat those. It adds what they rest on: how the boot loader hands over to the kernel, the physical windows and on-chip blocks the kernel maps, which kernel facilities exist and which are absent, which kernel driver backs each node, the system-call surface of each program, the PCI and USB chain that must work before an L1 can be seen, the display and reset paths, and an ordered list of what has to hold for the daemon to reach the point where it registers an L1.

Everything is read from the L2 1.44.0 image (IRIX 6.5.30, `l2.bin`, sha256 `6bcfda07a24cb4d56850922dab17a98223abcd14221d307e3a8c3778434e8761`). The kernel is the gzip stream at file offset `0x4a218`, inflated to 1,430,148 bytes (`kernel.bin`, sha256 `b2f8aa846a2c...` in the first 12 digits). The inflated image is flat PowerPC code linked at virtual `0xc0000000`; an address below is the virtual address, and `kernel.bin` offset = address - `0xc0000000` (the same convention as [l2/userspace.md](l2/userspace.md)). The kernel has no symbols, so routine names below are my descriptions. Disassembly used capstone's 32-bit big-endian PowerPC decoder; every address was read in the listing.

Grades are those of [../AGENTS.md](../AGENTS.md). A fact taken from a public source and not seen in the image is written as a hint with its source. Public sources named: the Linux 2.4 PowerPC 8xx platform and driver sources (the kernel's own banner says 2.4.7), the MPC860 user manual and the Tundra QSpan documentation. I did not re-open any of them in this pass (a Library search for the QSpan bridge and for the MPC860 PLPRCR register returned nothing), so register names that come from them are recalled, not checked, and are marked as hints.

Other versions: the same constants (the `ioremap` argument loads, the USB match word, the OHCI id table, the QSpan base, the panel names) occur as byte patterns in the kernels of all twenty other held L2 images from IRIX 6.5.13 on; the 6.5.12 kernel is an older build that lacks the QSpan, panel and OHCI-table patterns. That is graded inferred for those images: nothing but the patterns was compared.

## 1. Hand-off from the boot loader to the kernel

The kernel's first word is `mr r31,r3` at `0xc0000000`; it saves r3 to r7 and sets up the memory-management registers by hand (the standard 8xx start code). The platform-init routine at `0xc0132688` then reads the five registers as follows. Established.

| Register | Meaning | Read at |
|---|---|---|
| r3 | physical address of the board-information block; if nonzero, 28 bytes are copied from `r3 + 0xc0000000` to `0xc015ec74` | `0xc01326bc`, `0xc01326c0` |
| r4, r5 | physical start and end of an initial RAM disk, stored plus `0xc0000000` at `0xc015e428` and `0xc015e42c` | `0xc01326cc` to `0xc01326e0` |
| r6, r7 | physical start and end of the kernel command line; a zero byte is stored at the end address, then the text is copied to `0xc0117000` | `0xc01326ec` to `0xc0132704` |

The addition of `0xc0000000` shows that the loader passes **physical** addresses and that physical 0 is where the kernel is linked (virtual `0xc0000000` is physical 0). Established.

**Board-information block (28 bytes copied).** Offsets are those the kernel reads; the names are the public 8xx names for the same slots (hint).

| Offset | Use in the kernel | Evidence |
|---|---|---|
| `+0x04` | memory size in bytes; returned as the amount of RAM by the platform's end-of-memory hook (`0xc013267c`) and then capped by a `mem=` option | `0xc0132680`, `0xc01329b4` to `0xc01329d4` |
| `+0x08` | core clock in Hz; printed in MHz as "clock", and the divisor for the decrementer (`/16`) | `0xc000a684`, `0xc013250c` |
| `+0x0c` | bus clock in Hz; printed in MHz as "bus clock" | `0xc000a680` |
| `+0x10` to `+0x15` | the Ethernet address; the fast Ethernet driver copies six bytes from here into its station-address field | `0xc013c284` to `0xc013c2a8` |
| `+0x18` | console baud rate; looked up in a 21-entry table of terminal baud codes at `0xc012ca68` (0, 50 ... 460800), index 21 meaning "not found" | `0xc013bf6c` to `0xc013bf98` |

I found no reader of `+0x00` (the memory start) or of `+0x14` and `+0x16`: the only routines that form the block's address are platform init, the cpu-information routine, the serial console set-up and the Ethernet driver, and none loads those offsets. Established for the offsets read; the field names are hints. The 28-byte copy ends at `+0x1b`, so no later field of a longer public layout reaches the kernel.

The loader therefore has to supply, before the daemon can run: memory size, both clocks, the MAC address and the console rate. The image header's note that the loader fetches the MAC address from an I2C EEPROM ([l2/userspace.md](l2/userspace.md) section 5, from kernel strings) is consistent, because the kernel itself has no way to read an EEPROM (section 3).

**Command-line options the kernel understands** (the `__setup` table around `0xc015d160`; each row is the option string and the handler address). Established.

| Option | Handler | Note |
|---|---|---|
| `root=` | `0xc01310fc` | the kernel prints the standard "append a correct root= option" message if the root cannot be opened (`0xc0135208`) |
| `mem=` | parsed in `0xc01319dc` | caps RAM; accepts `k` and `m` suffixes |
| `noinitrd`, `ramdisk_start=`, `load_ramdisk=`, `prompt_ramdisk=`, `ramdisk=`, `ramdisk_size=`, `ramdisk_blocksize=` | `0xc0137904` to `0xc0137a10` | RAM-disk handling |
| `console=`, `panic=`, `reserve=`, `memfrac=`, `profile=`, `debug`, `quiet`, `l2cr=` | generic | |
| `wdt_8xx=` | `0xc0137648` | software watchdog switch (section 8) |
| `pci=` | `0xc01394ac` | PCI options |
| `SED137X` (searched with a substring match, then `color` or `mono`) | `0xc013aecc` | LCD panel choice (section 7) |

The initial command line is empty in the image (`0xc0117000` is zero in the file), so the root device, console and panel come from the loader. **What the loader passes is unknown** (section 11, item 1). The kernel can find a cramfs in a RAM disk (it prints "cramfs filesystem found at block"), and the 512-byte pad before the cramfs superblock ([l2/image-layout.md](l2/image-layout.md)) is compatible with the public cramfs reader, which accepts its superblock at offset 0 or at offset 512 (hint: Linux 2.4 `fs/cramfs/inode.c`). Which of the two the kernel's RAM-disk detector expects was not read.

## 2. Windows and on-chip blocks the kernel maps

During early memory-management setup (`0xc0132944`, called right after platform init) the kernel makes six calls to its I/O-remap routine (`0xc000ba60`, which tail-calls `0xc000ba84` with the "uncached" flag value 2). Before the kernel's memory initialisation has finished the routine returns the physical address as the virtual address, because the variable that bounds "descending" allocations (`0xc015e098`) is never written in the image text and so stays zero (read in `0xc000bb48` to `0xc000bb5c`). So these are one-to-one mappings. Established for the calls and the arguments; inferred for the one-to-one result (it rests on the variable never being stored, found by scanning every store with that offset).

| Physical base | Length | What it is | Evidence |
|---|---|---|---|
| `0xfa200000` | `0x10000` | the on-chip register block of the MPC860-family processor (the kernel's own interrupt, serial, timer, clock, watchdog, real-time-clock, I2C and Ethernet code all address it) | the section 4 users below; `0xc0132a1c` |
| `0xfa100000` | `0x1000` | **unknown**: mapped, but no instruction in the kernel and no word in any user-space program of the tree forms this address | `0xc0132a28`; scan of `lis` immediates in the kernel and in every ELF file of the tree |
| `0xfa210000` | `0x10000` | the PCI bridge's own register window (section 6); its address is also the initial value of the pointer the config-cycle routines load (`0xc012f2d0`) | `0xc0132a34`, `0xc012f2d0` |
| `0x10000000` | `0x20000` | the LCD controller: display memory from the base, registers in the last `0x20` bytes (section 7) | `0xc0132a40`; the LCD set-up at `0xc013ab10` |
| `0xfe000000` | `0x400000` | the flash window, the same 4 MiB the daemon maps through `/dev/mem` | `0xc0132a4c` |
| `0xfa210000` | `0x10000` | mapped a second time | `0xc0132a54` |

Separately, the PCI host set-up (`0xc01327d8`) maps `0x80000000` with length `0x20000000` (512 MiB) the same way (`0xc0132828` to `0xc0132830`) and stores the result in the host-bridge record; this is the CPU-side window onto PCI memory space. Established for the call; the role is inferred from the "QSpan PCI" failure text beside it.

The kernel never touches the flash window itself and has no flash driver (section 3); it maps it so that the window exists. The user-space mapping through `/dev/mem` is an independent `mmap` of the same physical range ([l2/libdrivers.md](l2/libdrivers.md)). Whether `/dev/mem` can map `0xfc000000`, the first base `libdrivers` tries, and what a load from an unpopulated address does is unknown (section 11, item 8).

## 3. Kernel facilities that exist, and those that do not

Established from the kernel's strings, its initialisation-call table and the registered-name records; the user-space need in the last column comes from the import and path scans of section 5.

| Facility | In the kernel | Needed by |
|---|---|---|
| Root from a RAM disk with gzip decompression; cramfs, romfs, minix and ext2 detectors | yes (`RAMDISK:` strings, `0x1009d0` to `0x100af0`) | the read-only root |
| cramfs | yes (file-system name `cramfs`) | the root |
| `proc`, `tmpfs` (also `shm`), `usbdevfs`, `pipefs`, `sockfs` | yes | `rc.sysinit` mounts `tmpfs` on `/tmp`, `/var`, `/etc` and `proc` and `usbdevfs` from `fstab`; the daemon does not use the files under `/proc/bus/usb` |
| `devfs`, `devpts`, any modular loading | **no** (`devfs` occurs only as part of `usbdevfs`; there is no `create_module` or `init_module` string and no module loader) | all nodes are static cramfs nodes; ptys are the legacy `ptyp`/`ttyp` pairs |
| System V semaphores | yes (`sysvipc/sem` and `ipc_init_ids` strings) | `libdrivers`' flash lock (`semget(0x0124f0e5,1,0x3b6)`, `semop`) |
| Sockets: Unix, TCP/IP, packet, netlink; loopback | yes | the daemon's listeners and clients |
| Kernel-level IP auto-configuration | **not found**: none of the usual "IP-Config", "Sending BOOTP" or `nfsroot` strings occurs, so the `rc.sysinit` comment "if we weren't auto-configured in the kernel" describes a case this kernel cannot produce (the address then comes from NVRAM through `l2staticip`, or from the DHCP client) | none |
| NFS, PPP network driver, SLIP | not found by string (the `/dev/ppp` node exists; `pppd` is present but nothing starts it) | none |
| MTD (flash as a block or character device) | **no**: the string `mtd` does not occur and there is no flash-map code | none: `/dev/mtd0` to `mtd3` are inert nodes |
| I2C core, `/dev` interface (`i2c-dev`) and the CPM I2C algorithm with a board adapter | yes | no program (section 8) |
| EEPROM or VPD client driver | **no** (no `eeprom` or `24c` string) | none |
| Real-time clock (generic PowerPC `rtc.c`) | yes, misc device | `hwclock` |
| Software watchdog (`wdt_mpc8xx`) | yes, misc device | none (section 8) |
| Frame buffer (`sgil2fb`) | yes | `l2gui` |
| Serial: four on-chip serial ports as `ttyS0` to `ttyS3`, plus a kernel console | yes | console, modem port, touch port |
| USB: core, hub, OHCI host controller driver, usbdevfs, the `sgil1` class driver | yes | the daemon in USB mode |
| PCI core with a QSpan-bridge host | yes | the OHCI controller |

The kernel's initialisation-call table, 25 entries at `0xc015d220` to `0xc015d283`, holds these in table order (the ones I identified): scheduler helper threads, user-count caches, the memory manager's page-out thread, `tmpfs`, file-lock and notification caches, System V IPC, the character-device core (`mem`, `tty`, `console`), the legacy pty driver, the RAM disk, the PCI proc entry, the real-time clock, USB core (`0xc013af88`), the OHCI driver registration (`0xc013b47c`), the `sgil1` driver (`0xc013b500`), and the four network families (TCP/IP at `0xc013d4c8`, Unix at `0xc013d82c`, netlink, packet). The 8xx serial driver, fast Ethernet, frame buffer, I2C and watchdog are not in that table; the ones I followed are reached from the 8xx platform and driver set-up paths instead. Established for the table contents; I did not read how the later drivers are called, so the relative order of those is unknown.

## 4. Which kernel driver backs each node

All of [l2/userspace.md](l2/userspace.md) section 3's nodes are listed there with their numbers; this table adds the backing driver and the on-chip resource behind it. Established unless marked.

| Node | Kernel side | Resource |
|---|---|---|
| `/dev/sgil1_N`, `/dev/sgil1_cs` | `sgil1`; three USB-driver records at `0xc012c22c`, `0xc012c260` and `0xc012c294`, same name, same probe (`0xc009e4b8`) and disconnect (`0xc009e794`) routines, same file-operations table (`0xc012c1e4`), and minor bases `0x40`, `0x50` and `0x60`, each covering 16 minors, so minors 64 to 111; minors 64 to 103 are the 40 slots and 104 is the control node | USB, section 6 |
| `/dev/ttyS0` to `/dev/ttyS3` | the on-chip serial driver: four port records of `0xcc` bytes from `0xc012c70c`, whose vector fields are `0x1e`, `0x1d`, `0x1c` and `0x1b` and whose type word (`+0x14`) has bit `0x10000` set in all four, which the driver's banner treats as "SCC" rather than "SMC" (`0xc013bb34`) | four on-chip SCC channels. The vector numbers match the public CPM vector list for SCC1 to SCC4 (hint, recalled) |
| console | the kernel has its own serial console set-up (`0xc013bf48`) that takes the baud from the board block (section 1) and sets 8 data bits, `CLOCAL`, `CREAD` (cflag low word `0x8b00` or-ed with the baud code) | one of the four ports; which one depends on `console=`, unknown |
| `/dev/fb0` | `sgil2fb` (major 29) over the LCD controller | section 7 |
| `/dev/rtc` (10,135) | the generic PowerPC clock driver calling the platform's get and set routines at `0xc000a5d4` and `0xc000a5ac` | the on-chip real-time clock: the counter at register-block offset `0x224` and the key at `0x324`, set with the two-word unlock/lock sequence `0x55ccaa33` then the value then `0xaa3355cc` (hint: MPC860 manual, RTC and RTCK registers) |
| `/dev/watchdog` (10,130) | `wdt_mpc8xx` | section 8 |
| `/dev/i2c-0` (89,0) | `i2c-dev` over the CPM I2C algorithm and an adapter whose module text reads "i2c RPX Lite/MBX" | on-chip I2C controller; no client attaches |
| `/dev/mem`, `/dev/kmem`, `/dev/port` | the standard memory-device driver (message "unable to get major %d for memory devs") | physical memory and I/O |
| `/dev/mtd*` | nothing | no driver |
| `/dev/ppp` | no PPP driver string; the node is inert | none |

The serial banner text is `ttyS%02d at 0x%04x is a %s`. The processor family is therefore one with four SCCs and a fast Ethernet controller, the MPC860T class (inferred from the register use; the manual was not re-read).

## 5. System-call and library surface of each program

Established from the dynamic symbol tables of each file in the 1.44.0 tree (undefined symbols that name a system call or its thin wrapper). These are the kernel entry points the programs rely on; library routines that need more than one call (`system`, `popen`, `syslog`) add `fork`, `execve`, `wait4`, `pipe`, `dup2`, `socket`/`connect` to `/dev/log`.

| File | Calls |
|---|---|
| `bin/l2` | `open`, `close`, `read`, `write`, `ioctl`, `select`, `socket`, `bind`, `setsockopt`, `sendto`, `recvfrom`, `tcgetattr`/`tcsetattr`/`cfset*speed` (the `ioctl` terminal requests), `fork`, `execv`, `system`, `dup2`, `setsid`, `sigaction`, `sleep`, `usleep`, `gettimeofday`, `time`, `getpid`, `syslog` |
| `lib/libRouter` | `socket`, `bind`, `listen`, `accept`, `connect`, `getsockopt`, `setsockopt`, `getsockname`, `getpeername`, `shutdown`, `fcntl`, `select`, `read`, `write`, `unlink`, `gettimeofday`, `signal`, `gethostname`, `gethostbyname` |
| `lib/libdrivers` | `open`, `mmap`, `semget`, `semop`, `close` |
| `lib/libmain` | `open`, `read`, `write`, `lseek`, `close`, `time`, `localtime`, `setenv` |
| `lib/libapi` | the socket calls above, `tcgetattr`/`tcsetattr`, `popen` (to run `/bin/ls -1 /dev/snsc/*c*`), `ioctl` |
| `bin/l2gui` | `open`, `ioctl`, `mmap`, `munmap`, `read`, `select`, `tcsetattr`, `fopen`, `system`, `signal` |
| `bin/l2part`, `bin/l2flash` | no direct system calls beyond `sleep`/`syslog`; the socket calls are in `libRouter` |
| `bin/partmgr` | `open`, `read`, `write`, `rename`, `unlink`, `time` |

What each call needs that the 1.44.0 root provides or does not:

- `syslog` writes to `/dev/log`; the root has `syslogd` and `klogd`, but `inittab` and `rc.sysinit` start neither. A program's `syslog` call therefore finds no listener and is dropped. Established from `inittab`, `rc.sysinit`, `inetd.conf`; the library's behaviour on a missing `/dev/log` is the standard one (hint).
- `system` and `popen` need `/bin/sh` (bash, present).
- The daemon's Unix-socket path is built by a helper in `libRouter` (`0xd078`) from `/usr/tmp/UNIX_%04x` with family 1. The root image has `/usr` as a directory holding only two symbolic links (`bin`, `sbin`) and no `/usr/tmp`, and `rc.sysinit` creates only `/var/run` and `/var/log`. A bind to that path therefore cannot succeed on the shipped root. Established for the absent directory; whether the failure is fatal, or whether the daemon's default (USB) mode ever binds it, is unknown (section 11, item 5).
- `l2gui` names fifteen font files under `/usr/local/microwin/fonts/`; the root has no `/usr/local`. What `l2gui` does when they are missing is unknown (section 11, item 6).
- `libapi`'s `sn1SCUseSCDevice` looks for `/dev/snsc/*c*`, a directory that no node and no kernel driver provides ([l2/userspace.md](l2/userspace.md) section 11).

## 6. The PCI and USB chain that must work before an L1 is seen

This is the part of the boot the daemon cannot influence. It is why `/dev/sgil1_cs` reads all zeros when it fails.

**PCI host.** The kernel's PCI host set-up (`0xc01327d8`) allocates one host-bridge record and prints "Can't alloc PCI controller structure for QSpan PCI" on failure (established; the name is the kernel's own string). It sets bus numbers 0 to `0xff`, the PCI-memory window base `0x80000000` (record `+0x1c`), the mapped window from section 2 (`+0x18`) and the configuration-access routine table at `0xc011a1fc`. Established.

Configuration cycles (the six routines at `0xc000a78c` to `0xc000a8b8`, with the worker routines at `0xc000a934`, `0xc000aa0c`, `0xc000aae4`, `0xc000abac`, `0xc000ac78`, `0xc000ad44`) work like this. Established from `0xc000abac` and `0xc000ad44` and the address builder at `0xc000a8f4`:

1. The bus number must be at most 7 and the device-and-function byte below `0x80`; anything else returns the standard "device not found" status `0x86`.
2. With interrupts off, the routine sets bit `0x1000` in the bridge register at offset `0x800` of the bridge window.
3. It stores the configuration address word at offset `0x500`: bus number in bits 16 to 23, device and function in bits 8 to 15, the register number with its low two bits cleared, and bit 0 set when the bus number is nonzero (a type-1 cycle).
4. Data moves through offset `0x504` of the bridge window: a 32-bit store or load for a word, and for bytes and halfwords at `0x504 + ((register xor 3) & 3)` (byte) or the equivalent for a halfword, which is the big-endian lane of the little-endian register. There is no software byte swap, so the bridge swaps in hardware (inferred from the absence of swap instructions).
5. It clears bit `0x1000` of offset `0x800` again.

The register names for offsets `0x500`, `0x504` and `0x800` are the QSpan configuration-address, configuration-data and miscellaneous registers in the public Linux QSpan code as I remember it (hint, not re-read).

**OHCI.** The USB host driver's PCI id table is at `0xc015cee8` and has two entries (established): vendor `0x1022` device `0x740c` with the driver-data bit that selects the "AMD756 erratum 4 workaround" message, and a class entry for class code `0x0c0310` (serial bus, USB, OHCI) for any vendor. The probe (`0xc013b388`) requires a nonzero interrupt line in the device record ("found OHCI device with no IRQ assigned"), takes the first memory region of the device, maps it with `ioremap`, and calls the controller set-up (`0xc013b158`), which requests the interrupt, resets the controller, waits in a ten-step delay loop, and starts it. The controller therefore needs: a PCI function that answers configuration reads, a memory region the kernel can assign inside `0x80000000` to `0x9fffffff`, and a working interrupt line. **What programs the interrupt line register, whether the kernel or the loader assigns the base address, and which interrupt the controller uses are unknown** (section 11, item 3).

**Hub and device enumeration.** The standard Linux 2.4 hub driver (strings `hub.c: ...`) walks ports and gives devices numbers; the kernel stops a device path at six hubs ([l2/userspace.md](l2/userspace.md) section 4, the 9-byte record). Whether an external hub sits between the root hub and the L1 connectors, and how many root ports there are, is unknown.

**Class driver match.** [l2/userspace.md](l2/userspace.md) section 10 records the match as a descriptor word, one configuration, one interface and a bulk-out then bulk-in pair. The exact test in `0xc009e4b8`, which this note adds, is: the device descriptor word at `+0xbc` of the device record equals `0x065e1234` (vendor `0x065e` and product `0x1234`, as the L1's descriptor in [l1/protocols.tsv](l1/protocols.tsv) row 6 shows); the byte at `+0xc5` (configuration count) is 1; the configuration has one interface; the probe's interface argument is 0; that interface's setting has two endpoints; the first endpoint's address-and-attributes word masked with `0x8003` equals 2 (OUT, bulk) and the second's masked with `0x8003` equals `0x8002` (IN, bulk). It then takes the first free entry of a 40-entry slot table of `0x88` bytes at `0xc0195384` (free means the first word and the byte at `+0xd` are both zero, `0xc009d62c`), fills it, allocates two URBs, logs "SGI L1 connected, minor: %d device: %d.%d" and sets the change flag that makes `sgil1_cs` readable (`0xc009d5c8`). Established. A forty-first device is not claimed (the allocation routine returns -1 and the probe returns 0).

## 7. The display

Established from the frame-buffer set-up at `0xc013ad74` and the LCD controller routine at `0xc013ab10`; the first sentence of each point is established, any extension is marked.

- The controller is an Epson SED1375-family part. The routine prints "SED1375" unconditionally (the 1374 string is also in the image but is not selected by the code read), display memory starts at physical `0x10000000`, and the register block is the top `0x20` bytes of a `0x20000`-byte decode, at `0x1001ffe0`. The frame buffer's start (`0xc015e4bc`), size (`0xc015e4c0`) and line length (`0xc015e4c4`) are variables set at `0xc013ab2c` to `0xc013ab68`.
- Two panels are built in, as records at `0xc012bcf8` (a Hantronix 320 by 240 4-bit greyscale panel, 160 bytes per line, 38,400 bytes) and `0xc012bd14` (a Hantronix 320 by 240 8-bit passive colour panel, 320 bytes per line, 76,800 bytes). The line length is `width << log2(bpp) / 8`, the size is line length times height (`0xc013ab4c` to `0xc013ab64`). The pointer to the active record is initialised in the image to the greyscale record (`0xc012f5f0` holds `0xc012bcf8`). A `SED137X ... color` or `... mono` text on the command line replaces it (`0xc013aecc`). So without an option the greyscale panel is used. The image holds no command line, so which the product ran with is unknown.
- The controller's registers are loaded from the panel record (`0xc013abb8` to `0xc013acd0`): register `+1` is packed from record bytes `+0x11` to `+0x15`; register `+2` from bytes `+0x10` and `+0x16` to `+0x19`; register `+3` gets 3; register `+4` gets width/8 - 1; registers `+5` and `+6` get the low and high bytes of height - 1, and `+0x12` and `+0x13` repeat them; register `+8` gets record byte `+0x1a` and `+0xa` gets `+0x1b`; register `+0x19` gets `0x1e`; the remaining registers from `+7` to `+0x1c` that are stored get zero. The per-panel bytes are in the records (greyscale: `+0x10..+0x1b` = `02 00 00 00 / 00 01 01 00 / 01 00 1e 3b`; colour: `03 03 01 00 / 00 01 01 00 / 00 00 10 01`); the meaning of each register is the manufacturer's and was not mapped here.
- The routine also writes a zero byte to `0x1000ffe0` (a display-memory address, not a register) at `0xc013ab7c`, and then fills `0x9600` (38,400) bytes of display memory from a table at `0xc01226f8`, byte by byte with a per-byte bit transformation (`0xc013acd8` to `0xc013ad58`). That count is the greyscale panel's frame size. Established for the loop bounds; the transformation and the table's picture are not decoded. The purpose of the single byte store is unknown.
- Frame-buffer attributes: start `0x10000000`; length equal to the size variable above; the panel record's byte `+0x10` (log2 of bits per pixel: 2 for the greyscale panel, 3 for the colour panel) is tested against 1 (`0xc013ae18` to `0xc013ae38`): a field at `+0xbc` of the frame-buffer record is set to 0 when it equals 1 and to 3 otherwise, and in the otherwise case a halfword at `+0xd0` is set to 1. By position these are the visual (0 monochrome, 3 pseudo-colour) and a colour-map length; both built-in panels take the otherwise case, so both report a palette visual, which is the branch of `l2gui` that skips the true-colour check and uses `FBIOGETCMAP` and `FBIOPUTCMAP` ([l2/userspace.md](l2/userspace.md) section 3). The offsets in the frame-buffer information record are matched to the public 2.4 layout from the position of each store, so the visual and length fields are inferred, not established. The `mmap` length `l2gui` passes is therefore the panel's size rounded up to a page: 40,960 bytes for the greyscale panel and 77,824 for the colour panel (inferred; this fills the gap [l2/userspace.md](l2/userspace.md) section 3 leaves for the length).
- The touch port `/dev/ttyS3` is the fourth on-chip SCC; the kernel treats it as an ordinary terminal and has no touch-panel driver (no string; inferred role as in [l2/userspace.md](l2/userspace.md)).

## 8. Reset, halt, watchdog, clock and I2C

- **Reset, halt and power-off are the same action.** The platform's three hooks for restart, power-off and halt (stores at `0xc0132774` to `0xc0132790` into the platform record at `0xc015e7f0`) are `0xc000a5e0` and two thin routines (`0xc000a624`, `0xc000a648`) that call it with 0. `0xc000a5e0` sets bit `0x80` in the register at offset `0x284` of the on-chip block, clears the machine-check-enable bit of the machine-state register, and then reads the byte at offset `0x28c`; if the processor survives it prints "Restart failed" and loops. The sequence is the one the public 8xx code uses to force a checkstop reset by touching an unimplemented register (hint: Linux 2.4 `m8xx_setup.c`; `0x284` is the PLL, low-power and reset control register in the MPC860 manual, recalled). So `/sbin/reboot` (the daemon's `init 6` and `/sbin/reboot` calls), `halt` (`init 0`) and the `poweroff` binary all end in a processor reset; nothing powers the L2 down. Established for the code; the register names are hints.
- **Watchdog.** `wdt_mpc8xx` (`0xc0137500`) reads the system protection control register at offset `+4`; if the software-watchdog enable bit (`0x4`) is clear it prints "SWT not enabled by firmware" and the driver does not start. If set, it loads the maximum timeout, sets two more bits (`0x3`), services the watchdog with the key pair `0x556c` and `0xaa39` at offset `0xe`, starts a kernel timer, and registers the misc device. No L2 program opens `/dev/watchdog`; that the kernel timer is what keeps servicing it is inferred (the timer callback at `0xc007f5e8` was not read), as is the meaning of the bits `0x3` (reset on timeout and a prescaler, from the MPC860 manual as recalled). The register accesses are established. Whether the loader enables the watchdog is unknown (section 11, item 2). The option `wdt_8xx=` can switch it off.
- **Real-time clock.** The hooks at `0xc000a5ac` and `0xc000a5d4` read and write the on-chip clock (section 4). `rc.sysinit`'s `hwclock --hctosys` and the daemon's `date` command (`hwclock -u --systohc`) go through `/dev/rtc`. The kernel also writes the clock-source and decrementer set-up (`0xc01324f0`) into the register at offset `0x280` with its key at `0x380`. Established for the stores; the register names are hints.
- **I2C.** The adapter and `/dev/i2c-0` exist; no client driver (no EEPROM, no sensor) is present, and no program opens the node ([l2/userspace.md](l2/userspace.md) section 5). So in 1.44.0 **no vital-product data is read by the kernel or by user space**; the board's own EEPROM, if any, is read only by the boot loader. Established for the kernel and the tree; inferred for the loader (no loader code read).

## 9. What the daemon needs, in order, to register an L1

The daemon's registration is the call `L2CfgMAddPort` with type 2 inside the open of a slot ([l2/userspace.md](l2/userspace.md) section 8, step 3), and the port becomes usable when a reply to the first configuration request has been processed ([l2/config-request-reply.md](l2/config-request-reply.md)). This list says what must hold underneath, step by step, and who provides it. Grades are per step; "established" means read in the 1.44.0 image.

1. **Loader to kernel.** Physical addresses in r3 to r7 as in section 1, with the board block holding memory size, clocks, MAC address and baud rate; a RAM disk range holding the cramfs, or a root given by `root=`. Provider: the boot loader (not read). Unknown how the root is delivered.
2. **Kernel up.** Memory-management set-up maps the windows of section 2; the on-chip block must be at `0xfa200000`. Provider: hardware chip-select set-up by the loader (unknown which windows the loader enabled).
3. **Console and `init`.** The kernel opens `/dev/console` (link to `ttyS1`) and execs `/bin/init`; the serial console set-up uses the baud from the board block. Established (strings "unable to open an initial console", "No init found").
4. **`rc.sysinit`.** `tmpfs` mounts, `mount -a` (`proc`, `usbdevfs`), `hwclock`, `l2passwd`, `l2staticip`, loopback and default route. These can fail without stopping boot; `usbdevfs` must exist as a file system type for `/proc/bus/usb` but the daemon does not use it. Established ([rootfs.md](l2/rootfs.md)).
5. **Daemon start (`inittab` entry `l2`).** `/dev/mem` must be openable and the flash window must be mappable and readable, or the daemon dies in its version lookup ([l2/user-mode.md](l2/user-mode.md)); a semaphore key must be creatable (`semget` works with System V IPC in the kernel); the sockets in section 3 must exist. Established for the dependencies; the cause of the early failure is read in [l2/user-mode.md](l2/user-mode.md).
6. **USB.** PCI configuration cycles through the QSpan bridge (section 6) must answer for the OHCI function; the OHCI function must have a nonzero interrupt line and a memory region; the controller must reset and the root hub must report a connected device; the device must enumerate with the descriptor shape in section 6. Provider: hardware plus kernel plus the loader (unknown which part programs the PCI function).
7. **Class driver.** The probe of section 6 claims the device, assigns it a slot, and raises the change flag.
8. **Daemon opens `/dev/sgil1_cs`, reads 0x28 status bytes, opens `/dev/sgil1_N` and registers.** From [l2/userspace.md](l2/userspace.md) section 8.
9. **Request and reply.** The first request goes out 0.75 s after the open, to the L1, over the bulk-out endpoint; the reply arrives on the bulk-in endpoint; the port state becomes 2 ([l2/config-request-reply.md](l2/config-request-reply.md)).

The Ethernet controller, the PHY, the display, the watchdog, the real-time clock and I2C do not appear in this chain. A missing Ethernet address affects the network listeners and the partition and flash-update clients (they connect to `localhost` over loopback); it does not affect USB registration.

## 10. Facts that correct or tighten earlier notes

- [l2/rootfs.md](l2/rootfs.md) says the kernel has "an I2C adapter on the communications processor" and "a USB host stack (`usb-ohci.c`, `hub.c`)"; it does not say the OHCI controller sits behind a PCI bridge, which is a QSpan-named host with a 512 MiB memory window (sections 2 and 6).
- [l2/image-layout.md](l2/image-layout.md) lists "a real-time clock driver" without saying it is the processor's own clock; section 4 says so.
- [l2/userspace.md](l2/userspace.md) section 3 says the `mmap` length of `/dev/fb0` "is not stated"; section 7 above gives the panel-derived length (inferred).
- The `/dev/mtd0` to `mtd3` nodes are inert because the kernel has no MTD (section 3); [l2/rootfs.md](l2/rootfs.md) already says no program opens them but not why they cannot work.

## 11. Unknowns and the software test that settles each

1. **What the boot loader passes in r3 to r7 (the root device, the initial RAM-disk range, the command line, the console, the panel option).** Test: disassemble the loader (`l2.bin` bytes `0x34` to `0x4a218`) from its entry; find the final branch into the kernel at `0xfe04a218` or its inflated copy; record the values loaded into r3 to r7 and the text stored for the command line.
2. **Whether the loader enables the software watchdog.** Test: in the same disassembly, look for a store to register-block offset `+4` (system protection control) and read the value; if none, the processor's reset value decides (MPC860 manual, SYPCR reset value).
3. **Who programs the OHCI function's interrupt line and base address, the interrupt used, and the USB topology (root ports, any external hub).** Test: in the loader, find the configuration-space writes through the bridge (stores to bridge offset `0x500` and `0x504`); in the kernel, read the interrupt routing at the interrupt-line read in the PCI device set-up and the 8xx interrupt-number translation; count ports from the root-hub descriptor routine (`hub.c` strings, power-on-to-power-good print).
4. **What the 4 KiB window at `0xfa100000` is.** Test: search the loader for any access with that base or an offset from it; a hit gives the register meaning, no hit leaves it a mapped-but-unused window.
5. **Whether a failed bind of the Unix-domain path is fatal, and whether the default USB mode ever binds it.** Test: read the callers of the address builder at `libRouter` `0xd078` and the error path after `bind`; find which of them the USB-mode start-up calls.
6. **What `l2gui` does when its font files are absent.** Test: in `l2gui`, find the `fopen` of the paths under `/usr/local/microwin/fonts/` and read what follows a null return.
7. **Which panel the product ran with, and the exact option syntax.** Test: read `0xc013aecc` for the token parse after `SED137X`, and the loader's command-line text (item 1).
8. **What a load from an unpopulated address does, for example the first flash base `0xfc000000` that `libdrivers` tries.** Test: read the 8xx machine-check vector at `0xc0000200` and its handler to see whether a user-mode machine check becomes `SIGBUS`; read `libdrivers`' probe at `0x17fc` to `0x19b4` to see which access it makes.
9. **The register meaning of the LCD controller's writes and the byte store at `0x1000ffe0`.** Test: compare the register offsets and bit packing at `0xc013abb8` to `0xc013ac9c` with the SED1375 manual; the manual is not in the held references.
10. **The QSpan register names and the exact role of bit `0x1000` at offset `0x800`.** Test: compare the five-step sequence in section 6 with the QSpan manual or the public Linux QSpan source; the match would turn the hints in section 6 into established names.

## 12. Method and limits

- Kernel facts: read in `kernel.bin` of 1.44.0 only, by following string references, the initialisation-call table, the platform record and the registered driver records. No function was run.
- User-space facts: the dynamic symbol tables and path strings of every ELF file in the 1.44.0 tree, and `inittab`, `rc.sysinit`, `fstab`.
- Other versions: byte-pattern presence only (see the opening).
- Nothing here is taken from a running unit.
