# The L2's configuration request to an L1, the L1's reply, and how the daemon turns it into a route

Read from L2 1.44.0 (`/stand/sysco/bin/l2` and `/stand/sysco/lib/libmain.ppclinux.so`, listings by `retools/tools/sysco/l2_list.py`; addresses are virtual addresses in the named file, the executable at `0x10000000`) and, for the L1 half, from L1 1.44.0 package 1 (`Fuel/PE/O300 1MB`, first bank). The L1 package for a C-brick of a larger system was not read; the L1 half is established for package 1 only. Grades as in [../../AGENTS.md](../../AGENTS.md).

The question this note answers: the daemon logs that it has no receiver for a response addressed to a brick, and lists no L1 in its configuration, although an L1 is attached over USB. The route to an L1 is not made by the USB attach. It is made by the L1's reply to the daemon's own recurring request.

## 1. What the attach creates, and what the reply adds

The configuration object is the structure at `0x10044640` in `bin/l2` (the "local node"; library routines take it as their first argument). Relevant layout (established from `L2CfgMAddPort` `0x13a8c`, `L2CfgMCfgPort` `0x15a54`, `L2CfgMFindBrickIP` `0x160e8`, all in `libmain`):

| Offset | Content | Grade |
|---|---|---|
| `+0x000` | the node's own address word (the key the route search copies out as "ip") | established |
| `+0x124` | next node in a singly linked list of nodes (0 at the end) | established |
| `+0x158` | count of ports in use | established |
| `+0x15c + 0x1c * slot` | port record, 40 slots (slot 0 to 39) | established |

Port record (`slot` is the daemon's device index): `+0x00` state (0 free, 1 attached but not configured, 2 configured), `+0x04` connection type (2 for USB), `+0x08` the descriptor number of the device, `+0x0c` and `+0x10` two allocated name strings, `+0x14` count of brick records, `+0x18` pointer to an array of 0x30-byte brick records (all established).

**USB attach.** `l1_open_usb_device` (`0x10004680` on, the add at `0x10004b04`) calls `L2CfgMAddPort(cfg, slot, name, path, 2, fd)`. That sets the port state to 1 and stores type 2 and the descriptor; it adds no brick record (`0x13a8c` to `0x13c3c`: the record count and the array are cleared). The `config` display counts such a port as a USB connection not configured. Established. The USB attach alone therefore never gives the daemon a route.

**The reply sets state 2 and fills the bricks.** The daemon's input handler (`0x100069b0`) reads one packet, stores its slot number into the packet's `+0x0c` (`0x10006d78` to `0x10006d80`), and calls the router entry `sub_10022e50`. Class 0 or 1 goes to `sub_10021158`. For a destination word whose top bit is set and top two bits are `10`, the address form is 14 and the task is bits 29 to 24; task 4 selects the handler at `0x100218b0`, which sends class 0 to `sub_1000dfb8` (the request side) and every other class to **`sub_1000e664`**, the handler of replies to the configuration request (established by decoding the two jump tables at `0x1002e4a0` and `0x1002e4e0`).

`sub_1000e664` (`0x1000e664` to `0x1000e9e4`, read in full):

1. It acts only when the status byte at packet `+0x13` is 0 (wire byte 3). Any other value skips to the tail.
2. It classifies the packet's source word (`+0x1c`): with the top bit clear, the top nibble; with the top bit set, 14 when the top two bits are `10`, else 15.
3. Nibble **0** (an L1-form source): it calls `L2CfgMCfgPort(cfg, packet[+0xc], packet)`. If that returns nonzero (the brick table changed) it cancels the slot's pending request timer and re-arms it with a 1 s delay (callback `0x1000d9a8`, debug name `extra cfg_ping_l1`), then calls `sub_1000f93c(1)`.
4. Nibble **14** (an L2-form source): it calls `L2CfgMCfgL2(cfg, packet)` instead.
5. Any other nibble: nothing is configured.

Every path then reaches the tail at `0x1000e8f0` (a status byte other than 0 jumps straight to it), which frees the packet (the call at `0x1000e9c0`). Before that, the tail tests the word at packet `+0x1c` again, the one classified above: when its form is 1 (top bit clear, top nibble 1) or 14 (top two bits `10`) and its task is 4 (low five bits for form 1, bits 29 to 24 for form 14), it stores class 2 at packet `+0x16` and calls `sub_10022afc(0x10043ce8, packet)` (`0x1000e9a0` to `0x1000e9b8`) before the free. The test is on the packet's `+0x1c` word, not on the destination word at `+0x18`. A nibble-0 reply, the L1 reply this note is about, fails the form test and goes straight to the free. Established for the test and the call; what `sub_10022afc` does with the packet (it routes on the same `+0x1c` word and increments a field of the packet's second word, read only to that point) and whether it keeps a reference to it were not followed.

