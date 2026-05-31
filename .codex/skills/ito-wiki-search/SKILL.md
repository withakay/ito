---
name: ito-wiki-search
description: Search and answer from the Ito `.ito/wiki/` layer first, with cited fallbacks to raw Ito artifacts when wiki coverage is missing, stale, or contradictory.
---

<!-- ITO:START -->
<!--ITO:VERSION:0.1.32-->

# Ito Wiki Search

Search the `.ito/wiki/` knowledge layer before re-synthesizing raw Ito artifacts. Use cited answers and make freshness explicit.

## When To Use

Use this skill when the user asks about:

- Existing Ito decisions, specs, modules, proposals, research, or archive history
- Project workflow context stored under `.ito/`
- Prior planning or research synthesis that may already be captured in the wiki
- Finding topic pages, source references, or known gaps in Ito knowledge

Do not use this as a general repository search skill. The wiki is Ito-scoped.

## Search Workflow

1. Locate the Ito root with `ito path ito-root` when needed.
2. Open `.ito/wiki/index.md` first if it exists.
3. Check `.ito/wiki/_meta/status.md` for freshness and known gaps.
4. Search likely topic, spec, research, and query pages under `.ito/wiki/`.
5. Read cited `source_refs` before making claims that depend on current truth.
6. If wiki coverage is missing, stale, or contradictory, warn briefly and fall back to raw Ito artifacts such as `.ito/specs/`, `.ito/changes/`, `.ito/research/`, `.ito/modules/`, `.ito/project.md`, `.ito/user-prompts/`, and `.ito/AGENTS.md`.
7. Answer with citations to wiki pages and, when needed, raw source artifacts.

## Answer Rules

- Prefer concise answers backed by file paths.
- State whether the answer came from fresh wiki coverage, stale wiki coverage, or raw-source fallback.
- Distinguish canonical summaries from advisory synthesis using page authority metadata.
- Defer to accepted specs and project guidance when a wiki page conflicts with source artifacts.
- Do not create durable wiki content for routine chat answers.

## Durable Artifact Rule

Only update the wiki when the search produces durable value, such as:

- A missing topic that future agents are likely to need
- A stale page whose source refs were revalidated
- A contradiction resolved against canonical Ito sources
- A cited query answer worth preserving under `.ito/wiki/queries/`

When updating, use the `ito-wiki` maintenance workflow and update `log.md` and `_meta/status.md` as needed.

## Fallback Message Pattern

Use a short warning when falling back:

> Wiki coverage is missing/stale for this topic, so I am checking raw Ito artifacts and will update the wiki if the result is durable.

<!-- ITO:END -->
