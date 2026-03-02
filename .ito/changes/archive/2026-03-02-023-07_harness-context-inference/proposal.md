# Change: Harness context inference + continuation nudges

## Why

When a session compacts (or when we switch harnesses), the agent often loses which Ito change/module it is working on and stalls instead of advancing the next ready task.

We want a small, consistent, cross-harness mechanism to re-establish the current Ito target and nudge the agent to continue immediately.

## What Changes

- Add a new capability spec for `harness-context-inference` that defines a stable contract for inferring the current Ito target (change/module) and generating continuation nudges.
- Add a single Ito CLI entrypoint that:
  - Infers the current target from local signals (cwd/worktree/branch), and
  - Emits a harness-friendly continuation snippet (text + optional JSON).
- Update harness adapters to consume that entrypoint:
  - OpenCode: toast when Ito prompt is injected, toast on worktree detection, and inject continuation context around compaction.
  - Claude Code: inject inferred context at session start and before compaction.
  - GitHub Copilot CLI: add custom instructions that reference the entrypoint; hooks remain best-effort and must not rely on prompt injection.

## Impact

- Affected specs:
  - `harness-context-inference` (new)
  - Potential follow-ups: `tool-adapters` and/or `agent-instructions` if we expand bootstrap artifacts to mention the new entrypoint.
- Affected code:
  - `ito-rs/crates/ito-core/` (inference logic)
  - `ito-rs/crates/ito-cli/` (CLI wiring)
  - Harness wiring: `.opencode/`, `.claude/`, `.github/`
