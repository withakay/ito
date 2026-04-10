use super::*;
use crate::process::{ProcessExecutionError, ProcessOutput};
use std::cell::RefCell;
use std::collections::VecDeque;

// ── Stub runner ───────────────────────────────────────────────────────────────

struct StubRunner {
    outputs: RefCell<VecDeque<Result<ProcessOutput, ProcessExecutionError>>>,
    /// Records the args of each invocation for assertion.
    calls: RefCell<Vec<Vec<String>>>,
}

impl StubRunner {
    fn with_outputs(outputs: Vec<Result<ProcessOutput, ProcessExecutionError>>) -> Self {
        Self {
            outputs: RefCell::new(outputs.into()),
            calls: RefCell::new(Vec::new()),
        }
    }
}

impl ProcessRunner for StubRunner {
    fn run(&self, request: &ProcessRequest) -> Result<ProcessOutput, ProcessExecutionError> {
        self.calls.borrow_mut().push(request.args.clone());
        self.outputs
            .borrow_mut()
            .pop_front()
            .expect("StubRunner ran out of queued outputs")
    }

    fn run_with_timeout(
        &self,
        _request: &ProcessRequest,
        _timeout: std::time::Duration,
    ) -> Result<ProcessOutput, ProcessExecutionError> {
        unreachable!("not used in coordination_worktree tests")
    }
}

fn ok(stdout: &str) -> Result<ProcessOutput, ProcessExecutionError> {
    Ok(ProcessOutput {
        exit_code: 0,
        success: true,
        stdout: stdout.to_string(),
        stderr: String::new(),
        timed_out: false,
    })
}

fn fail(stderr: &str) -> Result<ProcessOutput, ProcessExecutionError> {
    Ok(ProcessOutput {
        exit_code: 1,
        success: false,
        stdout: String::new(),
        stderr: stderr.to_string(),
        timed_out: false,
    })
}

// ── create: branch already exists locally ────────────────────────────────────

#[test]
fn create_uses_existing_local_branch() {
    let tmp = tempfile::TempDir::new().unwrap();
    let target = tmp.path().join("wt");

    // Sequence:
    //   1. rev-parse --verify  → success (branch exists locally)
    //   2. worktree add        → success
    let runner = StubRunner::with_outputs(vec![ok("abc123"), ok("")]);

    create_coordination_worktree_with_runner(&runner, tmp.path(), "coord/main", &target).unwrap();

    let calls = runner.calls.borrow();
    // First call must be rev-parse --verify
    assert_eq!(calls[0], ["rev-parse", "--verify", "coord/main"]);
    // Second call must be worktree add (no fetch, no orphan)
    assert_eq!(calls[1][0], "worktree");
    assert_eq!(calls[1][1], "add");

    // .ito subdirs must exist
    for subdir in ITO_SUBDIRS {
        assert!(
            target.join(".ito").join(subdir).exists(),
            "missing .ito/{subdir}"
        );
    }
}

// ── create: branch fetched from origin ───────────────────────────────────────

#[test]
fn create_fetches_branch_from_origin_when_not_local() {
    let tmp = tempfile::TempDir::new().unwrap();
    let target = tmp.path().join("wt");

    // Sequence:
    //   1. rev-parse --verify  → fail (not local)
    //   2. fetch origin        → success (exists on remote)
    //   3. worktree add        → success
    let runner = StubRunner::with_outputs(vec![fail(""), ok(""), ok("")]);

    create_coordination_worktree_with_runner(&runner, tmp.path(), "coord/main", &target).unwrap();

    let calls = runner.calls.borrow();
    assert_eq!(calls[1], ["fetch", "origin", "coord/main"]);
    assert_eq!(calls[2][0], "worktree");
}

// ── create: orphan branch when neither local nor remote ──────────────────────

