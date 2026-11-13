# Pre-container L1 images (6.5.9 to 6.5.15)

The L1 images of IRIX 6.5.9 to 6.5.15 are not containers. Each file is one ColdFire package, and the package starts with the same 32-byte header as the later packages. This note records what the six early images share, where each one's vector table starts, and what was not settled. The sets are in [firmware-versions.md](../../../evidence/firmware-versions.md) and [controllers.md](../../../evidence/controllers.md). The reader that parses them is `retools/tools/sysco/sysco_tables.py` (`pre_container_package`).

Everything below was measured from the shipped images. Grades follow [AGENTS.md](../../../AGENTS.md).

## Header words and body start

Every word is big-endian. The first 0x20 bytes are the same in all six: `ESTFBINR`, then the load address `0xffe00000` at 0x08 and a last address at 0x0c, then `0xff000000`, zero, and the load and last addresses again at 0x18 and 0x1c.

| IRIX | L1 | File bytes | 0x20 | 0x24 | 0x28 | 0x2c | Vector table at | Body bytes |
|---|---|---:|---|---|---|---|---|---:|
| 6.5.9 | 1.0.2 | 927,304 | `0x301017fc` | `0xffe00400` | `0xffe0042c` | `0xffe00430` | 0x20 | 927,272 |
| 6.5.10 | 1.1.6 | 961,604 | `0xffffffff` | `1` | `0x301017fc` | `0xffe00400` | 0x28 | 961,564 |
| 6.5.12 | 1.4.1 | 947,836 | `0xffffffff` | `2` | `0x301017fc` | `0xffe00400` | 0x28 | 947,796 |
| 6.5.13 | 1.6.0 | 957,104 | `0xffffffff` | `2` | `0x301017fc` | `0xffe00400` | 0x28 | 957,064 |
| 6.5.14 | 1.8.0 | 954,016 | `0xffffffff` | `3` | `0` | `0x301017fc` | 0x2c | 953,972 |
| 6.5.15 | 1.10.0 | 972,844 | `0xffffffff` | `3` | `0` | `0x301017fc` | 0x2c | 972,800 |

Established, from each file: the initial stack pointer `0x301017fc` and the reset vector `0xffe00400` sit at the offset in the "Vector table at" column, and that offset is the only place in the file where that pair occurs. The body is the file from that offset to its end. In every row, the last address minus the load address plus one equals the body length. So the header's last-address field describes the body, and the body starts where the vector table starts.

What the words at 0x20 to 0x2c mean is not established. `0xffffffff` at 0x20 appears in 6.5.10 and later. The count-like word after it (1, 2, 2, 3, 3) is not checked against any package count: each of these files holds one package. The zero at 0x28 in 6.5.14 and 6.5.15 is present only there. Unknown: the role of these words. Routes tried: comparing the six headers with one another and with the 1.x container headers. Next route: find instructions in these bodies that read the absolute addresses `0xffe00020` to `0xffe0002f`, or that load PC-relative from the first 0x40 bytes of the body, and read each one.

## One package per file

Established: each image holds exactly one vector-table start (the stack-pointer and reset-vector pair), and no other `ESTFBINR` marker. The later containers put one such start per package, at the start of each package. So these files hold one package, not a table of packages.

Established: each body holds its program twice, 0x70000 bytes apart, as the 1.x packages do. Body bytes from 0x10000 to 0x70000 agree with the bytes 0x70000 higher in 96.9 to 97.4 percent of positions (6.5.9 97.4, 6.5.10 97.2, 6.5.12 97.0, 6.5.13 96.9, 6.5.14 97.1, 6.5.15 97.0). The positions that differ were not characterised. Unknown: why they differ. The remaining body bytes after the second copy are 9,768 to 55,296 bytes of tail; their contents were not read.

Established: there is no build-stamp label and no product label such as `Fuel/PE 1MB image` in any of the six. The 1.x packages carry those labels at the end of each body. So nothing in these six images names a product.

## What this changes

The earlier note in [controllers.md](../../../evidence/controllers.md) described 6.5.12 to 6.5.15 as having a "package table at 0x2c" that the reader could not parse. That is wrong for these six: the word at 0x2c is the reset vector in 6.5.12 and 6.5.13, and the stack pointer in 6.5.14 and 6.5.15, and no table is present. The reader now takes each of these images as one package (`pre_container_package`), with the vector table at the offset above and no product label.

## The 6.5.16 to 6.5.19 containers

These four files are containers with two ColdFire packages (labels `1MB image` and `2MB image`), parsed by the existing reader. The package table at 0x30 holds two offset and length pairs, `(0x0, 0xedf24)` and `(0xedf24, 0x1842e4)` in 6.5.16, and the first package's vector table starts at 0x40, just after the table. The 1 MB image of each container is the one the early version diff uses. Which product package it is was not settled (see [version-diffs.md](../../../evidence/version-diffs.md#early-series-102-to-1183)).

## Request dispatch in the early sets

The general request dispatch of each early set is a compare chain of the same shape as the 1.44.0 chain in [request-codes.tsv](request-codes.tsv): a `move` of the request code into `d0`, `cmp.l` against a request code (`#$1001` or `#$1002`), a branch to a second compare (`bgt`) and a branch to the first handler (`beq`), a compare against the highest low-range code, and a jump through a table. Read from the bytes at each set's chain (established for each set):

