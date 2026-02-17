<!-- ITO:START -->
# Design: OpenCode Audit Hooks

## Goals

- Run Ito audit checks deterministically before tool execution in OpenCode.
- Keep the OpenCode plugin minimal and treat Ito CLI output as the source of truth.
- Avoid noticeable latency: do not spawn `ito` excessively.

## Hook Strategy

- Primary hook: OpenCode plugin `tool.execute.before`.
- Optional supporting hooks:
  - `experimental.chat.system.transform` to inject a one-time audit reminder/preamble.
  - `tool.execute.after` for logging-only and/or surfacing post-tool failures (non-blocking).

## Audit Policy (Plugin)

- Default action on `tool.execute.before`:
  - Run `ito audit validate`.
  - Run `ito audit reconcile` and, if drift is detected, either:
    - inject warning context and allow, or
    - auto-fix via `ito audit reconcile --fix` (only if configured), then inject context.
- Fail-closed only when `ito audit validate` returns a hard failure (invalid log / corrupted state).

## Performance

- Cache audit results per session for a short TTL (e.g., 5-15 seconds) to avoid running audits on every tool call in tight loops.
- Only re-run audits immediately for tool calls likely to change `.ito/` state (Edit/Write, Bash that touches `.ito/`, etc.).

## Installation / Update

- Ship plugin code under `.opencode/plugins/`.
- Ensure `ito init` installs it when `--tools` includes `opencode`.
- Ensure `ito update` refreshes it without clobbering user-owned config.

## Testing

- Unit tests for plugin behavior belong with the OpenCode adapter source (TypeScript) where feasible.
- Installer/update behavior must be covered in Rust tests:
  - plugin file appears after init
  - plugin file updates managed blocks on update
  - user edits in non-managed files are preserved where promised
<!-- ITO:END -->
