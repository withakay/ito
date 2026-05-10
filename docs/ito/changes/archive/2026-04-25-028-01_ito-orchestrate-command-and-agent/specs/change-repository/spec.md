<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: ChangeRepository provides lifecycle-aware canonical access

`ChangeRepository` SHALL provide a canonical view of change data across both active and archived lifecycle states, independent of whether the underlying implementation is filesystem-backed or remote-backed.

`ChangeRepository` SHALL accept both module-level change IDs (`NNN-NN_name`) and sub-module change IDs (`NNN.SS-NN_name`) as canonical identifiers, and SHALL expose the parsed `module_id` plus optional `sub_module_id` in returned change models and summaries.

`ChangeRepository` SHALL expose an `orchestrate` field on each change model, sourced from the `orchestrate:` block in `.ito/changes/<id>/.ito.yaml`, containing `depends_on` (list of change IDs this change must wait for) and `preferred_gates` (ordered list of gate names overriding the default pipeline for this change). Both sub-fields are optional and default to empty.

- **Requirement ID**: change-repository:lifecycle-aware-canonical-access

#### Scenario: List active changes through selected implementation

- **GIVEN** Ito resolves a repository implementation for the current persistence mode
- **WHEN** a caller requests active changes
- **THEN** `ChangeRepository` returns only active changes from that implementation
- **AND** each returned summary includes `module_id`
- **AND** sub-module changes also include `sub_module_id`

#### Scenario: List archived changes through the same repository

- **GIVEN** archived changes exist in the selected persistence implementation
- **WHEN** a caller requests archived changes
- **THEN** `ChangeRepository` returns those archived changes without requiring a separate archive repository type
- **AND** sub-module-qualified change IDs remain unchanged in the returned results

#### Scenario: Resolve a change by canonical ID regardless of lifecycle

- **GIVEN** a canonical change ID exists in either active or archived state
- **WHEN** a caller resolves or loads that change through `ChangeRepository`
- **THEN** the repository returns the matching change from the selected persistence implementation
- **AND** the returned change preserves the canonical ID exactly as provided

#### Scenario: Remote mode ignores stray local active-change markdown

- **GIVEN** remote persistence mode is active
- **AND** stale or stray markdown exists under local `.ito/changes/`
- **WHEN** `ChangeRepository` serves change reads
- **THEN** it uses the remote-backed implementation as the canonical source
- **AND** it does not merge in local active-change markdown implicitly

#### Scenario: Orchestrate metadata exposed on change model

- **GIVEN** a change's `.ito.yaml` contains an `orchestrate:` block with `depends_on` and `preferred_gates`
- **WHEN** a caller loads that change through `ChangeRepository`
- **THEN** the returned change model includes `orchestrate.depends_on` as a list of change ID strings
- **AND** includes `orchestrate.preferred_gates` as an ordered list of gate name strings

#### Scenario: Absent orchestrate block returns empty defaults

- **GIVEN** a change's `.ito.yaml` has no `orchestrate:` block
- **WHEN** a caller loads that change through `ChangeRepository`
- **THEN** `orchestrate.depends_on` is an empty list
- **AND** `orchestrate.preferred_gates` is an empty list
<!-- ITO:END -->
