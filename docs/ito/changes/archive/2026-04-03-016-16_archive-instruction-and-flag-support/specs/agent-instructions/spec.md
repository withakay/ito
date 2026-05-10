## ADDED Requirements

### Requirement: Archive instruction with change ID

The CLI SHALL support `ito agent instruction archive --change <id>` and emit a short instruction directing the agent to run `ito archive <change-id> --yes` and record the audit guardrail steps.

#### Scenario: Archive instruction with change flag

- **WHEN** an agent runs `ito agent instruction archive --change <change-id>`
- **THEN** the system prints instruction text that tells the agent to run `ito archive <change-id> --yes`
- **AND** the output includes the audit reconcile guardrail (`ito audit reconcile --change <id>` before archiving)

### Requirement: Archive instruction without change ID

The CLI SHALL support `ito agent instruction archive` (without `--change`) and emit generic archive guidance covering when to archive, what the command does, and the recommended pre-archive audit steps.

#### Scenario: Archive instruction without change flag

- **WHEN** an agent runs `ito agent instruction archive` with no `--change`
- **THEN** the system prints generic archive guidance (not an error)
- **AND** the output explains what `ito archive` does and when to use it
- **AND** the output includes available changes as a hint when any exist
