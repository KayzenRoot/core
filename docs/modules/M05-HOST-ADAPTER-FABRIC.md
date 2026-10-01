# M05 Host Adapter Fabric — operative standalone discovery entrypoint

Status: `STANDALONE_DISCOVERY_ONLY / PUBLIC_API_UNFROZEN / PRODUCT_IMPLEMENTATION_FORBIDDEN`  
Authority: CORE-D-205; CORE-D-206 effective on protected-main promotion of Work Order #182.  
Source: canonical Git and `docs/modules/00-MASTER-MODULE-MAP.md`; the original R1–R4 text below is historical non-operative candidate research only.

## Current scope and ownership

M05 is an unimplemented, optional host transport/session candidate for already admitted caller requests. M01 owns runtime, worker lifecycle and first-party local IPC; M02 owns local workspace and Git authority; M03 owns Work Orders and execution scope. M04 alone may eventually own durable Run/Attempt/Step journal truth, but its old admission is STALE and all M04 product integration remains BLOCKED_RE_ADMISSION pending issue #111's real previous-V1 consumer/journal inventory and a NEW standalone exact-source admission. M06 may later evaluate external capability evidence; M10/M11/M12/M22 own policy, isolation, actual effects and security. M23 is a future local context/evidence registry, **not** a provider/server integration or implemented dependency.

## Fail-closed host and protocol boundary

CORE starts and operates with only its local Git-canonical M01–M03 V2 baseline and no external project-intelligence/memory/retrieval installation, Docker service, MCP server, credentials or network. Any future generic stdio/MCP/local IPC/remote adapter is separately admitted with versioned protocol, verified endpoint identity, redacted tainted diagnostics, bounded framing, timeouts and explicit caller-owned policy/lease authority. Discovery, transport availability, host text or a successful protocol response never authorizes process/network/filesystem effects or proves a durable M04 commit. Unknown external side effects require an explicit reconciliation path, not blind retry.

## Current acceptance gate

The following retained R1–R4 material is dated prior planning, including retired pinned-provider assertions and the superseded old M04 lock wording. It cannot establish a live external provider, mandatory provider preference, former #4 runtime proof or M23 federation. All listed historical EV-M05 discovery candidates are PENDING, not passed executable tests. No crate, public DTO, numeric budget, SDK dependency, executable Work Order, Context Lock or product authorization is created here.


## Sanitized historical archive (non-operative; original provenance retained in Git history)

# M05 Host Adapter Fabric — Round 1 discovery candidate

Status: R1_R3_DOCUMENTED / R4_NON_AUTHORITATIVE_DISCOVERY_CANDIDATE  
Base: 3b7d184ad50ef22320d57572dfade965a98fbad4  
Work Order: https://github.com/KayzenRoot/core/issues/133  
Public contract frozen: NO | Product implementation authorized: NO | M04 contract decision: BLOCKED_EXTERNALLY

This planning document is a source-grounded candidate, not a final M05 interface, Work Order execution admission, or claim that M04/real local LEGACY_PROVIDER has passed. It may advance independently while M04 issue #111 waits for dated owner evidence. Nothing here updates the canonical Project Brain checkpoint or the ACTIVE M04 Context Lock.

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
- **M20–M24 own** Git/GitHub delivery, CI/release, security/supply-chain, LEGACY_PROVIDER federation and telemetry spine. M05 is not a Git client, security policy engine, LEGACY_PROVIDER RAG clone or second durable telemetry system.

The canonical planning order is M04 -> M05 -> M06. M05 may conduct discovery now, but binding an implementation DTO to M04 event/BRC/replay contracts must wait for issue #111's independent compatibility disposition.

## 3. SOLO, LEGACY_PROVIDER-connected and transport candidate boundaries

CORE SOLO must work without LEGACY_PROVIDER or remote network. A trusted compiled module may use M01's existing in-process paths; isolation-demanding first-party workers must reuse M01's length-framed local Unix-domain sockets/Windows named pipes, **not** default localhost TCP. M01's worker supervisor owns actual process creation/reaping and parent cancellation. Arbitrary dynamic libraries remain rejected.

