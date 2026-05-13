---
title: Coordination Branch Setup
summary: Coordination branch setup creates a missing origin branch from an empty root commit, using the repository empty tree hash and commit-tree, with SHA-256 support and fallback to SHA-1.
tags: []
related: []
keywords: []
createdAt: '2026-05-10T19:06:10.828Z'
updatedAt: '2026-05-10T19:06:10.828Z'
---
## Reason
Document coordination branch initialization behavior in git.rs

## Raw Concept
**Task:**
Document coordination branch setup behavior in git.rs

**Changes:**
- RemoteMissing now creates a coordination branch from an empty root commit
- Empty tree hash selection supports sha256 and defaults to sha1 on probe failure
- Empty commit-tree stdout is rejected to avoid pushing a delete refspec

**Files:**
- ito-rs/crates/ito-core/src/git.rs

**Flow:**
fetch origin/<branch> -> if Ready return -> if RemoteMissing compute empty tree hash -> commit-tree with Initialize coordination branch -> trim stdout -> reject empty hash -> push to origin

**Timestamp:** 2026-05-10

**Patterns:**
- `^refs/heads/<branch>$` - Remote branch ref created on origin
- `^<commit-hash>:refs/heads/<branch>$` - Push refspec for initializing coordination branch

## Narrative
### Structure
git.rs classifies coordination git errors into NonFastForward, ProtectedBranch, RemoteRejected, RemoteMissing, RemoteNotConfigured, and CommandFailed. fetch_coordination_branch_with_runner maps missing refs to RemoteMissing, and ensure_coordination_branch_on_origin_with_runner uses that signal to bootstrap the missing origin branch.

### Dependencies
Relies on git rev-parse --show-object-format for object-format detection, git commit-tree for creating the bootstrap commit, and git push for publishing the branch.

### Highlights
The design ensures the remote coordination branch is initialized from a clean empty tree rather than HEAD, preventing inheritance of the repository main tree. It includes explicit safeguards for blank commit-tree output and test coverage for both SHA-1 and SHA-256 repositories.

### Rules
Reject empty commit-tree stdout so a delete refspec is never produced.

### Examples
When origin/<branch> is missing, the code creates a commit with message Initialize coordination branch on the empty tree hash and pushes it as refs/heads/<branch>.

## Facts
- **coordination_branch_initialization**: A missing origin coordination branch must be created from an empty root commit, not repository HEAD, so ito/internal/changes does not inherit the main tree. [project]
- **remote_missing_flow**: ensure_coordination_branch_on_origin_with_runner handles RemoteMissing by creating a branch from the repository empty tree hash, running commit-tree, and pushing the resulting commit hash to refs/heads/<branch>. [project]
- **empty_tree_default_format**: empty_tree_hash defaults to SHA-1 when object-format detection fails or returns anything other than sha256. [project]
- **sha1_empty_tree_hash**: The SHA-1 empty tree hash is 4b825dc642cb6eb9a060e54bf8d69288fbee4904. [project]
- **sha256_empty_tree_hash**: The SHA-256 empty tree hash is 6ef19b41225c5369f1c104d45d8d85efa9b057b53b14b4b9b939dd74decc5321. [project]
- **coordination_branch_commit_message**: The coordination branch initialization commit message is Initialize coordination branch. [project]
- **coordination_branch_push_refspec**: push_coordination_branch_with_runner pushes the local ref as <commit-hash>:refs/heads/<branch>. [project]
- **coordination_branch_tests**: Tests cover avoiding HEAD when creating the remote branch, SHA-256 empty-tree support, SHA-1 fallback when object-format detection fails, commit-tree failure reporting, and rejecting empty commit-tree stdout. [project]
