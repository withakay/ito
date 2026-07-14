<!-- ITO:START -->
## ADDED Requirements

### Requirement: Proposal integration mode is explicit and safe by default

Ito SHALL expose `changes.proposal.integration_mode` as a typed configuration value with `pull_request` and `direct_merge` as the only accepted values. The value SHALL default to `pull_request`; `direct_merge` MUST require an explicit repository configuration choice.

- **Requirement ID**: main-first-implementation:proposal-integration-mode

#### Scenario: Pull request is the default

- **WHEN** a repository does not configure `changes.proposal.integration_mode`
- **THEN** Ito uses `pull_request`
- **AND** the tracked upstream target branch is the authoritative proposal ref

#### Scenario: Direct merge is explicitly selected

- **WHEN** a repository configures `changes.proposal.integration_mode` as `direct_merge`
- **THEN** Ito uses the local target branch as the authoritative proposal ref
- **AND** Ito does not require a pull-request remote-tracking ref

#### Scenario: Unsupported mode is rejected

- **WHEN** configuration contains any other proposal integration mode
- **THEN** Ito rejects the configuration with the allowed values and configuration path

### Requirement: Readiness uses an immutable authority snapshot

For each readiness evaluation, Ito SHALL resolve the mode-specific authoritative target ref once to an immutable commit OID and SHALL use that OID for every artifact, validation, integration-commit, ancestry, and worktree-base decision made by that evaluation.

- **Requirement ID**: main-first-implementation:immutable-authority-snapshot

#### Scenario: Pull-request authority is resolved

- **WHEN** readiness runs in `pull_request` mode
- **THEN** Ito resolves the configured target branch's tracked upstream ref to an OID
- **AND** the report identifies both the upstream ref and resolved OID

#### Scenario: Direct-merge authority is resolved

- **WHEN** readiness runs in `direct_merge` mode
- **THEN** Ito resolves the local target branch ref to an OID
- **AND** the report identifies both the local ref and resolved OID

#### Scenario: Authority cannot be established

- **WHEN** the mode-specific ref is absent, ambiguous, or cannot be resolved to a commit
- **THEN** readiness fails before implementation-side effects
- **AND** the failure explains which ref must be fetched, configured, or integrated

#### Scenario: Ref moves during evaluation

- **WHEN** the authoritative ref moves after readiness has resolved its OID
- **THEN** the in-progress evaluation continues to use the original OID consistently
- **AND** a later evaluation resolves a new snapshot

### Requirement: Prepare readiness is proven from authoritative Git contents

The centralized readiness service SHALL provide a `prepare` phase. It SHALL pass only when the change exists at the authority OID, every artifact required by the change's declared schema can be read from that Git tree, those tree contents pass strict change validation, and a proposal integration commit is reachable from the authority OID. Integration through the configured mode is the acceptance signal that a proposal has completed review before an explicit apply request.

- **Requirement ID**: main-first-implementation:prepare-readiness

#### Scenario: Integrated proposal is ready to prepare

- **WHEN** the authoritative Git tree contains a schema-valid change and all schema-required proposal artifacts
- **AND** the target history contains a commit that introduced `.ito/changes/<change-id>/.ito.yaml`
- **THEN** `prepare` readiness passes
- **AND** the result records the proposal integration commit

#### Scenario: Explicit apply is requested before integration

- **WHEN** an apply-instruction surface is requested for a proposal that is absent from the authoritative Git tree
- **THEN** `prepare` readiness fails
- **AND** Ito directs the user to review and integrate the proposal through the configured integration mode

#### Scenario: Authoritative proposal is incomplete or invalid

- **WHEN** a schema-required artifact is missing from the authority OID or the authoritative tree fails strict validation
- **THEN** `prepare` readiness fails with the missing paths or validation findings
- **AND** working-tree copies of those artifacts do not change the result

### Requirement: Execute readiness proves proposal ancestry and checkout identity

The centralized readiness service SHALL provide an `execute` phase that includes every `prepare` condition. It SHALL additionally require the current `HEAD` to descend from the proposal integration commit and require the current branch/worktree to be associated with the requested change rather than the authoritative target/control checkout.

- **Requirement ID**: main-first-implementation:execute-readiness

#### Scenario: Implementation descends from accepted proposal

- **WHEN** the current implementation `HEAD` contains the proposal integration commit in its ancestry
- **AND** the checkout is associated with the requested change under the configured worktree strategy
- **THEN** `execute` readiness passes

