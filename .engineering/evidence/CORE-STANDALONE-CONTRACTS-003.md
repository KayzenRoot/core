# CORE-STANDALONE-CONTRACTS-003: source-bound M02/M03 V2 migration candidate

Status: EXECUTION_CANDIDATE / NO_CI_OR_MERGE_CLAIM
Work Order: https://github.com/KayzenRoot/core/issues/178
Parent: https://github.com/KayzenRoot/core/issues/172
Original protected-main: 4ab40453a7050add41d6e65b1305979e565a4c41

Scope: rename only M02 and M03 obsolete external-project-service Rust and serialized fields into optional generic association/local canonical evidence, increase the *M02* envelope version and the *M03* serialized envelope version to 2, synchronize M03's exact admitted M02 version, update tests and append two dated source addenda. No new network/database/MCP/Docker dependency, no claim that a generic provider is verified by its own declaration, no implicit V1 decode, no change to canonical M04 frozen source, Work Order, active Context Lock or its evidence bundle. The last two remain old-bound until independently governed synchronized owner review; external prior-V1 consumer inventory #111 is UNKNOWN.

Future reviewed exact-head evidence required: full Governance and M01–M03 Linux/Windows Rust, all three bounded fuzz campaigns, Ubuntu dependency policy/advisory/SBOM/PRB, independent exact-main full run after protected squash; scoped logical KayzenRoot NOT INDEPENDENT review zero unresolved HIGH/CRITICAL/threads. This candidate makes no historical test-success assertion until the exact-head workflow is checked.
