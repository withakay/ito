#!/usr/bin/env bash
# Based on: https://morgan.cugerone.com/blog/workarounds-to-git-worktree-using-bare-repository-and-cannot-fetch-remote-branches/
set -euo pipefail

RED="\033[0;31m"
GREEN="\033[0;32m"
YELLOW="\033[0;33m"
CLEAR="\033[0m"

WT_VERSION="0.1.0"
script_dir=$(CDPATH= cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)
if [[ -f "$script_dir/VERSION" ]]; then
  WT_VERSION=$(cat "$script_dir/VERSION")
fi

usage() {
  cat <<EOF
Usage: git wtclone [-vh] REPO_URL [DIR_NAME]

Clone a repository into a bare worktree layout.

This will:
  - create a directory named DIR_NAME (defaults to the repo name)
  - clone the repo as a bare repo into .bare
  - fetch all branches
  - add a worktree for the default branch

FLAGS:
  -h, --help    Print this help
  -v, --verbose Verbose mode
  --version     Print version
EOF
  exit 1
}

die() {
  printf '%b%s%b\n' "$RED" "$1" "$CLEAR" >&2
  exit 1
}

VERBOSE=

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
    *)
      break
      ;;
  esac
done

url=${1:-}
[[ -n "$url" ]] || usage

basename=${url##*/}
name=${2:-${basename%.*}}

if [[ -n "$VERBOSE" ]]; then
  set -x
fi

mkdir -p "$name"
cd "$name"

git clone --bare "$url" .bare
printf 'gitdir: ./.bare\n' > .git

git config remote.origin.fetch "+refs/heads/*:refs/remotes/origin/*"
git fetch origin

main_branch=$(git symbolic-ref --quiet --short HEAD 2>/dev/null || true)
if [[ -z "$main_branch" ]]; then
  main_branch=main
fi

git worktree add "$main_branch"
git symbolic-ref HEAD refs/heads/bare

printf "%bCloned repo to %s%b\n" "$GREEN" "$name" "$CLEAR"
