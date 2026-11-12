# L1 side of the graphics board EEPROM read on a Tezro (host request 6, component 2, area 2)

Question. What does the host ask the tower L1 for the graphics board's EEPROM, and does a real L1 answer? Grades: E established, I inferred, U unknown. Host side: kudzu `docs/ip35-graphics-eeprom-label.md` (IRIX 6.5.30 `unix.IP35`).

- Host request (E): `read_ia` (kernel 0x929b8) builds a request with the component at message +4 and the area at +0x14 (area 2 is the board area), reading 64-byte chunks at rising offsets until a chunk is shorter than 64. A graphics read on a chimera host uses component 2. Any nonzero status word in the reply (the first four bytes) makes the host treat the read as failed; it adds no label (no failure text).
- L1 firmware (E, `ip53-eeprom-record-tables-1.44.0.md`): the tower (type 9) table has an ODYSSEY device at table entry 2, wire address 0xac, bus selector 4. The Origin 300 firmware answers -104 for an entry whose probe failed and -107 for a bad header or area checksum ([board-records.md](board-records.md); I for the 1.44.0 package 2 image).
### Q4. Does a real Tezro L1 answer components 1, 2 and 4 with a V12 fitted? (partly E, answer I/U)

- The type-9 (tower) L1 image registers an ODYSSEY EEPROM device (table entry 2, wire address 0xac,
  bus selector 4) with INTERFACE 0, IO9 1, SNOWBALL 3, NODE 4 around it
  (sysco `docs/l1/ip53-eeprom-record-tables-1.44.0.md`). So the firmware has a graphics-board record
  slot, and the kernel's component 2 equals that table position. (E)
- The same table puts NODE at 4 and INTERFACE at 0, yet the kernel reads the node board as component 0 on
  a chimera host. So component numbers are not simply table positions, or the host remaps. Which
  numbers the 0x0006 handler accepts, and how it maps them: **U** (sysco open question; the handler
  was not read). Because of this the claim "component 1 and 4 answer on a real tower" cannot be made.
- Answer: component 2 is expected to answer when the board is fitted and its EEPROM probe succeeded
  (**I**, from the table slot and the kernel's component 2); -104 is the not-present reply, so a
  real tower with a V12 answers it, otherwise the real kernel would also print the alert on every
  V12 Tezro, which the hinv captures with `ODY128VERSIONB` contradict (I). Components 1 and 4: **U**.
  Settles it: `l1cmd serial all` or a component read from a V12 tower; or reading the 0x0006 handler
  from the compare chain at 0xffe145f8 in the package-2 image.

### Model choice (M, not hardware evidence)

Answer component 2, area 2 (board) on the local node's L1. Area bytes (56, version 1, length 7):

```
01 07 00 00 00 00 c3 53 47 49 ce 4f 44 59 31 32 38 56 45 52 53 49 4f 4e 42
c6 45 4d 55 30 30 31 cc 30 33 30 2d 31 36 31 31 2d 30 30 31 c0 c1 43 ff ff
c0 c1 00 00 00 01
```

Manufacturer `SGI` (placeholder), name `ODY128VERSIONB`, serial `EMU001` (placeholder), part
`030-1611-001`, skipped slot 0xc0, SGI field 1 `C`, raw bytes 0xff 0xff, empty field 3, 0xc1 end, three
pad zeros, checksum 0x01 (sum of the 56 bytes is 0 mod 256). Reply chunks as the kernel requests: at most
the 64 bytes asked, a short final chunk (the 56-byte area ends the loop). A decode of this record by an
independent port of the rules above yields part `030-1611-001`, name `ODY128VERSIONB`, rev `C`
(own script, not the kernel). Keep the name away from `ODY128B1_2`, `CHIM_ODY`, `CHIODY` unless the
DVI register path is wanted. Also still answer component 0 as today.
