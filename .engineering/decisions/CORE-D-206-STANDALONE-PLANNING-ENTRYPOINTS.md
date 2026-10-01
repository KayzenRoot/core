# CORE-D-206 — Standalone planning entrypoints and exact historical provenance

Date: 2026-09-29
Status: ADOPTED_ON_PROTECTED_MAIN_PROMOTION_ONLY
Work Order: #182; parent #172.
Exact source baseline: `5af01f80d1debe737edd0dff5b4fd8afb02b4f9b`.

## Decision

Make the effective Project Overview and Backlog accurately describe independent, headless, Git-canonical CORE with M01/M02/M03 V2 implemented, M04 old admission STALE/BLOCKED pending #111, M05/M06 planning only and M23 Local Context & Evidence Registry a future unimplemented Work Order. Retire obsolete local service validation #4 as CLOSED / NOT_PLANNED; never claim the owner's retired provider was installed, working or uninstalled. For M05/M06, preserve all prior R1–R4 discovery research exactly, but expressly arclegacy_provider former pinned-provider, local runtime, M23 federation and old active-M04-lock clauses as non-operative. Future optional external generic providers require separately admitted identity/protocol/policy proof. No new host API, public DTO, crate, product code or M04 authorization is granted.

## Exact immutable original Git blob provenance

Each current entrypoint embeds its **entire** original source content verbatim after the arclegacy_provider marker, allowing readers and a no-network regression to prove historical records were preserved rather than erased or silently rewritten:

- `docs/project-brain/01-PROJECT-OVERVIEW.md`: prior Git blob `e9142649593ac93588fc99c40bb33f9fd928857f`.
- `docs/project-brain/14-BACKLOG.md`: prior Git blob `3c9a5f0762dec085dd3fca9b053b524650bb66d4`.
- `docs/modules/M05-HOST-ADAPTER-FABRIC.md`: prior Git blob `222dace08091637e0192eb0259ba13109e418ba7`.
- `docs/modules/M06-CAPABILITY-NEGOTIATION.md`: prior Git blob `fc1caaaba959e4f5982d4707885269527f3a3af4`.

Arclegacy_provider delimiter: `## Historical discovery arclegacy_provider (non-operative; exact prior Git blob follows)`.
Current effective text is only **before** that delimiter. Earlier original titles/status and all candidate EV names in the arclegacy_providerd suffix are prior dated context, not current rules, approval or passing tests.

## Non-modification/STOP

This Work Order does NOT modify the nine already bound M04 canonical source blobs, its Work Order, STALE Context Lock, Evidence Bundle, GEF, Rust product, Cargo/lockfile, CI settings or PRs #106/#118. Those remain bound to CORE-D-205 and current exact Git. This decision is a bounded **derived planning document**; it does not mutate the canonical checkpoint/decision ledger in the nine-source M04 lock. New entrypoint/arclegacy_provider-focused regression and governance validation must reject stale current headings, resurrected mandatory external server assumptions and altered arclegacy_provider fingerprints.

Protected exact-head FULL 11/11 on the candidate, inspected complete scoped diff/zero unresolved HIGH-CRITICAL or threads, owner-account audit explicitly NOT INDEPENDENT, no-bypass guarded squash and a **separate genuine FULL new-main push 11/11**, Ubuntu/Windows bounded fuzz/soak/PRB plus supply chain/SBOM are required before closing #182. #172 closes only after a separate inventory of all other active surfaces; #111 remains UNKNOWN/BLOCKING.
