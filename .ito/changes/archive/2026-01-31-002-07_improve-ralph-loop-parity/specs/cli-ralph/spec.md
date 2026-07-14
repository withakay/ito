## ADDED Requirements

### Requirement: Context injection commands

The system SHALL support adding and clearing per-change context used by the ralph loop.

#### Scenario: Add context appends to per-change context file

- **WHEN** executing `ito ralph --add-context "<text>" --change <change-id>`
- **THEN** the system SHALL append `<text>` to the per-change context file under `.ito/.state/ralph/<change-id>/`
- **AND** the system SHALL print a confirmation message

#### Scenario: Clear context empties the per-change context file

- **WHEN** executing `ito ralph --clear-context --change <change-id>`
- **THEN** the system SHALL clear the per-change context file under `.ito/.state/ralph/<change-id>/`
- **AND** the system SHALL print a confirmation message

### Requirement: Context is reloaded every iteration

The system SHALL reload the per-change context file at the start of every ralph iteration.

#### Scenario: Mid-loop context updates appear on the next iteration

- **GIVEN** a ralph loop is running for `--change <change-id>`
- **WHEN** new content is appended to the per-change context file between iterations
- **THEN** the next iteration prompt SHALL include the new context content

### Requirement: Iteration prompt includes structured preamble and labeled context

The system SHALL structure the per-iteration prompt with a preamble and a clearly labeled context section when context is present.

#### Scenario: Preamble is included in iteration prompt

- **WHEN** the system starts ralph iteration `N`
- **THEN** the prompt SHALL include a preamble indicating the current iteration number
- **AND** the prompt SHALL include explicit instructions and autonomy requirements for an iterative development loop

#### Scenario: Context section is labeled when context exists

- **GIVEN** the per-change context content is non-empty
- **WHEN** building the prompt for an iteration
- **THEN** the prompt SHALL include a section labeled `## Additional Context (added by user mid-loop)`

### Requirement: Robust completion promise detection

The system SHALL detect the completion promise in harness output even when the promise contains surrounding whitespace and newlines.

#### Scenario: Completion promise detection ignores whitespace

- **GIVEN** `--completion-promise COMPLETE`
- **WHEN** harness output contains `<promise>\nCOMPLETE\n</promise>`
- **THEN** the system SHALL treat the completion promise as detected

### Requirement: Loop resilience on harness failure

The system SHALL record harness failures as iteration results and continue iterating unless fail-fast is enabled.

#### Scenario: Non-zero harness exit does not stop the loop

- **GIVEN** a harness exits with a non-zero exit code on iteration `N`
- **AND** fail-fast mode is not enabled
- **WHEN** the iteration completes
- **THEN** the system SHALL record the failure in iteration history
- **AND** the system SHALL proceed to iteration `N+1` (subject to `--max-iterations`)

#### Scenario: Fail-fast stops the loop on harness failure

- **GIVEN** fail-fast mode is enabled
- **WHEN** a harness exits with a non-zero exit code
- **THEN** the system SHALL stop the loop
- **AND** the command SHALL exit with a failing exit code

### Requirement: Rich iteration history and reporting

The system SHALL persist per-iteration history including completion detection and basic execution telemetry.

#### Scenario: Each iteration records exit code and git change summary

- **WHEN** a ralph iteration completes
- **THEN** iteration history SHALL include the harness exit code
- **AND** iteration history SHALL include a summary of git changes (at minimum: count of changed files)

#### Scenario: Status command reports recent iteration outcomes

- **WHEN** executing `ito ralph --status --change <change-id>`
- **THEN** the output SHALL include the current iteration count
- **AND** the output SHALL include recent iteration outcomes (at minimum: duration, completion found, and exit code)

### Requirement: Prompt file input

The system SHALL support loading the user prompt from a file.

#### Scenario: Prompt loaded from file

- **WHEN** executing `ito ralph --prompt-file <path> --change <change-id>`
- **THEN** the system SHALL read `<path>` as the user prompt

### Requirement: Streaming control

The system SHALL support disabling live streaming of harness output.

#### Scenario: No-stream disables live output streaming

- **WHEN** executing `ito ralph "<prompt>" --no-stream --change <change-id>`
- **THEN** the system SHALL not stream harness output live
- **AND** the system SHALL still capture enough output to detect completion promises
