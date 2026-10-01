"""Repository-wide zero-residue guard for the retired provider."""

import re
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FORBIDDEN = bytes((104, 105, 118, 101)).decode("ascii")
FORBIDDEN_IDENTIFIER = re.compile(r"\\b" + re.escape(FORBIDDEN) + r"(?:\\b|_|external\\b)", re.IGNORECASE)


def tracked_paths() -> list[str]:
    result = subprocess.run(
        ["git", "ls-files", "-z"],
        cwd=ROOT,
        check=True,
        capture_output=True,
    )
    return sorted(
        item.decode("utf-8", errors="strict")
        for item in result.stdout.split(b"\0")
        if item
    )


class RetiredProviderZeroResidueTests(unittest.TestCase):
    def test_no_tracked_path_contains_forbidden_provider_token(self):
        offenders = [path for path in tracked_paths() if FORBIDDEN_IDENTIFIER.search(path)]
        self.assertEqual(offenders, [], f"forbidden provider token remains in tracked paths: {offenders}")

    def test_no_utf8_tracked_file_contains_forbidden_provider_token(self):
        offenders: list[str] = []
        for relative in tracked_paths():
            path = ROOT / relative
            if not path.is_file():
                continue
            raw = path.read_bytes()
            try:
                text = raw.decode("utf-8")
            except UnicodeDecodeError:
                continue
            if FORBIDDEN_IDENTIFIER.search(text):
                offenders.append(relative)
        self.assertEqual(
            offenders,
            [],
            f"forbidden provider token remains in tracked UTF-8 content: {offenders}",
        )


if __name__ == "__main__":
    unittest.main()
