## Why

The Ralph loop (`ito ralph` / `ito loop`) is a flagship workflow for autonomous iterative development. Porting it requires precise behavior matching (prompt assembly, loop state on disk, harness invocation, completion promise detection). It also benefits strongly from parity tests with harness stubs so tests do not require network access.

## What Changes

- Port `ito ralph` and `ito loop` to Rust.
- Implement loop state storage under `.ito/.state/ralph/<change>/` matching TS.
- Implement completion promise detection (`<promise>COMPLETE</promise>` and change-scoped promises).
- Add parity tests using stub harnesses (no network).

## Capabilities

### New Capabilities

- `rust-ralph`: Rust implementation of Ralph loop with parity and deterministic tests.

### Modified Capabilities

<!-- None. New Rust implementation. -->

## Impact

**Affected areas:**

- `ito-rs/crates/ito-harness/`, `ito-rs/crates/ito-cli/`, `ito-rs/crates/ito-workflow/`

**Behavioral impact:**

- None until Rust becomes default

**Risks:**

- PTY/interactive divergence; mitigated by PTY-driven tests and controlled stubs.
