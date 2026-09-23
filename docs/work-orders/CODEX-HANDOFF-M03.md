# CODEX HANDOFF — CORE-WO-M03-001

Status: AUTHORIZATION_ARMED / EFFECTIVE_ONLY_ON_CANONICAL_MAIN_PROMOTION
Increment: CORE-M03-FREEZE-001
Work Order: .engineering/work-orders/CORE-WO-M03-001.md
Context Lock: .engineering/context-locks/CORE-WO-M03-001.json
Evidence Bundle: .engineering/evidence/CORE-WO-M03-001.json
Module: M03 — Work Order Engine
Future execution branch: feat/m03-work-order-engine
Planning base: 786ad33a27d45eb435bc6e63f22174de74b0bb71
Frozen authorized base: ac90b1f48c5551e65ecadace95c59f7f0647062f
Admission increment: CORE-M03-ADMIT-001

## Hard authorization gate

Do not begin product implementation while this admission state exists only on a PR branch. Proceed only after CORE-M03-ADMIT-001 is independently reviewed and promoted to canonical origin/main, and the exact Context Lock on that canonical main is ACTIVE with authorizedBase `ac90b1f48c5551e65ecadace95c59f7f0647062f`, productImplementationAuthorized=true, and authorizationEffectiveOnlyOnCanonicalMain=true. Confirm the lock's Work Order/source blob bindings match the current canonical files. A lock present only on an admission PR, an approval in chat, a stale fingerprint, or a local edit does not satisfy the gate.

If any gate is absent, ambiguous, stale, conflicting, or unknown, STOP and report NOT_AUTHORIZED / STALE. Do not activate or repair the lock yourself.

## First actions after a valid admission

1. Verify the remote is exactly https://github.com/KayzenRoot/core.git; fetch origin and resolve canonical origin/main, branch, HEAD, cleanliness, and the admitted base.
2. Confirm final planning review/promotion and the separate admission delta on canonical main. Verify authorizedBase is an ancestor of the execution head and intervening commits contain governance/admission metadata only.
3. Validate the ACTIVE Context Lock, its Work Order blob SHA, and the exact nine canonical source fingerprints; stop on any mismatch.
4. Create feat/m03-work-order-engine from post-admission canonical main. Do not reuse this planning branch as the product execution branch.
5. Repeat the truthful HIVE v1.0.0 project/checkpoint preflight. Use the SOLO fallback only as permitted by the Work Order, record unresolved CORE context honestly, and never fabricate HIVE evidence.
6. Read canonical sources in Work Order order and run the repository governance validator before Pack A. Save the actual preflight record to docs/evidence/M03-PREFLIGHT.md.
7. Execute Packs A through H in order and stop at each packet gate. Do not enter CALIBRATION_ONLY until Pack G is complete.

## Execution behavior

- Follow CORE-WO-M03-001 and the exact Round 4 file/dependency map. Do not rediscover or redesign settled M03 architecture.
- Preserve the pure synchronous, stateless, value-only, zero-LLM boundary; caller-owned timeouts discard late results.
- Keep M02 workspace/repository truth outside the M03 core. Never mint Context Lock/governance approval or persistence authority.
- Maintain AEG completeness, PCM lossless mandatory-source reconstruction, scope deny precedence, packet DAG constraints, LPC compare-and-set semantics, and non-evergreen admission receipts.
- Do not invent numeric resource values. Apply only the Work Order's bounded calibration delta after measured evidence, then rerun all affected exact-head gates.
- Record each result against the exact candidate head. Preserve valid independent evidence and invalidate evidence whose source/head changed.
- Do not merge, self-approve, promote a checkpoint, release, or tag.

## Final return

Return in Brazilian Portuguese with exact authorized base/final head, lock and source fingerprints, Work Order and branch, files, Pack A–H results, all 23 AC-to-EV mappings, truthful HIVE result, tests/property/adversarial/fuzz/security/supply-chain/SBOM, calibration report and selected/rejected candidates, Windows/Ubuntu results, seven required hosted status contexts, failures/corrections, risks, proposed Checkpoint Delta, PR, and READY_FOR_REVIEW or BLOCKED.

Never return APPROVED. That verdict belongs to the independent governed reviewer. This handoff is armed but does not authorize work until the exact CORE-M03-ADMIT-001 state is present on canonical origin/main and the hard gate above is satisfied.