So the L1's reply must have class 1, destination task 4 of the L2 (the request's own source word), status 0 and a source word with top nibble 0. Established.

**`L2CfgMCfgPort(cfg, slot, packet)`** (`0x15a54`). Established from the full read:

- It reads descriptor 0 first (`0x15ac4` to `0x15af4`, kept in a local), then moves the old brick array aside (`0x15afc` to `0x15b24`) and stores state 2 (`0x15b30`), and only then compares descriptor 0's value with 6 and 7. The state is therefore set after descriptor 0 is read and before it is used, not before any descriptor is looked at.
- Descriptor 0 is read as an integer (descriptor byte `+0x24` low three bits zero: the word at `+0x28`; otherwise the payload by `IRArg`). Only the values 6 and 7 do anything.
- **7**: descriptor 2 is the record count `N`, descriptor 3 the record size `S`, descriptor 4 a byte array of `N` indices, descriptor 5 a data block holding `N` records of `S` bytes. For each `i` below `N` it calls the record builder `sub_156f4(port, i, indices[i], data + i*S, S)`. Descriptor 1 is read and not used.
- **6**: four records, from descriptors 1 to 4 (each a data block of at most 0x70 bytes), with the fixed indices 0, 1, 5 and 6 (the table at `libmain+0x31300`).
- The builder `sub_156f4` (`0x156f4`) copies up to 0x70 bytes of the record and writes a 0x30-byte brick record (below) at `array[i]`, growing the array with `realloc` (`malloc` for the first). At the end it compares the old and new arrays (count, then `memcmp` of count times 0x30) and returns 1 if they differ.
- When the packet pointer is 0 the port is cleared and freed (the close path calls it so, at `0x10004d60`).

**Brick record (0x30 bytes) built from a 0x70-byte source record** (`sub_156f4`; established from the stores; the value of each source word is read as a big-endian 32-bit word and only its low byte is kept, except where noted):

| Source offset | Brick record offset | Use found | Grade |
|---|---|---|---|
| (the index byte) | `+0x00` | index of the record in the L1's table; `FindBrickIP` takes the smallest | established |
| `+0x00` | `+0x01` | valid flag: **must be nonzero** for the record to be found | established |
| `+0x04` | `+0x02` | not used by the route search | established |
| `+0x08` | `+0x03` | **rack** (route key) | established |
| `+0x0c` | `+0x04` | **bay** (route key) | established |
| `+0x10` | `+0x05` | not used by the route search | established |
| `+0x3c` | `+0x06` | not used by the route search | established |
| `+0x14` (whole word) | `+0x08` | not used by the route search | established |
| `+0x18`, `+0x1c`, `+0x1d` | `+0x0c`, `+0x0d`, `+0x0e` | not used by the route search | established |
| `+0x2c`, `+0x34`, `+0x38` | `+0x0f`, `+0x10`, `+0x11` | not used by the route search | established |
| `+0x60` | `+0x12` | the value 0 here is replaced by 2 when `+0x01` is nonzero | established |
| `+0x40` | `+0x18` (12 bytes) | a string (a name, inferred) | established |
| `+0x4c` | `+0x24` (12 bytes) | a string (a serial or part text, inferred) | established |
| `+0x5c` | `+0x14` (whole word) | not used by the route search | established |