#[test]
fn create_makes_orphan_branch_when_not_on_remote() {
    let tmp = tempfile::TempDir::new().unwrap();
    let target = tmp.path().join("wt");

    // Sequence (primary path: git worktree add --orphan succeeds):
    //   1. rev-parse --verify              → fail (not local)
    //   2. fetch origin                    → fail with "couldn't find remote ref"
    //   3. worktree add --orphan           → success
    //   4. worktree remove --force <tmp>   → success (cleanup tmp worktree)
    //   5. worktree prune                  → success (cleanup metadata)
    //   6. worktree add <target> <branch>  → success
    let runner = StubRunner::with_outputs(vec![
        fail(""),
        fail("fatal: couldn't find remote ref coord/main"),
        ok(""),
        ok(""),
        ok(""),
        ok(""),
    ]);

    create_coordination_worktree_with_runner(&runner, tmp.path(), "coord/main", &target).unwrap();

    let calls = runner.calls.borrow();
    // worktree add --orphan
    assert_eq!(calls[2][0], "worktree");
    assert_eq!(calls[2][1], "add");
    assert!(
        calls[2].contains(&"--orphan".to_string()),
        "should use --orphan: {:?}",
        calls[2]
    );
    assert!(
        calls[2].contains(&"coord/main".to_string()),
        "should name the branch: {:?}",
        calls[2]
    );
    // final worktree add for the real target
    assert_eq!(calls[5][0], "worktree");
    assert_eq!(calls[5][1], "add");
}

#[test]
fn create_makes_orphan_branch_via_commit_tree_fallback() {
    let tmp = tempfile::TempDir::new().unwrap();
    let target = tmp.path().join("wt");

    // Sequence (fallback path: worktree add --orphan not supported):
    //   1. rev-parse --verify              → fail (not local)
    //   2. fetch origin                    → fail with "couldn't find remote ref"
    //   3. worktree add --orphan           → fail (old git)
    //   4. rev-parse --show-object-format  → ok("sha1")
    //   5. commit-tree <hash> -m "..."     → ok("deadbeef...")
    //   6. branch <branch> <commit>        → success
    //   7. worktree add <target> <branch>  → success
    let runner = StubRunner::with_outputs(vec![
        fail(""),
        fail("fatal: couldn't find remote ref coord/main"),
        fail("error: unknown option `--orphan'"),
        ok("sha1\n"),
        ok("deadbeef1234567890abcdef1234567890abcdef"),
        ok(""),
        ok(""),
    ]);

    create_coordination_worktree_with_runner(&runner, tmp.path(), "coord/main", &target).unwrap();

    let calls = runner.calls.borrow();
    assert_eq!(calls[3], ["rev-parse", "--show-object-format"]);
    assert_eq!(calls[4][0], "commit-tree");
    assert_eq!(calls[4][1], "4b825dc642cb6eb9a060e54bf8d69288fbee4904");
    assert!(
        calls[4].contains(&"Initialize coordination branch".to_string()),
        "commit-tree should include the init message: {:?}",
        calls[4]
    );
    // branch creation
    assert_eq!(calls[5][0], "branch");
    assert_eq!(calls[5][1], "coord/main");
    // final worktree add
    assert_eq!(calls[6][0], "worktree");
    assert_eq!(calls[6][1], "add");
}

#[test]
fn create_makes_orphan_branch_via_commit_tree_fallback_in_sha256_repo() {
    let tmp = tempfile::TempDir::new().unwrap();
    let target = tmp.path().join("wt");

    let runner = StubRunner::with_outputs(vec![
        fail(""),
        fail("fatal: couldn't find remote ref coord/main"),
        fail("error: unknown option `--orphan'"),
        ok("sha256\n"),
        ok("deadbeef1234567890abcdef1234567890abcdef"),
        ok(""),
        ok(""),
    ]);

    create_coordination_worktree_with_runner(&runner, tmp.path(), "coord/main", &target).unwrap();

    let calls = runner.calls.borrow();
    assert_eq!(calls[3], ["rev-parse", "--show-object-format"]);
    assert_eq!(calls[4][0], "commit-tree");
    assert_eq!(
        calls[4][1],
        "6ef19b41225c5369f1c104d45d8d85efa9b057b53b14b4b9b939dd74decc5321"
    );
}

// ── create: fetch fails for unexpected reason ─────────────────────────────────

#[test]
fn create_returns_error_when_fetch_fails_unexpectedly() {
    let tmp = tempfile::TempDir::new().unwrap();
    let target = tmp.path().join("wt");

    // Use an error that is neither "remote ref missing" nor "no remote" —
    // e.g. an SSH authentication failure.
    let runner = StubRunner::with_outputs(vec![
        fail(""),
        fail("fatal: Could not read from remote repository (permission denied)"),
    ]);

    let err = create_coordination_worktree_with_runner(&runner, tmp.path(), "coord/main", &target)
        .unwrap_err();

    let msg = err.to_string();
    assert!(msg.contains("coord/main"), "should name the branch: {msg}");
    assert!(msg.contains("origin"), "should mention origin: {msg}");
}

