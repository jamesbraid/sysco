# Origin 300 L1 package: boot-time memory-base values

The selected Origin 300 L1 package programs the same ColdFire memory-base values in its reset code and both loader candidates. This is a static reading of the image.

Its body has SHA-256 `9db44f3bb06ec90475035781766e1ca7e924f15a0ed215cf855c1f491275fdd1`. The package section map identifies the reset vector and both loader candidates.

| Packaged code location | Software-written value | Meaning and limit |
| --- | --- | --- |
| Reset entry `0xffe00400` | RAMBAR `0x30100001` | The reset path enables an SRAM base at `0x30100000`. The image uses `0x30100000` to `0x30101800` (stack top at `0x30101800`, vector copy of 1,024 bytes at `0x30100000`), which fits inside an 8 KiB SRAM. |
| Candidate entry `0xffe10b02` | RAMBAR `0x30100001`; MBAR `0x10000001` | One candidate enables the same SRAM base and an MBAR base at `0x10000000`. Which candidate runs is not settled by the image. |
| Candidate entry `0xffe80b02` | RAMBAR `0x30100001`; MBAR `0x10000001` | The other candidate has the same programmed values. Which candidate runs is not settled by the image. |

The MCF5206E on-chip SRAM is 8 KiB (hint: NXP MCF5206E user's manual, SRAM chapter; the original MCF5206 has 512 bytes) and RAMBAR's base field needs an 8 KiB aligned base; `0x30100000` is aligned. The firmware uses RAM from that base up to `0x30101e68` (the stack below `0x30101800` and a 1,240-byte state block at `0x30101990`), so it needs more than the original MCF5206 offers. Neither the manual nor the image says which ColdFire revision the board carries; the 1,024-byte vector copy and the stack are consistent with the MCF5206E but would not rule out external RAM mapped at the same address. Unknown. Routes tried: the reset code and both candidate entries. Next: whether the image programs the DRAM controller or chip-select registers (see the open question on on-chip peripherals) to cover `0x30000000`.