Possible future separately admitted host transports: versioned child process/stdio, MCP client, first-party local IPC, and an optional authenticated remote API adapter. No transport, SDK, auth backend, network listener or added crate is frozen by this Round 1. Remote/TLS credentials, sandbox enforcement and network authority require M11/M22 review, not a convenient M05 default.

LEGACY_PROVIDER remains a separately installed v1.0.0-pinned intelligence plane; any optional read-only MCP context session is a truthful observation, not proof of LEGACY_PROVIDER's release (protocol version differs from release tag), its live local corpus or the separately configured Codex client. LEGACY_PROVIDER's memory/retrieval ownership stays under LEGACY_PROVIDER and M23.

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
7. Timestamps, stdout, host text, MCP tools/list and LEGACY_PROVIDER-derived context are untrusted diagnostics, never ordering authority or source of permissions.
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
- EV-M05-D03: SOLO/no-LEGACY_PROVIDER behavior, missing host/provider typed errors, truthful degradation.
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

Round 1 proposes mission/ownership; SOLO versus LEGACY_PROVIDER substitution; ephemeral session versus durable M04 state; tentative lifecycle, failure and security boundaries; dimensional resource/telemetry requirements; candidate tests; compatibility conditions and explicit STOP. The exact versioned public/internal DTOs, exports, crate/file map, dependency/transport admission, approved host types, security policy, calibrated numeric resource caps, migration and final acceptance/DoD **remain UNFROZEN** for later rounds. Prefer one bounded module Work Order after final freeze, not speculative implementation here.

OUT OF SCOPE: Rust code, direct host invocation, installing credentials/providers, selecting model/tool/policy, remote listener, full MCP SDK, database, LEGACY_PROVIDER memory/RAG, M04 source or product contract amendment, synthetic external compatibility proof, checkpoint promotion, numeric performance claims.

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
- **LEGACY_PROVIDER:** LEGACY_PROVIDER MCP protocol `mcp-core-surface-v1` is not a Git release tag or proof that LEGACY_PROVIDER v1.0.0 is running on the owner's machine; local LEGACY_PROVIDER issue #4 and M23 integration authority are separate.

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
8. SOLO mode works with missing LEGACY_PROVIDER/remote endpoint and does not require a model, GitHub token or privileged OS operation.
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
| Reuse first-party `core-ipc` Unix socket/Windows local-only named pipe | Existing M01 `Frame`, `Handshake`, current CR1 header/version/epoch/limit checks and already implemented platform adapters. First-party isolated worker remains independent of LEGACY_PROVIDER or remote service. | M01 supervisor owns launch/drain; M11/M22 still must verify executable/OS peer, path/ACL and effective authority. `negotiate` does not verify required features/quality or third-party identity. No separate M05 framing codec. | **Reuse the existing admitted M01 primitive**, with M05 as a candidate session/mediation wrapper only. This is not an additional transport selection or M05 code approval. |
| Child process with separate stdio streams | Candidate narrow external adapter when a tool/host already supports a versioned framed/line protocol. Avoids remote network prerequisite but requires an independently governed subprocess boundary. | A hostile executable, malformed stdout/notification floods, mixed log/control output, deadlock on full stderr pipe, process-tree cleanup, leaked environment and ambiguous external effects. M01 supervises and M11/M12/M22 admit process execution. | **Evaluate**, not required or selected until executable provenance, framing and sandbox tests support it. Do not repurpose stdio diagnostic output as trusted semantics. |
| MCP client/protocol bridge | Candidate for tools served by a separately configured process or endpoint. The existing LEGACY_PROVIDER read-only MCP surface is an optional context-provider example, **not** proof of installed v1.0.0 on the owner's machine. | MCP protocol release/version and server software release are distinct; tools/list is untrusted data, server prompts/resources can contain hostile instructions, and transport can be stdio or remote with different trust models. Policy/lease/auth is not defined by MCP discovery. | **Evaluate as an external protocol adapter after separate admission**. Do not fork the LEGACY_PROVIDER retrieval/memory implementation or silently require an MCP SDK dependency. |
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

