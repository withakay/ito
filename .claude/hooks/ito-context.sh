#!/usr/bin/env bash
# Claude Code hook: inject Ito continuation context.

set -euo pipefail

event_name="${1:-PreCompact}"

ctx="$(ito agent instruction context 2>/dev/null || true)"
if [ -z "${ctx//[[:space:]]/}" ]; then
  exit 0
fi

wrapped="<EXTREMELY_IMPORTANT>\n${ctx}\n</EXTREMELY_IMPORTANT>"
escaped=$(python3 -c 'import json,sys; print(json.dumps(sys.stdin.read()))' <<<"$wrapped")

cat <<EOF
{
  "hookSpecificOutput": {
    "hookEventName": "${event_name}",
    "additionalContext": ${escaped}
  }
}
EOF

exit 0
