<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: Skill-first lifecycle routing
The `ito` skill SHALL route lifecycle intent only to the six retained phase skills: `ito-proposal`, `ito-research`, `ito-apply`, `ito-review`, `ito-archive`, and `ito-loop`. It MUST NOT invent or discover a helper skill for each CLI command.

#### Scenario: Lifecycle intent matches retained phase
- **WHEN** a user asks to propose, research, apply, review, archive, or iterate on an Ito change
- **THEN** `ito` invokes the corresponding retained lifecycle skill with the original context
- **AND** the lifecycle skill obtains detailed policy from the appropriate Ito instruction artifact

#### Scenario: Helper-shaped intent is absorbed by a phase
- **WHEN** a request concerns intake, planning, tasks, worktrees, verification, memory, wiki, orchestration, path lookup, update, cleanup, or finish behavior
- **THEN** `ito` selects the lifecycle phase that owns that concern
- **AND** does not attempt to invoke a retired helper skill name

### Requirement: CLI fallback for unmatched commands
The `ito` skill SHALL invoke the Ito CLI when input names a supported CLI operation that is not a lifecycle phase. It MUST preserve all original command arguments.

#### Scenario: No lifecycle skill matches
- **WHEN** a user invokes a CLI operation such as `version`, `list`, `show`, `status`, `validate`, `config`, or `path`
- **THEN** `ito` invokes the Ito CLI directly
- **AND** all original arguments are passed unchanged

#### Scenario: Retired helper name is requested
- **WHEN** a user explicitly requests a retired helper skill
- **THEN** `ito` explains which retained lifecycle phase now owns the behavior
- **AND** routes to that phase or the direct CLI instruction instead of silently failing

### Requirement: Fixed lifecycle inventory replaces wildcard skill discovery
The `ito` skill SHALL use the canonical seven-skill inventory rather than querying, caching, or dynamically routing to every installed `ito-*` skill.

#### Scenario: Router initializes
- **WHEN** the `ito` skill is first invoked
- **THEN** it knows the six retained phase destinations from its managed policy
- **AND** it does not build a wildcard `ito-*` discovery cache

#### Scenario: User extension shares Ito prefix
- **GIVEN** a user-owned skill uses an `ito-*` name outside the canonical inventory
- **WHEN** the router selects a lifecycle destination
- **THEN** the user extension does not change the canonical routing table automatically
<!-- ITO:END -->
