"""Run and govern the M01 PRB/WNF benchmark without invalid comparisons."""

from __future__ import annotations

import argparse
import json
import os
import statistics
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
POLICY_PATH = ROOT / "docs" / "evidence" / "prb" / "M01-PRB-POLICY.json"
BASELINE_PATH = ROOT / "docs" / "evidence" / "prb" / "M01-PRB-BASELINE.json"


def run_benchmark() -> dict:
    result = subprocess.run(
        ["cargo", "bench", "-p", "core-runtime", "--bench", "m01_baseline", "--locked"],
        cwd=ROOT,
        check=False,
        capture_output=True,
        text=True,
        timeout=300,
    )
    if result.returncode != 0:
        raise RuntimeError(
            f"benchmark failed with exit code {result.returncode}:\n{result.stderr[-4000:]}"
        )
    combined = result.stdout + "\n" + result.stderr
    decoder = json.JSONDecoder()
    for offset in range(len(combined) - 1, -1, -1):
        if combined[offset] != "{":
            continue
        try:
            payload, _ = decoder.raw_decode(combined[offset:])
        except json.JSONDecodeError:
            continue
        if payload.get("suite") == "BOOT":
            return payload
    raise RuntimeError("benchmark did not emit a machine-readable BOOT result")


def commit_sha() -> str:
    if os.environ.get("GITHUB_SHA"):
        return os.environ["GITHUB_SHA"]
    result = subprocess.run(
        ["git", "rev-parse", "HEAD"], cwd=ROOT, check=True, capture_output=True, text=True
    )
    return result.stdout.strip()


def compatible(current: dict, baseline: dict) -> bool:
    return all(
        current.get(field) == baseline.get(field)
        for field in ("work_normalization_fingerprint", "hardware_class_fingerprint", "toolchain")
    )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--mode", choices=("record", "check"), default="check")
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()

    policy = json.loads(POLICY_PATH.read_text(encoding="utf-8"))
    runs = [run_benchmark()]
    if args.mode == "record":
        runs.extend(run_benchmark() for _ in range(4))
    current = runs[0]
    if len(runs) > 1:
        current = dict(current)
        for metric in ("p50_ms", "p95_ms", "p99_ms"):
            current[metric] = statistics.median(float(run[metric]) for run in runs)
        current["baseline_runs"] = len(runs)
    baseline = json.loads(BASELINE_PATH.read_text(encoding="utf-8")) if BASELINE_PATH.exists() else None

    if args.mode == "record" or baseline is None:
        baseline_payload = {
            "policy_version": policy["policy_version"],
            "recorded_commit": commit_sha(),
            "result": current,
        }
        if args.mode == "record":
            BASELINE_PATH.parent.mkdir(parents=True, exist_ok=True)
            BASELINE_PATH.write_text(json.dumps(baseline_payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")
        report = {
            "status": "BASELINE_RECORDED" if args.mode == "record" else "BASELINE_REQUIRED",
            "decision": "first-valid-compatible-baseline",
            "policy_version": policy["policy_version"],
            "result": current,
        }
        exit_code = 0
    else:
        baseline_result = baseline["result"]
        if not compatible(current, baseline_result):
            report = {
                "status": "NO_COMPARABLE_BASELINE",
                "decision": "pass-with-explicit-incompatibility",
                "reason": "WNF, hardware class, or toolchain differs; no regression inference is permitted",
                "policy_version": policy["policy_version"],
                "result": current,
                "baseline": baseline_result,
            }
            exit_code = 0
        else:
            budget = float(policy["p50_max_regression_percent"])
            baseline_p50 = float(baseline_result["p50_ms"])
            current_p50 = float(current["p50_ms"])
            regression_percent = ((current_p50 - baseline_p50) / baseline_p50) * 100.0
            passed = regression_percent <= budget
            report = {
                "status": "PASS" if passed else "REGRESSION_REQUIRES_CORRECTION_OR_ADR",
                "decision": "compatible-baseline-comparison",
                "policy_version": policy["policy_version"],
                "p50_regression_percent": regression_percent,
                "allowed_p50_regression_percent": budget,
                "result": current,
                "baseline": baseline_result,
            }
            exit_code = 0 if passed else 1

    payload = json.dumps(report, indent=2, sort_keys=True) + "\n"
    if args.output:
        output = args.output if args.output.is_absolute() else ROOT / args.output
        output.parent.mkdir(parents=True, exist_ok=True)
        output.write_text(payload, encoding="utf-8")
    print(payload, end="")
    return exit_code


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as exc:
        print(f"M01 PRB failed: {exc}")
        raise SystemExit(1)
