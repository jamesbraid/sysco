# Inventory tables

Generated; do not edit. `retools/tools/inventory.py` writes the host-tool tables, `retools/tools/sysco/sysco_tables.py` the firmware tables. A run of both leaves `git status --short docs` empty when `root/` matches `tools/config/inventory-root.sha256`.

## Statuses

- `understood`: a curated row (software/firmware/l1/*.tsv) gives the item a grade (`established` or `inferred`), a role stated in at least a few words and a locator, and any byte check passes.
- `located`: the item's address, offset or hash is cited in a note under `docs/`.
- `undocumented`: neither.

## Firmware tables (`retools/tools/sysco/sysco_tables.py`)

| Table | One row per | Columns |
|---|---|---|
| `images.tsv` | distinct file in `root/` (byte-identical copies folded into `copies`) | path, release, kind, architecture, version, bytes, sha256, detail, status |
| `packages.tsv` | package inside each `l1.bin` | key `RELEASE:l1#INDEX`, offset, length, sha256, family (ColdFire or ARM Linux), product label, version, build stamp, detail (load and last address, reset stack pointer and entry, body hash; or kernel release and hash) |
| `firmware-functions.tsv` | candidate routine entry in each distinct ColdFire body | image key, address, evidence (`link.w a6` prologue, absolute `jsr` target), name and grade from `software/firmware/l1/functions.tsv` |
| `request-codes.tsv` | request code per task | task, code, handler address, name, grade, whether the handler lies in the body |
| `io-registers.tsv` | register or device offset the code touches | device, window base, offset, width, name, access, grade, evidence |
| `protocols.tsv` | protocol element (descriptor field, framing rule, route) | protocol, element, value, grade, evidence, byte check result |
| `firmware-summary.tsv` | table | row counts by status |
| `arm-linux-files.tsv` | entry of each CramFS in the Linux L1 packages (directories, files, symlinks, device nodes) | package key, path, type, mode, bytes, sha256 (files and symlinks), target (symlink target, or major:minor for device nodes) |

Candidate routine entries are found by byte pattern at even offsets (a `link.w a6` prologue, or the operand of an absolute `jsr`), so some are data. A candidate is a place to look, not a function count.

## Host-tool tables (`retools/tools/inventory.py`)

`binaries.tsv` and `functions.tsv` (ELF rows for `flashsc`, `l1cmd` and `flashpbay`), and the empty `mmio`, `tokens`, `ioctls`, `ucode` and `spec-registers` tables the shared script always writes. The host tools are stripped MIPS N32 executables; function rows come from their dynamic symbols. `flashsvf` and `l2cmd` have section headers the ELF reader rejects and are not in scope yet.
