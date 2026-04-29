---
children_hash: b79142c08a4751256ed6f057d48fd866c0e07582f9cd7308d870f8d4a87c270e
compression_ratio: 0.5602307431286053
condensation_order: 2
covers: [ito_templates/_index.md, ito_workflow/_index.md, release_workflow/_index.md, source_guides/_index.md]
covers_token_total: 2947
summary_level: d2
token_count: 1651
type: summary
---
# d2 Structural Summary

This level groups Ito knowledge into four major operational areas: template marker standardization, workflow/mirror safety, release guardrails, and source-guide maintenance. Each area is documented in a dedicated child entry set with clear drill-down paths for implementation details.

## Template bundle retrofit
See `ito_templates/_index.md` for the marker standardization retrofit across `ito-rs/crates/ito-templates/assets`.

- Core rule: plain `.md` files were retrofitted with `<!-- ITO:START -->` / `<!-- ITO:END -->` markers.
- Already pre-marked files were left unchanged.
- Verification confirmed no unmarked plain markdown in `ito-rs/crates/ito-templates/assets/adapters`, so adapter samples were not modified.
- Process shape: scan assets → add markers to plain markdown → preserve pre-marked files → verify adapter sample status.

Drill down to:
- `template_bundle_retrofit.md`
- `template_bundle_retrofit.abstract.md`
- `template_bundle_retrofit.overview.md`

## Ito workflow and coordination artifacts
See `ito_workflow/_index.md` for the coordination-backed workspace and mirror flows.

### Published mirror generation
- `published_mirror.path` config defaults to `docs/ito`.
- Path resolution rejects empty, absolute, parent-traversal, and project-root-only values.
- `ito publish` renders a read-only `docs/ito` tree from coordination-backed source of truth.
- Existing output is compared against regenerated content and replaced on drift.
- Layout includes `README.md`, `changes/active`, `changes/archive`, and `specs`.
- Symlinks are skipped during rendering.

### Worktree validation
- `ito worktree validate --change <id> [--json]` provides machine-readable status.
- Main/control checkouts hard-fail.
- Non-main mismatches are advisory and include recovery guidance.
- Exact change-id prefixes are used to avoid false positives, including suffix worktrees like `<change>-review`.

### Audit mirror synchronization
- Sync flow: detect git worktree → create temp worktree → fetch/checkout branch or orphan → merge JSONL → stage/commit → push or update ref → retry on conflict.
- Temp worktree and orphan branch names include PID + `SystemTime` nanos + atomic counter to prevent collisions.
- JSONL merge dedupes identical lines, preserves order, and collapses adjacent equivalent reconciled events.
- Retention keeps events within 30 days of the newest event and caps logs at 1000 events.
- Conflict handling retries once on push/ref conflicts, and the flow only runs inside a Git worktree.

### Orchestration consolidation
- The orchestration proposal was folded into `028-02_centralize-instruction-source-of-truth`.
- `agent-surface-taxonomy` separates direct entrypoint agents from delegated sub-agents.
- `ito agent instruction orchestrate` is the authoritative source for overlapping orchestration and multi-agent skills/prompts.
- Entry points such as `ito-general` and `ito-orchestrator` are distinguished from delegated roles like planner, researcher, worker, reviewer, and test-runner.

### Installer cleanup for obsolete specialist assets
- Update and forceful init/reinstall paths pre-clean renamed `ito-orchestrator-*` specialist assets.
- Cleanup removes files, broken symlinks, and empty legacy directories before writing new assets.
- `symlink_metadata` is used so broken symlinks are removed correctly.
- Plain init preserves untouched user files.
- Coordinator assets such as `ito-orchestrator.md` and `ito-orchestrator-workflow` are preserved.

Drill down to:
- `published_ito_mirror.md`
- `worktree_validation_flow.md`
- `audit_mirror_concurrency_and_temp_naming.md`
- `ito_orchestration_consolidation.md`
- `obsolete_specialist_cleanup.md`

## Release workflow guardrails
See `release_workflow/_index.md` for the release pipeline and its safety constraints.

### Main release pipeline
- Merge a release PR.
- `release-plz` publishes crates and creates version tags `vX.Y.Z`.
- `cargo-dist` builds artifacts and creates GitHub Releases.
- Homebrew formulas are updated in `withakay/homebrew-ito`.
- Release notes are polished afterward.
- The workflow split is explicit: `release-plz` handles versioning/publishing, while `cargo-dist` handles artifact builds, GitHub Releases, and Homebrew publishing.

### Build and coverage guardrails
- The `Makefile` coverage target derives `LLVM_COV` and `LLVM_PROFDATA` from the active `rustup` toolchain when unset.
- `ito-rs/tools/max_lines_baseline.txt` records existing oversized Rust files so only regressions or new violations fail.
- `cargo-deny` allows `wit-bindgen@0.51` as a tolerated wasip3 transitive duplicate.
- Execution shape: `make check` → coverage target resolves LLVM vars → `cargo-llvm-cov` → max-lines baseline check → `cargo-deny`.

### release-plz and repository hygiene
- `.ito` coordination paths must remain gitignored.
- Already tracked ignored files should be removed with `git rm --cached`.
- `release-plz.toml` stays at the repository root for repo discovery in temp clones.
- GitHub Actions runs release-plz on `main` with separate `release` and `release-pr` jobs.
- Operational settings include `allow_dirty = false`, `publish_allow_dirty = false`, workspace changelog updates, workspace dependency updates, `cliff.toml`, and `ito-cli` as the only package with git tags enabled.
- Protected paths include `^.ito/(changes|specs|modules|workflows|audit)$`.
- `git_only = true` must not be set.

### Manifesto instruction rendering and sync status
- `synced_at_generation` is only populated when coordination sync returns `Synchronized`.
- `RateLimited` is not a fresh success and must not be reported that way.
- Full `--operation` requires `--change`.
- Embedded operation instructions are scoped to the resolved change state.
- Unconfigured operations render as `null`.

Drill down to:
- `release_workflow.md`
- `build_and_coverage_guardrails.md`
- `release_plz_guardrails.md`
- `manifesto_instruction_implementation_notes.md`

## Source guide workflow
See `source_guides/_index.md` for the code map / atlas process used during apply work.

- Guide coverage spans root, `ito-rs/source-guide.md`, `ito-rs/crates/source-guide.md`, and per-crate `source-guide.md` files.
- `source-guide.json` tracks guide freshness.
- Before implementing an apply change, nearby guides should be inspected.
- The `source-guide` skill is used to set up or refresh guide files when needed.
- Guides are orientation aids only; important claims must still be verified against source.
- After structural changes, affected guides should be updated.

Drill down to:
- `source_guide_workflow.md`