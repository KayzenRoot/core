"""Raw Git archive integrity and no-network standalone auxiliary source gates."""

import ast
import hashlib
import re
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MARKER = b"\n\n## Historical discovery archive (non-operative; exact prior Git blob follows)\n\n"
PRIOR = {
    "docs/project-brain/00-README-UPLOAD-ORDER.md": "b25433e68e5d209c3767f1e727e0bffe9c0a6fc9",
    "docs/project-brain/12-LOCAL-DEPLOYMENT.md": "3988ac80814fba9b48569cfdb59f46218094ebf3",
    "docs/engineering/CORE-MODULAR-DELIVERY-MODEL.md": "0265293a529f9850cc63c72e8aedbe617951cafd",
    "docs/research/CORE-TECHNOLOGY-CANDIDATES.md": "7e6e33ecf635baf74990a361aa01b3072e13b34d",
}


def retired_provider_pattern() -> str:
    """Read the one validator-owned retired-provider identifier pattern without importing its executable checks."""
    tree = ast.parse((ROOT / "scripts/validate_governance.py").read_text(encoding="utf-8"))
    for node in tree.body:
        if not isinstance(node, ast.Assign):
            continue
        if not any(isinstance(target, ast.Name) and target.id == "RETIRED_PROVIDER_IDENTIFIER_PATTERN" for target in node.targets):
            continue
        value = ast.literal_eval(node.value)
        if not isinstance(value, str):
            raise AssertionError("retired-provider identifier pattern is not a string")
        return value
    raise AssertionError("retired-provider identifier pattern constant missing")


def archive_parts(path: str) -> tuple[str, bytes]:
    """Split unmodified source bytes before decoding only current operative policy."""
    source = (ROOT / path).read_bytes()
    if source.count(MARKER) != 1:
        raise AssertionError(f"archive marker absent or ambiguous: {path}")
    active, historic = source.split(MARKER, 1)
    return active.decode("utf-8"), historic


def git_blob_sha(source: bytes) -> str:
    """Recreate the exact raw Git blob fingerprint, preserving CRLF differences."""
    return hashlib.sha1(b"blob " + str(len(source)).encode("ascii") + b"\0" + source).hexdigest()


