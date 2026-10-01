"""Standalone planning-entrypoint regressions after current-tree sanitation."""

import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


class StandalonePlanningEntryPointsTests(unittest.TestCase):
    def test_overview_and_backlog_match_current_module_state(self):
        overview = (ROOT / "docs/project-brain/01-PROJECT-OVERVIEW.md").read_text(encoding="utf-8")
        backlog = (ROOT / "docs/project-brain/14-BACKLOG.md").read_text(encoding="utf-8")
        for required in (
            "M01 Core Runtime & Lifecycle: COMPLETE",
            "M02 Project / Workspace Adapter: COMPLETE V2",
            "M03 Work Order Engine: COMPLETE V2",
            "M04 Run / Attempt / Step Engine: BLOCKED_RE_ADMISSION",
        ):
            with self.subTest(required=required):
                self.assertIn(required, overview)
                self.assertIn(required, backlog)
        self.assertIn("M23 Local Context & Evidence Registry: FUTURE SCOPE only", backlog)

    def test_discovery_only_modules_remain_unadmitted(self):
        for relative in (
            "docs/modules/M05-HOST-ADAPTER-FABRIC.md",
            "docs/modules/M06-CAPABILITY-NEGOTIATION.md",
        ):
            with self.subTest(path=relative):
                text = (ROOT / relative).read_text(encoding="utf-8")
                self.assertIn("STANDALONE_DISCOVERY_ONLY", text)
                self.assertIn("issue #111", text)
                self.assertIn("non-operative", text.lower())
                self.assertIn("Sanitized historical archive", text)


if __name__ == "__main__":
    unittest.main()
