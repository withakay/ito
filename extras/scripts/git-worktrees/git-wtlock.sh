#!/usr/bin/env bash
set -euo pipefail

WT_VERSION="0.1.0"
script_dir=$(CDPATH= cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)
if [[ -f "$script_dir/VERSION" ]]; then
  WT_VERSION=$(cat "$script_dir/VERSION")
fi

usage() {
  cat <<'EOF'
Usage:
  git wtlock --lock <path> [--reason <text>]
  git wtlock --unlock <path>
  git wtlock --list
  git wtlock --version

Thin wrapper around `git worktree lock/unlock`.
EOF
  exit 1
}

reason="locked by git wtlock"

cmd=${1:-}
case "$cmd" in
  --version)
    printf '%s\n' "$WT_VERSION"
    exit 0
    ;;
  --list)
    git worktree list --porcelain | awk '
      $1=="worktree" {path=$2}
      $1=="branch" {branch=$2}
      $1=="locked" {locked=$0; sub(/^locked[ ]*/,"",locked); print path " " branch " locked=" locked; locked=""}
    '
    exit 0
    ;;
  --lock)
    path=${2:-}
    [[ -n "$path" ]] || usage
    shift 2
    if [[ ${1:-} == "--reason" ]]; then
      reason=${2:-}
      [[ -n "$reason" ]] || usage
      shift 2
    fi
    git worktree lock "$path" --reason "$reason"
    ;;
  --unlock)
    path=${2:-}
    [[ -n "$path" ]] || usage
    git worktree unlock "$path"
    ;;
  help|-h|--help|"")
    usage
    ;;
  *)
    usage
    ;;
esac
