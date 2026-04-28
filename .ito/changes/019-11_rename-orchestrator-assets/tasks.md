<!-- ITO:START -->
# Tasks

## Execution Notes

- Keep task status changes through `ito tasks start|complete`.
- Verify both source templates and generated install tests.

## Wave 1

- [ ] 1.1 Locate all `ito-orchestrator-*` skill and agent asset references
  - Files: `ito-rs/crates/ito-templates`, `ito-rs/crates/ito-cli/tests`
  - Action: Identify source asset files, manifest entries, and tests that encode old names.
  - Verify: `rg "ito-orchestrator-(planner|researcher|reviewer|worker)" ito-rs/crates/ito-templates ito-rs/crates/ito-cli/tests`
  - Done When: All source locations that need renaming are known.
- [ ] 1.2 Rename template skill and agent assets
  - Files: `ito-rs/crates/ito-templates/assets`, `ito-rs/crates/ito-templates/src/lib.rs`
  - Action: Rename affected asset files/directories and update the embedded asset manifest to concise names.
  - Verify: `rg "ito-orchestrator-(planner|researcher|reviewer|worker)" ito-rs/crates/ito-templates`
  - Done When: No template source asset path emits the old names.
- [ ] 1.3 Update tests and generated documentation expectations
  - Files: `ito-rs/crates/ito-cli/tests/init_more.rs`, generated rustdoc if required
  - Action: Adjust tests and generated expectations to match the new asset paths.
  - Verify: `cargo test -p ito-cli init_installs_orchestration_agents_and_skills`
  - Done When: Focused init/update asset tests pass.

## Wave 2

- [ ] 2.1 Run formatting and project checks
  - Files: repository-wide
  - Action: Format changed Rust code and run relevant verification.
  - Verify: `cargo fmt --check` and `make check`
  - Done When: Formatting and checks pass, or any pre-existing unrelated failure is documented.
<!-- ITO:END -->