- 6.5.9, 6.5.10, 6.5.12 and 6.5.13 load the handler address from a longword table with `movea.l (d8,pc,d0.l*4),a0` and then `jmp (a0)`. Each reads the code from its frame slot `-$18(a6)` before the first compare.
- 6.5.14 to 6.5.19 read a word table with `move.w (d8,pc,d0.l),d0`, sign-extend it and jump with `jmp (d8,pc,d0.l)`.
- 6.5.15 opens its chain with `#$1002`. The other nine sets open with `#$1001`.

The table in each set and the codes it accepts are in [version-diffs/l1-early-request-catalogue.tsv](../../../evidence/version-diffs/l1-early-request-catalogue.tsv).
# Pre-container L1 images (IRIX 6.5.9 to 6.5.15)

Six L1 images come before the container layout of [package-container.md](package-container.md): 1.0.2 (6.5.9), 1.1.6 (6.5.10), 1.4.1 (6.5.12), 1.6.0 (6.5.13), 1.8.0 (6.5.14) and 1.10.0 (6.5.15). Each file is one ColdFire MCF5206e package with no package table, no label and no ARM Linux package: none of the six holds a gzip stream or a CramFS image (established, by scanning each file for both signatures). The 6.5.9 and 6.5.10 images are in the base product (`usr/cpu/firmware/l1.bin`); the other four are in the sysco product. The layout below is the same in both places.

The 6.5.16 image is the first container, with a package count at `0x2c` (see [package-container.md](package-container.md)).

Grades follow [AGENTS.md](../../../AGENTS.md) section 3. Everything here was read from the six files. The reader finds the vector table with `pre_container_body` in the shared `retools/tools/sysco/sysco_tables.py`; the last-address and loader-table checks below were run on all six files separately.

## Header

| Offset | Content | Grade |
|---|---|---|
| `0x00` | ASCII `ESTFBINR` | established, all six |
| `0x08` | load address `0xffe00000` | established, all six |
| `0x0c` | last address: `0xffe00000` plus (file size minus V) minus 1, where V is the vector table offset below | established, all six |
| `0x10` | `0xff000000` | established, all six |
| `0x14` | `0` | established, all six |
| `0x18`, `0x1c` | the load and last addresses again | established, all six |
| `0x20` | `0xffffffff` in 6.5.10 to 6.5.15. In 6.5.9 this word is the stack pointer (see the vector table) | established |
| `0x24` | layout word: 1 (6.5.10), 2 (6.5.12, 6.5.13), 3 (6.5.14, 6.5.15). The container at 6.5.16 has 4 here | established values; meaning unknown |
| `0x28` | `0` in 6.5.14 and 6.5.15; the stack pointer in 6.5.10, 6.5.12 and 6.5.13; the third vector word in 6.5.9 | established |

## Vector table and mapping

The vector table begins with the initial stack pointer `0x301017fc`, followed by the reset PC `0xffe00400` and a third word `0xffe0042c`. The table's offset V in the file is:

| Image | V |
|---|---|
| 6.5.9 | `0x20` |
| 6.5.10, 6.5.12, 6.5.13 | `0x28` |
| 6.5.14, 6.5.15 | `0x2c` |

An image offset `o` maps to address `0xffe00000 + (o - V)`. This is established by the loader table: at address `0xffe01000` (image offset V + `0x1000`) the image holds two bank starts, `0xffe10000` and `0xffe80000`, in all six. The 1.44.0 packages use the same table at the same addresses ([personalities.md](personalities.md#the-bank-record)).

The stack pointer `0x301017fc` is the same word the 1.44.0 packages start with ([coldfire-package-sections.md](coldfire-package-sections.md)). That it is the initial stack pointer follows from its position in the vector table (inferred).

## Bank records

Each bank holds one 32-byte record. The first is in bank 1 (`0xffe10000`) and the second in bank 2 (`0xffe80000`); the two are `0x70000` apart in the file.

| Word | Content | Grade |
|---|---|---|
| 0 | `0x00030020`: type 3, length `0x20` | established, all six |
| 1 | bank start (`0xffe10000` or `0xffe80000`) | established, all six |
| 2 | bank size `0x70000` | established, all six |
| 3 | bytes from the bank start to the end of the record | established: equals the record's end address minus the bank start, for both records of all six images |
| 4 to 6 | three pointer-like words | unknown |
| 7 | one word, not decoded | unknown |

The record is not the 48-byte type 4 record of the 1.44.0 packages. Whether words 4 to 7 take part in a sum check like the one 1.44.0 uses was not tested. The four-byte pattern `0x00030020` also occurs elsewhere in these files (in code and data). A record must be found by position, and the word 3 check is what separates it from those hits.

## Version strings

The version reported for each image is the first occurrence of the most common `1.N.N` string. In 6.5.14 and 6.5.15 the image also holds `1.8.1` and `1.10.1`, each twice, a tie with the reported string. The reported strings (1.8.0 and 1.10.0) match the firmware note. The second strings are unknown; each occurs at two offsets `0x70000` apart, and no word read here refers to them.

## Routes tried and what is left

- The header words and the vector table were read in every image, and the mapping was tested against the loader table and the last-address rule.
- The bank records were found by pattern and tested by word 3 in both banks of all six images.
- Not settled: the meaning of the layout word at `0x24` and of the zero word at `0x28` in 6.5.14 and 6.5.15; words 4 to 7 of each bank record; the `1.8.1` and `1.10.1` strings. Nothing in the six images or the 1.44.0 package settles them so far.
