# Controller firmware versions

Every L1 and L2 firmware set found so far. All of it ships in the inst product `field_diags_sysco`; a set is the pair `l1.bin` and `l2.bin` plus the tools. Versions are the strings inside the images; the release notes of a patch name them differently in places, as noted.

Marks: established means measured directly from the file; inferred means a reading.

## Sets

| Source | L1 version | L1 file | L2 version | L2 file |
|---|---|---|---|---|
| IRIX 6.5.22 Overlays (3 of 3), `field_diags_sysco` | 1.24.8 | 3,723,916 bytes, sha256 `5b90997945dabdc099162d5237e302472f5c3c80a808a1de5abb246c290a963b`, 3 packages | 1.24.1 | 2,879,048 bytes, sha256 `74602ffa5bbd71791f70320459ad32626a78350e755be006db66bc5eb92c2ff6` |
| patch SG0005762 | 1.32.15 | 7,406,448 bytes, sha256 `2399ea17dfeb2af20fae4286f0cfeeffae6d3d9f637fcc02e880e5689f662656`, 4 packages | 1.32.4 | 2,915,908 bytes, sha256 `7956e9bdb89f38c22b5e778d6216fb8cd5bd9f4464ad2bcaf5dc1def34605154` |
| IRIX 6.5.30 Overlays, `field_diags_sysco` | 1.44.0 | 15,988,636 bytes, sha256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`, 7 packages | 1.44.0 | 2,924,652 bytes, sha256 `6bcfda07a24cb4d56850922dab17a98223abcd14221d307e3a8c3778434e8761` |
| patch SG0007149 | 1.48.1 (the legacy 2 MB ColdFire image reports 1.48.0) | 16,027,376 bytes, sha256 `536bfa4c6d0240b7afbc25bef94e160bdf1a675016c35e68ec52e1bfb1b451f9`, 7 packages | 1.48.0 | 2,993,732 bytes, sha256 `10258789f1cb5dd49d024870ff1d0f14a879a1c3b9aaf75610c51aa5af6f9e4f` |

The hash of every file, including the tools and the CPLD vectors, is in [../scripts/inventory-root.sha256](../scripts/inventory-root.sha256); [inventory/images.tsv](inventory/images.tsv) adds the kind and detail of each.

Established: the version strings above are the most frequent `1.NN.N` strings in each file. The SG0005762 release note says L1 1.32.14 and L2 1.32.4; the image's own string is 1.32.15. The SG0007149 release note says it carries System Controller software 1.7 to 1.16 compatibility fixes for IRIX 6.5.24 to 6.5.30 and does not name its firmware version; the images say 1.48.x.

Patch SG0005762 replaces earlier patches, and SG0007149 replaces SG0005762 (its release note). Patch SG0007114 carries no controller firmware (its subsystem is `ws_field_diags_irix_sw`, host diagnostics).

Other `field_diags_sysco` files on the same media:

| Item | 6.5.22 | 6.5.30 | SG0005762 | SG0007149 |
|---|---|---|---|---|
| `pbay.bin` (power-bay controller image) | 5,888 bytes, sha256 `7428aaf3d5beffcb23fb56aa9f82c013a6f3ebbb5f41175a61781bfcf366f9c6` | same | same | same |
| CPLD vector files (`.svf`) | `ioc4.svf`, `is1.svf` | `dm7`, `io9`, `io10`, `is2`, `is_cpld`, `sgc` | none | `dm7` |
| `flashsc` | 291,124 bytes | 419,780 bytes | 312,000 bytes | 407,920 bytes |

## Container layout

Every `l1.bin` and `l2.bin` starts with the ASCII marker `ESTFBINR`.

L1 (established): a big-endian package count at offset `0x2c` and, from `0x30`, that many big-endian offset and length pairs. The packages are contiguous and the last ends at the file size. Each package starts with `ESTF`. A ColdFire package has a 44-byte header whose words at `0x08` and `0x0c` are the load and last addresses (`0xffe00000` and `0xffeefd5f` for the Origin 300 image) and a body that is flat code. A Linux package starts with a similar header with load address zero and holds a gzip kernel followed by a little-endian cramfs.

[inventory/packages.tsv](inventory/packages.tsv) lists every package of every set with offset, length, hash, family, product label, version and build stamp. The product labels in the firmware are `Base 1MB`, `Fuel/PE 1MB` (`Fuel/PE/O300 1MB` from 1.44.0), `MIPS 2MB`, `Legacy 2MB`, an unlabelled `2MB`, `Legacy Linux L1` and `Linux L1`. The 6.5.22 set has three ColdFire packages, SG0005762 three ColdFire and one Linux, and the 1.44.0 and 1.48 sets five ColdFire and two Linux.

L2 (established): the `ESTFBINR` header, then the string `SGI L2 Controller ROM 0.01` at `0x34`, a boot loader (string `SGI L2 Controller Boot Loader` at `0x207c`), a gzip stream at `0x4a218` holding a PowerPC Linux 2.4.7 kernel (banner `2.4.7-sgil2`), and a big-endian cramfs. The same layout holds in all four sets; only the sizes and the kernel build date change (2003-08-19, 2004-10-12, 2006-05-23, 2007-01-12, in the order of the table).
