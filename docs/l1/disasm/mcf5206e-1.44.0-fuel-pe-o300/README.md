# L1 1.44.0, package 1 (Fuel/PE/O300 1MB), ColdFire body

Source: `l1.bin` of the IRIX 6.5.30 `field_diags_sysco` product (sha256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`), package 1 at container offset `0xefdd0`, 982,412 bytes, sha256 `0c969f9e98c73826495dc37bff685b3403e248993ba92194a288333d9d42b2f4`. A 44-byte header precedes a 982,368-byte body (sha256 `9db44f3bb06ec90475035781766e1ca7e924f15a0ed215cf855c1f491275fdd1`) that loads at `0xffe00000`. Version string 1.44.0, build stamp `20060717182038`.

| File | Contents |
|---|---|
| `l1.lst` | linear listing of the body: address, instruction bytes, mnemonic and operands; data decodes as `.dw` or implausible instructions |
| `l1.sum` | per-routine summary: calls, resolved references, table jumps, immediate compares |

The firmware's text strings are not committed; read them from the image in `root/`.

Decoder: a 68040-mode M68K decoder (capstone 5), which covers the ColdFire ISA_A subset the compiler emitted. Routine names used in `l1.sum` section headers are in [../../functions.tsv](../../functions.tsv).

The body holds the program twice, `0x70000` apart (version strings at body offsets `0x10ad4` and `0x80ad4`); the listing covers both. Of 114,688 words compared with the same body shifted by `0x70000`, 87,243 are identical, 6,937 differ by exactly that distance (address constants relocated for the other flash bank) and 20,508 differ otherwise.
