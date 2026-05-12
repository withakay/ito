<!-- ITO:START -->
## Context

Ito already has a proposal workflow, a research workflow, and a planning directory, but the current planning path is centered on creating `PROJECT.md`, `ROADMAP.md`, and `STATE.md`. In practice, those files are not driving the workflow, while exploratory work still lacks a dedicated planning entrypoint that can ask clarifying questions and turn rough ideas into plan documents that later inform proposal creation.

The implementation will need to touch both harness assets and Rust bootstrap logic. The embedded assets in `ito-rs/crates/ito-templates/assets/` are the source of truth for generated commands and skills, while checked-in `.opencode/` files in this repo should stay aligned for local development.

## Goals / Non-Goals

**Goals:**

- Add a dedicated `ito-plan` planning lane that runs before proposal creation.
- Make planning output lightweight and location-based: plan docs in `.ito/planning/`, research docs in `.ito/research/`.
- Remove automatic creation of the unused legacy planning documents.
- Keep the planning workflow clearly connected to later proposal creation without forcing proposal scaffolding too early.

**Non-Goals:**

- Introduce a rigid schema or validator for plan documents.
- Redesign the proposal, apply, or archive workflows.
- Rework the existing research workflow beyond the guidance needed to connect it to planning.

## Decisions

### Decision: Add a dedicated `ito-plan` command and skill instead of new agent instruction text

The planning workflow is primarily an agent prompt concern: it should switch the model into discovery mode, ask clarifying questions, and write results to the correct directory. A dedicated slash command plus skill gives users an obvious entrypoint without expanding the `ito agent instruction` surface.

Alternative considered:

- Reuse `ito-proposal-intake` directly. Rejected because proposal intake is still proposal-shaped, while planning needs a looser pre-proposal mode that may branch into multiple proposals later.

### Decision: Treat planning as a flexible workspace, not a fixed three-file template

`PROJECT.md`, `ROADMAP.md`, and `STATE.md` have become dead weight. Planning should be expressed as one or more markdown documents in `.ito/planning/`, named for the topic being explored, rather than forced through a single static structure.

Alternatives considered:

- Keep the legacy files and add a new command on top. Rejected because it preserves clutter and reinforces a workflow the project no longer uses.
- Replace the legacy files with a new required plan schema. Rejected because the request explicitly wants planning to remain less structured than proposals.

### Decision: Keep research as a sibling workflow and make the planning prompt point to it explicitly

Research already has a home in `.ito/research/`. The planning skill should direct deeper investigations there and encourage plans to reference relevant research outputs, rather than duplicating research content inside planning docs.

Alternative considered:

- Move research under `.ito/planning/`. Rejected because `.ito/research/` already exists, is active, and cleanly separates exploratory evidence from plan synthesis.

### Decision: Update Rust bootstrap behavior and template assets together

Stopping legacy file creation requires changes in Rust bootstrap logic and tests, while the new planning experience requires new harness assets. Shipping only one side would leave init/update behavior inconsistent with the agent-facing planning workflow.

## Risks / Trade-offs

- Legacy workflows may still assume `PROJECT.md`, `ROADMAP.md`, or `STATE.md` exist -> Mitigation: update planning specs, status behavior, and tests so the planning workspace no longer depends on those files.
- Flexible plan documents may vary in quality -> Mitigation: make the `ito-plan` skill explicitly ask questions, frame planning as pre-proposal work, and direct users to `.ito/research/` for deeper investigation.
- Existing repos may still contain old planning files -> Mitigation: treat them as user-owned historical documents; stop creating them automatically without deleting existing files.

## Migration Plan

1. Add the new `ito-plan` command and skill to embedded template assets and checked-in harness files.
2. Update planning bootstrap logic so `ito init` / planning initialization creates only the planning workspace, not the legacy planning markdown files.
3. Update planning-related specs and tests to reflect directory-based planning.
4. Leave any existing `PROJECT.md`, `ROADMAP.md`, and `STATE.md` files untouched in already-initialized repos.

## Open Questions

- Should any legacy `ito state` or roadmap-oriented CLI commands remain as compatibility shims, or should they be explicitly retired as part of this change?
<!-- ITO:END -->
