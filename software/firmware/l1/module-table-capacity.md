# L1 module table: capacity, who fills it, and what happens past the tenth record

Read from the IRIX 6.5.30 `l1.bin` (L1 1.44.0, sha256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`), ColdFire packages 0, 1 and 2 individually (bodies loaded at `0xffe00000`; package 1 has a committed listing, packages 0 and 2 were disassembled for this note with the same decoder settings and are not committed) and packages 3 and 5 for the capacity word only. Addresses are first-bank addresses. Grades as in [../../AGENTS.md](../../../AGENTS.md): established (whole routine read), inferred (reasoning stated), unknown (routes listed at the end). Written 2026-11-13; it supersedes row 1, 2, 3 and 14 of the capacity table in [../l2/l1-conversation.md](../l2/l1-conversation.md) section 12 where they differ (section "Corrections").

## Answer in short

- **The table holds 10 records of 0x70 bytes in packages 0, 1 and 2, whatever the brick type.** The count is stored once, as the immediate 10, and the 0x460-byte table is allocated from it. A host that walks 15 entries walks five records that do not exist in these packages. Established.
- **There is no "append" path and so no 11th record in the sense of an eleventh arrival.** Records are placed by position, chosen by the port a frame arrived on (and a fixed code for the brick itself). Nothing searches for a free slot, so nothing can "run out": no drop, no error code, no log line, no overwrite of another slot. Established (every writer enumerated below).
- **The only way past the table is the second group of five (positions 10 to 14).** It is written by the same arm that writes positions 5 to 9, when the arrival port is the brick's second node port. That arm has no bound check against the capacity word: it zero-fills 0x230 bytes and copies up to five records starting 0x460 bytes after the table base, that is, into whatever follows the allocation. No log, no reply, no reset: memory corruption. Established for the code; what the neighbouring memory is, is inferred (a heap block header, see below).
- **A C-brick (type `C`, packages 0 and 2) never reaches it, and neither does a Cx-brick (type `6`, package 2).** The `C` initializer gives it node ports 1 and 9 only; the `6` initializer opens five ports, 1 and 9 to 12, and never port 2 (section "Type 6" below). Port 2 is not opened, so the arm is entered for position 5 and never for position 10. Types that do open port 2 are `%` (the PE brick, packages 1 and 2), `5`, `O` and `!` (packages 3 and 5). Established for the initializer table and for the port-to-position byte table; inferred for "a frame cannot arrive on a port that is not opened".
- **The 0x000d message reads position 10 on every brick type, with no bound check:** the reply carries three words read from the 0x70-byte slot that starts 0x460 bytes after the table, which is the header of the next heap block plus the first word of its data. Established for the read; the content of that block is unknown statically.
- **Packages 3 and 5 are different:** their capacity depends on the brick type (45, 85 or 125 records for some types, else 10). The 10 holds only for packages 0, 1 and 2.

## 1. Where the table is and how large

| | Package 0 | Package 1 | Package 2 | Package 3 | Package 5 |
|---|---|---|---|---|---|
| table state block (pointer word, capacity word at +4) | `0x30000a78` | `0x30000aa4` | `0x300010f8` | `0x30001248` | `0x30001370` |
| initializer (store of the capacity, then the allocation) | `0xffe313d8` | `0xffe313d8` | `0xffe36bb0` | `0xffe311a2`, `0xffe311aa`, `0xffe311b2` | `0xffe43c5e`, `0xffe43c66`, `0xffe43c6e` |
| allocation = capacity x 0x70 (`muls.l 4(a5)`) | `0xffe313e0` | `0xffe313e0` | `0xffe36bb8` | `0xffe311b8` | `0xffe43c74` |
| capacity | 10 | 10 | 10 | per type (below) | per type (below) |
| record-by-index routine | `0xffe31f00` | `0xffe31f00` | `0xffe376d8` | `0xffe32174` | `0xffe44c30` |
| record-apply routine | `0xffe31f18` | `0xffe31f18` | `0xffe376f0` | `0xffe3218c` | `0xffe44c58` |

Read: in packages 0, 1 and 2 the initializer is straight-line code `moveq #10 / move.l d0, 4(a5) / moveq #0x70 / muls.l 4(a5)`, then a call to the heap allocator and a zero-fill of capacity x 0x70 bytes (package 1 `0xffe313d8` to `0xffe31406`). A scan of both flash banks of each package for any other instruction that stores into the capacity word through an address register loaded with the state block found only the initializer (packages 0, 1, 2: the two copies of the program; package 3: three stores per copy; package 5: three per copy). The 10 is therefore the only value the capacity word ever has in packages 0, 1 and 2. Established.

Record 0 is this brick: after the zero-fill the initializer stores 1 at `+0x00` (valid), the brick-type character at `+0x10` and 2 at `+0x60` (state), then rack at `+0x08`, bay at `+0x0c` (settings 1 and 2), partition at `+0x3c` (setting 14, [../l2/two-partition-nvram-and-console.md](../l2/two-partition-nvram-and-console.md)), name at `+0x40` (setting 15) (package 1 `0xffe3140a` to `0xffe31496`). Established.

**Packages 3 and 5** choose the capacity from the brick-type character through a 62-entry jump table (type minus `0x21`): in package 3, types `B L O` give 125, `@ D` give 45, `! + 5 J K ^` give 85, all others 10; in package 5, `$ B L O` give 125, `@ D` 45, `! + 5 J K ^` 85, others 10 (jump-table decode: package 3 at `0xffe31120`, package 5 at `0xffe43bdc`). Established. The earlier note read only the multiplication and called the capacity 10 in all five packages.

## 2. How a record gets a position

The records are not a list of modules in arrival order. They are a fixed layout, and the layout is filled by the receive handler of the configuration-exchange task (package 1 `0xffe325ec`, package 2 `0xffe37dc4`, same body) and by its response handler (`0xffe33ce4`):

| Position | Written when | By | Bound |
|---|---|---|---|
| 0 | start-up | the initializer | inside the table |
| 1 to 4 | selectors 1, 3, 14, 15: a frame on port 9, 10 or 11 gives position 1, 2 or 3 and a frame on any other port gives position 4; selectors 4, 5, 9, 12: ports 9, 10, 11, 12 give positions 1 to 4 and other ports nothing | the apply routine, arm for positions 1 to 4 (clear 0x70 bytes, mark pending, then copy one record) | inside |
| 5 to 9 | a frame of selector 2, 13 or 20 to 25 arrives on port 1 | the apply routine, group arm for position 5 (clear 0x230 bytes, copy up to five records) | inside: 5 x 0x70 = 0x230 ends exactly at the table end |
| 10 to 14 | the same selectors arrive on port 2 | the same group arm, position 10 | **not inside**: starts at the table end |
| 15 and up | never | | not reachable: the apply routine's first test is `index > 10 -> do nothing` |

Read: package 1 `0xffe3298e` to `0xffe329b4` (port 1 gives position 5, port 2 gives position 10, any other port nothing), `0xffe32a32` to `0xffe32b98` (ports 9 to 12 give 1 to 4), jump table at `0xffe3264a` (26 entries; selectors 2, 13, 20 to 25 go to `0xffe3298e`). Established. The response handler calls the apply routine for positions 1 to 4 only (`0xffe33e36` to `0xffe33eb2`). Established. Each port also has a link task (`0xffe2e344`) that calls the apply routine with the record index stored in the port's object, with a "pending" flag of 1 when the link comes up and 0 when it drops (`0xffe2e3ec`, `0xffe2e554`); the object's index words come from a RAM byte table (`0x30000a2c` onward in package 1) whose initial bytes were not read, so which positions the link tasks use is inferred to be 5 and 10 for ports 1 and 2.

### Which brick types open port 2

The peer-link initializer (package 1 `0xffe2dba4`) fills a per-port table of device handles (-1 meaning "no port") from a jump table over the brick-type character (`0xffe2dcd6`). Read from the stores:

| Types | Ports opened (handles at `0x1c + 4 x port` from the object base) | Node ports |
|---|---|---|
| `C 0 4 7 8 Q` (the C-brick and the 3-series, IP45, and similar) | 1 and 9 | port 1 only |
| `I X Y P G N & # = E V` and others in the I/O set | 9, 10 (and 11, 12 for `&` and `N`) | none |
| `6` | 9 to 12 and a port 1 handle | port 1 |
| `%` (PE brick) | 1 and 2 | ports 1 and 2 |
| `5` | 1 and 2 (and 9, 10 unless the pseudo-type is `1` or `2`) | ports 1 and 2 |
| `O`, `!` | 1 and 2 | ports 1 and 2 |

Established for the store sequences at `0xffe2dd48` to `0xffe2de06`; the character-to-arm map was decoded from the jump table. Packages 0 and 1 are identical here (normalized instruction streams equal), package 2 has the same code at a fixed offset. A brick only runs the arm for a port it has opened, so inferred: only `%`, `5`, `O` and `!` can produce position 10; the supported-brick strings put `%` in packages 1 and 2 and `5`, `O` in packages 3 and 5 ([personalities.md](personalities.md)).

### Type 6 (Cx-brick, package 2): ports and positions, re-read 2026-11-13

Package 2 peer-link initializer `0xffe3337c` (the package 1 routine is `0xffe2dba4`; the package 2 address here was found directly, not by the fixed offset). It fills the same per-port handle table (object `0x3000573c`, handle of port `p` at `0x1c + 4 p`, `-1` for none) from a jump table over the type character (word table `0xffe334ae`, index = character minus `0x21`). The arm of type `6` (`0xffe3354c`) stores handle 3 for port 9, 5 for port 10, 6 for port 11, 4 for port 12 and `0xd` for port 1. Port 2 (offset `0x24`) keeps `-1`. The arm shared by `0 4 7 8 C Q` (`0xffe3357a`) opens ports 1 and 9 (handles 3 and 4); `%` (`0xffe3356e`) opens ports 1 and 2; `5` opens 1 and 2, plus 9 and 10 unless the pseudo-type is `1` or `2`; `O` and `!` open 1 and 2. The loop after the jump table allocates one 0x268-byte object per port with a handle other than `-1` and opens it. The node count does not enter: the set of ports of a Cx-brick is the same with one, two, three or four node boards fitted (node presence is a type byte per node, `0x30003046`, `0x0f` meaning absent, read only by the node routines). Established for the stores and the loop.

The record position each port uses is a byte table in initialised data (`0x3000107f`, copied from flash: RAM address `0x30000000 + x` is at `0xffea8ff4 + x` in package 2): index 1 holds 5, index 2 holds 10, indices 3 to 8 hold 15, 20, 25, 30, 35, 40, indices 9 to 12 hold 1 to 4. So a Cx-brick's port 1 feeds position 5 and its ports 9 to 12 feed positions 1 to 4; no opened port feeds position 10, and none of the positions above 10 (ports 3 to 8) is opened by any type read. Established for the table bytes; inferred that the link task uses this word as the apply index (the earlier reading of the link task, section 2).

A Cx-brick therefore serves one node port (port 1, the router-side group) and four peer ports (9 to 12), whatever the number of node boards; the five opened ports are not the node boards' Bedrock serial links, which are separate software ports ([quad-uart-and-serial-links.md](quad-uart-and-serial-links.md), [../../platforms/bricks/router-board-records.md](../../platforms/bricks/router-board-records.md)). The table in section "Which brick types open port 2" above already lists `6` correctly (9 to 12 and port 1); only the sentence that named the types that never reach position 10 omitted it.

## 3. The group arm at position 10, step by step

Package 1, addresses as in section 1; packages 0 and 2 identical (package 2 is 0x57d8 higher).

1. `0xffe31f5e`: base = pointer word plus `0x70 x index`. For index 10 that is table + 0x460, 0x70 bytes past the 0x460-byte allocation. The routine copies 0x70 bytes from there into a stack save area (reads past the table).
2. `0xffe31f7e` to `0xffe31f8a`: **zero-fill of 0x230 bytes at that address** (`pea 0x230`, clear, base). Written past the table. For index 5 the same call clears positions 5 to 9 exactly.
3. With the pending flag set (all callers but the link-down call): store 1 into the state word (`+0x60`) of the slot at the base (`0xffe321c4` to `0xffe321d6`). Past the table for index 10.
4. Skip unless the message carries a status byte of 0; the copy length is the first descriptor's own length clamped to 0x70 (`0xffe321e0` to `0xffe321fa`).
5. `0xffe32228` onward: copy one descriptor into the slot at the base, a second into base + 0x70, a third into base + 0xe0 when the message has more than three descriptors, a fourth and fifth (base + 0x150, base + 0x1c0) when it has more than four. Each copy sets the slot's state word to 2 when the slot's first word is nonzero and its state is 1. So up to five slots, 0x230 bytes, past the table. Established.
6. `0xffe32362` onward: reads the slot's `+0x58` and `+0x64`/`+0x4c` words (past the table) to decide whether to send a configuration message (`0xffe31b10`, selector 13 for position 10) and to call two helpers (`0xffe2fb08` with the slot's string and a code of 0x1000, 0x2000 or 0x4000 plus the index, then `0xffe31eb4`, which runs `0xffe31d7c`); the roles of the helpers were not read.

