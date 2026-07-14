<!-- ITO:START -->
## Why

Ito currently jumps from open-ended idea discussion straight into proposal scaffolding, while `.ito/planning/` is bootstrapped with `PROJECT.md`, `ROADMAP.md`, and `STATE.md` files that are not meaningfully used. We need a lighter planning phase that helps shape ideas before they become one or more change proposals without creating dead planning artifacts.

## What Changes

- Add a dedicated `ito-plan` slash command and `ito-plan` skill for exploratory, question-driven planning before proposal creation.
- Define planning outputs as markdown documents authored under `.ito/planning/`, with related deep-dive research stored under `.ito/research/`.
- Stop auto-creating legacy planning bootstrap files: `.ito/planning/PROJECT.md`, `.ito/planning/ROADMAP.md`, and `.ito/planning/STATE.md`.
- Update planning initialization and status behavior to reflect a flexible planning workspace instead of a fixed three-document template.
- Keep proposal creation as a follow-up step from planning rather than folding proposal scaffolding into the planning prompt.

## Capabilities

### New Capabilities

- `planning-workflow`: Provide a dedicated pre-proposal planning prompt and planning artifact conventions for `.ito/planning/` and `.ito/research/`.

### Modified Capabilities

- `cli-plan`: Change planning initialization and status behavior so Ito no longer bootstraps or assumes the legacy planning document set.
- `ito-slash-command`: Install a dedicated `/ito-plan` slash command wrapper alongside the existing Ito command surfaces.

## Impact

- Agent-facing command and skill assets under `.opencode/` and `ito-rs/crates/ito-templates/assets/`.
- Rust planning bootstrap code, especially `ito-rs/crates/ito-core/src/planning_init.rs`, plus any domain helpers and CLI tests that assume the legacy planning files exist.
- Template/install behavior for new Ito-managed projects and upgraded projects.
<!-- ITO:END -->
