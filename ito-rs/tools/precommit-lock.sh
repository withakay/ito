#!/usr/bin/env bash
# precommit-lock.sh — Advisory lock for pre-commit hook runs.
#
# Prevents concurrent processes (agents, editors, watchers) from modifying
# the working tree while prek is stashing/unstashing during a git commit.
#
# Lock file location: <worktree-gitdir>/precommit.lock
# Lock file content:  PID <pid> TIMESTAMP <epoch> COMMAND <cmdline>
#
# Usage:
#   precommit-lock.sh acquire [--timeout SECS] [--owner-pid PID]
#   precommit-lock.sh release [--owner-pid PID]
#   precommit-lock.sh check
#   precommit-lock.sh wait [--timeout SECS]
#
# The --owner-pid flag records a specific PID as the lock owner (for stale
# detection). Use this when the lock script runs in a short-lived subprocess
# but the actual owner is a long-lived parent process (e.g., the hook wrapper).
#
# Stale lock detection: If the PID in the lock file is no longer running,
# the lock is considered stale and will be forcibly removed.

set -euo pipefail

LOCK_TIMEOUT="${LOCK_TIMEOUT:-60}"
STALE_THRESHOLD=600  # 10 minutes — force-remove regardless of PID

# Resolve the per-worktree git directory (not the shared .bare)
GIT_DIR="$(git rev-parse --git-dir 2>/dev/null)" || {
    echo "precommit-lock: not inside a git repository" >&2
    exit 1
}

LOCK_FILE="${GIT_DIR}/precommit.lock"

_lock_info() {
    if [ -f "$LOCK_FILE" ]; then
        cat "$LOCK_FILE"
    fi
}

_lock_pid() {
    if [ -f "$LOCK_FILE" ]; then
        awk '/^PID / { print $2 }' "$LOCK_FILE"
    fi
}

_lock_timestamp() {
    if [ -f "$LOCK_FILE" ]; then
        awk '/^TIMESTAMP / { print $2 }' "$LOCK_FILE"
    fi
}

_is_stale() {
    local pid ts now age
    pid="$(_lock_pid)"
    ts="$(_lock_timestamp)"

    # No lock file
    [ -z "$pid" ] && return 0

    # PID no longer running
    if ! kill -0 "$pid" 2>/dev/null; then
        return 0
    fi

    # Exceeded stale threshold
    now="$(date +%s)"
    age=$(( now - ${ts:-0} ))
    if [ "$age" -ge "$STALE_THRESHOLD" ]; then
        return 0
    fi

    return 1
}

_remove_stale() {
    if [ -f "$LOCK_FILE" ] && _is_stale; then
        echo "precommit-lock: removing stale lock ($(cat "$LOCK_FILE"))" >&2
        rm -f "$LOCK_FILE"
    fi
}

cmd_acquire() {
    local timeout="$LOCK_TIMEOUT"
    local owner_pid="$$"
    while [ $# -gt 0 ]; do
        case "$1" in
            --timeout) timeout="$2"; shift 2 ;;
            --owner-pid) owner_pid="$2"; shift 2 ;;
            *)
                echo "precommit-lock: warning: unknown option '$1'" >&2
                shift
                ;;
        esac
    done

    local deadline=$(( $(date +%s) + timeout ))

    while true; do
        _remove_stale

        if [ ! -f "$LOCK_FILE" ]; then
            # Write lock atomically via temp file + mv
            local tmp="${LOCK_FILE}.$$"
            {
                echo "PID ${owner_pid}"
                echo "TIMESTAMP $(date +%s)"
                echo "COMMAND $0 $*"
                echo "PPID ${PPID:-unknown}"
            } > "$tmp"
            mv "$tmp" "$LOCK_FILE" 2>/dev/null && {
                # Verify we own it (handle race with another acquirer)
                local file_pid
                file_pid="$(_lock_pid)"
                if [ "$file_pid" = "${owner_pid}" ]; then
                    return 0
                fi
            }
            rm -f "$tmp" 2>/dev/null || true
        fi

        if [ "$(date +%s)" -ge "$deadline" ]; then
            echo "precommit-lock: timed out waiting for lock after ${timeout}s" >&2
            echo "  Lock held by: $(_lock_info)" >&2
            echo "  Lock file: ${LOCK_FILE}" >&2
            return 1
        fi

        sleep 1
    done
}

cmd_release() {
    local owner_pid="$$"
    while [ $# -gt 0 ]; do
        case "$1" in
            --owner-pid) owner_pid="$2"; shift 2 ;;
            *)
                echo "precommit-lock: warning: unknown option '$1'" >&2
                shift
                ;;
        esac
    done

    local pid
    pid="$(_lock_pid)"

    if [ -z "$pid" ]; then
        return 0  # No lock to release
    fi

    # Only release our own lock (or stale ones)
    if [ "$pid" = "${owner_pid}" ] || [ "$pid" = "$$" ] || [ "$pid" = "${PPID:-}" ] || _is_stale; then
        rm -f "$LOCK_FILE"
        return 0
    fi

    echo "precommit-lock: lock owned by PID $pid, not releasing" >&2
    return 1
}

cmd_check() {
    _remove_stale
    if [ -f "$LOCK_FILE" ]; then
        echo "locked: $(_lock_info)"
        return 0
    else
        return 1
    fi
}

cmd_wait() {
    local timeout="$LOCK_TIMEOUT"
    while [ $# -gt 0 ]; do
        case "$1" in
            --timeout) timeout="$2"; shift 2 ;;
            *)
                echo "precommit-lock: warning: unknown option '$1'" >&2
                shift
                ;;
        esac
    done

    local deadline=$(( $(date +%s) + timeout ))

    while true; do
        _remove_stale

        if [ ! -f "$LOCK_FILE" ]; then
            return 0
        fi

        if [ "$(date +%s)" -ge "$deadline" ]; then
            echo "precommit-lock: timed out waiting for lock release after ${timeout}s" >&2
            return 1
        fi

        sleep 1
    done
}

# --- Main ---
case "${1:-}" in
    acquire) shift; cmd_acquire "$@" ;;
    release) shift; cmd_release "$@" ;;
    check)   cmd_check ;;
    wait)    cmd_wait "$@" ;;
    *)
        echo "Usage: precommit-lock.sh {acquire|release|check|wait} [--timeout SECS]" >&2
        exit 1
        ;;
esac
