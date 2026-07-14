<!-- ITO:START -->
## Context

Ito's current readiness signals are distributed. Change summaries infer “ready” from local artifact/task state, apply instructions can render from coordination-worktree contents, worktree creation starts from a mutable branch name, and task, Ralph, and orchestration paths each make their own assumptions. A proposal can therefore be locally complete but absent from `main`, or an implementation branch can contain copied files without containing the reviewed proposal's integration history.

Change `031-01_migrate-coordination-state-to-main` provides the migration boundary for repositories with legacy coordination storage. This change defines the lifecycle after tracked main-branch artifacts are authoritative: proposal review/integration reserves the change ID and establishes accepted intent before implementation begins.

## Goals / Non-Goals

**Goals:**

- Make proposal integration into authoritative `main` a machine-enforced prerequisite for apply and implementation.
- Support pull-request and direct-merge repositories while making pull requests the safe default.
- Centralize readiness so every implementation entry point reaches the same answer.
- Bind each answer to immutable Git OIDs and authoritative Git-tree contents.
- Reject copied/stale artifacts and branches that predate proposal integration.
- Preserve Ito's Ralph/loop iteration features after the main-first hand-off.

**Non-Goals:**

- Hosting-provider API review checks, required-approver policy, or automatic pull-request creation/merge.
- Replacing the migration and compatibility behavior owned by `031-01`.
- Removing the backend allocation mechanism or changing how concurrent numeric IDs are allocated.
- Automatically rebasing, merging, or rewriting an existing implementation branch.
- Gating proposal authoring, proposal validation, read-only inspection other than `list --ready`, or archive operations.

## Approach

Add a core `implementation_readiness` service with two phases:

| Phase | Required proof | Primary consumers |
| --- | --- | --- |
| `prepare` | Mode-specific authority resolves; schema-required artifacts exist and strictly validate in that Git tree; target history contains the proposal integration commit | Apply instructions, `list --ready`, new worktree ensure, standalone preflight |
| `execute` | All `prepare` proof; integration commit is an ancestor of current `HEAD`; checkout belongs to the change and is not target/control | Existing worktree setup/reuse, task start/complete, Ralph/loop, orchestration dispatch/resume |

The service returns a `ReadinessReport`, not a boolean. Consumers may format it for humans, serialize it, or translate it into an orchestration gate record, but MUST NOT reimplement readiness rules.

Evaluation order is deliberately side-effect-safe:

1. Load typed configuration and resolve the requested change ID.
2. Select the authoritative ref from integration mode.
3. Optionally refresh the single upstream target ref when explicitly requested.
4. Resolve the ref once to `AuthoritySnapshot.oid`.
5. Inspect and strictly validate schema-required artifacts from that Git tree.
6. Find the target-reachable commit that introduces the change marker.
7. For `execute`, check ancestry and checkout identity.
8. Return the report; only a successful caller may perform its intended side effect.

The integration commit is the commit reachable from the authority OID that first introduces `.ito/changes/<change-id>/.ito.yaml`. This works for merge commits, squash merges, and direct commits because discovery is performed in authoritative target history rather than proposal-branch history. The commit OID is evidence that accepted intent is part of `main`; files copied or independently committed elsewhere cannot reproduce that ancestry.

## Contracts / Interfaces

### Configuration

```json
{
  "changes": {
    "proposal": {
      "integration_mode": "pull_request"
    }
  }
}
```

`ProposalIntegrationMode` is a typed enum with `PullRequest` and `DirectMerge`. Its default is `PullRequest`. Existing repositories inherit the default; repositories intentionally integrating straight into a local target branch opt into `direct_merge`.

Authority selection is:

| Mode | Authoritative ref | Acceptance meaning |
| --- | --- | --- |
| `pull_request` | Configured target branch's tracked upstream ref, normally `refs/remotes/origin/main` | Integration into the reviewed upstream target is accepted intent |
| `direct_merge` | Local configured target branch, normally `refs/heads/main` | Direct integration into the explicitly opted-in target is accepted intent |

There is no fallback from missing pull-request authority to local `main`, a coordination branch, or working-tree files.

### Core types

- `ReadinessPhase::{Prepare, Execute}`
- `AuthoritySnapshot { integration_mode, target_ref, oid }`
- `ReadinessCondition { code, passed, message, remediation }`
- `ReadinessReport { change_id, phase, ready, authority, proposal_integration_oid, conditions }`
- `ReadinessRequest { change_id, phase, refresh_authority, current_checkout }`

The core API accepts Git/config/change-schema abstractions so history and tree behavior can be tested with fixture repositories without shelling through CLI handlers.

### CLI

`ito change preflight <change-id> --for prepare|execute [--refresh] [--json]` is the diagnostic surface. Text output is concise and actionable. JSON output writes only the stable report object to stdout, sends incidental diagnostics to stderr, and exits non-zero when `ready` is false.

