#!/usr/bin/env python3
"""Verify Ito's standard release and opt-in feature boundary."""

from __future__ import annotations

import subprocess
import tomllib
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]


def load_toml(path: str) -> dict:
    with (ROOT / path).open("rb") as handle:
        return tomllib.load(handle)


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


def cargo_tree(*args: str) -> str:
    command = [
        "cargo",
        "tree",
        "-p",
        "ito-cli",
        "--edges",
        "normal",
        "--prefix",
        "none",
        *args,
    ]
    return subprocess.run(
        command,
        cwd=ROOT,
        check=True,
        text=True,
        stdout=subprocess.PIPE,
    ).stdout


workspace = load_toml("Cargo.toml")
cli = load_toml("ito-rs/crates/ito-cli/Cargo.toml")
core = load_toml("ito-rs/crates/ito-core/Cargo.toml")
backend = load_toml("ito-rs/crates/ito-backend/Cargo.toml")
dist = load_toml("dist-workspace.toml")["dist"]

require(
    workspace["workspace"]["default-members"] == ["ito-rs/crates/ito-cli"],
    "workspace default-members must select only ito-cli",
)
require(cli["features"]["default"] == ["web"], "ito-cli defaults must be web-only")
require(
    cli["features"]["experimental"] == ["backend", "coordination-branch"],
    "ito-cli experimental aggregate must contain both opt-in features",
)
require(core["features"]["default"] == [], "ito-core defaults must remain empty")
require(
    backend["dependencies"]["ito-core"].get("default-features") is False
    and backend["dependencies"]["ito-core"].get("features") == ["backend"],
    "ito-backend must enable ito-core/backend explicitly",
)
require(backend["package"].get("publish") is not False, "ito-backend must remain publishable")
require(
    backend["package"]["metadata"]["dist"]["dist"] is False,
    "ito-backend must stay outside standard cargo-dist artifacts",
)

require(dist["packages"] == ["ito-cli"], "cargo-dist must package only ito-cli")
require(dist["default-features"] is False, "cargo-dist defaults must be disabled")
require(dist["features"] == ["web"], "cargo-dist must pin the web feature")
require(dist["all-features"] is False, "cargo-dist must not enable all features")

release_workflow = (ROOT / ".github/workflows/v-release.yml").read_text()
require(
    '"backend", "serve"' not in release_workflow and "service_block" not in release_workflow,
    "standard Homebrew publishing must not inject a backend service",
)
dockerfile = (ROOT / "infra/docker/Dockerfile").read_text()
require(
    "--no-default-features --features backend" in dockerfile,
    "backend Dockerfile must opt into the backend feature explicitly",
)

default_tree = cargo_tree()
backend_tree = cargo_tree("--no-default-features", "--features", "backend")
coordination_tree = cargo_tree(
    "--no-default-features", "--features", "coordination-branch"
)

require("ito-web " in default_tree, "shipping dependency graph must retain ito-web")
require("ito-backend " not in default_tree, "shipping graph must exclude ito-backend")
require("ito-backend " in backend_tree, "backend opt-in graph must include ito-backend")
require("ito-web " not in backend_tree, "backend-only graph must not imply web")
require(
    "ito-backend " not in coordination_tree,
    "coordination-only graph must not imply backend",
)
require("ito-web " not in coordination_tree, "coordination-only graph must not imply web")

shared = [name for name in ("rusqlite", "sha2", "hex") if f"{name} " in default_tree]
print("release feature boundary: ok")
print("standard artifact: ito-cli with explicit web feature")
print("experimental artifact: backend container opts into backend only")
print(f"shared default dependencies retained where used: {', '.join(shared) or 'none'}")