This appendix makes **no** change to M01, its accepted IPC wire protocol or single-account GEF rules. No M05 public request/receipt, crate, dependency, external command, network listener, LEGACY_PROVIDER client or numeric threshold is frozen. The ephemeral host-session proposal remains acyclic: M01 provides runtime/codec/registry, M05 observes mediated transport; M06 selects, M10/M11/M12 authorize and perform effects, M04 alone commits durable Run state, M18 reconciles unknown side effects, M22 sets security policy, M23 federates LEGACY_PROVIDER. The pending M04 external prior-V1 journal/binary/consumer inventory #111 and local LEGACY_PROVIDER/Docker/Codex issue #4 cannot be replaced by GitHub-only source proof.

**Round 3 STOP CONDITION:** promote this source-grounded **planning** appendix plus separate evidence record only after exact-head docs-only CI, bounded logical owner self-audit NOT INDEPENDENT with zero unresolved HIGH/CRITICAL, protected squash merge and full real main-push validation. Round 4 may propose a candidate semantic contract/acceptance graph only if it does not freeze M04-bound DTOs or silently choose unadmitted transport/dependency/numeric limits. Final planning freeze and a separate Work Order execution admission are still prerequisites for M05 code.


---

## Round 4 — candidate session laws, provisional acceptance graph and final-freeze prerequisites

Status: R4_NON_AUTHORITATIVE_CONTRACT_LAW_CANDIDATE  
Work Order: https://github.com/KayzenRoot/core/issues/145  
Exact initial R4 basis: fe4330b4abf772d9bb5f24615b5a3ffe05f13744  
Public M05 schemas, resource defaults, dependency set, execution admission: ALL UNFROZEN

Round 4 makes the **M04-independent** portion of the future adapter/session contract testable at the planning level while explicitly denying any product code or premature durable-M04 binding. The prior R1–R3 boundaries remain unchanged. Real old-M04-V1 external consumption is UNKNOWN under issue #111; proposed M04 V1 source PR #118 is DRAFT and implementation PR #106 has not been promoted. Consequently all invocation fields that embed M04 durable Run/BRC/ICF/idempotency bytes remain BLOCKED and cannot be guessed here. LEGACY_PROVIDER local v1.0.0/Codex proof issue #4 is separate.

### R4.1 — Boundary DTO *concepts*, not frozen public types

A future **HostSessionObservation** could comprise a fixed version/kind tag, ephemeral session identity, exact M01 boot epoch, adapter-generation number, transport class and protocol identity, observed negotiated version/finite frame cap, separately verified peer-proof **reference**, claimed external feature metadata tagged UNTRUSTED, bound registry provider/binding fingerprints, immutable failure/health state and a bounded reason code. No host-provided string may self-assign `Verified` trust or overwrite a policy/peer-proof reference; an observation is not a provider admission receipt.

A future **HostInvocationEnvelope** should carry the *caller's* prior policy authorization reference, separately validated M06/M01 capability lease and runtime generation, stable operation/correlation identity, session-generation binding, method/schema identity, finite payload/deadline/budget and a cancellation hook. It must not mint scope from natural-language task content. The exact representation of M04 Run/Attempt/Step linkage, external idempotency, persistent outcomes and journal append remains BLOCKED until M04 issue #111 has an authorized V1-all-NO or separately governed V2 disposition, new source/Context Lock admission and fresh owner review. A successful M05 transport handshake or caller request is not that admission.

A future **HostInvocationObservation** must separately encode whether bytes were definitely not delivered, delivery is acknowledged but effect unknown, the effect owner supplied independently validated proof, or transport failed without being able to disprove a previous side effect. The default after possible delivery followed by crash/cancel/timeout/disconnect is `EXTERNAL_EFFECT_UNKNOWN`, even when the local task's timeout handler returns. M05 must not use a successful stdout line, MCP response or wall-clock timestamp to mark M04 completed or authorize an automatic retry. Diagnostics are typed, size-bounded and redacted; raw credentials, paths, environment, host text and prompt content never become canonical result/evidence authority.

Candidate logical error families (not serialized enum names yet): `PROTOCOL_INVALID`, `PEER_UNVERIFIED`, `REQUIRED_FEATURE_MISSING`, `AUTHORITY_ABSENT_OR_STALE`, `SESSION_STALE`, `FRAME_RESOURCE_LIMIT`, `HOST_UNAVAILABLE`, `EXTERNAL_EFFECT_UNKNOWN`, `CANCELLATION_INCOMPLETE`, `OUTPUT_UNTRUSTED`, `INTERNAL_CONTRACT_VIOLATION`. Fatal versus retryability is decided by the effect/policy owner, not inferred from a transport exception.

