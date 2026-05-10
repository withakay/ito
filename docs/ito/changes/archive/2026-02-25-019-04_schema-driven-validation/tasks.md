<!-- ITO:START -->
# Tasks for: 019-04_schema-driven-validation

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 019-04_schema-driven-validation
ito tasks next 019-04_schema-driven-validation
ito tasks start 019-04_schema-driven-validation 1.1
ito tasks complete 019-04_schema-driven-validation 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Model and load schema validation.yaml

- **Files**: `ito-rs/crates/ito-core/src/templates/types.rs`
- **Dependencies**: None
- **Action**: Define serde models for `validation.yaml` (snake_case keys) and implement loading from the resolved schema directory.
- **Verify**: `make test`
- **Done When**: A schema directory containing `validation.yaml` can be parsed into a typed structure and surfaced to callers.
- **Updated At**: 2026-02-25
- **Status**: [x] complete

### Task 1.2: Resolve schema during change validation

- **Files**: `ito-rs/crates/ito-core/src/validate/mod.rs`
- **Dependencies**: Task 1.1
- **Action**: Update change validation to resolve the effective schema (change `.ito.yaml` override + defaults) and make it available to downstream validation steps.
- **Verify**: `make test`
- **Done When**: Validation code can determine the change's schema name/source deterministically.
- **Updated At**: 2026-02-25
- **Status**: [x] complete

### Task 1.3: Apply validation.yaml rules when present

- **Files**: `ito-rs/crates/ito-core/src/validate/mod.rs`
- **Dependencies**: Task 1.2
- **Action**: When `validation.yaml` exists, validate required artifacts (presence via schema `generates`) and dispatch configured validators by versioned id.
- **Verify**: `make test`
- **Done When**: `ito validate <change>` uses schema validation rules when available and reports the resolved schema name.
- **Updated At**: 2026-02-25
- **Status**: [x] complete

### Task 1.4: Legacy mode emits manual validation required issue

- **Files**: `ito-rs/crates/ito-core/src/validate/mod.rs`
- **Dependencies**: Task 1.2
- **Action**: When the schema has no `validation.yaml`, avoid Ito-specific delta/task assumptions and emit an explicit informational issue indicating manual validation is required.
- **Verify**: `make test`
- **Done When**: Unknown schemas do not fail solely due to missing Ito deltas, and an explicit manual-validation signal is returned.
- **Updated At**: 2026-02-25
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Schema-driven tracking file validation via apply.tracks

- **Files**: `ito-rs/crates/ito-core/src/validate/mod.rs`
- **Dependencies**: None
- **Action**: When configured by `validation.yaml`, validate the tracking file resolved from schema `apply.tracks` (and do not require `tasks.md` in that case).
- **Verify**: `make test`
- **Done When**: A schema that tracks `todo.md` validates `todo.md` and does not require `tasks.md`.
- **Updated At**: 2026-02-25
- **Status**: [x] complete

### Task 2.2: Add unit/integration tests for schema-driven validation

- **Files**: `ito-rs/crates/ito-core/src/validate/`, `ito-rs/crates/ito-core/src/templates/`
- **Dependencies**: Task 2.1
- **Action**: Add tests covering: (1) schema with `validation.yaml`, (2) schema without `validation.yaml` (manual-validation issue), (3) apply.tracks uses non-default tracking filename.
- **Verify**: `make test`
- **Done When**: Tests fail before implementation and pass after, and they lock in the expected behavior in specs.
- **Updated At**: 2026-02-25
- **Status**: [x] complete

### Task 2.3: Run full checks

- **Files**: `ito-rs/`
- **Dependencies**: Task 2.2
- **Action**: Run workspace checks and fix any lint/docs failures.
- **Verify**: `make check`
- **Done When**: `make check` passes cleanly.
- **Updated At**: 2026-02-25
- **Status**: [x] complete
<!-- ITO:END -->