If every one of the words at source offsets `+0x00`, `+0x04`, `+0x08`, `+0x0c`, `+0x10` is at most 0xff, the low bytes are taken as they are; otherwise `ntohl` is applied first, which is the identity on this big-endian CPU, so the two branches give the same bytes. A source record shorter than 0x70 bytes is zero padded.

## 2. The route search

The case that failed: a packet for destination word `0x00044215` (the partition service's reply to the boot firmware's fetch, class 1). `sub_10021158` (`0x10021158` to `0x10021ea4`):

- Top bit clear, so the form is the top nibble, here 0. It is not 14 or 15, and the middle 16 bits (`(w >> 12) & 0xffff`) are not 0xffff, so it reaches the brick route at `0x10021cac` (established).
- **rack = `(w >> 18) & 0x3ff`, bay = `(w >> 12) & 0x3f`.** For `0x00044215` that is rack 1, bay 4 (the tag in bits 11 to 5 and the task in bits 4 to 0 do not enter the search; here tag 16 and task 21).
- It calls `L2CfgMFindBrickIP(cfg, rack, bay, &ip, &slot, &index)` (`0x160e8`). That walks every node on the list, every port whose state is **2**, and every brick record of the port, and accepts a record whose valid byte (`+0x01`) is nonzero, rack byte (`+0x03`) equals rack and bay byte (`+0x04`) equals bay, keeping the one with the smallest index byte (`+0x00`). It returns the slot, or -1 when none matched; `ip` is the owning node's `+0x00` word and `index` the matching record's index byte.
- A result of -1 gives "no receiver" (error -0x260, printed as `NO RECEIVER` with the packet in text form) unless the node's word `+0x10` is 1 and word `+0x14` names a default node, in which case the packet is forwarded there. On the stock configuration neither holds.
- With a match, if the node is the local node and the packet's form is not 1, the daemon sends the packet through the slot's writer: `sub_10007478(fd, packet)` where `fd` is the word at `0x100512f0 + slot * 0x48` (established). A match on a remote node forwards to it; a form-1 packet with a local match is dropped (`NR R.S+IR_L2!`).

**Where rack and bay come from: the message.** They are the low bytes of source words `+0x08` and `+0x0c` of the records the L1 sends. The daemon's NVRAM rack number is not consulted anywhere in this search (the search reads only the node list, port records and brick records). Established for the search; what makes an L1 put a given rack and bay into its record is in section 4.

## 3. What a real L1 (package 1) answers

The daemon's request is class 0, code 0, destination `0x0ffff00e`, source `0x84000001` in the stock run, descriptors (integer 6, serial string or integer 0, integer 0) ([../l1/usb-strings-and-transfers.md](../l1/usb-strings-and-transfers.md)). The L1 registers `config_exchange_handler` (`0xffe33ce4`) for that address; the request handler `config_exchange_request` (`0xffe325ec`) was read in full for this selector, with the helpers `0xffe388c8`, `0xffe388ec`, `0xffe3892c`, `0xffe38998`, the reply builder `0xffe77eb4`, and `response_send` (`0xffe78e0c`).

Steps for selector 6 with three descriptors (established):

