"""Pure, no-network regression for standalone planning entrypoints and archived provenance."""

import hashlib
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
ARCHIVE = "\n\n## Historical discovery archive (non-operative; exact prior Git blob follows)\n\n"
ARCHIVE_BYTES = ARCHIVE.encode("utf-8")
PRIOR = {
    "docs/project-brain/01-PROJECT-OVERVIEW.md": "e9142649593ac93588fc99c40bb33f9fd928857f",
    "docs/project-brain/14-BACKLOG.md": "3c9a5f0762dec085dd3fca9b053b524650bb66d4",
    "docs/modules/M05-HOST-ADAPTER-FABRIC.md": "222dace08091637e0192eb0259ba13109e418ba7",
    "docs/modules/M06-CAPABILITY-NEGOTIATION.md": "fc1caaaba959e4f5982d4707885269527f3a3af4",
}


def git_blob_from_bytes(contents: bytes) -> str:
    """Hash exact raw archive bytes using the Git blob header, without newline conversion."""
    return hashlib.sha1(b"blob " + str(len(contents)).encode("ascii") + b"\0" + contents).hexdigest()


def effective_and_archive(path: str) -> tuple[str, bytes]:
    """Split raw source bytes first; decode active policy only after locating the archive."""
    content = (ROOT / path).read_bytes()
    if content.count(ARCHIVE_BYTES) != 1:
        raise AssertionError(f"missing/ambiguous archive divider: {path}")
    active, archive = content.split(ARCHIVE_BYTES, 1)
    return active.decode("utf-8"), archive


class StandalonePlanningEntryPointsTests(unittest.TestCase):
    """Reject resurrected active provider dependencies and silent historical rewrites."""

    def test_verbatim_archived_old_blobs(self):
        """Prove every archived original matches the exact accepted pre-cleanout Git blob."""
        for path, sha in PRIOR.items():
            with self.subTest(path=path):
                active, archived = effective_and_archive(path)
                self.assertTrue(active.startswith("# "))
                self.assertEqual(git_blob_from_bytes(archived), sha)

    def test_archive_line_ending_mutations_change_git_identity(self):
        """Catch raw LF-to-CRLF drift that text decoding would silently normalize."""
        for path, old_sha in PRIOR.items():
            with self.subTest(path=path):
                _, archive = effective_and_archive(path)
                self.assertIn(b"\n", archive)
                mutated = archive.replace(b"\n", b"\r\n", 1)
                self.assertNotEqual(git_blob_from_bytes(mutated), old_sha)

    def test_effective_text_has_no_retired_service_authority(self):
        """Prevent former runtime/version/server assumptions leaking before the archive."""
        for path in PRIOR:
            with self.subTest(path=path):
                active, _ = effective_and_archive(path)
                self.assertNotRegex(active, r"(?i)\bhive\b")
                self.assertNotIn("BOOTSTRAP_BASELINE", active)
                self.assertNotIn("ACTIVE M04 Context Lock", active)
                self.assertNotIn("M23 HIVE Sync", active)

    def test_overview_matches_implemented_and_blocked_status(self):
        """Require the accurate standalone M01–M03 implementation and blocked M04 gates."""
        active, _ = effective_and_archive("docs/project-brain/01-PROJECT-OVERVIEW.md")
        for required in (
            "ACTIVE_STANDALONE_M01_M03_PROMOTED",
            "M01 Core Runtime & Lifecycle: COMPLETE",
            "M02 Project / Workspace Adapter: COMPLETE V2",
            "M03 Work Order Engine: COMPLETE V2",
            "M04 Run / Attempt / Step Engine: BLOCKED_RE_ADMISSION",
            "UNKNOWN/BLOCKING under issue #111",
            "M05 Host Adapter Fabric and M06 Capability Negotiation: planning/discovery only",
            "NOT_PLANNED",
            "no claim is made about the owner's machine",
        ):
            with self.subTest(required=required):
                self.assertIn(required, active)

    def test_backlog_exact_module_and_m23_status(self):
        """Count three implemented modules and forbid premature registry/federation claims."""
        active, _ = effective_and_archive("docs/project-brain/14-BACKLOG.md")
        for required in ("M01 Core Runtime & Lifecycle: COMPLETE", "M02 Project / Workspace Adapter: COMPLETE V2",
                         "M03 Work Order Engine: COMPLETE V2", "M04 Run / Attempt / Step Engine: BLOCKED_RE_ADMISSION",
                         "M23 Local Context & Evidence Registry: FUTURE SCOPE only", "#111", "NOT_PLANNED"):
            with self.subTest(required=required):
                self.assertIn(required, active)

    def test_m05_m06_fail_closed_standalone_entrypoints(self):
        """Keep historical research from being confused with admitted product contracts."""
        for path, name in (
            ("docs/modules/M05-HOST-ADAPTER-FABRIC.md", "M05"),
            ("docs/modules/M06-CAPABILITY-NEGOTIATION.md", "M06"),
        ):
            with self.subTest(module=name):
                active, archived = effective_and_archive(path)
                self.assertIn("STANDALONE_DISCOVERY_ONLY", active)
                self.assertIn("issue #111", active)
                self.assertIn("M23 is a future local context/evidence registry", active)
                self.assertIn("non-operative", active.lower())
                self.assertIn(b"## Round 4", archived)
                self.assertNotIn("product code is approved", active.lower())


if __name__ == "__main__":
    unittest.main()
