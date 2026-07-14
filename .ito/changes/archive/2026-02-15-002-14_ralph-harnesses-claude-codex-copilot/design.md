<!-- ITO:START -->
## Context

Ralph runs an iterative loop by building a prompt and invoking a harness (external agent runtime). The harness contract is `ito_core::harness::Harness` which returns stdout/stderr, an exit code, and a duration. Ralph detects completion by scanning stdout for `<promise>...`.

This change adds harness implementations that spawn well-known CLIs:

- Claude Code: `claude`
- OpenAI Codex: `codex`
- GitHub Copilot CLI: `copilot`

## Goals / Non-Goals

**Goals:**

- Add `claude`, `codex`, and `github-copilot` as selectable `ito ralph --harness` values.
- Implement harnesses by spawning the corresponding CLI with a non-interactive, single-run mode.
- Support `--model` and `--allow-all` consistently across the harnesses.
- Keep all tests offline and deterministic.

**Non-Goals:**

- Building a first-party API client for Anthropic/OpenAI/GitHub.
- Implementing a fully interactive TUI/PTY bridge for harnesses.
- Guaranteeing identical output formatting across harnesses.

## Decisions

### Decision: Use each CLI's non-interactive mode per iteration

Each Ralph iteration is a single harness invocation; the harness MUST exit so Ralph can validate, commit, and decide whether to continue.

Chosen invocations:

- Claude Code: `claude -p <prompt>` (print mode), with `--model <model>` when set.
- Codex: `codex exec <prompt>` (stable non-interactive mode), with `--model <model>` when set.
- Copilot: `copilot -p <prompt>` (non-interactive mode), with `--model <model>` when set.

For the harness selector, `github-copilot` is the canonical name (matching Ito tool/config identifiers); `copilot` is supported as a user-friendly alias.

Rationale:

- All three tools document a non-interactive prompt mode intended for scripting/automation.
- The harness trait already captures stdout/stderr and exit code, which maps cleanly to these CLIs.

### Decision: Map Ito `--allow-all` to each tool's yolo/permission bypass

Ralph already has `--allow-all` (dangerous). For these harnesses:

- Claude Code: pass `--dangerously-skip-permissions`.
- Codex: pass `--yolo`.
- Copilot: pass `--yolo` (equivalent to allowing all permissions).

When `--allow-all` is NOT set, the harnesses run in their defaults (which may prompt or restrict tools depending on local configuration).

### Decision: Count working-tree changes for all non-stub harnesses

Ralph currently counts git changes only for the OpenCode harness. Claude/Codex/Copilot can also edit the working tree, so change counting should not be harness-specific. The count will remain best-effort and default to 0 when the repo cannot be inspected.

## Risks / Trade-offs

- **Local CLI variance**: these CLIs evolve quickly; flags may change. Mitigation: keep the harness modules small, add unit tests for argument construction, and provide clear error messages when the binary is missing.
- **Permission prompting**: in non-`--allow-all` mode, some CLIs may block on approvals. Mitigation: document that `--allow-all` is the supported non-interactive automation mode.
- **Output structure**: completion promise detection relies on stdout text. Mitigation: keep the tools in text output modes and continue to detect completion only from stdout.
<!-- ITO:END -->
