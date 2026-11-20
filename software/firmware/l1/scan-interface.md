# L1 1.44.0 C-brick: the JTAG scan interface on the parallel port, and what a release expects back

What the C-brick L1 does with `PPDAT`/`PPDDR` (the MCF5206e parallel port) when it resets and releases the node, and what the scan chain behind the port must answer for the release not to fail with `SCAN:scan error - unable to reset scan hardware`. Read from L1 1.44.0, package 0 (`Base 1MB`, C-brick) of the IRIX 6.5.30 `l1.bin` (sha256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`), first bank, through a linear listing. Grades: E established (routine read in full), S published manual, I inferred, U unknown. The pulse and the early-exit rules are also in [../../../platforms/bricks/l1-bedrock-handoffs.md](../../../platforms/bricks/l1-bedrock-handoffs.md) section 2.

## 1. The pins (E for the code, S for the register)

`MBAR+0x1c5` (`PPDDR`) is written `0xfe` and `PAR` (`MBAR+0xca`) bits 5:4 are cleared at start-up (`0xffe71ee0`), so parallel-port pins 7:1 are outputs and pin 0 is an input, all as general-purpose I/O (MCF5206E user's manual rev. 1, chapter 10: `PPDDR` bit 1 = output, reset 0; `PPDAT` reads the pin level for inputs and the driven level for outputs, reset 0; `PAR` bits 5:4 clear select the parallel-port function). The firmware's own print routine (`0xffe723a4`, text `TAP values: trst= psi= tck= tms= tdi= tdo=`) names the bits:

| `PPDAT` bit | Mask | Signal | Direction | Active level |
|---:|---|---|---|---|
| 0 | `0x01` | TDO | input | read; sampled as 1 or 0 |
| 3 | `0x08` | PSI | output | high in the reset pulse and while scanning |
| 4 | `0x10` | TRST (TRSTN) | output | **active low**: the pulse's `0x28` clears it for one write and the manual's `scan reset hard` is "via TRSTN" (S for the name, I that the pin is the TAP's TRSTN) |
| 5 | `0x20` | TMS | output | high drives the TAP towards Test-Logic-Reset |
| 6 | `0x40` | TCK | output | the shift routines raise it after setting TDI and TMS and lower it again |
| 7 | `0x80` | TDI | output | data, most significant bit of the word first |

Bits 1 and 2 are outputs the code never changes (they keep what `0x31` and the later values give them, 0). The names come from the firmware's print format and from the L1 and L2 controller guide's `scan` commands (007-3938-004, printed pp. 86 to 88: `scan reset hard` "via TRSTN", `scan reset soft` "via TMS", `scan set trst/tck/tms/tdi/psi 0|1`, `scan get`, `scan ids`, `scan sel <addr> <CER> [<MR> [<IOR>]]` "Select SIC", `scan count` "IR and BYPASS registers"). That PSI is a SIC-select line is inferred. The board nets are U.

## 2. The reset pulse (E)

`0xffe7219c`, run by the hold wrapper (`0xffe4f2f0`, every `reset hold`, power-down and power-up hold) and twice per release (below), does nothing when the word at `0x30000044` is zero (it is 1 after start-up). It reads `PPDAT` once, then writes: read value OR `0x08`, that OR `0x18`, `0x38`, `0x28`, `0x38`, `0x30`. It never reads the result back and has no timeout or retry. Start-up (`0xffe71ee0`) writes `0x31`, `PPDDR` `0xfe`, then `0x38`, `0x28`, `0x38`, `0x30` once; the evidence sample `0x08, 0x18, 0x38, 0x28, 0x38, 0x30` is this pulse.

## 3. The release scan (E for the order, I for the reading of the instruction values)

The release wrapper (`0xffe4f144`) calls the reset-brick routine `0xffe72214(type, &error)` when the software watchdog is armed (`SYPCR` bit 7, armed at start-up). For type `C` the routine: returns -2300 if `0x30000044` is zero; runs the pulse; finds no hook (`+0xc0` is not installed in package 0); dispatches through the table at `0xffe722a8` to one 24-bit scan, `0xffe74474(0x18, 0, 7, ...)`; returns -2301 if the scan returns 0; runs the pulse again; returns success. Other brick types skip the scan (`4` among them); `%`, `A`, `C`, `G`, `I`, `N`, `P`, `R`, `X`, `Y` run it with their own constants. There is no loop around the scan, no retry and no timeout: one failed check ends it.

Two driver routines do all the pin work (registered at `0x30000060`/`0x30000064`): `0xffe725a4` shifts with TMS and TDI per bit and `0xffe72448` shifts TDI with TMS held. Per bit, both write the byte with TCK low, **read `PPDAT` and take bit 0 as the TDO sample**, write it with TCK high, then write it with TCK low again; bits go out and come in most significant first, 32 per word. A TAP-reset call (`0xffe76008`) is 32 such clocks with TMS high; the Shift-IR and Shift-DR entries are fixed TMS strings (`0xffe76308`: from Test-Logic-Reset `0,1,1,0,0` for IR; from Run-Test/Idle `1,1,0,0` for IR and `1,0,0` for DR), so the model needs the standard IEEE 1149.1 state machine.

The scan `0xffe74474` for `C`, in order, with the base byte in brackets (`0x38` = TRST, TMS, PSI; `0x18` = TRST, PSI):

1. TAP reset, 32 clocks [`0x38`]. A failing driver call would end here; the drivers always report success.
2. IR scan, 12 bits, value 4, then DR scan, 8 bits, value 0 (the type's SIC address argument): the "SIC selection sequence". The captured data is masked and not compared. I for the meaning.
3. IR scan, 12 bits, all ones (BYPASS). **The first 12 TDO samples must be `1` followed by eleven `0`s** (compared as `0x80000000` after masking to the top 12 bits). On mismatch the diagnostic "Unexpected data shifted out of IR: 0x%08lX." goes to the error buffer and the scan fails.
4. DR scan, 32 bits of `0xa800ffea`. **The 32 samples must be that word delayed by one bit, `0x54007ff5`** (first sample 0), else "Unexpected data shifted out of BYPASS register." and the scan fails.
5. A second 12-bit IR scan (value 8) with the same capture check as step 3, then an 8-bit DR scan whose capture is masked away (type `C` passes mask 0, so nothing is compared).
6. TAP reset [`0x20`: TMS high, TRST low, PSI low] then TAP reset [`0x30`: TMS high, TRST high, PSI low]; success (return 1). The first of these asserts TRSTN for its 32 clocks.

So the minimum chain that passes is one TAP whose instruction register captures `...01` (LSB first `1, 0, 0, ...`) over at least 12 bits and whose bypass register is one bit long, answering with correct TCK-edge behaviour. The IR length of that device is not tested beyond 12 captured bits (a device with a longer IR also passes step 3; 12 is the shifted width, I for a 12-bit IR). Reads of `PPDAT` that return 0, as an unmodelled register does, fail step 3 on the first sample. E for the compares; the mapping of -2301 to the console text is I: the SCAN error class table holds "unable to reset scan hardware" as its entry for -2301, and the release returns -2301 when the scan returns 0.

IDCODE reads (`scan ids`) and the length probes ("Unable to calculate IR chain length", "SIC count of %d is invalid") belong to scan variants and console commands that the type `C` release does not select (flags `0x18` set only the end-of-scan TAP resets). U for what the real chain's IDCODEs are.

## 4. What power-up does with the port

`power up` holds the node (pulse), walks the supplies, then `reset(5)` (hold hook, 10 ticks, release): each release is pulse, scan, pulse as above, and the reset runs the hold-and-release pair twice when the predicate of [power-reset-cbrick.md](power-reset-cbrick.md) asks for the second reset. After a successful release nothing else touches `PPDAT` during power-up; the node is left running. A model needs: a TAP behind `PPDAT` for the whole of step 3, and nothing afterwards. E for the order.

## 5. Does a documented configuration avoid the scan error?

No. The release runs the scan on a C-brick whenever `SYPCR` bit 7 is set (the watchdog is armed at every start-up) and the type is `C`. The L1 guide documents the `scan` family as a diagnostic on every brick and describes no setting that disables it. The scan is skipped for brick types outside the table, and when the watchdog is not armed, but neither is a configuration the manuals describe. The legitimate way through is a TAP that answers as above. E for the code paths, S for the absence of a documented option.

## 6. Sequence details for an implementer (E; the pin names are as in section 1)

**TMS strings.** The TAP state machine is the standard IEEE 1149.1 one. After a TAP reset (32 clocks, TMS high) the TAP is in Test-Logic-Reset. Entry strings, one TMS bit per TCK, most significant bit of the constant first (`0xffe76308`):

| Entry | From | TMS bits | Lands in |
|---|---|---|---|
| IR | Test-Logic-Reset | `0 1 1 0 0` (5 clocks) | Shift-IR |
| IR | Run-Test/Idle | `1 1 0 0` (4 clocks) | Shift-IR |
| DR | Run-Test/Idle | `1 0 0` (3 clocks) | Shift-DR |

The SIC-select IR scan enters from Test-Logic-Reset; every later IR and DR scan enters from Run-Test/Idle. During the data clocks TMS is low except on the **last data bit**, where it is high (Shift to Exit1). Two more clocks follow: TMS `1` (Exit1 to Update) and TMS `0` (Update to Run-Test/Idle). All scans of the C-brick release pass the exit argument 1, so **every scan ends in Run-Test/Idle by way of Update**; the Pause ending (TMS `0 0` after Exit1) exists in the routine for other callers and is not used here. Clock counts after the entry string:

| Scan | Data clocks | TMS during data | Then |
|---|---:|---|---|
| 12-bit IR | 12 | 0 on the first 11, 1 on the 12th | 2 clocks: 1, 0 |
| 8-bit DR | 8 | 0 on the first 7, 1 on the 8th | 2 clocks: 1, 0 |
| 32-bit DR | 32 | 0 on the first 31, 1 on the 32nd | 2 clocks: 1, 0 |

The routine splits a scan into whole 32-bit words shifted with TMS low through the bulk driver (`0xffe72448`) and a last word shifted with the TMS vector through the TMS-and-TDI driver (`0xffe725a4`); none of these three scans has a bulk part. TDI data go out most significant bit of a 32-bit word first, and the firmware bit-reverses its values first, so an instruction or data value goes out least significant bit first. The TAP resets are `0xffe76008` through the bulk driver: 32 clocks with TMS high and TDI 0, three stores per clock (below), at base `0x38` (first), `0x20` (second to last) and `0x30` (last).

**Order inside one clock (both drivers).** Instruction order, per bit: (1) compute the byte (base bits, plus TDI in bit 7, plus TMS in bit 5 for the TMS-and-TDI driver) with TCK low and **store it to `PPDAT`**; (2) **load `PPDAT`** and keep bit 0 as the TDO sample; (3) OR in TCK and **store** (rising edge); (4) a few register instructions that place the sample in the result word; (5) toggle TCK off and **store** (falling edge). So the order is write with TCK low, read, raise TCK, lower TCK, and **TDO is read after TDI and TMS are set up and before the rising edge**, i.e. TDO must already show the value shifted out at the previous falling edge. The three stores and the load are back to back: no delay routine and no other `PPDAT` access between them, only the register instructions named (the loop also calls the helper at `0xffe412ee` once per driver call, before the first bit, which is not a `PPDAT` access). The TMS-and-TDI driver, on entry, loads `PPDAT` and stores it back with bit 3 (PSI) set or cleared to match the flags when it differs, so a call may add one load and one store before the first clock.

**The six pulse writes** (section 2), with the byte after each, assuming `PPDAT` reads `0x30` first (the last value any earlier scan or the start-up pulse leaves). Bit 0 and bits 2:1 of the first two writes come from the value read:

| # | Write | PSI | TRST | TMS | TCK | TDI |
|---:|---|---|---|---|---|---|
| 1 | read OR `0x08` (`0x38`) | 1 | 1 (not asserted) | 1 | 0 | 0 |
| 2 | that OR `0x18` (`0x38`) | 1 | 1 | 1 | 0 | 0 |
| 3 | `0x38` | 1 | 1 | 1 | 0 | 0 |
| 4 | `0x28` | 1 | **0 (asserted)** | 1 | 0 | 0 |
| 5 | `0x38` | 1 | 1 | 1 | 0 | 0 |
| 6 | `0x30` | 0 | 1 | 1 | 0 | 0 |

TCK stays low throughout the pulse. If the value read had other bits set, writes 1 and 2 carry them.

**`PPDDR` before or after `PPDAT`.** At start-up (`0xffe71ee0`) the firmware writes `PPDAT` `0x31` first and `PPDDR` `0xfe` after it, then the four pulse values. With `PPDDR` still 0 (reset value) the `0x31` write only latches, and per the MCF5206E manual (chapter 10, the `PPDAT` note) the latched value is what the pin drives when it becomes an output: TMS and TRST high, bit 0 high but bit 0 stays an input. Afterwards the firmware does write bit 0: the pulse routine and the drivers read `PPDAT` and store the read value (bit 0 = the TDO sample) back with other bits changed. Bit 0 is an input (`PPDDR` bit 0 is 0), so those writes have no effect, as the manual says; a model must not let them change the TDO sample.

**Failures.** Every check in the C-brick scan is fatal and they all end the same way. A failed TAP reset call, the first IR compare (`0xffe7494c`), the BYPASS pattern compare (`0xffe74988`) and the second IR compare (`0xffe74b9e`) each write a diagnostic into the error buffer (`Unexpected data shifted out of IR: 0x%08lX.`, `Unexpected data shifted out of BYPASS register.`, `Unable to drive TAP interface for scan operation`), free the scan buffers and return 0 from `0xffe74474`; there is no other exit for failure and no retry. The release routine turns that 0 into -2301 (class SCAN, the console text "unable to reset scan hardware") and returns failure to the power-up path, whatever compare failed. The drivers never report failure, so only data compares can fail, and a TAP that is simply missing (TDO reads 0 or 1 constantly) fails at the first IR compare whether TDO reads constant 0 or constant 1.
