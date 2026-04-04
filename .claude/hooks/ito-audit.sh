#!/usr/bin/env bash

set -euo pipefail

# Read hook payload from stdin to satisfy Claude hook contract.
HOOK_PAYLOAD="$(cat || true)"
if [ -z "${HOOK_PAYLOAD}" ]; then
  HOOK_PAYLOAD='{}'
fi

# Attempt audit validation — warn on failure but never block.
# Blocking here creates a catch-22: the fix command (ito audit reconcile --fix)
# also requires Bash, which this hook would block.
if ! ito audit validate >/dev/null 2>&1; then
  cat <<'EOF'
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "additionalContext": "Ito audit validation failed. Run `ito audit validate` to inspect issues and `ito audit reconcile --fix` if drift must be repaired."
  }
}
EOF
  exit 0
fi

RECONCILE_OUTPUT="$(ito audit reconcile 2>&1 || true)"
if printf '%s' "${RECONCILE_OUTPUT}" | grep -Eq 'drift items found|^\s+-\s+(Missing|Extra):'; then
  cat <<'EOF'
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "additionalContext": "Ito audit drift detected. Run `ito audit reconcile --fix` to write compensating events before continuing."
  }
}
EOF
fi

exit 0
