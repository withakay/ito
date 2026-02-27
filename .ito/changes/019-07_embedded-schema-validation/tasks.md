<!-- ITO:START -->
# Tasks for: 019-07_embedded-schema-validation

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates

```bash
ito tasks status 019-07_embedded-schema-validation
ito tasks next 019-07_embedded-schema-validation
ito tasks start 019-07_embedded-schema-validation 1.1
ito tasks complete 019-07_embedded-schema-validation 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add validation.yaml to embedded schemas

- **Files**: `ito-rs/crates/ito-templates/assets/schemas/**/validation.yaml`
- **Dependencies**: None
- **Action**:
  - Add `validation.yaml` (version 1) next to each embedded schema's `schema.yaml`.
  - Use existing validator ids (`ito.delta-specs.v1`, `ito.tasks-tracking.v1`) where applicable.
- **Verify**: `make test`
- **Done When**: Embedded schemas include `validation.yaml` and `ito validate` loads it.
- **Updated At**: 2026-02-27
- **Status**: [ ] pending

### Task 1.2: Update export tests to cover validation.yaml

- **Files**: `ito-rs/crates/ito-cli/tests/templates_schemas_export.rs`
- **Dependencies**: Task 1.1
- **Action**: Assert exported schemas include `validation.yaml` for at least `spec-driven` (and any other embedded schema updated).
- **Verify**: `make test`
- **Done When**: Tests protect export of `validation.yaml`.
- **Updated At**: 2026-02-27
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Full verification

- **Files**: N/A
- **Dependencies**: Wave 1
- **Action**: Run checks and strict change validation.
- **Verify**: `ito validate 019-07_embedded-schema-validation --strict && make check && make test`
- **Done When**: Validation and tests are green.
- **Updated At**: 2026-02-27
- **Status**: [ ] pending
<!-- ITO:END -->
