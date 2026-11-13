# L1 firmware image layout: the 1.44.0 container, its ColdFire packages, how a controller picks and runs one, and what a C-brick needs

Read from the IRIX 6.5.30 `l1.bin` (L1 1.44.0, 15,988,636 bytes, sha256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`) and from the 6.5.30 host updater `usr/sbin/sysco/flashsc` (419,780 bytes, sha256 `1000c5530d2636929134684d2480651babe1e26230d12271dc0c4fbc8c075f95`, big-endian MIPS N32). Package numbers are the container's directory order. Packages 0, 1, 2, 3 and 5 are ColdFire MCF5206e flat images; 4 and 6 are the two ARM Linux packages and are not covered here. Grades follow [AGENTS.md](../../../AGENTS.md): **established** is read in full in the image and follows from what was read, **inferred** is a reasoned reading from partial evidence, **unknown** names the software test that would settle it. Addresses beginning `0xffe`/`0xfff` are the ColdFire address space; `flashsc` addresses begin `0x1000`.

This note joins facts spread over [package-container.md](package-container.md), [personalities.md](personalities.md), [board-boot-contract.md](board-boot-contract.md) and [flash-nvram-and-image-records.md](flash-nvram-and-image-records.md) with new reads of the updater and of the two other loaders. It records only what was measured again for this note; where an older note already holds the full reading it is linked rather than copied. A statement here that differs from an older note is a correction and says so.

## 1. Container and package headers

Established, recomputed from the file.

- The container starts with the marker `ESTFBINR`. The word at `0x2c` is the package count (7); from `0x30` follow seven big-endian (offset, length) pairs. The pairs are contiguous and the last ends at the file size.
- Every package starts with a 44-byte header of big-endian words, then (package 0 only, see below) a directory, then the flat body:

| Header offset | ColdFire packages 0, 1, 2, 3, 5 | Grade |
|---|---|---|
| `0x00`, `0x04` | `ESTF`, `BINR` | established |
| `0x08` | load address `0xffe00000` in all five | established |
| `0x0c` | last address (below) | established: last minus load plus one equals the body length in all five |
| `0x10` | `0xff000000` | established value; meaning unknown |
| `0x14` | `0` | established value |
| `0x18`, `0x1c` | the load and last addresses again | established |
| `0x20` | `0xffffffff` | established. The updater treats this word as the marker of the newer header form: `FlashValidateFileHeader` (`0x1000d710`) tests `+0x20 == -1` and otherwise uses a 32-byte header. |
| `0x24` | Header revision: `4` in packages 0, 1, 2 and 3; **`5` in package 5** (container offset `0x8e1250 + 0x24`). The two ARM packages, 4 and 6, carry `3`. The updater accepts a revision only if it is below 6 (`sltiu` at `0x1000d774`), so 5 is the highest it takes. | established |
| `0x28` | flags: package 0 = `2`; package 1 = `0`; packages 2, 3, 5 = `1` | established values. **Bit 1 (value 2) means a directory of 8-byte (offset, size) entries follows the header**: when the bit is set the updater adds `4 + 8*count` bytes to the header length. The validator (`FlashValidateFileHeader`, `0x1000d7f4`-`0x1000d82c`) applies the test to every accepted revision: it first adds `0x28` for a revision below 3 or `0x2c` for revision 3 and up (`0x1000d7bc`-`0x1000d7ec`), then reads the flags word, and the directory test follows in both cases. The two other sites that test the bit (`0x1000b9dc` and `0x1000bd54`) skip it for revisions below 3 (`sltiu` at `0x1000b9cc` and `0x1000bd44`). Every ColdFire package here has revision 4 or 5, so the difference does not matter for these files. Its listing text prints the flags word and then "Contains %d flash images" with an offset and size for each. Bit 0 is not read by any routine looked at; see U1. |

- **Package 0 is the one package with a directory after its header.** Its 44-byte header is followed by the container's count and seven pairs (`0x2c` to `0x67`), so its body starts at file offset `0x68`, not `0x2c`. The container header is package 0's header, and the container directory is package 0's flag-2 image directory. Established: header length `0x2c + 4 + 8*7 = 0x68` agrees with the first body word pair `0x301017fc 0xffe00400` at `0x68`. This refines the earlier reading that only the first 44 bytes were shared.
- Established: the 32-bit sum of the first ten, eleven, twelve or thirteen words of each package is not zero, so this note does not use a header word-sum as a check. The updater's own header checks (revision, length) are the ones read above.

### 1.1 Per-package measurements

| Package | Label (record word 11) | Container offset | Range length | Body at file offset | Body length | Last address | sha256 of the body |
|---|---|---:|---:|---:|---:|---|---|
| 0 | `Base 1MB image` | `0x000000` | `0x0efdd0` | `0x68` | `0x0efd68` | `0xffeefd67` | `3c8a7aa93eb38502289cf0c2921ea91e6800228005bd8abd67425ce615f7b364` |
| 1 | `Fuel/PE/O300 1MB image` | `0x0efdd0` | `0x0efd8c` | `0x2c` | `0x0efd60` | `0xffeefd5f` | `9db44f3bb06ec90475035781766e1ca7e924f15a0ed215cf855c1f491275fdd1` |
| 2 | `MIPS 2MB image` | `0x1dfb5c` | `0x1ac9ac` | `0x2c` | `0x1ac980` | `0xfffac97f` | `ac81f501f01c6789e756ab1f5ec69fadb1e4c04225511d541340831686e04e70` |
| 3 | `Legacy 2MB image` | `0x38c508` | `0x1b0c4c` | `0x2c` | `0x1b0c20` | `0xfffb0c1f` | `d13e4959222bad3308e71ba9c854efae01770afb942a9e562212eaeb19fff6fa` |
| 5 | `2MB image` | `0x8e1250` | `0x1b2458` | `0x2c` | `0x1b242c` | `0xfffb242b` | `5d4521de0baec59a9592cee0aa871cc67cd309c85f4dd303a290e937f49dbf8c` |

Established. The body is the flat image of flash: byte `n` of the body is the byte at `0xffe00000 + n`. The sha256 of each exact container range is in [package-container.md](package-container.md) for packages 0 to 4; for package 5 it is `562aa31da2e8d4dfbb94274fb383b078c78c25d84b5c84c747b11ed9d481a68f`.

## 2. The flat body: one memory map for all five packages

Established unless marked. Offsets are body addresses.

| Address range | Content | Same in all five? |
|---|---|---|
| `0xffe00000`-`0xffe003ff` | Vector table: word 0 initial stack pointer `0x301017fc`, word 1 reset entry `0xffe00400`; the other 254 words are the addresses of 254 four-byte `bsr.w` stubs at `0xffe0042c`-`0xffe00820`, one per vector: vector `k` (2 to 255) is `0xffe0042c + 4*(k-2)`. All stubs branch to `0xffe00824`, a routine that saves the registers, derives the vector number from the return address and ends in a `halt` word (`4ac8`) at `0xffe0083e`. The first stub also sits directly after the loader call in the reset entry (`jsr` at `0xffe00426`), so a return from the loader halts there too. Established in all five packages ([board-boot-contract.md](board-boot-contract.md)) | yes; the first `0x400` bytes hash equal (`ebe3fd5f988a...`) |
| `0xffe00400`-`0xffe00b37` | Boot loader: reset entry `0xffe00400`, retained-record set-up `0xffe00a80`, candidate scan `0xffe00994`, record check `0xffe008fc`, address validator `0xffe008ac`, hand-over `0xffe00840` | packages 0 and 1 byte-identical; packages 2, 3, 5 byte-identical to each other; the two groups differ in exactly nine bytes, the upper bound constant at `0xffe008c2`, `0xffe009ae` and `0xffe00ae2` (see 3.1) |
| `0xffe00b38`-`0xffe00ffb` | `0xff` fill | yes |
| `0xffe00ffc` | one word, value 1; the flash-state block (section 5.4) holds this address at its `+0x28`. Role unknown | yes |
| `0xffe01000`-`0xffe01007` | Candidate table: two entries, then `0xffffffff` terminators. Entry 0 = bank A start, entry 1 = bank B start | packages 0, 1 `0xffe10000`, `0xffe80000`; packages 2, 3, 5 `0xffe10000`, `0xfff00000` |
| `0xffe01008`-`0xffe0ffff` | `0xff` fill | yes |
| bank A at `0xffe10000` | one program copy, section 2.1 | |
| bank B at `0xffe80000` (1 MB) or `0xfff00000` (2 MB) | second program copy, linked for its own address | |

Bank span: `0x70000` in packages 0 and 1, `0xf0000` in packages 2, 3 and 5 (record word 2). The last bank ends at the last address of section 1.1.

### 2.1 Inside a bank

Established, from the first bank of each package.

| Bank offset | Content | Grade |
|---|---|---|
| `+0x00` | Pointer to the bank's 48-byte record (section 2.2) | established |
| `+0x04` | Initial stack pointer `0x301017fc`. The 1,024 bytes starting here are the vector table the loader copies to `0x30100000` | established |
| `+0x08` | Program entry address | established |
| `+0x0c`... | Vector table remainder, code, tables and initialised data | established |
| record | The 48-byte record | established |
| record end | The relocation table (section 2.3) | established |
| table end | `0xff` fill to the end of the span | established |

Entry addresses, established: package 0 `0xffe10b02` / `0xffe80b02`; package 1 the same; package 2 `0xffe10b02` / `0xfff00b02`; package 3 `0xffe10a92` / `0xfff00a92`; package 5 `0xffe10ab2` / `0xfff00ab2`. The first instructions are the same sequence in all five (read with a 68040-mode decoder, up to the heap set-up after the VBR load): RAMBAR `0x30100001`, stack `0x30101800`, status `0x2700`, MBAR `0x10000001`, the board set-up routine, VBR `0x30100000`. Board set-up routine addresses: package 0 `0xffe54ebc`, 1 `0xffe54b0c`, 2 `0xffe75320`, 3 `0xffe6ab78`, 5 `0xffe798d4`.

Established: each package's program in bank B is a separately linked copy. For example in package 1, 87,068 of 98,304 words after the first 64 KiB are identical, 6,937 differ by exactly `0x70000` and 4,299 differ otherwise ([coldfire-package-sections.md](coldfire-package-sections.md)). The programs copy initialised data from their own bank (package 1 `0xffe7f0e4`, found in the board set-up routine at `0xffe54ba6`), so bank B cannot run from bank A's bytes.

### 2.2 The record

Twelve big-endian words, sum zero modulo 2^32 (verified here for all ten records); words 0 to 11 as in [personalities.md](personalities.md#the-bank-record). Record addresses (first bank), build stamp and size class, established:

| Package | Record (bank A) | Record (bank B) | Build stamp (word 4) | Version | Word 3 used length | Word 8 bank sum | Word 10 |
|---|---|---|---|---|---|---|---|
| 0 | `0xffe7fd24` | `0xffeefd24` | `20060717181954` | `1.44.0` | `0x6fd54` | `0x313a7822` | 1 |
| 1 | `0xffe7fd1c` | `0xffeefd1c` | `20060717182038` | `1.44.0` | `0x6fd4c` | `0xa83e0909` | 1 |
| 2 | `0xffeaa28c` | `0xfff9a28c` | `20060717182050` | `1.44.0` | `0x9a2bc` | `0x61fc0ef0` | 2 |
| 3 | `0xffeaf75c` | `0xfff9f75c` | `20060717182113` | `1.44.0` | `0x9f78c` | `0x5140be21` | 2 |
| 5 | `0xffeb1798` | `0xfffa1798` | `20060717182146` | `1.44.0` | `0xa17c8` | `0xcce88870` | 2 |

For every package, the stamp of bank B equals the stamp of bank A, and word 8 equals the additive word sum of the bank from its start up to the record, recomputed here for both banks of all five packages (established; the bank B sums differ from the bank A sums shown, because the two copies are linked for different addresses). Word 4 is the address of the 14-character stamp string, not the digits themselves (for example `0xffe7a47c` in package 0 bank A; the loader dereferences it at `0xffe00a14`-`0xffe00a2e`). The stamp is a 14-digit string in the form year, month, day, hour, minute, second; the loader compares it as a string (section 3). Words 9 and 11 point to the supported-brick string and label (`CIPXYN&?RG` / `%4A?` / `CIPXYN&%6780VQE9A?RG` / `5O#=?` / `O5#=?`).

### 2.3 The relocation table behind each record

Established (recomputed over the container, all five packages, both layouts read from bank A and bank B).

The record's word 6 points to a relocation table that starts at the record's end. Header words (4 x 32 bit): revision 1, byte size, a checksum, and the **bank offset** (equal to the bank span: `0x70000` or `0xf0000`). Entries follow the header; the size counts the header. A zero word follows the table in all five (package 0 `0xffe7fd64`, package 2 `0xffebc97c`).

| Package | Table (bank A) | Size | Entries | Checksum word | Sum of all words of the table |
|---|---|---:|---|---|---|
| 0 | `0xffe7fd54` | `0x10` | none | `0xfff8ffef` | 0 |
| 1 | `0xffe7fd4c` | `0x10` | none | `0xfff8ffef` | 0 |
| 2 | `0xffeaa2bc` | `0x126c0` | 18,858 (12,785 kind 1, 6,071 kind 2, 2 kind 3) | `0x0a4f659b` | 0 |
| 3 | `0xffeaf78c` | `0x11490` | 17,694 (11,094, 6,598, 2) | `0xd3c0f08f` | 0 |
| 5 | `0xffeb17c8` | `0x10c60` | 17,170 (10,038, 7,130, 2) | `0x4b98b1e7` | 0 |

- **Checksum (established).** The 32-bit sum of every word of the table, header included, is zero in all five, so the checksum word is the negated sum of the other words. This settles the earlier open question about its range (U4).
- **Entry rules (established from `FlashImageData`, `0x1000dfdc`-`0x1000e150`).** The top byte of an entry is its kind and the low 24 bits are a byte offset from the start of the bank. Kind 1 adds the multiplier to the first (high) halfword of the addressed word, kind 2 adds it to the second halfword, kind 3 occupies two table words and exclusive-ors the addressed word with the table word after the entry. The multiplier is the bank offset's upper halfword (`0xf` for the 2 MB packages), and the routine is not applied when the multiplier argument is zero (`0x1000df44`), which is what the updater passes when the slot it writes is present in the file (`0x1000f4b0`).
- **What the table does (established for packages 2, 3 and 5).** Applying those rules to bank A's bytes from the bank start to the end of the table reproduces bank B's bytes with no mismatching word, over the whole program span (`0xac97c`, `0xb0c1c` and `0xb2428` bytes). So for these three a tool can derive either bank from the other; the updater does this only when the file lacks the target bank's image (section 5.2).
- **Packages 0 and 1 (established).** The table is empty, and it cannot derive one bank from the other: bank B differs from bank A in 12,351 words (package 0) and 12,269 words (package 1) of the span the two have in common. Their second bank must be shipped in the file. The earlier statement that the table lets a tool derive one bank's bytes from the other's holds only for packages 2, 3 and 5.

## 3. Reset, and how the controller selects what to run

This is the only selection the controller makes. There is no step in the controller that chooses between packages: a controller holds exactly the one package (or the other) that was last written to its flash, and the loader chooses between that package's two banks. Choosing a package is the host updater's job (section 5.1).

### 3.1 Loader, in order

Established for packages 0 and 1 from the listing and for 2, 3 and 5 from the byte comparison in section 2 (the loaders differ only in the three upper-bound constants, below). Reading detail for package 1 is in [board-boot-contract.md](board-boot-contract.md); additions and corrections follow.

1. Reset vector `0xffe00400`: RAMBAR `0x30100001`, stack `0x30101800`, status register `0x2700`, call `0xffe00a80` with MBAR base `0x10000000` and the register `a6` as a second argument (stored in the retained record at `+0x8`).
2. `0xffe00a80` keeps a 0x4d8-byte retained record at `0x30101990`, marker `0xa1b2c3d4`. If the marker is missing it zeroes the record and writes it. It then reads the byte at `0x10000040` (`move.b $40(a0)` at `0xffe00ac8`, `a0` being the base `0x10000000` passed by the reset entry) and tests bit 5. **This is not the MCF5206e reset-status register.** Nothing writes `MBAR` before the read: the only `movec` in the loader loads `RAMBAR` (`0xffe0040c`), and the first `MBAR` write is in the program entry (`0xffe10b2e`, second copy `0xffe80b2e`). The manual says the `MBAR` valid bit is cleared at system reset and the base field is compared only while the bit is set (manual page 8-2), and that `CS0` is asserted for every access except CPU-space and internal-peripheral accesses until `CSMR0` is first written (page 9-8, section 9.3.2). The loader never writes `CSMR0`, so the read is an external byte cycle answered through `CS0`, that is, by the boot flash. Body offset `0x40` is `0xff` in all five packages (the high byte of vector 16, which is `0xffe00464`; the next byte, `0xe0`, also has bit 5 set), so bit 5 reads **set** on every start. The branch if the bit is clear (`beq` at `0xffe00ad0` to `0xffe00b02`, which clears record word `+0x10`) is therefore never taken by the shipped flash. Established for the loader code and the flash bytes; inferred for the flash wiring (that the low address bits reach the part unchanged, which no image states; U2). If the bit is set and the record word at `+0x18` is a word-aligned address in range (`0xffe00ad6`-`0xffe00aec`) whose record passes the check, that address is used without scanning (`0xffe00af2`-`0xffe00b00`).
3. Otherwise `0xffe00994` scans. It starts at the table `0xffe01000`, advances to the first `0xffffffff` word, and fails with no image if that word reaches the upper bound. It then walks entries **backwards**, masks each entry with `0xffffff00`, runs the record check (`0xffe008fc`) and, for a valid entry:
   - a set bit 0 in the table word forces that entry immediately (`0xffe009f8`);
   - otherwise it compares the entry's build stamp with the best so far as signed characters, byte by byte (`0xffe00a2e`-`0xffe00a54`): a smaller stamp is not taken, a larger one replaces the best, an equal one keeps the earlier-seen candidate. Because the walk is backwards the earlier-seen candidate is the later table entry, bank B.
4. The record check (`0xffe008fc`) returns nonzero (reject) in three cases: the retained record is valid and its word at `+0x10` equals the candidate address (`0xffe00916`-`0xffe0091e`); the candidate address, or the record pointer in its first word, is below `0xffe00000`, at or above the upper bound, or not word-aligned (`0xffe008ac`-`0xffe008ee`); or the 32-bit sum of the record words is nonzero (a record whose type halfword is below 3 skips its first four bytes). **The first case matters:** the retained word at `+0x10` holds the image that was started first since the last cold reset, so after a watchdog reset the scan skips that image and takes the other bank, unless the word at `+0x18` names an image.
5. Hand-over (`0xffe00840`): the entry address is bank word 2. The loader updates the retained record (`+0x14` goes to `+0xc`, the new entry goes to `+0x14`, `+0x18` is cleared, `+0x10` is set if empty), copies 1,024 bytes from bank `+4` to `0x30100000`, loads VBR with `0x30100000` and calls the entry. If the call returns, or no image was found, execution reaches a `halt` word (`0xffe00b1c`). In particular, if the only valid bank is the one recorded at `+0x10`, a scan after a watchdog reset finds no image and the loader halts.

Upper bound constants (established, the nine differing bytes of section 2): `0xfff00000` in packages 0 and 1 (`0xffe008c2`, `0xffe009ae`, `0xffe00ae2`), `0xfffffff8` in packages 2, 3 and 5. This is the only loader difference between the 1 MB and 2 MB packages. Hence bank B at `0xfff00000` of a 2 MB package is acceptable to the 2 MB loader and would be rejected by the 1 MB one.

Consequence for a clean flash image of any of these five packages: both banks carry equal stamps, so a start with no valid retained record (power-on with the SRAM contents lost, which makes the marker test fail) runs **bank B** (the later table entry), not bank A, and records it at `+0x10`. The main task clears `+0x10` later in the same start (section 3.4), so the skip does not last: a later start with the record intact runs bank B again unless the reset reaches the loader before that clear. Established from the algorithm and the recomputed stamps; it was not run. That the SRAM survives a watchdog or external reset is inferred from the program's use of it: the manual calls the SRAM contents undefined after a hardware reset (page 5-3) and says a watchdog reset resets the internal registers as a normal reset does (page 6-84, section 6.11.3), but names no retention guarantee (U10).

### 3.2 What the retained record tells the next boot

Established: the loader writes the previous (`+0xc`), current (`+0x14`) and first-since-record-creation (`+0x10`) image addresses, and the marker check zeroes the whole record when the marker is missing. The `+0x10` word is also cleared by the main task once it runs (section 3.4). The loader's bit-5 clear arm is never taken with the shipped flash (step 2), so it does not clear the word. (Earlier text said a cold reset clears it; that rested on reading the byte as the reset-status register and is withdrawn.)

**The writer of `+0x18` (established, package 1).** The restart routine at `0xffe56654` (reached from the `reboot_l1` request `0x4002` at `0xffe13d76`, reason 1) loads `a5` with `0x30101990` (`0xffe56674`) and stores its image argument at `$18(a5)` (`move.l d7,$18(a5)` at `0xffe566e0`); it also writes the argument and its complement over the first two words of the vector copies at `0x30100400` and `0x30100000`. A search for the literal `0x301019a8` cannot find this store, because the address is formed as base plus displacement. The caller passes the RAM word at `0x30000928` (`0xffe13d84`), which is word `+0x10` of the flash-state block of section 5.4, the **running bank** word. So a controller restart asks the loader to start the bank the program is running from. Established from the code and the block layout. The other four packages pass the word at their own block base plus `0x10` (package 0 `0x300008fc`, package 2 `0x30000f24`, package 3 `0x30001094`, package 5 `0x300010c0`, from [brick-request-handlers.md](brick-request-handlers.md)), which agrees with the per-package block addresses of section 5.4. That nothing else writes the running-bank word was not searched. The routine then masks interrupts and spins until the watchdog fires; it halts instead if the watchdog is not enabled (`0xffe566bc`).

**What the loader does with it.** Step 2 uses `+0x18` only if the record check passes, and the check rejects an address equal to `+0x10` (step 4). So after such a restart, if the running bank is the one recorded at `+0x10` (the first image started since the record was created), the request is rejected and the scan runs, which skips that bank and takes the other; if the running bank is not the `+0x10` image, the same bank is started again. Inferred from the two routines read together; not run. The restart routine's other reasons (for example `0x3e6` at `0xffe56718`) do not store `+0x18`.

Inferred: the record sits in the on-chip SRAM, which is why it can survive a reset (U10). The record is also read by other routines (for example `0xffe565c2`, `0xffe56674`, `0xffe56aee`) as the reset-cause and crash record; those are not part of this note.

### 3.3 After the loader: how a package reaches its board personality

Established ([personalities.md](personalities.md) and [personality-board-init.md](personality-board-init.md), read again for the call sites above). The program starts the same kernel in all five, reads the board identification records and sets a one-byte type (`0x3f` until set), then calls a dispatcher with that type. A type the package does not list takes a fallback that logs a defaulting message, stores `?`, and runs a blank initialiser. The supported-brick string is tested by the host updater (5.1). No controller routine that reads record word 9 was found, but the whole program was not searched (U7). If the controller does not test it, a package running on a brick it does not list still boots, with the blank personality.

### 3.4 Who clears `+0x10`, and which bank each start runs

Writers of `+0x10` (the table is read from the listings; the main-task clear is the one [l1-board-contract.md](../../../devices/l1_board/README.md) reads in full):

| Writer | When | Grade |
|---|---|---|
| The loader's clear arm | only if the bit-5 read returns clear, which the shipped flash prevents (step 2) | established that it is not taken |
| The main task, in the image the loader started | once per start: the call pair that clears the word (through the helper that clears `0x301019a0`) is found by byte search in all five packages, immediately before the watchdog arm in packages 0 and 1 | established for the clear and the arm in all five packages: the call after the clear is the same routine shape everywhere (package 0 `0xffe10764` then `0xffe56684`, package 1 `0xffe10764` then `0xffe562d4`, package 2 `0xffe10764` then `0xffe76ae8`, packages 3 and 5 `0xffe10704` then `0xffe69620` and `0xffe7837c`): it tests a RAM flag, clears it, and writes `0xfe`, `0x00`, `0x55` and `0xaa` to MBAR offsets `0x41` to `0x43` (SYPCR, SWIVR and SWSR of the manual, sections 8.3.2.7 to 8.3.2.9) |
| Two other callers of the same clear helper, and one inline clear, in package 1 | each is followed directly by a restart request with reason 1 and image argument 0; one of the callers sits in the routine that handles a missing MAC-address EEPROM | inferred (package 1 listing only); the other callers in the other four packages were found by byte search and not read (U13) |

The bank each start runs, worked from the algorithm, the equal stamps and the clear above. None of these was run.

| Start | Retained record | Bank run |
|---|---|---|
| Power-on with the SRAM contents lost | marker missing, record zeroed | **B**, recorded at `+0x10` |
| Reset after the main task's clear, record intact, `+0x18` empty | `+0x10` empty | **B**: both banks pass, stamps tie, the later table entry wins |
| Reset after the clear, record lost | marker missing | **B** |
| Reset between hand-over and the clear, record intact | `+0x10` names B | **A**, because the record check rejects B |
| Reset after a controller restart through `0x4002` after the clear | `+0x18` names the running bank | that bank again (3.2) |
| Reset after a reason 1 restart with image argument 0 (the other callers above), record intact | `+0x10` empty, `+0x18` zero | **B**, whichever bank was running |

A clean flash therefore runs bank B on each ordinary start, and bank A only after a reset that reaches the loader between hand-over and the clear. Nothing shown here makes such a reset happen (U12). This replaces the reading in 3.1 that a record left intact keeps the first bank, and the statement in 3.2 that `+0x10` is cleared only when the record is lost.

## 4. Reset-time hardware the five packages share

Established: the chip-select routine (package 1 `0xffe549d8`, package 0 `0xffe54d88`, package 2 `0xffe751ec`, package 3 `0xffe6aa44`, package 5 `0xffe797a0`) is the same 96 instructions in each, with the same operands except addresses in the RAM window and branch and call targets. The contents (CS0 flash at `0xffe00000`, 16-bit, CS1 `0x20000000`, CS2 RAM `0x30000000`, CS3 to CS5 at `0x40000000`/`0x50000000`/`0x60000000`) are in [board-boot-contract.md](board-boot-contract.md). Two points specific to this note:

- **Flash size is detected, not configured.** The routine writes CS0's mask as `0x001f0000` (2 MB) and rewrites it as `0x000f0000` when the first 1,024 words at `0xffe00000` equal the first 1,024 words at `0xfff00000`. The identity routine then stores the class in a RAM byte: 1 when the mask register at MBAR+`0x68` reads `0x000f0000`, 2 when it reads `0x001f0000` (package 1 `0xffe5527e`-`0xffe552a0`). The routine has the same shape in all five packages but the RAM byte is at a different address in each: package 0 `0x300002cf` (base `0x300002cc` loaded at `0xffe5561c`, byte stored at `0xffe55650`), package 1 `0x300002fb` (base `0x300002f8`, store at `0xffe552a0`), package 2 `0x300008cf` (base `0x300008cc`, `0xffe75ab4`), package 3 `0x30000a5d` (base `0x30000a5c`, store at `0xffe6b150`), package 5 `0x30000be1` (base `0x30000be0`, store at `0xffe79ebc`). Established from the bytes of each package. So a 1 MB part that mirrors itself is class 1; a 2 MB part, or a larger one whose upper half differs from the lower, is class 2. For package 2 (body longer than 1 MB) the words at `0xfff00000` are bank B's first words (`0xfff9a28c`...), different from the vector table at `0xffe00000`, so the probe reports 2 MB on a 2 MB part.
- That runtime class selects the flash geometry the flash task uses (section 5.4). It is distinct from the package's static class (record word 10 and the flash-state block word at `+0x14`), which the controller reports to the updater.

## 5. What the updater writes and where

Read from `flashsc` 6.5.30. Function names are the symbol names the file carries; addresses are file addresses. `FlashUpdateRemote` is at `0x1000ebf0`, `FlashUpdateRemoteBoot` at `0x100103c0`, `FlashUpdateRemoteRaw` at `0x100112e0`, `FlashGetImageInfo` at `0x1000c270`, the bank parser at `0x1000d060`, `FlashValidateImage` at `0x100172a0`, `FlashUseNewestImage` at `0x1000e900` with the matcher at `0x1000e920` and the selector at `0x1000ea20`, `FlashImageData` at `0x1000dea0`.

### 5.1 Which package is offered to a controller

Established.

1. `FlashUpdateRemote` sends request `0x0072` (flash status, arguments 0 and 5) to the controller and keeps reply argument 0 (the image family) and 6 (running-bank selector); when the reply has eleven or more arguments it also keeps 7 (brick byte), 8 (static class), 9 and 10 (bank A and B addresses), otherwise those four are zero (`0x1000ee14`-`0x1000f04c`). For the producing routine in package 1 (`0xffe3c166`), see [package-container.md](package-container.md).
2. `FlashGetImageInfo` builds one 532-byte info record per package of the file. A flat ColdFire package (load address `0xffe00000`) gets family 1 (`0x1000c8a8`).
3. The selector scans the info array, calling the matcher on each. The matcher (`0x1000e920`) rejects a package unless: its family equals the reply's family; when the brick byte is nonzero and the package has a supported-brick string, `strchr` finds the byte in it (the `#` rule of [package-container.md](package-container.md) appends `=`); and when the reply's static class is nonzero and the package's class (record word 10) is nonzero, they are equal (`0x1000e9c4`-`0x1000e9ec`). The first package that passes is used; the scan runs forward unless `FlashUseNewestImage` has set its flag, in which case it runs backward (`0x1000ea8c`).
4. With a force argument of `-1` the class filter is dropped (`0x1000f1d0`-`0x1000f1ec`). Inferred: this is the `-f` path.

Consequence for a C-brick (brick byte `C`): only packages 0 and 2 list `C`, and they have classes 1 and 2, so the class reported by the controller decides: class 1 gets package 0, class 2 gets package 2. Packages 1, 3 and 5 are never offered to a `C` controller without the force argument. Established from the strings and the matcher; not run.

### 5.2 The write

Established from the call shape and set-up; the controller side is in 5.4.

- **Parsing the file.** For a flat ColdFire package `FlashGetImageInfo` takes the header length from section 1, the load address `0xffe00000` and the body length, and splits the body: (a) a **boot block** at `0xffe00000`, length the smaller of the remainder and `0x10000` (`0x1000c8b0`-`0x1000c950`); (b) **image A** at `0xffe10000`, parsed by `0x1000d060` and checked by `FlashValidateImage`; (c) **image B** at `0xffe80000` or `0xfff00000`, parsed the same way. If the A parse fails the cursor advances by `0x70000` or `0xf0000` (`0xf0000` when more than `0xf0000` bytes remain, `0x1000c9a8`-`0x1000c9cc`). Each parsed image keeps its start, the record fields (word 3 used length, word 8 sum, word 9 string, word 10 class, word 11 label) and the relocation table pointer. The A/B header, revision, length and string compare ("A/B supported brick types mismatch", "A/B revision mismatch" and others) are made between the two records.
- **Which bank is written.** Reply argument 6 is zero when the running bank is bank A (the flash-state block compares its running-bank word with its bank A word; section 5.4). With argument 6 equal to zero the updater writes image slot 1 (B); otherwise slot 0 (A) (`0x1000f27c`). So an update always replaces the bank the controller is **not** running from.
- **Target address.** The address sent is the one the controller reported for that bank (reply arguments 9 and 10), or when those are absent `0xffe10000 + slot * 0x70000` (`0x1000f574`-`0x1000f584`) for a ColdFire info record. It is compared with the file slot's own start address (`0x1000f5a8`); on a mismatch the updater prints a message and stops unless the force argument is `-1`. For the 2 MB packages the controller reports `0xfff00000` for bank B (section 5.4) and the file slot starts there, so they agree; the 1 MB formula `0xffe80000` is not used for them because the reported value is nonzero.
- **Length.** The erase and write length is the first valid image's `slot + 12`: record word 3 (plus 4 if the record type is below 4) plus the relocation table's byte size (`0x1000d154`-`0x1000d1a0`). Inferred from the arithmetic: bank start through the record, then the relocation table, which for package 0 is `0x6fd54 + 0x10 = 0x6fd64` bytes, ending at `0xffe7fd64`. The zero word after the table (`0xffe7fd64`) and the `0xff` fill to the end of the span are not covered by the length. For package 2 the written span is `0x9a2bc + 0x126c0 = 0xac97c` bytes.
- **Erase** (request `0x0074`): arguments address and byte count; slices of one tenth of the length unless a slice length is given; reply-wait argument 140 (write requests use 30) (`0x1000f6b8`-`0x1000f79c`).
- **Write** (request `0x0073`): arguments address plus the number of bytes already written, and a payload; chunk size 2,000 bytes for a ColdFire info record (the info record's family word is 1; `0x1000fb5c`) or 4,096 otherwise, or a caller-supplied size. The chunk is fetched by `FlashImageData` with the relocation multiplier zero, so the file's own bytes go to flash unchanged. Every request is attempted twice with a 15-second sleep between.
- **Check.** After the last chunk the updater sends `0x0072` again and reads the flash-status reply. Messages "Image 'A' is INVALID" and "revision mismatch" are the failures ([host-tool-options.md](../../userland/host-tool-options.md)).
- **Not written** by this path: the boot block at `0xffe00000`-`0xffe0ffff` (the loader, the table at `0xffe01000` and the vector area) and the other bank. Inferred from the address and length arithmetic above: the erase starts at the slot address and the length ends at the relocation table.

### 5.3 The boot block variant

`FlashUpdateRemoteBoot` (`0x100103c0`, reached from the boot options; the option mapping was not read) writes the boot block of the info record: length `info + 156` (64 KiB), data from `info + 144`, and the info record's `+148` holding `0xffe00000`. Established from `0x10010910`-`0x10010934` and `0x10010a04`: the **addresses it sends start at zero** (the start word `68(sp)` is cleared at `0x1001091c`) and advance by the chunk offset; the `0xffe00000` word is passed to `FlashImageData` but is not added to the request address. Established about the controller: its erase and write handlers in package 1 add a bias word (initial value zero, no writer found) and reject any address below `0xffe00000` with error `0xfffffaec` (section 5.4). So as read, a boot-block write addressed from zero reaches the package 1 handlers as an out-of-range request. Whether another ColdFire handler, a different task, or a bias set elsewhere makes this path valid is unknown (U3). The boot block is therefore not shown to be field-updatable on these ColdFire packages; this does not affect booting.

### 5.4 The controller side: what the erase and write requests accept

Established, package 1 (`0xffe3c41e` erase, `0xffe3c4c2` write; helpers `0xffe704d4`, `0xffe706c8`, `0xffe7046c`, `0xffe7067c`). The helpers were not read in the other packages.

- **Flash-state block**, copied from initialised data inside each bank. Its RAM address differs by package: it is the block's address in the bank minus the copy source of the board set-up routine (the `movea.l #src,a1` that precedes the copy to `0x30000000`) plus `0x30000000`. Package 0: source `0xffe7f114`, block `0x300008ec`. Package 1: source `0xffe7f0e4`, block `0x30000918` (so the block is at `0xffe7f0e4 + 0x918 = 0xffe7f9fc`). Package 2: source `0xffea8ff4`, block `0x30000f14`. Package 3: source `0xffeae322`, block `0x30001084`. Package 5: source `0xffeb02de`, block `0x300010b0`. The remaining RAM addresses in this subsection are package 1's; for another package subtract `0x30000918` and add its block address. Established. Words, in order from `+0x00`: base `0xffe00000`; upper limit (`0xfff00000` in packages 0 and 1, `0xfffffff8` in packages 2, 3, 5); bank A address `0xffe10000`; bank B address (`0xffe80000` or `0xfff00000`); **running bank** (the bank this copy of the program lives in, so `0xffe10000` when the program runs from bank A and the B address when from B); static class (1, or 2); bank span (`0x70000` or `0xf0000`); candidate table `0xffe01000`; two zero words; the address `0xffe00ffc`; zero. Addresses of the initial data (bank A copy, bank B copy): package 0 `0xffe7fa00`, `0xffeefa00`; package 1 `0xffe7f9fc`, `0xffeef9fc`; package 2 `0xffea9f08`, `0xfff99f08`; package 3 `0xffeaf3a6`, `0xfff9f3a6`; package 5 `0xffeb138e`, `0xfffa138e` (found by pattern in the body; the packages 3 and 5 blocks are at 2-byte alignment). In every block the running-bank word is the start of the bank that holds that copy (verified for all ten blocks). The update reply (request `0x0072`) argument 6 is "running bank differs from bank A", argument 8 the static class, arguments 9 and 10 the bank A and B words ([package-container.md](package-container.md)).
- **Geometry state** at `0x30000944`. Initialised by `0xffe7046c` from the runtime class byte (`0x300002fb`, section 4): class 2 selects the sector-end table at `0xffe70840` (35 sectors and an upper limit of base plus `0x1ffff8`); otherwise the table at `0xffe707f4` (19 sectors) and the limit stays `0xfff00000`. The tables hold the last offset of each sector. The 1 MB table lists fifteen 64 KiB sectors ending at `0xeffff`, then sectors ending `0xf7fff`, `0xf9fff`, `0xfbfff` and `0xfffff` (32, 8, 8 and 16 KiB, a top-boot part). The 2 MB table lists thirty-one 64 KiB sectors ending at `0x1effff`, then `0x1f7fff`, `0x1f9fff`, `0x1fbfff`, `0x1fffff`.
- **Erase** (`0xffe704d4`): the address argument plus a bias word at `0x30000950` (initial value zero) must be at least the base and, with the length added, below the limit; otherwise the helper returns `0xfffffaec`. Within range it erases every sector the span overlaps, as a sequence of sector-erase commands with erase-suspend polling ([flash-nvram-and-image-records.md](flash-nvram-and-image-records.md)). The whole-flash erase `0xffe7067c` passes base minus bias and span, so the address is a flash address, not an offset (bias zero).
- **Write** (`0xffe706c8`): the same range test; then halfword writes compared with the source, a program command for each unequal halfword (`0xffe3c4c2` dispatch passes address, payload pointer, length, error cell). Returns `0xfffffaeb` on a program failure and `0xfffffaea` on a verify mismatch.
- No request restricts the target to the non-running bank; that restriction is in the updater. Established for the controller side: the helper does not read the running-bank word.

## 6. What it takes to boot package 0 or package 2 for a C-brick

Everything below is what the software of these packages needs from the machine around it, restricted to what was read. It assumes a ColdFire MCF5206e-equivalent processor with its on-chip peripherals; chip-level register behaviour is in [board-boot-contract.md](board-boot-contract.md) and the User's Manual rev. 1 (hint).

1. **Flash contents.** The package body at `0xffe00000` on the CS0 chip select, as a 16-bit port. Package 0: `0xefd68` bytes from file offset `0x68` (ends at `0xffeefd67`). Package 2: `0x1ac980` bytes from file offset `0x2c` (ends at `0xfffac97f`). The rest of the part may be `0xff`. Established.
2. **Flash size.** Package 2 is longer than 1 MB (`0x1ac980` bytes), so it needs a 2 MB part, and the part's upper megabyte must differ from the lower one in its first 1,024 words, which it does (section 4). Package 0 fits in the first megabyte; on a 1 MB part that mirrors itself the probe reports class 1, on a larger part with blank upper space it reports class 2 and the flash task then uses the 2 MB sector table and limits (5.4). Established for the logic.
3. **Reset behaviour.** The first two body words (stack, entry) must be read as the reset vector, so flash must answer at address zero at reset. Out of reset `CS0` is the global chip select and is asserted for every external access (manual page 9-8), so the boot flash answers whatever address the processor presents, including zero and `0x10000040`; the port width and acknowledge mode of that global select come from the `IRQ1`, `IRQ4` and `IRQ7` pins at reset, which no image states (U2). The loader needs the on-chip 8 KB SRAM at `0x30100000` (RAMBAR, set by the loader itself). Its read at `0x10000040` before the program sets MBAR is **a flash read, not a read of the reset-status register** (section 3.1 step 2): the flash must return, for byte offset `0x40` of the part (the high byte of vector 16), a value with bit 5 set, which the shipped images do (`0xff`). Nothing in the loader reads the real reset-status register; the kernel does, later (`0xffe565b4`).
4. **External devices before the program runs.** The program configures CS1 (`0x20000000`, 8-bit, 64 KB window), CS2 (RAM at `0x30000000`). The size is probed (package 1 `0xffe54a90`-`0xffe54ac6`): the routine saves the word at `0x3001fffc`, clears it, writes all ones to `0x3007fffc`, and if `0x3001fffc` still reads zero keeps 512 KB. If it does not read zero, the two addresses alias, so it sets the CS2 mask to `0x10000` and the recorded size to `0x20000` (128 KB). It then restores the saved word, CS3 to CS5 (8-bit devices; quad UART at `0x50000000` unless the type is `G`). Details and register values are in [board-boot-contract.md](board-boot-contract.md) and [quad-uart-and-serial-links.md](quad-uart-and-serial-links.md).
5. **Type identification.** The brick-type byte for a C-brick is `C`. It comes from, in order: the board identification record `0xff` (a name starting `IP35` gives `C`), record `0xfe`, `0xfd`/`0xfc`, then the persistent setting with key 9 (stored `C` is accepted). Packages 0 and 2 run an identity routine of the same shape (package 0 `0xffe55604`, package 2 `0xffe75a68`; their record loaders are different routines at different addresses: package 0 `0xffe6bd54`, called at `0xffe55658`, package 2 `0xffe8da88`, called at `0xffe75abc`, against package 1's `0xffe6bc04`); the first prefix compare in each tests the name `IP35` (package 0: the literal at `0xffe55f8c`, compared at `0xffe55678`; package 2: `0xffe763f0`, compared at `0xffe75adc`), followed by `KCARL1`, `IP34`, `POWERON`, `X2PWR`, `NIPWR` and `NPWR` as in package 1. The identification records are read from the board EEPROM through the M-Bus engine ([mbus.md](mbus.md), [board-records.md](board-records.md)); without them the type falls to the stored setting, and without that to `?`, the blank personality (section 3.3).
6. **C-brick board code.** For `C` the dispatcher of package 0 (`0xffe44394`) calls the node-board initialiser at `0xffe45504` (jump-table entry for `C` at `0xffe44424`); package 2 first tries its first dispatcher (`0xffea38f4`, which handles `7`, `6`, `9`, `%`, `A`) and then the second (`0xffe4a404`), whose jump table has the same offsets as package 0's but whose `C` entry (`0xffe4a494`) calls a different initialiser, `0xffe4b574`. The initialisers sit at different addresses, so the addresses of package 0 do not carry over to package 2 (whether the two bodies match was not compared) The code those initialisers install and what it drives (the processor-module expanders, the I2C bus select, power-up sequence, node reset) are in [personality-base-package.md](personality-base-package.md) and [power-reset-cbrick.md](power-reset-cbrick.md). The expander and bus-select hooks, EEPROM table counts and the PIMM status read are specific to `C`; package 2 contains that code (source unit 12 literals) in addition to the Origin 300 modules.
7. **Difference between the two packages that matters at boot.** Package 0's loader has upper bound `0xfff00000` and bank B at `0xffe80000`; package 2's loader has `0xfffffff8` and bank B at `0xfff00000`; package 2's static class is 2. Both banks of both packages carry equal stamps, so a clean flash runs bank B (3.1).

What was not established about booting a C-brick: what the actual hardware presents on the board EEPROM, expanders and node link (U6); that no further check outside what was read stops the boot.

## 7. Unknowns and the software test that settles each

| ID | Unknown | Test |
|---|---|---|
| U1 | Meaning of header flag bit 0 (set in packages 2, 3, 5, clear in 0 and 1) | Search every `flashsc` build from 6.5.20 to the newest patch, and the ARM-side flash utilities, for reads of the header word at `+0x28` that test bit 0; if none, the bit is carried but unread. Also compare against the L1 images of 6.5.16 to 6.5.19 (revision 3 headers) where the 2 MB image exists. |
| U2 | (chip side settled, [empty-board-boot-path.md](empty-board-boot-path.md) section 5) Whether the fitted board returns the shipped flash byte for the loader's read at `0x10000040` (section 3.1 step 2). The chip side is answered by the manual (global `CS0`, pages 8-2 and 9-8); what is open is the board: the flash address wiring, and the port width the `IRQ1`/`IRQ4`/`IRQ7` pins select for the global select. | Measured so far: byte `0x40` of the vector table is `0xff` and byte `0x41` is `0xe0` (bit 5 set in both) in every ColdFire package of every `l1.bin` from IRIX 6.5.16 to 6.5.30 and in the 3.24 CD and patches 5762 and 7149 images; the 6.5.12 to 6.5.15 containers use a layout this note did not parse. To close it, read the loader of each of those older images and confirm it tests the same byte and bit; the board wiring itself is outside the images. |
| U3 | Which controller path accepts the boot-block request (`FlashUpdateRemoteBoot`), given the package 1 handlers reject addresses below `0xffe00000` | Read the request `0x0073`/`0x0074` handlers of the ARM Linux L1 packages and the flash task of package 0 and 2 (same dispatch shape, not read here); search for any writer of the bias word at `0x30000950`. If an ARM or other task accepts offset zero, the boot-block variant targets that controller family. |
| U4 | (settled) Relocation entry semantics and checksum range | Settled in section 2.3: the rules reproduce bank B from bank A over the whole program span for packages 2, 3 and 5 with no mismatching word, and the whole-table sum is zero. Residual test: apply the same rules to the 2 MB packages of the other releases (6.5.22 to 6.5.29 package 2, package 3, patch images) and confirm zero mismatches. |
| U5 | (settled for package 1) Writer of the retained-record word at `+0x18` | Settled in section 3.2: the restart routine (`0xffe56654`, store at `0xffe566e0`), with the running-bank word as its argument. Residual test: confirm the same store in the restart routines of packages 0, 2, 3 and 5 (the packages' `0x4002` handlers pass the running-bank word, section 3.2) and search all five for any other store through `0x30101990 + 0x18`. |
| U6 | The state a C-brick presents to the package: EEPROM records `0xff` to `0xfc`, expander states, PIMM status | Collect from the package the exact byte requirements of the identity routine (package 0 record loader `0xffe6bd54`, package 2 `0xffe8da88`, and their prefix compares) and of the C initialiser (package 0 `0xffe45504`, package 2 `0xffe4b574`, and callees); each unknown register read becomes a table row. No hardware measurement is required: the reads are in the images. |
| U7 | Whether the controller ever rejects a package whose supported-brick string lacks its type | Search all five packages for loads of record word 9 (pointer at record `+0x24`) from the program; if none, only the updater enforces it. |
| U8 | What the word at `0xffe00ffc` (value 1) is for | Find readers of the flash-state word at block base plus `0x28` (package 0 `0x30000914`, package 1 `0x30000940`, package 2 `0x30000f3c`, package 3 `0x300010ac`, package 5 `0x300010d8`) and of the address `0xffe00ffc` itself. |
| U9 | Whether the status-reply producer (request `0x0072`) in packages 0, 2, 3 and 5 fills arguments 6 to 10 from the flash-state block exactly as package 1 does | Read the `0x0072` handler of each package (the dispatch shape is the same as package 1's `0xffe3c166`) and check which RAM words it copies; the initial data of all five blocks is read (5.4). |
| U10 | Whether the on-chip SRAM, hence the retained record, keeps its contents across a watchdog reset and across an external reset (section 3.1) | The manual calls the contents undefined after a hardware reset (page 5-3) and gives no retention rule; the program relies on retention. Read the errata revision 3 and the later ColdFire reference manuals for any retention statement. The images can show only that the program depends on it. |
| U11 | Closed: the call after the main task's clear of `+0x10` is the watchdog arm in packages 2, 3 and 5 as in packages 0 and 1 (section 3.4). | Read in all five packages, same routine shape. |
| U12 | Which reset sources can reach the loader between the hand-over and the main task's clear, and what the bit-5 byte reads then (the flash byte, which is set). This is the same question as U20 in [l1-board-contract.md](../../../devices/l1_board/README.md). | Read the reset sections of the manual for the sources. Search the code ahead of the clear for any timeout, fault restart or deliberate reset. If none, a reset cannot reach the window from the program, and bank A runs only after a board reset source fires there. |
| U13 | Every writer of the retained record's `+0x10` and `+0x18` words in all five packages, and whether each restart that empties `+0x10` and passes image argument 0 leaves bank B to the next start. | Enumerate every caller of the clear helper and every store through the record base plus `0x10` or `0x18` in all five packages. For each, read the restart that follows and its image argument. |

## 8. Sources

- `l1.bin` 1.44.0, packages 0, 1, 2, 3 and 5 as above; the package 1 listing in `software/firmware/l1/disasm/mcf5206e-1.44.0-fuel-pe-o300/l1.lst` for the addresses in sections 3, 4 and 5.4. The other four packages were read with a 68040-mode decoder and byte comparison.
- `flashsc` 6.5.30 (N32), routines listed in section 5.
- MCF5206e User's Manual, revision 1 (a hint, not a finding): the `MBAR` reset state (page 8-2), the reset-status register (page 8-13), SRAM initialisation (page 5-3), the software watchdog reset (section 6.11.3, page 6-84) and the global chip select (section 9.3.2, page 9-8). Section 3.1 step 2 depends on the page 8-2 and 9-8 statements together with the flash bytes.
- Further `l1.bin` files of IRIX 6.5.16 to 6.5.30, the 3.24 CD and two patches, for the byte measurement of U2.
- Older notes of this directory as linked.

## Review (2026-11-10)

An independent review checked this note against the same `l1.bin` and `flashsc` (same sha256 values as above) and the manual. Each point below was re-checked in the bytes before it was accepted.

Changed, with the reading that now stands:

1. **Header revision of package 5.** It is `5`, not `4` (section 1). The updater's acceptance bound (below 6) was already right.
2. **Vector stubs.** 254 stubs at `0xffe0042c`-`0xffe00820`, one per vector, not 232 up to `0xffe0081e` (section 2). The same wrong count in [board-boot-contract.md](board-boot-contract.md) is corrected there.
3. **The reset-status read in the loader.** It is a flash read through the global `CS0`, not the reset-status register, because `MBAR` is unwritten at that point (section 3.1 step 2). Bit 5 reads set from the shipped bytes, so the arm that clears `+0x10` is never taken. The claim that a cold reset clears `+0x10` is withdrawn (section 3.2), the consequence paragraph is rewritten, and the question in U2 is split into the part the manual answers and the board part that stays open. The chip-side grade for the bit-5 branch is inferred where it was established; the code reading stays established.
4. **Writer of `+0x18`.** Found: the restart routine stores through a base register, which a literal search could not see. Its argument is the running-bank word, which gives a consequence for the loader (section 3.2). U5 is settled for package 1.
5. **RAM size probe.** Rewritten from the code (section 6 item 4). The earlier text misdescribed the test.
6. **Addresses that belong to package 1.** The flash-state block, the geometry and bias words, the flash-size class byte, the record loader and the C-brick initialiser now carry per-package addresses (sections 4, 5.4, 6 and U6, U8). The "same routine" wording is kept only for the shape.
7. **Relocation table.** Packages 0 and 1 have an empty table and bank B cannot be derived from bank A; for packages 2, 3 and 5 the rules reproduce bank B exactly and the whole-table sum is zero (section 2.3). U4 is settled.
8. **Directory test.** The validator applies it to every revision, with header length `0x28` below revision 3 and `0x2c` from 3; the two other sites skip it below revision 3 (section 1).

Added by the re-check, not in the review: word 4 of the record is a pointer to the stamp string, and word 8 holds for bank B too (section 2.2); the vector 16 bytes at offsets `0x40` and `0x41` of 6.5.16 to 6.5.30 images (U2); the manual's silence on SRAM retention (U10).

Not taken up: the summary this pass worked from listed eight items in full; any further item was not available, so none is addressed here. Two addresses in the review's evidence are not used as given: for package 0 the flash-size class byte is stored at `0xffe55650` (the base is loaded at `0xffe5561c`; `0xffe55622` is the `moveq` before the comparison), and the record-loader call is the instruction at `0xffe55658`, not `0xffe5565c`, which is its operand.
