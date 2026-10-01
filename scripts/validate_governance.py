from __future__ import annotations

import json
import re
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
RETIRED_PROVIDER_IDENTIFIER_PATTERN = rf"\\b{re.escape(bytes((104, 105, 118, 101)).decode('ascii'))}(?:\\b|_|external\\b)"

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
    ".engineering/context-locks/CORE-WO-M04-001.json",
    ".engineering/evidence/CORE-WO-M04-001.json",
)

CANONICAL_PROJECT_SOURCES = (
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
    """Reject a violated governance invariant with an explicit blocking reason."""
    raise SystemExit(f"GOVERNANCE VALIDATION FAILED: {message}")


def section(text: str, heading: str) -> str:
    """Extract one nonempty checkpoint section for deterministic bridge comparison."""
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


def git_blob_sha(relative: str) -> str:
    """Calculate Git's canonical blob ID using repository clean filters."""
    # Let Git apply clean filters (especially CRLF normalization on Windows)
    # before calculating the canonical blob ID.
    result = subprocess.run(
        ["git", "hash-object", f"--path={relative}", relative],
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    return result.stdout.strip()


for relative in REQUIRED:
    path = ROOT / relative
    if not path.is_file():
        fail(f"missing required file: {relative}")

checkpoint = (ROOT / CANONICAL_PROJECT_SOURCES[0]).read_text(encoding="utf-8")
for heading in CHECKPOINT_HEADINGS:
    section(checkpoint, heading)

bridge_checkpoint = (ROOT / ".engineering/CHECKPOINT.md").read_text(encoding="utf-8")
machine_checkpoint = json.loads(
    (ROOT / ".engineering/CHECKPOINT.json").read_text(encoding="utf-8")
)
if machine_checkpoint.get("canonicalCheckpoint") != CANONICAL_PROJECT_SOURCES[0]:
    fail("machine checkpoint does not point to canonical local Git checkpoint")
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
if source_bridge.get("canonicalCheckpoint") != CANONICAL_PROJECT_SOURCES[0]:
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
if manifest.get("runtime") != "CORE_STANDALONE" or "legacy_provider" in manifest:
    fail("standalone manifest must not require an external project server")
if profile.get("capabilities", {}).get("standaloneRequired") is not True:
    fail("GEF standalone profile missing")
for retired in ("docs/LEGACY_PROVIDER-INTEGRATION.md", "scripts/legacy_provider_mcp.py",
                "scripts/legacy_provider_mcp_probe.py", "scripts/legacy_provider_bootstrap.py",
                "scripts/legacy_provider_evidence.py", "scripts/legacy_provider-bootstrap.ps1",
                "tests/test_legacy_provider_bootstrap.py", "tests/test_legacy_provider_evidence.py",
                "tests/test_legacy_provider_mcp.py", "tests/test_legacy_provider_mcp_probe.py"):
    if (ROOT / retired).exists():
        fail(f"retired service path reintroduced: {retired}")
if "[mcp_servers.legacy_provider]" in (ROOT / ".codex/config.toml").read_text(encoding="utf-8"):
    fail("project MCP service must not be required")

for relative in CANONICAL_PROJECT_SOURCES:
    if not (ROOT / relative).is_file():
        fail(f"local canonical source missing: {relative}")

# Current-tree zero-residue gate. The forbidden token is derived without storing it contiguously.
forbidden_provider_token = bytes((104, 105, 118, 101)).decode("ascii")
tracked = subprocess.run(
    ["git", "ls-files", "-z"],
    cwd=ROOT,
    check=True,
    capture_output=True,
).stdout.split(b"\\0")
for raw_relative in tracked:
    if not raw_relative:
        continue
    relative = raw_relative.decode("utf-8")
    if forbidden_provider_token.casefold() in relative.casefold():
        fail(f"retired provider token remains in tracked path: {relative}")
    current_path = ROOT / relative
    if not current_path.is_file():
        continue
    raw = current_path.read_bytes()
    try:
        text_body = raw.decode("utf-8")
    except UnicodeDecodeError:
        continue
    if forbidden_provider_token.casefold() in text_body.casefold():
        fail(f"retired provider token remains in tracked UTF-8 content: {relative}")

if not (ROOT / ".engineering/decisions/CORE-D-206-STANDALONE-PLANNING-ENTRYPOINTS.md").is_file():
    fail("standalone planning decision CORE-D-206 missing")
if not (ROOT / ".engineering/decisions/CORE-D-207-STANDALONE-SUPPORT-SURFACES.md").is_file():
    fail("standalone auxiliary decision CORE-D-207 missing")
if not (ROOT / ".engineering/decisions/CORE-D-209-CURRENT-TREE-SANITATION.md").is_file():
    fail("current-tree sanitation decision CORE-D-209 missing")

overview = (ROOT / "docs/project-brain/01-PROJECT-OVERVIEW.md").read_text(encoding="utf-8")
backlog = (ROOT / "docs/project-brain/14-BACKLOG.md").read_text(encoding="utf-8")
for required in ("M01 Core Runtime & Lifecycle: COMPLETE", "M02 Project / Workspace Adapter: COMPLETE V2",
                 "M03 Work Order Engine: COMPLETE V2", "M04 Run / Attempt / Step Engine: BLOCKED_RE_ADMISSION"):
    if required not in overview or required not in backlog:
        fail(f"standalone implemented/blocked module status mismatch: {required}")
if "M23 Local Context & Evidence Registry: FUTURE SCOPE only" not in backlog:
    fail("future local context registry status drift")

for relative, marker in {
    "docs/project-brain/00-README-UPLOAD-ORDER.md": "CURRENT_STANDALONE_GIT_CANONICAL",
    "docs/project-brain/12-LOCAL-DEPLOYMENT.md": "STANDALONE_M01_M03_IMPLEMENTED",
    "docs/engineering/CORE-MODULAR-DELIVERY-MODEL.md": "CURRENT_STANDALONE_MODULAR_DELIVERY",
    "docs/research/CORE-TECHNOLOGY-CANDIDATES.md": "STANDALONE_RESEARCH_ONLY",
}.items():
    current = (ROOT / relative).read_text(encoding="utf-8")
    if marker not in current or "CORE-D-207" not in current:
        fail(f"standalone support surface drift: {relative}")

new_template = (ROOT / ".github/pull_request_template.md").read_text(encoding="utf-8")
if "## Standalone source preflight" not in new_template or "NOT INDEPENDENT" not in new_template:
    fail("PR template must require standalone source evidence and disclose owner review")

# Fail-closed M04 STALE source identity and single-account owner-audit boundary.
# Matching canonical Git fingerprints NEVER reauthorize superseded historical execution.
review_policy_id = "SINGLE_ACCOUNT_OWNER_SELF_AUDIT"
review_policy_path = ".engineering/gef/GEF-REVIEW-PROTOCOL.md"
context_lock_path = ".engineering/context-locks/CORE-WO-M04-001.json"
evidence_path = ".engineering/evidence/CORE-WO-M04-001.json"
gef_current_path = ".engineering/gef/GEF-CURRENT.json"

context_lock = json.loads((ROOT / context_lock_path).read_text(encoding="utf-8"))
evidence = json.loads((ROOT / evidence_path).read_text(encoding="utf-8"))
gef_current = json.loads((ROOT / gef_current_path).read_text(encoding="utf-8"))
review_protocol = (ROOT / review_policy_path).read_text(encoding="utf-8")

if context_lock.get("status") != "STALE" or context_lock.get("productImplementationAuthorized") is not False:
    fail("superseded M04 lock must be STALE and forbid product implementation")
if context_lock.get("executionStatus") != "BLOCKED_RE_ADMISSION" or context_lock.get("cutoverDecision") != "CORE-D-205":
    fail("M04 must block historical execution authority after cutover")
if context_lock.get("externalV1ConsumersUnknownBlocking") is not True:
    fail("M04 prior-V1 external consumer inventory must remain BLOCKING/UNKNOWN")
if evidence.get("status") != "BLOCKED_RE_ADMISSION" or evidence.get("contextLock", {}).get("status") != "STALE":
    fail("M04 Evidence Bundle must report BLOCKED_RE_ADMISSION and STALE lock")
if evidence.get("contextLock", {}).get("productImplementationAuthorized") is not False:
    fail("M04 Evidence Bundle still grants historical implementation authority")
if gef_current.get("productImplementationAuthorized") is not False or gef_current.get("activeWorkOrder") is not None:
    fail("GEF must not route execution to the stale historical M04 Work Order")
if gef_current.get("currentExecutionGate") != "BLOCKED_RE_ADMISSION":
    fail("GEF did not preserve the standalone re-admission gate")
if context_lock.get("reviewIdentityPolicy") != review_policy_id:
    fail("active Context Lock review identity policy mismatch")
if context_lock.get("ownerAuditIdentity") != "KayzenRoot":
    fail("active Context Lock owner-audit identity mismatch")
if gef_current.get("reviewIdentityPolicy") != review_policy_id:
    fail("GEF current review identity policy mismatch")
if gef_current.get("ownerAuditIdentity") != "KayzenRoot":
    fail("GEF current owner-audit identity mismatch")
if gef_current.get("reviewAuditIndependence") is not False:
    fail("GEF current must disclose that the owner audit is not independent")
if "NOT INDEPENDENT" not in review_protocol or "absence of another identity alone is never" not in review_protocol:
    fail("GEF review protocol does not define the transparent single-account audit")
if context_lock.get("reviewPolicy") != {
    "path": review_policy_path,
    "blobSha": git_blob_sha(review_policy_path),
}:
    fail("active Context Lock review-policy fingerprint is stale")

expected_canonical_sources = {
    "docs/project-brain/13-CHECKPOINT.md",
    "docs/project-brain/16-DECISIONS-LEDGER.md",
    "docs/project-brain/03-SCOPE.md",
    "docs/project-brain/15-DEFINITION-OF-DONE.md",
    "docs/project-brain/04-ARCHITECTURE.md",
    "docs/project-brain/02-REQUIREMENTS.md",
    "docs/project-brain/10-SECURITY-GOVERNANCE.md",
    "docs/project-brain/11-TEST-PLAN.md",
    "docs/modules/M04-RUN-ATTEMPT-STEP-ENGINE.md",
}
canonical_sources = context_lock.get("canonicalSources", {})
if set(canonical_sources) != expected_canonical_sources:
    fail("active Context Lock must bind exactly the nine canonical M04 source files")
for relative, expected_sha in canonical_sources.items():
    if expected_sha != git_blob_sha(relative):
        fail(f"active Context Lock source fingerprint is stale: {relative}")

work_order = context_lock.get("workOrderSource", {})
if work_order.get("path") != ".engineering/work-orders/CORE-WO-M04-001.md":
    fail("active Context Lock Work Order path mismatch")
if work_order.get("blobSha") != git_blob_sha(work_order["path"]):
    fail("active Context Lock Work Order fingerprint is stale")

source_check = evidence.get("sourceCheck", {})
if source_check.get("candidateCanonicalSourceBlobShas") != canonical_sources:
    fail("M04 Evidence Bundle canonical source fingerprints drift from Context Lock")
if source_check.get("workOrderBlobSha") != work_order.get("blobSha"):
    fail("M04 Evidence Bundle Work Order fingerprint drift")
if source_check.get("reviewPolicyBlobSha") != context_lock["reviewPolicy"]["blobSha"]:
    fail("M04 Evidence Bundle review-policy fingerprint drift")
if evidence.get("contextLock", {}).get("blobSha") != git_blob_sha(context_lock_path):
    fail("M04 Evidence Bundle Context Lock fingerprint is stale")
if evidence.get("reviewIdentityPolicy") != review_policy_id:
    fail("M04 Evidence Bundle review identity policy mismatch")
owner_audit = evidence.get("ownerSelfAudit", {})
if owner_audit.get("identity") != "KayzenRoot" or owner_audit.get("independent") is not False:
    fail("M04 Evidence Bundle must record the KayzenRoot audit as not independent")
if owner_audit.get("required") is not True:
    fail("M04 Evidence Bundle is missing the required owner-audit record")
if evidence.get("independentReview", {}).get("required") is not False:
    fail("M04 Evidence Bundle still requires an independent reviewer identity")
ev023 = evidence.get("evidenceNodes", {}).get("EV-M04-023", {})
if ev023.get("owner") != "SOLO_OWNER_AUDITOR":
    fail("EV-M04-023 owner must be the single-account owner-auditor stage")

print("CORE governance validation: PASS")
print("GEF: v1.0.0 @ 866fe3af8cccc65c929aaf6a47a924401fa448b3")
print("CORE runtime: STANDALONE / no project server")
print("GEF checkpoint/source bridges: CONSISTENT")
print(f"Required artifacts: {len(REQUIRED)}")
