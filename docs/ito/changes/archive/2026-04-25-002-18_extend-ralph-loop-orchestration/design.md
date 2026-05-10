<!-- ITO:START -->
## Context

The current Ito Ralph loop already has important building blocks: a Rust core runner, change/module/repo targeting, worktree awareness, completion validation, retriable harness crash handling, and a thin `/ito-loop` wrapper. The downloaded upstream reference script, `ralphy.reference.sh`, shows a broader operator workflow around the loop: richer project context, task-source awareness, stronger restart/resume behavior, fail-soft queue execution, progress visibility, and workflow ergonomics across Git and AI harnesses.

The repo analysis showed two important constraints. First, Ito already has a change-centric workflow and should absorb useful Ralphy behavior into `ito ralph` and `ito-loop`, not introduce a parallel `.ralphy` product. Second, the current wrapper contract is inconsistent across specs and installed assets: the spec still mentions `.opencode/commands/loop.md`, while the shipped assets use `ito-loop.md` and describe restart behavior that is only partially implemented.

## Goals / Non-Goals

**Goals:**

- Make change-scoped Ralph runs self-sufficient by giving them the execution context they actually need.
- Define explicit, testable queue behavior for continue-ready and continue-module flows.
- Improve Ralph's persisted state, `--status` output, and restart-context generation so operators can resume reliably.
- Support alternate task-source modes inspired by Ralphy, including markdown, YAML, and GitHub issue driven execution.
- Support orchestrated git automation and parallel execution for workflows that opt into them.
- Support optional browser automation and operator notifications when the required tools are available.
- Align the `/ito-loop` wrapper with the shipped command path, safe defaults, and bounded restart behavior.
- Use the upstream script as a feature reference while keeping the resulting design Ito-native.

**Non-Goals:**

- Recreate the standalone `.ralphy --init` configuration system verbatim inside Ito.
- Implement every upstream cosmetic behavior exactly as shell-script parity regardless of Ito architecture.

## Decisions

### Decision: Absorb Ralphy by capability, not by flag-for-flag parity

This change will treat `ralphy.reference.sh` as a reference feature matrix, not a direct porting checklist. The proposal focuses on the Ralphy behaviors that fit Ito's existing change-centric workflow: execution context, queueing, reporting, and wrapper orchestration.

**Alternatives considered:**

- **Full script parity in one pass**: rejected because it would import a parallel workflow model (`.ralphy`, PRD sources, PR automation, browser/notification integrations) that conflicts with Ito's converged change workflow.
- **Tiny prompt-only parity fix**: rejected because it would ignore the wrapper, queue, and status gaps that most affect practical autonomous use.

### Decision: Model brownfield parity as richer change execution context

Instead of adding a standalone brownfield mode, change-scoped Ralph runs will assemble a richer Ito-native execution context: proposal, task progress, next actionable tasks, module/design context when present, persisted Ralph context, and validation failure context.

**Alternatives considered:**

- **Add `.ralphy` project config/init**: rejected for this change because Ito already has project config, guidance, and change artifacts.
- **Keep relying on ad hoc user prompts**: rejected because the QA script already shows that Ralph needs extra hand-authored prompts to do proposal/apply work reliably.

### Decision: Keep `ito ralph` as the core loop engine and put restart orchestration in `/ito-loop`

The Rust Ralph runtime should remain the source of truth for iteration, completion validation, and queue execution. The installed `/ito-loop` wrapper should be the opinionated launcher that applies defaults, selects the right target mode, and performs bounded restart-context enrichment when it supervises a rerun.

**Alternatives considered:**

- **Move all orchestration into core Ralph**: rejected because wrapper-level launch policy and harness defaults belong in installed command/skill assets.
- **Leave `/ito-loop` as a pure pass-through**: rejected because its current promise of restart support and safe defaults becomes misleading.

### Decision: Add broader parity features as opt-in orchestration modes

Task-source fan-in, branch/PR automation, parallel execution, browser support, and notifications should be modeled as explicit Ralph orchestration modes and options, not as mandatory behavior for every Ralph invocation.

**Alternatives considered:**

- **Always-on automation for every run**: rejected because it would make ordinary change-scoped Ralph runs too heavy and surprising.
- **Keep these features out of Ito entirely**: rejected because the user explicitly wants the richer Ralphy workflow available inside Ito.

### Decision: Queue execution should be fail-soft with aggregate reporting

For `--continue-ready` and `--continue-module`, Ralph should continue through eligible changes even when one targeted change fails, then report an aggregate outcome at the end. This better matches the upstream script's resilient operator workflow and is a better fit for module/repo sweeps.

**Alternatives considered:**

- **Abort on first change failure**: rejected because it makes autonomous queue execution brittle and leaves other ready work untouched.
- **Always return success if some changes finish**: rejected because operators still need an overall failure signal when any change run fails.

### Decision: Expand state and status reporting around restartability

Ralph state should record more than iteration count and changed-file count. It should support operator-visible restart summaries and post-run debugging with fields like exit outcome, validation acceptance/rejection, effective working directory, and per-target queue results.

**Alternatives considered:**

- **Keep current minimal state**: rejected because it is insufficient for wrapper restarts and operator diagnosis.
- **Store full logs in state**: rejected because it would bloat state and duplicate harness logs.

## Risks / Trade-offs

- **Larger execution context increases prompt size** -> Mitigation: keep context structured and derived from Ito artifacts instead of dumping whole directories.
- **Fail-soft queueing can hide the first failure during long sweeps** -> Mitigation: emit per-change results and return an aggregate failure when any change run fails.
- **Wrapper/core responsibility boundaries could drift again** -> Mitigation: update both specs and installed assets together, and test the wrapper contract explicitly.
- **Using the upstream script as a reference can tempt scope creep** -> Mitigation: keep non-Ito-native features explicitly listed as non-goals in this change.

## Migration Plan

1. Formalize the required parity surface in specs and design using `ralphy.reference.sh` as the reference input.
2. Implement richer prompt assembly, queue behavior, state/reporting, and wrapper alignment in the Rust CLI/core plus template assets.
3. Update tests and QA so proposal/apply-style Ralph runs no longer require fragile hand-crafted prompts.
4. Validate the new change package and follow-on implementation against core, CLI, and template test suites.

## Open Questions

- Should bounded wrapper restarts be limited to early exits/timeouts only, or also apply to selected non-fatal harness failures?
- How much of the `ito agent instruction apply --change <id>` guidance should be rendered directly into the Ralph prompt versus summarized into a shorter execution checklist?
<!-- ITO:END -->
