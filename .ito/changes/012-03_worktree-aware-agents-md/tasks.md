# Tasks for: 012-03_worktree-aware-agents-md

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential waves
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 012-03_worktree-aware-agents-md
ito tasks next 012-03_worktree-aware-agents-md
ito tasks start 012-03_worktree-aware-agents-md 1.1
ito tasks complete 012-03_worktree-aware-agents-md 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Expand WorktreeWizardResult to carry config values

- **Files**: `ito-rs/crates/ito-cli/src/app/worktree_wizard.rs`
- **Dependencies**: None
- **Action**:
  Expand `WorktreeWizardResult` to carry the chosen worktree config values (enabled, strategy, layout_dir_name, integration_mode, default_branch). Currently the struct only has `_ran` and `_enabled` booleans. The result must carry enough information for the installer to render templates. Create a `WorktreeTemplateContext` struct (or equivalent) that can be serialized for Jinja2 rendering.
- **Verify**: `cargo test --workspace -p ito-cli`
- **Done When**: `WorktreeWizardResult` carries all worktree config fields; `run_worktree_wizard()` populates them
- **Updated At**: 2026-02-08
- **Status**: [x] complete

### Task 1.2: Reorder init to run worktree wizard before install_default_templates

- **Files**: `ito-rs/crates/ito-cli/src/app/init.rs`
- **Dependencies**: Task 1.1
- **Action**:
  Move the worktree wizard call (lines ~155-163) to execute before `install_default_templates()` (line ~143). Pass the resolved worktree config into the installer. For non-interactive mode, load worktree config from global config if it exists, or default to disabled.
- **Verify**: `cargo test --workspace -p ito-cli`
- **Done When**: Worktree wizard runs before template installation; worktree config is passed to the installer
- **Updated At**: 2026-02-08
- **Status**: [x] complete

### Task 1.3: Thread worktree config through install_default_templates

- **Files**: `ito-rs/crates/ito-core/src/installers/mod.rs`
- **Dependencies**: Task 1.2
- **Action**:
  Add an optional worktree config parameter to `install_default_templates()` and thread it through to `install_project_templates()`. For now, just accept and store it — the rendering logic comes in Wave 2. Update all call sites (init.rs, update.rs) to pass the config.
- **Verify**: `cargo build --workspace && cargo test --workspace`
- **Done When**: `install_default_templates` accepts worktree config; all call sites compile and pass
- **Updated At**: 2026-02-08
- **Status**: [x] complete

### Task 1.4: Add Jinja2 rendering function for project templates

- **Files**: `ito-rs/crates/ito-templates/src/lib.rs`
- **Dependencies**: None
- **Action**:
  Add a `render_project_template(template_bytes: &[u8], worktree_config: &WorktreeContext) -> Result<String>` function that uses the existing `minijinja` infrastructure (already a dependency of this crate, used in `instructions.rs`). Define a `WorktreeContext` struct that implements `Serialize` with all fields needed by the AGENTS.md and skill templates (enabled, strategy, layout_dir_name, integration_mode, default_branch). Include `minijinja::value::Value` conversion. If the template bytes are not valid UTF-8 or don't contain Jinja2 syntax, return the bytes unchanged.
