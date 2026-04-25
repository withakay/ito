<!-- ITO:START -->
# Tasks for: 011-04_ito-init-update

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates.
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 011-04_ito-init-update
ito tasks next 011-04_ito-init-update
ito tasks start 011-04_ito-init-update 1.1
ito tasks complete 011-04_ito-init-update 1.1
```

______________________________________________________________________

## Wave 1: Gap Analysis and Test Harness

- **Depends On**: None

### Task 1.1: Classify Config Setup Coverage

- **Files**: `ito-rs/crates/ito-config`, `ito-rs/crates/ito-cli`, relevant tests
- **Dependencies**: None
- **Action**: Audit the project config model and classify fields as init-managed, update-refreshable, runtime-only, or intentionally excluded.
- **Verify**: `cargo test -p ito-cli config_coverage -- --nocapture` or the nearest added coverage test command
- **Done When**: Every config field has a coverage classification and missing classifications fail tests.
- **Requirements**: cli-init:setup-config-coverage, cli-update:refreshable-config-flag-coverage, config-schema:setup-coverage-classification
- **Updated At**: 2026-04-25
- **Status**: [ ] pending

### Task 1.2: Add Existing-Config Init Default Tests

- **Files**: `ito-rs/crates/ito-cli/tests`, `ito-rs/crates/ito-test-support`
- **Dependencies**: None
- **Action**: Add regression tests for interactive init defaults when existing config enables tmux, worktrees, and the bare sibling strategy.
- **Verify**: `cargo test -p ito-cli init_existing_config_defaults -- --nocapture` or the nearest added init test command
- **Done When**: Tests fail against the current behavior and assert selected defaults plus preserved resulting config.
- **Requirements**: cli-init:existing-config-wizard-defaults
- **Updated At**: 2026-04-25
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2: Init and Update Behavior

- **Depends On**: Wave 1

### Task 2.1: Prefill Init Wizard from Existing Config

- **Files**: `ito-rs/crates/ito-cli/src`, `ito-rs/crates/ito-core/src`, `ito-rs/crates/ito-config/src`
- **Dependencies**: None
- **Action**: Load existing config before rendering init prompts and use explicit values as prompt defaults, falling back to current defaults only when values are unset.
- **Verify**: `cargo test -p ito-cli init_existing_config_defaults -- --nocapture`
- **Done When**: Rerunning interactive init preselects existing tmux/worktree/bare sibling values and preserves accepted defaults.
- **Requirements**: cli-init:existing-config-wizard-defaults
- **Updated At**: 2026-04-25
- **Status**: [ ] pending

### Task 2.2: Add Missing Init and Update Flags

- **Files**: `ito-rs/crates/ito-cli/src`, `ito-rs/crates/ito-core/src`, docs/help snapshots if present
- **Dependencies**: Task 2.1
- **Action**: Add any missing non-interactive flags identified by the config gap analysis and ensure absent flags preserve existing explicit config.
- **Verify**: `cargo test -p ito-cli init_update_config_flags -- --nocapture` or the nearest added flag test command
- **Done When**: Covered init/update settings have flags or documented exclusions, flags override config intentionally, and no-flag update preserves explicit config.
- **Requirements**: cli-init:setup-config-coverage, cli-update:refreshable-config-flag-coverage
- **Updated At**: 2026-04-25
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3: Verification and Documentation

- **Depends On**: Wave 2

### Task 3.1: Update User-Facing Help and Docs

- **Files**: `README.md`, `docs/`, CLI help snapshots if present
- **Dependencies**: None
- **Action**: Document any new init/update flags and clarify that rerunning init uses existing config values as defaults.
- **Verify**: `cargo test -p ito-cli help -- --nocapture` or relevant docs/help verification
- **Done When**: User-facing docs and help match the implemented flag surface.
- **Requirements**: cli-init:setup-config-coverage, cli-update:refreshable-config-flag-coverage
- **Updated At**: 2026-04-25
- **Status**: [ ] pending

### Task 3.2: Run Proposal and Implementation Gates

- **Files**: `.ito/changes/011-04_ito-init-update`, repo test outputs
- **Dependencies**: Task 3.1
- **Action**: Validate the change proposal and run the project quality gate after implementation.
- **Verify**: `ito validate 011-04_ito-init-update --strict` and `make check`
- **Done When**: Ito validation and repo checks pass, or failures are documented with follow-up work.
- **Requirements**: cli-init:existing-config-wizard-defaults, cli-init:setup-config-coverage, cli-update:refreshable-config-flag-coverage, config-schema:setup-coverage-classification
- **Updated At**: 2026-04-25
- **Status**: [ ] pending
<!-- ITO:END -->