### R4.2 — Provisional session transition and authority matrix

The following are **planning-only** laws, not a final API or completed implementation state-machine proof:

| Current state | Event and independently required fact | Candidate new state/outcome | Unsafe transition denied |
| --- | --- | --- | --- |
| `DISCOVERED` | Verified configuration, bounded source/provenance and M01 epoch supplied by trusted caller | `CONFIG_VALIDATED` | Discovery alone to READY or host invocation |
| `DISCOVERED/CONFIG_VALIDATED` | Unknown executable origin, unapproved transport, invalid policy/source or absent finite budgets | `REJECTED` | Downgrade to a less-trusted implicit fallback |
| `CONFIG_VALIDATED` | M01-authorized transport connected and validated peer-proof presented | `HANDSHAKING` | Self-declared host fingerprint elevated to trusted peer |
| `HANDSHAKING` | Compatible major, exact M01 epoch, finite frame cap, caller/M06 required-minor/features verified and independent peer proof | `READY` | `core-ipc::negotiate` alone taken as authority to invoke |
| `HANDSHAKING` | Bad CR1 header, unsupported major, stale epoch, invalid endpoint/feature or resource exhaustion | `REJECTED` or `QUARANTINED` according to *later* policy | Accept partial/invalid messages as successful handshake |
| `READY` | Valid **current** caller policy + capability lease + session generation + M11/M12 action admission | `READY` with bounded permitted invocation observation | Sending merely because transport or tool list is available |
| `READY` | M01 parent shutdown/drain or explicit bounded caller revocation | `DRAINING` | New action accepted after drain barrier |
| `READY/DRAINING` | Host crash/disconnect or stale runtime epoch/generation | `LOST`, in-flight effects `UNKNOWN` unless independently proven | Late reply on old generation reactivates session |
| `READY/HANDSHAKING` | Proven malicious/unverifiable peer or policy quarantine | `QUARANTINED` | Automatic reconnect/unquarantine as trusted without M01/M22 |
| `DRAINING` | Finite cleanup; in-flight unknown effects captured for effect owner | `CLOSED` with bounded closeout receipt | Claim shutdown resolved ambiguous external actions |
| `REJECTED/LOST/QUARANTINED/CLOSED` | New M01-authorized generation, separate new endpoint admission | NEW `DISCOVERED` identity, not mutation of terminal session | Reuse old identity or late old-epoch result as current |

Pure guard ordering proposal: first verify session epoch/generation, then independent peer authority and current external lease/policy receipt, then compatibility/finite budgets, and only then make a caller-authorized request *eligible*. Future negative tests must cover simultaneous revocation and send, late reply versus drain, reset after quarantine and cancellation racing actual side effects. Terminal history belongs to M04 if an M04 record exists; this ephemeral session table does not supersede M04's immutable journal.

### R4.3 — External effect, idempotency and source integrity laws

`read_frame` now rejects invalid CR1 fixed-header fields **before reading/allocating body** after separately audited PR #140. The M01 codec already prevents known-malformed stream headers causing unnecessary body waits; neither it nor M05 proves host identity, minimum required feature, correct tool result or exactly-once effect. Keep its source and responsibility unchanged.

Proposed *delivery proof* classes: `NOT_SENT_PROVEN`, `MAYBE_SENT_OR_ACK_ONLY` and `EFFECT_OWNER_PROVEN`. These are conceptual and must be backed by real verifiable source evidence before being asserted. A successful local write, process completion or response framing cannot distinguish an action that committed remotely from one that returned no acknowledgment. If the adapter cannot **prove** non-delivery, classify as possible delivery, preserve immutable input id/correlation and return UNKNOWN. Never replay a potentially non-idempotent external operation in M05, create a new M04 Attempt or falsify a cleanup receipt; M18/effect-owner policy must separately reconcile.

An optional future provider idempotency token is a **provider contract claim**, not automatic durable safety. M04's canonical idempotency domain/fingerprint is not known to M05 at this gate and must not be copied from blocked draft PR #118. M05 correlation identities scope one session/epoch/generation, and any cross-epoch dedup or persisted replay must be separately admitted and independently tested.

