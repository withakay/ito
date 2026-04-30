---
children_hash: d03aa00084a797e16614a579f841856e14672e5450bdc5bb54611a167ba79bc0
compression_ratio: 0.4629141014365289
condensation_order: 2
covers: [ito_templates/_index.md, ito_workflow/_index.md, release_workflow/_index.md, source_guides/_index.md]
covers_token_total: 3411
summary_level: d2
token_count: 1579
type: summary
---
# Structural Summary (d2)

## Ito workflow and supporting process knowledge
The `ito_workflow` and related entries define the operational backbone for Ito: safe mirror publishing, worktree validation, audit mirror synchronization, orchestration consolidation, obsolete specialist cleanup, and DDD discovery behavior. The overall pattern is to keep coordination-backed state authoritative, expose read-only outputs where needed, and enforce guardrails around publishing, discovery, and agent surfaces.

### `ito_workflow/_index.md`
This is the umbrella overview for workflow behavior. It ties together:
- **Published Ito Mirror**: `published_ito_mirror.md` defines the read-only `docs/ito` mirror generated from coordination-backed state. The mirror path is configurable via `changes.published_mirror.path` (default `docs/ito`), path resolution is safety-checked, symlinks are skipped, and the output layout includes `README.md`, `changes/active`, `changes/archive`, and `specs`.
- **Worktree Validation Flow**: `worktree_validation_flow.md` defines `ito worktree validate --change <id> [--json]`. It uses exact change-id prefix matching, hard-fails on main/control checkouts, and provides advisory mismatch guidance on non-main worktrees.
- **Audit Mirror Concurrency and Temp Naming**: `audit_mirror_concurrency_and_temp_naming.md` covers mirror sync internals in `mirror.rs`, including temp worktree/orphan branch naming with `pid` + timestamp + atomic counter, JSONL deduping and ordering, log truncation to 30 days / 1000 events, and one retry on push/ref update conflicts.
- **Ito Orchestration Consolidation**: `ito_orchestration_consolidation.md` centralizes overlapping orchestration and multi-agent skill/prompt guidance under change `028-02_centralize-instruction-source-of-truth`. It distinguishes entrypoint agents (`ito-general`, `ito-orchestrator`) from delegated sub-agents (`planner`, `researcher`, `worker`, `reviewer`, `test-runner`).
- **Obsolete Specialist Cleanup**: `obsolete_specialist_cleanup.md` documents cleanup of renamed orchestrator assets on update/forceful init/reinstall paths, including removal of legacy `ito-orchestrator-*` markdown and `SKILL.md` assets while preserving coordinator assets.
- **DDD Discovery Workflow**: `ddd_discovery_workflow.md` defines evidence-first discovery for `001-34_add-ddd-discovery-workflow`. It combines grill-with-docs ideas, requires repository evidence before user questions, and uses boundary probes, glossary conflict checks, and lazy capture artifacts like `CONTEXT.md`, `CONTEXT-MAP.md`, and ADRs before promotion.

### Structural relationships
- `context.md` is the workflow overview that anchors safe mirror generation and synchronization.
- `published_ito_mirror.md` and `audit_mirror_concurrency_and_temp_naming.md` both govern safe mirror publication/sync.
- `worktree_validation_flow.md` complements mirror safety by validating worktree state before operations.
- `ddd_discovery_workflow.md` connects discovery practice to `ito_orchestration_consolidation.md` and `source_guides/source_guide_workflow.md` through guardrail-aware questioning and consensus discovery.

## Release workflow and guardrails
The `release_workflow` group captures the Ito release pipeline and the constraints that keep releases, coverage, and manifest rendering consistent.

### `release_workflow/_index.md`
The main release flow is:
1. merge a release PR
2. `release-plz` publishes crates and tags `vX.Y.Z`
3. `cargo-dist` builds artifacts and creates GitHub Releases
4. Homebrew formulas are updated in `withakay/homebrew-ito`
5. release notes are polished afterward

Key division:
- `release-plz` handles versioning/publishing
- `cargo-dist` handles artifact builds, GitHub Releases, and Homebrew publishing

Supporting configuration files:
- `.github/workflows/release-plz.yml`
- `.github/workflows/v-release.yml`
- `.github/workflows/polish-release-notes.yml`
- `dist-workspace.toml`
- `release-plz.toml`

Key child entries:
- **`build_and_coverage_guardrails.md`**: `make check` coverage now derives `LLVM_COV` and `LLVM_PROFDATA` from the active `rustup` toolchain when unset; `ito-rs/tools/max_lines_baseline.txt` tracks legacy oversized files; `cargo-deny` allows `wit-bindgen@0.51` as a wasip3 transitive duplicate.
- **`release_plz_guardrails.md`**: `.ito` coordination paths must stay gitignored; tracked ignored files should be removed with `git rm --cached`; `release-plz.toml` remains at repo root; workflows run release-plz on `main` with separate `release` and `release-pr` jobs; `allow_dirty = false`, `publish_allow_dirty = false`, workspace changelog and dependency updates enabled.
- **`manifesto_instruction_implementation_notes.md`**: `synced_at_generation` is only set when coordination sync returns `Synchronized`; `RateLimited` is not fresh success; full `--operation` requires `--change`; unconfigured operations render as `null`.

### Cross-entry relationships
- `release_workflow.md` is the parent release-process overview.
- `build_and_coverage_guardrails.md` and `release_plz_guardrails.md` refine execution and repo-state constraints.
- `manifesto_instruction_implementation_notes.md` defines sync-reporting and instruction-rendering behavior during coordination/release flows.

## Source guide workflow
The `source_guides` entries describe the code-atlas layer used during Ito apply work.

### `source_guides/_index.md`
`source-guide.md` files function as a navigation-oriented code map, not as the source of truth. The workflow is:
1. check for nearby `source-guide.md` files before apply work
2. refresh or create stale/missing guides
3. read guides for orientation
4. verify important claims against source
5. update affected guides after structural changes

Structural pattern:
- root `source-guide.md`
- `ito-rs/source-guide.md`
- `ito-rs/crates/source-guide.md`
- per-crate `source-guide.md`
- freshness tracked in `source-guide.json`

Key rule:
- guides support orientation and context, but source verification is required for important claims.

## Overall drill-down map
- `ito_workflow/` — safe mirror publishing, validation, audit sync, orchestration, discovery
- `release_workflow/` — release pipeline, coverage guardrails, release-plz policy
- `source_guides/` — source-guide atlas and apply-time navigation rules