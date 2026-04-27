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

The manifesto SHALL be derived from the effective merged Ito configuration and SHALL include relevant config capsules only in a redacted form that omits secrets and local-only sensitive details by default. At minimum, secret-bearing values and machine-local absolute paths outside the project scope MUST be removed or replaced with placeholders while preserving the behavioral meaning of the affected setting.

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

### Requirement: Manifesto declares workflow state and profile constraints

The manifesto SHALL declare a state capsule and SHALL define the allowed and forbidden moves for one active capability profile selected from `planning`, `proposal-only`, `review-only`, `apply`, `archive`, or `full`. Resolved scope and workflow state SHALL narrow the requested profile's permissions rather than expand them.

- **Requirement ID**: `agent-instructions:manifesto-state-and-profile`

#### Scenario: Proposal-only profile forbids implementation work

- **WHEN** the user runs `ito agent instruction manifesto --profile proposal-only`
- **THEN** the rendered manifesto states that proposal, spec, design, and task artifacts may be created or revised
- **AND** the rendered manifesto forbids product-code edits and implementation claims

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

### Requirement: Manifesto variants control embedded instruction detail

The system SHALL support at least `light` and `full` manifesto variants. `light` SHALL provide a compact protocol contract, and `full` SHALL embed deterministically selected rendered Ito instruction content while keeping manifesto-level rules authoritative. Variant selection controls output detail; profile selection controls lifecycle permissions.

- **Requirement ID**: `agent-instructions:manifesto-variant-rendering`

#### Scenario: Light variant stays compact

- **WHEN** the user runs `ito agent instruction manifesto --variant light`
- **THEN** the output includes the state machine, config and state capsules, and concise playbooks
- **AND** the output does not expand into the full body of related instruction artifacts by default

#### Scenario: Full variant embeds related instruction content

- **WHEN** the user runs `ito agent instruction manifesto --variant full`
- **THEN** the output includes rendered Ito instruction content selected by a fixed rule based on scope, resolved state, and active profile
- **AND** the manifesto's global MUST and MUST NOT rules take precedence over any embedded instruction text

#### Scenario: Full variant with explicit operation scopes embedded instructions

- **WHEN** the user runs `ito agent instruction manifesto --variant full --operation apply`
- **AND** `apply` is permitted by the current scope, resolved state, and active profile
- **THEN** the output embeds only the rendered `apply` instruction for operation-specific guidance
- **AND** the manifesto still includes its global contract, state capsule, and policy sections

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
