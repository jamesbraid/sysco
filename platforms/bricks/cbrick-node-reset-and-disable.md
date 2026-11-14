# Compute bricks: what reset, soft reset, NMI and disable do per node (type `C` and type `6`)

This note settles one question from the earlier hand-off note ([l1-bedrock-handoffs.md](l1-bedrock-handoffs.md), section 11): on the two L1 personalities that have several processor parts behind one controller, which console commands reach one node and which reach the whole brick, which lines or registers each one drives, and what each one tells the L2. It re-reads every routine named in that section and in [power-reset-cbrick.md](../../software/firmware/l1/power-reset-cbrick.md); section 6 marks each earlier claim confirmed, corrected, or not re-read.

Grades follow [../../AGENTS.md](../../AGENTS.md). **E** established (routine read in full, claim follows), **I** inferred (reasoning stated), **U** unknown (static route listed in section 8).

## 0. Inputs and how they were read

- Image: IRIX 6.5.30 `l1.bin`, L1 1.44.0, container sha256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
- **Package 0** (`Base 1MB image`, body sha256 `3c8a7aa93eb38502289cf0c2921ea91e6800228005bd8abd67425ce615f7b364`): type `C`. It has no type `6` code. **Package 2** (`MIPS 2MB image`, body sha256 `ac81f501f01c6789e756ab1f5ec69fadb1e4c04225511d541340831686e04e70`): both `C` and `6`. Both load at `0xffe00000`; addresses are first-bank. Container offset = `0x68` + (address - `0xffe00000`) in package 0 and `0x1dfb5c + 0x2c` + (address - `0xffe00000`) in package 2. Packages 1, 3 and 5 carry neither type (supported-brick strings in [personalities.md](../../software/firmware/l1/personalities.md)); they were not read for this note.
- Method: every package body was decoded linearly with the capstone M68K decoder in 68040 mode (the same decoder as the committed listings), jump tables and word tables were read from the raw bytes, and calls were found twice, once from the listing and once from the raw bytes at any alignment. The package 0 and package 2 type `C` hook bodies were compared instruction by instruction with address operands masked: hold (`0xffe44bd4` / `0xffe4ac44`), release (`0xffe44dfc` / `0xffe4ae6c`), second-reset query (`0xffe4458c` / `0xffe4a5fc`) are identical, and the soft reset and NMI hooks were compared by eye (same instructions, other call targets and flag address).
- "Node" in this note is what the console calls it. On type `C` the image has two processor-module slots (`PIMM0`, `PIMM1`, two CPUs each) behind one control block; how many Bedrock chips that is is not in the image (U1).

## 1. Answer

| Command | Type `C` (packages 0 and 2) | Type `6` (package 2) |
|---|---|---|
| `reset`, `reset hold` | Brick-wide. Hold hook `0xffe44bd4` clears the four reset-group lines, then the per-module `VCC OK` step. `reset` follows with a 10-tick wait and the release hook. Carries no node argument. E | Brick-wide in the command, per node in the hooks: the same sequence once per node that is present and enabled (hook `0xffe50ad0`, iterator `0xffe5e0d4`, per-node routine `0xffe63dec`). A disabled node is skipped. E |
| `softreset` | Pulse of `BEDROCK CNTL 1` bit 7: set, 10 ticks, clear (hook `0xffe445e8`). E | Same pulse on the `ASIC CTRL 1` expander of every present and enabled node, one after another (hook `0xffe50b68`, per-node `0xffe63750`). E |
| `softreset <n>` | Identical to `softreset`. The hook never reads its argument. E | Pulse for node `n` only. A disabled node gives the "node disabled" error, an absent node "node not present", an index past the count "invalid node index". E |
| `nmi`, `nmi <n>` | Pulse of `BEDROCK CNTL 1` bit 6: clear then set, no delay (hook `0xffe4463c`). `nmi <n>` is identical to `nmi`. E | Same pulse on the `ASIC CTRL 1` bit 6 of every enabled node, or of node `n` (hook `0xffe50b88`, per-node `0xffe637d8`). E |
| `reset <n>` | Not a command on this type. Package 0 has no such row. In package 2 the handler refuses type `C` with the "not supported on this brick type" line. E | Holds node `n` and leaves it held (handler `0xffe167a0`, per-node `0xffe63dec`). Nothing in the command releases it. Index accepted 0 to 4. E for the calls, I for "stays held until a brick-wide release" |
| `node`, `node e`, `node d`, `node <n> e`, `node <n> d` | Do nothing. Package 0 has no such rows: the text falls to the empty-set row `fru\|pci\|prom\|node <command>` whose handler (`0xffe141b4`) only prints the unsupported-type line. In package 2 the `node` handler (`0xffe165c8`) refuses type `C` the same way. E | Handler `0xffe165c8`. Disable: refuses unless the supply-state routine returns 0, then runs the type `6` hook per node and sets bit `n` of persistent setting `0x70`. Enable clears the bit. No expander line is driven. E |
| `cpu d`, `cpu <n>x d` | Sets bit `0x40` (odd CPU) or `0x80` (even CPU) of `PIMM0 POWER` (CPUs 0 and 1) or `PIMM1 POWER` (CPUs 2 and 3), and (per the earlier note) stores 1 in setting `0x58 + CPU`. The only per-part disable. E for the lines | Sets one of bits `0x04`, `0x08`, `0x10`, `0x20` of the node's `CPU CTRL` expander (node = CPU index shifted right by 2). Hook `0xffe63cb4`. E for the target, I for polarity (same set-to-disable as type `C`) |

