<!-- ITO:START -->
# Design: Claude Code Audit Hooks

## Hook Strategy

- Primary hook: Claude Code `PreToolUse`.
- Matcher policy:
  - Always run for tools that can mutate repo state (Bash, Edit, Write).
  - Optionally run for Read/Glob/Grep if desired for "always-on" drift detection.

## Hook Script Contract

- Input: JSON on stdin (hook payload).
- Output:
  - Exit 0 and (optional) JSON on stdout to inject additionalContext.
  - Exit 2 to block tool execution when audit validation is a hard failure.

## Audit Policy

- Run `ito audit validate` on every matched PreToolUse.
- If drift is detected, run `ito audit reconcile` and inject a warning.
- Only auto-fix drift via `ito audit reconcile --fix` if we decide the project policy should be "keep it up to date"; otherwise prefer warn-only.

## Installation

- Install hook configuration into `.claude/settings.json` (marker-managed where possible).
- Install hook script under `.claude/hooks/ito-audit.sh` (or similar) and keep it tiny.

## Testing

- Installer tests:
  - init installs settings + script
  - update refreshes managed blocks
  - local overrides (e.g., `.claude/settings.local.json`) remain untouched
<!-- ITO:END -->
