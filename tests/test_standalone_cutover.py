"""Fail-closed regression: no required project server."""
import json
import tomllib
import unittest
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
RETIRED = ("docs/HIVE-INTEGRATION.md", "scripts/hive_mcp.py", "scripts/hive_mcp_probe.py",
           "scripts/hive_bootstrap.py", "scripts/hive_evidence.py", "scripts/hive-bootstrap.ps1",
           "tests/test_hive_bootstrap.py", "tests/test_hive_evidence.py",
           "tests/test_hive_mcp.py", "tests/test_hive_mcp_probe.py")
class StandaloneTests(unittest.TestCase):
    def test_retired_paths_absent(self):
        for path in RETIRED:
            with self.subTest(path=path):
                self.assertFalse((ROOT / path).exists())
    def test_no_project_mcp(self):
        self.assertFalse(tomllib.loads((ROOT / ".codex/config.toml").read_text(encoding="utf-8")).get("mcp_servers"))
    def test_manifest_standalone(self):
        manifest = json.loads((ROOT / ".engineering/BOOTSTRAP-MANIFEST.json").read_text(encoding="utf-8"))
        self.assertEqual(manifest["runtime"], "CORE_STANDALONE")
        self.assertNotIn("hive", manifest)
if __name__ == "__main__":
    unittest.main()
