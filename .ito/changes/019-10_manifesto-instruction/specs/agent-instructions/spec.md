<!-- ITO:START -->
## ADDED Requirements

### Requirement: Manifesto instruction artifact is available

The agent instruction system SHALL render a first-class `manifesto` artifact for project-wide use and SHALL support optional change-scoped rendering for an exact change ID. When a requested change cannot be resolved from the authoritative project state, the request SHALL fail with an actionable error instead of fabricating state.

- **Requirement ID**: `agent-instructions:manifesto-artifact-availability`

#### Scenario: Project-wide manifesto render

- **WHEN** the user runs `ito agent instruction manifesto`
- **THEN** the system renders a manifesto document without requiring a change ID
- **AND** the output describes Ito operating constraints using the effective project configuration

#### Scenario: Change-scoped manifesto render

- **WHEN** the user runs `ito agent instruction manifesto --change <change-id>`
- **THEN** the system renders a manifesto document scoped to that exact change ID
- **AND** the output includes change-specific state rather than only project-wide guidance

#### Scenario: Unresolved change request fails

- **WHEN** the user runs `ito agent instruction manifesto --change <change-id>`
- **AND** `<change-id>` does not resolve from the authoritative change repository
- **THEN** the system fails the request with an actionable error
- **AND** the system does not invent a state capsule for the missing change

### Requirement: Manifesto output is config-bound and redacted

The manifesto SHALL be derived from the effective merged Ito configuration and SHALL include relevant config capsules only in a redacted form that omits secrets and local-only sensitive details by default. The config and state capsules SHALL cover only workflow-relevant settings and resolved facts needed to follow Ito rules, including worktree policy, coordination policy, memory configuration state, user-guidance overlays, and change-state metadata. Project-scoped paths MUST be rendered relative to the project root, and non-project absolute paths MUST be removed or replaced with placeholders while preserving the behavioral meaning of the affected setting.

- **Requirement ID**: `agent-instructions:manifesto-config-redaction`

#### Scenario: Effective config is reflected in manifesto rules

- **WHEN** worktree, coordination, memory, or user-guidance settings affect agent behavior
- **THEN** the rendered manifesto reflects those settings in its rules or capsules
- **AND** the output is based on the merged effective config rather than hardcoded defaults

#### Scenario: Sensitive values are redacted

- **WHEN** the effective configuration contains secrets or local-only path details
- **THEN** the rendered manifesto omits or redacts those values by default
- **AND** the remaining capsule still communicates the behavioral rule that depends on them

#### Scenario: Machine-local paths are summarized safely

- **WHEN** the effective configuration or resolved state contains absolute machine-local paths outside the project root
- **THEN** the rendered manifesto replaces those paths with redacted or summarized placeholders
- **AND** project-scoped paths that are necessary to follow Ito workflow remain understandable

#### Scenario: Project-scoped paths stay portable

- **WHEN** the effective configuration or resolved state contains project-scoped absolute paths
- **THEN** the rendered manifesto expresses those paths relative to the project root
- **AND** the rendered manifesto does not expose raw machine-local absolute paths such as `/Users/...`

### Requirement: Manifesto declares workflow state and profile constraints

The manifesto SHALL declare a state capsule and SHALL define the allowed and forbidden moves for one active capability profile selected from `planning`, `proposal-only`, `review-only`, `apply`, `archive`, or `full`. Resolved scope and workflow state SHALL narrow the requested profile's permissions rather than expand them.

Canonical workflow states are:

- `no-change-selected`
- `proposal-drafting`
- `review-needed`
- `apply-ready`
- `applying`
- `reviewing-implementation`
- `archive-ready`
- `finished`

The state operation sets are:

- `no-change-selected`: `inspect`, `select-change`, `propose-change`
- `proposal-drafting`: `proposal`, `specs`, `design`, `tasks`, `validate`, `review`
- `review-needed`: `review`, `revise-artifacts`
- `apply-ready`: `worktree-ensure`, `apply`, `validate`
- `applying`: `implement`, `task-update`, `validate`, `revise-artifacts`
- `reviewing-implementation`: `review`, `fix`, `validate`
- `archive-ready`: `archive`, `reconcile`
- `finished`: `cleanup`, `memory-capture`, `report`