There is no compare of the index against the capacity word anywhere in the arm, no log line and no error return. The arms for positions 1 to 4 and the other loops that clear or copy several slots (`0xffe32040` to `0xffe3207c`, `0xffe33ba8` to `0xffe33c46`, the selector-6 builder) do compare against the capacity word (`cmp.l 4(a4), dN` then `bge`), which with a capacity of 10 makes them skip everything beyond 9. Established for each named routine.

Link-down: the same arm is entered with the pending flag 0 and no packet (`0xffe2e554`): it zero-fills the group (past the table for index 10), then returns unless the old state was nonzero; for the old state nonzero and a compute-type old record it reads the other group head (`index 5 <-> 10`, `0xffe32082`), so link-down on port 1 reads position 10 (one past the table) and checks its type and name. Established.

## 4. What is after the table

The table is a block from the L1's own allocator (`0xffe55d34`, a wrapper that takes a lock and calls the pool routine `0xffe404a4` and its worker `0xffe40740`; the pool carries the tag word `0x44594e41` at `+0x14`). Read from the worker: blocks are in one circular doubly linked list; each block has a 16-byte header, then data; header words: `+0x00` next block, `+0x04` previous block, `+0x08` byte: 1 for free and 0 for in use, `+0x0c` the pool. An allocation size is rounded up to a multiple of 4 and the block is split only when the leftover is at least the pool's minimum block plus 0x10 (`0xffe407ac` to `0xffe407e6`). So the first byte past a 0x460-byte table is, when the block was split, the header of the next block; otherwise slack up to the minimum block size, then the header. Established for the allocator; inferred for "split" (the table is allocated during start-up, when the pool is nearly empty).