**A host that gives type `C` a per-node disable is wrong in four ways.** (1) No console command exists to do it, in either package; `node d` is refused or swallowed. (2) The only soft-reset, NMI and hold lines on `C` are single bits of one expander, so nothing selects a node: `softreset 1` is `softreset`. (3) The only persistent disable state on `C` is per CPU (settings `0x58` to `0x5b`), applied through `PIMM n POWER` lines and re-applied at board init; no node bit exists. (4) The node-disable bit mask (setting `0x70`) and the iterator that honours it exist only in the type `6` code; a disabled `C` node would not be skipped by anything.

## 2. Which brick types may run each command (E)

The handler of every row below first runs the type check `0xffe140d4` (package 0) or `0xffe198ac` (package 2). It succeeds when the brick-type character is in the row's set string, or when the character's class letter is, and returns 0; otherwise it writes the unsupported-type line and returns -1 (read in full, package 0 `0xffe140d4` to `0xffe141b0`). The class table is 90 bytes at `0xffe21014` (package 0) and `0xffe267ec` (package 2), indexed by the type character; `C` and `6` both map to class `c`.

| Row | Set string | Where read |
|---|---|---|
| `softreset` | `OC14567890QA!@BDSFHL` | [command-table.tsv](../../software/firmware/l1/command-table.tsv), packages 0 and 2 |
| `softreset <n>` | `C56@` | same |
| `nmi` | `c12!` (class `c`) | same |
| `nmi <n>`, `nmi <rack_slot> <word>` | `C1256@`, `C125@` | same |
| `cpu`, `cpu e`, `cpu d` and the node forms | `c` | same |
| `node`, `node e` | `6780EQVO@BFHL` | package 2 only, table 2 |
| `node d` | `6780QO@BFHL` | package 2 only |
| `node <n> e`, `node <n> d`, `reset <n>` | `6@` and `6` | package 2 only |

The package 2 table that holds the `node` rows is registered by the common tail of the board-descriptor setup (call `0xffea399a` to `0xffe18030`), so it is present on every brick type there, and the class check is the only restriction. Help lines for the three node-addressed rows read `soft reset node <exp>`, `NMI (non-maskable interrupt) node <exp>` and, on the type `6` row `reset <n>`, `reset node <exp>`; the node commands read `enable node <exp>` and `disable node <exp>`.

## 3. Type `C` in detail

**Selection of the hooks.** Package 0: the board-descriptor dispatcher (`0xffe44394`, word jump table at `0xffe443bc`, indexed by type character minus `0x26`) sends `C` to `0xffe44424`, which calls the descriptor initialiser `0xffe45504`. Package 2: the dispatcher `0xffea38f4` has no `C` case and falls to `0xffe4a404` (table `0xffe4a42c`), whose `C` entry (`0xffe4a494`) calls `0xffe4b574`. Both initialisers store the same set of hook pointers, listed with the package 0 address first and the package 2 address second:

