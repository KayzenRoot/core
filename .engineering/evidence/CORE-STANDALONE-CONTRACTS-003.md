# CORE-STANDALONE-CONTRACTS-003: source-bound M02/M03 V2 migration candidate

Status: EXECUTION_CANDIDATE / NO_CI_OR_MERGE_CLAIM
Work Order: https://github.com/KayzenRoot/core/issues/178
Parent: https://github.com/KayzenRoot/core/issues/172
Original protected-main: 4ab40453a7050add41d6e65b1305979e565a4c41

Scope: rename only M02 and M03 obsolete external-project-service Rust and serialized fields into optional generic association/local canonical evidence, increase the *M02* envelope version and the *M03* serialized envelope version to 2, synchronize M03's exact admitted M02 version, update tests and append two dated source addenda. No new network/database/MCP/Docker dependency, no claim that a generic provider is verified by its own declaration, no implicit V1 decode, no change to canonical M04 frozen source, Work Order, active Context Lock or its evidence bundle. The last two remain old-bound until independently governed synchronized owner review; external prior-V1 consumer inventory #111 is UNKNOWN.

Future reviewed exact-head evidence required: full Governance and M01–M03 Linux/Windows Rust, all three bounded fuzz campaigns, Ubuntu dependency policy/advisory/SBOM/PRB, independent exact-main full run after protected squash; scoped logical KayzenRoot NOT INDEPENDENT review zero unresolved HIGH/CRITICAL/threads. This candidate makes no historical test-success assertion until the exact-head workflow is checked.


## V2 numeric calibration correction (2026-09-29)

The first V2 full hosted run [#36514942247](https://github.com/KayzenRoot/core/actions/runs/36514942247) found outdated V1 assertions in the M03 public contract tests (Ubuntu and Windows). The targeted Correction Delta updates V2 expected version, asserts V1 and future-version rejection, adds M02 legacy envelope/attach-request negative tests, and retains both V2 malformed-payload redaction and V1 secret non-echo assertions. Subsequent staging [run #36583803534](https://github.com/KayzenRoot/core/actions/runs/36583803534) surfaced stale M03 benchmark resource ceilings: the old 32-packet-chain V1 maxima no longer cover V2 serialized bytes. An exact-fixture probe with temporary 100,000-byte measurement ceilings at `8303e2bf976bad607ba90ed5eef3312101a5fbaa`, [run #36584226356](https://github.com/KayzenRoot/core/actions/runs/36584226356), recorded matching Ubuntu and Windows request **78,630** and frozen **79,195** bytes (prior V1 **78,333** / **78,898**, +297 each). The candidate pins those two V2 ceilings exactly and adds equality assertions to the existing bounded benchmark; temporary probe ceilings are not the release values. No other budget dimension or semantic contract is changed by this calibration delta. Historical V1 evidence remains dated provenance.

**Final candidate gate:** fresh exact-head full hosted 11/11 including M03 Linux/Windows benchmark, all bounded fuzz and required supply-chain; scoped owner audit `NOT INDEPENDENT` with zero unresolved HIGH/CRITICAL/threads; protected squash and independently new FULL 11/11 main-push. No final CI/merge claim is made in this pre-CI file; the final exact-head run and reviewer outcome must be recorded on PR #179 / Work Order #178.
