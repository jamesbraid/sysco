# The 2 MB packages (2, 3, 5) against the Origin 300 package, routine by routine

Read from the IRIX 6.5.30 `l1.bin` (L1 1.44.0). Package numbers and labels follow [personalities.md](personalities.md): 2 `MIPS 2MB image`, 3 `Legacy 2MB image`, 5 `2MB image`; package 1 `Fuel/PE/O300 1MB image` is the reference, shortened to "o300". The routine pairing is [personality-diffs/routines.tsv](../personality-diffs/routines.tsv). The generic 1 MB package is in [personality-base-package.md](personality-base-package.md); how each package maps the brick type to board code is in [personality-board-init.md](personality-board-init.md). Addresses are first-bank body addresses. Grades follow [AGENTS.md](../../AGENTS.md).

## How the "changed" pairs split

Each changed pair was compared byte by byte, assigning every differing byte to a 32-bit word that holds a flash pointer in both packages or a RAM address in both.

| Package | Changed | Same length, only pointer and RAM words differ | Same length, other bytes differ | Different length |
|---:|---:|---:|---:|---:|
| 2 | 51 | 32 | 4 | 15 |
| 3 | 445 | 53 | 92 | 300 |
| 5 | 423 | 50 | 105 | 268 |

Established for the first column (the classification was computed for every pair of each package). Pairs in the first column are the same code instruction for instruction, moved. The "other bytes" column for package 2 is four pairs, all read below as range-boundary artifacts, not code differences. For packages 3 and 5 the same column was counted but not read pair by pair: a differing byte outside a pointer or RAM word is a different instruction or a range that starts in the middle of an instruction, and the two cases were not separated there. Inferred for packages 3 and 5 that most of those pairs are real code changes, from the size of the added and removed code below.

## Package 2: the generic 1 MB package's code plus the Origin 300 modules

Established from the pairing and the module literals. Package 2 contains every routine of package 0's board code (`IP35.c`, `IOlegacy.c`, `Kcar.c`, `NL3R.c`, `brick_base.c`) and the Origin 300 modules `Asterix.c`, `BIC.c`, `PE.c` and `eeprom_write.c`, but not `IP45.c`. Against package 1 it has 95 routines with no partner (35,296 bytes) and 583 added routines (about 540 KB).

Package 1 routines absent from package 2, by module literal: four `IP45.c` routines (812 bytes, `0xffe44ff4` to `0xffe458c4`: the Origin 300 node-board code and its bus-select hook), one `L1Req.c` routine (80 bytes at `0xffe11ce0`, the request task initialiser that package 2 replaces below), two `vrm.c` routines (64 bytes), one `uart_16654.c` routine, one `fan.c` routine and one `temp.c` routine. The rest (85 routines, 34,216 bytes) cite no source file literal.

Package 2 routines with no package 1 partner, by module literal, largest first: `jchip.c` 5 routines (2,998 bytes) and `jtag_cfg.c` 8 (2,018), `llpfun.c` 1 (1,736), together the JTAG scan layer; `IP5359.c` 8 (2,564); `pcix.c` 8 (2,536); `uart_1678x.c` 5 (1,528); `IP35.c` 6 (1,460); `eeprom_2m.c` 2 (880); `ChiServ.c` 1 (848), `ChiWS.c` 1 (116), `Twister.c` 1 (60); `brick_base.c` 1 (412, the second dispatcher read in the board-init note); `SysCom_2m.c` 2 (382); `IOlegacy.c` 2 (356); `PIC.c` 2 (332); `Kcar.c` 2 (284); `usb_isp1181.c` 1 (260); `vrm.c`, `pwrLED.c`, `NL3R.c`, `fan.c`, `uart_16654.c`, `autofan.c` and `temp.c` one each. 520 added routines (539,888 bytes) cite no literal.

### The fifteen pairs whose length differs

Established where marked, otherwise inferred from the strings and call lists the two routines carry.