### R4.4 — Security/data classification and negative-test obligations

Threat inputs: fake host executable/process ID, forged peer certificate/manifest, symlink/pipe-path substitution, Windows local-pipe impersonation, Unix socket permissions, valid CR1 bytes from an unauthorized peer, version/minor downgrade, untrusted stdout/MCP tool descriptions, oversized or partial streams, stale capability lease, cancellation/host restart, prompt injection and poisoned evidence references.

All independently verifiable trust claims must come from M11/M22 policy-provenance sources, not the external host's self-authored metadata. Capability compatibility/binding remains M06, actual action permission and sandbox M10/M11/M12, external retry M18, secret lifecycle M22, costs M19 and LEGACY_PROVIDER intelligence/federation M23. Any stdout/stderr/MCP resource text is tainted; no content from it may silently become canonical Work Order edits, executable shell argv, authority, verification verdict or privileged Git operation. A headless SOLO checkout must remain usable without LEGACY_PROVIDER/remote process and truthful when an optional adapter is absent.

All limits and deadlines must be finite/validated as per R3 dimensions **before product acceptance**, but R4 invents no numeric defaults and publishes no unmeasured performance improvement. Windows and Unix need separate peer/security, cancellation/drain and IPC fixtures; successful identical payload frames are not proof of identical OS privilege behavior.

### R4.5 — Provisional M05 acceptance-evidence graph, every node PENDING

The below is a **future** traceability candidate. It neither creates a final frozen AEG nor implies that passing existing M01 tests proves M05. Conditional nodes may become explicitly NOT APPLICABLE **only** through the final governed transport/Scope freeze; they cannot be silently deleted.

| Future node | Evidence to require before *implemented* M05 acceptance |
| --- | --- |
| EV-M05-001 | Exact admitted public contract/version schema plus backwards/unsupported-major compatibility tests. |
| EV-M05-002 | Exhaustive ephemeral session-state legal transition table and deterministic receipts. |
| EV-M05-003 | Illegal transition/property tests, terminal/quarantine and no hidden recovery. |
| EV-M05-004 | M01 CR1 canonical framing/handshake golden vectors reused through the actual `core-ipc` boundary. |
| EV-M05-005 | M06 required-minor/feature/protocol downgrade and critical-unknown refusal. |
| EV-M05-006 | Runtime epoch, adapter generation, correlation and stale/late cross-session rejection. |
| EV-M05-007 | Independent peer provenance/OS identity/source proof for each admitted transport. |
| EV-M05-008 | Host-supplied manifest/protocol fingerprint spoofing and trust-escalation negative fixtures. |
| EV-M05-009 | Live M01/M06 lease expiry, revocation, binding-fingerprint/generation mismatch rejection. |
| EV-M05-010 | M10/M11/M12 caller action-admission receipt mandatory before any effectful send. |
| EV-M05-011 | Exact caller Scope/Work Order/reference provenance; no ambient working directory or host-reported path authority. |
| EV-M05-012 | Finite frame/request/response/resource budget cap-1/cap/cap+1 atomic refusal. |
| EV-M05-013 | Malformed/fragmented/oversized/deep/notification-flood protocol/property/fuzz corpus. |
| EV-M05-014 | If admitted, child-process/stdio process-tree isolation and stdout/stderr/deadlock/kill tests; otherwise formally scoped-out evidence. |
| EV-M05-015 | If admitted, MCP client tool/resource/prompt injection, version/feature and tainted-output tests; otherwise formally scoped-out evidence. |
| EV-M05-016 | Remote disabled by default and no hidden network; if later admitted, separate TLS/auth/egress threat/evidence. |
| EV-M05-017 | Cancel/timeout/crash between before-send/possible-delivery/after-reply with mandatory UNKNOWN classification. |
| EV-M05-018 | Caller-owned idempotency, duplicate/late result refusal and no automatic ambiguous-effect replay. |
| EV-M05-019 | M01 process/worker crash, restart, quarantine, drain and prior-epoch reply revocation. |
| EV-M05-020 | SOLO local-first operation and explicit absence of optional host/LEGACY_PROVIDER dependencies. |
| EV-M05-021 | Optional LEGACY_PROVIDER read-only context bridge: actual owner-host runtime/client proof separately captured if claimed, never inferred from Git tags or protocol identity. |
| EV-M05-022 | Secret/credential/host text/path/diagnostic redaction, prompt-injection taint and bounded receipt payloads. |
| EV-M05-023 | Zero-LLM deterministic transport/session core and no hidden Git/network/process/database mutation outside admitted host ports. |
| EV-M05-024 | Reproducible same-platform M01-baseline comparisons, resource calibration and selected/rejected finite budget evidence. |
| EV-M05-025 | Exact-head Ubuntu workspace/integration/hostile fixtures and bounded fuzz where applicable. |
| EV-M05-026 | Exact-head Windows workspace/integration/hostile fixtures, including local-pipe security boundary. |
| EV-M05-027 | Exact-head dependency/license/advisory/provenance/SBOM and architecture graph proof; no unapproved SDK/new deps. |
| EV-M05-028 | KayzenRoot owner exact-head self-audit explicitly NOT INDEPENDENT, no unresolved HIGH/CRITICAL and protected-push verification. |

