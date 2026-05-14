## Key points
- `git.rs` documents and implements coordination branch initialization for a missing remote origin branch.
- When `fetch_coordination_branch_with_runner` returns `RemoteMissing`, the system bootstraps the branch from an **empty root commit** rather than from `HEAD`, avoiding inheritance of the repository main tree.
- The empty tree hash is chosen based on repository object format: **SHA-256** is supported explicitly, with **SHA-1 as the fallback/default** when detection fails or returns another format.
- Initialization uses `git commit-tree` with the message **"Initialize coordination branch"**, then trims stdout and rejects blank output to prevent accidentally generating a delete refspec.
- The resulting commit is pushed with the refspec pattern **`<commit-hash>:refs/heads/<branch>`**.
- Error handling categorizes coordination git failures into: `NonFastForward`, `ProtectedBranch`, `RemoteRejected`, `RemoteMissing`, `RemoteNotConfigured`, and `CommandFailed`.

## Structure / sections summary
- **Reason**: States the goal—document coordination branch initialization behavior in `git.rs`.
- **Raw Concept**:
  - Summarizes the change set and main flow:
    1. fetch `origin/<branch>`
    2. if ready, return
    3. if missing, compute empty tree hash
    4. run `commit-tree`
    5. trim and validate stdout
    6. push to origin
  - Lists affected file: `ito-rs/crates/ito-core/src/git.rs`
  - Defines ref patterns for origin branches and push refspecs.
- **Narrative**:
  - **Structure**: Describes how `git.rs` classifies coordination git errors and how missing refs are handled.
  - **Dependencies**: Notes reliance on `git rev-parse --show-object-format`, `git commit-tree`, and `git push`.
  - **Highlights**: Emphasizes empty-tree initialization, no inheritance from `HEAD`, and safeguards against blank output.
  - **Rules**: Explicitly forbids empty `commit-tree` stdout to avoid a delete refspec.
  - **Examples**: Shows the branch-creation flow for a missing `origin/<branch>`.
- **Facts**:
  - Enumerates canonical hashes for SHA-1 and SHA-256 empty trees.
  - Confirms the initialization commit message and push refspec.
  - Notes test coverage for SHA-1/SHA-256 behavior, fallback behavior, failure reporting, and empty-output rejection.

## Notable entities, patterns, or decisions
- **Entities**:
  - `git.rs`
  - `fetch_coordination_branch_with_runner`
  - `ensure_coordination_branch_on_origin_with_runner`
  - `push_coordination_branch_with_runner`
  - `git rev-parse --show-object-format`
  - `git commit-tree`
  - `git push`
- **Ref/regex patterns**:
  - Remote branch ref: `^refs/heads/<branch>$`
  - Push refspec: `^<commit-hash>:refs/heads/<branch>$`
- **Important constants**:
  - SHA-1 empty tree hash: `4b825dc642cb6eb9a060e54bf8d69288fbee4904`
  - SHA-256 empty tree hash: `6ef19b41225c5369f1c104d45d8d85efa9b057b53b14b4b9b939dd74decc5321`
- **Design decision**:
  - Missing coordination branches are initialized from a clean empty tree, not from existing repository content.
- **Safety decision**:
  - Empty `commit-tree` stdout is rejected to avoid producing a delete refspec.
