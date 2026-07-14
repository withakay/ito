# Ito Wiki Schema

The Ito wiki is a durable synthesis layer for future proposal, research,
review, and archive work in this repository. It summarizes Ito artifacts and
links back to source truth; it does not replace accepted specs, change
proposals, task files, or project guidance.

## Source Boundary

Default sources are Ito-owned artifacts:

- `.ito/specs/`
- `.ito/changes/`
- `.ito/research/`
- `.ito/modules/`
- `.ito/project.md`
- `.ito/user-prompts/`
- `.ito/AGENTS.md`

External files can be linked when they clarify an Ito decision, but the wiki
should not mirror source code or broad project documentation by default.

## Page Metadata

Durable topic pages should include:

```yaml
page_type: topic
authority: advisory-synthesis
freshness: fresh
last_reviewed: YYYY-MM-DD
source_refs:
- .ito/specs/example/spec.md
known_gaps: []
```

## Maintenance Rules

- Prefer topic pages over one page per archived change.
- Cite raw Ito sources for claims that future agents may rely on.
- Warn and fall back to raw artifacts when coverage is missing, stale, or contradictory.
- Update `index.md`, `log.md`, and `_meta/status.md` after meaningful maintenance.
- Preserve existing wiki content during normal upgrades unless a marker-managed section explicitly says otherwise.
