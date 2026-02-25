<!-- ITO:START -->
# Tasks for: 001-24_schema-validation-format-specs

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 001-24_schema-validation-format-specs
ito tasks next 001-24_schema-validation-format-specs
ito tasks start 001-24_schema-validation-format-specs 1.1
ito tasks complete 001-24_schema-validation-format-specs 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Draft proposal and validate change scaffolding

- **Files**: `.ito/changes/001-24_schema-validation-format-specs/proposal.md`
- **Dependencies**: None
- **Action**: Write proposal with capabilities `delta-specs` and `tasks-tracking`.
- **Verify**: `ito validate 001-24_schema-validation-format-specs --strict`
- **Done When**: Proposal passes strict validation.
- **Updated At**: 2026-02-25
- **Status**: [x] complete

### Task 1.2: Draft v1 format specs as delta specs

- **Files**: `.ito/changes/001-24_schema-validation-format-specs/specs/delta-specs/spec.md`, `.ito/changes/001-24_schema-validation-format-specs/specs/tasks-tracking/spec.md`
- **Dependencies**: Task 1.1
- **Action**: Define v1 requirements and scenarios for both formats, including stable validator ids.
- **Verify**: `ito validate 001-24_schema-validation-format-specs --strict`
- **Done When**: Both delta spec files parse and validate; scenarios use `#### Scenario:` headings.
- **Updated At**: 2026-02-25
- **Status**: [x] complete

### Task 1.3: Capture minimal design decisions

- **Files**: `.ito/changes/001-24_schema-validation-format-specs/design.md`
- **Dependencies**: Task 1.1
- **Action**: Record decisions for spec locations, validator ids, and how errors cite ids.
- **Verify**: `ito validate 001-24_schema-validation-format-specs --strict`
- **Done When**: Design doc exists (if needed) and passes validation.
- **Updated At**: 2026-02-25
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Add validator id registry entries and doc references

- **Files**: `ito-rs/crates/**`, `schemas/**`, `.ito/specs/delta-specs/spec.md`, `.ito/specs/tasks-tracking/spec.md`
- **Dependencies**: None
- **Action**: Implement the wiring so schema validation can reference `ito.delta-specs.v1` and `ito.tasks-tracking.v1`, and archive produces normative spec docs under `.ito/specs/`.
- **Verify**: `make check`
- **Done When**: Validator ids are recognized and documentation paths are stable.
- **Updated At**: 2026-02-25
- **Status**: [x] complete

### Task 2.2: Update delta spec validation messaging to cite validator id

- **Files**: `ito-rs/crates/**`
- **Dependencies**: Task 2.1
- **Action**: Ensure delta spec validation failures cite `ito.delta-specs.v1`.
- **Verify**: `make check`
- **Done When**: Failing delta specs produce issues that include the validator id.
- **Updated At**: 2026-02-25
- **Status**: [x] complete

### Task 2.3: Update tasks tracking validation messaging to cite validator id

- **Files**: `ito-rs/crates/**`
- **Dependencies**: Task 2.1
- **Action**: Ensure tasks tracking validation failures cite `ito.tasks-tracking.v1`.
- **Verify**: `make check`
- **Done When**: Failing tasks tracking files produce issues that include the validator id.
- **Updated At**: 2026-02-25
- **Status**: [x] complete

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Add tests for validator id citation in issues

- **Files**: `ito-rs/crates/**/tests/**`
- **Dependencies**: None
- **Action**: Add tests that assert validation issues include the relevant validator id for representative failures.
- **Verify**: `make check`
- **Done When**: Tests cover both formats and pass.
- **Updated At**: 2026-02-25
- **Status**: [x] complete

### Task 3.2: Final strict validation of the change

- **Files**: `.ito/changes/001-24_schema-validation-format-specs/**`
- **Dependencies**: Task 3.1
- **Action**: Run strict validation and fix any formatting/schema issues.
- **Verify**: `ito validate 001-24_schema-validation-format-specs --strict`
- **Done When**: Change validates cleanly in strict mode.
- **Updated At**: 2026-02-25
- **Status**: [x] complete

### Task 3.3: Address review feedback for format spec citations

- **Files**: `ito-rs/crates/ito-core/src/validate/issue.rs`, `ito-rs/crates/ito-core/src/validate/mod.rs`, `ito-rs/crates/ito-core/tests/validate.rs`
- **Dependencies**: None
- **Action**: Preserve non-object metadata in format-spec enrichment; ensure the "too many deltas" info issue is also enriched; add unit tests for `with_format_spec` edge cases.
- **Verify**: `make check`
- **Done When**: Validation issues remain enriched; tests cover metadata preservation and idempotent message suffix.
- **Updated At**: 2026-02-25
- **Status**: [x] complete

<!-- ITO:END -->
