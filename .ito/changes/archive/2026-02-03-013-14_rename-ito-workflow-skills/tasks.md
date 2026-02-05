# Tasks for: 013-14_rename-ito-workflow-skills

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Note**: This change should be implemented BEFORE 013-12 and 013-13 to avoid double-renaming

```bash
ito tasks status 013-14_rename-ito-workflow-skills
ito tasks next 013-14_rename-ito-workflow-skills
```

______________________________________________________________________

## Wave 1: Rename skill directories

### Task 1.1: Rename ito-proposal to ito-write-change-proposal

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.opencode/skills/`
- **Action**:
  - `mv ito-proposal ito-write-change-proposal`
- **Verify**: Directory exists at new path, not at old path
- **Done When**: Skill directory renamed
- **Updated At**: 2026-02-01
- **Status**: [x] complete

### Task 1.2: Rename ito-apply to ito-apply-change-proposal

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.opencode/skills/`
- **Action**:
  - `mv ito-apply ito-apply-change-proposal`
- **Verify**: Directory exists at new path, not at old path
- **Done When**: Skill directory renamed
- **Updated At**: 2026-02-01
- **Status**: [x] complete

______________________________________________________________________

## Wave 2: Update skill frontmatter

### Task 2.1: Update ito-write-change-proposal SKILL.md

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.opencode/skills/ito-write-change-proposal/SKILL.md`
- **Action**:
  - Update `name` field to `ito-write-change-proposal`
  - Update `description` to: "Use when creating, designing, planning, proposing, specifying a feature, change, requirement, enhancement, fix, modification, or spec. Use when writing tasks, proposals, or specifications for new work."
- **Done When**: Frontmatter updated with new name and keyword-rich description
- **Updated At**: 2026-02-01
- **Status**: [x] complete

### Task 2.2: Update ito-apply-change-proposal SKILL.md

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.opencode/skills/ito-apply-change-proposal/SKILL.md`
- **Action**:
  - Update `name` field to `ito-apply-change-proposal`
  - Update `description` to: "Use when implementing, executing, applying, building, coding, or developing a feature, change, requirement, enhancement, fix, or modification. Use when running tasks from a spec or proposal."
- **Done When**: Frontmatter updated with new name and keyword-rich description
- **Updated At**: 2026-02-01
- **Status**: [x] complete

______________________________________________________________________

## Wave 3: Update ito router

### Task 3.1: Update ito skill routing

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.opencode/skills/ito/SKILL.md`
- **Action**:
  - Update routing to target `ito-write-change-proposal` and `ito-apply-change-proposal`
  - Keep short aliases working: `proposal` → `ito-write-change-proposal`, `apply` → `ito-apply-change-proposal`
- **Done When**: Router uses new skill names
- **Updated At**: 2026-02-01
- **Status**: [x] complete

______________________________________________________________________

## Wave 4: Update cross-references

### Task 4.1: Update all ito-* skill references

- **Files**: All files in `ito-rs/crates/ito-templates/assets/default/project/.opencode/skills/ito-*/SKILL.md`
- **Action**:
  - Replace `ito-proposal` with `ito-write-change-proposal`
  - Replace `ito-apply` with `ito-apply-change-proposal`
- **Verify**: `grep -r "ito-proposal\|ito-apply" ito-rs/crates/ito-templates/assets/default/project/.opencode/skills/` returns no results (except router aliases)
- **Done When**: All references updated
- **Updated At**: 2026-02-01
- **Status**: [x] complete

### Task 4.2: Update 013-12 and 013-13 proposals

- **Files**: `.ito/changes/013-12_*/`, `.ito/changes/013-13_*/`
- **Action**:
  - Update all references to use new skill names
- **Done When**: Proposals reference new names
- **Updated At**: 2026-02-01
- **Status**: [-] shelved

______________________________________________________________________

## Wave 5: Verification

### Task 5.1: Build and test

- **Action**:
  - Run `cargo build --workspace`
  - Run `cargo test --workspace`
  - Grep for old names to ensure none remain
- **Verify**: `grep -r "ito-proposal\|ito-apply" ito-rs/` returns only router alias mentions
- **Done When**: All tests pass, no stray old references
- **Updated At**: 2026-02-01
- **Status**: [x] complete

______________________________________________________________________

## Task Status Legend

- `[ ] pending` - Not started
- `[ ] in-progress` - Currently working
- `[x] complete` - Finished and verified
- `[-] shelved` - Deferred
