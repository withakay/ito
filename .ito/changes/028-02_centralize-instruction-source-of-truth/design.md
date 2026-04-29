<!-- ITO:START -->
## Context

Ito currently distributes workflow behavior across baked-in instruction templates, shared skills, harness command files, and installed agent prompts. Orchestration shows the problem clearly: `ito agent instruction orchestrate` renders only a light wrapper, while the `ito-orchestrate` skill and orchestrator role prompts contain much of the operational policy. The same pattern is emerging for Ito memory, where skills can become the de facto workflow definition instead of a route to `ito agent instruction memory-*` artifacts.

Worktree creation exposed a related operational gap: new worktrees can have regular `.ito/changes`, `.ito/specs`, `.ito/modules`, `.ito/workflows`, and `.ito/audit` directories instead of coordination-worktree symlinks. Sync detects the problem but only warns, leaving the agent to guess how to repair it.

## Goals / Non-Goals

**Goals:**

- Make baked-in instruction templates the source of truth for reusable Ito workflows.
- Convert skills and agents for orchestrate/orchestrator and memory into thin loaders that render and follow the relevant instruction artifact.
- Expand `ito agent instruction orchestrate` enough that orchestrator agents do not need duplicated policy in prompts or skills.
- Add safe, explicit coordination symlink repair during worktree sync or initialization.

**Non-Goals:**

- Remove skills or agents entirely.
- Remove project-specific `.ito/user-prompts/*.md` guidance.
- Invent a new workflow engine for orchestration.
- Automatically overwrite non-empty local `.ito` state directories when repair might destroy user data.

## Approach

Move durable workflow content into instruction templates under `ito-rs/crates/ito-templates/assets/instructions/agent/`. Skills should become adapter documents: they describe when to use the workflow, call `ito agent instruction <artifact>`, and handle missing-instruction or setup fallback. Agent prompts should define role boundaries and reporting expectations, then immediately defer to the rendered instruction.

For orchestration, expand `orchestrate.md.j2` with the canonical process: source-of-truth precedence, coordinator duties, role dispatch, dependency planning, gate semantics, run-state layout, event log semantics, failure policy, remediation packet shape, resume behavior, and how `.ito/user-prompts/orchestrate.md` augments baked-in guidance.

For memory, keep `ito-memory` as the human/agent entrypoint, but ensure the actual capture/search/query workflows are defined in memory instruction artifacts and discoverable from help.

For coordination worktrees, update the sync/initialization path so it can create missing symlinks and replace empty generated directories with symlinks. If an existing real directory is non-empty, report a precise error with expected source and destination paths instead of silently failing or giving generic `ito init` guidance.

## Contracts / Interfaces

- CLI instruction artifacts: `ito agent instruction orchestrate`, `memory-capture`, `memory-search`, and `memory-query`.
- Installed skill templates: shared `ito-*` skills under `ito-rs/crates/ito-templates/assets/skills/`.
- Installed agent templates: orchestrator role prompts under `ito-rs/crates/ito-templates/assets/agents/` for each harness.
- Coordination symlinks: `.ito/{changes,specs,modules,workflows,audit}` pointing at the configured coordination worktree.

## Data / State

The instruction migration changes template content, not persisted domain data. Coordination symlink repair affects filesystem state in worktrees:

| Existing path state | Repair behavior |
| --- | --- |
| Missing | Create symlink to expected coordination path |
| Existing correct symlink | Leave unchanged |
| Existing wrong symlink | Fail with actual and expected targets |
| Existing empty real directory | Replace with symlink and report repair |
| Existing non-empty real directory | Fail with safe manual remediation guidance |

## Decisions

- Instruction templates are the canonical workflow layer because they can inject project context, config, testing policy, and harness-specific suggestions at render time.
- Skills remain as discovery and invocation affordances because agents already know how to load skills, but skills must not become independent policy forks.
- Agent prompts stay role-specific because coordinator, planner, researcher, worker, and reviewer responsibilities differ, but role prompts should not duplicate gate order or run-state policy.
- Symlink repair is safe only for missing paths or empty generated directories; non-empty directories require human/agent review to avoid data loss.

## Risks / Trade-offs

- Existing duplicated skill text may be useful context. Mitigation: migrate durable content into the instruction template before thinning skills.
- Different harness templates can drift. Mitigation: update source templates in `ito-templates/assets` and add tests that assert instruction-defer language across installed harnesses.
- Automatic symlink repair can be dangerous if too aggressive. Mitigation: only repair missing or empty paths and fail with explicit guidance otherwise.

## Verification Strategy

- Add/adjust CLI tests for `ito agent instruction orchestrate` to assert the rendered instruction includes source-of-truth, roles, gates, state, remediation, and resume sections.
- Add template tests asserting orchestrate and memory skills direct agents to the corresponding instruction artifacts.
- Add template tests asserting orchestrator agent prompts defer to `ito agent instruction orchestrate` and remain role-specific.
- Add worktree/coordination tests for missing symlink repair, empty directory replacement, wrong symlink rejection, and non-empty directory refusal.
- Run focused cargo tests for affected crates, then `make check` before completion.

## Migration / Rollback

Existing user projects keep their local `.ito/user-prompts/*.md` guidance. `ito init --upgrade` and `ito update` refresh Ito-managed blocks and installed templates. Rollback is reverting template and sync behavior changes; no data migration is required beyond safe symlink repair.

## Open Questions

- Should there be a lint or test helper that prevents future Ito skills from embedding large workflow sections when an instruction artifact exists?
<!-- ITO:END -->