1. The reply's descriptor list starts as one integer, the selector (`0xffe388c8`: count 1, descriptor 0 an integer).
2. Serial (`0xffe3267e` to `0xffe326fe`): with at least two descriptors the string (or integer value) of descriptor 1 is the serial. A nonzero value is compared with the text at `+0x64` of the first record of the L1's table and, when different, copied there and the setting is updated (`0xffe2fb08`, argument `0x8000`). An integer 0 (an empty daemon setting) is skipped.
3. With three descriptors (`0xffe32702`), descriptor 2 is read as an integer `a2`. The L1 counts the entries of its table whose word at `+0x60` is nonzero (the table is the 0x70-byte records at the pointer in RAM word `0x30000aa4`, count in `0x30000aa8`) and reallocates its two scratch arrays when that count changed.
4. It then **restarts the descriptor list with the integer 7** (`0xffe327b0`: the second call to the init helper resets the count to 1), so the reply's first descriptor is 7 and not the selector.
5. When `a2` is nonzero the build stops there: the reply carries the single descriptor 7. When `a2` is 0 (the daemon always sends 0) it walks the table: skip entries with `+0x60` zero; for an entry whose index is a multiple of 5, compare its string at `+0x40` against those already collected and skip the whole group of five (index plus 4) if one matches; otherwise collect the entry: record its index as a byte and copy its 0x70 bytes. `N` is the number collected.
6. Descriptors appended, in order (`0xffe32862` to `0xffe328d4`): integer `a2`, integer `N`, integer `0x70`, a byte array of the `N` indices (descriptor type 1, no copy), a data block of `N * 0x70` bytes (type 1 with the free flag, copied). This is the shape `L2CfgMCfgPort` expects for selector 7.
7. Tail (`0xffe3294e` to `0xffe32c40`): the L1 marks word `+0x58` of its first table record -1 and sends two requests of its own (selectors 2 and 13) through `0xffe31b10`, both **before** it builds and sends the reply (the code falls through to `0xffe32bba`, where the reply is built). The test at `0xffe3296a` cannot skip them: the flags come from `addq.l #1` on a value that is 0 or 1, so the result is 1 or 2 and the `beq` never branches (established from the listing). Each request goes to destination `0x0ffff00e`, class 0, code 0; these carry five consecutive table records and are not part of the reply (their consumer was not followed). The reply is then sent by one of two builders chosen by the request's `+0x0c` word: `response_send` (request `+0x0c` in 0, 3 to 8 or above 12; table at `0xffe32bd0`) or `packet_build_wrapper` (1, 2, 9, 10, 11, 12). The L1-side meaning of that word for a packet that arrived over USB was not read.

Reply fields, for both builders (established from `0xffe77eb4` and its two callers):

| Field | Value | Grade |
|---|---|---|
| class | 1 (response) | established |
| destination word | a copy of the request's source word (`0x84000001` in the stock run) | established |
| source word | the first word of the L1's endpoint record at `0x30004f88`, set by `endpoint_init` with task 14: the low 20 bits of the L1's identity word `0x30000080` in bits 31 to 12 and the task in the low five bits (top bit clear), so `(identity << 12) \| 0x0e`, with the other low bits as the record held them | established |
| status byte (`+0x13`, wire byte 3) | 0 | established |
| code | the request's code (0) through `response_send`; the request's `+0x0c` value through the other builder. The daemon's handler does not read it | established |
| sequence | the request's, through `response_send` | established |
| descriptors | 7, `a2` (0), `N`, `0x70`, byte array of `N` indices, `N * 0x70` data bytes | established |

The identity word's low 20 bits (the form in `0xffe782a4`) are the rack in bits 15 to 6 and the bay in bits 5 to 0, which is what the `0x000c` request reply computes as `rack << 6 | bay` from the table words at `+0x08` and `+0x0c` ([../l1/brick-request-handlers-detail.md](../l1/brick-request-handlers-detail.md)). Inferred that the source word's rack and bay are the L1's own rack and bay settings (settings 1 and 2, [../l1/settings-record.md](../l1/settings-record.md)); the routine that stores the identity word is `0xffe78314`, see "Who writes the identity word and the table" below.

**Where the table's rack and bay come from.** The record words at `+0x08` and `+0x0c` are the same words the `0x000c` handler reports as rack and bay of each attached port. The L1's own record is entry 0 of the table and takes them from settings 1 and 2 (next section).

