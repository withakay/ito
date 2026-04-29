<!-- ITO:START -->
## Context

Ito currently distributes workflow behavior across baked-in instruction templates, shared skills, harness command files, and installed agent prompts. Orchestration shows the problem clearly: `ito agent instruction orchestrate` renders only a light wrapper, while the `ito-orchestrate` skill, `ito-orchestrator-workflow`, subagent-driven-development guidance, and orchestrator role prompts contain overlapping operational policy.

The generated agent surface also lacks a clear activation taxonomy. Some Ito agents are valid delegated sub-agents, but `ito-general` and `ito-orchestrator` are intended as primary entrypoints that users can activate directly. When they are installed or described only as sub-agents, users lose the intended direct workflow entrypoints and the model is harder to explain.

The same source-of-truth drift is emerging for Ito memory, where skills can become the de facto workflow definition instead of a route to `ito agent instruction memory-*` artifacts.

Worktree creation exposed a related operational gap: new worktrees can have regular `.ito/changes`, `.ito/specs`, `.ito/modules`, `.ito/workflows`, and `.ito/audit` directories instead of coordination-worktree symlinks. Sync detects the problem but only warns, leaving the agent to guess how to repair it.

## Goals / Non-Goals

**Goals:**

- Make baked-in instruction templates the source of truth for reusable Ito workflows.
- Define a small canonical Ito agent taxonomy with direct entrypoints and delegated role sub-agents.
- Ensure `ito-general` and `ito-orchestrator` are directly activatable where the harness supports direct agents.
- Convert skills and agents for orchestrate/orchestrator, multi-agent/subagent orchestration, and memory into thin loaders that render and follow the relevant instruction artifact.
- Expand `ito agent instruction orchestrate` enough that orchestrator agents do not need duplicated policy in prompts or skills.
- Add safe, explicit coordination symlink repair during worktree sync or initialization.

**Non-Goals:**

- Remove all skills or all agents.
- Remove project-specific `.ito/user-prompts/*.md` guidance.
- Invent a new workflow engine for orchestration.
- Automatically overwrite non-empty local `.ito` state directories when repair might destroy user data.
- Guarantee identical direct/sub-agent mechanics across harnesses that expose different agent activation models.

## Approach

Move durable workflow content into instruction templates under `ito-rs/crates/ito-templates/assets/instructions/agent/`. Skills should become adapter documents: they describe when to use the workflow, call `ito agent instruction <artifact>`, and handle missing-instruction or setup fallback. Agent prompts should define activation mode, role boundaries, and reporting expectations, then immediately defer to the rendered instruction when a matching artifact exists.

Create a canonical generated-agent inventory and enforce it in template tests. The intended baseline is:

| Surface | Activation | Purpose |
| --- | --- | --- |
| `ito-general` | Direct entrypoint | Broad Ito development/help workflow for a user-facing session |
| `ito-orchestrator` | Direct entrypoint | Coordinates multi-change or multi-agent execution |
| `ito-planner` | Delegated sub-agent | Produces decomposition/run plans for an orchestrator |
| `ito-researcher` | Delegated sub-agent | Performs read-only context gathering |
| `ito-worker` | Delegated sub-agent | Implements assigned work packets |
| `ito-reviewer` | Delegated sub-agent | Reviews worker changes and gate results |
| `ito-test-runner` | Delegated sub-agent | Runs verification commands with curated output |

Existing adjacent agents such as `ito-quick` and `ito-thinking` should be reviewed during implementation. They should either be justified as direct variants of `ito-general` with clear non-overlapping triggers, folded into `ito-general` guidance, or removed from the Ito-managed generated surface.

For orchestration, expand `orchestrate.md.j2` with the canonical process: source-of-truth precedence, direct `ito-orchestrator` activation, delegated role dispatch, dependency planning, gate semantics, run-state layout, event log semantics, failure policy, remediation packet shape, resume behavior, and how `.ito/user-prompts/orchestrate.md` augments baked-in guidance.

For skills/prompts, consolidate overlapping orchestration surfaces. `ito-orchestrate` remains the user-facing skill/command adapter that renders `ito agent instruction orchestrate`. `ito-orchestrator-workflow` should become local workflow guidance only if it carries project-specific policy; otherwise it should fold into the instruction template. Subagent-driven-development and test-with-subagent content should either become sections of the orchestrate instruction, role-agent guidance, or separate non-overlapping skills with clear triggers.