The R1/R2 discovery identifiers `EV-M05-D01..D14` and R3 fixture directions are planning ancestry, not already completed EV-M05-001..028 implementation evidence. M04's **different** EV-M04-001..023 remain globally pending. A future final frozen AEG must explicitly link each accepted requirement/criterion to evidence owner, fixture, platform and STOP gate before code is authorized.

### R4.6 — File-map-to-evidence and the 19-dimension planning gap

The R3 **candidate** `crates/core-host-adapter` single-crate map remains conditional. The eventual contract/session/provenance/budget/outcome modules should have pure/fake-port law tests that exercise most EV-M05-001..013 and 017..023 with no actual process or M04 store. OS-local `core-ipc` wrapper fixtures exercise EV-M05-004/006/007/025/026. Any future stdio/MCP/remote adapter must receive separate path/dependency, sandbox, optionality and negative fixture admission, not be declared mandatory by the candidate file table. The executable final file map, exact Cargo dependencies, public crate exports, M04 lineage DTOs and transport selection remain UNFROZEN.

Current planning dimension coverage: mission/ownership and LEGACY_PROVIDER non-duplication documented R1; SOLO/optional LEGACY_PROVIDER, trust and failure bounds R1–R2; candidate transport/dependency/file map plus calibration protocol R3; provisional contract laws/AEG, STOP and security/DoD traceability this R4. **Pending for final freeze:** authoritative M04 V1 or V2 compatibility disposition and admitted immutable source/lock; exact M06/M10/M11/M12 action-admission and peer-provenance port contracts; selected host types and their SDK/dependency threat review; measured finite numeric budget defaults and regression floors; admitted final crate/file map/trait API; immutable Work Order+Context Lock handoff; actual local LEGACY_PROVIDER evidence only if such integration is claimed; reviewer-admitted implementation DoD and redacted Evidence Bundle. No missing dimension is silently represented as complete.

### R4.7 — Round 5 entry gate and STOP

**Round 4 STOP:** this appendix and separate source-bound evidence record pass exact-head docs-only Governance and required successful status contexts; a separate bounded logical owner audit `OWNER_SELF_AUDIT_APPROVED / NOT INDEPENDENT` records source/lock binding, zero unresolved HIGH/CRITICAL and zero review threads; protected squash merge is followed by **real full** main-push CI 11/11. Closure means R4 planning **candidate documented only**, never admission of a public API, M05 code, a new transport, arbitrary host authority, numeric resource values or M04 durable-contract version.

**Round 5 prerequisite:** after real external M04 consumer inventory issue #111 yields an owner/source-governed V1-no-legacy or separate versioned V2 archival/migration disposition, explicitly reconcile accepted M04 handoff with M05's caller-owned opaque effect references; obtain separately admitted M06/M10/M11/M12/M22 trust/authority contract details, transport/dependency candidate acceptance and measured calibration. Only then compile a final proposed M05 Work Order, pending Context Lock, source/criteria/evidence graph and executor handoff. A distinct reviewed/promoted execution-admission delta remains required before product code, and LEGACY_PROVIDER #4 is never auto-proven by this planning work.
