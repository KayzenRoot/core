# CORE standalone cutover

**Owner direction:** 2026-09-28, Work Order [#172](https://github.com/KayzenRoot/core/issues/172). Original protected main: `b3d1d6e540759718c8af8e56c4a2ceaa97e16b41`.

**Phase 1 candidate:** remove all project LEGACY_PROVIDER bootstrap/MCP/API scripts, their dedicated tests, CI and governance mandatory runtime pin, repo Codex MCP config and environment template. Replace active executor guidance and module map with direct local canonical Git source authority. Retain historical audit logs as dated provenance only.

**Remaining separate migration gates:** remove LEGACY_PROVIDER Rust-specific `require_legacy_provider`, provider ownership/ranking, M02 LEGACY_PROVIDER-named association fields, runtime/soak fixtures and old optional synchronization semantics; re-admit frozen M04 canonical sources, Work Order/Context Lock and evidence fingerprints **together**, under an explicit new dated cutover decision. Old M04 prior-V1 external consumer [#111](https://github.com/KayzenRoot/core/issues/111) is still UNKNOWN. Do not claim user's actual PC uninstallation or run M06 implementation from this PR.

**STOP:** independent exact-head full CI and audit, protected squash merge and new real FULL main-push for this phase, followed by separately governed Rust/canonical source migration.