// ── create: no origin remote falls through to orphan ─────────────────────────

#[test]
fn create_makes_orphan_when_origin_not_configured() {
    let tmp = tempfile::TempDir::new().unwrap();
    let target = tmp.path().join("wt");

    // Sequence (primary path: git worktree add --orphan succeeds):
    //   1. rev-parse --verify              → fail (not local)
    //   2. fetch origin                    → fail with "does not appear to be a git repository"
    //   3. worktree add --orphan           → success
    //   4. worktree remove --force <tmp>   → success (cleanup)
    //   5. worktree prune                  → success (cleanup)
    //   6. worktree add <target> <branch>  → success
    let runner = StubRunner::with_outputs(vec![
        fail(""),
        fail("fatal: 'origin' does not appear to be a git repository"),
        ok(""),
        ok(""),
        ok(""),
        ok(""),
    ]);

    create_coordination_worktree_with_runner(&runner, tmp.path(), "coord/main", &target).unwrap();

    let calls = runner.calls.borrow();
    // Should have fallen through to orphan creation via worktree add --orphan
    assert_eq!(calls[2][0], "worktree");
    assert_eq!(calls[2][1], "add");
    assert!(
        calls[2].contains(&"--orphan".to_string()),
        "should use --orphan: {:?}",
        calls[2]
    );
}

// ── create: worktree add fails ────────────────────────────────────────────────

#[test]
fn create_returns_error_when_worktree_add_fails() {
    let tmp = tempfile::TempDir::new().unwrap();
    let target = tmp.path().join("wt");

    let runner = StubRunner::with_outputs(vec![ok("abc123"), fail("fatal: 'wt' already exists")]);

    let err = create_coordination_worktree_with_runner(&runner, tmp.path(), "coord/main", &target)
        .unwrap_err();

    let msg = err.to_string();
    assert!(msg.contains("coord/main"), "should name the branch: {msg}");
    assert!(
        msg.contains("already exists") || msg.contains("worktree"),
        "msg: {msg}"
    );
}

// ── create: orphan commit-tree fails (fallback path) ─────────────────────────

#[test]
fn create_returns_error_when_orphan_commit_fails() {
    let tmp = tempfile::TempDir::new().unwrap();
    let target = tmp.path().join("wt");

    // Simulate: worktree add --orphan not supported, then commit-tree fails
    // (e.g. git user identity not configured).
    let runner = StubRunner::with_outputs(vec![
        fail(""),                                           // rev-parse --verify
        fail("fatal: couldn't find remote ref coord/main"), // fetch
        fail("error: unknown option `--orphan'"),           // worktree add --orphan
        ok("sha1\n"),                                       // rev-parse --show-object-format
        fail("error: unable to auto-detect email address"), // commit-tree
    ]);

    let err = create_coordination_worktree_with_runner(&runner, tmp.path(), "coord/main", &target)
        .unwrap_err();

    let msg = err.to_string();
    assert!(msg.contains("coord/main"), "should name the branch: {msg}");
    assert!(
        msg.contains("user.email") || msg.contains("commit"),
        "msg: {msg}"
    );
}

// ── remove: clean removal succeeds ───────────────────────────────────────────

#[test]
fn remove_runs_worktree_remove_then_prune() {
    let tmp = tempfile::TempDir::new().unwrap();
    let target = tmp.path().join("wt");

    // Sequence:
    //   1. worktree remove     → success
    //   2. worktree prune      → success
    let runner = StubRunner::with_outputs(vec![ok(""), ok("")]);

    remove_coordination_worktree_with_runner(&runner, tmp.path(), &target).unwrap();

    let calls = runner.calls.borrow();
    assert_eq!(calls[0][0..2], ["worktree", "remove"]);
    assert!(
        !calls[0].contains(&"--force".to_string()),
        "should not use --force on first try"
    );
    assert_eq!(calls[1], ["worktree", "prune"]);
}

// ── remove: falls back to --force ────────────────────────────────────────────

