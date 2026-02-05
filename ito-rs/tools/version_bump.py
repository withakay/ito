#!/usr/bin/env python3

from __future__ import annotations

import argparse
import re
from pathlib import Path


_SEMVER_RE = re.compile(
    r"^(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)"
    r"(?:-(?P<pre>[0-9A-Za-z.-]+))?(?:\+(?P<build>[0-9A-Za-z.-]+))?$"
)


def _parse_semver(version: str) -> tuple[int, int, int]:
    m = _SEMVER_RE.fullmatch(version)
    if not m:
        raise SystemExit(f"version is not valid semver: {version}")
    return (int(m.group("major")), int(m.group("minor")), int(m.group("patch")))


def _bump_base(base: tuple[int, int, int], bump: str) -> tuple[int, int, int]:
    major, minor, patch = base
    if bump == "none":
        return (major, minor, patch)
    if bump == "major":
        return (major + 1, 0, 0)
    if bump == "minor":
        return (major, minor + 1, 0)
    if bump == "patch":
        return (major, minor, patch + 1)
    raise SystemExit(f"unknown bump segment: {bump}")


def bump_workspace_package_version(manifest: Path, stamp: str, bump: str) -> str:
    text = manifest.read_text(encoding="utf-8").splitlines(True)

    out: list[str] = []
    in_ws_pkg = False
    replaced = False
    new_version: str | None = None

    for line in text:
        if line.strip() == "[workspace.package]":
            in_ws_pkg = True
            out.append(line)
            continue

        if in_ws_pkg and line.lstrip().startswith("[") and line.strip().endswith("]"):
            in_ws_pkg = False

        if in_ws_pkg and not replaced:
            m = re.match(r'^version\s*=\s*"([^"]+)"\s*$', line.strip())
            if m:
                orig = m.group(1)

                base_version = orig.split("-local.")[0]
                base = _parse_semver(base_version)
                bumped = _bump_base(base, bump)

                new_version = f"{bumped[0]}.{bumped[1]}.{bumped[2]}-local.{stamp}"
                out.append(f'version = "{new_version}"\n')
                replaced = True
                continue

        out.append(line)

    if not replaced or not new_version:
        raise SystemExit(f"[workspace.package] version not found in {manifest}")

    manifest.write_text("".join(out), encoding="utf-8")
    return new_version


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--manifest", required=True)
    parser.add_argument("--stamp", required=True, help="YYYYMMDDHHMM")
    parser.add_argument(
        "--bump",
        choices=["none", "patch", "minor", "major"],
        default="none",
        help="Bump semver base segment before applying -local.<stamp> (default: none)",
    )
    args = parser.parse_args()

    manifest = Path(args.manifest)
    if not manifest.exists():
        raise SystemExit(f"manifest not found: {manifest}")

    stamp = args.stamp
    if not re.fullmatch(r"\d{12}", stamp):
        raise SystemExit(f"invalid stamp (expected YYYYMMDDHHMM): {stamp}")

    print(bump_workspace_package_version(manifest, stamp, args.bump))


if __name__ == "__main__":
    main()
