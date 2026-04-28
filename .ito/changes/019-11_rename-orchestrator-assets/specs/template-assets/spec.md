<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: Orchestration Asset Names

Ito template assets SHALL install orchestration-related specialist role agents and skills using concise `ito-*` names without the redundant `ito-orchestrator-*` prefix.

This requirement applies to the specialist role names `planner`, `researcher`, `reviewer`, and `worker`. It does not rename the top-level `ito-orchestrator` agent or the `ito-orchestrator-workflow` skill.

#### Scenario: Project specialist skills use concise names

- **GIVEN** the project contains generated specialist skill assets
- **WHEN** the rename is applied
- **THEN** `.agents/skills/ito-orchestrator-planner/SKILL.md` is renamed to `.agents/skills/ito-planner/SKILL.md`
- **AND** `.agents/skills/ito-orchestrator-researcher/SKILL.md` is renamed to `.agents/skills/ito-researcher/SKILL.md`
- **AND** `.agents/skills/ito-orchestrator-reviewer/SKILL.md` is renamed to `.agents/skills/ito-reviewer/SKILL.md`
- **AND** `.agents/skills/ito-orchestrator-worker/SKILL.md` is renamed to `.agents/skills/ito-worker/SKILL.md`

#### Scenario: OpenCode specialist agent files use concise names

- **GIVEN** Ito installs OpenCode specialist agent files
- **WHEN** the rename is applied to source templates and generated outputs
- **THEN** `ito-rs/crates/ito-templates/assets/agents/opencode/ito-orchestrator-planner.md` is renamed to `ito-rs/crates/ito-templates/assets/agents/opencode/ito-planner.md`
- **AND** `ito-rs/crates/ito-templates/assets/agents/opencode/ito-orchestrator-researcher.md` is renamed to `ito-rs/crates/ito-templates/assets/agents/opencode/ito-researcher.md`
- **AND** `ito-rs/crates/ito-templates/assets/agents/opencode/ito-orchestrator-reviewer.md` is renamed to `ito-rs/crates/ito-templates/assets/agents/opencode/ito-reviewer.md`
- **AND** `ito-rs/crates/ito-templates/assets/agents/opencode/ito-orchestrator-worker.md` is renamed to `ito-rs/crates/ito-templates/assets/agents/opencode/ito-worker.md`
- **AND** `.opencode/agent/ito-orchestrator-planner.md` is renamed to `.opencode/agent/ito-planner.md`
- **AND** `.opencode/agent/ito-orchestrator-researcher.md` is renamed to `.opencode/agent/ito-researcher.md`
- **AND** `.opencode/agent/ito-orchestrator-reviewer.md` is renamed to `.opencode/agent/ito-reviewer.md`
- **AND** `.opencode/agent/ito-orchestrator-worker.md` is renamed to `.opencode/agent/ito-worker.md`

#### Scenario: Claude Code specialist agent files use concise names

- **GIVEN** Ito installs Claude Code specialist agent files
- **WHEN** the rename is applied to source templates and generated outputs
- **THEN** `ito-rs/crates/ito-templates/assets/agents/claude-code/ito-orchestrator-planner.md` is renamed to `ito-rs/crates/ito-templates/assets/agents/claude-code/ito-planner.md`
- **AND** `ito-rs/crates/ito-templates/assets/agents/claude-code/ito-orchestrator-researcher.md` is renamed to `ito-rs/crates/ito-templates/assets/agents/claude-code/ito-researcher.md`
- **AND** `ito-rs/crates/ito-templates/assets/agents/claude-code/ito-orchestrator-reviewer.md` is renamed to `ito-rs/crates/ito-templates/assets/agents/claude-code/ito-reviewer.md`
- **AND** `ito-rs/crates/ito-templates/assets/agents/claude-code/ito-orchestrator-worker.md` is renamed to `ito-rs/crates/ito-templates/assets/agents/claude-code/ito-worker.md`
- **AND** `.claude/agents/ito-orchestrator-planner.md` is renamed to `.claude/agents/ito-planner.md`
- **AND** `.claude/agents/ito-orchestrator-researcher.md` is renamed to `.claude/agents/ito-researcher.md`
- **AND** `.claude/agents/ito-orchestrator-reviewer.md` is renamed to `.claude/agents/ito-reviewer.md`
- **AND** `.claude/agents/ito-orchestrator-worker.md` is renamed to `.claude/agents/ito-worker.md`

#### Scenario: GitHub Copilot specialist agent files use concise names

