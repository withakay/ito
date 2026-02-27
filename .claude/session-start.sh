#!/usr/bin/env bash
# Minimal SessionStart hook shim for Claude Code integration
# Points to Ito CLI instruction artifacts instead of embedding workflow content

set -euo pipefail

# Output a minimal pointer to the Ito CLI bootstrap artifact
# This hook does NOT embed workflow content - it delegates to the CLI
additional_context=$(cat <<'EOF'
<EXTREMELY_IMPORTANT>

Ito workflows are managed by the Ito CLI.

To bootstrap Ito workflows in Claude Code, run:

```bash
ito agent instruction bootstrap --tool claude
```

This command returns the canonical preamble and available workflow artifacts.

For a list of available instruction artifacts, run:
```bash
ito agent instruction --list
```
</EXTREMELY_IMPORTANT>
EOF
)

escaped_additional_context=$(python3 -c 'import json, sys; print(json.dumps(sys.stdin.read()))' <<<"$additional_context")

cat <<EOF
{
  "hookSpecificOutput": {
    "hookEventName": "SessionStart",
    "additionalContext": ${escaped_additional_context}
  }
}
EOF

exit 0
