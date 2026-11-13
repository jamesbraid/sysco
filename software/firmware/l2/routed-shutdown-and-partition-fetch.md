# A routed `shutdown -p` at the L2, and the partition-id fetch: what each answer means

Written 2026-11-13. Follows the chain from the IRIX 6.5.30 kernel (IP35 kernel objects), through the L1, to the L2 daemon (L2 1.44.0, `bin/l2` sha256 `9923b1ee573c2815...`, `l2part` `7fb4ccec5bf686f6...`, `lib/libmain`, `lib/libpart`; `l2.bin` sha256 `6bcfda07a24cb4d56850922dab17a98223abcd14221d307e3a8c3778434e8761`), and fixes one reading: **a status byte of 8 in an answer produced by the L2 itself means "could not be delivered", not "the command failed".** L2 addresses are virtual addresses in the named file (executable at `0x10000000`, libraries at 0). Grades: established (whole routine read), inferred (reasoning stated), unknown (routes tried). Earlier claims that were carried forward are marked "re-read: confirmed", "re-read: corrected" or "not re-read" in the table near the end. The detailed sequence note this extends is [l1-conversation.md](l1-conversation.md) (sections 6, 7 and 12); the L1 side of an undeliverable message is [../l1/power-reset-cbrick.md](../l1/power-reset-cbrick.md) section 1, read here only for the comparison.

## Answer in short

1. **What an IRIX `shutdown -p` leads to at the L2.** The kernel half is read: a power-off command sets a kernel flag; when the kernel halts, its master CPU runs `sc_power_down_all`, which sends one text command to the local L1 every 3 seconds until power drops. On a system with a router-map entry the text is `p <n> pwr d`, `<n>` being the partition number it first reads from its own L1 (message code 9, task 6); otherwise it is `* pwr d`. The `p` form is addressed to `0x1ffff003` (form 1, module field `0xffff`, task 3): the L1 forwards it to the L2, whose task-3 handler runs it as a command line. The `*` form is addressed to `0x0ffff003`: the local L1's own command interpreter. Established for the kernel routines; the user-space program that makes the system call is not in any file read, so which arguments `shutdown -p` passes is unknown.
2. **At the L2**, `p <n> pwr d` selects every non-router brick whose partition byte is `<n>` (a router only when everything outside the partition is already off), then sends one text packet `power down` per brick, each with a reply wanted, to `rack << 18 | slot << 12 | 3`. The reply to the sender is built once, after the last L1 answered or after 15 seconds. Established ([l1-conversation.md](l1-conversation.md) section 6, structure re-read here).
3. **Status byte 8** is produced in one place in the L2, the send routine's failure branch: status = `(-result) mod 100`, result `-0x260` (no receiver) gives 8. It is a transport answer: class 1, no descriptors, 20 bytes. A command that ran and failed is a different answer: status byte 0, descriptor 0 = -1, the error text in a string descriptor. Established, with the instructions.
4. **The partition-id fetch** (task 10, code 1, one integer: the brick) gets: (a) a status-8 transport answer from the L2 itself when no process holds task 10 yet (`l2part` not registered); (b) a 36-byte answer with status 0 and the partition number otherwise; (c) value 0, never -1, when the brick is not in the database, because the service clamps negative results to 0. Established.
5. **An L1 that has no L2** answers the same `0x1ffff003` command with its own undeliverable answer, whose status byte is also 8 (code -1608). Both controllers end with 8 for different internal codes; the wire byte does not say which one answered. Established for both routines.

## 1. The kernel half (IRIX 6.5.30, IP35 kernel objects)

