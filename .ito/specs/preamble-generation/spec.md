# Preamble Generation Specification

## Purpose

Define the `preamble-generation` capability, including required behavior and validation scenarios, so it remains stable and testable.


## Requirements

### Requirement: Generate structured iteration preamble

The system SHALL generate a structured preamble for each ralph loop iteration that includes iteration count, task description, instructions, autonomy rules, and completion signal format.

#### Scenario: Basic preamble structure

- **WHEN** the ralph loop starts an iteration
- **THEN** the preamble SHALL include a header with "Ralph Wiggum Loop - Iteration N"
- **THEN** the preamble SHALL include the current iteration number, max iterations (if set), and min iterations

#### Scenario: Task section inclusion

- **WHEN** the preamble is generated with a user prompt
- **THEN** the preamble SHALL include a "## Your Task" section containing the user's prompt
- **THEN** the task section SHALL appear after any additional context sections

#### Scenario: Instructions section

- **WHEN** the preamble is generated
- **THEN** the preamble SHALL include a "## Instructions" section with 5 numbered steps
- **THEN** the instructions SHALL include: read current state, update todo list, make progress, run tests/verification, output completion promise

#### Scenario: Critical rules section

- **WHEN** the preamble is generated
- **THEN** the preamble SHALL include a "## Critical Rules" section
- **THEN** critical rules SHALL specify: only output completion promise when truly done, do not lie or provide false promises, try different approaches if stuck, check work before claiming completion, loop continues until success, update todo list each iteration

#### Scenario: Autonomy requirements section

- **WHEN** the preamble is generated
- **THEN** the preamble SHALL include a "## AUTONOMY REQUIREMENTS (CRITICAL)" section
- **THEN** autonomy requirements SHALL specify: DO NOT ASK QUESTIONS, DO NOT USE THE QUESTION TOOL, make reasonable assumptions, use best judgment, choose reasonable approach and proceed, orchestrator cannot respond to questions, trust training and make decisions autonomously

#### Scenario: Completion promise format

- **WHEN** the preamble is generated with a completion promise value
- **THEN** the preamble SHALL embed the completion promise in both the instructions and critical rules sections
- **THEN** the completion promise SHALL be formatted as `<promise>{completionPromise}</promise>`

#### Scenario: Iteration progress display

- **WHEN** the preamble is generated with max iterations set
- **THEN** the preamble SHALL display "Current Iteration: N / MAX (min: MIN)"
- **WHEN** the preamble is generated without max iterations
- **THEN** the preamble SHALL display "Current Iteration: N (unlimited) (min: MIN)"
