# CORE-M01-LEASE-LIVE-001: bound-provider live admission correctness correction

Status: CORRECTION_CANDIDATE / NO_CI_ASSERTION_UNTIL_EXACT_HEAD  
Work Order: https://github.com/KayzenRoot/core/issues/163  
Exact initial protected main: `09cc3ed6c7d01c483f78a725bdbc81e88702c36b`

## Actual source defect and accepted authority

Accepted frozen M01 spec `docs/modules/M01-CORE-RUNTIME-LIFECYCLE.md` Git blob `1bbb4e1b3d02e775b41c77259ce44f1ad9afb287` states quarantined providers cannot receive **new** leases while existing coherent leases may drain to a safe boundary or be explicitly revoked by authorized policy. Existing `crates/core-registry/src/lib.rs` Git blob `91bfa3d02789cfb36fdf8bdeddedc5991d5f6e14` cloned the live descriptor at bind time; later `quarantine_provider`/`set_provider_health` modified only registered provider state. `acquire_lease_with_generation` checked only the old binding's cloned health and could issue a new lease after actual quarantine/degradation.

The bounded correction rechecks the live registered provider **inside the same exclusive registry lock used to create a lease** and refuses a missing/mismatched identity, unhealthy, unready or quarantined descriptor before any lease insertion; no second registry or new public API. Three targeted inline Rust tests exercise quarantine with an active prior lease and cloned registry, degraded/unknown/unavailable/unready state and restoration, and mismatched/missing identity. Existing `validate_lease` stays unchanged so previously issued leases retain established safe-boundary/revoke semantics; the patch does not add automatic provider substitution or claim that runtime health policy is fully implemented.

## Independent assurance still required

The correction must pass a **fresh exact-head M01-mode** CI with actual Linux/Windows M01 fmt/Clippy/tests, M01 bounded fuzz, Ubuntu dependency/advisory/SBOM/PRB, Governance Python and required no-op M02/M03 status contexts; then a scoped logical owner self-audit `NOT INDEPENDENT` with no unresolved HIGH/CRITICAL/threads, protected squash merge and **new FULL main-push 11/11** including real M01/M02/M03 Linux/Windows and all bounded fuzz. No local-only test result, remote host identity or external consumer attestation is asserted by merely committing code.

M04 external previous-V1 inventory [#111](https://github.com/KayzenRoot/core/issues/111) UNKNOWN, draft source PR #118 and unmerged product #106 BLOCKED with 23 global EV-M04 PENDING. Real owner-local LEGACY_PROVIDER/Codex #4 remains OPEN. Frozen M04 Context Lock and all other project/code modules untouched.
