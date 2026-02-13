#![allow(dead_code)]

use std::path::Path;

use ito_test_support::reset_dir;

pub(crate) fn write(path: impl AsRef<Path>, contents: &str) {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(path, contents).unwrap();
}

pub(crate) fn reset_repo(dst: &Path, src: &Path) {
    reset_dir(dst, src).unwrap();
}

pub(crate) fn make_repo_with_spec_change_fixture() -> tempfile::TempDir {
    let td = tempfile::tempdir().expect("repo");
    write(td.path().join("README.md"), "# temp\n");

    // Minimal module.
    write(
        td.path().join(".ito/modules/000_ungrouped/module.md"),
        "# Ungrouped\n\n## Purpose\nModule for ad-hoc changes. This purpose is long enough.\n\n## Scope\n- *\n\n## Changes\n- [ ] 000-01_test-change\n",
    );

    // Valid spec.
    write(
        td.path().join(".ito/specs/alpha/spec.md"),
        "# Alpha\n\n## Purpose\nThis purpose text is intentionally long enough to avoid strict-mode warnings.\n\n## Requirements\n\n### Requirement: Alpha Behavior\nThe system SHALL do the alpha thing.\n\n#### Scenario: Alpha works\n- **WHEN** the user triggers alpha\n- **THEN** the system performs alpha\n",
    );

    // Invalid spec (missing Purpose/Requirements structure in strict mode).
    write(
        td.path().join(".ito/specs/beta/spec.md"),
        "# Beta\n\nThis spec is intentionally invalid.\n",
    );

    // Valid change with one valid delta.
    write(
        td.path()
            .join(".ito/changes/000-01_test-change/proposal.md"),
        "## Why\nTest fixture\n\n## What Changes\n- Adds a small delta\n\n## Impact\n- None\n",
    );
    write(
        td.path().join(".ito/changes/000-01_test-change/tasks.md"),
        "## 1. Implementation\n- [x] 1.1 Do a thing\n",
    );
    write(
        td.path()
            .join(".ito/changes/000-01_test-change/specs/alpha/spec.md"),
        "## ADDED Requirements\n\n### Requirement: Alpha Delta\nThe system SHALL include alpha delta behavior in strict validation.\n\n#### Scenario: Delta ok\n- **WHEN** running validation\n- **THEN** it passes\n",
    );

    // An invalidly named change directory to exercise validation error paths.
    write(
        td.path().join(".ito/changes/not-a-change/proposal.md"),
        "## Why\nBad id\n\n## What Changes\n- None\n\n## Impact\n- None\n",
    );

    // An ambiguous item id: both a spec and a (badly-named) change directory.
    write(
        td.path().join(".ito/changes/alpha/proposal.md"),
        "## Why\nAmbiguous\n\n## What Changes\n- None\n\n## Impact\n- None\n",
    );

    td
}

pub(crate) fn make_empty_repo() -> tempfile::TempDir {
    let td = tempfile::tempdir().expect("repo");
    write(td.path().join("README.md"), "# temp\n");
    td
}

pub(crate) fn make_repo_all_valid() -> tempfile::TempDir {
    let td = tempfile::tempdir().expect("repo");
    write(td.path().join("README.md"), "# temp\n");

    // Module.
    write(
        td.path().join(".ito/modules/000_ungrouped/module.md"),
        "# Ungrouped\n\n## Purpose\nModule for ad-hoc changes. This purpose is long enough.\n\n## Scope\n- *\n\n## Changes\n- [ ] 000-01_test-change\n",
    );

    // Valid spec.
    write(
        td.path().join(".ito/specs/alpha/spec.md"),
        "# Alpha\n\n## Purpose\nThis purpose text is intentionally long enough to avoid strict-mode warnings.\n\n## Requirements\n\n### Requirement: Alpha Behavior\nThe system SHALL do the alpha thing.\n\n#### Scenario: Alpha works\n- **WHEN** the user triggers alpha\n- **THEN** the system performs alpha\n",
    );

    // Valid change with one valid delta.
    write(
        td.path()
            .join(".ito/changes/000-01_test-change/proposal.md"),
        "## Why\nTest fixture\n\n## What Changes\n- Adds a small delta\n\n## Impact\n- None\n",
    );
    write(
        td.path().join(".ito/changes/000-01_test-change/tasks.md"),
        "## 1. Implementation\n- [x] 1.1 Do a thing\n",
    );
    write(
        td.path()
            .join(".ito/changes/000-01_test-change/specs/alpha/spec.md"),
        "## ADDED Requirements\n\n### Requirement: Alpha Delta\nThe system SHALL include alpha delta behavior in strict validation.\n\n#### Scenario: Delta ok\n- **WHEN** running validation\n- **THEN** it passes\n",
    );

    td
}

pub(crate) fn make_repo_changes_dir_but_empty() -> tempfile::TempDir {
    let td = tempfile::tempdir().expect("repo");
    write(td.path().join("README.md"), "# temp\n");
    std::fs::create_dir_all(td.path().join(".ito/changes")).unwrap();
    td
}

