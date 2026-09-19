from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

REQUIRED = (
    "AGENTS.md",
    ".engineering/SOURCE-HIERARCHY.md",
    ".engineering/BOOTSTRAP-MANIFEST.json",
    ".engineering/gef/GEF-ADOPTION.md",
    ".engineering/gef/GEF-PROJECT-PROFILE.json",
    ".engineering/gef/GEF-POLICY.md",
    ".engineering/gef/GEF-EXECUTION-PROTOCOL.md",
    ".engineering/gef/GEF-REVIEW-PROTOCOL.md",
    ".engineering/gef/GEF-EVIDENCE-SPEC.md",
    "docs/project-brain/02-REQUIREMENTS.md",
    "docs/project-brain/03-SCOPE.md",
    "docs/project-brain/04-ARCHITECTURE.md",
    "docs/project-brain/13-CHECKPOINT.md",
    "docs/project-brain/14-BACKLOG.md",
    "docs/project-brain/15-DEFINITION-OF-DONE.md",
    "docs/project-brain/16-DECISIONS-LEDGER.md",
    "docs/HIVE-INTEGRATION.md",
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


def fail(message: str) -> None:
    raise SystemExit(f"GOVERNANCE VALIDATION FAILED: {message}")


for relative in REQUIRED:
    path = ROOT / relative
    if not path.is_file():
        fail(f"missing required file: {relative}")

checkpoint = (ROOT / HIVE_GOVERNANCE[0]).read_text(encoding="utf-8")
for heading in CHECKPOINT_HEADINGS:
    if heading not in checkpoint:
        fail(f"checkpoint missing heading: {heading}")

profile = json.loads(
    (ROOT / ".engineering/gef/GEF-PROJECT-PROFILE.json").read_text(encoding="utf-8")
)
if profile.get("gefVersion") != "1.0.0":
    fail("GEF profile is not pinned to v1.0.0")
if tuple(profile.get("sourceHierarchy", ())[:6]) != (
    "docs/project-brain/13-CHECKPOINT.md",
    "docs/project-brain/16-DECISIONS-LEDGER.md",
    "docs/project-brain/03-SCOPE.md",
    "docs/project-brain/15-DEFINITION-OF-DONE.md",
    "docs/project-brain/04-ARCHITECTURE.md",
    "docs/project-brain/02-REQUIREMENTS.md",
):
    fail("GEF source hierarchy does not match CORE canonical order")

manifest = json.loads(
    (ROOT / ".engineering/BOOTSTRAP-MANIFEST.json").read_text(encoding="utf-8")
)
if manifest.get("gef", {}).get("version") != "1.0.0":
    fail("bootstrap manifest GEF pin mismatch")
if manifest.get("hive", {}).get("version") != "1.0.0":
    fail("bootstrap manifest HIVE pin mismatch")

for relative in HIVE_GOVERNANCE:
    if not (ROOT / relative).is_file():
        fail(f"HIVE governance path missing: {relative}")

print("CORE governance validation: PASS")
print("GEF: v1.0.0")
print("HIVE compatibility: v1.0.0")
print(f"Required artifacts: {len(REQUIRED)}")
