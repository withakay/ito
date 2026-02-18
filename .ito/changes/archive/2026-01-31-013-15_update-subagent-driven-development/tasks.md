# Tasks for: 013-15_update-subagent-driven-development

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Depends On**: 013-14 (rename skills) should be done first

```bash
ito tasks status 013-15_update-subagent-driven-development
ito tasks next 013-15_update-subagent-driven-development
```

______________________________________________________________________

## Wave 1: Remove deprecated references

### Task 1.1: Replace superpowers:* references

- **Files**: `ito-skills/skills/subagent-driven-development/SKILL.md`
- **Action**:
  - Replace all `superpowers:*` skill references with `ito-*` names
  - e.g., `superpowers:verification-before-completion` → `ito-verification-before-completion`
- **Verify**: `grep -i superpowers ito-skills/skills/subagent-driven-development/SKILL.md` returns no results
- **Done When**: No superpowers references remain
- **Status**: [x] completed

### Task 1.2: Replace executing-plans references

- **Files**: `ito-skills/skills/subagent-driven-development/SKILL.md`
- **Action**:
  - Replace `executing-plans` with `ito-apply-change-proposal`
- **Verify**: `grep executing-plans ito-skills/skills/subagent-driven-development/SKILL.md` returns no results
- **Done When**: No executing-plans references remain
- **Status**: [x] completed

### Task 1.3: Replace writing-plans references

- **Files**: `ito-skills/skills/subagent-driven-development/SKILL.md`
- **Action**:
  - Replace `writing-plans` with `ito-write-change-proposal`
- **Verify**: `grep writing-plans ito-skills/skills/subagent-driven-development/SKILL.md` returns no results
- **Done When**: No writing-plans references remain
- **Status**: [x] completed

______________________________________________________________________

## Wave 2: Update to ito workflow

### Task 2.1: Replace docs/plans/ with ito artifacts

- **Files**: `ito-skills/skills/subagent-driven-development/SKILL.md`
- **Action**:
  - Replace `docs/plans/` references with `.ito/changes/<id>/tasks.md`
- **Verify**: `grep "docs/plans" ito-skills/skills/subagent-driven-development/SKILL.md` returns no results
- **Done When**: No docs/plans references remain
- **Status**: [x] completed

### Task 2.2: Replace TodoWrite with ito tasks CLI

- **Files**: `ito-skills/skills/subagent-driven-development/SKILL.md`
- **Action**:
  - Replace `TodoWrite` with `ito tasks start/complete/shelve` commands
  - Update any task tracking examples
- **Verify**: `grep -i todowrite ito-skills/skills/subagent-driven-development/SKILL.md` returns no results
- **Done When**: No TodoWrite references remain
- **Status**: [x] completed

### Task 2.3: Update subagent context

- **Files**: `ito-skills/skills/subagent-driven-development/SKILL.md`
- **Action**:
  - Update subagent prompt to use `ito agent instruction apply --change <id>` for context
- **Done When**: Subagent context uses ito CLI
- **Status**: [x] completed

______________________________________________________________________

## Wave 3: Update embedded template

### Task 3.1: Sync embedded template

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.opencode/skills/ito-subagent-driven-development/SKILL.md`
- **Action**:
  - Copy updated skill from `ito-skills/skills/subagent-driven-development/SKILL.md`
- **Verify**: Files match
- **Done When**: Embedded template updated
- **Status**: [x] completed

______________________________________________________________________

## Wave 4: Verification

### Task 4.1: Verify no deprecated references

- **Action**:
  - `grep -E "superpowers:|executing-plans|writing-plans|docs/plans|TodoWrite" ito-skills/skills/subagent-driven-development/SKILL.md`
- **Done When**: Grep returns no results
- **Status**: [x] completed

______________________________________________________________________

## Task Status Legend

- `[ ] pending` - Not started
- `[>] in-progress` - Currently working
- `[x] complete` - Finished and verified
- `[-] shelved` - Deferred
