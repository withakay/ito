# Distribution And Agents

```yaml
page_type: topic
authority: advisory-synthesis
freshness: fresh
last_reviewed: 2026-07-13
source_refs:
  - docs/ito/specs/agent-instructions/spec.md
  - docs/ito/specs/agent-surface-taxonomy/spec.md
  - docs/ito/specs/instruction-source-of-truth/spec.md
  - docs/ito/specs/template-assets/spec.md
  - docs/ito/specs/distribution/spec.md
  - docs/ito/specs/ito-skill-routing/spec.md
known_gaps: []
```

Ito ships managed instructions, prompts, commands, skills, and agent templates
for multiple harnesses. Template assets are the source for generated harness
surfaces; generated files should stay thin and refreshable through Ito update
flows.

## Current Pattern

- Shared skill and command assets live under `ito-templates` and are embedded
  into the Rust binary.
- The default Ito-managed inventory is exactly seven lifecycle skills:
  `ito`, `ito-proposal`, `ito-research`, `ito-apply`, `ito-review`,
  `ito-archive`, and `ito-loop`.
- Harness-specific output paths differ, but behavior should stay driven by
  shared templates and skill instructions where possible.
- Agent instructions should route to Ito CLI instruction artifacts rather than
  duplicating long policy text in every harness surface.
- Native agent definitions remain separate for OpenCode, Claude, GitHub
  Copilot, and Pi. Codex does not receive role definitions synthesized as
  discoverable skills.

## Knowledge Guidance

Wiki search/navigation belongs to `ito-research`; durable wiki maintenance and
archive follow-through belong to `ito-archive`. These remain Ito-scoped and do
not add separate discoverable skills.
