# Tasks for: 009-01_central-logging-and-telemetry

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential (or parallel if tool supports)
- **Created**: 2026-01-31

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Define event schema, ids, and on-disk layout

- **Files**: `ito-rs/`, `.ito/specs/`, docs
- **Dependencies**: None
- **Action**:
  - Define a stable `command_id` scheme and enumerate known CLI entrypoints.
  - Define execution event fields (timestamp, durations, outcome, etc.).
  - Define the log directory layout and file naming (session/project grouping).
  - Define the salted project hashing strategy and where salt is stored.
- **Verify**: N/A
- **Done When**: Schema and layout are documented in specs/design and are implementable
- **Updated At**: 2026-01-31
- **Status**: \[x\] complete

### Task 1.2: Implement `ito-logging` crate and integrate into `ito-rs`

- **Files**: `ito-rs/crates/`, `ito-rs/`
- **Dependencies**: Task 1.1
- **Action**:
  - Add a logging crate that writes JSONL execution events.
  - Integrate it into CLI entrypoints so every command logs start/end/outcome.
  - Ensure logging is best-effort (failures do not affect command exit).
- **Verify**: `make test`
- **Done When**: Running Ito produces central logs and all tests pass
- **Updated At**: 2026-01-31
- **Status**: \[x\] complete

### Task 1.3: Add session persistence and project id hashing

- **Files**: `ito-rs/`, `.ito/` state handling
- **Dependencies**: Task 1.2
- **Action**:
  - Persist `session_id` in a project `.ito/` state file and reuse within the session.
  - Compute `project_id` as a salted hash without recording raw paths.
- **Verify**: `make test`
- **Done When**: Events include stable `session_id` and privacy-preserving `project_id`
- **Updated At**: 2026-01-31
- **Status**: \[x\] complete

### Task 1.4: Implement `ito stats`

- **Files**: `ito-rs/`
- **Dependencies**: Task 1.3
- **Action**:
  - Add a command that reads local logs and prints counts by `command_id`.
  - Ensure it can list known commands and show zero usage.
- **Verify**: `make test`
- **Done When**: `ito stats` works locally and is covered by tests
- **Updated At**: 2026-01-31
- **Status**: \[x\] complete

### Task 1.5: Document logging, privacy, and opt-out

- **Files**: docs
- **Dependencies**: Task 1.4
- **Action**:
  - Document log locations, fields, and how to use logs for debugging.
  - Document how to disable logging/stats if desired.
- **Verify**: N/A
- **Done When**: Docs are clear and reflect actual behavior
- **Updated At**: 2026-01-31
- **Status**: \[x\] complete

______________________________________________________________________

## Checkpoints

### Checkpoint: Review Implementation

- **Type**: checkpoint (requires human approval)
- **Dependencies**: All Wave 1 tasks
- **Action**: Review the implementation before proceeding
- **Done When**: User confirms implementation is correct
- **Updated At**: 2026-01-31
- **Status**: [x] completed
