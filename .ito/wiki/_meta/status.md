# Ito Wiki Status

```yaml
page_type: workflow-note
authority: advisory-synthesis
freshness: fresh
last_reviewed: 2026-07-14
source_refs:
- .ito/specs/
- .ito/changes/
- .ito/changes/archive/
- .ito/research/
known_gaps:
- Module summaries are not yet seeded.
  - Spec coverage is grouped by theme rather than exhaustively summarized one spec per page.
```

## Coverage

- Workflow and change lifecycle: seeded in `topics/workflow.md`
- Runtime, storage, and backend model: seeded in `topics/runtime-and-storage.md`
- Agent distribution and instruction surfaces: seeded in `topics/distribution-and-agents.md`
- Validation and quality gates: seeded in `topics/validation-quality-gates.md`
- Rust port research: seeded in `research/rust-port.md`

## Next Maintenance Step

Expand topic pages when a proposal touches one of the linked specs or archives.
If a page is stale, update the page and append a short entry to `log.md`.
