---
children_hash: c58f0bbb8c142e3465fdf6e3d2ba659dc1f23af30ff9e10b23715b54374af723
compression_ratio: 0.9112391930835735
condensation_order: 3
covers: [development/_index.md]
covers_token_total: 1735
summary_level: d3
token_count: 1581
type: summary
---
# d3 Structural Summary: development

This domain captures the core operational knowledge for Ito across template retrofit, workflow/mirror safety, release guardrails, and source-guide maintenance. The child entries under `development/_index.md` form a practical map of how Ito’s coordination-backed repository is structured and protected during publish, validate, release, and apply workflows.

## 1) Template bundle retrofit
Drill down: `template_bundle_retrofit.md`, `template_bundle_retrofit.abstract.md`, `template_bundle_retrofit.overview.md`

- Retrofit standardizes template assets in `ito-rs/crates/ito-templates/assets` by adding `<!-- ITO:START -->` / `<!-- ITO:END -->` markers to plain `.md` files.
- Already pre-marked files are preserved unchanged.
- Verification confirmed no unmarked plain markdown in `ito-rs/crates/ito-templates/assets/adapters`, so adapter samples were not modified.
- Overall flow: scan assets → mark plain markdown → preserve existing markers → verify adapter sample status.

## 2) Ito workflow and coordination artifacts
Drill down: `published_ito_mirror.md`, `worktree_validation_flow.md`, `audit_mirror_concurrency_and_temp_naming.md`, `ito_orchestration_consolidation.md`, `obsolete_specialist_cleanup.md`

### Published mirror generation
- `published_mirror.path` defaults to `docs/ito`.
- Path resolution rejects empty, absolute, parent-traversal, and project-root-only values.
- `ito publish` renders a read-only `docs/ito` tree from the coordination-backed source of truth.
- Existing output is diffed against regenerated content and replaced on drift.
- Output layout includes `README.md`, `changes/active`, `changes/archive`, and `specs`.
- Symlinks are skipped during rendering.

### Worktree validation
- `ito worktree validate --change <id> [--json]` provides machine-readable status.
- Main/control checkouts fail hard.
- Non-main mismatches are advisory and include recovery guidance.
- Change matching uses exact prefixes to avoid false positives, including suffix worktrees like `<change>-review`.

### Audit mirror synchronization
- Sync flow: detect git worktree → create temp worktree → fetch/checkout branch or orphan → merge JSONL → stage/commit → push or update ref → retry on conflict.
- Temp worktree and orphan branch names incorporate PID + `SystemTime` nanos + atomic counter to avoid collisions.
- JSONL merge dedupes identical lines, preserves order, and collapses adjacent equivalent reconciled events.
- Retention keeps events within 30 days of the newest event and caps logs at 1000 events.
- Conflict handling retries once on push/ref conflicts.
- The flow only runs inside a Git worktree.

### Orchestration consolidation
- The orchestration proposal was folded into `028-02_centralize-instruction-source-of-truth`.
- `agent-surface-taxonomy` distinguishes direct entrypoint agents from delegated sub-agents.
- `ito agent instruction orchestrate` is the authoritative source for overlapping orchestration and multi-agent skills/prompts.
- Entry points such as `ito-general` and `ito-orchestrator` are separated from delegated roles like planner, researcher, worker, reviewer, and test-runner.

### Obsolete specialist cleanup
- Update and forceful init/reinstall paths pre-clean renamed `ito-orchestrator-*` specialist assets.
- Cleanup removes files, broken symlinks, and empty legacy directories before writing new assets.
- `symlink_metadata` is used so broken symlinks are removed correctly.
- Plain init preserves untouched user files.
- Coordinator assets such as `ito-orchestrator.md` and `ito-orchestrator-workflow` are preserved.

## 3) Release workflow guardrails
Drill down: `release_workflow.md`, `build_and_coverage_guardrails.md`, `release_plz_guardrails.md`, `manifesto_instruction_implementation_notes.md`

### Main release pipeline
- Merge a release PR.
- `release-plz` publishes crates and creates version tags `vX.Y.Z`.
- `cargo-dist` builds artifacts and creates GitHub Releases.
- Homebrew formulas are updated in `withakay/homebrew-ito`.
- Release notes are polished afterward.
- The split is explicit: `release-plz` handles versioning/publishing, while `cargo-dist` handles artifact builds, GitHub Releases, and Homebrew publishing.

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

## 4) Source guide workflow
Drill down: `source_guide_workflow.md`

- Guide coverage spans the repo root, `ito-rs/source-guide.md`, `ito-rs/crates/source-guide.md`, and per-crate `source-guide.md` files.
- `source-guide.json` tracks guide freshness.
- Nearby guides should be inspected before implementing an apply change.
- The `source-guide` skill is used to set up or refresh guides when needed.
- Guides are orientation aids only; important claims still need verification against source.
- After structural changes, affected guides should be updated.