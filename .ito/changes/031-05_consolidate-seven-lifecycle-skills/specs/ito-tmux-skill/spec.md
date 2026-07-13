<!-- ITO:START -->
## REMOVED Requirements

### Requirement: Tmux skill is distributed with Ito
The system SHALL embed and install a managed tmux skill alongside other Ito-managed skills.

**Reason**: Tmux control is outside Ito's core spec-driven lifecycle and would violate the exact seven-skill inventory.
**Migration**: Use user-installed terminal automation outside Ito when needed; Ito provides no replacement tmux lifecycle skill.

#### Scenario: Init omits tmux integration
- **WHEN** a supported harness is initialized or updated
- **THEN** no `tmux` or `ito-tmux` skill directory is emitted

### Requirement: Tmux skill includes helper scripts
The installed tmux skill SHALL include Bash helper scripts for session discovery and pane polling.

**Reason**: The tmux skill and its integration are removed rather than feature-gated or folded into a lifecycle phase.
**Migration**: Ownership-aware cleanup removes unmodified managed copies; user-owned scripts are preserved as non-Ito extensions.

#### Scenario: Managed helper scripts are retired
- **WHEN** update encounters unmodified Ito-managed tmux helper scripts at known legacy paths
- **THEN** it removes those scripts and empty managed directories
<!-- ITO:END -->
