"""Deterministic offline M04 inventory tool tests. No real owner attestation."""

from __future__ import annotations

import contextlib
import copy
import io
import json
import tempfile
import unittest
from datetime import date
from pathlib import Path

from scripts.m04_legacy_inventory import (
    CATEGORIES, COVERAGE, InvalidInventory, MAX_INVENTORY_BYTES,
    assess_inventory, main, neutral_template, read_inventory,
    validate_inventory,
)

TODAY = date(2026, 9, 27)


def completed_example() -> dict[str, object]:
    candidate = neutral_template(today=TODAY)
    candidate["as_of"] = "2026-09-27"
    candidate["owner_attested"] = True
    candidate["coverage"] = {name: "CHECKED" for name in COVERAGE}
    candidate["findings"] = {name: "NO" for name in CATEGORIES}
    candidate["evidence_recorded"] = {name: True for name in CATEGORIES}
    return candidate


class M04OfflineInventoryTests(unittest.TestCase):
    def test_template_is_neutral_and_cannot_accidentally_pass(self) -> None:
        template = neutral_template(today=TODAY)
        self.assertFalse(template["owner_attested"])
        self.assertEqual(set(template["findings"].values()), {"UNKNOWN"})
        self.assertEqual(set(template["coverage"].values()), {"UNKNOWN"})
        self.assertEqual(set(template["evidence_recorded"].values()), {False})
        result = assess_inventory(template, today=TODAY)
        self.assertEqual(result["status"], "BLOCKED_INCOMPLETE")
        self.assertFalse(result["external_absence_proven"])
        self.assertFalse(result["breaking_v1_approved"])

    def test_complete_no_is_only_structural_review_candidate(self) -> None:
        result = assess_inventory(completed_example(), today=TODAY)
        self.assertEqual(result["status"], "NO_LEGACY_CANDIDATE_REQUIRES_REVIEW")
        self.assertEqual(result["checked_scope_count"], 4)
        self.assertEqual(result["recorded_evidence_count"], 3)
        self.assertFalse(result["external_absence_proven"])
        self.assertFalse(result["breaking_v1_approved"])
        self.assertFalse(result["m04_issue_111_closed"])
        self.assertTrue(result["owner_and_source_review_required"])

    def test_any_yes_requires_separately_governed_v2_even_with_unknown(self) -> None:
        for category in CATEGORIES:
            with self.subTest(category=category):
                item = completed_example()
                item["findings"][category] = "YES"
                self.assertEqual(
                    assess_inventory(item, today=TODAY)["status"], "V2_DISPOSITION_REQUIRED"
                )
                item["findings"][CATEGORIES[0]] = "UNKNOWN"
                self.assertEqual(
                    assess_inventory(item, today=TODAY)["status"], "V2_DISPOSITION_REQUIRED"
                )

    def test_unknown_prevents_claiming_no_legacy(self) -> None:
        for category in CATEGORIES:
            with self.subTest(category=category):
                item = completed_example()
                item["findings"][category] = "UNKNOWN"
                self.assertEqual(assess_inventory(item, today=TODAY)["status"], "BLOCKED_INCOMPLETE")

    def test_all_no_requires_every_scope_attestation_and_evidence(self) -> None:
        base = completed_example()
        base["owner_attested"] = False
        self.assertEqual(assess_inventory(base, today=TODAY)["status"], "BLOCKED_INCOMPLETE")
        for scope in COVERAGE:
            with self.subTest(scope=scope):
                item = completed_example()
                item["coverage"][scope] = "UNKNOWN"
                self.assertEqual(assess_inventory(item, today=TODAY)["status"], "BLOCKED_INCOMPLETE")
        for category in CATEGORIES:
            with self.subTest(category=category):
                item = completed_example()
                item["evidence_recorded"][category] = False
                self.assertEqual(assess_inventory(item, today=TODAY)["status"], "BLOCKED_INCOMPLETE")

    def test_rejects_unknown_keys_at_every_level(self) -> None:
        for parent in (None, "coverage", "findings", "evidence_recorded"):
            with self.subTest(parent=parent):
                item = completed_example()
                target = item if parent is None else item[parent]
                target["local_private_path"] = "C:/secret/account"
                with self.assertRaises(InvalidInventory):
                    validate_inventory(item, today=TODAY)

    def test_rejects_missing_keys_at_every_level(self) -> None:
        for parent, field in (
            (None, "schema"),
            ("coverage", COVERAGE[0]),
            ("findings", CATEGORIES[0]),
            ("evidence_recorded", CATEGORIES[0]),
        ):
            item = completed_example()
            target = item if parent is None else item[parent]
            del target[field]
            with self.subTest(parent=parent):
                with self.assertRaises(InvalidInventory):
                    validate_inventory(item, today=TODAY)

    def test_rejects_type_coercion_and_unsupported_enum(self) -> None:
        mutations = (
            ("owner_attested", None, 1),
            ("owner_attested", None, "true"),
            ("coverage", COVERAGE[0], True),
            ("coverage", COVERAGE[0], "MAYBE"),
            ("findings", CATEGORIES[0], False),
            ("findings", CATEGORIES[0], "no"),
            ("evidence_recorded", CATEGORIES[0], 1),
            ("evidence_recorded", CATEGORIES[0], "yes"),
            ("schema", None, "OTHER"),
        )
        for parent, key, bad in mutations:
            item = completed_example()
            if key is None:
                item[parent] = bad
            else:
                item[parent][key] = bad
            with self.subTest(parent=parent, key=key, bad=bad):
                with self.assertRaises(InvalidInventory):
                    validate_inventory(item, today=TODAY)

    def test_rejects_malformed_or_future_dates(self) -> None:
        for value in ("20260927", "2026-13-01", "2026-02-29", "2999-01-01", True):
            item = completed_example()
            item["as_of"] = value
            with self.subTest(value=value):
                with self.assertRaises(InvalidInventory):
                    validate_inventory(item, today=TODAY)

    def test_rejects_duplicate_json_keys_and_oversize_input(self) -> None:
        with tempfile.TemporaryDirectory() as root:
            path = Path(root) / "owner.json"
            path.write_text('{"schema":"first","schema":"second"}', encoding="utf-8")
            with self.assertRaises(InvalidInventory):
                read_inventory(path)
            path.write_bytes(b"x" * (MAX_INVENTORY_BYTES + 1))
            with self.assertRaises(InvalidInventory):
                read_inventory(path)

    def test_cli_output_does_not_leak_unknown_input_or_path(self) -> None:
        canary = "C:/private/owner/secret-credential"
        with tempfile.TemporaryDirectory() as root:
            path = Path(root) / "sensitive-file-name.json"
            item = completed_example()
            item["private_secret"] = canary
            path.write_text(json.dumps(item), encoding="utf-8")
            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                code = main(["--inventory", str(path)])
            self.assertEqual(code, 4)
            output = stdout.getvalue()
            self.assertEqual(json.loads(output)["status"], "INVALID")
            self.assertNotIn(canary, output)
            self.assertNotIn(root, output)
            self.assertNotIn(str(path), output)

    def test_cli_exit_codes_for_unknown_yes_and_all_no(self) -> None:
        for answer, expected_status, expected_exit in (
            ("UNKNOWN", "BLOCKED_INCOMPLETE", 2),
            ("YES", "V2_DISPOSITION_REQUIRED", 3),
            ("NO", "NO_LEGACY_CANDIDATE_REQUIRES_REVIEW", 0),
        ):
            with self.subTest(answer=answer), tempfile.TemporaryDirectory() as root:
                item = completed_example()
                item["findings"][CATEGORIES[0]] = answer
                path = Path(root) / "input.json"
                path.write_text(json.dumps(item), encoding="utf-8")
                stdout = io.StringIO()
                with contextlib.redirect_stdout(stdout):
                    result_code = main(["--inventory", str(path)])
                output = json.loads(stdout.getvalue())
                self.assertEqual(result_code, expected_exit)
                self.assertEqual(output["status"], expected_status)
                self.assertFalse(output["breaking_v1_approved"])


if __name__ == "__main__":
    unittest.main()
