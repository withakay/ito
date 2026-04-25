<!-- ITO:START -->
## Context

Ito already renders worktree-aware instructions from configuration and installs worktree-related skill guidance. The current guidance focuses on where worktrees live and how to create them, but it does not make fresh per-change worktrees a clear invariant across proposal and apply phases.

## Goals / Non-Goals

**Goals:**

- Make worktree-enabled guidance consistently direct agents away from main/control checkouts for change work.
- Align branch and worktree names with the full change ID so the filesystem, branch list, and `.ito/changes/<id>` match.
- Preserve support for multiple worktrees for one change through explicit full-change-ID prefixes.

**Non-Goals:**

- Add runtime enforcement or hook-based blocking. That is covered by the follow-on change `012-07_guard-opencode-worktree-path`.
- Change configured worktree layout strategies.
- Require worktrees when `worktrees.enabled=false`.

## Decisions

- Decision: Put the rule set in generated instruction artifacts and installed skill templates, not only in `.ito/AGENTS.md`. This ensures proposal, apply, and skill-driven flows all receive the same guidance.
- Decision: Use the full change ID as the canonical branch/worktree stem. This preserves module and sub-module context and makes worktree names auditable by inspection.
- Decision: Permit extra same-change worktrees only with a full-change-ID prefix plus suffix. This avoids over-constraining legitimate review or experiment worktrees while preventing ambiguity across changes.

## Risks / Trade-offs

- Risk: Existing tests may assert older “create or reuse” phrasing. Mitigation: update tests to assert “fresh per change” semantics where worktrees are enabled.
- Risk: Guidance could become too verbose. Mitigation: keep the invariant in a concise reusable section and reference it from relevant artifacts.
<!-- ITO:END -->
