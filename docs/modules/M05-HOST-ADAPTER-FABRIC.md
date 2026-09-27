# M05 Host Adapter Fabric — Round 1 discovery candidate

Status: R1_R2_DOCUMENTED / R3_NON_AUTHORITATIVE_DISCOVERY_CANDIDATE  
Base: 3b7d184ad50ef22320d57572dfade965a98fbad4  
Work Order: https://github.com/KayzenRoot/core/issues/133  
Public contract frozen: NO | Product implementation authorized: NO | M04 contract decision: BLOCKED_EXTERNALLY

This planning document is a source-grounded candidate, not a final M05 interface, Work Order execution admission, or claim that M04/real local HIVE has passed. It may advance independently while M04 issue #111 waits for dated owner evidence. Nothing here updates the canonical Project Brain checkpoint or the ACTIVE M04 Context Lock.

## 1. Mission

M05 will mediate between CORE's previously admitted, versioned host-invocation requests and a bounded host adapter session. Its proposed responsibilities are a transport/session boundary, handshake observations, explicit endpoint provenance, finite framing, bounded request/response mediation, cancellation/timeout observation, typed disconnection failures, redacted diagnostics, and normalization of external *observations*. It prevents each later model/tool/agent integration from implementing its own unsafe process and protocol plumbing. It cannot manufacture execution authority from a connected process, MCP tool description, model output or configuration.

## 2. Module authority matrix

- **M01 owns** supervisor, worker lifecycle, core module/capability registry, runtime epochs, cancellation root, versioned local IPC primitives and module health/degradation. M05 may lease/use these exact primitives but must not fork a second registry or worker supervisor.
- **M02 owns** explicit validated workspace basis and read-only Git/local proof. M05 must never treat an ambient working directory or host-declared path as action authority.
- **M03 owns** canonical admitted Work Order, scope, policy/source/Context Lock binding. M05 cannot compile a free-form instruction into execution authorization.
- **M04 owns** immutable Run/Attempt/Step lineage, durable journal, atomic commits, cancellation truth and replay. M05 may hand a bounded typed result/reference back to an authorized effect-owning caller; it never writes M04 journal or chooses the pending breaking-V1 versus V2 policy. M04 must not gain an M05 dependency.
- **M06 owns** capability negotiation, provider eligibility and compatible binding decisions. M05 exposes *observed* protocol/feature declarations and transport state only. It must not silently substitute a lower-quality provider.
- **M07/M08/M09 own** specialist, sequential agent and model/effort routing. M05 does not choose the agent or model.
- **M10/M11/M12/M13 own** execution policy, isolation/leases, actual command/tool execution and mutations. M05 only transports requests after those owners admit them; process availability never authorizes shell, filesystem, network, repo write or arbitrary provider action.
- **M14–M19 own** verification, evidence, review, corrections, recovery, cost/resource policy. M05 observations alone cannot prove a tool effect succeeded or grant retry/recovery.
- **M20–M24 own** Git/GitHub delivery, CI/release, security/supply-chain, HIVE federation and telemetry spine. M05 is not a Git client, security policy engine, HIVE RAG clone or second durable telemetry system.

The canonical planning order is M04 -> M05 -> M06. M05 may conduct discovery now, but binding an implementation DTO to M04 event/BRC/replay contracts must wait for issue #111's independent compatibility disposition.

## 3. SOLO, HIVE-connected and transport candidate boundaries

CORE SOLO must work without HIVE or remote network. A trusted compiled module may use M01's existing in-process paths; isolation-demanding first-party workers must reuse M01's length-framed local Unix-domain sockets/Windows named pipes, **not** default localhost TCP. M01's worker supervisor owns actual process creation/reaping and parent cancellation. Arbitrary dynamic libraries remain rejected.

Possible future separately admitted host transports: versioned child process/stdio, MCP client, first-party local IPC, and an optional authenticated remote API adapter. No transport, SDK, auth backend, network listener or added crate is frozen by this Round 1. Remote/TLS credentials, sandbox enforcement and network authority require M11/M22 review, not a convenient M05 default.

HIVE remains a separately installed v1.0.0-pinned intelligence plane; any optional read-only MCP context session is a truthful observation, not proof of HIVE's release (protocol version differs from release tag), its live local corpus or the separately configured Codex client. HIVE's memory/retrieval ownership stays under HIVE and M23.

