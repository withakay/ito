---
children_hash: 4b111d14c8b5ecf6b9c6592d8ca452de9207c02af93ef7bf6b91b5d25f92f998
compression_ratio: 0.3413885692366705
condensation_order: 2
covers: [ito_templates/_index.md, ito_workflow/_index.md, release_workflow/_index.md]
covers_token_total: 2607
summary_level: d2
token_count: 890
type: summary
---
# d2 Structural Summary

## development

### ito_templates
Marker retrofitting for `ito-rs/crates/ito-templates/assets` focused on standardizing markdown files with `<!-- ITO:START -->` / `<!-- ITO:END -->` markers.

- **Primary rule**: plain `.md` files are retrofitted; already marked markdown is left unchanged.
- **Verification outcome**: all plain markdown in the assets tree was updated, while adapter samples were checked separately and found already compliant.
- **Process pattern**: `scan assets -> add markers to plain markdown -> preserve pre-marked files -> verify adapter status`
- Drill down via `template_bundle_retrofit.md` for the detailed retrofit and verification record.

### ito_workflow
Workflow knowledge for how Ito publishes, validates, and maintains coordination-backed assets, with emphasis on safety, drift control, and concurrent update resilience.

- **Published mirror generation** (`published_ito_mirror.md`): generates a read-only `docs/ito` mirror from coordination-backed state; path resolution is strict, symlinks are skipped, and the CLI compares generated output against the existing mirror to detect and replace drift.
- **Audit mirror concurrency and temp naming** (`audit_mirror_concurrency_and_temp_naming.md`): sync uses unique temp worktree and orphan branch names with `pid + nanos + atomic sequence` naming to avoid collisions; flow is `detect worktree -> temp worktree -> fetch/checkout branch or orphan -> merge JSONL -> stage/commit -> push/update ref -> retry on conflict`.
- **Worktree validation flow** (`worktree_validation_flow.md`): `ito worktree validate --change <id> [--json]` provides machine-readable status; exact change-id prefix matching avoids false positives, with hard failures for main/control and advisories for non-main mismatches.
- **Obsolete specialist cleanup** (`obsolete_specialist_cleanup.md`): update and force reinstall/init paths pre-clean obsolete orchestrator specialist assets at harness level, removing legacy planner/researcher/reviewer/worker files while preserving coordinator assets and leaving plain init user files untouched.
- Cross-cutting themes: strict safety checks, read-only published output, concurrency resilience, drift/control management, and migration hygiene.

## release_workflow
Release and verification knowledge spanning publishing automation, build guardrails, and manifesto instruction rendering rules.

- **Release Workflow** (`release_workflow.md`): release-plz merges the release PR, publishes crates.io releases, creates tags, and cargo-dist publishes GitHub Releases; Homebrew formula updates go to `withakay/homebrew-ito`, followed by release note polishing.
- **Build and Coverage Guardrails** (`build_and_coverage_guardrails.md`): `make check` resolves `LLVM_COV` and `LLVM_PROFDATA` from the active rustup toolchain when unset, then runs `cargo-llvm-cov`; oversized Rust files are checked against `ito-rs/tools/max_lines_baseline.txt` so regressions fail without blocking legacy issues; `cargo-deny` allows `wit-bindgen@0.51` as a narrowly scoped wasip3 duplicate.
- **Manifesto Instruction Implementation Notes** (`manifesto_instruction_implementation_notes.md`): `synced_at_generation` is only set when sync returns `Synchronized`; `RateLimited` means no fresh sync; `full --operation` requires `--change`; unconfigured operations render as `null`.
- Structural relationship: the first child covers release automation, the second enforces CI/build confidence, and the third constrains how sync and operation state is rendered and reported.