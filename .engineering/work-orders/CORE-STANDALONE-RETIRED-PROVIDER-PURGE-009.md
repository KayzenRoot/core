# CORE-STANDALONE-RETIRED-PROVIDER-PURGE-009

Status: `EXECUTION_ACTIVE`  
Issue: #191  
Parent: #172  
Exact authorized base: `9d03bba3bf35b242bf0468f5fa7737980364c8e5`  
Branch: `governance/core-retired-provider-purge-009`  
GEF: `v1.1.1`  
Assurance: `ELEVATED`

## Objective

Remove every current-tree textual/path residue of the retired provider while preserving historical provenance in Git history.

## Required end state

- zero forbidden-provider token in Git-tracked path names;
- zero forbidden-provider token, case-insensitive, in UTF-8 Git-tracked content;
- provider-neutral runtime/config/CLI/bench naming;
- governance still fails closed against reintroduction without storing the forbidden literal contiguously;
- M04 remains `STALE / BLOCKED_RE_ADMISSION` with product implementation unauthorized;
- #111 remains `UNKNOWN/BLOCKING`.

## Authorized scope

Repository-wide sanitation of current tracked text/path residue, including docs, evidence, tests, decisions, work orders, logs, JSON, fuzz corpus, runtime metadata/config tests and stale-lock fingerprints required for internal coherence.

## Non-authorized

- Git history rewrite;
- force push;
- branch/ruleset weakening;
- M04 product re-admission;
- #111 resolution;
- M05/M06/M23 product implementation.

## TDD

1. RED: repository-wide zero-residue test fails against the current base.
2. GREEN: purge/sanitize current tree and preserve functional safeguards.
3. Focused regression + governance + FULL exact-head.
4. Review and protected promotion only after all gates.

## STOP

No merge until zero-residue proof, exact-head FULL success, zero unresolved HIGH/CRITICAL, zero unresolved threads and owner self-audit. Separate actual protected-main FULL is required before issue close.
