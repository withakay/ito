<!-- ITO:START -->
## ADDED Requirements

### Requirement: First-run setup detection

The system SHALL detect when no `orchestrate.md` exists in the project's user-prompts directory and emit setup guidance directing the agent to load the `ito-orchestrate-setup` skill before invoking `ito agent instruction orchestrate`.

- **Requirement ID**: orchestrate-setup:first-run-detection

#### Scenario: Missing orchestrate.md triggers setup guidance

- **WHEN** `ito agent instruction orchestrate` is invoked and `.ito/user-prompts/orchestrate.md` does not exist
- **THEN** the system prints a setup guidance message with the skill load instruction
- **AND** exits with a non-zero status code without rendering the orchestrator document

#### Scenario: Explicit setup flag bypasses instruction rendering

- **WHEN** `ito orchestrate --setup` is invoked (or equivalent harness command)
- **THEN** the system prints guidance to load `ito-orchestrate-setup` regardless of whether `orchestrate.md` already exists

### Requirement: Stack detection

The setup wizard skill SHALL detect the project stack by scanning the worktree root for indicator files: `Cargo.toml` → `rust`, `package.json` → `typescript`, `pyproject.toml` → `python`, `go.mod` → `go`. If none are found, the wizard SHALL default to `generic`.

- **Requirement ID**: orchestrate-setup:stack-detection

#### Scenario: Single indicator file detected

- **WHEN** the worktree root contains `Cargo.toml` and no other stack indicator files
- **THEN** the wizard selects the `rust` preset and proceeds without asking the user for a stack choice

#### Scenario: Multiple indicator files detected

- **WHEN** the worktree root contains more than one stack indicator file
- **THEN** the wizard presents the detected stacks to the user and asks them to confirm which preset to use

#### Scenario: No indicator files detected

- **WHEN** no known indicator files are found
- **THEN** the wizard selects the `generic` preset and informs the user

### Requirement: Skill and agent cross-reference

The setup wizard skill SHALL cross-reference available skills in the project against preset recommended skills, and detect available agent definitions, then present both as suggestions during setup. Detected agents SHALL be presented as suggestions only and SHALL NOT be auto-wired.

- **Requirement ID**: orchestrate-setup:cross-reference

#### Scenario: Matching skill detected and highlighted

- **WHEN** the `rust` preset recommends the `rust-style` skill and that skill exists in the project
- **THEN** the wizard highlights the skill as already available and notes it will be referenced in the generated workflow skill

#### Scenario: Missing recommended skill flagged

- **WHEN** a preset recommends a skill that is not found in the project
- **THEN** the wizard notes it as missing and advises the user on how to install it, but does not block setup completion

### Requirement: Setup outputs

The setup wizard skill SHALL produce two outputs upon completion: (1) a populated `orchestrate.md` written to `.ito/user-prompts/orchestrate.md`, and (2) a generated `ito-orchestrator-workflow` skill written to the project's skill directory.

- **Requirement ID**: orchestrate-setup:outputs

#### Scenario: Both outputs are written on successful setup

- **WHEN** the setup wizard completes without error
- **THEN** `.ito/user-prompts/orchestrate.md` exists with valid front matter and populated MUST/PREFER sections
- **AND** the `ito-orchestrator-workflow` skill exists in the project skill directory with stack-appropriate content

#### Scenario: Existing orchestrate.md is overwritten only after confirmation

- **WHEN** `ito orchestrate --setup` is invoked and `orchestrate.md` already exists
- **THEN** the wizard presents the existing configuration to the user and asks for confirmation before overwriting
<!-- ITO:END -->
