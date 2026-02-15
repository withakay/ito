#!/usr/bin/env python3

from __future__ import annotations

import argparse
import re
from dataclasses import dataclass
from pathlib import Path


_SEMVER_RE = re.compile(
    r"^(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)"
    r"(?:-(?P<pre>[0-9A-Za-z.-]+))?(?:\+(?P<build>[0-9A-Za-z.-]+))?$"
)


@dataclass(frozen=True)
class Semver:
    major: int
    minor: int
    patch: int

    def as_tuple(self) -> tuple[int, int, int]:
        return (self.major, self.minor, self.patch)

    def __str__(self) -> str:
        return f"{self.major}.{self.minor}.{self.patch}"


def _parse_base_semver(version: str) -> Semver:
    m = _SEMVER_RE.fullmatch(version)
    if not m:
        raise SystemExit(f"version is not valid semver: {version}")
    return Semver(int(m.group("major")), int(m.group("minor")), int(m.group("patch")))


def _base_part(version: str) -> str:
    # Keep only MAJOR.MINOR.PATCH; ignore any -pre or +build.
    m = _SEMVER_RE.fullmatch(version)
    if not m:
        raise SystemExit(f"version is not valid semver: {version}")
    return f"{m.group('major')}.{m.group('minor')}.{m.group('patch')}"


def _read_workspace_version(workspace_manifest: Path) -> str:
    try:
        import tomllib  # py3.11+
    except ModuleNotFoundError as e:
        raise SystemExit(
            "python3 must include tomllib (Python 3.11+) to run this script"
        ) from e

    data = tomllib.loads(workspace_manifest.read_text(encoding="utf-8"))
    version = data.get("workspace", {}).get("package", {}).get("version")
    if not isinstance(version, str) or not version:
        raise SystemExit(f"workspace package version not found in {workspace_manifest}")
    return version


def _replace_version_in_section(
    *,
    manifest: Path,
    section_header: str,
    allow_workspace_version: bool,
    base_version: Semver,
    new_version: str,
) -> tuple[bool, str | None]:
    text = manifest.read_text(encoding="utf-8").splitlines(True)

    out: list[str] = []
    in_section = False
    replaced = False
    saw_workspace_version = False
    old_version: str | None = None

    for line in text:
        stripped = line.strip()

        if stripped == section_header:
            in_section = True
            out.append(line)
            continue

        if in_section and line.lstrip().startswith("[") and stripped.endswith("]"):
            in_section = False

        if in_section:
            if allow_workspace_version and stripped == "version.workspace = true":
                saw_workspace_version = True
                out.append(line)
                continue

            if not replaced:
                m = re.match(r'^version\s*=\s*"([^"]+)"\s*$', stripped)
                if m:
                    ov = m.group(1)
                    old_version = ov
                    old_base = _parse_base_semver(_base_part(ov))
                    if old_base.as_tuple() > base_version.as_tuple():
                        raise SystemExit(
                            f"{manifest}: version {old_version} is higher than workspace version {base_version}"
                        )
                    out.append(f'version = "{new_version}"\n')
                    replaced = True
                    continue

        out.append(line)

    if saw_workspace_version:
        # When version.workspace = true, the actual version is owned by the workspace.
        # We still validate workspace version separately; nothing to rewrite here.
        return (False, None)

    if not replaced:
        raise SystemExit(f"{section_header} version not found in {manifest}")

    manifest.write_text("".join(out), encoding="utf-8")
    return (True, old_version)


