---
children_hash: 6674cb53be2fed4511178023cb5e29fb667760cf83d534bedd9cae5b0247b61a
compression_ratio: 0.750749850029994
condensation_order: 2
covers: [ito_templates/_index.md, ito_workflow/_index.md, release_workflow/_index.md, source_guides/_index.md]
covers_token_total: 3334
summary_level: d2
token_count: 2503
type: summary
---
# Ito Context Tree Structural Summary

## Development: template standardization, workflow, release, and source-guide orientation

This level captures the main structural knowledge across four domains: template retrofit, Ito workflow, release guardrails, and source-guide workflow. The entries focus on operational rules, safe synchronization, deterministic publishing, release hygiene, and how documentation guides are used during apply work.

---

## `ito_templates/_index.md` — Template Bundle Retrofit

Primary concern: standardizing marker usage across `ito-rs/crates/ito-templates/assets`.

- **Core outcome**
  - All plain `.md` files in `ito-rs/crates/ito-templates/assets` were retrofitted with `<!-- ITO:START -->` / `<!-- ITO:END -->`.
  - Files already pre-marked were left unchanged.
  - Verification found no unmarked plain markdown in `ito-rs/crates/ito-templates/assets/adapters`, so adapter samples were not modified.

- **Key rule**
  - **Plain markdown** → add ITO markers
  - **Already marked markdown** → preserve as-is

- **Process pattern**
  - scan assets → add markers to plain markdown → skip pre-marked files → verify adapter sample status

- **Drill-down**
  - `template_bundle_retrofit.md`
  - `template_bundle_retrofit.abstract.md`
  - `template_bundle_retrofit.overview.md`

---

## `ito_workflow/_index.md` — Coordination-backed sync, mirror publication, validation, and cleanup

This topic describes how Ito maintains writable coordination state while generating read-only published mirrors and managing worktree sync, validation, and installer cleanup.

### Core architecture
- `context.md` defines the top-level rule: Ito publishes a **read-only mirror** of coordination-backed state into `docs/ito`.
- The authoritative source remains **coordination-backed writable state**.
- Important concepts:
  - `published_mirror.path`
  - safe project-relative path resolution
  - drift detection
  - read-only mirror generation

### Publication and mirror management
- `published_ito_mirror.md`
  - Default mirror path: `docs/ito` via `changes.published_mirror.path`
  - Rejects unsafe paths: empty, absolute, parent traversal, project-root-only
  - Renders deterministic read-only tree:
    - `README.md`
    - `changes/active`
    - `changes/archive`
    - `specs`
  - Publish CLI loads cascading config, compares generated output to existing mirror, detects drift, and replaces the mirror from coordination state.
  - Symlinks are skipped during generation.

- `audit_mirror_concurrency_and_temp_naming.md`
  - Audit mirror sync uses unique temp worktree and orphan branch names based on:
    - process ID
    - `SystemTime` timestamp
    - atomic counter
  - JSONL merge behavior:
    - dedupe identical lines
    - preserve order
    - collapse adjacent reconciled events by incrementing count
  - Retention policy:
    - 30 days
    - 1000 events
  - Conflict retries once for:
    - non-fast-forward push conflicts
    - ref update conflicts
  - Sync flow:
    - detect git worktree → create temp worktree → fetch/checkout branch or orphan → merge JSONL → stage/commit → push or update ref → retry on conflict

### Worktree synchronization and repair
- `coordination_symlink_repair_and_sync.md`
  - Repairs missing `.ito/` links and broken symlinks whose targets were removed.
  - Empty generated `.ito/` directories may be safely replaced.
  - Explicit failure cases:
    - wrong symlink targets
    - non-empty duplicate `.ito/` directories
  - Symlinks are wired **before** health checks.
  - Missing origin/remote configuration becomes non-fatal `RateLimited` after local repair.
  - Responsibilities:
    - symlink creation/repair/teardown
    - worktree provisioning
    - auto-commit
    - sync-state persistence
    - fetch/fast-forward orchestration
  - Sync sequence:
    - provision/init → resolve worktree path → create/reuse worktree → wire `.ito` symlinks → update `.gitignore` → health check → fetch → fast-forward → rate-limit check → auto-commit → push → persist sync state

### Validation rules
- `worktree_validation_flow.md`
  - `ito worktree validate --change <id> [--json]` emits machine-readable status.
  - Main/control checkouts are **hard failures**.
  - Mismatches outside main are advisory and include recovery guidance.
  - Matching uses exact change-id prefixes, including suffix worktrees like `<change>-review`.
  - Intended for OpenCode pre-tool hooks to block only unsafe scenarios.

### Installer and template cleanup
- `obsolete_specialist_cleanup.md`
  - Cleanup runs on update flows and forceful reinstall/init paths.
  - Installer performs a pre-pass to remove legacy assets before writing new ones.
  - Broken legacy symlinks are removed via `symlink_metadata`.
  - Legacy assets renamed from `ito-orchestrator-*` to `ito-*`.
  - Coordinator assets such as `ito-orchestrator.md` and `ito-orchestrator-workflow` are preserved.
  - Plain init leaves user files untouched.

