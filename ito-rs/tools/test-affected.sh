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
#   ito-domain      -> ito-core, ito-cli
#   ito-templates   -> ito-core, ito-cli
#   ito-logging     -> ito-cli
#   ito-core        -> ito-cli
#   ito-cli         -> (leaf)
#   ito-web         -> (leaf)
#   ito-test-support -> ito-core (dev), ito-cli (dev), ito-domain (dev)

declare -A DEPENDENTS
DEPENDENTS[ito-common]="ito-config ito-domain ito-core ito-cli"
DEPENDENTS[ito-config]="ito-core ito-cli"
DEPENDENTS[ito-domain]="ito-core ito-cli"
DEPENDENTS[ito-templates]="ito-core ito-cli"
DEPENDENTS[ito-logging]="ito-cli"
DEPENDENTS[ito-core]="ito-cli"
DEPENDENTS[ito-test-support]="ito-core ito-cli ito-domain"
DEPENDENTS[ito-cli]=""
DEPENDENTS[ito-web]=""

# BFS worklist to collect transitive dependents.
# Seed with the directly changed crates, then expand until stable.
AFFECTED_CRATES=()
WORKLIST=("${CHANGED_CRATES[@]}")

while [ ${#WORKLIST[@]} -gt 0 ]; do
    crate="${WORKLIST[0]}"
    WORKLIST=("${WORKLIST[@]:1}")

    # Skip if already seen
    found=0
    for existing in "${AFFECTED_CRATES[@]+"${AFFECTED_CRATES[@]}"}"; do
        [ "$existing" = "$crate" ] && found=1 && break
    done
    [ "$found" -eq 1 ] && continue

    AFFECTED_CRATES+=("$crate")

    # Enqueue direct dependents for further expansion
    for dep in ${DEPENDENTS[$crate]:-}; do
        found=0
        for existing in "${AFFECTED_CRATES[@]+"${AFFECTED_CRATES[@]}"}"; do
            [ "$existing" = "$dep" ] && found=1 && break
        done
        [ "$found" -eq 0 ] && WORKLIST+=("$dep")
    done
done

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
