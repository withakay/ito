<!-- ITO:START -->
# Tasks for: 001-31_tools-config-tmux

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 001-31_tools-config-tmux
ito tasks next 001-31_tools-config-tmux
ito tasks start 001-31_tools-config-tmux 1.1
ito tasks complete 001-31_tools-config-tmux 1.1
```

______________________________________________________________________

## Wave 1: Config schema — add tools.tmux.enabled

- **Depends On**: None

### Task 1.1: Add tools struct to Rust config types

- **Files**: `ito-rs/crates/ito-config/src/` (config struct, likely `lib.rs` or `model.rs`)
- **Dependencies**: None
- **Action**: Add a `tools: ToolsConfig` field to the top-level config struct; define `ToolsConfig { tmux: TmuxConfig }` and `TmuxConfig { enabled: bool }` with serde default of `true` for `enabled`; ensure the field is optional in deserialization (missing = default)
- **Verify**: `cargo check -p ito-config` passes; unit test confirms `tools.tmux.enabled` defaults to `true` when absent from JSON
- **Done When**: Config types compile; default behaviour tested
- **Updated At**: 2026-03-22
- **Status**: [x] complete

### Task 1.2: Regenerate and commit config JSON schema

- **Files**: `schemas/ito-config.schema.json`
- **Dependencies**: Task 1.1
- **Action**: Run the schema generation command (e.g., `make schema` or `cargo run --bin generate-schema`) to regenerate `schemas/ito-config.schema.json`; verify the output includes `tools.tmux.enabled` as a boolean with default `true`
- **Verify**: `git diff schemas/ito-config.schema.json` shows the new `tools` key; `make check` passes schema staleness check
- **Done When**: Schema file updated and committed; CI check passes
- **Updated At**: 2026-03-22
- **Status**: [x] complete

______________________________________________________________________

## Wave 2: ito init — tmux preference prompt

- **Depends On**: Wave 1

### Task 2.1: Add tmux prompt to interactive ito init

- **Files**: `ito-rs/crates/ito-cli/src/commands/init.rs`
- **Dependencies**: None
- **Action**: After existing init prompts (storage mode, etc.), add a yes/no prompt with exact text `Do you use tmux?`; write the boolean result to `tools.tmux.enabled` in the generated project config file
- **Verify**: Manual: `ito init` in a temp dir shows the prompt; resulting config contains `tools.tmux.enabled`
- **Done When**: Prompt appears in interactive flow; config written correctly for both yes and no answers
- **Updated At**: 2026-03-22
- **Status**: [x] complete

### Task 2.2: Add --no-tmux flag to ito init

- **Files**: `ito-rs/crates/ito-cli/src/cli.rs`, `ito-rs/crates/ito-cli/src/commands/init.rs`
- **Dependencies**: Task 2.1
- **Action**: Add `--no-tmux` boolean flag to the `init` subcommand; when set, skip the tmux prompt and write `tools.tmux.enabled = false`; when absent in non-interactive mode, write `tools.tmux.enabled = true`
- **Verify**: `ito init --no-tmux --yes` (in a temp dir) produces config with `tools.tmux.enabled = false` without prompting
- **Done When**: Flag works; integration test covers both `--no-tmux` and default (true) paths
- **Updated At**: 2026-03-22
- **Status**: [x] complete

______________________________________________________________________

## Wave 3: Viewer and skill self-governance

- **Depends On**: Wave 1

### Task 3.1: Enforce tools.tmux.enabled in proposal viewer dispatch

- **Files**: `ito-rs/crates/ito-core/src/viewer/registry.rs` (once 001-29 is merged), or note as a dependency on 001-29
- **Dependencies**: None
- **Action**: In the viewer registry's `available_viewers()` method, filter out `TmuxNvimViewer` when `tools.tmux.enabled = false`; in the `--viewer tmux-nvim` flag path, reject with: `"tmux is disabled in config (tools.tmux.enabled = false). Run 'ito init' to update this preference."`
- **Verify**: Unit test: with config `tools.tmux.enabled = false`, `available_viewers()` does not include tmux-nvim; `--viewer tmux-nvim` returns the expected error
- **Done When**: Tests pass; viewer prompt never shows tmux option when disabled
- **Updated At**: 2026-03-22
- **Status**: [ ] pending

### Task 3.2: Add self-governance note to tmux skill SKILL.md

- **Files**: `ito-rs/crates/ito-templates/assets/skills/tmux/SKILL.md`
- **Dependencies**: None
- **Action**: Add a section to SKILL.md (e.g., `## Ito Integration`) instructing agents: before suggesting any tmux-based workflow step, check `tools.tmux.enabled` in the resolved Ito config; if false, omit the suggestion entirely and do not reference tmux alternatives
- **Verify**: SKILL.md contains the self-governance section; text is clear and actionable for an agent reading it
- **Done When**: Section written and reviewed
- **Updated At**: 2026-03-22
- **Status**: [-] shelved

______________________________________________________________________

## Wave 4: Tests and validation

- **Depends On**: Wave 2, Wave 3

### Task 4.1: Write integration test for init tmux prompt

- **Files**: `ito-rs/crates/ito-cli/tests/`
- **Dependencies**: None
- **Action**: Write an integration test that runs `ito init` with a stubbed TTY, answers `No` to the tmux prompt, and asserts the written config contains `tools.tmux.enabled = false`; add a second test asserting `--no-tmux` produces the same result non-interactively
- **Verify**: `cargo test -p ito-cli init_tmux` passes
- **Done When**: Both tests green
- **Updated At**: 2026-03-22
- **Status**: [ ] pending

### Task 4.2: Validate change package

- **Files**: N/A
- **Dependencies**: Task 4.1
- **Action**: Run `ito validate 001-31 --strict`
- **Verify**: Exits 0 with no errors
- **Done When**: Validation passes
- **Updated At**: 2026-03-22
- **Status**: [ ] pending

______________________________________________________________________

## Wave Guidelines

- Waves group tasks that can run in parallel within the wave
- Wave N depends on all prior waves completing
- Task dependencies within a wave are fine; cross-wave deps use the wave dependency
<!-- ITO:END -->
