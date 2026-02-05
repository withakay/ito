# Change: Change and Module Repository Pattern

## Why

Following the success of `TaskRepository` (005-11), we should apply the same pattern to Changes and Modules. Currently, change and module data access is scattered across the codebase:

- `ito-cli/src/app/list.rs` - reads change directories, loads proposals
- `ito-cli/src/app/common.rs` - resolves change IDs, loads change metadata
- `ito-cli/src/app/tasks.rs` - loads change paths
- `ito-cli/src/app/validate.rs` - iterates changes for validation
- `ito-cli/src/app/archive.rs` - loads change artifacts
- `ito-cli/src/app/show.rs` - loads change content
- `ito-cli/src/app/instructions.rs` - loads change artifacts

Each location has its own logic for:
- Finding change directories
- Loading proposal.md, design.md, tasks.md
- Parsing specs directories
- Determining artifact completeness

This duplication leads to inconsistencies, bugs, and makes it hard to add new features that need change data.

## What Changes

- Create `ChangeRepository` in `ito-workflow` for loading and querying changes
- Create `ModuleRepository` in `ito-workflow` for loading and querying modules
- Define domain models: `Change`, `ChangeSummary`, `Module`, `ModuleSummary`
- Migrate CLI commands to use repositories instead of direct path access
- Integrate `TaskRepository` into `ChangeRepository` for unified access

## Capabilities

### New Capabilities

- `change-repository`: Centralized access to change data with methods like `get()`, `list()`, `list_by_module()`, `list_incomplete()`, returning domain objects instead of raw paths
- `module-repository`: Centralized access to module data with methods like `get()`, `list()`, `list_with_changes()`, returning domain objects
- `change-domain-model`: Rich `Change` and `ChangeSummary` types encapsulating all change artifacts (proposal, design, specs, tasks) with computed properties (status, completeness)

### Modified Capabilities

- `list-command`: Refactored to use `ChangeRepository` and `ModuleRepository`
- `status-command`: Refactored to use `ChangeRepository`
- `validate-command`: Refactored to use `ChangeRepository`

## Impact

- **Architecture**: Establishes clean data access layer for core domain objects
- **Consistency**: Single source of truth for loading changes and modules
- **Testability**: Repositories can be mocked for unit testing
- **Performance**: Opportunity for caching loaded changes
- **Future Features**: Easier to add queries like "find all incomplete changes" or "find changes by status"