#[test]
fn remove_falls_back_to_force_when_clean_remove_fails() {
    let tmp = tempfile::TempDir::new().unwrap();
    let target = tmp.path().join("wt");

    // Sequence:
    //   1. worktree remove          → fail (dirty worktree)
    //   2. worktree remove --force  → success
    //   3. worktree prune           → success
    let runner = StubRunner::with_outputs(vec![
        fail("fatal: 'wt' contains modified or untracked files"),
        ok(""),
        ok(""),
    ]);

    remove_coordination_worktree_with_runner(&runner, tmp.path(), &target).unwrap();

    let calls = runner.calls.borrow();
    assert_eq!(calls[1][0..3], ["worktree", "remove", "--force"]);
    assert_eq!(calls[2], ["worktree", "prune"]);
}

// ── remove: both remove attempts fail ────────────────────────────────────────

#[test]
fn remove_returns_error_when_force_remove_also_fails() {
    let tmp = tempfile::TempDir::new().unwrap();
    let target = tmp.path().join("wt");

    let runner = StubRunner::with_outputs(vec![
        fail("fatal: not a worktree"),
        fail("fatal: not a worktree"),
        ok(""),
    ]);

    let err = remove_coordination_worktree_with_runner(&runner, tmp.path(), &target).unwrap_err();

    let msg = err.to_string();
    assert!(
        msg.contains("worktree") || msg.contains(target.to_string_lossy().as_ref()),
        "should mention the worktree: {msg}",
    );
    assert!(msg.contains("Fix:"), "should suggest a fix: {msg}");
}

// ── remove: prune fails ───────────────────────────────────────────────────────

#[test]
fn remove_returns_error_when_prune_fails() {
    let tmp = tempfile::TempDir::new().unwrap();
    let target = tmp.path().join("wt");

    let runner = StubRunner::with_outputs(vec![ok(""), fail("error: unable to prune")]);

    let err = remove_coordination_worktree_with_runner(&runner, tmp.path(), &target).unwrap_err();

    let msg = err.to_string();
    assert!(msg.contains("prune"), "should mention prune: {msg}");
}

// ── auto_commit: successful commit path ──────────────────────────────────────

#[test]
fn auto_commit_stages_and_commits_when_changes_exist() {
    let tmp = tempfile::TempDir::new().unwrap();
    let wt = tmp.path().join("coord-wt");

    // Sequence:
    //   1. git -C <wt> add -A                  → success
    //   2. git -C <wt> diff --cached --quiet   → exit 1 (changes exist)
    //   3. git -C <wt> commit -m "msg"         → success
    let runner = StubRunner::with_outputs(vec![
        ok(""),
        Ok(ProcessOutput {
            exit_code: 1,
            success: false,
            stdout: String::new(),
            stderr: String::new(),
            timed_out: false,
        }),
        ok(""),
    ]);

    auto_commit_coordination_with_runner(&runner, &wt, "chore: sync state").unwrap();

    let calls = runner.calls.borrow();
    assert_eq!(calls.len(), 3);

    // add -A
    assert_eq!(calls[0], ["-C", wt.to_str().unwrap(), "add", "-A"]);
    // diff --cached --quiet
    assert_eq!(
        calls[1],
        ["-C", wt.to_str().unwrap(), "diff", "--cached", "--quiet"]
    );
    // commit -m
    assert_eq!(
        calls[2],
        [
            "-C",
            wt.to_str().unwrap(),
            "commit",
            "-m",
            "chore: sync state"
        ]
    );
}

// ── auto_commit: no-op when nothing staged ────────────────────────────────────

#[test]
fn auto_commit_is_noop_when_nothing_staged() {
    let tmp = tempfile::TempDir::new().unwrap();
    let wt = tmp.path().join("coord-wt");

    // Sequence:
    //   1. git add -A                  → success
    //   2. git diff --cached --quiet   → exit 0 (nothing staged)
    //   (no commit call)
    let runner = StubRunner::with_outputs(vec![ok(""), ok("")]);

    auto_commit_coordination_with_runner(&runner, &wt, "chore: sync state").unwrap();

    let calls = runner.calls.borrow();
    assert_eq!(
        calls.len(),
        2,
        "commit must not be called when nothing staged"
    );
}

// ── auto_commit: git add failure ─────────────────────────────────────────────

