# The old inst compression (IRIX 5.x to 6.2)

Each IRIX 5.x and 6.x `inst` product ships a text index (`.idb`) and one or more image files (`.sw`, `.man`, ...). The index has one line per file with `size(N)`, the checksum `sum(S)` and, for files stored in the image, the compressed size. The task was to decode the compressed streams of the older builds (the first sixteen bytes of every stream were seen to be the same).

## Finding

**Established** for the patch images listed below. The streams of IRIX 5.3 and 6.2 images are classic `compress(1)` data: each starts with the two magic bytes `1f 9d` followed by the flag byte `0x90` (16-bit codes, block mode), and decompresses with `gzip -dc`. The shared leading bytes are not a header of the format. **Inferred**: they are the compressed form of the identical first bytes of every ELF file, so any two executables begin with the same compressed prefix (in the 6.2 patch the entries for `sbin/nvlog` and `sbin/pset` begin `1f 9d 90 7f 8a 30 31` and then differ in one byte, `12` against `22` or `11`).

The archive layout is the one used by the 6.5 media: a twelve-byte tag, then for each file a two-byte big-endian name length, the name, and that many bytes of stream. The tag is `im001V530P00` in the 5.3 samples and `im001V620P02` in the 6.2 sample, followed by one zero byte before the first name length (offset 13). Entries whose compressed size is 0 in the index (the release-notes files) are not in the archive. Files whose entry has no compressed size are stored uncompressed.

## What was tested

Three archives were unpacked with a small reader written for this check (read the index, walk the archive, decode each stream with `gzip -dc`, compare the decoded length with `size(N)` and the 16-bit rotating sum with `sum(S)`), and every in-archive entry matched on both:

| Archive | Release | In-archive entries | Matched |
|---|---|---:|---:|
| `patchSG0001099.eoe1_sw` (Indigo2 IMPACT graphics patch) | 5.3 | 8 | 8 |
| `patchSG0001157.eoe1_sw` | 5.3 | 3 | 3 |
| `patchSG0003566.eoe_sw` (Challenge and Onyx kernel and tools patch) | 6.2 | 173 | 173 |

The 6.2 archive carries the Everest daemon: `usr/etc/sysctlrd`, size 45,988 bytes, stored in 24,578, sum 57809 (matches), a 32-bit big-endian MIPS-II executable with a build line dated January 1999 for IRIX 6.2. The checksum that the index calls `f(...)` is not the zlib CRC-32 of the file; it was not needed.

A byte search of the decoded 6.2 daemon finds the same protocol strings as the 6.5 daemon (the `06Y` and `%2x` length-mode strings, "Result length mismatch", the spinner string, the `dgprnestf:vlhk` option string, "System controller firmware is downrev", "Get scale failed", "Event: %s", the time format `%m%d%y%H%M%S`), and the diagnostic table lacks three of the 6.5 CPU progress texts ("CPU initializing SCACHE.", "CPU testing CC Join counters.", "CPU testing CC tags."). It is a 32-bit build, not read routine by routine.

## Not tested

No IRIX 5.0 to 6.2 *system disc* image was available here, so the media streams themselves were not decoded; the archives tested are patch images whose format version tags (`V530`, `V620`) are those of the same releases. The same method (the `.idb` offsets and sizes, then `gzip -dc`) should apply to the discs. Nothing was held for IRIX 5.0 to 5.2 or 6.0 to 6.1.

## Public decoders

The format is documented and extracted by public tools: a Go program `sgix` (github.com/depp/sgix) reads an `.idb` with its `.sw` files; a shell script `swextract` (github.com/misuchiru03/swextract) does the same and notes that it does not handle `.sw64` images; the forum thread "TIP: .sw/.idb distribution image format" on the Nekonomicon describes extracting one file by hand (find the file, copy that many bytes after the name, the data starts with `1f 9d`). These were not run here. **Hints**, not findings.
