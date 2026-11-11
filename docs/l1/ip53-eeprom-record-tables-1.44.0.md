# L1 1.44.0 package 2: the board-EEPROM device tables of the Chimera server (type 7) and workstation (type 9) profiles

Question. When the compute-brick L1 runs as the 2U server profile (Origin 350, Onyx 350; type character `7`) or the tower
workstation profile (Tezro; type `9`), which EEPROM devices does it register, in what order, and what differs between
the two? This is the data the brick-record requests (`0x0006`, `0x0007`) and the `serial all` console listing draw on.

Sources. IRIX 6.5.30 `l1.bin` (container sha256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`),
range 2, the `MIPS 2MB image` package (see [package-container.md](package-container.md)): the 44-byte `ESTF` header
removed, the body loaded at `0xffe00000`, so address = 0xffe00000 + body offset. Bodies were read as bytes and with
`retools/tools/hostfacts.py` (ColdFire). Nothing was run. Published captures of `serial all` from an Origin 350, an Onyx 350
graphics module and a tower Tezro (generic description, counts below) are cited for agreement only.

## Answer

### 1. Where the type is decided

**Established.** The type is decided by the identity routine of [personalities.md](personalities.md) (package 2:
`0xffe75a68`). In this package the board names it compares are at `0xffe76427` (`WS_INT_53`, gives `9`) and
`0xffe76431` (`2U_INT_53`, gives `7`); each string occurs twice, once per bank copy (the second at +0xf0000). The two
per-type initialisers are chosen by the dispatch at `0xffea38f6` to `0xffea3970`: `7` to `0xffe5f858`, `9` to
`0xffe61bc0`. Re-read: the type-9 initialiser (`0xffe61bc0` to `0xffe61cd2`) fills a descriptor and stores the letter `P` at
descriptor +0xc (the required first letter of the system serial; [serial-number-requests.md](serial-number-requests.md)
had read the same store from the dispatch). It contains no board-name comparison of its own, so nothing after the
identity routine changes the `9`. Riser-check strings (`2U_AGP_RISER`, `2U_RISER_5V` and the per-type "set" lines) sit at
`0xffe60d94` to `0xffe60e24`; the routine that uses them was not re-read here, and the type-3 reply rewrite that turns a
cached `8` into `7` is in [brick-request-handlers.md](brick-request-handlers.md). Nothing in the type-9 initialiser or its
table refers to a riser (see section 2).

### 2. The device tables

**Established (data read, entry by entry).** Each profile has a table of 12-byte entries, big-endian: a word whose bytes
are [bus selector, wire address, record shape, 0], a word whose first byte is the record kind (1 board record, 2 memory
module, 4 power supply: the kinds `0x0007` reports), then a pointer to the entry's name. The wire addresses match the
four reserved identity records of [personalities.md](personalities.md): `0xa4` is the interface board (record `0xff`).
Tables, in order:

| index | type 7 (2U server), table at `0xffe5fd50` | type 9 (tower workstation), table at `0xffe62064` |
|---|---|---|
| 0 | INTERFACE (`0xa4`, selector 0) | INTERFACE (`0xa4`, selector 0) |
| 1 | IO9 (`0xac`, selector 3) | IO9 (`0xac`, selector 3) |
| 2 | ODYSSEY (`0xac`, selector 4) | ODYSSEY (`0xac`, selector 4) |
| 3 | RISER (`0xa8`, selector 3) | SNOWBALL (`0xa8`, selector 4) |
| 4 | NODE (`0xac`, selector 2) | NODE (`0xac`, selector 2) |
| 5 to 12 | DIMM 0, 2, 4, 6, 1, 3, 5, 7 (`0xa0`, selectors 1 to 8 in the high nibble, kind 2) | the same eight |
| 13 | SNOWBALL (`0xa8`, selector 4) | IO DGHTR (`0xa8`, selector 3) |
| 14 | PS 1 (`0xa0`, selector 5, kind 4) | end of the record table |
| 15 | PS 2 (`0xa2`, selector 5, kind 4) | |

Entries after those (INTERFACE 0 to 3, ODYSSEY, NODE, BEDROCK, NODE FANS and the like, with a different second word) belong
to a separate device table that follows; they are not records. So the type-7 profile registers 16 EEPROM devices and the
type-9 profile 14, in these orders. The workstation profile has no riser and no power-supply records, and has an I/O
daughter board (`IO DGHTR`) at the riser's wire address and bus selector. (The `0xa8` shared by RISER, SNOWBALL and
`IO DGHTR` comes from the table; whether the same physical part answers is not shown.)
**Agreement.** The order of the type-7 table with the DIMM block removed (INTERFACE, IO9, ODYSSEY, RISER, NODE, SNOWBALL,
PS 1, PS 2) is the order of the rows a published `serial all` of a 2U module with graphics prints (one capture; the
DIMM rows are listed separately there).

### 3. What is not settled: the component number a request uses

**Unknown.** The record numbering the host sees (the component index of `0x0006` and `0x0007`) is the registration
order on an Origin 300 (node 0, I/O board 1, then memory modules). For package 2 the handler that maps a request's index to
a table entry was not read: it is a compare chain with no table to point at, the `EEPROM not present` string is referenced
from the `0x0007` handler at `0xffe14736`, and the request-hook installer was not found. If the index were the table
position, component 0 would be the interface board and the node would be component 4. Two facts argue otherwise or at least
split the question, so neither answer is taken: the IP35 PROM and kernel are shared with the C-brick, where component 0 is
the node board, and a real tower's `hinv` prints the node board as `IP53_4CPU` at `/node`; yet the same real `hinv` prints
the interface board at two crosstalk widgets and the I/O board under the IOC4, which a node-only component 0 would not
produce. Route that settles it: read the package 2 handler for `0x0006` from the compare chain at `0xffe145f8` on (the
`0x0004` handler is at `0xffe145f8`; `0x0008` at `0xffe14b98`) and the code that installs the request hook.

### 4. Power code

**Established, bounded to the tables and handlers read.** Both profiles' device tables hold a processor-module code input
(`NODE PSC` at `0xffe62af1` in the type-9 expander table; `IP53 PSC` and `IP59 PSC` at `0xffe6105c` and `0xffe61108` in
the type-7 one) and per-generation margin and voltage entries named `IP53` or `IP59`. None of the host request
handlers in [brick-request-handlers.md](brick-request-handlers.md) returns a processor-module code: the code stays inside
the controller. **Inferred:** the host's view of the same code is the low nibble of the Bedrock board word, which the PROM
reads and labels "board/PIMM PSC" (chips repository, `docs/questions/ip35-board-word-and-config-block.md`).