#[test]
fn auto_commit_returns_error_when_git_add_fails() {
    let tmp = tempfile::TempDir::new().unwrap();
    let wt = tmp.path().join("coord-wt");

    let runner = StubRunner::with_outputs(vec![fail(
        "fatal: not a git repository (or any of the parent directories): .git",
    )]);

    let err = auto_commit_coordination_with_runner(&runner, &wt, "chore: sync state").unwrap_err();

    let msg = err.to_string();
    assert!(
        msg.contains("stage") || msg.contains("add"),
        "should mention staging: {msg}"
    );
    assert!(
        msg.contains(wt.to_string_lossy().as_ref()),
        "should include worktree path: {msg}"
    );
    assert!(msg.contains("Fix:"), "should suggest a fix: {msg}");
}

// ── auto_commit: commit failure ───────────────────────────────────────────────

#[test]
fn auto_commit_returns_error_when_commit_fails() {
    let tmp = tempfile::TempDir::new().unwrap();
    let wt = tmp.path().join("coord-wt");

    let runner = StubRunner::with_outputs(vec![
        ok(""),
        Ok(ProcessOutput {
            exit_code: 1,
            success: false,
            stdout: String::new(),
            stderr: String::new(),
            timed_out: false,
        }),
        fail("error: unable to auto-detect email address"),
    ]);

    let err = auto_commit_coordination_with_runner(&runner, &wt, "chore: sync state").unwrap_err();

    let msg = err.to_string();
    assert!(
        msg.contains("commit") || msg.contains("user.email"),
        "should mention commit or user.email: {msg}"
    );
    assert!(
        msg.contains(wt.to_string_lossy().as_ref()),
        "should include worktree path: {msg}"
    );
}

// ── integration: real git repo ────────────────────────────────────────────────

/// End-to-end test using a real temporary git repository.
///
/// Verifies that:
/// - A coordination worktree is created with the correct branch.
/// - The `.ito/` subdirectory structure is present inside the worktree.
/// - The worktree is cleanly removed and pruned.
#[test]
fn integration_create_and_remove_coordination_worktree() {
    let tmp = tempfile::TempDir::new().unwrap();
    let repo = tmp.path().join("repo");
    let worktree = tmp.path().join("coord-wt");

    // Initialise a bare-minimum git repo with an initial commit so we can
    // branch from it.
    init_test_repo(&repo);

    create_coordination_worktree(&repo, "coord/test", &worktree).expect("create should succeed");

    // Worktree directory must exist.
    assert!(worktree.exists(), "worktree directory should exist");

    // All .ito subdirs must be present.
    for subdir in ITO_SUBDIRS {
        let dir = worktree.join(".ito").join(subdir);
        assert!(dir.exists(), ".ito/{subdir} should exist inside worktree");
    }

    // The branch must be checked out in the worktree.
    // Use `git rev-parse --abbrev-ref HEAD` inside the worktree — reading
    // the HEAD file directly is unreliable because in a linked worktree the
    // `.git` entry is a file (gitdir pointer), not a directory.
    let head_output = std::process::Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir(&worktree)
        .output()
        .expect("git rev-parse should run");
    let head = String::from_utf8_lossy(&head_output.stdout)
        .trim()
        .to_string();
    assert!(
        head.contains("coord/test"),
        "worktree HEAD should reference coord/test, got: {head}",
    );

    remove_coordination_worktree(&repo, &worktree).expect("remove should succeed");

    assert!(
        !worktree.exists(),
        "worktree directory should be gone after removal"
    );
}

/// End-to-end test for `auto_commit_coordination` using a real git repo.
///
/// Verifies that:
/// - Writing a file and calling `auto_commit_coordination` produces a new commit.
/// - Calling it again with no changes is a no-op (commit count unchanged).
#[test]
fn integration_auto_commit_coordination() {
    let tmp = tempfile::TempDir::new().unwrap();
    let repo = tmp.path().join("repo");
    let worktree = tmp.path().join("coord-wt");

    init_test_repo(&repo);
    create_coordination_worktree(&repo, "coord/auto-commit-test", &worktree)
        .expect("create should succeed");

    let git = |args: &[&str]| -> String {
        let out = std::process::Command::new("git")
            .args(args)
            .current_dir(&worktree)
            .env("GIT_AUTHOR_NAME", "Test")
            .env("GIT_AUTHOR_EMAIL", "test@example.com")
            .env("GIT_COMMITTER_NAME", "Test")
            .env("GIT_COMMITTER_EMAIL", "test@example.com")
            .output()
            .expect("git command failed");
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    };

    let commit_count = || -> usize { git(&["rev-list", "--count", "HEAD"]).parse().unwrap_or(0) };

    let before = commit_count();

    // Write a file so there is something to commit.
    fs::write(worktree.join("state.json"), b"{}").unwrap();

    auto_commit_coordination(&worktree, "chore: sync state").expect("auto_commit should succeed");

    assert_eq!(
        commit_count(),
        before + 1,
        "a new commit should have been created"
    );

    // Second call with no new changes must be a no-op.
    auto_commit_coordination(&worktree, "chore: sync state")
        .expect("auto_commit no-op should succeed");

    assert_eq!(
        commit_count(),
        before + 1,
        "no extra commit when nothing changed"
    );

    remove_coordination_worktree(&repo, &worktree).expect("remove should succeed");
}

