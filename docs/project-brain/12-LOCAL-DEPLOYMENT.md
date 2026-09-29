# CORE Local Deployment

CORE runs as a standalone local application/runtime from its own repository and Cargo workspace. No project Docker stack, repository MCP service, project-registration daemon or external memory database is required.

## Local baseline
- Rust toolchain and repository dependencies declared by the workspace;
- local filesystem/Git access only where explicitly owned by the applicable adapter;
- GitHub access only for delivery/CI workflows that explicitly require it;
- no ambient network or external context authority inside deterministic core modules.

Optional future external providers must be separately admitted through provider-neutral contracts and are never a startup dependency.
