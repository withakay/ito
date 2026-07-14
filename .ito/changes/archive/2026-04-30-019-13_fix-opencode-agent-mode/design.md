<!-- ITO:START -->
## Context

The embedded OpenCode agent templates under `ito-templates/assets/agents/opencode/` do not declare `mode: subagent`, but previously installed `.opencode/agents/*.md` files can still carry that field. The existing update path preserves markerless agent bodies and only rewrites the `model` field in frontmatter, so stale `mode: subagent` metadata survives indefinitely.

## Goals / Non-Goals

- Goals:
  - Remove stale subagent metadata from OpenCode agent frontmatter during refresh.
  - Preserve the existing body-preservation semantics for legacy agent files.
  - Cover both fresh install and update paths with focused tests.
- Non-Goals:
  - Rework agent template bodies.
  - Change other harness formats unless they share the same stale OpenCode-specific field shape later.

## Decision

- Normalize OpenCode agent frontmatter in the existing installer update helper by stripping `mode: subagent` and `subagent:` lines only for `.opencode/agents/*.md` files, while continuing to refresh the model field.

## Verification

- Focused installer unit tests in `ito-core`.
- Focused OpenCode init/update regression tests in `ito-cli/tests/init_more.rs`.
<!-- ITO:END -->
