# Tasks for: 030-06_validation-contract-and-ci-doctor

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 030-06_validation-contract-and-ci-doctor
ito tasks next 030-06_validation-contract-and-ci-doctor
ito tasks start 030-06_validation-contract-and-ci-doctor 1.1
ito tasks complete 030-06_validation-contract-and-ci-doctor 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Define validation contract tests

- **Files**: `ito-rs/crates/ito-core/tests/**`, `ito-rs/crates/ito-cli/tests/**`
- **Dependencies**: None
- **Action**: Add tests for validation plan discovery from config, Makefile, and safe fallback detection.
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: Tests assert command source, commands, fallback reason, and JSON shape.
- **Updated At**: 2026-05-31
- **Status**: [ ] pending

### Task 1.2: Define CI doctor and affected-test tests

- **Files**: `ito-rs/crates/ito-core/tests/**`, `ito-rs/crates/ito-cli/tests/**`
- **Dependencies**: None
- **Action**: Add tests for `ito test affected --json` targeted/fallback plans and `ito doctor ci --json` fixture CI failure summaries.
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: Tests cover failed jobs, failed steps, links, and actionable excerpts.
- **Updated At**: 2026-05-31
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Implement validation planner and runner

- **Files**: `ito-rs/crates/ito-core/src/**`, `ito-rs/crates/ito-config/src/**`
- **Dependencies**: None
- **Action**: Add validation config types, plan discovery, command-source reporting, command execution, durations, and concise failure excerpt capture.
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: `ito check --json` can report pass and fail results from the configured validation plan.
- **Updated At**: 2026-05-31
- **Status**: [ ] pending

### Task 2.2: Implement affected-test planning

- **Files**: `ito-rs/crates/ito-core/src/**`, `ito-rs/tools/**`
- **Dependencies**: None
- **Action**: Add affected-test planning with safe fallback to full validation when affected mapping is unavailable.
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: Affected plans are deterministic and explain fallback reasons.
- **Updated At**: 2026-05-31
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Implement CI doctor and workflow integration

- **Files**: `ito-rs/crates/ito-cli/src/**`, `ito-rs/crates/ito-core/src/**`, `ito-rs/crates/ito-templates/assets/**`
- **Dependencies**: None
- **Action**: Add `ito check --json`, `ito test affected --json`, `ito doctor ci --json`, and integration points for Ralph and completion verification.
- **Verify**: `make check`
- **Done When**: Commands work, agent workflows use the validation contract, and project validation passes.
- **Updated At**: 2026-05-31
- **Status**: [ ] pending
