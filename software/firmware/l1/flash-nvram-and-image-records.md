# Origin 300 L1 1.44.0: flash, settings storage, clock, image records and the alternate USB path

Read from L1 1.44.0, package 1 (`Fuel/PE/O300 1MB image`, body sha256 `9db44f3bb06ec90475035781766e1ca7e924f15a0ed215cf855c1f491275fdd1`, load address `0xffe00000`). Addresses are in the first program copy. The seven-package composite is tabulated in [package-container.md](package-container.md).

## Flash programming contract

- **Bus (established).** The flash is on `CS0` as a 16-bit port; every command and data access is a halfword (`0xffe7f7fc`, `0xffe7f8d8`). Command addresses are byte offsets from the flash base `0xffe00000` (the base is the first word of the RAM pair at `0x30000918`, initial value `0xffe00000`, second word `0xfff00000`).
- **Command set (established, tables `0xffe708cc`-`0xffe7094c`, writers `0xffe7f7dc`-`0xffe7f9f8`).** Each sequence is a list of eight-byte entries (a byte offset from the flash base, then a long whose low halfword is written; the value is repeated in both halves, for example `0x00aa00aa`). Unlock = `0xaa` at offset `0xaaa` then `0x55` at `0x554`. Reset to read mode = unlock then `0xf0` at offset 0 (`0xffe708cc`, 3 entries, also written at the end of the autoselect routine). Program halfword = unlock, `0xa0` at `0xaaa` (`0xffe70924`), then the data halfword at its address. Sector erase = the reset sequence, then the eight-entry prefix at `0xffe708e4` (unlock, `0x80` at `0xaaa`, unlock), then `0x30` at an address inside the sector (`0xffe7f8d8`); while the erase runs the routine polls, writes `0xb0` (erase suspend, `0xffe7f8f4`), polls again, then writes `0x30` again (resume, `0xffe7f92c`) in a loop, so interrupts are serviced during an erase. Autoselect = unlock then `0x90` at `0xaaa` (`0xffe7093c`), then read the halfword at offset 0 (manufacturer) and offset 2 (device), then reset (`0xffe7f9a2`-`0xffe7f9f8`). No chip-erase sequence exists in the image. The offsets `0xaaa` and `0x554` are the common 16-bit NOR command addresses `0x555` and `0x2aa` doubled for a byte-addressed bus.
- **Completion polling (established, `0xffe7f810`-`0xffe7f8ae`).** Each pass first services the watchdog (`0x55`, `0xaa` at `MBAR+0x43`), then reads two consecutive halfwords from the flash base and compares data bit 6 (`0x0040`). Equal means the operation has finished and the routine returns 0. Different with data bit 5 (`0x0020`) of the first read set records that fact; seeing bit 6 still toggling on a later pass after that makes it write the reset sequence and return -1 (failure). When the argument is 0 the routine does one pass and returns 1 if the bits still differ (busy); a nonzero argument makes it loop. So a part that returns plain array contents to the first read after a command completes the operation at once, while one that toggles bit 6 and raises bit 5 takes the failure arm.
- **Sector map (established).** Two top-boot tables of sector end addresses relative to the base, selected by the flash-size byte `0x300002fb`, read at `0xffe55828` and holding `0` in the initialised data (its writer was not found by an absolute-address search; the table choice is therefore inferred to follow the same 1 MB or 2 MB size that the chip-select probe uses, see [the board contract](board-boot-contract.md)). For 2 MB (`0xffe70840`, 35 entries, log text "2MB flash part!!" at `0xffe70954`): `0x00ffff, 0x01ffff, ... 0x1effff` (31 sectors of 64 KB), then `0x1f7fff` (32 KB), `0x1f9fff` (8 KB), `0x1fbfff` (8 KB), `0x1fffff` (16 KB). For 1 MB (`0xffe707f4`, 19 entries): 15 sectors of 64 KB ending `0x0effff`, then `0x0f7fff`, `0x0f9fff`, `0x0fbfff`, `0x0fffff`. These are the boot-sector layouts of the 8-megabit and 16-megabit members of the common top-boot 16-bit NOR family; the image never names a part (the code expects exactly these sector boundaries and the command set above).
- **Not settled.** The manufacturer and device identifiers the autoselect read is meant to return: the readout routine stores both halfwords for its caller, and no comparison constant for them was found in the image.

