# Source Guide: ito-test-support

## Responsibility
`ito-test-support` provides shared test-only helpers for integration and snapshot tests. It should not be used by production code paths.

## Entry Points
- `src/lib.rs`: command execution helpers, output normalization, candidate binary resolution.
- `src/mock_repos.rs`: in-memory repository fakes for domain/core tests.
- `src/pty/mod.rs`: PTY helpers for interactive command tests.

## Design
- Stabilize tests by normalizing HOME paths, ANSI escapes, line endings, and environment variables.
- Keep helpers generic enough to share across crates without importing production policy.

## Flow
1. Tests create temp repos/homes.
2. Helpers resolve the compiled `ito` binary or test candidate.
3. Commands run with deterministic environment and captured output.
4. Assertions compare normalized output.

## Integration
- Used by `ito-cli` integration tests and lower-level crate tests needing fake repositories.

## Gotchas
- Candidate binary resolution scans adjacent `deps`; changes here can affect many integration tests.
- Avoid making helpers depend on application crates in ways that create cycles.

## Tests
- Targeted: `cargo test -p ito-test-support`.
