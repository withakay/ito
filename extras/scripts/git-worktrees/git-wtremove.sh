#!/usr/bin/env bash
set -euo pipefail

RED="\033[0;31m"
YELLOW="\033[0;33m"
CLEAR="\033[0m"

WT_VERSION="0.1.0"
script_dir=$(CDPATH= cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)
if [[ -f "$script_dir/VERSION" ]]; then
  WT_VERSION=$(cat "$script_dir/VERSION")
fi

VERBOSE=
FORCE=

usage() {
  cat <<'EOF'
Usage: git wtremove [-vh] [--force] WORKTREE_NAME...

Removes a worktree and prunes stale worktree metadata.

Safety:
  - Refuses to remove locked worktrees.
  - Refuses to remove paths that are not registered worktrees.

FLAGS:
  -h, --help    Print this help
  -v, --verbose Verbose mode
  --force       Force removal (passes --force to git worktree remove)
  --version     Print version
EOF
  exit 1
}

die() {
  if [[ -n "$VERBOSE" ]]; then
    set +x
  fi
  printf '%b%s%b\n' "$RED" "$1" "$CLEAR" >&2
  exit 1
}

warn() {
  printf '%b%s%b\n' "$YELLOW" "$1" "$CLEAR" >&2
}

abs_path() {
  local p=$1
  if [[ -d "$p" ]]; then
    (CDPATH= cd -- "$p" && pwd -P)
    return 0
  fi

  if [[ "$p" = /* ]]; then
    printf '%s\n' "$p"
  else
    printf '%s\n' "$(pwd -P)/$p"
  fi
}

collect_worktrees_porcelain() {
  git worktree list --porcelain 2>/dev/null || true
}

worktree_info_for_path() {
  # Prints two lines to stdout:
  #   locked=<empty|reason>
  #   branch=<empty|refs/heads/...>
  local want=$1
  local want_abs
  want_abs=$(abs_path "$want")

  local locked=""
  local branch=""
  local current=""
  local in_match=false

  local line
  while IFS= read -r line; do
    case "$line" in
      worktree\ *)
        current=${line#worktree }
        in_match=false

        if [[ "$current" == "$want_abs" ]]; then
          in_match=true
          locked=""
          branch=""
        fi
        ;;
      locked\ *)
        if [[ "$in_match" == "true" ]]; then
          locked=${line#locked }
        fi
        ;;
      locked)
        if [[ "$in_match" == "true" ]]; then
          locked="locked"
        fi
        ;;
      branch\ *)
        if [[ "$in_match" == "true" ]]; then
          branch=${line#branch }
        fi
        ;;
    esac
  done < <(collect_worktrees_porcelain)

  printf 'locked=%s\n' "$locked"
  printf 'branch=%s\n' "$branch"
}

remove_one() {
  local name=$1

  is_worktree=$(git rev-parse --is-inside-work-tree 2>/dev/null || echo false)
  if $is_worktree; then
    parent_dir=".."
  else
    parent_dir="."
  fi

  local final_dir="$parent_dir/$name"
  if [[ ! -d "$final_dir" ]]; then
    warn "Unable to find directory $final_dir, skipping"
    return 0
  fi

  local info
  info=$(worktree_info_for_path "$final_dir")
  local locked
  locked=$(printf '%s\n' "$info" | sed -n 's/^locked=//p')
  local branch_ref
  branch_ref=$(printf '%s\n' "$info" | sed -n 's/^branch=//p')

  if [[ -z "$locked" && -z "$branch_ref" ]]; then
    die "$final_dir is not a registered worktree"
  fi

  if [[ -n "$locked" ]]; then
    die "refusing to remove locked worktree: $final_dir (${locked})"
  fi

  warn "removing $final_dir"

  if [[ -n "$VERBOSE" ]]; then
    set -x
  fi

  if [[ -n "$FORCE" ]]; then
    git worktree remove --force "$final_dir"
  else
    git worktree remove "$final_dir"
  fi

  git worktree prune

  if [[ "$branch_ref" == refs/heads/* ]]; then
    local branch_name=${branch_ref#refs/heads/}
    git branch -D "$branch_name" >/dev/null 2>&1 || true
  fi

  if [[ -n "$VERBOSE" ]]; then
    set +x
  fi
}

while true; do
  case "${1:-}" in
    help|-h|--help)
      usage
      ;;
    --version)
      printf '%s\n' "$WT_VERSION"
      exit 0
      ;;
    -v|--verbose)
      VERBOSE=true
      shift
      ;;
    --force)
      FORCE=true
      shift
      ;;
    *)
      break
      ;;
  esac
done

[[ -n "${1:-}" ]] || usage

while [[ -n "${1:-}" ]]; do
  remove_one "$1"
  shift
done