## 4. Proposed candidate contracts (not frozen public APIs)

The future handshake *observation* should be bounded and versioned: session identity, actual observed protocol schema/major version, independently verifiable endpoint/process provenance references, declared features (untrusted until M06/policy corroboration), finite frame limits, M01 epoch and adapter activation generation. Missing/unknown critical protocol extensions, downgrade or unverifiable endpoint identity fail typed, never silently succeed.

The future invocation envelope should carry caller-supplied **previously authorized** policy/capability-lease reference, opaque operation identity, versioned typed expected response, finite deadline/stream budget and M01 cancellation reference. Whether any M04 Run/Attempt/Step identity is mandatory and its final serialization MUST remain undecided until M04 V1/V2 resolution and M06/M11/M12 planning; do not import the pending PR #118 draft as canonical truth.

The future result should distinguish: transport success, externally confirmed effect, **EXTERNAL_EFFECT_UNKNOWN**, transport failure, protocol incompatibility and caller cancellation. A successful transport reply, returned text or elapsed-time diagnostic is not an M04 journal commit, evidence acceptance or proof that an external side effect happened exactly once. Return bounded references with provenance, not raw prompts/credentials/paths or untrusted host content in canonical evidence. Preserve taint marking for later verification.

## 5. Candidate lifecycle and invariants

Round 2 may evaluate `DISCOVERED -> CONFIG_VALIDATED -> HANDSHAKING -> READY -> DRAINING -> CLOSED`, with typed `REJECTED/QUARANTINED/LOST` paths. These labels are candidate names, **not** a frozen state machine.

1. Discovery and handshake alone grant ZERO process/tool/network/filesystem authority.
2. Every request/result is scoped to one immutable M01 runtime epoch, adapter generation and request correlation identity. Stale/late messages cannot publish current-gen success.
3. Reject incompatible major versions, absent mandatory features, unsupported security-critical extensions, identity mismatch and noncanonical/oversized frames.
4. Use caller-owned immutable operation identity and explicit idempotency contract; transport-level resend must not replay a possibly non-idempotent external effect.
5. Cancellation and deadlines are **observational**. A host may have performed an action despite timeout or connection loss; respond EXTERNAL_EFFECT_UNKNOWN and hand off reconciliation to the effect owner/M18 rather than guessing failure or blind retry.
6. M01 shutdown drives bounded draining, cleanup and worker reaping; quarantined/lost adapters receive no new invocations. An in-flight M04 durable state transition must remain M04's sole responsibility.
7. Timestamps, stdout, host text, MCP tools/list and HIVE-derived context are untrusted diagnostics, never ordering authority or source of permissions.
8. The protocol/parser/handshake path is deterministic and zero-LLM. No model decides protocol compatibility or trust.

## 6. Threat/failure model for Round 2

Endpoint impersonation or substitution between discovery/connection; forged manifest, version downgrade, unknown critical feature, cross-session identity/epoch injection, unsolicited/reordered/late frames, partial/oversized/infinite streams or notification floods, secret-bearing stdout/stderr, host-supplied path traversal, parent-child process escape, unauthenticated remote endpoint, protocol-versus-release confusion, external crash after real side effect but before local receipt, duplicate idempotency claims, malicious model-facing host content, provider flapping, resource starvation and Unix/Windows IPC semantic divergences.

Safety direction: fail typed, bound resources, never print raw error/payload secrets, preserve source/taint provenance, avoid hidden host process/network authority. M11/M22 independently govern actual sandbox, auth and secret lifecycle.

## 7. Resources, telemetry and calibration

Candidate finite dimensions: handshake/frame/payload bytes, nesting depth, concurrent sessions/inflight calls, stdout/stderr diagnostic bytes, notification count, host startup/idle/read/write/cancel/drain deadlines, bounded reconnect attempts, artifact-reference count/size, process RSS/CPU and IO, telemetry cardinality, and context-copy/token overhead. **No invented numerical defaults**. Later calibration must report reproducible cold/warm startup, normal/adversarial throughput, error/reconnect/cancel latency, Linux/Windows differences, selected/rejected candidate caps and regression floors.

A future event should carry stable error code, runtime epoch, adapter generation, protocol version, bounded redacted counters and no secret-bearing host text. M24 owns event transport/observability, M19 resource/cost policy, M15 evidence truth.

