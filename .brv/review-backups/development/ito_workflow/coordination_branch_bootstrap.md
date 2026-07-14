---
title: Coordination Branch Bootstrap
summary: Git coordination branch bootstrap uses an empty-tree root commit, rejects empty stdout, and applies object-format-aware hashing with SHA-256 support and SHA-1 fallback.
tags: []
related: [development/ito_workflow/coordination_branch_git_behavior.md, development/ito_workflow/coordination_branch_setup.md]
keywords: []
createdAt: '2026-07-13T18:18:50.145Z'
updatedAt: '2026-07-13T18:18:50.145Z'
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

**Flow:**
detect missing branch -> create empty-tree root commit -> reject empty stdout -> hash with object-format-aware logic -> push refs/heads/<branch>

**Timestamp:** 2026-07-13T18:18:34.426Z

## Narrative
### Structure
This knowledge concerns git branch bootstrap behavior for coordination and origin branch setup, with emphasis on commit creation and hashing safeguards.

### Dependencies
Relies on git object-format handling and safe interpretation of stdout from commit-tree or mktree commands.

### Highlights
Branch initialization must start from an empty-tree root commit, never from caller HEAD. The process is guarded by rejection of empty command output before any hash is used.

### Rules
Initialize missing origin branches from a clean empty-tree root commit rather than HEAD. Reject empty git stdout before using hashes.

### Examples
When bootstrapping a missing remote branch, create it from an empty-tree root commit and push the created commit as <oid>:refs/heads/<branch>.

## Facts
- **root_commit_bootstrap**: Initialization must use a root commit and must not include parent refs (-p). [convention]
- **branch_bootstrap_source**: The caller HEAD is never used for branch bootstrap. [convention]
- **origin_branch_bootstrap**: Missing origin branches should be initialized from a clean empty-tree root commit rather than HEAD. [project]
- **empty_stdout_validation**: Empty commit-tree or mktree stdout must be rejected before hashing or using the result. [convention]
- **object_format_hashing**: Coordination branch bootstrap is object-format-aware and supports SHA-256 with SHA-1 fallback. [project]
