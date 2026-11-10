# Challenge and Onyx (Everest) system controller: host side

The POWER CHALLENGE, CHALLENGE L and XL, POWER Onyx and Onyx systems (IP19, IP21 and IP25 CPU boards) have a system controller that sequences power, runs the blowers, watches the supply voltages and the air temperature, keeps an event log in battery-backed RAM and drives the front-panel LCD. No firmware for it is in hand. What the shipped host software shows is the wire protocol: the host sends short text commands over a UART that the CPU board exposes, and the controller answers in text.

These notes read that host software in the IRIX 6.5 Foundation 1 set (`root/6.5/`). Controller internals (CPU, firmware storage) stay unknown; see [../open-questions.md](../open-questions.md).

## Inputs

| Input | Path under `root/6.5/` | Size | SHA-256 (first 16) | What it is |
|---|---|---:|---|---|
| daemon | `usr/etc/sysctlrd` | 57,504 | `c92b4b43c67fc22e` | N32 MIPS-III executable, stripped, dynamic symbols kept; 47 named routines in the dynamic symbol table |
| driver, IP19 | `usr/cpu/sysgen/IP19boot/sysctlr.o` | 12,608 | `013a31704b9fef8a` | 64-bit MIPS-III relocatable with symbols and debug sections |
| driver, IP21 | `usr/cpu/sysgen/IP21boot/sysctlr.o` | 12,680 | `33735cc1a2d0cda6` | as above |
| driver, IP25 | `usr/cpu/sysgen/IP25boot/sysctlr.o` | 12,608 | `69fcdd19ae300f9d` | as above |
| message tool | `sbin/scmsg` | 9,596 | `92a124eee0e14ebf` | N32 executable, stripped; one function |
| header | `usr/include/sys/EVEREST/sysctlr.h` | 4,288 | `6d87557ec1f999d5` | shipped header that names the item codes and alarm letters |

The same files exist in `root/6.4-origin/` as different builds (daemon 42,352 bytes; drivers 12,688, 12,952 and 12,744 bytes; message tool 9,364 bytes). Those builds are listed in [../inventory/images.tsv](../inventory/images.tsv) and were compared only by string and constant checks here (see [daemon-wire.md](daemon-wire.md), version notes). The builds of IRIX 5.0 to 6.3 were decoded from the system discs (see [inst-compression.md](inst-compression.md)) and compared with these in [early-builds.md](early-builds.md).

## Notes

- [driver.md](driver.md): the character-device driver (`/dev/sysctlr`), its twelve routines, the state block, the device registers.
- [daemon-wire.md](daemon-wire.md): how the daemon frames requests and parses replies, the item catalogue, alarms, retries, power-down sequence.
- [records.md](records.md): the environment reply, the log reply, the PROM diagnostic table and board-type names.
- [inventory-record.md](inventory-record.md): the 1,768-byte inventory record of `syssgi` selector 102 and the routine that reports it.
- [scmsg.md](scmsg.md): the one-shot front-panel message tool.
- [prom-client.md](prom-client.md): the standalone IP19 diagnostic as a second client; the serial-number requests.
- [early-builds.md](early-builds.md): the daemon, message tool, driver and header of IRIX 5.0 to 6.4, what changed and when.
- [inst-compression.md](inst-compression.md): the old inst image layout and compression, and how to extract it.

## Grades

Entries follow [../../AGENTS.md](../../AGENTS.md). An item taken only from the shipped header or a man page and not seen in a binary is marked **hint**.
