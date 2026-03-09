#!/usr/bin/env python3

from __future__ import annotations

import sys
from pathlib import Path
import re


def _contains_local(version: str) -> bool:
    v = version.strip().lower()
    return "local" in v


def _scan_without_tomllib(manifest_text: str) -> list[str]:
    offenders: list[str] = []
    section: str | None = None

    dependency_re = re.compile(r"^(?P<name>[A-Za-z0-9_.-]+)\s*=\s*\{(?P<body>.*)\}\s*$")
    version_re = re.compile(r'version\s*=\s*["\'](?P<version>[^"\']+)["\']')

    for raw_line in manifest_text.splitlines():
        line = raw_line.split("#", 1)[0].strip()
        if not line:
            continue

        if line.startswith("[") and line.endswith("]"):
            section = line[1:-1].strip()
            continue

        if section == "workspace.package" and line.startswith("version"):
            let_match = version_re.search(line)
            if let_match is not None:
                version = let_match.group("version")
                if _contains_local(version):
                    offenders.append(f"[workspace.package] version = {version!r}")
            continue

        if section == "workspace.dependencies":
            dep_match = dependency_re.match(line)
            if dep_match is None:
                continue
            version_match = version_re.search(dep_match.group("body"))
            if version_match is None:
                continue
            version = version_match.group("version")
            if _contains_local(version):
                name = dep_match.group("name")
                offenders.append(
                    f"[workspace.dependencies] {name}.version = {version!r}"
                )

    return offenders


def main() -> int:
    tomllib = None
    try:
        import tomllib  # py3.11+
    except ModuleNotFoundError:
        tomllib = None

    repo_root = Path(__file__).resolve().parents[2]
    manifest = repo_root / "Cargo.toml"
    if not manifest.exists():
        print(f"error: workspace manifest not found: {manifest}", file=sys.stderr)
        return 1

    manifest_text = manifest.read_text(encoding="utf-8")

    if tomllib is None:
        offenders = _scan_without_tomllib(manifest_text)
        if not offenders:
            return 0
        print("error: Cargo.toml contains local version metadata:", file=sys.stderr)
        for o in offenders:
            print(f"  - {o}", file=sys.stderr)
        print("", file=sys.stderr)
        print(
            "Do not commit '-local' / '.local' version strings to Cargo.toml. "
            "For local builds, stamp the *reported* CLI version via build-time env vars:",
            file=sys.stderr,
        )
        print("  - ITO_LOCAL_VERSION_STAMP=YYYYMMDDHHMM", file=sys.stderr)
        print("  - (or) ITO_LOCAL_VERSION=1", file=sys.stderr)
        return 1

    try:
        data = tomllib.loads(manifest_text)
    except Exception as e:
        print(f"error: failed to parse {manifest}: {e}", file=sys.stderr)
        return 1

    workspace = data.get("workspace")
    if not isinstance(workspace, dict):
        return 0

    offenders: list[str] = []

    ws_pkg = workspace.get("package")
    if isinstance(ws_pkg, dict):
        v = ws_pkg.get("version")
        if isinstance(v, str) and _contains_local(v):
            offenders.append(f"[workspace.package] version = {v!r}")

    ws_deps = workspace.get("dependencies")
    if isinstance(ws_deps, dict):
        for name, spec in ws_deps.items():
            if not isinstance(spec, dict):
                continue
            v = spec.get("version")
            if isinstance(v, str) and _contains_local(v):
                offenders.append(f"[workspace.dependencies] {name}.version = {v!r}")

    if not offenders:
        return 0

    print("error: Cargo.toml contains local version metadata:", file=sys.stderr)
    for o in offenders:
        print(f"  - {o}", file=sys.stderr)
    print("", file=sys.stderr)
    print(
        "Do not commit '-local' / '.local' version strings to Cargo.toml. "
        "For local builds, stamp the *reported* CLI version via build-time env vars:",
        file=sys.stderr,
    )
    print("  - ITO_LOCAL_VERSION_STAMP=YYYYMMDDHHMM", file=sys.stderr)
    print("  - (or) ITO_LOCAL_VERSION=1", file=sys.stderr)
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
