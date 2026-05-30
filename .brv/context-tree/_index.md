---
children_hash: e6506634b9f444972b2e32aedbd65b21f5c5496ca0db358d5978540e16a4590e
compression_ratio: 0.8898652606912713
condensation_order: 3
covers: [development/_index.md]
covers_token_total: 1707
summary_level: d3
token_count: 1519
type: summary
---
# development

Development centers on Ito's release, installer, coordination, and apply-time source-guide workflows. The dominant design pattern is **coordination-backed source of truth**: writable state lives in coordination storage, while published artifacts and mirrors are generated from it.

## release_workflow
End-to-end release and installer pipeline, from merge to published artifacts.

- **release_workflow.md** - Core release chain: merge release PR -> `release-plz` publishes crates and tags `vX.Y.Z` -> `cargo-dist` builds GitHub Releases -> Homebrew formula updates push to `withakay/homebrew-ito` -> release notes are polished.
- **installer_release_assets.md** - Installer scripts for Unix and Windows; prefers `cargo-dist` assets for current releases, falls back to legacy version-pinned archives, requires SHA-256 verification before extraction, and copies the built `ito` binary into the install directory.
- **build_and_coverage_guardrails.md** - Build and verification guardrails including rustup-derived `LLVM_COV`/`LLVM_PROFDATA` when unset, baseline-based max-lines enforcement, and the narrow `wit-bindgen@0.51` deny exception.
- **manifesto_instruction_implementation_notes.md** - Sync-status rendering rules for manifesto generation; `synced_at_generation` appears only when coordination sync returns `Synchronized`, while `RateLimited` is not treated as fresh success.
- **release_plz_guardrails.md** - `release-plz` config and ignore rules for coordination paths; keep `.ito/changes`, `.ito/specs`, `.ito/modules`, `.ito/workflows`, and `.ito/audit` gitignored, untrack accidentally tracked `.ito/changes` files, and never enable `git_only = true`.
- **context.md** - Umbrella release/installer summary covering artifact naming, fallback downloads, checksum validation, platform-specific install steps, and Windows PATH handling.

## ito_workflow
Coordination-backed workflow for publishing, syncing, validation, bootstrap, orchestration, cleanup, and review gates.

- **published_ito_mirror.md** - Read-only `docs/ito` mirror generated from coordination-backed state; uses safe path resolution, skips symlinks, detects drift, and replaces stale output.
- **coordination_branch_git_behavior.md** - Branch bootstrap and reservation rules; missing remote branches are created from an empty tree root commit rather than HEAD, with explicit Git error classification.
- **coordination_symlink_repair_and_sync.md** - `.ito/` symlink wiring, repair, health checks, and sync sequencing; missing or broken-target links are safe, while wrong targets and non-empty duplicate directories are unsafe.
- **audit_mirror_concurrency_and_temp_naming.md** - Collision-safe temp naming for audit mirrors, bounded log handling, deduped JSONL merges, and limited retry behavior for push/ref-update conflicts.
- **worktree_validation_flow.md** - Read-only validation command for change work; main/control checkouts hard-fail, while non-main mismatches are advisory and machine-readable.
- **ito_config_gotcha.md** - Distinguishes global CLI config from repo-local effective config; normal repo worktrees use `storage=worktree`, while the coordination worktree must use `storage=embedded`.
- **ito_orchestration_consolidation.md** - Consolidates orchestration into the central `ito agent instruction orchestrate` path and introduces `agent-surface-taxonomy` to separate direct entrypoints from delegated sub-agents.
- **ddd_discovery_workflow.md** - Domain-discovery gating and domain-grill question strategy; separates business/domain capability from Ito capability and escalates only for high-impact ambiguity or opt-in.
- **obsolete_specialist_cleanup.md** - Installer cleanup for renamed `ito-orchestrator-*` specialist assets during update and forceful init/reinstall flows, while preserving coordinator assets.
- **pre_push_adversarial_code_review.md** - Pre-push quality gate requiring adversarial code review before pushing a change branch or opening a PR; blocks P0/P1 issues while allowing minor low-risk deferrals.

## source_guides
Source-guide atlas used during Ito apply work to keep nearby guides current and verify claims against source.

- **source_guide_workflow.md** - Canonical guide hierarchy (`source-guide.md` at root, repo, crate, and per-crate levels), freshness tracking via `source-guide.json`, and the rule that guides are orientation aids rather than authoritative source.

## Cross-cutting patterns
- **Coordination-backed source of truth** - writable state stays in coordination storage; published artifacts are generated mirrors.
- **Release safety** - release-plz and installer flows avoid unsafe Git state, stale artifacts, and unchecked downloads.
- **Verification-first behavior** - checksums, worktree validation, source-guide verification, and adversarial review prioritize source truth over generated views.
- **Config boundary separation** - global CLI config and repo-local coordination config are intentionally distinct.
- **Guarded automation** - sync, bootstrap, publish, and review flows classify failures explicitly and avoid accidental destructive operations.

## Drill-down map
- **release_workflow.md** - full release pipeline
- **installer_release_assets.md** - installer scripts, targets, fallback archives, checksum flow
- **build_and_coverage_guardrails.md** - build/coverage and static guardrails
- **manifesto_instruction_implementation_notes.md** - sync-status and manifesto rendering rules
- **release_plz_guardrails.md** - `release-plz` config and coordination-path ignore rules
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
