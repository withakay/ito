<!-- ITO:START -->
# Tasks for: 031-02_enforce-main-first-implementation

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates after the proposal is integrated into authoritative `main` and the implementation worktree passes readiness.
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 031-02_enforce-main-first-implementation
ito tasks next 031-02_enforce-main-first-implementation
ito tasks start 031-02_enforce-main-first-implementation 1.1
ito tasks complete 031-02_enforce-main-first-implementation 1.1
```

______________________________________________________________________

## Wave 1: Core authority and readiness

- **Depends On**: None

### Task 1.1: Add proposal integration configuration

- **Files**: `ito-rs/crates/ito-config/src/config/types.rs`, config schema/default/example generation and associated tests/docs
- **Dependencies**: None
- **Action**: Add typed `changes.proposal.integration_mode`, default it to `pull_request`, accept only `pull_request`/`direct_merge`, and update cascading-config, JSON-schema, generated example, and documentation coverage.
- **Verify**: Run the focused `ito-config` test suite through the repository test-runner workflow and inspect generated schema/example diffs.
- **Done When**: Defaults, explicit modes, invalid values, and cascading overrides are covered and public config artifacts agree.
- **Requirements**: main-first-implementation:proposal-integration-mode
- **Updated At**: 2026-07-13
- **Status**: [x] complete

### Task 1.2: Implement immutable authority resolution

- **Files**: `ito-rs/crates/ito-core/src/implementation_readiness.rs`, sibling test module, Git/config repository adapters
- **Dependencies**: Task 1.1
- **Action**: Introduce readiness request/report types and resolve pull-request upstream or direct-merge local authority exactly once to an OID. Add narrowly scoped upstream refresh support and structured failures without fallback authority.
- **Verify**: Run focused core tests against temporary repositories covering both modes, missing refs, refresh failure, and a ref that moves after snapshot creation.
- **Done When**: Every report names its authority ref/OID consistently and all downstream operations consume the captured OID.
- **Requirements**: main-first-implementation:immutable-authority-snapshot, main-first-implementation:readiness-reporting
- **Updated At**: 2026-07-13
- **Status**: [x] complete

### Task 1.3: Implement authoritative tree validation and integration-commit discovery

- **Files**: `ito-rs/crates/ito-core/src/implementation_readiness.rs`, schema/artifact validation adapters, sibling tests
- **Dependencies**: Task 1.2
- **Action**: Load `.ito.yaml` and schema-required proposal artifacts from the authority Git tree, run strict validation on captured contents, and locate the target-reachable commit that introduces the change marker. Never fill gaps from the working tree, symlinks, backend, or coordination store.
- **Verify**: Run focused core tests for complete, missing, invalid, merge, squash, direct-commit, legacy-coordination, and copied-working-tree fixtures.
- **Done When**: `prepare` passes only for strictly valid authoritative artifacts and reports the integration commit.
- **Requirements**: main-first-implementation:prepare-readiness, main-first-implementation:immutable-authority-snapshot
- **Updated At**: 2026-07-13
- **Status**: [x] complete

### Task 1.4: Implement execute ancestry and checkout checks

- **Files**: `ito-rs/crates/ito-core/src/implementation_readiness.rs`, worktree/checkout identity helpers, sibling tests
- **Dependencies**: Task 1.3
- **Action**: Layer `execute` on `prepare`, require integration-commit ancestry, associate the current branch/worktree with the full change ID under configured strategies, and reject target/control checkouts and copied-file bypasses.
- **Verify**: Run focused core tests for post-integration worktrees, pre-integration branches, copied committed/uncommitted artifacts, suffixed change worktrees, and target/control checkouts.
- **Done When**: Execute readiness is deterministic, structured, and side-effect free across the checkout matrix.
- **Requirements**: main-first-implementation:execute-readiness
- **Updated At**: 2026-07-13
- **Status**: [x] complete

______________________________________________________________________

## Wave 2: CLI and lifecycle enforcement

- **Depends On**: Wave 1

### Task 2.1: Expose readiness preflight and update ready listing

- **Files**: `ito-rs/crates/ito-cli/src/cli.rs`, new CLI app handler/tests, `ito-rs/crates/ito-cli/src/app/list.rs`, `ito-rs/crates/ito-core/src/list.rs`, snapshots
- **Dependencies**: None
- **Action**: Add `ito change preflight <change-id> --for prepare|execute [--refresh] [--json]`; define clean text/JSON/non-zero behavior; change `ito list --ready` to consume centralized `prepare` reports instead of local artifact/task completeness.
- **Verify**: Run focused CLI preflight and list snapshot/integration tests, including stdout/stderr separation and copied-local-artifact exclusion.
- **Done When**: Humans and automation can inspect the same report used by `list --ready`.
- **Requirements**: main-first-implementation:readiness-reporting, main-first-implementation:entrypoint-enforcement
- **Updated At**: 2026-07-13
- **Status**: [x] complete

### Task 2.2: Gate apply rendering and worktree lifecycle

- **Files**: `ito-rs/crates/ito-cli/src/app/instructions.rs`, `ito-rs/crates/ito-cli/src/app/manifesto_instructions.rs`, `ito-rs/crates/ito-core/src/worktree_ensure.rs`, `ito-rs/crates/ito-cli/src/commands/worktree.rs`, `ito-rs/crates/ito-templates/assets/instructions/agent/apply.md.j2`, worktree/apply tests
- **Dependencies**: None
- **Action**: Require `prepare` before all apply-instruction paths and new worktree creation; create from the report OID; require `execute` before existing-worktree reuse/setup; remove manual `wt switch --create` guidance.
- **Verify**: Run focused apply/template/worktree tests and end-to-end tests where the target ref moves after readiness and an old worktree predates proposal integration.
- **Done When**: No apply/setup path bypasses the central gate and worktree creation uses the captured OID.
- **Requirements**: main-first-implementation:prepare-readiness, main-first-implementation:verified-worktree-base, main-first-implementation:entrypoint-enforcement
- **Updated At**: 2026-07-13
- **Status**: [ ] pending

### Task 2.3: Gate task mutations

- **Files**: `ito-rs/crates/ito-cli/src/commands/tasks.rs`, task repository adapters and CLI tests
- **Dependencies**: None
- **Action**: Evaluate `execute` before `tasks start` and `tasks complete` for implementation changes, preserving current state on failure and returning the shared report/remediation.
- **Verify**: Run focused filesystem and backend task tests that snapshot persistence before and after failed readiness.
- **Done When**: Failed readiness produces zero local or remote task-state mutations.
- **Requirements**: main-first-implementation:execute-readiness, main-first-implementation:entrypoint-enforcement
- **Updated At**: 2026-07-13
- **Status**: [ ] pending

### Task 2.4: Gate Ralph and loop execution

- **Files**: `ito-rs/crates/ito-cli/src/commands/ralph.rs`, `ito-rs/crates/ito-cli/src/commands/ralph/`, `ito-rs/crates/ito-core/src/ralph/`, loop/Ralph managed skill and command templates, tests
- **Dependencies**: None
- **Action**: Evaluate `execute` before resolving/running iterations, launching a harness, mutating task state, or enabling Git automation for every change-scoped Ralph/loop surface.
- **Verify**: Run Ralph smoke/runtime tests with a failing pre-integration branch and assert no harness launch, task mutation, iteration record, or commit.
- **Done When**: Iteration remains available after readiness but cannot start before main-first ancestry is established.
- **Requirements**: main-first-implementation:execute-readiness, main-first-implementation:entrypoint-enforcement
- **Updated At**: 2026-07-13
- **Status**: [ ] pending

### Task 2.5: Gate orchestration dispatch and resume

- **Files**: `ito-rs/crates/ito-core/src/orchestrate/`, orchestration CLI handlers, `ito-rs/crates/ito-templates/assets/instructions/agent/orchestrate.md.j2`, orchestrator skills/agents, tests
- **Dependencies**: None
- **Action**: Add readiness as a mandatory orchestration gate before initial, resumed, and remediation dispatch. Store the structured failure in run/gate state and do not create a worker packet when it fails.
- **Verify**: Run orchestration gate/run-state tests for start, resume, and repeated dispatch with passing and failing ancestry.
- **Done When**: Every implementation dispatch path records and enforces the same execute report.
- **Requirements**: main-first-implementation:execute-readiness, main-first-implementation:entrypoint-enforcement
- **Updated At**: 2026-07-13
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3: Distribution and end-to-end proof

- **Depends On**: Wave 2

### Task 3.1: Refresh managed guidance and public documentation

- **Files**: `docs/config.md`, lifecycle documentation, `ito-rs/crates/ito-templates/assets/commands/`, `ito-rs/crates/ito-templates/assets/skills/`, generated harness assets and snapshots
- **Dependencies**: None
- **Action**: Document proposal-only review/integration followed by implementation from authoritative main; document both modes, preflight remediation, iteration preservation, and migration sequencing; regenerate managed assets through repository tooling.
- **Verify**: Run template parity/snapshot tests and search distributed apply/worktree guidance to prove no manual `wt switch --create` bypass remains.
- **Done When**: CLI help, docs, skills, commands, and generated harness assets describe one consistent main-first lifecycle.
- **Requirements**: main-first-implementation:proposal-integration-mode, main-first-implementation:verified-worktree-base, main-first-implementation:readiness-reporting
- **Updated At**: 2026-07-13
- **Status**: [ ] pending

### Task 3.2: Prove the full lifecycle and side-effect boundaries

- **Files**: cross-crate integration tests and `demos/` under this change
- **Dependencies**: Task 3.1
- **Action**: Build end-to-end fixtures for proposal-only branch, upstream integration, OID-based implementation worktree, task/Ralph execution, orchestration resume, pre-integration rejection, copied-artifact rejection, and direct-merge opt-in. Capture the passing lifecycle and key rejection cases in a Showboat demo using real command output.
- **Verify**: Through the repository test-runner workflow, run focused cross-crate suites, `ito validate 031-02_enforce-main-first-implementation --strict`, and `make check`; dispatch the required Rust quality/review passes before completion.
- **Done When**: Tests and the executable demo prove all entry points share the gate, failures are mutation-free, both integration modes work, and the repository check passes.
- **Requirements**: main-first-implementation:proposal-integration-mode, main-first-implementation:immutable-authority-snapshot, main-first-implementation:prepare-readiness, main-first-implementation:execute-readiness, main-first-implementation:verified-worktree-base, main-first-implementation:entrypoint-enforcement, main-first-implementation:readiness-reporting
- **Updated At**: 2026-07-13
- **Status**: [ ] pending

______________________________________________________________________

## Wave Guidelines

- Complete and verify Wave 1 before wiring callers.
- Wave 2 tasks may proceed in parallel after the central service contract is stable.
- Complete managed distribution and end-to-end proof only after all entry points are gated.
- Keep exactly one task in progress per implementation worktree and use `ito tasks` for status transitions.
<!-- ITO:END -->
