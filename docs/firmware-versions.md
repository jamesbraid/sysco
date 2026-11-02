# Controller firmware versions

Every L1 and L2 firmware set found so far. All of it ships in the inst product `field_diags_sysco`; a set is the pair `l1.bin` and `l2.bin` plus the tools. Versions are the strings inside the images; the release notes of a patch, and of 6.5.27 to 6.5.29, name them differently in places, as noted.

Marks: established means measured directly from the file; inferred means a reading.

## Sets

| Source | L1 version | L1 file | L2 version | L2 file |
|---|---|---|---|---|
| IRIX 6.5.20 Overlays (4 of 4), `field_diags_sysco` | 1.20.6 | 2,752,144 bytes, sha256 `adc39a287c525f1d6ac9bb8812671546f26c9029809d3bd487f5f8ab7318f94d`, 2 packages | 1.20.1 | 2,870,856 bytes, sha256 `5a55427a10c658ac6f6c64aa677647511bd24bc574ca7b6abce722162738c7ec` |
| IRIX 6.5.21 Overlays (4 of 4), `field_diags_sysco` | 1.22.2 | 2,766,272 bytes, sha256 `d5aaba306f71d9bfe0aa83d5029e14f2b9cf7485a4c0c689297fc4f71d136fbb`, 2 packages | 1.22.0 | 2,870,852 bytes, sha256 `21dddd3a7d15fd314b56cd86c812fdf9ba698738c01ed73c80e37b249951939b` |
| IRIX 6.5.22 Overlays (3 of 3), `field_diags_sysco` | 1.24.8 | 3,723,916 bytes, sha256 `5b90997945dabdc099162d5237e302472f5c3c80a808a1de5abb246c290a963b`, 3 packages | 1.24.1 | 2,879,048 bytes, sha256 `74602ffa5bbd71791f70320459ad32626a78350e755be006db66bc5eb92c2ff6` |
| IRIX 6.5.23 Overlays (3 of 3), `field_diags_sysco` | 1.26.5 | 3,763,312 bytes, sha256 `15fe4284aa152ba30d1a8d9c67f63ee2acf1d19b0653068befc89146c12187f1`, 3 packages | 1.26.0 | 2,907,716 bytes, sha256 `7dea788c8155e5dcea8bdbd6de52f9aedc6fce85982571cb0c885c4859792320` |
| IRIX 6.5.24 Overlays (3 of 3), `field_diags_sysco` | 1.28.3 | 3,759,416 bytes, sha256 `12e1052718a311de3dfd6424820e680fbaedace11a249c463a1ff5bbca2236c1`, 3 packages | 1.28.1 | 2,907,716 bytes, sha256 `896b744032fa6b66513a43dd8e667abd6620692a7ec5e59e51c38a31001a1c55` |
| IRIX 6.5.25 Overlays (3 of 3), `field_diags_sysco` | 1.30.6 | 3,817,584 bytes, sha256 `239378307da84254c56ed7499818aa372fe0aabd54e668da931da34f13ac620d`, 3 packages | 1.30.0 | 2,911,812 bytes, sha256 `d7e9188f009ba4ded732c99cc39d796c5a8db52bdfb2b7b4bfa95bb274fa26cc` |
| IRIX 6.5.26 Overlays (3 of 3), `field_diags_sysco` | 1.32.6 | 3,846,136 bytes, sha256 `1fdcc3409f4e0633335d9b51d506818ce971dacbe6ababe1d4fbcbd52855e3dc`, 3 packages | 1.32.2 | 2,915,908 bytes, sha256 `23d228888cd670fa2e60f6b2ad75ccd88604218dee1b483044f22e7fb0f9d304` |
| IRIX 6.5.27 Overlays (3 of 3), `field_diags_sysco` | 1.34.2 | 7,523,600 bytes, sha256 `45c872a661d4a4ad20a4cb482fe2aee921ee9ae52784382be125b3d7b7836572`, 4 packages, one Linux | 1.34.0 | 2,928,200 bytes, sha256 `ff72f9e7537fb2856e557bf63dd6dd08a2b6f2af755926278a433631c608d74a` |
| IRIX 6.5.28 Overlays (3 of 3), `field_diags_sysco` | 1.38.4 | 9,293,972 bytes, sha256 `e5b65e5f28d7fba6013a8618f55c34bed7f60f7c6f46d7e9c5b8b7b2eabbe7b3`, 5 packages, one Linux | 1.38.8 | 2,940,916 bytes, sha256 `6f6245244a8096448b5a4e49f7c0c66e806a4acd19dd1528b7e1f2621776f242` |
| IRIX 6.5.29 Overlays (3 of 3), `field_diags_sysco` | 1.40.5 | 9,295,164 bytes, sha256 `80f4a616601d187bc3839a2758f2e7f2d0db962ad7fd8f2d96d41bcd15bdf88c`, 5 packages, one Linux | 1.40.1 | 2,940,912 bytes, sha256 `b3e3ac9755b2f86de7b3859e425d68d332873f582d4f5f9f486c84ea4a6f9197` |
| patch SG0005762 | 1.32.15 | 7,406,448 bytes, sha256 `2399ea17dfeb2af20fae4286f0cfeeffae6d3d9f637fcc02e880e5689f662656`, 4 packages | 1.32.4 | 2,915,908 bytes, sha256 `7956e9bdb89f38c22b5e778d6216fb8cd5bd9f4464ad2bcaf5dc1def34605154` |
| IRIX 6.5.30 Overlays, `field_diags_sysco` | 1.44.0 | 15,988,636 bytes, sha256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`, 7 packages | 1.44.0 | 2,924,652 bytes, sha256 `6bcfda07a24cb4d56850922dab17a98223abcd14221d307e3a8c3778434e8761` |
| patch SG0007149 | 1.48.1 (the legacy 2 MB ColdFire image reports 1.48.0) | 16,027,376 bytes, sha256 `536bfa4c6d0240b7afbc25bef94e160bdf1a675016c35e68ec52e1bfb1b451f9`, 7 packages | 1.48.0 | 2,993,732 bytes, sha256 `10258789f1cb5dd49d024870ff1d0f14a879a1c3b9aaf75610c51aa5af6f9e4f` |
| SGI Internal Support Tools 3.24 for Linux (L3 and SGIconsole disc), `snxsc_firmware` 1.62.1-1 | 1.62.1 | 13,258,744 bytes, sha256 `b4b5634949a93f05ad6eb24462e6cd7f10104dd42ff5c2ae8616dd5d87697790`, 5 packages (three ColdFire, two Linux) | 1.62.0 | 2,994,152 bytes, sha256 `e21bf0b0ea0463ff1d0981da401e1ebac5489087217a69838671b44007646d27` |

The hash of every file, including the tools and the JTAG vectors, is in [../scripts/inventory-root.sha256](../scripts/inventory-root.sha256); [inventory/images.tsv](inventory/images.tsv) adds the kind and detail of each.

Established: the version strings above are the most frequent `1.NN.N` strings in each file. The SG0005762 release note says L1 1.32.14 and L2 1.32.4; the image's own string is 1.32.15. The identification section of the 6.5.27, 6.5.28 and 6.5.29 release notes names L1 and L2 1.6.0. Neither image holds that string (a byte search of each L1 and L2 finds none); the images carry 1.34.x, 1.38.x and 1.40.x, and every package inside each L1 reports the same version. The notes of 6.5.20 to 6.5.26 name the versions the images carry. The SG0007149 release note says it carries System Controller software 1.7 to 1.16 compatibility fixes for IRIX 6.5.24 to 6.5.30 and does not name its firmware version; the images say 1.48.x.

Patch SG0005762 replaces earlier patches, and SG0007149 replaces SG0005762 (its release note). The 6.5.20 to 6.5.29 sets are measured from the `field_diags_sysco` product on each release's last Overlays disc (4 of 4 for 6.5.20 and 6.5.21, 3 of 3 for the rest). The newest set held is the 3.24 disc, a Linux product for the L3 controller, SGIconsole and Itanium Altix servers: its RPM also ships `l1_mips-1.48.1.bin`, `l2_mips-1.48.0.bin` (byte-identical to patch SG0007149) and the 1.62 images. Its L1 container has three ColdFire packages (`NL3R 1MB`, `Legacy 2MB`, `2MB`) and two Linux packages (`Legacy Linux L1`, `Linux L1`); the L2 kernel was built 2008-10-27. The `pbay-00.18.bin` in the RPM is the same 5,888-byte file as every `pbay.bin`, so the RPM's own file name labels that file power-bay firmware 00.18 (a name, not a string read from the image). Patch SG0007114 carries no controller firmware (its subsystem is `ws_field_diags_irix_sw`, host diagnostics).

Other `field_diags_sysco` files on the same media:

| Item | 6.5.22 | 6.5.30 | SG0005762 | SG0007149 |
|---|---|---|---|---|
| `pbay.bin` (power-bay controller image) | 5,888 bytes, sha256 `7428aaf3d5beffcb23fb56aa9f82c013a6f3ebbb5f41175a61781bfcf366f9c6` | same | same | same |
| SVF JTAG vector files (`.svf`) | `ioc4.svf`, `is1.svf` | `dm7`, `io9`, `io10`, `is2`, `is_cpld`, `sgc` | none | `dm7` |
| `flashsc` | 291,124 bytes | 419,780 bytes | 312,000 bytes | 407,920 bytes |

The SVF vector files differ by set. 6.5.20 has none and no `flashsvf`. 6.5.21 has `ioc4.svf` only, byte-identical to the 6.5.22 copy. 6.5.23 has `io9.svf`, `io10.svf` and `is1.svf`. 6.5.24 has `dm7.svf`, `io9.svf`, `io10.svf`, `is1.svf` and `sgc.svf`. 6.5.25 has `dm7.svf`, `io9.svf`, `io10.svf`, `is2.svf` and `sgc.svf`. 6.5.26 to 6.5.29 add `is_cpld.svf` to those; 6.5.26 and 6.5.27 hold a 58,643-byte `is_cpld.svf` where 6.5.28 and 6.5.29 hold 56,946 bytes. `pbay.bin` is the same 5,888-byte file in every set. The four CD-IST 3.24 vectors are SVF text with Xilinx IDCODE reads; their exact IDs and file labels are in [controllers.md](controllers.md). The IRIX-set vectors are inventoried, and their IDCODE reads and parts are in [controllers.md](controllers.md). The rest of each file is not read.

## Container layout

Every `l1.bin` and `l2.bin` starts with the ASCII marker `ESTFBINR`.

L1 (established): a big-endian package count at offset `0x2c` and, from `0x30`, that many big-endian offset and length pairs. The packages are contiguous and the last ends at the file size. Each package starts with `ESTF`. A ColdFire package has a 44-byte header whose words at `0x08` and `0x0c` are the load and last addresses (`0xffe00000` and `0xffeefd5f` for the Origin 300 image) and a body that is flat code. A Linux package starts with a similar header with load address zero and holds a gzip kernel followed by a little-endian cramfs.

[inventory/packages.tsv](inventory/packages.tsv) lists every package of every set with offset, length, hash, family, product label, version and build stamp. The product labels in the firmware are `Base 1MB`, `Fuel/PE 1MB` (`Fuel/PE/O300 1MB` from 1.38.4), `MIPS 2MB`, `Legacy 2MB`, an unlabelled `2MB`, `Legacy Linux L1` and `Linux L1`. The 6.5.20 and 6.5.21 sets have two ColdFire packages, labelled `1MB image` and `2MB image`. The 6.5.23 to 6.5.26 sets have three ColdFire packages with the 6.5.22 labels. The 6.5.27 set adds one Linux package (kernel 2.4.25-vrs2-swl8c-sgil1), and the 6.5.28 and 6.5.29 sets have four ColdFire packages (`Base 1MB`, `Fuel/PE/O300 1MB`, `MIPS 2MB` and `2MB`) and the same Linux package. The 6.5.22 set has three ColdFire packages, SG0005762 three ColdFire and one Linux, and the 1.44.0 and 1.48 sets five ColdFire and two Linux.

L2 (established): the `ESTFBINR` header, then the string `SGI L2 Controller ROM 0.01` at `0x34`, a boot loader (string `SGI L2 Controller Boot Loader` at `0x207c`), a gzip stream at `0x4a218` holding a PowerPC Linux 2.4.7 kernel (banner `2.4.7-sgil2`), and a big-endian cramfs. Every L2 image measured (the thirteen sets above, including both patches) has this layout and the banner `2.4.7-sgil2`; only the sizes and the kernel build date change. Kernel build dates in release order: 6.5.20 2003-02-25, 6.5.21 2003-05-21, 6.5.22 2003-08-19, 6.5.23 2003-11-21, 6.5.24 2004-02-19, 6.5.25 2004-05-13, 6.5.26 2004-09-13, SG0005762 2004-10-12, 6.5.27 2004-12-14, 6.5.28 2005-06-21, 6.5.29 2005-08-25, 6.5.30 2006-05-23, SG0007149 2007-01-12. The carved tree, the kernel, the offsets and the daemon are read in [l2/](l2/README.md).
