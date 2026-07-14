## Context

Ito already stores the right raw materials for a durable knowledge layer, but those materials are optimized for source truth and workflow execution, not repeated synthesis. Specs express current truth, changes capture intended deltas, research captures investigations, modules capture scope, and agent guidance captures workflow conventions. The missing piece is a maintained intermediate layer that turns those raw artifacts into a browseable, interlinked, cumulative wiki.

The wiki must stay Ito-scoped. It can cite or link to files outside `.ito/` when they matter, but it must not turn into a general-purpose project wiki that mirrors arbitrary source code or repo docs.

## Goals / Non-Goals

- Goals:
  - Create a persistent `.ito/wiki/` root that LLMs can maintain incrementally.
  - Make the wiki useful for proposal creation, research synthesis, archived-change recall, and cross-reference discovery.
  - Prefer topic pages with rich links to specs, modules, archived changes, research, architecture notes, and relevant external documentation.
  - Keep the wiki Obsidian-friendly and plain-markdown-first.
  - Add explicit maintenance, search, and lint workflows so wiki freshness is reviewable and not magical.
  - Seed this repo with a useful initial wiki.
- Non-Goals:
  - Building a full general-repo documentation system.
  - Requiring embeddings, a database, a background daemon, or new CLI subcommands in the first iteration.
  - Replacing specs, proposals, research artifacts, or project guidance as source-of-truth documents.
  - Blocking proposal, research, or archive workflows solely because the wiki is absent or stale.

## Decisions

- Decision: The wiki root lives at `.ito/wiki/`.
  - Why: It keeps the knowledge layer clearly within Ito's domain and makes source boundaries legible.
  - Alternatives considered: `docs/wiki/` or project-root `wiki/`, rejected because they blur the line between Ito workflow knowledge and general project docs.

- Decision: Page authority is case-by-case and explicit.
  - Why: Some pages summarize canonical specs and must defer to source artifacts; other pages record durable decisions, query syntheses, or planning aids that are useful but advisory until promoted to specs or guidance.
  - Consequence: Every non-trivial wiki page needs authority/freshness/source metadata so agents know how to use it.

- Decision: The first implementation is skills/templates-only.
  - Why: The highest-value outcome is better agent behavior and durable markdown, not a new CLI surface. Keeping the first iteration skill-driven reduces implementation risk and avoids locking in premature automation.
  - Alternatives considered: `ito wiki lint` or `ito wiki refresh`, rejected for this change but left as a possible follow-up once the wiki schema proves stable.

- Decision: Freshness behavior is warn-and-update.
  - Why: Stale or missing wiki content should never dead-end planning or archive work. Agents should warn, fall back to raw Ito sources, and update the wiki when the output has durable value.
  - Alternatives considered: blocking archive or planning on wiki refresh, rejected as too disruptive for the first iteration.

- Decision: Archived changes are summarized into topic pages by default.
  - Why: The desired wiki should support graph-style navigation across concepts, specs, decisions, and documentation rather than produce one page per archived change by default.
  - Consequence: Individual archived-change pages are reserved for historically important changes or cases where a topic page would become too large.

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
├── topics/
├── specs/
├── research/
└── queries/
```

The exact page set can evolve, but the reserved root files should remain stable so harnesses have predictable entry points.

## Page Model

Each durable wiki page should include enough metadata for agents to reason about authority and freshness. The schema should define, at minimum:

- Page type: topic, spec-summary, research-synthesis, query-result, workflow-note, or decision-note
- Authority: canonical-summary, advisory-synthesis, decision-record, or query-artifact
- Source references: links to source specs, changes, modules, research, architecture, guidance, or intentionally cited external files
- Freshness: last-reviewed timestamp, source window or source refs reviewed, stale/unknown/fresh status, and known gaps
- Cross-links: explicit wiki links and source links that allow graph traversal

## Risks / Trade-offs

- Scope creep: The wiki could drift into general repo documentation.
  - Mitigation: enforce `.ito`-first source boundaries and describe external files as linked references, not default sources.
- Staleness: If the wiki is not refreshed during normal workflows, it becomes misleading.
  - Mitigation: warn-and-update workflow guidance, source metadata, lint checks, and `_meta/status.md`.
- Authority confusion: Agents might treat wiki summaries as canonical when specs disagree.
  - Mitigation: require page authority metadata and instruct agents to defer to raw Ito artifacts when authority is lower or conflicts appear.
- Duplication: The wiki may duplicate parts of specs or research.
  - Mitigation: require synthesis and cross-linking rather than page-for-page mirroring.
- Installer risk: `.ito/wiki/**` contains LLM-authored mutable content but starts from templates.
  - Mitigation: define scaffold files as seeded/user-owned after creation unless a marker-managed section is explicitly present.

## Migration Plan

1. Add the `.ito/wiki/` scaffold and schema assets.
2. Add wiki maintenance, search, and lint skills.
3. Integrate warn-and-update guidance into proposal, research, and archive workflows.
4. Add tests for scaffold installation, preservation, skill embedding, and instruction output.
5. Seed this repo's `.ito/wiki/` from current specs, modules, research, high-signal archived changes, and architecture guidance.

## Follow-Up Options

- Add CLI helpers such as `ito wiki lint` or `ito wiki refresh` after the markdown schema stabilizes.
- Add richer graph/index generation if the initial manual index becomes hard to maintain.
- Add optional qmd/vector search once wiki page quality and boundaries are proven.
