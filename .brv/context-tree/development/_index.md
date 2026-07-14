---
children_hash: 48115032b4033f5bfede283d95fc619a3e13feca84aafe104ca53a6e9fb68473
compression_ratio: 0.4582489309025433
condensation_order: 2
covers: [context.md, ito_templates/_index.md, ito_workflow/_index.md, release_workflow/_index.md, source_guides/_index.md]
covers_token_total: 4443
summary_level: d2
token_count: 2036
type: summary
---
# development — Structural Summary

This domain captures Ito’s release/installer workflows and the broader operational structures around safe distribution, worktree coordination, and documentation-driven release behavior.

## Primary topic clusters

### `ito_workflow/`
Ito’s coordination-backed workflow layer. The central pattern is **coordination state as the writable source of truth**, with read-only published outputs and strict safety gates around Git, worktrees, and symlinked state.

- **Mirror publication and sync**
  - `published_ito_mirror.md` defines the published `docs/ito` mirror as generated from coordination-backed state.
  - Mirror generation is read-only, skips symlinks, rejects unsafe paths, and only replaces content after drift detection.
  - `audit_mirror_concurrency_and_temp_naming.md` covers temp naming, JSONL merge/dedup behavior, retention limits, and bounded retry on conflicts.

- **Worktree validation and safety**
  - `worktree_validation_flow.md` defines `ito worktree validate --change <id> [--json]`.
  - Main/control checkouts are hard failures; non-main mismatches are advisory.
  - Matching uses exact change-id prefixes to avoid false positives and feeds machine-readable hook status.
  - `pre_push_adversarial_code_review.md` adds a pre-push review gate to block P0/P1 issues before pushing or opening PRs.

- **Coordination branch and symlink lifecycle**
  - `coordination_branch_bootstrap.md` requires a clean empty-tree root commit, forbids parent refs during init, rejects empty `commit-tree`/`mktree` stdout, and pushes the created commit as `<oid>:refs/heads/<branch>`.
  - It is object-format aware, supporting SHA-256 with SHA-1 fallback.
  - `coordination_symlink_repair_and_sync.md` defines repair/sync rules: fix missing or correct links, fail on wrong targets or ambiguous duplicates, wire symlinks before health checks, then fetch/fast-forward/auto-commit/push.
  - `ito_config_gotcha.md` records config and storage split: global CLI config at `~/.config/ito/config.json`, repo-local `.ito/config.json`, and `storage=embedded` for the coordination worktree at `~/.local/share/ito/withakay/ito` to avoid self-symlink validation.

- **Workflow governance and consolidation**
  - `ddd_discovery_workflow.md` documents a gated discovery process with capability boundary checks, ownership boundaries, and optional “rigorous domain-grill” behavior.
  - `ito_orchestration_consolidation.md` consolidates orchestration into the canonical `ito agent instruction orchestrate` path and introduces an `agent-surface-taxonomy`.
  - `obsolete_specialist_cleanup.md` covers removing obsolete `ito-orchestrator` assets during update/reinstall while preserving coordinator assets.

### `release_workflow/`
Ito’s release and installer pipeline, spanning `release-plz`, `cargo-dist`, installer assets, and release guardrails.

- **Release pipeline**
  - `release_workflow.md` defines the end-to-end path: merge release PR → `release-plz` publishes/tags `vX.Y.Z` → `cargo-dist` builds GitHub Releases → Homebrew formula updates → release notes polishing.
  - Key files include `.github/workflows/release-plz.yml`, `.github/workflows/v-release.yml`, `.github/workflows/polish-release-notes.yml`, `dist-workspace.toml`, and `release-plz.toml`.
  - Important rule: do not set `git_only = true` in `release-plz.toml`.

- **Installer distribution**
  - `installer_release_assets.md` documents Unix and Windows install scripts.
  - Flow: detect platform → determine target triple → resolve tag/version → download primary asset and checksum → fall back to legacy archive if needed → verify SHA-256 → extract → locate binary → install.
  - Current naming conventions:
    - Unix: `ito-cli-${TARGET}.tar.xz`
    - Windows: `ito-cli-$target.zip`
    - Legacy fallback: `ito-v<tag>-<target>.*`
  - Windows supports optional `AddToPath`; AMD64 maps to `x86_64-pc-windows-msvc`.

