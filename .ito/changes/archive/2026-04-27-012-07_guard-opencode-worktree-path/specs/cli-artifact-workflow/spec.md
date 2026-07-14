<!-- ITO:START -->
## ADDED Requirements

### Requirement: OpenCode pre-tool hook guards worktree path

When worktrees are enabled, the OpenCode Ito plugin SHALL run a quick pre-tool validation before relevant tool execution to reduce the chance that agents modify main/control or the wrong worktree.

- **Requirement ID**: cli-artifact-workflow:opencode-worktree-pretool-guard

#### Scenario: Pre-tool hook rejects main checkout for change work

- **GIVEN** `worktrees.enabled=true`
- **AND** the active change ID is `012-07_guard-opencode-worktree-path`
- **AND** OpenCode is running from the main/control checkout
- **WHEN** a tool invocation would read, write, edit, or run commands for change work
- **THEN** the plugin invokes the worktree validation command before allowing the tool
- **AND** the plugin prevents or warns against the tool use with instructions to move to the correct worktree

#### Scenario: Pre-tool hook allows matching worktree

- **GIVEN** `worktrees.enabled=true`
- **AND** the active change ID is `012-07_guard-opencode-worktree-path`
- **AND** OpenCode is running from a branch or path containing `012-07_guard-opencode-worktree-path`
- **WHEN** a relevant tool invocation occurs
- **THEN** the plugin allows the tool without additional user interaction

#### Scenario: Hook remains fast

- **GIVEN** the OpenCode pre-tool hook is enabled
- **WHEN** multiple tool invocations occur in the same session
- **THEN** the plugin caches successful validation briefly
- **AND** it avoids expensive repository scans on every tool invocation

#### Scenario: Missing active change is advisory

- **GIVEN** `worktrees.enabled=true`
- **AND** the plugin cannot determine an active change ID from the current instruction context or environment
- **WHEN** a relevant tool invocation occurs outside main/control
- **THEN** the plugin does not block solely because the change ID is unknown
- **AND** it may emit advisory guidance asking the agent to validate the current change worktree explicitly
<!-- ITO:END -->