What the zero-fill at position 10 does to that header: next = 0, previous = 0, flag = 0 (in use), pool = 0, then the following 0x220 bytes of the next block's data. The later allocator calls walk the list through those pointers (`0xffe40788` to `0xffe407a8`: `movea.l (a5), a5` and `tst.b 8(a5)` with a5 = 0), and the free routine checks the pool word and tag and returns an error code for a block whose pool word is 0 (`0xffe40542` to `0xffe40562`). What the CPU does for a load at address 0 or 8 on this board was not read, so whether the next allocation ends in a fault, an error return or a reset is unknown; that something other than the table is overwritten is established.

## 5. Every routine that adds or updates a record, and every reader of position 10

| Routine (package 1) | Role | Positions | Bound against capacity |
|---|---|---|---|
| `0xffe3132c` initializer | allocates, zero-fills, fills record 0 | 0 | by construction |
| `0xffe31f18` apply | the only routine that writes positions 1 to 14 | 1 to 4, 5, 10 (5 and 10 as groups of five) | none for 5 and 10 |
| `0xffe31ec8` copy | copy of up to 0x70 bytes into a slot, marks state 2 | any | none (caller decides) |
| `0xffe325ec` receive | chooses the position from the arrival port | 1 to 4, 5, 10 | see section 2 |
| `0xffe33ce4` response | responses of the exchange | 1 to 4 | n/a |
| `0xffe334a0`, `0xffe3360c` (events 1, 2), `0xffe33944` (event 6, partition), `0xffe339e0`, `0xffe33a84`, `0xffe33730` | peer notices: find the record by rack and bay and update a field of it | found record only | walk bounded by the capacity word; **a notice for an unknown brick finds nothing and is dropped silently** |
| `0xffe32ca8` and `0xffe32c4c` | find by rack and bay, code lookup | walk | bounded by the capacity word |
| `0xffe33b20` | per-level clear and one-record copies | multiples of 10 and 5 | compare with the capacity word, `bge` skip |
| `0xffe30294` | system roles: reads positions 0 to cap-1, then 0, 5 and (only when the local type is `5`) 10 | read | none for 10 |
| `0xffe12264` (package 1; `0xffe145b6` package 2) in the 0x000d handler | reads position 5, then 10 | read | **none** |
| `0xffe32e20`, `0xffe32f90` (record print) | print a slot and 1, 2 or 4 following slots by type | read | none |
| `0xffe31b10` send builder | for selectors 2 and 13 appends five consecutive slots from the base (positions 0 to 4); selectors 4, 5, 9, 12: position 0 | read | positions 0 to 4 only |

