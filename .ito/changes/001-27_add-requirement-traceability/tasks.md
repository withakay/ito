<!-- ITO:START -->
# Tasks for: 001-27_add-requirement-traceability

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 001-27_add-requirement-traceability
ito tasks next 001-27_add-requirement-traceability
ito tasks start 001-27_add-requirement-traceability 1.1
ito tasks complete 001-27_add-requirement-traceability 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Extend delta and task parsing for traceability metadata

- **Files**: `ito-rs/crates/ito-domain/src/**`, `ito-rs/crates/ito-core/src/**`
- **Dependencies**: None
- **Action**: Add parsing/model support for `Requirement ID` metadata in delta specs and `Requirements` metadata in enhanced task blocks, while preserving current behavior for changes that do not use either field.
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: Parsed change artifacts expose requirement ids and task references to validation/review code, and fixture coverage includes absent/present metadata cases.
- **Updated At**: 2026-03-22
- **Status**: [ ] pending

### Task 1.2: Define traceability computation semantics

- **Files**: `ito-rs/crates/ito-domain/src/**`, `ito-rs/crates/ito-core/src/**`
- **Dependencies**: None
- **Action**: Implement shared traceability computation for change-local requirement coverage, including covered/uncovered requirements, unresolved task references, duplicate ids, and shelved-task behavior.
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: A single reusable computation path exists for validation and review rendering, with tests for covered, uncovered, duplicate, and unknown-reference cases.
- **Updated At**: 2026-03-22
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Integrate traceability into `ito validate`

- **Files**: `ito-rs/crates/ito-core/src/validate/**`, `ito-rs/crates/ito-cli/src/**`
- **Dependencies**: None
- **Action**: Extend change validation to surface duplicate or unknown references as errors and uncovered requirements as warnings by default / errors in `--strict` mode.
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: Validation output clearly reports traceability issues with actionable messages and strict-mode severity behaves as designed.
- **Updated At**: 2026-03-22
- **Status**: [ ] pending

### Task 2.2: Add traceability context to review instructions

- **Files**: `ito-rs/crates/ito-core/src/**`, `ito-rs/crates/ito-templates/assets/**`
- **Dependencies**: None
- **Action**: Update review instruction generation to include a computed traceability summary and reviewer prompts when a change provides requirement ids and enhanced tasks.
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: `ito agent instruction review --change <id>` surfaces covered and uncovered requirements plus unresolved references for traced changes.
- **Updated At**: 2026-03-22
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Update authoring templates and examples

- **Files**: `.opencode/skills/**`, `ito-rs/crates/ito-templates/assets/**`, `.ito/AGENTS.md`
- **Dependencies**: None
- **Action**: Update proposal/spec/tasks guidance and templates so new changes naturally include requirement ids and enhanced task requirement references without requiring a separate matrix artifact.
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: Generated guidance shows the new metadata fields and explains when they should be used.
- **Updated At**: 2026-03-22
- **Status**: [ ] pending

### Task 3.2: Add end-to-end fixtures for traced and untraced changes

- **Files**: `ito-rs/crates/ito-core/tests/**`, `ito-rs/crates/ito-domain/tests/**`, `ito-rs/crates/ito-cli/tests/**`
- **Dependencies**: None
- **Action**: Add end-to-end tests covering traced enhanced-task changes, legacy checkbox changes, and strict/non-strict validation behavior.
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: Regression coverage proves additive rollout and prevents future drift in traceability behavior.
- **Updated At**: 2026-03-22
- **Status**: [ ] pending
<!-- ITO:END -->
