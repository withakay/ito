# Tasks for: 012-05_prefill-init-wizard-from-config

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 012-05_prefill-init-wizard-from-config
ito tasks next 012-05_prefill-init-wizard-from-config
ito tasks start 012-05_prefill-init-wizard-from-config 1.1
ito tasks complete 012-05_prefill-init-wizard-from-config 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add failing tests for config-driven init wizard defaults

- **Files**: `ito-rs/crates/ito-cli/tests/init_more.rs`, `ito-rs/crates/ito-cli/src/app/init.rs`
- **Dependencies**: None
- **Action**: Add tests that set existing worktree config keys and assert interactive init uses them as defaults without forcing re-selection.
- **Verify**: `cargo test -p ito-cli init_more`
- **Done When**: Tests fail before implementation and cover enablement/strategy/integration defaults.
- **Updated At**: 2026-02-28
- **Status**: [ ] pending

### Task 1.2: Implement defaulting behavior in the init wizard

- **Files**: `ito-rs/crates/ito-cli/src/app/init.rs`
- **Dependencies**: Task 1.1
- **Action**: Read resolved config values and use them as interactive prompt defaults; keep prompts for missing keys.
- **Verify**: `cargo test -p ito-cli init_more`
- **Done When**: Interactive init is "next-next-next" when config is already set.
- **Updated At**: 2026-02-28
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Ensure unchanged defaults do not rewrite config

- **Files**: `ito-rs/crates/ito-cli/src/app/init.rs`, `ito-rs/crates/ito-config/`
- **Dependencies**: None
- **Action**: Ensure init only persists config when values differ from existing config; update output to reflect when nothing changed.
- **Verify**: `cargo test -p ito-cli init_more && make check`
- **Done When**: Tests verify no-op persistence when defaults are accepted.
- **Updated At**: 2026-02-28
- **Status**: [ ] pending