| Offset | Role | Package 0 | Package 2 |
|---|---|---|---|
| `+0x80`, `+0x84` | CPU enable, CPU disable | `0xffe44a90`, `0xffe44ab8` | `0xffe4ab00`, `0xffe4ab28` |
| `+0x98` | hold | `0xffe44bd4` | `0xffe4ac44` |
| `+0x9c` | release | `0xffe44dfc` | `0xffe4ae6c` |
| `+0xa0` | second-reset query | `0xffe4458c` | `0xffe4a5fc` |
| `+0xa4` | soft reset | `0xffe445e8` | `0xffe4a658` |
| `+0xa8` | NMI | `0xffe4463c` | `0xffe4a6ac` |

Neither initialiser stores `+0x90` or `+0x94`, the node enable and disable slots that the type `6` initialiser fills (section 4). The descriptor base is `0x30001e30` (package 0) and `0x3000281c` (package 2). A slot that was never stored is read as null; the wrappers test for null and return 0 (E for the wrapper test; I that RAM starts clear). E for all the stores.

**Soft reset and NMI hooks (E, both packages).** Each hook first tests the board-initialised flag (byte `0x30000098` in package 0, `0x300005a0` in package 2) and returns -900 (`0xfffffc7c`) without touching any line when it is zero. Soft reset then sets bit 7 (`0x80`) of expander 6, waits 10 ticks, clears the bit. NMI clears bit 6 (`0x40`) of expander 6, then sets it, with no wait. Neither body reads a stack argument: no access to the first argument slot appears in either routine. Expander 6 is the sixth record of the expander table (package 0 `0xffe455dc`, package 2 `0xffe4b64c`; 16-byte records: segment, wire address, input mask, applied value, name pointer): segment 3, wire address `0x42`, input mask 0, applied value `0x72`, name `BEDROCK CNTL 1` (string at `0xffe45b7f`). The set routine (`0xffe65ef8` / `0xffe87a34`) and the clear routine (`0xffe65f1c` / `0xffe87a58`) call the masked writer `0xffe65e84(index, mask, value)`, which replaces the masked bits of the expander's shadow byte and sends the byte (read in full, package 0). Idle levels (soft reset low, NMI high) follow from the applied value `0x72` and the order of the writes (E for the writes, I for which level asserts the line on the board).

**The wrappers.** `softreset` and `nmi` handlers call `0xffe4f0ec(reason, node)` and `0xffe4f0a4(reason, node)`; the node byte is the typed number or `0xff`. The wrapper logs (unless the reason is 5), loads the hook from the descriptor (`+0xa4` or `+0xa8`), calls it with the node byte and returns its result, or 0 when the slot is null. The hook ignores the byte (E).

**Hold and release (E for the sequences; they match section 3 of the earlier note).** Hold (`0xffe44bd4`): compare the two module type bytes (`0x30000090`, `0x30000091`) with settings `0x56` and `0x57` and, on a mismatch, rerun the default voltage margins and store the types; then, if the board flag is set, clear `BEDROCK CNTL 1` bit 7, `CNTL 1` bit 1, `BEDROCK CNTL 2` bit 0, `CNTL 1` bit 0 in that order; then per module set the `VCC OK` bits `0xc0` of expander 13 and 15. Release (`0xffe44dfc`): strap write to `CNTL 2` under mask `0xf8`, 10 ticks, clear `CNTL 2` bit 3, 10 ticks, set `CNTL 1` bit 0, 10 ticks, module `VCC OK` clears with 3- and 10-tick waits, then set `CNTL 2` bit 0 and `CNTL 1` bit 1. `reset` runs hold, waits 10 ticks and runs release; during `power up` it runs twice, 400 ticks apart, for the module types the second-reset query accepts (read in full, package 0 `0xffe4f324`).

