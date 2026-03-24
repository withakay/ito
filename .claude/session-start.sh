#!/usr/bin/env bash
# SessionStart hook for Claude Code integration.
# Delegates workflow content to the Ito CLI and injects best-effort continuation context.

set -euo pipefail

base_context=$(cat <<'EOF'
<EXTREMELY_IMPORTANT>

Ito workflows are managed by the Ito CLI.

To bootstrap Ito workflows in Claude Code, run:

```bash
ito agent instruction bootstrap --tool claude
```

If you lose which change/module you're working on, run:

```bash
ito agent instruction context
```

</EXTREMELY_IMPORTANT>
EOF
)

ctx="$(ito agent instruction context 2>/dev/null || true)"
if [ -n "${ctx//[[:space:]]/}" ]; then
  base_context="${base_context}\n\n${ctx}"
fi

escaped=$(python3 -c 'import json,sys; print(json.dumps(sys.stdin.read()))' <<<"$base_context")

cat <<EOF
{
  "hookSpecificOutput": {
    "hookEventName": "SessionStart",
    "additionalContext": $escaped
  }
}
EOF

exit 0
