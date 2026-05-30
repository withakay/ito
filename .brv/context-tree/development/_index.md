---
children_hash: dc68d69879efd2dbb7b8c0ebd26f58417d3e3b9c80dab59b401e31435033382b
compression_ratio: 0.4002469135802469
condensation_order: 2
covers: [context.md, ito_templates/_index.md, ito_workflow/_index.md, release_workflow/_index.md, source_guides/_index.md]
covers_token_total: 4050
summary_level: d2
token_count: 1621
type: summary
---
# development

## Scope
The development domain centers on Ito's release and installer workflows, with a strong focus on safe distribution, coordination-backed state, and apply-time source navigation. It includes release artifact naming and verification, installer platform flows, workflow guardrails, source-guide usage for apply work, and pre-push quality gates.

## Major areas

### release_workflow
Covers the end-to-end release and installer pipeline. The key chain is: merge release PR -> `release-plz` publishes crates and tags `vX.Y.Z` -> `cargo-dist` builds GitHub Releases -> Homebrew formula updates are pushed to `withakay/homebrew-ito` -> release notes are polished.

- **release_workflow.md**: Core pipeline, release automation files, and the rule not to set `git_only = true` in `release-plz.toml`.
- **installer_release_assets.md**: Installer scripts for Unix and Windows; current releases prefer `cargo-dist` assets, with legacy version-pinned archives as fallback. SHA-256 verification is required before extraction, and the built `ito` binary is copied into the install directory.
- **build_and_coverage_guardrails.md**: Build and verification guardrails, including rustup-derived `LLVM_COV`/`LLVM_PROFDATA` when unset, baseline-based max-lines enforcement, and the narrow `wit-bindgen@0.51` deny exception.
- **manifesto_instruction_implementation_notes.md**: Sync-status rendering rules for manifesto generation; `synced_at_generation` only appears when coordination sync returns `Synchronized`, while `RateLimited` is not treated as fresh success.
- **release_plz_guardrails.md**: Release-plz config and ignore rules for coordination paths; keep `.ito/changes`, `.ito/specs`, `.ito/modules`, `.ito/workflows`, and `.ito/audit` gitignored, untrack any accidentally tracked files under `.ito/changes`, and do not enable `git_only = true`.
- **context.md**: Umbrella summary of release asset naming, fallback download behavior, checksum validation, platform-specific install steps, and Windows PATH handling.

### ito_workflow
Captures the coordination-backed workflow that powers publishing, syncing, validation, bootstrap behavior, orchestration, cleanup, and pre-push review gates. The shared pattern is writable coordination state with read-only mirrors and careful handling of Git, symlinks, config, worktree state, and quality gates.

- **published_ito_mirror.md**: Read-only `docs/ito` mirror generated from coordination-backed state; uses safe path resolution, skips symlinks, detects drift, and replaces stale output.
- **coordination_branch_git_behavior.md**: Coordination branch bootstrap and reservation rules; missing remote branches are created from an empty tree root commit, not from HEAD, and Git errors are classified into explicit categories.
- **coordination_symlink_repair_and_sync.md**: `.ito/` symlink wiring, repair, health checks, and sync sequencing; safe states include missing or broken-target links, while wrong targets and non-empty duplicate directories are unsafe.
- **audit_mirror_concurrency_and_temp_naming.md**: Collision-safe temp naming for audit mirrors, bounded log handling, deduped JSONL merges, and limited retry behavior for push/ref-update conflicts.
- **worktree_validation_flow.md**: Read-only validation command for change work; main/control checkouts hard-fail, while non-main mismatches are advisory and machine-readable.
- **ito_config_gotcha.md**: Distinguishes global CLI config from repo-local effective config; normal repo worktrees use `storage=worktree`, while the coordination worktree must use `storage=embedded`.
- **ito_orchestration_consolidation.md**: Consolidates orchestration behavior into the central `ito agent instruction orchestrate` path and introduces `agent-surface-taxonomy` to separate direct entrypoints from delegated sub-agents.
- **ddd_discovery_workflow.md**: Domain-discovery gating and domain-grill question strategy; the workflow distinguishes business/domain capability from Ito capability and only escalates rigorous questioning for high-impact ambiguity or opt-in.
- **obsolete_specialist_cleanup.md**: Installer cleanup for renamed `ito-orchestrator-*` specialist assets during update and forceful init/reinstall flows, while preserving coordinator assets.
- **pre_push_adversarial_code_review.md**: Pre-push quality gate requiring adversarial review before pushing a change branch or opening a PR; blocks P0/P1 issues while allowing minor low-risk deferrals.

### source_guides
Defines the source-guide atlas workflow used during Ito apply work to keep nearby guides current and to verify claims against source.

- **source_guide_workflow.md**: Canonical description of the guide hierarchy (`source-guide.md` at root, repo, crate, and per-crate levels), freshness tracking via `source-guide.json`, and the rule that guides are orientation aids rather than authoritative source.

## Cross-cutting patterns
- **Coordination-backed source of truth:** writable state stays in coordination storage; published artifacts are generated mirrors.
- **Release safety:** release-plz and installer flows avoid unsafe git state, stale artifacts, and unchecked downloads.
- **Verification-first behavior:** checksums, worktree validation, source-guide verification, and adversarial review prioritize source truth.
- **Config boundary separation:** global CLI config and repo-local coordination config are intentionally distinct.
- **Guarded automation:** sync, bootstrap, publish, and review flows classify failures explicitly and avoid accidental destructive operations.

## Drill-down map
- **release_workflow.md** - full release pipeline
- **installer_release_assets.md** - installer scripts, targets, fallback archives, checksum flow
- **build_and_coverage_guardrails.md** - build/coverage and static guardrails
- **manifesto_instruction_implementation_notes.md** - sync-status and manifesto rendering rules
- **release_plz_guardrails.md** - release-plz config and coordination-path ignore rules
- **context.md** - umbrella release/installer summary
- **published_ito_mirror.md** - read-only mirror generation and drift handling
- **coordination_branch_git_behavior.md** - branch bootstrap and Git error handling
- **coordination_symlink_repair_and_sync.md** - `.ito/` repair and sync sequencing
- **audit_mirror_concurrency_and_temp_naming.md** - temp naming, dedupe, and retry behavior
- **worktree_validation_flow.md** - validation status and recovery guidance
- **ito_config_gotcha.md** - config storage boundaries
- **ito_orchestration_consolidation.md** - orchestration source-of-truth
- **ddd_discovery_workflow.md** - gated discovery workflow
- **obsolete_specialist_cleanup.md** - legacy specialist asset cleanup
- **pre_push_adversarial_code_review.md** - adversarial review quality gate
- **source_guide_workflow.md** - source-guide atlas workflow
