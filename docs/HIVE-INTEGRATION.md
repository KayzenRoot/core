# HIVE v1.0.0 Integration

CORE is prepared for the stable HIVE v1.0.0 release at commit `a53b5b9fcf55c32a5696180fb1b1ef80ccd1edcf`.

## Architecture

HIVE remains a separate local-first runtime. CORE is mounted read-only beneath HIVE's configured `HIVE_PROJECTS_ROOT`.

Do not copy the HIVE backend, database or Docker stack into CORE.

## Required layout

HIVE v1.0.0 loads these exact governance files from tracked Git state:

- `docs/project-brain/13-CHECKPOINT.md`
- `docs/project-brain/03-SCOPE.md`
- `docs/project-brain/15-DEFINITION-OF-DONE.md`
- `docs/project-brain/04-ARCHITECTURE.md`
- `docs/project-brain/16-DECISIONS-LEDGER.md`

CORE materializes all five.

## Local HIVE setup

In the separate HIVE checkout:

1. checkout release `v1.0.0`;
2. set `HIVE_PROJECTS_ROOT` to the parent/root directory that contains the local CORE checkout;
3. start HIVE with its supported Docker Compose procedure;
4. verify `http://localhost:8000/api/v1/health`.

Then, from CORE:

```powershell
python scripts/hive_bootstrap.py --relative-path core
```

If CORE is nested below the configured root, pass the POSIX relative path, for example `projects/core`.

The script:
- verifies HIVE health;
- resolves or registers CORE;
- reinspects its Git state;
- triggers repository indexing;
- synchronizes the retrieval corpus;
- fails if CORE is not READY or indexing/retrieval sync is not current.

## Stable HIVE MCP surface

HIVE v1.0.0 exposes the read-only tools:

- `project.list`
- `project.status`
- `context.build`
- `context.search`
- `memory.search`
- `memory.get`
- `checkpoint.read`

The MCP server uses stdio. From the HIVE repository context the stable server module is started through the API container as:

```text
docker compose exec -T api python -m app.mcp_server
```

Configure the executor/MCP client so the command executes against the installed HIVE checkout. Machine-specific paths are intentionally not committed to CORE.

## Executor contract

Every implementation Work Order uses HIVE-first preflight. Start at checkpoint/project status, expand through relevant decisions/scope, then module/code/test evidence. Do not read the entire repository by default.

HIVE memory and retrieved summaries are derived context. Tracked canonical files and Git remain authoritative.


## Codex project-scoped MCP

CORE includes `.codex/config.toml` with a required STDIO server named `hive`. The launcher is `scripts/hive_mcp.py`.

The launcher resolves HIVE in this order:
1. `HIVE_REPO_PATH`, when defined;
2. a sibling checkout named `hive`;
3. a sibling checkout named `Hive`.

It then starts the stable HIVE MCP module through the already-running HIVE Docker Compose API service.

This keeps machine-specific filesystem paths out of Git while making HIVE mandatory for normal Codex work in a trusted CORE checkout. If HIVE is absent or Docker is unavailable, startup fails explicitly rather than silently dropping the intelligence layer.

After cloning CORE and HIVE side by side, the normal shape is:

```text
workspace/
  hive/
  core/
```

No project-local OpenAI credential or provider setting is committed.
