#!/usr/bin/env bash
set -euo pipefail

action=${1:?usage: main-first-fixture.sh <init|integrate|retire-old|cleanup> <fixture-root>}
fixture_root=${2:?usage: main-first-fixture.sh <init|integrate|retire-old|cleanup> <fixture-root>}
change_id=031-02_enforce-main-first-implementation

git_in_fixture() {
  git -C "$fixture_root" "$@"
}

case "$action" in
  init)
    rm -rf "$fixture_root"
    mkdir -p "$fixture_root"
    git_in_fixture init --initial-branch=main >/dev/null
    git_in_fixture config user.name "Ito Demo"
    git_in_fixture config user.email "ito-demo@example.invalid"
    git_in_fixture config commit.gpgsign false

    mkdir -p "$fixture_root/.ito"
    cat >"$fixture_root/.ito/config.json" <<'JSON'
{
  "changes": {
    "proposal": { "integration_mode": "direct_merge" },
    "coordination_branch": { "storage": "embedded" }
  },
  "worktrees": {
    "enabled": true,
    "strategy": "checkout_subdir",
    "default_branch": "main",
    "layout": { "base_dir": ".", "dir_name": "ito-worktrees" },
    "apply": { "enabled": true }
  }
}
JSON
    printf '# Main-first demo\n' >"$fixture_root/README.md"
    git_in_fixture add .
    git_in_fixture commit --no-gpg-sign --no-verify -m "configure main-first demo" >/dev/null
    git_in_fixture switch -c "$change_id" >/dev/null

    change_dir="$fixture_root/.ito/changes/$change_id"
    mkdir -p "$change_dir/specs/main-first"
    printf 'schema: spec-driven\n' >"$change_dir/.ito.yaml"
    cat >"$change_dir/proposal.md" <<'MARKDOWN'
# Proposal

Require a reviewed proposal on authoritative main before implementation.
MARKDOWN
    cat >"$change_dir/design.md" <<'MARKDOWN'
# Design

Resolve one immutable authority OID and gate every implementation entry point.
MARKDOWN
    cat >"$change_dir/tasks.md" <<'MARKDOWN'
## Wave 1
- **Depends On**: None

### Task 1.1: Demonstrate main-first execution
- **Dependencies**: None
- **Updated At**: 2026-07-13
- **Status**: [ ] pending
MARKDOWN
    cat >"$change_dir/specs/main-first/spec.md" <<'MARKDOWN'
## ADDED Requirements

### Requirement: Main-first execution
Ito SHALL reject implementation until the reviewed proposal is integrated into main.

#### Scenario: Accepted proposal
- **WHEN** implementation begins
- **THEN** its checkout contains the proposal integration commit
MARKDOWN
    ;;
  integrate)
    git_in_fixture add ".ito/changes/$change_id"
    git_in_fixture commit --no-gpg-sign --no-verify -m "propose main-first execution" >/dev/null
    git_in_fixture switch main >/dev/null
    git_in_fixture merge --no-ff "$change_id" --no-gpg-sign -m "integrate reviewed proposal" >/dev/null
    git_in_fixture switch "$change_id" >/dev/null
    ;;
  retire-old)
    git_in_fixture switch main >/dev/null
    git_in_fixture branch -D "$change_id" >/dev/null
    ;;
  cleanup)
    rm -rf "$fixture_root"
    ;;
  *)
    echo "unknown action: $action" >&2
    exit 2
    ;;
esac
