# Tools

Everything runs in a container built from `Containerfile`, so the host needs only Podman, git and python3.

    tools/re-run python3 scripts/inventory.py [--update-root-manifest]
    tools/re-run python3 scripts/sysco_tables.py
    tools/re-run python3 scripts/check_claims.py --status all
    tools/re-run --ghidra-export ROOT_FILE ...    # Ghidra decompile export, see re-run's header

- `inventory.py` is shared with the sibling repositories. Here it reads the stripped MIPS N32 host tools (`flashsc`, `l1cmd`, `flashpbay`) and checks `root/` against `scripts/inventory-root.sha256`. Its configuration is `scripts/inventory-config.json`.
- `sysco_tables.py` is specific to this repository. It cuts the L1 containers into packages, finds candidate routine entries in each ColdFire body, joins the curated tables in `docs/l1/` to the images, checks their addresses and byte patterns, and writes `docs/inventory/` and the firmware block of `docs/coverage.md`.
- `check_claims.py` compares documented claims with the images; the pre-push hook runs it on changed documents.
- `re-run --ghidra` uses `Containerfile.ghidra` (ColdFire language `68000:BE:32:Coldfire`, PowerPC `PowerPC:BE:32:default`, ARM `ARM:LE:32:v4`). The export helper expects ELF input, so raw firmware bodies need a project import with an explicit language and base address first.
- `check-shared` verifies the pin of the `retools/` submodule that holds the shared files; see its header.
- `README-analysis.md` describes the shared analysis tools, which target MIPS ELF objects and kernels and do not apply to the controller images.
