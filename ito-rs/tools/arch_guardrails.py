#!/usr/bin/env python3

import json
import re
import subprocess
import sys
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
WORKSPACE_MANIFEST = REPO_ROOT / "ito-rs" / "Cargo.toml"

FORBIDDEN_CRATE_EDGES = {
    "ito-domain": {"ito-cli", "ito-web", "ito-core"},
    "ito-core": {"ito-cli", "ito-web"},
    "ito-cli": {"ito-domain"},  # Phase 6: CLI must route through ito-core
}

REQUIRED_CRATE_EDGES = {
    "ito-core": {"ito-domain"},
    "ito-cli": {"ito-core"},
    "ito-web": {"ito-core"},
}

# Domain purity baseline.  Production code is zero-tolerance; the sole
# exception is discovery.rs whose 9 std::fs hits are ALL in #[cfg(test)]
# fixture setup (production paths use the FileSystem trait).
DOMAIN_API_BASELINE: dict[str, dict[str, int]] = {
    "miette::": {},
    "std::fs": {
        "ito-rs/crates/ito-domain/src/discovery.rs": 9,  # test-only
    },
    "std::process::Command": {},
}

# Phase 5: miette must not leak into ito-core production code.
# Test code (tests/ dir) is excluded from this check.
# Exception: the harness module (merged from the former ito-harness crate) uses
# miette directly for the Harness trait's Result type and error construction.
CORE_API_BASELINE: dict[str, dict[str, int]] = {
    "miette::": {
        "ito-rs/crates/ito-core/src/harness/types.rs": 1,
        "ito-rs/crates/ito-core/src/harness/opencode.rs": 3,
        "ito-rs/crates/ito-core/src/harness/stub.rs": 4,
    },
    "miette!": {
        "ito-rs/crates/ito-core/src/harness/opencode.rs": 2,
        "ito-rs/crates/ito-core/src/harness/stub.rs": 3,
    },
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

    for source_crate, required_targets in REQUIRED_CRATE_EDGES.items():
        source = packages.get(source_crate)
        if source is None:
            violations.append(f"missing workspace crate: {source_crate}")
            continue

        dep_names = {dependency["name"] for dependency in source["dependencies"]}
        for required_target in sorted(required_targets):
            if required_target not in dep_names:
                violations.append(
                    f"missing required dependency edge: {source_crate} -> {required_target}"
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


def check_core_api_bans() -> list[str]:
    """Check that ito-core production code does not use banned APIs (e.g. miette).

    Only checks src/ files — tests/ are excluded since integration tests may
    need miette (e.g. for implementing the Harness trait).
    The harness module (merged from the former ito-harness crate) has baselines.
    """
    core_src = REPO_ROOT / "ito-rs" / "crates" / "ito-core" / "src"
    violations: list[str] = []

    for symbol, baseline in CORE_API_BASELINE.items():
        actual_counts: dict[str, int] = {}

        for path in sorted(core_src.rglob("*.rs")):
            rel_path = path.relative_to(REPO_ROOT).as_posix()
            contents = path.read_text(encoding="utf-8")
            # Skip counting occurrences inside doc comments (lines starting with ///)
            non_doc_lines = [
                line
                for line in contents.splitlines()
                if not line.strip().startswith("///")
            ]
            count = count_occurrences("\n".join(non_doc_lines), symbol)
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


def check_cli_no_default_features_web_decoupling() -> list[str]:
    command = [
        "cargo",
        "tree",
        "--manifest-path",
        str(WORKSPACE_MANIFEST),
        "-p",
        "ito-cli",
        "--no-default-features",
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
        stderr = exc.stderr.strip()
        details = stderr or "cargo tree command failed"
        return [
            f"unable to verify ito-cli no-default-features dependency tree: {details}"
        ]

    tree_output = completed.stdout
    if re.search(r"^ito-web v", tree_output, flags=re.MULTILINE):
        return ["ito-cli --no-default-features still pulls ito-web in dependency tree"]

    return []


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
    core_api_violations = check_core_api_bans()
    no_web_violations = check_cli_no_default_features_web_decoupling()

    report("crate edge rules", edge_violations)
    report("ito-domain API bans", api_violations)
    report("ito-core API bans", core_api_violations)
    report("ito-cli no-default-features decoupling", no_web_violations)

    if edge_violations or api_violations or core_api_violations or no_web_violations:
        return 1

    print("Architecture guardrails passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
