from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

REQUIRED = (
    "AGENTS.md",
    ".codex/config.toml",
    ".engineering/SOURCE-HIERARCHY.md",
    ".engineering/PROJECT-OVERVIEW.md",
    ".engineering/CHECKPOINT.md",
    ".engineering/CHECKPOINT.json",
    ".engineering/BOOTSTRAP-MANIFEST.json",
    ".engineering/gef/GEF-ADOPTION.md",
    ".engineering/gef/GEF-PROJECT-PROFILE.json",
    ".engineering/gef/GEF-SOURCE-BRIDGE.json",
    ".engineering/gef/GEF-POLICY.md",
    ".engineering/gef/GEF-EXECUTION-PROTOCOL.md",
    ".engineering/gef/GEF-REVIEW-PROTOCOL.md",
    ".engineering/gef/GEF-EVIDENCE-SPEC.md",
    "docs/project-brain/01-PROJECT-OVERVIEW.md",
    "docs/project-brain/02-REQUIREMENTS.md",
    "docs/project-brain/03-SCOPE.md",
    "docs/project-brain/04-ARCHITECTURE.md",
    "docs/project-brain/10-SECURITY-GOVERNANCE.md",
    "docs/project-brain/11-TEST-PLAN.md",
    "docs/project-brain/12-LOCAL-DEPLOYMENT.md",
    "docs/project-brain/13-CHECKPOINT.md",
    "docs/project-brain/14-BACKLOG.md",
    "docs/project-brain/15-DEFINITION-OF-DONE.md",
    "docs/project-brain/16-DECISIONS-LEDGER.md",
    "docs/HIVE-INTEGRATION.md",
    "scripts/hive_mcp.py",
)

HIVE_GOVERNANCE = (
    "docs/project-brain/13-CHECKPOINT.md",
    "docs/project-brain/03-SCOPE.md",
    "docs/project-brain/15-DEFINITION-OF-DONE.md",
    "docs/project-brain/04-ARCHITECTURE.md",
    "docs/project-brain/16-DECISIONS-LEDGER.md",
)

CHECKPOINT_HEADINGS = (
    "## STATUS",
    "## VERSION",
    "## PHASE",
    "## OBJECTIVE",
    "## IN PROGRESS",
    "## BLOCKERS",
    "## NEXT STEP",
)

SHARED_CHECKPOINT_FIELDS = {
    "status": "## STATUS",
    "version": "## VERSION",
    "phase": "## PHASE",
    "nextStep": "## NEXT STEP",
}


def fail(message: str) -> None:
    raise SystemExit(f"GOVERNANCE VALIDATION FAILED: {message}")


def section(text: str, heading: str) -> str:
    lines = text.splitlines()
    try:
        start = lines.index(heading) + 1
    except ValueError:
        fail(f"checkpoint missing heading: {heading}")
    values: list[str] = []
    for line in lines[start:]:
        if line.startswith("## "):
            break
        if line.strip():
            values.append(line.strip())
    if not values:
        fail(f"checkpoint heading has no value: {heading}")
    return "\n".join(values)


for relative in REQUIRED:
    path = ROOT / relative
    if not path.is_file():
        fail(f"missing required file: {relative}")

checkpoint = (ROOT / HIVE_GOVERNANCE[0]).read_text(encoding="utf-8")
for heading in CHECKPOINT_HEADINGS:
    section(checkpoint, heading)

bridge_checkpoint = (ROOT / ".engineering/CHECKPOINT.md").read_text(encoding="utf-8")
machine_checkpoint = json.loads(
    (ROOT / ".engineering/CHECKPOINT.json").read_text(encoding="utf-8")
)
if machine_checkpoint.get("canonicalCheckpoint") != HIVE_GOVERNANCE[0]:
    fail("machine checkpoint does not point to canonical HIVE checkpoint")
for field, heading in SHARED_CHECKPOINT_FIELDS.items():
    canonical_value = section(checkpoint, heading)
    bridge_value = section(bridge_checkpoint, heading)
    machine_value = machine_checkpoint.get(field)
    if bridge_value != canonical_value:
        fail(f"human GEF checkpoint drift for {field}")
    if machine_value != canonical_value:
        fail(f"machine GEF checkpoint drift for {field}")

profile = json.loads(
    (ROOT / ".engineering/gef/GEF-PROJECT-PROFILE.json").read_text(encoding="utf-8")
)
if profile.get("gefVersion") != "1.0.0":
    fail("GEF profile is not pinned to v1.0.0")
expected_hierarchy = (
    "docs/project-brain/13-CHECKPOINT.md",
    "docs/project-brain/16-DECISIONS-LEDGER.md",
    "docs/project-brain/03-SCOPE.md",
    "docs/project-brain/15-DEFINITION-OF-DONE.md",
    "docs/project-brain/04-ARCHITECTURE.md",
    "docs/project-brain/02-REQUIREMENTS.md",
)
if tuple(profile.get("sourceHierarchy", ())[:6]) != expected_hierarchy:
    fail("GEF source hierarchy does not match CORE canonical order")

source_bridge = json.loads(
    (ROOT / ".engineering/gef/GEF-SOURCE-BRIDGE.json").read_text(encoding="utf-8")
)
if source_bridge.get("canonicalCheckpoint") != HIVE_GOVERNANCE[0]:
    fail("GEF source bridge checkpoint mismatch")
bridge_sources = source_bridge.get("domains", {})
for domain, relative in {
    "PROJECT_STATE": "docs/project-brain/13-CHECKPOINT.md",
    "DECISION": "docs/project-brain/16-DECISIONS-LEDGER.md",
    "SCOPE": "docs/project-brain/03-SCOPE.md",
    "COMPLETION": "docs/project-brain/15-DEFINITION-OF-DONE.md",
    "ARCHITECTURE": "docs/project-brain/04-ARCHITECTURE.md",
    "REQUIREMENT": "docs/project-brain/02-REQUIREMENTS.md",
    "SECURITY": "docs/project-brain/10-SECURITY-GOVERNANCE.md",
    "VALIDATION": "docs/project-brain/11-TEST-PLAN.md",
    "DEPLOYMENT": "docs/project-brain/12-LOCAL-DEPLOYMENT.md",
}.items():
    if bridge_sources.get(domain) != relative:
        fail(f"GEF source bridge mismatch for {domain}")

manifest = json.loads(
    (ROOT / ".engineering/BOOTSTRAP-MANIFEST.json").read_text(encoding="utf-8")
)
if manifest.get("gef", {}).get("version") != "1.0.0":
    fail("bootstrap manifest GEF pin mismatch")
if manifest.get("gef", {}).get("releaseCommit") != "866fe3af8cccc65c929aaf6a47a924401fa448b3":
    fail("bootstrap manifest GEF release commit mismatch")
if manifest.get("hive", {}).get("version") != "1.0.0":
    fail("bootstrap manifest HIVE pin mismatch")
if manifest.get("hive", {}).get("releaseCommit") != "a53b5b9fcf55c32a5696180fb1b1ef80ccd1edcf":
    fail("bootstrap manifest HIVE release commit mismatch")

for relative in HIVE_GOVERNANCE:
    if not (ROOT / relative).is_file():
        fail(f"HIVE governance path missing: {relative}")

print("CORE governance validation: PASS")
print("GEF: v1.0.0 @ 866fe3af8cccc65c929aaf6a47a924401fa448b3")
print("HIVE compatibility: v1.0.0 @ a53b5b9fcf55c32a5696180fb1b1ef80ccd1edcf")
print("GEF checkpoint/source bridges: CONSISTENT")
print(f"Required artifacts: {len(REQUIRED)}")