For memory, keep `ito-memory` as the human/agent entrypoint, but ensure the actual capture/search/query workflows are defined in memory instruction artifacts and discoverable from help.

For coordination worktrees, update the sync/initialization path so it can create missing symlinks and replace empty generated directories with symlinks. If an existing real directory is non-empty, report a precise error with expected source and destination paths instead of silently failing or giving generic `ito init` guidance.

## Contracts / Interfaces

- CLI instruction artifacts: `ito agent instruction orchestrate`, `memory-capture`, `memory-search`, and `memory-query`.
- Installed skill templates: shared `ito-*` skills under `ito-rs/crates/ito-templates/assets/skills/`.
- Installed agent templates: `ito-general`, `ito-orchestrator`, and delegated role prompts under `ito-rs/crates/ito-templates/assets/agents/` for each harness.
- Installed command/prompt templates: harness-specific command files that should route to instruction artifacts rather than duplicating workflow policy.
- Coordination symlinks: `.ito/{changes,specs,modules,workflows,audit}` pointing at the configured coordination worktree.

## Data / State

The instruction and agent-surface migration changes template content and installed asset placement, not persisted domain data. Installer/update paths may remove or stop generating obsolete Ito-managed orchestration assets when they have been folded into canonical surfaces.

Coordination symlink repair affects filesystem state in worktrees:

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
- `ito-general` and `ito-orchestrator` are direct entrypoints because they represent user-invoked session modes, not work packets delegated by another Ito agent.
- Planner/researcher/worker/reviewer/test-runner remain delegated role agents because their responsibilities are bounded by an orchestrator or another primary workflow.
- Agent prompts stay role-specific because responsibilities differ, but role prompts should not duplicate gate order or run-state policy.
- Symlink repair is safe only for missing paths or empty generated directories; non-empty directories require human/agent review to avoid data loss.

## Risks / Trade-offs

- Existing duplicated skill text may be useful context. Mitigation: migrate durable content into instruction templates before thinning skills.
- Users may rely on generated names that become obsolete. Mitigation: document the new surface inventory and add installer cleanup/migration guidance for Ito-managed assets.
- Different harness templates can drift. Mitigation: update source templates in `ito-templates/assets` and add tests that assert direct/delegated inventory and instruction-defer language across installed harnesses.
- Harnesses differ in direct-agent support. Mitigation: define desired activation semantics and map them to the closest supported harness mechanism in template tests.
- Automatic symlink repair can be dangerous if too aggressive. Mitigation: only repair missing or empty paths and fail with explicit guidance otherwise.

## Verification Strategy

- Add/adjust CLI tests for `ito agent instruction orchestrate` to assert the rendered instruction includes source-of-truth, direct entrypoints, delegated roles, gates, state, remediation, and resume sections.
- Add template tests asserting orchestrate and memory skills direct agents to the corresponding instruction artifacts.
- Add template tests asserting `ito-general` and `ito-orchestrator` are installed as direct entrypoints where supported, while planner/researcher/worker/reviewer/test-runner are installed or described as delegated role sub-agents.
- Add template tests or snapshot checks that fail when obsolete/overlapping orchestration and multi-agent skill/prompt surfaces are generated without an explicit inventory entry.
- Add worktree/coordination tests for missing symlink repair, empty directory replacement, wrong symlink rejection, and non-empty directory refusal.
- Run focused cargo tests for affected crates, then `make check` before completion.

## Migration / Rollback

Existing user projects keep their local `.ito/user-prompts/*.md` guidance. `ito init --upgrade` and `ito update` refresh Ito-managed blocks and installed templates. Obsolete Ito-managed orchestration surfaces should be removed only when they are known generated assets, with migration guidance pointing to the canonical direct agent or skill. Rollback is reverting template and sync behavior changes; no data migration is required beyond safe symlink repair.

## Open Questions

- Should `ito-quick` and `ito-thinking` remain direct Ito-managed agents, become documented modes of `ito-general`, or move out of the default generated surface?
- Should there be a lint or test helper that prevents future Ito skills from embedding large workflow sections when an instruction artifact exists?
<!-- ITO:END -->
