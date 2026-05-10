## Why

Agent harnesses (OpenCode, Claude Code, Codex, GitHub Copilot) allow defining agents/subagents via markdown files with YAML frontmatter that specifies which AI model to use (e.g., `model: anthropic/claude-sonnet-4-5`). Keeping these model references current is tedious: new models arrive frequently, different harnesses support different providers, and there's no automated way to update configurations across harnesses. We need a CLI tool to manage agent model configurations with models.dev as the authoritative data source.

## What Changes

- Add `ito agent model` command group for managing agent model configurations
- Integrate with models.dev API to fetch current model information (pricing, capabilities, context windows)
- Support 4 harnesses: OpenCode, Claude Code, Codex, GitHub Copilot
- Define 3 Ito agent tiers: `ito-quick`, `ito-general`, `ito-thinking`
- Create agent templates for each harness, installable via `ito init`
- Support per-harness model mapping with provider constraints (Claude→anthropic, Codex→openai, GH Copilot→github-copilot, OpenCode→any)
- Add `models.dev` Rust client crate for fetching model data
- Centralize all Ito configuration defaults in a single location
- Create JSON schema for config validation (editor autocomplete, validation)

## Capabilities

### New Capabilities

- `agent-model-registry`: Fetch and cache model information from models.dev (providers, pricing, limits, capabilities)
- `agent-model-update`: Update model references in agent/subagent markdown files across harnesses
- `cli-agent-model`: CLI commands for listing, comparing, and updating agent models
- `agent-templates`: Harness-specific agent templates for ito-quick, ito-general, ito-thinking
- `config-defaults`: Centralized default values for all Ito configuration
- `config-schema`: JSON schema for Ito configuration validation

### Modified Capabilities

- `cli-agent-config`: Extend agent command group to include model subcommands
- `cli-init`: Install agent templates during `ito init`
- `config`: Add harness and agent model configuration to existing config system

## Impact

- **Code**: New crate `ito-models` for models.dev integration; extensions to `ito-cli` for commands
- **Dependencies**: Add `models_dev` crate or implement minimal HTTP client for models.dev API
- **Configuration**: Harness-specific model mappings in ito config
- **Files affected**: Agent markdown files in `~/.config/opencode/agent/`, `.claude/agents/`, etc.
