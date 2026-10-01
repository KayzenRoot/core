# CORE-D-209 — Current-tree retired-provider sanitation

Date: 2026-10-01  
Status: ADOPTED_ON_PROTECTED_MAIN_PROMOTION_ONLY  
Work Order: #191; parent #172.  
Exact authorized base: `9d03bba3bf35b242bf0468f5fa7737980364c8e5`.  
GEF basis: `v1.1.1`.

## Decision

The current CORE tree must contain zero textual or path residue of the retired provider. Historical provenance remains available through immutable Git history instead of being duplicated byte-for-byte inside current files.

This decision supersedes only the in-tree archive-retention mechanics of CORE-D-206, CORE-D-207 and CORE-D-208. Their standalone authority, module-state decisions, safety constraints and provider-neutral architecture remain in force.

Current runtime, configuration, CLI, benchmark, documentation and governance names must be provider-neutral. A repository-wide guard derives the forbidden token without storing it contiguously and fails closed if it reappears in a tracked path or UTF-8 tracked file.

M04 remains `STALE / BLOCKED_RE_ADMISSION`, with `productImplementationAuthorized=false`. Updating stale-lock fingerprints for sanitized canonical sources does not re-admit execution. Issue #111 remains `UNKNOWN/BLOCKING`.

## Provenance

Exact superseded payloads are not destroyed: they remain recoverable in Git history at commits preceding this sanitation increment. The live tree intentionally contains only sanitized current-state copies.

## Promotion

Promotion requires zero-residue proof, exact-head FULL success, zero unresolved HIGH/CRITICAL, zero unresolved review threads, transparent owner self-audit marked `NOT INDEPENDENT`, protected expected-head squash, and a separate successful protected-main FULL.
