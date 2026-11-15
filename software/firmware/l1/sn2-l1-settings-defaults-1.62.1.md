# The ARM L1 1.62.1 settings record: how it is initialised, what setting 3 holds by default, and who changes it

The node firmware's first question to the L1 (request `0x0001`, [sn2-prom-link-1.62.1.md](sn2-prom-link-1.62.1.md)) is answered with the value of non-volatile setting 3, the debug switches. This note records where that value lives in the ARM L1, what a blank or reset L1 holds there, and which commands and requests change it, so an emulated L1 can answer as an unconfigured one would. Grades follow [AGENTS.md](../../../AGENTS.md).

Source: the `Linux L1 image` (package 4) of `l1-1.62.1.bin`, its `/stand/sysco/bin/l1`: ARM, 1,999,676 bytes, SHA-256 `22cb62871e447a394acbd662f4f49f3146580afec93839953b37a5354545bfc5` ([sn2-l1-1.62.1-packages.md](sn2-l1-1.62.1-packages.md)). The program keeps its routine names in its dynamic symbol table; addresses are virtual addresses (base `0x8000`). Everything below was read as disassembly, nothing was run. String addresses are virtual addresses too.

## Where the settings live

**Established (read in full: `nvramInit` `0x2c5ac` to `0x2d51c`, the repair routine `0x2d51c` to `0x2dc3c`, the header rebuild `0x2dca0`, `nvramGet` `0x2ff74`, `nvramSet` `0x2dda8`, the flush routine `0x2c490`).**

