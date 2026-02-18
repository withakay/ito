# Tasks for: 005-13_agent-model-manager

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Tracking**: Use `ito tasks` commands

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Create ito-models crate

- **Files**: ito-rs/crates/ito-models/
- **Dependencies**: None
- **Action**: Create new crate for models.dev integration
- **Verify**: `cargo build -p ito-models`
- **Done When**: Crate compiles
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 1.2: Define model data types

- **Files**: ito-rs/crates/ito-models/src/types.rs
- **Dependencies**: Task 1.1
- **Action**: Define Model, Provider, ModelCapability structs
- **Verify**: `cargo test -p ito-models`
- **Done When**: Types compile and serialize
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 1.3: Implement models.dev API client

- **Files**: ito-rs/crates/ito-models/src/client.rs
- **Dependencies**: Task 1.2
- **Action**: Create ModelsDevClient with fetch_models()
- **Verify**: `cargo test -p ito-models`
- **Done When**: Can fetch models from API
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 1.4: Implement model cache

- **Files**: ito-rs/crates/ito-models/src/cache.rs
- **Dependencies**: Task 1.2
- **Action**: Create ModelCache with load/save/is_stale
- **Verify**: `cargo test -p ito-models`
- **Done When**: Cache persists and checks TTL
- **Updated At**: 2026-02-05
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Create ModelRegistry facade

- **Files**: ito-rs/crates/ito-models/src/registry.rs
- **Dependencies**: None
- **Action**: Create registry with filtering methods
- **Verify**: `cargo test -p ito-models`
- **Done When**: Registry provides filtered access
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 2.2: Add model comparison

- **Files**: ito-rs/crates/ito-models/src/compare.rs
- **Dependencies**: Task 2.1
- **Action**: Create compare_models function
- **Verify**: `cargo test -p ito-models`
- **Done When**: Can compare two models
- **Updated At**: 2026-02-05
- **Status**: [x] complete

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 1

### Task 3.1: Define agent types

- **Files**: ito-rs/crates/ito-models/src/agent.rs
- **Dependencies**: None
- **Action**: Define Harness, AgentTier, AgentFile types
- **Verify**: `cargo test -p ito-models`
- **Done When**: Types compile with tests
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 3.2: Implement agent discovery

- **Files**: ito-rs/crates/ito-models/src/discovery.rs
- **Dependencies**: Task 3.1
- **Action**: Create discover_agents function
- **Verify**: `cargo test -p ito-models`
- **Done When**: Discovers agents across harnesses
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 3.3: Implement agent file update

- **Files**: ito-rs/crates/ito-models/src/update.rs
- **Dependencies**: Task 3.2
- **Action**: Create update_agent_model with backup
- **Verify**: `cargo test -p ito-models`
- **Done When**: Updates model preserving content
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 3.4: Implement rollback

- **Files**: ito-rs/crates/ito-models/src/rollback.rs
- **Dependencies**: Task 3.3
- **Action**: Create rollback_all function
- **Verify**: `cargo test -p ito-models`
- **Done When**: Restores from backups
- **Updated At**: 2026-02-05
- **Status**: [x] complete

______________________________________________________________________

## Wave 4

- **Depends On**: Wave 2, Wave 3

### Task 4.1: Add model command group to CLI

- **Files**: ito-rs/crates/ito-cli/src/app/model.rs
- **Dependencies**: None
- **Action**: Create agent model command group
- **Verify**: `ito agent model --help`
- **Done When**: Help displays subcommands
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 4.2: Implement model list command

- **Files**: ito-rs/crates/ito-cli/src/app/model.rs
- **Dependencies**: Task 4.1
- **Action**: Create list command with filters
- **Verify**: `ito agent model list`
- **Done When**: Lists models with filters
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 4.3: Implement model show command

- **Files**: ito-rs/crates/ito-cli/src/app/model.rs
- **Dependencies**: Task 4.1
- **Action**: Create show command
- **Verify**: `ito agent model show <id>`
- **Done When**: Shows model details
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 4.4: Implement model compare command

- **Files**: ito-rs/crates/ito-cli/src/app/model.rs
- **Dependencies**: Task 4.1
- **Action**: Create compare command
- **Verify**: `ito agent model compare <a> <b>`
- **Done When**: Shows comparison
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 4.5: Implement model agents command

- **Files**: ito-rs/crates/ito-cli/src/app/model.rs
- **Dependencies**: Task 4.1
- **Action**: Create agents command
- **Verify**: `ito agent model agents`
- **Done When**: Lists discovered agents
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 4.6: Implement model update command

- **Files**: ito-rs/crates/ito-cli/src/app/model.rs
- **Dependencies**: Task 4.1
- **Action**: Create update command
- **Verify**: `ito agent model update --dry-run`
- **Done When**: Updates agents with backup
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 4.7: Implement model rollback command

- **Files**: ito-rs/crates/ito-cli/src/app/model.rs
- **Dependencies**: Task 4.1
- **Action**: Create rollback command
- **Verify**: `ito agent model rollback`
- **Done When**: Restores from backups
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 4.8: Implement model refresh command

- **Files**: ito-rs/crates/ito-cli/src/app/model.rs
- **Dependencies**: Task 4.1
- **Action**: Create refresh command
- **Verify**: `ito agent model refresh`
- **Done When**: Refreshes cache
- **Updated At**: 2026-02-05
- **Status**: [x] complete

______________________________________________________________________

## Wave 5

- **Depends On**: Wave 4

### Task 5.1: Create agent templates for OpenCode

- **Files**: ito-rs/crates/ito-templates/assets/agents/opencode/
- **Dependencies**: None
- **Action**: Create ito-quick.md, ito-general.md, ito-thinking.md
- **Verify**: Templates exist with valid frontmatter
- **Done When**: Three templates created
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 5.2: Create agent templates for Claude Code

