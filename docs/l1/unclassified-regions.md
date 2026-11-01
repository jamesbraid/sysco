# Origin 300 L1 1.44.0: code and data inside Ghidra gaps

The table belongs to a general text formatter. This changes the interpretation of the gap map. It does not establish a new L1 request or a host-visible USB reply. Elsewhere, substantial erased-looking and string-rich regions remain unclassified. Static byte profile alone does not prove that they contain no executable paths.

A fresh interval reconciliation checks that these data carvings are disjoint. None of these counts selects the bank that runs on a fitted IP45 board.

The analyzed 982,368-byte body after its measured 44-byte package prefix has SHA-256 `9db44f3bb06ec90475035781766e1ca7e924f15a0ed215cf855c1f491275fdd1`.

Ghidra decoded 213,926 instructions occupying 662,700 bytes, or 67.46% of the body. Its 1,914 unclassified spans occupy 319,668 bytes, or 32.54%. A strict length greater than 1,024 bytes selects 36 spans totaling 241,634 bytes (75.59% of unclassified bytes). Nine spans are longer than 4,096 bytes and total 193,200 bytes. That classification is a tool result, not proof of data.

## All spans longer than 1 KiB

The rows list each first-bank span and its second-bank counterpart where the gap has identical bounds at address plus `0x70000`. `Same` means identical bytes. `Relocated` means matching bounds but different bytes. The three byte columns count `0xff`, NUL and printable ASCII (`0x20`–`0x7e`) respectively. their categories are exclusive.

| First-bank span | Bytes | FF | NUL | Printable | Mirror |
|---|---:|---:|---:|---:|---|
| `0xffe00b38`–`0xffe1040c` | 63,700 | 62,910 | 17 | 98 | none |
| `0xffe1100a`–`0xffe115b0` | 1,446 | 113 | 149 | 601 | relocated |
| `0xffe115ee`–`0xffe11cd4` | 1,766 | 73 | 632 | 859 | relocated |
| `0xffe1bfd0`–`0xffe26f9c` | 45,004 | 1,390 | 15,431 | 24,812 | relocated |
| `0xffe2bf82`–`0xffe2cf94` | 4,114 | 0 | 143 | 3,904 | same |
| `0xffe362dc`–`0xffe367e4` | 1,288 | 3 | 321 | 909 | relocated |
| `0xffe38378`–`0xffe3888c` | 1,300 | 1 | 57 | 1,213 | same |
| `0xffe39b52`–`0xffe3b87c` | 7,466 | 313 | 1,155 | 4,835 | relocated |
| `0xffe447aa`–`0xffe44ff4` | 2,122 | 120 | 654 | 808 | relocated |
| `0xffe45c88`–`0xffe4630c` | 1,668 | 108 | 499 | 728 | relocated |
| `0xffe49926`–`0xffe4a1fc` | 2,262 | 110 | 575 | 1,216 | relocated |
| `0xffe4e336`–`0xffe4ec24` | 2,286 | 34 | 271 | 1,840 | relocated |
| `0xffe5091e`–`0xffe50eb8` | 1,434 | 6 | 53 | 1,344 | relocated |
| `0xffe53738`–`0xffe53ea0` | 1,896 | 0 | 33 | 1,840 | same |
| `0xffe56e74`–`0xffe57644` | 2,000 | 0 | 134 | 1,817 | same |
| `0xffe5b9aa`–`0xffe5d990` | 8,166 | 327 | 589 | 6,345 | relocated |
| `0xffe6fd20`–`0xffe703a8` | 1,672 | 3 | 226 | 1,386 | same |
| `0xffe7f0e0`–`0xffe7f7dc` | 1,788 | 120 | 1,137 | 219 | relocated |
| `0xffe7f9fa`–`0xffe8040c` | 2,578 | 978 | 706 | 115 | none |

The first row has a contiguous 61,433-byte `0xff` run. That run is consistent with unused flash padding, but the whole 63,700-byte span contains other bytes and cannot be declared padding. The 45,004-byte pairs at `0xffe1bfd0` and `0xffe8bfd0` have many printable and NUL bytes but different hashes, as expected of possible relocated pointers or text pools. Across the 36 large spans, a simple search found 3,334 printable ASCII runs of at least four characters followed by NUL, occupying 71,078 bytes including terminators. Their bytes may overlap other binary structures. The gap `0xffe7f9fa`–`0xffe8040c` crosses the apparent `0xffe80000` bank boundary. The selected package does not supply a section or relocation map that would establish the role of every byte there. The 44-byte package boundary establishes body extent, not internal section types.

