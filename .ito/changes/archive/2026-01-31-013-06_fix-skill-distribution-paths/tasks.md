# Tasks for: 013-06_fix-skill-distribution-paths

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Use the tasks CLI to drive status updates

```bash
ito tasks status 013-06_fix-skill-distribution-paths
ito tasks next 013-06_fix-skill-distribution-paths
ito tasks start 013-06_fix-skill-distribution-paths 1.1
ito tasks complete 013-06_fix-skill-distribution-paths 1.1
```

______________________________________________________________________

## Wave 1: Restructure Embedded Assets

- **Depends On**: None

### Task 1.1: Rename embedded skill folders with ito- prefix

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.opencode/skills/`
- **Dependencies**: None
- **Action**:
  - Move `ito-skills/brainstorming/` → `ito-brainstorming/`
  - Move `ito-skills/dispatching-parallel-agents/` → `ito-dispatching-parallel-agents/`
  - (etc. for all 14 skills)
  - Remove empty `ito-skills/` directory
- **Verify**: `ls ito-rs/crates/ito-templates/assets/default/project/.opencode/skills/ | grep ito-`
- **Done When**: All skills are directly under `skills/` with `ito-` prefix, no `ito-skills/` folder exists
- **Updated At**: 2026-01-31
- **Status**: [x] complete

______________________________________________________________________

## Wave 2: Update Distribution Code

- **Depends On**: Wave 1

### Task 2.1: Create ITO_SKILLS constant and ito_skills_manifests() function

- **Files**: `ito-rs/crates/ito-core/src/distribution.rs`
- **Dependencies**: None
- **Action**:
  - Added `ITO_SKILLS` const listing all 14 skill names
  - Created `ito_skills_manifests()` function that generates FileManifest entries with:
    - Source: `skills/<name>/SKILL.md` (relative to ito-skills/)
    - Dest: `ito-<name>/SKILL.md` (under target skills dir)
- **Verify**: `cargo test -p ito-core`
- **Done When**: Function generates correct manifests with ito- prefix
- **Updated At**: 2026-01-31
- **Status**: [x] complete

### Task 2.2: Fix opencode_manifests() to use flat structure

- **Files**: `ito-rs/crates/ito-core/src/distribution.rs`
- **Dependencies**: Task 2.1
- **Action**:
  - Changed to use `ito_skills_manifests(&skills_dir)` for flat structure
  - Skills go to `.opencode/skills/ito-<skill>/SKILL.md`
- **Verify**: `cargo test -p ito-core`
- **Done When**: OpenCode skills install to flat path structure with prefix
- **Updated At**: 2026-01-31
- **Status**: [x] complete

### Task 2.3: Add skill distribution to claude_manifests()

- **Files**: `ito-rs/crates/ito-core/src/distribution.rs`
- **Dependencies**: Task 2.1
- **Action**:
  - Added `ito_skills_manifests(&skills_dir)` call
  - Skills go to `.claude/skills/ito-<skill>/SKILL.md`
- **Verify**: `cargo test -p ito-core`
- **Done When**: Claude harness receives skills on `ito init --tools claude`
- **Updated At**: 2026-01-31
- **Status**: [x] complete

### Task 2.4: Add skill distribution to codex_manifests()

- **Files**: `ito-rs/crates/ito-core/src/distribution.rs`
- **Dependencies**: Task 2.1
- **Action**:
  - Added `ito_skills_manifests(&skills_dir)` call
  - Skills go to `.codex/skills/ito-<skill>/SKILL.md`
- **Verify**: `cargo test -p ito-core`
- **Done When**: Codex harness receives skills on `ito init --tools codex`
- **Updated At**: 2026-01-31
- **Status**: [x] complete

______________________________________________________________________

## Wave 3: Update Documentation

- **Depends On**: Wave 2

### Task 3.1: Rewrite README.opencode.md

- **Files**: `ito-skills/docs/README.opencode.md`
- **Dependencies**: None
- **Action**:
  - Removed all symlink instructions (symlinks are forbidden)
  - Documented the flat `ito-<skill>` structure
  - Explained skills are installed via `ito init --tools opencode`
  - Added cleanup instructions for old `skills/ito-skills/` path
- **Verify**: Read the file and confirm no symlink references exist
- **Done When**: Documentation is correct and mentions only copying/flat structure
- **Updated At**: 2026-01-31
- **Status**: [x] complete

______________________________________________________________________

## Wave 4: Verification

- **Depends On**: Wave 3

### Task 4.1: Build and test full distribution

- **Files**: N/A
- **Dependencies**: All prior tasks
- **Action**:
  - Ran `cargo build --workspace` - ✓ passed
  - Ran `cargo test --workspace` - ✓ passed
  - Tested `ito init --tools opencode` - ✓ skills installed to `.opencode/skills/ito-*`
  - Tested `ito init --tools claude` - ✓ skills installed to `.claude/skills/ito-*`
  - Tested `ito init --tools codex` - ✓ skills installed to `.codex/skills/ito-*`
- **Verify**: `cargo test --workspace && cargo build --release`
- **Done When**: All tests pass, manual verification confirms correct paths
- **Updated At**: 2026-01-31
- **Status**: [x] complete

______________________________________________________________________

## Task Status Legend

- `[ ] pending` - Not started yet
- `[ ] in-progress` - Currently being worked on
- `[x] complete` - Finished and verified
- `[-] shelved` - Intentionally not-to-be-done (reversible)
