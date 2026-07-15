<!-- ITO:START -->
# Ito Authority Cutover

## Purpose

This spec defines the dependency, preservation, authority, parity, guidance,
and release-readiness guarantees for making tracked `.ito` state on `main` the
sole writable Ito authority.

## Requirements

### Requirement: Cutover is gated by the preceding core-reset changes
Ito's repository authority migration MUST NOT begin until changes `031-01` through `031-05` have each been approved, implemented, strictly validated, verified by their required test evidence, and integrated as ancestors of one reviewed main-bound cutover branch. The cutover branch SHALL start from `main` and merge all five dependency implementations before any authority mutation begins.
- **Requirement ID**: ito-authority-cutover:dependency-gated-cutover
- **Tags**: behavior, stateful

#### Rules / Invariants
- A proposal draft, task-complete marker, or implementation commit not integrated into the cutover branch MUST NOT satisfy the dependency gate.
- The recorded readiness evidence MUST identify the integrated commit for every dependency.

#### Scenario: All dependency changes are integrated
- **GIVEN** changes `031-01` through `031-05` are approved and their implementation commits are integrated into the main-bound cutover branch
- **AND** strict validation and required checks pass for those integrated commits
- **WHEN** the `031-06` implementation begins
- **THEN** its authority mutation begins only after all five commits are ancestors of the branch created from `main`
- **AND** the dependency evidence records all five integrated commits

#### Scenario: A dependency is incomplete
- **GIVEN** any change from `031-01` through `031-05` is unapproved, not integrated into the cutover branch, invalid, or failing a required check
- **WHEN** the cutover readiness gate is evaluated
- **THEN** migration stops before snapshots, configuration changes, or filesystem replacement occur

### Requirement: External coordination state is snapshotted and preserved
Before replacing any managed link, the migration SHALL record the external coordination worktree path, branch, commit identity, managed symlink targets, complete relative-path inventories, and deterministic content hashes for `.ito/{changes,specs,modules,workflows,audit}`. The external coordination worktree, branch, and files MUST remain byte-for-byte unchanged throughout migration and review.
- **Requirement ID**: ito-authority-cutover:external-state-preservation
- **Tags**: behavior, stateful

#### Rules / Invariants
- Snapshot evidence MUST be captured before the first destination mutation.
- Broken links, unexpected link targets, missing managed paths, and destination collisions MUST be reported explicitly.
- No migration step may delete, rename, commit in, reset, push, or rewrite the external coordination checkout or branch.

#### Scenario: Healthy external state is snapshotted
- **GIVEN** the five managed repository paths resolve to the expected external coordination checkout
- **WHEN** migration evidence is captured
- **THEN** the evidence records the source Git identity, link metadata, file inventories, and content hashes
- **AND** a second independent hash pass can reproduce the recorded values

#### Scenario: Ambiguous or conflicting state is found
- **GIVEN** a managed path is missing, broken, unexpectedly targeted, or conflicts with a real destination directory
- **WHEN** the pre-copy inspection runs
- **THEN** migration stops and reports the exact discrepancy
- **AND** neither source nor destination content is deleted or overwritten

### Requirement: Tracked main state becomes the sole writable Ito authority
The migration SHALL replace the managed coordination links with real tracked `.ito/{changes,specs,modules,workflows,audit}` directories whose inventories and file hashes match the approved source snapshot, apart from documented repository sentinels required for Git to preserve an empty authority root. Repository configuration MUST set coordination storage to embedded and disabled, MUST keep backend mode disabled, and MUST remove tmux configuration and managed tmux assets. After reviewed integration, `main` SHALL be the sole writable Ito authority.
- **Requirement ID**: ito-authority-cutover:tracked-main-authority
- **Tags**: behavior, stateful

#### State Transitions
| From | Event | To | Notes |
| --- | --- | --- | --- |
| external-authoritative | snapshot source | prepared | Source remains unchanged |
| prepared | materialize and configure | reviewable | Real tracked directories match the snapshot |
| reviewable | merge reviewed cutover | main-authoritative | `main` becomes the writable authority |