### Who writes the identity word and the table (established, L1 1.44.0 package 1 `Fuel/PE/O300 1MB`, first program copy)

- **The identity word.** `0x30000080` is the field at `+0x10` of a state block at `0x30000070` that the packet layer keeps. Its only store is in the routine at `0xffe78314` (`0xffe78350`), which stores its argument masked to 20 bits there and `1` or `2` at `+0x0c` of the block. That routine has one caller, in the main task's start-up path at `0xffe10602`. The caller reads setting 1 (the rack) and setting 2 (the bay) from the settings record ([../l1/settings-record.md](../l1/settings-record.md)), forms `((rack & 0x3ff) << 6) | (bay & 0x3f)` (`0xffe105d8` to `0xffe10600`) and passes it. The packet layer's init at `0xffe782a4` reads the word back (`and.l` with `0xfffff` at `0xffe782b8`). So the low 20 bits of the source word are exactly the L1's rack and bay settings, taken once at start-up; no other instruction stores the word, so a later change of a setting does not reach it (inferred from the absence of stores).
- **The table.** The word at `0x30000aa4` is the table pointer and the word at `0x30000aa8` its capacity. Both are stored by the routine at `0xffe3132c` (`0xffe313da` stores the capacity `10`; `0xffe313f0` stores the pointer returned by the allocator at `0xffe55d34` for `10 * 0x70` bytes). It is called once, from the main task's start-up path at `0xffe10622`, right after the identity word is set. It clears the block and fills entry 0, the L1's own record: `+0x00` = 1 (valid), `+0x60` = 2, `+0x08` = setting 1 (rack), `+0x0c` = setting 2 (bay), `+0x10` = the brick-type byte at `0x300002f8` (the byte the board initializer chose), `+0x14` = the result of `0xffe547d0(1)`, `+0x3c` = setting 14 (partition; a value of `0xff` or more is replaced by 0 and logged), `+0x40` = a copy of the string behind setting 15, `+0x4c` = a copy of the string behind setting 13 (the system serial number, copied through a global buffer at `0x30004df4`), `+0x34` and `+0x38` = the results of `0xffe4c2fc` and `0xffe4b7c0`, `+0x18` and `+0x2c` cleared. The routine then switches on the brick type for further fields (jump table at `0xffe314f0`, not read in full). Entries 1 to 9 are zero after this routine; they are written later by the record-apply routine at `0xffe31f18` (not read in full here), while `0xffe31f00` only computes an entry's address (`base + index * 0x70`).
- **What a fresh L1 reports.** Settings 1 and 2 live in the battery-backed settings record. When neither the record nor its mirror passes its checksum, start-up zeroes the record, gives it its default version and a fresh checksum ([../l1/flash-nvram-and-image-records.md](../l1/flash-nvram-and-image-records.md)); rack (bits 23:14 of the first word) and bay (byte `+2` bits 5:0) are then 0. Inferred: a fresh L1 therefore reports rack 0 and bay 0, identity word 0 and source word `0x0e` for task 14, in its own table entry and in the reply to the first configuration request. Not established: the version-nibble upgrade path (jump table `0xffe5a858`) was not read for defaults it might write into those fields, and the board initializer at `0xffe55254` runs before the settings are read and was not checked for a store into the record.
- **Initial values.** The flash image carries initial values for the whole block `0x30000000` to `0x30000c38` (copied at boot, [../l1/board-boot-contract.md](../l1/board-boot-contract.md)): the identity word and the table pointer are both 0 there in both program copies, so nothing reads a stale pointer before the two routines run.

