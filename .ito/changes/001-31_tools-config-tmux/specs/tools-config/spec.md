<!-- ITO:START -->
## ADDED Requirements

### Requirement: Tools configuration namespace

The Ito configuration schema SHALL support a `tools` namespace for per-tool preferences. The `tools` namespace is designed to be extended for additional tools without structural changes.

#### Scenario: tools.tmux.enabled defaults to true

- **WHEN** `tools.tmux.enabled` is absent from all config sources
- **THEN** the system treats `tools.tmux.enabled` as `true`

#### Scenario: tools.tmux.enabled set to false suppresses tmux suggestions

- **WHEN** `tools.tmux.enabled` is `false` in the resolved config
- **THEN** any Ito workflow or command that would surface a tmux-specific option SHALL omit it
- **AND** `--viewer tmux-nvim` is rejected with: "tmux is disabled in config (tools.tmux.enabled = false)"

#### Scenario: tools.tmux.enabled set to true permits tmux suggestions

- **WHEN** `tools.tmux.enabled` is `true` in the resolved config
- **AND** the `tmux` binary is available on PATH
- **THEN** Ito workflows MAY surface tmux-specific options

#### Scenario: tools config key is the canonical workflow gate

- **WHEN** any Ito-generated instruction or interactive command would suggest a tmux-based workflow step
- **THEN** it MUST first check `tools.tmux.enabled`
- **AND** omit the suggestion entirely if the value is `false`
<!-- ITO:END -->
