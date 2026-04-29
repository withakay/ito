---
children_hash: 24a1564304aa0a69bc46b0881f3b41f2b96c611e7648a406c29ba30577558dcc
compression_ratio: 0.8779731127197518
condensation_order: 3
covers: [development/_index.md]
covers_token_total: 967
summary_level: d3
token_count: 849
type: summary
---
# d3 Structural Summary

## development

### ito_templates
Template retrofitting for `ito-rs/crates/ito-templates/assets` standardizes markdown files with `<!-- ITO:START -->` / `<!-- ITO:END -->` markers.

- Plain `.md` files are retrofitted; already marked markdown remains unchanged.
- Verification confirmed all plain markdown assets were updated, while adapter samples were checked separately and were already compliant.
- Core pattern: `scan assets -> add markers to plain markdown -> preserve pre-marked files -> verify adapter status`.
- Drill down: `template_bundle_retrofit.md`.

### ito_workflow
Workflow knowledge for publishing, validation, and maintenance of coordination-backed assets, with emphasis on safety, drift control, and concurrent update resilience.

- **Published mirror generation** (`published_ito_mirror.md`): builds a read-only `docs/ito` mirror from coordination-backed state; path resolution is strict, symlinks are skipped, and generated output is compared against the existing mirror to detect and replace drift.
- **Audit mirror concurrency and temp naming** (`audit_mirror_concurrency_and_temp_naming.md`): sync uses unique temp worktree and orphan branch names with `pid + nanos + atomic sequence` naming to avoid collisions. Flow: `detect worktree -> temp worktree -> fetch/checkout branch or orphan -> merge JSONL -> stage/commit -> push/update ref -> retry on conflict`.
- **Worktree validation flow** (`worktree_validation_flow.md`): `ito worktree validate --change <id> [--json]` provides machine-readable status; exact change-id prefix matching avoids false positives, with hard failures for main/control mismatches and advisories for non-main mismatches.
- **Obsolete specialist cleanup** (`obsolete_specialist_cleanup.md`): update and force reinstall/init paths pre-clean obsolete orchestrator specialist assets at the harness level, removing legacy planner/researcher/reviewer/worker files while preserving coordinator assets and leaving plain init user files untouched.
- Shared themes: strict safety checks, read-only published output, concurrency resilience, drift/control management, and migration hygiene.

### release_workflow
Release and verification knowledge covering publishing automation, build guardrails, and manifesto instruction rendering rules.

- **Release Workflow** (`release_workflow.md`): release-plz merges the release PR, publishes crates.io releases, creates tags, and cargo-dist publishes GitHub Releases; Homebrew formula updates go to `withakay/homebrew-ito`, followed by release note polishing.
- **Build and Coverage Guardrails** (`build_and_coverage_guardrails.md`): `make check` resolves `LLVM_COV` and `LLVM_PROFDATA` from the active rustup toolchain when unset, then runs `cargo-llvm-cov`; oversized Rust files are checked against `ito-rs/tools/max_lines_baseline.txt` so regressions fail without blocking legacy issues; `cargo-deny` allows `wit-bindgen@0.51` as a narrowly scoped wasip3 duplicate.
- **Manifesto Instruction Implementation Notes** (`manifesto_instruction_implementation_notes.md`): `synced_at_generation` is only set when sync returns `Synchronized`; `RateLimited` means no fresh sync; `full --operation` requires `--change`; unconfigured operations render as `null`.
- Structural split: release automation, CI/build confidence, and sync/operation state rendering are documented as separate concerns.