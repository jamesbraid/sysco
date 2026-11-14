# Origin 300 L1 1.44.0: the node hold and release sequence, and what "reset pulse A" and "reset line B" are

What the L1 writes to its I/O expanders, in order, when the console or an automatic path holds the Origin 300 node in reset and releases it, and which routine is the soft reset and which the NMI. Brick letter `4`, board source literal `IP45.c`.

Read from L1 1.44.0, package 1 (`Fuel/PE/O300 1MB image`, body sha256 `9db44f3bb06ec90475035781766e1ca7e924f15a0ed215cf855c1f491275fdd1`, loaded at `0xffe00000`), first program copy, in the committed listing ([disasm/mcf5206e-1.44.0-fuel-pe-o300/l1.lst](disasm/mcf5206e-1.44.0-fuel-pe-o300/l1.lst)). It repeats the routine-level reading in [../../../platforms/bricks/l1-bedrock-handoffs.md](../../../platforms/bricks/l1-bedrock-handoffs.md) (section 4) after a second read of every routine named here, and adds the byte values, the wire addresses, the power-up ordering and the answer to the naming question. Expander indexes, wire addresses and input masks are those of [ip45-io-expanders.md](ip45-io-expanders.md).

Grades follow [../../../AGENTS.md](../../../AGENTS.md): **established** (the whole routine was read and the claim follows), **inferred** (reasoning stated), **unknown**.

## 1. "Reset pulse A" is the soft-reset hook and "reset line B" is the NMI hook (established)

The board's operations record (`0x30001f20`) is filled by the initialiser `0xffe45bac`. It stores five reset-related routines, and nothing else in the image stores or calls any of them:

| Slot | Routine | Role in the record | Bus sheet name |
|---|---|---|---|
| `+0x98` | `0xffe453e4` | hold | node enable steps |
| `+0x9c` | `0xffe4552c` | release | node enable steps, strap and bridge writes |
| `+0xa0` | `0xffe45060` | "a second reset is needed" test | not named |
| `+0xa4` | `0xffe45084` | **soft reset** | **reset pulse A** |
| `+0xa8` | `0xffe450e0` | **NMI** | **reset line B** |

The two addresses `0xffe45084` and `0xffe450e0` occur in the whole body only as the immediates at `0xffe45c4e` and `0xffe45c58`, which store them into slots `+0xa4` and `+0xa8`. The slots are read by exactly two routines: the soft-reset wrapper `0xffe4ed3c` (`tst.l $a4(a5)` at `0xffe4ed70`, indirect call at `0xffe4ed80`) and the NMI wrapper `0xffe4ecf4` (`$a8(a5)`, `0xffe4ed1a` to `0xffe4ed2c`). So reset pulse A and reset line B are, byte for byte, the Origin 300's soft reset and NMI.

The wrappers' callers:

| Wrapper | Callers | Reaches it |
|---|---|---|
| soft reset `0xffe4ed3c` | `0xffe1bf3a` (the `softreset` handler `0xffe1bea0`) | console `softreset` and `softreset <n>`; the type set of the handler includes `4` (established, handoffs section 6) |
| NMI `0xffe4ecf4` | `0xffe1be04` (the `nmi` handler), `0xffe48e44`, `0xffe51ac4` | the console `nmi` handler refuses type `4` (its type sets are `c12!`, `C1256@`, `C125@`); the front-panel NMI button path at `0xffe51ac4` is the one that applies; the trigger of `0xffe48e44` was not followed (unknown) |

Both hooks begin with the guard that returns `-2500` while the board byte `0x3000009c` holds `0xff`. Hook bodies:

- **Soft reset `0xffe45084`.** Set `ASIC CTRL 1` (index 2) mask `0x80`; wait 10 ticks; clear mask `0x80`. Established. Bytes from the start-up shadow `0x72`: `0xf2`, then `0x72`.
- **NMI `0xffe450e0`.** Clear `ASIC CTRL 1` mask `0x40`; set mask `0x40`; no delay between the two writes. Established. Bytes: `0x32`, then `0x72`.

