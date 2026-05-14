---
consolidated_at: '2026-05-13T19:07:50.153Z'
consolidated_from: [{date: '2026-05-13T19:07:50.153Z', path: development/ito_workflow/coordination_branch_setup.md, reason: 'These files cover the same coordination-branch bootstrap behavior in git.rs, including missing remote branch initialization from an empty tree/root commit, empty stdout rejection, push refspec shape, and error classification. The git-behavior file is richer and includes reservation flow details, so it should be the merge target.'}]
related: [development/ito_workflow/coordination_symlink_repair_and_sync.md]
---
# Coordination Branch Git Behavior

## Reason
Document coordination branch bootstrapping and reservation rules from git.rs

## Raw Concept
**Task:**
Document coordination branch initialization, fetch/push classification, and reservation flow behavior in git.rs

**Changes:**
- Added empty-tree-based coordination branch bootstrap for missing remote branches
- Rejected empty commit-tree and mktree output
- Defined coordination git error classification and reservation worktree flow
- Added SHA-256 empty-tree support with SHA-1 fallback for missing origin branch initialization

**Files:**
- ito-rs/crates/ito-core/src/git.rs

**Flow:**
detect missing remote branch -> create empty tree -> create root commit -> push init refspec; otherwise fetch and reserve via detached temp worktree

**Timestamp:** 2026-05-13

**Author:** ByteRover

**Patterns:**
- `^# Ito coordination worktree symlinks$` - Gitignore marker block for coordination symlinks
- `^refs/heads/<branch>$` - Remote branch ref created on origin
- `^<commit-hash>:refs/heads/<branch>$` - Push refspec for initializing coordination branch

## Narrative
### Structure
The module exposes fetch, push, reservation, and branch-setup helpers plus core wrappers, with dedicated cleanup for temporary worktrees.

### Dependencies
Relies on git worktree checks, remote fetch/push commands, branch-name validation, temporary worktree cleanup, and repository object-format detection for empty-tree hashing.

### Highlights
Branch setup returns Ready when the remote branch already exists and Created when it must be initialized. Push failures are classified for non-fast-forward, protected branch, remote rejected, remote missing, remote not configured, and command failures. Missing origin branches are bootstrapped from a clean empty root commit rather than HEAD, with SHA-256 support and SHA-1 fallback.

### Rules
git commit-tree for initialization must not include -p
The pushed ref must be <commit_hash>:refs/heads/<coordination-branch>
Initialization commit must be a root commit
Reject empty commit-tree stdout so a delete refspec is never produced.

### Examples
Branch bootstrap flow: fetch origin/<branch> -> if missing create empty tree commit -> trim stdout -> reject empty hash -> push init refspec.

## Facts
- **coordination_branch_bootstrap**: Coordination branch initialization must not use the caller’s HEAD. [convention]
- **coordination_branch_init_flow**: When origin/<coordination-branch> is missing, the branch is initialized by creating an empty tree with git mktree, creating a root commit with git commit-tree without -p, and pushing that commit to origin/<coordination-branch>. [project]
- **empty_hash_rejection**: Empty stdout from git mktree and git commit-tree must be rejected so a blank hash is never pushed. [convention]
- **reservation_branch_safety**: Reservation flows must ensure, fetch, and checkout the coordination branch before committing metadata to avoid leaking implementation history from the caller’s HEAD. [convention]
- **coordination_git_error_kinds**: The coordination git helper classifies remote missing, remote not configured, non-fast-forward, protected branch, remote rejected, and generic command failures. [project]
- **empty_tree_default_format**: empty_tree_hash defaults to SHA-1 when object-format detection fails or returns anything other than sha256. [project]
- **sha1_empty_tree_hash**: The SHA-1 empty tree hash is 4b825dc642cb6eb9a060e54bf8d69288fbee4904. [project]
- **sha256_empty_tree_hash**: The SHA-256 empty tree hash is 6ef19b41225c5369f1c104d45d8d85efa9b057b53b14b4b9b939dd74decc5321. [project]
- **coordination_branch_commit_message**: The coordination branch initialization commit message is Initialize coordination branch. [project]
- **coordination_branch_push_refspec**: push_coordination_branch_with_runner pushes the local ref as <commit-hash>:refs/heads/<branch>. [project]
- **coordination_branch_tests**: Tests cover avoiding HEAD when creating the remote branch, SHA-256 empty-tree support, SHA-1 fallback when object-format detection fails, commit-tree failure reporting, and rejecting empty commit-tree stdout. [project]
