---
title: Coordination Branch Bootstrap
summary: Git coordination branch bootstrap uses an empty-tree root commit, rejects empty stdout, and applies object-format-aware hashing with SHA-256 support and SHA-1 fallback.
tags: []
related: [development/ito_workflow/coordination_branch_git_behavior.md, development/ito_workflow/coordination_branch_setup.md]
keywords: []
createdAt: '2026-07-13T18:18:50.145Z'
updatedAt: '2026-07-13T18:18:50.145Z'
consolidated_at: '2026-07-13T18:45:02.738Z'
consolidated_from: [{date: '2026-07-13T18:45:02.738Z', path: development/ito_workflow/coordination_branch_git_behavior.md, reason: 'These files all document the same coordination-branch bootstrap behavior: missing origin branches are initialized from an empty-tree root commit, empty git stdout is rejected, object-format-aware hashing is used, and the push refspec format is specified. The bootstrap note is the richest concrete source, while the setup and git behavior docs are overlapping variants that should be consolidated to avoid duplicate rules and facts.'}, {date: '2026-07-13T18:45:02.738Z', path: development/ito_workflow/coordination_branch_setup.abstract.md, reason: 'These files all document the same coordination-branch bootstrap behavior: missing origin branches are initialized from an empty-tree root commit, empty git stdout is rejected, object-format-aware hashing is used, and the push refspec format is specified. The bootstrap note is the richest concrete source, while the setup and git behavior docs are overlapping variants that should be consolidated to avoid duplicate rules and facts.'}, {date: '2026-07-13T18:45:02.738Z', path: development/ito_workflow/coordination_branch_setup.overview.md, reason: 'These files all document the same coordination-branch bootstrap behavior: missing origin branches are initialized from an empty-tree root commit, empty git stdout is rejected, object-format-aware hashing is used, and the push refspec format is specified. The bootstrap note is the richest concrete source, while the setup and git behavior docs are overlapping variants that should be consolidated to avoid duplicate rules and facts.'}, {date: '2026-07-13T18:45:02.738Z', path: development/ito_workflow/coordination_branch_bootstrap.abstract.md, reason: 'These files all document the same coordination-branch bootstrap behavior: missing origin branches are initialized from an empty-tree root commit, empty git stdout is rejected, object-format-aware hashing is used, and the push refspec format is specified. The bootstrap note is the richest concrete source, while the setup and git behavior docs are overlapping variants that should be consolidated to avoid duplicate rules and facts.'}, {date: '2026-07-13T18:45:02.738Z', path: development/ito_workflow/coordination_branch_bootstrap.overview.md, reason: 'These files all document the same coordination-branch bootstrap behavior: missing origin branches are initialized from an empty-tree root commit, empty git stdout is rejected, object-format-aware hashing is used, and the push refspec format is specified. The bootstrap note is the richest concrete source, while the setup and git behavior docs are overlapping variants that should be consolidated to avoid duplicate rules and facts.'}]
---
## Reason
Curate git bootstrap and hashing rules from RLM context

## Raw Concept
**Task:**
Document git coordination branch bootstrap rules and hash handling

**Changes:**
- Reinforced empty-tree root commit bootstrap for missing origin branches
- Clarified that parent refs must not be included during initialization
- Added validation to reject empty git stdout before hashing
- Captured object-format-aware hashing behavior with SHA-256 support and SHA-1 fallback
- Documented coordination branch setup behavior when `fetch_coordination_branch_with_runner` returns `RemoteMissing`
- Documented the push refspec and initialization commit message used for branch creation
- Recorded that missing origin branches are initialized from a clean empty-tree root commit rather than from HEAD

**Flow:**
detect missing branch -> create empty-tree root commit -> reject empty stdout -> hash with object-format-aware logic -> push refs/heads/<branch>

**Timestamp:** 2026-07-13T18:18:34.426Z

## Narrative
### Structure
This knowledge concerns git branch bootstrap behavior for coordination and origin branch setup, with emphasis on commit creation, hashing safeguards, fetch/push classification, and reservation flow rules.

### Dependencies
Relies on git object-format handling, safe interpretation of stdout from commit-tree or mktree commands, remote fetch/push commands, branch-name validation, and detached temporary worktree cleanup.

### Highlights
Branch initialization must start from an empty-tree root commit, never from caller HEAD. The process is guarded by rejection of empty command output before any hash is used. Branch setup returns Ready when the remote branch already exists and Created when it must be initialized.

### Rules
Initialize missing origin branches from a clean empty-tree root commit rather than HEAD. Reject empty git stdout before using hashes. `git commit-tree` for initialization must not include `-p`. The pushed ref must be `<commit_hash>:refs/heads/<coordination-branch>`. Initialization commit must be a root commit.

### Examples
When bootstrapping a missing remote branch, create it from an empty-tree root commit and push the created commit as `<oid>:refs/heads/<branch>`. Branch bootstrap flow: fetch `origin/<branch>` -> if missing create empty tree commit -> trim stdout -> reject empty hash -> push init refspec.

## Facts
- **root_commit_bootstrap**: Initialization must use a root commit and must not include parent refs (`-p`). [convention]
- **branch_bootstrap_source**: The caller HEAD is never used for branch bootstrap. [convention]
- **origin_branch_bootstrap**: Missing origin branches should be initialized from a clean empty-tree root commit rather than HEAD. [project]
- **empty_stdout_validation**: Empty commit-tree or mktree stdout must be rejected before hashing or using the result. [convention]
- **object_format_hashing**: Coordination branch bootstrap is object-format-aware and supports SHA-256 with SHA-1 fallback. [project]
- **coordination_branch_init_flow**: When origin/<coordination-branch> is missing, the branch is initialized by creating an empty tree with git mktree, creating a root commit with git commit-tree without `-p`, and pushing that commit to origin/<coordination-branch>. [project]
- **coordination_git_error_kinds**: The coordination git helper classifies remote missing, remote not configured, non-fast-forward, protected branch, remote rejected, and generic command failures. [project]
- **empty_tree_default_format**: empty_tree_hash defaults to SHA-1 when object-format detection fails or returns anything other than sha256. [project]
- **sha1_empty_tree_hash**: The SHA-1 empty tree hash is `4b825dc642cb6eb9a060e54bf8d69288fbee4904`. [project]
- **sha256_empty_tree_hash**: The SHA-256 empty tree hash is `6ef19b41225c5369f1c104d45d8d85efa9b057b53b14b4b9b939dd74decc5321`. [project]
- **coordination_branch_commit_message**: The coordination branch initialization commit message is `Initialize coordination branch`. [project]
- **coordination_branch_push_refspec**: `push_coordination_branch_with_runner` pushes the local ref as `<commit-hash>:refs/heads/<branch>`. [project]
- **coordination_branch_tests**: Tests cover avoiding HEAD when creating the remote branch, SHA-256 empty-tree support, SHA-1 fallback when object-format detection fails, commit-tree failure reporting, and rejecting empty commit-tree stdout. [project]

## Related
- `development/ito_workflow/coordination_branch_git_behavior.md`
- `development/ito_workflow/coordination_branch_setup.md`