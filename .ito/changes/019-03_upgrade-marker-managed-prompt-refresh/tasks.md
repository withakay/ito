# Tasks for: 019-03_upgrade-marker-managed-prompt-refresh

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 019-03_upgrade-marker-managed-prompt-refresh
ito tasks next 019-03_upgrade-marker-managed-prompt-refresh
ito tasks start 019-03_upgrade-marker-managed-prompt-refresh 1.1
ito tasks complete 019-03_upgrade-marker-managed-prompt-refresh 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add init upgrade mode surface

- **Files**: `ito-rs/crates/ito-cli/src/commands/init.rs`, `ito-rs/crates/ito-cli/src/app.rs`
- **Dependencies**: None
- **Action**: Add/route `--upgrade` support for `ito init` so it executes managed template refresh semantics (compatible with existing update mode).
- **Verify**: `cargo test -p ito-cli init -- --nocapture`
- **Done When**: `ito init --upgrade` is accepted and routes through managed upgrade path with clear help text/behavior.
- **Updated At**: 2026-02-22
- **Status**: [ ] pending

### Task 1.2: Enforce marker-scoped prompt/template merge policy during upgrade

- **Files**: `ito-rs/crates/ito-core/src/installers/mod.rs`, `ito-rs/crates/ito-core/src/templates/mod.rs`
- **Dependencies**: Task 1.1
- **Action**: Ensure prompt/template upgrades replace content only between Ito markers and preserve all content outside markers.
- **Verify**: `cargo test -p ito-core installers -- --nocapture`
- **Done When**: Installer merge logic guarantees marker-only replacement for managed prompt/template files.
- **Updated At**: 2026-02-22
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Add tests for marker-only replacement and missing-marker fail-safe behavior

- **Files**: `ito-rs/crates/ito-core/tests/*`, `ito-rs/crates/ito-cli/tests/*`
- **Dependencies**: None
- **Action**: Add coverage for successful marker-scoped upgrades and safe no-op behavior with guidance when markers are missing.
- **Verify**: `cargo test -p ito-core && cargo test -p ito-cli`
- **Done When**: Tests cover both normal upgrade and fail-safe edge cases.
- **Updated At**: 2026-02-22
- **Status**: [ ] pending

### Task 2.2: Update docs/help for preferred upgrade workflow

- **Files**: `ito-rs/crates/ito-cli/src/commands/init.rs`, `AGENTS.md`, `.ito/AGENTS.md` (if needed)
- **Dependencies**: Task 2.1
- **Action**: Document `ito init --upgrade` as the explicit prompt/template upgrade path while maintaining compatibility messaging for `--update`.
- **Verify**: `ito init --help`
- **Done When**: Help and guidance consistently describe non-destructive upgrade behavior.
- **Updated At**: 2026-02-22
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Evaluate dedicated agent upgrade skill follow-up

- **Files**: `.opencode/skills/` (if approved), `.ito/specs/` (if new capability needed)
- **Dependencies**: None
- **Action**: Decide whether to add a separate `ito-upgrade` skill that orchestrates validation plus CLI upgrade command as a follow-up change.
- **Verify**: `ito list --specs`
- **Done When**: Decision is captured; if in-scope, follow-up proposal is created rather than bundling optional UX in this core change.
- **Updated At**: 2026-02-22
- **Status**: [ ] pending
