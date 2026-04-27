<!-- ITO:START -->
## Context

The existing OpenCode Ito plugin already runs fast pre-tool checks for audit drift and includes some worktree awareness. The desired behavior is stronger: before change work happens, OpenCode should quickly confirm that the session is not in main/control and, when an active change ID is known, that the branch or path visibly maps to that change.

## Goals / Non-Goals

**Goals:**

- Add a CLI command that plugin hooks and humans can run to validate the current worktree for a change.
- Keep the guard fast enough for pre-tool use through short TTL caching and simple git/config checks.
- Treat main/control checkout detection as the most important failure.
- Treat change ID branch/path mismatches as a useful guard without banning same-change suffix worktrees.

**Non-Goals:**

- Build a full policy engine for all tool calls.
- Require perfect active-change inference in every OpenCode session.
- Enforce this guard for non-OpenCode adapters in this follow-on change.

## Decisions

- Decision: Implement validation as a CLI command first, then have `ito-skills.js` shell out to it. This keeps the policy testable outside OpenCode and avoids duplicating config/path logic in JavaScript.
- Decision: Make main/control detection the hard guard. Change ID mismatch checks can be warning or blocking based on CLI output and plugin policy, but the command must clearly identify mismatches.
- Decision: Accept branch/path values that contain or start with the full change ID, so `012-07_guard-opencode-worktree-path-review` remains valid for same-change review work.
- Decision: Use JSON output for hook callers and concise text output for humans.

## Risks / Trade-offs

- Risk: Hook overhead can slow agent tool use. Mitigation: cache successful validation for a short TTL and avoid expensive scans.
- Risk: Active change ID inference can be incomplete. Mitigation: support explicit change ID input and avoid blocking solely on an unknown change when already outside main/control.
- Risk: Overly strict matching could block legitimate same-change sub-worktrees. Mitigation: allow full-change-ID prefixes plus suffixes.
<!-- ITO:END -->
