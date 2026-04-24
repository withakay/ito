<!-- ITO:START -->
## Context

The repository already has several pieces of coordination-worktree behavior, but they are split across multiple call sites. `ito create`, `ito tasks start`, and apply-instruction generation already do best-effort coordination fetches, coordination-worktree auto-commit already exists in core, and coordination health checks already detect missing or broken symlinks. What is missing is a single reusable sync pathway that validates exact `.ito/` wiring, commits and pushes coordination state, and can be called frequently from skills without spamming the remote.

## Goals / Non-Goals

**Goals:**

- Add one thin CLI entry point for coordination-worktree sync.
- Reuse and centralize existing coordination fetch, commit, and health-check behavior.
- Detect miswired `.ito/` links and duplicate real directories before any remote push.
- Keep repeated skill-driven sync calls quiet by rate-limiting redundant pushes locally.
- Record last successful sync state so the quiet window is based on actual successful pushes, not guesswork.
- Make the quiet window configurable with a default of 120 seconds and allow explicit bypass via `ito sync --force`.
- Make archive a two-stage coordination-first workflow so archived changes disseminate to working copies before they are integrated into `main`.
- Let archive guidance and follow-up behavior consult config to decide whether `main` integration happens by direct merge, PR, PR with auto-merge, or manual follow-up.
- Make `ito finish` ask whether to archive now for completed worktree-backed changes so finish remains the natural handoff point without forcing archive every time.
- Keep CLI-generated instruction templates as the source of truth for workflow interactions and keep skills/commands as thin wrappers over those templates.
- Add clear sync touchpoints to agent instructions and the relevant mirrored skill files.

**Non-Goals:**

- Replacing backend artifact sync flows.
- Adding interactive merge or conflict-resolution behavior to sync.
- Expanding sync to non-worktree storage modes beyond a short no-op.
- Bypassing repository protections or review policies when PR-based archive integration is selected.

## Decisions

- Decision: Add a top-level `ito sync` command and keep the CLI layer thin.
  Alternatives considered: keep scattered best-effort fetch hooks only; add sync as a hidden helper instead of a user-facing command.
  Rationale: the current behavior is fragmented, while a first-class command gives both humans and skills one consistent entry point.

- Decision: Implement sync orchestration in `ito-core` using existing coordination helpers for path resolution, auto-commit, and git operations.
  Alternatives considered: keep separate implementations in each CLI call site.
  Rationale: reuse reduces drift and makes future sync touchpoints call the same core behavior.

- Decision: Treat sync validation as stricter than the current health check by requiring each `.ito/` path to resolve to the expected coordination-worktree target, not just any existing target.
  Alternatives considered: continue accepting any existing symlink target.
  Rationale: the user request explicitly calls out preventing drift and duplication, which the current existence-only check does not fully cover.

- Decision: Apply the quiet window only to redundant remote pushes, not to local validation, and expose `ito sync --force` to bypass the quiet window when needed.
  Alternatives considered: skip the entire sync command during the quiet window; omit a bypass flag and force users to wait for the interval.
  Rationale: local validation is cheap and still protects against drift, while suppressing only redundant pushes avoids remote spam without hiding local breakage; `--force` keeps recovery and explicit handoff cases straightforward.

- Decision: Store the last successful sync timestamp and synchronized state fingerprint outside committed `.ito/` artifacts, preferably in repo-local git metadata shared by sibling worktrees.
  Alternatives considered: store the timestamp in tracked `.ito/` files; keep the state only in process memory.
  Rationale: tracked state would create churn and duplicate commits, while process-local state would not rate-limit across repeated CLI/skill invocations.

- Decision: Add `changes.coordination_branch.sync_interval_seconds` as the configuration key for the quiet window and default it to 120 seconds.
  Alternatives considered: hard-code the interval permanently; place the setting under an unrelated top-level sync section.
  Rationale: the setting belongs with coordination-branch behavior, and a 120-second default is conservative enough to reduce Git server noise while still keeping coordination reasonably fresh.

- Decision: Fail on non-fast-forward/diverged remote coordination history with actionable guidance instead of attempting an automatic merge.
  Alternatives considered: implicit merge or rebase during `ito sync`.
  Rationale: sync will run frequently and non-interactively from skills, so it should stay deterministic and safe.

- Decision: Treat worktree-mode archive as a two-stage lifecycle: coordination archive first, `main` integration second.
  Alternatives considered: archive only on the coordination branch; archive only on `main`; dual-write coordination and `main` in one undifferentiated step.
  Rationale: coordination-first archive makes the archived result visible to all worktrees quickly, while keeping `main` as the final canonical history after a distinct integration step.

- Decision: Add `changes.archive.main_integration_mode` with values `direct_merge`, `pull_request`, `pull_request_auto_merge`, and `coordination_only`, defaulting to `pull_request`.
  Alternatives considered: hard-code a single archive integration behavior; model auto-merge as a separate boolean; omit a manual/coordination-only escape hatch.
  Rationale: the enum keeps archive follow-up deterministic for agents, `pull_request` is the safest default, and `coordination_only` remains useful when teams want dissemination without immediate `main` integration.

- Decision: Treat `ito finish` as the point where the workflow asks whether to archive now.
  Alternatives considered: always archive during finish; keep archive entirely separate from finish.
  Rationale: the user wants finish to be the natural place for the decision, but not an absolute rule; prompting during finish keeps lag low without making archive mandatory.

- Decision: Keep CLI-generated instruction templates as the source of truth for sync/archive/finish workflows.
  Alternatives considered: let skills or wrapper commands carry their own workflow logic.
  Rationale: templated CLI instructions can be tested and validated centrally, while skills remain thin wrappers that do not drift from the canonical workflow.

## Risks / Trade-offs

- Diverged remote coordination history blocks automation -> Return a clear repair path and avoid implicit merges.
- Overly broad rate limiting could delay legitimate pushes -> Key suppression to unchanged coordination state, not just wall-clock time.
- Mis-recorded sync metadata could suppress a needed push -> Only update the sync record after a successful push completes.
- Exact-target validation could flag previously tolerated but wrong symlinks -> That is intentional, but the error text needs to explain the expected target clearly.
- Skill integration could become noisy if too many touchpoints call sync -> Limit call sites to mutation and handoff boundaries, and rely on the quiet window.
- Direct-merge archive integration could be too aggressive for some repos -> Keep `pull_request` as the default and make direct merge explicit opt-in.
- Auto-merge PR mode may fail under repository policy -> Treat auto-merge as a requested follow-up, not a bypass of branch protection or review requirements.
- Finish-time archive prompting can still leave dissemination lag when the user declines -> Make the prompt explicit about the trade-off and show how to archive later with `ito-archive`.

## Migration Plan

1. Add the `cli-sync` command surface and core sync orchestration.
2. Extend coordination validation to distinguish exact-target mismatches from merely existing targets.
3. Add coordination-first archive lifecycle support and archive integration-mode configuration.
4. Update the existing create/tasks/instruction/archive/finish call sites to use the shared sync helper and archive policy where appropriate.
5. Add sync, archive, and finish guidance to the relevant CLI instruction templates and mirrored skill wrappers.
6. Verify the new command, archive policy configuration, and updated call sites with unit and integration tests before rollout.

## Open Questions

- None for proposal scope; the change now assumes a configurable sync interval with a 120-second default and a configurable archive integration mode with `pull_request` as the default.
<!-- ITO:END -->