## 8. Candidate test and evidence matrix, all PENDING

- EV-M05-D01: M01 supervisor/registry reuse and acyclic dependencies.
- EV-M05-D02: no assumption that pending M04 V1 changes are canonical; no M04 reverse dependency.
- EV-M05-D03: SOLO/no-HIVE behavior, missing host/provider typed errors, truthful degradation.
- EV-M05-D04: endpoint/provenance, runtime epoch and adapter-generation spoof rejection.
- EV-M05-D05: protocol version/feature downgrade and unknown critical field fail-closed fixtures.
- EV-M05-D06: malformed/fragmented/oversized/deep/unsolicited frame fuzz with hard caps.
- EV-M05-D07: host death, timeout, cancellation, unknown external effect and late reply races.
- EV-M05-D08: discovery/transport never implicitly grants tool/network/filesystem authority.
- EV-M05-D09: secret/header/path/stdout redaction and untrusted response/prompt-injection isolation.
- EV-M05-D10: Unix-domain-socket and Windows-named-pipe cross-platform differential fixtures.
- EV-M05-D11: explicit caller-owned idempotency, versioned receipts and ambiguous-effect reconciliation.
- EV-M05-D12: cold/warm/memory/CPU/IO/stream/cancellation calibration and reproducible baselines.
- EV-M05-D13: deterministic zero-LLM and no hidden repo/network mutation without admission.
- EV-M05-D14: exact-head Linux/Windows, supply-chain/SBOM, bounded fuzz and NOT INDEPENDENT owner audit after future admission.

These are **only discovery IDs**, not a frozen M05 AEG, not completed tests and not substitutes for the 23 still-PENDING global EV-M04 obligations.

## 9. Planning coverage and exclusions

Round 1 proposes mission/ownership; SOLO versus HIVE substitution; ephemeral session versus durable M04 state; tentative lifecycle, failure and security boundaries; dimensional resource/telemetry requirements; candidate tests; compatibility conditions and explicit STOP. The exact versioned public/internal DTOs, exports, crate/file map, dependency/transport admission, approved host types, security policy, calibrated numeric resource caps, migration and final acceptance/DoD **remain UNFROZEN** for later rounds. Prefer one bounded module Work Order after final freeze, not speculative implementation here.

OUT OF SCOPE: Rust code, direct host invocation, installing credentials/providers, selecting model/tool/policy, remote listener, full MCP SDK, database, HIVE memory/RAG, M04 source or product contract amendment, synthetic external compatibility proof, checkpoint promotion, numeric performance claims.

## 10. Next legal increment and STOP

A later Round 2 must reconcile exact actual M01 worker/registry interfaces with the M06/M11/M12 policy boundary; define candidate transport/handshake semantics and hostile fixtures. Detailed M04-bound invocation DTOs remain dependent on the separately governed outcome of issue #111. Round 3+ may evaluate finite budgets and process/transport options with measured evidence; final planning freeze and a **separate** execution-admission delta are mandatory before M05 code.

Round 1 may close only after exact-head governance/CI, an explicitly NOT INDEPENDENT scoped owner self-audit with zero unresolved HIGH/CRITICAL, protected docs-only squash merge and real full main-push validation. This promotes a documented **discovery candidate only**, never public-contract freeze or product implementation authority.


---

## Round 2 — source-backed host handshake and failure laws (discovery candidate)

Status: R2_NON_AUTHORITATIVE_CANDIDATE  
Parent Work Order: https://github.com/KayzenRoot/core/issues/136  
Planning base: eb231f57e8dffff0db811cf4b87c645e7022af59  
Public API frozen: NO | Rust code authorized: NO | External M04 V1 decision: UNKNOWN / BLOCKED

This round refines **transport/session semantics only** using existing source, not an invented generic IPC layer or newly authorized external execution capability. M04 durable state, M06 selection, M11 isolation/authentication, M12 tool execution, M18 recovery and M22 security policy remain with their owning modules.

### R2.1 — Actual M01 source contract, verified before this proposal

The existing `crates/core-ipc/src/lib.rs` exposes `ProtocolVersion` (CURRENT major=1, minor=0), `Frame` with an epoch and binary payload, `Handshake` with version/epoch/max_frame_size, `encode`, `decode`, `negotiate`, and async `read_frame/write_frame`. The accepted first-party control frame uses a 19-byte header: `CR1` magic (3), major/minor (2+2), u32 declared length (4) and epoch u64 (8). Encoding rejects oversized payloads and payload lengths unrepresentable as u32; decoding rejects short/bad magic, incompatible major, excessive declared length, stale epoch, truncation and trailing bytes.