## Bounded data and executable paths

There are 220 distinct PC-indexed `jmp` anchors in unclassified spans. Those guarded prefixes occupy **10,916 bytes entirely within gaps**. Of their 5,458 table entries, 5,446 target existing decoded instruction rows. Twelve entries target two unclassified spans, all six entries per bank in the following table. No guarded entry goes outside the decoded or unclassified body partitions.

| Bank | Table data | Executable gap | Entry targets | Continuation |
|---|---|---|---|---|
| First | `0xffe7b470`–`0xffe7b47c` | `0xffe7b47c`–`0xffe7b4e4` | `0xffe7b47c`, `0xffe7b490`, `0xffe7b4a4` twice, `0xffe7b4b8`, `0xffe7b4d0` | `0xffe7b4e4` |
| Second | `0xffeeb470`–`0xffeeb47c` | `0xffeeb47c`–`0xffeeb4e4` | `0xffeeb47c`, `0xffeeb490`, `0xffeeb4a4` twice, `0xffeeb4b8`, `0xffeeb4d0` | `0xffeeb4e4` |

The first-bank guard at `0xffe7b45e`–`0xffe7b462` constrains the masked index to six entries. The second guard is relocated by `0x70000`. Each listed target is an instruction boundary. Four entry blocks branch directly to the already decoded continuation. The fifth falls into it. Entries zero, one, two and three each advance an argument pointer by four bytes and copy the preceding 32-bit value to the formatter's local value slot. Entry four narrows that value to an unsigned 16-bit number. Entry five sign-extends its lower 16 bits. The executable bytes are identical between banks (SHA-256 `bfbb2a40490d8c6ed24e7399bec85c1ac2185580a8e9eaf63bfadeeb9b5d2d28`). The 12-byte table prefixes are also identical (SHA-256 `451d3e4e29493d04eabfb61ecc8b9c41af6ed7cc495324369f7627c58089c648`).

Its beginning scans characters and handles an ASCII percent character through another indexed dispatch. Three checked direct first-bank callers at `0xffe7a494`, `0xffe7a508` and `0xffe7a592` pass a text source and argument pointer with an initialized output context, then finalize that context. Their second-bank copies follow the same shape. This supports a generic formatter interpretation. It does not identify the final output sink or which L1 console message, if any, uses these five cases. The discovery is unrelated to the four USB setup/status C timeouts and cannot fill their unknown lower controller status stage.

This proves **100 more data bytes**, not the remaining 3,244 bytes in those two gaps. The guarded table prefixes and USB descriptor prefixes establish 11,016 disjoint data bytes. The two mixed spans establish 208 executable bytes. At this first classifier pass, the other 308,444 gap bytes remained unresolved by these strict checks. The later disjoint carving above reduces that count. Neither number claims that all unresolved bytes are executable candidates.

The direct absolute branch/call scan found zero targets inside any gap. Counting the anchor as an executable target would miss precisely the 208-byte discovery. Indirect calls, runtime-relocated pointers and data-driven branches are also outside the direct scan. Do not raise instruction coverage by forcing all gap bytes into code.

## Targeted regions and remaining evidence

The first-bank USB window `0xffe6f080`–`0xffe716de` intersects 23 gap spans/2,188 bytes, including the descriptor prefix. The M-Bus controller and EEPROM-reader window `0xffe6ac90`–`0xffe6d23c` intersects 36/1,404. The early console window `0xffe1b0f8`–`0xffe1bfd0` intersects three/16. The text-output wrapper and formatter window `0xffe7a464`–`0xffe7b8b2` intersects 14/340, including the 116-byte mixed gap. These are window intersections, not device-wide totals. The M-Bus image reading locates the controller and IP45 registration paths. It does not establish all bytes in their gap windows as tables.

To resolve other gaps, inspect a revision-bound package format or loader map showing flash sections, vector/relocation records and selected boot bank. Then trace every computed and indirect destination on each reachable path. Printable/NUL counts cannot do that. For erased regions, a matching flash map or runtime program-counter trace must rule out code in the non-`0xff` islands. A Ghidra `complete` status or a static direct-call negative is insufficient evidence for those questions.

## Reconciled original-gap partition

Every one of its 398 data intervals stays inside an original Ghidra gap. Sorted intervals have no overlap with each other or with the 208 proven executable bytes. Two fresh runs produced that same hash, and `--verify-only` exited zero. The tracked script hashes to `bcc073637a04921b82086f25818822fff908af086d25f32160460307ec58d208`.

The arithmetic is `319,668 - 29,166 - 208 = 290,294` unresolved gap bytes.
