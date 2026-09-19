"""Generate a deterministic CycloneDX dependency SBOM from Cargo metadata."""

from __future__ import annotations

import argparse
import hashlib
import json
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]


def cargo_metadata() -> dict:
    raw = subprocess.check_output(
        ["cargo", "metadata", "--locked", "--format-version", "1"],
        cwd=ROOT,
        text=True,
    )
    return json.loads(raw)


def purl(package: dict) -> str:
    return f"pkg:cargo/{package['name']}@{package['version']}"


def build_bom(metadata: dict) -> dict:
    packages = sorted(metadata["packages"], key=lambda item: (item["name"], item["version"]))
    components = []
    refs = {}
    for package in packages:
        ref = f"pkg:{package['name']}@{package['version']}"
        refs[package["id"]] = ref
        component = {
            "type": "library",
            "bom-ref": ref,
            "name": package["name"],
            "version": package["version"],
            "purl": purl(package),
        }
        if package.get("license"):
            component["licenses"] = [{"license": {"id": package["license"]}}]
        components.append(component)

    dependencies = []
    for node in sorted(metadata.get("resolve", {}).get("nodes", []), key=lambda item: item["id"]):
        ref = refs.get(node["id"])
        if ref is None:
            continue
        dependencies.append(
            {
                "ref": ref,
                "dependsOn": sorted(
                    refs[dependency["pkg"]]
                    for dependency in node.get("deps", [])
                    if dependency.get("pkg") in refs
                ),
            }
        )

    return {
        "bomFormat": "CycloneDX",
        "specVersion": "1.5",
        "serialNumber": "urn:uuid:00000000-0000-0000-0000-000000000001",
        "version": 1,
        "metadata": {
            "tools": [{"vendor": "Rust", "name": "cargo metadata"}],
            "component": {"type": "application", "name": "core", "version": "0.1.0"},
        },
        "components": components,
        "dependencies": dependencies,
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()

    bom = build_bom(cargo_metadata())
    payload = (json.dumps(bom, indent=2, sort_keys=True) + "\n").encode("utf-8")
    output = args.output if args.output.is_absolute() else ROOT / args.output
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_bytes(payload)
    digest = hashlib.sha256(payload).hexdigest()
    output.with_suffix(output.suffix + ".sha256").write_text(
        f"{digest}  {output.name}\n", encoding="utf-8"
    )
    print(json.dumps({"path": str(output), "sha256": digest}, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
