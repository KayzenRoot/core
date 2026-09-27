"""Offline, fail-closed structural preflight for an owner-supplied M04 V1 inventory.

A passing preflight is NEVER evidence that an external V1 consumer is absent.
No filesystem scanning, network, repository mutation, HIVE use or raw input output.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from datetime import date
from pathlib import Path
from typing import Any

SCHEMA = "CORE_M04_V1_CONSUMER_INVENTORY_V1"
RESULT_SCHEMA = "CORE_M04_V1_INVENTORY_PREFLIGHT_V1"
MAX_INVENTORY_BYTES = 32_768

CATEGORIES = (
    "exported_v1_build_or_api",
    "retained_v1_journal_or_snapshot",
    "downstream_consumer_or_deployment",
)
COVERAGE = (
    "repositories_forks_and_branches",
    "local_builds_and_export_locations",
    "durable_state_and_backups",
    "downstream_clients_and_deployments",
)
ANSWERS = frozenset({"YES", "NO", "UNKNOWN"})
COVERAGE_VALUES = frozenset({"CHECKED", "UNKNOWN"})
ISO_DATE = re.compile(r"[0-9]{4}-[0-9]{2}-[0-9]{2}\Z")


class InvalidInventory(ValueError):
    """Typed error with no user-supplied data or local path in its message."""


class RedactedInventoryParser(argparse.ArgumentParser):
    """Avoid argparse echoing a malformed secret or local owner filename."""

    def error(self, message: str) -> None:
        raise InvalidInventory("invalid_cli_arguments")


def neutral_template(*, today: date | None = None) -> dict[str, Any]:
    """Unknown-by-default example, deliberately not an owner attestation."""
    return {
        "schema": SCHEMA,
        "as_of": (today or date.today()).isoformat(),
        "owner_attested": False,
        "coverage": {key: "UNKNOWN" for key in COVERAGE},
        "findings": {key: "UNKNOWN" for key in CATEGORIES},
        "evidence_recorded": {key: False for key in CATEGORIES},
    }


def _keys(value: object, expected: tuple[str, ...]) -> dict[str, Any]:
    if type(value) is not dict or set(value) != set(expected):
        raise InvalidInventory("invalid_inventory_schema")
    return value


def validate_inventory(value: object, *, today: date | None = None) -> dict[str, Any]:
    """Strictly validate shape only; never assess truth of owner statements."""
    data = _keys(value, ("schema", "as_of", "owner_attested", "coverage", "findings", "evidence_recorded"))
    if data["schema"] != SCHEMA:
        raise InvalidInventory("invalid_inventory_schema")
    stamp = data["as_of"]
    if type(stamp) is not str or ISO_DATE.fullmatch(stamp) is None:
        raise InvalidInventory("invalid_inventory_date")
    try:
        audit_date = date.fromisoformat(stamp)
    except ValueError as exc:
        raise InvalidInventory("invalid_inventory_date") from exc
    if audit_date > (today or date.today()):
        raise InvalidInventory("future_inventory_date")
    if type(data["owner_attested"]) is not bool:
        raise InvalidInventory("invalid_inventory_attestation")

    coverage = _keys(data["coverage"], COVERAGE)
    findings = _keys(data["findings"], CATEGORIES)
    evidence = _keys(data["evidence_recorded"], CATEGORIES)
    if any(type(v) is not str or v not in COVERAGE_VALUES for v in coverage.values()):
        raise InvalidInventory("invalid_inventory_coverage")
    if any(type(v) is not str or v not in ANSWERS for v in findings.values()):
        raise InvalidInventory("invalid_inventory_findings")
    if any(type(v) is not bool for v in evidence.values()):
        raise InvalidInventory("invalid_inventory_evidence")
    return data


def assess_inventory(value: object, *, today: date | None = None) -> dict[str, object]:
    """Classify explicit owner inputs, never infer a negative from a search."""
    data = validate_inventory(value, today=today)
    findings = data["findings"]
    coverage = data["coverage"]
    evidence = data["evidence_recorded"]
    yes = sum(answer == "YES" for answer in findings.values())
    unknown = sum(answer == "UNKNOWN" for answer in findings.values())
    checked = sum(value == "CHECKED" for value in coverage.values())
    recorded = sum(value is True for value in evidence.values())

    if yes:
        status = "V2_DISPOSITION_REQUIRED"
    elif (
        unknown == 0
        and data["owner_attested"] is True
        and checked == len(COVERAGE)
        and recorded == len(CATEGORIES)
    ):
        status = "NO_LEGACY_CANDIDATE_REQUIRES_REVIEW"
    else:
        status = "BLOCKED_INCOMPLETE"

    # These safeguards are fixed, even if the manifest claims all-NO.
    return {
        "schema": RESULT_SCHEMA,
        "status": status,
        "yes_count": yes,
        "unknown_count": unknown,
        "checked_scope_count": checked,
        "recorded_evidence_count": recorded,
        "owner_attested": data["owner_attested"],
        "external_absence_proven": False,
        "breaking_v1_approved": False,
        "owner_and_source_review_required": True,
        "m04_issue_111_closed": False,
    }


def _unique_pairs(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise InvalidInventory("duplicate_inventory_field")
        result[key] = value
    return result


def read_inventory(path: Path) -> object:
    """Bound input memory; duplicate JSON keys fail rather than last-key wins."""
    with path.open("rb") as source:
        data = source.read(MAX_INVENTORY_BYTES + 1)
    if len(data) > MAX_INVENTORY_BYTES:
        raise InvalidInventory("inventory_too_large")
    return json.loads(data.decode("utf-8"), object_pairs_hook=_unique_pairs)


def main(argv: list[str] | None = None) -> int:
    parser = RedactedInventoryParser(
        description="Offline, redacted structural preflight; never approves an M04 V1 break"
    )
    choice = parser.add_mutually_exclusive_group(required=True)
    choice.add_argument("--template", action="store_true", help="Print UNKNOWN-only example")
    choice.add_argument("--inventory", type=Path, help="Local owner-prepared JSON to validate")
    try:
        args = parser.parse_args(argv)
    except InvalidInventory:
        print(json.dumps({"status": "INVALID", "reason": "invalid_cli_arguments"}))
        return 4
    if args.template:
        print(json.dumps(neutral_template(), indent=2, sort_keys=True))
        return 0

    try:
        result = assess_inventory(read_inventory(args.inventory))
    except (InvalidInventory, OSError, UnicodeError, ValueError, TypeError, RecursionError):
        # Do not expose the filename, owner evidence, raw JSON or OS error.
        print(json.dumps({"status": "INVALID", "reason": "invalid_or_unavailable_inventory"}))
        return 4

    print(json.dumps(result, indent=2, sort_keys=True))
    if result["status"] == "NO_LEGACY_CANDIDATE_REQUIRES_REVIEW":
        return 0  # Structural completeness, not contract acceptance.
    if result["status"] == "V2_DISPOSITION_REQUIRED":
        return 3
    return 2


if __name__ == "__main__":
    sys.exit(main())
