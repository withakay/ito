## Context

Ito already stores the right raw materials for a durable knowledge layer, but those materials are optimized for source truth and workflow execution, not for repeated synthesis. Specs express current truth, changes capture intended deltas, research captures investigations, and modules capture scope. The missing piece is a maintained intermediate layer that turns those raw artifacts into a browseable, interlinked, cumulative wiki.

The wiki must stay Ito-scoped. It can cite or link to files outside `.ito/` when they matter, but it must not turn into a general-purpose project wiki that mirrors arbitrary source code or repo docs.

## Goals / Non-Goals

- Goals:
  - Create a persistent `.ito/wiki/` root that LLMs can maintain incrementally.
  - Make the wiki useful for proposal creation, research synthesis, and archived-change recall.
  - Keep the wiki Obsidian-friendly and plain-markdown-first.
  - Add explicit maintenance workflows so wiki freshness is reviewable and not magical.
- Non-Goals:
  - Building a full general-repo documentation system.
  - Requiring embeddings, a database, or a background daemon in the first iteration.
  - Replacing specs, proposals, or research artifacts as source-of-truth documents.

## Decisions

- Decision: The wiki root lives at `.ito/wiki/`.
  - Why: It keeps the knowledge layer clearly within Ito's domain and makes source boundaries legible.
  - Alternatives considered: `docs/wiki/` or project-root `wiki/`, rejected because they blur the line between Ito workflow knowledge and general project docs.

- Decision: The initial source boundary is Ito-owned artifacts plus explicit outbound references.
  - Why: The user wants a wiki for planning, research, changes, modules, and specs inside `.ito/`, not a general knowledge mirror of the repo.
  - Alternatives considered: broad repo-wide ingestion, rejected because it would dilute the wiki and raise maintenance cost.

- Decision: The first version is index-first, markdown-first, and skill-driven.
  - Why: The user's desired pattern emphasizes persistent synthesis over raw RAG, and the repo already supports skill/instruction-driven workflows.
  - Alternatives considered: introducing qmd or vector search immediately, rejected as an optional later enhancement.

- Decision: Archive, proposal, and research flows should deliberately touch the wiki.
  - Why: These are the highest-value lifecycle moments for keeping the knowledge layer current and useful.
  - Alternatives considered: standalone manual wiki updates only, rejected because it would make the wiki stale quickly.

## Proposed Wiki Shape

```text
.ito/wiki/
├── index.md
├── log.md
├── overview.md
├── _meta/
│   ├── config.yaml
│   ├── schema.md
│   └── status.md
├── concepts/
├── changes/
├── specs/
├── research/
└── queries/
```

The exact page set can evolve, but the reserved root files should remain stable so harnesses have predictable entry points.

## Risks / Trade-offs

- Scope creep: The wiki could drift into general repo documentation.
  - Mitigation: enforce `.ito`-first source boundaries and describe external files as linked references, not default sources.
- Staleness: If the wiki is not refreshed during normal workflows, it becomes misleading.
  - Mitigation: add explicit archive/proposal/research touchpoints and status tracking.
- Duplication: The wiki may duplicate parts of specs or research.
  - Mitigation: require synthesis and cross-linking rather than page-for-page mirroring.

## Migration Plan

1. Add the `.ito/wiki/` scaffold and schema assets.
2. Add wiki skills and workflow guidance.
3. Integrate archive/proposal/research touchpoints.
4. Optionally do an initial wiki refresh from existing `.ito/` artifacts.

## Open Questions

- Should archived changes receive their own durable wiki pages by default, or should the wiki summarize at the capability/topic level unless a change is historically important?
- Should query artifacts always be filed under `queries/`, or only when the answer has durable value?
- Should a later iteration add CLI helpers for wiki refresh/search, or keep the first version entirely skill/instruction driven?
