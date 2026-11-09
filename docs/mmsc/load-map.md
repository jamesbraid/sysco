# MMSC load map and symbols

Read from `usr/cpu/firmware/mmscfw.bin` in IRIX 6.5.30 (MMSC 2.3, 1,023,075 bytes), and measured on all ten MMSC images held (6.4, 6.4-origin, 6.5, 6.5.2, 6.5.5, 6.5.6, 6.5.7, 6.5.9, 6.5.10, 6.5.30). 6.5.7, 6.5.9, 6.5.10 and 6.5.30 are byte-identical.

## Segment map (established, 6.5.30)

| Segment | Load address | Size | File offset | Evidence |
|---|---|---|---|---|
| text | 0x108000 to 0x1afbd8 | 687,064 | 32 | header entry word; `_sysInit` is at 0x108000; `_etext` is 0x1afbd8 |
| data | 0x1afbd8 to 0x1d87d8 | 166,912 | 687,096 | follows text with no padding; `_edata` is 0x1d87d8 |
| bss | 0x1d87d8 to 0x1f6970 | 123,288 | none | follows data; `_end` is 0x1f6970 |
| nlist | not loaded | 7,907 records of 12 bytes | 0xd07f8 | header symbol-size word, 94,884 |
| names | not loaded | 74,183 bytes | 0xe7a9c | led by a four-byte length that counts itself |

The file is one a.out, magic 0407 (an unpadded single image, not a paged one), little-endian. The header's last two words are not relocation sizes: the first is the constant 0x5aa5a55a and the second is zero. The image is linked at its load address and needs no relocation.

Why data follows text directly (established): four independent measurements agree, and `mmsc_aout.py --check` repeats them on any image.

1. The three segment-closing symbols (`_etext`, `_edata`, `_end`) equal the ends computed from the header sizes.
2. Every text, data and bss symbol lies inside its own segment.
3. `_vxWorksVersion` and `_creationDate` are pointer words in data. With data placed at 0x1afbd8 they lead to the strings "5.3" and "Wed Dec 08 12:45:18 1999" held in data. Moving the assumed base by 4, 8 or 16 bytes in either direction, or rounding it up to a 4 KB page, leaves 3 times fewer aligned data words that point at a string, or none (24 against at most 7 in the 6.4 image).
4. 1,976 operands after push, mov and moffs opcodes in text equal a data or bss symbol address exactly.

Also seen: `_sysRomSignature` (data, 0x1afe44) holds the bytes 55 aa, and 1,600 of the 2,176 exported text symbols that start with an underscore begin with the frame prologue `push ebp; mov ebp,esp`.

Inferred, not read: that the boot ROM copies this image to 0x108000 before running it. The image shows only that it is linked there. How it is loaded is outside the image.

## The symbol table (established, 6.5.30)

All 7,907 records parse; every name offset lies inside the name table and ends in a NUL.

| Kind | Records | Notes |
|---|---|---|
| text | 5,078 | 2,233 global, 1,358 local routines and labels, 1,487 position markers |
| data | 752 | 367 global, 174 local, 211 `.data` markers |
| bss | 461 | 294 global, 167 local |
| abs | 156 | values are small constants (field offsets and the like), not addresses |
| debug (stab) | 1,460 | lsym 620, sline 615, stab type 0x40 52, stsym 35, psym 32, fun 29, lbrac 23, rbrac 23, lcsym 16, gsym 9, so 6 |

Position markers are the names `gcc2_compiled.` (300), `___gnu_compiled_c` (295), `.text` (280), `.data` (211) and 612 records for 611 distinct object-file names (`tmp.o`, `kernel.o`, `tcp_output.o` and so on, some with a build-machine path). They label the start of an object, not a routine; the Ghidra import and the TSV skip them. The count of object names here (611) is a plain count of names ending `.o`; `controllers.md` gives 602 modules, counted another way and not reconciled.

The debug records are sparse (29 function records for 5,078 text symbols), so function boundaries come from the text symbols alone. Symbol sizes are the gap to the next symbol of the same segment: an inference, not recorded in the table.

## Other releases

The same layout holds in every image held; only sizes move.

| Release | MMSC | Size | Text | Data load address | Data | Bss | nlist records |
|---|---|---|---|---|---|---|---|
| 6.4-origin | none | 995,050 | 0xa19b0 | 0x1a99b0 | 0x28b68 | 0x1c6b8 | 7,803 |
| 6.4 | none | 998,378 | 0xa2450 | 0x1aa450 | 0x28b88 | 0x1c8d0 | 7,827 |
| 6.5 | none | 1,016,714 | 0xa6528 | 0x1ae528 | 0x28bb8 | 0x1c9e8 | 7,886 |
| 6.5.2 | none | 1,005,559 | 0xa3e60 | 0x1abe60 | 0x28b48 | 0x1c9d8 | 7,847 |
| 6.5.5 | 2.1 | 1,023,103 | 0xa7bb8 | 0x1afbb8 | 0x28c00 | 0x1ca50 | 7,906 |
| 6.5.6 | 2.2 | 1,023,060 | 0xa7bc8 | 0x1afbc8 | 0x28c00 | 0x1ca60 | 7,906 |
| 6.5.7, 6.5.9, 6.5.10, 6.5.30 | 2.3 | 1,023,075 | 0xa7bd8 | 0x1afbd8 | 0x28c00 | 0x1e198 | 7,907 |

"none" means the image carries no version string. In each image the self-check passes: the segment-closing symbols equal the computed ends and the data-pointer measurement picks the contiguous base.

## Tools

The parser is `retools/tools/sysco/mmsc_aout.py` (in the sgi-retools submodule). Its unit test is `retools/tools/sysco/test_mmsc_aout.py`; with `MMSC_IMAGE` set to an image it also runs the real-image checks.

    tools/re-run python3 retools/tools/sysco/mmsc_aout.py IMAGE --segments --check
    tools/re-run python3 retools/tools/sysco/mmsc_aout.py IMAGE --disasm _cmdVER --count 20
    tools/re-run python3 retools/tools/sysco/mmsc_aout.py IMAGE --tsv OUT.tsv

For Ghidra, make the table as above, then import the image as a raw binary with language `x86:LE:32:default` and run `retools/ghidra/import/mmsc-aout/ghidra_scripts/MmscAoutImport.java` as a pre-script with the image path and the table as arguments. The script replaces the raw block with the three segments, labels 4,591 symbols and creates 3,328 functions before analysis. `retools/ghidra/import/mmsc-aout/tests/run-ghidra.sh` runs this headless, and `check.py` there compares the result with the parser: blocks, a function at every global text symbol and the symbol count. It passed on 6.5.30 (3 blocks, 3,589 functions after analysis, 4,591 imported symbols, no failures). Ghidra's decompiler logged two warnings during analysis (an unreadable address and a flow into unmapped memory, in two routines); their cause was not investigated.
