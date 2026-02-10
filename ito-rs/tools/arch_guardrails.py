#!/usr/bin/env python3
"""Architecture guardrails for the Ito workspace.

Enforces onion-architecture dependency direction and domain purity rules
that cannot be expressed in standard Rust ecosystem tooling (cargo-deny,
clippy, etc.).

Where possible, checks delegate to Cargo-native commands.  Only checks
that *require* custom logic remain here.

See also:
  - ito-rs/deny.toml          (cargo-deny: license/advisory checks)
  - .pre-commit-config.yaml   (prek hooks including this script)
  - Makefile                   (arch-guardrails target)

Migration note (015-13): Bespoke baseline-count checks for ito-domain
and ito-core API bans are intentionally kept as a temporary guardrail
until compiler-backed enforcement (e.g. custom clippy lints or module
visibility) replaces them.  Each baseline is documented with scope and
migration intent.
"""

import json
import subprocess
import sys
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
WORKSPACE_MANIFEST = REPO_ROOT / "ito-rs" / "Cargo.toml"


# ── Onion layer dependency rules ─────────────────────────────────────
#
# Layers (inside-out): ito-domain → ito-core → adapters (cli, web)
#
# cargo-deny cannot enforce intra-workspace edges, so we check via
# cargo metadata (the lightest Cargo-native data source).

FORBIDDEN_CRATE_EDGES: dict[str, set[str]] = {
    "ito-domain": {"ito-cli", "ito-web", "ito-core"},
    "ito-core": {"ito-cli", "ito-web"},
    "ito-cli": {"ito-domain"},  # must route through ito-core
}

REQUIRED_CRATE_EDGES: dict[str, set[str]] = {
    "ito-core": {"ito-domain", "ito-config"},
    "ito-cli": {"ito-core"},
    "ito-web": {"ito-core"},
}


# ── Domain purity baselines (temporary) ──────────────────────────────
#
# These string-count baselines enforce that production code in ito-domain
# does not import APIs that belong to outer layers or the OS.
#
# Migration intent: replace with compiler-backed enforcement (e.g.
# cfg-gated module visibility, custom clippy lints, or Rust 2024 module
# restrictions) once such tooling matures.
#
# Format: { "symbol_needle": { "relative/path.rs": max_count, ... } }
# An empty dict means zero tolerance (no occurrences allowed anywhere).

DOMAIN_API_BASELINE: dict[str, dict[str, int]] = {
    # miette is an outer-layer error-reporting crate; domain must not use it.
    "miette::": {},
    # std::fs — domain must be deterministic.  The 9 hits in discovery.rs
    # are ALL in #[cfg(test)] fixture setup (production uses FileSystem trait).
    # audit/context.rs uses std::fs for session ID persistence (6 hits), which is
    # justified as context resolution is inherently tied to EventContext (domain).
    "std::fs": {
        "ito-rs/crates/ito-domain/src/discovery.rs": 9,
        "ito-rs/crates/ito-domain/src/audit/context.rs": 6,
    },
    # std::process::Command — domain must not spawn processes.
    # audit/context.rs uses it to run git commands for context resolution (1 hit),
    # which is necessary for EventContext which belongs in the domain layer.
    "std::process::Command": {
        "ito-rs/crates/ito-domain/src/audit/context.rs": 1,
    },
}


# ── Helpers ───────────────────────────────────────────────────────────


def _cargo_metadata() -> dict:
    """Return parsed cargo metadata for the workspace (no-deps)."""
    cmd = [
        "cargo",
        "metadata",
        "--manifest-path",
        str(WORKSPACE_MANIFEST),
        "--format-version",
        "1",
        "--no-deps",
    ]
    try:
        result = subprocess.run(
            cmd, cwd=REPO_ROOT, check=True, capture_output=True, text=True
        )
    except subprocess.CalledProcessError as exc:
        print("FAIL: unable to run cargo metadata", file=sys.stderr)
        print(exc.stderr, file=sys.stderr)
        raise SystemExit(1) from exc
    return json.loads(result.stdout)


def _report(name: str, violations: list[str]) -> None:
    if not violations:
        print(f"  OK: {name}")
    else:
        print(f"  FAIL: {name}")
        for v in violations:
            print(f"    - {v}")


# ── Check: crate edge rules (cargo metadata) ─────────────────────────


def check_crate_edges() -> list[str]:
    """Verify forbidden and required workspace dependency edges."""
    metadata = _cargo_metadata()
    packages = {p["name"]: p for p in metadata["packages"]}
    violations: list[str] = []

    for src, forbidden in FORBIDDEN_CRATE_EDGES.items():
        pkg = packages.get(src)
        if pkg is None:
            violations.append(f"missing workspace crate: {src}")
            continue
        dep_names = {d["name"] for d in pkg["dependencies"]}
        for target in sorted(forbidden):
            if target in dep_names:
                violations.append(f"forbidden edge: {src} -> {target}")

    for src, required in REQUIRED_CRATE_EDGES.items():
        pkg = packages.get(src)
        if pkg is None:
            violations.append(f"missing workspace crate: {src}")
            continue
        dep_names = {d["name"] for d in pkg["dependencies"]}
        for target in sorted(required):
            if target not in dep_names:
                violations.append(f"missing required edge: {src} -> {target}")

    return violations


# ── Check: CLI feature decoupling (Cargo-native) ─────────────────────


def check_cli_feature_decoupling() -> list[str]:
    """Verify ito-cli builds without default features (no ito-web dep).

    Uses ``cargo check`` with --no-default-features, which is a standard
    Cargo command.  This replaced the previous ``cargo tree`` text-parsing
    approach (015-13).
    """
    cmd = [
        "cargo",
        "check",
        "--manifest-path",
        str(WORKSPACE_MANIFEST),
        "-p",
        "ito-cli",
        "--no-default-features",
        "--quiet",
    ]
    try:
        subprocess.run(cmd, cwd=REPO_ROOT, check=True, capture_output=True, text=True)
    except subprocess.CalledProcessError as exc:
        details = exc.stderr.strip() or "cargo check failed"
        return [f"ito-cli --no-default-features build failed: {details}"]
    return []


# ── Check: domain API bans (temporary baseline) ──────────────────────


def check_domain_api_bans() -> list[str]:
    """Check that ito-domain production code respects API baselines.

    Temporary: these string-count checks will be replaced with
    compiler-backed enforcement when suitable tooling is available.
    """
    domain_src = REPO_ROOT / "ito-rs" / "crates" / "ito-domain" / "src"
    violations: list[str] = []

    for symbol, baseline in DOMAIN_API_BASELINE.items():
        for path in sorted(domain_src.rglob("*.rs")):
            rel = path.relative_to(REPO_ROOT).as_posix()
            count = path.read_text(encoding="utf-8").count(symbol)
            if count == 0:
                continue
            allowed = baseline.get(rel)
            if allowed is None:
                violations.append(f"new {symbol} usage in {rel} ({count} hits)")
            elif count > allowed:
                violations.append(
                    f"increased {symbol} usage in {rel} ({count} > baseline {allowed})"
                )

    return violations


# ── Main ──────────────────────────────────────────────────────────────


def main() -> int:
    print("Architecture guardrails")
    print("=" * 40)

    edge = check_crate_edges()
    decoupling = check_cli_feature_decoupling()
    domain_bans = check_domain_api_bans()

    _report("crate edge rules", edge)
    _report("CLI feature decoupling", decoupling)
    _report("domain API bans (temporary baseline)", domain_bans)

    if edge or decoupling or domain_bans:
        print()
        print("Architecture guardrails FAILED.")
        return 1

    print()
    print("Architecture guardrails passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
