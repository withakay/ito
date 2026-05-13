---
children_hash: bcb2e2e53b423bd3a64fddc2020e93d85307e56d89efc1067bc8eb88189f287f
compression_ratio: 0.2996570825639673
condensation_order: 2
covers: [ito_templates/_index.md, ito_workflow/_index.md, release_workflow/_index.md, source_guides/_index.md]
covers_token_total: 3791
summary_level: d2
token_count: 1136
type: summary
---
# Structural Summary: Ito Knowledge Indexes

## High-level map
These entries describe the main operational knowledge around Ito’s coordination-backed workflows, release guardrails, and source-guide navigation. The collection is organized around three major areas:

- **`ito_workflow/_index.md`** — coordination, sync, validation, config, orchestration, and cleanup behavior
- **`release_workflow/_index.md`** — release pipeline and publishing guardrails
- **`source_guides/_index.md`** — source-guide atlas workflow for apply work
- **`ito_templates/_index.md`** — template bundle marker retrofit across assets

## Core structural themes

### Coordination-backed state and safe mirrors
The **`ito_workflow`** cluster centers on maintaining a writable coordination source of truth while exposing safe read-only mirrors and validation surfaces. It emphasizes:
- generated mirror state, not direct mutable output
- safe symlink repair and sync sequencing
- Git bootstrap and refspec safety
- bounded retry and retention behavior
- machine-readable validation and sync status

Key drill-down entries:
- **`published_ito_mirror.md`** — read-only mirror generation, drift detection, safe path handling
- **`coordination_branch_git_behavior.md`** — branch bootstrap, push refspec shape, Git error classification
- **`coordination_symlink_repair_and_sync.md`** — `.ito/` repair rules, health checks, sync order
- **`audit_mirror_concurrency_and_temp_naming.md`** — collision-safe temp naming, merge dedupe, retry logic
- **`worktree_validation_flow.md`** — validation policy for main/control vs other worktrees
- **`ito_config_gotcha.md`** — global config vs repo-local effective config, embedded vs worktree storage
- **`ito_orchestration_consolidation.md`** — orchestration source-of-truth and agent surface taxonomy
- **`ddd_discovery_workflow.md`** — gated domain discovery and boundary probing
- **`obsolete_specialist_cleanup.md`** — cleanup of renamed specialist assets during update/reinstall

### Release pipeline and repo-state guardrails
The **`release_workflow`** cluster defines how Ito is released and which repository-state constraints must hold during publishing. The release process is split between version publishing and artifact distribution, with explicit guardrails around coverage, manifest rendering, and `.ito` tracking rules.

Key drill-down entries:
- **`release_workflow.md`** — end-to-end release sequence
- **`build_and_coverage_guardrails.md`** — `make check`, coverage toolchain setup, max-lines baseline, `cargo-deny`
- **`release_plz_guardrails.md`** — `release-plz.toml`, dirty-state rules, `.ito` gitignore policy
- **`manifesto_instruction_implementation_notes.md`** — sync-status semantics and instruction rendering rules

### Source-guide atlas for apply work
The **`source_guides`** cluster documents a navigation-first workflow for apply tasks. Source guides are treated as orientation aids and code maps, but not as authoritative sources; important claims must still be verified against source.

Key drill-down entry:
- **`source_guide_workflow.md`** — canonical guide atlas workflow and refresh rules

### Template bundle retrofit
The **`ito_templates`** cluster records the marker retrofit applied across template assets. The main structural distinction is simple: plain markdown files receive ITO markers, while already marked files remain unchanged.

Key drill-down entries:
- **`template_bundle_retrofit.md`** — primary retrofit summary and verification facts
- **`template_bundle_retrofit.abstract.md`** — abstracted structural view
- **`template_bundle_retrofit.overview.md`** — approach overview

## Cross-cutting patterns

- **Safety-first state handling:** bootstrapping, sync, and publishing all avoid accidental state corruption.
- **Coordination source of truth:** writable coordination storage feeds generated outputs and validation surfaces.
- **Explicit config boundaries:** repo-local settings govern behavior, while global config stays separate.
- **Machine-readable operations:** validation and sync outputs are designed for automation and hooks.
- **Guardrails over convenience:** release, mirror, and worktree flows prefer correctness, bounded retries, and clear failure guidance.

## Drill-down guide
- **`ito_templates/_index.md`** — template marker retrofit
- **`ito_workflow/_index.md`** — coordination, sync, validation, config, orchestration
- **`release_workflow/_index.md`** — release pipeline and guardrails
- **`source_guides/_index.md`** — source-guide workflow for apply work