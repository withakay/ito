# Tasks for: 013-13_merge-writing-plans-into-ito-write-change-proposal

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential

```bash
ito tasks status 013-13_merge-writing-plans-into-ito-write-change-proposal
ito tasks next 013-13_merge-writing-plans-into-ito-write-change-proposal
```

______________________________________________________________________

## Wave 1: Enhance ito-write-change-proposal

### Task 1.1: Add task granularity guidance to ito-write-change-proposal

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.opencode/skills/ito-write-change-proposal/SKILL.md`
- **Action**:
  - Add guidance on bite-sized tasks (2-5 min steps)
  - Explain why small tasks enable verification and steady progress
- **Done When**: ito-write-change-proposal describes task granularity best practices
- **Status**: [x] completed

### Task 1.2: Add TDD flow guidance to ito-write-change-proposal

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.opencode/skills/ito-write-change-proposal/SKILL.md`
- **Action**:
  - Add TDD flow for implementation tasks: failing test → run → implement → run → commit
  - Document why TDD ensures verifiable tasks
- **Done When**: ito-write-change-proposal includes TDD task structure
- **Status**: [x] completed

### Task 1.3: Add task structure best practices to ito-write-change-proposal

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.opencode/skills/ito-write-change-proposal/SKILL.md`
- **Action**:
  - Add guidance: tasks should specify exact file paths, what code to write, exact commands
  - Emphasize tasks should be self-contained and unambiguous
- **Done When**: ito-write-change-proposal includes task structure guidance
- **Status**: [x] completed

### Task 1.4: Add plan header guidance to ito-write-change-proposal

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.opencode/skills/ito-write-change-proposal/SKILL.md`
- **Action**:
  - Add guidance on documenting context: goal, architecture, tech stack
  - Reference how this maps to ito's proposal.md and design.md
- **Done When**: ito-write-change-proposal includes context documentation guidance
- **Status**: [x] completed

______________________________________________________________________

## Wave 2: Update referencing skills

### Task 2.1: Update subagent-driven-development references

- **Files**: `ito-skills/skills/subagent-driven-development/SKILL.md`
- **Action**:
  - Replace references to `writing-plans` with `ito-write-change-proposal`
  - Remove any remaining `superpowers:` prefixes
- **Verify**: `grep -E "writing-plans|superpowers:" ito-skills/skills/subagent-driven-development/SKILL.md` returns no results
- **Done When**: No legacy references remain
- **Status**: [x] completed

______________________________________________________________________

## Wave 3: Remove writing-plans

### Task 3.1: Delete writing-plans from ito-skills

- **Files**: `ito-skills/skills/writing-plans/`
- **Action**:
  - Remove entire directory
- **Verify**: `ls ito-skills/skills/writing-plans` fails
- **Done When**: Directory deleted
- **Status**: [x] completed

### Task 3.2: Delete writing-plans from embedded templates

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.opencode/skills/ito-writing-plans/`
- **Action**:
  - Remove entire directory
- **Verify**: `ls ito-rs/crates/ito-templates/assets/default/project/.opencode/skills/ito-writing-plans` fails
- **Done When**: Directory deleted
- **Status**: [x] completed

### Task 3.3: Remove writing-plans from distribution.rs

- **Files**: `ito-rs/crates/ito-core/src/distribution.rs`
- **Action**:
  - Remove `"writing-plans"` from ITO_SKILLS array
- **Verify**: `grep writing-plans ito-rs/crates/ito-core/src/distribution.rs` returns no results
- **Done When**: writing-plans removed from distribution
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
