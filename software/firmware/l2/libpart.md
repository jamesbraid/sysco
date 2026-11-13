# libpart: the partition mode database

Read from L2 1.44.0, `/stand/sysco/lib/libpart.ppclinux.so` (27,608 bytes; identical bytes in 1.48.0). Addresses are virtual addresses in the library (text from `0x2b70`). Per-routine rows are in [exports.tsv](exports.tsv); the request service that uses the library is in [partition-task.md](partition-task.md). Grades as in [../../AGENTS.md](../../../AGENTS.md). This note settles three things the partition note left open: what the header word at `0x10` is, what the checksum is, and the layout of the database body.

## Header and checksum

The 24-byte header (copied to `part_db_hdr`, `0x2ed40`) is as in [partition-task.md](partition-task.md): magic `0x5a4c4a21`, total size, checksum, timestamp, a word at `0x10`, version (must be 1).

- **The word at `0x10` is the identifier of the selected mode.** In `unpack_mode_db` (`0x3950`), when the process is the daemon (the flag `isDaemon`, `0x16368`), the mode loop stops at the first mode whose identifier byte equals this header word (`0x4d50` to `0x4d6c`) and keeps that mode in `workingmode`. Established; `PART_SET_MODE` therefore switches the active mode without touching the rest.
- **The checksum is the POSIX `cksum` CRC.** `chksum` (`0x5944`) walks the bytes with the table at `0x15ca4` (polynomial `0x04c11db7`), shifting the register left a byte at a time and indexing with the top byte xor the data byte; then it folds in the byte length, least significant byte first, until the length is exhausted; and returns the complement. That is the algorithm of the standard `cksum` utility; `get_cksum` (`0x5260`) only calls it. Established from the whole routine.
- Return values of `unpack_mode_db`: the timestamp (`part_db_hdr` word `0xc`) on success; 0 when the magic word is zero (an empty buffer); -1 for a nonzero wrong magic; -3 for a checksum mismatch; -4 for a version other than 1; -6, -7, -8, -9 for the four allocation failures (modes, the environment blob table, one blob, the mode array); -10 when an environment block cannot be split (`unpack_envs` fails). A value of 1 appears only on branches guarded by `check_sanity`, whose test jumps to the same place whichever way it goes (dead in 1.44.0).

## Body layout

The body starts at database offset `0x18`. All integers are big-endian and are read a byte at a time (`memcpy` of 1, 2 or 8 bytes into the low end of a word or doubleword). Established by reading `unpack_mode_db` from `0x3c20` to `0x5028`.

1. **Layouts.** One byte `NA`, then `NA` layouts, each: a halfword whose bit 15 is a flag (stored at mode offset `0x18`) and whose low 15 bits are the brick count `C` (offset `0x1c`); one byte (offset `0x20`; zero makes `partid_for_brick` answer 0, so I read it as "this layout has partitions"; inferred); one 64-bit big-endian word (offsets `0x28` and `0x2c`), a bitmask of the partition numbers in use; then `C` pairs of a halfword brick identifier and a byte partition number.
2. **Environment blobs.** A halfword `NB`, then `NB` blobs each prefixed by a halfword length. Each blob is a run of NUL-terminated `NAME=VALUE` strings ended by an empty string (`unpack_envs`, `0x50b4`, copies each string into its own allocation until it meets an empty one, stores the count and a null pointer, returns 0; 1 when an allocation fails).
3. **Modes.** One byte `NM`, then `NM` modes each: one identifier byte (offset 0); 16 bytes of name (`strncpy` of 16 into offset 4, so a 16-character name has no terminator); one byte, the index of a layout from step 1 (offset `0x14`); one byte `NP` and `NP` pairs of a partition number byte (0 to 63) and a halfword index of an environment blob from step 2.

After reading, each mode is completed from the layout it names: the count, flag, byte and mask words and the brick table are copied from that layout, and for every set bit of the mask `unpack_envs` splits the blob chosen for that partition into the mode's own strings. The temporary layout and blob arrays are freed. So the same brick-to-partition table can serve several modes, and each mode only chooses which environment each partition gets.

The unpacked mode record is `0x8630` bytes (the allocation size, and `0x430 + 64 * 0x208`, so partition numbers are 0 to 63):

| Offset | Content |
|---|---|
| `0x00` | identifier (the byte lands in the low byte of the word) |
| `0x04` | name, 16 bytes |
| `0x14` | layout index |
| `0x18` | the flag from the layout's count halfword (bit 15) |
| `0x1c` | brick count |
| `0x20` | the layout's byte (zero means unpartitioned) |
| `0x28` | 64-bit mask of partitions in use |
| `0x30` | brick table: 4 bytes a brick, halfword identifier at `+0`, partition byte at `+2` (room for 256 entries up to `0x430`) |
| `0x430 + 0x208 * p` | partition `p`: word blob index, word count of variables at `+4`, up to 128 string pointers from `+8` |

