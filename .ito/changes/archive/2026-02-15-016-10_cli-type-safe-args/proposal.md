<!-- ITO:START -->
## Why

CLI argument values that map to domain concepts (harness names, tool IDs, output formats) are represented as bare `String` fields parsed via manual `match` arms. Adding a new variant requires updating multiple disconnected locations — constants, match arms, help text, tests — with no compiler enforcement. As the CLI surface grows, this manual synchronisation is a maintenance risk and a source of bugs.

## What Changes

- Introduce a **bridge type pattern** for CLI arguments that represent domain enums: domain-layer enums in `ito-core` (no `clap` dependency) paired with adapter-layer enums in `ito-cli` that derive `clap::ValueEnum`, connected by exhaustive `From` impls that cause compile errors when variants are out of sync.
- **Tracer bullet**: refactor `--harness` on `ito ralph` from `String` to the bridge pattern (`HarnessName` enum in `ito-core`, `HarnessArg` in `ito-cli`).
- After validating the pattern on `--harness`, identify and convert other string-typed CLI args that represent closed sets of domain values.
- Remove manual `USER_FACING`, `HARNESS_HELP` constants and the hand-maintained match arm in `ralph.rs` — replaced by enum-driven dispatch.

## Capabilities

### New Capabilities

- `cli-bridge-types`: Bridge type pattern for type-safe CLI argument parsing across the `ito-core` / `ito-cli` boundary. Defines the pattern, constraints, and conventions for converting domain enums to clap-parseable adapter enums without leaking `clap` into core.

### Modified Capabilities

- `cli-ralph`: The `--harness` flag changes from a free-form `String` to a `ValueEnum`-derived type with compile-time variant enforcement. User-visible behaviour is unchanged (same accepted values, same defaults, same error messages).

## Impact

- **`ito-core/src/harness/types.rs`**: `HarnessName` becomes an enum with `Display`, `FromStr`, and iteration support. No new external dependencies.
- **`ito-cli/src/cli.rs`**: New `HarnessArg` enum deriving `clap::ValueEnum`. Replaces `pub harness: String` on `RalphArgs`.
- **`ito-cli/src/commands/ralph.rs`**: Match arm becomes exhaustive enum match. `From<HarnessArg> for HarnessName` enforces variant coverage at compile time.
- **Existing tests**: Snapshot tests for `ralph --help` will update. Smoke tests for unknown harness errors will update to reflect clap-level validation.
- **No user-facing breaking changes**: Accepted `--harness` values, defaults, and error behaviour remain the same.
- **Future work**: After the tracer bullet validates the pattern, the same approach applies to other string-typed args across the CLI surface (tool IDs in `init --tools`, output format selectors, etc.).
<!-- ITO:END -->
