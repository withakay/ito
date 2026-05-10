<!-- ITO:START -->
## Why

Rust unit tests are currently mixed into production modules, which makes day-to-day code navigation noisier and increases diff churn for unrelated changes. Standardizing on separate, consistently named test modules keeps production code focused while keeping tests easy to find.

## What Changes

- Adopt a repository standard that unit tests for a Rust module live in a sibling `*_tests.rs` file (e.g., `foo.rs` -> `foo_tests.rs`, `foo/mod.rs` -> `foo/foo_tests.rs`).
- Update existing Rust modules in this repository to follow the standard (move inline `#[cfg(test)] mod tests { ... }` blocks into the corresponding `*_tests.rs` file).
- Document the convention in contributor/developer guidance and keep it discoverable.
- (Optional) Add a lightweight check (pre-commit/CI) that flags newly introduced inline unit test modules when a `*_tests.rs` sibling is expected.

## Capabilities

### New Capabilities

- `rust-test-file-conventions`: Define the required naming and placement rules for Rust unit test modules in this repository.

### Modified Capabilities

- (none)

## Impact

- Touches Rust source layout across `ito-rs/` (and any other Rust crates in-repo) without changing runtime behavior.
- Updates contributor guidance (likely `AGENTS.md` and/or `ito-rs/AGENTS.md`, and potentially docs).
- If enforcement is added, updates developer tooling configuration (e.g., `Makefile`, `.pre-commit-config.yaml`, and/or CI).
<!-- ITO:END -->
