# CORE M01 recovery validation

Date: 2026-09-19
Source prompt: `CORE-M01-CODEX-RECOVERY-PROMPT-001.pdf`
Work Order: `CORE-WO-M01-001`

## Validation basis

- Branch: `feat/m01-core-runtime`.
- Code/evidence validation basis before this documentation-only record: `44bca1be92ff35192fbfbd20957d1ba5043be9c9`.
- Frozen base: `fdb4dbe165e74b009c43df3874b6043c9b94710b`.
- Remote before publication: `origin/feat/m01-core-runtime` at `45ba7c8848419d5768cd0bfdf524575a02959ca5`.
- Local basis was clean and ahead of the remote by 10 commits.

## Deterministic gates

- `python scripts/validate_governance.py`: PASS.
- `cargo metadata --locked --no-deps`: PASS.
- `cargo tree --workspace --locked`: PASS.
- `cargo fmt --all -- --check`: PASS.
- `cargo clippy --workspace --all-targets --locked -- -D warnings`: PASS.
- `cargo test --workspace --all-targets --locked`: PASS; 22 tests passed, 0 failed, 0 skipped.
- CLI `version`, `validate`, `doctor` and `start`: PASS; `start` returned `ReadyEligible` and `llm_calls` remained zero.
- BOOT benchmark: 25 samples; WNF `3e70c0176fa97ef4c43e95976b5c95b13fa205e8cceb4740bf361f74fa666127`; p50 `77.2245 ms`, p95 `107.5072 ms`, p99 `121.5817 ms`. The policy remains baseline-relative and WNF/hardware-bound.
- First-party unsafe inventory: no unsafe block found; the only textual match is a safe configuration error message containing the word `unsafe`.

## HIVE synchronization observed in this recovery

- HIVE health: `status=ok`, version `1.0.0`.
- CORE inspection: `READY` at `44bca1be92ff35192fbfbd20957d1ba5043be9c9`.
- Repository index run `137b9872-27bb-4b0a-bc9d-2054a22a8323`: `COMPLETED`; 92 discovered, 92 reused, 0 changed, 0 added.
- Retrieval corpus run `92687f5a-2ea2-42f5-8ef1-6b3e38f26af2`: `COMPLETED`; 177 current references, 177 reused, 0 new, 0 removed.
- HIVE's container-side inspection reported `working_tree_clean=false`; host Git showed a clean tracked working tree. This discrepancy is retained as an integration residual rather than suppressed.

## Remaining governed gates

- `cargo-audit`, `cargo-deny` and `cargo-fuzz` are not installed on this host; no advisory/license/SBOM campaign or long-running fuzz campaign is claimed.
- Hosted Windows/Unix transport, deep failure-injection and soak/resource-growth tiers remain independent review/CI work.
- Independent exact-head governed review is pending; no merge, promotion or checkpoint closeout is claimed.

## Verdict

`READY_FOR_REVIEW`

This verdict means the local implementation and observed evidence are ready for the independent gates above. It is not a completion, merge or promotion claim.
