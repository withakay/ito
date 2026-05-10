<!-- ITO:START -->
## ADDED Requirements

### Requirement: Apply instruction includes memory-capture reminder when capture is configured

The agent instruction artifact for `apply` SHALL include a trailing
"Capture memories" reminder section when Ito's `memory.capture` operation
is configured (either as a `skill` or a `command`). The reminder SHALL
instruct the agent to (a) review what was learned during the session,
(b) identify items worth persisting (decisions, gotchas, non-obvious
patterns, architecture rationale), and (c) invoke
`ito agent instruction memory-capture` with appropriate `--context`,
`--file`, and/or `--folder` inputs.

- **Requirement ID**: `agent-instructions:apply-memory-capture-reminder`

#### Scenario: Capture configured — reminder present

- **WHEN** `ito agent instruction apply --change <id>` is rendered and `memory.capture.configured` is `true`
- **THEN** the output contains a terminal section headed `Capture memories` (or equivalent)
- **AND** that section references `ito agent instruction memory-capture`
- **AND** the section mentions the three input flags (`--context`, `--file`, `--folder`)

#### Scenario: Capture not configured — reminder absent

- **WHEN** `ito agent instruction apply --change <id>` is rendered and `memory.capture.configured` is `false`
- **THEN** the output does not contain a `Capture memories` section
- **AND** no reference to `memory-capture`, `memory-search`, or `memory-query` is included

#### Scenario: Only search/query configured — apply reminder still absent

- **WHEN** `memory.search` and/or `memory.query` are configured but `memory.capture` is not
- **THEN** the rendered apply output does not contain a `Capture memories` section (the reminder is keyed on `memory.capture` specifically, not any memory configuration)

### Requirement: Finish instruction includes memory-capture and wrap-up reminders

The agent instruction artifact for `finish` SHALL include:

1. The same memory-capture reminder as the apply artifact (rendered only
   when `memory.capture` is configured).
2. A "Refresh archive and specs" reminder that tells the agent to:
   - Confirm the change has been archived (or run `ito archive <id> --yes`).
   - Confirm canonical specs under `.ito/specs/` reflect the delivered
     change (deltas merged).
   - Confirm agent-facing docs (e.g. `AGENTS.md`, `.ito/AGENTS.md`) are
     up to date with the new capability.

- **Requirement ID**: `agent-instructions:finish-wrap-up-reminder`

#### Scenario: Capture configured and not yet archived

- **WHEN** `ito agent instruction finish --change <id>` is rendered, `memory.capture.configured` is `true`, and the change is not yet archived
- **THEN** the output contains a `Capture memories` section referencing `ito agent instruction memory-capture`
- **AND** the output contains a `Refresh archive and specs` section
- **AND** that wrap-up section lists the specs and docs checks
- **AND** that wrap-up section does not repeat the archive confirmation step covered by the existing archive prompt

#### Scenario: Capture not configured and not yet archived

- **WHEN** `ito agent instruction finish --change <id>` is rendered, `memory.capture.configured` is `false`, and the change is not yet archived
- **THEN** the output does not contain a `Capture memories` section
- **AND** the output contains a `Refresh archive and specs` section
- **AND** that wrap-up section lists the specs and docs checks
- **AND** that wrap-up section does not repeat the archive confirmation step covered by the existing archive prompt

#### Scenario: Capture configured and already archived

- **WHEN** `ito agent instruction finish --change <id>` is rendered, `memory.capture.configured` is `true`, and the change is already archived
- **THEN** the output contains a `Capture memories` section referencing `ito agent instruction memory-capture`
- **AND** the output contains a `Refresh archive and specs` section
- **AND** that wrap-up section lists the archive confirmation, specs, and docs checks

#### Scenario: Capture not configured and already archived

- **WHEN** `ito agent instruction finish --change <id>` is rendered, `memory.capture.configured` is `false`, and the change is already archived
- **THEN** the output does not contain a `Capture memories` section
- **AND** the output contains a `Refresh archive and specs` section
- **AND** that wrap-up section lists the archive confirmation, specs, and docs checks
<!-- ITO:END -->
