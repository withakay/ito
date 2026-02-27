<!-- ITO:START -->
# Tasks for: 019-06_schema-selection-guidance

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 019-06_schema-selection-guidance
ito tasks next 019-06_schema-selection-guidance
ito tasks start 019-06_schema-selection-guidance 1.1
ito tasks complete 019-06_schema-selection-guidance 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add `schemas` instruction artifact (text + JSON)

- **Files**: `ito-rs/crates/ito-cli/src/app/instructions.rs`, `ito-rs/crates/ito-templates/assets/instructions/agent/`
- **Dependencies**: None
- **Action**:
  - Add `ito agent instruction schemas`.
  - Output includes a short schema chooser guide and a list of schemas.
  - Add `--json` output with a stable contract (name, description, artifacts, source).
- **Verify**: `make test`
- **Done When**: CLI outputs deterministic text and JSON and includes all embedded schemas.
- **Updated At**: 2026-02-27
- **Status**: [x] complete

### Task 1.2: Update `ito-write-change-proposal` to ask for schema selection

- **Files**: `ito-rs/crates/ito-templates/assets/skills/ito-write-change-proposal/SKILL.md`
- **Dependencies**: Task 1.1
- **Action**:
  - Add a step that runs `ito agent instruction schemas` and asks the user to pick.
  - Default to `spec-driven` if the user doesn’t care.
  - Use `ito create change ... --schema <name>` when scaffolding.
- **Verify**: `make test`
- **Done When**: Skill guidance consistently prompts for schema selection and applies it.
- **Updated At**: 2026-02-27
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Update bootstrap guidance to mention schemas

- **Files**: `ito-rs/crates/ito-templates/assets/instructions/agent/bootstrap.md.j2`
- **Dependencies**: None
- **Action**: Add a short “Schemas” section pointing at `ito agent instruction schemas`.
- **Verify**: `make test`
- **Done When**: Bootstrap rendered for supported tools includes schema discoverability.
- **Updated At**: 2026-02-27
- **Status**: [x] complete

### Task 2.2: Add tests for schema listing output

- **Files**: `ito-rs/crates/ito-cli/tests/**`, `ito-rs/crates/ito-templates/tests/**`
- **Dependencies**: Task 2.1
- **Action**: Add tests covering determinism and JSON shape.
- **Verify**: `make test`
- **Done When**: Tests protect schema listing output against regressions.
- **Updated At**: 2026-02-27
- **Status**: [x] complete

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Full verification

- **Files**: N/A
- **Dependencies**: None
- **Action**: Run `make check` and `make test`.
- **Verify**: `make check && make test`
- **Done When**: Clean checks, clean tests.
- **Updated At**: 2026-02-27
- **Status**: [x] complete
<!-- ITO:END -->
