## Context

This change standardizes how harnesses (OpenCode, Claude Code, GitHub Copilot CLI) infer the current Ito target and how they nudge continuation after compaction.

Today, each harness tends to lose the “current change/module” across compaction, and the user must restate the target manually.

## Goals / Non-Goals

- Goals:
  - Provide a stable, testable inference algorithm for “what are we working on?”
  - Centralize inference logic in Ito (thin harness adapters).
  - Make continuation nudges concise and consistent.
- Non-Goals:
  - Perfect inference in all repos (best-effort, deterministic, no network).
  - Changing Ito task semantics (this only points to existing commands).

## Decisions

- Decision: Add an Ito CLI entrypoint that harness adapters can call.
  - Preferred shape: `ito agent instruction context` with `--json` support.
  - Rationale: keeps harness-facing output in the existing `ito agent instruction` namespace and avoids introducing a brand-new top-level command.

- Decision: Support both “agent-visible text” and “machine-readable JSON”.
  - Text: a short context + continuation nudge suitable for injection.
  - JSON: structured fields for harness scripts/plugins.

- Decision: Inference signals (ordered):
  1. Explicit change id or module id present in the working directory path.
  2. Explicit change id or module id present in the git branch name.
  3. Explicit change id present in a provided hint string (for harnesses that can pass it).
  4. Otherwise: no target.

## Risks / Trade-offs

- Risk: false-positive inference (e.g., random numbers in branch names).
  - Mitigation: strict regex for change ids (`NNN-CC_name`) and conservative parsing for module ids (only when embedded in known Ito paths).

- Risk: harness differences in hook/event capabilities.
  - Mitigation: keep harness adapters thin; fall back to “next command” guidance when compaction events cannot be detected.

## Migration Plan

- Phase 1: Land the inference entrypoint + tests.
- Phase 2: Update harness templates/adapters to call it.
- Phase 3: (Optional) enhance bootstrap artifacts to mention the feature.

## Open Questions

- Should the entrypoint accept an explicit `--cwd` override to support harnesses that run hooks outside the repo?
- Should we persist the inferred target in a local state file for higher reliability across sessions?
