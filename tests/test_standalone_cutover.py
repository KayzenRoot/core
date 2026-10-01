"""Fail-closed standalone CORE regression."""

import json
import tomllib
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


class StandaloneTests(unittest.TestCase):
    def test_no_project_mcp(self):
        config = tomllib.loads((ROOT / ".codex/config.toml").read_text(encoding="utf-8"))
        self.assertFalse(config.get("mcp_servers"))

    def test_manifest_standalone(self):
        manifest = json.loads((ROOT / ".engineering/BOOTSTRAP-MANIFEST.json").read_text(encoding="utf-8"))
        self.assertEqual(manifest["runtime"], "CORE_STANDALONE")

    def test_current_tree_zero_residue_guard_is_installed(self):
        self.assertTrue((ROOT / "tests/test_no_retired_provider_residue.py").is_file())
        self.assertTrue((ROOT / ".engineering/decisions/CORE-D-209-CURRENT-TREE-SANITATION.md").is_file())


if __name__ == "__main__":
    unittest.main()
