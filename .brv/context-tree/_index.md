---
children_hash: d5083a18fd90ccd86778ea57445989b3159fecd724713de838aeb5cdd185e1af
compression_ratio: 0.9085439229843562
condensation_order: 3
covers: [development/_index.md]
covers_token_total: 1662
summary_level: d3
token_count: 1510
type: summary
---
# Development Workspace Structural Summary

The `development` domain is the umbrella for Ito operational knowledge: workflow safety, release guardrails, and source-navigation conventions. The main pattern across its child entries is **authoritative coordination state first, read-only or verified outputs second, and explicit guardrails around publication, release, and discovery**.

## `ito_workflow/_index.md` — operational workflow backbone
Covers the core Ito lifecycle for publishing, validating, syncing, consolidating orchestration, cleaning obsolete assets, and conducting discovery.

- **`published_ito_mirror.md`**: defines the read-only `docs/ito` mirror generated from coordination-backed state.
  - Mirror location is configurable via `changes.published_mirror.path` (default `docs/ito`).
  - Path resolution is safety-checked; symlinks are skipped.
  - Output layout includes `README.md`, `changes/active`, `changes/archive`, and `specs`.
- **`worktree_validation_flow.md`**: defines `ito worktree validate --change <id> [--json]`.
  - Uses exact change-id prefix matching.
  - Hard-fails on main/control checkouts.
  - Provides advisory mismatch guidance on non-main worktrees.
- **`audit_mirror_concurrency_and_temp_naming.md`**: covers mirror sync internals in `mirror.rs`.
  - Temp worktree/orphan branch names use `pid` + timestamp + atomic counter.
  - JSONL events are deduped and ordered.
  - Logs are truncated to 30 days / 1000 events.
  - Push/update conflicts get one retry.
- **`ito_orchestration_consolidation.md`**: centralizes overlapping orchestration and multi-agent skill/prompt guidance under change `028-02_centralize-instruction-source-of-truth`.
  - Separates entrypoint agents: `ito-general`, `ito-orchestrator`.
  - Separates delegated sub-agents: `planner`, `researcher`, `worker`, `reviewer`, `test-runner`.
- **`obsolete_specialist_cleanup.md`**: documents cleanup of renamed orchestrator assets during update/forceful init/reinstall paths.
  - Removes legacy `ito-orchestrator-*` markdown and `SKILL.md` assets.
  - Preserves coordinator assets.
- **`ddd_discovery_workflow.md`**: defines evidence-first discovery for `001-34_add-ddd-discovery-workflow`.
  - Combines grill-with-docs ideas.
  - Requires repository evidence before user questions.
  - Uses boundary probes, glossary conflict checks, and lazy capture artifacts like `CONTEXT.md`, `CONTEXT-MAP.md`, and ADRs before promotion.

### Key relationships
- `context.md` anchors safe mirror generation and synchronization.
- `published_ito_mirror.md` and `audit_mirror_concurrency_and_temp_naming.md` both govern mirror publication/sync safety.
- `worktree_validation_flow.md` complements mirror safety by validating worktree state before operations.
- `ddd_discovery_workflow.md` links discovery practice to `ito_orchestration_consolidation.md` and `source_guides/source_guide_workflow.md` through guardrail-aware questioning.

## `release_workflow/_index.md` — release pipeline and guardrails
Captures the Ito release process and the constraints that keep publishing, coverage, and manifest rendering consistent.

### Main release flow
1. Merge a release PR
2. `release-plz` publishes crates and tags `vX.Y.Z`
3. `cargo-dist` builds artifacts and creates GitHub Releases
4. Homebrew formulas are updated in `withakay/homebrew-ito`
5. Release notes are polished afterward

### Division of responsibilities
- `release-plz` handles versioning and publishing.
- `cargo-dist` handles artifact builds, GitHub Releases, and Homebrew publishing.

### Supporting config files
- `.github/workflows/release-plz.yml`
- `.github/workflows/v-release.yml`
- `.github/workflows/polish-release-notes.yml`
- `dist-workspace.toml`
- `release-plz.toml`

### Child entries
- **`build_and_coverage_guardrails.md`**
  - `make check` coverage derives `LLVM_COV` and `LLVM_PROFDATA` from the active `rustup` toolchain when unset.
  - `ito-rs/tools/max_lines_baseline.txt` tracks legacy oversized files.
  - `cargo-deny` allows `wit-bindgen@0.51` as a wasip3 transitive duplicate.
- **`release_plz_guardrails.md`**
  - `.ito` coordination paths must stay gitignored.
  - Tracked ignored files should be removed with `git rm --cached`.
  - `release-plz.toml` remains at repo root.
  - Workflows run release-plz on `main` with separate `release` and `release-pr` jobs.
  - `allow_dirty = false`, `publish_allow_dirty = false`, workspace changelog and dependency updates are enabled.
- **`manifesto_instruction_implementation_notes.md`**
  - `synced_at_generation` is only set when coordination sync returns `Synchronized`.
  - `RateLimited` is not fresh success.
  - Full `--operation` requires `--change`.
  - Unconfigured operations render as `null`.

### Cross-entry relationships
- `release_workflow.md` is the parent process overview.
- `build_and_coverage_guardrails.md` and `release_plz_guardrails.md` refine execution and repository-state constraints.
- `manifesto_instruction_implementation_notes.md` defines sync-reporting and instruction-rendering behavior during coordination/release flows.

## `source_guides/_index.md` — source atlas for apply-time navigation
Describes `source-guide.md` files as a navigation-oriented code map, not the source of truth.

### Workflow
1. Check for nearby `source-guide.md` files before apply work
2. Refresh or create stale/missing guides
3. Read guides for orientation
4. Verify important claims against source
5. Update affected guides after structural changes

### Structural pattern
- Root `source-guide.md`
- `ito-rs/source-guide.md`
- `ito-rs/crates/source-guide.md`
- Per-crate `source-guide.md`
- Freshness tracked in `source-guide.json`

### Key rule
- Guides support orientation and context, but important claims must still be source-verified.

## Drill-down map
- `ito_workflow/` — safe mirror publishing, validation, audit sync, orchestration, discovery
- `release_workflow/` — release pipeline, coverage guardrails, release-plz policy
- `source_guides/` — source-guide atlas and apply-time navigation rules