**What the daemon dereferences in a reply, and what faults.** `sub_10022e50` reads class (`+0x16`) and the destination word (`+0x18`); the router reads the source word (`+0x1c`); `sub_1000e664` reads `+0x13`, `+0x1c`, `+0x0c`; `L2CfgMCfgPort` reads the descriptor bytes at `+0x24`, `+0x2c`, `+0x34`, `+0x3c`, `+0x44`, `+0x4c` whatever the descriptor count says. For selector 7 it takes descriptors 4 and 5 as pointers when their type bits are nonzero and as the integer value otherwise; **an integer in those places with `N > 0` is dereferenced as an address** (`lbz` through descriptor 4's value at `0x15d70`, `memcpy` from descriptor 5's value in `sub_156f4`), which faults. A data block shorter than `N * S` is read past its end without a fault from this routine. A selector other than 6 or 7 sets the port to state 2 and also drops its brick records (the array pointer and count are cleared before the selector is read, libmain `0x15b10` to `0x15b30`), which counts as a change when records existed ([../l2-l1-conversation.md](../l2-l1-conversation.md) section 11). Established for the dereferences; that this is the cause of the fault the stock run saw with a guessed reply is plausible, not established. The stock run's signal 11 trapped at `0x10004c94`, the return address after the device-name call (`L2CfgMDeviceName`, `0x10004c90`) in the device-close routine (`sub_10004bf8`), not inside `L2CfgMCfgPort` or the record builder, so the fault may be on a later path than the dereferences above.

## 4. The request period

The first request follows the device open by 0.75 s (`EventAddTimeOut(0, 750000, 0x1000d9a8, slot)` at `0x10004b54`). The callback re-arms itself at its end (`0x1000dc5c` to `0x1000dcf8`) with `EventAddTimeOut(period, 0, ...)`, the first argument in seconds, and:

- `period` is the word at `0x10043d54` (`L2<->L1 freq` in the `cfg` settings text at `0x1002ca80`), **unless the port record's state word (`0x1004479c + slot * 0x1c`, the first word of the record) is not 2, in which case it is 2**. The earlier reading of this test as a port type was wrong: the word tested is the state, which `L2CfgMAddPort` sets to 1 and `L2CfgMCfgPort` sets to 2.
- So the period is 2 s from the first re-arm until a reply has been processed by `L2CfgMCfgPort`, and the key 15 value afterwards. The re-arm happens in the callback, before the reply to the request it just sent can arrive: the sequence is first request at 0.75 s, second at 2 s later, third at the key 15 period after that. With no processed reply the period stays 2 s forever. A reply that changes the brick table cancels the pending timer and re-arms at 1 s once (section 1). Established. The stock run with a guessed reply shows requests at 0.0, 2.0, 7.0, 12.0 s, which fits a reply that reached state 2 without changing the table; a reply that carries bricks should give about 0, 1, 6, 11 s ([../l2-l1-conversation.md](../l2-l1-conversation.md) section 5).
- **Key 15** (`nvramGet(15)` returns the 32-bit word at offset `+0x58` of the internal-use area, `0xd200` in `libmain`). The area is zero-filled on a fresh or checksum-failing store (`0xcaf8`), so key 15 reads 0. At start-up (`sub_1000d620`, called from `0x10003b64`) the daemon loads keys 11 to 15; if any of 11, 12, 13, 14 or 15 reads 0 it writes the defaults (`sub_1000d570`: key 11 = 1, 12 = 3, 13 = 2, 14 = 5, **15 = 5**) and reads them back. So on a fresh store key 15 is 0 before the first start and 5 after it; the default in force is 5 s. The settings text names the keys: 14 `L2<->L2 freq`, 11 `min`, 12 `max`, 13 `ping from sender valid` (1 yes, else no), 15 `L2<->L1 freq`; the word at `0x10043d3c` (`cfg throttle`, default 2) is not an NVRAM key.
- The callback sends only while the word at `0x10043c00` is nonzero (initial value 1, `.sdata`); it reschedules either way.

## 5. The flashed-version lookup

`main` calls `sub_10003ca4` (`0x10003ca4`) and stores its result as the "Flashed L2 version". The routine:

