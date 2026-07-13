<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: Skill-first command routing
The `ito` skill SHALL route lifecycle intent only to the six retained phase skills: `ito-proposal`, `ito-research`, `ito-apply`, `ito-review`, `ito-archive`, and `ito-loop`. It MUST NOT invent or discover a helper skill for each CLI command.

#### Scenario: Lifecycle intent matches a retained phase
- **WHEN** a user asks to propose, research, apply, review, archive, or iterate on an Ito change
- **THEN** `ito` invokes the corresponding retained lifecycle skill with the original context
- **AND** the lifecycle skill obtains detailed policy from the appropriate Ito instruction artifact

#### Scenario: Helper-shaped intent is absorbed by a phase
- **WHEN** a request concerns intake, planning, tasks, worktrees, verification, memory, wiki, orchestration, path lookup, update, cleanup, or finish behavior
- **THEN** `ito` selects the lifecycle phase that owns that concern
- **AND** it does not attempt to invoke a retired helper skill name

### Requirement: CLI fallback for unmatched commands
The `ito` skill SHALL invoke the Ito CLI when input names a supported CLI operation that is not a lifecycle phase. A missing retained lifecycle skill SHALL be reported as an installation error rather than silently changing a lifecycle request into a different CLI workflow.

#### Scenario: No lifecycle skill matches
- **WHEN** a user invokes a CLI operation such as `version`, `list`, `show`, `status`, `validate`, `config`, or `path`
- **THEN** `ito` invokes the Ito CLI directly
- **AND** all original arguments are passed unchanged

#### Scenario: Retained lifecycle skill is missing
- **WHEN** a request matches a retained lifecycle phase but its canonical skill is unavailable
- **THEN** `ito` reports the missing managed lifecycle skill
- **AND** recommends refreshing the managed installation instead of routing through a retired helper or unrelated CLI command

### Requirement: Argument passthrough
The `ito` skill MUST pass every argument through unchanged and in its original order to the selected retained lifecycle skill or direct CLI target.

#### Scenario: Lifecycle arguments are preserved
- **WHEN** a user invokes a retained phase with a change ID and flags
- **THEN** `ito` invokes the retained lifecycle skill with the same change ID and flags
- **AND** no argument is reordered, rewritten, or discarded

#### Scenario: CLI fallback arguments are preserved
- **WHEN** a user invokes a direct CLI operation with subcommands and flags
- **THEN** `ito` passes the complete original argument sequence to the CLI

## REMOVED Requirements

### Requirement: Skill discovery
The `ito` skill SHALL discover installed `ito-*` skills dynamically and cache the result for routing.

**Reason**: Wildcard discovery makes the user-facing surface grow with every helper and allows harness-local state to redefine the lifecycle contract.
**Migration**: Route through the fixed canonical lifecycle inventory. User/project extensions remain installable but do not alter Ito's managed routing table automatically.

#### Scenario: Router uses the fixed lifecycle inventory
- **WHEN** `ito` initializes routing
- **THEN** it uses the six retained phase destinations from managed policy
- **AND** it does not query or cache every installed `ito-*` skill
<!-- ITO:END -->