`negotiate(local, remote)` currently requires **equal major and epoch** and chooses `min(minor)` and `min(max_frame_size)`. It does NOT independently authenticate a host, negotiate capability features, enforce a required minimum minor, validate trust/authority, bind an M04 operation or establish that a remote process actually uses a named Git release. Those are separate candidate caller/M06/M11/M22 obligations. A compatible version tuple is a transport observation, never sufficient execution authorization.

`read_frame` does check the declared length before allocating the body; its **full** magic/major/epoch validation currently occurs when `decode` is called after the bounded body read. For any future admission of an untrusted/third-party transport, assess whether prevalidating header authenticity and epoch **before** accepting a declared body would reduce avoidable bounded work. If this proves a defect, use a separate M01-scoped security Correction Delta and tests, not an unauthorized modification under this M05 documentation Work Order.

Existing OS adapters are `core_ipc::unix` (Unix domain socket) and `core_ipc::windows` (Windows named pipe); the Windows server requests `reject_remote_clients(true)` and byte pipe mode. Neither transport choice, a Unix socket path nor that Windows option alone verifies a peer process identity or grants trust. Concrete Unix peer credentials, filesystem permissions, Windows pipe ACLs, process ancestry and executable provenance must be assessed by M11/M22 with a platform-specific threat/assurance policy. Do not launch a second unreviewed codec in M05.

### R2.2 — Reuse the actual registry and lease seams

`core-registry` already owns ModuleRegistry, CapabilityRegistry, provider health/quarantine, deterministic bindings, generation-aware leases, expiry/revocation, snapshot and subscription primitives. `core-contracts` already defines `RuntimeGeneration` (boot, config, module graph, capability graph, policy), `CapabilityProviderDescriptor`, `CapabilityBindingReceipt`, `CapabilityLease`, `IsolationClass` and `ProviderOrigin`. A descriptor fingerprint over self-declared metadata can prove equality to those metadata bytes, **not** third-party identity authenticity; a host process must never raise its own trust or assurance class.

Proposed host session adapters should carry only immutable, caller-validated **references** to the current binding/lease/generation. M05 may compare a returned adapter-generation/endpoint-fingerprint observation with previously admitted input, but **only** M01/M06's registry validates or revokes leases and only later M10/M11/M22 policy can admit executable external operations. Failure to independently verify provider origin, active lease, required feature/policy/quality or generation returns a typed refusal before any host action.

### R2.3 — Candidate first-party handshake procedure

1. **DISCOVERED (zero authority):** caller supplies an endpoint description and independently obtained M01 runtime epoch/worker identity; M05 records no client-provided trust claim as approval.
2. **CONFIG_VALIDATED:** M01/M11/M22 supply actual process/transport/peer authorization and finite resource bounds. Invalid or unknown origin fails closed. Local namespace/path is *not* authority by itself.
3. **HANDSHAKING:** for already-admitted first-party IPC, reuse M01's `Handshake` and its `negotiate` result. Verify both endpoints have the exact current runtime epoch, a compatible protocol major and a bounded, actually admitted frame size. Caller/M06 additionally rejects required-feature loss or an insufficient negotiated minor; never silently treat minor=min as full feature compatibility.
4. **READY (not yet an invocation permit):** publish only a typed observed session receipt binding actual peer proof reference, current M01 epoch, adapter generation, negotiated protocol, finite caps and optional untrusted advertised feature claims. No M04 Run/Attempt/Step or host invocation becomes authorized merely because READY was reached.
5. **DRAIN/CLOSE or QUARANTINE/LOSS:** propagate M01 cancellation and bounded deadlines, revoke *new* transport admissions and report active-call ambiguity. Only the effect-owning caller/M18 decides reconciliation or retry.

This sequence is a **candidate**, not an admitted public method, complete state transition matrix, new IPC protocol, or permission to use external providers. A future external MCP/stdio/remote adapter needs a **separate protocol translation and provenance review** rather than treating the `CR1` frame as if every protocol shares its byte format.

### R2.4 — Request/result/cancellation candidate invariants

