"""Standalone support-surface regressions after current-tree sanitation."""

import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

SUPPORT = {
    "docs/project-brain/00-README-UPLOAD-ORDER.md": "CURRENT_STANDALONE_GIT_CANONICAL",
    "docs/project-brain/12-LOCAL-DEPLOYMENT.md": "STANDALONE_M01_M03_IMPLEMENTED",
    "docs/engineering/CORE-MODULAR-DELIVERY-MODEL.md": "CURRENT_STANDALONE_MODULAR_DELIVERY",
    "docs/research/CORE-TECHNOLOGY-CANDIDATES.md": "STANDALONE_RESEARCH_ONLY",
}


class StandaloneSupportDocsTests(unittest.TestCase):
    def test_support_surfaces_keep_current_standalone_authority(self):
        for relative, required in SUPPORT.items():
            with self.subTest(path=relative):
                text = (ROOT / relative).read_text(encoding="utf-8")
                self.assertIn(required, text)
                self.assertIn("CORE-D-207", text)
                self.assertIn("Sanitized historical archive", text)

    def test_original_provenance_is_delegated_to_git_history(self):
        decision = (ROOT / ".engineering/decisions/CORE-D-209-CURRENT-TREE-SANITATION.md").read_text(encoding="utf-8")
        self.assertIn("Git history", decision)
        self.assertIn("zero-residue", decision)

    def test_pr_template_remains_standalone(self):
        template = (ROOT / ".github/pull_request_template.md").read_text(encoding="utf-8")
        self.assertIn("## Standalone source preflight", template)
        self.assertIn("NOT INDEPENDENT", template)


if __name__ == "__main__":
    unittest.main()
