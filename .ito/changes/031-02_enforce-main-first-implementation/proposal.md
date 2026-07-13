<!-- ITO:START -->
## Why

Ito can currently begin implementation from proposal files that exist only in a coordination worktree or an implementation branch. That makes proposal IDs vulnerable to collisions, lets reviewed intent remain absent from `main`, and allows copied local artifacts to look ready even when the implementation branch never incorporated the accepted proposal. The workflow needs a single, enforceable hand-off: review and integrate the proposal into authoritative `main`, then create or continue implementation from that integrated Git history.

## What Changes

- Add `changes.proposal.integration_mode` with `pull_request` as the default and `direct_merge` as an explicit opt-in. Pull-request integration treats the tracked upstream target branch as authoritative; direct-merge integration treats the local target branch as authoritative.
- Add one centralized readiness service with `prepare` and `execute` phases. `prepare` proves that the reviewed proposal is present and valid in authoritative `main` before an explicit apply request or implementation worktree is prepared. `execute` additionally proves that the current implementation checkout descends from the proposal's integration commit.
- Resolve the authoritative target ref to one immutable commit OID per readiness evaluation. Read the schema-required proposal artifacts from that Git tree, not from the working tree or legacy coordination storage, and identify the target-reachable commit that introduced the change.
- Expose the shared result through an inspectable preflight command and enforce it at apply-instruction generation, `ito list --ready`, worktree ensure/setup, task start/complete, Ralph and loop execution, and orchestration dispatch/resume.
- Create implementation worktrees from the verified authority OID. Remove generated guidance that can bypass Ito's worktree/readiness path with a manual `wt switch` command.
- Return actionable text and structured JSON failures before any implementation-side effect. Merely copying proposal files into a branch MUST NOT satisfy readiness or ancestry checks.

## Change Shape

- **Type**: contract
- **Risk**: high
- **Stateful**: yes
- **Public Contract**: cli, config
- **Design Needed**: yes
- **Design Reason**: The change establishes Git authority, ancestry, and enforcement semantics shared by multiple lifecycle entry points.

## Capabilities

### New Capabilities

- `main-first-implementation`: Define proposal integration modes, immutable Git authority snapshots, prepare/execute readiness, and mandatory enforcement across implementation entry points.

### Modified Capabilities

- None. Existing commands consume the new cross-cutting readiness capability without redefining their existing domain contracts.

## Impact

- Configuration: typed config, defaults, JSON schema, generated examples, and documentation for `changes.proposal.integration_mode`.
- Core: Git authority resolution, tree-based artifact loading/validation, proposal integration-commit discovery, ancestry checks, worktree identity checks, and reusable readiness reports.
- CLI: a readiness preflight surface plus apply, list, worktree, task, Ralph/loop, and orchestration enforcement.
- Templates and managed skills: main-first lifecycle guidance and removal of manual worktree-creation bypasses.
- Tests: configuration defaults, Git-history fixtures, copied-artifact rejection, side-effect ordering, entry-point coverage, and pull-request/direct-merge behavior.
<!-- ITO:END -->
