# CORE-M06-DISCOVERY-R1: source-bound capability negotiation baseline

Status: NON_AUTHORITATIVE_R1_CANDIDATE / NO_IMPLEMENTATION
Work Order: https://github.com/KayzenRoot/core/issues/154
Initial exact protected-main: 7c68b829541ddbbc97a1a957f64883e23c26a94a

## Actual canonical/source inputs inspected before drafting

- Master Module Map, `docs/modules/00-MASTER-MODULE-MAP.md`, blob `df2736686ada12cd5cd4fcdc5cd933c84d91ab25`: M06 is Capability Negotiation, planning order M04 -> M05 -> M06, M05–M24 discovery-only until actual planning freezes.
- Existing accepted M01 `crates/core-registry/src/lib.rs`, blob `91bfa3d02789cfb36fdf8bdeddedc5991d5f6e14`: deterministic required feature/contract/authority/assurance/trust/quality/origin/health/latency/cost filtering, owner-dependent origin preference, existing authoritative `resolve/bind/substitute/acquire_lease/validate_lease`, generation coherence and revocation. **This cannot be duplicated as a new M06 registry or second incompatible selector**.
- Existing accepted M01 `crates/core-contracts/src/lib.rs`, blob `0cd1d56a4ac761ff1c8b761cde0e0d19dfa76bc9`: `SemVer`, `CapabilityRequirement`, `CapabilityProviderDescriptor`, `ProviderOrigin`, `CapabilityOwnership`, `RuntimeGeneration` and `CapabilityLease`.
- Non-authoritative M05 R1–R4 `docs/modules/M05-HOST-ADAPTER-FABRIC.md`, blob `222dace08091637e0192eb0259ba13109e418ba7`: future host transport observations only, not a finalized public M05 API or proof of authenticated host/tool authority.
- CORE Modular Delivery Model `docs/engineering/CORE-MODULAR-DELIVERY-MODEL.md`, blob `0265293a529f9850cc63c72e8aedbe617951cafd`, and Source Hierarchy `.engineering/SOURCE-HIERARCHY.md`, blob `19e138183baaf633917ee32022db536dfc582556`.

## Honest bounded discovery and STOP

This Round 1 **does not implement or approve** M06, new provider selection, multi-provider atomic binding, any M01 contract modification, M05 final host DTO, external adapter, runtime/network permission, final numerical budgets or actual benchmark. It distinguishes *untrusted host-declared* feature/origin metadata from separately independently verified M11/M22 origin-policy evidence, delegates binding and leases to accepted M01, and preserves M10/M11/M12 actual effect authority. The 14 EV-M06-D* identifiers are prospective test ideas **ALL PENDING**, not a frozen AEG or current executable evidence.

M04 prior-V1 externally exported binary/API/retained journal/snapshot/downstream status remains UNKNOWN under [issue #111](https://github.com/KayzenRoot/core/issues/111); original M04 contract source PR #118 DRAFT and product #106 unmerged, all 23 global EV-M04 pending. LEGACY_PROVIDER owner-local Docker/index/MCP plus distinct Codex client proof [issue #4](https://github.com/KayzenRoot/core/issues/4) remains OPEN. No external owner inventory or live LEGACY_PROVIDER evidence was accessed.

This candidate must pass fresh exact-head docs-only Governance and successful required status contexts, logical owner audit `NOT INDEPENDENT` with zero unresolved HIGH/CRITICAL and review threads, protected squash merge and **independent actual FULL 11/11 main-push validation** before discovery-only Work Order closeout. Actual future M06 final freeze and separate execution-admission delta remain mandatory.
