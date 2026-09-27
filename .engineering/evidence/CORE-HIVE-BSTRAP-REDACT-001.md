# CORE-HIVE-BSTRAP-REDACT-001: local setup diagnostic privacy hardening

Status: CODE_CANDIDATE / NOT_ACTUAL_LOCAL_HIVE_PROOF  
Initial exact main: `1ea8a043a38b6000607306dc40f2dc459d82b7c3`  
Work Order: https://github.com/KayzenRoot/core/issues/148

## Exact source defect and bounded patch

Existing `scripts/hive_bootstrap.py` blob `e44498a367c6fa5ea5e182b5bf14d697f3fb12e5` emitted whole health payloads, project UUID and relative path in the success summary, and raw HTTP error bodies, endpoint/connection details and arbitrary API inspect/corpus payloads on failure. Those values can contain private machine paths and sensitive service metadata. The separately shipped read-only `scripts/hive_evidence.py` and `scripts/hive_mcp_probe.py` deliberately redact them, but normal bootstrap logs could still be shared accidentally.

This bounded correction preserves stateful HIVE v1.0.0 `GET health -> GET/POST project -> POST inspect -> POST index -> POST corpus sync` order, while printing only static progress, a syntactically validated 40-hex Git HEAD and typed final status/enums. HTTP status uses a numeric typed reason, URL errors never echo URLs or provider messages, and unexpected exceptions emit only a static generic reason. Registry collision, non-ready inspect, bad Git head and incomplete indexing/corpus fail typed. Tests use synthetic canary paths, UUID, API body/connection failures and no Docker or user files. There is no new dependency or change to HIVE API's actual call structure.

## Assurance and external boundary

Obtain new exact-head required CI/pytest/unittest and logical GEF self-audit `NOT INDEPENDENT`, zero unresolved HIGH/CRITICAL, protected squash merge and independent FULL 11/11 main-push before closing. No real HIVE v1.0.0 Docker, local project reindex, MCP server or separate Codex client was executed through GitHub review. Parent actual-runtime [issue #4](https://github.com/KayzenRoot/core/issues/4) and M04 prior-V1 compatibility [#111](https://github.com/KayzenRoot/core/issues/111) remain OPEN; unmerged M04 source/implementation #118/#106 and active M04 Context Lock are untouched.
