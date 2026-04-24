<!-- ITO:START -->
## ADDED Requirements

### Requirement: Apply instruction includes memory-capture reminder when memory is configured

The agent instruction artifact for `apply` SHALL include a trailing
"Capture memories" reminder section when Ito's `memory` config is present
and valid. The reminder SHALL instruct the agent to (a) review what was
learned during the session, (b) identify items worth persisting (decisions,
gotchas, non-obvious patterns, architecture rationale), and (c) invoke the
configured memory provider to store them.

- **Requirement ID**: `agent-instructions:apply-memory-reminder`

#### Scenario: Memory configured — reminder present

- **WHEN** `ito agent instruction apply --change <id>` is rendered and `memory.configured` is `true`
- **THEN** the output contains a terminal section headed `Capture memories` (or equivalent)
- **AND** that section references `ito agent instruction memory-capture` (or the rendered provider command/skill)

#### Scenario: Memory not configured — reminder absent

- **WHEN** `ito agent instruction apply --change <id>` is rendered and `memory.configured` is `false`
- **THEN** the output does not contain a `Capture memories` section
- **AND** no reference to the memory artifacts is included

### Requirement: Finish instruction includes memory-capture and wrap-up reminders

The agent instruction artifact for `finish` SHALL include:

1. The same memory-capture reminder as the apply artifact (rendered only
   when memory is configured).
2. A "Refresh archive and specs" reminder that tells the agent to:
   - Confirm the change has been archived (or run `ito archive <id> --yes`).
   - Confirm canonical specs under `.ito/specs/` reflect the delivered
     change (deltas merged).
   - Confirm agent-facing docs (e.g. `AGENTS.md`, `.ito/AGENTS.md`) are
     up to date with the new capability.

- **Requirement ID**: `agent-instructions:finish-wrap-up-reminder`

#### Scenario: Memory configured — both reminders present

- **WHEN** `ito agent instruction finish --change <id>` is rendered and `memory.configured` is `true`
- **THEN** the output contains a `Capture memories` section referencing `ito agent instruction memory-capture`
- **AND** the output contains a `Refresh archive and specs` section listing the three wrap-up checks above

#### Scenario: Memory not configured — wrap-up reminder still present

- **WHEN** `ito agent instruction finish --change <id>` is rendered and `memory.configured` is `false`
- **THEN** the output does not contain a `Capture memories` section
- **AND** the output still contains the `Refresh archive and specs` section

#### Scenario: Wrap-up reminder does not duplicate archive prompt

- **WHEN** the existing finish archive prompt ("Do you want to archive this change now?") is rendered
- **AND** the new `Refresh archive and specs` reminder is rendered
- **THEN** the archive step is referenced at most once per finish output (either via the existing prompt or the wrap-up reminder, not both, determined by whether the change is already archived)
<!-- ITO:END -->
