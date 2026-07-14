# Ito Wiki Overview

```yaml
page_type: topic
authority: advisory-synthesis
freshness: fresh
last_reviewed: 2026-07-14
source_refs:
- .ito/specs/
- .ito/changes/
- .ito/changes/archive/
- .ito/research/SUMMARY.md
known_gaps:
  - This is a first-pass synthesis, not exhaustive spec coverage.
```

Ito is a change-driven workflow tool with a Rust workspace implementation,
template-managed agent surfaces, and Ito-managed proposal/spec/task artifacts.
The repository keeps accepted capability specs in `.ito/specs/`, active and
archived changes in `.ito/changes/`, and research notes under `.ito/research/`.

## Current Shape

- The CLI and core crates manage changes, modules, specs, tasks, validation,
  archive flows, repository runtime selection, backend sync, and worktree-aware
  workflows.
- Template assets install AGENTS guidance, commands, skills, agent prompts,
  schema templates, and workflow instructions for multiple harnesses.
- Proposal and specification authority is tracked directly under `.ito/` on
  main. Worktrees may isolate implementation code, but they do not own Ito
  authority.

## How Future Agents Should Use This Wiki

- Start with the topic page closest to the requested work.
- Treat source refs as the authority trail, then inspect raw specs or changes
  before editing behavior.
- If wiki coverage is stale, say so and update the page after the source work
  is complete.
