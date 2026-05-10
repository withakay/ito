## Why

Users currently must type exact module and change IDs with proper zero-padding (e.g., `001-02_my-change`). This creates friction - users shouldn't need to remember padding conventions. Additionally, when running `/ito-proposal` without a module specified, the workflow should offer smart defaults rather than requiring the user to look up module IDs manually.

## What Changes

- **Interactive module selection**: When `/ito-proposal` is invoked without a module ID, prompt the user with choices:
  - Use the last worked-on module
  - Create a new module
  - Use ungrouped (module 000)
- **Flexible ID parsing**: Accept various ID formats and normalize them:
  - `001`, `1` → module `001`
  - `001_foo`, `1_foo` → module `001_foo`
  - `1-2_bar`, `001-02_bar`, `1-00003_bar` → change `001-02_bar` or `001-03_bar`
- **Update documentation**: Reflect these UX improvements in `docs/agent-workflow.md`

## Capabilities

### New Capabilities

- `flexible-id-parser`: Regex-based parser that accepts loose ID formats and normalizes them to canonical padded format. Handles module IDs (NNN), change IDs (NNN-NN_name), and mixed formats.
- `interactive-module-selection`: Skill enhancement that prompts users for module choice when not specified, offering last-used, new, or ungrouped options.

### Modified Capabilities

- `agent-workflow-docs`: Update documentation to describe flexible ID input and interactive module selection

## Impact

- **CLI**: All commands accepting module/change IDs will use the new parser
- **Skills**: `ito-proposal` skill updated with interactive prompts
- **Docs**: `docs/agent-workflow.md` updated with new input formats
- **Tests**: Comprehensive test coverage for ID parsing edge cases