Registration with an L2 is not a separate path: the L2 asks for the table (selector 6 or 7) and the L1 answers from the table; the L1 does not push records to an L2. Established for the answer path (next section); the peer notices above are the "registration" of a brick with another L1 and never add a record. The routine that copies a whole record into a slot (`0xffe31ec8`) has seven callers in package 1: six inside the apply routine and one in the bounded clear-and-copy handler `0xffe33c30`; the apply routine has 16 callers (two link-task sites, seven in the receive handler, seven in the response handler). Other routines update single fields of a slot they found by rack and bay or by a fixed index; those field writers were not each enumerated, and the 54 call sites of the record-by-index routine were inspected for their index argument: constants 0 to 5 and 10, loops bounded by the capacity word, and index words taken from the port objects (the last group not traced to a value). Established for the copy routine's callers and for the apply routine's arms; the field writers are not exhaustive.

The 0x000d handler (package 1 `0xffe1220c`, read in full) loads the three words at `+8`, `+0xc` and `+0x10` of the slot for position 5, then of the slot for position 10, and puts `word8 << 6 | wordC` and word10 into the reply for each; it checks nothing, so the second pair is read from table + 0x468, + 0x46c and + 0x470, that is the next block header's flag word, its pool pointer and the first data word of the next block (inferred from section 4). In package 5 the record-by-index routine returns 0 for an index at or above the capacity, and the handler uses it without a test, so it reads addresses 8, 0xc and 0x10 instead (`0xffe44c30` to `0xffe44c54`).

