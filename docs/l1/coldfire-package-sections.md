# Origin 300 L1 1.44.0 package sections and boot candidates

The selected IRIX 6.5.30 package contains a ColdFire reset region and two loader candidates. The reset region has an explicit two-entry pointer table. Both pointed candidates pass the package's own additive metadata check. These facts identify where the boot code can look. They do not identify which flash bank ran on a particular Origin 300.

## Checked section anchors

| Body offset or address | Checked contents | Limit |
|---|---|---|
| `0x000000`–`0x0003ff` | The first two big-endian words give an initial stack pointer `0x301017fc` and reset entry `0xffe00400`. Later words in this interval point into early reset code. | This is the packaged vector region. The reset-time physical mapping of address zero on a fitted L1 board is unmeasured. |
| `0xffe00400`–`0xffe00b38` | Decoded early reset and loader path. It initializes RAM, checks a retained candidate pointer under a branch condition, and otherwise scans the flash pointer table. | The retained-state branch condition and board reset state are not established from the package alone. |
| `0xffe01000`–`0xffe0100b` | Two aligned entries point to `0xffe10000` and `0xffe80000`. The third word is all ones. Every subsequent measured word up to `0xffe10000` is all ones. | The two entries are present in this package. Installed flash could differ. |
| `0xffe10000` and `0xffe80000` | Each candidate starts with a pointer to its metadata record, then the initial stack pointer and a candidate entry address. The entries are `0xffe10b02` and `0xffe80b02`. | These are loader candidates, not independently captured active banks. |
| `0xffe7fd1c`–`0xffe7fd4b` and `0xffeefd1c`–`0xffeefd4b` | Each 48-byte record starts with type word four and length word `0x30`. Each has 12 big-endian 32-bit words whose sum modulo `2^32` is zero. | This is the firmware's record check, not a whole-bank checksum or proof that flash was programmed correctly. |

The candidate starts are `0x70000` bytes apart. Corresponding code and data after their first 64 KiB show matching content with address-bearing words adjusted between the candidates. That correspondence supports two related images. It does not make every byte data or prove that either copy is a complete flash-bank dump. The selected body stops at `0xffeefd5f`, 20 bytes after the second 48-byte metadata record. It does not extend to the loader's `0xfff00000` upper search bound. The 63,700-byte early Ghidra gap and the other unclassified spans need pointer or control-flow evidence before classification.

## What the boot branch establishes

The loader scans aligned table words from `0xffe01000` toward `0xfff00000` until an all-ones terminator, then considers entries in reverse order. For an entry, it first checks that the entry and its first pointed word are aligned and within `0xffe00000`–`0xffefffff`. It uses the pointed word as a metadata address. A record with type below three skips its first four bytes for the sum. These two type-four records include all 48 bytes. A nonzero 32-bit sum rejects the candidate. The original package bytes give zero for both records. The table entries have their low flag bit clear. The branch can retain a candidate based on a text field in the metadata. The two measured records point to the same build stamp and version. Those fields are the published `20060717182038` and `1.44.0` values, not evidence of a distinct later bank.

Before that scan, the reset path can accept a saved candidate pointer in RAM when a flag and range checks permit it. The branch validates that candidate's metadata too. After choosing a candidate, another routine copies 1,024 bytes from just after its first pointer into RAM at `0x30100000`, sets the vector base there and calls the candidate entry. This establishes a loader pointer consumer and an execution path for a valid candidate. It does not establish how a fitted board maps flash at reset or which candidate was selected during a real boot. A revision-matched flash map or boot-time program-counter trace, paired with the installed-bank bytes and reset state, would settle that question.
