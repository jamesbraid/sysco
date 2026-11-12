# Published sources

Public documents the notes cite. A hint from here is not a finding until the same fact is read in an image (AGENTS.md).

| Source | Used for |
|---|---|
| SGI, *L1 and L2 Controller Software User's Guide*, document 007-3938 (revisions 002 to 006) | controller commands, debug switches, supported products by revision |
| SGI, *Origin 300 User's Guide*, 007-4365, and *Origin 300 System with NUMAlink Module User's Guide*, 007-4498 | I/O-8 and IP45 component callouts, connectors, debug command examples |
| SGI, *Altix 330 System User's Guide*, 007-4790 | the Altix-family L1 controller (a later controller generation) |
| Freescale/NXP, *MCF5206e ColdFire Integrated Microprocessor User's Manual*, revision 1, and errata revision 3 | MBAR register map, M-Bus registers, RAMBAR and MBAR control registers |
| USB 1.0 specification (1996) | descriptor and standard-request meanings |
| Agere, *USS-820D USB Device Controller* data sheet, revision 4 (June 2001) | front-matter facts about the controller family the firmware probe resembles; the register table was not available |
| Maxim/Dallas, *DS1780 CPU Peripheral Monitor* data sheet | power-on slave address of a candidate monitor on the M-Bus |
| Public forum posts on Origin 300 L1 firmware updates and two-brick setups | firmware version strings, L2 console logs of USB enumeration |
| Public community pages on the Origin 3400 L2 hardware | L2 CPU (MPC860T), memory and flash sizes |
| SGI, *SGIconsole Hardware Connectivity Guide*, 007-4340-003 | which systems carry an MSC, MMSC, L1 or L2 |
| SGI, *Origin2000 and Onyx2 Diagnostic Roadmap*, 108-0164-002, printed B-1 (PDF page 55), Figure B-1 | one MSC and one management I2C bus per four-node module, shared NVRAM on that bus ([module management](origin2000-module-management.md), hint) |
| SGI, *Origin200/2000 IP27 PROM Technical Reference Manual*, 108-0170-001, printed pages 1-4 and 1-5 | module number range, unassigned value and automatic assignment ([module management](origin2000-module-management.md), hint) |
| SGI, *Origin 3000 Series Owner's Guide*, 007-4240-002 | L2 board, its ports, and the USB and RS-422 topology of an Origin 3000 rack ([L2 board contract, section 8a](l2-board-contract.md)) |
| SGI, *Origin 3000 Series Technical Configuration Owner's Guide*, 007-4311-002 | L2 board size and power, and which systems need an L2 ([L2 board contract, section 8a](l2-board-contract.md)) |
| SGI, *Origin2000 Rackmount Owner's Guide*, 007-3456-003; *Origin2000 Deskside Owner's Guide*, 007-3453-002; *Onyx2 Deskside Workstation Owner's Guide*, 007-3454-005; *Onyx2 Rackmount Owner's Guide*, 007-3457-005 | MSC, ELSC and MMSC naming, cabling and displays |
| SGI, *Origin 200 and Origin 200 GIGAchannel Owner's Guide*, 007-3708-002 | MSC placement, DIP switch, MSC firmware-version note |
| SGI, *Deskside POWER CHALLENGE and CHALLENGE L Owner's Guide*, 007-1732-060; *POWER CHALLENGE and CHALLENGE XL Rackmount Owner's Guide*, 007-1735-050; *POWER Onyx and Onyx Deskside Owner's Guide*, 007-1733-070; *POWER Onyx and Onyx Rackmount Owner's Guide*, 007-1736-060 | the Challenge and Onyx system controller |
| Philips, *PCD8584 I2C-bus controller* data sheet (and the Philips I2C-bus specification it cites) | PCF8584 register selection, control bits and clock-control fields ([PCF8584 page](pcf8584.md), inferred; the sheet was not read in this pass) |
| Linux kernel source, `drivers/i2c/algos/i2c-algo-pcf.c` | the own-address shift onto the wire ([PCF8584 page](pcf8584.md), inferred) |
| Linux 2.5.0 SN controller driver, `arch/ia64/sn/io/l1_command.c` | controller configuration block bytes 0x701 to 0x70d ([module management](origin2000-module-management.md), inferred) |
| IRIX 6.5.30 IP27 PROM 6.156 payload, SHA-256 `c2c69bf33c375b43ec7ac511fab41ddf595be1b6838ae717e58a9a1273437fdd` | the early PCF liveness probe: its reads, deadline and address form ([PCF8584 page](pcf8584.md), established from disassembly) |
| Linux kernel source, `arch/mips/sgi-ip27/ip27-nmi.c` | MMSC NMI delivery limits (hint) |
| Intel, *28F008SA* 8-Mbit flash memory data sheet | identifier bytes `0x89`/`0xa2`, block size and command set the MMSC flash code uses (hint) |
| Chuck Forsberg, *XMODEM/YMODEM Protocol Reference* (1988) | XMODEM-CRC and 1K packet framing, control bytes (hint) |
