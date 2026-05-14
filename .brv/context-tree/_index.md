---
children_hash: 1f54e5ae938b0881c2c4044f9fe44acfbd3135a3c4a75649fd50d99fd663f0b1
compression_ratio: 0.8105004101722724
condensation_order: 3
covers: [development/_index.md]
covers_token_total: 1219
summary_level: d3
token_count: 988
type: summary
---
# Structural Summary: Development Knowledge Index

## Scope
This level groups the main operational knowledge for Ito: coordination-backed workflows, release guardrails, source-guide navigation, and template retrofit work. The child indexes under `development/_index.md` define the project’s structural knowledge map.

## Major branches

### `ito_workflow/_index.md`
Core operational knowledge for coordination, sync, validation, config, orchestration, and cleanup. This branch focuses on keeping a writable coordination source of truth while exposing safe generated or read-only surfaces.

Key themes:
- Generated mirror state rather than direct mutable output
- Safe `.ito/` symlink repair and sync sequencing
- Git bootstrap, refspec safety, and error classification
- Bounded retry, retention, and validation behavior
- Machine-readable sync and health reporting

Drill down into:
- `published_ito_mirror.md` — read-only mirror generation, drift detection, safe path handling
- `coordination_branch_git_behavior.md` — branch bootstrap, push refspec shape, Git error classification
- `coordination_symlink_repair_and_sync.md` — `.ito/` repair rules, health checks, sync order
- `audit_mirror_concurrency_and_temp_naming.md` — collision-safe temp naming, merge dedupe, retry logic
- `worktree_validation_flow.md` — validation policy for main/control vs other worktrees
- `ito_config_gotcha.md` — global vs repo-local config, embedded vs worktree storage
- `ito_orchestration_consolidation.md` — orchestration source of truth and agent surface taxonomy
- `ddd_discovery_workflow.md` — gated domain discovery and boundary probing
- `obsolete_specialist_cleanup.md` — cleanup of renamed specialist assets during update/reinstall

### `release_workflow/_index.md`
Release pipeline knowledge and repository-state guardrails for publishing Ito. This branch splits version publishing from artifact distribution and emphasizes controlled release conditions.

Key themes:
- End-to-end release sequencing
- Coverage and toolchain guardrails
- Dirty-state and `.ito` tracking rules
- Manifest rendering and sync-status semantics

Drill down into:
- `release_workflow.md` — end-to-end release sequence
- `build_and_coverage_guardrails.md` — `make check`, coverage setup, max-lines baseline, `cargo-deny`
- `release_plz_guardrails.md` — `release-plz.toml`, dirty-state rules, `.ito` gitignore policy
- `manifesto_instruction_implementation_notes.md` — sync-status semantics and instruction rendering rules

### `source_guides/_index.md`
Navigation-first workflow for apply tasks. Source guides are treated as orientation and code maps, but not authoritative sources; claims must still be verified against source.

Drill down into:
- `source_guide_workflow.md` — canonical guide atlas workflow and refresh rules

### `ito_templates/_index.md`
Template retrofit knowledge across template assets. The main distinction is between plain markdown files that receive ITO markers and already marked files that remain unchanged.

Drill down into:
- `template_bundle_retrofit.md` — primary retrofit summary and verification facts
- `template_bundle_retrofit.abstract.md` — abstracted structural view
- `template_bundle_retrofit.overview.md` — approach overview

## Cross-cutting structural patterns
- **Safety-first state handling** across bootstrap, sync, and publishing
- **Coordination source of truth** feeding generated outputs and validation surfaces
- **Explicit config boundaries** between repo-local and global settings
- **Machine-readable operations** for validation, sync, and hook integration
- **Guardrails over convenience** with bounded retries and clear failure modes

## Drill-down map
- `ito_templates/_index.md` — template marker retrofit
- `ito_workflow/_index.md` — coordination, sync, validation, config, orchestration
- `release_workflow/_index.md` — release pipeline and guardrails
- `source_guides/_index.md` — source-guide workflow for apply work
