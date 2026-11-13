# Tools

Everything runs in a container built from `retools/tools/Containerfile`, so the host needs only Podman, git and python3.

    retools/tools/re-run python3 retools/tools/inventory.py [--update-root-manifest]
    retools/tools/re-run python3 retools/tools/sysco/sysco_tables.py
    retools/tools/re-run --ghidra-export ROOT_FILE ...    # Ghidra decompile export, see re-run's header

- `inventory.py` is shared with the sibling repositories. Here it reads the stripped MIPS N32 host tools (`flashsc`, `l1cmd`, `flashpbay`) and checks `root/` against `tools/config/inventory-root.sha256`. Its configuration is `tools/config/inventory-config.json`.
- `sysco_tables.py` is specific to this repository. It cuts the L1 containers into packages, finds candidate routine entries in each ColdFire body, joins the curated tables in `software/firmware/l1/` to the images, checks their addresses and byte patterns, and writes `evidence/inventory/` and the firmware block of `evidence/coverage.md`.
- `re-run --ghidra` uses `retools/tools/Containerfile.ghidra` (ColdFire language `68000:BE:32:Coldfire`, PowerPC `PowerPC:BE:32:default`, ARM `ARM:LE:32:v4`). The export helper expects ELF input, so raw firmware bodies need a project import with an explicit language and base address first.
- `retools/tools/README-analysis.md` describes the shared analysis tools, which target MIPS ELF objects and kernels and do not apply to the controller images.
