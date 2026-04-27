---
name: ito-wiki
description: Maintain and lint the Ito `.ito/wiki/` knowledge layer. Use when setting up, refreshing, repairing, ingesting durable synthesis into, or checking freshness of an Ito wiki.
---

<!-- ITO:START -->

# Ito Wiki Maintenance

Maintain the repo-local `.ito/wiki/` knowledge layer. The wiki is an LLM-maintained synthesis layer over Ito artifacts; it does not replace specs, proposals, research, modules, or project guidance as source truth.

## When To Use

Use this skill when you need to:

- Set up or inspect `.ito/wiki/`
- Refresh stale topic pages after proposal, research, review, or archive work
- Ingest durable synthesis from Ito artifacts into wiki pages
- Repair broken wiki links, missing metadata, or stale status notes
- Lint wiki health before relying on it for planning or research

Do not use this skill to mirror arbitrary repository documentation or source code. External files may be linked as supporting references, but default ingestion sources must stay Ito-owned.

## Source Boundary

Default sources are:

- `.ito/specs/`
- `.ito/changes/`
- `.ito/research/`
- `.ito/modules/`
- `.ito/project.md`
- `.ito/user-prompts/`
- `.ito/AGENTS.md`

Link non-Ito files only when they clarify a decision, workflow, or source reference already anchored in Ito artifacts.

## Maintenance Workflow

1. Find the wiki root with `ito path ito-root`, then inspect `.ito/wiki/index.md` and `.ito/wiki/_meta/status.md` if they exist.
2. If the wiki is missing, create only the scaffold files described by `.ito/wiki/_meta/schema.md` or the project template. Do not invent a large wiki in one pass.
3. Read the relevant raw Ito artifacts for the task before editing any wiki page.
4. Update the most relevant topic page first. Prefer topic synthesis over one page per change.
5. Add or update page metadata: `page_type`, `authority`, `freshness`, `last_reviewed`, `source_refs`, and `known_gaps`.
6. Cite source paths for claims future agents may rely on.
7. Update `index.md` for new important pages, `log.md` for meaningful maintenance events, and `_meta/status.md` for freshness or coverage changes.

## Warn-And-Update Behavior

Stale, missing, or contradictory wiki coverage must not block work.

- Warn briefly that wiki coverage is stale, missing, or conflicting.
- Fall back to raw Ito sources.
- Continue the requested workflow.
- Update the wiki afterward when the result has durable value.

## Lint Checklist

Check these before treating the wiki as useful context:

- Reserved files exist: `index.md`, `overview.md`, `log.md`, `_meta/config.yaml`, `_meta/schema.md`, `_meta/status.md`
- Durable pages have page type, authority, freshness, source refs, and known gaps
- Source refs point primarily to Ito artifacts
- Topic pages synthesize instead of copying whole source files
- `index.md` links to important topic pages
- `log.md` records meaningful maintenance events
- `_meta/status.md` identifies stale areas and next maintenance steps
- Wiki pages defer to canonical specs or guidance when conflicts appear

## Repair Rules

- Do the smallest repair that restores trustworthy navigation or freshness.
- Preserve project-authored wiki content during upgrades and refreshes.
- Do not delete pages unless they are clearly duplicate, empty, or harmful; prefer marking known gaps.
- If a page conflicts with source truth, update the page and cite the source that resolved the conflict.

<!-- ITO:END -->
