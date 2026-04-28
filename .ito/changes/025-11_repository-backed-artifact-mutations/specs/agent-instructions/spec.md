## ADDED Requirements

### Requirement: Generated active-work guidance routes Ito artifact edits through Ito mutation commands

When generated Ito instruction artifacts or installed harness guidance teach an agent how to change active-work Ito artifacts, they SHALL route that work through Ito artifact mutation commands rather than direct file edits.

This guidance MAY still permit ordinary file-edit tools for non-Ito project code, but it SHALL distinguish those from the authoritative mutation path for active-work Ito artifacts.

#### Scenario: Apply instructions direct artifact edits through Ito commands

- **WHEN** `ito agent instruction apply --change <id>` is rendered for a change that requires proposal, tasks, design, or spec-delta updates
- **THEN** the output directs the agent to `ito patch` and/or `ito write` for those Ito artifact mutations
- **AND** the output distinguishes those artifact mutations from ordinary code-file edits elsewhere in the repo

#### Scenario: Installed harness guidance remains aligned with rendered instructions

- **WHEN** Ito installs or updates harness guidance assets for supported harnesses
- **THEN** the guidance those assets provide about active-work Ito artifact mutations matches the rendered Ito instruction workflow
- **AND** it does not teach direct file edits as the primary path for updating Ito active-work artifacts
