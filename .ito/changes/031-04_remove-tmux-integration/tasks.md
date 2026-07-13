<!-- ITO:START -->
# Tasks for: 031-04_remove-tmux-integration

## Execution Notes
- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 031-04_remove-tmux-integration
ito tasks next 031-04_remove-tmux-integration
ito tasks start 031-04_remove-tmux-integration 1.1
ito tasks complete 031-04_remove-tmux-integration 1.1
```

______________________________________________________________________
## Wave 1: Removal contract tests

- **Depends On**: None

### Task 1.1: Specify config and schema absence
- **Files**: `ito-rs/crates/ito-config/src/config/types_tests.rs`; `ito-rs/crates/ito-config/tests/schema.rs`; `ito-rs/crates/ito-cli/tests/init_tmux.rs`; `schemas/ito-config.schema.json`
- **Dependencies**: None
- **Action**: Replace positive tmux-default/init assertions with failing tests that require the tmux DTO, key, prompt, flags, and generated schema entries to be absent and legacy input to receive deterministic removed/unknown-key handling.
- **Verify**: `cargo test -p ito-config tmux -- --nocapture && cargo test -p ito-cli --test init_tmux -- --nocapture`
- **Done When**: RED tests cover config deserialization, schema generation, init/update help and prompts, and removal of the public key without changing worktree defaults.
- **Updated At**: 2026-07-13
- **Status**: [x] complete

### Task 1.2: Specify viewer and installed-asset absence
- **Files**: `ito-rs/crates/ito-core/src/viewer/viewer_tests.rs`; `ito-rs/crates/ito-templates/src/lib_tests.rs`; `ito-rs/crates/ito-core/tests/distribution.rs`; `ito-rs/crates/ito-cli/tests/view_proposal.rs`
- **Dependencies**: None
- **Action**: Add failing tests asserting `tmux-nvim` is neither registered nor accepted and no harness manifest embeds/installs `ito-tmux` or its scripts. Retain positive coverage for remaining viewers and Ralph/loop assets.
- **Verify**: `cargo test -p ito-core viewer && cargo test -p ito-templates tmux && cargo test -p ito-cli --test view_proposal`
- **Done When**: Tests define exact absence across viewer names, embedded files, executable manifests, and every supported harness destination.
- **Updated At**: 2026-07-13
- **Status**: [x] complete

______________________________________________________________________
## Wave 2: Remove runtime and configuration code

- **Depends On**: Wave 1

### Task 2.1: Remove tmux configuration and init surfaces
- **Files**: `ito-rs/crates/ito-config/src/config/types.rs`; `ito-rs/crates/ito-config/src/config.rs`; `ito-rs/crates/ito-cli/src/cli.rs`; `ito-rs/crates/ito-cli/src/app/init.rs`; `ito-rs/crates/ito-cli/src/app/update.rs`
- **Dependencies**: None
- **Action**: Delete `ToolsConfig`/`TmuxConfig` where they are tmux-only, remove config defaults and resolution branches, and remove init/update prompt, flag, context, and diagnostic paths. Regenerate schema after the Rust contract is final.
- **Verify**: `cargo test -p ito-config && cargo test -p ito-cli init_ && cargo run -p ito-cli -- util config-schema --check`
- **Done When**: The CLI and schema expose no tmux preference, legacy input is handled intentionally, and all non-tmux configuration tests pass.
- **Updated At**: 2026-07-13
- **Status**: [x] complete

### Task 2.2: Remove tmux viewer implementation
- **Files**: `ito-rs/crates/ito-core/src/viewer/tmux_nvim.rs`; `ito-rs/crates/ito-core/src/viewer/mod.rs`; `ito-rs/crates/ito-core/src/viewer/registry.rs`; `ito-rs/crates/ito-cli/src/commands/view.rs`
- **Dependencies**: None
- **Action**: Delete the tmux viewer module and remove its enum/registry/probe/open/argument-validation paths without changing remaining viewer selection behavior.
- **Verify**: `cargo test -p ito-core viewer && cargo test -p ito-cli view_`
- **Done When**: `tmux-nvim` is unknown, no runtime process invocation references tmux, and all remaining viewers retain their prior availability/error behavior.
- **Updated At**: 2026-07-13
- **Status**: [x] complete

______________________________________________________________________
## Wave 3: Assets, cleanup, and proof

- **Depends On**: Wave 2

### Task 3.1: Delete embedded assets and prune managed installations
- **Files**: `ito-rs/crates/ito-templates/assets/skills/ito-tmux/`; `ito-rs/crates/ito-templates/src/legacy.rs`; `ito-rs/crates/ito-templates/src/legacy_tests.rs`; `ito-rs/crates/ito-core/src/distribution.rs`; `ito-rs/crates/ito-cli/tests/init_obsolete_cleanup.rs`
- **Dependencies**: None
- **Action**: Remove the skill/scripts and every current template reference. Add exact legacy cleanup entries for prior harness destinations, including executable helpers and broken symlinks, using existing managed ownership rules.
- **Verify**: `cargo test -p ito-templates legacy && cargo test -p ito-core distribution && cargo test -p ito-cli --test init_obsolete_cleanup`
- **Done When**: Fresh installs contain no tmux assets; upgrade/update removes obsolete Ito-managed copies idempotently; unrelated and user-owned paths are preserved.
- **Updated At**: 2026-07-13
- **Status**: [>] in-progress

### Task 3.2: Update active docs and run scoped zero-reference verification
- **Files**: `docs/src/content/docs/`; `.ito/wiki/topics/distribution-and-agents.md`; `CHANGELOG.md`; current generated/config fixtures and snapshot tests
- **Dependencies**: Task 3.1
- **Action**: Remove tmux from current product guidance and generated fixtures, add a breaking-removal release note, retain immutable archive/changelog history, and document the independent-install migration path.
- **Verify**: `make check && cargo test --workspace --all-features --exclude ito-web && rg -n "tmux|tmux-nvim|ito-tmux|tools\.tmux" ito-rs schemas docs/src .github Makefile`
- **Done When**: Default and all-feature checks pass, Ralph/loop smoke tests remain green, and the scoped search has no live integration references other than an intentional release note describing removal.
- **Updated At**: 2026-07-13
- **Status**: [ ] pending

______________________________________________________________________
## Wave Guidelines
- Waves group tasks that can run in parallel within the wave
- Wave N depends on all prior waves completing
- Keep exactly one task in progress at a time for this change
- Historical archived changes remain untouched and are excluded from the zero-live-reference assertion
<!-- ITO:END -->
