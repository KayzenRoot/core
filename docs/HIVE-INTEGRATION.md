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

## Local read-only, redacted evidence collector

After the existing `hive_bootstrap.py` command has registered and indexed the
actual CORE checkout, run this helper from **that same local CORE checkout**:

```powershell
python scripts/hive_evidence.py --relative-path core
```

Use `--relative-path projects/core` if CORE is nested under
`HIVE_PROJECTS_ROOT`, and optionally `--hive-repo <your HIVE checkout>` if
the HIVE checkout is not adjacent or specified in `HIVE_REPO_PATH`.

The collector makes **read-only** requests to a loopback-only HIVE API and
verifies the local checkout is the exact HIVE `v1.0.0` release commit. It
compares actual local CORE Git HEAD with HIVE's registered inspection and
completed index, checks the corpus is CURRENT for the **same** index run,
and outputs only safe release/Git SHA/status fields. It never emits raw
health payloads, UUIDs, local absolute paths, environment, credentials or
retrieved documents. `BLOCKED` means the local evidence is not complete,
not that the repo's CI or GitHub API failed; rerun the ordinary
`hive_bootstrap.py` preparation if local HIVE inspection/index is stale.

Paste the redacted helper JSON into Issue #4. The helper intentionally prints
`PENDING_REAL_CODEX_TOOL_INVOCATIONS` for the final MCP proof, which must
come from actual bounded `checkpoint.read` and `context.search` tool calls
made in the connected Codex client. Do not change that field manually or
claim that a local check happened just because unit tests passed in CI.

## Verify the local HIVE MCP server without exposing retrieved data

After the released redacted HIVE API/index/corpus helper succeeds on your real
local machine, run from the **same clean and indexed local CORE checkout**:

```powershell
python scripts/hive_mcp_probe.py --relative-path core
```

The MCP probe first requires the same full local HIVE v1.0.0 release and
fresh/current indexed CORE proof as `hive_evidence.py`. It then starts the
real v1.0.0 MCP server through `docker compose exec -T api python -m app.mcp_server`,
establishes a bounded JSON-RPC session, verifies the exact seven stable
read-only tools, and makes the real `checkpoint.read` and two-result-bounded
`context.search` calls. It prints only safe release/head, tool-count and
boolean/count outcomes, **never** complete checkpoints, private search snippets,
UUIDs, local machine paths or Docker logs. Failures emit typed `BLOCKED`.

Use `--hive-repo` when your HIVE checkout is not a sibling and no
`HIVE_REPO_PATH` is configured. The actual CORE checkout must have a clean
tracked Git worktree: the official HIVE `checkpoint.read` rejects dirty
project inventories even if an old project registry response still says READY.

**Important boundary:** this verifies the real local HIVE **server**, not
registration and invocation by the separate **Codex client**. Issue #4 still
requires genuine redacted `checkpoint.read` and `context.search` calls made
inside that actual client before closing the end-to-end integration gate.
