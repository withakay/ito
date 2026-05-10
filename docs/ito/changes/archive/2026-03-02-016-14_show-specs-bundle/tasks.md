# Tasks for: 016-14_show-specs-bundle

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 016-14_show-specs-bundle
ito tasks next 016-14_show-specs-bundle
ito tasks start 016-14_show-specs-bundle 1.1
ito tasks complete 016-14_show-specs-bundle 1.1
ito tasks shelve 016-14_show-specs-bundle 1.1
ito tasks unshelve 016-14_show-specs-bundle 1.1
ito tasks show 016-14_show-specs-bundle
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add core API to bundle all main specs

- **Files**: `ito-rs/crates/ito-core/src/show/**`, `ito-rs/crates/ito-core/tests/show.rs`
- **Dependencies**: None
- **Action**:
  - Add a core helper that lists `.ito/specs/*` (sorted), reads each `.ito/specs/<id>/spec.md`, and returns:
    - A single markdown string containing per-spec metadata comments + verbatim spec markdown, OR
    - A JSON-friendly struct `{ specCount, specs: [{ id, path, markdown }] }`.
  - Ensure all path fields are absolute.
- **Verify**: `make test`
- **Done When**: Core tests cover deterministic ordering and metadata/path requirements.
- **Updated At**: 2026-02-25
- **Status**: [x] complete

### Task 1.2: Wire `ito show specs` in the CLI

- **Files**: `ito-rs/crates/ito-cli/src/cli.rs`, `ito-rs/crates/ito-cli/src/app/show.rs`
- **Dependencies**: None
- **Action**:
  - Add a `specs` subcommand under `ito show` (plural).
  - `ito show specs` prints the bundled markdown stream.
  - `ito show specs --json` prints the JSON structure.
- **Verify**: `make test`
- **Done When**: The command works in both clap and legacy argv parsing paths.
- **Updated At**: 2026-02-25
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Add CLI integration coverage for `ito show specs`

- **Files**: `ito-rs/crates/ito-cli/tests/**` (or existing harness tests if present)
- **Dependencies**: None
- **Action**: Add tests that:
  - Assert output contains metadata comments including spec id and absolute source path.
  - Assert ordering is ascending by spec id.
  - Assert JSON includes absolute paths.
- **Verify**: `make test`
- **Done When**: Tests fail if ordering or metadata regress.
- **Updated At**: 2026-02-25
- **Status**: [x] complete

______________________________________________________________________

## Checkpoints

### Checkpoint: Proposal Review

- **Type**: checkpoint (requires human approval)
- **Dependencies**: None
- **Action**: Review the proposal and spec delta.
- **Done When**: User approves the proposal.
- **Updated At**: 2026-02-25
- **Status**: [x] complete
