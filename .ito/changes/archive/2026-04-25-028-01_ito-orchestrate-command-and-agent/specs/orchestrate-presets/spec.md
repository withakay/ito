<!-- ITO:START -->
## ADDED Requirements

### Requirement: Built-in preset library

The system SHALL ship five built-in presets — `rust`, `typescript`, `python`, `go`, `generic` — each stored as a YAML file at `ito-rs/crates/ito-templates/assets/presets/orchestrate/<stack>.yaml`. Each preset SHALL specify gate configuration, recommended skills, and agent role suggestions.

- **Requirement ID**: orchestrate-presets:library

#### Scenario: Preset is loaded when specified in orchestrate.md

- **WHEN** `orchestrate.md` front matter specifies `preset: rust`
- **THEN** the orchestrator loads `presets/orchestrate/rust.yaml` and applies its gate config and recommended skills to the run plan

#### Scenario: Unknown preset name fails with a clear error

- **WHEN** `orchestrate.md` specifies a preset name that does not match any built-in preset file
- **THEN** the system emits an error naming the unknown preset and lists available built-in preset names
- **AND** exits without beginning the run

### Requirement: Preset agent role suggestions

Each preset SHALL include an `agent_roles` section mapping logical roles (`apply-worker`, `review-worker`, `security-worker`) to suggested agent names for common harnesses. These suggestions SHALL be presented to the user during setup and injected as advisory guidance into the rendered orchestrator instruction; they SHALL NOT be auto-wired.

- **Requirement ID**: orchestrate-presets:agent-roles

#### Scenario: Agent role suggestions are advisory only

- **WHEN** a preset specifies `agent_roles.apply-worker: rust-engineer`
- **THEN** the orchestrator instruction document includes this as a suggestion
- **AND** the orchestrator agent is free to use a different agent if the suggested one is unavailable in the active harness

#### Scenario: User-prompt agent overrides take precedence over preset suggestions

- **WHEN** `orchestrate.md` explicitly names an agent for a role
- **THEN** that name is used in the rendered instruction, overriding the preset suggestion
<!-- ITO:END -->
