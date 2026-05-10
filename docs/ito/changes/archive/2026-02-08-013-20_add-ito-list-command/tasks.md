# Tasks for: 013-20_add-ito-list-command

## Execution Notes

- **Tool**: OpenCode
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking

```bash
ito tasks status 013-20_add-ito-list-command
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Create ito-list skill

- **Files**: ito-rs/crates/ito-templates/assets/skills/ito-list/SKILL.md
- **Dependencies**: None
- **Action**:
  Create SKILL.md with CLI reference for `ito list` (all flags: --changes, --specs, --modules, --ready, --completed, --partial, --pending, --sort, --json), workflow steps (parse intent, run CLI, interpret results, suggest next actions), and examples.
- **Verify**: `cargo build -p ito-cli`
- **Done When**: Skill file exists and is embedded at compile time
- **Updated At**: 2026-02-08
- **Status**: [x] complete

### Task 1.2: Create ito-list command template

- **Files**: ito-rs/crates/ito-templates/assets/commands/ito-list.md
- **Dependencies**: None
- **Action**:
  Create command template with YAML frontmatter (name, description, category, tags), $ARGUMENTS capture, managed block delegating to the ito-list skill, and fallback guardrail.
- **Verify**: `cargo build -p ito-cli`
- **Done When**: Command file exists and is embedded at compile time
- **Updated At**: 2026-02-08
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Verify distribution and tests

- **Files**: None
- **Dependencies**: None
- **Action**:
  Verify that existing distribution tests pass — `all_manifests_use_embedded_assets` confirms the new files are correctly embedded and installable to all harnesses.
- **Verify**: `cargo test -p ito-core --test distribution`
- **Done When**: All 7 distribution tests pass
- **Updated At**: 2026-02-08
- **Status**: [x] complete
