# Tasks for: 006-19_llm-user-guidance

## Execution Notes

- **Tool**: Rust (`ito-rs/`)
- **Mode**: Sequential
- **Tracking**: Prefer `ito tasks` CLI updates

```bash
ito tasks status 006-19_llm-user-guidance
ito tasks next 006-19_llm-user-guidance
ito tasks start 006-19_llm-user-guidance 1.1
ito tasks complete 006-19_llm-user-guidance 1.1
ito tasks show 006-19_llm-user-guidance
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add Guidance File Template

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.ito/user-guidance.md`
- **Dependencies**: None
- **Action**:
  - Add a new guidance file to the default project templates.
  - Include a managed header block explaining its purpose.
  - Ensure user guidance content lives outside the managed block.
- **Verify**: `make test`
- **Done When**: `ito init` installs the file and subsequent `ito update` preserves user edits.
- **Updated At**: 2026-01-31
- **Status**: \[x\] complete

### Task 1.2: Inject Guidance Into Instruction Artifacts

- **Files**: `ito-rs/crates/ito-core/src/workflow/mod.rs`, `ito-rs/crates/ito-cli/src/main.rs`
- **Dependencies**: Task 1.1
- **Action**:
  - Load `.ito/user-guidance.md` if present.
  - Append a `## User Guidance` section to `ito agent instruction <artifact>` outputs.
  - Ensure schema instructions remain authoritative.
- **Verify**: `make test`
- **Done When**: Instruction output includes user guidance content when file exists.
- **Updated At**: 2026-01-31
- **Status**: \[x\] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Add Tests

- **Files**: `ito-rs/crates/ito-core/tests/*`, `ito-rs/crates/ito-templates/tests/*`
- **Dependencies**: None
- **Action**:
  - Add unit tests verifying guidance injection into instruction outputs.
  - Add installer/template tests verifying the guidance file exists and preserves user edits (managed block update behavior).
- **Verify**: `make test`
- **Done When**: Tests fail without feature and pass with it.
- **Updated At**: 2026-01-31
- **Status**: \[x\] complete

______________________________________________________________________

## Wave 3 (Checkpoint)

- **Depends On**: Wave 2

### Task 3.1: Review Spec + UX

- **Type**: checkpoint (requires human approval before proceeding)
- **Files**: `.ito/changes/006-19_llm-user-guidance/proposal.md`, `.ito/changes/006-19_llm-user-guidance/design.md`, `.ito/changes/006-19_llm-user-guidance/specs/**/spec.md`
- **Dependencies**: Task 2.1
- **Action**:
  - Confirm file path and naming.
  - Confirm which instruction artifacts get the guidance injection.
  - Confirm size-limit behavior (warn vs truncate).
- **Done When**: User confirms direction.
- **Updated At**: 2026-01-31
- **Status**: [x] completed
