"""No-network regression for M01/M02/M03 standalone authority disposition."""

import hashlib
import json
import re
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MARKER = "\n\n## Prior accepted module record (exact prior Git blob follows)\n\n"
MARKER_BYTES = MARKER.encode("utf-8")
RETIRED_PROVIDER_IDENTIFIER = re.compile(r"\bhive(?:\b|_|external\b)", re.IGNORECASE)

MODULES = {
    "docs/modules/M01-CORE-RUNTIME-LIFECYCLE.md": {
        "prior_blob": "46851bb0dfd2f00ee36f790a64e3b6ba8971d2a9",
        "revision": "Owner-directed standalone architecture amendment (2026-09-28)",
        "extra": None,
    },
    "docs/modules/M02-PROJECT-WORKSPACE-ADAPTER.md": {
        "prior_blob": "3690e81e50f6fed1a28dea56bd22e0c83383b33d",
        "revision": "Owner-directed standalone contract revision (2026-09-28)",
        "extra": "V2",
    },
    "docs/modules/M03-WORK-ORDER-ENGINE.md": {
        "prior_blob": "8e73e5fb7f80f7b695f957bbf0d0c12fa25974ed",
        "revision": "Owner-directed standalone context contract revision (2026-09-28)",
        "extra": "Standalone V2 numeric calibration (2026-09-29)",
    },
}


def git_blob_sha(payload: bytes) -> str:
    """Return the exact Git blob SHA-1 for raw bytes."""
    header = b"blob " + str(len(payload)).encode("ascii") + b"\0"
    return hashlib.sha1(header + payload).hexdigest()


def active_and_prior(path: str) -> tuple[str, bytes]:
    """Split current authority text from the byte-exact predecessor record."""
    raw = (ROOT / path).read_bytes()
    if raw.count(MARKER_BYTES) != 1:
        raise AssertionError(f"missing/ambiguous module-record marker: {path}")
    active, prior = raw.split(MARKER_BYTES, 1)
    return active.decode("utf-8"), prior


class StandaloneModuleDocsDispositionTests(unittest.TestCase):
    """Keep current authority explicit while retaining accepted prior bytes."""

    def test_each_module_has_current_standalone_authority_overlay(self):
        """Require a provider-neutral current-authority overlay on every module."""
        for path, expected in MODULES.items():
            with self.subTest(path=path):
                active, _ = active_and_prior(path)
                self.assertTrue(active.startswith("> **CURRENT STANDALONE AMENDMENT; CORE-D-208 OVERLAY PENDING PROMOTION (2026-10-01):**"))
                self.assertIn(expected["revision"], active)
                if expected["extra"]:
                    self.assertIn(expected["extra"], active)
                self.assertIn("provider-neutral", active.lower())
                self.assertIn("non-operative for new execution", active.lower())
                self.assertIn("core-d-208 overlay is not canonical until the required promotion conditions are complete", active.lower())
                self.assertIsNone(RETIRED_PROVIDER_IDENTIFIER.search(active))

    def test_prior_module_records_are_byte_exact_git_blobs(self):
        """Prove the preserved predecessor payloads retain exact Git identities."""
        for path, expected in MODULES.items():
            with self.subTest(path=path):
                _, prior = active_and_prior(path)
                self.assertEqual(git_blob_sha(prior), expected["prior_blob"])

    def test_line_ending_mutation_breaks_provenance(self):
        """Reject newline normalization that would rewrite historical bytes."""
        for path, expected in MODULES.items():
            with self.subTest(path=path):
                _, prior = active_and_prior(path)
                self.assertIn(b"\n", prior)
                mutated = prior.replace(b"\n", b"\r\n", 1)
                self.assertNotEqual(git_blob_sha(mutated), expected["prior_blob"])

    def test_decision_records_bounded_precedence_and_exact_blobs(self):
        """Bind CORE-D-208 to the Work Order and exact predecessor identities."""
        path = ROOT / ".engineering/decisions/CORE-D-208-STANDALONE-MODULE-DOC-DISPOSITION.md"
        self.assertTrue(path.exists(), "CORE-D-208 decision is required")
        decision = path.read_text(encoding="utf-8")
        self.assertIn("Work Order: #187; parent #172.", decision)
        self.assertIn("provider-neutral", decision.lower())
        self.assertIn("retired-provider-specific", decision.lower())
        for module_path, expected in MODULES.items():
            with self.subTest(path=module_path):
                self.assertIn(module_path, decision)
                self.assertIn(expected["prior_blob"], decision)

    def test_increment_lock_distinguishes_inputs_from_authorized_outputs(self):
        """Keep frozen inputs distinct from the exact authorized output set."""
        lock_path = ROOT / ".engineering/context-locks/CORE-STANDALONE-MODULE-DOCS-007.json"
        self.assertTrue(lock_path.exists(), "increment Context Lock is required")
        lock = json.loads(lock_path.read_text(encoding="utf-8"))
        self.assertEqual(lock["sourceFingerprintRole"], "AUTHORIZED_INPUT_BASELINES")
        self.assertEqual(lock["authorizedBase"], "5c2952223395747522576fa46e11063c493ca3a3")
        self.assertFalse(lock["productImplementationAuthorized"])
        for path, expected in MODULES.items():
            with self.subTest(path=path):
                self.assertEqual(lock["canonicalSources"][path], expected["prior_blob"])
        expected_mutations = [
            "docs/modules/M01-CORE-RUNTIME-LIFECYCLE.md",
            "docs/modules/M02-PROJECT-WORKSPACE-ADAPTER.md",
            "docs/modules/M03-WORK-ORDER-ENGINE.md",
            ".engineering/decisions/CORE-D-208-STANDALONE-MODULE-DOC-DISPOSITION.md",
            ".engineering/context-locks/CORE-STANDALONE-MODULE-DOCS-007.json",
            "tests/test_standalone_module_docs_disposition.py",
        ]
        self.assertEqual(lock["authorizedMutationPaths"], expected_mutations)

    def test_m04_stale_lock_is_not_part_of_this_disposition(self):
        """Ensure this document increment cannot reactivate or rewrite M04."""
        lock_path = ROOT / ".engineering/context-locks/CORE-WO-M04-001.json"
        lock = lock_path.read_text(encoding="utf-8")
        self.assertIn('"status": "STALE"', lock)
        self.assertIn('"productImplementationAuthorized": false', lock)
        for path in MODULES:
            self.assertNotIn(f'"{path}"', lock)


if __name__ == "__main__":
    unittest.main()