Existing entry points call the service with a fixed phase rather than asking users to select one. Apply includes both direct artifact rendering and manifesto/apply routing. Ralph includes direct Ralph commands and generated loop surfaces. Orchestration checks before initial dispatch, resumed dispatch, and any later worker remediation packet that would execute the change.

### Artifact loading and validation

The loader starts with `.ito/changes/<change-id>/.ito.yaml` at the authority OID, resolves the declared schema, and enumerates that schema's required apply prerequisites. It reads those paths with Git tree APIs (equivalent to `<oid>:<path>`), preserving the authority OID throughout validation. It MUST NOT consult symlink targets, the current working directory, or the legacy coordination store to fill missing authoritative paths.

Strict validation runs against the captured tree contents. Diagnostics retain artifact paths and validator codes so CLI and orchestration consumers can render the same failure.

## Data / State

Readiness creates no durable database record. `AuthoritySnapshot` and `ReadinessReport` are immutable values scoped to one evaluation. A moving branch ref does not alter an in-flight decision; the next evaluation observes a new OID.

The only optional pre-evaluation mutation is `--refresh` in pull-request mode, limited to fetching the configured upstream target ref. If refresh fails, no authority snapshot is claimed. Normal entry-point calls may use the already available authoritative ref and give remediation that recommends the explicit refreshed preflight when it appears stale.

Worktree creation consumes the successful report's OID directly instead of resolving the target branch name again. After creation, the worktree is checked as an `execute` checkout before setup commands run. An existing worktree is never silently repaired: Ito reports whether it must be recreated, rebased, or merged.

## Decisions

### Pull request is the default, direct merge is opt-in

Pull-request integration gives review and a shared upstream point of truth. Direct merge remains useful for local or deliberately lightweight repositories, but selecting it is an explicit statement that integration into local `main` is the acceptance boundary.

### Git history is the acceptance proof

No new “approved” flag is introduced. Such a flag could drift from Git or be copied with the proposal. Integration into the configured authoritative target is the review/acceptance signal, and the target-reachable introduction commit binds that signal to history.

### Artifact authority and ancestry are separate checks

`prepare` proves what `main` accepted. `execute` proves the implementation branch incorporated that acceptance. Both are required because a valid authoritative proposal does not repair a branch created too early, and complete local files do not prove authoritative acceptance.

### One service owns all gates

The list, apply, worktree, tasks, Ralph, and orchestration paths receive a shared report. This prevents command-specific definitions of “ready” and makes future implementation entry points easy to audit.

### Gate before side effects

Callers perform readiness before task persistence, worktree creation/setup, harness launch, agent dispatch, or commit automation. Failure behavior can therefore be tested as an unchanged-state invariant.

## Risks / Trade-offs

- Pull-request mode depends on an available upstream tracking ref. The failure is intentionally conservative and includes refresh/configuration remediation.
- Large repositories may make repeated tree validation and history discovery expensive. Cache only by immutable authority OID, change ID, schema version, and phase; never cache by mutable branch name alone.
- Requiring proposal ancestry exposes implementation branches created before proposal integration. The tool reports remediation but does not rewrite user history.
- A direct commit to upstream `main` satisfies Git integration in pull-request mode even if the host did not enforce review. Hosting-provider review attestation is deliberately outside this change.
- Central gating touches many entry points. A single core conformance suite plus per-entry-point “no side effect on failure” tests limits behavioral drift.

## Verification Strategy

- Typed-config tests cover defaults, explicit values, invalid values, cascading overrides, schema, and generated documentation/examples.
- Temporary Git repository tests cover pull-request and direct-merge refs, merge and squash histories, moving refs, missing upstreams, missing/invalid tree artifacts, and integration-commit discovery.
- Execute tests cover correct ancestry, pre-integration branches, copied local/committed artifacts, target/control checkout rejection, and accepted suffixed worktrees associated with the same change.
- CLI snapshot/integration tests cover text/JSON preflight shape and non-zero failures.
- Each wired entry point gets a failing-readiness test that proves no task, worktree setup, harness, agent, or orchestration dispatch side effect occurred.
- Worktree end-to-end tests prove creation starts from the captured OID even when the target ref moves after evaluation.
- Template tests prove apply guidance uses the Ito worktree command and contains no manual `wt switch --create` escape hatch.
- Run focused crate tests, `ito validate 031-02_enforce-main-first-implementation --strict`, and the repository's `make check` workflow; capture the end-to-end lifecycle in a Showboat demo.

## Migration / Rollback

Roll out after `031-01` has made tracked main-branch proposal artifacts authoritative. Existing repositories receive `pull_request`; repositories without an upstream PR workflow must set `changes.proposal.integration_mode` to `direct_merge` before applying changes.

Release notes must tell users to integrate open proposals into the configured target and recreate/rebase implementation branches that predate integration. Legacy coordination content remains readable only under `031-01`'s compatibility rules and is never accepted as readiness evidence.

Rollback can remove entry-point enforcement and the preflight command while retaining the config field as a tolerated no-op for one compatibility window. No readiness database or artifact migration must be reversed.

## Open Questions

None.
<!-- ITO:END -->