1. `p = flash_read_addr(0xfe040000)`; `delta = p - 0xfe040000`. The address is window offset `0x40000` when the window base is `0xfe000000` ([libdrivers.md](libdrivers.md)).
2. Under `flash_lock`, `FlashValidateImage(p, 0xfe040000, 0x300000, 0, 0, &header, 0)`; then `flash_unlock`.
3. On a negative result the version is the empty string (`0x10027214`, the byte at that address is 0). Otherwise the version is the C string at `ntohl(header + 0x14) + delta`. The result is copied into a 32-byte buffer (`0x10051e18`).

An empty flashed version prints the corrupt-image line. The *current* version (`Current L2 version`) is the 32 bytes of `/tmp/l2ver` when that file exists, else the flashed version is copied there; an empty flashed version leaves it empty and the daemon prints the placeholder for an unknown version. Both lines in the run came from one failed validation.

`FlashValidateImage` (`libmain` `0x1d0d4` to `0x1d590`, read in full; arguments `ptr`, `vbase`, `size`, `verbose`, `_`, `&header`, `&reloc`; all words big-endian). It fails, returning -1 with a text in a 0x37a14 buffer, on the first of:

| Test | Text |
|---|---|
| `ptr` is -1 | local image pointer |
| the word at `ptr` (virtual address `P` of the header) is below `vbase` or above `vbase + size` | image header pointer |
| the halfword at `P + 2` (header size `hs`) is below 0x20, or `P + hs` is above `vbase + size` | bad image header size |
| the sum of the words of the `hs` header bytes is not 0 modulo 2^32 | bad image header checksum |
| the word at `P + 0x18` (virtual address of the relocation table) is 0 | relocation table missing |
| the word at `P + 0x20` is nonzero and the sum of the words from `ptr` over `ntohl(P + 0xc)` bytes differs from it | bad image checksum |
| (only when `P + 0x20` is nonzero) the sum of the words of the table, whose byte length is the second word of the table, is not 0 | bad relocation table checksum |

A zero word at `P + 0x20` is accepted ("no image checksum") and skips both checksum tests. On success the routine returns 0 and stores the local header pointer.

**What 1.44.0 has there.** Placing the payload of `l2.bin` so that its first byte after the 0x2c-byte file header is flash window offset 0 (the file header's load address is `0xfe000000`; established that only this placement passes) gives, at window offset `0x40000`, the word `0xfe2ca00c` (a pointer to the 32-byte header at the end of the image). The header words are `0x00030020` (the halfword at +2 is 0x20), `0xfe040000`, `0x0028a02c`, `0x0028a02c` (the checked length), `0xfe2c9ff4`, `0xfe2ca004` (the version string, `1.44.0`), `0xfe2ca030` (the relocation table) and `0x0721df60`; the word at `+0x20` is `0xee2d1953`, which equals the sum of the `0x28a02c` bytes from window offset `0x40000` (computed on the carved file, not run through the daemon), and the table is `1, 0x10, 0xffffffef, 0` (sum 0). So a flash that holds the payload from offset 0 passes every test and the daemon reads `1.44.0`; the file with its own 0x2c-byte header at offset 0, or erased flash, fails the pointer test (erased flash gives the word `0xffffffff`). Established by hand computation over the bytes of `l2.bin`; not by running the daemon on such a flash.

## Unknowns

- Whether the settings upgrade path or the board initializer can leave a non-zero rack or bay in a fresh record, which would change the "rack 0, bay 0" reading above. Test that would settle it: read the version upgrade steps (jump table `0xffe5a858`) and the stores of `0xffe55254` for writes to record bytes `+0x00` to `+0x02`.
- What the L1 puts in the request's `+0x0c` word for a packet that arrived over USB, which decides the builder used for the reply (the two differ only in the code halfword, which the daemon ignores).
- The consumer of the L1's own requests (selectors 2 and 13) to `0x0ffff00e`, which the L1 sends before its reply to the first configuration request.
- The format of the `config` display (`L2CfgMPrint` and its variants, the text for a configured USB port).
- The C-brick L1 package: the same reply shape is assumed from the shared request handler design, not read.
