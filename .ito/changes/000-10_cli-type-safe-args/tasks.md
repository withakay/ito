# Tasks: 000-10_cli-type-safe-args

## Wave 1: Domain enum (ito-core)

- [ ] 1.1: Convert `HarnessName` from newtype struct to enum with variants `Opencode`, `Claude`, `Codex`, `GithubCopilot`, `Stub`
- [ ] 1.2: Implement `Display` for `HarnessName` (returns CLI-facing string: `opencode`, `claude`, `codex`, `copilot`, `stub`)
- [ ] 1.3: Implement `FromStr` for `HarnessName` (accepts `copilot` and `github-copilot` for `GithubCopilot`)
- [ ] 1.4: Add `HarnessName::user_facing()` returning iterator over non-internal variants
- [ ] 1.5: Remove `USER_FACING`, `HARNESS_HELP`, `COPILOT` constants and `help_text()` method
- [ ] 1.6: Update all `ito-core` code referencing old `HarnessName` constants (e.g. `HarnessName::OPENCODE` → `HarnessName::Opencode`)
- [ ] 1.7: Update `ito-core` tests

## Wave 2: Bridge enum (ito-cli)

- [ ] 2.1: Define `HarnessArg` enum in `ito-cli/src/cli.rs` deriving `clap::ValueEnum`
- [ ] 2.2: Mark `Stub` variant with `#[value(skip)]` to hide from help/completions but still accept
- [ ] 2.3: Add `#[value(alias = "github-copilot")]` on the `Copilot` variant
- [ ] 2.4: Implement `From<HarnessArg> for HarnessName` with exhaustive match
- [ ] 2.5: Change `RalphArgs.harness` from `String` to `HarnessArg`
- [ ] 2.6: Replace string match arm in `ralph.rs` with enum match on `HarnessArg`
- [ ] 2.7: Update snapshot tests and `ralph_unknown_harness_returns_clear_error` test

## Wave 3: Verify and document

- [ ] 3.1: Run `make check` — all 17 checks pass
- [ ] 3.2: Run `make test` — all tests pass
- [ ] 3.3: Verify `ito ralph --help` output shows harnesses correctly
- [ ] 3.4: Add doc comment on `HarnessArg` explaining the bridge pattern for future maintainers