## Boot-flash layout and image records

- **Image header (composite).** Each ColdFire package begins with 44 bytes (`ESTF`, `BINR`, start `0xffe00000`, end, then fields) followed by the body. The 44-byte header of package 1 reads, as big-endian words: `0x45535446`, `0x42494e52`, `0xffe00000`, `0xffeefd5f`, `0xff000000`, `0`, `0xffe00000`, `0xffeefd5f`, `0xffffffff`, `4`, `0`. Range 0's header is 104 bytes because it also carries the composite directory (seven offset and length pairs from `0x30`, count 7 at `0x2c`). An updater stripping the header gets the flat body that the flash holds from `0xffe00000`.
- **Per-copy metadata record (established, body `0xffe7fd1c`; second copy `0xffeefd1c`).** 48 bytes, twelve big-endian words:

| Word | Value in package 1 | Reading | Grade |
|---|---|---|---|
| 0 | `0x00040030` | record type 4, length `0x30` | established |
| 1 | `0xffe10000` (second copy `0xffe80000`) | start of this program copy | established |
| 2 | `0x00070000` | span reserved for the copy | established |
| 3 | `0x0006fd4c` | used length | established |
| 4 | pointer to `20060717182038` | build stamp, the string the loader compares | established |
| 5 | pointer to `1.44.0` | version string | established |
| 6 | `0xffe7fd4c` | pointer to a 16-byte table that the status routine calls the relocation table (zero word sum; see below) | established |
| 7 | `0x58487f20` | balancing word (the 12-word sum is zero); no code reads it as data (below) | established |
| 8 | `0xa83e0909` | 32-bit big-endian word sum of the first `word 3` bytes of the copy (computed here: equal); compared by the status routine (below) | established |
| 9 | pointer to the string `%4A?` | accepted brick letters | established |
| 10 | `1` | (not decoded) | established |
| 11 | pointer to `Fuel/PE/O300 1MB image` | image label | established |

  Only the 12-word sum is verified by the loader (see [the board contract](board-boot-contract.md)). That word 8 equals the sum of the copy's bytes is established by computation; the status routine below is the code that compares it.

  **Who reads the record (established, package 1, 1.44.0).** The record is reached only through the first word of each program copy (`0xffe10000` holds `0xffe7fd1c`, `0xffe80000` holds `0xffeefd1c`; no other instruction or table names the record address). Two routines read it. The loader's pair (`0xffe008ac`, `0xffe008fc`) takes that first word as the record pointer, requires it to be a word-aligned address in `0xffe00000`-`0xffefffff`, takes the halfword at offset 2 as the length and requires the word sum over that length to be zero. The status routine `0xffe3be5c` (called twice from the flash-status handler at `0xffe3c23c` and `0xffe3c2e0`, once per copy) repeats those checks, and in addition: requires the length to be at least `0x20`, requires word 6 to be nonzero (else it logs that the relocation table is missing), requires the 16-byte table at word 6 to sum to zero over the length in its second word, and compares the sum of the first `word 3` bytes of the copy with word 8, accepting a zero word 8 as "no image checksum". The handler then reads words 4 and 5 (build stamp and version pointers) from the record the routine returns, and nothing else. **Word 7 is therefore read only inside the zero-sum loops**, as part of the twelve words, and by no instruction as an operand of its own: its value is whatever makes the sum zero, so a writer of the record computes it last. Word 6 points to a table, not to the end of the used area: the table (`0x00000001`, `0x00000010`, `0xfff8ffef`, `0x00070000` at `0xffe7fd4c`) is checked as a relocation table by the routine's own diagnostic text, and the code does not use its contents beyond the checksum (what the four words mean is unknown; a test that would settle it is to find the code that consumes the table when a copy is relocated, which no routine of package 1 was found to do). Not read: the status routines of the other four ColdFire packages.
