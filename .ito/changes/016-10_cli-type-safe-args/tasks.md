# Tasks: 016-10_cli-type-safe-args

## Wave 1: Domain enum (ito-core)

- [x] 1.1: Convert `HarnessName` from newtype struct to enum with variants `Opencode`, `Claude`, `Codex`, `GithubCopilot`, `Stub`
- [x] 1.2: Implement `Display` for `HarnessName` (returns CLI-facing string: `opencode`, `claude`, `codex`, `copilot`, `stub`)
- [x] 1.3: Implement `FromStr` for `HarnessName` (accepts `copilot` and `github-copilot` for `GithubCopilot`)
- [x] 1.4: Add `HarnessName::user_facing()` returning iterator over non-internal variants
- [x] 1.5: Remove `USER_FACING`, `HARNESS_HELP`, `COPILOT` constants and `help_text()` method
- [x] 1.6: Update all `ito-core` code referencing old `HarnessName` constants (e.g. `HarnessName::OPENCODE` → `HarnessName::Opencode`)
- [x] 1.7: Update `ito-core` tests

## Wave 2: Bridge enum (ito-cli)

- [x] 2.1: Define `HarnessArg` enum in `ito-cli/src/cli.rs` deriving `clap::ValueEnum`
- [x] 2.2: Mark `Stub` variant with `#[value(skip)]` to hide from help/completions but still accept
- [x] 2.3: Add `#[value(alias = "github-copilot")]` on the `Copilot` variant
- [x] 2.4: Implement `From<HarnessArg> for HarnessName` with exhaustive match
- [x] 2.5: Change `RalphArgs.harness` from `String` to `HarnessArg`
- [x] 2.6: Replace string match arm in `ralph.rs` with enum match on `HarnessArg`
- [x] 2.7: Update snapshot tests and `ralph_unknown_harness_returns_clear_error` test

## Wave 3: Verify and document

- [x] 3.1: Run `make check` — all 17 checks pass
- [x] 3.2: Run `make test` — all tests pass
- [x] 3.3: Verify `ito ralph --help` output shows harnesses correctly
- [x] 3.4: Add doc comment on `HarnessArg` explaining the bridge pattern for future maintainers
