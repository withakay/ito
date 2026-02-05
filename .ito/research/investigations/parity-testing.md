# Parity Testing Strategy (TS oracle vs Rust candidate)

## Goals

- Compare stdout, stderr, and exit codes for every command.
- Compare on-disk side effects for mutating commands.
- Validate interactive flows via PTY.

## Harness Shape

Rust test support should provide:

- `run_oracle(args, env, cwd) -> (status, stdout, stderr)`
- `run_candidate(args, env, cwd) -> (status, stdout, stderr)`
- `run_with_fs_diff(...)` for commands that write to disk

## Snapshot Testing

- Use `insta` for snapshotting:
  - `--help` outputs
  - human-readable list/show/validate output
  - normalized JSON (via `serde_json::Value` snapshots)

Normalization policy:

- Prefer no normalization.
- Allow normalization only for known nondeterminism (timestamps, temp paths)
  and document each case.

## Filesystem Parity

For commands like `init` and `update`:

- Run both oracle and candidate in isolated temp dirs.
- Compare file trees:
  - Same relative paths
  - Same file bytes
  - Same line endings where applicable
- Exclude ephemeral state directories only when the oracle does.

Suggested crates:

- `tempfile` for isolated workspaces
- `walkdir` for tree enumeration
- `similar` or `pretty_assertions` for diffs

## PTY-driven Tests

Interactive commands must be tested in a PTY:

- Prompt selection flows (`show`, `validate`, `create`, `archive`)
- Multi-select flows (`init` tool selection)
- `ralph` loop output and completion promise detection

Recommended crates:

- `expectrl` (expect-like PTY interactions)
- `portable-pty` (cross-platform PTY backend)

## Determinism Controls

Set these env vars for tests unless the scenario requires defaults:

- `TZ=UTC`
- `LC_ALL=C`
- `NO_COLOR=1` for snapshot stability when validating text output
- `ITO_INTERACTIVE=0` for non-interactive behaviors