- **Files**: ito-rs/crates/ito-templates/assets/agents/claude-code/
- **Dependencies**: None
- **Action**: Create templates with model: haiku/sonnet/opus
- **Verify**: Templates exist with valid frontmatter
- **Done When**: Three templates created
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 5.3: Create skills for Codex

- **Files**: ito-rs/crates/ito-templates/assets/agents/codex/
- **Dependencies**: None
- **Action**: Create SKILL.md format templates
- **Verify**: Templates exist with valid format
- **Done When**: Three skills created
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 5.4: Create agent templates for GitHub Copilot

- **Files**: ito-rs/crates/ito-templates/assets/agents/github-copilot/
- **Dependencies**: None
- **Action**: Create templates with Copilot format
- **Verify**: Templates exist with valid format
- **Done When**: Three templates created
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 5.5: Implement template placeholder resolution

- **Files**: ito-rs/crates/ito-templates/src/agents.rs
- **Dependencies**: Task 5.1
- **Action**: Resolve {{model}} placeholders from config
- **Verify**: `cargo test -p ito-templates`
- **Done When**: Placeholders resolve correctly
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 5.6: Update ito init to install agents

- **Files**: ito-rs/crates/ito-core/src/installers/
- **Dependencies**: Task 5.5
- **Action**: Add agent installation to init
- **Verify**: `ito init` creates agents
- **Done When**: Init installs agent templates
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 5.7: Update ito update to refresh agents

- **Files**: ito-rs/crates/ito-cli/src/commands/update.rs
- **Dependencies**: Task 5.6
- **Action**: Add agent refresh to update
- **Verify**: `ito update` refreshes agents
- **Done When**: Update refreshes agent models
- **Updated At**: 2026-02-05
- **Status**: [x] complete

______________________________________________________________________

## Wave 6

- **Depends On**: Wave 5

### Task 6.1: Add unit tests for ito-models

- **Files**: ito-rs/crates/ito-models/src/*.rs
- **Dependencies**: None
- **Action**: Add tests for all public functions
- **Verify**: `cargo test -p ito-models`
- **Done When**: 80% coverage target
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 6.2: Add integration tests

- **Files**: ito-rs/crates/ito-cli/tests/
- **Dependencies**: Task 6.1
- **Action**: Add CLI integration tests
- **Verify**: `cargo test --test agent_model`
- **Done When**: Integration tests pass
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 6.3: Create centralized config defaults

- **Files**: ito-rs/crates/ito-core/src/config/defaults.rs
- **Dependencies**: None
- **Action**: Create defaults.rs with all config defaults
- **Verify**: `cargo test -p ito-core`
- **Done When**: Defaults centralized
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 6.4: Add harnesses section to config schema

- **Files**: ito-rs/crates/ito-core/src/config/
- **Dependencies**: Task 6.3
- **Action**: Add harness config types
- **Verify**: `cargo test -p ito-core`
- **Done When**: Config accepts harnesses
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 6.5: Generate JSON schema for config

- **Files**: ito-rs/crates/ito-core/src/config/schema.rs
- **Dependencies**: Task 6.4
- **Action**: Add schemars for JSON schema
- **Verify**: `cargo test -p ito-core`
- **Done When**: Schema generates correctly
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 6.6: Add config schema CLI command

- **Files**: ito-rs/crates/ito-cli/src/commands/config/
- **Dependencies**: Task 6.5
- **Action**: Create `ito config schema` command
- **Verify**: `ito config schema | jq .`
- **Done When**: Outputs valid JSON schema
- **Updated At**: 2026-02-05
- **Status**: [x] complete

______________________________________________________________________

## Wave 7

- **Depends On**: Wave 6

### Task 7.1: Update subagent-driven-development skill

- **Files**: ito-rs/crates/ito-templates/assets/skills/subagent-driven-development/SKILL.md
- **Dependencies**: None
- **Action**: Reference ito-general/ito-quick
- **Verify**: Skill references ito agents
- **Done When**: Skill updated
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 7.2: Update dispatching-parallel-agents skill

- **Files**: ito-rs/crates/ito-templates/assets/skills/dispatching-parallel-agents/SKILL.md
- **Dependencies**: None
- **Action**: Add agent tier guidance
- **Verify**: Skill includes guidance
- **Done When**: Skill updated
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 7.3: Update requesting-code-review skill

- **Files**: ito-rs/crates/ito-templates/assets/skills/requesting-code-review/SKILL.md
- **Dependencies**: None
- **Action**: Reference appropriate agents
- **Verify**: Skill references agents
- **Done When**: Skill updated
- **Updated At**: 2026-02-05
- **Status**: [x] complete

### Task 7.4: Update brainstorming skill

- **Files**: ito-rs/crates/ito-templates/assets/skills/brainstorming/SKILL.md
- **Dependencies**: None
- **Action**: Reference ito-thinking
- **Verify**: Skill references ito-thinking
- **Done When**: Skill updated
- **Updated At**: 2026-02-05
- **Status**: [x] complete

______________________________________________________________________

## Wave 8

- **Depends On**: Wave 7

### Task 8.1: Implementation review

- **Type**: checkpoint
- **Files**: All implementation files
- **Dependencies**: None
- **Action**: Review against specs, run tests
- **Verify**: All tests pass
- **Done When**: Human confirms complete
- **Updated At**: 2026-02-05
- **Status**: [x] complete

______________________________________________________________________

## Task Status Legend

- `[ ] pending` - Not started yet
- `[>] in-progress` - Currently being worked on
- `[x] complete` - Finished and verified
- `[-] shelved` - Intentionally not-to-be-done
