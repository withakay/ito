#!/usr/bin/env python3

import json
import subprocess
import sys
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
WORKSPACE_MANIFEST = REPO_ROOT / "ito-rs" / "Cargo.toml"

FORBIDDEN_CRATE_EDGES = {
    "ito-domain": {"ito-cli", "ito-web"},
}

DOMAIN_API_BASELINE: dict[str, dict[str, int]] = {
    "std::fs": {
        "ito-rs/crates/ito-domain/src/changes/repository.rs": 2,
        "ito-rs/crates/ito-domain/src/discovery.rs": 9,
        "ito-rs/crates/ito-domain/src/modules/repository.rs": 2,
        "ito-rs/crates/ito-domain/src/planning.rs": 5,
        "ito-rs/crates/ito-domain/src/tasks/repository.rs": 2,
        "ito-rs/crates/ito-domain/src/workflow.rs": 8,
    },
    "std::process::Command": {},
}


def run_cargo_metadata() -> dict:
    command = [
        "cargo",
        "metadata",
        "--manifest-path",
        str(WORKSPACE_MANIFEST),
        "--format-version",
        "1",
        "--no-deps",
    ]
    try:
        completed = subprocess.run(
            command,
            cwd=REPO_ROOT,
            check=True,
            capture_output=True,
            text=True,
        )
    except subprocess.CalledProcessError as exc:
        print("FAIL: unable to run cargo metadata", file=sys.stderr)
        print(exc.stderr, file=sys.stderr)
        raise SystemExit(1) from exc

    return json.loads(completed.stdout)


def check_crate_edges() -> list[str]:
    metadata = run_cargo_metadata()
    packages = {pkg["name"]: pkg for pkg in metadata["packages"]}

    violations: list[str] = []
    for source_crate, forbidden_targets in FORBIDDEN_CRATE_EDGES.items():
        source = packages.get(source_crate)
        if source is None:
            violations.append(f"missing workspace crate: {source_crate}")
            continue

        dep_names = {dependency["name"] for dependency in source["dependencies"]}
        for forbidden_target in sorted(forbidden_targets):
            if forbidden_target in dep_names:
                violations.append(
                    f"forbidden dependency edge: {source_crate} -> {forbidden_target}"
                )

    return violations


def count_occurrences(contents: str, needle: str) -> int:
    return contents.count(needle)


def check_domain_api_bans() -> list[str]:
    domain_src = REPO_ROOT / "ito-rs" / "crates" / "ito-domain" / "src"
    violations: list[str] = []

    for symbol, baseline in DOMAIN_API_BASELINE.items():
        actual_counts: dict[str, int] = {}

        for path in sorted(domain_src.rglob("*.rs")):
            rel_path = path.relative_to(REPO_ROOT).as_posix()
            count = count_occurrences(path.read_text(encoding="utf-8"), symbol)
            if count > 0:
                actual_counts[rel_path] = count

        for rel_path, count in sorted(actual_counts.items()):
            allowed = baseline.get(rel_path)
            if allowed is None:
                violations.append(f"new {symbol} usage in {rel_path} ({count} matches)")
                continue
            if count > allowed:
                violations.append(
                    f"increased {symbol} usage in {rel_path} ({count} > baseline {allowed})"
                )

    return violations


def report(group_name: str, violations: list[str]) -> None:
    if not violations:
        print(f"OK: {group_name}")
        return

    print(f"FAIL: {group_name}")
    for violation in violations:
        print(f"  - {violation}")


def main() -> int:
    edge_violations = check_crate_edges()
    api_violations = check_domain_api_bans()

    report("crate edge rules", edge_violations)
    report("ito-domain API bans", api_violations)

    if edge_violations or api_violations:
        return 1

    print("Architecture guardrails passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