- **Identity:** request correlation, adapter session, worker identity, current M01 runtime epoch and registry binding generation must be bound together. Cross-session IDs, stale generation and mismatched observed peer fail typed.
- **Delivery versus effect:** a successful write/transport acknowledgment does not prove an external operation completed; a reply received by an authenticated connection is not automatically trustworthy semantic evidence. If a request might have reached the host before crash, cancellation, timeout or connection loss, return `EXTERNAL_EFFECT_UNKNOWN` and preserve the invocation fingerprint/authority references for the effect owner to reconcile. M05 itself never retries an ambiguous non-idempotent operation.
- **Pure transport outcome:** M05 observations cannot call `M04StateStoreV1.compare_and_commit`, mint M03 admission, select M06 provider, execute an M12 command or mutate files. Later code must not wire a known-new V1 journal request field before M04 issue #111 resolves.
- **Streams:** enforce finite header/body size, bounded per-session pending messages, bounded diagnostic bytes and read/write/cancellation deadlines. If resource evidence is incomplete, return a typed blocked/degraded outcome, not a successful truncated result. Values require later reproducible calibration; no provisional cap is an accepted production threshold.
- **Untrusted data:** keep raw host text, stdout/stderr, model prompts, env vars, authentication headers, local paths, peer IDs and M04 journal contents out of durable public evidence. Use bounded, typed reason codes and redacted counters. Future LLM consumers must treat host text as untrusted input.
- **HIVE:** HIVE MCP protocol `mcp-core-surface-v1` is not a Git release tag or proof that HIVE v1.0.0 is running on the owner's machine; local HIVE issue #4 and M23 integration authority are separate.

### R2.5 — Candidate typed failure/ambiguity matrix (names not frozen)

| Situation | Candidate M05 observation | Later owner/gate |
| --- | --- | --- |
| Bad `CR1` magic, incompatible major, stale epoch, frame too large, truncated or trailing bytes | `TRANSPORT_PROTOCOL_REJECTED`, preserving precise safe M01 `IpcError` classification | M01 codec; M05 adapter maps errors without printing untrusted frame |
| Negotiated minor insufficient for required feature or unknown security-critical extension | `REQUIRED_FEATURE_UNAVAILABLE` / fail closed | M06 compatibility and M10/M22 policy; `negotiate` alone does not check this |
| Peer process/protocol identity not independently corroborated | `ENDPOINT_PROVENANCE_UNVERIFIED` | M11/M22, zero invocation authority |
| Valid transport but invalid/expired/revoked capability lease or changed runtime generation | `AUTHORITY_STALE_OR_ABSENT` | M01/M06 validate actual active lease; M05 must not forge |
| Host crash/timeout/disconnect after potential request delivery | `EXTERNAL_EFFECT_UNKNOWN` | Effect owner and M18, never blind M05 resend |
| Known-undelivered request before any side effect could occur | `REQUEST_NOT_DELIVERED` **only with proof of non-delivery** | Effect owner must validate claim before any retry |
| Duplicate, late or cross-session completion | `STALE_OR_UNEXPECTED_REPLY` | M05 rejects publication, M04 immutable state unaffected |
| Read/write/cancellation/diagnostic/resource limit exceeded | `TRANSPORT_RESOURCE_EXHAUSTED` | M01 lifecycle and M19 finite budget; no partial success |
| Authenticated/authorized first-party handshake succeeded but no applicable execution policy exists | `SESSION_READY / INVOCATION_BLOCKED` | M10/M11/M12 own actual action permit |

### R2.6 — Security and executable negative-fixture plan

All **future** tests below are PENDING, not claimed to have executed on M05:

1. Real `core-ipc` golden frame/header round trip and major/minor/epoch/min-cap negotiation at the current exact dependency.
2. Bad magic, truncated header, oversize u32 length, trailing bytes, stale epoch, unsupported major and required-minor feature-lowering negative cases.
3. Verify bounded `read_frame` memory and timing for a forged header with a max-size declared body. Decide whether a separate M01 correction is justified before admitting any third-party codec.
4. Unix socket path substitution/permissions and platform-specific peer identity proof; Windows named-pipe remote rejection, ACL and local-user impersonation adversarial cases.
5. Invalid/expired/revoked binding generation and lease; provider fingerprint computed from false metadata must not be misrepresented as authenticated trust.
6. Cross-session/cross-epoch/duplicate correlation, unsolicited notification flood, max+1 bytes, partial stream, crash after request delivery and cancellation/late reply races.
7. Header or stdout containing private filesystem path, prompt text, token/credential or provider-supplied instructions must never enter public receipts/logs or become permissions.
8. SOLO mode works with missing HIVE/remote endpoint and does not require a model, GitHub token or privileged OS operation.
9. Later admitted Linux/Windows exact-head tests, bounded fuzz/property coverage, benchmark-calibrated ceilings, supply-chain/SBOM and NOT INDEPENDENT owner audit; no future evidence result is inferred from this Round 2 doc-only CI.

