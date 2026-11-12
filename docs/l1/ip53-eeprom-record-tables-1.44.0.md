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

### 3. The component number a request uses

**Established (L1 1.44.0, package 2, read from the body).** Both handlers resolve the host's component number through one hook, not through the table position. The 0x0007 handler starts at `0xffe146ba` and the 0x0006 handler at `0xffe14876` (five words required, `-100` otherwise). Each reads the request's source word at `+0x1c` of the request block (the sender tag is bits 11 to 5 of it, or `0xff` when the word is negative: `0xffe146ca` to `0xffe146de`, `0xffe14886` to `0xffe1489a`), takes word 1 of the request, and calls the function pointer stored at `0x30000fe4` with the tag first and word 1 second (`0xffe14704` to `0xffe14710`, `0xffe148c0` to `0xffe148cc`). The result is the record number. The 0x0007 handler then compares it with the registered count (`0xffe8cd54` returns the count read from the table block at `0x30000c64`), answers `-104` after logging `EEPROM not present` when the presence byte is 0 (`0xffe8cd6c`) and `-105` when the number is not below the count, and otherwise indexes the record array at 0x78 bytes per record (`0xffe14766` to `0xffe1476e`).

The hook is installed by `0xffe14004` (stores `0xffe13be4` at `0x30000fe4`, called from the type dispatch routine at `0xffea39a0`) and is the routine at `0xffe13be4`. It switches on the brick type byte at `0x300008cc` (`0xffe75fc8`) through a table of 39 words at `0xffe13c1a` (types `0x30` to `0x56`). For types `0`, `7`, `8`, `9`, `E`, `Q` and `V` it returns word 1 unchanged except that 0 gives 4 and 4 gives 0 (`0xffe13e0a` to `0xffe13e18`); the tag is not used. Types `5`, `6` and `O` use the tag and other rules (`0xffe13c68`, `0xffe13dd8`, `0xffe13c8a`) and were not read further; every other type returns word 1 unchanged.

So for the type 7 and type 9 profiles component 0 is table entry 4, the node, and component 4 is table entry 0, the interface board; components 1 to 3 and 5 and up are the table entries of the same number. The number does not depend on which link or sender the request came from. This matches the C-brick convention that component 0 is the node board.

**Inferred.** That record slot *i* holds the data of table entry *i*. The table registration (`0xffe8cb14`, called with the type 7 table at `0xffe5fd50` and count `0x10` at `0xffe5efa0`) stores the table pointer, the count, a presence array of count bytes and an array of count records of 0x78 bytes; the per-entry load that fills the slots was read in package 1 ([mbus.md](mbus.md)), not re-read in package 2. The type 9 count was not read.

Still unread: the hook's branches for types `5`, `6` and `O`, and the part of the 0x0006 handler after the hook call.

### 4. Power code

**Established, bounded to the tables and handlers read.** Both profiles' device tables hold a processor-module code input
(`NODE PSC` at `0xffe62af1` in the type-9 expander table; `IP53 PSC` and `IP59 PSC` at `0xffe6105c` and `0xffe61108` in
the type-7 one) and per-generation margin and voltage entries named `IP53` or `IP59`. None of the host request
handlers in [brick-request-handlers.md](brick-request-handlers.md) returns a processor-module code: the code stays inside
the controller. **Inferred:** the host's view of the same code is the low nibble of the Bedrock board word, which the PROM
reads and labels "board/PIMM PSC" (chips repository, `docs/questions/ip35-board-word-and-config-block.md`).
