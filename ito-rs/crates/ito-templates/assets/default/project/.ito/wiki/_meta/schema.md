<!-- ITO:START -->

# Ito Wiki Schema

The Ito wiki is a durable, LLM-maintained synthesis layer over Ito artifacts. It is not the source of truth for requirements, accepted behavior, or workflow policy unless a page explicitly says so and links to the governing artifact.

## Source Boundary

Default ingestion sources are Ito-owned artifacts:

- `.ito/specs/`
- `.ito/changes/`
- `.ito/research/`
- `.ito/modules/`
- `.ito/project.md`
- `.ito/user-prompts/`
- `.ito/AGENTS.md`

External project files may be linked as supporting references when they explain an Ito decision. Do not mirror source code, generated docs, or general project documentation into this wiki by default.

## Page Types

- `topic` - synthesis across multiple Ito artifacts around one concept
- `spec-summary` - summary of accepted specs that defers to `.ito/specs/`
- `research-synthesis` - durable synthesis from research artifacts
- `query-result` - cited answer worth retaining beyond one chat
- `workflow-note` - workflow guidance derived from Ito instructions or project guidance
- `decision-note` - durable decision record with source links and tradeoffs

## Authority Levels

- `canonical-summary` - summarizes canonical sources but defers to those sources on conflict
- `advisory-synthesis` - useful interpretation or planning aid, not source truth
- `decision-record` - records a durable decision and should identify where it is enforced
- `query-artifact` - preserved answer with citations and bounded freshness

## Expected Metadata

Every durable page should include, near the top:

```yaml
page_type: topic
authority: advisory-synthesis
freshness: unknown
last_reviewed: YYYY-MM-DD
source_refs:
  - .ito/specs/example/spec.md
known_gaps:
  - Not reviewed yet
```

## Maintenance Rules

- Prefer topic pages over one page per archived change.
- Cite raw Ito sources for claims that future agents may rely on.
- Warn and fall back to raw artifacts when wiki coverage is absent, stale, or contradictory.
- Update `index.md`, `log.md`, and `_meta/status.md` after meaningful maintenance.
- Preserve existing wiki content during normal upgrades unless a marker-managed section explicitly says otherwise.

<!-- ITO:END -->
