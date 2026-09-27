# CORE-CI-REL-002 — bounded M01/M02 CI fixture maintenance packet

Status: `TEST_ONLY_MAINTENANCE_CANDIDATE / EXACT_HEAD_REVIEW_REQUIRED`
Branch: `fix/ci-rel-002-m01-m02-fixtures`
Authorized canonical baseline: `7046bf204896bbd1de7c746d346a53e183fa21c2`
Related defects: [M02 Linux Git deadline #115](https://github.com/KayzenRoot/core/issues/115), [M01 Windows shutdown timing #112](https://github.com/KayzenRoot/core/issues/112).
Historical precedent: reviewed and merged test-only correction [CORE-CI-REL-001 / PR #89](https://github.com/KayzenRoot/core/pull/89).

## Scope and authority

This bounded CI-reliability correction changes **only test fixtures** in
`crates/core-workspace/tests/git_system.rs` and the `#[cfg(test)]` tests in
`crates/core-runtime/src/lib.rs`. It may add this maintenance packet. It MUST
NOT change production shutdown or Git inspector behavior; M01/M02 public
contracts, canonical Requirements, Architecture, M04 Work Order, M04 active
Context Lock, security policy, resource defaults, dependencies, or historical
audit findings are out of scope. No global M04 EV may be promoted here.

## Problem and constrained correction

1. On exact unchanged head `8442a5...`, M01 Ubuntu's full workspace test
   first failed the M02 fake-Git deadline fixture with `Ok(None)` instead of
   the typed 500 ms timeout, while M02 Ubuntu passed. The exact-head
   targeted M01 Ubuntu retry passed, proving nondeterminism/runner sensitivity
   but not a single root cause. The old fixture uses a shell script and an
   external `sleep` binary on Unix but a native Rust binary on Windows.
   Compile the same native Rust 30-second fake Git helper on both platforms,
   then preflight an *empty-environment invocation* using a bounded explicit
   ready marker and reap the probe before testing inspector behavior.
   The inspector's 500 ms production input budget and expected typed timeout
   remain unchanged. Never skip the test or turn `Ok(None)` into PASS.
2. The M01 cooperative-worker success tests have been observed failing under
   Windows hosted concurrency while standalone M01 checks of the same head
   pass. Explicitly await spawned worker *readiness* before shutdown and give
   the test-only success path a 2,000 ms scheduling/journaling margin instead
   of 500/1,000 ms. All workers must still report completion within that
   margin, `receipt.clean` must remain true, and journal proof must hold.
   The independent `shutdown_timeout_escalates_only_after_deadline`
   negative test retains its 5 ms deadline and still must prove that a hung
   worker escalates fail-closed. No runtime default/production timeout change.

## Verification and STOP

- Code-reviewed exact delta remains these two test-only surfaces plus packet.
- Rustfmt, strict workspace Clippy, locked workspace tests on Windows/Ubuntu,
  bounded M01/M02/M03 fuzz, governance, dependency/license/advisory and SBOM
  configured jobs pass on the *final exact head*. Rerun only when supported
  by an observed failure and preserve first-attempt history.
- For issue #115: demonstrate that the actual empty-env native helper starts,
  writes its explicit ready marker and remains alive before the inspector
  invokes it; a failed preflight identifies helper problems instead of
  misattributing them to the inspector. The 500 ms timeout still yields typed
  `ResourceBudgetExceeded`; no following Git inspection is poisoned.
- For issue #112: verify cooperative-worker completion and clean receipt for
  the readied tasks, and the separate noncompleting-worker deadline still
  yields an unclean receipt. This addresses fixture scheduling reliability,
  **not** an unproven production shutdown root cause.
- Require full exact-head owner audit by the sole operational identity
  `KayzenRoot`, explicitly `NOT INDEPENDENT`, zero unresolved HIGH/CRITICAL,
  no native GitHub self-APPROVE and protected-main squash promotion only after
  the audit. Only then decide whether both maintenance issues can close;
  record residual unknown production hypotheses separately if any remain.

STOP on any product-code drift, genuine production deadline/notification
failure, unchanged-head nondeterminism in the hardened fixture, failed
security/CI step, or invalid governance/source basis. No CI waiver.
