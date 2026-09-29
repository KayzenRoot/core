"""Standalone regression: CORE repository config must not require a project MCP service."""
import json
import tomllib
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

class StandaloneRepositoryTests(unittest.TestCase):
    def test_no_required_project_mcp(self):
        config = tomllib.loads((ROOT / ".codex/config.toml").read_text(encoding="utf-8"))
        self.assertFalse(config.get("mcp_servers", {}))

    def test_bootstrap_manifest_is_standalone(self):
        manifest = json.loads((ROOT / ".engineering/BOOTSTRAP-MANIFEST.json").read_text(encoding="utf-8"))
        self.assertEqual(manifest["runtime"], "CORE_STANDALONE")

    def test_ci_compiles_only_standalone_governance_tools(self):
        workflow = (ROOT / ".github/workflows/governance.yml").read_text(encoding="utf-8")
        self.assertIn("scripts/validate_governance.py", workflow)
        self.assertIn("scripts/ci_impact.py", workflow)
        self.assertNotIn("[mcp_servers.", (ROOT / ".codex/config.toml").read_text(encoding="utf-8"))

if __name__ == "__main__":
    unittest.main()