`ASIC CTRL 1` is the expander on bus segment 3 at wire address `0x42`, the same address and the same applied byte `0x72` as the C-brick's `BEDROCK CNTL 1`, and the hook bodies are identical to the C-brick's apart from the guard value. That the pulse on bit 7 is a soft reset and the clear-then-set on bit 6 is an NMI is therefore a statement about the firmware's own names for the hooks (established: they are the soft-reset and NMI slots of the operations record). What the bits do at the Bedrock chip is not in the image: the polarity at the chip, and whether the two lines are separate chip inputs, are unknown. The service manual names the NMI signal as coming from "I/O expander B_0100001x" (Origin 300 Server Owner's Guide 007-4498-002, printed page 194), which is wire address `0x42`; that agrees with the address and is the only independent statement.

## 2. Hold: what it writes, in order (established)

Hold hook `0xffe453e4`, called from the hold wrapper `0xffe4eeec` (no delay of its own) and from `0xffe4ef40` (the `reset hold` command, power down, power up), stops at the first expander error and returns it:

| # | Write | Expander (index, segment, wire) | Bytes from the start-up shadow |
|---:|---|---|---|
| 1 | `XBRIDGE CONTROL` mask `0x0b` := `0` | index 10, segment 5, `0x42` | `0xd4` (no bit of `0x0b` was set) |
| 2 | `ASIC CTRL 1` clear `0x80` | index 2, segment 3, `0x42` | `0x72` |
| 3 | `ASIC CTRL 1` clear `0x02` | same | `0x70` |
| 4 | `ASIC CTRL 2` clear `0x01` | index 3, segment 3, `0x44` | `0x8a` |
| 5 | `ASIC CTRL 1` clear `0x01` | index 2 | `0x70` |
| 6 | helper `0xffe45348(4, 8)`: when `POWER CONTROL` (index 4) is present: for processor-module PSC codes 1 to 4 only, a margin step `0xffe4e148(8, 14)` and a 10-tick wait; then `POWER CONTROL` set `0x10`, then set `0x20` | index 4, segment 4, `0x46` | `0xff`, `0xff` |

Step 1 selects segment 5 of the bus switch first (`0x07` to the switch at `0xee`, see ip45-io-expanders.md). The expander in step 1 is named by the byte at `0x3000009c`, which `IP45_Init` (`0xffe44ff4`, called with `0x0a` at `0xffe45ab2`) sets to 10. The order of the four `ASIC` clears is that of the C-brick hold. The start-up shadows `0x72` and `0x8a` already have bit 0 clear in both `ASIC` bytes, so the cold-start state is a held state; the hold only adds the clear of `ASIC CTRL 1` bit 1.

What "held" means at the pins is an inference: the group is the three bits `ASIC CTRL 1` bit 0, `ASIC CTRL 1` bit 1 and `ASIC CTRL 2` bit 0 (every hold clears them, every release sets them last and in that order). Held is "not all three set". Bit 7 of `ASIC CTRL 1` (the soft-reset line) is cleared by the hold and never set by the release.

## 3. Release: what it writes, in order (established)

Release hook `0xffe4552c`, called from the release wrapper `0xffe4ed94`:

| # | Step | Detail |
|---:|---|---|
| a | the predicate `0xffe4c2fc()` (a test over the supply records, not read in full: unknown) picks the path | when it returns zero the strap rule and step b are skipped (branch `0xffe45552` to `0xffe45604`) and the working values `d7` and `d6` stay 0 |
| b | strap rule, then 10 ticks, `XBRIDGE CONTROL` mask `0x0b` := `0x0a`, 10 ticks, mask `0x0b` := `0x0b` | only when the predicate is nonzero. The strap rule sets `d7` = `0x80` and `d6` = 0 when the global word `0x3000015c` is set and setting `0x2a` is nonzero, or when it is clear and module record 1 is valid with a nonzero `+0x30` word; otherwise (valid record, zero word) `d7` = 0 and `d6` = 4; with no valid record settings `0x2a` and `0x2b` are cleared first and `d7` = 0, `d6` = 4 |
| c | `d7 |= 0x08`; `|= 0x20` when setting `0x28` is nonzero; `|= 0x10` when setting `0x29` is nonzero | |
| d | `ASIC CTRL 2` mask `0xf8` := `d7` | wire `0x44` |
| e | `XBRIDGE CONTROL` mask `0x24` := `d6`; 10 ticks | wire `0x42`, segment 5 |
| f | `ASIC CTRL 2` clear `0x08`; 10 ticks | |
| g | helper `0xffe45488(4, 8)`: `POWER CONTROL` clear `0x10`, clear `0x20`, 3 ticks; for PSC codes 1 to 4 the margin step `0xffe4e0a0(8)` and 10 ticks | wire `0x46`, segment 4 |
| h | `ASIC CTRL 1` set `0x01`; 10 ticks | |
| i | `ASIC CTRL 2` set `0x01` | |
| j | `ASIC CTRL 1` set `0x02` | |

