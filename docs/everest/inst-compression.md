# The old inst image format (IRIX 5.0 to 6.2)

Every IRIX 5.x and 6.x `inst` product ships a text index (`<product>.idb`) and image files beside it (`<product>.sw`, `<product>.man`, and so on). The index has one line per installed file. Lines for regular files carry the original size (`size(N)`), a 16-bit checksum (`sum(S)`) and, when the file is stored compressed, the stored size (`cmpsize(N)`; zero or absent means the bytes are stored as they are). This note settles how the early builds are read, so that the earlier Challenge and Onyx controller tools can be extracted. Grades follow [../../AGENTS.md](../../AGENTS.md).

## Finding

**Established** for the base software images of the system discs of IRIX 5.0, 5.1.1, 5.2, 5.3, 6.0 and 6.1 (product `eoe1`) and 6.2 (product `eoe`). Every compressed record is classic Unix `compress` data: it starts with the bytes `1f 9d 90` (magic, then flags `0x90`: 16-bit codes with block mode) and decodes with `gzip -dc`. Nothing else is needed.

The index of these builds has **no offset field**. The `off(N)` attribute that later indexes carry first appears in the IRIX 6.3 index (6,257 of its lines have it); none of the eight early indexes read (5.0 to 6.2) has it. A decoder that seeks to an offset taken from the index, as the Go program named under the public decoders does, cannot place a file from these indexes. The archive has to be walked from its start.

## Archive layout

**Established** by walking the images (table below). An image starts with a 12-byte tag, `im001` followed by a 7-character version (for example `im001V530P00`), then one zero byte, then the records, packed with no padding, in the order of the index lines that name that image:

| Field | Size | Meaning | Grade |
|---|---:|---|---|
| name length | 2 bytes, big-endian | length of the install path | established |
| name | that many bytes | the install path, as in the index | established |
| data | `cmpsize` bytes, or `size` bytes when `cmpsize` is zero or absent | the stream, or the stored bytes | established |

In every image the walk ends exactly at the end of the file and the name in each record equals the name of the next index line for that image. The index lines belonging to one `.sw` image are those whose subsystem field begins with the image's file name (`eoe1.sw.unix`, `eoe1.sw.dlpi` and so on); lines of the `.man` image belong to that file. Directory and link lines have no record.

The first sixteen bytes of an image file are the tag, the zero byte, the first record's length and the first characters of its name, so they are the same in every image of one release. They are not a header of the compression. What many streams do share is a prefix inside the data: `1f 9d 90 7f 8a 30 31 12 40 40 00 00 08 13 2a 14` starts 295 of the 2,087 compressed records of the 5.0 image and 673 of the 4,655 of the 6.2 image. It is the compressed form of the start of a 32-bit big-endian MIPS executable: the first two nine-bit codes of the stream are `0x7f` and `0x45`, the first two bytes of the ELF magic, and every executable begins with the same header bytes.

## What was tested

A reader written for this check (walk the records in index order, decode each compressed stream with `gzip -dc`, compare the decoded length with `size(N)` and the checksum with `sum(S)`) covered every record of the main software image of seven system discs. The checksum is the BSD rotating sum (rotate right by one bit, then add the byte, 16 bits); the plain byte sum does not match.

| Disc, software image | Image tag | Records | Matched on length and checksum | Stored uncompressed |
|---|---|---:|---:|---:|
| IRIX 5.0, `eoe1.sw` | `im001V500P00` | 2,087 | 2,087 | 0 |
| IRIX 5.1.1, `eoe1.sw` | `im001V500P00` | 2,554 | 2,554 | 0 |
| IRIX 5.2, `eoe1.sw` | `im001V511P00` | 2,457 | 2,457 | 0 |
| IRIX 5.3, `eoe1.sw` | `im001V530P00` | 2,681 | 2,681 | 115 |
| IRIX 6.0, `eoe1.sw` | `im001V512P00` | 2,571 | 2,571 | 0 |
| IRIX 6.1, `eoe1.sw` | `im001V610P00` | 2,514 | 2,514 | 132 |
| IRIX 6.2, `eoe.sw` | `im001V620P02` | 5,006 | 5,006 | 351 |

The system discs "IRIX 5.2 for TKO Onyx" and IRIX 6.0.1 carry the same product (tags `im001V511P00` and `im001V512P00`); only their Everest files were decoded, and those matched. The 6.3 index carries `off(N)`, and the daemon of that disc decodes with the offset-based extraction tool already used for the later discs.

## The Everest files

The files that the Challenge and Onyx host side needs were extracted and compared with the index: the daemon, the message tool, the driver object for each CPU board, the shipped header, the boot-time init script, the two `config` flags (`cpumeter` and `cleanpowerdown`, three bytes each) and the driver's kernel configuration file. All 78 files from nine images (the seven above plus the two named) match their size and checksum. They are read in [early-builds.md](early-builds.md). The 5.0 index names the driver object `usr/sysgen/boot/sysctlr.o` and the message tool `etc/scmsg`; the later builds use `usr/cpu/sysgen/IP19boot/sysctlr.o` and `sbin/scmsg`.

An earlier pass, on three patch images (two IRIX 5.3 graphics patches and one IRIX 6.2 Challenge and Onyx kernel and tools patch, 173 in-archive entries), reached the same decode and layout; it is not repeated here. That pass recorded a 6.2 patch daemon of 45,988 bytes with the length-mode strings and the option string `dgprnestf:vlhk`, which the 6.2 base daemon does not have (it has `dgprnestf:vhk`; see [early-builds.md](early-builds.md)). That patch daemon was read by byte search only.

## Public decoders

- A Go program `sgix` (github.com/depp/sgix) reads an `.idb` with its `.sw`. **Read**: its extraction code seeks to an offset taken from the index line and, when the stored size is positive, pipes that many bytes through the system `uncompress` program. It relies on the offset field, so it cannot extract the images above. It also never checks the name it reads at the offset.
- A shell script `swextract` (github.com/misuchiru03/swextract) reads an `.idb` and extracts from the matching `.sw`; its README says `.sw64` images are not handled yet. **Hint**: its code was not read.
- A forum thread on the format ("TIP: .sw/.idb distribution image format") describes cutting one file out by hand: the data starts with `1f 9d`. **Hint**: not read.

No public tool that walks an index without offsets was found. The layout table above is enough to write one.