pub(crate) fn write_local_ito_skills(root: &Path) {
    // Avoid network fetches for adapter installation by providing a minimal local
    // ito-skills/ directory.
    let base = root.join("ito-skills");

    // Minimal adapter files.
    write(
        base.join("adapters/opencode/ito-skills.js"),
        "// test plugin\n",
    );
    write(
        base.join("adapters/claude/session-start.sh"),
        "#!/usr/bin/env sh\necho test\n",
    );
    write(base.join(".codex/ito-skills-bootstrap.md"), "# Bootstrap\n");

    // Must match ito-core `distribution.rs` ITO_SKILLS list.
    let skills = [
        "brainstorming",
        "dispatching-parallel-agents",
        "finishing-a-development-branch",
        "receiving-code-review",
        "requesting-code-review",
        "research",
        "subagent-driven-development",
        "systematic-debugging",
        "test-driven-development",
        "using-git-worktrees",
        "using-ito-skills",
        "verification-before-completion",
        "writing-skills",
    ];
    for skill in skills {
        write(
            base.join(format!("skills/{skill}/SKILL.md")),
            &format!("# {skill}\n"),
        );
    }
}

/// Builds a deterministic minimal argument vector for initializing the repository tooling.
///
/// The returned vector contains, in order: `"init"`, the provided repository path as a string, `"--tools"`, and `"none"`.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// let args = init_minimal_args(Path::new("/repo"));
/// assert_eq!(
///     args,
///     vec!["init".to_string(), "/repo".to_string(), "--tools".to_string(), "none".to_string()]
/// );
/// ```
pub(crate) fn init_minimal_args(repo_path: &Path) -> Vec<String> {
    // Keep args deterministic and avoid interactive prompts.
    vec![
        "init".to_string(),
        repo_path.to_string_lossy().to_string(),
        "--tools".to_string(),
        "none".to_string(),
    ]
}

/// Create a vector of string slices that borrow from the provided Strings.
///
/// The returned vector contains `&str` slices that reference the input `String` values.
///
/// # Examples
///
/// ```
/// let s = vec!["a".to_string(), "bc".to_string()];
/// let refs = args_to_strs(&s);
/// assert_eq!(refs, vec!["a", "bc"]);
/// ```
pub(crate) fn args_to_strs(args: &[String]) -> Vec<&str> {
    args.iter().map(|s| s.as_str()).collect()
}

/// Runs a git command in the given repository directory and asserts it completed successfully.
///
/// The command is executed with the working directory set to `repo` and with `GIT_DIR` and
/// `GIT_WORK_TREE` removed from the environment to avoid interfering with repository selection.
/// Panics if the `git` executable cannot be spawned or if the command exits with a non-zero status;
/// stdout and stderr are included in the panic message.
///
/// # Examples
///
/// ```
/// // Runs `git --version` in a temporary directory to verify invocation succeeds.
/// let tmp = tempfile::tempdir().unwrap();
/// run_git(tmp.path(), &["--version"]);
/// ```
fn run_git(repo: &Path, args: &[&str]) {
    let output = std::process::Command::new("git")
        .args(args)
        .current_dir(repo)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .output()
        .expect("git command should run");
    assert!(
        output.status.success(),
        "git command failed: git {}\nstdout:\n{}\nstderr:\n{}",
        args.join(" "),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Initialize a git repository at `repo`, configure a test user identity, stage `README.md` if present, and create an initial commit.
///
/// # Examples
///
/// ```
/// use std::fs;
/// use tempfile::tempdir;
///
/// let td = tempdir().unwrap();
/// let repo = td.path();
/// fs::write(repo.join("README.md"), "hello").unwrap();
///
/// // crate-local test helper
/// ito_cli::tests::support::git_init_with_initial_commit(repo);
///
/// assert!(repo.join(".git").exists());
/// ```
pub(crate) fn git_init_with_initial_commit(repo: &Path) {
    run_git(repo, &["init"]);
    run_git(repo, &["config", "user.email", "test@example.com"]);
    run_git(repo, &["config", "user.name", "Test User"]);
    let readme = repo.join("README.md");
    if readme.exists() {
        run_git(repo, &["add", "README.md"]);
    }
    run_git(repo, &["commit", "--no-verify", "-m", "initial"]);
}

/// Creates a bare Git repository in a temporary directory for use as an `origin` remote in tests.
///
/// The returned `TempDir` contains the bare repository; keep it alive while the remote is needed.
///
/// # Examples
///
/// ```
/// let remote = make_bare_remote();
/// // use `remote.path()` as the git remote URL while `remote` is kept alive
/// assert!(remote.path().exists());
/// ```
pub(crate) fn make_bare_remote() -> tempfile::TempDir {
    let td = tempfile::tempdir().expect("remote");
    let output = std::process::Command::new("git")
        .args(["init", "--bare", td.path().to_string_lossy().as_ref()])
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .output()
        .expect("git bare init should run");
    assert!(
        output.status.success(),
        "git bare init failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    td
}

/// Configure the Git `origin` remote for a test repository to point at the provided bare remote.
///
/// Call this after `git_init_with_initial_commit` when a test needs to fetch from or push to an origin.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// let repo = tempfile::tempdir().unwrap();
/// // prepare a repository with an initial commit
/// git_init_with_initial_commit(repo.path());
/// // create a bare remote
/// let remote = make_bare_remote();
/// // add origin pointing at the bare remote
/// add_origin(repo.path(), remote.path());
/// ```
pub(crate) fn add_origin(repo: &Path, remote: &Path) {
    run_git(
        repo,
        &["remote", "add", "origin", remote.to_string_lossy().as_ref()],
    );
}