Read from relocatable objects of the IP35 kernel build (`kernel.o` sha256 `2e847fdcadfdb958...`, `l1_command.o` `ab5c3f4dce4eda0b...`, `l1.o` `cf461fa67eb3150e...`, and the `utssys` member of the OS archive `f6f3d41ee232e0f1...`). Offsets are into each object's `.text`. The linear decode was made with the MIPS64 toolchain of the container; routine names are those in the objects' symbol tables, except where a routine has none (`restart` below is a local label of the object's debug line table).

| Step | Routine and offset | What it does | Grade |
|---|---|---|---|
| 1 | `uadmin`, `utssys` `0x1078` | the system call: accepts commands 1, 2, 4, 8 and 256, others return 22. **Command 256 with function 9 stores 1 into `power_off_flag` and does nothing else** (`0x11c8` to `0x11dc`). Command 2 and the others go through file-system and host operation tables (not followed) | established for the flag; the other arms not followed |
| 2 | `prom_reinit`, `kernel.o` `0x35c` | if `power_off_received` is nonzero: `elsc_pwr_off(get_elsc())`, then spin; else `he_arcs_restart` | established |
| 3 | `he_arcs_restart` `0x1238`, `he_arcs_reboot` `0x1258` | call the local routine `restart` (`0x10c0`) with code `0x0ead0030` and `0x0ead0040` | established |
| 4 | `restart`, `0x10c0` to `0x1230` | writes the code to every node's slot, waits 1 s, then (reboot_on_panic 2: `elsc_system_reset`) or, otherwise, in a loop with a 3 s delay (`us_delay(3000000)`): when `power_off_flag` is set, `sc_power_down_all(get_elsc())`; when it is clear, `ni_reset_port()`. The return value is ignored | established for the calls and the delay; the exit condition of the loop is not decoded |
| 5 | `sc_power_down_all`, `l1_command.o` `0x18b8` | if the word at (per-CPU data pointer) `+0x8c68` is greater than 0: `p = elsc_partition_get(elsc)` (sign-extended low byte), `sprintf(buf, "p %d pwr d", p)`, `sc_command_interp(elsc, 1, 1023, 63, buf, 0)`; otherwise `sc_power_down(elsc)`: `sprintf(buf, "* pwr d")`, `sc_command_interp(elsc, 0, 1023, 63, buf, 0)` | established |
| 6 | `elsc_partition_get`, `l1_command.o` `0xee0` | opens the destination `0x0ffff000` (the local L1), builds a message of task 6, code 9, no descriptors, sends it, and reads one integer from the answer; returns -100 or -103 on failure (`0xf08` to `0xfb8`) | established |
| 7 | `sc_command_interp`, `l1_command.o` `0x1620` | destination word = `type << 28 | rack << 18 | bay << 12` with the type 1 (partition form) or 0 (`*` form), rack 1023, bay 63: `0x1ffff000` and `0x0ffff000`; `sc_construct_msg` ORs the task, 3, into the word, gives message code 0 and the text as the payload; sends with `sc_command_kern`; on a negative result copies the answer's text and returns -112 | established for the word and the call; the payload layout (one string descriptor) inferred from the argument values 0, 2, 1 |

The word at per-CPU pointer `+0x8c68` is the same word `router_map_get` (`kernel.o` `0x11798`) bounds its walk with: it is the entry count of the router map, so the partition form is used only on a system with at least one router-map entry (inferred from the shared offset; the field is not named in the object). Established that the two forms exist and that the branch is on that word.

`sc_power_down_all` therefore asks the **local L1** twice per cycle on the partition form: once for the partition id (`0x0ffff000`, task 6, code 9; the L1 answers from its setting 14, [two-partition-nvram-and-console.md](two-partition-nvram-and-console.md)) and once for the shutdown. An id of 128 or more prints as a negative number (`p -128 pwr d`) and a failed read prints as the error code (`p -100 pwr d`); neither is a valid prefix for the L2's parser (inferred from the sign-extension at `0x18e4` to `0x18f0` and the parser below).

A second, separate path: when the L1 has already told the kernel that a power-off was asked for (`power_off_received`), `elsc_pwr_off` (`l1_command.o` `0xbec`) sends task 6 code `0x2005` to the local L1 (the general power-down handler, [../l1/request-codes.tsv](../l1/request-codes.tsv), reason 3), and no L2 is involved. Established.

## 2. The L1's part

The L1 receives a task-3 text addressed to `0x1ffff003`. With an L2 attached and its route known, it forwards the text; with no L2 it answers at once with code -1608, whose status byte is 8 ([../l1/power-reset-cbrick.md](../l1/power-reset-cbrick.md) section 1, read there for packages 0, 1 and 2). I re-read the L1's response routine (package 1 `0xffe78e0c`): it takes the negative of the code, divides by 100 (`divs.l` at `0xffe78e66`) and passes the **remainder** to the packet builder (`0xffe77eb4`) as the status; the quotient is not passed. So the L1's byte is `(-code) mod 100` exactly as the L2's. Established for the status computation in package 1; the same routine in packages 0 and 2 is stream-identical ([../l1/power-reset-cbrick.md](../l1/power-reset-cbrick.md)).

## 3. What the L2 does with the text (structure re-read, detail carried)

- **Receive.** Destination `0x1ffff003`: form 1, module `0xffff`, task 3, class 0, to `sub_1001af4c`, which reads the text from the **second** descriptor and runs it through the command-line executor `sub_10010aac`. Carried from [l1-conversation.md](l1-conversation.md) section 6; the receive routine itself was **not re-read**.
- **Target.** `p <n>` or `partition <n>` fills a 64-byte partition mask at `target + 0x1b4` with the shared range parser (libsys `0xccd0`), clears the 0x100-byte rack mask and the 0x40-byte slot mask. Re-read: the call passes size `0x40` (`CmdProcess`, libsys `0xbf88` to `0xbf90`), a number is accepted when it is below the size (`cmpw`, `blt` at libsys `0xce28`), and a lone number `n` sets byte `n` to 1 with no lower bound (libsys `0xd0c0` to `0xd0d0`). **So `p 0` is accepted and selects partition 0; 64 and above are refused.** Established.
- **Selection** (`sub_1001e554`, [l1-conversation.md](l1-conversation.md) section 6). Re-read: loops 40 ports (`cmpwi 0x27`) and skips a record with rack 0 or above 0xff or slot 0 or above 0x3f; in partition mode takes a record whose type is not `R`, `D` or `-` and whose partition byte indexes a nonzero mask byte. A partition-0 brick (a fresh or an unlisted one) is therefore selected by `p 0` and by nothing else but `p *`.
- **Power down arm** (`0x10014390` to `0x100149e4`). Re-read: the 0x27 port loop, the `R`/`D`/`-` skips (`0x100144c4`, `0x100144f4`, `0x10014524`), the router step that builds a packet per router with the text `power down` (`0x1002c1ac`), and the broadcast through `sub_1001c90c` with class mask `0xff`, the text, wait flag 1, delay 0, broadcast flag 1. Confirmed. The order in which the queue runner serves the classes (routers, I/O, compute, other) is carried from [power-queue.md](power-queue.md) and was **not re-read** in this note.
- **Wire.** One router packet per brick: destination `rack << 18 | slot << 12 | 3` (the sender builds it at `0x1001cba8`/`0x1001cbc0`: `oris 0x300`, `ori 3`), class 0, two descriptors (integer 0, the text `power down` as a string). Re-read for the task bits and the string type 0xa (`0x1001cbf4`); the descriptor values carried.

## 4. The answers

### 4.1 The answer to the line (carried, structure not re-read)

One class-1 packet to the sender's saved source word, descriptor 0 an integer, then the text in string descriptors; built once, after the last L1 answered or the 15 s timer ended ([l1-conversation.md](l1-conversation.md) section 12.1, not re-read here).

| Situation | Status byte | Descriptor 0 | Text |
|---|---|---|---|
| line parsed, nothing to do (`no L1s found...`, a parse error) | 0 | 0 | the error line |
| all bricks answered cleanly | 0 | 0 | empty (an L1 prints nothing on success) |
| a brick answered with a nonzero first descriptor | 0 | -1 | that brick's text |
| a brick did not answer in 15 s | 0 | -1 | `ERROR: no response from <address>` |
| a packet to a brick could not be sent | 0 | -1 | `ERROR: couldn't send packet to <address>, <error text>` |
| the L2's command library was busy | 0 | -1 | `ERROR: command library is busy` |

### 4.2 The answer the L2 gives for a packet it cannot deliver (re-read in full)

`sub_10022e50` (`bin/l2` `0x10022e50` to `0x10023190`) is the single entry that sends a router packet. It reads the packet's class (`lbz 0x16`); class 1 goes to `sub_10021158`, class 2 to `sub_10021eb4`, a class above 2 makes the result `-0x259` (`0x10022fd8`). On a nonzero result:

1. if the result is `-0x260` it prints `NO RECEIVER` and the packet in text form (`0x10022ff4` to `0x10023030`);
2. **if the packet's class is 0 (a command packet, as opposed to an answer)** it builds an answer: `IRouterMakePacket(router, destination = the packet's source word at +0x1c, task = (byte +0x17 >> 3) & 0x1f, code = the halfword at +0x22, class 1, byte +0x12, 0 descriptors)` (`0x10023048` to `0x100230b8`);
3. it stores into the answer's status byte (`0x13`) the value `(0 - result) mod 100` (`subf`, `divw` by `0x64`, `mulli`, `subf`; `0x100230ec` to `0x10023108`) and copies the original packet's sequence bytes (`0x10023110` to `0x10023154`);
4. it sends the answer back through the router (`sub_10021158`, `0x10023164`) and frees the original.

Results and bytes: `-0x260` (-608) gives **8**; `-0x259` (-601) gives 1; `-0x25c` (-604) gives 4 (the last code is carried from the earlier note, not re-read). A packet of class 1 or 2 that fails is freed with no answer. The answer has no descriptors, so it is 20 bytes: length `0x14`, status byte 8, class 1, the original packet's code and sequence.

So **8 means: the router found no receiver for the destination** (here `-0x260`). It is a property of delivery, not of what the command line said. It differs from every status-0 answer in 4.1, where the transport worked and the failure is in descriptor 0 and the text. Established.

### 4.3 What status 8 does and does not tell a reader

- The same byte results from the L1's `no L2` answer (code -1608, remainder 8, section 2) and from the L2's `no receiver` (code -608). Which controller answered is not in the byte; it is in the packet's source word (the L1's own address or the L2's local address). Established for both computations.
- A text command to task 3 that the L2 cannot route (for example because its source brick is in no port's table) draws the same class-1, status-8 answer; a class-1 packet that cannot be routed draws none.
- The boot firmware that fetches a partition id tests this byte (section 5): any nonzero status is a failed fetch.

## 5. The partition-id fetch (task 10, code 1)

**Who asks.** The stock IP35 boot firmware sends class 0, destination `0x1ffff00a`, code 1, one integer descriptor (the brick), and requires the answer's status byte to be 0 and its value word to differ from -1 ([l1-conversation.md](l1-conversation.md) section 7, boot firmware payload read there at `0x57450` to `0x574f8`; not re-read here). Whether IRIX makes the same fetch is unknown: the kernel objects read ask the local L1 for the partition (section 1, step 6), not the L2.

**What answers.** `l2part` registers for task 10 (`sn1SCInitL2App(host, 0, 10)`), runs only at run level 5 and starts after a 5 s sleep ([partition-task.md](partition-task.md)). A packet for `0x1ffff00a` goes through the local-client lookup (`L2CfgMFindNode`, `bin/l2` `0x100215d0` to `0x100215e0`); with no client, the routing falls through to the form tables and, when nothing matches, to `-0x260`. Re-read: the lookup and the fall-through; the individual `-0x260` return for this form was not isolated among the 15 sites in the routine. Hence:

| Case | Answer | Status byte | Value | Grade |
|---|---|---|---|---|
| daemon not registered (first seconds, or any run level other than 5) | the L2's own transport answer, 20 bytes, no descriptors | **8** | none | established for the answer; inferred that this is the branch taken (section above) |
| registered, the brick is listed (directly or through its rack when the layout is rack-wide) | 36 bytes: length `0x24`, class 1, count 2, code 1, descriptor 0 (status) 0, descriptor 1 the partition number | 0 | the partition byte | established |
| registered, the brick is not listed | same 36-byte answer | 0 | **0, not -1** | established |
| registered, the layout has no partitions (its flag byte is 0) | same | 0 | 0 | established |
| registered, a code other than 1 to 8 | 28 bytes, one descriptor, result -3 | 0 | none | carried (not re-read) |

Re-read, `l2part`: the handler for code 1 (`0x10001384` to `0x10001430`) takes the brick from the halfword at packet `+0x2a` when the first descriptor is an integer (else `IRArg`), sign-extends it, calls the wrapper (`0x10002054`), which calls `get_partid_for_brick` and **replaces a negative result with 0** (`bge`, `li 0` at `0x10002090` to `0x1000209c`), and stores result 0, the partition in descriptor 1, count 2. `partid_for_brick` (libpart `0x5374`) returns 0 when the layout's flag byte is zero, otherwise masks the brick id to `0x0fc0` when the layout is rack-wide, scans `count` entries (halfword id at `+0x30 + 4i`, signed partition byte at `+0x32 + 4i`) and returns the byte, or -1 when absent. So the PROM's own test for -1 can never fire on this service: an unlisted brick reads as partition 0 with status 0. Established.

## 6. Re-read ledger for the carried claims

| Claim carried from earlier notes | Status here |
|---|---|
| Sender of the shutdown line is IRIX (hint in the L1 note) | **re-read: confirmed for the kernel half** (section 1); user-space part unknown |
| Task 3 receive reads text from descriptor 2 | not re-read |
| `p <rng>` fills the partition mask; `p` cannot address partition 0 ([l1-conversation.md](l1-conversation.md) section 13.3: "the `p` parser then cannot address it (it counts partitions from 1)") | **re-read: corrected.** `p 0` is accepted (no lower bound in the lone-number arm) and sets byte 0 to 1; selection then takes partition-0 bricks. The statement in section 13.3 was wrong; a dated note is added there |
| Selection takes non-router bricks with a nonzero mask byte at the brick's partition | re-read: confirmed (ports, skips, types); the mask-byte test itself carried |
| Power down: router step, broadcast with class mask `0xff` | re-read: confirmed |
| Class service order | not re-read |
| Answer built once, after last reply or 15 s | not re-read |
| Send routine builds a status `(-result) mod 100`; `-0x260` gives 8 | re-read in full: confirmed |
| Task-10 handler, wrapper clamp, `partid_for_brick` | re-read: confirmed |
| PROM requires status 0 and value not -1 | not re-read |
| Status 8 for an unregistered task-10 client | inferred from the lookup fall-through (see section 5) |

## Unknowns

| Question | Route |
|---|---|
| Which arguments the user-space `shutdown -p` passes to the system call, and whether it also sends text to the L2 itself | find the program's shipped binary and read its call sites for the system call; not among the files read |
| Exit condition of the kernel's `restart` loop | decode `0x1188` to `0x11b4` of `kernel.o` against the stack frame layout |
| The field named by the offset `+0x8c68` of the per-CPU data | read the object's type information or a second routine that tests the same word with a string |
| Which `-0x260` site the unregistered task-10 packet takes | trace `sub_10021158` from `0x100215e0` through the form-1 table at `0x1002e4a0` |
| What the PROM does on a nonzero status | read the code after `0x574e4` in the boot firmware payload |
