#!/usr/bin/env bash
# Based on: https://github.com/llimllib/personal_code/blob/master/homedir/.local/bin/worktree
#
# Adjusted to work with bare repos.

set -e

RED="\033[0;31m"
GREEN="\033[0;32m"
YELLOW="\033[0;33m"
CLEAR="\033[0m"
VERBOSE=
LOCK_AFTER_CREATE=

WT_VERSION="0.1.0"
script_dir=$(CDPATH= cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)
if [[ -f "$script_dir/VERSION" ]]; then
  WT_VERSION=$(cat "$script_dir/VERSION")
fi

function usage {
    cat <<EOF
Usage: git wtadd [-vh] [--lock] WORKTREE_NAME [BRANCH_NAME]

Create a git worktree named WORKTREE_NAME based on BRANCH_NAME.
If no BRANCH_NAME is provided, it will default to the current branch.

Will copy over any .env, .envrc, .tool-versions, or mise.toml files to the
new worktree as well as node_modules.

FLAGS:
  -h, --help    Print this help
  -v, --verbose Verbose mode
  --lock        Lock the created worktree (prevents removal/prune)
  --version     Print version
EOF
    kill -INT $$
}

function die {
    printf '%b%s%b\n' "$RED" "$1" "$CLEAR" >&2
    # exit the script, but if it was sourced, don't kill the shell
    kill -INT $$
}

function warn {
    printf '%b%s%b\n' "$YELLOW" "$1" "$CLEAR" >&2
}

function cp_cow {
    # Prefer copy-on-write when available.
    if ! /bin/cp -Rc "$1" "$2" 2>/dev/null; then
        if ! /bin/cp -R --reflink "$1" "$2" 2>/dev/null; then
            if ! /bin/cp -R "$1" "$2" 2>/dev/null; then
                warn "Unable to copy file $1 to $2 - folder may not exist"
            fi
        fi
    fi
}

function _worktree {
    if [ -z "${1:-}" ]; then
        usage
    fi

    if [ -n "$VERBOSE" ]; then
        set -x
    fi
    branchname="$1"

    dirname=${branchname//\//_}

    is_worktree=$(git rev-parse --is-inside-work-tree)
    if $is_worktree; then
        parent_dir=".."
    else
        parent_dir="."
    fi

    if git for-each-ref --format='%(refname:lstrip=2)' refs/heads | grep -E "^$branchname$" > /dev/null 2>&1; then
        git worktree add "$parent_dir/$dirname" "$branchname" || die "failed to create git worktree $branchname"
    elif git for-each-ref --format='%(refname:lstrip=3)' refs/remotes/origin | grep -E "^$branchname$" > /dev/null 2>&1; then
        git worktree add "$parent_dir/$dirname" "$branchname" || die "failed to create git worktree $branchname"
    else
        git worktree add -b "$branchname" "$parent_dir/$dirname" || die "failed to create git worktree $branchname"
    fi

    if [ -d "node_modules" ]; then
      cp_cow node_modules "$parent_dir/$dirname"/node_modules
    fi

    IFS=$'\n'

    platform=$(uname)
    if $is_worktree; then
        copy_source="."
    else
        copy_source=./$(git rev-parse --abbrev-ref HEAD)
    fi
    if [ "$platform" = "Darwin" ]; then
        # shellcheck disable=SC2207
        files_to_copy=( $(find -E "$copy_source" -not -path '*node_modules*' -and \
                -iregex '.*\/\.(envrc|env|env.local|tool-versions|mise.toml)' ) )
    else
        # shellcheck disable=SC2207
        files_to_copy=( $(find "$copy_source" -not -path '*node_modules*' -and \
                -regextype posix-extended -iregex '.*\/\.(envrc|env|env.local|tool-versions|mise.toml)' ) )
    fi

    for f in "${files_to_copy[@]}"; do
      target_path="${f#$copy_source/}"
      cp_cow "$f" "$parent_dir/$dirname/$target_path"
    done

    unset IFS

    if ! env -u GIT_DIR -u GIT_WORK_TREE git -C "$parent_dir/$dirname" pull >/dev/null 2>&1; then
        warn "Unable to run git pull, there may not be an upstream"
    fi

    if [ -f "$parent_dir/$dirname/.envrc" ]; then
        direnv allow "$parent_dir/$dirname"
    fi

    if [[ -n "$LOCK_AFTER_CREATE" ]]; then
        if ! git worktree lock "$parent_dir/$dirname" --reason "locked by git wtadd" >/dev/null 2>&1; then
            warn "Unable to lock worktree $parent_dir/$dirname"
        fi
    fi

    printf "%bcreated worktree %s%b\n" "$GREEN" "$parent_dir/$dirname" "$CLEAR"
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
        --lock)
            LOCK_AFTER_CREATE=true
            shift
            ;;
        *)
            break
            ;;
    esac
done

_worktree "$@"