The profile operation sets are:

- `planning`: `inspect`, `select-change`, `propose-change`, `report`
- `proposal-only`: `proposal`, `specs`, `design`, `tasks`, `validate`, `review`, `revise-artifacts`, `report`
- `review-only`: `inspect`, `review`, `report`
- `apply`: `worktree-ensure`, `apply`, `implement`, `task-update`, `validate`, `review`, `fix`, `revise-artifacts`, `report`
- `archive`: `archive`, `reconcile`, `cleanup`, `memory-capture`, `report`
- `full`: union of all profile operation sets above

The allowed operations rendered by the manifesto SHALL be the intersection of the active profile operation set and the resolved workflow state's operation set.

For embedded instruction selection, only these artifact-mapped operations correspond to rendered instruction bodies: `proposal`, `specs`, `design`, `tasks`, `apply`, `review`, `archive`, `finish`. Generic operations such as `inspect`, `select-change`, `propose-change`, `implement`, `fix`, `report`, `reconcile`, `task-update`, `cleanup`, `memory-capture`, and `revise-artifacts` SHALL influence gating and explanatory text but SHALL NOT cause the system to invent new embedded instruction artifact bodies.

State resolution SHALL follow this order:

1. If no valid change is in scope, resolve `no-change-selected`.
2. If the change is already archived, resolve `finished`.
3. If required proposal-phase artifacts are missing or change validation fails, resolve `proposal-drafting`.
4. If implementation tasks show any in-progress work or any completed work while remaining tasks still exist, resolve `applying`.
5. If all implementation tasks are complete and the change is not archived, resolve `reviewing-implementation` unless authoritative approval-to-archive state is available.
6. If required proposal-phase artifacts are present, validation passes, and no implementation task progress has started, resolve `apply-ready` unless authoritative proposal-review state indicates `review-needed`.
7. Resolve `review-needed` or `archive-ready` only when authoritative host or repository state explicitly indicates those phases.

- **Requirement ID**: `agent-instructions:manifesto-state-and-profile`

#### Scenario: Proposal-only profile forbids implementation work

- **WHEN** the user runs `ito agent instruction manifesto --profile proposal-only`
- **THEN** the rendered manifesto states that proposal, spec, design, and task artifacts may be created or revised
- **AND** the rendered manifesto forbids product-code edits and implementation claims

#### Scenario: Review-only profile forbids mutation

- **WHEN** the user runs `ito agent instruction manifesto --profile review-only`
- **THEN** the rendered manifesto allows inspection and review operations only
- **AND** the rendered manifesto forbids proposal editing, product-code edits, archive actions, and implementation claims

#### Scenario: Planning profile remains advisory

- **WHEN** the user runs `ito agent instruction manifesto --profile planning`
- **THEN** the rendered manifesto allows inspection, change selection, and reporting operations only
- **AND** the rendered manifesto forbids artifact mutation and product-code edits

#### Scenario: Change-scoped render includes workflow state

- **WHEN** the user runs `ito agent instruction manifesto --change <change-id>`
- **THEN** the rendered manifesto includes a state capsule for that change
- **AND** the manifesto lists allowed and forbidden operations for the inferred or resolved state

#### Scenario: No-change-selected state narrows apply-oriented profiles

- **WHEN** the user runs `ito agent instruction manifesto --profile apply`
- **AND** no change is selected
- **THEN** the rendered manifesto remains in `no-change-selected`
- **AND** the rendered manifesto forbids apply and archive operations until a valid change is selected

#### Scenario: Resolved state narrows requested profile

- **WHEN** the user runs `ito agent instruction manifesto --change <change-id> --profile apply`
- **AND** the resolved state for `<change-id>` is `archive-ready` or `finished`
- **THEN** the rendered manifesto keeps `apply` as the requested profile context
- **AND** the allowed operations are limited to the operations permitted by the resolved state
- **AND** the manifesto does not claim that implementation is currently allowed

