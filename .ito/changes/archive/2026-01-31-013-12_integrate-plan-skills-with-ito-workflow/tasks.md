# Tasks for: 013-12_integrate-plan-skills-with-ito-workflow

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential

```bash
ito tasks status 013-12_integrate-plan-skills-with-ito-workflow
ito tasks next 013-12_integrate-plan-skills-with-ito-workflow
```

______________________________________________________________________

## Wave 1: Enhance ito-apply-change-proposal

### Task 1.1: Add batch execution with checkpoints to ito-apply-change-proposal

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.opencode/skills/ito-apply-change-proposal/SKILL.md`
- **Action**:
  - Add batch execution pattern (default 3 tasks)
  - Add "report and wait for feedback" between batches
  - Document checkpoint flow
- **Done When**: ito-apply-change-proposal describes batch execution with review checkpoints
- **Status**: [x] completed

### Task 1.2: Add critical review step to ito-apply-change-proposal

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.opencode/skills/ito-apply-change-proposal/SKILL.md`
- **Action**:
  - Add pre-execution review step
  - Document raising concerns before starting
  - Require user confirmation or no concerns to proceed
- **Done When**: ito-apply-change-proposal includes critical review before execution
- **Status**: [x] completed

### Task 1.3: Add stop conditions to ito-apply-change-proposal

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.opencode/skills/ito-apply-change-proposal/SKILL.md`
- **Action**:
  - Add "When to stop and ask for help" section
  - List blockers: missing dependency, test fails, unclear instruction, repeated verification failure
  - Emphasize: stop and ask rather than guess
- **Done When**: ito-apply-change-proposal has explicit stop conditions
- **Status**: [x] completed

### Task 1.4: Add completion handoff to ito-apply-change-proposal

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.opencode/skills/ito-apply-change-proposal/SKILL.md`
- **Action**:
  - Add handoff to `ito-finishing-a-development-branch` after all tasks complete
  - Document the transition
- **Done When**: ito-apply-change-proposal hands off to completion skill
- **Status**: [x] completed

### Task 1.5: Add branch safety check to ito-apply-change-proposal

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.opencode/skills/ito-apply-change-proposal/SKILL.md`
- **Action**:
  - Add check for main/master branch
  - Require explicit consent before proceeding on protected branch
- **Done When**: ito-apply-change-proposal warns about protected branches
- **Status**: [x] completed

______________________________________________________________________

## Wave 2: Update referencing skills

### Task 2.1: Update writing-plans to reference ito-apply-change-proposal

- **Files**: `ito-skills/skills/writing-plans/SKILL.md`
- **Action**:
  - Replace references to `executing-plans` with `ito-apply-change-proposal`
  - Remove `superpowers:` prefix from any skill references
- **Verify**: `grep -E "executing-plans|superpowers:" ito-skills/skills/writing-plans/SKILL.md` returns no results
- **Done When**: writing-plans points to ito-apply-change-proposal
- **Status**: [x] completed

### Task 2.2: Update subagent-driven-development references

- **Files**: `ito-skills/skills/subagent-driven-development/SKILL.md`
- **Action**:
  - Remove all `superpowers:*` references
  - Replace `executing-plans` with `ito-apply-change-proposal`
  - Update to modern skill names
- **Verify**: `grep -E "executing-plans|superpowers:" ito-skills/skills/subagent-driven-development/SKILL.md` returns no results
- **Done When**: No legacy references remain
- **Status**: [x] completed

______________________________________________________________________

## Wave 3: Remove executing-plans

### Task 3.1: Delete executing-plans from ito-skills

- **Files**: `ito-skills/skills/executing-plans/`
- **Action**:
  - Remove entire directory
- **Verify**: `ls ito-skills/skills/executing-plans` fails
- **Done When**: Directory deleted
- **Status**: [x] completed

### Task 3.2: Delete executing-plans from embedded templates

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.opencode/skills/ito-executing-plans/`
- **Action**:
  - Remove entire directory
- **Verify**: `ls ito-rs/crates/ito-templates/assets/default/project/.opencode/skills/ito-executing-plans` fails
- **Done When**: Directory deleted
- **Status**: [x] completed

### Task 3.3: Remove executing-plans from distribution.rs

- **Files**: `ito-rs/crates/ito-core/src/distribution.rs`
- **Action**:
  - Remove `"executing-plans"` from ITO_SKILLS array
- **Verify**: `grep executing-plans ito-rs/crates/ito-core/src/distribution.rs` returns no results
- **Done When**: executing-plans removed from distribution
- **Status**: [x] completed

______________________________________________________________________

## Wave 4: Verification

### Task 4.1: Build and test

- **Action**:
  - Run `cargo build --workspace`
  - Run `cargo test --workspace`
- **Done When**: All tests pass
- **Status**: [x] completed

______________________________________________________________________

## Task Status Legend

- `[ ] pending` - Not started
- `[ ] in-progress` - Currently working
- `[x] complete` - Finished and verified
- `[-] shelved` - Deferred
