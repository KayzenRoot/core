from __future__ import annotations

import unittest

from scripts.hive_bootstrap import resolve_registered_project


class HiveProjectResolutionTests(unittest.TestCase):
    def test_exact_relative_path_wins(self) -> None:
        projects = [
            {"project_id": "wrong", "name": "CORE", "relative_path": "archive/core"},
            {"project_id": "right", "name": "Renamed CORE", "relative_path": "core"},
        ]
        result = resolve_registered_project(projects, name="CORE", relative_path="core")
        self.assertIsNotNone(result)
        self.assertEqual(result["project_id"], "right")

    def test_same_name_different_path_fails_closed(self) -> None:
        projects = [{"project_id": "other", "name": "CORE", "relative_path": "other/core"}]
        with self.assertRaisesRegex(RuntimeError, "different path"):
            resolve_registered_project(projects, name="CORE", relative_path="core")

    def test_missing_project_returns_none(self) -> None:
        projects = [{"project_id": "other", "name": "Widget", "relative_path": "widget"}]
        self.assertIsNone(
            resolve_registered_project(projects, name="CORE", relative_path="core")
        )


if __name__ == "__main__":
    unittest.main()
