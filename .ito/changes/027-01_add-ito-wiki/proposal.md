<!-- ITO:START -->
## Why

Ito already accumulates a large amount of high-value knowledge inside `.ito/`: accepted specs, active and archived changes, research artifacts, module definitions, project guidance, and architectural decisions. Today that knowledge remains fragmented across raw markdown files, so every planning or research session asks an LLM to rediscover and re-synthesize the same context from scratch. That loses continuity, hides contradictions, and makes deep proposal work more expensive than it needs to be.

We want a persistent `.ito/wiki/` layer that is owned and maintained by the LLM harness. The wiki should sit between raw Ito artifacts and future conversations: it should synthesize the important parts once, keep them current as changes are archived or research evolves, and remain explicitly scoped to Ito documentation rather than turning into a general project wiki.

## What Changes

- Add a new `.ito/wiki/` knowledge layer with a documented schema, index, log, status, and overview files for Obsidian-friendly browsing and LLM maintenance.
- Define wiki source boundaries so the wiki is built from Ito-owned artifacts (`changes`, `specs`, `research`, `modules`, project guidance, architecture) plus explicit links to outside sources when useful, without becoming a general repository wiki.
- Add maintenance workflows for refresh, ingest, query, and lint so the wiki can be incrementally updated instead of regenerated from scratch.
- Add installable Ito wiki skills for searching the wiki and maintaining it.
- Integrate the wiki into Ito workflows so proposal and research work can consult it, and archive flows can explicitly refresh it after a change is archived or synced.

## Capabilities

### New Capabilities

- `ito-wiki`: Persistent `.ito/wiki/` structure, schema, boundaries, and navigation artifacts.
- `ito-wiki-maintenance`: Incremental wiki refresh, focused ingest, query/file-back, and lint workflows over Ito artifacts.
- `ito-wiki-skill`: Installable skills for searching and maintaining the Ito wiki.
- `ito-wiki-workflow-integration`: Guidance and workflow touchpoints that connect proposal, research, and archive flows to the wiki.

### Modified Capabilities

- _(none - introduce the wiki as additive workflow infrastructure first, then layer narrower modifications later if needed)_

## Impact

- **Installed project artifacts**: New `.ito/wiki/` scaffold and schema/config assets in the default project template
- **Skills**: New wiki-oriented skills added to shared skill assets and harness installs
- **Instructions**: Proposal, research, and archive-facing instructions/guidance will gain wiki consultation or refresh steps
- **Scope guardrail**: Wiki content stays centered on Ito artifacts; non-Ito files remain source references only when intentionally linked
- **Risk**: Medium - additive, but cross-cutting across templates, instructions, and workflow habits. The main risk is accidental scope creep into a general project wiki, which this proposal explicitly forbids.
<!-- ITO:END -->
