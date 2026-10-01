# CORE-M04-INV-CLI-002: fail-closed local inventory CLI argument redaction

**Status:** CORRECTION_CANDIDATE / NO_OWNER_INVENTORY_FACTS  
**Work Order:** https://github.com/KayzenRoot/core/issues/151  
**Protected-main initial base:** `27d208f45b1eaa2ff54b97f75d83293ab5e3ea64`

## Exact source bindings and bounded security defect

Prior `scripts/m04_legacy_inventory.py` Git blob `94da77a247ed198c50866fdb66db38df9af6067c`, previously protected-merged under issue #130. Its `main(argv)` used default `argparse.ArgumentParser`, and `parse_args(argv)` ran outside the redacted file-read exception handler. Unknown, malformed, missing or contradictory CLI args could therefore cause argparse to echo an owner-supplied local private filename or accidentally pasted secret in stderr. The valid JSON assessment was already privacy-bounded and UNKNOWN-by-default, and no release/real external use may be inferred from running it.

Proposed minimal correction: `RedactedInventoryParser.error` raises a static typed `InvalidInventory`. `main` catches that before opening any file and prints a fixed invalid reason/exit 4 with no private argv; keep neutral --template, --inventory, 32 KiB input cap, duplicate-key/type/date checks and all 0/2/3/4 assessment meanings unchanged. Tests use synthetic canary filename/secret for unexpected/mutually exclusive/missing/positional args and assert no leakage or stderr. The runbook documents the non-attestation limit.

## Exact-head assurance to complete

Fresh full CI required because a script and tests changed; separately record exact new head, all required successful status contexts, actual Governance Python tests, M01/M02/M03 Linux/Windows/supply-chain/fuzz, bounded logical GEF owner self-audit **NOT INDEPENDENT** with zero unresolved HIGH/CRITICAL/review threads, protected squash merge and independent new **FULL 11/11 main-push** success before closing this correction-only Work Order. No owner machine was scanned, no external M04 consumers disproven and no local LEGACY_PROVIDER/Codex calls performed. [External M04 gate #111](https://github.com/KayzenRoot/core/issues/111) stays OPEN, with PR #118 DRAFT and #106 unmerged; [LEGACY_PROVIDER local gate #4](https://github.com/KayzenRoot/core/issues/4) stays OPEN. Frozen M04 sources and active lock remain unchanged.