### Cross-entry relationships
- `context.md` and `published_ito_mirror.md` define the mirror/source-of-truth split.
- `coordination_symlink_repair_and_sync.md` and `audit_mirror_concurrency_and_temp_naming.md` cover sync reliability and concurrency.
- `worktree_validation_flow.md` provides read-only safety checks.
- `obsolete_specialist_cleanup.md` handles installer migration and legacy asset cleanup.

---

## `release_workflow/_index.md` — Release pipeline and guardrails

This topic defines the Ito release flow and the constraints that keep publishing, coverage, and manifest rendering consistent.

### Main release pipeline
- `release_workflow.md`
  - Release sequence:
    - merge release PR
    - `release-plz` publishes crates and tags versions `vX.Y.Z`
    - `cargo-dist` builds artifacts and creates GitHub Releases
    - Homebrew formulas update in `withakay/homebrew-ito`
    - release notes are polished afterward
  - Role split:
    - `release-plz` = versioning/publishing
    - `cargo-dist` = artifact builds, GitHub Releases, Homebrew publishing
  - Key config files:
    - `.github/workflows/release-plz.yml`
    - `.github/workflows/v-release.yml`
    - `.github/workflows/polish-release-notes.yml`
    - `dist-workspace.toml`
    - `release-plz.toml`
  - Important rule:
    - Do not set `git_only = true` in `release-plz.toml`, because it can miscalculate repository paths during diff/worktree operations.

### Build and coverage guardrails
- `build_and_coverage_guardrails.md`
  - `Makefile` coverage target derives `LLVM_COV` and `LLVM_PROFDATA` from the active `rustup` toolchain when unset.
  - `ito-rs/tools/max_lines_baseline.txt` tracks existing oversized Rust files, so only regressions or new violations fail.
  - `cargo-deny` allows `wit-bindgen@0.51` as a wasip3 transitive duplicate.
  - Workflow shape:
    - `make check` → coverage target resolves LLVM vars → `cargo-llvm-cov` runs → max-lines check compares against baseline → `cargo-deny` accepts the duplicate
  - Key pattern:
    - `^wit-bindgen@0.51$`

### Release-plz coordination and gitignore rules
- `release_plz_guardrails.md`
  - `.ito` coordination paths must remain gitignored.
  - Already tracked ignored files should be removed with `git rm --cached`.
  - `release-plz.toml` stays at repository root for repo discovery in temp clones.
  - GitHub Actions runs release-plz on `main` with separate `release` and `release-pr` jobs.
  - Important settings:
    - `allow_dirty = false`
    - `publish_allow_dirty = false`
    - workspace changelog updates enabled
    - workspace dependency updates enabled
    - changelog config uses `cliff.toml`
    - `ito-cli` is the only package with git tags enabled
  - Protected paths:
    - `^.ito/(changes|specs|modules|workflows|audit)$`
  - Rules emphasized:
    - keep `.ito/changes`, `.ito/specs`, `.ito/modules`, `.ito/workflows`, `.ito/audit` gitignored
    - if tracked ignored files appear under `.ito/changes`, untrack them with `git rm --cached`
    - do not unignore `.ito/changes`
    - do not set `git_only = true`

### Manifesto instruction rendering and sync status
- `manifesto_instruction_implementation_notes.md`
  - `synced_at_generation` is only set when coordination sync returns `Synchronized`.
  - `RateLimited` does **not** count as fresh success.
  - full `--operation` requires `--change`.
  - embedded operation instructions are scoped to the resolved change state.
  - unconfigured operations render as `null`.

### Cross-entry relationships
- `release_workflow.md` is the parent release-process overview.
- `build_and_coverage_guardrails.md` and `release_plz_guardrails.md` constrain execution and repository state.
- `manifesto_instruction_implementation_notes.md` defines rendering and sync reporting semantics.

---

## `source_guides/_index.md` — Source Guide Workflow

Ito uses source guides as a code map/code atlas during apply work.

- **Structural model**
  - Guide coverage spans:
    - root `source-guide.md`
    - `ito-rs/source-guide.md`
    - `ito-rs/crates/source-guide.md`
    - per-crate `source-guide.md`
  - `source-guide.json` tracks freshness.

- **Operational rules**
  - Inspect nearby guides before implementing apply changes.
  - Refresh missing or stale guides when needed.
  - Guides are orientation aids, not the final authority.
  - Verify implementation claims against source.
  - Update affected guides after structural changes.

- **Drill-down**
  - `source_guide_workflow.md` for the full hierarchy, freshness tracking, and verification rules

---

## Shared patterns across the topics

- Prefer safe, deterministic generation over implicit mutation.
- Treat coordination state as authoritative; published artifacts are derived mirrors.
- Fail loudly on ambiguous or wrong-target filesystem state.
- Use retry-on-conflict only for clearly retryable Git races.
- Preserve machine-readable status and exact-prefix matching where automation depends on correctness.
- Use guides and summaries as orientation, but verify source-of-truth behavior in the underlying files.