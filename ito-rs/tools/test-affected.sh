#!/usr/bin/env bash
# Test Impact Analysis: run tests only for crates affected by recent changes.
#
# Usage:
#   ./tools/test-affected.sh              # diff against HEAD~1
#   ./tools/test-affected.sh main         # diff against main branch
#   ./tools/test-affected.sh HEAD~3       # diff against 3 commits ago
#
# The script:
#  1. Finds files changed since the given base ref (default: HEAD~1)
#  2. Maps changed files to workspace crate names
#  3. Expands to transitive dependents (crates that depend on changed crates)
#  4. Runs tests for affected crates only
#
# If no crate-level changes are detected, exits with a message (no tests run).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

BASE_REF="${1:-HEAD~1}"

# Step 1: Find changed files under ito-rs/ relative to repo root.
# We strip the leading "ito-rs/" prefix so paths look like "crates/<name>/...".
REPO_ROOT="$(cd "$WORKSPACE_DIR/.." && pwd)"
RAW_FILES=$(cd "$REPO_ROOT" && git diff --name-only "$BASE_REF" -- ito-rs/ 2>/dev/null) || {
    echo "ERROR: git diff failed for base ref '$BASE_REF'. Is it a valid ref?" >&2
    exit 1
}
CHANGED_FILES=$(echo "$RAW_FILES" | sed 's|^ito-rs/||')

if [ -z "$CHANGED_FILES" ]; then
    echo "No files changed since $BASE_REF — skipping tests."
    exit 0
fi

# Step 2: Map file paths to crate names
# Files under crates/<name>/ belong to crate <name>
CHANGED_CRATES=()
while IFS= read -r file; do
    if [[ "$file" =~ ^crates/([^/]+)/ ]]; then
        crate="${BASH_REMATCH[1]}"
        # Deduplicate
        found=0
        for existing in "${CHANGED_CRATES[@]+"${CHANGED_CRATES[@]}"}"; do
            if [ "$existing" = "$crate" ]; then
                found=1
                break
            fi
        done
        if [ "$found" -eq 0 ]; then
            CHANGED_CRATES+=("$crate")
        fi
    fi
done <<< "$CHANGED_FILES"

if [ ${#CHANGED_CRATES[@]} -eq 0 ]; then
    echo "No workspace crate files changed since $BASE_REF — skipping tests."
    exit 0
fi

echo "Changed crates: ${CHANGED_CRATES[*]}"

# Step 3: Expand to transitive dependents using the known dependency graph.
# This is hardcoded for speed — avoids a `cargo metadata` call (~1-2s overhead).
# NOTE: Update this map when crates are added/removed or dependencies change.
# You can verify with: cargo metadata --format-version 1 | jq '.packages[].dependencies'
#
# Dependency graph (A depends on B means: if B changes, test A):
#   ito-common      -> ito-config, ito-domain, ito-core, ito-cli
#   ito-config      -> ito-core, ito-cli
#   ito-domain      -> ito-core, ito-test-support
#   ito-templates   -> ito-core, ito-web
#   ito-logging     -> ito-cli
#   ito-core        -> ito-cli, ito-web
#   ito-cli         -> (leaf)
#   ito-web         -> ito-cli (optional dependency with default feature)
#   ito-test-support -> ito-cli (dev)

declare -A DEPENDENTS
DEPENDENTS[ito-common]="ito-config ito-domain ito-core ito-cli"
DEPENDENTS[ito-config]="ito-core ito-cli"
DEPENDENTS[ito-domain]="ito-core ito-test-support"
DEPENDENTS[ito-templates]="ito-core ito-web"
DEPENDENTS[ito-logging]="ito-cli"
DEPENDENTS[ito-core]="ito-cli ito-web"
DEPENDENTS[ito-test-support]="ito-cli"
DEPENDENTS[ito-cli]=""
DEPENDENTS[ito-web]="ito-cli"

# BFS worklist to collect transitive dependents.
# Seed with the directly changed crates, then expand until stable.
declare -A affected_set
for crate in "${CHANGED_CRATES[@]}"; do
    affected_set[$crate]=1
done

worklist=("${CHANGED_CRATES[@]}")
while [ ${#worklist[@]} -gt 0 ]; do
    crate=${worklist[0]}
    worklist=("${worklist[@]:1}")

    for dep in ${DEPENDENTS[$crate]:-}; do
        if [[ -z "${affected_set[$dep]}" ]]; then
            affected_set[$dep]=1
            worklist+=("$dep")
        fi
    done
done

AFFECTED_CRATES=("${!affected_set[@]}")
echo "Affected crates (with dependents): ${AFFECTED_CRATES[*]}"

# Step 4: Build the test command
PKG_FLAGS=()
for crate in "${AFFECTED_CRATES[@]}"; do
    PKG_FLAGS+=("-p" "$crate")
done

RUSTFLAGS="${RUSTFLAGS:--D warnings}"
export RUSTFLAGS

if cargo nextest --version >/dev/null 2>&1; then
    echo "Running: cargo nextest run ${PKG_FLAGS[*]}"
    cargo nextest run --manifest-path "$REPO_ROOT/Cargo.toml" "${PKG_FLAGS[@]}"
else
    echo "Running: cargo test ${PKG_FLAGS[*]}"
    cargo test --manifest-path "$REPO_ROOT/Cargo.toml" "${PKG_FLAGS[@]}"
fi