def _sync_workspace_dep_versions(
    *,
    manifest: Path,
    members: list[str],
    new_version: str,
) -> None:
    """Rewrite version constraints for internal crates in [workspace.dependencies].

    Cargo's semver resolver treats pre-release versions as incompatible with
    a bare ``^MAJOR.MINOR.PATCH`` requirement (e.g. ``^0.1.4`` does **not**
    match ``0.1.4-local.202602131725``).  When we stamp a local pre-release
    version we must therefore update the ``version`` field inside each
    internal workspace dependency entry so that the constraint explicitly
    matches the stamped version.
    """
    text = manifest.read_text(encoding="utf-8")

    # Build set of crate names from workspace member paths (last path component)
    internal_crate_names: set[str] = set()
    for member in members:
        crate_name = Path(member).name
        internal_crate_names.add(crate_name)

    # Replace version = "..." inside inline-table dependency entries for internal crates.
    # Matches lines like:  ito-common = { path = "...", version = "0.1.4" }
    # We only touch lines whose crate name (with underscores normalised to hyphens) is
    # in the internal set.
    def _replace_dep_version(m: re.Match[str]) -> str:
        crate_name = m.group("crate")
        # Cargo normalises underscores to hyphens for crate names
        if crate_name.replace("-", "_") not in {
            n.replace("-", "_") for n in internal_crate_names
        }:
            return m.group(0)
        prefix = m.group("prefix")
        suffix = m.group("suffix")
        return f'{prefix}"{new_version}"{suffix}'

    pattern = re.compile(
        r'^(?P<prefix>(?P<crate>[a-zA-Z_][a-zA-Z0-9_-]*)\s*=\s*\{[^}]*version\s*=\s*)"[^"]+?"(?P<suffix>.*)$',
        re.MULTILINE,
    )
    new_text = pattern.sub(_replace_dep_version, text)
    if new_text != text:
        manifest.write_text(new_text, encoding="utf-8")


def _read_workspace_members(workspace_manifest: Path) -> list[str]:
    try:
        import tomllib  # py3.11+
    except ModuleNotFoundError as e:
        raise SystemExit(
            "python3 must include tomllib (Python 3.11+) to run this script"
        ) from e

    data = tomllib.loads(workspace_manifest.read_text(encoding="utf-8"))
    members = data.get("workspace", {}).get("members")
    if not isinstance(members, list) or not members:
        raise SystemExit(f"workspace members not found in {workspace_manifest}")

    out: list[str] = []
    for m in members:
        if not isinstance(m, str) or not m:
            raise SystemExit(
                f"invalid workspace member entry in {workspace_manifest}: {m!r}"
            )
        out.append(m)
    return out


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Sync Cargo versions to workspace version + stamp"
    )
    parser.add_argument(
        "--workspace-manifest",
        default="Cargo.toml",
        help="Path to the workspace Cargo.toml",
    )
    parser.add_argument("--stamp", required=True, help="YYYYMMDDHHMM")
    args = parser.parse_args()

    if not re.fullmatch(r"\d{12}", args.stamp):
        raise SystemExit(f"invalid stamp (expected YYYYMMDDHHMM): {args.stamp}")

    workspace_manifest = Path(args.workspace_manifest)
    if not workspace_manifest.exists():
        raise SystemExit(f"workspace manifest not found: {workspace_manifest}")

    workspace_version = _read_workspace_version(workspace_manifest)
    base_version = _parse_base_semver(_base_part(workspace_version))
    new_version = f"{base_version}-local.{args.stamp}"

    # 1) Update workspace.package version
    _replace_version_in_section(
        manifest=workspace_manifest,
        section_header="[workspace.package]",
        allow_workspace_version=False,
        base_version=base_version,
        new_version=new_version,
    )

    # 2) Update workspace.dependencies version constraints for internal crates
    members = _read_workspace_members(workspace_manifest)
    _sync_workspace_dep_versions(
        manifest=workspace_manifest,
        members=members,
        new_version=new_version,
    )

    # 3) Update member crate versions
    workspace_dir = workspace_manifest.parent
    updated: list[Path] = []
    for member in members:
        member_manifest = (workspace_dir / member / "Cargo.toml").resolve()
        if not member_manifest.exists():
            raise SystemExit(f"member manifest not found: {member_manifest}")

        did_replace, _old = _replace_version_in_section(
            manifest=member_manifest,
            section_header="[package]",
            allow_workspace_version=True,
            base_version=base_version,
            new_version=new_version,
        )
        if did_replace:
            updated.append(member_manifest)

    print(new_version)


if __name__ == "__main__":
    main()