**`cpu d` (E for the hook `0xffe449e4` in package 0; the console handler and the settings stores are the earlier note's reading and were not re-read).** `0xffe449e4(index, enable)`: bit 1 of the CPU index picks the module (expander 12 `PIMM0 POWER` or 14 `PIMM1 POWER`, names at `0xffe45bcd` and `0xffe45be6`), bit 0 the mask (`0x40` odd, `0x80` even). Enable clears the bit, disable sets it. The module must be present (type not `0x0f`, else -901), and for a type outside 3, 4, 5, 7 and 10 enable returns success without writing and disable returns -902. The earlier note says the command refuses unless the supply-state routine (`0xffe4c6ac`) returns 0 and, on success, stores 1 or 0 in setting `0x58` + CPU index; the setter calls with key `0x58` + index are present (`0xffe528b4`, `0xffe5299a`), their values were not re-read.

## 4. Type `6` in detail (package 2)

**Selection and hooks.** The identity code ends by calling the dispatcher `0xffea38f4` with the descriptor base `0x3000281c` and the type; its `6` case (`0xffea3928`) calls the initialiser `0xffe517c4`. The initialiser stores `+0x80` `0xffe63c30` (enable CPU), `+0x84` `0xffe63cb4` (disable CPU), `+0x90` `0xffe635f0` (enable node), `+0x94` `0xffe63404` (disable node), `+0x98` `0xffe50ad0` (hold), `+0x9c` `0xffe50b1c` (release), `+0xa4` `0xffe50b68` (soft reset), `+0xa8` `0xffe50b88` (NMI), `+0xc0` `0xffe51790` (scan hook run by the release wrapper), and the node count 4 at `+0x88` (E).

**Iterator `0xffe5e0d4(routine, node)` (E).** With node `0xff` it calls the routine for every index below the count at `0x300028a4` for which the enabled test `0xffe5e090` is true (bit `n` of setting `0x70` clear), and returns the first nonzero result. With one node it checks the index against the same count (error `-4201`), calls the routine if enabled, and otherwise returns `-4203` when the node exists (`0xffe5e050`) or `-4204` when it does not. **Corrected 2026-11-13:** the words for these codes are in section 9 (E). `-4201` is "invalid node number", not "invalid node index": the string `invalid node index` at `0xffe7e3e4` belongs to a different module's table (the Twister group), and so does `power must be down to enable/disable nodes`.

**Per-node routines (E).** Node record = 28 bytes at the array behind `0x30000618`; byte `+4` is the first of the node's expanders and `+3` of it is `ASIC CTRL 1`, `+4` `ASIC CTRL 2`, `+5` `CPU CTRL` (name order in the string pool at `0xffe5d89f` onward; the base index itself was not traced to the pool, so the names are I).
- Soft reset `0xffe63750`: set `0x80` on `ASIC CTRL 1`, 10 ticks, clear `0x80`.
- NMI `0xffe637d8`: clear `0x40` on `ASIC CTRL 1`, then set `0x40`.
- Hold `0xffe63dec`: if the node's type byte (table `0x30003046`) is not `0x0f`, compare it with the stored type setting (record byte `+0x11` is the key) and on a change log it and store it; then clear `ASIC CTRL 1` `0x80`, `ASIC CTRL 1` `0x02`, `ASIC CTRL 2` `0x01`, `ASIC CTRL 1` `0x01` (the type `C` order), then the module step `0xffe63d34`. The release hook runs `0xffe6405c` per enabled node.

**`node d` (E).** `0xffe5e1f8(n)` returns -4202 when the supply-state routine (`0xffe6cb10`) is nonzero (`Node:power must be down to enable/disable NODEs`, string `0xffe7e4f7`; the string at `0xffe7e3f7`, worded `...enable/disable nodes`, belongs to another module group); otherwise it loops over present nodes (node `0xff`) or takes one. Per node `0xffe5e188` range-checks, calls the hook at `+0x94` (`0xffe63404`, which calls `0xffe63174(n, 0)`), and on success sets bit `n` of setting `0x70` (getter `0xffe7a420`, setter `0xffe795e8`). Enable is the mirror (`0xffe5e278`, hook `0xffe635f0`, bit cleared). The hook closure was searched: following direct calls to depth 8 (130 routines) from `0xffe63174` never reaches the expander writers (`0xffe879c0`, `0xffe87a34`, `0xffe87a58`, `0xffe65e84`). The hook changes monitoring flags only (I for "monitoring": the flag roles come from neighbouring error text). By contrast `0xffe63cb4` (CPU disable) reaches the writers in three calls.

**Other readers of setting `0x70` (E for the sites, U for the effects).** The enabled test (`0xffe5e090`), the power-up diagnostic (`0xffe5e406`), the scan-description builder `0xffe51678` (a set bit gives the node state 0 instead of 2 in its entry list, mode-dependent; U for when), `0xffe63608`, `0xffe6393c`, and the enable/disable code above.

**`reset <n>` (E).** Handler `0xffe167a0` runs the class check (`6` only), compares the index with 4 (`subq`, branch if lower or same), so 0 to 4 are accepted although the node count is 4; it then calls `0xffe63dec(n)` directly, not through the iterator, so a disabled node is held anyway. Index 4 reads the type table one byte past the four entries (consequence U). Section 9 adds the printed words and what is and is not known about index 4.

## 5. What reaches the L2 (E unless marked)

The event builder is `0xffe33060(kind, a, b)` in package 0 (`0xffe38838` in package 2); it hands a message to the sender `0xffe79084`. Following direct calls to depth 8 or 9 from each routine below:

| Routine | Reaches the builder? |
|---|---|
| `softreset` and `nmi` handlers, wrappers, and the type `C` hooks (package 0) | No |
| The same on type `6` (handlers `0xffe21678`, `0xffe21434`, wrapper `0xffe6f550`, hooks, per-node routines, iterator) | No |
| `node`, `node d` handlers and hooks (package 2) | No |
| Hold wrapper `0xffe4f29c` (package 0), used by `reset`, `reset hold` and power-down | Yes: `0xffe36a04` clears the service state of ports 2 to 9, then calls `0xffe3681c(0)`, which stores 0 in word `+0x14` of record 0 and, only when the word was different, calls `0xffe33238`, which sends a **kind-2** event carrying that word (builder case `0xffe33106`). I that the word is the console subchannel (the `select` console command and the message that selects a subchannel write it through the same routine; the earlier note on the L1/L2 conversation says the same) |
| Hold hook margin path (`0xffe44bd4` through `0xffe4e40c`), and package 2 `reset <n>` through `0xffe6e870` | Yes, only when a module type changed since the last hold: the margin reset may send the margin-state event (builder kind 5, record word `+0x38`; package 2 `0xffe38a70`). I for what the event is |
| Power-state notice (kind 4) | Not from any of the above: only the setter `0xffe33268` sends it |

So the claim in the earlier note, "a reset sends no notice", is true of the power-state notice and false as a statement about every message: a brick-wide hold can send the kind-2 event, and a module-type change adds a margin event. `softreset`, `nmi`, `node d` and `softreset <n>` tell the L2 nothing. Whether an attached L2 learns of a disabled node (setting `0x70`) was not found: no reader of the setting is on a path to the builder (U4).

## 6. Ledger of earlier claims

| Claim (source) | Status |
|---|---|
| Type `C` has no node disable command; `cpu` is the only per-part disable (handoffs 11.1) | re-read: confirmed; adds that `node` commands are refused or swallowed on `C` (section 2) |
| `node`, `node d`, `softreset <n>`, `nmi <n>` on `C` act on the whole brick through one shared bit pulse (assignment wording) | re-read: corrected. `softreset <n>` and `nmi <n>` do; `node` and `node d` do nothing on `C` |
| The node argument is never read by the `C` hooks (handoffs 11.2) | re-read: confirmed in packages 0 and 2 for soft reset, NMI, hold and release |
| Type `6` has per-node reset, NMI and disable with its own hooks and an iterator (11.2, 11.4) | re-read: confirmed; reset and NMI per node, disable per node as a setting bit with no line driven |
| "reset node" in the help text belongs to `softreset <exp>` (11.1) | re-read: corrected. `soft reset node <exp>` belongs to `softreset <n>`; `reset node <exp>` is the help of the type `6` row `reset <n>`. The earlier note's table already listed `reset <n>` correctly. Edited in place |
| Soft-reset hooks `0xffe445e8`, `0xffe4463c`; type `6` hooks `0xffe50b68`, `0xffe50b88`, `0xffe63750`, `0xffe637d8` (11.2) | re-read: confirmed, all instructions |
| Iterator rules, error table (11.2) | re-read: confirmed for the rules; error text mapping not followed |
| "Tells the L2 nothing" (11.2) | re-read: confirmed for `softreset`, `nmi`; qualified for hold (section 5). Edited in place |
| `reset <n>` holds one node and does not release (11.3) | re-read: confirmed; adds that it bypasses the iterator and the disabled bit, and accepts index 4 |
| Per-CPU disable on `C` (11.4) | re-read: confirmed for `0xffe449e4`; the board-init loop `0xffe44b1e` and the guard's meaning of "all supplies off" were not re-read (not re-read, grade as earlier note) |
| `node d` guards, stores, hooks (11.4) | re-read: confirmed; adds the other readers of setting `0x70` |
| Power-up diagnostic flag bytes at `0x30000609` and following (11.4) | not re-read (grade stays as the earlier note's) |
| Supply-state routine returns "all on" (several notes) | partly re-read. `0xffe4c6ac` returns 1 when every monitored supply input matches its expected level, 0 when none does, -1 otherwise. That 1 means "powered on" is the earlier notes' reading and was not re-derived. `softreset` and `reset` need 1; `cpu d`, `node d` need 0 |
| `FLED_DISABLED`, `FLED_KILLED` strings (11.5) | not re-read |
| Type `@` hooks (11.6) | read this time: see section 7 |

## 7. Other findings

- **Type `@`, `8`, `0`, `V`, `Q`, `E` in package 2 have no descriptor initialiser reached by the dispatch ladders.** The ladder at `0xffea38f4` has cases for `7`, `%`, `6`, `9` and `A` only and the ladder at `0xffe4a404` for `C`, `R`, `&`, `I`, `N`, `P`, `X`, `Y`, `G`; every other character ends at `0xffe4a4ca`, which prints "The '%s' brick type is not supported in this image!" and "Defaulting to 'unknown' brick type" (strings `0xffe4a546`, `0xffe4a576`). This is inferred: the callers of `0xffea38f4` were not traced for a remapping of the character before the call (one caller, `0xffe75fb4`, passes the type byte that was just stored as setting 9). It would mean the hooks of type `@` the earlier note could not find do not exist in 1.44.0.
- The `node` and `reset <n>` rows are not specific to type `6`: they are registered for every type in package 2 and restricted only by the class check (section 2).

## 8. Open items and the static route for each

| # | Item | Why open | Route |
|---|---|---|---|
| U1 | How many Bedrock chips one type `C` control block drives, and which pin `BEDROCK CNTL 1` bits 6 and 7 reach | the image names the expander, not the chip | published C-brick hardware description for the pin map; in the IP35 PROM image, the paths that raise a soft reset or NMI |
| U2 | Pin reached by `PIMM n POWER` bits `0x40`, `0x80` and what a node does when one is set | names only | processor-module hardware description; the PROM's CPU-presence read |
| U3 | Whether a type `6` node count of 4 means four Bedrock chips; what index 4 of `reset <n>` reads | not in the image | node record array filler behind `0x30000618`; the dispatch from the identity routine to the node registration |
| U4 | Whether an attached L2 learns of a disabled node | no reader of setting `0x70` found on a path to the event builder | the L1 inventory and configuration reply builders for setting `0x70`, and the L2 side of [l2/config-request-reply.md](../../software/firmware/l2/config-request-reply.md) |
| U5 | Meaning of the kind-2 event on the receiver and what the console subchannel word holds after a hold | receiver not read | the kind-2 case of the peer event handler in each package |
| U6 | Effect of setting `0x70` on the scan-description builder `0xffe51678` | argument modes not decoded | decode its callers' mode values and the routine `0xffe9f1f0` |
| U7 | Which error code maps to which string in the type `6` error table | **resolved in section 9** | |
| U8 | Packages 1, 3 and 5 | not read | repeat sections 2 and 3 on their descriptor initialisers for type `4`, `5` and `O` |

## 9. Console texts, error words, `cpu d`, power-up and the L2 (re-read 2026-11-13, L1 1.44.0)

Same image and addresses as sections 0 to 4. Package 0 is type `C`; package 2 holds the `node` and `reset <n>` handlers. The handlers of `brick`, `cpu` and `router` were compared package 0 against package 2 with address operands masked: 537, 182 and 159 instructions, no difference, and every text quoted below exists in both images (E).

**Error words (E).** One routine turns an error code into text (package 0 `0xffe5c0e0`, package 2 `0xffe7c544`). The code's magnitude is split into a hundreds group, taken from `(|code| - 300) / 100`, and an index of `|code| mod 100`; the group picks a 12-byte record (module name, highest index, pointer to a string array) from the table at `0xffe5c84c` (package 0) or `0xffe7ccb0` (package 2), and the index picks the string. The result is `module:message`. (A second routine, package 2 `0xffe7c640`, returns the message alone.) Codes of magnitude 1 to 299 print as `<text> [errno <n>]`; codes outside 300 to 4400 print "Unknown Module ... message ...". Disassemblers that print `divs.l d3, d0` for these divides hide that the remainder lands in `d3`; the instruction word is `0x4c43 0x0803`.

| Code | Printed text, after `ERROR: ` | Raised by (package 2 unless noted) |
|---|---|---|
| `-4200` | `Node:node commands are not supported on this brick type` | `node` enable or disable and the iterator when the descriptor slot for the hook is null (`0xffe5e1c6`, `0xffe5e2b6`) |
| `-4201` | `Node:invalid node number` | node index at or above the node count (`0xffe5e140`, `0xffe5e1a4`, `0xffe5e294`) |
| `-4202` | `Node:power must be down to enable/disable NODEs` | `node d` / `node e` when the supply-state routine `0xffe6cb10` is nonzero (`0xffe5e214`) |
| `-4203` | `Node:node disabled` | iterator, one node, node present but its bit in setting `0x70` is set (`0xffe5e16c`) |
| `-4204` | `Node:node not present` | iterator, one node, node absent (`0xffe5e174`) |
| `-4205` | the string slot is empty (null pointer) | not returned by any routine read |
| `-4206` | `Node:write error` | not traced |
| `-2400`, `-2401`, `-2402` | `CPU:cpu commands are not supported on this brick type`, `CPU:invalid cpu number`, `CPU:power must be down to enable/disable CPUs` | `cpu e` / `cpu d`, section 9.3 |
| `-3300` | `IP5359:IP5359_UNINITIALIZED` (the group names the node ASIC family; `reset <n>`'s own routine prints it without the module name) | per-node hold when the node-board flag byte at `0x30000618` is 0 (`0xffe63ecc`) |

The `reset <n>` handler (`0xffe167a0`) prints the message alone: a failing hold shows `ERROR: <message>` with no `Node:` prefix.

**9.1 `node` (E).** The only accepted order is the number between the words: patterns are `node`, `node e`, `node d`, `node <number> e`, `node <number> d`. A pattern's words are matched one by one and text left over after the last word rejects the entry (parser `0xffe61f18` in package 0, same shape in package 2); a rejected entry only moves the parser to the next entry. So `node 2 d` parses and `node d 2` does not match any `node` row. In package 2 the table that holds these rows is registered before the large table (the descriptor setup that registers it, `0xffea399a`, runs from `0xffe105a0`; the large table is registered from the command task start-up at `0xffe10610`), so `node 2 d` reaches its own row first (E for the call order, I that the parser walks tables in registration order, as the earlier command-table note reads). `node d 2` then falls to the catch-all row `fru|pci|prom|node <command>` (its last pattern word takes the rest of the line), whose handler prints `command not supported on this brick type.` (I; the fall-through relies on that same ordering). On package 0 neither order parses to a node row. `node` with no word prints a header `NODE Present Enabled` and a rule, then one line per node of the form `%4d %7s %7s` with `Present` or `No Pop`, `Enabled` or `Disabled` (strings `0xffe191a8` to `0xffe191f9`). `node d` / `node e` with no number run the iterator over every present node; a success prints nothing. An out-of-range `reset <n>` prints `ERROR: invalid NODE index %d.` through a fixed test `n <= 4`, independent of the node count.

**9.2 `reset <n>` index 4 (I).** The handler accepts 0 to 4 although the node count is 4, then calls the per-node hold `0xffe63dec` directly. Index 4 therefore addresses a fifth 28-byte record of the node-record array behind the pointer at `0x3000061c` and the fifth byte of the node type table at `0x30003046`. The hold reads the record byte `+0x11` as a setting number, the record byte `+4` as an expander index and writes expander lines through it. What lies at those places is not decided by the code (the allocation of the array was not located), so the effect of index 4 is unknown; the scan-description builder in section 9.5 starts at `0x3000304a` and reads four bytes downward, so byte 4 is outside what the rest of the code treats as a node. A hold of any index returns `-3300` while the node-board flag byte is clear.

**9.3 `cpu d` on type `C` (package 0; E unless marked).** Handler `0xffe14d88`, rows `cpu`, `cpu e`, `cpu d`, `cpu <word> e`, `cpu <word> d`.
- Arguments: none (all CPUs) or one word. The word is a letter `a` to `d` (CPU 0 to 3), or a digit and a letter. A bad word prints `ERROR: invalid CPU (valid format is '<node>[a|b|c|d]', or 'a|b|c|d' for single-node brick types)`. The digit form is checked against a node count in RAM (`0x30001eb8`) that the type `C` descriptor never stores; so the digit form is probably refused on `C` (I).
- Refusal while powered: the supply-state routine `0xffe4c6ac` must return 0 (no supply on). Any other result, including its "partly on" result of -1, gives `ERROR: CPU:power must be down to enable/disable CPUs` (`-2402`).
- Action per CPU: bounds check against the CPU count at descriptor `+0x74` (4 on `C`), `-2401` if out; descriptor slot `+0x84` (the CPU disable hook `0xffe44ab8`, which calls `0xffe449e4`) or `-2400` if null; on success the setting `0x58 + CPU index` is set to 1 (`cpu e` writes 0 through `+0x80`). With no word the loop covers every present CPU and returns the first failure. Success prints nothing.
- Persistence: setting `0x58 + n` is a bit of the settings record in the battery-backed window ([../../software/firmware/l1/settings-record.md](../../software/firmware/l1/settings-record.md), setting 88 is byte `+0x14` bit 7), written by the same setter as every setting. So the choice survives an L1 restart. The board-initialisation hook of type `C` (descriptor slot `+4`, `0xffe453f8`) calls the routine at `0xffe44ae0`, which runs `0xffe449e4(index, enabled)` for each CPU with `enabled` = "setting `0x58 + n` is 0", so the stored choice is re-applied at start-up (E for the loop; I that the hook runs at every L1 start, because it is reached through a stored pointer).
- `cpu` alone prints the header `CPU Present Enabled` and one row per CPU (`%c%c    %d       %d`). With no CPU count it prints `command not supported on this brick type.`.

**9.4 "Node Disabled" (E for the site, I for the destination).** The text is at package 2 `0xffe60d74` and is used once, by the routine at `0xffe5e368` which the board-initialisation hook of brick type `7` (descriptor slot `+4` stored by `0xffe5f858`, hook `0xffe5ef60`, call at `0xffe5f142`) runs. It reads setting `0x70`; when any bit is set it prints `Node Disabled` once (not per node; the print routine `0xffe7ad90` adds a newline when the text has none) and sets three flag bytes at `0x30000609`, `+3` and `+1`. The routine is not reachable from the type `6` board initialiser (`0xffe50d18`, searched to depth 12), and the type `7` descriptor does not store the node enable or disable hooks, so on type `7` the console cannot set the bit. The line is printed at L1 start-up on type `7`, never by type `6`, and never by package 0 (the text is absent there). The print routine's destination was not followed; it is the same routine that prints the progress line `Checking for Type` just before, so it is a console progress line (I). This corrects the earlier reading that the power-up diagnostic reads setting `0x70` for type `6`.

**9.5 What a node disable does at power-up, and what the L2 sees (E unless marked).**
- Power-up skips disabled nodes in software, not by a line. The type `6` hold hook `0xffe50ad0` runs the per-node hold `0xffe63dec` through the iterator `0xffe5e0d4`, and the release hook `0xffe50b1c` runs `0xffe6405c` through the same iterator. The iterator calls only nodes whose bit in setting `0x70` is clear (`0xffe5e090`). `power up` reaches both hooks through the hold and release wrappers, so a disabled node gets neither the hold nor the release sequence. The disable itself (`node d`) drives no line (section 4). Both statements are true: a reading that a disabled node is "left out of power-up" is right about the iterator skip, and a reading that the disable "drives no line" is right about the disable command. What state the lines of a skipped node are left in is not in the code (U).
- Messages to the L2: the skip adds none. The brick-wide hold wrapper's kind 2 message (section 5) does not depend on the node set. The only node-dependent message is the margin event inside the per-node hold, sent when the node's type byte differs from the stored type; a skipped node never reaches that code, so a disabled node's type change is not announced (E for the reach: the hold reaches builder `0xffe38838` through `0xffe6e870`, the iterator's call does not). Nothing in the configuration reply carries setting `0x70`, as far as section 5 found.
- The scan-description builder `0xffe51678` (hook slot `+0xc0`, run by the release wrapper) gives each present node state 2, or 0 when the node's disable bit is set, in mode -3; modes -2 and -1 give 0 and 5. It tests `8 >> k` against the setting while it walks the type table from node 3 down to node 0, so read directly it pairs node 3 with bit 0, whereas the enabled test pairs node n with bit n. Whether that mirror is by design is unknown. The consumer, `0xffe9f1f0`, does not reach the event builder in 14 levels of direct calls (E for that search; indirect calls were not followed).
