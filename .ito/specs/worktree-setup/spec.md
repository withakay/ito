<!-- ITO:START -->
## ADDED Requirements

### Requirement: Setup command execution after worktree initialization

After include files are copied into a new worktree, the system SHALL execute the configured
setup command(s) inside the new worktree. Setup is defined by `worktrees.init.setup` in the
Ito config and is optional. When no setup is configured, this step is a no-op.

- **Requirement ID**: `worktree-setup:command-execution`

#### Scenario: Single command configured

- **WHEN** `worktrees.init.setup` is `"make init"` and a new worktree is initialized
- **THEN** the system runs `make init` with the new worktree as the working directory and
  streams stdout/stderr to the user

#### Scenario: Command list configured

- **WHEN** `worktrees.init.setup` is `["npm ci", "npm run build:types"]` and a new worktree is initialized
- **THEN** the system runs each command in order with the new worktree as the working directory;
  if any command exits non-zero, the error is reported and subsequent commands are not run

#### Scenario: No setup configured

- **WHEN** `worktrees.init.setup` is absent from the config
- **THEN** the initialization completes silently with no command execution

#### Scenario: Setup command refers to an included script

- **WHEN** `worktrees.init.setup` is `"./scripts/worktree-init.sh"` and that script was
  copied via the include list
- **THEN** the script is executed from the new worktree root after the copy step

### Requirement: Standalone re-run via `ito worktree setup`

The system SHALL provide `ito worktree setup --change <id>` as a standalone command that
re-runs the configured setup inside an existing worktree without recreating it or re-copying
files. This supports re-running after dependency changes (e.g. `package.json` updated).

- **Requirement ID**: `worktree-setup:standalone-rerun`

#### Scenario: Re-run on existing worktree

- **WHEN** `ito worktree setup --change <id>` is run on a worktree that already exists
- **THEN** the setup command(s) run in the worktree and exit with the command's exit code

#### Scenario: No setup configured — informative output

- **WHEN** `ito worktree setup --change <id>` is run and no setup command is configured
- **THEN** the command exits 0 with an informational message on stderr that no setup is configured

#### Scenario: Worktree does not exist

- **WHEN** `ito worktree setup --change <id>` is run and the worktree does not exist
- **THEN** the command exits non-zero with an error message directing the user to run
  `ito worktree ensure` first

### Requirement: `worktree-init` instruction artifact

The system SHALL provide `ito agent instruction worktree-init --change <id>` that emits
the initialization steps (file copy summary and setup commands) as human/agent-readable
text. This allows harnesses that cannot execute subprocesses to still know what steps are
needed and perform them manually.

- **Requirement ID**: `worktree-setup:instruction-artifact`

#### Scenario: Setup command present in output

- **WHEN** `ito agent instruction worktree-init --change <id>` is run and a setup command
  is configured
- **THEN** the output contains the target worktree path, the include file patterns, and the
  exact command(s) to execute

#### Scenario: No setup — no-op guidance

- **WHEN** `ito agent instruction worktree-init --change <id>` is run and no setup is configured
- **THEN** the output states that no additional setup is required after file copy

#### Scenario: Worktrees disabled — clear guidance

- **WHEN** `ito agent instruction worktree-init --change <id>` is run and `worktrees.enabled`
  is `false`
- **THEN** the output states that worktrees are not enabled for this project

### Requirement: Setup runs as part of `ito worktree ensure`

When `ito worktree ensure --change <id>` creates a new worktree, the setup command SHALL
run automatically after initialization. If the worktree already existed and was already
initialized, setup SHALL NOT run again automatically (to avoid re-running expensive installs
on every `ensure` call).

- **Requirement ID**: `worktree-setup:ensure-integration`

#### Scenario: New worktree — setup runs automatically

- **WHEN** `ito worktree ensure` creates a new worktree
- **THEN** include files are copied, then setup commands execute, then the worktree path
  is printed to stdout

#### Scenario: Existing worktree — setup skipped

- **WHEN** `ito worktree ensure` finds the worktree already exists
- **THEN** setup is NOT re-run; the path is printed to stdout immediately
<!-- ITO:END -->