- The settings are a 0x2000-byte image in RAM (`0x32e2a8`), loaded at start from the file `/work/l1_NVRAM` and written back whole (seek to 0, write 0x2000 bytes) after every `nvramSet`. `/work` is a YAFFS flash partition (`/etc/fstab`, [arm-linux-boot-scripts-and-flash-tools.md](arm-linux-boot-scripts-and-flash-tools.md)), so the values survive a power cycle. The file is opened read-write with create (flags `0x1042`, mode `0666`); a read error only logs "Can't initialize L1 NVRAM from file". A missing or empty file therefore leaves the image all zero, which fails the header check below and takes the blank path.
- Integrity is a byte-sum rule, no magic number: the routine at `0x60eac` adds a region's bytes and the caller wants zero; `0x60f18` stores the two's complement. Header: 8 bytes at the image start, byte 0 high nibble non-zero, byte 1 (offset of the first record in 8-byte units) non-zero, byte 7 the checksum. Record 1: `0x3cc` bytes at `8 * header[1]` (checksum at `+0x3cb`), mirror copy at `+0x3cc`. Record 3: `0xb4` bytes at record 1 `+0x7e0` (byte 0 = 3, checksum `+0x893` from the image's record base), mirror at `+0x894`. Record 2: `0x24` bytes at `+0x798` (byte 0 = 1), mirror at `+0x7bc`. The record version is the high nibble of record byte 0; the current version is 10, versions 1 to 9 run an upgrade chain (jump table `0x2c7dc`) that rewrites older layouts.
- This is not the ColdFire layout: the ColdFire record is `0x70` bytes with the version in the low nibble of a big-endian first byte ([settings-window-blank-image.md](settings-window-blank-image.md)). The setting numbers are shared (the ARM getter has 0xd0 entries against 0xb3), the byte positions are not assumed equal.
- **Setting 3 (established).** `nvramGet(3)` (table entry for key 3 at `0x303b4`) loads the little-endian 32-bit word at record `+0x1c`, shifts it right by 8 and zero-extends to 16 bits: bits 23:8 of that word, that is bytes `+0x1d` (low half) and `+0x1e` (high half). `nvramSet(3, v)` (`0x2e298`) replaces bits 23:8 with the low 16 bits of `v`, then recomputes the checksums, refreshes the mirrors and writes the file (`0x2fe78` to `0x2ff68`). The result is always 0 to `0xffff`.

## What a blank or reset image holds

**Established.**

- **Blank, zeroed, erased or corrupt header.** `nvramInit` checks the header (`0x2c718` to `0x2c75c`); a failed sum, a zero version nibble or a zero offset byte calls `0x2dca0`, which zeroes the 8 header bytes, stores version 1 (byte 0 `0x10`), offset 1 and the checksum, logs "checksum Error - common header initialized", sets the record pointers and calls the record initialiser `0x2d51c` directly.
- **The record initialiser takes each record independently** (`0x2d51c`): a valid primary (sum zero and a non-zero version) is kept; otherwise a valid mirror is copied over it (log "nvram checksum error - core data invalid, using backup copy"); otherwise the record is zeroed and rebuilt from the defaults below (log "nvram checksum error - initializing core data."). Records 2 and 3 are handled the same way. When record 1 passed as it stood but its version nibble is not 10, the tail of the routine (`0x2db70` to `0x2dc38`) rewrites the nibble to 4, clears the 7 bytes at `+0x3c4`, logs "Rebooting L1 to clear up NVRAM revision confusion" and restarts the L1; a blank image does not take this branch.
- **Defaults of record 1.** The whole `0x3cc` bytes are zeroed first (`memset`, `0x2d5c8`); then only these stores are made (read in full, `0x2d5d8` to `0x2d760`): byte `+0x00` version 10 in the high nibble (value `0xa0`); `+0x20` = 5; `+0x04` bits 1, 2 and 3 set (`0x0e`); `+0x11` = 0; `+0x17` ends as `0x3f`; `+0x21` ends as `0x30`; `+0x2e` = `0x7e`; `+0x9a` = `0x40`; checksum at `+0x3cb`; mirror copy. Start-up then raises setting `0x11` to 2 if it is below 2 and setting `0x3d` to 2 if it is 0 (`0x2d16c` to `0x2d1a8`). The record-3 and record-2 rebuilds zero their bodies and store their version bytes (3 and 1) and checksums.
- **No default is written to `+0x1c`, `+0x1d`, `+0x1e` or `+0x1f`.** A scan of `nvramInit`, the header rebuild and the record initialiser for any access at those four offsets finds none, so after a rebuild they are zero. **Setting 3 is 0 on a blank or reset ARM L1, so request `0x0001` is answered with status 0 and the integer 0** (bit 11 clear). The ColdFire image's default is also zero for the same reason ([settings-window-blank-image.md](settings-window-blank-image.md): every byte not listed is zero, and `+0x1c` is not listed); its `debug` command reads and writes the same 16-bit field ([debug-console.md](debug-console.md)).
- **Mirror recovery matters.** A record whose primary fails the sum but whose mirror passes is restored from the mirror, so a partly damaged image keeps its previous setting 3; only an image with no valid copy goes to zero.

## Who changes setting 3

**Established (constant-key call-site scan of the whole program for `nvramSet` and `nvramGet` with key 3; each site read).**

- **Console `debug`** (handler `0xd2780`, command records at `0x180420` and `0x180464`, help text "get virtual debug switches" and "set virtual debug switches"; behind `SMPCmdCheckBrickType` for the module's brick type). With no value it reads setting 3 and prints "debug switches set to 0x%04x"; with a value it calls `nvramSet(3, value)` at `0xd2838`, reads it back and prints the same line. The value is the console's integer, truncated to 16 bits by the setter.
- **Request `0x1001`** from the node (handler at `0xcae2c`, `nvramSet(3, value)` at `0xcae94`; it needs at least two arguments, the second an integer or an immediate argument, and replies status 0). So the node firmware can store a debug word into the L1 as well as read one; whether the SHub 1 PROM ever sends it was not checked.
- **Request `0x0001`** reads it (`0xc8b78`, reply status 0 and one integer).
- A diagnostic compare (`0xeab84`, "debug switch mismatch: 0x%x") reads it against an expected value; it does not change it.

**Inferred.** Nothing else writes setting 3: sites that pass a computed key (not a literal) were not individually traced. The L1 itself shows no use of the value apart from these.

**`nvram reset` does not reset it (established for the handler, inferred for what the called routines write).** The console command `nvram reset` ("return L1 settings to factory default", handler `0xdc2e8`, case 0) prints "Changing NVRAM values to default settings" with a table (environmental monitoring on, temperature monitoring on, margin normal, syscom protocol ppp) and performs exactly: `EnvAuto(1)`, `EnvTemp(1)`, one call through a function pointer (`0x3315c0`, argument 2) and `nvramSet(8, 1)`. It stores no key 3. Forcing the blank path instead means removing or corrupting `/work/l1_NVRAM`; the refresh script described in the boot-scripts note erases the `/work` partition and so has that effect.

## What an unconfigured L1 reports

- Request `0x0001`: status 0, one integer, 0, unless a `debug` command or a request `0x1001` has stored something since. **Established** for the 1.62.1 Linux L1 image from the code above. For a particular deployed Altix L1 the stored value is **unknown**: only a read of that machine or a published log would show whether it was ever set. The sysco notes hold no such capture.
- The node firmware tests bit 11 of the value; zero takes the interconnect-discovery branch rather than the skip (the SAL boot-sequence page of the chips repository states what that branch does on a node with no NUMAlink peer).

## Not settled

- The upgrade chain for record versions 1 to 9 (jump table `0x2c7dc`): only its lack of any access at `+0x1c` to `+0x1f` was checked.
- Settings 61 and `0x11` in the ARM getter (table entries `0x30860`, `0x3050c`) were not read; the fix-ups assume the setting numbers match the ColdFire table.
- Whether anything on the node side sends request `0x1001`.
