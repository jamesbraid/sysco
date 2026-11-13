# L1 side of the graphics board EEPROM read on a Tezro (host request 6, component 2, area 2)

Question. What does the host ask the tower L1 for the graphics board's EEPROM, and does a real L1 answer? Grades: E established, I inferred, U unknown. Host side: kudzu `docs/ip35-graphics-eeprom-label.md` (IRIX 6.5.30 `unix.IP35`).

- Host request (E): `read_ia` (kernel 0x929b8) builds a request with the component at message +4 and the area at +0x14 (area 2 is the board area), reading 64-byte chunks at rising offsets until a chunk is shorter than 64. A graphics read on a chimera host uses component 2. Any nonzero status word in the reply (the first four bytes) makes the host treat the read as failed; it adds no label (no failure text).
- L1 firmware (E, `ip53-eeprom-record-tables-1.44.0.md`): the tower (type 9) table has an ODYSSEY device at table entry 2, wire address 0xac, bus selector 4. The Origin 300 firmware answers -104 for an entry whose probe failed and -107 for a bad header or area checksum ([board-records.md](board-records.md); I for the 1.44.0 package 2 image).
### Q4. Does a real Tezro L1 answer components 1, 2 and 4 with a V12 fitted? (partly E, answer I/U)

- The type-9 (tower) L1 image registers an ODYSSEY EEPROM device (table entry 2, wire address 0xac,
  bus selector 4) with INTERFACE 0, IO9 1, SNOWBALL 3, NODE 4 around it
  (sysco `software/firmware/l1/ip53-eeprom-record-tables-1.44.0.md`). So the firmware has a graphics-board record
  slot, and the kernel's component 2 equals that table position. (E)
- The same table puts NODE at 4 and INTERFACE at 0, and the kernel reads the node board as component 0 on
  a chimera host. Both agree: the package 2 request hook (`0xffe13be4`) swaps components 0 and 4 for brick
  types 7 and 9 and passes every other number through, so component 0 is the node, 4 the interface board,
  and component 2 is table entry 2 (E; see `software/firmware/l1/ip53-eeprom-record-tables-1.44.0.md` section 3). That
  record slot *i* holds table entry *i*'s data after registration is **I**. Components 1 and 4 answer when
  their records are present, by the same rule.
- Answer: component 2 is expected to answer when the board is fitted and its EEPROM probe succeeded
  (**I**, from the table slot and the kernel's component 2); -104 is the not-present reply, so a
  real tower with a V12 answers it, otherwise the real kernel would also print the alert on every
  V12 Tezro, which the hinv captures with `ODY128VERSIONB` contradict (I). Components 1 and 4: see the mapping above.
  Settles it: `l1cmd serial all` or a component read from a V12 tower.