class StandaloneSupportDocsTests(unittest.TestCase):
    """Fail closed if retired service requirements or archived content drift."""

    def test_prior_git_blobs_are_byte_exact(self):
        """Require all four dated originals to retain their exact pre-cutover bytes."""
        for path, sha in PRIOR.items():
            with self.subTest(path=path):
                active, historic = archive_parts(path)
                self.assertTrue(active.startswith("# "))
                self.assertEqual(git_blob_sha(historic), sha)

    def test_crlf_changes_cannot_pass_archive_identity(self):
        """Reject a raw newline mutation that text decoding might normalize."""
        for path, sha in PRIOR.items():
            with self.subTest(path=path):
                _, historic = archive_parts(path)
                self.assertIn(b"\n", historic)
                self.assertNotEqual(git_blob_sha(historic.replace(b"\n", b"\r\n", 1)), sha)

    def test_current_docs_have_no_mandatory_retired_provider(self):
        """Validate all live entrypoint text before the historical divider."""
        for path in PRIOR:
            with self.subTest(path=path):
                active, _ = archive_parts(path)
                self.assertIsNone(re.search(retired_provider_pattern(), active, flags=re.IGNORECASE))
                self.assertIn("CORE-D-207", active)

    def test_prefixed_retired_provider_name_is_rejected(self):
        """Reject HIVE_PROJECTS_ROOT rather than checking only the isolated vendor word."""
        pattern = retired_provider_pattern()
        self.assertIsNotNone(re.search(pattern, "HIVE_PROJECTS_ROOT required", flags=re.IGNORECASE))
        self.assertIsNotNone(re.search(pattern, "HiveExternal must start", flags=re.IGNORECASE))
        self.assertIsNone(re.search(pattern, "archive contains old evidence", flags=re.IGNORECASE))
        self.assertIsNone(re.search(pattern, "hived data remains historical", flags=re.IGNORECASE))
        self.assertIsNone(re.search(pattern, "hiver process", flags=re.IGNORECASE))
        for path in PRIOR:
            with self.subTest(path=path):
                effective, _ = archive_parts(path)
                self.assertIsNone(re.search(pattern, effective, flags=re.IGNORECASE))
        current_template = (ROOT / ".github/pull_request_template.md").read_text(encoding="utf-8")
        self.assertIsNone(re.search(pattern, current_template, flags=re.IGNORECASE))

    def test_source_pack_and_local_deployment(self):
        """Assert canonical local hierarchy and honestly incomplete distribution."""
        source, _ = archive_parts("docs/project-brain/00-README-UPLOAD-ORDER.md")
        for name in ("13-CHECKPOINT.md", "16-DECISIONS-LEDGER.md", "03-SCOPE.md",
                     "15-DEFINITION-OF-DONE.md", "04-ARCHITECTURE.md", "02-REQUIREMENTS.md"):
            self.assertIn(name, source)
        self.assertIn("CURRENT_STANDALONE_GIT_CANONICAL", source)
        deployment, _ = archive_parts("docs/project-brain/12-LOCAL-DEPLOYMENT.md")
        self.assertIn("STANDALONE_M01_M03_IMPLEMENTED", deployment)
        self.assertIn("PRODUCT_DISTRIBUTION_NOT_ADMITTED", deployment)
        self.assertIn("#111", deployment)

    def test_modular_harness_and_retired_federation(self):
        """Block mandatory third-party preflight and unearned product claims."""
        delivery, _ = archive_parts("docs/engineering/CORE-MODULAR-DELIVERY-MODEL.md")
        self.assertIn("CURRENT_STANDALONE_MODULAR_DELIVERY", delivery)
        self.assertIn("harness", delivery.lower())
        self.assertIn("NOT INDEPENDENT", delivery)
        research, _ = archive_parts("docs/research/CORE-TECHNOLOGY-CANDIDATES.md")
        self.assertIn("STANDALONE_RESEARCH_ONLY", research)
        self.assertIn("NOT_PLANNED", research)
        self.assertIn("M23 is a FUTURE Local Context & Evidence Registry", research)

    def test_pr_template_is_standalone(self):
        """Deny a required external project service in current PR admission."""
        template = (ROOT / ".github/pull_request_template.md").read_text(encoding="utf-8")
        self.assertIn("## Standalone source preflight", template)
        self.assertIn("Separate ACTUAL new-main", template)
        self.assertIn("NOT INDEPENDENT", template)
        self.assertIsNone(re.search(retired_provider_pattern(), template, flags=re.IGNORECASE))

    def test_decision_records_exact_prior_git_shas(self):
        """Ensure dated decision carries every original provenance identity."""
        decision = (ROOT / ".engineering/decisions/CORE-D-207-STANDALONE-SUPPORT-SURFACES.md").read_text(encoding="utf-8")
        for path, sha in PRIOR.items():
            self.assertIn(path, decision)
            self.assertIn(sha, decision)
        self.assertIn("ff569cdd20be8bab3579e595905930d8277fe650", decision)


    def test_decision_records_original_and_reconciled_bases(self):
        """Keep original admission provenance distinct from the current reconciled protected base."""
        decision = (ROOT / ".engineering/decisions/CORE-D-207-STANDALONE-SUPPORT-SURFACES.md").read_text(encoding="utf-8")
        self.assertIn(
            "Original authorized protected-main base: `f27e3a3f1be64d2dfedc762dea30dd1dce7cab17`.",
            decision,
        )
        self.assertIn(
            "Reconciled protected-main base after GEF v1.1.1 adoption: `7593644c04725ad6a75fcda5cf2e59fad04c1a81`.",
            decision,
        )
        self.assertIn("PR #186", decision)


if __name__ == "__main__":
    unittest.main()