## 6. The reply builders

- **Selector 6 and 7 reply (the L2's recurring query)**, package 1 `0xffe32702` to `0xffe328d8`: counts the slots with a nonzero state word among positions 0 to capacity - 1 (`0xffe3273c` to `0xffe32758`), (re)allocates two scratch arrays of that many entries, then walks positions 0 to capacity - 1. For a position whose state is nonzero: when the position is a multiple of 5 and something has been collected, it compares the slot's name (`+0x40`) with the name of every collected record; if one is equal it skips five positions (`d2 += 4`, then 1), otherwise it appends the index byte and a 0x70-byte copy. With capacity 10 only position 5 can trigger the skip: **a peer group is not reported when its head's name equals the name of record 0 or of any of positions 1 to 4 already collected** (so two bricks with equal names hide the neighbour's group). Re-read in package 0 (`0xffe327e8` to `0xffe3282a`, the remainder of `d2 / 5` hidden by the decoder as `divs.l d1, d0`): the compare runs only when the position is a multiple of 5 and something is collected; its inner loop walks every collected record from the first, not just earlier group heads, comparing the name at `+0x40` with `strcmp` (`0xffe7d35c`), and stops at the first equal one. A non-head position is never compared. The reply is: integer 7, the count, the size 0x70, the index byte array, the record block. Never more than 10 records, never position 10 or above. Established; the `divs.l` by 5 is at `0xffe327ec` in packages 0 and 1, `0xffe37fc4` in package 2, `0xffe32ab0` in package 3, `0xffe45580` in package 5 (the addresses in the earlier note are two bytes off, and the one given for package 0 and 1 is a loop test).
- **Fill of an outgoing message** (`0xffe31b10`): positions 0 to 4 only (section 5). Established.
- **The answer to a selector 2 or 13 message from a peer** carries this brick's own positions 0 to 4 (five 0x70-byte blocks, `0xffe329b8` to `0xffe32a2e`), not the group it holds for the peer. Established.

