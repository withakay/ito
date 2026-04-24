<!-- ITO:START -->
## ADDED Requirements

### Requirement: Include-file resolution from config

When initializing a new worktree, the system SHALL read the `worktrees.init.include` list
from the resolved Ito config (a list of glob patterns) and copy all matching files from the
main worktree root into the new worktree root, preserving relative paths.

- **Requirement ID**: `worktree-init-files:config-include`

#### Scenario: Matching files copied

- **WHEN** a new worktree is initialized and `worktrees.init.include` contains `[".env", ".envrc"]`
- **THEN** `.env` and `.envrc` are copied from the main worktree into the new worktree root if they exist in the source

#### Scenario: Non-existent source file silently skipped

- **WHEN** a glob in `worktrees.init.include` matches no files in the source worktree
- **THEN** the initialization completes without error and no file is created in the destination

#### Scenario: Glob pattern expansion

- **WHEN** a glob pattern such as `"*.local.toml"` is listed in `worktrees.init.include`
- **THEN** all matching files in the main worktree root are copied to the new worktree

### Requirement: Include-file resolution from `.worktree-include` file

When initializing a new worktree, the system SHALL also check for a `.worktree-include`
file in the main worktree root. If present, it SHALL be parsed as a list of glob patterns
(one per line, `#`-prefixed comment lines and blank lines ignored) and those patterns SHALL
be added to the include set.

- **Requirement ID**: `worktree-init-files:file-include`

#### Scenario: File-based globs merged with config globs

- **WHEN** both `worktrees.init.include` and `.worktree-include` specify patterns
- **THEN** the union of both sets is used; a file matched by either source is copied

#### Scenario: `.worktree-include` absent — no error

- **WHEN** `.worktree-include` does not exist in the main worktree root
- **THEN** initialization proceeds using only the config-based include list

#### Scenario: Comment and blank line handling

- **WHEN** `.worktree-include` contains blank lines and lines starting with `#`
- **THEN** those lines are ignored and do not produce errors or spurious file copies

### Requirement: Include files copied before coordination symlinks

During worktree initialization, the include-file copy step SHALL complete before the
coordination-branch symlink step so that the worktree is fully usable (can build and test)
as soon as initialization finishes.

- **Requirement ID**: `worktree-init-files:init-ordering`

#### Scenario: Initialization order

- **WHEN** a new worktree is initialized
- **THEN** include files are copied first, then coordination-branch symlinks are created

### Requirement: Idempotent initialization

Running worktree initialization on an already-initialized worktree SHALL be safe. Existing
destination files SHALL be overwritten with the source versions; files not in the include
set SHALL not be deleted.

- **Requirement ID**: `worktree-init-files:idempotent`

#### Scenario: Re-initialization overwrites include files

- **WHEN** `ito worktree ensure --change <id>` is run on a worktree that already exists and already has a `.env` file
- **THEN** the `.env` file is overwritten from the source and no error is returned
<!-- ITO:END -->