// ── maybe_auto_commit_coordination ───────────────────────────────────────────

/// Returns a minimal `ito.json` payload that sets `coordination_branch.storage`
/// to the given value.
fn write_ito_json_with_storage(project_root: &std::path::Path, storage: &str) {
    let json = format!(r#"{{"changes":{{"coordination_branch":{{"storage":"{storage}"}}}}}}"#);
    std::fs::write(project_root.join("ito.json"), json).unwrap();
}

#[test]
fn maybe_auto_commit_is_noop_when_storage_is_embedded() {
    let tmp = tempfile::TempDir::new().unwrap();
    let project_root = tmp.path();

    // Write config with embedded storage.
    write_ito_json_with_storage(project_root, "embedded");

    // The worktree directory does not exist, but even if it did the function
    // should return Ok without attempting a commit because storage != worktree.
    let result = maybe_auto_commit_coordination(
        project_root,
        &project_root.join(".ito"),
        "chore: sync state",
    );

    assert!(
        result.is_ok(),
        "embedded storage should be a no-op: {result:?}"
    );
}

#[test]
fn maybe_auto_commit_is_noop_when_worktree_dir_does_not_exist() {
    let tmp = tempfile::TempDir::new().unwrap();
    let project_root = tmp.path();

    // Write config with worktree storage (the default).
    write_ito_json_with_storage(project_root, "worktree");

    // The coordination worktree directory does not exist on disk, so the
    // function should skip the commit silently.
    let result = maybe_auto_commit_coordination(
        project_root,
        &project_root.join(".ito"),
        "chore: sync state",
    );

    assert!(
        result.is_ok(),
        "missing worktree dir should be a no-op: {result:?}"
    );
}

#[test]
fn maybe_auto_commit_calls_auto_commit_when_worktree_mode_and_dir_exists() {
    let tmp = tempfile::TempDir::new().unwrap();
    let project_root = tmp.path();

    // Write config with worktree storage and an explicit worktree_path so we
    // can control where the function looks.
    let coord_wt = tmp.path().join("coord-wt");
    std::fs::create_dir_all(&coord_wt).unwrap();

    let json = serde_json::json!({
        "changes": {
            "coordination_branch": {
                "storage": "worktree",
                "worktree_path": coord_wt.to_str().unwrap()
            }
        }
    });
    std::fs::write(project_root.join("ito.json"), json.to_string()).unwrap();

    // Initialise a real git repo inside coord_wt so auto_commit_coordination
    // can actually run git commands.
    init_test_repo(&coord_wt);

    let result = maybe_auto_commit_coordination(
        project_root,
        &project_root.join(".ito"),
        "chore: sync state",
    );

    // The call should succeed (git add + diff --cached + optional commit).
    assert!(
        result.is_ok(),
        "worktree mode with existing dir should succeed: {result:?}"
    );
}

/// Initialises a minimal git repository with a single empty commit.
fn init_test_repo(repo: &std::path::Path) {
    fs::create_dir_all(repo).unwrap();

    let run = |args: &[&str]| {
        std::process::Command::new("git")
            .args(args)
            .current_dir(repo)
            .env("GIT_AUTHOR_NAME", "Test")
            .env("GIT_AUTHOR_EMAIL", "test@example.com")
            .env("GIT_COMMITTER_NAME", "Test")
            .env("GIT_COMMITTER_EMAIL", "test@example.com")
            .output()
            .expect("git command failed")
    };

    run(&["init", "-b", "main"]);
    run(&["config", "user.email", "test@example.com"]);
    run(&["config", "user.name", "Test"]);
    run(&["commit", "--allow-empty", "-m", "init"]);
}