### R2.7 — Deferred decisions / explicit non-admission

Pending M06/M11/M12 and M04 disposition: concrete M05 public request/result DTOs, representation of M04 lineage and actual idempotency guarantees, authenticated provider discovery, remote/TLS choice and secrets, first required external protocol, sandbox defaults, exact module/crate/file map, dependency admission, tool-execution semantics, numeric resource budgets, acceptance and production DoD.

**Round 2 STOP:** source-backed append and new evidence record, exact-head docs-only Governance with successful preserved status contexts, scoped owner self-audit NOT INDEPENDENT with zero unresolved HIGH/CRITICAL, protected squash merge and real full main-push CI. This documents a candidate transport/handshake failure model only, not public-contract freeze, executed M05 tests or any permission to advance blocked M04.


---

## Round 3 — host transport options, candidate single-crate map and evidence calibration

Status: R3_NON_AUTHORITATIVE_DISCOVERY_CANDIDATE  
Planning Work Order: https://github.com/KayzenRoot/core/issues/142  
Exact initial Round 3 main basis: d02462309f47632e409d0d3a7d6b265809b5dcb4  
Product implementation and public DTO freeze: NOT AUTHORIZED

Round 3 identifies an incremental and verifiable **technology direction**, not an admitted dependency set or permission to create a host process. The admitted M01 `core-ipc` already supplies the first-party CR1 framed protocol; after a separately scoped fix in PR #140 its parser rejects a malformed fixed header before allocating/reading a body. A transport handshake still does not authenticate a third-party host, select a capability or authorize an execution side effect.

### R3.1 — Source-backed option matrix, no invented results

| Candidate | Source basis and SOLO behavior | Unresolved trust/security and cost | Round 3 disposition |
| --- | --- | --- | --- |
| Reuse first-party `core-ipc` Unix socket/Windows local-only named pipe | Existing M01 `Frame`, `Handshake`, current CR1 header/version/epoch/limit checks and already implemented platform adapters. First-party isolated worker remains independent of HIVE or remote service. | M01 supervisor owns launch/drain; M11/M22 still must verify executable/OS peer, path/ACL and effective authority. `negotiate` does not verify required features/quality or third-party identity. No separate M05 framing codec. | **Reuse the existing admitted M01 primitive**, with M05 as a candidate session/mediation wrapper only. This is not an additional transport selection or M05 code approval. |
| Child process with separate stdio streams | Candidate narrow external adapter when a tool/host already supports a versioned framed/line protocol. Avoids remote network prerequisite but requires an independently governed subprocess boundary. | A hostile executable, malformed stdout/notification floods, mixed log/control output, deadlock on full stderr pipe, process-tree cleanup, leaked environment and ambiguous external effects. M01 supervises and M11/M12/M22 admit process execution. | **Evaluate**, not required or selected until executable provenance, framing and sandbox tests support it. Do not repurpose stdio diagnostic output as trusted semantics. |
| MCP client/protocol bridge | Candidate for tools served by a separately configured process or endpoint. The existing HIVE read-only MCP surface is an optional context-provider example, **not** proof of installed v1.0.0 on the owner's machine. | MCP protocol release/version and server software release are distinct; tools/list is untrusted data, server prompts/resources can contain hostile instructions, and transport can be stdio or remote with different trust models. Policy/lease/auth is not defined by MCP discovery. | **Evaluate as an external protocol adapter after separate admission**. Do not fork the HIVE retrieval/memory implementation or silently require an MCP SDK dependency. |
| Authenticated remote API transport | Not needed for first SOLO/local-first M05 implementation; later service providers may require a network path. | TLS/identity/credential lifecycle, replay protection, egress policy, revocation, remote quota, tool/result trust and distributed ambiguous effects remain unfrozen M11/M19/M22/M23 responsibilities. | **DEFER to explicit future dependency/security admission**; no remote listener or network package is authorized by this discovery. |
| Arbitrary in-process dynamic/plugin libraries | Conflicts with accepted M01 process-isolation and no-arbitrary-dynamic-loading policy. | Foreign memory safety/privilege crossings and unbounded side effects; would bypass host and sandbox boundary. | **Excluded** unless a later full architecture/security Change Request explicitly supersedes M01 policy. |