- **GIVEN** Ito installs GitHub Copilot specialist agent files
- **WHEN** the rename is applied to source templates and generated outputs
- **THEN** `ito-rs/crates/ito-templates/assets/agents/github-copilot/ito-orchestrator-planner.md` is renamed to `ito-rs/crates/ito-templates/assets/agents/github-copilot/ito-planner.md`
- **AND** `ito-rs/crates/ito-templates/assets/agents/github-copilot/ito-orchestrator-researcher.md` is renamed to `ito-rs/crates/ito-templates/assets/agents/github-copilot/ito-researcher.md`
- **AND** `ito-rs/crates/ito-templates/assets/agents/github-copilot/ito-orchestrator-reviewer.md` is renamed to `ito-rs/crates/ito-templates/assets/agents/github-copilot/ito-reviewer.md`
- **AND** `ito-rs/crates/ito-templates/assets/agents/github-copilot/ito-orchestrator-worker.md` is renamed to `ito-rs/crates/ito-templates/assets/agents/github-copilot/ito-worker.md`
- **AND** `.github/agents/ito-orchestrator-planner.md` is renamed to `.github/agents/ito-planner.md`
- **AND** `.github/agents/ito-orchestrator-researcher.md` is renamed to `.github/agents/ito-researcher.md`
- **AND** `.github/agents/ito-orchestrator-reviewer.md` is renamed to `.github/agents/ito-reviewer.md`
- **AND** `.github/agents/ito-orchestrator-worker.md` is renamed to `.github/agents/ito-worker.md`

#### Scenario: Pi specialist agent files use concise names

- **GIVEN** Ito installs Pi specialist agent files
- **WHEN** the rename is applied to source templates and generated outputs
- **THEN** `ito-rs/crates/ito-templates/assets/agents/pi/ito-orchestrator-planner.md` is renamed to `ito-rs/crates/ito-templates/assets/agents/pi/ito-planner.md`
- **AND** `ito-rs/crates/ito-templates/assets/agents/pi/ito-orchestrator-researcher.md` is renamed to `ito-rs/crates/ito-templates/assets/agents/pi/ito-researcher.md`
- **AND** `ito-rs/crates/ito-templates/assets/agents/pi/ito-orchestrator-reviewer.md` is renamed to `ito-rs/crates/ito-templates/assets/agents/pi/ito-reviewer.md`
- **AND** `ito-rs/crates/ito-templates/assets/agents/pi/ito-orchestrator-worker.md` is renamed to `ito-rs/crates/ito-templates/assets/agents/pi/ito-worker.md`
- **AND** `.pi/agents/ito-orchestrator-planner.md` is renamed to `.pi/agents/ito-planner.md`
- **AND** `.pi/agents/ito-orchestrator-researcher.md` is renamed to `.pi/agents/ito-researcher.md`
- **AND** `.pi/agents/ito-orchestrator-reviewer.md` is renamed to `.pi/agents/ito-reviewer.md`
- **AND** `.pi/agents/ito-orchestrator-worker.md` is renamed to `.pi/agents/ito-worker.md`

#### Scenario: Codex specialist agent-skill files use concise names

- **GIVEN** Ito installs Codex specialist agent-skill files
- **WHEN** the rename is applied to source templates and generated outputs
- **THEN** `ito-rs/crates/ito-templates/assets/agents/codex/ito-orchestrator-planner/SKILL.md` is renamed to `ito-rs/crates/ito-templates/assets/agents/codex/ito-planner/SKILL.md`
- **AND** `ito-rs/crates/ito-templates/assets/agents/codex/ito-orchestrator-researcher/SKILL.md` is renamed to `ito-rs/crates/ito-templates/assets/agents/codex/ito-researcher/SKILL.md`
- **AND** `ito-rs/crates/ito-templates/assets/agents/codex/ito-orchestrator-reviewer/SKILL.md` is renamed to `ito-rs/crates/ito-templates/assets/agents/codex/ito-reviewer/SKILL.md`
- **AND** `ito-rs/crates/ito-templates/assets/agents/codex/ito-orchestrator-worker/SKILL.md` is renamed to `ito-rs/crates/ito-templates/assets/agents/codex/ito-worker/SKILL.md`

#### Scenario: Installed specialist metadata uses concise names

- **GIVEN** renamed specialist agent or skill files contain front matter or inline role references
- **WHEN** the rename is applied
- **THEN** every `name: ito-orchestrator-planner` metadata value is updated to `name: ito-planner`
- **AND** every `name: ito-orchestrator-researcher` metadata value is updated to `name: ito-researcher`
- **AND** every `name: ito-orchestrator-reviewer` metadata value is updated to `name: ito-reviewer`
- **AND** every `name: ito-orchestrator-worker` metadata value is updated to `name: ito-worker`
- **AND** orchestration preset agent references use `ito-planner`, `ito-researcher`, `ito-reviewer`, and `ito-worker` for the corresponding roles

#### Scenario: Obsolete orchestrator-prefixed assets are not emitted

- **GIVEN** Ito emits its template asset list
- **WHEN** generated assets are inspected
- **THEN** no specialist role asset path uses `ito-orchestrator-planner`, `ito-orchestrator-researcher`, `ito-orchestrator-reviewer`, or `ito-orchestrator-worker`
- **AND** template manifest tests and init/update tests assert the concise names listed in this requirement
<!-- ITO:END -->
