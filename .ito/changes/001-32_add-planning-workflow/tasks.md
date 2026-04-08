<!-- ITO:START -->
# Tasks for: 001-32_add-planning-workflow

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 001-32_add-planning-workflow
ito tasks next 001-32_add-planning-workflow
ito tasks start 001-32_add-planning-workflow 1.1
ito tasks complete 001-32_add-planning-workflow 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add planning prompt assets

- **Files**: `ito-rs/crates/ito-templates/assets/commands/ito-plan.md`, `ito-rs/crates/ito-templates/assets/skills/ito-plan/SKILL.md`
- **Dependencies**: None
- **Action**: Add the embedded `ito-plan` command and skill so the planning workflow exists as a first-class pre-proposal entrypoint.
- **Verify**: `cargo test -p ito-templates`
- **Done When**: Embedded assets define `ito-plan` and its skill guidance, including `.ito/planning/` and `.ito/research/` conventions.
- **Requirements**: `planning-workflow:pre-proposal-planning`, `planning-workflow:planning-and-research-locations`, `ito-slash-command:planning-slash-command-installation`
- **Updated At**: 2026-04-08
- **Status**: [ ] pending

### Task 1.2: Sync checked-in OpenCode harness files

- **Files**: `.opencode/commands/ito-plan.md`, `.opencode/skills/ito-plan/SKILL.md`
- **Dependencies**: Task 1.1
- **Action**: Mirror the new planning command and skill into the checked-in OpenCode project files used for local development.
- **Verify**: `ito init --upgrade --help`
- **Done When**: The repo-local OpenCode command and skill files match the embedded planning workflow assets.
- **Requirements**: `planning-workflow:pre-proposal-planning`, `planning-workflow:planning-and-research-locations`, `ito-slash-command:planning-slash-command-installation`
- **Updated At**: 2026-04-08
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Stop bootstrapping legacy planning documents

- **Files**: `ito-rs/crates/ito-core/src/planning_init.rs`, `ito-rs/crates/ito-domain/src/planning.rs`
- **Dependencies**: None
- **Action**: Remove automatic creation of `.ito/planning/PROJECT.md`, `.ito/planning/ROADMAP.md`, and `.ito/planning/STATE.md`, and simplify planning bootstrap helpers around a directory-based planning workspace.
- **Verify**: `cargo test -p ito-core planning_init -- --nocapture`
- **Done When**: Planning initialization creates only the planning workspace and no longer writes the legacy planning markdown files.
- **Requirements**: `cli-plan:planning-workspace-initialization`, `cli-plan:remove-fixed-template-quality`
- **Updated At**: 2026-04-08
- **Status**: [ ] pending

### Task 2.2: Update planning CLI behavior and tests

- **Files**: `ito-rs/crates/ito-cli/src/commands/plan.rs`, `ito-rs/crates/ito-core/tests/planning_init.rs`, `ito-rs/crates/ito-cli/tests/plan_state_more.rs`, `ito-rs/crates/ito-cli/tests/misc_more.rs`
- **Dependencies**: Task 2.1
- **Action**: Change planning status and related CLI behavior to report a flexible planning workspace instead of assuming roadmap/state files, and update tests accordingly.
- **Verify**: `cargo test -p ito-cli plan -- --nocapture && cargo test -p ito-core planning_init -- --nocapture`
- **Done When**: Planning CLI output matches the new workspace semantics and the affected Rust tests pass.
- **Requirements**: `cli-plan:planning-workspace-status`, `cli-plan:planning-error-handling`, `cli-plan:remove-project-state-management`, `cli-plan:remove-roadmap-milestone-management`
- **Updated At**: 2026-04-08
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Update installed workflow guidance references

- **Files**: `.opencode/commands/ito.md`, `.opencode/skills/ito/SKILL.md`, `ito-rs/crates/ito-templates/assets/commands/ito.md`, `ito-rs/crates/ito-templates/assets/skills/ito/SKILL.md`, related planning references under `ito-rs/crates/ito-templates/assets/default/project/`
- **Dependencies**: None
- **Action**: Update routing and guidance text anywhere Ito points users toward planning so it references the new `ito-plan` pre-proposal workflow instead of the legacy planning document model.
- **Verify**: `rg -n "PROJECT\.md|ROADMAP\.md|STATE\.md|ito-plan" .opencode ito-rs/crates/ito-templates/assets`
- **Done When**: User-facing guidance consistently points planning requests toward `ito-plan` and no longer presents the legacy planning files as the primary workflow.
- **Requirements**: `planning-workflow:pre-proposal-planning`, `planning-workflow:planning-and-research-locations`, `ito-slash-command:planning-slash-command-installation`
- **Updated At**: 2026-04-08
- **Status**: [ ] pending

### Task 3.2: Validate change package and affected code paths

- **Files**: `.ito/changes/001-32_add-planning-workflow/`, affected Rust and asset files from prior tasks
- **Dependencies**: Task 3.1
- **Action**: Run Ito validation and the targeted Rust checks needed to prove the planning workflow change is coherent.
- **Verify**: `ito validate 001-32_add-planning-workflow --strict && cargo test -p ito-core planning_init -- --nocapture && cargo test -p ito-cli plan -- --nocapture`
- **Done When**: The change validates strictly and the targeted planning-related Rust tests pass.
- **Requirements**: `planning-workflow:pre-proposal-planning`, `planning-workflow:planning-and-research-locations`, `cli-plan:planning-workspace-initialization`, `cli-plan:planning-workspace-status`, `cli-plan:planning-error-handling`, `ito-slash-command:planning-slash-command-installation`
- **Updated At**: 2026-04-08
- **Status**: [ ] pending

______________________________________________________________________

## Wave Guidelines

- Waves group tasks that can run in parallel within the wave
- Wave N depends on all prior waves completing
- Task dependencies within a wave are fine; cross-wave deps use the wave dependency
- Checkpoint waves require human approval before proceeding
<!-- ITO:END -->