| Package 2 (bytes) | Package 1 (bytes) | What differs | Grade |
|---|---|---|---|
| `0xffe14030` (200) | `0xffe11d30` (120) | The request task initialiser. Package 2 first tests a RAM flag and, when it is set, logs `L1Req already initialized!` and returns; it then creates a semaphore named `REQ_SEMA`, and only then registers the request endpoint and its callback, logging `can't register for requests (%d)` on failure. Package 1 registers the endpoint and callback with no flag and no semaphore. | established for the added guard and semaphore (instruction diff read); the removal of the o300 partner is the `L1Req.c` routine above |
| `0xffe198f0` (156) | `0xffe140d4` (224) | Package 1's routine calls the brick-type accessor, tests the first byte of its string argument against `0x7e` and then runs two cumulative add-and-branch tests on the type character; package 2's 156-byte routine has no accessor call. Both call a memory allocator and the allocator's free. Different routines paired by size. | inferred (call lists and the first dozen instructions read) |
| `0xffe3a430` (30), `0xffe3a44e` (54) | `0xffe34378` (54), `0xffe343c0` (84) | Small helpers that divide by 24 or 20 and compare a selector with fixed offsets in package 1; package 2's pair is shorter. Not the same code. | inferred (instruction shape only; what the helpers compute was not followed) |
| `0xffe4dd8a` (50) | `0xffe44508` (60) | Package 1 pushes `0xff` and calls the record loader `0xffe468a0`; package 2's bytes decode as a data word and a fragment, so its range does not start at a routine. A pairing artifact. | inferred (the package 2 bytes were not placed in a routine) |
| `0xffe4e86c` (264) | `0xffe46222` (234) | Package 2's range begins at the routine's `link` instruction; package 1's pair begins 30 bytes later in the same routine (its first word is data). The body is the same loop (read one expander status byte per entry; log `Error reading XBridge control IOExp status: %s` or `Error reading Port control IOExp status: %s`, source literal `XBridge.c`; disable the expander). The remaining differences are string pointers. | established |
| `0xffe4ea08` (16) | `0xffe45130` (16) | Package 1 calls `0xffe4709c` and returns 0; package 2 returns the byte at the RAM address `0x300005b4` (package 0's `0xffe47e70` does the same with `0x300000a4`; see the base-package note). Different routines paired by size. | established |
| `0xffe67054` (36) | `0xffe48370` (28) | Same call (one routine, then return); package 2's range includes the following `unlk` and `rts` of the same routine, which package 1's range ends before. Boundary artifact. | established |
| `0xffe7e9f4` (104) | `0xffe5d990` (192) | Both log `Error reading BIC %d present status: %s`. Package 1's 192-byte routine stores the record pointer and a count into a RAM block and reads the expander per entry; package 2's range starts after those stores. The pairing split one routine. | inferred |
| `0xffe850f8` (232) | `0xffe636f0` (244) | The UART register self-test (`UART invalid SPR register: 0x%02x, exp: 0x%02x, addr: 0x%08x`, `Register LISR error: %d`, source literal `uart_16654.c`). The two ranges start at different points of the same routine (package 1's begins with extra instructions that package 2's range lacks); string pointers differ. | inferred (literals and call shape matched; the range starts were not aligned) |
| `0xffe9058c` (152) | `0xffe6d3e4` (148) | Same calls; package 2's range includes the `link` instruction. Boundary artifact. | established |
| `0xffea0c70` (236) | `0xffe76ce4` (138) | A formatted-text routine that allocates a buffer, formats into it and appends. Package 2 adds a block after the first format: a ten-argument call of `0xffe74324` taking a word at +0x34 of the record in `a5`, then, on failure, a text append, and otherwise two more words formatted into the output. | established for the added block as an instruction diff; the meaning of the arguments is unknown |
| `0xffea0f5c` (916) | `0xffe76fd0` (528) | The scan-port text builder. Both allocate buffers, format, read the scan port and append. Package 2's is 388 bytes larger and has more calls into text helpers (`0xffea3fa4` five times against four in package 1). | inferred (call lists only) |
| `0xffea7438` (288) | `0xffe7d52c` (284) | Same code; package 2 has four extra zero bytes before the pad word at `0xffea7552`. | established |
| `0xffea8040` (90) | `0xffe7e130` (108) | A routine that tests a word against `0x80000000`; package 1's range includes the end of the previous routine (`movem.l` restoring `d0` and `d1`). Boundary artifact. | inferred |
| `0xffea9828` (1,216) | `0xffe7f344` (1,176) | The JTAG scan routine. The error text is `scan error - brick type does not support JTAG sequence` in package 2 and `scan error - chip not found in current scan configuration` in package 1; the body is otherwise of the same size class. | inferred (strings and size only) |

### Four same-length pairs with other bytes

Established (instruction diffs read): `0xffe76ace` (14 bytes), `0xffe93644` (60) and `0xffe94f60` (48) begin in the middle of an instruction in one package, so their first word differs and the rest is string pointers only (`0xffe94f60` also differs in two more instructions after its first word, which were not followed). `0xffe4ea08` is the real difference above.

## Packages 3 and 5: a different board family

Established from the module literals each routine references. Packages 3 and 5 share their board code (`IP41.c`, `IP57.c`, `IObrick.c`, `Opus.c`, `SHUB.c`, `promIce.c`, `L1FRU_2m.c`) and the JTAG and PCI-X layers with package 2, and drop everything node-board specific to the Origin 300.

Package 1 routines with no package 3 partner: 961 (about 143 KB with no literal, plus `power.c` 15 routines 7,828 bytes, `Asterix.c` 8 routines 3,032, `fan.c` 10 routines 2,412, `L1USB.c` 4 routines 2,140, `L1Cfg_1.c` 1 routine 1,580, `Env.c` 1 routine 1,316, `SysCom.c` 3 routines 1,252, `monitor.c` 4 routines 984, `test.c` 944, `BIC.c` 3 routines 836, `sn1sc.c` 2 routines 828, `IP45.c` 4 routines 812, `ioExp.c` 3 routines 784 and smaller). Against package 5: 1,098 (`L1Req.c` 3 routines 8,598 bytes, `power.c` 7,828, `Env.c` 5 routines 4,444, `port.c` 13 routines 3,136, `Asterix.c` 3,032, `L1Cfg_1.c` 2 routines 2,622, `fan.c` 2,412, `B2B_1.c` 1,952, `monitor.c` 8 routines 1,640 and others).

Package 3 routines with no package 1 partner: 1,634 (634,522 bytes with no literal; with literals, `SHUB.c` 20 routines 12,712 bytes, `power.c` 9 routines 6,166, `jchip.c` 3,962, `IP41.c` 8 routines 3,704, `pcix.c` 3,140, `fan.c` 2,636, `IP57.c` 6 routines 2,544, `L1USB.c` 4 routines 2,140, `Opus.c` 6 routines 2,032, `jtag_cfg.c` 1,976, `llpfun.c` 1,960, `eeprom.c` 1,684, `uart_1678x.c` 1,528, `L1Cfg_1.c` 1,258, `monitor.c`, `SysCom.c`, `ioExp.c`, `test.c`, `vrm.c`, `display.c`, `vsc055.c`, `pwrLED.c`, `Env.c`, `i2cMux.c`, `promIce.c`, `PBayITask.c`, `SMP.c`, `SysCom_2m.c`).

Several modules the packages share with package 1 (`power.c`, `fan.c`, `monitor.c`, `Env.c`, `L1Cfg_1.c`, `SysCom.c`, `test.c`, `L1USB.c`) appear on both the removed and the added side. Established as a fact about the pairing; inferred that those are routines whose body changed enough for the matcher to drop the pair, not routines new to the package, because the module literal is present on both sides.

Request codes the two packages add are in [personalities.md](personalities.md). The personality selection of both is the shorter record list and the four-character dispatcher in [personality-board-init.md](personality-board-init.md).

## Where each personality is chosen, in one place

Established (see [personalities.md](personalities.md) and [personality-board-init.md](personality-board-init.md) for the reads): no strap, EEPROM jumper or flash record is read to pick board code at run time. The type character is derived from the board's identification EEPROM records (`0xff`, `0xfe`, `0xfd`, `0xfc`), with a persistent setting (key 9) as the fallback, and a dispatcher then calls the initialiser for that character. The flash record only says which packages the updater may offer for a brick type. Each package contains only the board code its type list needs.
