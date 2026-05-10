<!-- ITO:START -->
## Why

Ito already accumulates high-value knowledge inside `.ito/`: accepted specs, active and archived changes, research artifacts, module definitions, project guidance, architecture notes, and workflow decisions. Today that knowledge is fragmented across raw markdown files, so every planning or research session asks an LLM to rediscover and re-synthesize the same context from scratch. That loses continuity, hides contradictions, and makes deep proposal work more expensive than it needs to be.

We want a persistent `.ito/wiki/` layer that is owned and maintained by the LLM harness. The wiki should sit between raw Ito artifacts and future conversations: it should synthesize important decisions once, keep cross-references discoverable, and make planning/research/archive work faster without replacing specs, proposals, or research artifacts as source truth.

## What Changes

- Add a new `.ito/wiki/` knowledge layer with a documented schema, index, log, status, and overview files for Obsidian-friendly browsing and LLM maintenance.
- Define page authority case-by-case: some wiki pages are synthesized summaries of authoritative Ito artifacts, while other pages may record durable decisions or query outputs that are advisory until promoted into specs or project guidance.
- Define wiki source boundaries so default sources remain Ito-owned artifacts (`changes`, `specs`, `research`, `modules`, project guidance, architecture), with explicit links to outside files when useful, without becoming a general repository wiki.
- Add skills/templates-only maintenance workflows for refresh, ingest, query, search, and lint. No CLI subcommands, database, daemon, or embedding dependency are part of this first iteration.
- Add installable Ito wiki skills for searching, maintaining, and linting the wiki with cited answers and freshness checks.
- Integrate warn-and-update guidance into proposal, research, and archive workflows: stale or absent wiki content should not block work, but agents should warn, fall back to raw Ito sources, and update the wiki when useful.
- Seed this repository with useful topic-oriented wiki pages that link to specs, modules, research, archived changes, and other relevant documentation so future sessions can navigate a graph of Ito knowledge.

## Capabilities

### New Capabilities

- `ito-wiki`: Persistent `.ito/wiki/` structure, schema, boundaries, authority metadata, navigation artifacts, and graph-friendly topic pages.
- `ito-wiki-maintenance`: Incremental wiki refresh, focused ingest, query/file-back, source-link tracking, lint/freshness workflows, and warn-and-update behavior over Ito artifacts.
- `ito-wiki-skill`: Installable skills for searching, maintaining, and linting the Ito wiki with cited answers and explicit write boundaries.
- `ito-wiki-workflow-integration`: Guidance and workflow touchpoints that connect proposal, research, and archive flows to the wiki without making wiki freshness a hard blocker.

### Modified Capabilities

- _(none - introduce the wiki as additive workflow infrastructure first, then layer narrower modifications later if needed)_

## Impact

- **Installed project artifacts**: New `.ito/wiki/` scaffold and schema/config assets in the default project template
- **Skills**: New wiki-oriented skills added to shared skill assets and harness installs
- **Instructions**: Proposal, research, and archive-facing instructions/guidance gain wiki consultation, warning, and update steps
- **Initial content**: This repo receives a useful first `.ito/wiki/` seeded from current Ito specs, modules, research, high-signal archived changes, and architecture guidance
- **Scope guardrail**: Wiki content stays centered on Ito artifacts; non-Ito files remain explicitly linked references, not default ingestion sources
- **Upgrade safety**: Template installation and upgrades must preserve existing wiki content and only install missing scaffold files or marker-managed seed sections where explicitly documented
- **Risk**: Medium - additive, but cross-cutting across templates, skills, instructions, and workflow habits. The main risks are stale synthesis, accidental scope creep into a general project wiki, and agents treating wiki summaries as more authoritative than specs without checking page authority metadata.

## Success Criteria

- A new Ito project receives a valid `.ito/wiki/` scaffold without overwriting existing wiki content on upgrade.
- The wiki schema defines page types, page authority, source references, freshness metadata, cross-linking conventions, and lint expectations.
- Wiki search produces cited, low-noise answers from wiki pages first, then falls back to raw Ito artifacts when wiki coverage is missing or stale.
- Proposal, research, and archive workflows use warn-and-update behavior: they warn on stale/absent wiki data, continue from raw sources, and update the wiki when the output has durable value.
- The repo-local initial wiki contains useful topic pages with links to specs, modules, changes, research, and relevant documentation.
<!-- ITO:END -->