- **Verify**: `cargo test --workspace -p ito-templates`
- **Done When**: Rendering function exists, handles both Jinja2 and plain templates, has unit tests
- **Updated At**: 2026-02-08
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Convert AGENTS.md template to Jinja2

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/AGENTS.md`
- **Dependencies**: None
- **Action**:
  Add a conditional "Worktree Workflow" section inside the managed block. Use Jinja2 `{% if %}` / `{% elif %}` blocks for each strategy. When worktrees are enabled, include: strategy name, directory name, exact path pattern, exact `git worktree add` command, integration mode, and a directive not to ask the user for worktree location. When disabled, include an explicit "not configured" message directing agents not to create worktrees. Model the template structure on the existing `apply.md.j2` which already has per-strategy blocks.
- **Verify**: Render test with each strategy + disabled state (Task 2.3)
- **Done When**: Template renders correctly for all 4 states (3 strategies + disabled)
- **Updated At**: 2026-02-08
- **Status**: [x] complete

### Task 2.2: Rewrite worktree skill template with Jinja2

- **Files**: `ito-rs/crates/ito-templates/assets/skills/using-git-worktrees/SKILL.md`
- **Dependencies**: None
- **Action**:
  Replace the "Directory Selection Process" section (lines 16-55), "Safety Verification" section (lines 57-79), and "Creation Steps" case/path logic (lines 81-148) with Jinja2 conditionals that render exact instructions based on config. Remove the "grep AGENTS.md" and "ask user" fallbacks. Keep the "Common Mistakes", "Red Flags", and "Integration" sections but update them to remove references to directory-discovery. For disabled state, instruct agents to work in current checkout. Preserve the YAML frontmatter.
- **Verify**: Render test with each strategy + disabled state (Task 2.3)
- **Done When**: Skill template contains no vague discovery heuristics; renders precise commands for each strategy
- **Updated At**: 2026-02-08
- **Status**: [x] complete

### Task 2.3: Unit tests for template rendering

- **Files**: `ito-rs/crates/ito-templates/tests/` (new test file)
- **Dependencies**: Task 2.1, Task 2.2
- **Action**:
  Write tests that render both templates (AGENTS.md and worktree skill) with each of the 4 config states:
  1. `checkout_subdir` with default `ito-worktrees` dir name
  2. `checkout_siblings` with custom dir name
  3. `bare_control_siblings` with default dir name
  4. Worktrees disabled

  Assert that rendered output contains the expected directory paths, `git worktree add` commands, strategy names, and does NOT contain discovery heuristics ("grep AGENTS.md", "ask the user").
- **Verify**: `cargo test --workspace -p ito-templates`
- **Done When**: All 8 render scenarios (4 states x 2 templates) pass
- **Updated At**: 2026-02-08
- **Status**: [x] complete

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 1, Wave 2

### Task 3.1: Render AGENTS.md via Jinja2 in the installer

- **Files**: `ito-rs/crates/ito-core/src/installers/mod.rs`
- **Dependencies**: None
- **Action**:
  In `install_project_templates()`, when processing AGENTS.md, render it via the new `render_project_template()` function with the worktree config before passing to `write_one()`. The managed block mechanism must still work correctly — the rendered content replaces the block between markers. Handle the case where no worktree config is provided (default to disabled).
- **Verify**: `cargo test --workspace -p ito-core`
- **Done When**: AGENTS.md is rendered with worktree config during init and update
- **Updated At**: 2026-02-08
- **Status**: [ ] pending

### Task 3.2: Render skills via Jinja2 in the installer

- **Files**: `ito-rs/crates/ito-core/src/distribution.rs`
- **Dependencies**: None
- **Action**:
  In `install_manifests()`, when writing skill files, check if the content contains Jinja2 syntax (`{%` or `{{`). If so, render via `render_project_template()` with worktree config before writing. If not, write verbatim (preserving current behavior). The worktree config must be threaded through from `install_adapter_files()`.
- **Verify**: `cargo test --workspace -p ito-core`
- **Done When**: Skills with Jinja2 syntax are rendered; skills without are unchanged
- **Updated At**: 2026-02-08
- **Status**: [ ] pending

### Task 3.3: Update ito update to pass worktree config

- **Files**: `ito-rs/crates/ito-cli/src/app/update.rs`
- **Dependencies**: Task 3.1, Task 3.2
- **Action**:
  Ensure `ito update` loads the current worktree config from global config and passes it to the installer, so that AGENTS.md and skills are re-rendered with current config on update.
- **Verify**: `cargo test --workspace -p ito-cli`
- **Done When**: `ito update` renders templates with current worktree config
- **Updated At**: 2026-02-08
- **Status**: [ ] pending

______________________________________________________________________

## Wave 4

- **Depends On**: Wave 3

### Task 4.1: Update init integration tests

- **Files**: `ito-rs/crates/ito-cli/tests/init_more.rs`
- **Dependencies**: None
- **Action**:
  Update existing tests for AGENTS.md marker handling to account for rendered worktree content. Add new integration tests that:
  1. Run init with worktrees enabled and verify AGENTS.md contains strategy-specific content
  2. Run init with worktrees disabled and verify AGENTS.md contains "not configured" message
  3. Run update after changing worktree config and verify AGENTS.md managed block is re-rendered
  4. Verify user content outside managed block is preserved through re-rendering
- **Verify**: `cargo test --workspace -p ito-cli`
- **Done When**: All existing tests pass; new integration tests cover the 4 scenarios above
- **Updated At**: 2026-02-08
- **Status**: [ ] pending

### Task 4.2: End-to-end verification

- **Files**: N/A
- **Dependencies**: Task 4.1
- **Action**:
  Run `make check && make test` to verify all workspace tests pass, clippy is clean, and docs build. Manually inspect a rendered AGENTS.md and skill to confirm the output reads naturally and contains precise, actionable worktree instructions.
- **Verify**: `make check && make test`
- **Done When**: Full CI-equivalent passes; rendered output reviewed and approved
- **Updated At**: 2026-02-08
- **Status**: [ ] pending

______________________________________________________________________

## Task Status Legend

- `[ ] pending` - Not started yet
- `[ ] in-progress` - Currently being worked on
- `[x] complete` - Finished and verified
- `[-] shelved` - Intentionally not-to-be-done (reversible)