#### Scenario: Materialized state matches the source
- **GIVEN** the external snapshot is complete and destination collision checks pass
- **WHEN** the five managed paths are copied into real repository directories
- **THEN** every copied relative path and content hash matches the snapshot
- **AND** any extra tracked sentinel is empty, non-semantic, and documented as a Git representation exception
- **AND** the directories are tracked rather than symlinked
- **AND** the external source still matches its pre-migration hashes

#### Scenario: Main-authoritative configuration is resolved
- **GIVEN** the cutover configuration is loaded
- **WHEN** Ito resolves persistence and tool settings
- **THEN** coordination is disabled with embedded storage
- **AND** backend mode is disabled
- **AND** no tmux setting or managed tmux asset is active

### Requirement: Published mirror is retired only after parity is proven
The migration MUST compare the materialized authoritative active changes, archived changes, and specs with the committed `docs/ito` mirror before removing the mirror. Every difference SHALL be explained as an intentional path/layout normalization or resolved before retirement; an unexplained missing or changed artifact MUST block removal.
- **Requirement ID**: ito-authority-cutover:mirror-parity-before-retirement
- **Tags**: behavior, stateful

#### Scenario: Mirror parity is established
- **GIVEN** tracked `.ito` state has been materialized from the approved source snapshot
- **WHEN** normalized inventories and hashes are compared with `docs/ito`
- **THEN** active changes, archived changes, and current specs have complete accounted-for parity
- **AND** the parity evidence is recorded before mirror files or configuration are removed

#### Scenario: Mirror content is unmatched
- **GIVEN** `docs/ito` contains an artifact or content difference not represented by the materialized state or an approved normalization rule
- **WHEN** mirror parity is checked
- **THEN** mirror retirement stops
- **AND** the unmatched content remains available for investigation

### Requirement: Canonical and generated guidance converges on main authority
The canonical specs, wiki sources, project guidance, instruction templates, user documentation, configuration schema, and generated harness assets SHALL consistently describe tracked `.ito` state on `main` as authoritative. Assets removed or gated by changes `031-03` through `031-05` MUST NOT reappear during regeneration.
- **Requirement ID**: ito-authority-cutover:guidance-and-asset-convergence
- **Tags**: behavior

#### Rules / Invariants
- Canonical template sources MUST be updated before generated harness outputs.
- Wiki source metadata MUST index tracked `.ito` artifacts instead of the retired mirror.
- Historical and migration documentation may name retired surfaces only when clearly labeled as historical or transitional.

#### Scenario: Managed surfaces are regenerated
- **GIVEN** canonical config and template sources reflect the completed core reset
- **WHEN** schema and harness asset generators run
- **THEN** a second generation pass produces no diff
- **AND** default generated surfaces contain neither tmux assets nor removed lifecycle skills
- **AND** coordination, backend, and other experimental surfaces appear only where their explicit feature policy allows them

#### Scenario: Source-of-truth references are audited
- **GIVEN** the migration documentation and generated assets are complete
- **WHEN** source guidance, wiki metadata, specs, and user docs are searched for authority claims
- **THEN** current guidance points to tracked `.ito` state on `main`
- **AND** no current workflow instructs users to author in `docs/ito` or the external coordination checkout

### Requirement: Release readiness proves both supported feature lanes
The reset release MUST be blocked until the default feature lane and the explicit all-features lane each pass their required formatting, lint, build, test, schema, template, documentation, and non-publishing release-plan checks. Two independent reviewers SHALL assess the migration, and a requirement audit MUST link every requirement in this change to passing evidence.
- **Requirement ID**: ito-authority-cutover:dual-lane-release-verification
- **Tags**: behavior, stateful

#### Scenario: Both lanes and reviews pass
- **GIVEN** migration and regeneration are complete
- **WHEN** release readiness is evaluated
- **THEN** default-feature and all-features CI evidence is green
- **AND** schema, template, documentation, and release-plan checks are green
- **AND** two independent reviews have no unresolved blocking findings
- **AND** the requirement audit accounts for every requirement ID

#### Scenario: Any release gate fails
- **GIVEN** any feature lane, generated-artifact check, release-plan check, review, or requirement mapping is incomplete or failing
- **WHEN** release readiness is evaluated
- **THEN** no reset release or version tag is created
- **AND** the failing gate remains visible with remediation evidence
<!-- ITO:END -->
