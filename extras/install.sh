#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage:
  extras/install.sh git-worktrees [--prefix <dir>] [--dry-run] [--force] [--aliases] [--alias <name>=<cmd>]

Installs optional helper scripts into a directory on your PATH.

Defaults:
  --prefix  ~/.local/bin

Notes:
  - This installer installs Git external subcommands (git-*) so you can run them
    via `git <name>`.
  - It drops the `.sh` extension at install time.
  - Use --aliases/--alias to install short Git aliases into your global config.
EOF
}

die() {
  printf '%s\n' "$1" >&2
  exit 1
}

main() {
  if [[ ${1:-} == "-h" || ${1:-} == "--help" || ${1:-} == "help" ]]; then
    usage
    exit 0
  fi

  local extra=${1:-}
  [[ -n "$extra" ]] || die "missing extra name (try: git-worktrees)"
  shift || true

  local prefix="$HOME/.local/bin"
  local dry_run=false
  local force=false
  local install_aliases=false
  local -a alias_specs=()

  while [[ $# -gt 0 ]]; do
    case "$1" in
      --prefix)
        prefix=${2:-}
        [[ -n "$prefix" ]] || die "--prefix requires a value"
        shift 2
        ;;
      --dry-run)
        dry_run=true
        shift
        ;;
      --force)
        force=true
        shift
        ;;
      --aliases)
        install_aliases=true
        shift
        ;;
      --alias)
        spec=${2:-}
        [[ -n "$spec" ]] || die "--alias requires a value like wta=wtadd"
        alias_specs+=("$spec")
        shift 2
        ;;
      *)
        die "unknown argument: $1"
        ;;
    esac
  done

  case "$extra" in
    git-worktrees)
      if [[ "$install_aliases" == "true" ]]; then
        alias_specs+=("wta=wtadd" "wtls=wtlist" "wtrm=wtremove")
      fi
      install_git_worktrees "$prefix" "$dry_run" "$force" "${alias_specs[@]}"
      ;;
    *)
      die "unknown extra: $extra"
      ;;
  esac
}

install_git_worktrees() {
  local prefix=$1
  local dry_run=$2
  local force=$3
  shift 3
  local -a alias_specs=("$@")

  local root
  root=$(CDPATH= cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)

  local src_dir="$root/extras/scripts/git-worktrees"
  [[ -d "$src_dir" ]] || die "missing scripts dir: $src_dir"

  if [[ "$dry_run" == "false" ]]; then
    mkdir -p "$prefix"
  fi

  local installed=0
  local f
  for f in "$src_dir"/git-*.sh; do
    [[ -f "$f" ]] || continue

    local base
    base=$(basename "$f")
    local target_name=${base%.sh}
    local target="$prefix/$target_name"

    if [[ -e "$target" && "$force" != "true" ]]; then
      printf 'skip %s (exists; use --force)\n' "$target"
      continue
    fi

    if [[ "$dry_run" == "true" ]]; then
      printf 'install %s -> %s\n' "$f" "$target"
    else
      install -m 0755 "$f" "$target"
      printf 'installed %s\n' "$target"
    fi

    installed=$((installed + 1))
  done

  if [[ $installed -eq 0 ]]; then
    die "no scripts found in $src_dir"
  fi

  if [[ ${#alias_specs[@]} -gt 0 ]]; then
    install_git_aliases "$dry_run" "$force" "${alias_specs[@]}"
  fi
}

install_git_aliases() {
  local dry_run=$1
  local force=$2
  shift 2
  local -a alias_specs=("$@")

  local spec
  for spec in "${alias_specs[@]}"; do
    case "$spec" in
      *=*)
        ;;
      *)
        die "invalid --alias value: $spec (expected name=cmd, e.g. wta=wtadd)"
        ;;
    esac

    local name=${spec%%=*}
    local cmd=${spec#*=}
    [[ -n "$name" ]] || die "invalid alias name in: $spec"
    [[ -n "$cmd" ]] || die "invalid alias command in: $spec"

    local key="alias.$name"
    local existing=""
    existing=$(git config --global --get "$key" 2>/dev/null || true)

    if [[ -n "$existing" && "$existing" != "$cmd" && "$force" != "true" ]]; then
      printf 'skip git %s (alias exists: %s; use --force)\n' "$name" "$existing"
      continue
    fi

    if [[ "$dry_run" == "true" ]]; then
      printf 'set git %s -> %s\n' "$name" "$cmd"
    else
      git config --global "$key" "$cmd"
      printf 'set git %s -> %s\n' "$name" "$cmd"
    fi
  done
}

main "$@"
