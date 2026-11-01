# IRIX files needed for this repository

This lists the proprietary SGI files a reader copies from their own IRIX media or patches into a local `root/` directory. They are not tracked in source control. Paths only: this repository holds no file contents.

All controller firmware ships in one inst product, `field_diags_sysco`, which installs under `/usr/cpu/firmware/sysco` and `/usr/sbin`. `root/` holds one directory per source, each mirroring the install paths: `root/6.5.30/`, `root/6.5.22/` and `root/patches/SGnnnnnnn/`. `scripts/inventory-root.sha256` pins the SHA-256 of every file the tables are measured from.

Files wanted from each source (install path inside the source directory):

```files
/usr/cpu/firmware/sysco/l1.bin
/usr/cpu/firmware/sysco/l2.bin
/usr/cpu/firmware/sysco/pbay.bin
/usr/cpu/firmware/sysco/*.svf
/usr/sbin/flashsc
/usr/sbin/sysco/flashsc
/usr/sbin/l1cmd
/usr/sbin/l2cmd
/usr/sbin/flashpbay
/usr/sbin/flashsvf
```

## Sources

| Directory in `root/` | Media | Notes |
|---|---|---|
| `6.5.30/` | the retail IRIX 6.5.30 overlays, product `field_diags_sysco` (the `field_diags_sysco.sw` image and its `.idb` on Overlays 3 of 3) | L1 1.44.0, L2 1.44.0. `flashsc` is installed as `/usr/sbin/sysco/flashsc` (this product's idb has no `/usr/sbin/flashsc` link); the `l1cmd`, `l2cmd`, `flashpbay` and `flashsvf` tools sit in `/usr/sbin`. The `.svf` files are CPLD programming vectors, not controller firmware. |
| `6.5.22/` | IRIX 6.5.22 Overlays (3 of 3), product `field_diags_sysco` | L1 1.24.8, L2 1.24.1. `/usr/sbin/flashsc` is a link to `sysco/flashsc`. The only `.svf` files are `ioc4.svf` and `is1.svf`. |
| `patches/SG0005762/` | patch SG0005762, subsystem `patchSG0005762.field_diags_sysco_sw` | L1 1.32.15, L2 1.32.4. The patch's own release note says L1 1.32.14. |
| `patches/SG0007149/` | patch SG0007149, subsystem `patchSG0007149.field_diags_sysco_sw` | L1 1.48.0 and 1.48.1, L2 1.48.0. Replaces SG0005762. |

Patch SG0007114 (`ws_field_diags_irix_sw`) ships host-side diagnostic programs and no controller firmware; nothing from it is needed.

Hashes of the carriers these were read from:

```
d2899e9aac3f6f62a446d881ccc2f214afc181c195272ab0e658ca974528557f  IRIX 6.5.22 Overlays (3 of 3).iso
54b60017f395f824138e6164f7c19f440626d10d23e449a9c55f95f2a4c24473  patch SG0005762 tardist
f73660c9e85b1dbd615856377e8480a4c8a39f76fc86d21cc23533b0787acd29  patch SG0007114 tardist
9f4b8abf97c0903f697f75888ae1363b19e385a85c9d34446973975eecf9930a  patch SG0007149 tardist
```

The 6.5.30 image is `field_diags_sysco.sw`, 21,355,926 bytes; its idb records `l1.bin` at 15,988,636 bytes and `l2.bin` at 2,924,652 bytes, which the extracted files match.

Patches install with the IRIX software manager; a patch image is an `im001` archive described by the `.idb` beside it, the same layout as the media's product images. Unpack a product or patch with any decoder for that layout and keep only the paths above.

Not present, wanted next: the other 6.5.2x and 6.5.3x releases (6.5.20 to 6.5.29 overlay discs and the customer diagnostics discs), which may carry other L1 and L2 versions between 1.24 and 1.44, and any patch newer than SG0007149.

```archives
```

```links
```