- **Request `0x0072`** (flash status) lists the images (established in [protocols.tsv](protocols.tsv)): its eleven or thirteen values come from this record and from the loader's retained record; both are read from the flash contents.

## Programming and erase requests `0x0073` and `0x0074`

Read from the Origin 300 package of 1.44.0 (package 1). Both are handled by the flash task (task 16). Each handler decodes its arguments through the argument accessor (`0xffe79344`, which returns an integer argument from either of two storage forms), calls a worker, and replies with one integer (the worker's result), plus a second integer (the failing flash address) when the result is negative (`0xffe3c41e`-`0xffe3c58e`, established).

| Request | Handler | Arguments in | Worker | Effect |
|---|---|---|---|---|
| `0x0074` erase | `0xffe3c41e` | offset (integer), length (integer) | `0xffe704d4` | erases every sector that overlaps the byte range `offset` to `offset + length - 1`, one sector at a time |
| `0x0073` program | `0xffe3c4c2` | offset (integer), data (binary argument, whose halfword length at argument offset `0x2e` gives the byte count) | `0xffe706c8` | programs the data as halfwords at `offset` |

Common rules (established, `0xffe704d4` and `0xffe706c8`):

- Both first run the flash-geometry initialiser (`0xffe7046c`) if the geometry record at `0x30000944` is not yet set. It stores the flash base (`0xffe00000`) in word 0; when the flash-size byte at `0x300002fb` is 2 it also selects the 2 MB sector table at `0xffe70840` (35 entries) and sets the end of the writable window to base plus `0x1ffff8`. The 1 MB table is the initial content of the record.
- The offset is added to the record's word 3 (a base offset into the flash, initial value read as 0 here) and the resulting address must lie inside the window held at `0x30000918` (start in word 0, end in word 1, initial `0xffe00000` and `0xfff00000`), else the request fails with result `-0x514` and returns the offset as the failing address.
- Interrupts are masked (to level 7) for the whole operation and restored afterwards; the erase loop services the watchdog (`0x55`, `0xaa` at `MBAR+0x43`) before every sector.
- A programming or erase step runs from a RAM copy of the flash code (`0x300006f8`, `0x3000087c`, `0x300007cc`), not from the flash itself.

Erase (`0xffe704d4`): for each sector in the table whose range overlaps the request it skips a sector whose words are already all ones, refuses (result `-0x514`, failing address = the sector start) a sector that the compares at `0xffe705aa`-`0xffe705ee` place inside a protected span of the record at `0x30000918` (the span starts at word 4 and its length is word 6; inferred from the compare pairs, not run), and otherwise calls the sector erase worker. A failed erase returns `-0x517` with the sector address.

Program (`0xffe706c8`): first writes the read-mode reset sequence (`0xffe708cc`, three entries), then for each halfword of the data: if the flash already holds that value it skips it; otherwise it programs it, and reads it back; a mismatch after programming returns `-0x515` with the halfword address. The data length need not be a sector multiple. There is no erase inside this request, so the host must erase first (the host tool's call order, [host-tool-options.md](../../userland/host-tool-options.md): status, erase, then program).

Not read: the protected span's values (set outside these routines) and the value the base offset in word 3 takes in practice. Test to settle: read the writers of `0x30000944` word 3 and `0x30000918` words 4 and 6 in the start-up code, or run the erase request against a test flash and see which sector it refuses.

## Settings storage in the NVRAM window

The 2 KB battery-backed window at `CS1` (`0x20000000`) holds, from offset 0:

- **Header (established, `0xffe5a6a8`-`0xffe5a74a`, check at `0xffe5a7de`).** 8 bytes: the low nibble of byte 0 is the header version (1 when initialised here), byte 1 is the offset of the record area in units of 8 bytes (1, so the area starts at `0x20000008`), and byte 7 makes the 8-byte sum zero. A header whose sum is nonzero, whose version nibble is 0 or whose offset byte is 0 is rebuilt: the 8 header bytes are zeroed and then the version, the offset and the checksum are written.
- **Record area (established, `0xffe5a188`-`0xffe5a74a`, `0xffe59e94`).** From `0x20000008`, in order: a 0x70-byte settings record, its mirror (+0x70), a 0x24-byte record (+0xe0), its mirror (+0x104), a 0xb4-byte record (+0x128) and its mirror (+0x1dc); total 0x290 bytes, so the area ends at `0x20000298`. Every record's last byte makes the byte sum of the record zero (`0xffe5b3cc` is the negated sum, `0xffe5b3a4` the check). The start-up rule (`0xffe5a188`): if the primary fails its sum (or its first-byte version nibble is 0), the mirror is tried; a valid mirror is copied over the primary (`0xffe5a3ae`); with neither valid the record is zeroed, given its default version, a fresh checksum, and copied to the mirror. The first record's current version nibble is `0xa` (created that way at `0xffe5a210`); a jump table at `0xffe5a858` upgrades older version nibbles 0 to 9 in steps. The second record's first byte is its version, `3` when created; the 0x24-byte record is written by `0xffe59e94`, which also copies words at `+0xe4` to `+0xec` and `+0xf0`.
- **Settings numbers in the first record (established, getter `0xffe59590`, jump table `0xffe595b0`, 179 entries, 47 of them reach the default arm).** Offsets are from the record start:

| Setting | Location in the record | Grade |
|---|---|---|
| 1 rack | bits 23:14 of the big-endian word at +0 (10 bits) | established |
| 2 bay | byte +2, bits 5:0 | established |
| 3 debug switches | bits 23:8 of the word at +0x1c (bytes +0x1d and +0x1e) | established |
| 5 | byte +0x20 | established |
| 8, 10 | byte +0x21, two bits each at bits 2:1 and 6:5 | established |
| 14 | byte +0x1f | established |
| 16 | byte +0x47 | established |
| 17 | byte +0x11 | established |
| 18 (the two-bit field) | byte +0x14, bits 3:2 | established |
| 19 (event subchannel) | byte +0x2f, bits 4:0 | established |
| 44, 55 | byte +0x46 bit 3, byte +0x16 bit 2 | established |
| 56, 57 | byte +0x16 bit 1, byte +0x16 bit 0 | established |
| 61, 62 | byte +0x17 bits 2:1, byte +0x17 bit 0 | established |

  The other entries reach the same record through computed offsets or multi-step paths and were not decoded one by one. The setter (`0xffe58758`, same jump structure) rewrites the field and then recomputes the record checksum and the mirror (`0xffe594de`). Which setting number is the partition id (14 in the table) rests on the request catalogue's name for that setting; the field itself is the byte shown.
- **Clock-calendar (established, read `0xffe5e32c`, write `0xffe5e5c4`).** Registers at `0x200007f8` upward, one BCD byte each: control at `0x7f8` (bit 6 freezes the read latch, bit 7 write enable; the low six bits are the two century digits: tens in bits 5:4 and hundreds in bits 3:0), seconds `0x7f9`, minutes `0x7fa`, hours `0x7fb`, day `0x7fc`, date `0x7fd`, month `0x7fe`, year `0x7ff`. The read routine converts BCD to a year as `thousands*1000 + hundreds*100 + year`, and the write routine adds `0x76c` (1900) to a tm-style year. The layout is that of the common 2 KB battery-backed RAM with calendar at the top of its window (the existing datasheet for that part family covers its register behaviour).
- **Settings in other records (unknown).** The contents of the 0x24- and 0xb4-byte records were not decoded.

## The alternate USB path: revision `0x81` (established)

The USB probe (`0xffe6db6c`) writes `0xb5` to the scratch register at offset `0x1b`, reads the revision register at `0x18` twice, and returns true only when it reads `0x81`. In that case the caller (`0xffe6dba8`) calls the pointer in RAM at `0x300006f4` with the slot number, base address, level and callback arguments, and does not run the USS-820-shaped initialiser at `0xffe70a98`. The initialised pointer value in this package is `0xffe70464`, a routine consisting of `link`, `unlk`, `rts` (`0xffe70464`): it does nothing. The neighbouring pointers at `0x300006c0`-`0x300006f0` all point at similar empty routines or at functions of the sibling drivers (`0xffe6fe00`-`0xffe6fe80`). So a controller that answers `0x81` leaves the L1's USB side inert, with no endpoint, descriptor or interrupt activity; the other revisions (`0x10`, `0x11`, `0x13` quiet, other high-nibble-1 values with a warning) take the register-driven path described in [usb-controller-registers.md](usb-controller-registers.md). Slot 0 responds at `0x40000000` (when a USB controller is counted); slot 1 at `0x60000000` only when the count is 2; their interrupt levels are the arguments 2 and 3 given to the driver (`0xffe55198`; see [the board contract](board-boot-contract.md)).

### The callback pointer in the other packages and sets

Found by the shape of the probe (a `cmp.l #$81,d0` after the write of `0xb5` to offset `0x1b`, followed by a call through a RAM word) and by the start-up block copy to RAM at `0x30000000` (`move.l #$30000000,d1` then `movea.l #source,a1`). In every ColdFire package of IRIX 6.5.23 onward (L1 1.26.5 to 1.48.1, both patches) the copied flash word for the callback is the address of the same 8-byte empty routine (`link`, `unlk`, `rts`), so the revision-`0x81` path leaves the USB side inert in all of them (established for the pointer values and the routine bytes; the caller was read in full only in package 1 of 1.44.0, the others are matched by shape, inferred).

| Package (1.44.0) | Callback RAM word | Copy source | Flash word | Empty routine |
|---|---|---|---|---|
| 0 `Base 1MB` | `0x300006c8` | `0xffe7f114` | `0xffe7f7dc` | `0xffe705b4` |
| 1 `Fuel/PE/O300 1MB` | `0x300006f4` | `0xffe7f0e4` | `0xffe7f7d8` | `0xffe70464` |
| 2 `MIPS 2MB` | `0x30000cf0` | `0xffea8ff4` | `0xffea9ce4` | `0xffe93610` |
| 3 `Legacy 2MB` | `0x30000e60` | `0xffeae322` | `0xffeaf182` | `0xffe9784c` |
| 5 `2MB` | `0x30000e8c` | `0xffeb02de` | `0xffeb116a` | `0xffe97fec` |

The 1.48.1 packages 0 to 3 have the same values; the 1.48.1 package 5 is a different build (copy source `0xffead98e`, routine `0xffe9527c`). The RAM word of package 1 moved between sets: `0x30000b4c` in 1.26.5 and 1.28.3, `0x300006ec` in 1.30.6, `0x300006e8` in 1.32.6, `0x300006e4` in 1.32.15 and `0x300006f4` from 1.34.2 on. In 1.24.8 and the two sets before it (6.5.20, 6.5.21 and 6.5.22) neither the `cmp.l #$81` nor the `moveq #$b5` shape occurs in any package, so those images have no revision-`0x81` branch of this shape; whether their USB probe is coded another way was not read (inferred).

## Not covered here

The front-panel display text path, the LED expanders and button reads, and the JTAG master's target are not re-read in this note; see [mbus.md](mbus.md) and the parallel-port table in [io-registers.tsv](io-registers.tsv). Their remaining questions are in [../open-questions.md](../../../questions/README.md).
