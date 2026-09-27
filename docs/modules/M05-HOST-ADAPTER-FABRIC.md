# M05 Host Adapter Fabric — Round 1 discovery candidate

Status: NON_AUTHORITATIVE_DISCOVERY_R1_CANDIDATE  
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

Round 2 may evaluate \`DISCOVERED -> CONFIG_VALIDATED -> HANDSHAKING -> READY -> DRAINING -> CLOSED\`, with typed \`REJECTED/QUARANTINED/LOST\` paths. These labels are candidate names, **not** a frozen state machine.

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