Steps c to j always run. Step e writes its mask `0x24` whatever the predicate gave.

Byte values from a hold state (`ASIC CTRL 1` `0x70`, `ASIC CTRL 2` `0x8a`, `XBRIDGE CONTROL` `0xd4`, `POWER CONTROL` `0xff`) with no stored strap settings and the predicate false: `ASIC CTRL 2` `0x0a`; `XBRIDGE CONTROL` `0xd0`; `ASIC CTRL 2` `0x02`; `POWER CONTROL` `0xef`, then `0xcf`; `ASIC CTRL 1` `0x71`; `ASIC CTRL 2` `0x03`; `ASIC CTRL 1` `0x73`. The final pair `0x73` and `0x03` is the C-brick's released state. With the predicate true, two more `XBRIDGE CONTROL` writes (`0xde`, then `0xdf`) come first and step e writes `0xd4` when `d6` is 4.

Delays are counts of firmware ticks passed to `0xffe40e54`. A tick is one count of the timer-1 interrupt; its length is the 100 Hz inferred in [board-boot-contract.md](board-boot-contract.md) (inferred), so 10 ticks is 0.1 s. The release spends 10 + 10 + 3 + 10 = 33 ticks with the predicate false and a processor-module code outside 1 to 4, 43 ticks with a code of 1 to 4, and 20 ticks more in either case when the predicate is true. The hold spends 10 ticks only for codes 1 to 4.

## 4. Which commands run which sequence (established)

| Command or path | Sequence | Evidence |
|---|---|---|
| `reset` | hold, 10 ticks, release; a second pass after 400 ticks only inside `power up` and only for PSC codes 2 to 4 (`0xffe45060`) | reset wrapper `0xffe4ef74` |
| `reset hold` | hold only, preceded by six writes to the on-chip parallel-port data register (`0xffe7204c`), when the scan driver is initialised | `0xffe198f4` calls `0xffe4ef40(1)` |
| `power up` | log; **hold (`0xffe4c908` calls `0xffe4ef40(5)`)**; the supply walk; set the "just powered" word; then, because the routine's argument is nonzero, 50 ticks (the jump-table arm of type `4` is the one with the wait) and `reset(5)` at `0xffe4cdae` | `0xffe4c554` to `0xffe4ce84` |
| `power up hold` | the same up to and including the supply walk and the "just powered" word; the argument is zero, so the test at `0xffe4cd3c` skips the 50-tick wait and the `reset(5)` | the node is left held with its supplies on |
| `power down` | hold (`0xffe4c662` calls `0xffe4ef40(5)`), then the supplies off | `0xffe4c554` neighbours |

The argument of the power-up routine is nonzero for `power up` and zero for `power up hold`: that follows from the test `0xffe4cd3c` and from the tail at `0xffe4ce3c` (types `#`, `=` and `A` only) that runs the extra hold exactly when the argument is zero; the console parser that fills the argument was not re-read (inferred). The ordering, as the firmware drives it, is therefore: the group is held before the node supplies are enabled, and `power up` releases it 50 ticks (0.5 s at the inferred tick rate) after the supply walk.

## Not settled

| Item | Why open |
|---|---|
| Polarity and destination of `ASIC CTRL 1` bits 7 and 6, and of the group bits | the image names expanders, not chip pins; the polarity at the Bedrock input is not in any image |
| The predicate `0xffe4c2fc` and the margin routines `0xffe4e148`, `0xffe4e0a0` | not read through; the margin step is inferred to move the supply record 8 margin to 14 for hold and to restore it at release |
| The trigger of the NMI call at `0xffe48e44` | the enclosing routine's caller was not followed |
| Wall-clock length of a tick | the processor clock is not measured |
