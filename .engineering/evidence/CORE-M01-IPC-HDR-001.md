# CORE-M01-IPC-HDR-001: stream header early-rejection correction

Status: CODE_CANDIDATE / NO_PRODUCTION_SECURITY_ATTESTATION
Work Order: https://github.com/KayzenRoot/core/issues/139
Exact initial main: `8e68182514318a91502d4babffafb957485dade9`
Exact original M01 IPC source blob: `ff9e63800811f627ea02057daf122cfae6056db8`

## Source-backed defect and bounded correction

The previous `read_frame` read the fixed 19-byte CR1 header, checked only the declared length, then allocated/read the entire permissible payload before `decode` checked bad magic, incompatible major and stale runtime epoch. This permits unnecessary bounded body buffering and waiting for a peer whose already-received header proves it is invalid. No real-world exploit or unbounded allocation claim is made.

Introduce one pure `checked_header` used by both `decode` and `read_frame`; its rejection order remains: truncated header -> bad magic -> wrong major -> oversize length -> stale epoch. Validate all known fixed-header evidence **before any body allocation or read**; preserve protocol 1.0 wire bytes, 19-byte header, function signatures and existing `IpcError` variants. Use checked header+length arithmetic instead of overflow-prone addition. Keep existing Unix sockets, Windows named pipes, registry/supervisor and caller-owned timeout semantics unchanged.

## Regression and assurance gates

Added an in-file deterministic Tokio duplex test that supplies just an invalid fixed header then disconnects, without delivering the declared nonzero body. It checks typed `BadMagic`, `ProtocolMismatch`, `StaleEpoch` and `FrameTooLarge`, rather than misleading `Io(UnexpectedEof)`; existing valid framing, platform IPC and handshake tests remain. No new dependencies or numeric resource limits. No automatic proof of third-party endpoint authentication: header validity and authorized process identity are separate M11/M22 obligations, while M05 Round 2 remains non-authoritative discovery.

Record exact-head 11/11 hosted CI including M01 Ubuntu/Windows Clippy/tests/supply-chain/SBOM/PRB/fuzz as applicable, GEF logical owner audit explicitly NOT INDEPENDENT with zero unresolved HIGH/CRITICAL, protected squash merge and full **new main-push** 11/11 before closing. Historical CI alone is insufficient. All canonical M04 sources/lock, M05 docs, M04 issue #111 and local HIVE issue #4 remain unaffected.
