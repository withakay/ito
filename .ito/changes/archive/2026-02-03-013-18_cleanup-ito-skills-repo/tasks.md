# Tasks for: 013-18_cleanup-ito-skills-repo

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential (simple cleanup)
- **Risk**: Low - removing unused files

```bash
ito tasks status 013-18_cleanup-ito-skills-repo
ito tasks next 013-18_cleanup-ito-skills-repo
ito tasks start 013-18_cleanup-ito-skills-repo 1.1
ito tasks complete 013-18_cleanup-ito-skills-repo 1.1
```

---

## Wave 1: Remove Unused Directories

- **Depends On**: None

### Task 1.1: Remove unused directories from ito-skills

- **Files**: `ito-skills/`
- **Dependencies**: None
- **Action**:
  - Remove `ito-skills/adapters/`
  - Remove `ito-skills/agents/`
  - Remove `ito-skills/commands/`
  - Remove `ito-skills/hooks/`
  - Remove `ito-skills/lib/`
  - Remove `ito-skills/tests/`
  - Remove `ito-skills/docs/`
  - Remove `ito-skills/.claude-plugin/`
  - Remove `ito-skills/.codex/`
  - Remove `ito-skills/.github/`
  - Remove `ito-skills/.opencode/`
- **Verify**: `ls ito-skills/` shows only skills/, LICENSE, .gitignore, .gitattributes
- **Done When**: Only skills/ and essential files remain
- **Updated At**: 2026-02-01
- **Status**: [x] complete

### Task 1.2: Remove unused files from ito-skills

- **Files**: `ito-skills/`
- **Dependencies**: Task 1.1
- **Action**:
  - Remove `ito-skills/README.md`
  - Remove `ito-skills/RELEASE-NOTES.md`
  - Keep `ito-skills/LICENSE`
  - Keep `ito-skills/.gitignore`
  - Keep `ito-skills/.gitattributes`
- **Verify**: `ls -la ito-skills/` shows minimal structure
- **Done When**: Only essential files remain
- **Updated At**: 2026-02-01
- **Status**: [x] complete

---

## Wave 2: Verification

- **Depends On**: Wave 1

### Task 2.1: Verify distribution still works

- **Files**: `ito-rs/crates/ito-core/src/distribution.rs`
- **Dependencies**: Task 1.2
- **Action**:
  - Run `cargo test -p ito-core` to verify distribution tests pass
  - Run `ito init` in a test directory to verify skills install correctly
  - Verify all 12 skills are present after init
- **Verify**: `cargo test -p ito-core && ito init --force && ls .opencode/skills/ito-*`
- **Done When**: All distribution tests pass, skills install correctly
- **Updated At**: 2026-02-01
- **Status**: [x] complete

### Task 2.2: Review and checkpoint

- **Type**: checkpoint (requires human approval)
- **Files**: `ito-skills/`
- **Dependencies**: Task 2.1
- **Action**:
  - Human review of cleaned-up structure
  - Confirm no needed files were removed
  - Approve for archive
- **Done When**: Human approves cleanup
- **Updated At**: 2026-02-01
- **Status**: [ ] pending

---

## Task Status Legend

- `[ ] pending` - Not started yet
- `[>] in-progress` - Currently being worked on
- `[x] complete` - Finished and verified
- `[-] shelved` - Intentionally not-to-be-done (reversible)
