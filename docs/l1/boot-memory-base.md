# Origin 300 L1 package: boot-time memory-base values

The selected Origin 300 L1 package programs the same ColdFire memory-base values in its reset code and both loader candidates. This is a static reading of the packaged software.

Its body has SHA-256 `9db44f3bb06ec90475035781766e1ca7e924f15a0ed215cf855c1f491275fdd1`. The package section map identifies the reset vector and both loader candidates.

| Packaged code location | Software-written value | Meaning and limit |
| --- | --- | --- |
| Reset entry `0xffe00400` | RAMBAR `0x30100001` | The reset path enables an SRAM base at `0x30100000`. The package does not prove the fitted SRAM size or physical mapping. |
| Candidate entry `0xffe10b02` | RAMBAR `0x30100001`; MBAR `0x10000001` | One candidate enables the same SRAM base and an MBAR base at `0x10000000`. It is not a captured active bank. |
| Candidate entry `0xffe80b02` | RAMBAR `0x30100001`; MBAR `0x10000001` | The other candidate has the same programmed values. It is not a captured active bank. |

Its MCF5206e on-chip SRAM is 8 KiB and requires an 8 KiB aligned base. The selected software's `0x30100000` is aligned accordingly. A match between software values and a published chip register description is **not** component fitment evidence. The public IP45/I/O-8 board search has not resolved the fitted ColdFire revision or SRAM capacity.


