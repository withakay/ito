<!-- ITO:START -->
## ADDED Requirements

### Requirement: Manifesto instruction artifact is available

The agent instruction system SHALL render a first-class `manifesto` artifact for project-wide use and SHALL support optional change-scoped rendering for an exact change ID.

- **Requirement ID**: `agent-instructions:manifesto-artifact-availability`

#### Scenario: Project-wide manifesto render

- **WHEN** the user runs `ito agent instruction manifesto`
- **THEN** the system renders a manifesto document without requiring a change ID
- **AND** the output describes Ito operating constraints using the effective project configuration

#### Scenario: Change-scoped manifesto render

- **WHEN** the user runs `ito agent instruction manifesto --change <change-id>`
- **THEN** the system renders a manifesto document scoped to that exact change ID
- **AND** the output includes change-specific state rather than only project-wide guidance

### Requirement: Manifesto output is config-bound and redacted

The manifesto SHALL be derived from the effective merged Ito configuration and SHALL include relevant config capsules only in a redacted form that omits secrets and local-only sensitive details by default.

- **Requirement ID**: `agent-instructions:manifesto-config-redaction`

#### Scenario: Effective config is reflected in manifesto rules

- **WHEN** worktree, coordination, memory, or user-guidance settings affect agent behavior
- **THEN** the rendered manifesto reflects those settings in its rules or capsules
- **AND** the output is based on the merged effective config rather than hardcoded defaults

#### Scenario: Sensitive values are redacted

- **WHEN** the effective configuration contains secrets or local-only path details
- **THEN** the rendered manifesto omits or redacts those values by default
- **AND** the remaining capsule still communicates the behavioral rule that depends on them

### Requirement: Manifesto declares workflow state and profile constraints

The manifesto SHALL declare a state capsule and SHALL define the allowed and forbidden moves for one active capability profile selected from `planning`, `proposal-only`, `review-only`, `apply`, `archive`, or `full`.

- **Requirement ID**: `agent-instructions:manifesto-state-and-profile`

#### Scenario: Proposal-only profile forbids implementation work

- **WHEN** the user runs `ito agent instruction manifesto --profile proposal-only`
- **THEN** the rendered manifesto states that proposal, spec, design, and task artifacts may be created or revised
- **AND** the rendered manifesto forbids product-code edits and implementation claims

#### Scenario: Change-scoped render includes workflow state

- **WHEN** the user runs `ito agent instruction manifesto --change <change-id>`
- **THEN** the rendered manifesto includes a state capsule for that change
- **AND** the manifesto lists allowed and forbidden operations for the inferred or resolved state

### Requirement: Manifesto variants control embedded instruction detail

The system SHALL support at least `light` and `full` manifesto variants. `light` SHALL provide a compact protocol contract, and `full` SHALL embed relevant rendered Ito instruction content while keeping manifesto-level rules authoritative.

- **Requirement ID**: `agent-instructions:manifesto-variant-rendering`

#### Scenario: Light variant stays compact

- **WHEN** the user runs `ito agent instruction manifesto --variant light`
- **THEN** the output includes the state machine, config and state capsules, and concise playbooks
- **AND** the output does not expand into the full body of related instruction artifacts by default

#### Scenario: Full variant embeds related instruction content

- **WHEN** the user runs `ito agent instruction manifesto --variant full`
- **THEN** the output includes relevant rendered Ito instruction content for the requested scope
- **AND** the manifesto's global MUST and MUST NOT rules take precedence over any embedded instruction text

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
