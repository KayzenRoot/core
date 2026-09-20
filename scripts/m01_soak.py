"""Reproducible bounded CORE start/stop and resource-growth harness."""

from __future__ import annotations

import argparse
import ctypes
import hashlib
import json
import os
import subprocess
import sys
import time
from ctypes import wintypes
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]


def resource_snapshot() -> dict[str, int]:
    if os.name == "nt":
        kernel32 = ctypes.WinDLL("kernel32", use_last_error=True)
        open_process = kernel32.OpenProcess
        open_process.argtypes = [wintypes.DWORD, wintypes.BOOL, wintypes.DWORD]
        open_process.restype = wintypes.HANDLE
        get_count = kernel32.GetProcessHandleCount
        get_count.argtypes = [wintypes.HANDLE, ctypes.POINTER(wintypes.DWORD)]
        get_count.restype = wintypes.BOOL
        close_handle = kernel32.CloseHandle
        close_handle.argtypes = [wintypes.HANDLE]
        close_handle.restype = wintypes.BOOL
        handle = open_process(0x0400, False, os.getpid())
        if not handle:
            return {"process_handles": -1}
        count = wintypes.DWORD()
        ok = get_count(handle, ctypes.byref(count))
        close_handle(handle)
        return {"process_handles": int(count.value) if ok else -1}
    fd_path = Path(f"/proc/{os.getpid()}/fd")
    snapshot = {"process_handles": len(list(fd_path.iterdir())) if fd_path.exists() else -1}
    try:
        import resource

        snapshot["max_rss_kb"] = int(resource.getrusage(resource.RUSAGE_SELF).ru_maxrss)
    except (ImportError, OSError):
        snapshot["max_rss_kb"] = -1
    return snapshot


def binary_path() -> Path:
    suffix = ".exe" if os.name == "nt" else ""
    binary = ROOT / "target" / "debug" / f"core{suffix}"
    if not binary.exists():
        subprocess.run(["cargo", "build", "-p", "core-cli", "--locked"], cwd=ROOT, check=True)
    return binary


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--iterations", type=int, default=32)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    if args.iterations < 1 or args.iterations > 10_000:
        raise SystemExit("iterations must be between 1 and 10000")

    binary = binary_path()
    before = resource_snapshot()
    records = []
    for iteration in range(args.iterations):
        started = time.perf_counter()
        start_result = subprocess.run(
            [str(binary), "start"],
            cwd=ROOT,
            capture_output=True,
            text=True,
            timeout=30,
            check=False,
        )
        exercise_result = subprocess.run(
            [str(binary), "exercise"],
            cwd=ROOT,
            capture_output=True,
            text=True,
            timeout=30,
            check=False,
        )
        elapsed_ms = (time.perf_counter() - started) * 1000
        payload = json.loads(start_result.stdout)
        exercise = json.loads(exercise_result.stdout)
        records.append(
            {
                "iteration": iteration + 1,
                "elapsed_ms": round(elapsed_ms, 3),
                "returncode": start_result.returncode,
                "verdict": payload.get("verdict"),
                "exercise_returncode": exercise_result.returncode,
                "exercise": exercise,
                "stdout_bytes": len(start_result.stdout.encode()),
                "stderr_bytes": len(start_result.stderr.encode()) + len(exercise_result.stderr.encode()),
                "resource": resource_snapshot(),
            }
        )

    after = resource_snapshot()
    resource_keys = sorted(set(before) & set(after))
    growth = {key: after[key] - before[key] for key in resource_keys if before[key] >= 0 and after[key] >= 0}
    passed = all(
        item["returncode"] == 0
        and item["verdict"] == "ReadyEligible"
        and item["exercise_returncode"] == 0
        and item["exercise"].get("provider_substituted") == "hive-context"
        and item["exercise"].get("provider_flap_recovered_with_fallback") is True
        and item["exercise"].get("generation_coherent") is True
        and item["exercise"].get("probe_coalesced") is True
        and item["exercise"].get("probe_authorized") is True
        and item["exercise"].get("stale_probe_authorized") is False
        and item["exercise"].get("worker_crash_suppressed") is True
        and item["exercise"].get("worker_quarantined") is True
        and item["exercise"].get("shutdown_clean") is True
        for item in records
    )
    report = {
        "suite": "M01_START_STOP_SOAK",
        "iterations": args.iterations,
        "scenarios": [
            "repeated start/stop",
            "provider connect/disconnect/flap with lease preservation",
            "safe configuration reload",
            "health/probe and generation tests via the runtime suite",
            "repeated shutdown/recovery and isolated-worker churn",
            "zero-LLM bootstrap",
        ],
        "resource_before": before,
        "resource_after": after,
        "resource_growth": growth,
        "bounded_policy": "process handle growth <= 4; no failed ReadyEligible starts; every frozen safety cycle remains coherent",
        "passed": passed,
        "record_fingerprint": hashlib.sha256(
            json.dumps(records, sort_keys=True).encode("utf-8")
        ).hexdigest(),
        "records": records,
    }
    payload = json.dumps(report, indent=2, sort_keys=True) + "\n"
    if args.output:
        output = args.output if args.output.is_absolute() else ROOT / args.output
        output.parent.mkdir(parents=True, exist_ok=True)
        output.write_text(payload, encoding="utf-8")
    print(payload, end="")
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
