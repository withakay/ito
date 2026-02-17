# Tasks for: 023-03_github-copilot-audit-hooks

## Execution Notes

- **Tool**: GitHub Copilot templates + Rust (installer/tests)
- **Mode**: Sequential

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Update Copilot setup steps workflow template

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.github/workflows/copilot-setup-steps.yml`
- **Dependencies**: None
- **Action**: Add steps to run `ito audit validate` (and decide whether to reconcile/fix) before the agent begins.
- **Verify**: `make test -p ito-templates` (or workspace tests)
- **Done When**: Workflow template enforces audit validation.
- **Updated At**: 2026-02-17
- **Status**: [ ] pending

### Task 1.2: Update Copilot Ito prompt templates with audit guardrails

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.github/prompts/*.prompt.md`
- **Dependencies**: None
- **Action**: Add short guidance that instructs the agent to run audit validate/reconcile before mutating `.ito/` state.
- **Verify**: `make test -p ito-templates` (or workspace tests)
- **Done When**: Prompt templates consistently emphasize audits.
- **Updated At**: 2026-02-17
- **Status**: [ ] pending

### Task 1.3: Installer tests for `.github/` assets

- **Files**: `ito-rs/crates/ito-cli/tests/update_smoke.rs` (or new tests)
- **Dependencies**: Task 1.1
- **Action**: Add tests verifying `ito init` installs and `ito update` refreshes Copilot workflow assets deterministically.
- **Verify**: `make test`
- **Done When**: Tests cover `.github/workflows/copilot-setup-steps.yml` installation and update behavior.
- **Updated At**: 2026-02-17
- **Status**: [ ] pending