## 7. Packages compared

Package 0 against package 1 over `0xffe31300` to `0xffe34400` (4,175 instructions each, code addresses and the `0x3000xxxx` words masked, immediates and register operands kept): identical but for one mis-decoded literal (the state-block address). Package 2 against package 1 over the same routines: identical but for the data decode at both ends (5 differences, all in string data or outside the routines). Package 3 against package 1: the same instruction stream apart from the line-number constants passed to the assertion calls and the string data; the capacity selection above is the only place where the capacity differs. Package 5 differs in the record-by-index routine only (bound test returning 0) and uses the same 0x230 group clear (`0xffe44cbe`). Established by the comparison; package 3 and 5 beyond the routines named were not read.

## Corrections to earlier notes

1. [../l2/l1-conversation.md](../l2/l1-conversation.md) section 12 row 1: package 0's capacity store is at `0xffe313d8` (not `0xffe31414`, which is the store of the state word in package 1); in packages 3 and 5 the capacity is chosen by type (10, 45, 85, 125), and the allocation size there is established, not inferred.
2. Row 2: the compare runs only at positions that are multiples of 5 and against all collected names, not "a group against its own group"; the divide addresses were off (section 6).
3. Row 3 stands for capacity (10 in packages 0, 1, 2) but the firmware's own position space is 15 wide (0, 1 to 4, 5 to 9, 10 to 14): the second group of five is real code that the allocation does not cover. A host should treat positions 10 to 14 as outside the table for packages 0, 1, 2.
4. Row 14: the memory past the table is not "unread allocator rounding": it is the next heap block's header and data, and position 10 is also **written** (zero-filled and copied into) by the group arm, not only read.

## Unknowns

| Question | Route tried or to try |
|---|---|
| What block follows the table | allocation order in the initializer's callees; the allocator is deterministic but its state depends on earlier allocations in each of the five start-up paths; follow every allocation before `0xffe313e8` |
| Whether a load at address 0 or 8 faults on this board | memory map of the chip selects and the vector setup in `0xffe00400` to `0xffe00540`; not read |
| Initial values of the port objects' index words (`0x30000a2c` onward) | trace the start-up copy of initialised data from the flash image to RAM |
| Which messages carry selector 2, 13 or 20 to 25 with a group of records | the senders at `0xffe31b10` read; selectors 20 to 25 not decoded |
