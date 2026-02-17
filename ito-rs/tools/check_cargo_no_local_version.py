#!/usr/bin/env python3

from __future__ import annotations

import sys
from pathlib import Path


def _contains_local(version: str) -> bool:
    v = version.strip().lower()
    return "local" in v


def main() -> int:
    try:
        import tomllib  # py3.11+
    except ModuleNotFoundError:
        print("error: python3 must include tomllib (Python 3.11+)", file=sys.stderr)
        return 1

    repo_root = Path(__file__).resolve().parents[2]
    manifest = repo_root / "Cargo.toml"
    if not manifest.exists():
        print(f"error: workspace manifest not found: {manifest}", file=sys.stderr)
        return 1

    try:
        data = tomllib.loads(manifest.read_text(encoding="utf-8"))
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