#### Scenario: Full profile still respects state machine

- **WHEN** the user runs `ito agent instruction manifesto --change <change-id> --profile full`
- **AND** the resolved state for `<change-id>` is `review-needed`
- **THEN** the rendered manifesto allows only the operations in the intersection of `full` profile permissions and `review-needed` state permissions
- **AND** the rendered manifesto does not allow apply or archive operations until the state changes

### Requirement: Manifesto variants control embedded instruction detail

The system SHALL support at least `light` and `full` manifesto variants. `light` SHALL provide a compact protocol contract, and `full` SHALL embed deterministically selected rendered Ito instruction content while keeping manifesto-level rules authoritative. Variant selection controls output detail; profile selection controls lifecycle permissions. The `--operation` selector SHALL only be accepted with `variant=full`.

- **Requirement ID**: `agent-instructions:manifesto-variant-rendering`

#### Scenario: Light variant stays compact

- **WHEN** the user runs `ito agent instruction manifesto --variant light`
- **THEN** the output includes the state machine, config and state capsules, and concise playbooks
- **AND** the output does not expand into the full body of related instruction artifacts by default

#### Scenario: Full variant embeds related instruction content

- **WHEN** the user runs `ito agent instruction manifesto --variant full`
- **THEN** the output includes rendered Ito instruction content selected by a fixed rule based on scope, resolved state, and active profile
- **AND** the manifesto's global MUST and MUST NOT rules take precedence over any embedded instruction text

#### Scenario: Full variant without explicit operation uses fixed artifact order

- **WHEN** the user runs `ito agent instruction manifesto --variant full`
- **AND** no `--operation` is provided
- **THEN** the output embeds the allowed subset of this fixed ordered artifact list: `proposal`, `specs`, `design`, `tasks`, `apply`, `review`, `archive`, `finish`
- **AND** artifact inclusion is determined by the intersection of current scope, resolved state, and active profile
- **AND** the output preserves that artifact order when multiple instruction bodies are embedded

#### Scenario: Full variant with explicit operation scopes embedded instructions

- **WHEN** the user runs `ito agent instruction manifesto --variant full --operation apply`
- **AND** `apply` is permitted by the current scope, resolved state, and active profile
- **THEN** the output embeds only the rendered `apply` instruction for operation-specific guidance
- **AND** the manifesto still includes its global contract, state capsule, and policy sections

#### Scenario: Operation selector is rejected for light variant

- **WHEN** the user runs `ito agent instruction manifesto --variant light --operation apply`
- **THEN** the system fails the request with an actionable error
- **AND** the system does not render a manifesto with embedded operation-specific content

#### Scenario: Incompatible explicit operation fails

- **WHEN** the user runs `ito agent instruction manifesto --variant full --operation apply`
- **AND** `apply` is forbidden by the current scope, resolved state, or active profile
- **THEN** the system fails the request with an actionable error
- **AND** the system does not embed an `apply` instruction body

#### Scenario: Variant and profile full are disambiguated

- **WHEN** the user runs `ito agent instruction manifesto --variant full --profile full`
- **THEN** the rendered output treats `variant=full` as output-detail selection
- **AND** the rendered output treats `profile=full` as lifecycle-permission selection
- **AND** the rendered contract describes those dimensions distinctly

### Requirement: Manifesto artifact is discoverable through instruction interfaces

The instruction system SHALL expose `manifesto` as a supported agent-instruction artifact anywhere users inspect available instruction artifacts, including CLI help and machine-readable instruction responses.

- **Requirement ID**: `agent-instructions:manifesto-discoverability`

#### Scenario: Help output lists manifesto

- **WHEN** the user inspects `ito agent instruction` help output
- **THEN** `manifesto` appears as an available artifact

#### Scenario: Machine-readable response identifies manifesto artifact

- **WHEN** the system returns a machine-readable instruction response for the manifesto artifact
- **THEN** the response identifies the artifact as `manifesto`
- **AND** the response structure is consistent with other agent-instruction artifacts
<!-- ITO:END -->
