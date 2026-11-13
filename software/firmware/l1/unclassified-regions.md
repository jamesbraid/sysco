# Origin 300 L1 1.44.0: data and padding the linear listing cannot place

The committed listing is a linear decode, so data decodes as implausible instructions. This note records what is known about the large non-code spans, read from the body bytes (body offset = address minus `0xffe00000`; the second bank repeats each span `0x70000` higher).

## Padding

The span `0xffe00b38`–`0xffe1040c` (63,700 bytes) holds 62,910 `0xff` bytes, 17 NUL bytes and 98 printable bytes, with a longest `0xff` run of 61,433 bytes. Established: it is mostly erased-flash fill. The non-`0xff` bytes in it are not classified. The reset code ends at `0xffe00b38` and the first loader candidate starts at `0xffe10000` ([coldfire-package-sections.md](coldfire-package-sections.md)).

## Tables and text

- USB descriptors: the device descriptor at `0xffe6fd20` and configuration tree at `0xffe6fd32` ([usb-descriptor-fields.md](usb-descriptor-fields.md)).
- Settings jump tables: four 358-byte tables ([unguarded-indexed-anchors.md](unguarded-indexed-anchors.md)).
- A six-entry jump table at `0xffe7b470`–`0xffe7b47c` in a text formatter (guard `0xffe7b45e`). Entries zero to three each advance an argument pointer by four and store the preceding 32-bit word in a local; entry four stores it zero-extended from 16 bits, entry five sign-extended from 16 bits; all continue at `0xffe7b4e4`. The formatter scans for a percent character; three callers at `0xffe7a494`, `0xffe7a508` and `0xffe7a592` pass a text source and an argument pointer. Inferred: it is a general `printf`-style formatter; the output sink is not read.
- The EEPROM record table at `0xffe45da0`, string-rich regions around `0xffe1bfd0`–`0xffe26f9c` (45,004 bytes of printable text and NULs, the command and message pools) and `0xffe5b9aa`–`0xffe5d990` (8,166 bytes), all different in the second bank because they hold pointers.

## What remains

Some of the remaining erased-looking and string-rich spans may hold code reached only through computed or indirect jumps; a direct-branch scan finds no branch into any of them, which does not exclude that. Do not force these bytes into code to raise coverage. Routes: follow every indirect call and jump-table in the body (the listing's `jmp` and `jsr (a0)` forms), and compare the spans with the same spans in the other 1.44.0 ColdFire packages and in 1.48.1.