- **Build and verification guardrails**
  - `build_and_coverage_guardrails.md` adds environment-aware coverage setup, a max-lines baseline file, and a narrow `cargo-deny` exception for `wit-bindgen@0.51`.
  - Coverage tooling derives `LLVM_COV` and `LLVM_PROFDATA` from the active rustup toolchain when unset.
  - Oversized-file enforcement is baseline-driven rather than absolute.

- **Manifesto generation behavior**
  - `manifesto_instruction_implementation_notes.md` ties `synced_at_generation` to successful coordination sync only.
  - `RateLimited` is not treated as fresh sync success.
  - Full `--operation` requires `--change`; unconfigured operations render as `null`.

- **Release-plz guardrails**
  - `release_plz_guardrails.md` preserves `.gitignore` behavior for coordination paths and keeps `.ito/changes`, `.ito/specs`, `.ito/modules`, `.ito/workflows`, and `.ito/audit` ignored.
  - If ignored files become tracked, they are untracked with `git rm --cached` rather than unignored.
  - Configuration facts include `allow_dirty = false`, `publish_allow_dirty = false`, workspace changelog/dependency updates enabled, and `cliff.toml` as changelog config.

- **Shared umbrella context**
  - `context.md` summarizes installer release asset naming, fallback behavior, checksum verification, platform install steps, and PATH handling.

### `source_guides/`
A source-navigation layer for Ito apply work.

- `source_guide_workflow.md` treats `source-guide.md` files as a code atlas for orientation, not authority.
- Workflow pattern: check nearby guides → refresh stale ones → read for orientation → verify important claims against source → update guides after structural changes.
- Guide hierarchy spans root, `ito-rs`, `ito-rs/crates`, and per-crate guides, with freshness tracked in `source-guide.json`.

### `ito_templates/`
Template bundle retrofit knowledge for standardizing markdown markers.

- `template_bundle_retrofit.md` documents the retrofit of `<!-- ITO:START -->` / `<!-- ITO:END -->` markers across `ito-rs/crates/ito-templates/assets`.
- Core rule: retrofit plain markdown, leave already marked files unchanged.
- Adapter samples were checked separately and did not require modification.

## Cross-cutting patterns

- **Source of truth**: coordination-backed state and source files remain authoritative; mirrors, guides, and published outputs are derived or advisory.
- **Safety-first behavior**: explicit rejection of unsafe paths, self-referential worktrees, wrong symlink targets, empty bootstrap output, and main/control checkout validation.
- **Release robustness**: fallback archives, checksum verification, platform-specific targets, and guarded release-plz configuration.
- **Operational clarity**: exact file paths, branch names, CLI signatures, and config locations are preserved as primary drill-down anchors.

## Drill-down map

- Mirror generation and read-only published output: `ito_workflow/published_ito_mirror.md`
- Mirror concurrency, temp naming, and retention: `ito_workflow/audit_mirror_concurrency_and_temp_naming.md`
- Worktree validation and hook status: `ito_workflow/worktree_validation_flow.md`
- Pre-push adversarial review: `ito_workflow/pre_push_adversarial_code_review.md`
- Coordination branch bootstrap: `ito_workflow/coordination_branch_bootstrap.md`
- Symlink repair and sync orchestration: `ito_workflow/coordination_symlink_repair_and_sync.md`
- Config path and storage mode gotcha: `ito_workflow/ito_config_gotcha.md`
- DDD discovery gating: `ito_workflow/ddd_discovery_workflow.md`
- Orchestration consolidation: `ito_workflow/ito_orchestration_consolidation.md`
- Legacy specialist cleanup: `ito_workflow/obsolete_specialist_cleanup.md`
- Release pipeline: `release_workflow/release_workflow.md`
- Installer assets and fallback flow: `release_workflow/installer_release_assets.md`
- Build and coverage guardrails: `release_workflow/build_and_coverage_guardrails.md`
- Manifesto sync behavior: `release_workflow/manifesto_instruction_implementation_notes.md`
- Release-plz guardrails: `release_workflow/release_plz_guardrails.md`
- Source-guide atlas workflow: `source_guides/source_guide_workflow.md`
- Template marker retrofit: `ito_templates/template_bundle_retrofit.md`