The copy loop does not check the brick count against the 256-entry table or the partition number against 63: the only bound checks in the function sit behind the dead `check_sanity` test. A database that fails its checksum is rejected, but one that passes and overflows these tables is not caught by this code. (Established as absence of checks in the function.)

## Lookups

- `partid_for_brick(mode, brick)` (`0x5374`): returns 0 when the mode's byte at `0x20` is zero. Otherwise, when the flag at `0x18` is set, the brick identifier is masked to bits 6 to 11 (`0x0fc0`, the rack field; see below) before the search, so one entry covers a whole rack. It scans the brick table for a matching halfword and returns the partition byte as a signed value, or -1 if absent. `get_partid_for_brick` (`0x546c`) is the same on `workingmode`, taking the brick as a signed halfword.
- `env_for_part(mode, part, &len, buf)` (`0x54c8`): when bit `part` of the mask is set, copies each of that partition's variables into `buf` as `NAME`, a NUL, `VALUE`, a NUL (it copies the `NAME=VALUE` string and replaces the `=` with a NUL), then always appends two more NULs, stores the byte length through `&len` and returns the variable count; with the bit clear it writes only the two NULs and returns 0. `get_env_for_part` (`0x5650`) is the same on `workingmode`. This is the exact reply layout of request 2 in [partition-task.md](partition-task.md).
- `check_reboot_status(old, new)` (`0x56ac`): calls `get_system_bricks` (an import) for the bricks of this system (halfword array `sysbrick`, count `sysbricknum`, `0x16410`) and for each brick compares the partition in `old` with the partition in `new`. If they differ it sets `part_status[old]` to 1 (when `old` is not negative) and `part_status[new]` to 1 (when `new` is not negative and the new mask has that bit), counting each, and returns the count (or the negative error from `get_system_bricks`). `part_status` is a 64-entry word array at `0x16610`. The role (which partitions must reboot after a mode change) is inferred from the name and the comparison.
- `save_mode` (`0x58b0`) copies `workingmode` over `savedmode` (`0x8630` bytes); `compare_modes` (`0x58f8`) is `check_reboot_status(savedmode, workingmode)`. The partition task calls them around a mode change.
- `create_default_db` (`0x52b4`): clears the 64 KiB buffer `dbbuf`, writes the header with magic, size `0x18`, checksum of an empty body, timestamp 0, mode word 0 and version 1. No modes.

## Brick and rack identifiers

A brick identifier is a 16-bit value: bits 15 to 12 index the type letters `crikxdpn%9012345` (the table `brick_types`, `0x15c90`; an index above 15 cannot occur and the formatter would answer `ERROR`), bits 11 to 6 are the rack number, bits 5 to 0 the slot. Read from `format_brick_id` (`0x2cd0`), `format_rack_id` (`0x3390`), `parse_rack_id` (`0x350c`) and the error paths of `parse_brick_id` (`0x2f24`).

The rack number prints as three digits that are not decimal: if bit 5 of the rack number is set the first digit is 1, the second is bits 4 to 2 (0 to 7) and the third is `1 +` bits 1 to 0 (1 to 4); if clear the first digit is 0, the second is bits 4 to 3 (0 to 3) and the third is `1 +` bits 2 to 0 (1 to 8). So rack numbers 0 to 7 print as `001` to `008`, 8 to 31 continue as `011` to `038`, and 32 to 63 print as `101` to `174` (the third digit is 1 to 4 there). The brick text is rack digits, the type letter, then the slot in two decimal digits (a leading `0` when the slot is below 10). `parse_rack_id` takes 1 to 3 digits (shorter strings are left-padded with zeros to three), and returns `rack << 6`, or -2 for a length outside 1 to 3, -3 for a non-digit, -4 for a first digit above 1, -5 and -6 for a second or third digit out of range. `parse_brick_id` rejects a string of the wrong length (-2; the compare against 6 is at `0x2f64`, the direction not transcribed), the same three digit checks (-3 to -6), one letter that must be alphabetic (-7) and found in `brick_types` after `tolower` (-8, index at most 15), then digits for the slot (-10 on a bad digit, -11 above 63). The final packing of the parse is not transcribed; it mirrors the format routine.

The mapping of each type letter to a brick product is not read here: no routine in this library names one.