These are governance dispositions based on the **existing architecture** and identified threats, not benchmark scores or a relative speed ranking.

### R3.2 — Proposed one-crate decomposition, all paths provisional

If final planning later admits M05, a *candidate* single cohesive crate `crates/core-host-adapter` can hold **only** transport/adapter-session semantics and independently testable ports. It must not be created by this R3 change:

| Candidate path | One responsibility | Negative boundary |
| --- | --- | --- |
| `src/lib.rs` | Re-export the admitted candidate host-port/session API after final contract freeze | No runtime supervisor or M04 journal writes |
| `src/contract.rs` | Versioned bounded observed handshake, session/invocation/outcome references and typed errors, if later admitted | No new M04 V1-specific Run/BRC/idempotency serializer while #111 blocked |
| `src/session.rs` | Pure session-generation/epoch/correlation state machine and finite admission/cancel/drain transitions | No model decisions, command invocation, retry strategy, provider replacement or unbounded timers |
| `src/local.rs` | First-party adapter wrappers **around existing** `core-ipc` encode/decode/handshake and M01-owned local transport | No duplicated CR1 codec, second socket server or worker manager |
| `src/stdio.rs` | Only if separately admitted: bounded parsed stdio session adapter with stdout control/stderr diagnostic separation | No implicit shell, unverified executable or arbitrary environment inheritance |
| `src/provenance.rs` | Compare independently provided peer-proof/lease/epoch/generation references; report unverified status | No self-attested trust upgrade, credential storage or duplicate capability registry |
| `src/budget.rs` | Validate finite caller-supplied bounds against later calibrated profiles | No unbounded sentinel, guessed defaults or self-tuning policy |
| `src/outcome.rs` | Typed transport-level observations including `EXTERNAL_EFFECT_UNKNOWN`; bounded redaction | No claim that a tool/model result is verified, idempotent or durably committed |
| `tests/host_session_laws.rs` | Deterministic state/error/replay/cancellation and fake-port law fixtures | No real process/network required to prove pure state laws |
| `tests/local_ipc_boundary.rs` | Existing M01 CR1 differential/hostile-header tests through public `core-ipc` API | No replacing existing M01 tests or asserting host authentication from successful framing |
| `tests/stdio_boundary.rs` | Future governed child-process, stdout/stderr, timeout and kill tests | Omit if stdio transport is not separately admitted |

Candidate dependency direction: `core-contracts` for already accepted stable `RuntimeGeneration`, `CapabilityLease` and failure/identity dimensions; `core-ipc` **only** for admitted first-party frames; other existing shared primitives as individually justified. The M01 supervisor, CapabilityRegistry, M06 negotiated decision, M11 sandbox/authentication and M12 invocation authority should be **injected caller-owned ports/receipts**, never recreated or imported through an M01 `core-runtime` reverse dependency. No direct M04 dependency is selected until #111 resolves; optional stdio/MCP/remote SDK and Tokio feature changes require a separately reviewed dependency-admission delta. This table is a future file **map proposal only**, not a promise that every file/module will be necessary.

### R3.3 — Proposed cross-platform transport and trust experiment

Future test/evaluation execution must pin exact source/lock/dependency fingerprints and collect comparable Linux and Windows evidence. Prepare a fixture corpus whose authority is **synthetic** and whose content has no user secrets:

1. **First-party local baseline:** exercise existing `core-ipc` CR1 round-trip and handshake at small, typical, near-cap and cap+1 frame sizes under the existing M01 public APIs. Record actual configured cap and test all known error codes and epoch/major/minor boundaries. Reuse existing M01 regression for invalid header rejection before body read; do not count those M01 tests as M05 implementation evidence.
2. **Candidate session laws:** fake an authorized caller-supplied M01 epoch, binding generation and separately corroborated peer proof. Permute stale/changed generation, invalid peer proof, unavailable required feature and expired/revoked capability lease. Verify no session READY implies an invocation permit.
3. **Proposed stdio/protocol bridge:** if later admitted, isolate a controlled fake subprocess under sandbox/policy with independent stdout control and stderr diagnostics. Inject oversized/fragmented frames, stdout noise, burst notifications, partial writes, suppressed output, buffer backpressure and bogus tool descriptions. Do not test by invoking real user tools or scanning the filesystem.
4. **Cancellation/ambiguity:** deterministic in-memory barrier injects disconnect/crash/timeout before delivery, after delivery but before reply, and after reply but before caller receipt. Only provably-undelivered cases may state non-delivery. Unknown effect must never be auto-replayed and late prior-epoch responses must not publish success.
5. **Cross-platform host security:** Unix socket filesystem permission/peer-credential fixture and Windows named-pipe remote rejection/ACL/impersonation fixtures are separate M11/M22-owned gates. Same bytes on both platforms do not prove same peer trust.
6. **Zero-LLM/data boundary:** host handshake/framing, refusal, cancellation and closeout require no inference. Synthetic malicious server text, command suggestions and fake policy overrides must remain tainted and out of canonical authority.
7. **Supply-chain/evidence:** exact-head formatting, Clippy, Windows/Ubuntu fixture tests, bounded fuzz for frame/session/adversarial transcripts, dependency/license/advisory/SBOM and typed owner self-audit `NOT INDEPENDENT` are future blockers for *implemented* M05. Historical M01/full-main CI is prerequisite context, not proof a future M05 build passes.

### R3.4 — Measurement protocol before numeric limits

A later calibration gate must capture toolchain/OS/CPU/RAM, reproducible fixture generation, exact command/profile, raw samples, selected and rejected candidate ceilings and reasoned comparison to **the same machine's** existing M01 IPC baseline. Separate startup/warm and steady-state regimes; use paired repetitions rather than comparing unrelated hosted runs. Record distribution including median and tails, peak resident memory, CPU, I/O/syscalls, bounded queue growth, buffer copies/allocation pressure, throughput, response latency, cancellation-to-quiescence, crash/restart behavior, redaction size and protocol overhead. Mark which measurements are platform-specific and which are synthetic-only.

Candidate knobs requiring finite, validated production values **before** M05 final product admission: maximum frame/control bytes, body nesting, concurrent sessions/in-flight requests, queued notifications, aggregate buffered stdout/stderr, idle/read/write/deadline/cancellation/drain budgets, child-process restart count and quarantine cooldown, correlation table cardinality, artifact reference count/bytes and redacted telemetry retention. A smaller sampled corpus cannot establish safety at larger untested scales. Never treat P99 from a single run as a guarantee or invent a speedup from the existing docs-only 17-18 s CI observations, which have no bearing on M05 IPC performance.

For each knob, later implementation must demonstrate `cap-1/cap/cap+1` behavior, bounded memory/time under malicious partial streams, atomic typed limit errors with no partial-authorized success, and selected/rejected candidates with reproducible justification. Failure to calibrate keeps that field **PENDING**, not a permissive unlimited default or extrapolated production setting.

### R3.5 — Gate ownership, compatibility and next Round

This appendix makes **no** change to M01, its accepted IPC wire protocol or single-account GEF rules. No M05 public request/receipt, crate, dependency, external command, network listener, HIVE client or numeric threshold is frozen. The ephemeral host-session proposal remains acyclic: M01 provides runtime/codec/registry, M05 observes mediated transport; M06 selects, M10/M11/M12 authorize and perform effects, M04 alone commits durable Run state, M18 reconciles unknown side effects, M22 sets security policy, M23 federates HIVE. The pending M04 external prior-V1 journal/binary/consumer inventory #111 and local HIVE/Docker/Codex issue #4 cannot be replaced by GitHub-only source proof.

**Round 3 STOP CONDITION:** promote this source-grounded **planning** appendix plus separate evidence record only after exact-head docs-only CI, bounded logical owner self-audit NOT INDEPENDENT with zero unresolved HIGH/CRITICAL, protected squash merge and full real main-push validation. Round 4 may propose a candidate semantic contract/acceptance graph only if it does not freeze M04-bound DTOs or silently choose unadmitted transport/dependency/numeric limits. Final planning freeze and a separate Work Order execution admission are still prerequisites for M05 code.