#### Scenario: Proposal files were copied without ancestry

- **WHEN** the current checkout contains local or committed copies of every proposal artifact
- **BUT** the proposal integration commit is not an ancestor of `HEAD`
- **THEN** `execute` readiness fails
- **AND** Ito instructs the user to recreate, rebase, or merge the implementation branch from authoritative `main`

#### Scenario: Execution is attempted from target or control checkout

- **WHEN** execution is attempted from the authoritative target branch or a read-only control checkout
- **THEN** `execute` readiness fails before tasks, harnesses, agents, or setup commands run

### Requirement: Worktrees preserve the verified authority boundary

Ito SHALL create a new implementation worktree from the authority snapshot OID returned by successful `prepare` readiness. Existing worktrees SHALL pass `execute` readiness before setup or implementation continues, and generated apply guidance MUST use the guarded Ito worktree path rather than an unguarded manual worktree command.

- **Requirement ID**: main-first-implementation:verified-worktree-base

#### Scenario: New implementation worktree is created

- **WHEN** `ito worktree ensure` prepares a change with no existing implementation worktree
- **THEN** it creates the change branch/worktree from the verified authority OID
- **AND** the resulting `HEAD` contains the proposal integration commit

#### Scenario: Existing worktree predates proposal integration

- **WHEN** `ito worktree ensure` or `ito worktree setup` finds an existing worktree whose `HEAD` does not descend from the integration commit
- **THEN** Ito rejects continued setup or implementation with ancestry remediation

#### Scenario: Apply guidance is rendered

- **WHEN** Ito renders apply instructions for a worktree-enabled repository
- **THEN** the instructions invoke the guarded Ito worktree workflow
- **AND** they do not provide a manual `wt switch --create` bypass

### Requirement: Every implementation entry point enforces the central gate

Ito SHALL call the same readiness service at all in-scope implementation entry points. Apply-instruction generation, `ito list --ready`, and new-worktree preparation SHALL enforce `prepare`; worktree setup or reuse, task start/complete, Ralph or loop execution, and orchestration dispatch/resume SHALL enforce `execute`. A failing gate MUST stop the entry point before it mutates task state, creates or configures a worktree, launches a harness or agent, or dispatches work.

- **Requirement ID**: main-first-implementation:entrypoint-enforcement

#### Scenario: Ready listing uses prepare readiness

- **WHEN** a user requests `ito list --ready`
- **THEN** Ito includes only changes whose centralized `prepare` evaluation passes at an authority snapshot
- **AND** local artifact completeness alone is insufficient

#### Scenario: Task state mutation is gated

- **WHEN** `ito tasks start` or `ito tasks complete` is requested in a checkout that fails `execute`
- **THEN** Ito leaves task persistence unchanged
- **AND** returns the shared readiness failure

#### Scenario: Iteration is gated

- **WHEN** Ralph or an Ito loop is requested for a change that fails `execute`
- **THEN** no iteration, harness process, commit automation, or task mutation begins

#### Scenario: Orchestration is gated on every dispatch path

- **WHEN** orchestration starts, resumes, or is about to dispatch implementation work for a change
- **THEN** it evaluates `execute` before dispatch
- **AND** a failed evaluation is recorded as a blocked gate rather than delegated to a worker

### Requirement: Readiness is inspectable and actionable

Ito SHALL expose the centralized evaluation through `ito change preflight <change-id> --for prepare|execute`, with optional authority refresh and JSON output. Text and JSON results SHALL report phase, status, integration mode, authority ref, authority OID when resolved, proposal integration commit when found, failed conditions, and remediation. A failed preflight SHALL return a non-zero exit status.

- **Requirement ID**: main-first-implementation:readiness-reporting

#### Scenario: Successful JSON preflight

- **WHEN** a caller requests JSON for a passing preflight
- **THEN** Ito emits one machine-readable readiness report containing the verified OIDs and conditions
- **AND** exits successfully

#### Scenario: Failed JSON preflight

- **WHEN** a caller requests JSON for a failing preflight
- **THEN** Ito emits the same stable report shape with failed conditions and remediation
- **AND** exits non-zero without mixing prose into standard output

#### Scenario: Authority refresh is requested

- **WHEN** a caller supplies `--refresh` in `pull_request` mode
- **THEN** Ito refreshes only the configured upstream target ref before resolving the authority OID
- **AND** a refresh failure prevents a readiness claim
<!-- ITO:END -->
