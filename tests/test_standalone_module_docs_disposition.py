"""M01/M02/M03 authority regressions after current-tree sanitation."""

import json
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MODULES = (
    "docs/modules/M01-CORE-RUNTIME-LIFECYCLE.md",
    "docs/modules/M02-PROJECT-WORKSPACE-ADAPTER.md",
    "docs/modules/M03-WORK-ORDER-ENGINE.md",
)


class StandaloneModuleDocsDispositionTests(unittest.TestCase):
    def test_modules_expose_promoted_standalone_authority(self):
        for relative in MODULES:
            with self.subTest(path=relative):
                text = (ROOT / relative).read_text(encoding="utf-8")
                self.assertTrue(text.startswith("> **CURRENT STANDALONE AUTHORITY"))
                self.assertIn("CORE-D-208", text)
                self.assertIn("Sanitized prior module record", text)
                self.assertIn("Git history", text)

    def test_disposition_is_superseded_only_for_in_tree_archive_retention(self):
        d208 = (ROOT / ".engineering/decisions/CORE-D-208-STANDALONE-MODULE-DOC-DISPOSITION.md").read_text(encoding="utf-8")
        d209 = (ROOT / ".engineering/decisions/CORE-D-209-CURRENT-TREE-SANITATION.md").read_text(encoding="utf-8")
        self.assertIn("CORE-D-209", d208)
        self.assertIn("Git history", d209)

    def test_m04_remains_blocked(self):
        lock = json.loads((ROOT / ".engineering/context-locks/CORE-WO-M04-001.json").read_text(encoding="utf-8"))
        self.assertEqual(lock["status"], "STALE")
        self.assertIs(lock["productImplementationAuthorized"], False)
        self.assertEqual(lock["executionStatus"], "BLOCKED_RE_ADMISSION")


if __name__ == "__main__":
    unittest.main()
